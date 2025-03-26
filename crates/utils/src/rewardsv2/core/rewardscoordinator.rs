///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinator {
    struct DistributionRoot { bytes32 root; uint32 rewardsCalculationEndTimestamp; uint32 activatedAt; bool disabled; }
    struct EarnerTreeMerkleLeaf { address earner; bytes32 earnerTokenRoot; }
    struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
    struct OperatorReward { address operator; uint256 amount; }
    struct RewardsMerkleClaim { uint32 rootIndex; uint32 earnerIndex; bytes earnerTreeProof; EarnerTreeMerkleLeaf earnerLeaf; uint32[] tokenIndices; bytes[] tokenTreeProofs; TokenTreeMerkleLeaf[] tokenLeaves; }
    struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
    struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
    struct TokenTreeMerkleLeaf { address token; uint256 cumulativeEarnings; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IRewardsCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct DistributionRoot { bytes32 root; uint32 rewardsCalculationEndTimestamp; uint32 activatedAt; bool disabled; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DistributionRoot {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsCalculationEndTimestamp: u32,
        #[allow(missing_docs)]
        pub activatedAt: u32,
        #[allow(missing_docs)]
        pub disabled: bool,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bool,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u32, u32, bool);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DistributionRoot> for UnderlyingRustTuple<'_> {
            fn from(value: DistributionRoot) -> Self {
                (
                    value.root,
                    value.rewardsCalculationEndTimestamp,
                    value.activatedAt,
                    value.disabled,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DistributionRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    root: tuple.0,
                    rewardsCalculationEndTimestamp: tuple.1,
                    activatedAt: tuple.2,
                    disabled: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DistributionRoot {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DistributionRoot {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.rewardsCalculationEndTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.activatedAt),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.disabled,
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
        impl alloy_sol_types::SolType for DistributionRoot {
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
        impl alloy_sol_types::SolStruct for DistributionRoot {
            const NAME: &'static str = "DistributionRoot";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DistributionRoot(bytes32 root,uint32 rewardsCalculationEndTimestamp,uint32 activatedAt,bool disabled)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.root)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.rewardsCalculationEndTimestamp,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.activatedAt)
                        .0,
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.disabled,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DistributionRoot {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.root)
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rewardsCalculationEndTimestamp,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.activatedAt,
                    )
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.disabled,
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
                    &rust.root,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.rewardsCalculationEndTimestamp,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.activatedAt,
                    out,
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.disabled,
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
    struct EarnerTreeMerkleLeaf { address earner; bytes32 earnerTokenRoot; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EarnerTreeMerkleLeaf {
        #[allow(missing_docs)]
        pub earner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub earnerTokenRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<EarnerTreeMerkleLeaf> for UnderlyingRustTuple<'_> {
            fn from(value: EarnerTreeMerkleLeaf) -> Self {
                (value.earner, value.earnerTokenRoot)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for EarnerTreeMerkleLeaf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    earner: tuple.0,
                    earnerTokenRoot: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for EarnerTreeMerkleLeaf {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for EarnerTreeMerkleLeaf {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.earner,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.earnerTokenRoot),
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
        impl alloy_sol_types::SolType for EarnerTreeMerkleLeaf {
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
        impl alloy_sol_types::SolStruct for EarnerTreeMerkleLeaf {
            const NAME: &'static str = "EarnerTreeMerkleLeaf";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EarnerTreeMerkleLeaf(address earner,bytes32 earnerTokenRoot)",
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
                            &self.earner,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.earnerTokenRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for EarnerTreeMerkleLeaf {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.earner,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.earnerTokenRoot,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.earner,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.earnerTokenRoot,
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
    struct RewardsMerkleClaim { uint32 rootIndex; uint32 earnerIndex; bytes earnerTreeProof; EarnerTreeMerkleLeaf earnerLeaf; uint32[] tokenIndices; bytes[] tokenTreeProofs; TokenTreeMerkleLeaf[] tokenLeaves; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsMerkleClaim {
        #[allow(missing_docs)]
        pub rootIndex: u32,
        #[allow(missing_docs)]
        pub earnerIndex: u32,
        #[allow(missing_docs)]
        pub earnerTreeProof: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub earnerLeaf: <EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub tokenIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub tokenTreeProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
        pub tokenLeaves: alloy::sol_types::private::Vec<
            <TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            EarnerTreeMerkleLeaf,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
            alloy::sol_types::sol_data::Array<TokenTreeMerkleLeaf>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::Bytes,
            <EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            alloy::sol_types::private::Vec<
                <TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<RewardsMerkleClaim> for UnderlyingRustTuple<'_> {
            fn from(value: RewardsMerkleClaim) -> Self {
                (
                    value.rootIndex,
                    value.earnerIndex,
                    value.earnerTreeProof,
                    value.earnerLeaf,
                    value.tokenIndices,
                    value.tokenTreeProofs,
                    value.tokenLeaves,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RewardsMerkleClaim {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    rootIndex: tuple.0,
                    earnerIndex: tuple.1,
                    earnerTreeProof: tuple.2,
                    earnerLeaf: tuple.3,
                    tokenIndices: tuple.4,
                    tokenTreeProofs: tuple.5,
                    tokenLeaves: tuple.6,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RewardsMerkleClaim {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RewardsMerkleClaim {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rootIndex),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.earnerIndex),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.earnerTreeProof,
                    ),
                    <EarnerTreeMerkleLeaf as alloy_sol_types::SolType>::tokenize(
                        &self.earnerLeaf,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenTreeProofs),
                    <alloy::sol_types::sol_data::Array<
                        TokenTreeMerkleLeaf,
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenLeaves),
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
        impl alloy_sol_types::SolType for RewardsMerkleClaim {
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
        impl alloy_sol_types::SolStruct for RewardsMerkleClaim {
            const NAME: &'static str = "RewardsMerkleClaim";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsMerkleClaim(uint32 rootIndex,uint32 earnerIndex,bytes earnerTreeProof,EarnerTreeMerkleLeaf earnerLeaf,uint32[] tokenIndices,bytes[] tokenTreeProofs,TokenTreeMerkleLeaf[] tokenLeaves)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(<EarnerTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(
                    <EarnerTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components
                    .push(<TokenTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(
                    <TokenTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_components(),
                );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.rootIndex)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.earnerIndex)
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.earnerTreeProof,
                        )
                        .0,
                    <EarnerTreeMerkleLeaf as alloy_sol_types::SolType>::eip712_data_word(
                            &self.earnerLeaf,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenIndices)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.tokenTreeProofs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        TokenTreeMerkleLeaf,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tokenLeaves)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RewardsMerkleClaim {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.rootIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.earnerIndex,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.earnerTreeProof,
                    )
                    + <EarnerTreeMerkleLeaf as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.earnerLeaf,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenTreeProofs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        TokenTreeMerkleLeaf,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tokenLeaves,
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
                    &rust.rootIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.earnerIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.earnerTreeProof,
                    out,
                );
                <EarnerTreeMerkleLeaf as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.earnerLeaf,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Bytes,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenTreeProofs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    TokenTreeMerkleLeaf,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tokenLeaves,
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
    /**```solidity
    struct TokenTreeMerkleLeaf { address token; uint256 cumulativeEarnings; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenTreeMerkleLeaf {
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub cumulativeEarnings: alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<TokenTreeMerkleLeaf> for UnderlyingRustTuple<'_> {
            fn from(value: TokenTreeMerkleLeaf) -> Self {
                (value.token, value.cumulativeEarnings)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TokenTreeMerkleLeaf {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    token: tuple.0,
                    cumulativeEarnings: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TokenTreeMerkleLeaf {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TokenTreeMerkleLeaf {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.cumulativeEarnings,
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
        impl alloy_sol_types::SolType for TokenTreeMerkleLeaf {
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
        impl alloy_sol_types::SolStruct for TokenTreeMerkleLeaf {
            const NAME: &'static str = "TokenTreeMerkleLeaf";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TokenTreeMerkleLeaf(address token,uint256 cumulativeEarnings)",
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
                            &self.token,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.cumulativeEarnings,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TokenTreeMerkleLeaf {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.token,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.cumulativeEarnings,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.token,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.cumulativeEarnings,
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
    /**Creates a new wrapper around an on-chain [`IRewardsCoordinator`](self) contract instance.

    See the [wrapper's documentation](`IRewardsCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IRewardsCoordinatorInstance<T, P, N> {
        IRewardsCoordinatorInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IRewardsCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IRewardsCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IRewardsCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IRewardsCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IRewardsCoordinatorInstance")
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
        > IRewardsCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinator`](self) contract instance.

        See the [wrapper's documentation](`IRewardsCoordinatorInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IRewardsCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IRewardsCoordinatorInstance<T, P, N> {
            IRewardsCoordinatorInstance {
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
        > IRewardsCoordinatorInstance<T, P, N>
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
        > IRewardsCoordinatorInstance<T, P, N>
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
library IRewardsCoordinator {
    struct DistributionRoot {
        bytes32 root;
        uint32 rewardsCalculationEndTimestamp;
        uint32 activatedAt;
        bool disabled;
    }
    struct EarnerTreeMerkleLeaf {
        address earner;
        bytes32 earnerTokenRoot;
    }
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
    struct RewardsMerkleClaim {
        uint32 rootIndex;
        uint32 earnerIndex;
        bytes earnerTreeProof;
        EarnerTreeMerkleLeaf earnerLeaf;
        uint32[] tokenIndices;
        bytes[] tokenTreeProofs;
        TokenTreeMerkleLeaf[] tokenLeaves;
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
    struct TokenTreeMerkleLeaf {
        address token;
        uint256 cumulativeEarnings;
    }
}

interface RewardsCoordinator {
    event AVSRewardsSubmissionCreated(address indexed avs, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    event ActivationDelaySet(uint32 oldActivationDelay, uint32 newActivationDelay);
    event ClaimerForSet(address indexed earner, address indexed oldClaimer, address indexed claimer);
    event DefaultOperatorSplitBipsSet(uint16 oldDefaultOperatorSplitBips, uint16 newDefaultOperatorSplitBips);
    event DistributionRootDisabled(uint32 indexed rootIndex);
    event DistributionRootSubmitted(uint32 indexed rootIndex, bytes32 indexed root, uint32 indexed rewardsCalculationEndTimestamp, uint32 activatedAt);
    event Initialized(uint8 version);
    event OperatorAVSSplitBipsSet(address indexed caller, address indexed operator, address indexed avs, uint32 activatedAt, uint16 oldOperatorAVSSplitBips, uint16 newOperatorAVSSplitBips);
    event OperatorDirectedAVSRewardsSubmissionCreated(address indexed caller, address indexed avs, bytes32 indexed operatorDirectedRewardsSubmissionHash, uint256 submissionNonce, IRewardsCoordinator.OperatorDirectedRewardsSubmission operatorDirectedRewardsSubmission);
    event OperatorPISplitBipsSet(address indexed caller, address indexed operator, uint32 activatedAt, uint16 oldOperatorPISplitBips, uint16 newOperatorPISplitBips);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event RewardsClaimed(bytes32 root, address indexed earner, address indexed claimer, address indexed recipient, address token, uint256 claimedAmount);
    event RewardsForAllSubmitterSet(address indexed rewardsForAllSubmitter, bool indexed oldValue, bool indexed newValue);
    event RewardsSubmissionForAllCreated(address indexed submitter, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    event RewardsSubmissionForAllEarnersCreated(address indexed tokenHopper, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    event RewardsUpdaterSet(address indexed oldRewardsUpdater, address indexed newRewardsUpdater);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegationManager, address _strategyManager, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 __GENESIS_REWARDS_TIMESTAMP);

    function CALCULATION_INTERVAL_SECONDS() external view returns (uint32);
    function GENESIS_REWARDS_TIMESTAMP() external view returns (uint32);
    function MAX_FUTURE_LENGTH() external view returns (uint32);
    function MAX_RETROACTIVE_LENGTH() external view returns (uint32);
    function MAX_REWARDS_DURATION() external view returns (uint32);
    function activationDelay() external view returns (uint32);
    function beaconChainETHStrategy() external view returns (address);
    function calculateEarnerLeafHash(IRewardsCoordinator.EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function calculateTokenLeafHash(IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function checkClaim(IRewardsCoordinator.RewardsMerkleClaim memory claim) external view returns (bool);
    function claimerFor(address) external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorDirectedAVSRewardsSubmission(address avs, IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function createRewardsForAllEarners(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    function createRewardsForAllSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    function cumulativeClaimed(address, address) external view returns (uint256);
    function currRewardsCalculationEndTimestamp() external view returns (uint32);
    function defaultOperatorSplitBips() external view returns (uint16);
    function delegationManager() external view returns (address);
    function disableRoot(uint32 rootIndex) external;
    function domainSeparator() external view returns (bytes32);
    function getCurrentClaimableDistributionRoot() external view returns (IRewardsCoordinator.DistributionRoot memory);
    function getCurrentDistributionRoot() external view returns (IRewardsCoordinator.DistributionRoot memory);
    function getDistributionRootAtIndex(uint256 index) external view returns (IRewardsCoordinator.DistributionRoot memory);
    function getDistributionRootsLength() external view returns (uint256);
    function getOperatorAVSSplit(address operator, address avs) external view returns (uint16);
    function getOperatorPISplit(address operator) external view returns (uint16);
    function getRootIndexFromHash(bytes32 rootHash) external view returns (uint32);
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _defaultSplitBips) external;
    function isAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    function isOperatorDirectedAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    function isRewardsForAllSubmitter(address) external view returns (bool);
    function isRewardsSubmissionForAllEarnersHash(address, bytes32) external view returns (bool);
    function isRewardsSubmissionForAllHash(address, bytes32) external view returns (bool);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function processClaim(IRewardsCoordinator.RewardsMerkleClaim memory claim, address recipient) external;
    function processClaims(IRewardsCoordinator.RewardsMerkleClaim[] memory claims, address recipient) external;
    function renounceOwnership() external;
    function rewardsUpdater() external view returns (address);
    function setActivationDelay(uint32 _activationDelay) external;
    function setClaimerFor(address claimer) external;
    function setDefaultOperatorSplit(uint16 split) external;
    function setOperatorAVSSplit(address operator, address avs, uint16 split) external;
    function setOperatorPISplit(address operator, uint16 split) external;
    function setPauserRegistry(address newPauserRegistry) external;
    function setRewardsForAllSubmitter(address _submitter, bool _newValue) external;
    function setRewardsUpdater(address _rewardsUpdater) external;
    function strategyManager() external view returns (address);
    function submissionNonce(address) external view returns (uint256);
    function submitRoot(bytes32 root, uint32 rewardsCalculationEndTimestamp) external;
    function transferOwnership(address newOwner) external;
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
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_strategyManager",
        "type": "address",
        "internalType": "contract IStrategyManager"
      },
      {
        "name": "_CALCULATION_INTERVAL_SECONDS",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_MAX_REWARDS_DURATION",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_MAX_RETROACTIVE_LENGTH",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_MAX_FUTURE_LENGTH",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "__GENESIS_REWARDS_TIMESTAMP",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CALCULATION_INTERVAL_SECONDS",
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
    "name": "GENESIS_REWARDS_TIMESTAMP",
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
    "name": "MAX_FUTURE_LENGTH",
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
    "name": "MAX_RETROACTIVE_LENGTH",
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
    "name": "MAX_REWARDS_DURATION",
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
    "name": "activationDelay",
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
    "name": "calculateEarnerLeafHash",
    "inputs": [
      {
        "name": "leaf",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.EarnerTreeMerkleLeaf",
        "components": [
          {
            "name": "earner",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "earnerTokenRoot",
            "type": "bytes32",
            "internalType": "bytes32"
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
    "name": "calculateTokenLeafHash",
    "inputs": [
      {
        "name": "leaf",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.TokenTreeMerkleLeaf",
        "components": [
          {
            "name": "token",
            "type": "address",
            "internalType": "contract IERC20"
          },
          {
            "name": "cumulativeEarnings",
            "type": "uint256",
            "internalType": "uint256"
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
    "name": "checkClaim",
    "inputs": [
      {
        "name": "claim",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.RewardsMerkleClaim",
        "components": [
          {
            "name": "rootIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerTreeProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "earnerLeaf",
            "type": "tuple",
            "internalType": "struct IRewardsCoordinator.EarnerTreeMerkleLeaf",
            "components": [
              {
                "name": "earner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "earnerTokenRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "tokenIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "tokenTreeProofs",
            "type": "bytes[]",
            "internalType": "bytes[]"
          },
          {
            "name": "tokenLeaves",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.TokenTreeMerkleLeaf[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract IERC20"
              },
              {
                "name": "cumulativeEarnings",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
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
    "name": "claimerFor",
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
    "name": "createAVSRewardsSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorDirectedRewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.OperatorDirectedRewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
            "internalType": "struct IRewardsCoordinator.OperatorReward[]",
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
    "name": "createRewardsForAllEarners",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
    "name": "createRewardsForAllSubmission",
    "inputs": [
      {
        "name": "rewardsSubmissions",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.RewardsSubmission[]",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
    "name": "cumulativeClaimed",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "contract IERC20"
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
    "name": "currRewardsCalculationEndTimestamp",
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
    "name": "defaultOperatorSplitBips",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "delegationManager",
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
    "name": "disableRoot",
    "inputs": [
      {
        "name": "rootIndex",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "getCurrentClaimableDistributionRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.DistributionRoot",
        "components": [
          {
            "name": "root",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rewardsCalculationEndTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "activatedAt",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "disabled",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentDistributionRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.DistributionRoot",
        "components": [
          {
            "name": "root",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rewardsCalculationEndTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "activatedAt",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "disabled",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDistributionRootAtIndex",
    "inputs": [
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
        "internalType": "struct IRewardsCoordinator.DistributionRoot",
        "components": [
          {
            "name": "root",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "rewardsCalculationEndTimestamp",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "activatedAt",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "disabled",
            "type": "bool",
            "internalType": "bool"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getDistributionRootsLength",
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
    "name": "getOperatorAVSSplit",
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
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorPISplit",
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
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRootIndexFromHash",
    "inputs": [
      {
        "name": "rootHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
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
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "initialPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_rewardsUpdater",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_activationDelay",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_defaultSplitBips",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isAVSRewardsSubmissionHash",
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
    "name": "isOperatorDirectedAVSRewardsSubmissionHash",
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
    "name": "isRewardsForAllSubmitter",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isRewardsSubmissionForAllEarnersHash",
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
    "name": "isRewardsSubmissionForAllHash",
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
    "name": "processClaim",
    "inputs": [
      {
        "name": "claim",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinator.RewardsMerkleClaim",
        "components": [
          {
            "name": "rootIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerTreeProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "earnerLeaf",
            "type": "tuple",
            "internalType": "struct IRewardsCoordinator.EarnerTreeMerkleLeaf",
            "components": [
              {
                "name": "earner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "earnerTokenRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "tokenIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "tokenTreeProofs",
            "type": "bytes[]",
            "internalType": "bytes[]"
          },
          {
            "name": "tokenLeaves",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.TokenTreeMerkleLeaf[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract IERC20"
              },
              {
                "name": "cumulativeEarnings",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processClaims",
    "inputs": [
      {
        "name": "claims",
        "type": "tuple[]",
        "internalType": "struct IRewardsCoordinator.RewardsMerkleClaim[]",
        "components": [
          {
            "name": "rootIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "earnerTreeProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "earnerLeaf",
            "type": "tuple",
            "internalType": "struct IRewardsCoordinator.EarnerTreeMerkleLeaf",
            "components": [
              {
                "name": "earner",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "earnerTokenRoot",
                "type": "bytes32",
                "internalType": "bytes32"
              }
            ]
          },
          {
            "name": "tokenIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "tokenTreeProofs",
            "type": "bytes[]",
            "internalType": "bytes[]"
          },
          {
            "name": "tokenLeaves",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.TokenTreeMerkleLeaf[]",
            "components": [
              {
                "name": "token",
                "type": "address",
                "internalType": "contract IERC20"
              },
              {
                "name": "cumulativeEarnings",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          }
        ]
      },
      {
        "name": "recipient",
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
    "name": "rewardsUpdater",
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
    "name": "setActivationDelay",
    "inputs": [
      {
        "name": "_activationDelay",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "setDefaultOperatorSplit",
    "inputs": [
      {
        "name": "split",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorAVSSplit",
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
        "name": "split",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setOperatorPISplit",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "split",
        "type": "uint16",
        "internalType": "uint16"
      }
    ],
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
    "name": "setRewardsForAllSubmitter",
    "inputs": [
      {
        "name": "_submitter",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_newValue",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setRewardsUpdater",
    "inputs": [
      {
        "name": "_rewardsUpdater",
        "type": "address",
        "internalType": "address"
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
    "name": "submissionNonce",
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
    "name": "submitRoot",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "rewardsCalculationEndTimestamp",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "AVSRewardsSubmissionCreated",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "submissionNonce",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "rewardsSubmissionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "rewardsSubmission",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRewardsCoordinator.RewardsSubmission",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ActivationDelaySet",
    "inputs": [
      {
        "name": "oldActivationDelay",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "newActivationDelay",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ClaimerForSet",
    "inputs": [
      {
        "name": "earner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldClaimer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "claimer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DefaultOperatorSplitBipsSet",
    "inputs": [
      {
        "name": "oldDefaultOperatorSplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      },
      {
        "name": "newDefaultOperatorSplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DistributionRootDisabled",
    "inputs": [
      {
        "name": "rootIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DistributionRootSubmitted",
    "inputs": [
      {
        "name": "rootIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "root",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "rewardsCalculationEndTimestamp",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "activatedAt",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
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
    "name": "OperatorAVSSplitBipsSet",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "activatedAt",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "oldOperatorAVSSplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      },
      {
        "name": "newOperatorAVSSplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorDirectedAVSRewardsSubmissionCreated",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorDirectedRewardsSubmissionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "submissionNonce",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "operatorDirectedRewardsSubmission",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRewardsCoordinator.OperatorDirectedRewardsSubmission",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
            "internalType": "struct IRewardsCoordinator.OperatorReward[]",
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorPISplitBipsSet",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "activatedAt",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "oldOperatorPISplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      },
      {
        "name": "newOperatorPISplitBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
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
    "name": "RewardsClaimed",
    "inputs": [
      {
        "name": "root",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      },
      {
        "name": "earner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "claimer",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "indexed": false,
        "internalType": "contract IERC20"
      },
      {
        "name": "claimedAmount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsForAllSubmitterSet",
    "inputs": [
      {
        "name": "rewardsForAllSubmitter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "oldValue",
        "type": "bool",
        "indexed": true,
        "internalType": "bool"
      },
      {
        "name": "newValue",
        "type": "bool",
        "indexed": true,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsSubmissionForAllCreated",
    "inputs": [
      {
        "name": "submitter",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "submissionNonce",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "rewardsSubmissionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "rewardsSubmission",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRewardsCoordinator.RewardsSubmission",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsSubmissionForAllEarnersCreated",
    "inputs": [
      {
        "name": "tokenHopper",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "submissionNonce",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "rewardsSubmissionHash",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "rewardsSubmission",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IRewardsCoordinator.RewardsSubmission",
        "components": [
          {
            "name": "strategiesAndMultipliers",
            "type": "tuple[]",
            "internalType": "struct IRewardsCoordinator.StrategyAndMultiplier[]",
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
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RewardsUpdaterSet",
    "inputs": [
      {
        "name": "oldRewardsUpdater",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newRewardsUpdater",
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
pub mod RewardsCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101806040523480156200001257600080fd5b506040516200572e3803806200572e8339810160408190526200003591620002e4565b868686868686866200004885826200037e565b63ffffffff1615620000ed5760405162461bcd60e51b815260206004820152606060248201527f52657761726473436f6f7264696e61746f723a2047454e455349535f5245574160448201527f5244535f54494d455354414d50206d7573742062652061206d756c7469706c6560648201527f206f662043414c43554c4154494f4e5f494e54455256414c5f5345434f4e4453608482015260a4015b60405180910390fd5b620000fc62015180866200037e565b63ffffffff16156200019d5760405162461bcd60e51b815260206004820152605760248201527f52657761726473436f6f7264696e61746f723a2043414c43554c4154494f4e5f60448201527f494e54455256414c5f5345434f4e4453206d7573742062652061206d756c746960648201527f706c65206f6620534e415053484f545f434144454e4345000000000000000000608482015260a401620000e4565b6001600160a01b0396871661012052949095166101405263ffffffff92831660805290821660a052811660c05291821660e0521661010052620001df620001f2565b5050466101605250620003b09350505050565b600054610100900460ff16156200025c5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401620000e4565b60005460ff9081161015620002af576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114620002c757600080fd5b50565b805163ffffffff81168114620002df57600080fd5b919050565b600080600080600080600060e0888a0312156200030057600080fd5b87516200030d81620002b1565b60208901519097506200032081620002b1565b95506200033060408901620002ca565b94506200034060608901620002ca565b93506200035060808901620002ca565b92506200036060a08901620002ca565b91506200037060c08901620002ca565b905092959891949750929550565b600063ffffffff80841680620003a457634e487b7160e01b600052601260045260246000fd5b92169190910692915050565b60805160a05160c05160e051610100516101205161014051610160516152e762000447600039600061206e0152600081816105460152613d6f015260006108a00152600081816104700152613c690152600081816103c40152612a1301526000818161051f0152613c270152600081816107ec01526139c401526000818161073e01528181613a7d0152613b4d01526152e76000f3fe608060405234801561001057600080fd5b50600436106103825760003560e01c8063865c6953116101de578063d4540a551161010f578063f2fde38b116100ad578063fabc1cbc1161007c578063fabc1cbc14610931578063fbf1e2c114610944578063fce36c7d14610957578063ff9f6cce1461096a57600080fd5b8063f2fde38b146108f0578063f698da2514610903578063f8cd84481461090b578063f96abf2e1461091e57600080fd5b8063e063f81f116100e9578063e063f81f14610875578063e810ce2114610888578063ea4d3c9b1461089b578063ed71e6a2146108c257600080fd5b8063d4540a551461083c578063dcbb03b31461084f578063de02e5031461086257600080fd5b8063a0169ddd1161017c578063b3dbb0e011610156578063b3dbb0e0146107b4578063bb7e451f146107c7578063bf21a8aa146107e7578063c46db6061461080e57600080fd5b8063a0169ddd14610760578063a50a1d9c14610773578063aebd8bae1461078657600080fd5b80639104c319116101b85780639104c319146107035780639be3d4e41461071e5780639cb9a5fa146107265780639d45c2811461073957600080fd5b8063865c6953146106b4578063886f1195146106df5780638da5cb5b146106f257600080fd5b80633efe1db6116102b85780635c975abb116102565780636d21117e116102305780636d21117e14610663578063715018a6146106915780637b8f8b0514610699578063863cb9a9146106a157600080fd5b80635c975abb146106335780635e9d83481461063b57806363f6a7981461064e57600080fd5b80634d18cc35116102925780634d18cc35146105de57806358baaa3e146105f5578063595c6a67146106085780635ac86ab71461061057600080fd5b80633efe1db6146105925780634596021c146105a55780634b943960146105b857600080fd5b8063149bc8721161032557806337838ed0116102ff57806337838ed01461051a57806339b70e38146105415780633a8c0786146105685780633ccc861d1461057f57600080fd5b8063149bc872146104a55780632b9f64a4146104c657806336af41fa1461050757600080fd5b80630eb38345116103615780630eb383451461044357806310d67a2f14610458578063131433b41461046b578063136439dd1461049257600080fd5b806218572c1461038757806304a0c502146103bf5780630e9a53cf146103fb575b600080fd5b6103aa6103953660046145b1565b60d16020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016103b6565b61040361097d565b604080518251815260208084015163ffffffff908116918301919091528383015116918101919091526060918201511515918101919091526080016103b6565b6104566104513660046145dc565b610a81565b005b6104566104663660046145b1565b610b03565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6104566104a0366004614615565b610bbf565b6104b86104b3366004614646565b610cfe565b6040519081526020016103b6565b6104ef6104d43660046145b1565b60cc602052600090815260409020546001600160a01b031681565b6040516001600160a01b0390911681526020016103b6565b6104566105153660046146ae565b610d74565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6104ef7f000000000000000000000000000000000000000000000000000000000000000081565b60cb546103e690600160a01b900463ffffffff1681565b61045661058d366004614703565b610f3e565b6104566105a036600461475e565b610fa3565b6104566105b336600461478a565b611274565b6105cb6105c63660046145b1565b61131b565b60405161ffff90911681526020016103b6565b60cb546103e690600160c01b900463ffffffff1681565b6104566106033660046147e1565b611377565b610456611388565b6103aa61061e3660046147fc565b606654600160ff9092169190911b9081161490565b6066546104b8565b6103aa61064936600461481f565b61144f565b60cb546105cb90600160e01b900461ffff1681565b6103aa610671366004614854565b60cf60209081526000928352604080842090915290825290205460ff1681565b6104566114dc565b60ca546104b8565b6104566106af3660046145b1565b6114f0565b6104b86106c2366004614880565b60cd60209081526000928352604080842090915290825290205481565b6065546104ef906001600160a01b031681565b6033546001600160a01b03166104ef565b6104ef73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b610403611501565b6104566107343660046148ae565b61159f565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b61045661076e3660046145b1565b6117d8565b610456610781366004614915565b611837565b6103aa610794366004614854565b60d260209081526000928352604080842090915290825290205460ff1681565b6104566107c2366004614930565b611848565b6104b86107d53660046145b1565b60ce6020526000908152604090205481565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6103aa61081c366004614854565b60d060209081526000928352604080842090915290825290205460ff1681565b61045661084a36600461495c565b611a7b565b61045661085d3660046149cf565b611bc3565b610403610870366004614615565b611e1a565b6105cb610883366004614880565b611eac565b6103e6610896366004614615565b611f19565b6104ef7f000000000000000000000000000000000000000000000000000000000000000081565b6103aa6108d0366004614854565b60d360209081526000928352604080842090915290825290205460ff1681565b6104566108fe3660046145b1565b611ff4565b6104b861206a565b6104b8610919366004614646565b6120a8565b61045661092c3660046147e1565b6120b9565b61045661093f366004614615565b6122ef565b60cb546104ef906001600160a01b031681565b6104566109653660046146ae565b61244b565b6104566109783660046146ae565b6125ca565b60408051608081018252600080825260208201819052918101829052606081019190915260ca545b8015610a5857600060ca6109ba600184614a2c565b815481106109ca576109ca614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff161580156060830181905291925090610a3a5750806040015163ffffffff164210155b15610a455792915050565b5080610a5081614a59565b9150506109a5565b505060408051608081018252600080825260208201819052918101829052606081019190915290565b610a89612778565b6001600160a01b038216600081815260d1602052604080822054905160ff9091169284151592841515927f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c9190a4506001600160a01b0391909116600090815260d160205260409020805460ff1916911515919091179055565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b56573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b7a9190614a70565b6001600160a01b0316336001600160a01b031614610bb35760405162461bcd60e51b8152600401610baa90614a8d565b60405180910390fd5b610bbc816127d2565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610c07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c2b9190614ad7565b610c475760405162461bcd60e51b8152600401610baa90614af4565b60665481811614610cc05760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610baa565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b600080610d0e60208401846145b1565b8360200135604051602001610d579392919060f89390931b6001600160f81b031916835260609190911b6bffffffffffffffffffffffff19166001830152601582015260350190565b604051602081830303815290604052805190602001209050919050565b60665460019060029081161415610d9d5760405162461bcd60e51b8152600401610baa90614b3c565b33600090815260d1602052604090205460ff16610dcc5760405162461bcd60e51b8152600401610baa90614b73565b60026097541415610def5760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f335736848483818110610e1257610e12614a43565b9050602002810190610e249190614c21565b33600081815260ce60209081526040808320549051949550939192610e4f9290918591879101614d7a565b604051602081830303815290604052805190602001209050610e70836128c9565b33600090815260d0602090815260408083208484529091529020805460ff19166001908117909155610ea3908390614daa565b33600081815260ce602052604090819020929092559051829184917f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf27048290610eeb908890614dc2565b60405180910390a4610f1d333060408601803590610f0c90602089016145b1565b6001600160a01b0316929190612adf565b5050508080610f2b90614dd5565b915050610df7565b505060016097555050565b60665460029060049081161415610f675760405162461bcd60e51b8152600401610baa90614b3c565b60026097541415610f8a5760405162461bcd60e51b8152600401610baa90614bea565b6002609755610f998383612b50565b5050600160975550565b60665460039060089081161415610fcc5760405162461bcd60e51b8152600401610baa90614b3c565b60cb546001600160a01b03163314610ff65760405162461bcd60e51b8152600401610baa90614df0565b60cb5463ffffffff600160c01b9091048116908316116110925760405162461bcd60e51b815260206004820152604b60248201527f52657761726473436f6f7264696e61746f722e7375626d6974526f6f743a206e60448201527f657720726f6f74206d75737420626520666f72206e657765722063616c63756c60648201526a185d1959081c195c9a5bd960aa1b608482015260a401610baa565b428263ffffffff161061112b5760405162461bcd60e51b815260206004820152605560248201527f52657761726473436f6f7264696e61746f722e7375626d6974526f6f743a207260448201527f65776172647343616c63756c6174696f6e456e6454696d657374616d702063616064820152746e6e6f7420626520696e207468652066757475726560581b608482015260a401610baa565b60ca5460cb5460009061114b90600160a01b900463ffffffff1642614e44565b6040805160808101825287815263ffffffff878116602080840182815286841685870181815260006060880181815260ca8054600181018255925297517f42d72674974f694b5f5159593243114d38a5c39c89d6b62fee061ff523240ee160029092029182015592517f42d72674974f694b5f5159593243114d38a5c39c89d6b62fee061ff523240ee290930180549151975193871667ffffffffffffffff1990921691909117600160201b978716979097029690961760ff60401b1916600160401b921515929092029190911790945560cb805463ffffffff60c01b1916600160c01b840217905593519283529394508892908616917fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd08910160405180910390a45050505050565b6066546002906004908116141561129d5760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156112c05760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b8381101561130f576112fd8585838181106112e5576112e5614a43565b90506020028101906112f79190614e6c565b84612b50565b8061130781614dd5565b9150506112c8565b50506001609755505050565b6001600160a01b038116600090815260d5602090815260408083208151606081018352905461ffff80821683526201000082041693820193909352600160201b90920463ffffffff169082015261137190612ebd565b92915050565b61137f612778565b610bbc81612f0b565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156113d0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113f49190614ad7565b6114105760405162461bcd60e51b8152600401610baa90614af4565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60006114d48260ca61146460208301836147e1565b63ffffffff168154811061147a5761147a614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff1615156060820152612f7c565b506001919050565b6114e4612778565b6114ee600061324d565b565b6114f8612778565b610bbc8161329f565b60408051608081018252600080825260208201819052918101829052606081019190915260ca805461153590600190614a2c565b8154811061154557611545614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff1615156060820152919050565b606654600590602090811614156115c85760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156115eb5760405162461bcd60e51b8152600401610baa90614bea565b6002609755336001600160a01b0385161461168b5760405162461bcd60e51b815260206004820152605460248201527f52657761726473436f6f7264696e61746f722e6372656174654f70657261746f60448201527f724469726563746564415653526577617264735375626d697373696f6e3a2063606482015273616c6c6572206973206e6f74207468652041565360601b608482015260a401610baa565b60005b8281101561130f57368484838181106116a9576116a9614a43565b90506020028101906116bb9190614e82565b6001600160a01b038716600090815260ce60209081526040808320549051939450926116ed918a918591879101614fff565b6040516020818303038152906040528051906020012090506000611710846132fb565b6001600160a01b038a16600090815260d3602090815260408083208684529091529020805460ff1916600190811790915590915061174f908490614daa565b6001600160a01b038a16600081815260ce60205260409081902092909255905183919033907ffc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e0906117a39088908a90615026565b60405180910390a46117c1333083610f0c6040890160208a016145b1565b5050505080806117d090614dd5565b91505061168e565b33600081815260cc602052604080822080546001600160a01b031981166001600160a01b038781169182179093559251911692839185917fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca31291a4505050565b61183f612778565b610bbc816136cb565b606654600790608090811614156118715760405162461bcd60e51b8152600401610baa90614b3c565b336001600160a01b038416146118f95760405162461bcd60e51b815260206004820152604160248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72504960448201527f53706c69743a2063616c6c6572206973206e6f7420746865206f70657261746f6064820152603960f91b608482015260a401610baa565b61271061ffff831611156119805760405162461bcd60e51b815260206004820152604260248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72504960448201527f53706c69743a2073706c6974206d757374206265203c3d203130303030206269606482015261707360f01b608482015260a401610baa565b60cb5460009061199d90600160a01b900463ffffffff1642614e44565b6001600160a01b038516600090815260d5602090815260408083208151606081018352905461ffff80821683526201000082041693820193909352600160201b90920463ffffffff1690820152919250906119f790612ebd565b6001600160a01b038616600090815260d560205260409020909150611a1d908584613736565b6040805163ffffffff8416815261ffff838116602083015286168183015290516001600160a01b0387169133917fd1e028bd664486a46ad26040e999cd2d22e1e9a094ee6afe19fcf64678f16f749181900360600190a35050505050565b600054610100900460ff1615808015611a9b5750600054600160ff909116105b80611ab55750303b158015611ab5575060005460ff166001145b611b185760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610baa565b6000805460ff191660011790558015611b3b576000805461ff0019166101001790555b611b436137d1565b60c955611b508686613868565b611b598761324d565b611b628461329f565b611b6b83612f0b565b611b74826136cb565b8015611bba576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050565b60665460069060409081161415611bec5760405162461bcd60e51b8152600401610baa90614b3c565b336001600160a01b03851614611c755760405162461bcd60e51b815260206004820152604260248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72415660448201527f5353706c69743a2063616c6c6572206973206e6f7420746865206f706572617460648201526137b960f11b608482015260a401610baa565b61271061ffff83161115611cfd5760405162461bcd60e51b815260206004820152604360248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72415660448201527f5353706c69743a2073706c6974206d757374206265203c3d203130303030206260648201526269707360e81b608482015260a401610baa565b60cb54600090611d1a90600160a01b900463ffffffff1642614e44565b6001600160a01b03868116600090815260d46020908152604080832093891683529281528282208351606081018552905461ffff80821683526201000082041692820192909252600160201b90910463ffffffff1692810192909252919250611d8290612ebd565b6001600160a01b03808816600090815260d460209081526040808320938a16835292905220909150611db5908584613736565b6040805163ffffffff8416815261ffff838116602083015286168183015290516001600160a01b03878116929089169133917f48e198b6ae357e529204ee53a8e514c470ff77d9cc8e4f7207f8b5d490ae6934919081900360600190a4505050505050565b60408051608081018252600080825260208201819052918101829052606081019190915260ca8281548110611e5157611e51614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff161515606082015292915050565b6001600160a01b03828116600090815260d46020908152604080832093851683529281528282208351606081018552905461ffff80821683526201000082041692820192909252600160201b90910463ffffffff169281019290925290611f1290612ebd565b9392505050565b60ca546000905b63ffffffff811615611f85578260ca611f3a60018461503f565b63ffffffff1681548110611f5057611f50614a43565b9060005260206000209060020201600001541415611f7357611f1260018261503f565b80611f7d81615064565b915050611f20565b5060405162461bcd60e51b815260206004820152603760248201527f52657761726473436f6f7264696e61746f722e676574526f6f74496e6465784660448201527f726f6d486173683a20726f6f74206e6f7420666f756e640000000000000000006064820152608401610baa565b611ffc612778565b6001600160a01b0381166120615760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610baa565b610bbc8161324d565b60007f000000000000000000000000000000000000000000000000000000000000000046141561209b575060c95490565b6120a36137d1565b905090565b60006001610d0e60208401846145b1565b606654600390600890811614156120e25760405162461bcd60e51b8152600401610baa90614b3c565b60cb546001600160a01b0316331461210c5760405162461bcd60e51b8152600401610baa90614df0565b60ca5463ffffffff83161061217d5760405162461bcd60e51b815260206004820152603160248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152700d2dcecc2d8d2c840e4dedee892dcc8caf607b1b6064820152608401610baa565b600060ca8363ffffffff168154811061219857612198614a43565b906000526020600020906002020190508060010160089054906101000a900460ff16156122255760405162461bcd60e51b815260206004820152603560248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152741c9bdbdd08185b1c9958591e48191a5cd8589b1959605a1b6064820152608401610baa565b6001810154600160201b900463ffffffff1642106122a45760405162461bcd60e51b815260206004820152603660248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152751c9bdbdd08185b1c9958591e481858dd1a5d985d195960521b6064820152608401610baa565b60018101805460ff60401b1916600160401b17905560405163ffffffff8416907fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e90600090a2505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612342573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123669190614a70565b6001600160a01b0316336001600160a01b0316146123965760405162461bcd60e51b8152600401610baa90614a8d565b6066541981196066541916146124145760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610baa565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610cf3565b606654600090600190811614156124745760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156124975760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f3357368484838181106124ba576124ba614a43565b90506020028101906124cc9190614c21565b33600081815260ce602090815260408083205490519495509391926124f79290918591879101614d7a565b604051602081830303815290604052805190602001209050612518836128c9565b33600090815260cf602090815260408083208484529091529020805460ff1916600190811790915561254b908390614daa565b33600081815260ce602052604090819020929092559051829184917f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e628190612593908890614dc2565b60405180910390a46125b4333060408601803590610f0c90602089016145b1565b50505080806125c290614dd5565b91505061249f565b606654600490601090811614156125f35760405162461bcd60e51b8152600401610baa90614b3c565b33600090815260d1602052604090205460ff166126225760405162461bcd60e51b8152600401610baa90614b73565b600260975414156126455760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f33573684848381811061266857612668614a43565b905060200281019061267a9190614c21565b33600081815260ce602090815260408083205490519495509391926126a59290918591879101614d7a565b6040516020818303038152906040528051906020012090506126c6836128c9565b33600090815260d2602090815260408083208484529091529020805460ff191660019081179091556126f9908390614daa565b33600081815260ce602052604090819020929092559051829184917f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b90612741908890614dc2565b60405180910390a4612762333060408601803590610f0c90602089016145b1565b505050808061277090614dd5565b91505061264d565b6033546001600160a01b031633146114ee5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610baa565b6001600160a01b0381166128605760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610baa565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6128fb6128d68280615084565b6128e660808501606086016147e1565b6128f660a08601608087016147e1565b613952565b600081604001351161297f5760405162461bcd60e51b815260206004820152604160248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20616d6f756e742063616e6e6f74206265206064820152600360fc1b608482015260a401610baa565b6f4b3b4ca85a86c47a098a223fffffffff81604001351115612a095760405162461bcd60e51b815260206004820152603f60248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20616d6f756e7420746f6f206c61726765006064820152608401610baa565b612a3963ffffffff7f00000000000000000000000000000000000000000000000000000000000000001642614daa565b612a4960808301606084016147e1565b63ffffffff161115610bbc5760405162461bcd60e51b815260206004820152605360248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20737461727454696d657374616d7020746f6064820152726f2066617220696e207468652066757475726560681b608482015260a401610baa565b6040516001600160a01b0380851660248301528316604482015260648101829052612b4a9085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152613f41565b50505050565b600060ca612b6160208501856147e1565b63ffffffff1681548110612b7757612b77614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff16151560608201529050612bd88382612f7c565b6000612bea60808501606086016145b1565b6001600160a01b03808216600090815260cc60205260409020549192501680612c105750805b336001600160a01b03821614612c8e5760405162461bcd60e51b815260206004820152603c60248201527f52657761726473436f6f7264696e61746f722e70726f63657373436c61696d3a60448201527f2063616c6c6572206973206e6f742076616c696420636c61696d6572000000006064820152608401610baa565b60005b612c9e60a08701876150ce565b9050811015612eb55736612cb560e0880188615084565b83818110612cc557612cc5614a43565b6001600160a01b038716600090815260cd602090815260408083209302949094019450929091508290612cfa908501856145b1565b6001600160a01b03166001600160a01b0316815260200190815260200160002054905080826020013511612db45760405162461bcd60e51b815260206004820152605560248201527f52657761726473436f6f7264696e61746f722e70726f63657373436c61696d3a60448201527f2063756d756c61746976654561726e696e6773206d75737420626520677420746064820152741a185b8818dd5b5d5b185d1a5d9950db185a5b5959605a1b608482015260a401610baa565b6000612dc4826020850135614a2c565b6001600160a01b038716600090815260cd60209081526040822092935085018035929190612df290876145b1565b6001600160a01b0316815260208082019290925260400160002091909155612e349089908390612e24908701876145b1565b6001600160a01b03169190614013565b86516001600160a01b03808a1691878216918916907f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce3190612e7860208901896145b1565b604080519283526001600160a01b039091166020830152810186905260600160405180910390a45050508080612ead90614dd5565b915050612c91565b505050505050565b6000816040015163ffffffff1660001415612ee557505060cb54600160e01b900461ffff1690565b816040015163ffffffff16421015612efe578151611371565b506020015190565b919050565b60cb546040805163ffffffff600160a01b9093048316815291831660208301527faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b3910160405180910390a160cb805463ffffffff909216600160a01b0263ffffffff60a01b19909216919091179055565b806060015115612fd55760405162461bcd60e51b8152602060048201526030602482015260008051602061523283398151915260448201526f1c9bdbdd081a5cc8191a5cd8589b195960821b6064820152608401610baa565b806040015163ffffffff1642101561303c5760405162461bcd60e51b815260206004820152603660248201526000805160206152328339815191526044820152751c9bdbdd081b9bdd081858dd1a5d985d1959081e595d60521b6064820152608401610baa565b61304960c08301836150ce565b905061305860a08401846150ce565b9050146130d05760405162461bcd60e51b815260206004820152604c602482015260008051602061523283398151915260448201527f746f6b656e496e646963657320616e6420746f6b656e50726f6f6673206c656e60648201526b0cee8d040dad2e6dac2e8c6d60a31b608482015260a401610baa565b6130dd60e0830183615084565b90506130ec60c08401846150ce565b9050146131625760405162461bcd60e51b815260206004820152604a602482015260008051602061523283398151915260448201527f746f6b656e5472656550726f6f667320616e64206c6561766573206c656e67746064820152690d040dad2e6dac2e8c6d60b31b608482015260a401610baa565b805161318e9061317860408501602086016147e1565b6131856040860186615118565b86606001614043565b60005b61319e60a08401846150ce565b90508110156132485761323860808401356131bc60a08601866150ce565b848181106131cc576131cc614a43565b90506020020160208101906131e191906147e1565b6131ee60c08701876150ce565b858181106131fe576131fe614a43565b90506020028101906132109190615118565b61321d60e0890189615084565b8781811061322d5761322d614a43565b9050604002016141af565b61324181614dd5565b9050613191565b505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60cb546040516001600160a01b038084169216907f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb90600090a360cb80546001600160a01b0319166001600160a01b0392909216919091179055565b600061332a61330a8380615084565b61331a60808601606087016147e1565b6128f660a08701608088016147e1565b60006133396040840184615084565b9050116133b95760405162461bcd60e51b8152602060048201526054602482015260008051602061527283398151915260448201527f61746f724469726563746564526577617264735375626d697373696f6e3a206e6064820152731bc81bdc195c985d1bdc9cc81c995dd85c99195960621b608482015260a401610baa565b60008060005b6133cc6040860186615084565b90508110156135fe57366133e36040870187615084565b838181106133f3576133f3614a43565b6040029190910191506000905061340d60208301836145b1565b6001600160a01b0316141561348c5760405162461bcd60e51b815260206004820152605b6024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722063616e6e6f74206265203020616464726573730000000000608482015260a401610baa565b61349960208201826145b1565b6001600160a01b0316836001600160a01b0316106135475760405162461bcd60e51b81526020600482015260786024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f7273206d75737420626520696e20617363656e64696e67206f7260848201527f64657220746f2068616e646c65206475706c696361746573000000000000000060a482015260c401610baa565b61355460208201826145b1565b925060008160200135116135dc5760405162461bcd60e51b81526020600482015260616024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722072657761726420616d6f756e742063616e6e6f74206265206084820152600360fc1b60a482015260c401610baa565b6135ea602082013585614daa565b935050806135f790614dd5565b90506133bf565b504261361060a08601608087016147e1565b61362060808701606088016147e1565b61362a9190614e44565b63ffffffff16106136c45760405162461bcd60e51b81526020600482015260766024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722d64697265637465642072657761726473207375626d697373608482015275696f6e206973206e6f7420726574726f61637469766560501b60a482015260c401610baa565b5092915050565b60cb546040805161ffff600160e01b9093048316815291831660208301527fe6cd4edfdcc1f6d130ab35f73d72378f3a642944fb4ee5bd84b7807a81ea1c4e910160405180910390a160cb805461ffff909216600160e01b0261ffff60e01b19909216919091179055565b8254600160201b900463ffffffff164210613795578254600160201b900463ffffffff1661377e5760cb548354600160e01b90910461ffff1661ffff19909116178355613795565b825462010000810461ffff1661ffff199091161783555b825463ffffffff909116600160201b0267ffffffff000000001961ffff90931662010000029290921667ffffffffffff00001990911617179055565b604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b6065546001600160a01b031615801561388957506001600160a01b03821615155b61390b5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610baa565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261394e826127d2565b5050565b826139c25760405162461bcd60e51b8152602060048201526046602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206e6f207374726174656769606482015265195cc81cd95d60d21b608482015260a401610baa565b7f000000000000000000000000000000000000000000000000000000000000000063ffffffff168163ffffffff161115613a785760405162461bcd60e51b815260206004820152605a602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206475726174696f6e20657860648201527f6365656473204d41585f524557415244535f4455524154494f4e000000000000608482015260a401610baa565b613aa27f000000000000000000000000000000000000000000000000000000000000000082615175565b63ffffffff1615613b485760405162461bcd60e51b8152602060048201526070602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206475726174696f6e206d7560648201527f73742062652061206d756c7469706c65206f662043414c43554c4154494f4e5f60848201526f494e54455256414c5f5345434f4e445360801b60a482015260c401610baa565b613b727f000000000000000000000000000000000000000000000000000000000000000083615175565b63ffffffff1615613c1e5760405162461bcd60e51b8152602060048201526076602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737461727454696d65737460648201527f616d70206d7573742062652061206d756c7469706c65206f662043414c43554c6084820152754154494f4e5f494e54455256414c5f5345434f4e445360501b60a482015260c401610baa565b8163ffffffff167f000000000000000000000000000000000000000000000000000000000000000063ffffffff1642613c579190614a2c565b11158015613c9157508163ffffffff167f000000000000000000000000000000000000000000000000000000000000000063ffffffff1611155b613d175760405162461bcd60e51b8152602060048201526057602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737461727454696d65737460648201527f616d7020746f6f2066617220696e207468652070617374000000000000000000608482015260a401610baa565b6000805b84811015612eb5576000868683818110613d3757613d37614a43565b613d4d92602060409092020190810191506145b1565b60405163198f077960e21b81526001600160a01b0380831660048301529192507f00000000000000000000000000000000000000000000000000000000000000009091169063663c1de490602401602060405180830381865afa158015613db8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613ddc9190614ad7565b80613e0357506001600160a01b03811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0145b613e7c5760405162461bcd60e51b8152602060048201526050602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20696e76616c69642073747260648201526f185d1959de4818dbdb9cda59195c995960821b608482015260a401610baa565b806001600160a01b0316836001600160a01b031610613f2f5760405162461bcd60e51b815260206004820152606f602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737472617465676965732060648201527f6d75737420626520696e20617363656e64696e67206f7264657220746f20686160848201526e6e646c65206475706c69636174657360881b60a482015260c401610baa565b9150613f3a81614dd5565b9050613d1b565b6000613f96826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166143009092919063ffffffff16565b8051909150156132485780806020019051810190613fb49190614ad7565b6132485760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610baa565b6040516001600160a01b03831660248201526044810182905261324890849063a9059cbb60e01b90606401612b13565b61404e602083615198565b6001901b8463ffffffff16106140d85760405162461bcd60e51b815260206004820152604360248201527f52657761726473436f6f7264696e61746f722e5f7665726966794561726e657260448201527f436c61696d50726f6f663a20696e76616c6964206561726e65724c656166496e6064820152620c8caf60eb1b608482015260a401610baa565b60006140e382610cfe565b905061412e84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508a92508591505063ffffffff8916614317565b612eb55760405162461bcd60e51b815260206004820152604660248201527f52657761726473436f6f7264696e61746f722e5f7665726966794561726e657260448201527f436c61696d50726f6f663a20696e76616c6964206561726e657220636c61696d60648201526510383937b7b360d11b608482015260a401610baa565b6141ba602083615198565b6001901b8463ffffffff16106142385760405162461bcd60e51b815260206004820152603c60248201527f52657761726473436f6f7264696e61746f722e5f766572696679546f6b656e4360448201527f6c61696d3a20696e76616c696420746f6b656e4c656166496e646578000000006064820152608401610baa565b6000614243826120a8565b905061428e84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508a92508591505063ffffffff8916614317565b612eb55760405162461bcd60e51b815260206004820152603f60248201527f52657761726473436f6f7264696e61746f722e5f766572696679546f6b656e4360448201527f6c61696d3a20696e76616c696420746f6b656e20636c61696d2070726f6f66006064820152608401610baa565b606061430f848460008561432f565b949350505050565b600083614325868585614460565b1495945050505050565b6060824710156143905760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610baa565b6001600160a01b0385163b6143e75760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610baa565b600080866001600160a01b0316858760405161440391906151d8565b60006040518083038185875af1925050503d8060008114614440576040519150601f19603f3d011682016040523d82523d6000602084013e614445565b606091505b5091509150614455828286614563565b979650505050505050565b60006020845161447091906151ea565b156144f75760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a401610baa565b8260205b8551811161455a5761450e6002856151ea565b61452f57816000528086015160205260406000209150600284049350614548565b8086015160005281602052604060002091506002840493505b614553602082614daa565b90506144fb565b50949350505050565b60608315614572575081611f12565b8251156145825782518084602001fd5b8160405162461bcd60e51b8152600401610baa91906151fe565b6001600160a01b0381168114610bbc57600080fd5b6000602082840312156145c357600080fd5b8135611f128161459c565b8015158114610bbc57600080fd5b600080604083850312156145ef57600080fd5b82356145fa8161459c565b9150602083013561460a816145ce565b809150509250929050565b60006020828403121561462757600080fd5b5035919050565b60006040828403121561464057600080fd5b50919050565b60006040828403121561465857600080fd5b611f12838361462e565b60008083601f84011261467457600080fd5b50813567ffffffffffffffff81111561468c57600080fd5b6020830191508360208260051b85010111156146a757600080fd5b9250929050565b600080602083850312156146c157600080fd5b823567ffffffffffffffff8111156146d857600080fd5b6146e485828601614662565b90969095509350505050565b6000610100828403121561464057600080fd5b6000806040838503121561471657600080fd5b823567ffffffffffffffff81111561472d57600080fd5b614739858286016146f0565b925050602083013561460a8161459c565b803563ffffffff81168114612f0657600080fd5b6000806040838503121561477157600080fd5b823591506147816020840161474a565b90509250929050565b60008060006040848603121561479f57600080fd5b833567ffffffffffffffff8111156147b657600080fd5b6147c286828701614662565b90945092505060208401356147d68161459c565b809150509250925092565b6000602082840312156147f357600080fd5b611f128261474a565b60006020828403121561480e57600080fd5b813560ff81168114611f1257600080fd5b60006020828403121561483157600080fd5b813567ffffffffffffffff81111561484857600080fd5b61430f848285016146f0565b6000806040838503121561486757600080fd5b82356148728161459c565b946020939093013593505050565b6000806040838503121561489357600080fd5b823561489e8161459c565b9150602083013561460a8161459c565b6000806000604084860312156148c357600080fd5b83356148ce8161459c565b9250602084013567ffffffffffffffff8111156148ea57600080fd5b6148f686828701614662565b9497909650939450505050565b803561ffff81168114612f0657600080fd5b60006020828403121561492757600080fd5b611f1282614903565b6000806040838503121561494357600080fd5b823561494e8161459c565b915061478160208401614903565b60008060008060008060c0878903121561497557600080fd5b86356149808161459c565b955060208701356149908161459c565b94506040870135935060608701356149a78161459c565b92506149b56080880161474a565b91506149c360a08801614903565b90509295509295509295565b6000806000606084860312156149e457600080fd5b83356149ef8161459c565b925060208401356149ff8161459c565b9150614a0d60408501614903565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b600082821015614a3e57614a3e614a16565b500390565b634e487b7160e01b600052603260045260246000fd5b600081614a6857614a68614a16565b506000190190565b600060208284031215614a8257600080fd5b8151611f128161459c565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215614ae957600080fd5b8151611f12816145ce565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b60208082526019908201527f5061757361626c653a20696e6465782069732070617573656400000000000000604082015260600190565b60208082526051908201527f52657761726473436f6f7264696e61746f723a2063616c6c6572206973206e6f60408201527f7420612076616c69642063726561746552657761726473466f72416c6c53756260608201527036b4b9b9b4b7b71039bab136b4ba3a32b960791b608082015260a00190565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b60008235609e19833603018112614c3757600080fd5b9190910192915050565b6000808335601e19843603018112614c5857600080fd5b830160208101925035905067ffffffffffffffff811115614c7857600080fd5b8060061b36038313156146a757600080fd5b818352600060208085019450826000805b86811015614cef578235614cae8161459c565b6001600160a01b03168852828401356bffffffffffffffffffffffff8116808214614cd7578384fd5b89860152506040978801979290920191600101614c9b565b50959695505050505050565b6000614d078283614c41565b60a08552614d1960a086018284614c8a565b9150506020830135614d2a8161459c565b6001600160a01b0316602085015260408381013590850152614d4e6060840161474a565b63ffffffff808216606087015280614d686080870161474a565b16608087015250508091505092915050565b60018060a01b0384168152826020820152606060408201526000614da16060830184614cfb565b95945050505050565b60008219821115614dbd57614dbd614a16565b500190565b602081526000611f126020830184614cfb565b6000600019821415614de957614de9614a16565b5060010190565b60208082526034908201527f52657761726473436f6f7264696e61746f723a2063616c6c6572206973206e6f6040820152733a103a3432903932bbb0b93239aab83230ba32b960611b606082015260800190565b600063ffffffff808316818516808303821115614e6357614e63614a16565b01949350505050565b6000823560fe19833603018112614c3757600080fd5b6000823560be19833603018112614c3757600080fd5b6000808335601e19843603018112614eaf57600080fd5b830160208101925035905067ffffffffffffffff811115614ecf57600080fd5b8036038313156146a757600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000614f138283614c41565b60c08552614f2560c086018284614c8a565b915050602080840135614f378161459c565b6001600160a01b0390811686830152604090614f5586830187614c41565b888603848a015280865290946000919085015b81831015614f99578635614f7b8161459c565b84168152868601358682015295840195600192909201918401614f68565b614fa560608a0161474a565b63ffffffff811660608c01529650614fbf60808a0161474a565b63ffffffff811660808c01529650614fda60a08a018a614e98565b9750955089810360a08b0152614ff1818888614ede565b9a9950505050505050505050565b60018060a01b0384168152826020820152606060408201526000614da16060830184614f07565b82815260406020820152600061430f6040830184614f07565b600063ffffffff8381169083168181101561505c5761505c614a16565b039392505050565b600063ffffffff82168061507a5761507a614a16565b6000190192915050565b6000808335601e1984360301811261509b57600080fd5b83018035915067ffffffffffffffff8211156150b657600080fd5b6020019150600681901b36038213156146a757600080fd5b6000808335601e198436030181126150e557600080fd5b83018035915067ffffffffffffffff82111561510057600080fd5b6020019150600581901b36038213156146a757600080fd5b6000808335601e1984360301811261512f57600080fd5b83018035915067ffffffffffffffff82111561514a57600080fd5b6020019150368190038213156146a757600080fd5b634e487b7160e01b600052601260045260246000fd5b600063ffffffff8084168061518c5761518c61515f565b92169190910692915050565b6000826151a7576151a761515f565b500490565b60005b838110156151c75781810151838201526020016151af565b83811115612b4a5750506000910152565b60008251614c378184602087016151ac565b6000826151f9576151f961515f565b500690565b602081526000825180602084015261521d8160408501602087016151ac565b601f01601f1916919091016040019291505056fe52657761726473436f6f7264696e61746f722e5f636865636b436c61696d3a2061746f724469726563746564526577617264735375626d697373696f6e3a206f52657761726473436f6f7264696e61746f722e5f76616c69646174654f70657252657761726473436f6f7264696e61746f722e5f76616c6964617465436f6d6da2646970667358221220081e7033322140012f1d9ef09b55b455e888c99c5a08a478635131f15ff9795b64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x80`@R4\x80\x15b\0\0\x12W`\0\x80\xFD[P`@Qb\0W.8\x03\x80b\0W.\x839\x81\x01`@\x81\x90Rb\0\x005\x91b\0\x02\xE4V[\x86\x86\x86\x86\x86\x86\x86b\0\0H\x85\x82b\0\x03~V[c\xFF\xFF\xFF\xFF\x16\x15b\0\0\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R```$\x82\x01R\x7FRewardsCoordinator: GENESIS_REWA`D\x82\x01R\x7FRDS_TIMESTAMP must be a multiple`d\x82\x01R\x7F of CALCULATION_INTERVAL_SECONDS`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[b\0\0\xFCb\x01Q\x80\x86b\0\x03~V[c\xFF\xFF\xFF\xFF\x16\x15b\0\x01\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R\x7FRewardsCoordinator: CALCULATION_`D\x82\x01R\x7FINTERVAL_SECONDS must be a multi`d\x82\x01R\x7Fple of SNAPSHOT_CADENCE\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01b\0\0\xE4V[`\x01`\x01`\xA0\x1B\x03\x96\x87\x16a\x01 R\x94\x90\x95\x16a\x01@Rc\xFF\xFF\xFF\xFF\x92\x83\x16`\x80R\x90\x82\x16`\xA0R\x81\x16`\xC0R\x91\x82\x16`\xE0R\x16a\x01\0Rb\0\x01\xDFb\0\x01\xF2V[PPFa\x01`RPb\0\x03\xB0\x93PPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x02\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01b\0\0\xE4V[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x02\xAFW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x02\xC7W`\0\x80\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14b\0\x02\xDFW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15b\0\x03\0W`\0\x80\xFD[\x87Qb\0\x03\r\x81b\0\x02\xB1V[` \x89\x01Q\x90\x97Pb\0\x03 \x81b\0\x02\xB1V[\x95Pb\0\x030`@\x89\x01b\0\x02\xCAV[\x94Pb\0\x03@``\x89\x01b\0\x02\xCAV[\x93Pb\0\x03P`\x80\x89\x01b\0\x02\xCAV[\x92Pb\0\x03``\xA0\x89\x01b\0\x02\xCAV[\x91Pb\0\x03p`\xC0\x89\x01b\0\x02\xCAV[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80b\0\x03\xA4WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qa\x01 Qa\x01@Qa\x01`QaR\xE7b\0\x04G`\09`\0a n\x01R`\0\x81\x81a\x05F\x01Ra=o\x01R`\0a\x08\xA0\x01R`\0\x81\x81a\x04p\x01Ra<i\x01R`\0\x81\x81a\x03\xC4\x01Ra*\x13\x01R`\0\x81\x81a\x05\x1F\x01Ra<'\x01R`\0\x81\x81a\x07\xEC\x01Ra9\xC4\x01R`\0\x81\x81a\x07>\x01R\x81\x81a:}\x01Ra;M\x01RaR\xE7`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x82W`\x005`\xE0\x1C\x80c\x86\\iS\x11a\x01\xDEW\x80c\xD4T\nU\x11a\x01\x0FW\x80c\xF2\xFD\xE3\x8B\x11a\0\xADW\x80c\xFA\xBC\x1C\xBC\x11a\0|W\x80c\xFA\xBC\x1C\xBC\x14a\t1W\x80c\xFB\xF1\xE2\xC1\x14a\tDW\x80c\xFC\xE3l}\x14a\tWW\x80c\xFF\x9Fl\xCE\x14a\tjW`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x08\xF0W\x80c\xF6\x98\xDA%\x14a\t\x03W\x80c\xF8\xCD\x84H\x14a\t\x0BW\x80c\xF9j\xBF.\x14a\t\x1EW`\0\x80\xFD[\x80c\xE0c\xF8\x1F\x11a\0\xE9W\x80c\xE0c\xF8\x1F\x14a\x08uW\x80c\xE8\x10\xCE!\x14a\x08\x88W\x80c\xEAM<\x9B\x14a\x08\x9BW\x80c\xEDq\xE6\xA2\x14a\x08\xC2W`\0\x80\xFD[\x80c\xD4T\nU\x14a\x08<W\x80c\xDC\xBB\x03\xB3\x14a\x08OW\x80c\xDE\x02\xE5\x03\x14a\x08bW`\0\x80\xFD[\x80c\xA0\x16\x9D\xDD\x11a\x01|W\x80c\xB3\xDB\xB0\xE0\x11a\x01VW\x80c\xB3\xDB\xB0\xE0\x14a\x07\xB4W\x80c\xBB~E\x1F\x14a\x07\xC7W\x80c\xBF!\xA8\xAA\x14a\x07\xE7W\x80c\xC4m\xB6\x06\x14a\x08\x0EW`\0\x80\xFD[\x80c\xA0\x16\x9D\xDD\x14a\x07`W\x80c\xA5\n\x1D\x9C\x14a\x07sW\x80c\xAE\xBD\x8B\xAE\x14a\x07\x86W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x11a\x01\xB8W\x80c\x91\x04\xC3\x19\x14a\x07\x03W\x80c\x9B\xE3\xD4\xE4\x14a\x07\x1EW\x80c\x9C\xB9\xA5\xFA\x14a\x07&W\x80c\x9DE\xC2\x81\x14a\x079W`\0\x80\xFD[\x80c\x86\\iS\x14a\x06\xB4W\x80c\x88o\x11\x95\x14a\x06\xDFW\x80c\x8D\xA5\xCB[\x14a\x06\xF2W`\0\x80\xFD[\x80c>\xFE\x1D\xB6\x11a\x02\xB8W\x80c\\\x97Z\xBB\x11a\x02VW\x80cm!\x11~\x11a\x020W\x80cm!\x11~\x14a\x06cW\x80cqP\x18\xA6\x14a\x06\x91W\x80c{\x8F\x8B\x05\x14a\x06\x99W\x80c\x86<\xB9\xA9\x14a\x06\xA1W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x063W\x80c^\x9D\x83H\x14a\x06;W\x80cc\xF6\xA7\x98\x14a\x06NW`\0\x80\xFD[\x80cM\x18\xCC5\x11a\x02\x92W\x80cM\x18\xCC5\x14a\x05\xDEW\x80cX\xBA\xAA>\x14a\x05\xF5W\x80cY\\jg\x14a\x06\x08W\x80cZ\xC8j\xB7\x14a\x06\x10W`\0\x80\xFD[\x80c>\xFE\x1D\xB6\x14a\x05\x92W\x80cE\x96\x02\x1C\x14a\x05\xA5W\x80cK\x949`\x14a\x05\xB8W`\0\x80\xFD[\x80c\x14\x9B\xC8r\x11a\x03%W\x80c7\x83\x8E\xD0\x11a\x02\xFFW\x80c7\x83\x8E\xD0\x14a\x05\x1AW\x80c9\xB7\x0E8\x14a\x05AW\x80c:\x8C\x07\x86\x14a\x05hW\x80c<\xCC\x86\x1D\x14a\x05\x7FW`\0\x80\xFD[\x80c\x14\x9B\xC8r\x14a\x04\xA5W\x80c+\x9Fd\xA4\x14a\x04\xC6W\x80c6\xAFA\xFA\x14a\x05\x07W`\0\x80\xFD[\x80c\x0E\xB3\x83E\x11a\x03aW\x80c\x0E\xB3\x83E\x14a\x04CW\x80c\x10\xD6z/\x14a\x04XW\x80c\x13\x143\xB4\x14a\x04kW\x80c\x13d9\xDD\x14a\x04\x92W`\0\x80\xFD[\x80b\x18W,\x14a\x03\x87W\x80c\x04\xA0\xC5\x02\x14a\x03\xBFW\x80c\x0E\x9AS\xCF\x14a\x03\xFBW[`\0\x80\xFD[a\x03\xAAa\x03\x956`\x04aE\xB1V[`\xD1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xB6V[a\x04\x03a\t}V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x83\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R``\x91\x82\x01Q\x15\x15\x91\x81\x01\x91\x90\x91R`\x80\x01a\x03\xB6V[a\x04Va\x04Q6`\x04aE\xDCV[a\n\x81V[\0[a\x04Va\x04f6`\x04aE\xB1V[a\x0B\x03V[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x04\xA06`\x04aF\x15V[a\x0B\xBFV[a\x04\xB8a\x04\xB36`\x04aFFV[a\x0C\xFEV[`@Q\x90\x81R` \x01a\x03\xB6V[a\x04\xEFa\x04\xD46`\x04aE\xB1V[`\xCC` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xB6V[a\x04Va\x05\x156`\x04aF\xAEV[a\rtV[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\xCBTa\x03\xE6\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04Va\x05\x8D6`\x04aG\x03V[a\x0F>V[a\x04Va\x05\xA06`\x04aG^V[a\x0F\xA3V[a\x04Va\x05\xB36`\x04aG\x8AV[a\x12tV[a\x05\xCBa\x05\xC66`\x04aE\xB1V[a\x13\x1BV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xB6V[`\xCBTa\x03\xE6\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04Va\x06\x036`\x04aG\xE1V[a\x13wV[a\x04Va\x13\x88V[a\x03\xAAa\x06\x1E6`\x04aG\xFCV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x04\xB8V[a\x03\xAAa\x06I6`\x04aH\x1FV[a\x14OV[`\xCBTa\x05\xCB\x90`\x01`\xE0\x1B\x90\x04a\xFF\xFF\x16\x81V[a\x03\xAAa\x06q6`\x04aHTV[`\xCF` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x14\xDCV[`\xCATa\x04\xB8V[a\x04Va\x06\xAF6`\x04aE\xB1V[a\x14\xF0V[a\x04\xB8a\x06\xC26`\x04aH\x80V[`\xCD` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`eTa\x04\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xEFV[a\x04\xEFs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x03a\x15\x01V[a\x04Va\x0746`\x04aH\xAEV[a\x15\x9FV[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x07n6`\x04aE\xB1V[a\x17\xD8V[a\x04Va\x07\x816`\x04aI\x15V[a\x187V[a\x03\xAAa\x07\x946`\x04aHTV[`\xD2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x07\xC26`\x04aI0V[a\x18HV[a\x04\xB8a\x07\xD56`\x04aE\xB1V[`\xCE` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xAAa\x08\x1C6`\x04aHTV[`\xD0` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x08J6`\x04aI\\V[a\x1A{V[a\x04Va\x08]6`\x04aI\xCFV[a\x1B\xC3V[a\x04\x03a\x08p6`\x04aF\x15V[a\x1E\x1AV[a\x05\xCBa\x08\x836`\x04aH\x80V[a\x1E\xACV[a\x03\xE6a\x08\x966`\x04aF\x15V[a\x1F\x19V[a\x04\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xAAa\x08\xD06`\x04aHTV[`\xD3` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x08\xFE6`\x04aE\xB1V[a\x1F\xF4V[a\x04\xB8a jV[a\x04\xB8a\t\x196`\x04aFFV[a \xA8V[a\x04Va\t,6`\x04aG\xE1V[a \xB9V[a\x04Va\t?6`\x04aF\x15V[a\"\xEFV[`\xCBTa\x04\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Va\te6`\x04aF\xAEV[a$KV[a\x04Va\tx6`\x04aF\xAEV[a%\xCAV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCAT[\x80\x15a\nXW`\0`\xCAa\t\xBA`\x01\x84aJ,V[\x81T\x81\x10a\t\xCAWa\t\xCAaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x80\x15``\x83\x01\x81\x90R\x91\x92P\x90a\n:WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\nEW\x92\x91PPV[P\x80a\nP\x81aJYV[\x91PPa\t\xA5V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x90V[a\n\x89a'xV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xD1` R`@\x80\x82 T\x90Q`\xFF\x90\x91\x16\x92\x84\x15\x15\x92\x84\x15\x15\x92\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C\x91\x90\xA4P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xD1` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bz\x91\x90aJpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\x8DV[`@Q\x80\x91\x03\x90\xFD[a\x0B\xBC\x81a'\xD2V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C+\x91\x90aJ\xD7V[a\x0CGW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\xF4V[`fT\x81\x81\x16\x14a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80a\r\x0E` \x84\x01\x84aE\xB1V[\x83` \x015`@Q` \x01a\rW\x93\x92\x91\x90`\xF8\x93\x90\x93\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x83R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x83\x01R`\x15\x82\x01R`5\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\r\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\0\x90\x81R`\xD1` R`@\x90 T`\xFF\x16a\r\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aKsV[`\x02`\x97T\x14\x15a\r\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a\x0E\x12Wa\x0E\x12aJCV[\x90P` \x02\x81\x01\x90a\x0E$\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a\x0EO\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0Ep\x83a(\xC9V[3`\0\x90\x81R`\xD0` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua\x0E\xA3\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x82\x90a\x0E\xEB\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a\x0F\x1D30`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a*\xDFV[PPP\x80\x80a\x0F+\x90aM\xD5V[\x91PPa\r\xF7V[PP`\x01`\x97UPPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0FgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x0F\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97Ua\x0F\x99\x83\x83a+PV[PP`\x01`\x97UPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x0F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aM\xF0V[`\xCBTc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x90\x83\x16\x11a\x10\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FRewardsCoordinator.submitRoot: n`D\x82\x01R\x7Few root must be for newer calcul`d\x82\x01Rj\x18]\x19Y\x08\x1C\x19\\\x9A[\xD9`\xAA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[B\x82c\xFF\xFF\xFF\xFF\x16\x10a\x11+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FRewardsCoordinator.submitRoot: r`D\x82\x01R\x7FewardsCalculationEndTimestamp ca`d\x82\x01Rtnnot be in the future`X\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCAT`\xCBT`\0\x90a\x11K\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`@\x80Q`\x80\x81\x01\x82R\x87\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x81R\x86\x84\x16\x85\x87\x01\x81\x81R`\0``\x88\x01\x81\x81R`\xCA\x80T`\x01\x81\x01\x82U\x92R\x97Q\x7FB\xD7&t\x97OiK_QYY2C\x11M8\xA5\xC3\x9C\x89\xD6\xB6/\xEE\x06\x1F\xF5#$\x0E\xE1`\x02\x90\x92\x02\x91\x82\x01U\x92Q\x7FB\xD7&t\x97OiK_QYY2C\x11M8\xA5\xC3\x9C\x89\xD6\xB6/\xEE\x06\x1F\xF5#$\x0E\xE2\x90\x93\x01\x80T\x91Q\x97Q\x93\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x97\x87\x16\x97\x90\x97\x02\x96\x90\x96\x17`\xFF`@\x1B\x19\x16`\x01`@\x1B\x92\x15\x15\x92\x90\x92\x02\x91\x90\x91\x17\x90\x94U`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1B\x84\x02\x17\x90U\x93Q\x92\x83R\x93\x94P\x88\x92\x90\x86\x16\x91\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91\x01`@Q\x80\x91\x03\x90\xA4PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x12\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x12\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x83\x81\x10\x15a\x13\x0FWa\x12\xFD\x85\x85\x83\x81\x81\x10a\x12\xE5Wa\x12\xE5aJCV[\x90P` \x02\x81\x01\x90a\x12\xF7\x91\x90aNlV[\x84a+PV[\x80a\x13\x07\x81aM\xD5V[\x91PPa\x12\xC8V[PP`\x01`\x97UPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xD5` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01` \x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x13q\x90a.\xBDV[\x92\x91PPV[a\x13\x7Fa'xV[a\x0B\xBC\x81a/\x0BV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF4\x91\x90aJ\xD7V[a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\xF4V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0a\x14\xD4\x82`\xCAa\x14d` \x83\x01\x83aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x14zWa\x14zaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01Ra/|V[P`\x01\x91\x90PV[a\x14\xE4a'xV[a\x14\xEE`\0a2MV[V[a\x14\xF8a'xV[a\x0B\xBC\x81a2\x9FV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCA\x80Ta\x155\x90`\x01\x90aJ,V[\x81T\x81\x10a\x15EWa\x15EaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x91\x90PV[`fT`\x05\x90` \x90\x81\x16\x14\x15a\x15\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x15\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x16\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FRewardsCoordinator.createOperato`D\x82\x01R\x7FrDirectedAVSRewardsSubmission: c`d\x82\x01Rsaller is not the AVS``\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0[\x82\x81\x10\x15a\x13\x0FW6\x84\x84\x83\x81\x81\x10a\x16\xA9Wa\x16\xA9aJCV[\x90P` \x02\x81\x01\x90a\x16\xBB\x91\x90aN\x82V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x93\x94P\x92a\x16\xED\x91\x8A\x91\x85\x91\x87\x91\x01aO\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x17\x10\x84a2\xFBV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\xD3` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90\x91Pa\x17O\x90\x84\x90aM\xAAV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x83\x91\x903\x90\x7F\xFC\x88\x88\xBF\xFDq\x1D\xA6\x0B\xC5\t+3\xF6w\xD8\x18\x96\xFE\x80\xEC\xC6w\xB8L\xFA\xB8\x18Db\xB6\xE0\x90a\x17\xA3\x90\x88\x90\x8A\x90aP&V[`@Q\x80\x91\x03\x90\xA4a\x17\xC130\x83a\x0F\x0C`@\x89\x01` \x8A\x01aE\xB1V[PPPP\x80\x80a\x17\xD0\x90aM\xD5V[\x91PPa\x16\x8EV[3`\0\x81\x81R`\xCC` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x93U\x92Q\x91\x16\x92\x83\x91\x85\x91\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x91\xA4PPPV[a\x18?a'xV[a\x0B\xBC\x81a6\xCBV[`fT`\x07\x90`\x80\x90\x81\x16\x14\x15a\x18qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x18\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FRewardsCoordinator.setOperatorPI`D\x82\x01R\x7FSplit: caller is not the operato`d\x82\x01R`9`\xF9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a'\x10a\xFF\xFF\x83\x16\x11\x15a\x19\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FRewardsCoordinator.setOperatorPI`D\x82\x01R\x7FSplit: split must be <= 10000 bi`d\x82\x01Raps`\xF0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCBT`\0\x90a\x19\x9D\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\xD5` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01` \x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x92P\x90a\x19\xF7\x90a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xD5` R`@\x90 \x90\x91Pa\x1A\x1D\x90\x85\x84a76V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81Ra\xFF\xFF\x83\x81\x16` \x83\x01R\x86\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x87\x16\x913\x91\x7F\xD1\xE0(\xBDfD\x86\xA4j\xD2`@\xE9\x99\xCD-\"\xE1\xE9\xA0\x94\xEEj\xFE\x19\xFC\xF6Fx\xF1ot\x91\x81\x90\x03``\x01\x90\xA3PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1A\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1A\xB5WP0;\x15\x80\x15a\x1A\xB5WP`\0T`\xFF\x16`\x01\x14[a\x1B\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B;W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1BCa7\xD1V[`\xC9Ua\x1BP\x86\x86a8hV[a\x1BY\x87a2MV[a\x1Bb\x84a2\x9FV[a\x1Bk\x83a/\x0BV[a\x1Bt\x82a6\xCBV[\x80\x15a\x1B\xBAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`fT`\x06\x90`@\x90\x81\x16\x14\x15a\x1B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x1CuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FRewardsCoordinator.setOperatorAV`D\x82\x01R\x7FSSplit: caller is not the operat`d\x82\x01Ra7\xB9`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a'\x10a\xFF\xFF\x83\x16\x11\x15a\x1C\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRewardsCoordinator.setOperatorAV`D\x82\x01R\x7FSSplit: split must be <= 10000 b`d\x82\x01Rbips`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCBT`\0\x90a\x1D\x1A\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x81R\x82\x82 \x83Q``\x81\x01\x85R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01` \x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x91\x92Pa\x1D\x82\x90a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x91Pa\x1D\xB5\x90\x85\x84a76V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81Ra\xFF\xFF\x83\x81\x16` \x83\x01R\x86\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x92\x90\x89\x16\x913\x91\x7FH\xE1\x98\xB6\xAE5~R\x92\x04\xEES\xA8\xE5\x14\xC4p\xFFw\xD9\xCC\x8EOr\x07\xF8\xB5\xD4\x90\xAEi4\x91\x90\x81\x90\x03``\x01\x90\xA4PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCA\x82\x81T\x81\x10a\x1EQWa\x1EQaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q``\x81\x01\x85R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01` \x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x90a\x1F\x12\x90a.\xBDV[\x93\x92PPPV[`\xCAT`\0\x90[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1F\x85W\x82`\xCAa\x1F:`\x01\x84aP?V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1FPWa\x1FPaJCV[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x14\x15a\x1FsWa\x1F\x12`\x01\x82aP?V[\x80a\x1F}\x81aPdV[\x91PPa\x1F V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRewardsCoordinator.getRootIndexF`D\x82\x01R\x7FromHash: root not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[a\x1F\xFCa'xV[`\x01`\x01`\xA0\x1B\x03\x81\x16a aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[a\x0B\xBC\x81a2MV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a \x9BWP`\xC9T\x90V[a \xA3a7\xD1V[\x90P\x90V[`\0`\x01a\r\x0E` \x84\x01\x84aE\xB1V[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a \xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aM\xF0V[`\xCATc\xFF\xFF\xFF\xFF\x83\x16\x10a!}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Rp\r-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x89-\xCC\x8C\xAF`{\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0`\xCA\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\x98Wa!\x98aJCV[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\x01\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Rt\x1C\x9B\xDB\xDD\x08\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01\x81\x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x10a\"\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Ru\x1C\x9B\xDB\xDD\x08\x18[\x1C\x99XY\x1EH\x18X\xDD\x1A]\x98]\x19Y`R\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01\x81\x01\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`@Qc\xFF\xFF\xFF\xFF\x84\x16\x90\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E\x90`\0\x90\xA2PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#f\x91\x90aJpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\x8DV[`fT\x19\x81\x19`fT\x19\x16\x14a$\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xF3V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a$\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a$\xBAWa$\xBAaJCV[\x90P` \x02\x81\x01\x90a$\xCC\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a$\xF7\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa%\x18\x83a(\xC9V[3`\0\x90\x81R`\xCF` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua%K\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x81\x90a%\x93\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a%\xB430`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[PPP\x80\x80a%\xC2\x90aM\xD5V[\x91PPa$\x9FV[`fT`\x04\x90`\x10\x90\x81\x16\x14\x15a%\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\0\x90\x81R`\xD1` R`@\x90 T`\xFF\x16a&\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aKsV[`\x02`\x97T\x14\x15a&EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a&hWa&haJCV[\x90P` \x02\x81\x01\x90a&z\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a&\xA5\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa&\xC6\x83a(\xC9V[3`\0\x90\x81R`\xD2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua&\xF9\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B\x90a'A\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a'b30`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[PPP\x80\x80a'p\x90aM\xD5V[\x91PPa&MV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xAAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a(`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a(\xFBa(\xD6\x82\x80aP\x84V[a(\xE6`\x80\x85\x01``\x86\x01aG\xE1V[a(\xF6`\xA0\x86\x01`\x80\x87\x01aG\xE1V[a9RV[`\0\x81`@\x015\x11a)\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: amount cannot be `d\x82\x01R`\x03`\xFC\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF\x81`@\x015\x11\x15a*\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: amount too large\0`d\x82\x01R`\x84\x01a\x0B\xAAV[a*9c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16BaM\xAAV[a*I`\x80\x83\x01``\x84\x01aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: startTimestamp to`d\x82\x01Rro far in the future`h\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra+J\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra?AV[PPPPV[`\0`\xCAa+a` \x85\x01\x85aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a+wWa+waJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x90Pa+\xD8\x83\x82a/|V[`\0a+\xEA`\x80\x85\x01``\x86\x01aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\xCC` R`@\x90 T\x91\x92P\x16\x80a,\x10WP\x80[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a,\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRewardsCoordinator.processClaim:`D\x82\x01R\x7F caller is not valid claimer\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0[a,\x9E`\xA0\x87\x01\x87aP\xCEV[\x90P\x81\x10\x15a.\xB5W6a,\xB5`\xE0\x88\x01\x88aP\x84V[\x83\x81\x81\x10a,\xC5Wa,\xC5aJCV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x02\x94\x90\x94\x01\x94P\x92\x90\x91P\x82\x90a,\xFA\x90\x85\x01\x85aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x80\x82` \x015\x11a-\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FRewardsCoordinator.processClaim:`D\x82\x01R\x7F cumulativeEarnings must be gt t`d\x82\x01Rt\x1A\x18[\x88\x18\xDD[][\x18]\x1A]\x99P\xDB\x18Z[YY`Z\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0a-\xC4\x82` \x85\x015aJ,V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x82 \x92\x93P\x85\x01\x805\x92\x91\x90a-\xF2\x90\x87aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x91\x90\x91Ua.4\x90\x89\x90\x83\x90a.$\x90\x87\x01\x87aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a@\x13V[\x86Q`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x90a.x` \x89\x01\x89aE\xB1V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x81\x01\x86\x90R``\x01`@Q\x80\x91\x03\x90\xA4PPP\x80\x80a.\xAD\x90aM\xD5V[\x91PPa,\x91V[PPPPPPV[`\0\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x14\x15a.\xE5WPP`\xCBT`\x01`\xE0\x1B\x90\x04a\xFF\xFF\x16\x90V[\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a.\xFEW\x81Qa\x13qV[P` \x01Q\x90V[\x91\x90PV[`\xCBT`@\x80Qc\xFF\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80``\x01Q\x15a/\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01Ro\x1C\x9B\xDB\xDD\x08\x1A\\\xC8\x19\x1A\\\xD8X\x9B\x19Y`\x82\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a0<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01Ru\x1C\x9B\xDB\xDD\x08\x1B\x9B\xDD\x08\x18X\xDD\x1A]\x98]\x19Y\x08\x1EY]`R\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[a0I`\xC0\x83\x01\x83aP\xCEV[\x90Pa0X`\xA0\x84\x01\x84aP\xCEV[\x90P\x14a0\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01R\x7FtokenIndices and tokenProofs len`d\x82\x01Rk\x0C\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`\xA3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a0\xDD`\xE0\x83\x01\x83aP\x84V[\x90Pa0\xEC`\xC0\x84\x01\x84aP\xCEV[\x90P\x14a1bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01R\x7FtokenTreeProofs and leaves lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x80Qa1\x8E\x90a1x`@\x85\x01` \x86\x01aG\xE1V[a1\x85`@\x86\x01\x86aQ\x18V[\x86``\x01a@CV[`\0[a1\x9E`\xA0\x84\x01\x84aP\xCEV[\x90P\x81\x10\x15a2HWa28`\x80\x84\x015a1\xBC`\xA0\x86\x01\x86aP\xCEV[\x84\x81\x81\x10a1\xCCWa1\xCCaJCV[\x90P` \x02\x01` \x81\x01\x90a1\xE1\x91\x90aG\xE1V[a1\xEE`\xC0\x87\x01\x87aP\xCEV[\x85\x81\x81\x10a1\xFEWa1\xFEaJCV[\x90P` \x02\x81\x01\x90a2\x10\x91\x90aQ\x18V[a2\x1D`\xE0\x89\x01\x89aP\x84V[\x87\x81\x81\x10a2-Wa2-aJCV[\x90P`@\x02\x01aA\xAFV[a2A\x81aM\xD5V[\x90Pa1\x91V[PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x16\x90\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB\x90`\0\x90\xA3`\xCB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a3*a3\n\x83\x80aP\x84V[a3\x1A`\x80\x86\x01``\x87\x01aG\xE1V[a(\xF6`\xA0\x87\x01`\x80\x88\x01aG\xE1V[`\0a39`@\x84\x01\x84aP\x84V[\x90P\x11a3\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R\x7FatorDirectedRewardsSubmission: n`d\x82\x01Rs\x1B\xC8\x1B\xDC\x19\\\x98]\x1B\xDC\x9C\xC8\x1C\x99]\xD8\\\x99\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0\x80`\0[a3\xCC`@\x86\x01\x86aP\x84V[\x90P\x81\x10\x15a5\xFEW6a3\xE3`@\x87\x01\x87aP\x84V[\x83\x81\x81\x10a3\xF3Wa3\xF3aJCV[`@\x02\x91\x90\x91\x01\x91P`\0\x90Pa4\r` \x83\x01\x83aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a4\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator cannot be 0 address\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a4\x99` \x82\x01\x82aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a5GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`x`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperators must be in ascending or`\x84\x82\x01R\x7Fder to handle duplicates\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a5T` \x82\x01\x82aE\xB1V[\x92P`\0\x81` \x015\x11a5\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator reward amount cannot be `\x84\x82\x01R`\x03`\xFC\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a5\xEA` \x82\x015\x85aM\xAAV[\x93PP\x80a5\xF7\x90aM\xD5V[\x90Pa3\xBFV[PBa6\x10`\xA0\x86\x01`\x80\x87\x01aG\xE1V[a6 `\x80\x87\x01``\x88\x01aG\xE1V[a6*\x91\x90aNDV[c\xFF\xFF\xFF\xFF\x16\x10a6\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator-directed rewards submiss`\x84\x82\x01Ruion is not retroactive`P\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[P\x92\x91PPV[`\xCBT`@\x80Qa\xFF\xFF`\x01`\xE0\x1B\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xE6\xCDN\xDF\xDC\xC1\xF6\xD10\xAB5\xF7=r7\x8F:d)D\xFBN\xE5\xBD\x84\xB7\x80z\x81\xEA\x1CN\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80Ta\xFF\xFF\x90\x92\x16`\x01`\xE0\x1B\x02a\xFF\xFF`\xE0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x10a7\x95W\x82T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a7~W`\xCBT\x83T`\x01`\xE0\x1B\x90\x91\x04a\xFF\xFF\x16a\xFF\xFF\x19\x90\x91\x16\x17\x83Ua7\x95V[\x82Tb\x01\0\0\x81\x04a\xFF\xFF\x16a\xFF\xFF\x19\x90\x91\x16\x17\x83U[\x82Tc\xFF\xFF\xFF\xFF\x90\x91\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19a\xFF\xFF\x90\x93\x16b\x01\0\0\x02\x92\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\x89WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9N\x82a'\xD2V[PPV[\x82a9\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: no strategi`d\x82\x01Re\x19\\\xC8\x1C\xD9]`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a:xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Z`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: duration ex`d\x82\x01R\x7Fceeds MAX_REWARDS_DURATION\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a:\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82aQuV[c\xFF\xFF\xFF\xFF\x16\x15a;HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`p`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: duration mu`d\x82\x01R\x7Fst be a multiple of CALCULATION_`\x84\x82\x01RoINTERVAL_SECONDS`\x80\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a;r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83aQuV[c\xFF\xFF\xFF\xFF\x16\x15a<\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: startTimest`d\x82\x01R\x7Famp must be a multiple of CALCUL`\x84\x82\x01RuATION_INTERVAL_SECONDS`P\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[\x81c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16Ba<W\x91\x90aJ,V[\x11\x15\x80\x15a<\x91WP\x81c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x11\x15[a=\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: startTimest`d\x82\x01R\x7Famp too far in the past\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0\x80[\x84\x81\x10\x15a.\xB5W`\0\x86\x86\x83\x81\x81\x10a=7Wa=7aJCV[a=M\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91PaE\xB1V[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cf<\x1D\xE4\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xDC\x91\x90aJ\xD7V[\x80a>\x03WP`\x01`\x01`\xA0\x1B\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14[a>|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: invalid str`d\x82\x01Ro\x18]\x19Y\xDEH\x18\xDB\xDB\x9C\xDAY\x19\\\x99Y`\x82\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a?/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`o`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: strategies `d\x82\x01R\x7Fmust be in ascending order to ha`\x84\x82\x01Rnndle duplicates`\x88\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[\x91Pa?:\x81aM\xD5V[\x90Pa=\x1BV[`\0a?\x96\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aC\0\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a2HW\x80\x80` \x01\x90Q\x81\x01\x90a?\xB4\x91\x90aJ\xD7V[a2HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra2H\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a+\x13V[a@N` \x83aQ\x98V[`\x01\x90\x1B\x84c\xFF\xFF\xFF\xFF\x16\x10a@\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRewardsCoordinator._verifyEarner`D\x82\x01R\x7FClaimProof: invalid earnerLeafIn`d\x82\x01Rb\x0C\x8C\xAF`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0a@\xE3\x82a\x0C\xFEV[\x90PaA.\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x89\x16aC\x17V[a.\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FRewardsCoordinator._verifyEarner`D\x82\x01R\x7FClaimProof: invalid earner claim`d\x82\x01Re\x10897\xB7\xB3`\xD1\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[aA\xBA` \x83aQ\x98V[`\x01\x90\x1B\x84c\xFF\xFF\xFF\xFF\x16\x10aB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRewardsCoordinator._verifyTokenC`D\x82\x01R\x7Flaim: invalid tokenLeafIndex\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0aBC\x82a \xA8V[\x90PaB\x8E\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x89\x16aC\x17V[a.\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FRewardsCoordinator._verifyTokenC`D\x82\x01R\x7Flaim: invalid token claim proof\0`d\x82\x01R`\x84\x01a\x0B\xAAV[``aC\x0F\x84\x84`\0\x85aC/V[\x94\x93PPPPV[`\0\x83aC%\x86\x85\x85aD`V[\x14\x95\x94PPPPPV[``\x82G\x10\x15aC\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01`\x01`\xA0\x1B\x03\x85\x16;aC\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0B\xAAV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaD\x03\x91\x90aQ\xD8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aD@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aDEV[``\x91P[P\x91P\x91PaDU\x82\x82\x86aEcV[\x97\x96PPPPPPPV[`\0` \x84QaDp\x91\x90aQ\xEAV[\x15aD\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FMerkle.processInclusionProofKecc`D\x82\x01R\x7Fak: proof length should be a mul`d\x82\x01Rj:4\xB862\x907\xB3\x10\x19\x99`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x82` [\x85Q\x81\x11aEZWaE\x0E`\x02\x85aQ\xEAV[aE/W\x81`\0R\x80\x86\x01Q` R`@`\0 \x91P`\x02\x84\x04\x93PaEHV[\x80\x86\x01Q`\0R\x81` R`@`\0 \x91P`\x02\x84\x04\x93P[aES` \x82aM\xAAV[\x90PaD\xFBV[P\x94\x93PPPPV[``\x83\x15aErWP\x81a\x1F\x12V[\x82Q\x15aE\x82W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x91\x90aQ\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xBCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aE\xC3W`\0\x80\xFD[\x815a\x1F\x12\x81aE\x9CV[\x80\x15\x15\x81\x14a\x0B\xBCW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aE\xEFW`\0\x80\xFD[\x825aE\xFA\x81aE\x9CV[\x91P` \x83\x015aF\n\x81aE\xCEV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aF'W`\0\x80\xFD[P5\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aF@W`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aFXW`\0\x80\xFD[a\x1F\x12\x83\x83aF.V[`\0\x80\x83`\x1F\x84\x01\x12aFtW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\x8CW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aF\xA7W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aF\xC1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xD8W`\0\x80\xFD[aF\xE4\x85\x82\x86\x01aFbV[\x90\x96\x90\x95P\x93PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aF@W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aG\x16W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG-W`\0\x80\xFD[aG9\x85\x82\x86\x01aF\xF0V[\x92PP` \x83\x015aF\n\x81aE\x9CV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x06W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aGqW`\0\x80\xFD[\x825\x91PaG\x81` \x84\x01aGJV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aG\x9FW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xB6W`\0\x80\xFD[aG\xC2\x86\x82\x87\x01aFbV[\x90\x94P\x92PP` \x84\x015aG\xD6\x81aE\x9CV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aG\xF3W`\0\x80\xFD[a\x1F\x12\x82aGJV[`\0` \x82\x84\x03\x12\x15aH\x0EW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x1F\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHHW`\0\x80\xFD[aC\x0F\x84\x82\x85\x01aF\xF0V[`\0\x80`@\x83\x85\x03\x12\x15aHgW`\0\x80\xFD[\x825aHr\x81aE\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15aH\x93W`\0\x80\xFD[\x825aH\x9E\x81aE\x9CV[\x91P` \x83\x015aF\n\x81aE\x9CV[`\0\x80`\0`@\x84\x86\x03\x12\x15aH\xC3W`\0\x80\xFD[\x835aH\xCE\x81aE\x9CV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xEAW`\0\x80\xFD[aH\xF6\x86\x82\x87\x01aFbV[\x94\x97\x90\x96P\x93\x94PPPPV[\x805a\xFF\xFF\x81\x16\x81\x14a/\x06W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI'W`\0\x80\xFD[a\x1F\x12\x82aI\x03V[`\0\x80`@\x83\x85\x03\x12\x15aICW`\0\x80\xFD[\x825aIN\x81aE\x9CV[\x91PaG\x81` \x84\x01aI\x03V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aIuW`\0\x80\xFD[\x865aI\x80\x81aE\x9CV[\x95P` \x87\x015aI\x90\x81aE\x9CV[\x94P`@\x87\x015\x93P``\x87\x015aI\xA7\x81aE\x9CV[\x92PaI\xB5`\x80\x88\x01aGJV[\x91PaI\xC3`\xA0\x88\x01aI\x03V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15aI\xE4W`\0\x80\xFD[\x835aI\xEF\x81aE\x9CV[\x92P` \x84\x015aI\xFF\x81aE\x9CV[\x91PaJ\r`@\x85\x01aI\x03V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15aJ>WaJ>aJ\x16V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81aJhWaJhaJ\x16V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\x82W`\0\x80\xFD[\x81Qa\x1F\x12\x81aE\x9CV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\xE9W`\0\x80\xFD[\x81Qa\x1F\x12\x81aE\xCEV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`Q\x90\x82\x01R\x7FRewardsCoordinator: caller is no`@\x82\x01R\x7Ft a valid createRewardsForAllSub``\x82\x01Rp6\xB4\xB9\xB9\xB4\xB7\xB7\x109\xBA\xB16\xB4\xBA:2\xB9`y\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aLXW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aLxW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aF\xA7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0\x80[\x86\x81\x10\x15aL\xEFW\x825aL\xAE\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x88R\x82\x84\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x82\x14aL\xD7W\x83\x84\xFD[\x89\x86\x01RP`@\x97\x88\x01\x97\x92\x90\x92\x01\x91`\x01\x01aL\x9BV[P\x95\x96\x95PPPPPPV[`\0aM\x07\x82\x83aLAV[`\xA0\x85RaM\x19`\xA0\x86\x01\x82\x84aL\x8AV[\x91PP` \x83\x015aM*\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`@\x83\x81\x015\x90\x85\x01RaMN``\x84\x01aGJV[c\xFF\xFF\xFF\xFF\x80\x82\x16``\x87\x01R\x80aMh`\x80\x87\x01aGJV[\x16`\x80\x87\x01RPP\x80\x91PP\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aM\xA1``\x83\x01\x84aL\xFBV[\x95\x94PPPPPV[`\0\x82\x19\x82\x11\x15aM\xBDWaM\xBDaJ\x16V[P\x01\x90V[` \x81R`\0a\x1F\x12` \x83\x01\x84aL\xFBV[`\0`\0\x19\x82\x14\x15aM\xE9WaM\xE9aJ\x16V[P`\x01\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FRewardsCoordinator: caller is no`@\x82\x01Rs:\x10:42\x9092\xBB\xB0\xB929\xAA\xB820\xBA2\xB9`a\x1B``\x82\x01R`\x80\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aNcWaNcaJ\x16V[\x01\x94\x93PPPPV[`\0\x825`\xFE\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[`\0\x825`\xBE\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xAFW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xCFW`\0\x80\xFD[\x806\x03\x83\x13\x15aF\xA7W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0aO\x13\x82\x83aLAV[`\xC0\x85RaO%`\xC0\x86\x01\x82\x84aL\x8AV[\x91PP` \x80\x84\x015aO7\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86\x83\x01R`@\x90aOU\x86\x83\x01\x87aLAV[\x88\x86\x03\x84\x8A\x01R\x80\x86R\x90\x94`\0\x91\x90\x85\x01[\x81\x83\x10\x15aO\x99W\x865aO{\x81aE\x9CV[\x84\x16\x81R\x86\x86\x015\x86\x82\x01R\x95\x84\x01\x95`\x01\x92\x90\x92\x01\x91\x84\x01aOhV[aO\xA5``\x8A\x01aGJV[c\xFF\xFF\xFF\xFF\x81\x16``\x8C\x01R\x96PaO\xBF`\x80\x8A\x01aGJV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x8C\x01R\x96PaO\xDA`\xA0\x8A\x01\x8AaN\x98V[\x97P\x95P\x89\x81\x03`\xA0\x8B\x01RaO\xF1\x81\x88\x88aN\xDEV[\x9A\x99PPPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aM\xA1``\x83\x01\x84aO\x07V[\x82\x81R`@` \x82\x01R`\0aC\x0F`@\x83\x01\x84aO\x07V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aP\\WaP\\aJ\x16V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x80aPzWaPzaJ\x16V[`\0\x19\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x9BW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aP\xB6W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\xE5W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\0W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ/W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQJW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aQ\x8CWaQ\x8CaQ_V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x82aQ\xA7WaQ\xA7aQ_V[P\x04\x90V[`\0[\x83\x81\x10\x15aQ\xC7W\x81\x81\x01Q\x83\x82\x01R` \x01aQ\xAFV[\x83\x81\x11\x15a+JWPP`\0\x91\x01RV[`\0\x82QaL7\x81\x84` \x87\x01aQ\xACV[`\0\x82aQ\xF9WaQ\xF9aQ_V[P\x06\x90V[` \x81R`\0\x82Q\x80` \x84\x01RaR\x1D\x81`@\x85\x01` \x87\x01aQ\xACV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFERewardsCoordinator._checkClaim: atorDirectedRewardsSubmission: oRewardsCoordinator._validateOperRewardsCoordinator._validateComm\xA2dipfsX\"\x12 \x08\x1Ep32!@\x01/\x1D\x9E\xF0\x9BU\xB4U\xE8\x88\xC9\x9CZ\x08\xA4xcQ1\xF1_\xF9y[dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106103825760003560e01c8063865c6953116101de578063d4540a551161010f578063f2fde38b116100ad578063fabc1cbc1161007c578063fabc1cbc14610931578063fbf1e2c114610944578063fce36c7d14610957578063ff9f6cce1461096a57600080fd5b8063f2fde38b146108f0578063f698da2514610903578063f8cd84481461090b578063f96abf2e1461091e57600080fd5b8063e063f81f116100e9578063e063f81f14610875578063e810ce2114610888578063ea4d3c9b1461089b578063ed71e6a2146108c257600080fd5b8063d4540a551461083c578063dcbb03b31461084f578063de02e5031461086257600080fd5b8063a0169ddd1161017c578063b3dbb0e011610156578063b3dbb0e0146107b4578063bb7e451f146107c7578063bf21a8aa146107e7578063c46db6061461080e57600080fd5b8063a0169ddd14610760578063a50a1d9c14610773578063aebd8bae1461078657600080fd5b80639104c319116101b85780639104c319146107035780639be3d4e41461071e5780639cb9a5fa146107265780639d45c2811461073957600080fd5b8063865c6953146106b4578063886f1195146106df5780638da5cb5b146106f257600080fd5b80633efe1db6116102b85780635c975abb116102565780636d21117e116102305780636d21117e14610663578063715018a6146106915780637b8f8b0514610699578063863cb9a9146106a157600080fd5b80635c975abb146106335780635e9d83481461063b57806363f6a7981461064e57600080fd5b80634d18cc35116102925780634d18cc35146105de57806358baaa3e146105f5578063595c6a67146106085780635ac86ab71461061057600080fd5b80633efe1db6146105925780634596021c146105a55780634b943960146105b857600080fd5b8063149bc8721161032557806337838ed0116102ff57806337838ed01461051a57806339b70e38146105415780633a8c0786146105685780633ccc861d1461057f57600080fd5b8063149bc872146104a55780632b9f64a4146104c657806336af41fa1461050757600080fd5b80630eb38345116103615780630eb383451461044357806310d67a2f14610458578063131433b41461046b578063136439dd1461049257600080fd5b806218572c1461038757806304a0c502146103bf5780630e9a53cf146103fb575b600080fd5b6103aa6103953660046145b1565b60d16020526000908152604090205460ff1681565b60405190151581526020015b60405180910390f35b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016103b6565b61040361097d565b604080518251815260208084015163ffffffff908116918301919091528383015116918101919091526060918201511515918101919091526080016103b6565b6104566104513660046145dc565b610a81565b005b6104566104663660046145b1565b610b03565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6104566104a0366004614615565b610bbf565b6104b86104b3366004614646565b610cfe565b6040519081526020016103b6565b6104ef6104d43660046145b1565b60cc602052600090815260409020546001600160a01b031681565b6040516001600160a01b0390911681526020016103b6565b6104566105153660046146ae565b610d74565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6104ef7f000000000000000000000000000000000000000000000000000000000000000081565b60cb546103e690600160a01b900463ffffffff1681565b61045661058d366004614703565b610f3e565b6104566105a036600461475e565b610fa3565b6104566105b336600461478a565b611274565b6105cb6105c63660046145b1565b61131b565b60405161ffff90911681526020016103b6565b60cb546103e690600160c01b900463ffffffff1681565b6104566106033660046147e1565b611377565b610456611388565b6103aa61061e3660046147fc565b606654600160ff9092169190911b9081161490565b6066546104b8565b6103aa61064936600461481f565b61144f565b60cb546105cb90600160e01b900461ffff1681565b6103aa610671366004614854565b60cf60209081526000928352604080842090915290825290205460ff1681565b6104566114dc565b60ca546104b8565b6104566106af3660046145b1565b6114f0565b6104b86106c2366004614880565b60cd60209081526000928352604080842090915290825290205481565b6065546104ef906001600160a01b031681565b6033546001600160a01b03166104ef565b6104ef73beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac081565b610403611501565b6104566107343660046148ae565b61159f565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b61045661076e3660046145b1565b6117d8565b610456610781366004614915565b611837565b6103aa610794366004614854565b60d260209081526000928352604080842090915290825290205460ff1681565b6104566107c2366004614930565b611848565b6104b86107d53660046145b1565b60ce6020526000908152604090205481565b6103e67f000000000000000000000000000000000000000000000000000000000000000081565b6103aa61081c366004614854565b60d060209081526000928352604080842090915290825290205460ff1681565b61045661084a36600461495c565b611a7b565b61045661085d3660046149cf565b611bc3565b610403610870366004614615565b611e1a565b6105cb610883366004614880565b611eac565b6103e6610896366004614615565b611f19565b6104ef7f000000000000000000000000000000000000000000000000000000000000000081565b6103aa6108d0366004614854565b60d360209081526000928352604080842090915290825290205460ff1681565b6104566108fe3660046145b1565b611ff4565b6104b861206a565b6104b8610919366004614646565b6120a8565b61045661092c3660046147e1565b6120b9565b61045661093f366004614615565b6122ef565b60cb546104ef906001600160a01b031681565b6104566109653660046146ae565b61244b565b6104566109783660046146ae565b6125ca565b60408051608081018252600080825260208201819052918101829052606081019190915260ca545b8015610a5857600060ca6109ba600184614a2c565b815481106109ca576109ca614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff161580156060830181905291925090610a3a5750806040015163ffffffff164210155b15610a455792915050565b5080610a5081614a59565b9150506109a5565b505060408051608081018252600080825260208201819052918101829052606081019190915290565b610a89612778565b6001600160a01b038216600081815260d1602052604080822054905160ff9091169284151592841515927f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c9190a4506001600160a01b0391909116600090815260d160205260409020805460ff1916911515919091179055565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610b56573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b7a9190614a70565b6001600160a01b0316336001600160a01b031614610bb35760405162461bcd60e51b8152600401610baa90614a8d565b60405180910390fd5b610bbc816127d2565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610c07573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c2b9190614ad7565b610c475760405162461bcd60e51b8152600401610baa90614af4565b60665481811614610cc05760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610baa565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b600080610d0e60208401846145b1565b8360200135604051602001610d579392919060f89390931b6001600160f81b031916835260609190911b6bffffffffffffffffffffffff19166001830152601582015260350190565b604051602081830303815290604052805190602001209050919050565b60665460019060029081161415610d9d5760405162461bcd60e51b8152600401610baa90614b3c565b33600090815260d1602052604090205460ff16610dcc5760405162461bcd60e51b8152600401610baa90614b73565b60026097541415610def5760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f335736848483818110610e1257610e12614a43565b9050602002810190610e249190614c21565b33600081815260ce60209081526040808320549051949550939192610e4f9290918591879101614d7a565b604051602081830303815290604052805190602001209050610e70836128c9565b33600090815260d0602090815260408083208484529091529020805460ff19166001908117909155610ea3908390614daa565b33600081815260ce602052604090819020929092559051829184917f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf27048290610eeb908890614dc2565b60405180910390a4610f1d333060408601803590610f0c90602089016145b1565b6001600160a01b0316929190612adf565b5050508080610f2b90614dd5565b915050610df7565b505060016097555050565b60665460029060049081161415610f675760405162461bcd60e51b8152600401610baa90614b3c565b60026097541415610f8a5760405162461bcd60e51b8152600401610baa90614bea565b6002609755610f998383612b50565b5050600160975550565b60665460039060089081161415610fcc5760405162461bcd60e51b8152600401610baa90614b3c565b60cb546001600160a01b03163314610ff65760405162461bcd60e51b8152600401610baa90614df0565b60cb5463ffffffff600160c01b9091048116908316116110925760405162461bcd60e51b815260206004820152604b60248201527f52657761726473436f6f7264696e61746f722e7375626d6974526f6f743a206e60448201527f657720726f6f74206d75737420626520666f72206e657765722063616c63756c60648201526a185d1959081c195c9a5bd960aa1b608482015260a401610baa565b428263ffffffff161061112b5760405162461bcd60e51b815260206004820152605560248201527f52657761726473436f6f7264696e61746f722e7375626d6974526f6f743a207260448201527f65776172647343616c63756c6174696f6e456e6454696d657374616d702063616064820152746e6e6f7420626520696e207468652066757475726560581b608482015260a401610baa565b60ca5460cb5460009061114b90600160a01b900463ffffffff1642614e44565b6040805160808101825287815263ffffffff878116602080840182815286841685870181815260006060880181815260ca8054600181018255925297517f42d72674974f694b5f5159593243114d38a5c39c89d6b62fee061ff523240ee160029092029182015592517f42d72674974f694b5f5159593243114d38a5c39c89d6b62fee061ff523240ee290930180549151975193871667ffffffffffffffff1990921691909117600160201b978716979097029690961760ff60401b1916600160401b921515929092029190911790945560cb805463ffffffff60c01b1916600160c01b840217905593519283529394508892908616917fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd08910160405180910390a45050505050565b6066546002906004908116141561129d5760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156112c05760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b8381101561130f576112fd8585838181106112e5576112e5614a43565b90506020028101906112f79190614e6c565b84612b50565b8061130781614dd5565b9150506112c8565b50506001609755505050565b6001600160a01b038116600090815260d5602090815260408083208151606081018352905461ffff80821683526201000082041693820193909352600160201b90920463ffffffff169082015261137190612ebd565b92915050565b61137f612778565b610bbc81612f0b565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156113d0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906113f49190614ad7565b6114105760405162461bcd60e51b8152600401610baa90614af4565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b60006114d48260ca61146460208301836147e1565b63ffffffff168154811061147a5761147a614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff1615156060820152612f7c565b506001919050565b6114e4612778565b6114ee600061324d565b565b6114f8612778565b610bbc8161329f565b60408051608081018252600080825260208201819052918101829052606081019190915260ca805461153590600190614a2c565b8154811061154557611545614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff1615156060820152919050565b606654600590602090811614156115c85760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156115eb5760405162461bcd60e51b8152600401610baa90614bea565b6002609755336001600160a01b0385161461168b5760405162461bcd60e51b815260206004820152605460248201527f52657761726473436f6f7264696e61746f722e6372656174654f70657261746f60448201527f724469726563746564415653526577617264735375626d697373696f6e3a2063606482015273616c6c6572206973206e6f74207468652041565360601b608482015260a401610baa565b60005b8281101561130f57368484838181106116a9576116a9614a43565b90506020028101906116bb9190614e82565b6001600160a01b038716600090815260ce60209081526040808320549051939450926116ed918a918591879101614fff565b6040516020818303038152906040528051906020012090506000611710846132fb565b6001600160a01b038a16600090815260d3602090815260408083208684529091529020805460ff1916600190811790915590915061174f908490614daa565b6001600160a01b038a16600081815260ce60205260409081902092909255905183919033907ffc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e0906117a39088908a90615026565b60405180910390a46117c1333083610f0c6040890160208a016145b1565b5050505080806117d090614dd5565b91505061168e565b33600081815260cc602052604080822080546001600160a01b031981166001600160a01b038781169182179093559251911692839185917fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca31291a4505050565b61183f612778565b610bbc816136cb565b606654600790608090811614156118715760405162461bcd60e51b8152600401610baa90614b3c565b336001600160a01b038416146118f95760405162461bcd60e51b815260206004820152604160248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72504960448201527f53706c69743a2063616c6c6572206973206e6f7420746865206f70657261746f6064820152603960f91b608482015260a401610baa565b61271061ffff831611156119805760405162461bcd60e51b815260206004820152604260248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72504960448201527f53706c69743a2073706c6974206d757374206265203c3d203130303030206269606482015261707360f01b608482015260a401610baa565b60cb5460009061199d90600160a01b900463ffffffff1642614e44565b6001600160a01b038516600090815260d5602090815260408083208151606081018352905461ffff80821683526201000082041693820193909352600160201b90920463ffffffff1690820152919250906119f790612ebd565b6001600160a01b038616600090815260d560205260409020909150611a1d908584613736565b6040805163ffffffff8416815261ffff838116602083015286168183015290516001600160a01b0387169133917fd1e028bd664486a46ad26040e999cd2d22e1e9a094ee6afe19fcf64678f16f749181900360600190a35050505050565b600054610100900460ff1615808015611a9b5750600054600160ff909116105b80611ab55750303b158015611ab5575060005460ff166001145b611b185760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610baa565b6000805460ff191660011790558015611b3b576000805461ff0019166101001790555b611b436137d1565b60c955611b508686613868565b611b598761324d565b611b628461329f565b611b6b83612f0b565b611b74826136cb565b8015611bba576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b50505050505050565b60665460069060409081161415611bec5760405162461bcd60e51b8152600401610baa90614b3c565b336001600160a01b03851614611c755760405162461bcd60e51b815260206004820152604260248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72415660448201527f5353706c69743a2063616c6c6572206973206e6f7420746865206f706572617460648201526137b960f11b608482015260a401610baa565b61271061ffff83161115611cfd5760405162461bcd60e51b815260206004820152604360248201527f52657761726473436f6f7264696e61746f722e7365744f70657261746f72415660448201527f5353706c69743a2073706c6974206d757374206265203c3d203130303030206260648201526269707360e81b608482015260a401610baa565b60cb54600090611d1a90600160a01b900463ffffffff1642614e44565b6001600160a01b03868116600090815260d46020908152604080832093891683529281528282208351606081018552905461ffff80821683526201000082041692820192909252600160201b90910463ffffffff1692810192909252919250611d8290612ebd565b6001600160a01b03808816600090815260d460209081526040808320938a16835292905220909150611db5908584613736565b6040805163ffffffff8416815261ffff838116602083015286168183015290516001600160a01b03878116929089169133917f48e198b6ae357e529204ee53a8e514c470ff77d9cc8e4f7207f8b5d490ae6934919081900360600190a4505050505050565b60408051608081018252600080825260208201819052918101829052606081019190915260ca8281548110611e5157611e51614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff161515606082015292915050565b6001600160a01b03828116600090815260d46020908152604080832093851683529281528282208351606081018552905461ffff80821683526201000082041692820192909252600160201b90910463ffffffff169281019290925290611f1290612ebd565b9392505050565b60ca546000905b63ffffffff811615611f85578260ca611f3a60018461503f565b63ffffffff1681548110611f5057611f50614a43565b9060005260206000209060020201600001541415611f7357611f1260018261503f565b80611f7d81615064565b915050611f20565b5060405162461bcd60e51b815260206004820152603760248201527f52657761726473436f6f7264696e61746f722e676574526f6f74496e6465784660448201527f726f6d486173683a20726f6f74206e6f7420666f756e640000000000000000006064820152608401610baa565b611ffc612778565b6001600160a01b0381166120615760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610baa565b610bbc8161324d565b60007f000000000000000000000000000000000000000000000000000000000000000046141561209b575060c95490565b6120a36137d1565b905090565b60006001610d0e60208401846145b1565b606654600390600890811614156120e25760405162461bcd60e51b8152600401610baa90614b3c565b60cb546001600160a01b0316331461210c5760405162461bcd60e51b8152600401610baa90614df0565b60ca5463ffffffff83161061217d5760405162461bcd60e51b815260206004820152603160248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152700d2dcecc2d8d2c840e4dedee892dcc8caf607b1b6064820152608401610baa565b600060ca8363ffffffff168154811061219857612198614a43565b906000526020600020906002020190508060010160089054906101000a900460ff16156122255760405162461bcd60e51b815260206004820152603560248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152741c9bdbdd08185b1c9958591e48191a5cd8589b1959605a1b6064820152608401610baa565b6001810154600160201b900463ffffffff1642106122a45760405162461bcd60e51b815260206004820152603660248201527f52657761726473436f6f7264696e61746f722e64697361626c65526f6f743a206044820152751c9bdbdd08185b1c9958591e481858dd1a5d985d195960521b6064820152608401610baa565b60018101805460ff60401b1916600160401b17905560405163ffffffff8416907fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e90600090a2505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612342573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123669190614a70565b6001600160a01b0316336001600160a01b0316146123965760405162461bcd60e51b8152600401610baa90614a8d565b6066541981196066541916146124145760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610baa565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610cf3565b606654600090600190811614156124745760405162461bcd60e51b8152600401610baa90614b3c565b600260975414156124975760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f3357368484838181106124ba576124ba614a43565b90506020028101906124cc9190614c21565b33600081815260ce602090815260408083205490519495509391926124f79290918591879101614d7a565b604051602081830303815290604052805190602001209050612518836128c9565b33600090815260cf602090815260408083208484529091529020805460ff1916600190811790915561254b908390614daa565b33600081815260ce602052604090819020929092559051829184917f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e628190612593908890614dc2565b60405180910390a46125b4333060408601803590610f0c90602089016145b1565b50505080806125c290614dd5565b91505061249f565b606654600490601090811614156125f35760405162461bcd60e51b8152600401610baa90614b3c565b33600090815260d1602052604090205460ff166126225760405162461bcd60e51b8152600401610baa90614b73565b600260975414156126455760405162461bcd60e51b8152600401610baa90614bea565b600260975560005b82811015610f33573684848381811061266857612668614a43565b905060200281019061267a9190614c21565b33600081815260ce602090815260408083205490519495509391926126a59290918591879101614d7a565b6040516020818303038152906040528051906020012090506126c6836128c9565b33600090815260d2602090815260408083208484529091529020805460ff191660019081179091556126f9908390614daa565b33600081815260ce602052604090819020929092559051829184917f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b90612741908890614dc2565b60405180910390a4612762333060408601803590610f0c90602089016145b1565b505050808061277090614dd5565b91505061264d565b6033546001600160a01b031633146114ee5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610baa565b6001600160a01b0381166128605760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610baa565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6128fb6128d68280615084565b6128e660808501606086016147e1565b6128f660a08601608087016147e1565b613952565b600081604001351161297f5760405162461bcd60e51b815260206004820152604160248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20616d6f756e742063616e6e6f74206265206064820152600360fc1b608482015260a401610baa565b6f4b3b4ca85a86c47a098a223fffffffff81604001351115612a095760405162461bcd60e51b815260206004820152603f60248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20616d6f756e7420746f6f206c61726765006064820152608401610baa565b612a3963ffffffff7f00000000000000000000000000000000000000000000000000000000000000001642614daa565b612a4960808301606084016147e1565b63ffffffff161115610bbc5760405162461bcd60e51b815260206004820152605360248201527f52657761726473436f6f7264696e61746f722e5f76616c69646174655265776160448201527f7264735375626d697373696f6e3a20737461727454696d657374616d7020746f6064820152726f2066617220696e207468652066757475726560681b608482015260a401610baa565b6040516001600160a01b0380851660248301528316604482015260648101829052612b4a9085906323b872dd60e01b906084015b60408051601f198184030181529190526020810180516001600160e01b03166001600160e01b031990931692909217909152613f41565b50505050565b600060ca612b6160208501856147e1565b63ffffffff1681548110612b7757612b77614a43565b600091825260209182902060408051608081018252600293909302909101805483526001015463ffffffff80821694840194909452600160201b810490931690820152600160401b90910460ff16151560608201529050612bd88382612f7c565b6000612bea60808501606086016145b1565b6001600160a01b03808216600090815260cc60205260409020549192501680612c105750805b336001600160a01b03821614612c8e5760405162461bcd60e51b815260206004820152603c60248201527f52657761726473436f6f7264696e61746f722e70726f63657373436c61696d3a60448201527f2063616c6c6572206973206e6f742076616c696420636c61696d6572000000006064820152608401610baa565b60005b612c9e60a08701876150ce565b9050811015612eb55736612cb560e0880188615084565b83818110612cc557612cc5614a43565b6001600160a01b038716600090815260cd602090815260408083209302949094019450929091508290612cfa908501856145b1565b6001600160a01b03166001600160a01b0316815260200190815260200160002054905080826020013511612db45760405162461bcd60e51b815260206004820152605560248201527f52657761726473436f6f7264696e61746f722e70726f63657373436c61696d3a60448201527f2063756d756c61746976654561726e696e6773206d75737420626520677420746064820152741a185b8818dd5b5d5b185d1a5d9950db185a5b5959605a1b608482015260a401610baa565b6000612dc4826020850135614a2c565b6001600160a01b038716600090815260cd60209081526040822092935085018035929190612df290876145b1565b6001600160a01b0316815260208082019290925260400160002091909155612e349089908390612e24908701876145b1565b6001600160a01b03169190614013565b86516001600160a01b03808a1691878216918916907f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce3190612e7860208901896145b1565b604080519283526001600160a01b039091166020830152810186905260600160405180910390a45050508080612ead90614dd5565b915050612c91565b505050505050565b6000816040015163ffffffff1660001415612ee557505060cb54600160e01b900461ffff1690565b816040015163ffffffff16421015612efe578151611371565b506020015190565b919050565b60cb546040805163ffffffff600160a01b9093048316815291831660208301527faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b3910160405180910390a160cb805463ffffffff909216600160a01b0263ffffffff60a01b19909216919091179055565b806060015115612fd55760405162461bcd60e51b8152602060048201526030602482015260008051602061523283398151915260448201526f1c9bdbdd081a5cc8191a5cd8589b195960821b6064820152608401610baa565b806040015163ffffffff1642101561303c5760405162461bcd60e51b815260206004820152603660248201526000805160206152328339815191526044820152751c9bdbdd081b9bdd081858dd1a5d985d1959081e595d60521b6064820152608401610baa565b61304960c08301836150ce565b905061305860a08401846150ce565b9050146130d05760405162461bcd60e51b815260206004820152604c602482015260008051602061523283398151915260448201527f746f6b656e496e646963657320616e6420746f6b656e50726f6f6673206c656e60648201526b0cee8d040dad2e6dac2e8c6d60a31b608482015260a401610baa565b6130dd60e0830183615084565b90506130ec60c08401846150ce565b9050146131625760405162461bcd60e51b815260206004820152604a602482015260008051602061523283398151915260448201527f746f6b656e5472656550726f6f667320616e64206c6561766573206c656e67746064820152690d040dad2e6dac2e8c6d60b31b608482015260a401610baa565b805161318e9061317860408501602086016147e1565b6131856040860186615118565b86606001614043565b60005b61319e60a08401846150ce565b90508110156132485761323860808401356131bc60a08601866150ce565b848181106131cc576131cc614a43565b90506020020160208101906131e191906147e1565b6131ee60c08701876150ce565b858181106131fe576131fe614a43565b90506020028101906132109190615118565b61321d60e0890189615084565b8781811061322d5761322d614a43565b9050604002016141af565b61324181614dd5565b9050613191565b505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60cb546040516001600160a01b038084169216907f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb90600090a360cb80546001600160a01b0319166001600160a01b0392909216919091179055565b600061332a61330a8380615084565b61331a60808601606087016147e1565b6128f660a08701608088016147e1565b60006133396040840184615084565b9050116133b95760405162461bcd60e51b8152602060048201526054602482015260008051602061527283398151915260448201527f61746f724469726563746564526577617264735375626d697373696f6e3a206e6064820152731bc81bdc195c985d1bdc9cc81c995dd85c99195960621b608482015260a401610baa565b60008060005b6133cc6040860186615084565b90508110156135fe57366133e36040870187615084565b838181106133f3576133f3614a43565b6040029190910191506000905061340d60208301836145b1565b6001600160a01b0316141561348c5760405162461bcd60e51b815260206004820152605b6024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722063616e6e6f74206265203020616464726573730000000000608482015260a401610baa565b61349960208201826145b1565b6001600160a01b0316836001600160a01b0316106135475760405162461bcd60e51b81526020600482015260786024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f7273206d75737420626520696e20617363656e64696e67206f7260848201527f64657220746f2068616e646c65206475706c696361746573000000000000000060a482015260c401610baa565b61355460208201826145b1565b925060008160200135116135dc5760405162461bcd60e51b81526020600482015260616024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722072657761726420616d6f756e742063616e6e6f74206265206084820152600360fc1b60a482015260c401610baa565b6135ea602082013585614daa565b935050806135f790614dd5565b90506133bf565b504261361060a08601608087016147e1565b61362060808701606088016147e1565b61362a9190614e44565b63ffffffff16106136c45760405162461bcd60e51b81526020600482015260766024820152600080516020615272833981519152604482015260008051602061525283398151915260648201527f70657261746f722d64697265637465642072657761726473207375626d697373608482015275696f6e206973206e6f7420726574726f61637469766560501b60a482015260c401610baa565b5092915050565b60cb546040805161ffff600160e01b9093048316815291831660208301527fe6cd4edfdcc1f6d130ab35f73d72378f3a642944fb4ee5bd84b7807a81ea1c4e910160405180910390a160cb805461ffff909216600160e01b0261ffff60e01b19909216919091179055565b8254600160201b900463ffffffff164210613795578254600160201b900463ffffffff1661377e5760cb548354600160e01b90910461ffff1661ffff19909116178355613795565b825462010000810461ffff1661ffff199091161783555b825463ffffffff909116600160201b0267ffffffff000000001961ffff90931662010000029290921667ffffffffffff00001990911617179055565b604080518082018252600a81526922b4b3b2b72630bcb2b960b11b60209182015281517f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866818301527f71b625cfad44bac63b13dba07f2e1d6084ee04b6f8752101ece6126d584ee6ea81840152466060820152306080808301919091528351808303909101815260a0909101909252815191012090565b6065546001600160a01b031615801561388957506001600160a01b03821615155b61390b5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610baa565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261394e826127d2565b5050565b826139c25760405162461bcd60e51b8152602060048201526046602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206e6f207374726174656769606482015265195cc81cd95d60d21b608482015260a401610baa565b7f000000000000000000000000000000000000000000000000000000000000000063ffffffff168163ffffffff161115613a785760405162461bcd60e51b815260206004820152605a602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206475726174696f6e20657860648201527f6365656473204d41585f524557415244535f4455524154494f4e000000000000608482015260a401610baa565b613aa27f000000000000000000000000000000000000000000000000000000000000000082615175565b63ffffffff1615613b485760405162461bcd60e51b8152602060048201526070602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a206475726174696f6e206d7560648201527f73742062652061206d756c7469706c65206f662043414c43554c4154494f4e5f60848201526f494e54455256414c5f5345434f4e445360801b60a482015260c401610baa565b613b727f000000000000000000000000000000000000000000000000000000000000000083615175565b63ffffffff1615613c1e5760405162461bcd60e51b8152602060048201526076602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737461727454696d65737460648201527f616d70206d7573742062652061206d756c7469706c65206f662043414c43554c6084820152754154494f4e5f494e54455256414c5f5345434f4e445360501b60a482015260c401610baa565b8163ffffffff167f000000000000000000000000000000000000000000000000000000000000000063ffffffff1642613c579190614a2c565b11158015613c9157508163ffffffff167f000000000000000000000000000000000000000000000000000000000000000063ffffffff1611155b613d175760405162461bcd60e51b8152602060048201526057602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737461727454696d65737460648201527f616d7020746f6f2066617220696e207468652070617374000000000000000000608482015260a401610baa565b6000805b84811015612eb5576000868683818110613d3757613d37614a43565b613d4d92602060409092020190810191506145b1565b60405163198f077960e21b81526001600160a01b0380831660048301529192507f00000000000000000000000000000000000000000000000000000000000000009091169063663c1de490602401602060405180830381865afa158015613db8573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190613ddc9190614ad7565b80613e0357506001600160a01b03811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0145b613e7c5760405162461bcd60e51b8152602060048201526050602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20696e76616c69642073747260648201526f185d1959de4818dbdb9cda59195c995960821b608482015260a401610baa565b806001600160a01b0316836001600160a01b031610613f2f5760405162461bcd60e51b815260206004820152606f602482015260008051602061529283398151915260448201527f6f6e526577617264735375626d697373696f6e3a20737472617465676965732060648201527f6d75737420626520696e20617363656e64696e67206f7264657220746f20686160848201526e6e646c65206475706c69636174657360881b60a482015260c401610baa565b9150613f3a81614dd5565b9050613d1b565b6000613f96826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166143009092919063ffffffff16565b8051909150156132485780806020019051810190613fb49190614ad7565b6132485760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610baa565b6040516001600160a01b03831660248201526044810182905261324890849063a9059cbb60e01b90606401612b13565b61404e602083615198565b6001901b8463ffffffff16106140d85760405162461bcd60e51b815260206004820152604360248201527f52657761726473436f6f7264696e61746f722e5f7665726966794561726e657260448201527f436c61696d50726f6f663a20696e76616c6964206561726e65724c656166496e6064820152620c8caf60eb1b608482015260a401610baa565b60006140e382610cfe565b905061412e84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508a92508591505063ffffffff8916614317565b612eb55760405162461bcd60e51b815260206004820152604660248201527f52657761726473436f6f7264696e61746f722e5f7665726966794561726e657260448201527f436c61696d50726f6f663a20696e76616c6964206561726e657220636c61696d60648201526510383937b7b360d11b608482015260a401610baa565b6141ba602083615198565b6001901b8463ffffffff16106142385760405162461bcd60e51b815260206004820152603c60248201527f52657761726473436f6f7264696e61746f722e5f766572696679546f6b656e4360448201527f6c61696d3a20696e76616c696420746f6b656e4c656166496e646578000000006064820152608401610baa565b6000614243826120a8565b905061428e84848080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508a92508591505063ffffffff8916614317565b612eb55760405162461bcd60e51b815260206004820152603f60248201527f52657761726473436f6f7264696e61746f722e5f766572696679546f6b656e4360448201527f6c61696d3a20696e76616c696420746f6b656e20636c61696d2070726f6f66006064820152608401610baa565b606061430f848460008561432f565b949350505050565b600083614325868585614460565b1495945050505050565b6060824710156143905760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610baa565b6001600160a01b0385163b6143e75760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610baa565b600080866001600160a01b0316858760405161440391906151d8565b60006040518083038185875af1925050503d8060008114614440576040519150601f19603f3d011682016040523d82523d6000602084013e614445565b606091505b5091509150614455828286614563565b979650505050505050565b60006020845161447091906151ea565b156144f75760405162461bcd60e51b815260206004820152604b60248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f664b65636360448201527f616b3a2070726f6f66206c656e6774682073686f756c642062652061206d756c60648201526a3a34b836329037b310199960a91b608482015260a401610baa565b8260205b8551811161455a5761450e6002856151ea565b61452f57816000528086015160205260406000209150600284049350614548565b8086015160005281602052604060002091506002840493505b614553602082614daa565b90506144fb565b50949350505050565b60608315614572575081611f12565b8251156145825782518084602001fd5b8160405162461bcd60e51b8152600401610baa91906151fe565b6001600160a01b0381168114610bbc57600080fd5b6000602082840312156145c357600080fd5b8135611f128161459c565b8015158114610bbc57600080fd5b600080604083850312156145ef57600080fd5b82356145fa8161459c565b9150602083013561460a816145ce565b809150509250929050565b60006020828403121561462757600080fd5b5035919050565b60006040828403121561464057600080fd5b50919050565b60006040828403121561465857600080fd5b611f12838361462e565b60008083601f84011261467457600080fd5b50813567ffffffffffffffff81111561468c57600080fd5b6020830191508360208260051b85010111156146a757600080fd5b9250929050565b600080602083850312156146c157600080fd5b823567ffffffffffffffff8111156146d857600080fd5b6146e485828601614662565b90969095509350505050565b6000610100828403121561464057600080fd5b6000806040838503121561471657600080fd5b823567ffffffffffffffff81111561472d57600080fd5b614739858286016146f0565b925050602083013561460a8161459c565b803563ffffffff81168114612f0657600080fd5b6000806040838503121561477157600080fd5b823591506147816020840161474a565b90509250929050565b60008060006040848603121561479f57600080fd5b833567ffffffffffffffff8111156147b657600080fd5b6147c286828701614662565b90945092505060208401356147d68161459c565b809150509250925092565b6000602082840312156147f357600080fd5b611f128261474a565b60006020828403121561480e57600080fd5b813560ff81168114611f1257600080fd5b60006020828403121561483157600080fd5b813567ffffffffffffffff81111561484857600080fd5b61430f848285016146f0565b6000806040838503121561486757600080fd5b82356148728161459c565b946020939093013593505050565b6000806040838503121561489357600080fd5b823561489e8161459c565b9150602083013561460a8161459c565b6000806000604084860312156148c357600080fd5b83356148ce8161459c565b9250602084013567ffffffffffffffff8111156148ea57600080fd5b6148f686828701614662565b9497909650939450505050565b803561ffff81168114612f0657600080fd5b60006020828403121561492757600080fd5b611f1282614903565b6000806040838503121561494357600080fd5b823561494e8161459c565b915061478160208401614903565b60008060008060008060c0878903121561497557600080fd5b86356149808161459c565b955060208701356149908161459c565b94506040870135935060608701356149a78161459c565b92506149b56080880161474a565b91506149c360a08801614903565b90509295509295509295565b6000806000606084860312156149e457600080fd5b83356149ef8161459c565b925060208401356149ff8161459c565b9150614a0d60408501614903565b90509250925092565b634e487b7160e01b600052601160045260246000fd5b600082821015614a3e57614a3e614a16565b500390565b634e487b7160e01b600052603260045260246000fd5b600081614a6857614a68614a16565b506000190190565b600060208284031215614a8257600080fd5b8151611f128161459c565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215614ae957600080fd5b8151611f12816145ce565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b60208082526019908201527f5061757361626c653a20696e6465782069732070617573656400000000000000604082015260600190565b60208082526051908201527f52657761726473436f6f7264696e61746f723a2063616c6c6572206973206e6f60408201527f7420612076616c69642063726561746552657761726473466f72416c6c53756260608201527036b4b9b9b4b7b71039bab136b4ba3a32b960791b608082015260a00190565b6020808252601f908201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c00604082015260600190565b60008235609e19833603018112614c3757600080fd5b9190910192915050565b6000808335601e19843603018112614c5857600080fd5b830160208101925035905067ffffffffffffffff811115614c7857600080fd5b8060061b36038313156146a757600080fd5b818352600060208085019450826000805b86811015614cef578235614cae8161459c565b6001600160a01b03168852828401356bffffffffffffffffffffffff8116808214614cd7578384fd5b89860152506040978801979290920191600101614c9b565b50959695505050505050565b6000614d078283614c41565b60a08552614d1960a086018284614c8a565b9150506020830135614d2a8161459c565b6001600160a01b0316602085015260408381013590850152614d4e6060840161474a565b63ffffffff808216606087015280614d686080870161474a565b16608087015250508091505092915050565b60018060a01b0384168152826020820152606060408201526000614da16060830184614cfb565b95945050505050565b60008219821115614dbd57614dbd614a16565b500190565b602081526000611f126020830184614cfb565b6000600019821415614de957614de9614a16565b5060010190565b60208082526034908201527f52657761726473436f6f7264696e61746f723a2063616c6c6572206973206e6f6040820152733a103a3432903932bbb0b93239aab83230ba32b960611b606082015260800190565b600063ffffffff808316818516808303821115614e6357614e63614a16565b01949350505050565b6000823560fe19833603018112614c3757600080fd5b6000823560be19833603018112614c3757600080fd5b6000808335601e19843603018112614eaf57600080fd5b830160208101925035905067ffffffffffffffff811115614ecf57600080fd5b8036038313156146a757600080fd5b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000614f138283614c41565b60c08552614f2560c086018284614c8a565b915050602080840135614f378161459c565b6001600160a01b0390811686830152604090614f5586830187614c41565b888603848a015280865290946000919085015b81831015614f99578635614f7b8161459c565b84168152868601358682015295840195600192909201918401614f68565b614fa560608a0161474a565b63ffffffff811660608c01529650614fbf60808a0161474a565b63ffffffff811660808c01529650614fda60a08a018a614e98565b9750955089810360a08b0152614ff1818888614ede565b9a9950505050505050505050565b60018060a01b0384168152826020820152606060408201526000614da16060830184614f07565b82815260406020820152600061430f6040830184614f07565b600063ffffffff8381169083168181101561505c5761505c614a16565b039392505050565b600063ffffffff82168061507a5761507a614a16565b6000190192915050565b6000808335601e1984360301811261509b57600080fd5b83018035915067ffffffffffffffff8211156150b657600080fd5b6020019150600681901b36038213156146a757600080fd5b6000808335601e198436030181126150e557600080fd5b83018035915067ffffffffffffffff82111561510057600080fd5b6020019150600581901b36038213156146a757600080fd5b6000808335601e1984360301811261512f57600080fd5b83018035915067ffffffffffffffff82111561514a57600080fd5b6020019150368190038213156146a757600080fd5b634e487b7160e01b600052601260045260246000fd5b600063ffffffff8084168061518c5761518c61515f565b92169190910692915050565b6000826151a7576151a761515f565b500490565b60005b838110156151c75781810151838201526020016151af565b83811115612b4a5750506000910152565b60008251614c378184602087016151ac565b6000826151f9576151f961515f565b500690565b602081526000825180602084015261521d8160408501602087016151ac565b601f01601f1916919091016040019291505056fe52657761726473436f6f7264696e61746f722e5f636865636b436c61696d3a2061746f724469726563746564526577617264735375626d697373696f6e3a206f52657761726473436f6f7264696e61746f722e5f76616c69646174654f70657252657761726473436f6f7264696e61746f722e5f76616c6964617465436f6d6da2646970667358221220081e7033322140012f1d9ef09b55b455e888c99c5a08a478635131f15ff9795b64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x03\x82W`\x005`\xE0\x1C\x80c\x86\\iS\x11a\x01\xDEW\x80c\xD4T\nU\x11a\x01\x0FW\x80c\xF2\xFD\xE3\x8B\x11a\0\xADW\x80c\xFA\xBC\x1C\xBC\x11a\0|W\x80c\xFA\xBC\x1C\xBC\x14a\t1W\x80c\xFB\xF1\xE2\xC1\x14a\tDW\x80c\xFC\xE3l}\x14a\tWW\x80c\xFF\x9Fl\xCE\x14a\tjW`\0\x80\xFD[\x80c\xF2\xFD\xE3\x8B\x14a\x08\xF0W\x80c\xF6\x98\xDA%\x14a\t\x03W\x80c\xF8\xCD\x84H\x14a\t\x0BW\x80c\xF9j\xBF.\x14a\t\x1EW`\0\x80\xFD[\x80c\xE0c\xF8\x1F\x11a\0\xE9W\x80c\xE0c\xF8\x1F\x14a\x08uW\x80c\xE8\x10\xCE!\x14a\x08\x88W\x80c\xEAM<\x9B\x14a\x08\x9BW\x80c\xEDq\xE6\xA2\x14a\x08\xC2W`\0\x80\xFD[\x80c\xD4T\nU\x14a\x08<W\x80c\xDC\xBB\x03\xB3\x14a\x08OW\x80c\xDE\x02\xE5\x03\x14a\x08bW`\0\x80\xFD[\x80c\xA0\x16\x9D\xDD\x11a\x01|W\x80c\xB3\xDB\xB0\xE0\x11a\x01VW\x80c\xB3\xDB\xB0\xE0\x14a\x07\xB4W\x80c\xBB~E\x1F\x14a\x07\xC7W\x80c\xBF!\xA8\xAA\x14a\x07\xE7W\x80c\xC4m\xB6\x06\x14a\x08\x0EW`\0\x80\xFD[\x80c\xA0\x16\x9D\xDD\x14a\x07`W\x80c\xA5\n\x1D\x9C\x14a\x07sW\x80c\xAE\xBD\x8B\xAE\x14a\x07\x86W`\0\x80\xFD[\x80c\x91\x04\xC3\x19\x11a\x01\xB8W\x80c\x91\x04\xC3\x19\x14a\x07\x03W\x80c\x9B\xE3\xD4\xE4\x14a\x07\x1EW\x80c\x9C\xB9\xA5\xFA\x14a\x07&W\x80c\x9DE\xC2\x81\x14a\x079W`\0\x80\xFD[\x80c\x86\\iS\x14a\x06\xB4W\x80c\x88o\x11\x95\x14a\x06\xDFW\x80c\x8D\xA5\xCB[\x14a\x06\xF2W`\0\x80\xFD[\x80c>\xFE\x1D\xB6\x11a\x02\xB8W\x80c\\\x97Z\xBB\x11a\x02VW\x80cm!\x11~\x11a\x020W\x80cm!\x11~\x14a\x06cW\x80cqP\x18\xA6\x14a\x06\x91W\x80c{\x8F\x8B\x05\x14a\x06\x99W\x80c\x86<\xB9\xA9\x14a\x06\xA1W`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x063W\x80c^\x9D\x83H\x14a\x06;W\x80cc\xF6\xA7\x98\x14a\x06NW`\0\x80\xFD[\x80cM\x18\xCC5\x11a\x02\x92W\x80cM\x18\xCC5\x14a\x05\xDEW\x80cX\xBA\xAA>\x14a\x05\xF5W\x80cY\\jg\x14a\x06\x08W\x80cZ\xC8j\xB7\x14a\x06\x10W`\0\x80\xFD[\x80c>\xFE\x1D\xB6\x14a\x05\x92W\x80cE\x96\x02\x1C\x14a\x05\xA5W\x80cK\x949`\x14a\x05\xB8W`\0\x80\xFD[\x80c\x14\x9B\xC8r\x11a\x03%W\x80c7\x83\x8E\xD0\x11a\x02\xFFW\x80c7\x83\x8E\xD0\x14a\x05\x1AW\x80c9\xB7\x0E8\x14a\x05AW\x80c:\x8C\x07\x86\x14a\x05hW\x80c<\xCC\x86\x1D\x14a\x05\x7FW`\0\x80\xFD[\x80c\x14\x9B\xC8r\x14a\x04\xA5W\x80c+\x9Fd\xA4\x14a\x04\xC6W\x80c6\xAFA\xFA\x14a\x05\x07W`\0\x80\xFD[\x80c\x0E\xB3\x83E\x11a\x03aW\x80c\x0E\xB3\x83E\x14a\x04CW\x80c\x10\xD6z/\x14a\x04XW\x80c\x13\x143\xB4\x14a\x04kW\x80c\x13d9\xDD\x14a\x04\x92W`\0\x80\xFD[\x80b\x18W,\x14a\x03\x87W\x80c\x04\xA0\xC5\x02\x14a\x03\xBFW\x80c\x0E\x9AS\xCF\x14a\x03\xFBW[`\0\x80\xFD[a\x03\xAAa\x03\x956`\x04aE\xB1V[`\xD1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xB6V[a\x04\x03a\t}V[`@\x80Q\x82Q\x81R` \x80\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x83\x01\x91\x90\x91R\x83\x83\x01Q\x16\x91\x81\x01\x91\x90\x91R``\x91\x82\x01Q\x15\x15\x91\x81\x01\x91\x90\x91R`\x80\x01a\x03\xB6V[a\x04Va\x04Q6`\x04aE\xDCV[a\n\x81V[\0[a\x04Va\x04f6`\x04aE\xB1V[a\x0B\x03V[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x04\xA06`\x04aF\x15V[a\x0B\xBFV[a\x04\xB8a\x04\xB36`\x04aFFV[a\x0C\xFEV[`@Q\x90\x81R` \x01a\x03\xB6V[a\x04\xEFa\x04\xD46`\x04aE\xB1V[`\xCC` R`\0\x90\x81R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03\xB6V[a\x04Va\x05\x156`\x04aF\xAEV[a\rtV[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\xCBTa\x03\xE6\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04Va\x05\x8D6`\x04aG\x03V[a\x0F>V[a\x04Va\x05\xA06`\x04aG^V[a\x0F\xA3V[a\x04Va\x05\xB36`\x04aG\x8AV[a\x12tV[a\x05\xCBa\x05\xC66`\x04aE\xB1V[a\x13\x1BV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x01a\x03\xB6V[`\xCBTa\x03\xE6\x90`\x01`\xC0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x04Va\x06\x036`\x04aG\xE1V[a\x13wV[a\x04Va\x13\x88V[a\x03\xAAa\x06\x1E6`\x04aG\xFCV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x04\xB8V[a\x03\xAAa\x06I6`\x04aH\x1FV[a\x14OV[`\xCBTa\x05\xCB\x90`\x01`\xE0\x1B\x90\x04a\xFF\xFF\x16\x81V[a\x03\xAAa\x06q6`\x04aHTV[`\xCF` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x14\xDCV[`\xCATa\x04\xB8V[a\x04Va\x06\xAF6`\x04aE\xB1V[a\x14\xF0V[a\x04\xB8a\x06\xC26`\x04aH\x80V[`\xCD` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`eTa\x04\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x04\xEFV[a\x04\xEFs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81V[a\x04\x03a\x15\x01V[a\x04Va\x0746`\x04aH\xAEV[a\x15\x9FV[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04Va\x07n6`\x04aE\xB1V[a\x17\xD8V[a\x04Va\x07\x816`\x04aI\x15V[a\x187V[a\x03\xAAa\x07\x946`\x04aHTV[`\xD2` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x07\xC26`\x04aI0V[a\x18HV[a\x04\xB8a\x07\xD56`\x04aE\xB1V[`\xCE` R`\0\x90\x81R`@\x90 T\x81V[a\x03\xE6\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xAAa\x08\x1C6`\x04aHTV[`\xD0` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x08J6`\x04aI\\V[a\x1A{V[a\x04Va\x08]6`\x04aI\xCFV[a\x1B\xC3V[a\x04\x03a\x08p6`\x04aF\x15V[a\x1E\x1AV[a\x05\xCBa\x08\x836`\x04aH\x80V[a\x1E\xACV[a\x03\xE6a\x08\x966`\x04aF\x15V[a\x1F\x19V[a\x04\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\xAAa\x08\xD06`\x04aHTV[`\xD3` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[a\x04Va\x08\xFE6`\x04aE\xB1V[a\x1F\xF4V[a\x04\xB8a jV[a\x04\xB8a\t\x196`\x04aFFV[a \xA8V[a\x04Va\t,6`\x04aG\xE1V[a \xB9V[a\x04Va\t?6`\x04aF\x15V[a\"\xEFV[`\xCBTa\x04\xEF\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x04Va\te6`\x04aF\xAEV[a$KV[a\x04Va\tx6`\x04aF\xAEV[a%\xCAV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCAT[\x80\x15a\nXW`\0`\xCAa\t\xBA`\x01\x84aJ,V[\x81T\x81\x10a\t\xCAWa\t\xCAaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x80\x15``\x83\x01\x81\x90R\x91\x92P\x90a\n:WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\nEW\x92\x91PPV[P\x80a\nP\x81aJYV[\x91PPa\t\xA5V[PP`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x90V[a\n\x89a'xV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\xD1` R`@\x80\x82 T\x90Q`\xFF\x90\x91\x16\x92\x84\x15\x15\x92\x84\x15\x15\x92\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C\x91\x90\xA4P`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\0\x90\x81R`\xD1` R`@\x90 \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0BVW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Bz\x91\x90aJpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0B\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\x8DV[`@Q\x80\x91\x03\x90\xFD[a\x0B\xBC\x81a'\xD2V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C+\x91\x90aJ\xD7V[a\x0CGW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\xF4V[`fT\x81\x81\x16\x14a\x0C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80a\r\x0E` \x84\x01\x84aE\xB1V[\x83` \x015`@Q` \x01a\rW\x93\x92\x91\x90`\xF8\x93\x90\x93\x1B`\x01`\x01`\xF8\x1B\x03\x19\x16\x83R``\x91\x90\x91\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01\x83\x01R`\x15\x82\x01R`5\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x14\x15a\r\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\0\x90\x81R`\xD1` R`@\x90 T`\xFF\x16a\r\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aKsV[`\x02`\x97T\x14\x15a\r\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a\x0E\x12Wa\x0E\x12aJCV[\x90P` \x02\x81\x01\x90a\x0E$\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a\x0EO\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x0Ep\x83a(\xC9V[3`\0\x90\x81R`\xD0` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua\x0E\xA3\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x82\x90a\x0E\xEB\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a\x0F\x1D30`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x92\x91\x90a*\xDFV[PPP\x80\x80a\x0F+\x90aM\xD5V[\x91PPa\r\xF7V[PP`\x01`\x97UPPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x0FgW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x0F\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97Ua\x0F\x99\x83\x83a+PV[PP`\x01`\x97UPV[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a\x0F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0F\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aM\xF0V[`\xCBTc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x90\x83\x16\x11a\x10\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FRewardsCoordinator.submitRoot: n`D\x82\x01R\x7Few root must be for newer calcul`d\x82\x01Rj\x18]\x19Y\x08\x1C\x19\\\x9A[\xD9`\xAA\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[B\x82c\xFF\xFF\xFF\xFF\x16\x10a\x11+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FRewardsCoordinator.submitRoot: r`D\x82\x01R\x7FewardsCalculationEndTimestamp ca`d\x82\x01Rtnnot be in the future`X\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCAT`\xCBT`\0\x90a\x11K\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`@\x80Q`\x80\x81\x01\x82R\x87\x81Rc\xFF\xFF\xFF\xFF\x87\x81\x16` \x80\x84\x01\x82\x81R\x86\x84\x16\x85\x87\x01\x81\x81R`\0``\x88\x01\x81\x81R`\xCA\x80T`\x01\x81\x01\x82U\x92R\x97Q\x7FB\xD7&t\x97OiK_QYY2C\x11M8\xA5\xC3\x9C\x89\xD6\xB6/\xEE\x06\x1F\xF5#$\x0E\xE1`\x02\x90\x92\x02\x91\x82\x01U\x92Q\x7FB\xD7&t\x97OiK_QYY2C\x11M8\xA5\xC3\x9C\x89\xD6\xB6/\xEE\x06\x1F\xF5#$\x0E\xE2\x90\x93\x01\x80T\x91Q\x97Q\x93\x87\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x97\x87\x16\x97\x90\x97\x02\x96\x90\x96\x17`\xFF`@\x1B\x19\x16`\x01`@\x1B\x92\x15\x15\x92\x90\x92\x02\x91\x90\x91\x17\x90\x94U`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1B\x84\x02\x17\x90U\x93Q\x92\x83R\x93\x94P\x88\x92\x90\x86\x16\x91\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91\x01`@Q\x80\x91\x03\x90\xA4PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x14\x15a\x12\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x12\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x83\x81\x10\x15a\x13\x0FWa\x12\xFD\x85\x85\x83\x81\x81\x10a\x12\xE5Wa\x12\xE5aJCV[\x90P` \x02\x81\x01\x90a\x12\xF7\x91\x90aNlV[\x84a+PV[\x80a\x13\x07\x81aM\xD5V[\x91PPa\x12\xC8V[PP`\x01`\x97UPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xD5` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01` \x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01Ra\x13q\x90a.\xBDV[\x92\x91PPV[a\x13\x7Fa'xV[a\x0B\xBC\x81a/\x0BV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xD0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xF4\x91\x90aJ\xD7V[a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\xF4V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`\0a\x14\xD4\x82`\xCAa\x14d` \x83\x01\x83aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x14zWa\x14zaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01Ra/|V[P`\x01\x91\x90PV[a\x14\xE4a'xV[a\x14\xEE`\0a2MV[V[a\x14\xF8a'xV[a\x0B\xBC\x81a2\x9FV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCA\x80Ta\x155\x90`\x01\x90aJ,V[\x81T\x81\x10a\x15EWa\x15EaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x91\x90PV[`fT`\x05\x90` \x90\x81\x16\x14\x15a\x15\xC8W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a\x15\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x16\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FRewardsCoordinator.createOperato`D\x82\x01R\x7FrDirectedAVSRewardsSubmission: c`d\x82\x01Rsaller is not the AVS``\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0[\x82\x81\x10\x15a\x13\x0FW6\x84\x84\x83\x81\x81\x10a\x16\xA9Wa\x16\xA9aJCV[\x90P` \x02\x81\x01\x90a\x16\xBB\x91\x90aN\x82V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x93\x94P\x92a\x16\xED\x91\x8A\x91\x85\x91\x87\x91\x01aO\xFFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P`\0a\x17\x10\x84a2\xFBV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x90\x81R`\xD3` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91U\x90\x91Pa\x17O\x90\x84\x90aM\xAAV[`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x83\x91\x903\x90\x7F\xFC\x88\x88\xBF\xFDq\x1D\xA6\x0B\xC5\t+3\xF6w\xD8\x18\x96\xFE\x80\xEC\xC6w\xB8L\xFA\xB8\x18Db\xB6\xE0\x90a\x17\xA3\x90\x88\x90\x8A\x90aP&V[`@Q\x80\x91\x03\x90\xA4a\x17\xC130\x83a\x0F\x0C`@\x89\x01` \x8A\x01aE\xB1V[PPPP\x80\x80a\x17\xD0\x90aM\xD5V[\x91PPa\x16\x8EV[3`\0\x81\x81R`\xCC` R`@\x80\x82 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x91\x82\x17\x90\x93U\x92Q\x91\x16\x92\x83\x91\x85\x91\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x91\xA4PPPV[a\x18?a'xV[a\x0B\xBC\x81a6\xCBV[`fT`\x07\x90`\x80\x90\x81\x16\x14\x15a\x18qW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\x01`\x01`\xA0\x1B\x03\x84\x16\x14a\x18\xF9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FRewardsCoordinator.setOperatorPI`D\x82\x01R\x7FSplit: caller is not the operato`d\x82\x01R`9`\xF9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a'\x10a\xFF\xFF\x83\x16\x11\x15a\x19\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FRewardsCoordinator.setOperatorPI`D\x82\x01R\x7FSplit: split must be <= 10000 bi`d\x82\x01Raps`\xF0\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCBT`\0\x90a\x19\x9D\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\xD5` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01` \x1B\x90\x92\x04c\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x91\x92P\x90a\x19\xF7\x90a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xD5` R`@\x90 \x90\x91Pa\x1A\x1D\x90\x85\x84a76V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81Ra\xFF\xFF\x83\x81\x16` \x83\x01R\x86\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x87\x16\x913\x91\x7F\xD1\xE0(\xBDfD\x86\xA4j\xD2`@\xE9\x99\xCD-\"\xE1\xE9\xA0\x94\xEEj\xFE\x19\xFC\xF6Fx\xF1ot\x91\x81\x90\x03``\x01\x90\xA3PPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x1A\x9BWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x1A\xB5WP0;\x15\x80\x15a\x1A\xB5WP`\0T`\xFF\x16`\x01\x14[a\x1B\x18W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x1B;W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x1BCa7\xD1V[`\xC9Ua\x1BP\x86\x86a8hV[a\x1BY\x87a2MV[a\x1Bb\x84a2\x9FV[a\x1Bk\x83a/\x0BV[a\x1Bt\x82a6\xCBV[\x80\x15a\x1B\xBAW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPPPV[`fT`\x06\x90`@\x90\x81\x16\x14\x15a\x1B\xECW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x1CuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FRewardsCoordinator.setOperatorAV`D\x82\x01R\x7FSSplit: caller is not the operat`d\x82\x01Ra7\xB9`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a'\x10a\xFF\xFF\x83\x16\x11\x15a\x1C\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRewardsCoordinator.setOperatorAV`D\x82\x01R\x7FSSplit: split must be <= 10000 b`d\x82\x01Rbips`\xE8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\xCBT`\0\x90a\x1D\x1A\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16BaNDV[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x81R\x82\x82 \x83Q``\x81\x01\x85R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01` \x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x91\x92Pa\x1D\x82\x90a.\xBDV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x91Pa\x1D\xB5\x90\x85\x84a76V[`@\x80Qc\xFF\xFF\xFF\xFF\x84\x16\x81Ra\xFF\xFF\x83\x81\x16` \x83\x01R\x86\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x92\x90\x89\x16\x913\x91\x7FH\xE1\x98\xB6\xAE5~R\x92\x04\xEES\xA8\xE5\x14\xC4p\xFFw\xD9\xCC\x8EOr\x07\xF8\xB5\xD4\x90\xAEi4\x91\x90\x81\x90\x03``\x01\x90\xA4PPPPPPV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R`\xCA\x82\x81T\x81\x10a\x1EQWa\x1EQaJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x90\x81R`\xD4` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x83Q``\x81\x01\x85R\x90Ta\xFF\xFF\x80\x82\x16\x83Rb\x01\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01` \x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x81\x01\x92\x90\x92R\x90a\x1F\x12\x90a.\xBDV[\x93\x92PPPV[`\xCAT`\0\x90[c\xFF\xFF\xFF\xFF\x81\x16\x15a\x1F\x85W\x82`\xCAa\x1F:`\x01\x84aP?V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1FPWa\x1FPaJCV[\x90`\0R` `\0 \x90`\x02\x02\x01`\0\x01T\x14\x15a\x1FsWa\x1F\x12`\x01\x82aP?V[\x80a\x1F}\x81aPdV[\x91PPa\x1F V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FRewardsCoordinator.getRootIndexF`D\x82\x01R\x7FromHash: root not found\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[a\x1F\xFCa'xV[`\x01`\x01`\xA0\x1B\x03\x81\x16a aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[a\x0B\xBC\x81a2MV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0F\x14\x15a \x9BWP`\xC9T\x90V[a \xA3a7\xD1V[\x90P\x90V[`\0`\x01a\r\x0E` \x84\x01\x84aE\xB1V[`fT`\x03\x90`\x08\x90\x81\x16\x14\x15a \xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x163\x14a!\x0CW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aM\xF0V[`\xCATc\xFF\xFF\xFF\xFF\x83\x16\x10a!}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Rp\r-\xCE\xCC-\x8D,\x84\x0EM\xED\xEE\x89-\xCC\x8C\xAF`{\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0`\xCA\x83c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\x98Wa!\x98aJCV[\x90`\0R` `\0 \x90`\x02\x02\x01\x90P\x80`\x01\x01`\x08\x90T\x90a\x01\0\n\x90\x04`\xFF\x16\x15a\"%W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Rt\x1C\x9B\xDB\xDD\x08\x18[\x1C\x99XY\x1EH\x19\x1A\\\xD8X\x9B\x19Y`Z\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01\x81\x01T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x10a\"\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FRewardsCoordinator.disableRoot: `D\x82\x01Ru\x1C\x9B\xDB\xDD\x08\x18[\x1C\x99XY\x1EH\x18X\xDD\x1A]\x98]\x19Y`R\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01\x81\x01\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`@Qc\xFF\xFF\xFF\xFF\x84\x16\x90\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E\x90`\0\x90\xA2PPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#f\x91\x90aJpV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aJ\x8DV[`fT\x19\x81\x19`fT\x19\x16\x14a$\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x0C\xF3V[`fT`\0\x90`\x01\x90\x81\x16\x14\x15a$tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[`\x02`\x97T\x14\x15a$\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a$\xBAWa$\xBAaJCV[\x90P` \x02\x81\x01\x90a$\xCC\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a$\xF7\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa%\x18\x83a(\xC9V[3`\0\x90\x81R`\xCF` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua%K\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x81\x90a%\x93\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a%\xB430`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[PPP\x80\x80a%\xC2\x90aM\xD5V[\x91PPa$\x9FV[`fT`\x04\x90`\x10\x90\x81\x16\x14\x15a%\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK<V[3`\0\x90\x81R`\xD1` R`@\x90 T`\xFF\x16a&\"W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aKsV[`\x02`\x97T\x14\x15a&EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x90aK\xEAV[`\x02`\x97U`\0[\x82\x81\x10\x15a\x0F3W6\x84\x84\x83\x81\x81\x10a&hWa&haJCV[\x90P` \x02\x81\x01\x90a&z\x91\x90aL!V[3`\0\x81\x81R`\xCE` \x90\x81R`@\x80\x83 T\x90Q\x94\x95P\x93\x91\x92a&\xA5\x92\x90\x91\x85\x91\x87\x91\x01aMzV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa&\xC6\x83a(\xC9V[3`\0\x90\x81R`\xD2` \x90\x81R`@\x80\x83 \x84\x84R\x90\x91R\x90 \x80T`\xFF\x19\x16`\x01\x90\x81\x17\x90\x91Ua&\xF9\x90\x83\x90aM\xAAV[3`\0\x81\x81R`\xCE` R`@\x90\x81\x90 \x92\x90\x92U\x90Q\x82\x91\x84\x91\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B\x90a'A\x90\x88\x90aM\xC2V[`@Q\x80\x91\x03\x90\xA4a'b30`@\x86\x01\x805\x90a\x0F\x0C\x90` \x89\x01aE\xB1V[PPP\x80\x80a'p\x90aM\xD5V[\x91PPa&MV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x0B\xAAV[`\x01`\x01`\xA0\x1B\x03\x81\x16a(`W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[a(\xFBa(\xD6\x82\x80aP\x84V[a(\xE6`\x80\x85\x01``\x86\x01aG\xE1V[a(\xF6`\xA0\x86\x01`\x80\x87\x01aG\xE1V[a9RV[`\0\x81`@\x015\x11a)\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`A`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: amount cannot be `d\x82\x01R`\x03`\xFC\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF\x81`@\x015\x11\x15a*\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: amount too large\0`d\x82\x01R`\x84\x01a\x0B\xAAV[a*9c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16BaM\xAAV[a*I`\x80\x83\x01``\x84\x01aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x11\x15a\x0B\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FRewardsCoordinator._validateRewa`D\x82\x01R\x7FrdsSubmission: startTimestamp to`d\x82\x01Rro far in the future`h\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra+J\x90\x85\x90c#\xB8r\xDD`\xE0\x1B\x90`\x84\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra?AV[PPPPV[`\0`\xCAa+a` \x85\x01\x85aG\xE1V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a+wWa+waJCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\x80\x81\x01\x82R`\x02\x93\x90\x93\x02\x90\x91\x01\x80T\x83R`\x01\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x94\x84\x01\x94\x90\x94R`\x01` \x1B\x81\x04\x90\x93\x16\x90\x82\x01R`\x01`@\x1B\x90\x91\x04`\xFF\x16\x15\x15``\x82\x01R\x90Pa+\xD8\x83\x82a/|V[`\0a+\xEA`\x80\x85\x01``\x86\x01aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16`\0\x90\x81R`\xCC` R`@\x90 T\x91\x92P\x16\x80a,\x10WP\x80[3`\x01`\x01`\xA0\x1B\x03\x82\x16\x14a,\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRewardsCoordinator.processClaim:`D\x82\x01R\x7F caller is not valid claimer\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0[a,\x9E`\xA0\x87\x01\x87aP\xCEV[\x90P\x81\x10\x15a.\xB5W6a,\xB5`\xE0\x88\x01\x88aP\x84V[\x83\x81\x81\x10a,\xC5Wa,\xC5aJCV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x80\x83 \x93\x02\x94\x90\x94\x01\x94P\x92\x90\x91P\x82\x90a,\xFA\x90\x85\x01\x85aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90P\x80\x82` \x015\x11a-\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FRewardsCoordinator.processClaim:`D\x82\x01R\x7F cumulativeEarnings must be gt t`d\x82\x01Rt\x1A\x18[\x88\x18\xDD[][\x18]\x1A]\x99P\xDB\x18Z[YY`Z\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0a-\xC4\x82` \x85\x015aJ,V[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCD` \x90\x81R`@\x82 \x92\x93P\x85\x01\x805\x92\x91\x90a-\xF2\x90\x87aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x91\x90\x91Ua.4\x90\x89\x90\x83\x90a.$\x90\x87\x01\x87aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x90a@\x13V[\x86Q`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x91\x87\x82\x16\x91\x89\x16\x90\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x90a.x` \x89\x01\x89aE\xB1V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x81\x01\x86\x90R``\x01`@Q\x80\x91\x03\x90\xA4PPP\x80\x80a.\xAD\x90aM\xD5V[\x91PPa,\x91V[PPPPPPV[`\0\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16`\0\x14\x15a.\xE5WPP`\xCBT`\x01`\xE0\x1B\x90\x04a\xFF\xFF\x16\x90V[\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a.\xFEW\x81Qa\x13qV[P` \x01Q\x90V[\x91\x90PV[`\xCBT`@\x80Qc\xFF\xFF\xFF\xFF`\x01`\xA0\x1B\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80Tc\xFF\xFF\xFF\xFF\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80``\x01Q\x15a/\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01Ro\x1C\x9B\xDB\xDD\x08\x1A\\\xC8\x19\x1A\\\xD8X\x9B\x19Y`\x82\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a0<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01Ru\x1C\x9B\xDB\xDD\x08\x1B\x9B\xDD\x08\x18X\xDD\x1A]\x98]\x19Y\x08\x1EY]`R\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[a0I`\xC0\x83\x01\x83aP\xCEV[\x90Pa0X`\xA0\x84\x01\x84aP\xCEV[\x90P\x14a0\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01R\x7FtokenIndices and tokenProofs len`d\x82\x01Rk\x0C\xEE\x8D\x04\r\xAD.m\xAC.\x8Cm`\xA3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a0\xDD`\xE0\x83\x01\x83aP\x84V[\x90Pa0\xEC`\xC0\x84\x01\x84aP\xCEV[\x90P\x14a1bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` aR2\x839\x81Q\x91R`D\x82\x01R\x7FtokenTreeProofs and leaves lengt`d\x82\x01Ri\r\x04\r\xAD.m\xAC.\x8Cm`\xB3\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x80Qa1\x8E\x90a1x`@\x85\x01` \x86\x01aG\xE1V[a1\x85`@\x86\x01\x86aQ\x18V[\x86``\x01a@CV[`\0[a1\x9E`\xA0\x84\x01\x84aP\xCEV[\x90P\x81\x10\x15a2HWa28`\x80\x84\x015a1\xBC`\xA0\x86\x01\x86aP\xCEV[\x84\x81\x81\x10a1\xCCWa1\xCCaJCV[\x90P` \x02\x01` \x81\x01\x90a1\xE1\x91\x90aG\xE1V[a1\xEE`\xC0\x87\x01\x87aP\xCEV[\x85\x81\x81\x10a1\xFEWa1\xFEaJCV[\x90P` \x02\x81\x01\x90a2\x10\x91\x90aQ\x18V[a2\x1D`\xE0\x89\x01\x89aP\x84V[\x87\x81\x81\x10a2-Wa2-aJCV[\x90P`@\x02\x01aA\xAFV[a2A\x81aM\xD5V[\x90Pa1\x91V[PPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x92\x16\x90\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB\x90`\0\x90\xA3`\xCB\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\0a3*a3\n\x83\x80aP\x84V[a3\x1A`\x80\x86\x01``\x87\x01aG\xE1V[a(\xF6`\xA0\x87\x01`\x80\x88\x01aG\xE1V[`\0a39`@\x84\x01\x84aP\x84V[\x90P\x11a3\xB9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R\x7FatorDirectedRewardsSubmission: n`d\x82\x01Rs\x1B\xC8\x1B\xDC\x19\\\x98]\x1B\xDC\x9C\xC8\x1C\x99]\xD8\\\x99\x19Y`b\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0\x80`\0[a3\xCC`@\x86\x01\x86aP\x84V[\x90P\x81\x10\x15a5\xFEW6a3\xE3`@\x87\x01\x87aP\x84V[\x83\x81\x81\x10a3\xF3Wa3\xF3aJCV[`@\x02\x91\x90\x91\x01\x91P`\0\x90Pa4\r` \x83\x01\x83aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a4\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator cannot be 0 address\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a4\x99` \x82\x01\x82aE\xB1V[`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a5GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`x`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperators must be in ascending or`\x84\x82\x01R\x7Fder to handle duplicates\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a5T` \x82\x01\x82aE\xB1V[\x92P`\0\x81` \x015\x11a5\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator reward amount cannot be `\x84\x82\x01R`\x03`\xFC\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a5\xEA` \x82\x015\x85aM\xAAV[\x93PP\x80a5\xF7\x90aM\xD5V[\x90Pa3\xBFV[PBa6\x10`\xA0\x86\x01`\x80\x87\x01aG\xE1V[a6 `\x80\x87\x01``\x88\x01aG\xE1V[a6*\x91\x90aNDV[c\xFF\xFF\xFF\xFF\x16\x10a6\xC4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aRr\x839\x81Q\x91R`D\x82\x01R`\0\x80Q` aRR\x839\x81Q\x91R`d\x82\x01R\x7Fperator-directed rewards submiss`\x84\x82\x01Ruion is not retroactive`P\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[P\x92\x91PPV[`\xCBT`@\x80Qa\xFF\xFF`\x01`\xE0\x1B\x90\x93\x04\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xE6\xCDN\xDF\xDC\xC1\xF6\xD10\xAB5\xF7=r7\x8F:d)D\xFBN\xE5\xBD\x84\xB7\x80z\x81\xEA\x1CN\x91\x01`@Q\x80\x91\x03\x90\xA1`\xCB\x80Ta\xFF\xFF\x90\x92\x16`\x01`\xE0\x1B\x02a\xFF\xFF`\xE0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[\x82T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16B\x10a7\x95W\x82T`\x01` \x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a7~W`\xCBT\x83T`\x01`\xE0\x1B\x90\x91\x04a\xFF\xFF\x16a\xFF\xFF\x19\x90\x91\x16\x17\x83Ua7\x95V[\x82Tb\x01\0\0\x81\x04a\xFF\xFF\x16a\xFF\xFF\x19\x90\x91\x16\x17\x83U[\x82Tc\xFF\xFF\xFF\xFF\x90\x91\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19a\xFF\xFF\x90\x93\x16b\x01\0\0\x02\x92\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x90\x91\x16\x17\x17\x90UV[`@\x80Q\x80\x82\x01\x82R`\n\x81Ri\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B` \x91\x82\x01R\x81Q\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x81\x83\x01R\x7Fq\xB6%\xCF\xADD\xBA\xC6;\x13\xDB\xA0\x7F.\x1D`\x84\xEE\x04\xB6\xF8u!\x01\xEC\xE6\x12mXN\xE6\xEA\x81\x84\x01RF``\x82\x01R0`\x80\x80\x83\x01\x91\x90\x91R\x83Q\x80\x83\x03\x90\x91\x01\x81R`\xA0\x90\x91\x01\x90\x92R\x81Q\x91\x01 \x90V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a8\x89WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a9\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a9N\x82a'\xD2V[PPV[\x82a9\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: no strategi`d\x82\x01Re\x19\\\xC8\x1C\xD9]`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x11\x15a:xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Z`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: duration ex`d\x82\x01R\x7Fceeds MAX_REWARDS_DURATION\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[a:\xA2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82aQuV[c\xFF\xFF\xFF\xFF\x16\x15a;HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`p`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: duration mu`d\x82\x01R\x7Fst be a multiple of CALCULATION_`\x84\x82\x01RoINTERVAL_SECONDS`\x80\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[a;r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83aQuV[c\xFF\xFF\xFF\xFF\x16\x15a<\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`v`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: startTimest`d\x82\x01R\x7Famp must be a multiple of CALCUL`\x84\x82\x01RuATION_INTERVAL_SECONDS`P\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[\x81c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16Ba<W\x91\x90aJ,V[\x11\x15\x80\x15a<\x91WP\x81c\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFF\x16\x11\x15[a=\x17W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`W`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: startTimest`d\x82\x01R\x7Famp too far in the past\0\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0\x80[\x84\x81\x10\x15a.\xB5W`\0\x86\x86\x83\x81\x81\x10a=7Wa=7aJCV[a=M\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91PaE\xB1V[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01R\x91\x92P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90cf<\x1D\xE4\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a=\xB8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xDC\x91\x90aJ\xD7V[\x80a>\x03WP`\x01`\x01`\xA0\x1B\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14[a>|W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: invalid str`d\x82\x01Ro\x18]\x19Y\xDEH\x18\xDB\xDB\x9C\xDAY\x19\\\x99Y`\x82\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a?/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`o`$\x82\x01R`\0\x80Q` aR\x92\x839\x81Q\x91R`D\x82\x01R\x7FonRewardsSubmission: strategies `d\x82\x01R\x7Fmust be in ascending order to ha`\x84\x82\x01Rnndle duplicates`\x88\x1B`\xA4\x82\x01R`\xC4\x01a\x0B\xAAV[\x91Pa?:\x81aM\xD5V[\x90Pa=\x1BV[`\0a?\x96\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aC\0\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a2HW\x80\x80` \x01\x90Q\x81\x01\x90a?\xB4\x91\x90aJ\xD7V[a2HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra2H\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01a+\x13V[a@N` \x83aQ\x98V[`\x01\x90\x1B\x84c\xFF\xFF\xFF\xFF\x16\x10a@\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FRewardsCoordinator._verifyEarner`D\x82\x01R\x7FClaimProof: invalid earnerLeafIn`d\x82\x01Rb\x0C\x8C\xAF`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[`\0a@\xE3\x82a\x0C\xFEV[\x90PaA.\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x89\x16aC\x17V[a.\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FRewardsCoordinator._verifyEarner`D\x82\x01R\x7FClaimProof: invalid earner claim`d\x82\x01Re\x10897\xB7\xB3`\xD1\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[aA\xBA` \x83aQ\x98V[`\x01\x90\x1B\x84c\xFF\xFF\xFF\xFF\x16\x10aB8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`<`$\x82\x01R\x7FRewardsCoordinator._verifyTokenC`D\x82\x01R\x7Flaim: invalid tokenLeafIndex\0\0\0\0`d\x82\x01R`\x84\x01a\x0B\xAAV[`\0aBC\x82a \xA8V[\x90PaB\x8E\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x85\x91PPc\xFF\xFF\xFF\xFF\x89\x16aC\x17V[a.\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FRewardsCoordinator._verifyTokenC`D\x82\x01R\x7Flaim: invalid token claim proof\0`d\x82\x01R`\x84\x01a\x0B\xAAV[``aC\x0F\x84\x84`\0\x85aC/V[\x94\x93PPPPV[`\0\x83aC%\x86\x85\x85aD`V[\x14\x95\x94PPPPPV[``\x82G\x10\x15aC\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x0B\xAAV[`\x01`\x01`\xA0\x1B\x03\x85\x16;aC\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x0B\xAAV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaD\x03\x91\x90aQ\xD8V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aD@W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aDEV[``\x91P[P\x91P\x91PaDU\x82\x82\x86aEcV[\x97\x96PPPPPPPV[`\0` \x84QaDp\x91\x90aQ\xEAV[\x15aD\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FMerkle.processInclusionProofKecc`D\x82\x01R\x7Fak: proof length should be a mul`d\x82\x01Rj:4\xB862\x907\xB3\x10\x19\x99`\xA9\x1B`\x84\x82\x01R`\xA4\x01a\x0B\xAAV[\x82` [\x85Q\x81\x11aEZWaE\x0E`\x02\x85aQ\xEAV[aE/W\x81`\0R\x80\x86\x01Q` R`@`\0 \x91P`\x02\x84\x04\x93PaEHV[\x80\x86\x01Q`\0R\x81` R`@`\0 \x91P`\x02\x84\x04\x93P[aES` \x82aM\xAAV[\x90PaD\xFBV[P\x94\x93PPPPV[``\x83\x15aErWP\x81a\x1F\x12V[\x82Q\x15aE\x82W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x0B\xAA\x91\x90aQ\xFEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xBCW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aE\xC3W`\0\x80\xFD[\x815a\x1F\x12\x81aE\x9CV[\x80\x15\x15\x81\x14a\x0B\xBCW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aE\xEFW`\0\x80\xFD[\x825aE\xFA\x81aE\x9CV[\x91P` \x83\x015aF\n\x81aE\xCEV[\x80\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15aF'W`\0\x80\xFD[P5\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aF@W`\0\x80\xFD[P\x91\x90PV[`\0`@\x82\x84\x03\x12\x15aFXW`\0\x80\xFD[a\x1F\x12\x83\x83aF.V[`\0\x80\x83`\x1F\x84\x01\x12aFtW`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\x8CW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aF\xA7W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15aF\xC1W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xD8W`\0\x80\xFD[aF\xE4\x85\x82\x86\x01aFbV[\x90\x96\x90\x95P\x93PPPPV[`\0a\x01\0\x82\x84\x03\x12\x15aF@W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aG\x16W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG-W`\0\x80\xFD[aG9\x85\x82\x86\x01aF\xF0V[\x92PP` \x83\x015aF\n\x81aE\x9CV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a/\x06W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15aGqW`\0\x80\xFD[\x825\x91PaG\x81` \x84\x01aGJV[\x90P\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15aG\x9FW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\xB6W`\0\x80\xFD[aG\xC2\x86\x82\x87\x01aFbV[\x90\x94P\x92PP` \x84\x015aG\xD6\x81aE\x9CV[\x80\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aG\xF3W`\0\x80\xFD[a\x1F\x12\x82aGJV[`\0` \x82\x84\x03\x12\x15aH\x0EW`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x1F\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aH1W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHHW`\0\x80\xFD[aC\x0F\x84\x82\x85\x01aF\xF0V[`\0\x80`@\x83\x85\x03\x12\x15aHgW`\0\x80\xFD[\x825aHr\x81aE\x9CV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15aH\x93W`\0\x80\xFD[\x825aH\x9E\x81aE\x9CV[\x91P` \x83\x015aF\n\x81aE\x9CV[`\0\x80`\0`@\x84\x86\x03\x12\x15aH\xC3W`\0\x80\xFD[\x835aH\xCE\x81aE\x9CV[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xEAW`\0\x80\xFD[aH\xF6\x86\x82\x87\x01aFbV[\x94\x97\x90\x96P\x93\x94PPPPV[\x805a\xFF\xFF\x81\x16\x81\x14a/\x06W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aI'W`\0\x80\xFD[a\x1F\x12\x82aI\x03V[`\0\x80`@\x83\x85\x03\x12\x15aICW`\0\x80\xFD[\x825aIN\x81aE\x9CV[\x91PaG\x81` \x84\x01aI\x03V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aIuW`\0\x80\xFD[\x865aI\x80\x81aE\x9CV[\x95P` \x87\x015aI\x90\x81aE\x9CV[\x94P`@\x87\x015\x93P``\x87\x015aI\xA7\x81aE\x9CV[\x92PaI\xB5`\x80\x88\x01aGJV[\x91PaI\xC3`\xA0\x88\x01aI\x03V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15aI\xE4W`\0\x80\xFD[\x835aI\xEF\x81aE\x9CV[\x92P` \x84\x015aI\xFF\x81aE\x9CV[\x91PaJ\r`@\x85\x01aI\x03V[\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15aJ>WaJ>aJ\x16V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81aJhWaJhaJ\x16V[P`\0\x19\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\x82W`\0\x80\xFD[\x81Qa\x1F\x12\x81aE\x9CV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\xE9W`\0\x80\xFD[\x81Qa\x1F\x12\x81aE\xCEV[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V[` \x80\x82R`Q\x90\x82\x01R\x7FRewardsCoordinator: caller is no`@\x82\x01R\x7Ft a valid createRewardsForAllSub``\x82\x01Rp6\xB4\xB9\xB9\xB4\xB7\xB7\x109\xBA\xB16\xB4\xBA:2\xB9`y\x1B`\x80\x82\x01R`\xA0\x01\x90V[` \x80\x82R`\x1F\x90\x82\x01R\x7FReentrancyGuard: reentrant call\0`@\x82\x01R``\x01\x90V[`\0\x825`\x9E\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aLXW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aLxW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aF\xA7W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0\x80[\x86\x81\x10\x15aL\xEFW\x825aL\xAE\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x16\x88R\x82\x84\x015k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x80\x82\x14aL\xD7W\x83\x84\xFD[\x89\x86\x01RP`@\x97\x88\x01\x97\x92\x90\x92\x01\x91`\x01\x01aL\x9BV[P\x95\x96\x95PPPPPPV[`\0aM\x07\x82\x83aLAV[`\xA0\x85RaM\x19`\xA0\x86\x01\x82\x84aL\x8AV[\x91PP` \x83\x015aM*\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x16` \x85\x01R`@\x83\x81\x015\x90\x85\x01RaMN``\x84\x01aGJV[c\xFF\xFF\xFF\xFF\x80\x82\x16``\x87\x01R\x80aMh`\x80\x87\x01aGJV[\x16`\x80\x87\x01RPP\x80\x91PP\x92\x91PPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aM\xA1``\x83\x01\x84aL\xFBV[\x95\x94PPPPPV[`\0\x82\x19\x82\x11\x15aM\xBDWaM\xBDaJ\x16V[P\x01\x90V[` \x81R`\0a\x1F\x12` \x83\x01\x84aL\xFBV[`\0`\0\x19\x82\x14\x15aM\xE9WaM\xE9aJ\x16V[P`\x01\x01\x90V[` \x80\x82R`4\x90\x82\x01R\x7FRewardsCoordinator: caller is no`@\x82\x01Rs:\x10:42\x9092\xBB\xB0\xB929\xAA\xB820\xBA2\xB9`a\x1B``\x82\x01R`\x80\x01\x90V[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aNcWaNcaJ\x16V[\x01\x94\x93PPPPV[`\0\x825`\xFE\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[`\0\x825`\xBE\x19\x836\x03\x01\x81\x12aL7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\xAFW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aN\xCFW`\0\x80\xFD[\x806\x03\x83\x13\x15aF\xA7W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0aO\x13\x82\x83aLAV[`\xC0\x85RaO%`\xC0\x86\x01\x82\x84aL\x8AV[\x91PP` \x80\x84\x015aO7\x81aE\x9CV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x86\x83\x01R`@\x90aOU\x86\x83\x01\x87aLAV[\x88\x86\x03\x84\x8A\x01R\x80\x86R\x90\x94`\0\x91\x90\x85\x01[\x81\x83\x10\x15aO\x99W\x865aO{\x81aE\x9CV[\x84\x16\x81R\x86\x86\x015\x86\x82\x01R\x95\x84\x01\x95`\x01\x92\x90\x92\x01\x91\x84\x01aOhV[aO\xA5``\x8A\x01aGJV[c\xFF\xFF\xFF\xFF\x81\x16``\x8C\x01R\x96PaO\xBF`\x80\x8A\x01aGJV[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x8C\x01R\x96PaO\xDA`\xA0\x8A\x01\x8AaN\x98V[\x97P\x95P\x89\x81\x03`\xA0\x8B\x01RaO\xF1\x81\x88\x88aN\xDEV[\x9A\x99PPPPPPPPPPV[`\x01\x80`\xA0\x1B\x03\x84\x16\x81R\x82` \x82\x01R```@\x82\x01R`\0aM\xA1``\x83\x01\x84aO\x07V[\x82\x81R`@` \x82\x01R`\0aC\x0F`@\x83\x01\x84aO\x07V[`\0c\xFF\xFF\xFF\xFF\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aP\\WaP\\aJ\x16V[\x03\x93\x92PPPV[`\0c\xFF\xFF\xFF\xFF\x82\x16\x80aPzWaPzaJ\x16V[`\0\x19\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\x9BW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aP\xB6W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aP\xE5W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQ\0W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aQ/W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aQJW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aF\xA7W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0c\xFF\xFF\xFF\xFF\x80\x84\x16\x80aQ\x8CWaQ\x8CaQ_V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0\x82aQ\xA7WaQ\xA7aQ_V[P\x04\x90V[`\0[\x83\x81\x10\x15aQ\xC7W\x81\x81\x01Q\x83\x82\x01R` \x01aQ\xAFV[\x83\x81\x11\x15a+JWPP`\0\x91\x01RV[`\0\x82QaL7\x81\x84` \x87\x01aQ\xACV[`\0\x82aQ\xF9WaQ\xF9aQ_V[P\x06\x90V[` \x81R`\0\x82Q\x80` \x84\x01RaR\x1D\x81`@\x85\x01` \x87\x01aQ\xACV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV\xFERewardsCoordinator._checkClaim: atorDirectedRewardsSubmission: oRewardsCoordinator._validateOperRewardsCoordinator._validateComm\xA2dipfsX\"\x12 \x08\x1Ep32!@\x01/\x1D\x9E\xF0\x9BU\xB4U\xE8\x88\xC9\x9CZ\x08\xA4xcQ1\xF1_\xF9y[dsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `AVSRewardsSubmissionCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e6281`.
    ```solidity
    event AVSRewardsSubmissionCreated(address indexed avs, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSRewardsSubmissionCreated {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub submissionNonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rewardsSubmissionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsSubmission:
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for AVSRewardsSubmissionCreated {
            type DataTuple<'a> = (IRewardsCoordinator::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "AVSRewardsSubmissionCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    69u8, 10u8, 54u8, 122u8, 56u8, 12u8, 78u8, 51u8, 158u8, 90u8, 231u8, 52u8,
                    12u8, 132u8, 100u8, 239u8, 39u8, 175u8, 119u8, 129u8, 173u8, 153u8, 69u8,
                    207u8, 232u8, 171u8, 216u8, 40u8, 248u8, 158u8, 98u8, 129u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    avs: topics.1,
                    submissionNonce: topics.2,
                    rewardsSubmissionHash: topics.3,
                    rewardsSubmission: data.0,
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
                    <IRewardsCoordinator::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
                        &self.rewardsSubmission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.avs.clone(),
                    self.submissionNonce.clone(),
                    self.rewardsSubmissionHash.clone(),
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
                    &self.avs,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.submissionNonce);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.rewardsSubmissionHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AVSRewardsSubmissionCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSRewardsSubmissionCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSRewardsSubmissionCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ActivationDelaySet(uint32,uint32)` and selector `0xaf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b3`.
    ```solidity
    event ActivationDelaySet(uint32 oldActivationDelay, uint32 newActivationDelay);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ActivationDelaySet {
        #[allow(missing_docs)]
        pub oldActivationDelay: u32,
        #[allow(missing_docs)]
        pub newActivationDelay: u32,
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
        impl alloy_sol_types::SolEvent for ActivationDelaySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivationDelaySet(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    175u8, 85u8, 124u8, 108u8, 2u8, 194u8, 8u8, 121u8, 72u8, 23u8, 167u8, 5u8,
                    96u8, 156u8, 250u8, 147u8, 95u8, 130u8, 115u8, 18u8, 161u8, 173u8, 253u8,
                    210u8, 100u8, 148u8, 182u8, 185u8, 93u8, 210u8, 180u8, 179u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldActivationDelay: data.0,
                    newActivationDelay: data.1,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.oldActivationDelay,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.newActivationDelay,
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
        impl alloy_sol_types::private::IntoLogData for ActivationDelaySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ActivationDelaySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ActivationDelaySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ClaimerForSet(address,address,address)` and selector `0xbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca312`.
    ```solidity
    event ClaimerForSet(address indexed earner, address indexed oldClaimer, address indexed claimer);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ClaimerForSet {
        #[allow(missing_docs)]
        pub earner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldClaimer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub claimer: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ClaimerForSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ClaimerForSet(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    186u8, 185u8, 71u8, 147u8, 77u8, 66u8, 224u8, 173u8, 32u8, 111u8, 37u8, 201u8,
                    202u8, 177u8, 139u8, 91u8, 182u8, 174u8, 20u8, 74u8, 207u8, 176u8, 15u8, 64u8,
                    180u8, 227u8, 170u8, 89u8, 89u8, 12u8, 163u8, 18u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    earner: topics.1,
                    oldClaimer: topics.2,
                    claimer: topics.3,
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
                    self.earner.clone(),
                    self.oldClaimer.clone(),
                    self.claimer.clone(),
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
                    &self.earner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldClaimer,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claimer,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ClaimerForSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ClaimerForSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ClaimerForSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DefaultOperatorSplitBipsSet(uint16,uint16)` and selector `0xe6cd4edfdcc1f6d130ab35f73d72378f3a642944fb4ee5bd84b7807a81ea1c4e`.
    ```solidity
    event DefaultOperatorSplitBipsSet(uint16 oldDefaultOperatorSplitBips, uint16 newDefaultOperatorSplitBips);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DefaultOperatorSplitBipsSet {
        #[allow(missing_docs)]
        pub oldDefaultOperatorSplitBips: u16,
        #[allow(missing_docs)]
        pub newDefaultOperatorSplitBips: u16,
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
        impl alloy_sol_types::SolEvent for DefaultOperatorSplitBipsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DefaultOperatorSplitBipsSet(uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    230u8, 205u8, 78u8, 223u8, 220u8, 193u8, 246u8, 209u8, 48u8, 171u8, 53u8,
                    247u8, 61u8, 114u8, 55u8, 143u8, 58u8, 100u8, 41u8, 68u8, 251u8, 78u8, 229u8,
                    189u8, 132u8, 183u8, 128u8, 122u8, 129u8, 234u8, 28u8, 78u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldDefaultOperatorSplitBips: data.0,
                    newDefaultOperatorSplitBips: data.1,
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.oldDefaultOperatorSplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.newDefaultOperatorSplitBips,
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
        impl alloy_sol_types::private::IntoLogData for DefaultOperatorSplitBipsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DefaultOperatorSplitBipsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DefaultOperatorSplitBipsSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DistributionRootDisabled(uint32)` and selector `0xd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e`.
    ```solidity
    event DistributionRootDisabled(uint32 indexed rootIndex);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DistributionRootDisabled {
        #[allow(missing_docs)]
        pub rootIndex: u32,
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
        impl alloy_sol_types::SolEvent for DistributionRootDisabled {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "DistributionRootDisabled(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    216u8, 80u8, 230u8, 229u8, 223u8, 164u8, 151u8, 183u8, 38u8, 97u8, 250u8,
                    115u8, 223u8, 41u8, 35u8, 70u8, 78u8, 174u8, 217u8, 220u8, 47u8, 241u8, 211u8,
                    203u8, 130u8, 188u8, 203u8, 254u8, 171u8, 229u8, 196u8, 30u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    rootIndex: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.rootIndex.clone())
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.rootIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DistributionRootDisabled {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DistributionRootDisabled> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DistributionRootDisabled) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DistributionRootSubmitted(uint32,bytes32,uint32,uint32)` and selector `0xecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd08`.
    ```solidity
    event DistributionRootSubmitted(uint32 indexed rootIndex, bytes32 indexed root, uint32 indexed rewardsCalculationEndTimestamp, uint32 activatedAt);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DistributionRootSubmitted {
        #[allow(missing_docs)]
        pub rootIndex: u32,
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsCalculationEndTimestamp: u32,
        #[allow(missing_docs)]
        pub activatedAt: u32,
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
        impl alloy_sol_types::SolEvent for DistributionRootSubmitted {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str =
                "DistributionRootSubmitted(uint32,bytes32,uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    236u8, 216u8, 102u8, 195u8, 193u8, 88u8, 250u8, 0u8, 191u8, 52u8, 216u8, 3u8,
                    213u8, 246u8, 2u8, 48u8, 0u8, 181u8, 112u8, 128u8, 188u8, 180u8, 138u8, 240u8,
                    4u8, 194u8, 180u8, 180u8, 107u8, 58u8, 253u8, 8u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    rootIndex: topics.1,
                    root: topics.2,
                    rewardsCalculationEndTimestamp: topics.3,
                    activatedAt: data.0,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.activatedAt,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.rootIndex.clone(),
                    self.root.clone(),
                    self.rewardsCalculationEndTimestamp.clone(),
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
                > as alloy_sol_types::EventTopic>::encode_topic(&self.rootIndex);
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.root);
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.rewardsCalculationEndTimestamp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for DistributionRootSubmitted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DistributionRootSubmitted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &DistributionRootSubmitted) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `OperatorAVSSplitBipsSet(address,address,address,uint32,uint16,uint16)` and selector `0x48e198b6ae357e529204ee53a8e514c470ff77d9cc8e4f7207f8b5d490ae6934`.
    ```solidity
    event OperatorAVSSplitBipsSet(address indexed caller, address indexed operator, address indexed avs, uint32 activatedAt, uint16 oldOperatorAVSSplitBips, uint16 newOperatorAVSSplitBips);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorAVSSplitBipsSet {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub activatedAt: u32,
        #[allow(missing_docs)]
        pub oldOperatorAVSSplitBips: u16,
        #[allow(missing_docs)]
        pub newOperatorAVSSplitBips: u16,
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
        impl alloy_sol_types::SolEvent for OperatorAVSSplitBipsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorAVSSplitBipsSet(address,address,address,uint32,uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    72u8, 225u8, 152u8, 182u8, 174u8, 53u8, 126u8, 82u8, 146u8, 4u8, 238u8, 83u8,
                    168u8, 229u8, 20u8, 196u8, 112u8, 255u8, 119u8, 217u8, 204u8, 142u8, 79u8,
                    114u8, 7u8, 248u8, 181u8, 212u8, 144u8, 174u8, 105u8, 52u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    caller: topics.1,
                    operator: topics.2,
                    avs: topics.3,
                    activatedAt: data.0,
                    oldOperatorAVSSplitBips: data.1,
                    newOperatorAVSSplitBips: data.2,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.activatedAt,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.oldOperatorAVSSplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorAVSSplitBips,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.caller.clone(),
                    self.operator.clone(),
                    self.avs.clone(),
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
                    &self.caller,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorAVSSplitBipsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorAVSSplitBipsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorAVSSplitBipsSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDirectedAVSRewardsSubmissionCreated(address,address,bytes32,uint256,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string))` and selector `0xfc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e0`.
    ```solidity
    event OperatorDirectedAVSRewardsSubmissionCreated(address indexed caller, address indexed avs, bytes32 indexed operatorDirectedRewardsSubmissionHash, uint256 submissionNonce, IRewardsCoordinator.OperatorDirectedRewardsSubmission operatorDirectedRewardsSubmission);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorDirectedAVSRewardsSubmissionCreated {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmissionHash: alloy::sol_types::private::FixedBytes<
            32,
        >,
        #[allow(missing_docs)]
        pub submissionNonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmission: <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorDirectedAVSRewardsSubmissionCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                IRewardsCoordinator::OperatorDirectedRewardsSubmission,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDirectedAVSRewardsSubmissionCreated(address,address,bytes32,uint256,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    252u8, 136u8, 136u8, 191u8, 253u8, 113u8, 29u8, 166u8, 11u8, 197u8, 9u8, 43u8,
                    51u8, 246u8, 119u8, 216u8, 24u8, 150u8, 254u8, 128u8, 236u8, 198u8, 119u8,
                    184u8, 76u8, 250u8, 184u8, 24u8, 68u8, 98u8, 182u8, 224u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    caller: topics.1,
                    avs: topics.2,
                    operatorDirectedRewardsSubmissionHash: topics.3,
                    submissionNonce: data.0,
                    operatorDirectedRewardsSubmission: data.1,
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.submissionNonce),
                    <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy_sol_types::SolType>::tokenize(
                        &self.operatorDirectedRewardsSubmission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.caller.clone(),
                    self.avs.clone(),
                    self.operatorDirectedRewardsSubmissionHash.clone(),
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
                    &self.caller,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operatorDirectedRewardsSubmissionHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorDirectedAVSRewardsSubmissionCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDirectedAVSRewardsSubmissionCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorDirectedAVSRewardsSubmissionCreated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorPISplitBipsSet(address,address,uint32,uint16,uint16)` and selector `0xd1e028bd664486a46ad26040e999cd2d22e1e9a094ee6afe19fcf64678f16f74`.
    ```solidity
    event OperatorPISplitBipsSet(address indexed caller, address indexed operator, uint32 activatedAt, uint16 oldOperatorPISplitBips, uint16 newOperatorPISplitBips);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorPISplitBipsSet {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub activatedAt: u32,
        #[allow(missing_docs)]
        pub oldOperatorPISplitBips: u16,
        #[allow(missing_docs)]
        pub newOperatorPISplitBips: u16,
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
        impl alloy_sol_types::SolEvent for OperatorPISplitBipsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorPISplitBipsSet(address,address,uint32,uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    209u8, 224u8, 40u8, 189u8, 102u8, 68u8, 134u8, 164u8, 106u8, 210u8, 96u8, 64u8,
                    233u8, 153u8, 205u8, 45u8, 34u8, 225u8, 233u8, 160u8, 148u8, 238u8, 106u8,
                    254u8, 25u8, 252u8, 246u8, 70u8, 120u8, 241u8, 111u8, 116u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    caller: topics.1,
                    operator: topics.2,
                    activatedAt: data.0,
                    oldOperatorPISplitBips: data.1,
                    newOperatorPISplitBips: data.2,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.activatedAt,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.oldOperatorPISplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorPISplitBips,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.caller.clone(),
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
                    &self.caller,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorPISplitBipsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorPISplitBipsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorPISplitBipsSet) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    110u8, 159u8, 205u8, 83u8, 152u8, 150u8, 252u8, 166u8, 14u8, 139u8, 15u8, 1u8,
                    221u8, 88u8, 2u8, 51u8, 228u8, 138u8, 107u8, 15u8, 125u8, 240u8, 19u8, 184u8,
                    155u8, 167u8, 245u8, 101u8, 134u8, 154u8, 205u8, 182u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
    /**Event with signature `RewardsClaimed(bytes32,address,address,address,address,uint256)` and selector `0x9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce31`.
    ```solidity
    event RewardsClaimed(bytes32 root, address indexed earner, address indexed claimer, address indexed recipient, address token, uint256 claimedAmount);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsClaimed {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub earner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub claimer: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub token: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub claimedAmount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for RewardsClaimed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "RewardsClaimed(bytes32,address,address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    149u8, 67u8, 219u8, 213u8, 85u8, 128u8, 132u8, 37u8, 134u8, 169u8, 81u8, 240u8,
                    56u8, 110u8, 36u8, 214u8, 138u8, 93u8, 249u8, 154u8, 226u8, 158u8, 59u8, 33u8,
                    101u8, 136u8, 180u8, 95u8, 214u8, 132u8, 206u8, 49u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    root: data.0,
                    earner: topics.1,
                    claimer: topics.2,
                    recipient: topics.3,
                    token: data.1,
                    claimedAmount: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.claimedAmount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.earner.clone(),
                    self.claimer.clone(),
                    self.recipient.clone(),
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
                    &self.earner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.claimer,
                );
                out[3usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsClaimed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsClaimed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsClaimed) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsForAllSubmitterSet(address,bool,bool)` and selector `0x4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c`.
    ```solidity
    event RewardsForAllSubmitterSet(address indexed rewardsForAllSubmitter, bool indexed oldValue, bool indexed newValue);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsForAllSubmitterSet {
        #[allow(missing_docs)]
        pub rewardsForAllSubmitter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub oldValue: bool,
        #[allow(missing_docs)]
        pub newValue: bool,
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
        impl alloy_sol_types::SolEvent for RewardsForAllSubmitterSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            const SIGNATURE: &'static str = "RewardsForAllSubmitterSet(address,bool,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    77u8, 230u8, 41u8, 62u8, 102u8, 141u8, 241u8, 57u8, 132u8, 34u8, 225u8, 222u8,
                    241u8, 33u8, 24u8, 5u8, 44u8, 21u8, 57u8, 160u8, 60u8, 191u8, 237u8, 193u8,
                    69u8, 137u8, 93u8, 72u8, 215u8, 104u8, 95u8, 28u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    rewardsForAllSubmitter: topics.1,
                    oldValue: topics.2,
                    newValue: topics.3,
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
                    self.rewardsForAllSubmitter.clone(),
                    self.oldValue.clone(),
                    self.newValue.clone(),
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
                    &self.rewardsForAllSubmitter,
                );
                out[2usize] =
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
                        &self.oldValue,
                    );
                out[3usize] =
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
                        &self.newValue,
                    );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsForAllSubmitterSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsForAllSubmitterSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsForAllSubmitterSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsSubmissionForAllCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf270482`.
    ```solidity
    event RewardsSubmissionForAllCreated(address indexed submitter, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsSubmissionForAllCreated {
        #[allow(missing_docs)]
        pub submitter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub submissionNonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rewardsSubmissionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsSubmission:
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for RewardsSubmissionForAllCreated {
            type DataTuple<'a> = (IRewardsCoordinator::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RewardsSubmissionForAllCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    81u8, 8u8, 139u8, 140u8, 137u8, 98u8, 141u8, 243u8, 168u8, 23u8, 64u8, 2u8,
                    194u8, 160u8, 52u8, 208u8, 21u8, 47u8, 206u8, 106u8, 248u8, 65u8, 93u8, 101u8,
                    27u8, 42u8, 71u8, 52u8, 191u8, 39u8, 4u8, 130u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    submitter: topics.1,
                    submissionNonce: topics.2,
                    rewardsSubmissionHash: topics.3,
                    rewardsSubmission: data.0,
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
                    <IRewardsCoordinator::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
                        &self.rewardsSubmission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.submitter.clone(),
                    self.submissionNonce.clone(),
                    self.rewardsSubmissionHash.clone(),
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
                    &self.submitter,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.submissionNonce);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.rewardsSubmissionHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsSubmissionForAllCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsSubmissionForAllCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsSubmissionForAllCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsSubmissionForAllEarnersCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b`.
    ```solidity
    event RewardsSubmissionForAllEarnersCreated(address indexed tokenHopper, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinator.RewardsSubmission rewardsSubmission);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsSubmissionForAllEarnersCreated {
        #[allow(missing_docs)]
        pub tokenHopper: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub submissionNonce: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub rewardsSubmissionHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsSubmission:
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for RewardsSubmissionForAllEarnersCreated {
            type DataTuple<'a> = (IRewardsCoordinator::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RewardsSubmissionForAllEarnersCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    82u8, 81u8, 182u8, 253u8, 239u8, 203u8, 93u8, 129u8, 20u8, 78u8, 115u8, 95u8,
                    105u8, 234u8, 76u8, 105u8, 95u8, 212u8, 59u8, 2u8, 137u8, 202u8, 83u8, 220u8,
                    7u8, 80u8, 51u8, 245u8, 252u8, 128u8, 6u8, 139u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    tokenHopper: topics.1,
                    submissionNonce: topics.2,
                    rewardsSubmissionHash: topics.3,
                    rewardsSubmission: data.0,
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
                    <IRewardsCoordinator::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
                        &self.rewardsSubmission,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.tokenHopper.clone(),
                    self.submissionNonce.clone(),
                    self.rewardsSubmissionHash.clone(),
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
                    &self.tokenHopper,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.submissionNonce);
                out[3usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.rewardsSubmissionHash,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsSubmissionForAllEarnersCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsSubmissionForAllEarnersCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsSubmissionForAllEarnersCreated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsUpdaterSet(address,address)` and selector `0x237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb`.
    ```solidity
    event RewardsUpdaterSet(address indexed oldRewardsUpdater, address indexed newRewardsUpdater);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RewardsUpdaterSet {
        #[allow(missing_docs)]
        pub oldRewardsUpdater: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newRewardsUpdater: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RewardsUpdaterSet {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RewardsUpdaterSet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 123u8, 130u8, 244u8, 56u8, 215u8, 95u8, 197u8, 104u8, 235u8, 171u8, 72u8,
                    75u8, 117u8, 176u8, 29u8, 146u8, 135u8, 185u8, 233u8, 139u8, 73u8, 11u8, 124u8,
                    35u8, 34u8, 22u8, 35u8, 182u8, 112u8, 93u8, 187u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldRewardsUpdater: topics.1,
                    newRewardsUpdater: topics.2,
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
                    self.oldRewardsUpdater.clone(),
                    self.newRewardsUpdater.clone(),
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
                    &self.oldRewardsUpdater,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newRewardsUpdater,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RewardsUpdaterSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsUpdaterSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RewardsUpdaterSet) -> alloy_sol_types::private::LogData {
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
    constructor(address _delegationManager, address _strategyManager, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 __GENESIS_REWARDS_TIMESTAMP);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _delegationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _strategyManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _CALCULATION_INTERVAL_SECONDS: u32,
        #[allow(missing_docs)]
        pub _MAX_REWARDS_DURATION: u32,
        #[allow(missing_docs)]
        pub _MAX_RETROACTIVE_LENGTH: u32,
        #[allow(missing_docs)]
        pub _MAX_FUTURE_LENGTH: u32,
        #[allow(missing_docs)]
        pub __GENESIS_REWARDS_TIMESTAMP: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u32,
                u32,
                u32,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (
                        value._delegationManager,
                        value._strategyManager,
                        value._CALCULATION_INTERVAL_SECONDS,
                        value._MAX_REWARDS_DURATION,
                        value._MAX_RETROACTIVE_LENGTH,
                        value._MAX_FUTURE_LENGTH,
                        value.__GENESIS_REWARDS_TIMESTAMP,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegationManager: tuple.0,
                        _strategyManager: tuple.1,
                        _CALCULATION_INTERVAL_SECONDS: tuple.2,
                        _MAX_REWARDS_DURATION: tuple.3,
                        _MAX_RETROACTIVE_LENGTH: tuple.4,
                        _MAX_FUTURE_LENGTH: tuple.5,
                        __GENESIS_REWARDS_TIMESTAMP: tuple.6,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
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
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategyManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._CALCULATION_INTERVAL_SECONDS,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_REWARDS_DURATION,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_RETROACTIVE_LENGTH,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_FUTURE_LENGTH,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.__GENESIS_REWARDS_TIMESTAMP,
                    ),
                )
            }
        }
    };
    /**Function with signature `CALCULATION_INTERVAL_SECONDS()` and selector `0x9d45c281`.
    ```solidity
    function CALCULATION_INTERVAL_SECONDS() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CALCULATION_INTERVAL_SECONDSCall {}
    ///Container type for the return parameters of the [`CALCULATION_INTERVAL_SECONDS()`](CALCULATION_INTERVAL_SECONDSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CALCULATION_INTERVAL_SECONDSReturn {
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
            impl ::core::convert::From<CALCULATION_INTERVAL_SECONDSCall> for UnderlyingRustTuple<'_> {
                fn from(value: CALCULATION_INTERVAL_SECONDSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CALCULATION_INTERVAL_SECONDSCall {
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
            impl ::core::convert::From<CALCULATION_INTERVAL_SECONDSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CALCULATION_INTERVAL_SECONDSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CALCULATION_INTERVAL_SECONDSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CALCULATION_INTERVAL_SECONDSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = CALCULATION_INTERVAL_SECONDSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CALCULATION_INTERVAL_SECONDS()";
            const SELECTOR: [u8; 4] = [157u8, 69u8, 194u8, 129u8];
            #[inline]
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
    /**Function with signature `GENESIS_REWARDS_TIMESTAMP()` and selector `0x131433b4`.
    ```solidity
    function GENESIS_REWARDS_TIMESTAMP() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_REWARDS_TIMESTAMPCall {}
    ///Container type for the return parameters of the [`GENESIS_REWARDS_TIMESTAMP()`](GENESIS_REWARDS_TIMESTAMPCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_REWARDS_TIMESTAMPReturn {
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
            impl ::core::convert::From<GENESIS_REWARDS_TIMESTAMPCall> for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_REWARDS_TIMESTAMPCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENESIS_REWARDS_TIMESTAMPCall {
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
            impl ::core::convert::From<GENESIS_REWARDS_TIMESTAMPReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_REWARDS_TIMESTAMPReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENESIS_REWARDS_TIMESTAMPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENESIS_REWARDS_TIMESTAMPCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = GENESIS_REWARDS_TIMESTAMPReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENESIS_REWARDS_TIMESTAMP()";
            const SELECTOR: [u8; 4] = [19u8, 20u8, 51u8, 180u8];
            #[inline]
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
    /**Function with signature `MAX_FUTURE_LENGTH()` and selector `0x04a0c502`.
    ```solidity
    function MAX_FUTURE_LENGTH() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_FUTURE_LENGTHCall {}
    ///Container type for the return parameters of the [`MAX_FUTURE_LENGTH()`](MAX_FUTURE_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_FUTURE_LENGTHReturn {
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
            impl ::core::convert::From<MAX_FUTURE_LENGTHCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_FUTURE_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_FUTURE_LENGTHCall {
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
            impl ::core::convert::From<MAX_FUTURE_LENGTHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_FUTURE_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_FUTURE_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_FUTURE_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_FUTURE_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_FUTURE_LENGTH()";
            const SELECTOR: [u8; 4] = [4u8, 160u8, 197u8, 2u8];
            #[inline]
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
    /**Function with signature `MAX_RETROACTIVE_LENGTH()` and selector `0x37838ed0`.
    ```solidity
    function MAX_RETROACTIVE_LENGTH() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RETROACTIVE_LENGTHCall {}
    ///Container type for the return parameters of the [`MAX_RETROACTIVE_LENGTH()`](MAX_RETROACTIVE_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RETROACTIVE_LENGTHReturn {
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
            impl ::core::convert::From<MAX_RETROACTIVE_LENGTHCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RETROACTIVE_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_RETROACTIVE_LENGTHCall {
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
            impl ::core::convert::From<MAX_RETROACTIVE_LENGTHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RETROACTIVE_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_RETROACTIVE_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_RETROACTIVE_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_RETROACTIVE_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_RETROACTIVE_LENGTH()";
            const SELECTOR: [u8; 4] = [55u8, 131u8, 142u8, 208u8];
            #[inline]
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
    /**Function with signature `MAX_REWARDS_DURATION()` and selector `0xbf21a8aa`.
    ```solidity
    function MAX_REWARDS_DURATION() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_REWARDS_DURATIONCall {}
    ///Container type for the return parameters of the [`MAX_REWARDS_DURATION()`](MAX_REWARDS_DURATIONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_REWARDS_DURATIONReturn {
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
            impl ::core::convert::From<MAX_REWARDS_DURATIONCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_REWARDS_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_REWARDS_DURATIONCall {
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
            impl ::core::convert::From<MAX_REWARDS_DURATIONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_REWARDS_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_REWARDS_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_REWARDS_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_REWARDS_DURATIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_REWARDS_DURATION()";
            const SELECTOR: [u8; 4] = [191u8, 33u8, 168u8, 170u8];
            #[inline]
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
    /**Function with signature `activationDelay()` and selector `0x3a8c0786`.
    ```solidity
    function activationDelay() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activationDelayCall {}
    ///Container type for the return parameters of the [`activationDelay()`](activationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activationDelayReturn {
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
            impl ::core::convert::From<activationDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: activationDelayCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activationDelayCall {
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
            impl ::core::convert::From<activationDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: activationDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activationDelayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = activationDelayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "activationDelay()";
            const SELECTOR: [u8; 4] = [58u8, 140u8, 7u8, 134u8];
            #[inline]
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
    /**Function with signature `calculateEarnerLeafHash((address,bytes32))` and selector `0x149bc872`.
    ```solidity
    function calculateEarnerLeafHash(IRewardsCoordinator.EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEarnerLeafHashCall {
        #[allow(missing_docs)]
        pub leaf:
            <IRewardsCoordinator::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateEarnerLeafHash((address,bytes32))`](calculateEarnerLeafHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEarnerLeafHashReturn {
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
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::EarnerTreeMerkleLeaf,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinator::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateEarnerLeafHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateEarnerLeafHashCall) -> Self {
                    (value.leaf,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateEarnerLeafHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { leaf: tuple.0 }
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
            impl ::core::convert::From<calculateEarnerLeafHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateEarnerLeafHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateEarnerLeafHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateEarnerLeafHashCall {
            type Parameters<'a> = (IRewardsCoordinator::EarnerTreeMerkleLeaf,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateEarnerLeafHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateEarnerLeafHash((address,bytes32))";
            const SELECTOR: [u8; 4] = [20u8, 155u8, 200u8, 114u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRewardsCoordinator::EarnerTreeMerkleLeaf as alloy_sol_types::SolType>::tokenize(
                        &self.leaf,
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
    /**Function with signature `calculateTokenLeafHash((address,uint256))` and selector `0xf8cd8448`.
    ```solidity
    function calculateTokenLeafHash(IRewardsCoordinator.TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateTokenLeafHashCall {
        #[allow(missing_docs)]
        pub leaf: <IRewardsCoordinator::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateTokenLeafHash((address,uint256))`](calculateTokenLeafHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateTokenLeafHashReturn {
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
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::TokenTreeMerkleLeaf,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinator::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateTokenLeafHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateTokenLeafHashCall) -> Self {
                    (value.leaf,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateTokenLeafHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { leaf: tuple.0 }
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
            impl ::core::convert::From<calculateTokenLeafHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: calculateTokenLeafHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateTokenLeafHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateTokenLeafHashCall {
            type Parameters<'a> = (IRewardsCoordinator::TokenTreeMerkleLeaf,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateTokenLeafHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateTokenLeafHash((address,uint256))";
            const SELECTOR: [u8; 4] = [248u8, 205u8, 132u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRewardsCoordinator::TokenTreeMerkleLeaf as alloy_sol_types::SolType>::tokenize(
                        &self.leaf,
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
    /**Function with signature `checkClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]))` and selector `0x5e9d8348`.
    ```solidity
    function checkClaim(IRewardsCoordinator.RewardsMerkleClaim memory claim) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkClaimCall {
        #[allow(missing_docs)]
        pub claim: <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]))`](checkClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkClaimReturn {
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
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::RewardsMerkleClaim,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkClaimCall) -> Self {
                    (value.claim,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claim: tuple.0 }
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
            impl ::core::convert::From<checkClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkClaimReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkClaimCall {
            type Parameters<'a> = (IRewardsCoordinator::RewardsMerkleClaim,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkClaimReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]))";
            const SELECTOR: [u8; 4] = [94u8, 157u8, 131u8, 72u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRewardsCoordinator::RewardsMerkleClaim as alloy_sol_types::SolType>::tokenize(
                        &self.claim,
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
    /**Function with signature `claimerFor(address)` and selector `0x2b9f64a4`.
    ```solidity
    function claimerFor(address) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`claimerFor(address)`](claimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForReturn {
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
            impl ::core::convert::From<claimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimerForCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimerForCall {
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
            impl ::core::convert::From<claimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimerForReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimerForReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimerFor(address)";
            const SELECTOR: [u8; 4] = [43u8, 159u8, 100u8, 164u8];
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
    ```solidity
    function createAVSRewardsSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
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
                    IRewardsCoordinator::RewardsSubmission,
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
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(address,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0x9cb9a5fa`.
    ```solidity
    function createOperatorDirectedAVSRewardsSubmission(address avs, IRewardsCoordinator.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorDirectedAVSRewardsSubmission(address,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])`](createOperatorDirectedAVSRewardsSubmissionCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
                    (value.avs, value.operatorDirectedRewardsSubmissions)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for createOperatorDirectedAVSRewardsSubmissionCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorDirectedRewardsSubmissions: tuple.1,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorDirectedAVSRewardsSubmission(address,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])";
            const SELECTOR: [u8; 4] = [156u8, 185u8, 165u8, 250u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        IRewardsCoordinator::OperatorDirectedRewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.operatorDirectedRewardsSubmissions,
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
    /**Function with signature `createRewardsForAllEarners(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xff9f6cce`.
    ```solidity
    function createRewardsForAllEarners(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllEarnersCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createRewardsForAllEarners(((address,uint96)[],address,uint256,uint32,uint32)[])`](createRewardsForAllEarnersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllEarnersReturn {}
    #[allow(
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createRewardsForAllEarnersCall> for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllEarnersCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createRewardsForAllEarnersCall {
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
            impl ::core::convert::From<createRewardsForAllEarnersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllEarnersReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createRewardsForAllEarnersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createRewardsForAllEarnersCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createRewardsForAllEarnersReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "createRewardsForAllEarners(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [255u8, 159u8, 108u8, 206u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::RewardsSubmission,
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
    /**Function with signature `createRewardsForAllSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0x36af41fa`.
    ```solidity
    function createRewardsForAllSubmission(IRewardsCoordinator.RewardsSubmission[] memory rewardsSubmissions) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllSubmissionCall {
        #[allow(missing_docs)]
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createRewardsForAllSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])`](createRewardsForAllSubmissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllSubmissionReturn {}
    #[allow(
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
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createRewardsForAllSubmissionCall> for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createRewardsForAllSubmissionCall {
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
            impl ::core::convert::From<createRewardsForAllSubmissionReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createRewardsForAllSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createRewardsForAllSubmissionCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsSubmission>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createRewardsForAllSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createRewardsForAllSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [54u8, 175u8, 65u8, 250u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IRewardsCoordinator::RewardsSubmission,
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
    /**Function with signature `cumulativeClaimed(address,address)` and selector `0x865c6953`.
    ```solidity
    function cumulativeClaimed(address, address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeClaimed(address,address)`](cumulativeClaimedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedReturn {
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
            impl ::core::convert::From<cumulativeClaimedCall> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeClaimedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<cumulativeClaimedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cumulativeClaimedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeClaimedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeClaimedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cumulativeClaimed(address,address)";
            const SELECTOR: [u8; 4] = [134u8, 92u8, 105u8, 83u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `currRewardsCalculationEndTimestamp()` and selector `0x4d18cc35`.
    ```solidity
    function currRewardsCalculationEndTimestamp() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currRewardsCalculationEndTimestampCall {}
    ///Container type for the return parameters of the [`currRewardsCalculationEndTimestamp()`](currRewardsCalculationEndTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currRewardsCalculationEndTimestampReturn {
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
            impl ::core::convert::From<currRewardsCalculationEndTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: currRewardsCalculationEndTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currRewardsCalculationEndTimestampCall {
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
            impl ::core::convert::From<currRewardsCalculationEndTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currRewardsCalculationEndTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currRewardsCalculationEndTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currRewardsCalculationEndTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currRewardsCalculationEndTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currRewardsCalculationEndTimestamp()";
            const SELECTOR: [u8; 4] = [77u8, 24u8, 204u8, 53u8];
            #[inline]
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
    /**Function with signature `defaultOperatorSplitBips()` and selector `0x63f6a798`.
    ```solidity
    function defaultOperatorSplitBips() external view returns (uint16);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultOperatorSplitBipsCall {}
    ///Container type for the return parameters of the [`defaultOperatorSplitBips()`](defaultOperatorSplitBipsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct defaultOperatorSplitBipsReturn {
        #[allow(missing_docs)]
        pub _0: u16,
    }
    #[allow(
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
            impl ::core::convert::From<defaultOperatorSplitBipsCall> for UnderlyingRustTuple<'_> {
                fn from(value: defaultOperatorSplitBipsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defaultOperatorSplitBipsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defaultOperatorSplitBipsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: defaultOperatorSplitBipsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for defaultOperatorSplitBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defaultOperatorSplitBipsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = defaultOperatorSplitBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "defaultOperatorSplitBips()";
            const SELECTOR: [u8; 4] = [99u8, 246u8, 167u8, 152u8];
            #[inline]
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
    /**Function with signature `delegationManager()` and selector `0xea4d3c9b`.
    ```solidity
    function delegationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerCall {}
    ///Container type for the return parameters of the [`delegationManager()`](delegationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationManagerReturn {
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
            impl ::core::convert::From<delegationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationManagerCall {
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
            impl ::core::convert::From<delegationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegationManager()";
            const SELECTOR: [u8; 4] = [234u8, 77u8, 60u8, 155u8];
            #[inline]
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
    /**Function with signature `disableRoot(uint32)` and selector `0xf96abf2e`.
    ```solidity
    function disableRoot(uint32 rootIndex) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableRootCall {
        #[allow(missing_docs)]
        pub rootIndex: u32,
    }
    ///Container type for the return parameters of the [`disableRoot(uint32)`](disableRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct disableRootReturn {}
    #[allow(
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
            impl ::core::convert::From<disableRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: disableRootCall) -> Self {
                    (value.rootIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rootIndex: tuple.0 }
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
            impl ::core::convert::From<disableRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: disableRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for disableRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for disableRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "disableRoot(uint32)";
            const SELECTOR: [u8; 4] = [249u8, 106u8, 191u8, 46u8];
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
                        &self.rootIndex,
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
    /**Function with signature `getCurrentClaimableDistributionRoot()` and selector `0x0e9a53cf`.
    ```solidity
    function getCurrentClaimableDistributionRoot() external view returns (IRewardsCoordinator.DistributionRoot memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentClaimableDistributionRootCall {}
    ///Container type for the return parameters of the [`getCurrentClaimableDistributionRoot()`](getCurrentClaimableDistributionRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentClaimableDistributionRootReturn {
        #[allow(missing_docs)]
        pub _0: <IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getCurrentClaimableDistributionRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentClaimableDistributionRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentClaimableDistributionRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentClaimableDistributionRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentClaimableDistributionRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentClaimableDistributionRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentClaimableDistributionRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentClaimableDistributionRootReturn;
            type ReturnTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentClaimableDistributionRoot()";
            const SELECTOR: [u8; 4] = [14u8, 154u8, 83u8, 207u8];
            #[inline]
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
    /**Function with signature `getCurrentDistributionRoot()` and selector `0x9be3d4e4`.
    ```solidity
    function getCurrentDistributionRoot() external view returns (IRewardsCoordinator.DistributionRoot memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDistributionRootCall {}
    ///Container type for the return parameters of the [`getCurrentDistributionRoot()`](getCurrentDistributionRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDistributionRootReturn {
        #[allow(missing_docs)]
        pub _0: <IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getCurrentDistributionRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentDistributionRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentDistributionRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentDistributionRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentDistributionRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentDistributionRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentDistributionRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentDistributionRootReturn;
            type ReturnTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentDistributionRoot()";
            const SELECTOR: [u8; 4] = [155u8, 227u8, 212u8, 228u8];
            #[inline]
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
    /**Function with signature `getDistributionRootAtIndex(uint256)` and selector `0xde02e503`.
    ```solidity
    function getDistributionRootAtIndex(uint256 index) external view returns (IRewardsCoordinator.DistributionRoot memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootAtIndexCall {
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getDistributionRootAtIndex(uint256)`](getDistributionRootAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootAtIndexReturn {
        #[allow(missing_docs)]
        pub _0: <IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getDistributionRootAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDistributionRootAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IRewardsCoordinator::DistributionRoot as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getDistributionRootAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDistributionRootAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDistributionRootAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDistributionRootAtIndexReturn;
            type ReturnTuple<'a> = (IRewardsCoordinator::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDistributionRootAtIndex(uint256)";
            const SELECTOR: [u8; 4] = [222u8, 2u8, 229u8, 3u8];
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
    /**Function with signature `getDistributionRootsLength()` and selector `0x7b8f8b05`.
    ```solidity
    function getDistributionRootsLength() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootsLengthCall {}
    ///Container type for the return parameters of the [`getDistributionRootsLength()`](getDistributionRootsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootsLengthReturn {
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
            impl ::core::convert::From<getDistributionRootsLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootsLengthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDistributionRootsLengthCall {
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
            impl ::core::convert::From<getDistributionRootsLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getDistributionRootsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDistributionRootsLengthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDistributionRootsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getDistributionRootsLength()";
            const SELECTOR: [u8; 4] = [123u8, 143u8, 139u8, 5u8];
            #[inline]
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
    /**Function with signature `getOperatorAVSSplit(address,address)` and selector `0xe063f81f`.
    ```solidity
    function getOperatorAVSSplit(address operator, address avs) external view returns (uint16);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorAVSSplitCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorAVSSplit(address,address)`](getOperatorAVSSplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorAVSSplitReturn {
        #[allow(missing_docs)]
        pub _0: u16,
    }
    #[allow(
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
            impl ::core::convert::From<getOperatorAVSSplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorAVSSplitCall) -> Self {
                    (value.operator, value.avs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorAVSSplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorAVSSplitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorAVSSplitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorAVSSplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorAVSSplitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorAVSSplitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorAVSSplit(address,address)";
            const SELECTOR: [u8; 4] = [224u8, 99u8, 248u8, 31u8];
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
    /**Function with signature `getOperatorPISplit(address)` and selector `0x4b943960`.
    ```solidity
    function getOperatorPISplit(address operator) external view returns (uint16);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorPISplitCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorPISplit(address)`](getOperatorPISplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorPISplitReturn {
        #[allow(missing_docs)]
        pub _0: u16,
    }
    #[allow(
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
            impl ::core::convert::From<getOperatorPISplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorPISplitCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorPISplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorPISplitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorPISplitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorPISplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorPISplitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorPISplitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorPISplit(address)";
            const SELECTOR: [u8; 4] = [75u8, 148u8, 57u8, 96u8];
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
    /**Function with signature `getRootIndexFromHash(bytes32)` and selector `0xe810ce21`.
    ```solidity
    function getRootIndexFromHash(bytes32 rootHash) external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRootIndexFromHashCall {
        #[allow(missing_docs)]
        pub rootHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRootIndexFromHash(bytes32)`](getRootIndexFromHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRootIndexFromHashReturn {
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
            impl ::core::convert::From<getRootIndexFromHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRootIndexFromHashCall) -> Self {
                    (value.rootHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRootIndexFromHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { rootHash: tuple.0 }
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
            impl ::core::convert::From<getRootIndexFromHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRootIndexFromHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRootIndexFromHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRootIndexFromHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRootIndexFromHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRootIndexFromHash(bytes32)";
            const SELECTOR: [u8; 4] = [232u8, 16u8, 206u8, 33u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.rootHash),
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
    /**Function with signature `initialize(address,address,uint256,address,uint32,uint16)` and selector `0xd4540a55`.
    ```solidity
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _defaultSplitBips) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _rewardsUpdater: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _activationDelay: u32,
        #[allow(missing_docs)]
        pub _defaultSplitBips: u16,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint256,address,uint32,uint16)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u32,
                u16,
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
                        value._pauserRegistry,
                        value.initialPausedStatus,
                        value._rewardsUpdater,
                        value._activationDelay,
                        value._defaultSplitBips,
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
                        _rewardsUpdater: tuple.3,
                        _activationDelay: tuple.4,
                        _defaultSplitBips: tuple.5,
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
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initialize(address,address,uint256,address,uint32,uint16)";
            const SELECTOR: [u8; 4] = [212u8, 84u8, 10u8, 85u8];
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.initialPausedStatus,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsUpdater,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._activationDelay,
                    ),
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self._defaultSplitBips,
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
    /**Function with signature `isAVSRewardsSubmissionHash(address,bytes32)` and selector `0x6d21117e`.
    ```solidity
    function isAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isAVSRewardsSubmissionHash(address,bytes32)`](isAVSRewardsSubmissionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashReturn {
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isAVSRewardsSubmissionHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isAVSRewardsSubmissionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isAVSRewardsSubmissionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isAVSRewardsSubmissionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isAVSRewardsSubmissionHash(address,bytes32)";
            const SELECTOR: [u8; 4] = [109u8, 33u8, 17u8, 126u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)` and selector `0xed71e6a2`.
    ```solidity
    function isOperatorDirectedAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorDirectedAVSRewardsSubmissionHashCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)`](isOperatorDirectedAVSRewardsSubmissionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorDirectedAVSRewardsSubmissionHashReturn {
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
            impl ::core::convert::From<isOperatorDirectedAVSRewardsSubmissionHashCall>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: isOperatorDirectedAVSRewardsSubmissionHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for isOperatorDirectedAVSRewardsSubmissionHashCall
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isOperatorDirectedAVSRewardsSubmissionHashReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: isOperatorDirectedAVSRewardsSubmissionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for isOperatorDirectedAVSRewardsSubmissionHashReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorDirectedAVSRewardsSubmissionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorDirectedAVSRewardsSubmissionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)";
            const SELECTOR: [u8; 4] = [237u8, 113u8, 230u8, 162u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isRewardsForAllSubmitter(address)` and selector `0x0018572c`.
    ```solidity
    function isRewardsForAllSubmitter(address) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isRewardsForAllSubmitter(address)`](isRewardsForAllSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterReturn {
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
            impl ::core::convert::From<isRewardsForAllSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsForAllSubmitterCall {
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
            impl ::core::convert::From<isRewardsForAllSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsForAllSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsForAllSubmitterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsForAllSubmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRewardsForAllSubmitter(address)";
            const SELECTOR: [u8; 4] = [0u8, 24u8, 87u8, 44u8];
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
    /**Function with signature `isRewardsSubmissionForAllEarnersHash(address,bytes32)` and selector `0xaebd8bae`.
    ```solidity
    function isRewardsSubmissionForAllEarnersHash(address, bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllEarnersHash(address,bytes32)`](isRewardsSubmissionForAllEarnersHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashReturn {
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsSubmissionForAllEarnersHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsSubmissionForAllEarnersHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsSubmissionForAllEarnersHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsSubmissionForAllEarnersHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRewardsSubmissionForAllEarnersHash(address,bytes32)";
            const SELECTOR: [u8; 4] = [174u8, 189u8, 139u8, 174u8];
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isRewardsSubmissionForAllHash(address,bytes32)` and selector `0xc46db606`.
    ```solidity
    function isRewardsSubmissionForAllHash(address, bytes32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllHash(address,bytes32)`](isRewardsSubmissionForAllHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashReturn {
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsSubmissionForAllHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isRewardsSubmissionForAllHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsSubmissionForAllHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsSubmissionForAllHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isRewardsSubmissionForAllHash(address,bytes32)";
            const SELECTOR: [u8; 4] = [196u8, 109u8, 182u8, 6u8];
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
    /**Function with signature `processClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]),address)` and selector `0x3ccc861d`.
    ```solidity
    function processClaim(IRewardsCoordinator.RewardsMerkleClaim memory claim, address recipient) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimCall {
        #[allow(missing_docs)]
        pub claim: <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`processClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]),address)`](processClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimReturn {}
    #[allow(
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
                IRewardsCoordinator::RewardsMerkleClaim,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<processClaimCall> for UnderlyingRustTuple<'_> {
                fn from(value: processClaimCall) -> Self {
                    (value.claim, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processClaimCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        claim: tuple.0,
                        recipient: tuple.1,
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
            impl ::core::convert::From<processClaimReturn> for UnderlyingRustTuple<'_> {
                fn from(value: processClaimReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processClaimReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processClaimCall {
            type Parameters<'a> = (
                IRewardsCoordinator::RewardsMerkleClaim,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = processClaimReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]),address)";
            const SELECTOR: [u8; 4] = [60u8, 204u8, 134u8, 29u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IRewardsCoordinator::RewardsMerkleClaim as alloy_sol_types::SolType>::tokenize(
                        &self.claim,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
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
    /**Function with signature `processClaims((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[])[],address)` and selector `0x4596021c`.
    ```solidity
    function processClaims(IRewardsCoordinator.RewardsMerkleClaim[] memory claims, address recipient) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimsCall {
        #[allow(missing_docs)]
        pub claims: alloy::sol_types::private::Vec<
            <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
        >,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`processClaims((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[])[],address)`](processClaimsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsMerkleClaim>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
                >,
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
            impl ::core::convert::From<processClaimsCall> for UnderlyingRustTuple<'_> {
                fn from(value: processClaimsCall) -> Self {
                    (value.claims, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processClaimsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        claims: tuple.0,
                        recipient: tuple.1,
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
            impl ::core::convert::From<processClaimsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: processClaimsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for processClaimsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processClaimsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<IRewardsCoordinator::RewardsMerkleClaim>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = processClaimsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processClaims((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[])[],address)";
            const SELECTOR: [u8; 4] = [69u8, 150u8, 2u8, 28u8];
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
                        IRewardsCoordinator::RewardsMerkleClaim,
                    > as alloy_sol_types::SolType>::tokenize(&self.claims),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
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
    /**Function with signature `rewardsUpdater()` and selector `0xfbf1e2c1`.
    ```solidity
    function rewardsUpdater() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsUpdaterCall {}
    ///Container type for the return parameters of the [`rewardsUpdater()`](rewardsUpdaterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct rewardsUpdaterReturn {
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
            impl ::core::convert::From<rewardsUpdaterCall> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsUpdaterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsUpdaterCall {
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
            impl ::core::convert::From<rewardsUpdaterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: rewardsUpdaterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for rewardsUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsUpdaterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsUpdaterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "rewardsUpdater()";
            const SELECTOR: [u8; 4] = [251u8, 241u8, 226u8, 193u8];
            #[inline]
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
    /**Function with signature `setActivationDelay(uint32)` and selector `0x58baaa3e`.
    ```solidity
    function setActivationDelay(uint32 _activationDelay) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setActivationDelayCall {
        #[allow(missing_docs)]
        pub _activationDelay: u32,
    }
    ///Container type for the return parameters of the [`setActivationDelay(uint32)`](setActivationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setActivationDelayReturn {}
    #[allow(
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
            impl ::core::convert::From<setActivationDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: setActivationDelayCall) -> Self {
                    (value._activationDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setActivationDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _activationDelay: tuple.0,
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
            impl ::core::convert::From<setActivationDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setActivationDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setActivationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setActivationDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setActivationDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setActivationDelay(uint32)";
            const SELECTOR: [u8; 4] = [88u8, 186u8, 170u8, 62u8];
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
                        &self._activationDelay,
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
    /**Function with signature `setDefaultOperatorSplit(uint16)` and selector `0xa50a1d9c`.
    ```solidity
    function setDefaultOperatorSplit(uint16 split) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDefaultOperatorSplitCall {
        #[allow(missing_docs)]
        pub split: u16,
    }
    ///Container type for the return parameters of the [`setDefaultOperatorSplit(uint16)`](setDefaultOperatorSplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setDefaultOperatorSplitReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u16,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setDefaultOperatorSplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: setDefaultOperatorSplitCall) -> Self {
                    (value.split,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setDefaultOperatorSplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { split: tuple.0 }
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
            impl ::core::convert::From<setDefaultOperatorSplitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setDefaultOperatorSplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setDefaultOperatorSplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setDefaultOperatorSplitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setDefaultOperatorSplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setDefaultOperatorSplit(uint16)";
            const SELECTOR: [u8; 4] = [165u8, 10u8, 29u8, 156u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.split,
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
    /**Function with signature `setOperatorAVSSplit(address,address,uint16)` and selector `0xdcbb03b3`.
    ```solidity
    function setOperatorAVSSplit(address operator, address avs, uint16 split) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorAVSSplitCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub split: u16,
    }
    ///Container type for the return parameters of the [`setOperatorAVSSplit(address,address,uint16)`](setOperatorAVSSplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorAVSSplitReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u16,
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
            impl ::core::convert::From<setOperatorAVSSplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorAVSSplitCall) -> Self {
                    (value.operator, value.avs, value.split)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorAVSSplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        split: tuple.2,
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
            impl ::core::convert::From<setOperatorAVSSplitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorAVSSplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorAVSSplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorAVSSplitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorAVSSplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorAVSSplit(address,address,uint16)";
            const SELECTOR: [u8; 4] = [220u8, 187u8, 3u8, 179u8];
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.split,
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
    /**Function with signature `setOperatorPISplit(address,uint16)` and selector `0xb3dbb0e0`.
    ```solidity
    function setOperatorPISplit(address operator, uint16 split) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorPISplitCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub split: u16,
    }
    ///Container type for the return parameters of the [`setOperatorPISplit(address,uint16)`](setOperatorPISplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorPISplitReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u16);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorPISplitCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorPISplitCall) -> Self {
                    (value.operator, value.split)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorPISplitCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        split: tuple.1,
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
            impl ::core::convert::From<setOperatorPISplitReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorPISplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorPISplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorPISplitCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorPISplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperatorPISplit(address,uint16)";
            const SELECTOR: [u8; 4] = [179u8, 219u8, 176u8, 224u8];
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
                    <alloy::sol_types::sol_data::Uint<16> as alloy_sol_types::SolType>::tokenize(
                        &self.split,
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
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
    ```solidity
    function setPauserRegistry(address newPauserRegistry) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setPauserRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newPauserRegistry: tuple.0,
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
            impl ::core::convert::From<setPauserRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `setRewardsForAllSubmitter(address,bool)` and selector `0x0eb38345`.
    ```solidity
    function setRewardsForAllSubmitter(address _submitter, bool _newValue) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsForAllSubmitterCall {
        #[allow(missing_docs)]
        pub _submitter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _newValue: bool,
    }
    ///Container type for the return parameters of the [`setRewardsForAllSubmitter(address,bool)`](setRewardsForAllSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsForAllSubmitterReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsForAllSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsForAllSubmitterCall) -> Self {
                    (value._submitter, value._newValue)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsForAllSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _submitter: tuple.0,
                        _newValue: tuple.1,
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
            impl ::core::convert::From<setRewardsForAllSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsForAllSubmitterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsForAllSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsForAllSubmitterCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsForAllSubmitterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsForAllSubmitter(address,bool)";
            const SELECTOR: [u8; 4] = [14u8, 179u8, 131u8, 69u8];
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
                        &self._submitter,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self._newValue,
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
    /**Function with signature `setRewardsUpdater(address)` and selector `0x863cb9a9`.
    ```solidity
    function setRewardsUpdater(address _rewardsUpdater) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsUpdaterCall {
        #[allow(missing_docs)]
        pub _rewardsUpdater: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setRewardsUpdater(address)`](setRewardsUpdaterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsUpdaterReturn {}
    #[allow(
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
            impl ::core::convert::From<setRewardsUpdaterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsUpdaterCall) -> Self {
                    (value._rewardsUpdater,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsUpdaterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _rewardsUpdater: tuple.0,
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
            impl ::core::convert::From<setRewardsUpdaterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsUpdaterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setRewardsUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsUpdaterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsUpdaterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setRewardsUpdater(address)";
            const SELECTOR: [u8; 4] = [134u8, 60u8, 185u8, 169u8];
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
                        &self._rewardsUpdater,
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
    /**Function with signature `submissionNonce(address)` and selector `0xbb7e451f`.
    ```solidity
    function submissionNonce(address) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceCall {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`submissionNonce(address)`](submissionNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceReturn {
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
            impl ::core::convert::From<submissionNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: submissionNonceCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submissionNonceCall {
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
            impl ::core::convert::From<submissionNonceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submissionNonceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submissionNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submissionNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = submissionNonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submissionNonce(address)";
            const SELECTOR: [u8; 4] = [187u8, 126u8, 69u8, 31u8];
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
    /**Function with signature `submitRoot(bytes32,uint32)` and selector `0x3efe1db6`.
    ```solidity
    function submitRoot(bytes32 root, uint32 rewardsCalculationEndTimestamp) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitRootCall {
        #[allow(missing_docs)]
        pub root: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub rewardsCalculationEndTimestamp: u32,
    }
    ///Container type for the return parameters of the [`submitRoot(bytes32,uint32)`](submitRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitRootReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<submitRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: submitRootCall) -> Self {
                    (value.root, value.rewardsCalculationEndTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        root: tuple.0,
                        rewardsCalculationEndTimestamp: tuple.1,
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
            impl ::core::convert::From<submitRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: submitRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submitRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submitRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "submitRoot(bytes32,uint32)";
            const SELECTOR: [u8; 4] = [62u8, 254u8, 29u8, 182u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.root),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.rewardsCalculationEndTimestamp,
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
    ///Container for all the [`RewardsCoordinator`](self) function calls.
    pub enum RewardsCoordinatorCalls {
        #[allow(missing_docs)]
        CALCULATION_INTERVAL_SECONDS(CALCULATION_INTERVAL_SECONDSCall),
        #[allow(missing_docs)]
        GENESIS_REWARDS_TIMESTAMP(GENESIS_REWARDS_TIMESTAMPCall),
        #[allow(missing_docs)]
        MAX_FUTURE_LENGTH(MAX_FUTURE_LENGTHCall),
        #[allow(missing_docs)]
        MAX_RETROACTIVE_LENGTH(MAX_RETROACTIVE_LENGTHCall),
        #[allow(missing_docs)]
        MAX_REWARDS_DURATION(MAX_REWARDS_DURATIONCall),
        #[allow(missing_docs)]
        activationDelay(activationDelayCall),
        #[allow(missing_docs)]
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        #[allow(missing_docs)]
        calculateEarnerLeafHash(calculateEarnerLeafHashCall),
        #[allow(missing_docs)]
        calculateTokenLeafHash(calculateTokenLeafHashCall),
        #[allow(missing_docs)]
        checkClaim(checkClaimCall),
        #[allow(missing_docs)]
        claimerFor(claimerForCall),
        #[allow(missing_docs)]
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        createOperatorDirectedAVSRewardsSubmission(createOperatorDirectedAVSRewardsSubmissionCall),
        #[allow(missing_docs)]
        createRewardsForAllEarners(createRewardsForAllEarnersCall),
        #[allow(missing_docs)]
        createRewardsForAllSubmission(createRewardsForAllSubmissionCall),
        #[allow(missing_docs)]
        cumulativeClaimed(cumulativeClaimedCall),
        #[allow(missing_docs)]
        currRewardsCalculationEndTimestamp(currRewardsCalculationEndTimestampCall),
        #[allow(missing_docs)]
        defaultOperatorSplitBips(defaultOperatorSplitBipsCall),
        #[allow(missing_docs)]
        delegationManager(delegationManagerCall),
        #[allow(missing_docs)]
        disableRoot(disableRootCall),
        #[allow(missing_docs)]
        domainSeparator(domainSeparatorCall),
        #[allow(missing_docs)]
        getCurrentClaimableDistributionRoot(getCurrentClaimableDistributionRootCall),
        #[allow(missing_docs)]
        getCurrentDistributionRoot(getCurrentDistributionRootCall),
        #[allow(missing_docs)]
        getDistributionRootAtIndex(getDistributionRootAtIndexCall),
        #[allow(missing_docs)]
        getDistributionRootsLength(getDistributionRootsLengthCall),
        #[allow(missing_docs)]
        getOperatorAVSSplit(getOperatorAVSSplitCall),
        #[allow(missing_docs)]
        getOperatorPISplit(getOperatorPISplitCall),
        #[allow(missing_docs)]
        getRootIndexFromHash(getRootIndexFromHashCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isAVSRewardsSubmissionHash(isAVSRewardsSubmissionHashCall),
        #[allow(missing_docs)]
        isOperatorDirectedAVSRewardsSubmissionHash(isOperatorDirectedAVSRewardsSubmissionHashCall),
        #[allow(missing_docs)]
        isRewardsForAllSubmitter(isRewardsForAllSubmitterCall),
        #[allow(missing_docs)]
        isRewardsSubmissionForAllEarnersHash(isRewardsSubmissionForAllEarnersHashCall),
        #[allow(missing_docs)]
        isRewardsSubmissionForAllHash(isRewardsSubmissionForAllHashCall),
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
        processClaim(processClaimCall),
        #[allow(missing_docs)]
        processClaims(processClaimsCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        rewardsUpdater(rewardsUpdaterCall),
        #[allow(missing_docs)]
        setActivationDelay(setActivationDelayCall),
        #[allow(missing_docs)]
        setClaimerFor(setClaimerForCall),
        #[allow(missing_docs)]
        setDefaultOperatorSplit(setDefaultOperatorSplitCall),
        #[allow(missing_docs)]
        setOperatorAVSSplit(setOperatorAVSSplitCall),
        #[allow(missing_docs)]
        setOperatorPISplit(setOperatorPISplitCall),
        #[allow(missing_docs)]
        setPauserRegistry(setPauserRegistryCall),
        #[allow(missing_docs)]
        setRewardsForAllSubmitter(setRewardsForAllSubmitterCall),
        #[allow(missing_docs)]
        setRewardsUpdater(setRewardsUpdaterCall),
        #[allow(missing_docs)]
        strategyManager(strategyManagerCall),
        #[allow(missing_docs)]
        submissionNonce(submissionNonceCall),
        #[allow(missing_docs)]
        submitRoot(submitRootCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl RewardsCoordinatorCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 24u8, 87u8, 44u8],
            [4u8, 160u8, 197u8, 2u8],
            [14u8, 154u8, 83u8, 207u8],
            [14u8, 179u8, 131u8, 69u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 20u8, 51u8, 180u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 155u8, 200u8, 114u8],
            [43u8, 159u8, 100u8, 164u8],
            [54u8, 175u8, 65u8, 250u8],
            [55u8, 131u8, 142u8, 208u8],
            [57u8, 183u8, 14u8, 56u8],
            [58u8, 140u8, 7u8, 134u8],
            [60u8, 204u8, 134u8, 29u8],
            [62u8, 254u8, 29u8, 182u8],
            [69u8, 150u8, 2u8, 28u8],
            [75u8, 148u8, 57u8, 96u8],
            [77u8, 24u8, 204u8, 53u8],
            [88u8, 186u8, 170u8, 62u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [94u8, 157u8, 131u8, 72u8],
            [99u8, 246u8, 167u8, 152u8],
            [109u8, 33u8, 17u8, 126u8],
            [113u8, 80u8, 24u8, 166u8],
            [123u8, 143u8, 139u8, 5u8],
            [134u8, 60u8, 185u8, 169u8],
            [134u8, 92u8, 105u8, 83u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 4u8, 195u8, 25u8],
            [155u8, 227u8, 212u8, 228u8],
            [156u8, 185u8, 165u8, 250u8],
            [157u8, 69u8, 194u8, 129u8],
            [160u8, 22u8, 157u8, 221u8],
            [165u8, 10u8, 29u8, 156u8],
            [174u8, 189u8, 139u8, 174u8],
            [179u8, 219u8, 176u8, 224u8],
            [187u8, 126u8, 69u8, 31u8],
            [191u8, 33u8, 168u8, 170u8],
            [196u8, 109u8, 182u8, 6u8],
            [212u8, 84u8, 10u8, 85u8],
            [220u8, 187u8, 3u8, 179u8],
            [222u8, 2u8, 229u8, 3u8],
            [224u8, 99u8, 248u8, 31u8],
            [232u8, 16u8, 206u8, 33u8],
            [234u8, 77u8, 60u8, 155u8],
            [237u8, 113u8, 230u8, 162u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [248u8, 205u8, 132u8, 72u8],
            [249u8, 106u8, 191u8, 46u8],
            [250u8, 188u8, 28u8, 188u8],
            [251u8, 241u8, 226u8, 193u8],
            [252u8, 227u8, 108u8, 125u8],
            [255u8, 159u8, 108u8, 206u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RewardsCoordinatorCalls {
        const NAME: &'static str = "RewardsCoordinatorCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 57usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CALCULATION_INTERVAL_SECONDS(_) => {
                    <CALCULATION_INTERVAL_SECONDSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::GENESIS_REWARDS_TIMESTAMP(_) => {
                    <GENESIS_REWARDS_TIMESTAMPCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_FUTURE_LENGTH(_) => {
                    <MAX_FUTURE_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_RETROACTIVE_LENGTH(_) => {
                    <MAX_RETROACTIVE_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::MAX_REWARDS_DURATION(_) => {
                    <MAX_REWARDS_DURATIONCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::activationDelay(_) => {
                    <activationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::beaconChainETHStrategy(_) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateEarnerLeafHash(_) => {
                    <calculateEarnerLeafHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateTokenLeafHash(_) => {
                    <calculateTokenLeafHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkClaim(_) => {
                    <checkClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimerFor(_) => {
                    <claimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createAVSRewardsSubmission(_) => {
                    <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorDirectedAVSRewardsSubmission(_) => {
                    <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createRewardsForAllEarners(_) => {
                    <createRewardsForAllEarnersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createRewardsForAllSubmission(_) => {
                    <createRewardsForAllSubmissionCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cumulativeClaimed(_) => {
                    <cumulativeClaimedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currRewardsCalculationEndTimestamp(_) => {
                    <currRewardsCalculationEndTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::defaultOperatorSplitBips(_) => {
                    <defaultOperatorSplitBipsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegationManager(_) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::disableRoot(_) => {
                    <disableRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentClaimableDistributionRoot(_) => {
                    <getCurrentClaimableDistributionRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentDistributionRoot(_) => {
                    <getCurrentDistributionRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDistributionRootAtIndex(_) => {
                    <getDistributionRootAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getDistributionRootsLength(_) => {
                    <getDistributionRootsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorAVSSplit(_) => {
                    <getOperatorAVSSplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorPISplit(_) => {
                    <getOperatorPISplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRootIndexFromHash(_) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isAVSRewardsSubmissionHash(_) => {
                    <isAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorDirectedAVSRewardsSubmissionHash(_) => {
                    <isOperatorDirectedAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRewardsForAllSubmitter(_) => {
                    <isRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRewardsSubmissionForAllEarnersHash(_) => {
                    <isRewardsSubmissionForAllEarnersHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isRewardsSubmissionForAllHash(_) => {
                    <isRewardsSubmissionForAllHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processClaim(_) => {
                    <processClaimCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processClaims(_) => {
                    <processClaimsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::rewardsUpdater(_) => {
                    <rewardsUpdaterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setActivationDelay(_) => {
                    <setActivationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setClaimerFor(_) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setDefaultOperatorSplit(_) => {
                    <setDefaultOperatorSplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorAVSSplit(_) => {
                    <setOperatorAVSSplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperatorPISplit(_) => {
                    <setOperatorPISplitCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsForAllSubmitter(_) => {
                    <setRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setRewardsUpdater(_) => {
                    <setRewardsUpdaterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyManager(_) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submissionNonce(_) => {
                    <submissionNonceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::submitRoot(_) => {
                    <submitRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
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
            )
                -> alloy_sol_types::Result<RewardsCoordinatorCalls>] = &[
                {
                    fn isRewardsForAllSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::isRewardsForAllSubmitter)
                    }
                    isRewardsForAllSubmitter
                },
                {
                    fn MAX_FUTURE_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <MAX_FUTURE_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::MAX_FUTURE_LENGTH)
                    }
                    MAX_FUTURE_LENGTH
                },
                {
                    fn getCurrentClaimableDistributionRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getCurrentClaimableDistributionRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorCalls::getCurrentClaimableDistributionRoot,
                            )
                    }
                    getCurrentClaimableDistributionRoot
                },
                {
                    fn setRewardsForAllSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setRewardsForAllSubmitter)
                    }
                    setRewardsForAllSubmitter
                },
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn GENESIS_REWARDS_TIMESTAMP(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <GENESIS_REWARDS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::GENESIS_REWARDS_TIMESTAMP)
                    }
                    GENESIS_REWARDS_TIMESTAMP
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::pause)
                    }
                    pause
                },
                {
                    fn calculateEarnerLeafHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <calculateEarnerLeafHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::calculateEarnerLeafHash)
                    }
                    calculateEarnerLeafHash
                },
                {
                    fn claimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <claimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::claimerFor)
                    }
                    claimerFor
                },
                {
                    fn createRewardsForAllSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <createRewardsForAllSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::createRewardsForAllSubmission)
                    }
                    createRewardsForAllSubmission
                },
                {
                    fn MAX_RETROACTIVE_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <MAX_RETROACTIVE_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::MAX_RETROACTIVE_LENGTH)
                    }
                    MAX_RETROACTIVE_LENGTH
                },
                {
                    fn strategyManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <strategyManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::strategyManager)
                    }
                    strategyManager
                },
                {
                    fn activationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <activationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::activationDelay)
                    }
                    activationDelay
                },
                {
                    fn processClaim(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <processClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::processClaim)
                    }
                    processClaim
                },
                {
                    fn submitRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <submitRootCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::submitRoot)
                    }
                    submitRoot
                },
                {
                    fn processClaims(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <processClaimsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::processClaims)
                    }
                    processClaims
                },
                {
                    fn getOperatorPISplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getOperatorPISplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::getOperatorPISplit)
                    }
                    getOperatorPISplit
                },
                {
                    fn currRewardsCalculationEndTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <currRewardsCalculationEndTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorCalls::currRewardsCalculationEndTimestamp,
                            )
                    }
                    currRewardsCalculationEndTimestamp
                },
                {
                    fn setActivationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setActivationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setActivationDelay)
                    }
                    setActivationDelay
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn checkClaim(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <checkClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::checkClaim)
                    }
                    checkClaim
                },
                {
                    fn defaultOperatorSplitBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <defaultOperatorSplitBipsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::defaultOperatorSplitBips)
                    }
                    defaultOperatorSplitBips
                },
                {
                    fn isAVSRewardsSubmissionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::isAVSRewardsSubmissionHash)
                    }
                    isAVSRewardsSubmissionHash
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getDistributionRootsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getDistributionRootsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::getDistributionRootsLength)
                    }
                    getDistributionRootsLength
                },
                {
                    fn setRewardsUpdater(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setRewardsUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setRewardsUpdater)
                    }
                    setRewardsUpdater
                },
                {
                    fn cumulativeClaimed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <cumulativeClaimedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::cumulativeClaimed)
                    }
                    cumulativeClaimed
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::owner)
                    }
                    owner
                },
                {
                    fn beaconChainETHStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::beaconChainETHStrategy)
                    }
                    beaconChainETHStrategy
                },
                {
                    fn getCurrentDistributionRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getCurrentDistributionRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::getCurrentDistributionRoot)
                    }
                    getCurrentDistributionRoot
                },
                {
                    fn createOperatorDirectedAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <createOperatorDirectedAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorCalls::createOperatorDirectedAVSRewardsSubmission,
                            )
                    }
                    createOperatorDirectedAVSRewardsSubmission
                },
                {
                    fn CALCULATION_INTERVAL_SECONDS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <CALCULATION_INTERVAL_SECONDSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::CALCULATION_INTERVAL_SECONDS)
                    }
                    CALCULATION_INTERVAL_SECONDS
                },
                {
                    fn setClaimerFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setClaimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setClaimerFor)
                    }
                    setClaimerFor
                },
                {
                    fn setDefaultOperatorSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setDefaultOperatorSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setDefaultOperatorSplit)
                    }
                    setDefaultOperatorSplit
                },
                {
                    fn isRewardsSubmissionForAllEarnersHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isRewardsSubmissionForAllEarnersHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorCalls::isRewardsSubmissionForAllEarnersHash,
                            )
                    }
                    isRewardsSubmissionForAllEarnersHash
                },
                {
                    fn setOperatorPISplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setOperatorPISplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setOperatorPISplit)
                    }
                    setOperatorPISplit
                },
                {
                    fn submissionNonce(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <submissionNonceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::submissionNonce)
                    }
                    submissionNonce
                },
                {
                    fn MAX_REWARDS_DURATION(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <MAX_REWARDS_DURATIONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::MAX_REWARDS_DURATION)
                    }
                    MAX_REWARDS_DURATION
                },
                {
                    fn isRewardsSubmissionForAllHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isRewardsSubmissionForAllHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::isRewardsSubmissionForAllHash)
                    }
                    isRewardsSubmissionForAllHash
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setOperatorAVSSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::setOperatorAVSSplit)
                    }
                    setOperatorAVSSplit
                },
                {
                    fn getDistributionRootAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getDistributionRootAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::getDistributionRootAtIndex)
                    }
                    getDistributionRootAtIndex
                },
                {
                    fn getOperatorAVSSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::getOperatorAVSSplit)
                    }
                    getOperatorAVSSplit
                },
                {
                    fn getRootIndexFromHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getRootIndexFromHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::getRootIndexFromHash)
                    }
                    getRootIndexFromHash
                },
                {
                    fn delegationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <delegationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::delegationManager)
                    }
                    delegationManager
                },
                {
                    fn isOperatorDirectedAVSRewardsSubmissionHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isOperatorDirectedAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorCalls::isOperatorDirectedAVSRewardsSubmissionHash,
                            )
                    }
                    isOperatorDirectedAVSRewardsSubmissionHash
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn calculateTokenLeafHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <calculateTokenLeafHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::calculateTokenLeafHash)
                    }
                    calculateTokenLeafHash
                },
                {
                    fn disableRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <disableRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::disableRoot)
                    }
                    disableRoot
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(RewardsCoordinatorCalls::unpause)
                    }
                    unpause
                },
                {
                    fn rewardsUpdater(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <rewardsUpdaterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(RewardsCoordinatorCalls::rewardsUpdater)
                    }
                    rewardsUpdater
                },
                {
                    fn createAVSRewardsSubmission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <createAVSRewardsSubmissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::createAVSRewardsSubmission)
                    }
                    createAVSRewardsSubmission
                },
                {
                    fn createRewardsForAllEarners(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <createRewardsForAllEarnersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::createRewardsForAllEarners)
                    }
                    createRewardsForAllEarners
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
                Self::CALCULATION_INTERVAL_SECONDS(inner) => {
                    <CALCULATION_INTERVAL_SECONDSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::GENESIS_REWARDS_TIMESTAMP(inner) => {
                    <GENESIS_REWARDS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_FUTURE_LENGTH(inner) => {
                    <MAX_FUTURE_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_RETROACTIVE_LENGTH(inner) => {
                    <MAX_RETROACTIVE_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MAX_REWARDS_DURATION(inner) => {
                    <MAX_REWARDS_DURATIONCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::activationDelay(inner) => {
                    <activationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::beaconChainETHStrategy(inner) => {
                    <beaconChainETHStrategyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateEarnerLeafHash(inner) => {
                    <calculateEarnerLeafHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateTokenLeafHash(inner) => {
                    <calculateTokenLeafHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkClaim(inner) => {
                    <checkClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::claimerFor(inner) => {
                    <claimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::createRewardsForAllEarners(inner) => {
                    <createRewardsForAllEarnersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createRewardsForAllSubmission(inner) => {
                    <createRewardsForAllSubmissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cumulativeClaimed(inner) => {
                    <cumulativeClaimedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currRewardsCalculationEndTimestamp(inner) => {
                    <currRewardsCalculationEndTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::defaultOperatorSplitBips(inner) => {
                    <defaultOperatorSplitBipsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegationManager(inner) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::disableRoot(inner) => {
                    <disableRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentClaimableDistributionRoot(inner) => {
                    <getCurrentClaimableDistributionRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentDistributionRoot(inner) => {
                    <getCurrentDistributionRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDistributionRootAtIndex(inner) => {
                    <getDistributionRootAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getDistributionRootsLength(inner) => {
                    <getDistributionRootsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorAVSSplit(inner) => {
                    <getOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorPISplit(inner) => {
                    <getOperatorPISplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRootIndexFromHash(inner) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isAVSRewardsSubmissionHash(inner) => {
                    <isAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorDirectedAVSRewardsSubmissionHash(inner) => {
                    <isOperatorDirectedAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRewardsForAllSubmitter(inner) => {
                    <isRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRewardsSubmissionForAllEarnersHash(inner) => {
                    <isRewardsSubmissionForAllEarnersHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isRewardsSubmissionForAllHash(inner) => {
                    <isRewardsSubmissionForAllHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::processClaim(inner) => {
                    <processClaimCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processClaims(inner) => {
                    <processClaimsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::rewardsUpdater(inner) => {
                    <rewardsUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setActivationDelay(inner) => {
                    <setActivationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setClaimerFor(inner) => {
                    <setClaimerForCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setDefaultOperatorSplit(inner) => {
                    <setDefaultOperatorSplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorAVSSplit(inner) => {
                    <setOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperatorPISplit(inner) => {
                    <setOperatorPISplitCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsForAllSubmitter(inner) => {
                    <setRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setRewardsUpdater(inner) => {
                    <setRewardsUpdaterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyManager(inner) => {
                    <strategyManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submissionNonce(inner) => {
                    <submissionNonceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::submitRoot(inner) => {
                    <submitRootCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::CALCULATION_INTERVAL_SECONDS(inner) => {
                    <CALCULATION_INTERVAL_SECONDSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::GENESIS_REWARDS_TIMESTAMP(inner) => {
                    <GENESIS_REWARDS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_FUTURE_LENGTH(inner) => {
                    <MAX_FUTURE_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_RETROACTIVE_LENGTH(inner) => {
                    <MAX_RETROACTIVE_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MAX_REWARDS_DURATION(inner) => {
                    <MAX_REWARDS_DURATIONCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::activationDelay(inner) => {
                    <activationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::calculateEarnerLeafHash(inner) => {
                    <calculateEarnerLeafHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateTokenLeafHash(inner) => {
                    <calculateTokenLeafHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkClaim(inner) => {
                    <checkClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimerFor(inner) => {
                    <claimerForCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::createRewardsForAllEarners(inner) => {
                    <createRewardsForAllEarnersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createRewardsForAllSubmission(inner) => {
                    <createRewardsForAllSubmissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cumulativeClaimed(inner) => {
                    <cumulativeClaimedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currRewardsCalculationEndTimestamp(inner) => {
                    <currRewardsCalculationEndTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::defaultOperatorSplitBips(inner) => {
                    <defaultOperatorSplitBipsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegationManager(inner) => {
                    <delegationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::disableRoot(inner) => {
                    <disableRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getCurrentClaimableDistributionRoot(inner) => {
                    <getCurrentClaimableDistributionRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentDistributionRoot(inner) => {
                    <getCurrentDistributionRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDistributionRootAtIndex(inner) => {
                    <getDistributionRootAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getDistributionRootsLength(inner) => {
                    <getDistributionRootsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorAVSSplit(inner) => {
                    <getOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorPISplit(inner) => {
                    <getOperatorPISplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRootIndexFromHash(inner) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isAVSRewardsSubmissionHash(inner) => {
                    <isAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorDirectedAVSRewardsSubmissionHash(inner) => {
                    <isOperatorDirectedAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRewardsForAllSubmitter(inner) => {
                    <isRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRewardsSubmissionForAllEarnersHash(inner) => {
                    <isRewardsSubmissionForAllEarnersHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isRewardsSubmissionForAllHash(inner) => {
                    <isRewardsSubmissionForAllHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::processClaim(inner) => {
                    <processClaimCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processClaims(inner) => {
                    <processClaimsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::rewardsUpdater(inner) => {
                    <rewardsUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setActivationDelay(inner) => {
                    <setActivationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setDefaultOperatorSplit(inner) => {
                    <setDefaultOperatorSplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorAVSSplit(inner) => {
                    <setOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperatorPISplit(inner) => {
                    <setOperatorPISplitCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setRewardsForAllSubmitter(inner) => {
                    <setRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setRewardsUpdater(inner) => {
                    <setRewardsUpdaterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::submissionNonce(inner) => {
                    <submissionNonceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::submitRoot(inner) => {
                    <submitRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`RewardsCoordinator`](self) events.
    pub enum RewardsCoordinatorEvents {
        #[allow(missing_docs)]
        AVSRewardsSubmissionCreated(AVSRewardsSubmissionCreated),
        #[allow(missing_docs)]
        ActivationDelaySet(ActivationDelaySet),
        #[allow(missing_docs)]
        ClaimerForSet(ClaimerForSet),
        #[allow(missing_docs)]
        DefaultOperatorSplitBipsSet(DefaultOperatorSplitBipsSet),
        #[allow(missing_docs)]
        DistributionRootDisabled(DistributionRootDisabled),
        #[allow(missing_docs)]
        DistributionRootSubmitted(DistributionRootSubmitted),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        OperatorAVSSplitBipsSet(OperatorAVSSplitBipsSet),
        #[allow(missing_docs)]
        OperatorDirectedAVSRewardsSubmissionCreated(OperatorDirectedAVSRewardsSubmissionCreated),
        #[allow(missing_docs)]
        OperatorPISplitBipsSet(OperatorPISplitBipsSet),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        PauserRegistrySet(PauserRegistrySet),
        #[allow(missing_docs)]
        RewardsClaimed(RewardsClaimed),
        #[allow(missing_docs)]
        RewardsForAllSubmitterSet(RewardsForAllSubmitterSet),
        #[allow(missing_docs)]
        RewardsSubmissionForAllCreated(RewardsSubmissionForAllCreated),
        #[allow(missing_docs)]
        RewardsSubmissionForAllEarnersCreated(RewardsSubmissionForAllEarnersCreated),
        #[allow(missing_docs)]
        RewardsUpdaterSet(RewardsUpdaterSet),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl RewardsCoordinatorEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                35u8, 123u8, 130u8, 244u8, 56u8, 215u8, 95u8, 197u8, 104u8, 235u8, 171u8, 72u8,
                75u8, 117u8, 176u8, 29u8, 146u8, 135u8, 185u8, 233u8, 139u8, 73u8, 11u8, 124u8,
                35u8, 34u8, 22u8, 35u8, 182u8, 112u8, 93u8, 187u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                69u8, 10u8, 54u8, 122u8, 56u8, 12u8, 78u8, 51u8, 158u8, 90u8, 231u8, 52u8, 12u8,
                132u8, 100u8, 239u8, 39u8, 175u8, 119u8, 129u8, 173u8, 153u8, 69u8, 207u8, 232u8,
                171u8, 216u8, 40u8, 248u8, 158u8, 98u8, 129u8,
            ],
            [
                72u8, 225u8, 152u8, 182u8, 174u8, 53u8, 126u8, 82u8, 146u8, 4u8, 238u8, 83u8,
                168u8, 229u8, 20u8, 196u8, 112u8, 255u8, 119u8, 217u8, 204u8, 142u8, 79u8, 114u8,
                7u8, 248u8, 181u8, 212u8, 144u8, 174u8, 105u8, 52u8,
            ],
            [
                77u8, 230u8, 41u8, 62u8, 102u8, 141u8, 241u8, 57u8, 132u8, 34u8, 225u8, 222u8,
                241u8, 33u8, 24u8, 5u8, 44u8, 21u8, 57u8, 160u8, 60u8, 191u8, 237u8, 193u8, 69u8,
                137u8, 93u8, 72u8, 215u8, 104u8, 95u8, 28u8,
            ],
            [
                81u8, 8u8, 139u8, 140u8, 137u8, 98u8, 141u8, 243u8, 168u8, 23u8, 64u8, 2u8, 194u8,
                160u8, 52u8, 208u8, 21u8, 47u8, 206u8, 106u8, 248u8, 65u8, 93u8, 101u8, 27u8, 42u8,
                71u8, 52u8, 191u8, 39u8, 4u8, 130u8,
            ],
            [
                82u8, 81u8, 182u8, 253u8, 239u8, 203u8, 93u8, 129u8, 20u8, 78u8, 115u8, 95u8,
                105u8, 234u8, 76u8, 105u8, 95u8, 212u8, 59u8, 2u8, 137u8, 202u8, 83u8, 220u8, 7u8,
                80u8, 51u8, 245u8, 252u8, 128u8, 6u8, 139u8,
            ],
            [
                110u8, 159u8, 205u8, 83u8, 152u8, 150u8, 252u8, 166u8, 14u8, 139u8, 15u8, 1u8,
                221u8, 88u8, 2u8, 51u8, 228u8, 138u8, 107u8, 15u8, 125u8, 240u8, 19u8, 184u8,
                155u8, 167u8, 245u8, 101u8, 134u8, 154u8, 205u8, 182u8,
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
                149u8, 67u8, 219u8, 213u8, 85u8, 128u8, 132u8, 37u8, 134u8, 169u8, 81u8, 240u8,
                56u8, 110u8, 36u8, 214u8, 138u8, 93u8, 249u8, 154u8, 226u8, 158u8, 59u8, 33u8,
                101u8, 136u8, 180u8, 95u8, 214u8, 132u8, 206u8, 49u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                175u8, 85u8, 124u8, 108u8, 2u8, 194u8, 8u8, 121u8, 72u8, 23u8, 167u8, 5u8, 96u8,
                156u8, 250u8, 147u8, 95u8, 130u8, 115u8, 18u8, 161u8, 173u8, 253u8, 210u8, 100u8,
                148u8, 182u8, 185u8, 93u8, 210u8, 180u8, 179u8,
            ],
            [
                186u8, 185u8, 71u8, 147u8, 77u8, 66u8, 224u8, 173u8, 32u8, 111u8, 37u8, 201u8,
                202u8, 177u8, 139u8, 91u8, 182u8, 174u8, 20u8, 74u8, 207u8, 176u8, 15u8, 64u8,
                180u8, 227u8, 170u8, 89u8, 89u8, 12u8, 163u8, 18u8,
            ],
            [
                209u8, 224u8, 40u8, 189u8, 102u8, 68u8, 134u8, 164u8, 106u8, 210u8, 96u8, 64u8,
                233u8, 153u8, 205u8, 45u8, 34u8, 225u8, 233u8, 160u8, 148u8, 238u8, 106u8, 254u8,
                25u8, 252u8, 246u8, 70u8, 120u8, 241u8, 111u8, 116u8,
            ],
            [
                216u8, 80u8, 230u8, 229u8, 223u8, 164u8, 151u8, 183u8, 38u8, 97u8, 250u8, 115u8,
                223u8, 41u8, 35u8, 70u8, 78u8, 174u8, 217u8, 220u8, 47u8, 241u8, 211u8, 203u8,
                130u8, 188u8, 203u8, 254u8, 171u8, 229u8, 196u8, 30u8,
            ],
            [
                230u8, 205u8, 78u8, 223u8, 220u8, 193u8, 246u8, 209u8, 48u8, 171u8, 53u8, 247u8,
                61u8, 114u8, 55u8, 143u8, 58u8, 100u8, 41u8, 68u8, 251u8, 78u8, 229u8, 189u8,
                132u8, 183u8, 128u8, 122u8, 129u8, 234u8, 28u8, 78u8,
            ],
            [
                236u8, 216u8, 102u8, 195u8, 193u8, 88u8, 250u8, 0u8, 191u8, 52u8, 216u8, 3u8,
                213u8, 246u8, 2u8, 48u8, 0u8, 181u8, 112u8, 128u8, 188u8, 180u8, 138u8, 240u8, 4u8,
                194u8, 180u8, 180u8, 107u8, 58u8, 253u8, 8u8,
            ],
            [
                252u8, 136u8, 136u8, 191u8, 253u8, 113u8, 29u8, 166u8, 11u8, 197u8, 9u8, 43u8,
                51u8, 246u8, 119u8, 216u8, 24u8, 150u8, 254u8, 128u8, 236u8, 198u8, 119u8, 184u8,
                76u8, 250u8, 184u8, 24u8, 68u8, 98u8, 182u8, 224u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RewardsCoordinatorEvents {
        const NAME: &'static str = "RewardsCoordinatorEvents";
        const COUNT: usize = 19usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AVSRewardsSubmissionCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AVSRewardsSubmissionCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSRewardsSubmissionCreated)
                }
                Some(
                    <ActivationDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ActivationDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ActivationDelaySet)
                }
                Some(<ClaimerForSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ClaimerForSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ClaimerForSet)
                }
                Some(
                    <DefaultOperatorSplitBipsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DefaultOperatorSplitBipsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DefaultOperatorSplitBipsSet)
                }
                Some(
                    <DistributionRootDisabled as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DistributionRootDisabled as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DistributionRootDisabled)
                }
                Some(
                    <DistributionRootSubmitted as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DistributionRootSubmitted as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DistributionRootSubmitted)
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
                    <OperatorAVSSplitBipsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorAVSSplitBipsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorAVSSplitBipsSet)
                }
                Some(
                    <OperatorDirectedAVSRewardsSubmissionCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorDirectedAVSRewardsSubmissionCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorDirectedAVSRewardsSubmissionCreated)
                }
                Some(
                    <OperatorPISplitBipsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorPISplitBipsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorPISplitBipsSet)
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
                Some(<RewardsClaimed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <RewardsClaimed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsClaimed)
                }
                Some(
                    <RewardsForAllSubmitterSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsForAllSubmitterSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsForAllSubmitterSet)
                }
                Some(
                    <RewardsSubmissionForAllCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsSubmissionForAllCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsSubmissionForAllCreated)
                }
                Some(
                    <RewardsSubmissionForAllEarnersCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsSubmissionForAllEarnersCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsSubmissionForAllEarnersCreated)
                }
                Some(
                    <RewardsUpdaterSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RewardsUpdaterSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RewardsUpdaterSet)
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
    impl alloy_sol_types::private::IntoLogData for RewardsCoordinatorEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSRewardsSubmissionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ActivationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ClaimerForSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DefaultOperatorSplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DistributionRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DistributionRootSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorAVSSplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorDirectedAVSRewardsSubmissionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorPISplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsForAllSubmitterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsSubmissionForAllCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsSubmissionForAllEarnersCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RewardsUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSRewardsSubmissionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ActivationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ClaimerForSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DefaultOperatorSplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DistributionRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DistributionRootSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorAVSSplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorDirectedAVSRewardsSubmissionCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorPISplitBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsForAllSubmitterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsSubmissionForAllCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsSubmissionForAllEarnersCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RewardsUpdaterSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`RewardsCoordinator`](self) contract instance.

    See the [wrapper's documentation](`RewardsCoordinatorInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> RewardsCoordinatorInstance<T, P, N> {
        RewardsCoordinatorInstance::<T, P, N>::new(address, provider)
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
        _delegationManager: alloy::sol_types::private::Address,
        _strategyManager: alloy::sol_types::private::Address,
        _CALCULATION_INTERVAL_SECONDS: u32,
        _MAX_REWARDS_DURATION: u32,
        _MAX_RETROACTIVE_LENGTH: u32,
        _MAX_FUTURE_LENGTH: u32,
        __GENESIS_REWARDS_TIMESTAMP: u32,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<RewardsCoordinatorInstance<T, P, N>>>
    {
        RewardsCoordinatorInstance::<T, P, N>::deploy(
            provider,
            _delegationManager,
            _strategyManager,
            _CALCULATION_INTERVAL_SECONDS,
            _MAX_REWARDS_DURATION,
            _MAX_RETROACTIVE_LENGTH,
            _MAX_FUTURE_LENGTH,
            __GENESIS_REWARDS_TIMESTAMP,
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
        _delegationManager: alloy::sol_types::private::Address,
        _strategyManager: alloy::sol_types::private::Address,
        _CALCULATION_INTERVAL_SECONDS: u32,
        _MAX_REWARDS_DURATION: u32,
        _MAX_RETROACTIVE_LENGTH: u32,
        _MAX_FUTURE_LENGTH: u32,
        __GENESIS_REWARDS_TIMESTAMP: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RewardsCoordinatorInstance::<T, P, N>::deploy_builder(
            provider,
            _delegationManager,
            _strategyManager,
            _CALCULATION_INTERVAL_SECONDS,
            _MAX_REWARDS_DURATION,
            _MAX_RETROACTIVE_LENGTH,
            _MAX_FUTURE_LENGTH,
            __GENESIS_REWARDS_TIMESTAMP,
        )
    }
    /**A [`RewardsCoordinator`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`RewardsCoordinator`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct RewardsCoordinatorInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for RewardsCoordinatorInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("RewardsCoordinatorInstance")
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
        > RewardsCoordinatorInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`RewardsCoordinator`](self) contract instance.

        See the [wrapper's documentation](`RewardsCoordinatorInstance`) for more details.*/
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
            _delegationManager: alloy::sol_types::private::Address,
            _strategyManager: alloy::sol_types::private::Address,
            _CALCULATION_INTERVAL_SECONDS: u32,
            _MAX_REWARDS_DURATION: u32,
            _MAX_RETROACTIVE_LENGTH: u32,
            _MAX_FUTURE_LENGTH: u32,
            __GENESIS_REWARDS_TIMESTAMP: u32,
        ) -> alloy_contract::Result<RewardsCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegationManager,
                _strategyManager,
                _CALCULATION_INTERVAL_SECONDS,
                _MAX_REWARDS_DURATION,
                _MAX_RETROACTIVE_LENGTH,
                _MAX_FUTURE_LENGTH,
                __GENESIS_REWARDS_TIMESTAMP,
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
            _delegationManager: alloy::sol_types::private::Address,
            _strategyManager: alloy::sol_types::private::Address,
            _CALCULATION_INTERVAL_SECONDS: u32,
            _MAX_REWARDS_DURATION: u32,
            _MAX_RETROACTIVE_LENGTH: u32,
            _MAX_FUTURE_LENGTH: u32,
            __GENESIS_REWARDS_TIMESTAMP: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegationManager,
                        _strategyManager,
                        _CALCULATION_INTERVAL_SECONDS,
                        _MAX_REWARDS_DURATION,
                        _MAX_RETROACTIVE_LENGTH,
                        _MAX_FUTURE_LENGTH,
                        __GENESIS_REWARDS_TIMESTAMP,
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
    impl<T, P: ::core::clone::Clone, N> RewardsCoordinatorInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> RewardsCoordinatorInstance<T, P, N> {
            RewardsCoordinatorInstance {
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
        > RewardsCoordinatorInstance<T, P, N>
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
        ///Creates a new call builder for the [`CALCULATION_INTERVAL_SECONDS`] function.
        pub fn CALCULATION_INTERVAL_SECONDS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CALCULATION_INTERVAL_SECONDSCall, N> {
            self.call_builder(&CALCULATION_INTERVAL_SECONDSCall {})
        }
        ///Creates a new call builder for the [`GENESIS_REWARDS_TIMESTAMP`] function.
        pub fn GENESIS_REWARDS_TIMESTAMP(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, GENESIS_REWARDS_TIMESTAMPCall, N> {
            self.call_builder(&GENESIS_REWARDS_TIMESTAMPCall {})
        }
        ///Creates a new call builder for the [`MAX_FUTURE_LENGTH`] function.
        pub fn MAX_FUTURE_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_FUTURE_LENGTHCall, N> {
            self.call_builder(&MAX_FUTURE_LENGTHCall {})
        }
        ///Creates a new call builder for the [`MAX_RETROACTIVE_LENGTH`] function.
        pub fn MAX_RETROACTIVE_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_RETROACTIVE_LENGTHCall, N> {
            self.call_builder(&MAX_RETROACTIVE_LENGTHCall {})
        }
        ///Creates a new call builder for the [`MAX_REWARDS_DURATION`] function.
        pub fn MAX_REWARDS_DURATION(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_REWARDS_DURATIONCall, N> {
            self.call_builder(&MAX_REWARDS_DURATIONCall {})
        }
        ///Creates a new call builder for the [`activationDelay`] function.
        pub fn activationDelay(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, activationDelayCall, N> {
            self.call_builder(&activationDelayCall {})
        }
        ///Creates a new call builder for the [`beaconChainETHStrategy`] function.
        pub fn beaconChainETHStrategy(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, beaconChainETHStrategyCall, N> {
            self.call_builder(&beaconChainETHStrategyCall {})
        }
        ///Creates a new call builder for the [`calculateEarnerLeafHash`] function.
        pub fn calculateEarnerLeafHash(
            &self,
            leaf: <IRewardsCoordinator::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateEarnerLeafHashCall, N> {
            self.call_builder(&calculateEarnerLeafHashCall { leaf })
        }
        ///Creates a new call builder for the [`calculateTokenLeafHash`] function.
        pub fn calculateTokenLeafHash(
            &self,
            leaf: <IRewardsCoordinator::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateTokenLeafHashCall, N> {
            self.call_builder(&calculateTokenLeafHashCall { leaf })
        }
        ///Creates a new call builder for the [`checkClaim`] function.
        pub fn checkClaim(
            &self,
            claim: <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkClaimCall, N> {
            self.call_builder(&checkClaimCall { claim })
        }
        ///Creates a new call builder for the [`claimerFor`] function.
        pub fn claimerFor(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimerForCall, N> {
            self.call_builder(&claimerForCall { _0 })
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(&createAVSRewardsSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorDirectedAVSRewardsSubmissionCall, N>
        {
            self.call_builder(&createOperatorDirectedAVSRewardsSubmissionCall {
                avs,
                operatorDirectedRewardsSubmissions,
            })
        }
        ///Creates a new call builder for the [`createRewardsForAllEarners`] function.
        pub fn createRewardsForAllEarners(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createRewardsForAllEarnersCall, N> {
            self.call_builder(&createRewardsForAllEarnersCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`createRewardsForAllSubmission`] function.
        pub fn createRewardsForAllSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createRewardsForAllSubmissionCall, N> {
            self.call_builder(&createRewardsForAllSubmissionCall { rewardsSubmissions })
        }
        ///Creates a new call builder for the [`cumulativeClaimed`] function.
        pub fn cumulativeClaimed(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeClaimedCall, N> {
            self.call_builder(&cumulativeClaimedCall { _0, _1 })
        }
        ///Creates a new call builder for the [`currRewardsCalculationEndTimestamp`] function.
        pub fn currRewardsCalculationEndTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currRewardsCalculationEndTimestampCall, N>
        {
            self.call_builder(&currRewardsCalculationEndTimestampCall {})
        }
        ///Creates a new call builder for the [`defaultOperatorSplitBips`] function.
        pub fn defaultOperatorSplitBips(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, defaultOperatorSplitBipsCall, N> {
            self.call_builder(&defaultOperatorSplitBipsCall {})
        }
        ///Creates a new call builder for the [`delegationManager`] function.
        pub fn delegationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationManagerCall, N> {
            self.call_builder(&delegationManagerCall {})
        }
        ///Creates a new call builder for the [`disableRoot`] function.
        pub fn disableRoot(
            &self,
            rootIndex: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, disableRootCall, N> {
            self.call_builder(&disableRootCall { rootIndex })
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`getCurrentClaimableDistributionRoot`] function.
        pub fn getCurrentClaimableDistributionRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentClaimableDistributionRootCall, N>
        {
            self.call_builder(&getCurrentClaimableDistributionRootCall {})
        }
        ///Creates a new call builder for the [`getCurrentDistributionRoot`] function.
        pub fn getCurrentDistributionRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentDistributionRootCall, N> {
            self.call_builder(&getCurrentDistributionRootCall {})
        }
        ///Creates a new call builder for the [`getDistributionRootAtIndex`] function.
        pub fn getDistributionRootAtIndex(
            &self,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDistributionRootAtIndexCall, N> {
            self.call_builder(&getDistributionRootAtIndexCall { index })
        }
        ///Creates a new call builder for the [`getDistributionRootsLength`] function.
        pub fn getDistributionRootsLength(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getDistributionRootsLengthCall, N> {
            self.call_builder(&getDistributionRootsLengthCall {})
        }
        ///Creates a new call builder for the [`getOperatorAVSSplit`] function.
        pub fn getOperatorAVSSplit(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorAVSSplitCall, N> {
            self.call_builder(&getOperatorAVSSplitCall { operator, avs })
        }
        ///Creates a new call builder for the [`getOperatorPISplit`] function.
        pub fn getOperatorPISplit(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorPISplitCall, N> {
            self.call_builder(&getOperatorPISplitCall { operator })
        }
        ///Creates a new call builder for the [`getRootIndexFromHash`] function.
        pub fn getRootIndexFromHash(
            &self,
            rootHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRootIndexFromHashCall, N> {
            self.call_builder(&getRootIndexFromHashCall { rootHash })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _rewardsUpdater: alloy::sol_types::private::Address,
            _activationDelay: u32,
            _defaultSplitBips: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _pauserRegistry,
                initialPausedStatus,
                _rewardsUpdater,
                _activationDelay,
                _defaultSplitBips,
            })
        }
        ///Creates a new call builder for the [`isAVSRewardsSubmissionHash`] function.
        pub fn isAVSRewardsSubmissionHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isAVSRewardsSubmissionHashCall, N> {
            self.call_builder(&isAVSRewardsSubmissionHashCall { _0, _1 })
        }
        ///Creates a new call builder for the [`isOperatorDirectedAVSRewardsSubmissionHash`] function.
        pub fn isOperatorDirectedAVSRewardsSubmissionHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorDirectedAVSRewardsSubmissionHashCall, N>
        {
            self.call_builder(&isOperatorDirectedAVSRewardsSubmissionHashCall { _0, _1 })
        }
        ///Creates a new call builder for the [`isRewardsForAllSubmitter`] function.
        pub fn isRewardsForAllSubmitter(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isRewardsForAllSubmitterCall, N> {
            self.call_builder(&isRewardsForAllSubmitterCall { _0 })
        }
        ///Creates a new call builder for the [`isRewardsSubmissionForAllEarnersHash`] function.
        pub fn isRewardsSubmissionForAllEarnersHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isRewardsSubmissionForAllEarnersHashCall, N>
        {
            self.call_builder(&isRewardsSubmissionForAllEarnersHashCall { _0, _1 })
        }
        ///Creates a new call builder for the [`isRewardsSubmissionForAllHash`] function.
        pub fn isRewardsSubmissionForAllHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isRewardsSubmissionForAllHashCall, N> {
            self.call_builder(&isRewardsSubmissionForAllHashCall { _0, _1 })
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
        ///Creates a new call builder for the [`processClaim`] function.
        pub fn processClaim(
            &self,
            claim: <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, processClaimCall, N> {
            self.call_builder(&processClaimCall { claim, recipient })
        }
        ///Creates a new call builder for the [`processClaims`] function.
        pub fn processClaims(
            &self,
            claims: alloy::sol_types::private::Vec<
                <IRewardsCoordinator::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
            >,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, processClaimsCall, N> {
            self.call_builder(&processClaimsCall { claims, recipient })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`rewardsUpdater`] function.
        pub fn rewardsUpdater(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, rewardsUpdaterCall, N> {
            self.call_builder(&rewardsUpdaterCall {})
        }
        ///Creates a new call builder for the [`setActivationDelay`] function.
        pub fn setActivationDelay(
            &self,
            _activationDelay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setActivationDelayCall, N> {
            self.call_builder(&setActivationDelayCall { _activationDelay })
        }
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setDefaultOperatorSplit`] function.
        pub fn setDefaultOperatorSplit(
            &self,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setDefaultOperatorSplitCall, N> {
            self.call_builder(&setDefaultOperatorSplitCall { split })
        }
        ///Creates a new call builder for the [`setOperatorAVSSplit`] function.
        pub fn setOperatorAVSSplit(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorAVSSplitCall, N> {
            self.call_builder(&setOperatorAVSSplitCall {
                operator,
                avs,
                split,
            })
        }
        ///Creates a new call builder for the [`setOperatorPISplit`] function.
        pub fn setOperatorPISplit(
            &self,
            operator: alloy::sol_types::private::Address,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorPISplitCall, N> {
            self.call_builder(&setOperatorPISplitCall { operator, split })
        }
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(&setPauserRegistryCall { newPauserRegistry })
        }
        ///Creates a new call builder for the [`setRewardsForAllSubmitter`] function.
        pub fn setRewardsForAllSubmitter(
            &self,
            _submitter: alloy::sol_types::private::Address,
            _newValue: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsForAllSubmitterCall, N> {
            self.call_builder(&setRewardsForAllSubmitterCall {
                _submitter,
                _newValue,
            })
        }
        ///Creates a new call builder for the [`setRewardsUpdater`] function.
        pub fn setRewardsUpdater(
            &self,
            _rewardsUpdater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsUpdaterCall, N> {
            self.call_builder(&setRewardsUpdaterCall { _rewardsUpdater })
        }
        ///Creates a new call builder for the [`strategyManager`] function.
        pub fn strategyManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyManagerCall, N> {
            self.call_builder(&strategyManagerCall {})
        }
        ///Creates a new call builder for the [`submissionNonce`] function.
        pub fn submissionNonce(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, submissionNonceCall, N> {
            self.call_builder(&submissionNonceCall { _0 })
        }
        ///Creates a new call builder for the [`submitRoot`] function.
        pub fn submitRoot(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
            rewardsCalculationEndTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitRootCall, N> {
            self.call_builder(&submitRootCall {
                root,
                rewardsCalculationEndTimestamp,
            })
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
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > RewardsCoordinatorInstance<T, P, N>
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
        ///Creates a new event filter for the [`AVSRewardsSubmissionCreated`] event.
        pub fn AVSRewardsSubmissionCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSRewardsSubmissionCreated, N> {
            self.event_filter::<AVSRewardsSubmissionCreated>()
        }
        ///Creates a new event filter for the [`ActivationDelaySet`] event.
        pub fn ActivationDelaySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ActivationDelaySet, N> {
            self.event_filter::<ActivationDelaySet>()
        }
        ///Creates a new event filter for the [`ClaimerForSet`] event.
        pub fn ClaimerForSet_filter(&self) -> alloy_contract::Event<T, &P, ClaimerForSet, N> {
            self.event_filter::<ClaimerForSet>()
        }
        ///Creates a new event filter for the [`DefaultOperatorSplitBipsSet`] event.
        pub fn DefaultOperatorSplitBipsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DefaultOperatorSplitBipsSet, N> {
            self.event_filter::<DefaultOperatorSplitBipsSet>()
        }
        ///Creates a new event filter for the [`DistributionRootDisabled`] event.
        pub fn DistributionRootDisabled_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DistributionRootDisabled, N> {
            self.event_filter::<DistributionRootDisabled>()
        }
        ///Creates a new event filter for the [`DistributionRootSubmitted`] event.
        pub fn DistributionRootSubmitted_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DistributionRootSubmitted, N> {
            self.event_filter::<DistributionRootSubmitted>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorAVSSplitBipsSet`] event.
        pub fn OperatorAVSSplitBipsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAVSSplitBipsSet, N> {
            self.event_filter::<OperatorAVSSplitBipsSet>()
        }
        ///Creates a new event filter for the [`OperatorDirectedAVSRewardsSubmissionCreated`] event.
        pub fn OperatorDirectedAVSRewardsSubmissionCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorDirectedAVSRewardsSubmissionCreated, N> {
            self.event_filter::<OperatorDirectedAVSRewardsSubmissionCreated>()
        }
        ///Creates a new event filter for the [`OperatorPISplitBipsSet`] event.
        pub fn OperatorPISplitBipsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorPISplitBipsSet, N> {
            self.event_filter::<OperatorPISplitBipsSet>()
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
        ///Creates a new event filter for the [`RewardsClaimed`] event.
        pub fn RewardsClaimed_filter(&self) -> alloy_contract::Event<T, &P, RewardsClaimed, N> {
            self.event_filter::<RewardsClaimed>()
        }
        ///Creates a new event filter for the [`RewardsForAllSubmitterSet`] event.
        pub fn RewardsForAllSubmitterSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsForAllSubmitterSet, N> {
            self.event_filter::<RewardsForAllSubmitterSet>()
        }
        ///Creates a new event filter for the [`RewardsSubmissionForAllCreated`] event.
        pub fn RewardsSubmissionForAllCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsSubmissionForAllCreated, N> {
            self.event_filter::<RewardsSubmissionForAllCreated>()
        }
        ///Creates a new event filter for the [`RewardsSubmissionForAllEarnersCreated`] event.
        pub fn RewardsSubmissionForAllEarnersCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsSubmissionForAllEarnersCreated, N> {
            self.event_filter::<RewardsSubmissionForAllEarnersCreated>()
        }
        ///Creates a new event filter for the [`RewardsUpdaterSet`] event.
        pub fn RewardsUpdaterSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsUpdaterSet, N> {
            self.event_filter::<RewardsUpdaterSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
