///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    struct OperatorReward { address operator; uint256 amount; }
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDirectedRewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorRewards:
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
        #[allow(missing_docs)]
        pub description: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<OperatorReward>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<<OperatorReward as alloy::sol_types::SolType>::RustType>,
            u32,
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
        impl ::core::convert::From<OperatorDirectedRewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorDirectedRewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.operatorRewards,
                    value.startTimestamp,
                    value.duration,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorDirectedRewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    operatorRewards: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                    description: tuple.5,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorDirectedRewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorDirectedRewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorRewards),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
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
        impl alloy_sol_types::SolType for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::SolStruct for OperatorDirectedRewardsSubmission {
            const NAME: &'static str = "OperatorDirectedRewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDirectedRewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,OperatorReward[] operatorRewards,uint32 startTimestamp,uint32 duration,string description)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components.push(<OperatorReward as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<OperatorReward as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorRewards,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.description,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorDirectedRewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorReward,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorRewards,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.description,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    OperatorReward,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorRewards,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.description,
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
    struct OperatorReward { address operator; uint256 amount; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorReward {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<OperatorReward> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorReward) -> Self {
                (value.operator, value.amount)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorReward {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    amount: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for OperatorReward {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for OperatorReward {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amount,
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
        impl alloy_sol_types::SolType for OperatorReward {
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
        impl alloy_sol_types::SolStruct for OperatorReward {
            const NAME: &'static str = "OperatorReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorReward(address operator,uint256 amount)",
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for OperatorReward {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
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
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
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
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        #[allow(missing_docs)]
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub startTimestamp: u32,
        #[allow(missing_docs)]
        pub duration: u32,
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
            alloy::sol_types::sol_data::Array<StrategyAndMultiplier>,
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<
                <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
            >,
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U256,
            u32,
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
        impl ::core::convert::From<RewardsSubmission> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsSubmission) -> Self {
                (
                    value.strategiesAndMultipliers,
                    value.token,
                    value.amount,
                    value.startTimestamp,
                    value.duration,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsSubmission {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategiesAndMultipliers: tuple.0,
                    token: tuple.1,
                    amount: tuple.2,
                    startTimestamp: tuple.3,
                    duration: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsSubmission {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsSubmission {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.strategiesAndMultipliers,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.duration),
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                );
                components.extend(
                    <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.strategiesAndMultipliers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.startTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.duration)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsSubmission {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        StrategyAndMultiplier,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategiesAndMultipliers,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.startTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.duration,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    StrategyAndMultiplier,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategiesAndMultipliers,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.startTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.duration,
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
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        impl ::core::convert::From<StrategyAndMultiplier> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAndMultiplier) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAndMultiplier {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyAndMultiplier {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
        impl alloy_sol_types::EventTopic for StrategyAndMultiplier {
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
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

    See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorTypesInstance<T, P, N> {
        IRewardsCoordinatorTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinatorTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IRewardsCoordinatorTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorTypesInstance")
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

        See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorTypesInstance<T, P, N> {
            IRewardsCoordinatorTypesInstance {
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
        > IRewardsCoordinatorTypesInstance<T, P, N>
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
library ISignatureUtilsMixinTypes {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod ISignatureUtilsMixinTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<SignatureWithSaltAndExpiry> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
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
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
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
    /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

    See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
        ISignatureUtilsMixinTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtilsMixinTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`ISignatureUtilsMixinTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsMixinTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsMixinTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsMixinTypesInstance")
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`ISignatureUtilsMixinTypes`](self) contract instance.

        See the [wrapper's documentation](`ISignatureUtilsMixinTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsMixinTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsMixinTypesInstance<T, P, N> {
            ISignatureUtilsMixinTypesInstance {
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
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
        > ISignatureUtilsMixinTypesInstance<T, P, N>
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
library IRewardsCoordinatorTypes {
    struct OperatorDirectedRewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        OperatorReward[] operatorRewards;
        uint32 startTimestamp;
        uint32 duration;
        string description;
    }
    struct OperatorReward {
        address operator;
        uint256 amount;
    }
    struct RewardsSubmission {
        StrategyAndMultiplier[] strategiesAndMultipliers;
        address token;
        uint256 amount;
        uint32 startTimestamp;
        uint32 duration;
    }
    struct StrategyAndMultiplier {
        address strategy;
        uint96 multiplier;
    }
}

library ISignatureUtilsMixinTypes {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface AwesomeVaultServiceManager {
    error DelayPeriodNotPassed();
    error OnlyRegistryCoordinator();
    error OnlyRewardsInitiator();
    error OnlyStakeRegistry();

    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);

    constructor(address _avsDirectory, address _registryCoordinator, address _stakeRegistry, address rewards_coordinator, address allocationManager, address _permissionController, address _awesomeVaultTaskManager);

    function addPendingAdmin(address admin) external;
    function avsDirectory() external view returns (address);
    function awesomeVaultTaskManager() external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    function getRestakeableStrategies() external view returns (address[] memory);
    function initialize(address initialOwner, address rewardsInitiator) external;
    function owner() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function removeAdmin(address admin) external;
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    function removePendingAdmin(address pendingAdmin) external;
    function renounceOwnership() external;
    function rewardsInitiator() external view returns (address);
    function setAppointee(address appointee, address target, bytes4 selector) external;
    function setClaimerFor(address claimer) external;
    function setRewardsInitiator(address newRewardsInitiator) external;
    function transferOwnership(address newOwner) external;
    function updateAVSMetadataURI(string memory _metadataURI) external;
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
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_stakeRegistry",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      },
      {
        "name": "rewards_coordinator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "allocationManager",
        "type": "address",
        "internalType": "contract IAllocationManager"
      },
      {
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
      },
      {
        "name": "_awesomeVaultTaskManager",
        "type": "address",
        "internalType": "contract IAwesomeVaultTaskManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "addPendingAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "avsDirectory",
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
    "name": "awesomeVaultTaskManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAwesomeVaultTaskManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "amount",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
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
    "name": "createOperatorDirectedAVSRewardsSubmission",
    "inputs": [
      {
        "name": "operatorDirectedRewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.StrategyAndMultiplier[]",
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
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "operatorRewards",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinatorTypes.OperatorReward[]",
            "components": [
              {
                "name": "operator",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "amount",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "startTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "duration",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "description",
            "type": "string",
            "internalType": "string"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
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
    "name": "getOperatorRestakedStrategies",
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
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRestakeableStrategies",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "address[]"
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
        "name": "rewardsInitiator",
        "type": "address",
        "internalType": "address"
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
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
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
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeAdmin",
    "inputs": [
      {
        "name": "admin",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removePendingAdmin",
    "inputs": [
      {
        "name": "pendingAdmin",
        "type": "address",
        "internalType": "address"
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
    "name": "rewardsInitiator",
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
    "name": "setAppointee",
    "inputs": [
      {
        "name": "appointee",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "target",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "selector",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setClaimerFor",
    "inputs": [
      {
        "name": "claimer",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsInitiator",
    "inputs": [
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
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
    "type": "function",
    "name": "updateAVSMetadataURI",
    "inputs": [
      {
        "name": "_metadataURI",
        "type": "string",
        "internalType": "string"
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
    "name": "RewardsInitiatorUpdated",
    "inputs": [
      {
        "name": "prevRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newRewardsInitiator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "DelayPeriodNotPassed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRewardsInitiator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyStakeRegistry",
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
pub mod AwesomeVaultServiceManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101603461024957601f6122f338819003918201601f19168301916001600160401b0383118484101761024d5780849260e094604052833981010312610249578051906001600160a01b0382168203610249576020810151906001600160a01b03821682036102495760408101516001600160a01b03811681036102495760608201516001600160a01b0381169390849003610249576080830151936001600160a01b03851685036102495760a0840151936001600160a01b03851685036102495760c00151956001600160a01b03871687036102495760805260c05260e052610100526101205260a0525f5460ff8160081c166101f45760ff808216036101ba575b5061014052604051612091908161026282396080518181816106d90152818161078801528181610ca50152610de3015260a051816104fe015260c0518181816101490152818161082d0152610aec015260e0518181816104a50152818161075601528181610c78015281816114b00152611a0d0152610100518181816115ca0152611a5901526101205181818161060501528181610b7901528181611026015281816110a50152611143015261014051816103f80152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f610102565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f5f3560e01c80631785f53c1461111f5780631fdb0cfd14611088578063279432eb1461100157806333cfb7b714610fdc5780633bc28c8c14610fb7578063485cc95514610e125780636b3aa72e14610dcd578063715018a614610d705780638da5cb5b14610d475780639926ee7d14610bdb5780639da16d8e14610b54578063a0169ddd14610ac7578063a20b99bf146107f0578063a364f4da1461073a578063a98fb3551461068c578063ba550880146105e8578063c1a8e2c514610427578063d4ad1d53146103e2578063e481af9d146103b6578063f2fde38b14610325578063fc299dee146102fc5763fce36c7d1461010c575f80fd5b346101e95760203660031901126101e9576004356001600160401b0381116102f85761013c903690600401611326565b610147929192611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690825b8181106102945750813b15610290576040519363fce36c7d60e01b8552816024860160206004880152526044850160448360051b87010192828690609e19813603015b8383106101f75788808b8181808c0381838f5af180156101ec576101d85750f35b816101e2916112b4565b6101e95780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198a820301865286358281121561028c5760206001928582930190608063ffffffff61027a826102406102328780611966565b60a0885260a088019161199a565b95898060a01b036102528983016111e4565b1688870152604081013560408701528361026e6060830161136d565b1660608701520161136d565b169101529801960194930191906101b7565b8980fd5b8280fd5b806102c96102b060206102aa600195878b611c5e565b01611952565b60406102bd84878b611c5e565b01359030903390611da1565b6102f26102dc60206102aa84878b611c5e565b8560406102ea85888c611c5e565b013591611dec565b01610174565b5080fd5b50346101e957806003193601126101e9576065546040516001600160a01b039091168152602090f35b50346101e95760203660031901126101e95761033f6111ce565b610347611c80565b6001600160a01b038116156103625761035f90611d36565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b50346101e957806003193601126101e9576103de6103d26119fe565b60405191829182611243565b0390f35b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760403660031901126101e9576104416111ce565b90602435916001600160401b0383116102f857366023840112156102f85782600401359261046e84611356565b9361047c60405195866112b4565b8085526024602086019160051b830101913683116105e457602401905b8282106105cc575050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036105bd57604051906104e082611285565b6001600160a01b0390811682523060208301908152604083019485527f000000000000000000000000000000000000000000000000000000000000000090911693909190843b156105b957604051636e3492b560e01b815260206004820181905292516001600160a01b03908116602483015293519093166044840152516060606484015280516084840181905260a48401929190910190845b81811061059d57505050818394818581819503925af180156101ec576101d85750f35b825163ffffffff1684526020938401939092019160010161057a565b8380fd5b634394dbdf60e11b8252600482fd5b602080916105d98461136d565b815201910190610499565b8480fd5b50346101e957806105f8366111f8565b610603929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156105e457604051630664120160e01b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b031916606483015282908290818381608481015b03925af180156101ec576101d85750f35b50346101e95760203660031901126101e957806004356001600160401b0381116107375736602382011215610737576106cf9036906024816004013591016112f0565b6106d7611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107355760405163a98fb35560e01b815260206004820152918391839182908490829061067b9060248301906118d7565b505b50fd5b50346101e95760203660031901126101e9576107546111ce565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036105bd5781907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610735576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e9576004356001600160401b0381116102f857610821903690600401611326565b919061082b611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316825b848110610a125750803b156102905760409391935191634e5cd2fd60e11b8352806044840130600486015260406024860152526064830160648260051b850101918691869760be19813603015b838a106108c5578880898181808b0381838e5af180156101ec576101d85750f35b909192939460631988820301845285358281121561028c578301906108fb6108ed8380611966565b60c0845260c084019161199a565b916001600160a01b03610910602083016111e4565b16602083015260206109256040830183611966565b848603604086015280865294909101938c905b8082106109e25750505063ffffffff6109536060830161136d565b16606083015263ffffffff61096a6080830161136d565b16608083015260a0810135601e19823603018112156109de570190602082359201906001600160401b0383116109de5782360382136109de57838360209485948460a0879660019a03910152818452848401378d838284010152601f801991011601019701940199019891909392936108a4565b8b80fd5b909194604080600192838060a01b036109fa8a6111e4565b16815260208901356020820152019601920190610938565b90929193829483955b610a33610a298585856118fb565b604081019061191d565b9050871015610a8357610a4a610a298585856118fb565b881015610a6f576001916020610a67928a60061b010135906113ee565b960195610a1b565b634e487b7160e01b86526032600452602486fd5b919550929391600191610ac190610aab813033610aa660206102aa898f8e6118fb565b611da1565b84610abc60206102aa868c8b6118fb565b611dec565b01610857565b50346101e95760203660031901126101e95780610ae26111ce565b610aea611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156107355760405163a0169ddd60e01b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e95780610b6f6111ce565b610b77611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561073557604051634f906cf960e01b81523060048201526001600160a01b039092166024830152829082908183816044810161067b565b50346101e95760403660031901126101e957610bf56111ce565b906024356001600160401b0381116102f857606060031982360301126102f85760405192610c2284611285565b81600401356001600160401b0381116105b9578201366023820112156105b957610c569036906024600482013591016112f0565b8452602084019060248301358252604460408601930135835260018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163303610d385792938493907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d345785610d1693819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a48501906118d7565b9151606484015251608483015203925af180156101ec576101d85750f35b8580fd5b634394dbdf60e11b8452600484fd5b50346101e957806003193601126101e9576033546040516001600160a01b039091168152602090f35b50346101e957806003193601126101e957610d89611c80565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760403660031901126101e957610e2c6111ce565b602435906001600160a01b03821682036102905782549160ff8360081c161592838094610faa575b8015610f93575b15610f375760ff198116600117855583610f26575b5060ff845460081c1615610ecd57610e8a610e8f92611d36565b611cd8565b610e965780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b61ffff19166101011784555f610e70565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b158015610e5b5750600160ff821614610e5b565b50600160ff821610610e54565b50346101e95760203660031901126101e95761035f610fd46111ce565b610e8a611c80565b50346101e95760203660031901126101e9576103de6103d2610ffc6111ce565b611490565b50346101e95760203660031901126101e9578061101c6111ce565b611024611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107355760405163eb5a4e8760e01b81523060048201526001600160a01b039092166024830152829082908183816044810161067b565b50346101e95780611098366111f8565b6110a3929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156105e457604051634a86c03760e11b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b0319166064830152829082908183816084810161067b565b50346111ca5760203660031901126111ca576111396111ce565b611141611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156111ca5760405163268959e560e01b81523060048201526001600160a01b039290921660248301525f908290604490829084905af180156111bf576111b1575080f35b6111bd91505f906112b4565b005b6040513d5f823e3d90fd5b5f80fd5b600435906001600160a01b03821682036111ca57565b35906001600160a01b03821682036111ca57565b60609060031901126111ca576004356001600160a01b03811681036111ca57906024356001600160a01b03811681036111ca57906044356001600160e01b0319811681036111ca5790565b60206040818301928281528451809452019201905f5b8181106112665750505090565b82516001600160a01b0316845260209384019390920191600101611259565b606081019081106001600160401b038211176112a057604052565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b038211176112a057604052565b6001600160401b0381116112a057601f01601f191660200190565b9291926112fc826112d5565b9161130a60405193846112b4565b8294818452818301116111ca578281602093845f960137010152565b9181601f840112156111ca578235916001600160401b0383116111ca576020808501948460051b0101116111ca57565b6001600160401b0381116112a05760051b60200190565b359063ffffffff821682036111ca57565b908160209103126111ca575160ff811681036111ca5790565b906113a182611356565b6113ae60405191826112b4565b82815280926113bf601f1991611356565b0190602036910137565b9081518110156113da570160200190565b634e487b7160e01b5f52603260045260245ffd5b919082018092116113fb57565b634e487b7160e01b5f52601160045260245ffd5b908160409103126111ca5760405190604082018281106001600160401b038211176112a0576040528051906001600160a01b03821682036111ca57602091835201516bffffffffffffffffffffffff811681036111ca57602082015290565b80518210156113da5760209160051b010190565b5f1981146113fb5760010190565b6040516309aa152760e11b81526001600160a01b039182166004820152907f000000000000000000000000000000000000000000000000000000000000000016602082602481845afa9182156111bf575f926118a3575b506040519163871ef04960e01b83526004830152602082602481845afa9182156111bf575f9261185f575b506001600160c01b0382169081159081156117fc575b506117e257805f915b6117bd575061ffff16611543816112d5565b9061155160405192836112b4565b808252611560601f19916112d5565b013660208301375f5f5b82518210806117b2575b156115c2576001811b84166001600160c01b031661159b575b61159690611482565b61156a565b9060016115969160ff60f81b8460f81b165f1a6115b882876113c9565b530191905061158d565b505f939250507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b815185101561167d5761160885836113c9565b5160f81c60405190633ca5a5f560e01b82526004820152602081602481875afa9081156111bf575f9161164b575b50611643906001926113ee565b9401936115f5565b90506020813d8211611675575b81611665602093836112b4565b810103126111ca57516001611636565b3d9150611658565b611688919450611397565b925f905f5b81518110156117ac576116a081836113c9565b5160f81c60405190633ca5a5f560e01b8252806004830152602082602481895afa9182156111bf575f92611779575b50905f915b8183106116e65750505060010161168d565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611742925f9161174b575b50838060a01b0390511661173c828d61146e565b52611482565b950191906116d4565b61176c915060403d8111611772575b61176481836112b4565b81019061140f565b5f611728565b503d61175a565b9091506020813d82116117a4575b81611794602093836112b4565b810103126111ca5751905f6116cf565b3d9150611787565b50505050565b506101008110611574565b5f1981018181116113fb5761ffff9116911661ffff81146113fb576001019080611531565b50506040516117f26020826112b4565b5f81525f36813790565b604051639aa1653d60e01b81529150602090829060049082905afa80156111bf5760ff915f91611830575b5016155f611528565b611852915060203d602011611858575b61184a81836112b4565b81019061137e565b5f611827565b503d611840565b9091506020813d60201161189b575b8161187b602093836112b4565b810103126111ca57516001600160c01b03811681036111ca57905f611512565b3d915061186e565b9091506020813d6020116118cf575b816118bf602093836112b4565b810103126111ca5751905f6114e7565b3d91506118b2565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91908110156113da5760051b8101359060be19813603018212156111ca570190565b903590601e19813603018212156111ca57018035906001600160401b0382116111ca57602001918160061b360383136111ca57565b356001600160a01b03811681036111ca5790565b9035601e19823603018112156111ca5701602081359101916001600160401b0382116111ca578160061b360383136111ca57565b916020908281520191905f905b8082106119b45750505090565b909192833560018060a01b0381168091036111ca5781526020840135906bffffffffffffffffffffffff82168092036111ca576040816001936020839401520194019201906119a7565b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156111bf5760ff915f91611c3f575b501680156117e2577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b808310611bc95750611a999150611397565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156111bf5760ff915f91611bab575b5016811015611ba457604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156111bf575f92611b71575b50905f915b818310611b1357505050600101611a9e565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611b68925f9161174b5750838060a01b0390511661173c828d61146e565b95019190611b01565b9091506020813d8211611b9c575b81611b8c602093836112b4565b810103126111ca5751905f611afc565b3d9150611b7f565b5092505050565b611bc3915060203d81116118585761184a81836112b4565b5f611ac4565b90604051633ca5a5f560e01b815260ff84166004820152602081602481885afa9081156111bf575f91611c0d575b50611c04906001926113ee565b92019190611a87565b90506020813d8211611c37575b81611c27602093836112b4565b810103126111ca57516001611bf7565b3d9150611c1a565b611c58915060203d6020116118585761184a81836112b4565b5f611a4f565b91908110156113da5760051b81013590609e19813603018212156111ca570190565b6033546001600160a01b03163303611c9457565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b03163303611d9257565b638e79fdb560e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152611dea91611de56084836112b4565b611eaa565b565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa9081156111bf575f91611e76575b50611dea93611e40916113ee565b60405163095ea7b360e01b60208201526001600160a01b0390931660248401526044808401919091528252611de56064836112b4565b90506020813d602011611ea2575b81611e91602093836112b4565b810103126111ca5751611dea611e32565b3d9150611e84565b90611f299160018060a01b03165f8060405193611ec86040866112b4565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d15611fba573d91611f0d836112d5565b92611f1b60405194856112b4565b83523d5f602085013e611fbe565b8051908115918215611f97575b505015611f3f57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b81925090602091810103126111ca576020015180151581036111ca575f80611f36565b6060915b919290156120205750815115611fd2575090565b3b15611fdb5790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b8251909150156120335750805190602001fd5b60405162461bcd60e51b8152602060048201529081906120579060248301906118d7565b0390fdfea2646970667358221220f67180e63c5d2f95005de488bb48e96b9a3040b21773e75eb6e37a705081da8764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01`4a\x02IW`\x1Fa\"\xF38\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02MW\x80\x84\x92`\xE0\x94`@R\x839\x81\x01\x03\x12a\x02IW\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02IW` \x81\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02IW`@\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02IW``\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x90\x84\x90\x03a\x02IW`\x80\x83\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x02IW`\xA0\x84\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x02IW`\xC0\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x87\x16\x87\x03a\x02IW`\x80R`\xC0R`\xE0Ra\x01\0Ra\x01 R`\xA0R_T`\xFF\x81`\x08\x1C\x16a\x01\xF4W`\xFF\x80\x82\x16\x03a\x01\xBAW[Pa\x01@R`@Qa \x91\x90\x81a\x02b\x829`\x80Q\x81\x81\x81a\x06\xD9\x01R\x81\x81a\x07\x88\x01R\x81\x81a\x0C\xA5\x01Ra\r\xE3\x01R`\xA0Q\x81a\x04\xFE\x01R`\xC0Q\x81\x81\x81a\x01I\x01R\x81\x81a\x08-\x01Ra\n\xEC\x01R`\xE0Q\x81\x81\x81a\x04\xA5\x01R\x81\x81a\x07V\x01R\x81\x81a\x0Cx\x01R\x81\x81a\x14\xB0\x01Ra\x1A\r\x01Ra\x01\0Q\x81\x81\x81a\x15\xCA\x01Ra\x1AY\x01Ra\x01 Q\x81\x81\x81a\x06\x05\x01R\x81\x81a\x0By\x01R\x81\x81a\x10&\x01R\x81\x81a\x10\xA5\x01Ra\x11C\x01Ra\x01@Q\x81a\x03\xF8\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01\x02V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x17\x85\xF5<\x14a\x11\x1FW\x80c\x1F\xDB\x0C\xFD\x14a\x10\x88W\x80c'\x942\xEB\x14a\x10\x01W\x80c3\xCF\xB7\xB7\x14a\x0F\xDCW\x80c;\xC2\x8C\x8C\x14a\x0F\xB7W\x80cH\\\xC9U\x14a\x0E\x12W\x80ck:\xA7.\x14a\r\xCDW\x80cqP\x18\xA6\x14a\rpW\x80c\x8D\xA5\xCB[\x14a\rGW\x80c\x99&\xEE}\x14a\x0B\xDBW\x80c\x9D\xA1m\x8E\x14a\x0BTW\x80c\xA0\x16\x9D\xDD\x14a\n\xC7W\x80c\xA2\x0B\x99\xBF\x14a\x07\xF0W\x80c\xA3d\xF4\xDA\x14a\x07:W\x80c\xA9\x8F\xB3U\x14a\x06\x8CW\x80c\xBAU\x08\x80\x14a\x05\xE8W\x80c\xC1\xA8\xE2\xC5\x14a\x04'W\x80c\xD4\xAD\x1DS\x14a\x03\xE2W\x80c\xE4\x81\xAF\x9D\x14a\x03\xB6W\x80c\xF2\xFD\xE3\x8B\x14a\x03%W\x80c\xFC)\x9D\xEE\x14a\x02\xFCWc\xFC\xE3l}\x14a\x01\x0CW_\x80\xFD[4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8Wa\x01<\x906\x90`\x04\x01a\x13&V[a\x01G\x92\x91\x92a\x1D~V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x82[\x81\x81\x10a\x02\x94WP\x81;\x15a\x02\x90W`@Q\x93c\xFC\xE3l}`\xE0\x1B\x85R\x81`$\x86\x01` `\x04\x88\x01RR`D\x85\x01`D\x83`\x05\x1B\x87\x01\x01\x92\x82\x86\x90`\x9E\x19\x816\x03\x01[\x83\x83\x10a\x01\xF7W\x88\x80\x8B\x81\x81\x80\x8C\x03\x81\x83\x8FZ\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x81a\x01\xE2\x91a\x12\xB4V[a\x01\xE9W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x8A\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x02\x8CW` `\x01\x92\x85\x82\x93\x01\x90`\x80c\xFF\xFF\xFF\xFFa\x02z\x82a\x02@a\x022\x87\x80a\x19fV[`\xA0\x88R`\xA0\x88\x01\x91a\x19\x9AV[\x95\x89\x80`\xA0\x1B\x03a\x02R\x89\x83\x01a\x11\xE4V[\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a\x02n``\x83\x01a\x13mV[\x16``\x87\x01R\x01a\x13mV[\x16\x91\x01R\x98\x01\x96\x01\x94\x93\x01\x91\x90a\x01\xB7V[\x89\x80\xFD[\x82\x80\xFD[\x80a\x02\xC9a\x02\xB0` a\x02\xAA`\x01\x95\x87\x8Ba\x1C^V[\x01a\x19RV[`@a\x02\xBD\x84\x87\x8Ba\x1C^V[\x015\x900\x903\x90a\x1D\xA1V[a\x02\xF2a\x02\xDC` a\x02\xAA\x84\x87\x8Ba\x1C^V[\x85`@a\x02\xEA\x85\x88\x8Ca\x1C^V[\x015\x91a\x1D\xECV[\x01a\x01tV[P\x80\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03?a\x11\xCEV[a\x03Ga\x1C\x80V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03bWa\x03_\x90a\x1D6V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9Wa\x03\xDEa\x03\xD2a\x19\xFEV[`@Q\x91\x82\x91\x82a\x12CV[\x03\x90\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x04Aa\x11\xCEV[\x90`$5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xF8W6`#\x84\x01\x12\x15a\x02\xF8W\x82`\x04\x015\x92a\x04n\x84a\x13VV[\x93a\x04|`@Q\x95\x86a\x12\xB4V[\x80\x85R`$` \x86\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x05\xE4W`$\x01\x90[\x82\x82\x10a\x05\xCCWPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xBDW`@Q\x90a\x04\xE0\x82a\x12\x85V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R0` \x83\x01\x90\x81R`@\x83\x01\x94\x85R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x93\x90\x91\x90\x84;\x15a\x05\xB9W`@Qcn4\x92\xB5`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x92Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R\x93Q\x90\x93\x16`D\x84\x01RQ```d\x84\x01R\x80Q`\x84\x84\x01\x81\x90R`\xA4\x84\x01\x92\x91\x90\x91\x01\x90\x84[\x81\x81\x10a\x05\x9DWPPP\x81\x83\x94\x81\x85\x81\x81\x95\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05zV[\x83\x80\xFD[cC\x94\xDB\xDF`\xE1\x1B\x82R`\x04\x82\xFD[` \x80\x91a\x05\xD9\x84a\x13mV[\x81R\x01\x91\x01\x90a\x04\x99V[\x84\x80\xFD[P4a\x01\xE9W\x80a\x05\xF86a\x11\xF8V[a\x06\x03\x92\x91\x92a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05\xE4W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x93\x90\x92\x16`D\x84\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01[\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x077W6`#\x82\x01\x12\x15a\x077Wa\x06\xCF\x906\x90`$\x81`\x04\x015\x91\x01a\x12\xF0V[a\x06\xD7a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x06{\x90`$\x83\x01\x90a\x18\xD7V[P[P\xFD[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x07Ta\x11\xCEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xBDW\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x075W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8Wa\x08!\x906\x90`\x04\x01a\x13&V[\x91\x90a\x08+a\x1D~V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x82[\x84\x81\x10a\n\x12WP\x80;\x15a\x02\x90W`@\x93\x91\x93Q\x91cN\\\xD2\xFD`\xE1\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01`d\x82`\x05\x1B\x85\x01\x01\x91\x86\x91\x86\x97`\xBE\x19\x816\x03\x01[\x83\x8A\x10a\x08\xC5W\x88\x80\x89\x81\x81\x80\x8B\x03\x81\x83\x8EZ\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x90\x91\x92\x93\x94`c\x19\x88\x82\x03\x01\x84R\x855\x82\x81\x12\x15a\x02\x8CW\x83\x01\x90a\x08\xFBa\x08\xED\x83\x80a\x19fV[`\xC0\x84R`\xC0\x84\x01\x91a\x19\x9AV[\x91`\x01`\x01`\xA0\x1B\x03a\t\x10` \x83\x01a\x11\xE4V[\x16` \x83\x01R` a\t%`@\x83\x01\x83a\x19fV[\x84\x86\x03`@\x86\x01R\x80\x86R\x94\x90\x91\x01\x93\x8C\x90[\x80\x82\x10a\t\xE2WPPPc\xFF\xFF\xFF\xFFa\tS``\x83\x01a\x13mV[\x16``\x83\x01Rc\xFF\xFF\xFF\xFFa\tj`\x80\x83\x01a\x13mV[\x16`\x80\x83\x01R`\xA0\x81\x015`\x1E\x19\x826\x03\x01\x81\x12\x15a\t\xDEW\x01\x90` \x825\x92\x01\x90`\x01`\x01`@\x1B\x03\x83\x11a\t\xDEW\x826\x03\x82\x13a\t\xDEW\x83\x83` \x94\x85\x94\x84`\xA0\x87\x96`\x01\x9A\x03\x91\x01R\x81\x84R\x84\x84\x017\x8D\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x97\x01\x94\x01\x99\x01\x98\x91\x90\x93\x92\x93a\x08\xA4V[\x8B\x80\xFD[\x90\x91\x94`@\x80`\x01\x92\x83\x80`\xA0\x1B\x03a\t\xFA\x8Aa\x11\xE4V[\x16\x81R` \x89\x015` \x82\x01R\x01\x96\x01\x92\x01\x90a\t8V[\x90\x92\x91\x93\x82\x94\x83\x95[a\n3a\n)\x85\x85\x85a\x18\xFBV[`@\x81\x01\x90a\x19\x1DV[\x90P\x87\x10\x15a\n\x83Wa\nJa\n)\x85\x85\x85a\x18\xFBV[\x88\x10\x15a\noW`\x01\x91` a\ng\x92\x8A`\x06\x1B\x01\x015\x90a\x13\xEEV[\x96\x01\x95a\n\x1BV[cNH{q`\xE0\x1B\x86R`2`\x04R`$\x86\xFD[\x91\x95P\x92\x93\x91`\x01\x91a\n\xC1\x90a\n\xAB\x8103a\n\xA6` a\x02\xAA\x89\x8F\x8Ea\x18\xFBV[a\x1D\xA1V[\x84a\n\xBC` a\x02\xAA\x86\x8C\x8Ba\x18\xFBV[a\x1D\xECV[\x01a\x08WV[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\n\xE2a\x11\xCEV[a\n\xEAa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x075W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\x0Boa\x11\xCEV[a\x0Bwa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a\x06{V[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x0B\xF5a\x11\xCEV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8W```\x03\x19\x826\x03\x01\x12a\x02\xF8W`@Q\x92a\x0C\"\x84a\x12\x85V[\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB9W\x82\x016`#\x82\x01\x12\x15a\x05\xB9Wa\x0CV\x906\x90`$`\x04\x82\x015\x91\x01a\x12\xF0V[\x84R` \x84\x01\x90`$\x83\x015\x82R`D`@\x86\x01\x93\x015\x83R`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\r8W\x92\x93\x84\x93\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r4W\x85a\r\x16\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x18\xD7V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x85\x80\xFD[cC\x94\xDB\xDF`\xE1\x1B\x84R`\x04\x84\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9Wa\r\x89a\x1C\x80V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x0E,a\x11\xCEV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x90W\x82T\x91`\xFF\x83`\x08\x1C\x16\x15\x92\x83\x80\x94a\x0F\xAAW[\x80\x15a\x0F\x93W[\x15a\x0F7W`\xFF\x19\x81\x16`\x01\x17\x85U\x83a\x0F&W[P`\xFF\x84T`\x08\x1C\x16\x15a\x0E\xCDWa\x0E\x8Aa\x0E\x8F\x92a\x1D6V[a\x1C\xD8V[a\x0E\x96W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\xFF\xFF\x19\x16a\x01\x01\x17\x84U_a\x0EpV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x0E[WP`\x01`\xFF\x82\x16\x14a\x0E[V[P`\x01`\xFF\x82\x16\x10a\x0ETV[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03_a\x0F\xD4a\x11\xCEV[a\x0E\x8Aa\x1C\x80V[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03\xDEa\x03\xD2a\x0F\xFCa\x11\xCEV[a\x14\x90V[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\x10\x1Ca\x11\xCEV[a\x10$a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a\x06{V[P4a\x01\xE9W\x80a\x10\x986a\x11\xF8V[a\x10\xA3\x92\x91\x92a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05\xE4W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x93\x90\x92\x16`D\x84\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01a\x06{V[P4a\x11\xCAW` 6`\x03\x19\x01\x12a\x11\xCAWa\x119a\x11\xCEV[a\x11Aa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x11\xCAW`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`$\x83\x01R_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x11\xBFWa\x11\xB1WP\x80\xF3[a\x11\xBD\x91P_\x90a\x12\xB4V[\0[`@Q=_\x82>=\x90\xFD[_\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAWV[``\x90`\x03\x19\x01\x12a\x11\xCAW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x11\xCAW\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12fWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12YV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x12\xA0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x12\xFC\x82a\x12\xD5V[\x91a\x13\n`@Q\x93\x84a\x12\xB4V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x11\xCAW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\x11\xCAW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x11\xCAW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x11\xCAWV[`\x01`\x01`@\x1B\x03\x81\x11a\x12\xA0W`\x05\x1B` \x01\x90V[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x11\xCAWV[\x90\x81` \x91\x03\x12a\x11\xCAWQ`\xFF\x81\x16\x81\x03a\x11\xCAW\x90V[\x90a\x13\xA1\x82a\x13VV[a\x13\xAE`@Q\x91\x82a\x12\xB4V[\x82\x81R\x80\x92a\x13\xBF`\x1F\x19\x91a\x13VV[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x13\xDAW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\x13\xFBWV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90\x81`@\x91\x03\x12a\x11\xCAW`@Q\x90`@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@R\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAW` \x91\x83R\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x11\xCAW` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x13\xDAW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x13\xFBW`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x18\xA3W[P`@Q\x91c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x18_W[P`\x01`\x01`\xC0\x1B\x03\x82\x16\x90\x81\x15\x90\x81\x15a\x17\xFCW[Pa\x17\xE2W\x80_\x91[a\x17\xBDWPa\xFF\xFF\x16a\x15C\x81a\x12\xD5V[\x90a\x15Q`@Q\x92\x83a\x12\xB4V[\x80\x82Ra\x15``\x1F\x19\x91a\x12\xD5V[\x016` \x83\x017__[\x82Q\x82\x10\x80a\x17\xB2W[\x15a\x15\xC2W`\x01\x81\x1B\x84\x16`\x01`\x01`\xC0\x1B\x03\x16a\x15\x9BW[a\x15\x96\x90a\x14\x82V[a\x15jV[\x90`\x01a\x15\x96\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa\x15\xB8\x82\x87a\x13\xC9V[S\x01\x91\x90Pa\x15\x8DV[P_\x93\x92PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x16}Wa\x16\x08\x85\x83a\x13\xC9V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x16KW[Pa\x16C\x90`\x01\x92a\x13\xEEV[\x94\x01\x93a\x15\xF5V[\x90P` \x81=\x82\x11a\x16uW[\x81a\x16e` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01a\x166V[=\x91Pa\x16XV[a\x16\x88\x91\x94Pa\x13\x97V[\x92_\x90_[\x81Q\x81\x10\x15a\x17\xACWa\x16\xA0\x81\x83a\x13\xC9V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R\x80`\x04\x83\x01R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x17yW[P\x90_\x91[\x81\x83\x10a\x16\xE6WPPP`\x01\x01a\x16\x8DV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x11\xBFW`\x01\x92a\x17B\x92_\x91a\x17KW[P\x83\x80`\xA0\x1B\x03\x90Q\x16a\x17<\x82\x8Da\x14nV[Ra\x14\x82V[\x95\x01\x91\x90a\x16\xD4V[a\x17l\x91P`@=\x81\x11a\x17rW[a\x17d\x81\x83a\x12\xB4V[\x81\x01\x90a\x14\x0FV[_a\x17(V[P=a\x17ZV[\x90\x91P` \x81=\x82\x11a\x17\xA4W[\x81a\x17\x94` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x16\xCFV[=\x91Pa\x17\x87V[PPPPV[Pa\x01\0\x81\x10a\x15tV[_\x19\x81\x01\x81\x81\x11a\x13\xFBWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x13\xFBW`\x01\x01\x90\x80a\x151V[PP`@Qa\x17\xF2` \x82a\x12\xB4V[_\x81R_6\x817\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x180W[P\x16\x15_a\x15(V[a\x18R\x91P` =` \x11a\x18XW[a\x18J\x81\x83a\x12\xB4V[\x81\x01\x90a\x13~V[_a\x18'V[P=a\x18@V[\x90\x91P` \x81=` \x11a\x18\x9BW[\x81a\x18{` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90_a\x15\x12V[=\x91Pa\x18nV[\x90\x91P` \x81=` \x11a\x18\xCFW[\x81a\x18\xBF` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x14\xE7V[=\x91Pa\x18\xB2V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x81\x10\x15a\x13\xDAW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x11\xCAW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x11\xCAWV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x11\xCAW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x11\xCAW\x81`\x06\x1B6\x03\x83\x13a\x11\xCAWV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\x19\xB4WPPP\x90V[\x90\x91\x92\x835`\x01\x80`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x11\xCAW\x81R` \x84\x015\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x11\xCAW`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a\x19\xA7V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x1C?W[P\x16\x80\x15a\x17\xE2W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a\x1B\xC9WPa\x1A\x99\x91Pa\x13\x97V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x1B\xABW[P\x16\x81\x10\x15a\x1B\xA4W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x1BqW[P\x90_\x91[\x81\x83\x10a\x1B\x13WPPP`\x01\x01a\x1A\x9EV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x11\xBFW`\x01\x92a\x1Bh\x92_\x91a\x17KWP\x83\x80`\xA0\x1B\x03\x90Q\x16a\x17<\x82\x8Da\x14nV[\x95\x01\x91\x90a\x1B\x01V[\x90\x91P` \x81=\x82\x11a\x1B\x9CW[\x81a\x1B\x8C` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x1A\xFCV[=\x91Pa\x1B\x7FV[P\x92PPPV[a\x1B\xC3\x91P` =\x81\x11a\x18XWa\x18J\x81\x83a\x12\xB4V[_a\x1A\xC4V[\x90`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R` \x81`$\x81\x88Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x1C\rW[Pa\x1C\x04\x90`\x01\x92a\x13\xEEV[\x92\x01\x91\x90a\x1A\x87V[\x90P` \x81=\x82\x11a\x1C7W[\x81a\x1C'` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01a\x1B\xF7V[=\x91Pa\x1C\x1AV[a\x1CX\x91P` =` \x11a\x18XWa\x18J\x81\x83a\x12\xB4V[_a\x1AOV[\x91\x90\x81\x10\x15a\x13\xDAW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1C\x94WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1D\x92WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x1D\xEA\x91a\x1D\xE5`\x84\x83a\x12\xB4V[a\x1E\xAAV[V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x1EvW[Pa\x1D\xEA\x93a\x1E@\x91a\x13\xEEV[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x84\x01R`D\x80\x84\x01\x91\x90\x91R\x82Ra\x1D\xE5`d\x83a\x12\xB4V[\x90P` \x81=` \x11a\x1E\xA2W[\x81a\x1E\x91` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQa\x1D\xEAa\x1E2V[=\x91Pa\x1E\x84V[\x90a\x1F)\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a\x1E\xC8`@\x86a\x12\xB4V[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a\x1F\xBAW=\x91a\x1F\r\x83a\x12\xD5V[\x92a\x1F\x1B`@Q\x94\x85a\x12\xB4V[\x83R=_` \x85\x01>a\x1F\xBEV[\x80Q\x90\x81\x15\x91\x82\x15a\x1F\x97W[PP\x15a\x1F?WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x11\xCAW` \x01Q\x80\x15\x15\x81\x03a\x11\xCAW_\x80a\x1F6V[``\x91[\x91\x92\x90\x15a  WP\x81Q\x15a\x1F\xD2WP\x90V[;\x15a\x1F\xDBW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a 3WP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a W\x90`$\x83\x01\x90a\x18\xD7V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 \xF6q\x80\xE6<]/\x95\0]\xE4\x88\xBBH\xE9k\x9A0@\xB2\x17s\xE7^\xB6\xE3zpP\x81\xDA\x87dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f5f3560e01c80631785f53c1461111f5780631fdb0cfd14611088578063279432eb1461100157806333cfb7b714610fdc5780633bc28c8c14610fb7578063485cc95514610e125780636b3aa72e14610dcd578063715018a614610d705780638da5cb5b14610d475780639926ee7d14610bdb5780639da16d8e14610b54578063a0169ddd14610ac7578063a20b99bf146107f0578063a364f4da1461073a578063a98fb3551461068c578063ba550880146105e8578063c1a8e2c514610427578063d4ad1d53146103e2578063e481af9d146103b6578063f2fde38b14610325578063fc299dee146102fc5763fce36c7d1461010c575f80fd5b346101e95760203660031901126101e9576004356001600160401b0381116102f85761013c903690600401611326565b610147929192611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690825b8181106102945750813b15610290576040519363fce36c7d60e01b8552816024860160206004880152526044850160448360051b87010192828690609e19813603015b8383106101f75788808b8181808c0381838f5af180156101ec576101d85750f35b816101e2916112b4565b6101e95780f35b80fd5b6040513d84823e3d90fd5b9091929394956043198a820301865286358281121561028c5760206001928582930190608063ffffffff61027a826102406102328780611966565b60a0885260a088019161199a565b95898060a01b036102528983016111e4565b1688870152604081013560408701528361026e6060830161136d565b1660608701520161136d565b169101529801960194930191906101b7565b8980fd5b8280fd5b806102c96102b060206102aa600195878b611c5e565b01611952565b60406102bd84878b611c5e565b01359030903390611da1565b6102f26102dc60206102aa84878b611c5e565b8560406102ea85888c611c5e565b013591611dec565b01610174565b5080fd5b50346101e957806003193601126101e9576065546040516001600160a01b039091168152602090f35b50346101e95760203660031901126101e95761033f6111ce565b610347611c80565b6001600160a01b038116156103625761035f90611d36565b80f35b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b50346101e957806003193601126101e9576103de6103d26119fe565b60405191829182611243565b0390f35b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760403660031901126101e9576104416111ce565b90602435916001600160401b0383116102f857366023840112156102f85782600401359261046e84611356565b9361047c60405195866112b4565b8085526024602086019160051b830101913683116105e457602401905b8282106105cc575050507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036105bd57604051906104e082611285565b6001600160a01b0390811682523060208301908152604083019485527f000000000000000000000000000000000000000000000000000000000000000090911693909190843b156105b957604051636e3492b560e01b815260206004820181905292516001600160a01b03908116602483015293519093166044840152516060606484015280516084840181905260a48401929190910190845b81811061059d57505050818394818581819503925af180156101ec576101d85750f35b825163ffffffff1684526020938401939092019160010161057a565b8380fd5b634394dbdf60e11b8252600482fd5b602080916105d98461136d565b815201910190610499565b8480fd5b50346101e957806105f8366111f8565b610603929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156105e457604051630664120160e01b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b031916606483015282908290818381608481015b03925af180156101ec576101d85750f35b50346101e95760203660031901126101e957806004356001600160401b0381116107375736602382011215610737576106cf9036906024816004013591016112f0565b6106d7611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107355760405163a98fb35560e01b815260206004820152918391839182908490829061067b9060248301906118d7565b505b50fd5b50346101e95760203660031901126101e9576107546111ce565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031633036105bd5781907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610735576040516351b27a6d60e11b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e9576004356001600160401b0381116102f857610821903690600401611326565b919061082b611d7e565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316825b848110610a125750803b156102905760409391935191634e5cd2fd60e11b8352806044840130600486015260406024860152526064830160648260051b850101918691869760be19813603015b838a106108c5578880898181808b0381838e5af180156101ec576101d85750f35b909192939460631988820301845285358281121561028c578301906108fb6108ed8380611966565b60c0845260c084019161199a565b916001600160a01b03610910602083016111e4565b16602083015260206109256040830183611966565b848603604086015280865294909101938c905b8082106109e25750505063ffffffff6109536060830161136d565b16606083015263ffffffff61096a6080830161136d565b16608083015260a0810135601e19823603018112156109de570190602082359201906001600160401b0383116109de5782360382136109de57838360209485948460a0879660019a03910152818452848401378d838284010152601f801991011601019701940199019891909392936108a4565b8b80fd5b909194604080600192838060a01b036109fa8a6111e4565b16815260208901356020820152019601920190610938565b90929193829483955b610a33610a298585856118fb565b604081019061191d565b9050871015610a8357610a4a610a298585856118fb565b881015610a6f576001916020610a67928a60061b010135906113ee565b960195610a1b565b634e487b7160e01b86526032600452602486fd5b919550929391600191610ac190610aab813033610aa660206102aa898f8e6118fb565b611da1565b84610abc60206102aa868c8b6118fb565b611dec565b01610857565b50346101e95760203660031901126101e95780610ae26111ce565b610aea611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b156107355760405163a0169ddd60e01b81526001600160a01b0390911660048201529082908290602490829084905af180156101ec576101d85750f35b50346101e95760203660031901126101e95780610b6f6111ce565b610b77611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b1561073557604051634f906cf960e01b81523060048201526001600160a01b039092166024830152829082908183816044810161067b565b50346101e95760403660031901126101e957610bf56111ce565b906024356001600160401b0381116102f857606060031982360301126102f85760405192610c2284611285565b81600401356001600160401b0381116105b9578201366023820112156105b957610c569036906024600482013591016112f0565b8452602084019060248301358252604460408601930135835260018060a01b037f0000000000000000000000000000000000000000000000000000000000000000163303610d385792938493907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690813b15610d345785610d1693819560405197889687958694639926ee7d60e01b865260018060a01b0316600486015260406024860152516060604486015260a48501906118d7565b9151606484015251608483015203925af180156101ec576101d85750f35b8580fd5b634394dbdf60e11b8452600484fd5b50346101e957806003193601126101e9576033546040516001600160a01b039091168152602090f35b50346101e957806003193601126101e957610d89611c80565b603380546001600160a01b0319811690915581906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a380f35b50346101e957806003193601126101e9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b50346101e95760403660031901126101e957610e2c6111ce565b602435906001600160a01b03821682036102905782549160ff8360081c161592838094610faa575b8015610f93575b15610f375760ff198116600117855583610f26575b5060ff845460081c1615610ecd57610e8a610e8f92611d36565b611cd8565b610e965780f35b61ff001981541681557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a180f35b60405162461bcd60e51b815260206004820152602b60248201527f496e697469616c697a61626c653a20636f6e7472616374206973206e6f74206960448201526a6e697469616c697a696e6760a81b6064820152608490fd5b61ffff19166101011784555f610e70565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b158015610e5b5750600160ff821614610e5b565b50600160ff821610610e54565b50346101e95760203660031901126101e95761035f610fd46111ce565b610e8a611c80565b50346101e95760203660031901126101e9576103de6103d2610ffc6111ce565b611490565b50346101e95760203660031901126101e9578061101c6111ce565b611024611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156107355760405163eb5a4e8760e01b81523060048201526001600160a01b039092166024830152829082908183816044810161067b565b50346101e95780611098366111f8565b6110a3929192611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156105e457604051634a86c03760e11b81523060048201526001600160a01b0393841660248201529390921660448401526001600160e01b0319166064830152829082908183816084810161067b565b50346111ca5760203660031901126111ca576111396111ce565b611141611c80565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316803b156111ca5760405163268959e560e01b81523060048201526001600160a01b039290921660248301525f908290604490829084905af180156111bf576111b1575080f35b6111bd91505f906112b4565b005b6040513d5f823e3d90fd5b5f80fd5b600435906001600160a01b03821682036111ca57565b35906001600160a01b03821682036111ca57565b60609060031901126111ca576004356001600160a01b03811681036111ca57906024356001600160a01b03811681036111ca57906044356001600160e01b0319811681036111ca5790565b60206040818301928281528451809452019201905f5b8181106112665750505090565b82516001600160a01b0316845260209384019390920191600101611259565b606081019081106001600160401b038211176112a057604052565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b038211176112a057604052565b6001600160401b0381116112a057601f01601f191660200190565b9291926112fc826112d5565b9161130a60405193846112b4565b8294818452818301116111ca578281602093845f960137010152565b9181601f840112156111ca578235916001600160401b0383116111ca576020808501948460051b0101116111ca57565b6001600160401b0381116112a05760051b60200190565b359063ffffffff821682036111ca57565b908160209103126111ca575160ff811681036111ca5790565b906113a182611356565b6113ae60405191826112b4565b82815280926113bf601f1991611356565b0190602036910137565b9081518110156113da570160200190565b634e487b7160e01b5f52603260045260245ffd5b919082018092116113fb57565b634e487b7160e01b5f52601160045260245ffd5b908160409103126111ca5760405190604082018281106001600160401b038211176112a0576040528051906001600160a01b03821682036111ca57602091835201516bffffffffffffffffffffffff811681036111ca57602082015290565b80518210156113da5760209160051b010190565b5f1981146113fb5760010190565b6040516309aa152760e11b81526001600160a01b039182166004820152907f000000000000000000000000000000000000000000000000000000000000000016602082602481845afa9182156111bf575f926118a3575b506040519163871ef04960e01b83526004830152602082602481845afa9182156111bf575f9261185f575b506001600160c01b0382169081159081156117fc575b506117e257805f915b6117bd575061ffff16611543816112d5565b9061155160405192836112b4565b808252611560601f19916112d5565b013660208301375f5f5b82518210806117b2575b156115c2576001811b84166001600160c01b031661159b575b61159690611482565b61156a565b9060016115969160ff60f81b8460f81b165f1a6115b882876113c9565b530191905061158d565b505f939250507f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690835b815185101561167d5761160885836113c9565b5160f81c60405190633ca5a5f560e01b82526004820152602081602481875afa9081156111bf575f9161164b575b50611643906001926113ee565b9401936115f5565b90506020813d8211611675575b81611665602093836112b4565b810103126111ca57516001611636565b3d9150611658565b611688919450611397565b925f905f5b81518110156117ac576116a081836113c9565b5160f81c60405190633ca5a5f560e01b8252806004830152602082602481895afa9182156111bf575f92611779575b50905f915b8183106116e65750505060010161168d565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611742925f9161174b575b50838060a01b0390511661173c828d61146e565b52611482565b950191906116d4565b61176c915060403d8111611772575b61176481836112b4565b81019061140f565b5f611728565b503d61175a565b9091506020813d82116117a4575b81611794602093836112b4565b810103126111ca5751905f6116cf565b3d9150611787565b50505050565b506101008110611574565b5f1981018181116113fb5761ffff9116911661ffff81146113fb576001019080611531565b50506040516117f26020826112b4565b5f81525f36813790565b604051639aa1653d60e01b81529150602090829060049082905afa80156111bf5760ff915f91611830575b5016155f611528565b611852915060203d602011611858575b61184a81836112b4565b81019061137e565b5f611827565b503d611840565b9091506020813d60201161189b575b8161187b602093836112b4565b810103126111ca57516001600160c01b03811681036111ca57905f611512565b3d915061186e565b9091506020813d6020116118cf575b816118bf602093836112b4565b810103126111ca5751905f6114e7565b3d91506118b2565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91908110156113da5760051b8101359060be19813603018212156111ca570190565b903590601e19813603018212156111ca57018035906001600160401b0382116111ca57602001918160061b360383136111ca57565b356001600160a01b03811681036111ca5790565b9035601e19823603018112156111ca5701602081359101916001600160401b0382116111ca578160061b360383136111ca57565b916020908281520191905f905b8082106119b45750505090565b909192833560018060a01b0381168091036111ca5781526020840135906bffffffffffffffffffffffff82168092036111ca576040816001936020839401520194019201906119a7565b604051639aa1653d60e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690602081600481855afa80156111bf5760ff915f91611c3f575b501680156117e2577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316905f9081905b808310611bc95750611a999150611397565b925f905f5b604051639aa1653d60e01b8152602081600481895afa80156111bf5760ff915f91611bab575b5016811015611ba457604051633ca5a5f560e01b815260ff821660048201819052602082602481895afa9182156111bf575f92611b71575b50905f915b818310611b1357505050600101611a9e565b604080516356e4026d60e11b815260ff83166004820152602481018590529396929391929190816044818b5afa9182156111bf57600192611b68925f9161174b5750838060a01b0390511661173c828d61146e565b95019190611b01565b9091506020813d8211611b9c575b81611b8c602093836112b4565b810103126111ca5751905f611afc565b3d9150611b7f565b5092505050565b611bc3915060203d81116118585761184a81836112b4565b5f611ac4565b90604051633ca5a5f560e01b815260ff84166004820152602081602481885afa9081156111bf575f91611c0d575b50611c04906001926113ee565b92019190611a87565b90506020813d8211611c37575b81611c27602093836112b4565b810103126111ca57516001611bf7565b3d9150611c1a565b611c58915060203d6020116118585761184a81836112b4565b5f611a4f565b91908110156113da5760051b81013590609e19813603018212156111ca570190565b6033546001600160a01b03163303611c9457565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b606554604080516001600160a01b038084168252841660208201529192917fe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e39190a16001600160a01b03166001600160a01b03199190911617606555565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b6065546001600160a01b03163303611d9257565b638e79fdb560e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152611dea91611de56084836112b4565b611eaa565b565b604051636eb1769f60e11b81523060048201526001600160a01b0383166024820152602081806044810103816001600160a01b0386165afa9081156111bf575f91611e76575b50611dea93611e40916113ee565b60405163095ea7b360e01b60208201526001600160a01b0390931660248401526044808401919091528252611de56064836112b4565b90506020813d602011611ea2575b81611e91602093836112b4565b810103126111ca5751611dea611e32565b3d9150611e84565b90611f299160018060a01b03165f8060405193611ec86040866112b4565b602085527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564602086015260208151910182855af13d15611fba573d91611f0d836112d5565b92611f1b60405194856112b4565b83523d5f602085013e611fbe565b8051908115918215611f97575b505015611f3f57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b81925090602091810103126111ca576020015180151581036111ca575f80611f36565b6060915b919290156120205750815115611fd2575090565b3b15611fdb5790565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b8251909150156120335750805190602001fd5b60405162461bcd60e51b8152602060048201529081906120579060248301906118d7565b0390fdfea2646970667358221220f67180e63c5d2f95005de488bb48e96b9a3040b21773e75eb6e37a705081da8764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[__5`\xE0\x1C\x80c\x17\x85\xF5<\x14a\x11\x1FW\x80c\x1F\xDB\x0C\xFD\x14a\x10\x88W\x80c'\x942\xEB\x14a\x10\x01W\x80c3\xCF\xB7\xB7\x14a\x0F\xDCW\x80c;\xC2\x8C\x8C\x14a\x0F\xB7W\x80cH\\\xC9U\x14a\x0E\x12W\x80ck:\xA7.\x14a\r\xCDW\x80cqP\x18\xA6\x14a\rpW\x80c\x8D\xA5\xCB[\x14a\rGW\x80c\x99&\xEE}\x14a\x0B\xDBW\x80c\x9D\xA1m\x8E\x14a\x0BTW\x80c\xA0\x16\x9D\xDD\x14a\n\xC7W\x80c\xA2\x0B\x99\xBF\x14a\x07\xF0W\x80c\xA3d\xF4\xDA\x14a\x07:W\x80c\xA9\x8F\xB3U\x14a\x06\x8CW\x80c\xBAU\x08\x80\x14a\x05\xE8W\x80c\xC1\xA8\xE2\xC5\x14a\x04'W\x80c\xD4\xAD\x1DS\x14a\x03\xE2W\x80c\xE4\x81\xAF\x9D\x14a\x03\xB6W\x80c\xF2\xFD\xE3\x8B\x14a\x03%W\x80c\xFC)\x9D\xEE\x14a\x02\xFCWc\xFC\xE3l}\x14a\x01\x0CW_\x80\xFD[4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8Wa\x01<\x906\x90`\x04\x01a\x13&V[a\x01G\x92\x91\x92a\x1D~V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x82[\x81\x81\x10a\x02\x94WP\x81;\x15a\x02\x90W`@Q\x93c\xFC\xE3l}`\xE0\x1B\x85R\x81`$\x86\x01` `\x04\x88\x01RR`D\x85\x01`D\x83`\x05\x1B\x87\x01\x01\x92\x82\x86\x90`\x9E\x19\x816\x03\x01[\x83\x83\x10a\x01\xF7W\x88\x80\x8B\x81\x81\x80\x8C\x03\x81\x83\x8FZ\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x81a\x01\xE2\x91a\x12\xB4V[a\x01\xE9W\x80\xF3[\x80\xFD[`@Q=\x84\x82>=\x90\xFD[\x90\x91\x92\x93\x94\x95`C\x19\x8A\x82\x03\x01\x86R\x865\x82\x81\x12\x15a\x02\x8CW` `\x01\x92\x85\x82\x93\x01\x90`\x80c\xFF\xFF\xFF\xFFa\x02z\x82a\x02@a\x022\x87\x80a\x19fV[`\xA0\x88R`\xA0\x88\x01\x91a\x19\x9AV[\x95\x89\x80`\xA0\x1B\x03a\x02R\x89\x83\x01a\x11\xE4V[\x16\x88\x87\x01R`@\x81\x015`@\x87\x01R\x83a\x02n``\x83\x01a\x13mV[\x16``\x87\x01R\x01a\x13mV[\x16\x91\x01R\x98\x01\x96\x01\x94\x93\x01\x91\x90a\x01\xB7V[\x89\x80\xFD[\x82\x80\xFD[\x80a\x02\xC9a\x02\xB0` a\x02\xAA`\x01\x95\x87\x8Ba\x1C^V[\x01a\x19RV[`@a\x02\xBD\x84\x87\x8Ba\x1C^V[\x015\x900\x903\x90a\x1D\xA1V[a\x02\xF2a\x02\xDC` a\x02\xAA\x84\x87\x8Ba\x1C^V[\x85`@a\x02\xEA\x85\x88\x8Ca\x1C^V[\x015\x91a\x1D\xECV[\x01a\x01tV[P\x80\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03?a\x11\xCEV[a\x03Ga\x1C\x80V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03bWa\x03_\x90a\x1D6V[\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9Wa\x03\xDEa\x03\xD2a\x19\xFEV[`@Q\x91\x82\x91\x82a\x12CV[\x03\x90\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x04Aa\x11\xCEV[\x90`$5\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\xF8W6`#\x84\x01\x12\x15a\x02\xF8W\x82`\x04\x015\x92a\x04n\x84a\x13VV[\x93a\x04|`@Q\x95\x86a\x12\xB4V[\x80\x85R`$` \x86\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x05\xE4W`$\x01\x90[\x82\x82\x10a\x05\xCCWPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xBDW`@Q\x90a\x04\xE0\x82a\x12\x85V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R0` \x83\x01\x90\x81R`@\x83\x01\x94\x85R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x93\x90\x91\x90\x84;\x15a\x05\xB9W`@Qcn4\x92\xB5`\xE0\x1B\x81R` `\x04\x82\x01\x81\x90R\x92Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`$\x83\x01R\x93Q\x90\x93\x16`D\x84\x01RQ```d\x84\x01R\x80Q`\x84\x84\x01\x81\x90R`\xA4\x84\x01\x92\x91\x90\x91\x01\x90\x84[\x81\x81\x10a\x05\x9DWPPP\x81\x83\x94\x81\x85\x81\x81\x95\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x05zV[\x83\x80\xFD[cC\x94\xDB\xDF`\xE1\x1B\x82R`\x04\x82\xFD[` \x80\x91a\x05\xD9\x84a\x13mV[\x81R\x01\x91\x01\x90a\x04\x99V[\x84\x80\xFD[P4a\x01\xE9W\x80a\x05\xF86a\x11\xF8V[a\x06\x03\x92\x91\x92a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05\xE4W`@Qc\x06d\x12\x01`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x93\x90\x92\x16`D\x84\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01[\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x077W6`#\x82\x01\x12\x15a\x077Wa\x06\xCF\x906\x90`$\x81`\x04\x015\x91\x01a\x12\xF0V[a\x06\xD7a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@Qc\xA9\x8F\xB3U`\xE0\x1B\x81R` `\x04\x82\x01R\x91\x83\x91\x83\x91\x82\x90\x84\x90\x82\x90a\x06{\x90`$\x83\x01\x90a\x18\xD7V[P[P\xFD[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x07Ta\x11\xCEV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x05\xBDW\x81\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x075W`@QcQ\xB2zm`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8Wa\x08!\x906\x90`\x04\x01a\x13&V[\x91\x90a\x08+a\x1D~V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x82[\x84\x81\x10a\n\x12WP\x80;\x15a\x02\x90W`@\x93\x91\x93Q\x91cN\\\xD2\xFD`\xE1\x1B\x83R\x80`D\x84\x010`\x04\x86\x01R`@`$\x86\x01RR`d\x83\x01`d\x82`\x05\x1B\x85\x01\x01\x91\x86\x91\x86\x97`\xBE\x19\x816\x03\x01[\x83\x8A\x10a\x08\xC5W\x88\x80\x89\x81\x81\x80\x8B\x03\x81\x83\x8EZ\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x90\x91\x92\x93\x94`c\x19\x88\x82\x03\x01\x84R\x855\x82\x81\x12\x15a\x02\x8CW\x83\x01\x90a\x08\xFBa\x08\xED\x83\x80a\x19fV[`\xC0\x84R`\xC0\x84\x01\x91a\x19\x9AV[\x91`\x01`\x01`\xA0\x1B\x03a\t\x10` \x83\x01a\x11\xE4V[\x16` \x83\x01R` a\t%`@\x83\x01\x83a\x19fV[\x84\x86\x03`@\x86\x01R\x80\x86R\x94\x90\x91\x01\x93\x8C\x90[\x80\x82\x10a\t\xE2WPPPc\xFF\xFF\xFF\xFFa\tS``\x83\x01a\x13mV[\x16``\x83\x01Rc\xFF\xFF\xFF\xFFa\tj`\x80\x83\x01a\x13mV[\x16`\x80\x83\x01R`\xA0\x81\x015`\x1E\x19\x826\x03\x01\x81\x12\x15a\t\xDEW\x01\x90` \x825\x92\x01\x90`\x01`\x01`@\x1B\x03\x83\x11a\t\xDEW\x826\x03\x82\x13a\t\xDEW\x83\x83` \x94\x85\x94\x84`\xA0\x87\x96`\x01\x9A\x03\x91\x01R\x81\x84R\x84\x84\x017\x8D\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x97\x01\x94\x01\x99\x01\x98\x91\x90\x93\x92\x93a\x08\xA4V[\x8B\x80\xFD[\x90\x91\x94`@\x80`\x01\x92\x83\x80`\xA0\x1B\x03a\t\xFA\x8Aa\x11\xE4V[\x16\x81R` \x89\x015` \x82\x01R\x01\x96\x01\x92\x01\x90a\t8V[\x90\x92\x91\x93\x82\x94\x83\x95[a\n3a\n)\x85\x85\x85a\x18\xFBV[`@\x81\x01\x90a\x19\x1DV[\x90P\x87\x10\x15a\n\x83Wa\nJa\n)\x85\x85\x85a\x18\xFBV[\x88\x10\x15a\noW`\x01\x91` a\ng\x92\x8A`\x06\x1B\x01\x015\x90a\x13\xEEV[\x96\x01\x95a\n\x1BV[cNH{q`\xE0\x1B\x86R`2`\x04R`$\x86\xFD[\x91\x95P\x92\x93\x91`\x01\x91a\n\xC1\x90a\n\xAB\x8103a\n\xA6` a\x02\xAA\x89\x8F\x8Ea\x18\xFBV[a\x1D\xA1V[\x84a\n\xBC` a\x02\xAA\x86\x8C\x8Ba\x18\xFBV[a\x1D\xECV[\x01a\x08WV[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\n\xE2a\x11\xCEV[a\n\xEAa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x075W`@Qc\xA0\x16\x9D\xDD`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x90\x82\x90\x82\x90`$\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\x0Boa\x11\xCEV[a\x0Bwa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@QcO\x90l\xF9`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a\x06{V[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x0B\xF5a\x11\xCEV[\x90`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\xF8W```\x03\x19\x826\x03\x01\x12a\x02\xF8W`@Q\x92a\x0C\"\x84a\x12\x85V[\x81`\x04\x015`\x01`\x01`@\x1B\x03\x81\x11a\x05\xB9W\x82\x016`#\x82\x01\x12\x15a\x05\xB9Wa\x0CV\x906\x90`$`\x04\x82\x015\x91\x01a\x12\xF0V[\x84R` \x84\x01\x90`$\x83\x015\x82R`D`@\x86\x01\x93\x015\x83R`\x01\x80`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x163\x03a\r8W\x92\x93\x84\x93\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\r4W\x85a\r\x16\x93\x81\x95`@Q\x97\x88\x96\x87\x95\x86\x94c\x99&\xEE}`\xE0\x1B\x86R`\x01\x80`\xA0\x1B\x03\x16`\x04\x86\x01R`@`$\x86\x01RQ```D\x86\x01R`\xA4\x85\x01\x90a\x18\xD7V[\x91Q`d\x84\x01RQ`\x84\x83\x01R\x03\x92Z\xF1\x80\x15a\x01\xECWa\x01\xD8WP\xF3[\x85\x80\xFD[cC\x94\xDB\xDF`\xE1\x1B\x84R`\x04\x84\xFD[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9Wa\r\x89a\x1C\x80V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U\x81\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\x80\xF3[P4a\x01\xE9W\x80`\x03\x196\x01\x12a\x01\xE9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[P4a\x01\xE9W`@6`\x03\x19\x01\x12a\x01\xE9Wa\x0E,a\x11\xCEV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x90W\x82T\x91`\xFF\x83`\x08\x1C\x16\x15\x92\x83\x80\x94a\x0F\xAAW[\x80\x15a\x0F\x93W[\x15a\x0F7W`\xFF\x19\x81\x16`\x01\x17\x85U\x83a\x0F&W[P`\xFF\x84T`\x08\x1C\x16\x15a\x0E\xCDWa\x0E\x8Aa\x0E\x8F\x92a\x1D6V[a\x1C\xD8V[a\x0E\x96W\x80\xF3[a\xFF\0\x19\x81T\x16\x81U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7FInitializable: contract is not i`D\x82\x01Rjnitializing`\xA8\x1B`d\x82\x01R`\x84\x90\xFD[a\xFF\xFF\x19\x16a\x01\x01\x17\x84U_a\x0EpV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x0E[WP`\x01`\xFF\x82\x16\x14a\x0E[V[P`\x01`\xFF\x82\x16\x10a\x0ETV[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03_a\x0F\xD4a\x11\xCEV[a\x0E\x8Aa\x1C\x80V[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9Wa\x03\xDEa\x03\xD2a\x0F\xFCa\x11\xCEV[a\x14\x90V[P4a\x01\xE9W` 6`\x03\x19\x01\x12a\x01\xE9W\x80a\x10\x1Ca\x11\xCEV[a\x10$a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x075W`@Qc\xEBZN\x87`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R\x82\x90\x82\x90\x81\x83\x81`D\x81\x01a\x06{V[P4a\x01\xE9W\x80a\x10\x986a\x11\xF8V[a\x10\xA3\x92\x91\x92a\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x05\xE4W`@QcJ\x86\xC07`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`$\x82\x01R\x93\x90\x92\x16`D\x84\x01R`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x82\x90\x82\x90\x81\x83\x81`\x84\x81\x01a\x06{V[P4a\x11\xCAW` 6`\x03\x19\x01\x12a\x11\xCAWa\x119a\x11\xCEV[a\x11Aa\x1C\x80V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x11\xCAW`@Qc&\x89Y\xE5`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`$\x83\x01R_\x90\x82\x90`D\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x11\xBFWa\x11\xB1WP\x80\xF3[a\x11\xBD\x91P_\x90a\x12\xB4V[\0[`@Q=_\x82>=\x90\xFD[_\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAWV[5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAWV[``\x90`\x03\x19\x01\x12a\x11\xCAW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90`D5`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x11\xCAW\x90V[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x12fWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x12YV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x12\xA0W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x12\xFC\x82a\x12\xD5V[\x91a\x13\n`@Q\x93\x84a\x12\xB4V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x11\xCAW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\x11\xCAW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x11\xCAW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x11\xCAWV[`\x01`\x01`@\x1B\x03\x81\x11a\x12\xA0W`\x05\x1B` \x01\x90V[5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x11\xCAWV[\x90\x81` \x91\x03\x12a\x11\xCAWQ`\xFF\x81\x16\x81\x03a\x11\xCAW\x90V[\x90a\x13\xA1\x82a\x13VV[a\x13\xAE`@Q\x91\x82a\x12\xB4V[\x82\x81R\x80\x92a\x13\xBF`\x1F\x19\x91a\x13VV[\x01\x90` 6\x91\x017V[\x90\x81Q\x81\x10\x15a\x13\xDAW\x01` \x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x82\x01\x80\x92\x11a\x13\xFBWV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90\x81`@\x91\x03\x12a\x11\xCAW`@Q\x90`@\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x12\xA0W`@R\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x11\xCAW` \x91\x83R\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x11\xCAW` \x82\x01R\x90V[\x80Q\x82\x10\x15a\x13\xDAW` \x91`\x05\x1B\x01\x01\x90V[_\x19\x81\x14a\x13\xFBW`\x01\x01\x90V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x18\xA3W[P`@Q\x91c\x87\x1E\xF0I`\xE0\x1B\x83R`\x04\x83\x01R` \x82`$\x81\x84Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x18_W[P`\x01`\x01`\xC0\x1B\x03\x82\x16\x90\x81\x15\x90\x81\x15a\x17\xFCW[Pa\x17\xE2W\x80_\x91[a\x17\xBDWPa\xFF\xFF\x16a\x15C\x81a\x12\xD5V[\x90a\x15Q`@Q\x92\x83a\x12\xB4V[\x80\x82Ra\x15``\x1F\x19\x91a\x12\xD5V[\x016` \x83\x017__[\x82Q\x82\x10\x80a\x17\xB2W[\x15a\x15\xC2W`\x01\x81\x1B\x84\x16`\x01`\x01`\xC0\x1B\x03\x16a\x15\x9BW[a\x15\x96\x90a\x14\x82V[a\x15jV[\x90`\x01a\x15\x96\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1Aa\x15\xB8\x82\x87a\x13\xC9V[S\x01\x91\x90Pa\x15\x8DV[P_\x93\x92PP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90\x83[\x81Q\x85\x10\x15a\x16}Wa\x16\x08\x85\x83a\x13\xC9V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R`\x04\x82\x01R` \x81`$\x81\x87Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x16KW[Pa\x16C\x90`\x01\x92a\x13\xEEV[\x94\x01\x93a\x15\xF5V[\x90P` \x81=\x82\x11a\x16uW[\x81a\x16e` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01a\x166V[=\x91Pa\x16XV[a\x16\x88\x91\x94Pa\x13\x97V[\x92_\x90_[\x81Q\x81\x10\x15a\x17\xACWa\x16\xA0\x81\x83a\x13\xC9V[Q`\xF8\x1C`@Q\x90c<\xA5\xA5\xF5`\xE0\x1B\x82R\x80`\x04\x83\x01R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x17yW[P\x90_\x91[\x81\x83\x10a\x16\xE6WPPP`\x01\x01a\x16\x8DV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x11\xBFW`\x01\x92a\x17B\x92_\x91a\x17KW[P\x83\x80`\xA0\x1B\x03\x90Q\x16a\x17<\x82\x8Da\x14nV[Ra\x14\x82V[\x95\x01\x91\x90a\x16\xD4V[a\x17l\x91P`@=\x81\x11a\x17rW[a\x17d\x81\x83a\x12\xB4V[\x81\x01\x90a\x14\x0FV[_a\x17(V[P=a\x17ZV[\x90\x91P` \x81=\x82\x11a\x17\xA4W[\x81a\x17\x94` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x16\xCFV[=\x91Pa\x17\x87V[PPPPV[Pa\x01\0\x81\x10a\x15tV[_\x19\x81\x01\x81\x81\x11a\x13\xFBWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a\x13\xFBW`\x01\x01\x90\x80a\x151V[PP`@Qa\x17\xF2` \x82a\x12\xB4V[_\x81R_6\x817\x90V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x91P` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x180W[P\x16\x15_a\x15(V[a\x18R\x91P` =` \x11a\x18XW[a\x18J\x81\x83a\x12\xB4V[\x81\x01\x90a\x13~V[_a\x18'V[P=a\x18@V[\x90\x91P` \x81=` \x11a\x18\x9BW[\x81a\x18{` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90_a\x15\x12V[=\x91Pa\x18nV[\x90\x91P` \x81=` \x11a\x18\xCFW[\x81a\x18\xBF` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x14\xE7V[=\x91Pa\x18\xB2V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90\x81\x10\x15a\x13\xDAW`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x11\xCAW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x11\xCAWV[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x11\xCAW\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x11\xCAW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x11\xCAW\x81`\x06\x1B6\x03\x83\x13a\x11\xCAWV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\x19\xB4WPPP\x90V[\x90\x91\x92\x835`\x01\x80`\xA0\x1B\x03\x81\x16\x80\x91\x03a\x11\xCAW\x81R` \x84\x015\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x80\x92\x03a\x11\xCAW`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a\x19\xA7V[`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90` \x81`\x04\x81\x85Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x1C?W[P\x16\x80\x15a\x17\xE2W\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90_\x90\x81\x90[\x80\x83\x10a\x1B\xC9WPa\x1A\x99\x91Pa\x13\x97V[\x92_\x90_[`@Qc\x9A\xA1e=`\xE0\x1B\x81R` \x81`\x04\x81\x89Z\xFA\x80\x15a\x11\xBFW`\xFF\x91_\x91a\x1B\xABW[P\x16\x81\x10\x15a\x1B\xA4W`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01\x81\x90R` \x82`$\x81\x89Z\xFA\x91\x82\x15a\x11\xBFW_\x92a\x1BqW[P\x90_\x91[\x81\x83\x10a\x1B\x13WPPP`\x01\x01a\x1A\x9EV[`@\x80QcV\xE4\x02m`\xE1\x1B\x81R`\xFF\x83\x16`\x04\x82\x01R`$\x81\x01\x85\x90R\x93\x96\x92\x93\x91\x92\x91\x90\x81`D\x81\x8BZ\xFA\x91\x82\x15a\x11\xBFW`\x01\x92a\x1Bh\x92_\x91a\x17KWP\x83\x80`\xA0\x1B\x03\x90Q\x16a\x17<\x82\x8Da\x14nV[\x95\x01\x91\x90a\x1B\x01V[\x90\x91P` \x81=\x82\x11a\x1B\x9CW[\x81a\x1B\x8C` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ\x90_a\x1A\xFCV[=\x91Pa\x1B\x7FV[P\x92PPPV[a\x1B\xC3\x91P` =\x81\x11a\x18XWa\x18J\x81\x83a\x12\xB4V[_a\x1A\xC4V[\x90`@Qc<\xA5\xA5\xF5`\xE0\x1B\x81R`\xFF\x84\x16`\x04\x82\x01R` \x81`$\x81\x88Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x1C\rW[Pa\x1C\x04\x90`\x01\x92a\x13\xEEV[\x92\x01\x91\x90a\x1A\x87V[\x90P` \x81=\x82\x11a\x1C7W[\x81a\x1C'` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQ`\x01a\x1B\xF7V[=\x91Pa\x1C\x1AV[a\x1CX\x91P` =` \x11a\x18XWa\x18J\x81\x83a\x12\xB4V[_a\x1AOV[\x91\x90\x81\x10\x15a\x13\xDAW`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x11\xCAW\x01\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1C\x94WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x84\x16` \x82\x01R\x91\x92\x91\x7F\xE1\x1C\xDD\xF1\x81jC1\x8C\xA1u\xBB\xC5,\xD0\x18T6\xE9\xCB\xEA\xD7\xC8:\xCCT\xA7>F\x17\x17\xE3\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x90\x91\x16\x17`eUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`eT`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1D\x92WV[c\x8Ey\xFD\xB5`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x1D\xEA\x91a\x1D\xE5`\x84\x83a\x12\xB4V[a\x1E\xAAV[V[`@Qcn\xB1v\x9F`\xE1\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R` \x81\x80`D\x81\x01\x03\x81`\x01`\x01`\xA0\x1B\x03\x86\x16Z\xFA\x90\x81\x15a\x11\xBFW_\x91a\x1EvW[Pa\x1D\xEA\x93a\x1E@\x91a\x13\xEEV[`@Qc\t^\xA7\xB3`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16`$\x84\x01R`D\x80\x84\x01\x91\x90\x91R\x82Ra\x1D\xE5`d\x83a\x12\xB4V[\x90P` \x81=` \x11a\x1E\xA2W[\x81a\x1E\x91` \x93\x83a\x12\xB4V[\x81\x01\x03\x12a\x11\xCAWQa\x1D\xEAa\x1E2V[=\x91Pa\x1E\x84V[\x90a\x1F)\x91`\x01\x80`\xA0\x1B\x03\x16_\x80`@Q\x93a\x1E\xC8`@\x86a\x12\xB4V[` \x85R\x7FSafeERC20: low-level call failed` \x86\x01R` \x81Q\x91\x01\x82\x85Z\xF1=\x15a\x1F\xBAW=\x91a\x1F\r\x83a\x12\xD5V[\x92a\x1F\x1B`@Q\x94\x85a\x12\xB4V[\x83R=_` \x85\x01>a\x1F\xBEV[\x80Q\x90\x81\x15\x91\x82\x15a\x1F\x97W[PP\x15a\x1F?WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[\x81\x92P\x90` \x91\x81\x01\x03\x12a\x11\xCAW` \x01Q\x80\x15\x15\x81\x03a\x11\xCAW_\x80a\x1F6V[``\x91[\x91\x92\x90\x15a  WP\x81Q\x15a\x1F\xD2WP\x90V[;\x15a\x1F\xDBW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x82Q\x90\x91P\x15a 3WP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a W\x90`$\x83\x01\x90a\x18\xD7V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 \xF6q\x80\xE6<]/\x95\0]\xE4\x88\xBBH\xE9k\x9A0@\xB2\x17s\xE7^\xB6\xE3zpP\x81\xDA\x87dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `DelayPeriodNotPassed()` and selector `0xfb623b04`.
    ```solidity
    error DelayPeriodNotPassed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayPeriodNotPassed {}
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
        impl ::core::convert::From<DelayPeriodNotPassed> for UnderlyingRustTuple<'_> {
            fn from(value: DelayPeriodNotPassed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayPeriodNotPassed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelayPeriodNotPassed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelayPeriodNotPassed()";
            const SELECTOR: [u8; 4] = [251u8, 98u8, 59u8, 4u8];
            #[inline]
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
    /**Custom error with signature `OnlyRegistryCoordinator()` and selector `0x8729b7be`.
    ```solidity
    error OnlyRegistryCoordinator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinator {}
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
        impl ::core::convert::From<OnlyRegistryCoordinator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinator()";
            const SELECTOR: [u8; 4] = [135u8, 41u8, 183u8, 190u8];
            #[inline]
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
    /**Custom error with signature `OnlyRewardsInitiator()` and selector `0x8e79fdb5`.
    ```solidity
    error OnlyRewardsInitiator();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRewardsInitiator {}
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
        impl ::core::convert::From<OnlyRewardsInitiator> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRewardsInitiator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRewardsInitiator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRewardsInitiator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRewardsInitiator()";
            const SELECTOR: [u8; 4] = [142u8, 121u8, 253u8, 181u8];
            #[inline]
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
    /**Custom error with signature `OnlyStakeRegistry()` and selector `0x46bf2281`.
    ```solidity
    error OnlyStakeRegistry();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyStakeRegistry {}
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
        impl ::core::convert::From<OnlyStakeRegistry> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyStakeRegistry) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyStakeRegistry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyStakeRegistry {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyStakeRegistry()";
            const SELECTOR: [u8; 4] = [70u8, 191u8, 34u8, 129u8];
            #[inline]
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
    /**Event with signature `RewardsInitiatorUpdated(address,address)` and selector `0xe11cddf1816a43318ca175bbc52cd0185436e9cbead7c83acc54a73e461717e3`.
    ```solidity
    event RewardsInitiatorUpdated(address prevRewardsInitiator, address newRewardsInitiator);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsInitiatorUpdated {
        #[allow(missing_docs)]
        pub prevRewardsInitiator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RewardsInitiatorUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "RewardsInitiatorUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8,
                    187u8, 197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8,
                    58u8, 204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevRewardsInitiator: data.0,
                    newRewardsInitiator: data.1,
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
                        &self.prevRewardsInitiator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newRewardsInitiator,
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
        impl alloy_sol_types::private::IntoLogData for RewardsInitiatorUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsInitiatorUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsInitiatorUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
    ```solidity
    constructor(address _avsDirectory, address _registryCoordinator, address _stakeRegistry, address rewards_coordinator, address allocationManager, address _permissionController, address _awesomeVaultTaskManager);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _avsDirectory: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _stakeRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rewards_coordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _awesomeVaultTaskManager: alloy::sol_types::private::Address,
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
                        value._avsDirectory,
                        value._registryCoordinator,
                        value._stakeRegistry,
                        value.rewards_coordinator,
                        value.allocationManager,
                        value._permissionController,
                        value._awesomeVaultTaskManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _avsDirectory: tuple.0,
                        _registryCoordinator: tuple.1,
                        _stakeRegistry: tuple.2,
                        rewards_coordinator: tuple.3,
                        allocationManager: tuple.4,
                        _permissionController: tuple.5,
                        _awesomeVaultTaskManager: tuple.6,
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
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._stakeRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.rewards_coordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._awesomeVaultTaskManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `addPendingAdmin(address)` and selector `0x279432eb`.
    ```solidity
    function addPendingAdmin(address admin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`addPendingAdmin(address)`](addPendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addPendingAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<addPendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<addPendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addPendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addPendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addPendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addPendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addPendingAdmin(address)";
            const SELECTOR: [u8; 4] = [39u8, 148u8, 50u8, 235u8];
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
                        &self.admin,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `awesomeVaultTaskManager()` and selector `0xd4ad1d53`.
    ```solidity
    function awesomeVaultTaskManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct awesomeVaultTaskManagerCall {}
    ///Container type for the return parameters of the [`awesomeVaultTaskManager()`](awesomeVaultTaskManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct awesomeVaultTaskManagerReturn {
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
            impl ::core::convert::From<awesomeVaultTaskManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: awesomeVaultTaskManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for awesomeVaultTaskManagerCall {
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
            impl ::core::convert::From<awesomeVaultTaskManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: awesomeVaultTaskManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for awesomeVaultTaskManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for awesomeVaultTaskManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = awesomeVaultTaskManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "awesomeVaultTaskManager()";
            const SELECTOR: [u8; 4] = [212u8, 173u8, 29u8, 83u8];
            #[inline]
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
    ```solidity
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionReturn {}
    #[allow(
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        rewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createAVSRewardsSubmissionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinatorTypes::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.rewardsSubmissions,
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
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0xa20b99bf`.
    ```solidity
    function createOperatorDirectedAVSRewardsSubmission(IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])`](createOperatorDirectedAVSRewardsSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionReturn {}
    #[allow(
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
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionCall) -> Self {
                    (value.operatorDirectedRewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorDirectedRewardsSubmissions: tuple.0,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorDirectedAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorDirectedAVSRewardsSubmission(((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])";
            const SELECTOR: [u8; 4] = [162u8, 11u8, 153u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.operatorDirectedRewardsSubmissions,
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
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
    ```solidity
    function deregisterOperatorFromAVS(address operator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
    #[allow(
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
            impl ::core::convert::From<deregisterOperatorFromAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<deregisterOperatorFromAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
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
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
    ```solidity
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
    #[allow(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `getOperatorRestakedStrategies(address)` and selector `0x33cfb7b7`.
    ```solidity
    function getOperatorRestakedStrategies(address operator) external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorRestakedStrategies(address)`](getOperatorRestakedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorRestakedStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
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
            impl ::core::convert::From<getOperatorRestakedStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getOperatorRestakedStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorRestakedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorRestakedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorRestakedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorRestakedStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorRestakedStrategies(address)";
            const SELECTOR: [u8; 4] = [51u8, 207u8, 183u8, 183u8];
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
    /**Function with signature `getRestakeableStrategies()` and selector `0xe481af9d`.
    ```solidity
    function getRestakeableStrategies() external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesCall {}
    ///Container type for the return parameters of the [`getRestakeableStrategies()`](getRestakeableStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRestakeableStrategiesReturn {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    #[allow(
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
            impl ::core::convert::From<getRestakeableStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getRestakeableStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRestakeableStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRestakeableStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRestakeableStrategiesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRestakeableStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRestakeableStrategies()";
            const SELECTOR: [u8; 4] = [228u8, 129u8, 175u8, 157u8];
            #[inline]
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
    /**Function with signature `initialize(address,address)` and selector `0x485cc955`.
    ```solidity
    function initialize(address initialOwner, address rewardsInitiator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub rewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.initialOwner, value.rewardsInitiator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        rewardsInitiator: tuple.1,
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
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,address)";
            const SELECTOR: [u8; 4] = [72u8, 92u8, 201u8, 85u8];
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
                        &self.rewardsInitiator,
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
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
    ```solidity
    function registerOperatorToAVS(address operator, ISignatureUtilsMixinTypes.SignatureWithSaltAndExpiry memory operatorSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
    #[allow(
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
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
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
            impl ::core::convert::From<registerOperatorToAVSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
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
                    <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    /**Function with signature `removeAdmin(address)` and selector `0x1785f53c`.
    ```solidity
    function removeAdmin(address admin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminCall {
        #[allow(missing_docs)]
        pub admin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removeAdmin(address)`](removeAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<removeAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminCall) -> Self {
                    (value.admin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { admin: tuple.0 }
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
            impl ::core::convert::From<removeAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAdmin(address)";
            const SELECTOR: [u8; 4] = [23u8, 133u8, 245u8, 60u8];
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
                        &self.admin,
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
    /**Function with signature `removeAppointee(address,address,bytes4)` and selector `0xba550880`.
    ```solidity
    function removeAppointee(address appointee, address target, bytes4 selector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`removeAppointee(address,address,bytes4)`](removeAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeAppointeeReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<removeAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<removeAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [186u8, 85u8, 8u8, 128u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `removePendingAdmin(address)` and selector `0x9da16d8e`.
    ```solidity
    function removePendingAdmin(address pendingAdmin) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminCall {
        #[allow(missing_docs)]
        pub pendingAdmin: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`removePendingAdmin(address)`](removePendingAdminCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removePendingAdminReturn {}
    #[allow(
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
            impl ::core::convert::From<removePendingAdminCall> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminCall) -> Self {
                    (value.pendingAdmin,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pendingAdmin: tuple.0,
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
            impl ::core::convert::From<removePendingAdminReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removePendingAdminReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removePendingAdminReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removePendingAdminCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removePendingAdminReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removePendingAdmin(address)";
            const SELECTOR: [u8; 4] = [157u8, 161u8, 109u8, 142u8];
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
                        &self.pendingAdmin,
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
    /**Function with signature `rewardsInitiator()` and selector `0xfc299dee`.
    ```solidity
    function rewardsInitiator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorCall {}
    ///Container type for the return parameters of the [`rewardsInitiator()`](rewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsInitiatorReturn {
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
            impl ::core::convert::From<rewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorCall {
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
            impl ::core::convert::From<rewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsInitiatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsInitiatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsInitiatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsInitiator()";
            const SELECTOR: [u8; 4] = [252u8, 41u8, 157u8, 238u8];
            #[inline]
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
    /**Function with signature `setAppointee(address,address,bytes4)` and selector `0x1fdb0cfd`.
    ```solidity
    function setAppointee(address appointee, address target, bytes4 selector) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeCall {
        #[allow(missing_docs)]
        pub appointee: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub target: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub selector: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`setAppointee(address,address,bytes4)`](setAppointeeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAppointeeReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<4>,
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
            impl ::core::convert::From<setAppointeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeCall) -> Self {
                    (value.appointee, value.target, value.selector)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        appointee: tuple.0,
                        target: tuple.1,
                        selector: tuple.2,
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
            impl ::core::convert::From<setAppointeeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAppointeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAppointeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAppointeeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<4>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAppointeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAppointee(address,address,bytes4)";
            const SELECTOR: [u8; 4] = [31u8, 219u8, 12u8, 253u8];
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
                        &self.appointee,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.target,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.selector),
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
    /**Function with signature `setClaimerFor(address)` and selector `0xa0169ddd`.
    ```solidity
    function setClaimerFor(address claimer) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForCall {
        #[allow(missing_docs)]
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address)`](setClaimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerForReturn {}
    #[allow(
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
            impl ::core::convert::From<setClaimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForCall) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claimer: tuple.0 }
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
            impl ::core::convert::From<setClaimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerForReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerForReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setClaimerFor(address)";
            const SELECTOR: [u8; 4] = [160u8, 22u8, 157u8, 221u8];
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
                        &self.claimer,
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
    /**Function with signature `setRewardsInitiator(address)` and selector `0x3bc28c8c`.
    ```solidity
    function setRewardsInitiator(address newRewardsInitiator) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorCall {
        #[allow(missing_docs)]
        pub newRewardsInitiator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsInitiator(address)`](setRewardsInitiatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsInitiatorReturn {}
    #[allow(
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
            impl ::core::convert::From<setRewardsInitiatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorCall) -> Self {
                    (value.newRewardsInitiator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newRewardsInitiator: tuple.0,
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
            impl ::core::convert::From<setRewardsInitiatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsInitiatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsInitiatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsInitiatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsInitiatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsInitiator(address)";
            const SELECTOR: [u8; 4] = [59u8, 194u8, 140u8, 140u8];
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
                        &self.newRewardsInitiator,
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
    ```solidity
    function updateAVSMetadataURI(string memory _metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        #[allow(missing_docs)]
        pub _metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURIReturn {}
    #[allow(
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
            impl ::core::convert::From<updateAVSMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value._metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _metadataURI: tuple.0,
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
            impl ::core::convert::From<updateAVSMetadataURIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateAVSMetadataURICall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
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
                        &self._metadataURI,
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
    ///Container for all the [`AwesomeVaultServiceManager`](self) function calls.
    pub enum AwesomeVaultServiceManagerCalls {
        #[allow(missing_docs)]
        addPendingAdmin(addPendingAdminCall),
        #[allow(missing_docs)]
        avsDirectory(avsDirectoryCall),
        #[allow(missing_docs)]
        awesomeVaultTaskManager(awesomeVaultTaskManagerCall),
        #[allow(missing_docs)]
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        createOperatorDirectedAVSRewardsSubmission(createOperatorDirectedAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        #[allow(missing_docs)]
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        #[allow(missing_docs)]
        getOperatorRestakedStrategies(getOperatorRestakedStrategiesCall),
        #[allow(missing_docs)]
        getRestakeableStrategies(getRestakeableStrategiesCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        owner(ownerCall),
        #[allow(missing_docs)]
        registerOperatorToAVS(registerOperatorToAVSCall),
        #[allow(missing_docs)]
        removeAdmin(removeAdminCall),
        #[allow(missing_docs)]
        removeAppointee(removeAppointeeCall),
        #[allow(missing_docs)]
        removePendingAdmin(removePendingAdminCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        rewardsInitiator(rewardsInitiatorCall),
        #[allow(missing_docs)]
        setAppointee(setAppointeeCall),
        #[allow(missing_docs)]
        setClaimerFor(setClaimerForCall),
        #[allow(missing_docs)]
        setRewardsInitiator(setRewardsInitiatorCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl AwesomeVaultServiceManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [23u8, 133u8, 245u8, 60u8],
            [31u8, 219u8, 12u8, 253u8],
            [39u8, 148u8, 50u8, 235u8],
            [51u8, 207u8, 183u8, 183u8],
            [59u8, 194u8, 140u8, 140u8],
            [72u8, 92u8, 201u8, 85u8],
            [107u8, 58u8, 167u8, 46u8],
            [113u8, 80u8, 24u8, 166u8],
            [141u8, 165u8, 203u8, 91u8],
            [153u8, 38u8, 238u8, 125u8],
            [157u8, 161u8, 109u8, 142u8],
            [160u8, 22u8, 157u8, 221u8],
            [162u8, 11u8, 153u8, 191u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [186u8, 85u8, 8u8, 128u8],
            [193u8, 168u8, 226u8, 197u8],
            [212u8, 173u8, 29u8, 83u8],
            [228u8, 129u8, 175u8, 157u8],
            [242u8, 253u8, 227u8, 139u8],
            [252u8, 41u8, 157u8, 238u8],
            [252u8, 227u8, 108u8, 125u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AwesomeVaultServiceManagerCalls {
        const NAME: &'static str = "AwesomeVaultServiceManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 22usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::addPendingAdmin(_) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::awesomeVaultTaskManager(_) => {
                    <awesomeVaultTaskManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorDirectedAVSRewardsSubmission(_) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorRestakedStrategies(_) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRestakeableStrategies(_) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAdmin(_) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeAppointee(_) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removePendingAdmin(_) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsInitiator(_) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAppointee(_) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setClaimerFor(_) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsInitiator(_) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateAVSMetadataURI(_) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::SELECTOR
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
                AwesomeVaultServiceManagerCalls,
            >] = &[
                {
                    fn removeAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <removeAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::removeAdmin)
                    }
                    removeAdmin
                },
                {
                    fn setAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <setAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::setAppointee)
                    }
                    setAppointee
                },
                {
                    fn addPendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <addPendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::addPendingAdmin)
                    }
                    addPendingAdmin
                },
                {
                    fn getOperatorRestakedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultServiceManagerCalls::getOperatorRestakedStrategies,
                            )
                    }
                    getOperatorRestakedStrategies
                },
                {
                    fn setRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::setRewardsInitiator)
                    }
                    setRewardsInitiator
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultServiceManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultServiceManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn removePendingAdmin(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <removePendingAdminCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::removePendingAdmin)
                    }
                    removePendingAdmin
                },
                {
                    fn setClaimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <setClaimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::setClaimerFor)
                    }
                    setClaimerFor
                },
                {
                    fn createOperatorDirectedAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultServiceManagerCalls::createOperatorDirectedAVSRewardsSubmission,
                            )
                    }
                    createOperatorDirectedAVSRewardsSubmission
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn removeAppointee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <removeAppointeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::removeAppointee)
                    }
                    removeAppointee
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultServiceManagerCalls::deregisterOperatorFromOperatorSets,
                            )
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn awesomeVaultTaskManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <awesomeVaultTaskManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::awesomeVaultTaskManager)
                    }
                    awesomeVaultTaskManager
                },
                {
                    fn getRestakeableStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::getRestakeableStrategies)
                    }
                    getRestakeableStrategies
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn rewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerCalls::rewardsInitiator)
                    }
                    rewardsInitiator
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerCalls>
                    {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultServiceManagerCalls::createAVSRewardsSubmission,
                            )
                    }
                    createAVSRewardsSubmission
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
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::awesomeVaultTaskManager(inner) => {
                    <awesomeVaultTaskManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::addPendingAdmin(inner) => {
                    <addPendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::awesomeVaultTaskManager(inner) => {
                    <awesomeVaultTaskManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createAVSRewardsSubmission(inner) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createOperatorDirectedAVSRewardsSubmission(inner) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorRestakedStrategies(inner) => {
                    <getOperatorRestakedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRestakeableStrategies(inner) => {
                    <getRestakeableStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeAdmin(inner) => {
                    <removeAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeAppointee(inner) => {
                    <removeAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removePendingAdmin(inner) => {
                    <removePendingAdminCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::rewardsInitiator(inner) => {
                    <rewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAppointee(inner) => {
                    <setAppointeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsInitiator(inner) => {
                    <setRewardsInitiatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AwesomeVaultServiceManager`](self) custom errors.
    pub enum AwesomeVaultServiceManagerErrors {
        #[allow(missing_docs)]
        DelayPeriodNotPassed(DelayPeriodNotPassed),
        #[allow(missing_docs)]
        OnlyRegistryCoordinator(OnlyRegistryCoordinator),
        #[allow(missing_docs)]
        OnlyRewardsInitiator(OnlyRewardsInitiator),
        #[allow(missing_docs)]
        OnlyStakeRegistry(OnlyStakeRegistry),
    }
    #[automatically_derived]
    impl AwesomeVaultServiceManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [70u8, 191u8, 34u8, 129u8],
            [135u8, 41u8, 183u8, 190u8],
            [142u8, 121u8, 253u8, 181u8],
            [251u8, 98u8, 59u8, 4u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AwesomeVaultServiceManagerErrors {
        const NAME: &'static str = "AwesomeVaultServiceManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DelayPeriodNotPassed(_) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRegistryCoordinator(_) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyRewardsInitiator(_) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyStakeRegistry(_) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::SELECTOR
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
                AwesomeVaultServiceManagerErrors,
            >] = &[
                {
                    fn OnlyStakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerErrors>
                    {
                        <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerErrors::OnlyStakeRegistry)
                    }
                    OnlyStakeRegistry
                },
                {
                    fn OnlyRegistryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerErrors>
                    {
                        <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerErrors::OnlyRegistryCoordinator)
                    }
                    OnlyRegistryCoordinator
                },
                {
                    fn OnlyRewardsInitiator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerErrors>
                    {
                        <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerErrors::OnlyRewardsInitiator)
                    }
                    OnlyRewardsInitiator
                },
                {
                    fn DelayPeriodNotPassed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultServiceManagerErrors>
                    {
                        <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultServiceManagerErrors::DelayPeriodNotPassed)
                    }
                    DelayPeriodNotPassed
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
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::DelayPeriodNotPassed(inner) => {
                    <DelayPeriodNotPassed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinator(inner) => {
                    <OnlyRegistryCoordinator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyRewardsInitiator(inner) => {
                    <OnlyRewardsInitiator as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyStakeRegistry(inner) => {
                    <OnlyStakeRegistry as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`AwesomeVaultServiceManager`](self) events.
    pub enum AwesomeVaultServiceManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        RewardsInitiatorUpdated(RewardsInitiatorUpdated),
    }
    #[automatically_derived]
    impl AwesomeVaultServiceManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
                225u8, 28u8, 221u8, 241u8, 129u8, 106u8, 67u8, 49u8, 140u8, 161u8, 117u8, 187u8,
                197u8, 44u8, 208u8, 24u8, 84u8, 54u8, 233u8, 203u8, 234u8, 215u8, 200u8, 58u8,
                204u8, 84u8, 167u8, 62u8, 70u8, 23u8, 23u8, 227u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AwesomeVaultServiceManagerEvents {
        const NAME: &'static str = "AwesomeVaultServiceManagerEvents";
        const COUNT: usize = 3usize;
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
                Some(<OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OwnershipTransferred)
                }
                Some(<RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsInitiatorUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RewardsInitiatorUpdated)
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
    impl alloy_sol_types::private::IntoLogData for AwesomeVaultServiceManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsInitiatorUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AwesomeVaultServiceManager`](self) contract instance.

    See the [wrapper's documentation](`AwesomeVaultServiceManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AwesomeVaultServiceManagerInstance<T, P, N> {
        AwesomeVaultServiceManagerInstance::<T, P, N>::new(address, provider)
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        allocationManager: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _awesomeVaultTaskManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AwesomeVaultServiceManagerInstance<T, P, N>>,
    > {
        AwesomeVaultServiceManagerInstance::<T, P, N>::deploy(
            provider,
            _avsDirectory,
            _registryCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            allocationManager,
            _permissionController,
            _awesomeVaultTaskManager,
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _stakeRegistry: alloy::sol_types::private::Address,
        rewards_coordinator: alloy::sol_types::private::Address,
        allocationManager: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _awesomeVaultTaskManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AwesomeVaultServiceManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _avsDirectory,
            _registryCoordinator,
            _stakeRegistry,
            rewards_coordinator,
            allocationManager,
            _permissionController,
            _awesomeVaultTaskManager,
        )
    }
    /**A [`AwesomeVaultServiceManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`AwesomeVaultServiceManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AwesomeVaultServiceManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AwesomeVaultServiceManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AwesomeVaultServiceManagerInstance")
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
        > AwesomeVaultServiceManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`AwesomeVaultServiceManager`](self) contract instance.

        See the [wrapper's documentation](`AwesomeVaultServiceManagerInstance`) for more details.*/
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
            _avsDirectory: alloy::sol_types::private::Address,
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            allocationManager: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _awesomeVaultTaskManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<AwesomeVaultServiceManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _avsDirectory,
                _registryCoordinator,
                _stakeRegistry,
                rewards_coordinator,
                allocationManager,
                _permissionController,
                _awesomeVaultTaskManager,
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _stakeRegistry: alloy::sol_types::private::Address,
            rewards_coordinator: alloy::sol_types::private::Address,
            allocationManager: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _awesomeVaultTaskManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _avsDirectory,
                        _registryCoordinator,
                        _stakeRegistry,
                        rewards_coordinator,
                        allocationManager,
                        _permissionController,
                        _awesomeVaultTaskManager,
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
    impl<T, P: ::core::clone::Clone, N> AwesomeVaultServiceManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AwesomeVaultServiceManagerInstance<T, P, N> {
            AwesomeVaultServiceManagerInstance {
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
        > AwesomeVaultServiceManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`addPendingAdmin`] function.
        pub fn addPendingAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, addPendingAdminCall, N> {
            self.call_builder(&addPendingAdminCall { admin })
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`awesomeVaultTaskManager`] function.
        pub fn awesomeVaultTaskManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, awesomeVaultTaskManagerCall, N> {
            self.call_builder(&awesomeVaultTaskManagerCall {})
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(&createAVSRewardsSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorDirectedAVSRewardsSubmissionCall, N>
        {
            self.call_builder(&createOperatorDirectedAVSRewardsSubmissionCall {
                operatorDirectedRewardsSubmissions,
            })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(&deregisterOperatorFromAVSCall { operator })
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromOperatorSetsCall, N>
        {
            self.call_builder(&deregisterOperatorFromOperatorSetsCall {
                operator,
                operatorSetIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorRestakedStrategies`] function.
        pub fn getOperatorRestakedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorRestakedStrategiesCall, N> {
            self.call_builder(&getOperatorRestakedStrategiesCall { operator })
        }
        ///Creates a new call builder for the [`getRestakeableStrategies`] function.
        pub fn getRestakeableStrategies(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRestakeableStrategiesCall, N> {
            self.call_builder(&getRestakeableStrategiesCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            rewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                rewardsInitiator,
            })
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtilsMixinTypes::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(&registerOperatorToAVSCall {
                operator,
                operatorSignature,
            })
        }
        ///Creates a new call builder for the [`removeAdmin`] function.
        pub fn removeAdmin(
            &self,
            admin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAdminCall, N> {
            self.call_builder(&removeAdminCall { admin })
        }
        ///Creates a new call builder for the [`removeAppointee`] function.
        pub fn removeAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeAppointeeCall, N> {
            self.call_builder(&removeAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`removePendingAdmin`] function.
        pub fn removePendingAdmin(
            &self,
            pendingAdmin: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, removePendingAdminCall, N> {
            self.call_builder(&removePendingAdminCall { pendingAdmin })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsInitiator`] function.
        pub fn rewardsInitiator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsInitiatorCall, N> {
            self.call_builder(&rewardsInitiatorCall {})
        }
        ///Creates a new call builder for the [`setAppointee`] function.
        pub fn setAppointee(
            &self,
            appointee: alloy::sol_types::private::Address,
            target: alloy::sol_types::private::Address,
            selector: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAppointeeCall, N> {
            self.call_builder(&setAppointeeCall {
                appointee,
                target,
                selector,
            })
        }
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setRewardsInitiator`] function.
        pub fn setRewardsInitiator(
            &self,
            newRewardsInitiator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsInitiatorCall, N> {
            self.call_builder(&setRewardsInitiatorCall {
                newRewardsInitiator,
            })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            _metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(&updateAVSMetadataURICall { _metadataURI })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > AwesomeVaultServiceManagerInstance<T, P, N>
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
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`RewardsInitiatorUpdated`] event.
        pub fn RewardsInitiatorUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsInitiatorUpdated, N> {
            self.event_filter::<RewardsInitiatorUpdated>()
        }
    }
}
