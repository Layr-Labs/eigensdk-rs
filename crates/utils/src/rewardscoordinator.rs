///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinatorTypes {
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
    clippy::style
)]
pub mod IRewardsCoordinatorTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct DistributionRoot { bytes32 root; uint32 rewardsCalculationEndTimestamp; uint32 activatedAt; bool disabled; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DistributionRoot {
        pub root: alloy::sol_types::private::FixedBytes<32>,
        pub rewardsCalculationEndTimestamp: u32,
        pub activatedAt: u32,
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
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            u32,
            u32,
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
        impl alloy_sol_types::SolType for DistributionRoot {
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
        impl alloy_sol_types::SolStruct for DistributionRoot {
            const NAME: &'static str = "DistributionRoot";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DistributionRoot(bytes32 root,uint32 rewardsCalculationEndTimestamp,uint32 activatedAt,bool disabled)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct EarnerTreeMerkleLeaf { address earner; bytes32 earnerTokenRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EarnerTreeMerkleLeaf {
        pub earner: alloy::sol_types::private::Address,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for EarnerTreeMerkleLeaf {
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
        impl alloy_sol_types::SolStruct for EarnerTreeMerkleLeaf {
            const NAME: &'static str = "EarnerTreeMerkleLeaf";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "EarnerTreeMerkleLeaf(address earner,bytes32 earnerTokenRoot)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct OperatorDirectedRewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; OperatorReward[] operatorRewards; uint32 startTimestamp; uint32 duration; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorDirectedRewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub operatorRewards: alloy::sol_types::private::Vec<
            <OperatorReward as alloy::sol_types::SolType>::RustType,
        >,
        pub startTimestamp: u32,
        pub duration: u32,
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
            alloy::sol_types::private::Vec<
                <OperatorReward as alloy::sol_types::SolType>::RustType,
            >,
            u32,
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
        impl ::core::convert::From<OperatorDirectedRewardsSubmission>
        for UnderlyingRustTuple<'_> {
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
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::private::SolTypeValue<Self>
        for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::SolType for OperatorDirectedRewardsSubmission {
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
        impl alloy_sol_types::SolStruct for OperatorDirectedRewardsSubmission {
            const NAME: &'static str = "OperatorDirectedRewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorDirectedRewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,OperatorReward[] operatorRewards,uint32 startTimestamp,uint32 duration,string description)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <OperatorReward as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <OperatorReward as alloy_sol_types::SolStruct>::eip712_components(),
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct OperatorReward { address operator; uint256 amount; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorReward {
        pub operator: alloy::sol_types::private::Address,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
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
        impl alloy_sol_types::SolType for OperatorReward {
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
        impl alloy_sol_types::SolStruct for OperatorReward {
            const NAME: &'static str = "OperatorReward";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorReward(address operator,uint256 amount)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct RewardsMerkleClaim { uint32 rootIndex; uint32 earnerIndex; bytes earnerTreeProof; EarnerTreeMerkleLeaf earnerLeaf; uint32[] tokenIndices; bytes[] tokenTreeProofs; TokenTreeMerkleLeaf[] tokenLeaves; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsMerkleClaim {
        pub rootIndex: u32,
        pub earnerIndex: u32,
        pub earnerTreeProof: alloy::sol_types::private::Bytes,
        pub earnerLeaf: <EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        pub tokenIndices: alloy::sol_types::private::Vec<u32>,
        pub tokenTreeProofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for RewardsMerkleClaim {
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
        impl alloy_sol_types::SolStruct for RewardsMerkleClaim {
            const NAME: &'static str = "RewardsMerkleClaim";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsMerkleClaim(uint32 rootIndex,uint32 earnerIndex,bytes earnerTreeProof,EarnerTreeMerkleLeaf earnerLeaf,uint32[] tokenIndices,bytes[] tokenTreeProofs,TokenTreeMerkleLeaf[] tokenLeaves)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <EarnerTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <EarnerTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <TokenTreeMerkleLeaf as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct RewardsSubmission { StrategyAndMultiplier[] strategiesAndMultipliers; address token; uint256 amount; uint32 startTimestamp; uint32 duration; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsSubmission {
        pub strategiesAndMultipliers: alloy::sol_types::private::Vec<
            <StrategyAndMultiplier as alloy::sol_types::SolType>::RustType,
        >,
        pub token: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        pub startTimestamp: u32,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for RewardsSubmission {
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
        impl alloy_sol_types::SolStruct for RewardsSubmission {
            const NAME: &'static str = "RewardsSubmission";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RewardsSubmission(StrategyAndMultiplier[] strategiesAndMultipliers,address token,uint256 amount,uint32 startTimestamp,uint32 duration)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <StrategyAndMultiplier as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
struct StrategyAndMultiplier { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAndMultiplier {
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
        impl alloy_sol_types::SolType for StrategyAndMultiplier {
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
        impl alloy_sol_types::SolStruct for StrategyAndMultiplier {
            const NAME: &'static str = "StrategyAndMultiplier";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyAndMultiplier(address strategy,uint96 multiplier)",
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
    /**```solidity
struct TokenTreeMerkleLeaf { address token; uint256 cumulativeEarnings; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TokenTreeMerkleLeaf {
        pub token: alloy::sol_types::private::Address,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.cumulativeEarnings),
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
        impl alloy_sol_types::SolType for TokenTreeMerkleLeaf {
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
        impl alloy_sol_types::SolStruct for TokenTreeMerkleLeaf {
            const NAME: &'static str = "TokenTreeMerkleLeaf";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TokenTreeMerkleLeaf(address token,uint256 cumulativeEarnings)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
    pub struct IRewardsCoordinatorTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IRewardsCoordinatorTypes`](self) contract instance.

See the [wrapper's documentation](`IRewardsCoordinatorTypesInstance`) for more details.*/
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
    > IRewardsCoordinatorTypesInstance<T, P, N> {
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
    error AmountExceedsMax();
    error AmountIsZero();
    error CurrentlyPaused();
    error DurationExceedsMax();
    error EarningsNotGreaterThanClaimed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputArrayLengthZero();
    error InvalidAddressZero();
    error InvalidCalculationIntervalSecondsRemainder();
    error InvalidClaimProof();
    error InvalidDurationRemainder();
    error InvalidEarner();
    error InvalidEarnerLeafIndex();
    error InvalidGenesisRewardsTimestampRemainder();
    error InvalidNewPausedStatus();
    error InvalidPermissions();
    error InvalidProofLength();
    error InvalidRoot();
    error InvalidRootIndex();
    error InvalidStartTimestampRemainder();
    error InvalidTokenLeafIndex();
    error NewRootMustBeForNewCalculatedPeriod();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorsNotInAscendingOrder();
    error PreviousSplitPending();
    error RewardsEndTimestampNotElapsed();
    error RootAlreadyActivated();
    error RootDisabled();
    error RootNotActivated();
    error SplitExceedsMax();
    error StartTimestampTooFarInFuture();
    error StartTimestampTooFarInPast();
    error StrategiesNotInAscendingOrder();
    error StrategyNotWhitelisted();
    error SubmissionNotRetroactive();
    error UnauthorizedCaller();

    event AVSRewardsSubmissionCreated(address indexed avs, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event ActivationDelaySet(uint32 oldActivationDelay, uint32 newActivationDelay);
    event ClaimerForSet(address indexed earner, address indexed oldClaimer, address indexed claimer);
    event DefaultOperatorSplitBipsSet(uint16 oldDefaultOperatorSplitBips, uint16 newDefaultOperatorSplitBips);
    event DistributionRootDisabled(uint32 indexed rootIndex);
    event DistributionRootSubmitted(uint32 indexed rootIndex, bytes32 indexed root, uint32 indexed rewardsCalculationEndTimestamp, uint32 activatedAt);
    event Initialized(uint8 version);
    event OperatorAVSSplitBipsSet(address indexed caller, address indexed operator, address indexed avs, uint32 activatedAt, uint16 oldOperatorAVSSplitBips, uint16 newOperatorAVSSplitBips);
    event OperatorDirectedAVSRewardsSubmissionCreated(address indexed caller, address indexed avs, bytes32 indexed operatorDirectedRewardsSubmissionHash, uint256 submissionNonce, IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission operatorDirectedRewardsSubmission);
    event OperatorPISplitBipsSet(address indexed caller, address indexed operator, uint32 activatedAt, uint16 oldOperatorPISplitBips, uint16 newOperatorPISplitBips);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event RewardsClaimed(bytes32 root, address indexed earner, address indexed claimer, address indexed recipient, address token, uint256 claimedAmount);
    event RewardsForAllSubmitterSet(address indexed rewardsForAllSubmitter, bool indexed oldValue, bool indexed newValue);
    event RewardsSubmissionForAllCreated(address indexed submitter, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event RewardsSubmissionForAllEarnersCreated(address indexed tokenHopper, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event RewardsUpdaterSet(address indexed oldRewardsUpdater, address indexed newRewardsUpdater);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegationManager, address _strategyManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 _GENESIS_REWARDS_TIMESTAMP);

    function CALCULATION_INTERVAL_SECONDS() external view returns (uint32);
    function GENESIS_REWARDS_TIMESTAMP() external view returns (uint32);
    function MAX_FUTURE_LENGTH() external view returns (uint32);
    function MAX_RETROACTIVE_LENGTH() external view returns (uint32);
    function MAX_REWARDS_DURATION() external view returns (uint32);
    function activationDelay() external view returns (uint32);
    function allocationManager() external view returns (address);
    function beaconChainETHStrategy() external view returns (address);
    function calculateEarnerLeafHash(IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function calculateTokenLeafHash(IRewardsCoordinatorTypes.TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function checkClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim) external view returns (bool);
    function claimerFor(address earner) external view returns (address claimer);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createOperatorDirectedAVSRewardsSubmission(address avs, IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
    function createRewardsForAllEarners(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createRewardsForAllSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function cumulativeClaimed(address earner, address token) external view returns (uint256 totalClaimed);
    function currRewardsCalculationEndTimestamp() external view returns (uint32);
    function defaultOperatorSplitBips() external view returns (uint16);
    function delegationManager() external view returns (address);
    function disableRoot(uint32 rootIndex) external;
    function getCurrentClaimableDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getCurrentDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getDistributionRootAtIndex(uint256 index) external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getDistributionRootsLength() external view returns (uint256);
    function getOperatorAVSSplit(address operator, address avs) external view returns (uint16);
    function getOperatorPISplit(address operator) external view returns (uint16);
    function getRootIndexFromHash(bytes32 rootHash) external view returns (uint32);
    function initialize(address initialOwner, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _defaultSplitBips) external;
    function isAVSRewardsSubmissionHash(address avs, bytes32 hash) external view returns (bool valid);
    function isOperatorDirectedAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    function isRewardsForAllSubmitter(address submitter) external view returns (bool valid);
    function isRewardsSubmissionForAllEarnersHash(address avs, bytes32 hash) external view returns (bool valid);
    function isRewardsSubmissionForAllHash(address avs, bytes32 hash) external view returns (bool valid);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function permissionController() external view returns (address);
    function processClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim, address recipient) external;
    function processClaims(IRewardsCoordinatorTypes.RewardsMerkleClaim[] memory claims, address recipient) external;
    function renounceOwnership() external;
    function rewardsUpdater() external view returns (address);
    function setActivationDelay(uint32 _activationDelay) external;
    function setClaimerFor(address claimer) external;
    function setClaimerFor(address earner, address claimer) external;
    function setDefaultOperatorSplit(uint16 split) external;
    function setOperatorAVSSplit(address operator, address avs, uint16 split) external;
    function setOperatorPISplit(address operator, uint16 split) external;
    function setRewardsForAllSubmitter(address _submitter, bool _newValue) external;
    function setRewardsUpdater(address _rewardsUpdater) external;
    function strategyManager() external view returns (address);
    function submissionNonce(address avs) external view returns (uint256 nonce);
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
        "name": "_GENESIS_REWARDS_TIMESTAMP",
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
    "name": "calculateEarnerLeafHash",
    "inputs": [
      {
        "name": "leaf",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf",
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
        "internalType": "struct IRewardsCoordinatorTypes.TokenTreeMerkleLeaf",
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
        "internalType": "struct IRewardsCoordinatorTypes.RewardsMerkleClaim",
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
            "internalType": "struct IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf",
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
            "internalType": "struct IRewardsCoordinatorTypes.TokenTreeMerkleLeaf[]",
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
        "name": "earner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "claimer",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "createRewardsForAllEarners",
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
    "name": "createRewardsForAllSubmission",
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
    "name": "cumulativeClaimed",
    "inputs": [
      {
        "name": "earner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "token",
        "type": "address",
        "internalType": "contract IERC20"
      }
    ],
    "outputs": [
      {
        "name": "totalClaimed",
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
    "name": "getCurrentClaimableDistributionRoot",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinatorTypes.DistributionRoot",
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
        "internalType": "struct IRewardsCoordinatorTypes.DistributionRoot",
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
        "internalType": "struct IRewardsCoordinatorTypes.DistributionRoot",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "hash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "valid",
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
        "name": "submitter",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "valid",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "hash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "valid",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "hash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "valid",
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
    "name": "processClaim",
    "inputs": [
      {
        "name": "claim",
        "type": "tuple",
        "internalType": "struct IRewardsCoordinatorTypes.RewardsMerkleClaim",
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
            "internalType": "struct IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf",
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
            "internalType": "struct IRewardsCoordinatorTypes.TokenTreeMerkleLeaf[]",
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
        "internalType": "struct IRewardsCoordinatorTypes.RewardsMerkleClaim[]",
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
            "internalType": "struct IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf",
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
            "internalType": "struct IRewardsCoordinatorTypes.TokenTreeMerkleLeaf[]",
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
    "name": "setClaimerFor",
    "inputs": [
      {
        "name": "earner",
        "type": "address",
        "internalType": "address"
      },
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "nonce",
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
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission",
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
        "internalType": "struct IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission",
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
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission",
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
        "internalType": "struct IRewardsCoordinatorTypes.RewardsSubmission",
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
  },
  {
    "type": "error",
    "name": "AmountExceedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "AmountIsZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DurationExceedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "EarningsNotGreaterThanClaimed",
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
    "name": "InvalidAddressZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCalculationIntervalSecondsRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidClaimProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidDurationRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEarner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEarnerLeafIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidGenesisRewardsTimestampRemainder",
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
    "name": "InvalidProofLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRoot",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidRootIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidStartTimestampRemainder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTokenLeafIndex",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NewRootMustBeForNewCalculatedPeriod",
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
    "name": "OperatorsNotInAscendingOrder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PreviousSplitPending",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RewardsEndTimestampNotElapsed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RootAlreadyActivated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RootDisabled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "RootNotActivated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SplitExceedsMax",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StartTimestampTooFarInFuture",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StartTimestampTooFarInPast",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategiesNotInAscendingOrder",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategyNotWhitelisted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SubmissionNotRetroactive",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UnauthorizedCaller",
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
pub mod RewardsCoordinator {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6101c0346102f157601f613b8a38819003918201601f19168301916001600160401b038311848410176102f557808492610140946040528339810103126102f1578051906001600160a01b03821682036102f1576020810151906001600160a01b03821682036102f1576040810151926001600160a01b03841684036102f15760608201516001600160a01b038116939091908483036102f1576080840151956001600160a01b03871687036102f1576100bb60a08601610309565b916100c860c08701610309565b936100d560e08801610309565b956100f06101206100e96101008b01610309565b9901610309565b98156102e25760805263ffffffff841680156102ce5763ffffffff81818b1606166102bf576201518063ffffffff9106166102b05760a05260c05260e05261010052610120526101405261016052610180526101a0525f5460ff8160081c1661025b5760ff80821610610221575b60405161386f908161031b82396080518181816105b801528181610cdf01528181610f620152611bae015260a05181818161170901526117c8015260c0518181816108d301526133b7015260e05181818161149501526118440152610100518181816111b0015261333501526101205181818161140e01526132fa01526101405181818161089a0152613379015261016051818181610429015261294a01526101805181818161056401526134b901526101a051818181610bb20152612f100152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f61015e565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b63223c7b3960e11b5f5260045ffd5b630e06bd3160e01b5f5260045ffd5b634e487b7160e01b5f52601260045260245ffd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b519063ffffffff821682036102f15756fe60806040526004361015610011575f80fd5b5f3560e01c806218572c146103b357806304a0c502146103ae5780630e9a53cf146103a95780630eb38345146103a4578063131433b41461039f578063136439dd1461039a578063149bc872146103955780632b9f64a41461039057806336af41fa1461038b57806337838ed01461038657806339b70e38146103815780633a8c07861461037c5780633ccc861d146103775780633efe1db6146103725780634596021c1461036d5780634657e26a146103685780634b943960146103635780634d18cc351461035e57806358baaa3e14610359578063595c6a67146103545780635ac86ab71461034f5780635c975abb1461034a5780635e9d83481461034557806363f6a798146103405780636d21117e1461033b578063715018a6146103365780637b8f8b0514610331578063863cb9a91461032c578063865c695314610327578063886f1195146103225780638da5cb5b1461031d5780639104c319146103185780639be3d4e4146103135780639cb9a5fa1461030e5780639d45c28114610309578063a0169ddd14610304578063a50a1d9c146102ff578063aebd8bae146102fa578063b3dbb0e0146102f5578063bb7e451f146102f0578063bf21a8aa146102eb578063c46db606146102e6578063ca8aa7c7146102e1578063dcbb03b3146102dc578063de02e503146102d7578063e063f81f146102d2578063e810ce21146102cd578063ea4d3c9b146102c8578063ed71e6a2146102c3578063f22cef85146102be578063f2fde38b146102b9578063f6efbb59146102b4578063f8cd8448146102af578063f96abf2e146102aa578063fabc1cbc146102a5578063fbf1e2c1146102a0578063fce36c7d1461029b5763ff9f6cce14610296575f80fd5b611d69565b611c71565b611c49565b611b85565b611aab565b611a87565b61196f565b6118de565b611786565b611738565b6116f4565b6116c8565b611669565b611616565b6114c4565b611480565b611432565b6113f2565b6113b7565b6112ab565b61125d565b61122d565b6111d4565b611194565b611022565b610fe7565b610fb9565b610f91565b610f4d565b610eee565b610ec1565b610ea4565b610e49565b610dfb565b610dd7565b610d77565b610d5a565b610d27565b610cb4565b610c87565b610c61565b610be1565b610b9d565b610b04565b6109c3565b610937565b610902565b6108be565b61087e565b610726565b610685565b610659565b610588565b610548565b6104b5565b61044d565b61040d565b6103cd565b6001600160a01b038116036103c957565b5f80fd5b346103c95760203660031901126103c9576004356103ea816103b8565b60018060a01b03165f5260d1602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c9575f3660031901126103c9576104a7610468611fe6565b6040519182918291909160608060808301948051845263ffffffff602082015116602085015263ffffffff604082015116604085015201511515910152565b0390f35b801515036103c957565b346103c95760403660031901126103c9576004356104d2816103b8565b602435906104df826104ab565b6104e76127bd565b60018060a01b0316805f5260d160205260ff60405f205416151582151590827f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c5f80a45f5260d160205260405f209060ff8019835416911515161790555f80f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760203660031901126103c95760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064357610612926105fe915f91610614575b5061209d565b61060d606654828116146120b3565b612849565b005b610636915060203d60201161063c575b61062e8183611ec8565b81019061207d565b5f6105f8565b503d610624565b612092565b60409060031901126103c957600490565b346103c95760403660031901126103c957602061067d61067836610648565b6120d3565b604051908152f35b346103c95760203660031901126103c9576004356106a2816103b8565b60018060a01b03165f5260cc602052602060018060a01b0360405f205416604051908152f35b9181601f840112156103c9578235916001600160401b0383116103c9576020808501948460051b0101116103c957565b60206003198201126103c957600435906001600160401b0382116103c957610722916004016106c8565b9091565b346103c957610734366106f8565b9061074c610746600280606654161490565b1561211c565b335f5260d160205261076460ff60405f205416612132565b61077360026097541415612148565b60026097555f5b82811061078b576106126001609755565b8061087861079c6001938686612194565b335f90815260ce602052604090205460405160208101906107d1816107c3868633876122be565b03601f198101835282611ec8565b519020906107de836128dc565b335f90815260d060205260409020610811906108049084905b905f5260205260405f2090565b805460ff19166001179055565b61081a816122e2565b335f90815260ce60205260409020556040517f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf27048233918061085a878261230b565b0390a4604061086b602083016120c9565b9101359030903390612990565b0161077a565b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c9575f3660031901126103c957602063ffffffff60cb5460a01c16604051908152f35b90816101009103126103c95790565b346103c95760403660031901126103c9576004356001600160401b0381116103c95761096a6109a1913690600401610928565b60243590610977826103b8565b610988610746600480606654161490565b61099760026097541415612148565b6002609755612a34565b6001609755005b63ffffffff8116036103c957565b35906109c1826109a8565b565b346103c95760403660031901126103c9576024356004356109e3826109a8565b6109f4610746600880606654161490565b60cb5491610a0c336001600160a01b03851614612132565b63ffffffff81169263ffffffff8160c01c16841115610af55763ffffffff7fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd0891610a5742871061231c565b610adb610a84610a7c610a6f60ca5463ffffffff1690565b9360a01c63ffffffff1690565b844216612332565b94610ab6610a90611ee9565b88815263ffffffff8316602082015263ffffffff881660408201525f606082015261234c565b60cb805463ffffffff60c01b191660c09290921b63ffffffff60c01b16919091179055565b60405163ffffffff9094168452169180602081015b0390a4005b631ca7e50b60e21b5f5260045ffd5b346103c95760403660031901126103c9576004356001600160401b0381116103c957610b349036906004016106c8565b60243591610b41836103b8565b610b52610746600480606654161490565b610b6160026097541415612148565b60026097553681900360fe1901925f5b838110156109a1578060051b83013590858212156103c957610b97836001938601612a34565b01610b71565b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760203660031901126103c957600435610bfe816103b8565b60018060a01b03165f5260d56020526104a7610c4c60405f2063ffffffff60405191610c2983611e8d565b5461ffff8116835261ffff8160101c16602084015260201c166040820152612bef565b60405161ffff90911681529081906020820190565b346103c9575f3660031901126103c957602063ffffffff60cb5460c01c16604051908152f35b346103c95760203660031901126103c957610612600435610ca7816109a8565b610caf6127bd565b612c26565b346103c9575f3660031901126103c95760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561064357610d1f915f91610614575061209d565b610612612815565b346103c95760203660031901126103c95760043560ff81168091036103c95760016020911b806066541614604051908152f35b346103c9575f3660031901126103c9576020606654604051908152f35b346103c95760203660031901126103c9576004356001600160401b0381116103c957610daa610dcc913690600401610928565b610dc6610dc08235610dbb816109a8565b611f6b565b50611fa5565b90612d0e565b602060405160018152f35b346103c9575f3660031901126103c957602061ffff60cb5460e01c16604051908152f35b346103c95760403660031901126103c957600435610e18816103b8565b6024359060018060a01b03165f5260cf60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c957610e616127bd565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346103c9575f3660031901126103c957602060ca54604051908152f35b346103c95760203660031901126103c957610612600435610ee1816103b8565b610ee96127bd565b612e80565b346103c95760403660031901126103c9576020610f44600435610f10816103b8565b60243590610f1d826103b8565b60018060a01b03165f5260cd835260405f209060018060a01b03165f5260205260405f2090565b54604051908152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c9575f3660031901126103c9576033546040516001600160a01b039091168152602090f35b346103c9575f3660031901126103c957602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b346103c9575f3660031901126103c957610fff611ef8565b5060ca545f19810190811161101d57610468610dc06104a792611f6b565b611f1c565b346103c95760403660031901126103c95760043561103f816103b8565b6024356001600160401b0381116103c95761105e9036906004016106c8565b9190611071610746602080606654161490565b61108261107d83612ec7565b61243c565b61109160026097541415612148565b60026097556001600160a01b038216915f5b8481106110b4576106126001609755565b8061118e6110c56001938887612452565b6001600160a01b0385165f90815260ce60205260409020549060405160208101906110f6816107c385878c876125b8565b5190208861110383612fb4565b93611125610804846107f78c60018060a01b03165f5260d360205260405f2090565b61112e816122e2565b6001600160a01b038a165f90815260ce60205260409020557ffc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e060405180611177873395836125dc565b0390a4309061118960203392016120c9565b612990565b016110a3565b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760203660031901126103c9576106126004356111f4816103b8565b336130de565b6024359061ffff821682036103c957565b6044359061ffff821682036103c957565b6084359061ffff821682036103c957565b346103c95760203660031901126103c95760043561ffff811681036103c957610612906112586127bd565b613136565b346103c95760403660031901126103c95760043561127a816103b8565b6024359060018060a01b03165f5260d260205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c95760403660031901126103c9576004356112c8816103b8565b6112d06111fa565b6112e1610746608080606654161490565b6112ed61107d83612ec7565b6112ff61271061ffff831611156125f3565b7fd1e028bd664486a46ad26040e999cd2d22e1e9a094ee6afe19fcf64678f16f7461133b63ffffffff60cb5460a01c1663ffffffff4216612332565b9160018060a01b03841693845f5260d560205261138f848361138a61136f60405f2063ffffffff60405191610c2983611e8d565b6001600160a01b039095165f90815260d56020526040902090565b613191565b6040805163ffffffff95909516855261ffff91821660208601529116908301523391606090a3005b346103c95760203660031901126103c9576004356113d4816103b8565b60018060a01b03165f5260ce602052602060405f2054604051908152f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760403660031901126103c95760043561144f816103b8565b6024359060018060a01b03165f5260d060205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760603660031901126103c9576004356114e1816103b8565b6024356114ed816103b8565b6114f561120b565b611506610746604080606654161490565b61151261107d84612ec7565b61152461271061ffff831611156125f3565b7f48e198b6ae357e529204ee53a8e514c470ff77d9cc8e4f7207f8b5d490ae693461156063ffffffff60cb5460a01c1663ffffffff4216612332565b9160018060a01b03851693845f5260d46020526115dc848361138a846115c76115ac6115a76115a28460405f209060018060a01b03165f5260205260405f2090565b6123fe565b612bef565b6001600160a01b03909c165f90815260d46020526040902090565b9060018060a01b03165f5260205260405f2090565b6040805163ffffffff95909516855261ffff968716602086015291909516908301526001600160a01b039093169233918060608101610af0565b346103c95760203660031901126103c9576104a7610468610dc060043561163b611ef8565b50611f6b565b60409060031901126103c957600435611659816103b8565b90602435611666816103b8565b90565b346103c9576116b86116a661167d36611641565b9060018060a01b03165f5260d460205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff60405191610c2983611e8d565b60405161ffff9091168152602090f35b346103c95760203660031901126103c95760206116e6600435612621565b63ffffffff60405191168152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760403660031901126103c957600435611755816103b8565b6024359060018060a01b03165f5260d360205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c95761179436611641565b6117a061107d83612ec7565b6040516336b87bd760e11b81526001600160a01b0383166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610643575f916118bf575b50801561181a575b9161181561061293612686565b6130de565b5060405163ba1a84e560e01b81526001600160a01b038316600482015291602083806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156106435761061293611815915f91611890575b50151591935050611808565b6118b2915060203d6020116118b8575b6118aa8183611ec8565b810190612677565b5f611884565b503d6118a0565b6118d8915060203d60201161063c5761062e8183611ec8565b5f611800565b346103c95760203660031901126103c9576004356118fb816103b8565b6119036127bd565b6001600160a01b0381161561191b5761061290612e38565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346103c95760a03660031901126103c95760043561198c816103b8565b6119ff60243560443561199e816103b8565b606435906119ab826109a8565b6119b361121c565b925f54956119e56119cf6119cb8960ff9060081c1690565b1590565b80988199611a79575b8115611a59575b5061269c565b866119f6600160ff195f5416175f55565b611a42576126ff565b611a0557005b611a1361ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b611a5461010061ff00195f5416175f55565b6126ff565b303b15915081611a6b575b505f6119df565b60ff1660011490505f611a64565b600160ff82161091506119d8565b346103c95760403660031901126103c957602061067d611aa636610648565b61271d565b346103c95760203660031901126103c957600435611ac8816109a8565b611ad9610746600880606654161490565b611aee60018060a01b0360cb54163314612132565b60ca549063ffffffff811691821015611b7657611b0c600191611f6b565b500163ffffffff8154611b2560ff8260401c1615612764565b60201c16421015611b6757805460ff60401b1916600160401b1790557fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e5f80a2005b630c36f66560e21b5f5260045ffd5b6394a8d38960e01b5f5260045ffd5b346103c95760203660031901126103c95760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610643575f91611c0e575b506001600160a01b03163303611bff576106129061277a565b63794821ff60e01b5f5260045ffd5b90506020813d602011611c41575b81611c2960209383611ec8565b810103126103c95751611c3b816103b8565b5f611be6565b3d9150611c1c565b346103c9575f3660031901126103c95760cb546040516001600160a01b039091168152602090f35b346103c957611c7f366106f8565b90611c91610746600180606654161490565b611ca060026097541415612148565b60026097555f5b828110611cb8576106126001609755565b80611d63611cc96001938686612194565b335f90815260ce60205260409020546040516020810190611cf0816107c3868633876122be565b51902090611cfd836128dc565b335f90815260cf60205260409020611d1a906108049084906107f7565b611d23816122e2565b335f90815260ce60205260409020556040517f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e628133918061085a878261230b565b01611ca7565b346103c957611d77366106f8565b90611d89610746601080606654161490565b335f5260d1602052611da160ff60405f205416612132565b611db060026097541415612148565b60026097555f5b828110611dc8576106126001609755565b80611e73611dd96001938686612194565b335f90815260ce60205260409020546040516020810190611e00816107c3868633876122be565b51902090611e0d836128dc565b335f90815260d260205260409020611e2a906108049084906107f7565b611e33816122e2565b335f90815260ce60205260409020556040517f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b33918061085a878261230b565b01611db7565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b03821117611ea857604052565b611e79565b608081019081106001600160401b03821117611ea857604052565b90601f801991011681019081106001600160401b03821117611ea857604052565b604051906109c1608083611ec8565b60405190611f0582611ead565b5f6060838281528260208201528260408201520152565b634e487b7160e01b5f52601160045260245ffd5b801561101d575f190190565b5f1981019190821161101d57565b9190820391821161101d57565b634e487b7160e01b5f52603260045260245ffd5b60ca54811015611f875760ca5f5260205f209060011b01905f90565b611f57565b8054821015611f87575f5260205f209060011b01905f90565b90604051611fb281611ead565b606060ff6001839580548552015463ffffffff8116602085015263ffffffff8160201c16604085015260401c161515910152565b611fee611ef8565b5060ca54805b6120195750612001611ee9565b5f81525f60208201525f60408201525f606082015290565b612028610dc0610dbb83611f3c565b906120396119cb6060840151151590565b80612057575b6120535761204d9150611f30565b80611ff4565b5090565b5061207561206c604084015163ffffffff1690565b63ffffffff1690565b42101561203f565b908160209103126103c95751611666816104ab565b6040513d5f823e3d90fd5b156120a457565b631d77d47760e21b5f5260045ffd5b156120ba57565b63c61dca5d60e01b5f5260045ffd5b35611666816103b8565b60208135916120e1836103b8565b01356040519060208201925f84526001600160601b03199060601b166021830152603582015260358152612116605582611ec8565b51902090565b1561212357565b63840a48d560e01b5f5260045ffd5b1561213957565b635c427cd960e01b5f5260045ffd5b1561214f57565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015611f875760051b81013590609e19813603018212156103c9570190565b9035601e19823603018112156103c95701602081359101916001600160401b0382116103c9578160061b360383136103c957565b916020908281520191905f905b8082106122045750505090565b9091928335612212816103b8565b6001600160a01b0316815260208401356001600160601b03811691908290036103c9576040816001936020839401520194019201906121f7565b90608063ffffffff8161227061226286806121b6565b60a0875260a08701916121ea565b94602081013561227f816103b8565b6001600160a01b03166020860152604081810135908601528260608201356122a6816109a8565b16606086015201356122b7816109a8565b1691015290565b611666939260609260018060a01b031682526020820152816040820152019061224c565b906001820180921161101d57565b906020820180921161101d57565b9190820180921161101d57565b90602061166692818152019061224c565b1561232357565b6306957c9160e11b5f5260045ffd5b9063ffffffff8091169116019063ffffffff821161101d57565b60ca54600160401b811015611ea85780600161236d920160ca5560ca611f8c565b9190916123eb57606060016109c19383518155019163ffffffff60208201511663ffffffff198454161783556123ca63ffffffff604083015116849067ffffffff0000000082549160201b169067ffffffff000000001916179055565b0151815460ff60401b191690151560401b68ff000000000000000016179055565b634e487b7160e01b5f525f60045260245ffd5b9060405161240b81611e8d565b604063ffffffff82945461ffff8116845261ffff8160101c16602085015260201c16910152565b35611666816109a8565b1561244357565b63932d94f760e01b5f5260045ffd5b9190811015611f875760051b8101359060be19813603018212156103c9570190565b9035601e19823603018112156103c95701602081359101916001600160401b0382116103c95781360383136103c957565b908060209392818452848401375f828201840152601f01601f1916010190565b91906124e26124d484806121b6565b60c0845260c08401916121ea565b9060208401356124f1816103b8565b6001600160a01b031660208281019190915261251060408601866121b6565b838503604085015280855293909101925f5b818110612584575050506125768461255061254360606116669798016109b6565b63ffffffff166060850152565b61256c61255f608083016109b6565b63ffffffff166080850152565b60a0810190612474565b9160a08185039101526124a5565b9091936040806001928735612598816103b8565b848060a01b03168152602088013560208201520195019101919091612522565b611666939260609260018060a01b03168252602082015281604082015201906124c5565b6040906116669392815281602082015201906124c5565b156125fa57565b63891c63df60e01b5f5260045ffd5b63ffffffff5f199116019063ffffffff821161101d57565b63ffffffff60ca54165b63ffffffff81166126455763504570e360e01b5f5260045ffd5b81612652610dbb83612609565b50541461266d5763ffffffff16801561101d575f190161262b565b6116669150612609565b908160209103126103c9575190565b1561268d57565b63fb494ea160e01b5f5260045ffd5b156126a357565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b61125892610ee9610caf926127186109c1989795612849565b612e38565b602081359161272b836103b8565b0135604051906020820192600160f81b84526001600160601b03199060601b166021830152603582015260358152612116605582611ec8565b1561276b57565b631b14174b60e01b5f5260045ffd5b61278b6066541982198116146120b3565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b6033546001600160a01b031633036127d157565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b903590601e19813603018212156103c957018035906001600160401b0382116103c957602001918160061b360383136103c957565b156128b757565b6310eb483f60e21b5f5260045ffd5b156128cd57565b63070b5a6f60e21b5f5260045ffd5b61293a6f4b3b4ca85a86c47a098a223fffffffff604061292693612900818061287b565b959060608301358097612912826109a8565b608085013592612921846109a8565b6132d5565b01356129338115156128b0565b11156128c6565b612943816109a8565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001642019081421161101d5763ffffffff161161298157565b637ee2b44360e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b0392831660248201529290911660448301526064808301939093529181526109c1916129d4608483611ec8565b61355b565b903590601e19813603018212156103c957018035906001600160401b0382116103c957602001918160051b360383136103c957565b9190811015611f875760061b0190565b15612a2557565b63aa385e8160e01b5f5260045ffd5b90612a44610dc0610dbb84612432565b90612a4f8284612d0e565b612a5b606084016120c9565b93612a86612a798660018060a01b03165f5260cc60205260405f2090565b546001600160a01b031690565b6001600160a01b03811615612be8575b90936001600160a01b0390911691612aaf338414612132565b6001600160a01b038616915f5b612ac960a08301836129d9565b9050811015612bde5780612aec600192612ae660e086018661287b565b90612a0e565b86867f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce318c612b4a612b2d8260018060a01b03165f5260cd60205260405f2090565b612b36876120c9565b60018060a01b03165f5260205260405f2090565b54612b8f612b86612b6b602089013593612b65818611612a1e565b84611f4a565b6001600160a01b039094165f90815260cd6020526040902090565b612b36886120c9565b55612ba3818a612b9e886120c9565b61362d565b612bae8c51956120c9565b604080519687526001600160a01b0391909116602087015285015260a086901b869003881693606090a401612abc565b5050505050509050565b5084612a96565b604081015163ffffffff169081612c0f57505061ffff60cb5460e01c1690565b61ffff914210612c2157602001511690565b511690565b60cb54907faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b36040805163ffffffff8560a01c16815263ffffffff84166020820152a163ffffffff60a01b1990911660a09190911b63ffffffff60a01b161760cb55565b15612c9057565b631437a2bb60e31b5f5260045ffd5b15612ca657565b6343714afd60e01b5f5260045ffd5b903590601e19813603018212156103c957018035906001600160401b0382116103c9576020019181360383136103c957565b9190811015611f875760051b0190565b90821015611f87576107229160051b810190612cb5565b919091612d29612d246119cb6060860151151590565b612764565b612d48612d4061206c604086015163ffffffff1690565b421015612c89565b60a0810190612d5782826129d9565b9050612d7460c0830191612d6b83856129d9565b91905014612c9f565b612dbb612d8182846129d9565b969050612d9660e0850197612d6b898761287b565b51612da360208501612432565b612db06040860186612cb5565b9160608701936136b4565b6080820135925f5b612dcd82856129d9565b9050811015612e2f5780612e2985612ae68a612e2285612e1981612e138c8f612e0860019d8f612e0290612e0d94508d6129d9565b90612ce7565b612432565b986129d9565b90612cf7565b9490938c61287b565b928a6136ff565b01612dc3565b50505050509050565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b60cb546001600160a01b0391821691829082167f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb5f80a36001600160a01b0319161760cb55565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af1908115610643575f91612f43575090565b611666915060203d60201161063c5761062e8183611ec8565b15612f6357565b63796cc52560e01b5f5260045ffd5b15612f7957565b63150358a160e21b5f5260045ffd5b15612f8f57565b630863a45360e11b5f5260045ffd5b15612fa557565b6310fb47f160e31b5f5260045ffd5b9061301a91613028612fc6828061287b565b606084019591612fed90612fd988612432565b6080870193612fe785612432565b926132d5565b61301461300e6040860197612e086130058a8961287b565b90501515612f5c565b91612432565b90612332565b63ffffffff42911610612f72565b5f928391825b613038838361287b565b90508410156130bd576130b460019161309561305887612ae6888861287b565b9161307b613074613068856120c9565b6001600160a01b031690565b1515612f88565b613087613068846120c9565b90858060a01b031610612f9e565b6130ae6020820135916130a98315156128b0565b6120c9565b976122fe565b9301929461302e565b50505050906116666f4b3b4ca85a86c47a098a223fffffffff8211156128c6565b6001600160a01b039081165f81815260cc6020526040812080546001600160a01b03198116958516958617909155909216917fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca3129080a4565b60cb54907fe6cd4edfdcc1f6d130ab35f73d72378f3a642944fb4ee5bd84b7807a81ea1c4e6040805161ffff8560e01c16815261ffff84166020820152a161ffff60e01b1990911660e09190911b61ffff60e01b161760cb55565b91909180549263ffffffff8460201c169384421115613216576109c1946131fd575060cb54825461ffff191660e09190911c61ffff161782555b815467ffffffffffff0000191660109190911b63ffff0000161760209290921b67ffffffff0000000016919091179055565b825461ffff191660109190911c61ffff161782556131cb565b637b1e25c560e01b5f5260045ffd5b1561322c57565b630dd0b9f560e21b5f5260045ffd5b9063ffffffff169081156132535763ffffffff160690565b634e487b7160e01b5f52601260045260245ffd5b1561326e57565b63ee66470560e01b5f5260045ffd5b1561328457565b633c1a94f160e21b5f5260045ffd5b1561329a57565b63041aa75760e11b5f5260045ffd5b156132b057565b632efd965160e11b5f5260045ffd5b156132c657565b63dfad9ca160e01b5f5260045ffd5b929161336e61336861206c6133b594956132f0871515612f5c565b61332863ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff83161115613225565b61336263ffffffff61335b7f0000000000000000000000000000000000000000000000000000000000000000809461323b565b1615613267565b8461323b565b1561327d565b63ffffffff61339f817f00000000000000000000000000000000000000000000000000000000000000001642611f4a565b91168091111590816134b0575b50929192613293565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f90815b8183106133f2575050505050565b6134006130a9848487612a0e565b60405163198f077960e21b81526001600160a01b03821660048201529091906020816024818a5afa92831561064357600193613464925f91613492575b50801561346c575b61344e906132a9565b838060a01b03168092848060a01b0316106132bf565b9201916133e4565b5060a084901b849003811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613445565b6134aa915060203d811161063c5761062e8183611ec8565b5f61343d565b905063ffffffff7f00000000000000000000000000000000000000000000000000000000000000001611155f6133ac565b6001600160401b038111611ea857601f01601f191660200190565b1561350357565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b60018060a01b03169060405190613573604083611ec8565b602082527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020830152823b156135e8575f816135c3948260208195519301915af16135bd6137af565b906137de565b8051806135ce575050565b816020806135e3936109c1950101910161207d565b6134fc565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b60405163a9059cbb60e01b60208201526001600160a01b0390921660248301526044808301939093529181526109c1916129d4606483611ec8565b929192613674826134e1565b916136826040519384611ec8565b8294818452818301116103c9578281602093845f960137010152565b156136a557565b6369ca16c960e01b5f5260045ffd5b91929063ffffffff169160018260051c1b8310156136f1576136e76136ec946136df6109c1976120d3565b933691613668565b613739565b61369e565b62c6c39d60e71b5f5260045ffd5b91929063ffffffff169160018260051c1b83101561372a576136e76136ec946136df6109c19761271d565b63054ff4df60e51b5f5260045ffd5b93909291601f8551166137a05791906020925b85518411613797576001831661377d575f528285015160205261377660405f209260011c936122f0565b929161374c565b838601515f5260205261377660405f209260011c936122f0565b92509350501490565b6313717da960e21b5f5260045ffd5b3d156137d9573d906137c0826134e1565b916137ce6040519384611ec8565b82523d5f602084013e565b606090565b909190156137ea575090565b8151156137fa5750805190602001fd5b604460209160405192839162461bcd60e51b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fdfea2646970667358221220885c6a97073df6643892a247599cb0fd91246ca891d5b0cd186def0d9c0d7bb064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\xC04a\x02\xF1W`\x1Fa;\x8A8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02\xF5W\x80\x84\x92a\x01@\x94`@R\x839\x81\x01\x03\x12a\x02\xF1W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xF1W` \x81\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xF1W`@\x81\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\x02\xF1W``\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x90\x91\x90\x84\x83\x03a\x02\xF1W`\x80\x84\x01Q\x95`\x01`\x01`\xA0\x1B\x03\x87\x16\x87\x03a\x02\xF1Wa\0\xBB`\xA0\x86\x01a\x03\tV[\x91a\0\xC8`\xC0\x87\x01a\x03\tV[\x93a\0\xD5`\xE0\x88\x01a\x03\tV[\x95a\0\xF0a\x01 a\0\xE9a\x01\0\x8B\x01a\x03\tV[\x99\x01a\x03\tV[\x98\x15a\x02\xE2W`\x80Rc\xFF\xFF\xFF\xFF\x84\x16\x80\x15a\x02\xCEWc\xFF\xFF\xFF\xFF\x81\x81\x8B\x16\x06\x16a\x02\xBFWb\x01Q\x80c\xFF\xFF\xFF\xFF\x91\x06\x16a\x02\xB0W`\xA0R`\xC0R`\xE0Ra\x01\0Ra\x01 Ra\x01@Ra\x01`Ra\x01\x80Ra\x01\xA0R_T`\xFF\x81`\x08\x1C\x16a\x02[W`\xFF\x80\x82\x16\x10a\x02!W[`@Qa8o\x90\x81a\x03\x1B\x829`\x80Q\x81\x81\x81a\x05\xB8\x01R\x81\x81a\x0C\xDF\x01R\x81\x81a\x0Fb\x01Ra\x1B\xAE\x01R`\xA0Q\x81\x81\x81a\x17\t\x01Ra\x17\xC8\x01R`\xC0Q\x81\x81\x81a\x08\xD3\x01Ra3\xB7\x01R`\xE0Q\x81\x81\x81a\x14\x95\x01Ra\x18D\x01Ra\x01\0Q\x81\x81\x81a\x11\xB0\x01Ra35\x01Ra\x01 Q\x81\x81\x81a\x14\x0E\x01Ra2\xFA\x01Ra\x01@Q\x81\x81\x81a\x08\x9A\x01Ra3y\x01Ra\x01`Q\x81\x81\x81a\x04)\x01Ra)J\x01Ra\x01\x80Q\x81\x81\x81a\x05d\x01Ra4\xB9\x01Ra\x01\xA0Q\x81\x81\x81a\x0B\xB2\x01Ra/\x10\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01^V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c\"<{9`\xE1\x1B_R`\x04_\xFD[c\x0E\x06\xBD1`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\xF1WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\x18W,\x14a\x03\xB3W\x80c\x04\xA0\xC5\x02\x14a\x03\xAEW\x80c\x0E\x9AS\xCF\x14a\x03\xA9W\x80c\x0E\xB3\x83E\x14a\x03\xA4W\x80c\x13\x143\xB4\x14a\x03\x9FW\x80c\x13d9\xDD\x14a\x03\x9AW\x80c\x14\x9B\xC8r\x14a\x03\x95W\x80c+\x9Fd\xA4\x14a\x03\x90W\x80c6\xAFA\xFA\x14a\x03\x8BW\x80c7\x83\x8E\xD0\x14a\x03\x86W\x80c9\xB7\x0E8\x14a\x03\x81W\x80c:\x8C\x07\x86\x14a\x03|W\x80c<\xCC\x86\x1D\x14a\x03wW\x80c>\xFE\x1D\xB6\x14a\x03rW\x80cE\x96\x02\x1C\x14a\x03mW\x80cFW\xE2j\x14a\x03hW\x80cK\x949`\x14a\x03cW\x80cM\x18\xCC5\x14a\x03^W\x80cX\xBA\xAA>\x14a\x03YW\x80cY\\jg\x14a\x03TW\x80cZ\xC8j\xB7\x14a\x03OW\x80c\\\x97Z\xBB\x14a\x03JW\x80c^\x9D\x83H\x14a\x03EW\x80cc\xF6\xA7\x98\x14a\x03@W\x80cm!\x11~\x14a\x03;W\x80cqP\x18\xA6\x14a\x036W\x80c{\x8F\x8B\x05\x14a\x031W\x80c\x86<\xB9\xA9\x14a\x03,W\x80c\x86\\iS\x14a\x03'W\x80c\x88o\x11\x95\x14a\x03\"W\x80c\x8D\xA5\xCB[\x14a\x03\x1DW\x80c\x91\x04\xC3\x19\x14a\x03\x18W\x80c\x9B\xE3\xD4\xE4\x14a\x03\x13W\x80c\x9C\xB9\xA5\xFA\x14a\x03\x0EW\x80c\x9DE\xC2\x81\x14a\x03\tW\x80c\xA0\x16\x9D\xDD\x14a\x03\x04W\x80c\xA5\n\x1D\x9C\x14a\x02\xFFW\x80c\xAE\xBD\x8B\xAE\x14a\x02\xFAW\x80c\xB3\xDB\xB0\xE0\x14a\x02\xF5W\x80c\xBB~E\x1F\x14a\x02\xF0W\x80c\xBF!\xA8\xAA\x14a\x02\xEBW\x80c\xC4m\xB6\x06\x14a\x02\xE6W\x80c\xCA\x8A\xA7\xC7\x14a\x02\xE1W\x80c\xDC\xBB\x03\xB3\x14a\x02\xDCW\x80c\xDE\x02\xE5\x03\x14a\x02\xD7W\x80c\xE0c\xF8\x1F\x14a\x02\xD2W\x80c\xE8\x10\xCE!\x14a\x02\xCDW\x80c\xEAM<\x9B\x14a\x02\xC8W\x80c\xEDq\xE6\xA2\x14a\x02\xC3W\x80c\xF2,\xEF\x85\x14a\x02\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x02\xB9W\x80c\xF6\xEF\xBBY\x14a\x02\xB4W\x80c\xF8\xCD\x84H\x14a\x02\xAFW\x80c\xF9j\xBF.\x14a\x02\xAAW\x80c\xFA\xBC\x1C\xBC\x14a\x02\xA5W\x80c\xFB\xF1\xE2\xC1\x14a\x02\xA0W\x80c\xFC\xE3l}\x14a\x02\x9BWc\xFF\x9Fl\xCE\x14a\x02\x96W_\x80\xFD[a\x1DiV[a\x1CqV[a\x1CIV[a\x1B\x85V[a\x1A\xABV[a\x1A\x87V[a\x19oV[a\x18\xDEV[a\x17\x86V[a\x178V[a\x16\xF4V[a\x16\xC8V[a\x16iV[a\x16\x16V[a\x14\xC4V[a\x14\x80V[a\x142V[a\x13\xF2V[a\x13\xB7V[a\x12\xABV[a\x12]V[a\x12-V[a\x11\xD4V[a\x11\x94V[a\x10\"V[a\x0F\xE7V[a\x0F\xB9V[a\x0F\x91V[a\x0FMV[a\x0E\xEEV[a\x0E\xC1V[a\x0E\xA4V[a\x0EIV[a\r\xFBV[a\r\xD7V[a\rwV[a\rZV[a\r'V[a\x0C\xB4V[a\x0C\x87V[a\x0CaV[a\x0B\xE1V[a\x0B\x9DV[a\x0B\x04V[a\t\xC3V[a\t7V[a\t\x02V[a\x08\xBEV[a\x08~V[a\x07&V[a\x06\x85V[a\x06YV[a\x05\x88V[a\x05HV[a\x04\xB5V[a\x04MV[a\x04\rV[a\x03\xCDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xC9WV[_\x80\xFD[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x03\xEA\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD1` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x04\xA7a\x04ha\x1F\xE6V[`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x85\x01R\x01Q\x15\x15\x91\x01RV[\x03\x90\xF3[\x80\x15\x15\x03a\x03\xC9WV[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x04\xD2\x81a\x03\xB8V[`$5\x90a\x04\xDF\x82a\x04\xABV[a\x04\xE7a'\xBDV[`\x01\x80`\xA0\x1B\x03\x16\x80_R`\xD1` R`\xFF`@_ T\x16\x15\x15\x82\x15\x15\x90\x82\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C_\x80\xA4_R`\xD1` R`@_ \x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90U_\x80\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06CWa\x06\x12\x92a\x05\xFE\x91_\x91a\x06\x14W[Pa \x9DV[a\x06\r`fT\x82\x81\x16\x14a \xB3V[a(IV[\0[a\x066\x91P` =` \x11a\x06<W[a\x06.\x81\x83a\x1E\xC8V[\x81\x01\x90a }V[_a\x05\xF8V[P=a\x06$V[a \x92V[`@\x90`\x03\x19\x01\x12a\x03\xC9W`\x04\x90V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x06}a\x06x6a\x06HV[a \xD3V[`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x06\xA2\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xC9W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xC9W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\xC9WV[` `\x03\x19\x82\x01\x12a\x03\xC9W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9Wa\x07\"\x91`\x04\x01a\x06\xC8V[\x90\x91V[4a\x03\xC9Wa\x0746a\x06\xF8V[\x90a\x07La\x07F`\x02\x80`fT\x16\x14\x90V[\x15a!\x1CV[3_R`\xD1` Ra\x07d`\xFF`@_ T\x16a!2V[a\x07s`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x07\x8BWa\x06\x12`\x01`\x97UV[\x80a\x08xa\x07\x9C`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x07\xD1\x81a\x07\xC3\x86\x863\x87a\"\xBEV[\x03`\x1F\x19\x81\x01\x83R\x82a\x1E\xC8V[Q\x90 \x90a\x07\xDE\x83a(\xDCV[3_\x90\x81R`\xD0` R`@\x90 a\x08\x11\x90a\x08\x04\x90\x84\x90[\x90_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x08\x1A\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x823\x91\x80a\x08Z\x87\x82a#\x0BV[\x03\x90\xA4`@a\x08k` \x83\x01a \xC9V[\x91\x015\x900\x903\x90a)\x90V[\x01a\x07zV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16`@Q\x90\x81R\xF3[\x90\x81a\x01\0\x91\x03\x12a\x03\xC9W\x90V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\tja\t\xA1\x916\x90`\x04\x01a\t(V[`$5\x90a\tw\x82a\x03\xB8V[a\t\x88a\x07F`\x04\x80`fT\x16\x14\x90V[a\t\x97`\x02`\x97T\x14\x15a!HV[`\x02`\x97Ua*4V[`\x01`\x97U\0[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xC9WV[5\x90a\t\xC1\x82a\t\xA8V[V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`$5`\x045a\t\xE3\x82a\t\xA8V[a\t\xF4a\x07F`\x08\x80`fT\x16\x14\x90V[`\xCBT\x91a\n\x0C3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a!2V[c\xFF\xFF\xFF\xFF\x81\x16\x92c\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16\x84\x11\x15a\n\xF5Wc\xFF\xFF\xFF\xFF\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91a\nWB\x87\x10a#\x1CV[a\n\xDBa\n\x84a\n|a\no`\xCATc\xFF\xFF\xFF\xFF\x16\x90V[\x93`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x84B\x16a#2V[\x94a\n\xB6a\n\x90a\x1E\xE9V[\x88\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`@\x82\x01R_``\x82\x01Ra#LV[`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\xC0\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x91\x90\x91\x17\x90UV[`@Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R\x16\x91\x80` \x81\x01[\x03\x90\xA4\0[c\x1C\xA7\xE5\x0B`\xE2\x1B_R`\x04_\xFD[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\x0B4\x906\x90`\x04\x01a\x06\xC8V[`$5\x91a\x0BA\x83a\x03\xB8V[a\x0BRa\x07F`\x04\x80`fT\x16\x14\x90V[a\x0Ba`\x02`\x97T\x14\x15a!HV[`\x02`\x97U6\x81\x90\x03`\xFE\x19\x01\x92_[\x83\x81\x10\x15a\t\xA1W\x80`\x05\x1B\x83\x015\x90\x85\x82\x12\x15a\x03\xC9Wa\x0B\x97\x83`\x01\x93\x86\x01a*4V[\x01a\x0BqV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x0B\xFE\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD5` Ra\x04\xA7a\x0CL`@_ c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[Ta\xFF\xFF\x81\x16\x83Ra\xFF\xFF\x81`\x10\x1C\x16` \x84\x01R` \x1C\x16`@\x82\x01Ra+\xEFV[`@Qa\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` c\xFF\xFF\xFF\xFF`\xCBT`\xC0\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x0C\xA7\x81a\t\xA8V[a\x0C\xAFa'\xBDV[a,&V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06CWa\r\x1F\x91_\x91a\x06\x14WPa \x9DV[a\x06\x12a(\x15V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`\xFF\x81\x16\x80\x91\x03a\x03\xC9W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `fT`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\r\xAAa\r\xCC\x916\x90`\x04\x01a\t(V[a\r\xC6a\r\xC0\x825a\r\xBB\x81a\t\xA8V[a\x1FkV[Pa\x1F\xA5V[\x90a-\x0EV[` `@Q`\x01\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x0E\x18\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xCF` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x0Eaa'\xBDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `\xCAT`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x0E\xE1\x81a\x03\xB8V[a\x0E\xE9a'\xBDV[a.\x80V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x0FD`\x045a\x0F\x10\x81a\x03\xB8V[`$5\x90a\x0F\x1D\x82a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCD\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x0F\xFFa\x1E\xF8V[P`\xCAT_\x19\x81\x01\x90\x81\x11a\x10\x1DWa\x04ha\r\xC0a\x04\xA7\x92a\x1FkV[a\x1F\x1CV[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x10?\x81a\x03\xB8V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\x10^\x906\x90`\x04\x01a\x06\xC8V[\x91\x90a\x10qa\x07F` \x80`fT\x16\x14\x90V[a\x10\x82a\x10}\x83a.\xC7V[a$<V[a\x10\x91`\x02`\x97T\x14\x15a!HV[`\x02`\x97U`\x01`\x01`\xA0\x1B\x03\x82\x16\x91_[\x84\x81\x10a\x10\xB4Wa\x06\x12`\x01`\x97UV[\x80a\x11\x8Ea\x10\xC5`\x01\x93\x88\x87a$RV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\xCE` R`@\x90 T\x90`@Q` \x81\x01\x90a\x10\xF6\x81a\x07\xC3\x85\x87\x8C\x87a%\xB8V[Q\x90 \x88a\x11\x03\x83a/\xB4V[\x93a\x11%a\x08\x04\x84a\x07\xF7\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xD3` R`@_ \x90V[a\x11.\x81a\"\xE2V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\xCE` R`@\x90 U\x7F\xFC\x88\x88\xBF\xFDq\x1D\xA6\x0B\xC5\t+3\xF6w\xD8\x18\x96\xFE\x80\xEC\xC6w\xB8L\xFA\xB8\x18Db\xB6\xE0`@Q\x80a\x11w\x873\x95\x83a%\xDCV[\x03\x90\xA40\x90a\x11\x89` 3\x92\x01a \xC9V[a)\x90V[\x01a\x10\xA3V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x11\xF4\x81a\x03\xB8V[3a0\xDEV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[`\x845\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xC9Wa\x06\x12\x90a\x12Xa'\xBDV[a16V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x12z\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD2` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x12\xC8\x81a\x03\xB8V[a\x12\xD0a\x11\xFAV[a\x12\xE1a\x07F`\x80\x80`fT\x16\x14\x90V[a\x12\xEDa\x10}\x83a.\xC7V[a\x12\xFFa'\x10a\xFF\xFF\x83\x16\x11\x15a%\xF3V[\x7F\xD1\xE0(\xBDfD\x86\xA4j\xD2`@\xE9\x99\xCD-\"\xE1\xE9\xA0\x94\xEEj\xFE\x19\xFC\xF6Fx\xF1ota\x13;c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16c\xFF\xFF\xFF\xFFB\x16a#2V[\x91`\x01\x80`\xA0\x1B\x03\x84\x16\x93\x84_R`\xD5` Ra\x13\x8F\x84\x83a\x13\x8Aa\x13o`@_ c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16_\x90\x81R`\xD5` R`@\x90 \x90V[a1\x91V[`@\x80Qc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85Ra\xFF\xFF\x91\x82\x16` \x86\x01R\x91\x16\x90\x83\x01R3\x91``\x90\xA3\0[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x13\xD4\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCE` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x14O\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD0` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W``6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x14\xE1\x81a\x03\xB8V[`$5a\x14\xED\x81a\x03\xB8V[a\x14\xF5a\x12\x0BV[a\x15\x06a\x07F`@\x80`fT\x16\x14\x90V[a\x15\x12a\x10}\x84a.\xC7V[a\x15$a'\x10a\xFF\xFF\x83\x16\x11\x15a%\xF3V[\x7FH\xE1\x98\xB6\xAE5~R\x92\x04\xEES\xA8\xE5\x14\xC4p\xFFw\xD9\xCC\x8EOr\x07\xF8\xB5\xD4\x90\xAEi4a\x15`c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16c\xFF\xFF\xFF\xFFB\x16a#2V[\x91`\x01\x80`\xA0\x1B\x03\x85\x16\x93\x84_R`\xD4` Ra\x15\xDC\x84\x83a\x13\x8A\x84a\x15\xC7a\x15\xACa\x15\xA7a\x15\xA2\x84`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a#\xFEV[a+\xEFV[`\x01`\x01`\xA0\x1B\x03\x90\x9C\x16_\x90\x81R`\xD4` R`@\x90 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[`@\x80Qc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85Ra\xFF\xFF\x96\x87\x16` \x86\x01R\x91\x90\x95\x16\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x923\x91\x80``\x81\x01a\n\xF0V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x04\xA7a\x04ha\r\xC0`\x045a\x16;a\x1E\xF8V[Pa\x1FkV[`@\x90`\x03\x19\x01\x12a\x03\xC9W`\x045a\x16Y\x81a\x03\xB8V[\x90`$5a\x16f\x81a\x03\xB8V[\x90V[4a\x03\xC9Wa\x16\xB8a\x16\xA6a\x16}6a\x16AV[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD4` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W` a\x16\xE6`\x045a&!V[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x17U\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD3` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9Wa\x17\x946a\x16AV[a\x17\xA0a\x10}\x83a.\xC7V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06CW_\x91a\x18\xBFW[P\x80\x15a\x18\x1AW[\x91a\x18\x15a\x06\x12\x93a&\x86V[a0\xDEV[P`@Qc\xBA\x1A\x84\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R\x91` \x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06CWa\x06\x12\x93a\x18\x15\x91_\x91a\x18\x90W[P\x15\x15\x91\x93PPa\x18\x08V[a\x18\xB2\x91P` =` \x11a\x18\xB8W[a\x18\xAA\x81\x83a\x1E\xC8V[\x81\x01\x90a&wV[_a\x18\x84V[P=a\x18\xA0V[a\x18\xD8\x91P` =` \x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[_a\x18\0V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x18\xFB\x81a\x03\xB8V[a\x19\x03a'\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x19\x1BWa\x06\x12\x90a.8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xC9W`\xA06`\x03\x19\x01\x12a\x03\xC9W`\x045a\x19\x8C\x81a\x03\xB8V[a\x19\xFF`$5`D5a\x19\x9E\x81a\x03\xB8V[`d5\x90a\x19\xAB\x82a\t\xA8V[a\x19\xB3a\x12\x1CV[\x92_T\x95a\x19\xE5a\x19\xCFa\x19\xCB\x89`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x98\x81\x99a\x1AyW[\x81\x15a\x1AYW[Pa&\x9CV[\x86a\x19\xF6`\x01`\xFF\x19_T\x16\x17_UV[a\x1ABWa&\xFFV[a\x1A\x05W\0[a\x1A\x13a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x1ATa\x01\0a\xFF\0\x19_T\x16\x17_UV[a&\xFFV[0;\x15\x91P\x81a\x1AkW[P_a\x19\xDFV[`\xFF\x16`\x01\x14\x90P_a\x1AdV[`\x01`\xFF\x82\x16\x10\x91Pa\x19\xD8V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x06}a\x1A\xA66a\x06HV[a'\x1DV[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x1A\xC8\x81a\t\xA8V[a\x1A\xD9a\x07F`\x08\x80`fT\x16\x14\x90V[a\x1A\xEE`\x01\x80`\xA0\x1B\x03`\xCBT\x163\x14a!2V[`\xCAT\x90c\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x10\x15a\x1BvWa\x1B\x0C`\x01\x91a\x1FkV[P\x01c\xFF\xFF\xFF\xFF\x81Ta\x1B%`\xFF\x82`@\x1C\x16\x15a'dV[` \x1C\x16B\x10\x15a\x1BgW\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E_\x80\xA2\0[c\x0C6\xF6e`\xE2\x1B_R`\x04_\xFD[c\x94\xA8\xD3\x89`\xE0\x1B_R`\x04_\xFD[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06CW_\x91a\x1C\x0EW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1B\xFFWa\x06\x12\x90a'zV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\x1CAW[\x81a\x1C)` \x93\x83a\x1E\xC8V[\x81\x01\x03\x12a\x03\xC9WQa\x1C;\x81a\x03\xB8V[_a\x1B\xE6V[=\x91Pa\x1C\x1CV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9Wa\x1C\x7F6a\x06\xF8V[\x90a\x1C\x91a\x07F`\x01\x80`fT\x16\x14\x90V[a\x1C\xA0`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x1C\xB8Wa\x06\x12`\x01`\x97UV[\x80a\x1Dca\x1C\xC9`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x1C\xF0\x81a\x07\xC3\x86\x863\x87a\"\xBEV[Q\x90 \x90a\x1C\xFD\x83a(\xDCV[3_\x90\x81R`\xCF` R`@\x90 a\x1D\x1A\x90a\x08\x04\x90\x84\x90a\x07\xF7V[a\x1D#\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x813\x91\x80a\x08Z\x87\x82a#\x0BV[\x01a\x1C\xA7V[4a\x03\xC9Wa\x1Dw6a\x06\xF8V[\x90a\x1D\x89a\x07F`\x10\x80`fT\x16\x14\x90V[3_R`\xD1` Ra\x1D\xA1`\xFF`@_ T\x16a!2V[a\x1D\xB0`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x1D\xC8Wa\x06\x12`\x01`\x97UV[\x80a\x1Esa\x1D\xD9`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x1E\0\x81a\x07\xC3\x86\x863\x87a\"\xBEV[Q\x90 \x90a\x1E\r\x83a(\xDCV[3_\x90\x81R`\xD2` R`@\x90 a\x1E*\x90a\x08\x04\x90\x84\x90a\x07\xF7V[a\x1E3\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B3\x91\x80a\x08Z\x87\x82a#\x0BV[\x01a\x1D\xB7V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[a\x1EyV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[`@Q\x90a\t\xC1`\x80\x83a\x1E\xC8V[`@Q\x90a\x1F\x05\x82a\x1E\xADV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x10\x1DW_\x19\x01\x90V[_\x19\x81\x01\x91\x90\x82\x11a\x10\x1DWV[\x91\x90\x82\x03\x91\x82\x11a\x10\x1DWV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xCAT\x81\x10\x15a\x1F\x87W`\xCA_R` _ \x90`\x01\x1B\x01\x90_\x90V[a\x1FWV[\x80T\x82\x10\x15a\x1F\x87W_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90`@Qa\x1F\xB2\x81a\x1E\xADV[```\xFF`\x01\x83\x95\x80T\x85R\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x85\x01Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16`@\x85\x01R`@\x1C\x16\x15\x15\x91\x01RV[a\x1F\xEEa\x1E\xF8V[P`\xCAT\x80[a \x19WPa \x01a\x1E\xE9V[_\x81R_` \x82\x01R_`@\x82\x01R_``\x82\x01R\x90V[a (a\r\xC0a\r\xBB\x83a\x1F<V[\x90a 9a\x19\xCB``\x84\x01Q\x15\x15\x90V[\x80a WW[a SWa M\x91Pa\x1F0V[\x80a\x1F\xF4V[P\x90V[Pa ua l`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a ?V[\x90\x81` \x91\x03\x12a\x03\xC9WQa\x16f\x81a\x04\xABV[`@Q=_\x82>=\x90\xFD[\x15a \xA4WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a \xBAWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[5a\x16f\x81a\x03\xB8V[` \x815\x91a \xE1\x83a\x03\xB8V[\x015`@Q\x90` \x82\x01\x92_\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra!\x16`U\x82a\x1E\xC8V[Q\x90 \x90V[\x15a!#WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a!9WV[c\\B|\xD9`\xE0\x1B_R`\x04_\xFD[\x15a!OWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\xC9W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W\x81`\x06\x1B6\x03\x83\x13a\x03\xC9WV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\"\x04WPPP\x90V[\x90\x91\x92\x835a\"\x12\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x015`\x01`\x01``\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x03\xC9W`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a!\xF7V[\x90`\x80c\xFF\xFF\xFF\xFF\x81a\"pa\"b\x86\x80a!\xB6V[`\xA0\x87R`\xA0\x87\x01\x91a!\xEAV[\x94` \x81\x015a\"\x7F\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`@\x81\x81\x015\x90\x86\x01R\x82``\x82\x015a\"\xA6\x81a\t\xA8V[\x16``\x86\x01R\x015a\"\xB7\x81a\t\xA8V[\x16\x91\x01R\x90V[a\x16f\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\"LV[\x90`\x01\x82\x01\x80\x92\x11a\x10\x1DWV[\x90` \x82\x01\x80\x92\x11a\x10\x1DWV[\x91\x90\x82\x01\x80\x92\x11a\x10\x1DWV[\x90` a\x16f\x92\x81\x81R\x01\x90a\"LV[\x15a##WV[c\x06\x95|\x91`\xE1\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\x1DWV[`\xCAT`\x01`@\x1B\x81\x10\x15a\x1E\xA8W\x80`\x01a#m\x92\x01`\xCAU`\xCAa\x1F\x8CV[\x91\x90\x91a#\xEBW```\x01a\t\xC1\x93\x83Q\x81U\x01\x91c\xFF\xFF\xFF\xFF` \x82\x01Q\x16c\xFF\xFF\xFF\xFF\x19\x84T\x16\x17\x83Ua#\xCAc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16\x84\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x01Q\x81T`\xFF`@\x1B\x19\x16\x90\x15\x15`@\x1Bh\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x90`@Qa$\x0B\x81a\x1E\x8DV[`@c\xFF\xFF\xFF\xFF\x82\x94Ta\xFF\xFF\x81\x16\x84Ra\xFF\xFF\x81`\x10\x1C\x16` \x85\x01R` \x1C\x16\x91\x01RV[5a\x16f\x81a\t\xA8V[\x15a$CWV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\xC9W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W\x816\x03\x83\x13a\x03\xC9WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90a$\xE2a$\xD4\x84\x80a!\xB6V[`\xC0\x84R`\xC0\x84\x01\x91a!\xEAV[\x90` \x84\x015a$\xF1\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x81\x01\x91\x90\x91Ra%\x10`@\x86\x01\x86a!\xB6V[\x83\x85\x03`@\x85\x01R\x80\x85R\x93\x90\x91\x01\x92_[\x81\x81\x10a%\x84WPPPa%v\x84a%Pa%C``a\x16f\x97\x98\x01a\t\xB6V[c\xFF\xFF\xFF\xFF\x16``\x85\x01RV[a%la%_`\x80\x83\x01a\t\xB6V[c\xFF\xFF\xFF\xFF\x16`\x80\x85\x01RV[`\xA0\x81\x01\x90a$tV[\x91`\xA0\x81\x85\x03\x91\x01Ra$\xA5V[\x90\x91\x93`@\x80`\x01\x92\x875a%\x98\x81a\x03\xB8V[\x84\x80`\xA0\x1B\x03\x16\x81R` \x88\x015` \x82\x01R\x01\x95\x01\x91\x01\x91\x90\x91a%\"V[a\x16f\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a$\xC5V[`@\x90a\x16f\x93\x92\x81R\x81` \x82\x01R\x01\x90a$\xC5V[\x15a%\xFAWV[c\x89\x1Cc\xDF`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\x1DWV[c\xFF\xFF\xFF\xFF`\xCAT\x16[c\xFF\xFF\xFF\xFF\x81\x16a&EWcPEp\xE3`\xE0\x1B_R`\x04_\xFD[\x81a&Ra\r\xBB\x83a&\tV[PT\x14a&mWc\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\x1DW_\x19\x01a&+V[a\x16f\x91Pa&\tV[\x90\x81` \x91\x03\x12a\x03\xC9WQ\x90V[\x15a&\x8DWV[c\xFBIN\xA1`\xE0\x1B_R`\x04_\xFD[\x15a&\xA3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x12X\x92a\x0E\xE9a\x0C\xAF\x92a'\x18a\t\xC1\x98\x97\x95a(IV[a.8V[` \x815\x91a'+\x83a\x03\xB8V[\x015`@Q\x90` \x82\x01\x92`\x01`\xF8\x1B\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra!\x16`U\x82a\x1E\xC8V[\x15a'kWV[c\x1B\x14\x17K`\xE0\x1B_R`\x04_\xFD[a'\x8B`fT\x19\x82\x19\x81\x16\x14a \xB3V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a'\xD1WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x03\xC9WV[\x15a(\xB7WV[c\x10\xEBH?`\xE2\x1B_R`\x04_\xFD[\x15a(\xCDWV[c\x07\x0BZo`\xE2\x1B_R`\x04_\xFD[a):oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF`@a)&\x93a)\0\x81\x80a({V[\x95\x90``\x83\x015\x80\x97a)\x12\x82a\t\xA8V[`\x80\x85\x015\x92a)!\x84a\t\xA8V[a2\xD5V[\x015a)3\x81\x15\x15a(\xB0V[\x11\x15a(\xC6V[a)C\x81a\t\xA8V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16B\x01\x90\x81B\x11a\x10\x1DWc\xFF\xFF\xFF\xFF\x16\x11a)\x81WV[c~\xE2\xB4C`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\t\xC1\x91a)\xD4`\x84\x83a\x1E\xC8V[a5[V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03\xC9WV[\x91\x90\x81\x10\x15a\x1F\x87W`\x06\x1B\x01\x90V[\x15a*%WV[c\xAA8^\x81`\xE0\x1B_R`\x04_\xFD[\x90a*Da\r\xC0a\r\xBB\x84a$2V[\x90a*O\x82\x84a-\x0EV[a*[``\x84\x01a \xC9V[\x93a*\x86a*y\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a+\xE8W[\x90\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91a*\xAF3\x84\x14a!2V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91_[a*\xC9`\xA0\x83\x01\x83a)\xD9V[\x90P\x81\x10\x15a+\xDEW\x80a*\xEC`\x01\x92a*\xE6`\xE0\x86\x01\x86a({V[\x90a*\x0EV[\x86\x86\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x8Ca+Ja+-\x82`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a+6\x87a \xC9V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta+\x8Fa+\x86a+k` \x89\x015\x93a+e\x81\x86\x11a*\x1EV[\x84a\x1FJV[`\x01`\x01`\xA0\x1B\x03\x90\x94\x16_\x90\x81R`\xCD` R`@\x90 \x90V[a+6\x88a \xC9V[Ua+\xA3\x81\x8Aa+\x9E\x88a \xC9V[a6-V[a+\xAE\x8CQ\x95a \xC9V[`@\x80Q\x96\x87R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x87\x01R\x85\x01R`\xA0\x86\x90\x1B\x86\x90\x03\x88\x16\x93``\x90\xA4\x01a*\xBCV[PPPPPP\x90PV[P\x84a*\x96V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x81a,\x0FWPPa\xFF\xFF`\xCBT`\xE0\x1C\x16\x90V[a\xFF\xFF\x91B\x10a,!W` \x01Q\x16\x90V[Q\x16\x90V[`\xCBT\x90\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3`@\x80Qc\xFF\xFF\xFF\xFF\x85`\xA0\x1C\x16\x81Rc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R\xA1c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17`\xCBUV[\x15a,\x90WV[c\x147\xA2\xBB`\xE3\x1B_R`\x04_\xFD[\x15a,\xA6WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x816\x03\x83\x13a\x03\xC9WV[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x01\x90V[\x90\x82\x10\x15a\x1F\x87Wa\x07\"\x91`\x05\x1B\x81\x01\x90a,\xB5V[\x91\x90\x91a-)a-$a\x19\xCB``\x86\x01Q\x15\x15\x90V[a'dV[a-Ha-@a l`@\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a,\x89V[`\xA0\x81\x01\x90a-W\x82\x82a)\xD9V[\x90Pa-t`\xC0\x83\x01\x91a-k\x83\x85a)\xD9V[\x91\x90P\x14a,\x9FV[a-\xBBa-\x81\x82\x84a)\xD9V[\x96\x90Pa-\x96`\xE0\x85\x01\x97a-k\x89\x87a({V[Qa-\xA3` \x85\x01a$2V[a-\xB0`@\x86\x01\x86a,\xB5V[\x91``\x87\x01\x93a6\xB4V[`\x80\x82\x015\x92_[a-\xCD\x82\x85a)\xD9V[\x90P\x81\x10\x15a./W\x80a.)\x85a*\xE6\x8Aa.\"\x85a.\x19\x81a.\x13\x8C\x8Fa.\x08`\x01\x9D\x8Fa.\x02\x90a.\r\x94P\x8Da)\xD9V[\x90a,\xE7V[a$2V[\x98a)\xD9V[\x90a,\xF7V[\x94\x90\x93\x8Ca({V[\x92\x8Aa6\xFFV[\x01a-\xC3V[PPPPP\x90PV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x82\x90\x82\x16\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB_\x80\xA3`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`\xCBUV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06CW_\x91a/CWP\x90V[a\x16f\x91P` =` \x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[\x15a/cWV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a/yWV[c\x15\x03X\xA1`\xE2\x1B_R`\x04_\xFD[\x15a/\x8FWV[c\x08c\xA4S`\xE1\x1B_R`\x04_\xFD[\x15a/\xA5WV[c\x10\xFBG\xF1`\xE3\x1B_R`\x04_\xFD[\x90a0\x1A\x91a0(a/\xC6\x82\x80a({V[``\x84\x01\x95\x91a/\xED\x90a/\xD9\x88a$2V[`\x80\x87\x01\x93a/\xE7\x85a$2V[\x92a2\xD5V[a0\x14a0\x0E`@\x86\x01\x97a.\x08a0\x05\x8A\x89a({V[\x90P\x15\x15a/\\V[\x91a$2V[\x90a#2V[c\xFF\xFF\xFF\xFFB\x91\x16\x10a/rV[_\x92\x83\x91\x82[a08\x83\x83a({V[\x90P\x84\x10\x15a0\xBDWa0\xB4`\x01\x91a0\x95a0X\x87a*\xE6\x88\x88a({V[\x91a0{a0ta0h\x85a \xC9V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x15\x15a/\x88V[a0\x87a0h\x84a \xC9V[\x90\x85\x80`\xA0\x1B\x03\x16\x10a/\x9EV[a0\xAE` \x82\x015\x91a0\xA9\x83\x15\x15a(\xB0V[a \xC9V[\x97a\"\xFEV[\x93\x01\x92\x94a0.V[PPPP\x90a\x16foK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF\x82\x11\x15a(\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xCC` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x95\x85\x16\x95\x86\x17\x90\x91U\x90\x92\x16\x91\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x90\x80\xA4V[`\xCBT\x90\x7F\xE6\xCDN\xDF\xDC\xC1\xF6\xD10\xAB5\xF7=r7\x8F:d)D\xFBN\xE5\xBD\x84\xB7\x80z\x81\xEA\x1CN`@\x80Qa\xFF\xFF\x85`\xE0\x1C\x16\x81Ra\xFF\xFF\x84\x16` \x82\x01R\xA1a\xFF\xFF`\xE0\x1B\x19\x90\x91\x16`\xE0\x91\x90\x91\x1Ba\xFF\xFF`\xE0\x1B\x16\x17`\xCBUV[\x91\x90\x91\x80T\x92c\xFF\xFF\xFF\xFF\x84` \x1C\x16\x93\x84B\x11\x15a2\x16Wa\t\xC1\x94a1\xFDWP`\xCBT\x82Ta\xFF\xFF\x19\x16`\xE0\x91\x90\x91\x1Ca\xFF\xFF\x16\x17\x82U[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x16`\x10\x91\x90\x91\x1Bc\xFF\xFF\0\0\x16\x17` \x92\x90\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x91\x90\x91\x17\x90UV[\x82Ta\xFF\xFF\x19\x16`\x10\x91\x90\x91\x1Ca\xFF\xFF\x16\x17\x82Ua1\xCBV[c{\x1E%\xC5`\xE0\x1B_R`\x04_\xFD[\x15a2,WV[c\r\xD0\xB9\xF5`\xE2\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x16\x90\x81\x15a2SWc\xFF\xFF\xFF\xFF\x16\x06\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a2nWV[c\xEEfG\x05`\xE0\x1B_R`\x04_\xFD[\x15a2\x84WV[c<\x1A\x94\xF1`\xE2\x1B_R`\x04_\xFD[\x15a2\x9AWV[c\x04\x1A\xA7W`\xE1\x1B_R`\x04_\xFD[\x15a2\xB0WV[c.\xFD\x96Q`\xE1\x1B_R`\x04_\xFD[\x15a2\xC6WV[c\xDF\xAD\x9C\xA1`\xE0\x1B_R`\x04_\xFD[\x92\x91a3na3ha la3\xB5\x94\x95a2\xF0\x87\x15\x15a/\\V[a3(c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15a2%V[a3bc\xFF\xFF\xFF\xFFa3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x94a2;V[\x16\x15a2gV[\x84a2;V[\x15a2}V[c\xFF\xFF\xFF\xFFa3\x9F\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x1FJV[\x91\x16\x80\x91\x11\x15\x90\x81a4\xB0W[P\x92\x91\x92a2\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_\x90\x81[\x81\x83\x10a3\xF2WPPPPPV[a4\0a0\xA9\x84\x84\x87a*\x0EV[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x8AZ\xFA\x92\x83\x15a\x06CW`\x01\x93a4d\x92_\x91a4\x92W[P\x80\x15a4lW[a4N\x90a2\xA9V[\x83\x80`\xA0\x1B\x03\x16\x80\x92\x84\x80`\xA0\x1B\x03\x16\x10a2\xBFV[\x92\x01\x91a3\xE4V[P`\xA0\x84\x90\x1B\x84\x90\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a4EV[a4\xAA\x91P` =\x81\x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[_a4=V[\x90Pc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x11\x15_a3\xACV[`\x01`\x01`@\x1B\x03\x81\x11a\x1E\xA8W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x15a5\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90`@Q\x90a5s`@\x83a\x1E\xC8V[` \x82R\x7FSafeERC20: low-level call failed` \x83\x01R\x82;\x15a5\xE8W_\x81a5\xC3\x94\x82` \x81\x95Q\x93\x01\x91Z\xF1a5\xBDa7\xAFV[\x90a7\xDEV[\x80Q\x80a5\xCEWPPV[\x81` \x80a5\xE3\x93a\t\xC1\x95\x01\x01\x91\x01a }V[a4\xFCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x80\x83\x01\x93\x90\x93R\x91\x81Ra\t\xC1\x91a)\xD4`d\x83a\x1E\xC8V[\x92\x91\x92a6t\x82a4\xE1V[\x91a6\x82`@Q\x93\x84a\x1E\xC8V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xC9W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x15a6\xA5WV[ci\xCA\x16\xC9`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a6\xF1Wa6\xE7a6\xEC\x94a6\xDFa\t\xC1\x97a \xD3V[\x936\x91a6hV[a79V[a6\x9EV[b\xC6\xC3\x9D`\xE7\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a7*Wa6\xE7a6\xEC\x94a6\xDFa\t\xC1\x97a'\x1DV[c\x05O\xF4\xDF`\xE5\x1B_R`\x04_\xFD[\x93\x90\x92\x91`\x1F\x85Q\x16a7\xA0W\x91\x90` \x92[\x85Q\x84\x11a7\x97W`\x01\x83\x16a7}W_R\x82\x85\x01Q` Ra7v`@_ \x92`\x01\x1C\x93a\"\xF0V[\x92\x91a7LV[\x83\x86\x01Q_R` Ra7v`@_ \x92`\x01\x1C\x93a\"\xF0V[\x92P\x93PP\x14\x90V[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[=\x15a7\xD9W=\x90a7\xC0\x82a4\xE1V[\x91a7\xCE`@Q\x93\x84a\x1E\xC8V[\x82R=_` \x84\x01>V[``\x90V[\x90\x91\x90\x15a7\xEAWP\x90V[\x81Q\x15a7\xFAWP\x80Q\x90` \x01\xFD[`D` \x91`@Q\x92\x83\x91bF\x1B\xCD`\xE5\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD\xFE\xA2dipfsX\"\x12 \x88\\j\x97\x07=\xF6d8\x92\xA2GY\x9C\xB0\xFD\x91$l\xA8\x91\xD5\xB0\xCD\x18m\xEF\r\x9C\r{\xB0dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806218572c146103b357806304a0c502146103ae5780630e9a53cf146103a95780630eb38345146103a4578063131433b41461039f578063136439dd1461039a578063149bc872146103955780632b9f64a41461039057806336af41fa1461038b57806337838ed01461038657806339b70e38146103815780633a8c07861461037c5780633ccc861d146103775780633efe1db6146103725780634596021c1461036d5780634657e26a146103685780634b943960146103635780634d18cc351461035e57806358baaa3e14610359578063595c6a67146103545780635ac86ab71461034f5780635c975abb1461034a5780635e9d83481461034557806363f6a798146103405780636d21117e1461033b578063715018a6146103365780637b8f8b0514610331578063863cb9a91461032c578063865c695314610327578063886f1195146103225780638da5cb5b1461031d5780639104c319146103185780639be3d4e4146103135780639cb9a5fa1461030e5780639d45c28114610309578063a0169ddd14610304578063a50a1d9c146102ff578063aebd8bae146102fa578063b3dbb0e0146102f5578063bb7e451f146102f0578063bf21a8aa146102eb578063c46db606146102e6578063ca8aa7c7146102e1578063dcbb03b3146102dc578063de02e503146102d7578063e063f81f146102d2578063e810ce21146102cd578063ea4d3c9b146102c8578063ed71e6a2146102c3578063f22cef85146102be578063f2fde38b146102b9578063f6efbb59146102b4578063f8cd8448146102af578063f96abf2e146102aa578063fabc1cbc146102a5578063fbf1e2c1146102a0578063fce36c7d1461029b5763ff9f6cce14610296575f80fd5b611d69565b611c71565b611c49565b611b85565b611aab565b611a87565b61196f565b6118de565b611786565b611738565b6116f4565b6116c8565b611669565b611616565b6114c4565b611480565b611432565b6113f2565b6113b7565b6112ab565b61125d565b61122d565b6111d4565b611194565b611022565b610fe7565b610fb9565b610f91565b610f4d565b610eee565b610ec1565b610ea4565b610e49565b610dfb565b610dd7565b610d77565b610d5a565b610d27565b610cb4565b610c87565b610c61565b610be1565b610b9d565b610b04565b6109c3565b610937565b610902565b6108be565b61087e565b610726565b610685565b610659565b610588565b610548565b6104b5565b61044d565b61040d565b6103cd565b6001600160a01b038116036103c957565b5f80fd5b346103c95760203660031901126103c9576004356103ea816103b8565b60018060a01b03165f5260d1602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c9575f3660031901126103c9576104a7610468611fe6565b6040519182918291909160608060808301948051845263ffffffff602082015116602085015263ffffffff604082015116604085015201511515910152565b0390f35b801515036103c957565b346103c95760403660031901126103c9576004356104d2816103b8565b602435906104df826104ab565b6104e76127bd565b60018060a01b0316805f5260d160205260ff60405f205416151582151590827f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c5f80a45f5260d160205260405f209060ff8019835416911515161790555f80f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760203660031901126103c95760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa91821561064357610612926105fe915f91610614575b5061209d565b61060d606654828116146120b3565b612849565b005b610636915060203d60201161063c575b61062e8183611ec8565b81019061207d565b5f6105f8565b503d610624565b612092565b60409060031901126103c957600490565b346103c95760403660031901126103c957602061067d61067836610648565b6120d3565b604051908152f35b346103c95760203660031901126103c9576004356106a2816103b8565b60018060a01b03165f5260cc602052602060018060a01b0360405f205416604051908152f35b9181601f840112156103c9578235916001600160401b0383116103c9576020808501948460051b0101116103c957565b60206003198201126103c957600435906001600160401b0382116103c957610722916004016106c8565b9091565b346103c957610734366106f8565b9061074c610746600280606654161490565b1561211c565b335f5260d160205261076460ff60405f205416612132565b61077360026097541415612148565b60026097555f5b82811061078b576106126001609755565b8061087861079c6001938686612194565b335f90815260ce602052604090205460405160208101906107d1816107c3868633876122be565b03601f198101835282611ec8565b519020906107de836128dc565b335f90815260d060205260409020610811906108049084905b905f5260205260405f2090565b805460ff19166001179055565b61081a816122e2565b335f90815260ce60205260409020556040517f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf27048233918061085a878261230b565b0390a4604061086b602083016120c9565b9101359030903390612990565b0161077a565b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c9575f3660031901126103c957602063ffffffff60cb5460a01c16604051908152f35b90816101009103126103c95790565b346103c95760403660031901126103c9576004356001600160401b0381116103c95761096a6109a1913690600401610928565b60243590610977826103b8565b610988610746600480606654161490565b61099760026097541415612148565b6002609755612a34565b6001609755005b63ffffffff8116036103c957565b35906109c1826109a8565b565b346103c95760403660031901126103c9576024356004356109e3826109a8565b6109f4610746600880606654161490565b60cb5491610a0c336001600160a01b03851614612132565b63ffffffff81169263ffffffff8160c01c16841115610af55763ffffffff7fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd0891610a5742871061231c565b610adb610a84610a7c610a6f60ca5463ffffffff1690565b9360a01c63ffffffff1690565b844216612332565b94610ab6610a90611ee9565b88815263ffffffff8316602082015263ffffffff881660408201525f606082015261234c565b60cb805463ffffffff60c01b191660c09290921b63ffffffff60c01b16919091179055565b60405163ffffffff9094168452169180602081015b0390a4005b631ca7e50b60e21b5f5260045ffd5b346103c95760403660031901126103c9576004356001600160401b0381116103c957610b349036906004016106c8565b60243591610b41836103b8565b610b52610746600480606654161490565b610b6160026097541415612148565b60026097553681900360fe1901925f5b838110156109a1578060051b83013590858212156103c957610b97836001938601612a34565b01610b71565b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760203660031901126103c957600435610bfe816103b8565b60018060a01b03165f5260d56020526104a7610c4c60405f2063ffffffff60405191610c2983611e8d565b5461ffff8116835261ffff8160101c16602084015260201c166040820152612bef565b60405161ffff90911681529081906020820190565b346103c9575f3660031901126103c957602063ffffffff60cb5460c01c16604051908152f35b346103c95760203660031901126103c957610612600435610ca7816109a8565b610caf6127bd565b612c26565b346103c9575f3660031901126103c95760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561064357610d1f915f91610614575061209d565b610612612815565b346103c95760203660031901126103c95760043560ff81168091036103c95760016020911b806066541614604051908152f35b346103c9575f3660031901126103c9576020606654604051908152f35b346103c95760203660031901126103c9576004356001600160401b0381116103c957610daa610dcc913690600401610928565b610dc6610dc08235610dbb816109a8565b611f6b565b50611fa5565b90612d0e565b602060405160018152f35b346103c9575f3660031901126103c957602061ffff60cb5460e01c16604051908152f35b346103c95760403660031901126103c957600435610e18816103b8565b6024359060018060a01b03165f5260cf60205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c957610e616127bd565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346103c9575f3660031901126103c957602060ca54604051908152f35b346103c95760203660031901126103c957610612600435610ee1816103b8565b610ee96127bd565b612e80565b346103c95760403660031901126103c9576020610f44600435610f10816103b8565b60243590610f1d826103b8565b60018060a01b03165f5260cd835260405f209060018060a01b03165f5260205260405f2090565b54604051908152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c9575f3660031901126103c9576033546040516001600160a01b039091168152602090f35b346103c9575f3660031901126103c957602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b346103c9575f3660031901126103c957610fff611ef8565b5060ca545f19810190811161101d57610468610dc06104a792611f6b565b611f1c565b346103c95760403660031901126103c95760043561103f816103b8565b6024356001600160401b0381116103c95761105e9036906004016106c8565b9190611071610746602080606654161490565b61108261107d83612ec7565b61243c565b61109160026097541415612148565b60026097556001600160a01b038216915f5b8481106110b4576106126001609755565b8061118e6110c56001938887612452565b6001600160a01b0385165f90815260ce60205260409020549060405160208101906110f6816107c385878c876125b8565b5190208861110383612fb4565b93611125610804846107f78c60018060a01b03165f5260d360205260405f2090565b61112e816122e2565b6001600160a01b038a165f90815260ce60205260409020557ffc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e060405180611177873395836125dc565b0390a4309061118960203392016120c9565b612990565b016110a3565b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760203660031901126103c9576106126004356111f4816103b8565b336130de565b6024359061ffff821682036103c957565b6044359061ffff821682036103c957565b6084359061ffff821682036103c957565b346103c95760203660031901126103c95760043561ffff811681036103c957610612906112586127bd565b613136565b346103c95760403660031901126103c95760043561127a816103b8565b6024359060018060a01b03165f5260d260205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c95760403660031901126103c9576004356112c8816103b8565b6112d06111fa565b6112e1610746608080606654161490565b6112ed61107d83612ec7565b6112ff61271061ffff831611156125f3565b7fd1e028bd664486a46ad26040e999cd2d22e1e9a094ee6afe19fcf64678f16f7461133b63ffffffff60cb5460a01c1663ffffffff4216612332565b9160018060a01b03841693845f5260d560205261138f848361138a61136f60405f2063ffffffff60405191610c2983611e8d565b6001600160a01b039095165f90815260d56020526040902090565b613191565b6040805163ffffffff95909516855261ffff91821660208601529116908301523391606090a3005b346103c95760203660031901126103c9576004356113d4816103b8565b60018060a01b03165f5260ce602052602060405f2054604051908152f35b346103c9575f3660031901126103c957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103c95760403660031901126103c95760043561144f816103b8565b6024359060018060a01b03165f5260d060205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760603660031901126103c9576004356114e1816103b8565b6024356114ed816103b8565b6114f561120b565b611506610746604080606654161490565b61151261107d84612ec7565b61152461271061ffff831611156125f3565b7f48e198b6ae357e529204ee53a8e514c470ff77d9cc8e4f7207f8b5d490ae693461156063ffffffff60cb5460a01c1663ffffffff4216612332565b9160018060a01b03851693845f5260d46020526115dc848361138a846115c76115ac6115a76115a28460405f209060018060a01b03165f5260205260405f2090565b6123fe565b612bef565b6001600160a01b03909c165f90815260d46020526040902090565b9060018060a01b03165f5260205260405f2090565b6040805163ffffffff95909516855261ffff968716602086015291909516908301526001600160a01b039093169233918060608101610af0565b346103c95760203660031901126103c9576104a7610468610dc060043561163b611ef8565b50611f6b565b60409060031901126103c957600435611659816103b8565b90602435611666816103b8565b90565b346103c9576116b86116a661167d36611641565b9060018060a01b03165f5260d460205260405f209060018060a01b03165f5260205260405f2090565b63ffffffff60405191610c2983611e8d565b60405161ffff9091168152602090f35b346103c95760203660031901126103c95760206116e6600435612621565b63ffffffff60405191168152f35b346103c9575f3660031901126103c9576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346103c95760403660031901126103c957600435611755816103b8565b6024359060018060a01b03165f5260d360205260405f20905f52602052602060ff60405f2054166040519015158152f35b346103c95761179436611641565b6117a061107d83612ec7565b6040516336b87bd760e11b81526001600160a01b0383166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610643575f916118bf575b50801561181a575b9161181561061293612686565b6130de565b5060405163ba1a84e560e01b81526001600160a01b038316600482015291602083806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9283156106435761061293611815915f91611890575b50151591935050611808565b6118b2915060203d6020116118b8575b6118aa8183611ec8565b810190612677565b5f611884565b503d6118a0565b6118d8915060203d60201161063c5761062e8183611ec8565b5f611800565b346103c95760203660031901126103c9576004356118fb816103b8565b6119036127bd565b6001600160a01b0381161561191b5761061290612e38565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346103c95760a03660031901126103c95760043561198c816103b8565b6119ff60243560443561199e816103b8565b606435906119ab826109a8565b6119b361121c565b925f54956119e56119cf6119cb8960ff9060081c1690565b1590565b80988199611a79575b8115611a59575b5061269c565b866119f6600160ff195f5416175f55565b611a42576126ff565b611a0557005b611a1361ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b611a5461010061ff00195f5416175f55565b6126ff565b303b15915081611a6b575b505f6119df565b60ff1660011490505f611a64565b600160ff82161091506119d8565b346103c95760403660031901126103c957602061067d611aa636610648565b61271d565b346103c95760203660031901126103c957600435611ac8816109a8565b611ad9610746600880606654161490565b611aee60018060a01b0360cb54163314612132565b60ca549063ffffffff811691821015611b7657611b0c600191611f6b565b500163ffffffff8154611b2560ff8260401c1615612764565b60201c16421015611b6757805460ff60401b1916600160401b1790557fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e5f80a2005b630c36f66560e21b5f5260045ffd5b6394a8d38960e01b5f5260045ffd5b346103c95760203660031901126103c95760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610643575f91611c0e575b506001600160a01b03163303611bff576106129061277a565b63794821ff60e01b5f5260045ffd5b90506020813d602011611c41575b81611c2960209383611ec8565b810103126103c95751611c3b816103b8565b5f611be6565b3d9150611c1c565b346103c9575f3660031901126103c95760cb546040516001600160a01b039091168152602090f35b346103c957611c7f366106f8565b90611c91610746600180606654161490565b611ca060026097541415612148565b60026097555f5b828110611cb8576106126001609755565b80611d63611cc96001938686612194565b335f90815260ce60205260409020546040516020810190611cf0816107c3868633876122be565b51902090611cfd836128dc565b335f90815260cf60205260409020611d1a906108049084906107f7565b611d23816122e2565b335f90815260ce60205260409020556040517f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e628133918061085a878261230b565b01611ca7565b346103c957611d77366106f8565b90611d89610746601080606654161490565b335f5260d1602052611da160ff60405f205416612132565b611db060026097541415612148565b60026097555f5b828110611dc8576106126001609755565b80611e73611dd96001938686612194565b335f90815260ce60205260409020546040516020810190611e00816107c3868633876122be565b51902090611e0d836128dc565b335f90815260d260205260409020611e2a906108049084906107f7565b611e33816122e2565b335f90815260ce60205260409020556040517f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b33918061085a878261230b565b01611db7565b634e487b7160e01b5f52604160045260245ffd5b606081019081106001600160401b03821117611ea857604052565b611e79565b608081019081106001600160401b03821117611ea857604052565b90601f801991011681019081106001600160401b03821117611ea857604052565b604051906109c1608083611ec8565b60405190611f0582611ead565b5f6060838281528260208201528260408201520152565b634e487b7160e01b5f52601160045260245ffd5b801561101d575f190190565b5f1981019190821161101d57565b9190820391821161101d57565b634e487b7160e01b5f52603260045260245ffd5b60ca54811015611f875760ca5f5260205f209060011b01905f90565b611f57565b8054821015611f87575f5260205f209060011b01905f90565b90604051611fb281611ead565b606060ff6001839580548552015463ffffffff8116602085015263ffffffff8160201c16604085015260401c161515910152565b611fee611ef8565b5060ca54805b6120195750612001611ee9565b5f81525f60208201525f60408201525f606082015290565b612028610dc0610dbb83611f3c565b906120396119cb6060840151151590565b80612057575b6120535761204d9150611f30565b80611ff4565b5090565b5061207561206c604084015163ffffffff1690565b63ffffffff1690565b42101561203f565b908160209103126103c95751611666816104ab565b6040513d5f823e3d90fd5b156120a457565b631d77d47760e21b5f5260045ffd5b156120ba57565b63c61dca5d60e01b5f5260045ffd5b35611666816103b8565b60208135916120e1836103b8565b01356040519060208201925f84526001600160601b03199060601b166021830152603582015260358152612116605582611ec8565b51902090565b1561212357565b63840a48d560e01b5f5260045ffd5b1561213957565b635c427cd960e01b5f5260045ffd5b1561214f57565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b9190811015611f875760051b81013590609e19813603018212156103c9570190565b9035601e19823603018112156103c95701602081359101916001600160401b0382116103c9578160061b360383136103c957565b916020908281520191905f905b8082106122045750505090565b9091928335612212816103b8565b6001600160a01b0316815260208401356001600160601b03811691908290036103c9576040816001936020839401520194019201906121f7565b90608063ffffffff8161227061226286806121b6565b60a0875260a08701916121ea565b94602081013561227f816103b8565b6001600160a01b03166020860152604081810135908601528260608201356122a6816109a8565b16606086015201356122b7816109a8565b1691015290565b611666939260609260018060a01b031682526020820152816040820152019061224c565b906001820180921161101d57565b906020820180921161101d57565b9190820180921161101d57565b90602061166692818152019061224c565b1561232357565b6306957c9160e11b5f5260045ffd5b9063ffffffff8091169116019063ffffffff821161101d57565b60ca54600160401b811015611ea85780600161236d920160ca5560ca611f8c565b9190916123eb57606060016109c19383518155019163ffffffff60208201511663ffffffff198454161783556123ca63ffffffff604083015116849067ffffffff0000000082549160201b169067ffffffff000000001916179055565b0151815460ff60401b191690151560401b68ff000000000000000016179055565b634e487b7160e01b5f525f60045260245ffd5b9060405161240b81611e8d565b604063ffffffff82945461ffff8116845261ffff8160101c16602085015260201c16910152565b35611666816109a8565b1561244357565b63932d94f760e01b5f5260045ffd5b9190811015611f875760051b8101359060be19813603018212156103c9570190565b9035601e19823603018112156103c95701602081359101916001600160401b0382116103c95781360383136103c957565b908060209392818452848401375f828201840152601f01601f1916010190565b91906124e26124d484806121b6565b60c0845260c08401916121ea565b9060208401356124f1816103b8565b6001600160a01b031660208281019190915261251060408601866121b6565b838503604085015280855293909101925f5b818110612584575050506125768461255061254360606116669798016109b6565b63ffffffff166060850152565b61256c61255f608083016109b6565b63ffffffff166080850152565b60a0810190612474565b9160a08185039101526124a5565b9091936040806001928735612598816103b8565b848060a01b03168152602088013560208201520195019101919091612522565b611666939260609260018060a01b03168252602082015281604082015201906124c5565b6040906116669392815281602082015201906124c5565b156125fa57565b63891c63df60e01b5f5260045ffd5b63ffffffff5f199116019063ffffffff821161101d57565b63ffffffff60ca54165b63ffffffff81166126455763504570e360e01b5f5260045ffd5b81612652610dbb83612609565b50541461266d5763ffffffff16801561101d575f190161262b565b6116669150612609565b908160209103126103c9575190565b1561268d57565b63fb494ea160e01b5f5260045ffd5b156126a357565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b61125892610ee9610caf926127186109c1989795612849565b612e38565b602081359161272b836103b8565b0135604051906020820192600160f81b84526001600160601b03199060601b166021830152603582015260358152612116605582611ec8565b1561276b57565b631b14174b60e01b5f5260045ffd5b61278b6066541982198116146120b3565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b6033546001600160a01b031633036127d157565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b903590601e19813603018212156103c957018035906001600160401b0382116103c957602001918160061b360383136103c957565b156128b757565b6310eb483f60e21b5f5260045ffd5b156128cd57565b63070b5a6f60e21b5f5260045ffd5b61293a6f4b3b4ca85a86c47a098a223fffffffff604061292693612900818061287b565b959060608301358097612912826109a8565b608085013592612921846109a8565b6132d5565b01356129338115156128b0565b11156128c6565b612943816109a8565b63ffffffff7f00000000000000000000000000000000000000000000000000000000000000001642019081421161101d5763ffffffff161161298157565b637ee2b44360e01b5f5260045ffd5b6040516323b872dd60e01b60208201526001600160a01b0392831660248201529290911660448301526064808301939093529181526109c1916129d4608483611ec8565b61355b565b903590601e19813603018212156103c957018035906001600160401b0382116103c957602001918160051b360383136103c957565b9190811015611f875760061b0190565b15612a2557565b63aa385e8160e01b5f5260045ffd5b90612a44610dc0610dbb84612432565b90612a4f8284612d0e565b612a5b606084016120c9565b93612a86612a798660018060a01b03165f5260cc60205260405f2090565b546001600160a01b031690565b6001600160a01b03811615612be8575b90936001600160a01b0390911691612aaf338414612132565b6001600160a01b038616915f5b612ac960a08301836129d9565b9050811015612bde5780612aec600192612ae660e086018661287b565b90612a0e565b86867f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce318c612b4a612b2d8260018060a01b03165f5260cd60205260405f2090565b612b36876120c9565b60018060a01b03165f5260205260405f2090565b54612b8f612b86612b6b602089013593612b65818611612a1e565b84611f4a565b6001600160a01b039094165f90815260cd6020526040902090565b612b36886120c9565b55612ba3818a612b9e886120c9565b61362d565b612bae8c51956120c9565b604080519687526001600160a01b0391909116602087015285015260a086901b869003881693606090a401612abc565b5050505050509050565b5084612a96565b604081015163ffffffff169081612c0f57505061ffff60cb5460e01c1690565b61ffff914210612c2157602001511690565b511690565b60cb54907faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b36040805163ffffffff8560a01c16815263ffffffff84166020820152a163ffffffff60a01b1990911660a09190911b63ffffffff60a01b161760cb55565b15612c9057565b631437a2bb60e31b5f5260045ffd5b15612ca657565b6343714afd60e01b5f5260045ffd5b903590601e19813603018212156103c957018035906001600160401b0382116103c9576020019181360383136103c957565b9190811015611f875760051b0190565b90821015611f87576107229160051b810190612cb5565b919091612d29612d246119cb6060860151151590565b612764565b612d48612d4061206c604086015163ffffffff1690565b421015612c89565b60a0810190612d5782826129d9565b9050612d7460c0830191612d6b83856129d9565b91905014612c9f565b612dbb612d8182846129d9565b969050612d9660e0850197612d6b898761287b565b51612da360208501612432565b612db06040860186612cb5565b9160608701936136b4565b6080820135925f5b612dcd82856129d9565b9050811015612e2f5780612e2985612ae68a612e2285612e1981612e138c8f612e0860019d8f612e0290612e0d94508d6129d9565b90612ce7565b612432565b986129d9565b90612cf7565b9490938c61287b565b928a6136ff565b01612dc3565b50505050509050565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b60cb546001600160a01b0391821691829082167f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb5f80a36001600160a01b0319161760cb55565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af1908115610643575f91612f43575090565b611666915060203d60201161063c5761062e8183611ec8565b15612f6357565b63796cc52560e01b5f5260045ffd5b15612f7957565b63150358a160e21b5f5260045ffd5b15612f8f57565b630863a45360e11b5f5260045ffd5b15612fa557565b6310fb47f160e31b5f5260045ffd5b9061301a91613028612fc6828061287b565b606084019591612fed90612fd988612432565b6080870193612fe785612432565b926132d5565b61301461300e6040860197612e086130058a8961287b565b90501515612f5c565b91612432565b90612332565b63ffffffff42911610612f72565b5f928391825b613038838361287b565b90508410156130bd576130b460019161309561305887612ae6888861287b565b9161307b613074613068856120c9565b6001600160a01b031690565b1515612f88565b613087613068846120c9565b90858060a01b031610612f9e565b6130ae6020820135916130a98315156128b0565b6120c9565b976122fe565b9301929461302e565b50505050906116666f4b3b4ca85a86c47a098a223fffffffff8211156128c6565b6001600160a01b039081165f81815260cc6020526040812080546001600160a01b03198116958516958617909155909216917fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca3129080a4565b60cb54907fe6cd4edfdcc1f6d130ab35f73d72378f3a642944fb4ee5bd84b7807a81ea1c4e6040805161ffff8560e01c16815261ffff84166020820152a161ffff60e01b1990911660e09190911b61ffff60e01b161760cb55565b91909180549263ffffffff8460201c169384421115613216576109c1946131fd575060cb54825461ffff191660e09190911c61ffff161782555b815467ffffffffffff0000191660109190911b63ffff0000161760209290921b67ffffffff0000000016919091179055565b825461ffff191660109190911c61ffff161782556131cb565b637b1e25c560e01b5f5260045ffd5b1561322c57565b630dd0b9f560e21b5f5260045ffd5b9063ffffffff169081156132535763ffffffff160690565b634e487b7160e01b5f52601260045260245ffd5b1561326e57565b63ee66470560e01b5f5260045ffd5b1561328457565b633c1a94f160e21b5f5260045ffd5b1561329a57565b63041aa75760e11b5f5260045ffd5b156132b057565b632efd965160e11b5f5260045ffd5b156132c657565b63dfad9ca160e01b5f5260045ffd5b929161336e61336861206c6133b594956132f0871515612f5c565b61332863ffffffff7f00000000000000000000000000000000000000000000000000000000000000001663ffffffff83161115613225565b61336263ffffffff61335b7f0000000000000000000000000000000000000000000000000000000000000000809461323b565b1615613267565b8461323b565b1561327d565b63ffffffff61339f817f00000000000000000000000000000000000000000000000000000000000000001642611f4a565b91168091111590816134b0575b50929192613293565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f90815b8183106133f2575050505050565b6134006130a9848487612a0e565b60405163198f077960e21b81526001600160a01b03821660048201529091906020816024818a5afa92831561064357600193613464925f91613492575b50801561346c575b61344e906132a9565b838060a01b03168092848060a01b0316106132bf565b9201916133e4565b5060a084901b849003811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac014613445565b6134aa915060203d811161063c5761062e8183611ec8565b5f61343d565b905063ffffffff7f00000000000000000000000000000000000000000000000000000000000000001611155f6133ac565b6001600160401b038111611ea857601f01601f191660200190565b1561350357565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b60018060a01b03169060405190613573604083611ec8565b602082527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020830152823b156135e8575f816135c3948260208195519301915af16135bd6137af565b906137de565b8051806135ce575050565b816020806135e3936109c1950101910161207d565b6134fc565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b60405163a9059cbb60e01b60208201526001600160a01b0390921660248301526044808301939093529181526109c1916129d4606483611ec8565b929192613674826134e1565b916136826040519384611ec8565b8294818452818301116103c9578281602093845f960137010152565b156136a557565b6369ca16c960e01b5f5260045ffd5b91929063ffffffff169160018260051c1b8310156136f1576136e76136ec946136df6109c1976120d3565b933691613668565b613739565b61369e565b62c6c39d60e71b5f5260045ffd5b91929063ffffffff169160018260051c1b83101561372a576136e76136ec946136df6109c19761271d565b63054ff4df60e51b5f5260045ffd5b93909291601f8551166137a05791906020925b85518411613797576001831661377d575f528285015160205261377660405f209260011c936122f0565b929161374c565b838601515f5260205261377660405f209260011c936122f0565b92509350501490565b6313717da960e21b5f5260045ffd5b3d156137d9573d906137c0826134e1565b916137ce6040519384611ec8565b82523d5f602084013e565b606090565b909190156137ea575090565b8151156137fa5750805190602001fd5b604460209160405192839162461bcd60e51b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fdfea2646970667358221220885c6a97073df6643892a247599cb0fd91246ca891d5b0cd186def0d9c0d7bb064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\x18W,\x14a\x03\xB3W\x80c\x04\xA0\xC5\x02\x14a\x03\xAEW\x80c\x0E\x9AS\xCF\x14a\x03\xA9W\x80c\x0E\xB3\x83E\x14a\x03\xA4W\x80c\x13\x143\xB4\x14a\x03\x9FW\x80c\x13d9\xDD\x14a\x03\x9AW\x80c\x14\x9B\xC8r\x14a\x03\x95W\x80c+\x9Fd\xA4\x14a\x03\x90W\x80c6\xAFA\xFA\x14a\x03\x8BW\x80c7\x83\x8E\xD0\x14a\x03\x86W\x80c9\xB7\x0E8\x14a\x03\x81W\x80c:\x8C\x07\x86\x14a\x03|W\x80c<\xCC\x86\x1D\x14a\x03wW\x80c>\xFE\x1D\xB6\x14a\x03rW\x80cE\x96\x02\x1C\x14a\x03mW\x80cFW\xE2j\x14a\x03hW\x80cK\x949`\x14a\x03cW\x80cM\x18\xCC5\x14a\x03^W\x80cX\xBA\xAA>\x14a\x03YW\x80cY\\jg\x14a\x03TW\x80cZ\xC8j\xB7\x14a\x03OW\x80c\\\x97Z\xBB\x14a\x03JW\x80c^\x9D\x83H\x14a\x03EW\x80cc\xF6\xA7\x98\x14a\x03@W\x80cm!\x11~\x14a\x03;W\x80cqP\x18\xA6\x14a\x036W\x80c{\x8F\x8B\x05\x14a\x031W\x80c\x86<\xB9\xA9\x14a\x03,W\x80c\x86\\iS\x14a\x03'W\x80c\x88o\x11\x95\x14a\x03\"W\x80c\x8D\xA5\xCB[\x14a\x03\x1DW\x80c\x91\x04\xC3\x19\x14a\x03\x18W\x80c\x9B\xE3\xD4\xE4\x14a\x03\x13W\x80c\x9C\xB9\xA5\xFA\x14a\x03\x0EW\x80c\x9DE\xC2\x81\x14a\x03\tW\x80c\xA0\x16\x9D\xDD\x14a\x03\x04W\x80c\xA5\n\x1D\x9C\x14a\x02\xFFW\x80c\xAE\xBD\x8B\xAE\x14a\x02\xFAW\x80c\xB3\xDB\xB0\xE0\x14a\x02\xF5W\x80c\xBB~E\x1F\x14a\x02\xF0W\x80c\xBF!\xA8\xAA\x14a\x02\xEBW\x80c\xC4m\xB6\x06\x14a\x02\xE6W\x80c\xCA\x8A\xA7\xC7\x14a\x02\xE1W\x80c\xDC\xBB\x03\xB3\x14a\x02\xDCW\x80c\xDE\x02\xE5\x03\x14a\x02\xD7W\x80c\xE0c\xF8\x1F\x14a\x02\xD2W\x80c\xE8\x10\xCE!\x14a\x02\xCDW\x80c\xEAM<\x9B\x14a\x02\xC8W\x80c\xEDq\xE6\xA2\x14a\x02\xC3W\x80c\xF2,\xEF\x85\x14a\x02\xBEW\x80c\xF2\xFD\xE3\x8B\x14a\x02\xB9W\x80c\xF6\xEF\xBBY\x14a\x02\xB4W\x80c\xF8\xCD\x84H\x14a\x02\xAFW\x80c\xF9j\xBF.\x14a\x02\xAAW\x80c\xFA\xBC\x1C\xBC\x14a\x02\xA5W\x80c\xFB\xF1\xE2\xC1\x14a\x02\xA0W\x80c\xFC\xE3l}\x14a\x02\x9BWc\xFF\x9Fl\xCE\x14a\x02\x96W_\x80\xFD[a\x1DiV[a\x1CqV[a\x1CIV[a\x1B\x85V[a\x1A\xABV[a\x1A\x87V[a\x19oV[a\x18\xDEV[a\x17\x86V[a\x178V[a\x16\xF4V[a\x16\xC8V[a\x16iV[a\x16\x16V[a\x14\xC4V[a\x14\x80V[a\x142V[a\x13\xF2V[a\x13\xB7V[a\x12\xABV[a\x12]V[a\x12-V[a\x11\xD4V[a\x11\x94V[a\x10\"V[a\x0F\xE7V[a\x0F\xB9V[a\x0F\x91V[a\x0FMV[a\x0E\xEEV[a\x0E\xC1V[a\x0E\xA4V[a\x0EIV[a\r\xFBV[a\r\xD7V[a\rwV[a\rZV[a\r'V[a\x0C\xB4V[a\x0C\x87V[a\x0CaV[a\x0B\xE1V[a\x0B\x9DV[a\x0B\x04V[a\t\xC3V[a\t7V[a\t\x02V[a\x08\xBEV[a\x08~V[a\x07&V[a\x06\x85V[a\x06YV[a\x05\x88V[a\x05HV[a\x04\xB5V[a\x04MV[a\x04\rV[a\x03\xCDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xC9WV[_\x80\xFD[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x03\xEA\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD1` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x04\xA7a\x04ha\x1F\xE6V[`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x85\x01R\x01Q\x15\x15\x91\x01RV[\x03\x90\xF3[\x80\x15\x15\x03a\x03\xC9WV[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x04\xD2\x81a\x03\xB8V[`$5\x90a\x04\xDF\x82a\x04\xABV[a\x04\xE7a'\xBDV[`\x01\x80`\xA0\x1B\x03\x16\x80_R`\xD1` R`\xFF`@_ T\x16\x15\x15\x82\x15\x15\x90\x82\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C_\x80\xA4_R`\xD1` R`@_ \x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90U_\x80\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x06CWa\x06\x12\x92a\x05\xFE\x91_\x91a\x06\x14W[Pa \x9DV[a\x06\r`fT\x82\x81\x16\x14a \xB3V[a(IV[\0[a\x066\x91P` =` \x11a\x06<W[a\x06.\x81\x83a\x1E\xC8V[\x81\x01\x90a }V[_a\x05\xF8V[P=a\x06$V[a \x92V[`@\x90`\x03\x19\x01\x12a\x03\xC9W`\x04\x90V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x06}a\x06x6a\x06HV[a \xD3V[`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x06\xA2\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03\xC9W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\xC9W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\xC9WV[` `\x03\x19\x82\x01\x12a\x03\xC9W`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9Wa\x07\"\x91`\x04\x01a\x06\xC8V[\x90\x91V[4a\x03\xC9Wa\x0746a\x06\xF8V[\x90a\x07La\x07F`\x02\x80`fT\x16\x14\x90V[\x15a!\x1CV[3_R`\xD1` Ra\x07d`\xFF`@_ T\x16a!2V[a\x07s`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x07\x8BWa\x06\x12`\x01`\x97UV[\x80a\x08xa\x07\x9C`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x07\xD1\x81a\x07\xC3\x86\x863\x87a\"\xBEV[\x03`\x1F\x19\x81\x01\x83R\x82a\x1E\xC8V[Q\x90 \x90a\x07\xDE\x83a(\xDCV[3_\x90\x81R`\xD0` R`@\x90 a\x08\x11\x90a\x08\x04\x90\x84\x90[\x90_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x08\x1A\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x823\x91\x80a\x08Z\x87\x82a#\x0BV[\x03\x90\xA4`@a\x08k` \x83\x01a \xC9V[\x91\x015\x900\x903\x90a)\x90V[\x01a\x07zV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16`@Q\x90\x81R\xF3[\x90\x81a\x01\0\x91\x03\x12a\x03\xC9W\x90V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\tja\t\xA1\x916\x90`\x04\x01a\t(V[`$5\x90a\tw\x82a\x03\xB8V[a\t\x88a\x07F`\x04\x80`fT\x16\x14\x90V[a\t\x97`\x02`\x97T\x14\x15a!HV[`\x02`\x97Ua*4V[`\x01`\x97U\0[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\xC9WV[5\x90a\t\xC1\x82a\t\xA8V[V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`$5`\x045a\t\xE3\x82a\t\xA8V[a\t\xF4a\x07F`\x08\x80`fT\x16\x14\x90V[`\xCBT\x91a\n\x0C3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a!2V[c\xFF\xFF\xFF\xFF\x81\x16\x92c\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16\x84\x11\x15a\n\xF5Wc\xFF\xFF\xFF\xFF\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91a\nWB\x87\x10a#\x1CV[a\n\xDBa\n\x84a\n|a\no`\xCATc\xFF\xFF\xFF\xFF\x16\x90V[\x93`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x84B\x16a#2V[\x94a\n\xB6a\n\x90a\x1E\xE9V[\x88\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`@\x82\x01R_``\x82\x01Ra#LV[`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\xC0\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x91\x90\x91\x17\x90UV[`@Qc\xFF\xFF\xFF\xFF\x90\x94\x16\x84R\x16\x91\x80` \x81\x01[\x03\x90\xA4\0[c\x1C\xA7\xE5\x0B`\xE2\x1B_R`\x04_\xFD[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\x0B4\x906\x90`\x04\x01a\x06\xC8V[`$5\x91a\x0BA\x83a\x03\xB8V[a\x0BRa\x07F`\x04\x80`fT\x16\x14\x90V[a\x0Ba`\x02`\x97T\x14\x15a!HV[`\x02`\x97U6\x81\x90\x03`\xFE\x19\x01\x92_[\x83\x81\x10\x15a\t\xA1W\x80`\x05\x1B\x83\x015\x90\x85\x82\x12\x15a\x03\xC9Wa\x0B\x97\x83`\x01\x93\x86\x01a*4V[\x01a\x0BqV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x0B\xFE\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD5` Ra\x04\xA7a\x0CL`@_ c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[Ta\xFF\xFF\x81\x16\x83Ra\xFF\xFF\x81`\x10\x1C\x16` \x84\x01R` \x1C\x16`@\x82\x01Ra+\xEFV[`@Qa\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` c\xFF\xFF\xFF\xFF`\xCBT`\xC0\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x0C\xA7\x81a\t\xA8V[a\x0C\xAFa'\xBDV[a,&V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x06CWa\r\x1F\x91_\x91a\x06\x14WPa \x9DV[a\x06\x12a(\x15V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`\xFF\x81\x16\x80\x91\x03a\x03\xC9W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `fT`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\r\xAAa\r\xCC\x916\x90`\x04\x01a\t(V[a\r\xC6a\r\xC0\x825a\r\xBB\x81a\t\xA8V[a\x1FkV[Pa\x1F\xA5V[\x90a-\x0EV[` `@Q`\x01\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x0E\x18\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xCF` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x0Eaa'\xBDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `\xCAT`@Q\x90\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x0E\xE1\x81a\x03\xB8V[a\x0E\xE9a'\xBDV[a.\x80V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x0FD`\x045a\x0F\x10\x81a\x03\xB8V[`$5\x90a\x0F\x1D\x82a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCD\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9Wa\x0F\xFFa\x1E\xF8V[P`\xCAT_\x19\x81\x01\x90\x81\x11a\x10\x1DWa\x04ha\r\xC0a\x04\xA7\x92a\x1FkV[a\x1F\x1CV[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x10?\x81a\x03\xB8V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\xC9Wa\x10^\x906\x90`\x04\x01a\x06\xC8V[\x91\x90a\x10qa\x07F` \x80`fT\x16\x14\x90V[a\x10\x82a\x10}\x83a.\xC7V[a$<V[a\x10\x91`\x02`\x97T\x14\x15a!HV[`\x02`\x97U`\x01`\x01`\xA0\x1B\x03\x82\x16\x91_[\x84\x81\x10a\x10\xB4Wa\x06\x12`\x01`\x97UV[\x80a\x11\x8Ea\x10\xC5`\x01\x93\x88\x87a$RV[`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\xCE` R`@\x90 T\x90`@Q` \x81\x01\x90a\x10\xF6\x81a\x07\xC3\x85\x87\x8C\x87a%\xB8V[Q\x90 \x88a\x11\x03\x83a/\xB4V[\x93a\x11%a\x08\x04\x84a\x07\xF7\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xD3` R`@_ \x90V[a\x11.\x81a\"\xE2V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\xCE` R`@\x90 U\x7F\xFC\x88\x88\xBF\xFDq\x1D\xA6\x0B\xC5\t+3\xF6w\xD8\x18\x96\xFE\x80\xEC\xC6w\xB8L\xFA\xB8\x18Db\xB6\xE0`@Q\x80a\x11w\x873\x95\x83a%\xDCV[\x03\x90\xA40\x90a\x11\x89` 3\x92\x01a \xC9V[a)\x90V[\x01a\x10\xA3V[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x06\x12`\x045a\x11\xF4\x81a\x03\xB8V[3a0\xDEV[`$5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[`D5\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[`\x845\x90a\xFF\xFF\x82\x16\x82\x03a\x03\xC9WV[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\xFF\xFF\x81\x16\x81\x03a\x03\xC9Wa\x06\x12\x90a\x12Xa'\xBDV[a16V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x12z\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD2` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x12\xC8\x81a\x03\xB8V[a\x12\xD0a\x11\xFAV[a\x12\xE1a\x07F`\x80\x80`fT\x16\x14\x90V[a\x12\xEDa\x10}\x83a.\xC7V[a\x12\xFFa'\x10a\xFF\xFF\x83\x16\x11\x15a%\xF3V[\x7F\xD1\xE0(\xBDfD\x86\xA4j\xD2`@\xE9\x99\xCD-\"\xE1\xE9\xA0\x94\xEEj\xFE\x19\xFC\xF6Fx\xF1ota\x13;c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16c\xFF\xFF\xFF\xFFB\x16a#2V[\x91`\x01\x80`\xA0\x1B\x03\x84\x16\x93\x84_R`\xD5` Ra\x13\x8F\x84\x83a\x13\x8Aa\x13o`@_ c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16_\x90\x81R`\xD5` R`@\x90 \x90V[a1\x91V[`@\x80Qc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85Ra\xFF\xFF\x91\x82\x16` \x86\x01R\x91\x16\x90\x83\x01R3\x91``\x90\xA3\0[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x13\xD4\x81a\x03\xB8V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCE` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x14O\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD0` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W``6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x14\xE1\x81a\x03\xB8V[`$5a\x14\xED\x81a\x03\xB8V[a\x14\xF5a\x12\x0BV[a\x15\x06a\x07F`@\x80`fT\x16\x14\x90V[a\x15\x12a\x10}\x84a.\xC7V[a\x15$a'\x10a\xFF\xFF\x83\x16\x11\x15a%\xF3V[\x7FH\xE1\x98\xB6\xAE5~R\x92\x04\xEES\xA8\xE5\x14\xC4p\xFFw\xD9\xCC\x8EOr\x07\xF8\xB5\xD4\x90\xAEi4a\x15`c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16c\xFF\xFF\xFF\xFFB\x16a#2V[\x91`\x01\x80`\xA0\x1B\x03\x85\x16\x93\x84_R`\xD4` Ra\x15\xDC\x84\x83a\x13\x8A\x84a\x15\xC7a\x15\xACa\x15\xA7a\x15\xA2\x84`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a#\xFEV[a+\xEFV[`\x01`\x01`\xA0\x1B\x03\x90\x9C\x16_\x90\x81R`\xD4` R`@\x90 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[`@\x80Qc\xFF\xFF\xFF\xFF\x95\x90\x95\x16\x85Ra\xFF\xFF\x96\x87\x16` \x86\x01R\x91\x90\x95\x16\x90\x83\x01R`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x923\x91\x80``\x81\x01a\n\xF0V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9Wa\x04\xA7a\x04ha\r\xC0`\x045a\x16;a\x1E\xF8V[Pa\x1FkV[`@\x90`\x03\x19\x01\x12a\x03\xC9W`\x045a\x16Y\x81a\x03\xB8V[\x90`$5a\x16f\x81a\x03\xB8V[\x90V[4a\x03\xC9Wa\x16\xB8a\x16\xA6a\x16}6a\x16AV[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD4` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[c\xFF\xFF\xFF\xFF`@Q\x91a\x0C)\x83a\x1E\x8DV[`@Qa\xFF\xFF\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W` a\x16\xE6`\x045a&!V[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x17U\x81a\x03\xB8V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD3` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\xC9Wa\x17\x946a\x16AV[a\x17\xA0a\x10}\x83a.\xC7V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06CW_\x91a\x18\xBFW[P\x80\x15a\x18\x1AW[\x91a\x18\x15a\x06\x12\x93a&\x86V[a0\xDEV[P`@Qc\xBA\x1A\x84\xE5`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R\x91` \x83\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x92\x83\x15a\x06CWa\x06\x12\x93a\x18\x15\x91_\x91a\x18\x90W[P\x15\x15\x91\x93PPa\x18\x08V[a\x18\xB2\x91P` =` \x11a\x18\xB8W[a\x18\xAA\x81\x83a\x1E\xC8V[\x81\x01\x90a&wV[_a\x18\x84V[P=a\x18\xA0V[a\x18\xD8\x91P` =` \x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[_a\x18\0V[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x18\xFB\x81a\x03\xB8V[a\x19\x03a'\xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x19\x1BWa\x06\x12\x90a.8V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\xC9W`\xA06`\x03\x19\x01\x12a\x03\xC9W`\x045a\x19\x8C\x81a\x03\xB8V[a\x19\xFF`$5`D5a\x19\x9E\x81a\x03\xB8V[`d5\x90a\x19\xAB\x82a\t\xA8V[a\x19\xB3a\x12\x1CV[\x92_T\x95a\x19\xE5a\x19\xCFa\x19\xCB\x89`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x98\x81\x99a\x1AyW[\x81\x15a\x1AYW[Pa&\x9CV[\x86a\x19\xF6`\x01`\xFF\x19_T\x16\x17_UV[a\x1ABWa&\xFFV[a\x1A\x05W\0[a\x1A\x13a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x1ATa\x01\0a\xFF\0\x19_T\x16\x17_UV[a&\xFFV[0;\x15\x91P\x81a\x1AkW[P_a\x19\xDFV[`\xFF\x16`\x01\x14\x90P_a\x1AdV[`\x01`\xFF\x82\x16\x10\x91Pa\x19\xD8V[4a\x03\xC9W`@6`\x03\x19\x01\x12a\x03\xC9W` a\x06}a\x1A\xA66a\x06HV[a'\x1DV[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045a\x1A\xC8\x81a\t\xA8V[a\x1A\xD9a\x07F`\x08\x80`fT\x16\x14\x90V[a\x1A\xEE`\x01\x80`\xA0\x1B\x03`\xCBT\x163\x14a!2V[`\xCAT\x90c\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x10\x15a\x1BvWa\x1B\x0C`\x01\x91a\x1FkV[P\x01c\xFF\xFF\xFF\xFF\x81Ta\x1B%`\xFF\x82`@\x1C\x16\x15a'dV[` \x1C\x16B\x10\x15a\x1BgW\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E_\x80\xA2\0[c\x0C6\xF6e`\xE2\x1B_R`\x04_\xFD[c\x94\xA8\xD3\x89`\xE0\x1B_R`\x04_\xFD[4a\x03\xC9W` 6`\x03\x19\x01\x12a\x03\xC9W`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06CW_\x91a\x1C\x0EW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1B\xFFWa\x06\x12\x90a'zV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a\x1CAW[\x81a\x1C)` \x93\x83a\x1E\xC8V[\x81\x01\x03\x12a\x03\xC9WQa\x1C;\x81a\x03\xB8V[_a\x1B\xE6V[=\x91Pa\x1C\x1CV[4a\x03\xC9W_6`\x03\x19\x01\x12a\x03\xC9W`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\xC9Wa\x1C\x7F6a\x06\xF8V[\x90a\x1C\x91a\x07F`\x01\x80`fT\x16\x14\x90V[a\x1C\xA0`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x1C\xB8Wa\x06\x12`\x01`\x97UV[\x80a\x1Dca\x1C\xC9`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x1C\xF0\x81a\x07\xC3\x86\x863\x87a\"\xBEV[Q\x90 \x90a\x1C\xFD\x83a(\xDCV[3_\x90\x81R`\xCF` R`@\x90 a\x1D\x1A\x90a\x08\x04\x90\x84\x90a\x07\xF7V[a\x1D#\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x813\x91\x80a\x08Z\x87\x82a#\x0BV[\x01a\x1C\xA7V[4a\x03\xC9Wa\x1Dw6a\x06\xF8V[\x90a\x1D\x89a\x07F`\x10\x80`fT\x16\x14\x90V[3_R`\xD1` Ra\x1D\xA1`\xFF`@_ T\x16a!2V[a\x1D\xB0`\x02`\x97T\x14\x15a!HV[`\x02`\x97U_[\x82\x81\x10a\x1D\xC8Wa\x06\x12`\x01`\x97UV[\x80a\x1Esa\x1D\xD9`\x01\x93\x86\x86a!\x94V[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x1E\0\x81a\x07\xC3\x86\x863\x87a\"\xBEV[Q\x90 \x90a\x1E\r\x83a(\xDCV[3_\x90\x81R`\xD2` R`@\x90 a\x1E*\x90a\x08\x04\x90\x84\x90a\x07\xF7V[a\x1E3\x81a\"\xE2V[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B3\x91\x80a\x08Z\x87\x82a#\x0BV[\x01a\x1D\xB7V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[a\x1EyV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1E\xA8W`@RV[`@Q\x90a\t\xC1`\x80\x83a\x1E\xC8V[`@Q\x90a\x1F\x05\x82a\x1E\xADV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x10\x1DW_\x19\x01\x90V[_\x19\x81\x01\x91\x90\x82\x11a\x10\x1DWV[\x91\x90\x82\x03\x91\x82\x11a\x10\x1DWV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xCAT\x81\x10\x15a\x1F\x87W`\xCA_R` _ \x90`\x01\x1B\x01\x90_\x90V[a\x1FWV[\x80T\x82\x10\x15a\x1F\x87W_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90`@Qa\x1F\xB2\x81a\x1E\xADV[```\xFF`\x01\x83\x95\x80T\x85R\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x85\x01Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16`@\x85\x01R`@\x1C\x16\x15\x15\x91\x01RV[a\x1F\xEEa\x1E\xF8V[P`\xCAT\x80[a \x19WPa \x01a\x1E\xE9V[_\x81R_` \x82\x01R_`@\x82\x01R_``\x82\x01R\x90V[a (a\r\xC0a\r\xBB\x83a\x1F<V[\x90a 9a\x19\xCB``\x84\x01Q\x15\x15\x90V[\x80a WW[a SWa M\x91Pa\x1F0V[\x80a\x1F\xF4V[P\x90V[Pa ua l`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a ?V[\x90\x81` \x91\x03\x12a\x03\xC9WQa\x16f\x81a\x04\xABV[`@Q=_\x82>=\x90\xFD[\x15a \xA4WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a \xBAWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[5a\x16f\x81a\x03\xB8V[` \x815\x91a \xE1\x83a\x03\xB8V[\x015`@Q\x90` \x82\x01\x92_\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra!\x16`U\x82a\x1E\xC8V[Q\x90 \x90V[\x15a!#WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a!9WV[c\\B|\xD9`\xE0\x1B_R`\x04_\xFD[\x15a!OWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\xC9W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W\x81`\x06\x1B6\x03\x83\x13a\x03\xC9WV[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\"\x04WPPP\x90V[\x90\x91\x92\x835a\"\x12\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x015`\x01`\x01``\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x03\xC9W`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a!\xF7V[\x90`\x80c\xFF\xFF\xFF\xFF\x81a\"pa\"b\x86\x80a!\xB6V[`\xA0\x87R`\xA0\x87\x01\x91a!\xEAV[\x94` \x81\x015a\"\x7F\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01R`@\x81\x81\x015\x90\x86\x01R\x82``\x82\x015a\"\xA6\x81a\t\xA8V[\x16``\x86\x01R\x015a\"\xB7\x81a\t\xA8V[\x16\x91\x01R\x90V[a\x16f\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\"LV[\x90`\x01\x82\x01\x80\x92\x11a\x10\x1DWV[\x90` \x82\x01\x80\x92\x11a\x10\x1DWV[\x91\x90\x82\x01\x80\x92\x11a\x10\x1DWV[\x90` a\x16f\x92\x81\x81R\x01\x90a\"LV[\x15a##WV[c\x06\x95|\x91`\xE1\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\x1DWV[`\xCAT`\x01`@\x1B\x81\x10\x15a\x1E\xA8W\x80`\x01a#m\x92\x01`\xCAU`\xCAa\x1F\x8CV[\x91\x90\x91a#\xEBW```\x01a\t\xC1\x93\x83Q\x81U\x01\x91c\xFF\xFF\xFF\xFF` \x82\x01Q\x16c\xFF\xFF\xFF\xFF\x19\x84T\x16\x17\x83Ua#\xCAc\xFF\xFF\xFF\xFF`@\x83\x01Q\x16\x84\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x01Q\x81T`\xFF`@\x1B\x19\x16\x90\x15\x15`@\x1Bh\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x90`@Qa$\x0B\x81a\x1E\x8DV[`@c\xFF\xFF\xFF\xFF\x82\x94Ta\xFF\xFF\x81\x16\x84Ra\xFF\xFF\x81`\x10\x1C\x16` \x85\x01R` \x1C\x16\x91\x01RV[5a\x16f\x81a\t\xA8V[\x15a$CWV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x81\x015\x90`\xBE\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x90V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03\xC9W\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W\x816\x03\x83\x13a\x03\xC9WV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90a$\xE2a$\xD4\x84\x80a!\xB6V[`\xC0\x84R`\xC0\x84\x01\x91a!\xEAV[\x90` \x84\x015a$\xF1\x81a\x03\xB8V[`\x01`\x01`\xA0\x1B\x03\x16` \x82\x81\x01\x91\x90\x91Ra%\x10`@\x86\x01\x86a!\xB6V[\x83\x85\x03`@\x85\x01R\x80\x85R\x93\x90\x91\x01\x92_[\x81\x81\x10a%\x84WPPPa%v\x84a%Pa%C``a\x16f\x97\x98\x01a\t\xB6V[c\xFF\xFF\xFF\xFF\x16``\x85\x01RV[a%la%_`\x80\x83\x01a\t\xB6V[c\xFF\xFF\xFF\xFF\x16`\x80\x85\x01RV[`\xA0\x81\x01\x90a$tV[\x91`\xA0\x81\x85\x03\x91\x01Ra$\xA5V[\x90\x91\x93`@\x80`\x01\x92\x875a%\x98\x81a\x03\xB8V[\x84\x80`\xA0\x1B\x03\x16\x81R` \x88\x015` \x82\x01R\x01\x95\x01\x91\x01\x91\x90\x91a%\"V[a\x16f\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a$\xC5V[`@\x90a\x16f\x93\x92\x81R\x81` \x82\x01R\x01\x90a$\xC5V[\x15a%\xFAWV[c\x89\x1Cc\xDF`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\x1DWV[c\xFF\xFF\xFF\xFF`\xCAT\x16[c\xFF\xFF\xFF\xFF\x81\x16a&EWcPEp\xE3`\xE0\x1B_R`\x04_\xFD[\x81a&Ra\r\xBB\x83a&\tV[PT\x14a&mWc\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\x1DW_\x19\x01a&+V[a\x16f\x91Pa&\tV[\x90\x81` \x91\x03\x12a\x03\xC9WQ\x90V[\x15a&\x8DWV[c\xFBIN\xA1`\xE0\x1B_R`\x04_\xFD[\x15a&\xA3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a\x12X\x92a\x0E\xE9a\x0C\xAF\x92a'\x18a\t\xC1\x98\x97\x95a(IV[a.8V[` \x815\x91a'+\x83a\x03\xB8V[\x015`@Q\x90` \x82\x01\x92`\x01`\xF8\x1B\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra!\x16`U\x82a\x1E\xC8V[\x15a'kWV[c\x1B\x14\x17K`\xE0\x1B_R`\x04_\xFD[a'\x8B`fT\x19\x82\x19\x81\x16\x14a \xB3V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a'\xD1WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x03\xC9WV[\x15a(\xB7WV[c\x10\xEBH?`\xE2\x1B_R`\x04_\xFD[\x15a(\xCDWV[c\x07\x0BZo`\xE2\x1B_R`\x04_\xFD[a):oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF`@a)&\x93a)\0\x81\x80a({V[\x95\x90``\x83\x015\x80\x97a)\x12\x82a\t\xA8V[`\x80\x85\x015\x92a)!\x84a\t\xA8V[a2\xD5V[\x015a)3\x81\x15\x15a(\xB0V[\x11\x15a(\xC6V[a)C\x81a\t\xA8V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16B\x01\x90\x81B\x11a\x10\x1DWc\xFF\xFF\xFF\xFF\x16\x11a)\x81WV[c~\xE2\xB4C`\xE0\x1B_R`\x04_\xFD[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\t\xC1\x91a)\xD4`\x84\x83a\x1E\xC8V[a5[V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03\xC9WV[\x91\x90\x81\x10\x15a\x1F\x87W`\x06\x1B\x01\x90V[\x15a*%WV[c\xAA8^\x81`\xE0\x1B_R`\x04_\xFD[\x90a*Da\r\xC0a\r\xBB\x84a$2V[\x90a*O\x82\x84a-\x0EV[a*[``\x84\x01a \xC9V[\x93a*\x86a*y\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R`@_ \x90V[T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a+\xE8W[\x90\x93`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91a*\xAF3\x84\x14a!2V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x91_[a*\xC9`\xA0\x83\x01\x83a)\xD9V[\x90P\x81\x10\x15a+\xDEW\x80a*\xEC`\x01\x92a*\xE6`\xE0\x86\x01\x86a({V[\x90a*\x0EV[\x86\x86\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x8Ca+Ja+-\x82`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a+6\x87a \xC9V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta+\x8Fa+\x86a+k` \x89\x015\x93a+e\x81\x86\x11a*\x1EV[\x84a\x1FJV[`\x01`\x01`\xA0\x1B\x03\x90\x94\x16_\x90\x81R`\xCD` R`@\x90 \x90V[a+6\x88a \xC9V[Ua+\xA3\x81\x8Aa+\x9E\x88a \xC9V[a6-V[a+\xAE\x8CQ\x95a \xC9V[`@\x80Q\x96\x87R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16` \x87\x01R\x85\x01R`\xA0\x86\x90\x1B\x86\x90\x03\x88\x16\x93``\x90\xA4\x01a*\xBCV[PPPPPP\x90PV[P\x84a*\x96V[`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x81a,\x0FWPPa\xFF\xFF`\xCBT`\xE0\x1C\x16\x90V[a\xFF\xFF\x91B\x10a,!W` \x01Q\x16\x90V[Q\x16\x90V[`\xCBT\x90\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3`@\x80Qc\xFF\xFF\xFF\xFF\x85`\xA0\x1C\x16\x81Rc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R\xA1c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17`\xCBUV[\x15a,\x90WV[c\x147\xA2\xBB`\xE3\x1B_R`\x04_\xFD[\x15a,\xA6WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\xC9W\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\xC9W` \x01\x91\x816\x03\x83\x13a\x03\xC9WV[\x91\x90\x81\x10\x15a\x1F\x87W`\x05\x1B\x01\x90V[\x90\x82\x10\x15a\x1F\x87Wa\x07\"\x91`\x05\x1B\x81\x01\x90a,\xB5V[\x91\x90\x91a-)a-$a\x19\xCB``\x86\x01Q\x15\x15\x90V[a'dV[a-Ha-@a l`@\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a,\x89V[`\xA0\x81\x01\x90a-W\x82\x82a)\xD9V[\x90Pa-t`\xC0\x83\x01\x91a-k\x83\x85a)\xD9V[\x91\x90P\x14a,\x9FV[a-\xBBa-\x81\x82\x84a)\xD9V[\x96\x90Pa-\x96`\xE0\x85\x01\x97a-k\x89\x87a({V[Qa-\xA3` \x85\x01a$2V[a-\xB0`@\x86\x01\x86a,\xB5V[\x91``\x87\x01\x93a6\xB4V[`\x80\x82\x015\x92_[a-\xCD\x82\x85a)\xD9V[\x90P\x81\x10\x15a./W\x80a.)\x85a*\xE6\x8Aa.\"\x85a.\x19\x81a.\x13\x8C\x8Fa.\x08`\x01\x9D\x8Fa.\x02\x90a.\r\x94P\x8Da)\xD9V[\x90a,\xE7V[a$2V[\x98a)\xD9V[\x90a,\xF7V[\x94\x90\x93\x8Ca({V[\x92\x8Aa6\xFFV[\x01a-\xC3V[PPPPP\x90PV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x82\x90\x82\x16\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB_\x80\xA3`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`\xCBUV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x06CW_\x91a/CWP\x90V[a\x16f\x91P` =` \x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[\x15a/cWV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a/yWV[c\x15\x03X\xA1`\xE2\x1B_R`\x04_\xFD[\x15a/\x8FWV[c\x08c\xA4S`\xE1\x1B_R`\x04_\xFD[\x15a/\xA5WV[c\x10\xFBG\xF1`\xE3\x1B_R`\x04_\xFD[\x90a0\x1A\x91a0(a/\xC6\x82\x80a({V[``\x84\x01\x95\x91a/\xED\x90a/\xD9\x88a$2V[`\x80\x87\x01\x93a/\xE7\x85a$2V[\x92a2\xD5V[a0\x14a0\x0E`@\x86\x01\x97a.\x08a0\x05\x8A\x89a({V[\x90P\x15\x15a/\\V[\x91a$2V[\x90a#2V[c\xFF\xFF\xFF\xFFB\x91\x16\x10a/rV[_\x92\x83\x91\x82[a08\x83\x83a({V[\x90P\x84\x10\x15a0\xBDWa0\xB4`\x01\x91a0\x95a0X\x87a*\xE6\x88\x88a({V[\x91a0{a0ta0h\x85a \xC9V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x15\x15a/\x88V[a0\x87a0h\x84a \xC9V[\x90\x85\x80`\xA0\x1B\x03\x16\x10a/\x9EV[a0\xAE` \x82\x015\x91a0\xA9\x83\x15\x15a(\xB0V[a \xC9V[\x97a\"\xFEV[\x93\x01\x92\x94a0.V[PPPP\x90a\x16foK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF\x82\x11\x15a(\xC6V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\xCC` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x95\x85\x16\x95\x86\x17\x90\x91U\x90\x92\x16\x91\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x90\x80\xA4V[`\xCBT\x90\x7F\xE6\xCDN\xDF\xDC\xC1\xF6\xD10\xAB5\xF7=r7\x8F:d)D\xFBN\xE5\xBD\x84\xB7\x80z\x81\xEA\x1CN`@\x80Qa\xFF\xFF\x85`\xE0\x1C\x16\x81Ra\xFF\xFF\x84\x16` \x82\x01R\xA1a\xFF\xFF`\xE0\x1B\x19\x90\x91\x16`\xE0\x91\x90\x91\x1Ba\xFF\xFF`\xE0\x1B\x16\x17`\xCBUV[\x91\x90\x91\x80T\x92c\xFF\xFF\xFF\xFF\x84` \x1C\x16\x93\x84B\x11\x15a2\x16Wa\t\xC1\x94a1\xFDWP`\xCBT\x82Ta\xFF\xFF\x19\x16`\xE0\x91\x90\x91\x1Ca\xFF\xFF\x16\x17\x82U[\x81Tg\xFF\xFF\xFF\xFF\xFF\xFF\0\0\x19\x16`\x10\x91\x90\x91\x1Bc\xFF\xFF\0\0\x16\x17` \x92\x90\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x91\x90\x91\x17\x90UV[\x82Ta\xFF\xFF\x19\x16`\x10\x91\x90\x91\x1Ca\xFF\xFF\x16\x17\x82Ua1\xCBV[c{\x1E%\xC5`\xE0\x1B_R`\x04_\xFD[\x15a2,WV[c\r\xD0\xB9\xF5`\xE2\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x16\x90\x81\x15a2SWc\xFF\xFF\xFF\xFF\x16\x06\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a2nWV[c\xEEfG\x05`\xE0\x1B_R`\x04_\xFD[\x15a2\x84WV[c<\x1A\x94\xF1`\xE2\x1B_R`\x04_\xFD[\x15a2\x9AWV[c\x04\x1A\xA7W`\xE1\x1B_R`\x04_\xFD[\x15a2\xB0WV[c.\xFD\x96Q`\xE1\x1B_R`\x04_\xFD[\x15a2\xC6WV[c\xDF\xAD\x9C\xA1`\xE0\x1B_R`\x04_\xFD[\x92\x91a3na3ha la3\xB5\x94\x95a2\xF0\x87\x15\x15a/\\V[a3(c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xFF\xFF\xFF\xFF\x83\x16\x11\x15a2%V[a3bc\xFF\xFF\xFF\xFFa3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x94a2;V[\x16\x15a2gV[\x84a2;V[\x15a2}V[c\xFF\xFF\xFF\xFFa3\x9F\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x1FJV[\x91\x16\x80\x91\x11\x15\x90\x81a4\xB0W[P\x92\x91\x92a2\x93V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_\x90\x81[\x81\x83\x10a3\xF2WPPPPPV[a4\0a0\xA9\x84\x84\x87a*\x0EV[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x8AZ\xFA\x92\x83\x15a\x06CW`\x01\x93a4d\x92_\x91a4\x92W[P\x80\x15a4lW[a4N\x90a2\xA9V[\x83\x80`\xA0\x1B\x03\x16\x80\x92\x84\x80`\xA0\x1B\x03\x16\x10a2\xBFV[\x92\x01\x91a3\xE4V[P`\xA0\x84\x90\x1B\x84\x90\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a4EV[a4\xAA\x91P` =\x81\x11a\x06<Wa\x06.\x81\x83a\x1E\xC8V[_a4=V[\x90Pc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x11\x15_a3\xACV[`\x01`\x01`@\x1B\x03\x81\x11a\x1E\xA8W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x15a5\x03WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90`@Q\x90a5s`@\x83a\x1E\xC8V[` \x82R\x7FSafeERC20: low-level call failed` \x83\x01R\x82;\x15a5\xE8W_\x81a5\xC3\x94\x82` \x81\x95Q\x93\x01\x91Z\xF1a5\xBDa7\xAFV[\x90a7\xDEV[\x80Q\x80a5\xCEWPPV[\x81` \x80a5\xE3\x93a\t\xC1\x95\x01\x01\x91\x01a }V[a4\xFCV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x80\x83\x01\x93\x90\x93R\x91\x81Ra\t\xC1\x91a)\xD4`d\x83a\x1E\xC8V[\x92\x91\x92a6t\x82a4\xE1V[\x91a6\x82`@Q\x93\x84a\x1E\xC8V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03\xC9W\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x15a6\xA5WV[ci\xCA\x16\xC9`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a6\xF1Wa6\xE7a6\xEC\x94a6\xDFa\t\xC1\x97a \xD3V[\x936\x91a6hV[a79V[a6\x9EV[b\xC6\xC3\x9D`\xE7\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a7*Wa6\xE7a6\xEC\x94a6\xDFa\t\xC1\x97a'\x1DV[c\x05O\xF4\xDF`\xE5\x1B_R`\x04_\xFD[\x93\x90\x92\x91`\x1F\x85Q\x16a7\xA0W\x91\x90` \x92[\x85Q\x84\x11a7\x97W`\x01\x83\x16a7}W_R\x82\x85\x01Q` Ra7v`@_ \x92`\x01\x1C\x93a\"\xF0V[\x92\x91a7LV[\x83\x86\x01Q_R` Ra7v`@_ \x92`\x01\x1C\x93a\"\xF0V[\x92P\x93PP\x14\x90V[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[=\x15a7\xD9W=\x90a7\xC0\x82a4\xE1V[\x91a7\xCE`@Q\x93\x84a\x1E\xC8V[\x82R=_` \x84\x01>V[``\x90V[\x90\x91\x90\x15a7\xEAWP\x90V[\x81Q\x15a7\xFAWP\x80Q\x90` \x01\xFD[`D` \x91`@Q\x92\x83\x91bF\x1B\xCD`\xE5\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD\xFE\xA2dipfsX\"\x12 \x88\\j\x97\x07=\xF6d8\x92\xA2GY\x9C\xB0\xFD\x91$l\xA8\x91\xD5\xB0\xCD\x18m\xEF\r\x9C\r{\xB0dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `AmountExceedsMax()` and selector `0x1c2d69bc`.
```solidity
error AmountExceedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AmountExceedsMax {}
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
        impl ::core::convert::From<AmountExceedsMax> for UnderlyingRustTuple<'_> {
            fn from(value: AmountExceedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AmountExceedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AmountExceedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AmountExceedsMax()";
            const SELECTOR: [u8; 4] = [28u8, 45u8, 105u8, 188u8];
            #[inline]
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
    /**Custom error with signature `AmountIsZero()` and selector `0x43ad20fc`.
```solidity
error AmountIsZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AmountIsZero {}
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
        impl ::core::convert::From<AmountIsZero> for UnderlyingRustTuple<'_> {
            fn from(value: AmountIsZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AmountIsZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AmountIsZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AmountIsZero()";
            const SELECTOR: [u8; 4] = [67u8, 173u8, 32u8, 252u8];
            #[inline]
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
    /**Custom error with signature `DurationExceedsMax()` and selector `0x3742e7d4`.
```solidity
error DurationExceedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DurationExceedsMax {}
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
        impl ::core::convert::From<DurationExceedsMax> for UnderlyingRustTuple<'_> {
            fn from(value: DurationExceedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DurationExceedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DurationExceedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DurationExceedsMax()";
            const SELECTOR: [u8; 4] = [55u8, 66u8, 231u8, 212u8];
            #[inline]
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
    /**Custom error with signature `EarningsNotGreaterThanClaimed()` and selector `0xaa385e81`.
```solidity
error EarningsNotGreaterThanClaimed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct EarningsNotGreaterThanClaimed {}
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
        impl ::core::convert::From<EarningsNotGreaterThanClaimed>
        for UnderlyingRustTuple<'_> {
            fn from(value: EarningsNotGreaterThanClaimed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for EarningsNotGreaterThanClaimed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for EarningsNotGreaterThanClaimed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "EarningsNotGreaterThanClaimed()";
            const SELECTOR: [u8; 4] = [170u8, 56u8, 94u8, 129u8];
            #[inline]
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
    /**Custom error with signature `InvalidAddressZero()` and selector `0x10c748a6`.
```solidity
error InvalidAddressZero();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAddressZero {}
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
        impl ::core::convert::From<InvalidAddressZero> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAddressZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAddressZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAddressZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAddressZero()";
            const SELECTOR: [u8; 4] = [16u8, 199u8, 72u8, 166u8];
            #[inline]
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
    /**Custom error with signature `InvalidCalculationIntervalSecondsRemainder()` and selector `0x4478f672`.
```solidity
error InvalidCalculationIntervalSecondsRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCalculationIntervalSecondsRemainder {}
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
        impl ::core::convert::From<InvalidCalculationIntervalSecondsRemainder>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCalculationIntervalSecondsRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidCalculationIntervalSecondsRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCalculationIntervalSecondsRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCalculationIntervalSecondsRemainder()";
            const SELECTOR: [u8; 4] = [68u8, 120u8, 246u8, 114u8];
            #[inline]
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
    /**Custom error with signature `InvalidClaimProof()` and selector `0x69ca16c9`.
```solidity
error InvalidClaimProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidClaimProof {}
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
        impl ::core::convert::From<InvalidClaimProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidClaimProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidClaimProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidClaimProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidClaimProof()";
            const SELECTOR: [u8; 4] = [105u8, 202u8, 22u8, 201u8];
            #[inline]
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
    /**Custom error with signature `InvalidDurationRemainder()` and selector `0xee664705`.
```solidity
error InvalidDurationRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidDurationRemainder {}
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
        impl ::core::convert::From<InvalidDurationRemainder>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidDurationRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidDurationRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidDurationRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidDurationRemainder()";
            const SELECTOR: [u8; 4] = [238u8, 102u8, 71u8, 5u8];
            #[inline]
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
    /**Custom error with signature `InvalidEarner()` and selector `0xfb494ea1`.
```solidity
error InvalidEarner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEarner {}
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
        impl ::core::convert::From<InvalidEarner> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEarner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEarner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEarner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEarner()";
            const SELECTOR: [u8; 4] = [251u8, 73u8, 78u8, 161u8];
            #[inline]
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
    /**Custom error with signature `InvalidEarnerLeafIndex()` and selector `0x6361ce80`.
```solidity
error InvalidEarnerLeafIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEarnerLeafIndex {}
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
        impl ::core::convert::From<InvalidEarnerLeafIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEarnerLeafIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEarnerLeafIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEarnerLeafIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEarnerLeafIndex()";
            const SELECTOR: [u8; 4] = [99u8, 97u8, 206u8, 128u8];
            #[inline]
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
    /**Custom error with signature `InvalidGenesisRewardsTimestampRemainder()` and selector `0x0e06bd31`.
```solidity
error InvalidGenesisRewardsTimestampRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidGenesisRewardsTimestampRemainder {}
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
        impl ::core::convert::From<InvalidGenesisRewardsTimestampRemainder>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidGenesisRewardsTimestampRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidGenesisRewardsTimestampRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidGenesisRewardsTimestampRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidGenesisRewardsTimestampRemainder()";
            const SELECTOR: [u8; 4] = [14u8, 6u8, 189u8, 49u8];
            #[inline]
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
    /**Custom error with signature `InvalidProofLength()` and selector `0x4dc5f6a4`.
```solidity
error InvalidProofLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProofLength {}
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
        impl ::core::convert::From<InvalidProofLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProofLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProofLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProofLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProofLength()";
            const SELECTOR: [u8; 4] = [77u8, 197u8, 246u8, 164u8];
            #[inline]
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
    /**Custom error with signature `InvalidRoot()` and selector `0x504570e3`.
```solidity
error InvalidRoot();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRoot {}
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
        impl ::core::convert::From<InvalidRoot> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRoot) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRoot {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRoot {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRoot()";
            const SELECTOR: [u8; 4] = [80u8, 69u8, 112u8, 227u8];
            #[inline]
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
    /**Custom error with signature `InvalidRootIndex()` and selector `0x94a8d389`.
```solidity
error InvalidRootIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidRootIndex {}
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
        impl ::core::convert::From<InvalidRootIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidRootIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidRootIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidRootIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidRootIndex()";
            const SELECTOR: [u8; 4] = [148u8, 168u8, 211u8, 137u8];
            #[inline]
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
    /**Custom error with signature `InvalidStartTimestampRemainder()` and selector `0xf06a53c4`.
```solidity
error InvalidStartTimestampRemainder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidStartTimestampRemainder {}
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
        impl ::core::convert::From<InvalidStartTimestampRemainder>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidStartTimestampRemainder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidStartTimestampRemainder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidStartTimestampRemainder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidStartTimestampRemainder()";
            const SELECTOR: [u8; 4] = [240u8, 106u8, 83u8, 196u8];
            #[inline]
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
    /**Custom error with signature `InvalidTokenLeafIndex()` and selector `0xa9fe9be0`.
```solidity
error InvalidTokenLeafIndex();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTokenLeafIndex {}
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
        impl ::core::convert::From<InvalidTokenLeafIndex> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTokenLeafIndex) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTokenLeafIndex {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTokenLeafIndex {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTokenLeafIndex()";
            const SELECTOR: [u8; 4] = [169u8, 254u8, 155u8, 224u8];
            #[inline]
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
    /**Custom error with signature `NewRootMustBeForNewCalculatedPeriod()` and selector `0x729f942c`.
```solidity
error NewRootMustBeForNewCalculatedPeriod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NewRootMustBeForNewCalculatedPeriod {}
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
        impl ::core::convert::From<NewRootMustBeForNewCalculatedPeriod>
        for UnderlyingRustTuple<'_> {
            fn from(value: NewRootMustBeForNewCalculatedPeriod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for NewRootMustBeForNewCalculatedPeriod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NewRootMustBeForNewCalculatedPeriod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NewRootMustBeForNewCalculatedPeriod()";
            const SELECTOR: [u8; 4] = [114u8, 159u8, 148u8, 44u8];
            #[inline]
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
    /**Custom error with signature `OperatorsNotInAscendingOrder()` and selector `0x87da3f88`.
```solidity
error OperatorsNotInAscendingOrder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorsNotInAscendingOrder {}
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
        impl ::core::convert::From<OperatorsNotInAscendingOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorsNotInAscendingOrder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorsNotInAscendingOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorsNotInAscendingOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorsNotInAscendingOrder()";
            const SELECTOR: [u8; 4] = [135u8, 218u8, 63u8, 136u8];
            #[inline]
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
    /**Custom error with signature `PreviousSplitPending()` and selector `0x7b1e25c5`.
```solidity
error PreviousSplitPending();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct PreviousSplitPending {}
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
        impl ::core::convert::From<PreviousSplitPending> for UnderlyingRustTuple<'_> {
            fn from(value: PreviousSplitPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PreviousSplitPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PreviousSplitPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PreviousSplitPending()";
            const SELECTOR: [u8; 4] = [123u8, 30u8, 37u8, 197u8];
            #[inline]
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
    /**Custom error with signature `RewardsEndTimestampNotElapsed()` and selector `0x0d2af922`.
```solidity
error RewardsEndTimestampNotElapsed();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RewardsEndTimestampNotElapsed {}
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
        impl ::core::convert::From<RewardsEndTimestampNotElapsed>
        for UnderlyingRustTuple<'_> {
            fn from(value: RewardsEndTimestampNotElapsed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for RewardsEndTimestampNotElapsed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RewardsEndTimestampNotElapsed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RewardsEndTimestampNotElapsed()";
            const SELECTOR: [u8; 4] = [13u8, 42u8, 249u8, 34u8];
            #[inline]
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
    /**Custom error with signature `RootAlreadyActivated()` and selector `0x30dbd994`.
```solidity
error RootAlreadyActivated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootAlreadyActivated {}
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
        impl ::core::convert::From<RootAlreadyActivated> for UnderlyingRustTuple<'_> {
            fn from(value: RootAlreadyActivated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootAlreadyActivated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RootAlreadyActivated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RootAlreadyActivated()";
            const SELECTOR: [u8; 4] = [48u8, 219u8, 217u8, 148u8];
            #[inline]
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
    /**Custom error with signature `RootDisabled()` and selector `0x1b14174b`.
```solidity
error RootDisabled();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootDisabled {}
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
        impl ::core::convert::From<RootDisabled> for UnderlyingRustTuple<'_> {
            fn from(value: RootDisabled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootDisabled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RootDisabled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RootDisabled()";
            const SELECTOR: [u8; 4] = [27u8, 20u8, 23u8, 75u8];
            #[inline]
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
    /**Custom error with signature `RootNotActivated()` and selector `0xa1bd15d8`.
```solidity
error RootNotActivated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RootNotActivated {}
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
        impl ::core::convert::From<RootNotActivated> for UnderlyingRustTuple<'_> {
            fn from(value: RootNotActivated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RootNotActivated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for RootNotActivated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "RootNotActivated()";
            const SELECTOR: [u8; 4] = [161u8, 189u8, 21u8, 216u8];
            #[inline]
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
    /**Custom error with signature `SplitExceedsMax()` and selector `0x891c63df`.
```solidity
error SplitExceedsMax();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SplitExceedsMax {}
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
        impl ::core::convert::From<SplitExceedsMax> for UnderlyingRustTuple<'_> {
            fn from(value: SplitExceedsMax) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SplitExceedsMax {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SplitExceedsMax {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SplitExceedsMax()";
            const SELECTOR: [u8; 4] = [137u8, 28u8, 99u8, 223u8];
            #[inline]
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
    /**Custom error with signature `StartTimestampTooFarInFuture()` and selector `0x7ee2b443`.
```solidity
error StartTimestampTooFarInFuture();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StartTimestampTooFarInFuture {}
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
        impl ::core::convert::From<StartTimestampTooFarInFuture>
        for UnderlyingRustTuple<'_> {
            fn from(value: StartTimestampTooFarInFuture) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StartTimestampTooFarInFuture {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StartTimestampTooFarInFuture {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StartTimestampTooFarInFuture()";
            const SELECTOR: [u8; 4] = [126u8, 226u8, 180u8, 67u8];
            #[inline]
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
    /**Custom error with signature `StartTimestampTooFarInPast()` and selector `0x08354eae`.
```solidity
error StartTimestampTooFarInPast();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StartTimestampTooFarInPast {}
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
        impl ::core::convert::From<StartTimestampTooFarInPast>
        for UnderlyingRustTuple<'_> {
            fn from(value: StartTimestampTooFarInPast) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StartTimestampTooFarInPast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StartTimestampTooFarInPast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StartTimestampTooFarInPast()";
            const SELECTOR: [u8; 4] = [8u8, 53u8, 78u8, 174u8];
            #[inline]
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
    /**Custom error with signature `StrategiesNotInAscendingOrder()` and selector `0xdfad9ca1`.
```solidity
error StrategiesNotInAscendingOrder();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategiesNotInAscendingOrder {}
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
        impl ::core::convert::From<StrategiesNotInAscendingOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategiesNotInAscendingOrder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategiesNotInAscendingOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategiesNotInAscendingOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategiesNotInAscendingOrder()";
            const SELECTOR: [u8; 4] = [223u8, 173u8, 156u8, 161u8];
            #[inline]
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
    /**Custom error with signature `StrategyNotWhitelisted()` and selector `0x5dfb2ca2`.
```solidity
error StrategyNotWhitelisted();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyNotWhitelisted {}
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
        impl ::core::convert::From<StrategyNotWhitelisted> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyNotWhitelisted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyNotWhitelisted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyNotWhitelisted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategyNotWhitelisted()";
            const SELECTOR: [u8; 4] = [93u8, 251u8, 44u8, 162u8];
            #[inline]
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
    /**Custom error with signature `SubmissionNotRetroactive()` and selector `0x540d6284`.
```solidity
error SubmissionNotRetroactive();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SubmissionNotRetroactive {}
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
        impl ::core::convert::From<SubmissionNotRetroactive>
        for UnderlyingRustTuple<'_> {
            fn from(value: SubmissionNotRetroactive) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SubmissionNotRetroactive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SubmissionNotRetroactive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SubmissionNotRetroactive()";
            const SELECTOR: [u8; 4] = [84u8, 13u8, 98u8, 132u8];
            #[inline]
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
    /**Event with signature `AVSRewardsSubmissionCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e6281`.
```solidity
event AVSRewardsSubmissionCreated(address indexed avs, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
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
        pub rewardsSubmission: <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IRewardsCoordinatorTypes::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "AVSRewardsSubmissionCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                69u8,
                10u8,
                54u8,
                122u8,
                56u8,
                12u8,
                78u8,
                51u8,
                158u8,
                90u8,
                231u8,
                52u8,
                12u8,
                132u8,
                100u8,
                239u8,
                39u8,
                175u8,
                119u8,
                129u8,
                173u8,
                153u8,
                69u8,
                207u8,
                232u8,
                171u8,
                216u8,
                40u8,
                248u8,
                158u8,
                98u8,
                129u8,
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
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &AVSRewardsSubmissionCreated,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ActivationDelaySet(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                85u8,
                124u8,
                108u8,
                2u8,
                194u8,
                8u8,
                121u8,
                72u8,
                23u8,
                167u8,
                5u8,
                96u8,
                156u8,
                250u8,
                147u8,
                95u8,
                130u8,
                115u8,
                18u8,
                161u8,
                173u8,
                253u8,
                210u8,
                100u8,
                148u8,
                182u8,
                185u8,
                93u8,
                210u8,
                180u8,
                179u8,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldActivationDelay),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.newActivationDelay),
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ClaimerForSet(address,address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                186u8,
                185u8,
                71u8,
                147u8,
                77u8,
                66u8,
                224u8,
                173u8,
                32u8,
                111u8,
                37u8,
                201u8,
                202u8,
                177u8,
                139u8,
                91u8,
                182u8,
                174u8,
                20u8,
                74u8,
                207u8,
                176u8,
                15u8,
                64u8,
                180u8,
                227u8,
                170u8,
                89u8,
                89u8,
                12u8,
                163u8,
                18u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DefaultOperatorSplitBipsSet(uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                230u8,
                205u8,
                78u8,
                223u8,
                220u8,
                193u8,
                246u8,
                209u8,
                48u8,
                171u8,
                53u8,
                247u8,
                61u8,
                114u8,
                55u8,
                143u8,
                58u8,
                100u8,
                41u8,
                68u8,
                251u8,
                78u8,
                229u8,
                189u8,
                132u8,
                183u8,
                128u8,
                122u8,
                129u8,
                234u8,
                28u8,
                78u8,
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
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.oldDefaultOperatorSplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &DefaultOperatorSplitBipsSet,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "DistributionRootDisabled(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                216u8,
                80u8,
                230u8,
                229u8,
                223u8,
                164u8,
                151u8,
                183u8,
                38u8,
                97u8,
                250u8,
                115u8,
                223u8,
                41u8,
                35u8,
                70u8,
                78u8,
                174u8,
                217u8,
                220u8,
                47u8,
                241u8,
                211u8,
                203u8,
                130u8,
                188u8,
                203u8,
                254u8,
                171u8,
                229u8,
                196u8,
                30u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { rootIndex: topics.1 }
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &DistributionRootDisabled,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "DistributionRootSubmitted(uint32,bytes32,uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                236u8,
                216u8,
                102u8,
                195u8,
                193u8,
                88u8,
                250u8,
                0u8,
                191u8,
                52u8,
                216u8,
                3u8,
                213u8,
                246u8,
                2u8,
                48u8,
                0u8,
                181u8,
                112u8,
                128u8,
                188u8,
                180u8,
                138u8,
                240u8,
                4u8,
                194u8,
                180u8,
                180u8,
                107u8,
                58u8,
                253u8,
                8u8,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.activatedAt),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &DistributionRootSubmitted,
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorAVSSplitBipsSet(address,address,address,uint32,uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                72u8,
                225u8,
                152u8,
                182u8,
                174u8,
                53u8,
                126u8,
                82u8,
                146u8,
                4u8,
                238u8,
                83u8,
                168u8,
                229u8,
                20u8,
                196u8,
                112u8,
                255u8,
                119u8,
                217u8,
                204u8,
                142u8,
                79u8,
                114u8,
                7u8,
                248u8,
                181u8,
                212u8,
                144u8,
                174u8,
                105u8,
                52u8,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.activatedAt),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.oldOperatorAVSSplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &OperatorAVSSplitBipsSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorDirectedAVSRewardsSubmissionCreated(address,address,bytes32,uint256,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string))` and selector `0xfc8888bffd711da60bc5092b33f677d81896fe80ecc677b84cfab8184462b6e0`.
```solidity
event OperatorDirectedAVSRewardsSubmissionCreated(address indexed caller, address indexed avs, bytes32 indexed operatorDirectedRewardsSubmissionHash, uint256 submissionNonce, IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission operatorDirectedRewardsSubmission);
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
        pub operatorDirectedRewardsSubmission: <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
                IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorDirectedAVSRewardsSubmissionCreated(address,address,bytes32,uint256,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                252u8,
                136u8,
                136u8,
                191u8,
                253u8,
                113u8,
                29u8,
                166u8,
                11u8,
                197u8,
                9u8,
                43u8,
                51u8,
                246u8,
                119u8,
                216u8,
                24u8,
                150u8,
                254u8,
                128u8,
                236u8,
                198u8,
                119u8,
                184u8,
                76u8,
                250u8,
                184u8,
                24u8,
                68u8,
                98u8,
                182u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.submissionNonce),
                    <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
        impl alloy_sol_types::private::IntoLogData
        for OperatorDirectedAVSRewardsSubmissionCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorDirectedAVSRewardsSubmissionCreated>
        for alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorPISplitBipsSet(address,address,uint32,uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                209u8,
                224u8,
                40u8,
                189u8,
                102u8,
                68u8,
                134u8,
                164u8,
                106u8,
                210u8,
                96u8,
                64u8,
                233u8,
                153u8,
                205u8,
                45u8,
                34u8,
                225u8,
                233u8,
                160u8,
                148u8,
                238u8,
                106u8,
                254u8,
                25u8,
                252u8,
                246u8,
                70u8,
                120u8,
                241u8,
                111u8,
                116u8,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.activatedAt),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.oldOperatorPISplitBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newOperatorPISplitBips,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.caller.clone(), self.operator.clone())
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RewardsClaimed(bytes32,address,address,address,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                149u8,
                67u8,
                219u8,
                213u8,
                85u8,
                128u8,
                132u8,
                37u8,
                134u8,
                169u8,
                81u8,
                240u8,
                56u8,
                110u8,
                36u8,
                214u8,
                138u8,
                93u8,
                249u8,
                154u8,
                226u8,
                158u8,
                59u8,
                33u8,
                101u8,
                136u8,
                180u8,
                95u8,
                214u8,
                132u8,
                206u8,
                49u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            const SIGNATURE: &'static str = "RewardsForAllSubmitterSet(address,bool,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                77u8,
                230u8,
                41u8,
                62u8,
                102u8,
                141u8,
                241u8,
                57u8,
                132u8,
                34u8,
                225u8,
                222u8,
                241u8,
                33u8,
                24u8,
                5u8,
                44u8,
                21u8,
                57u8,
                160u8,
                60u8,
                191u8,
                237u8,
                193u8,
                69u8,
                137u8,
                93u8,
                72u8,
                215u8,
                104u8,
                95u8,
                28u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.rewardsForAllSubmitter,
                );
                out[2usize] = <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
                    &self.oldValue,
                );
                out[3usize] = <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic(
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
            fn from(
                this: &RewardsForAllSubmitterSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsSubmissionForAllCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf270482`.
```solidity
event RewardsSubmissionForAllCreated(address indexed submitter, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
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
        pub rewardsSubmission: <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IRewardsCoordinatorTypes::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RewardsSubmissionForAllCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                81u8,
                8u8,
                139u8,
                140u8,
                137u8,
                98u8,
                141u8,
                243u8,
                168u8,
                23u8,
                64u8,
                2u8,
                194u8,
                160u8,
                52u8,
                208u8,
                21u8,
                47u8,
                206u8,
                106u8,
                248u8,
                65u8,
                93u8,
                101u8,
                27u8,
                42u8,
                71u8,
                52u8,
                191u8,
                39u8,
                4u8,
                130u8,
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
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
        impl From<&RewardsSubmissionForAllCreated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RewardsSubmissionForAllCreated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RewardsSubmissionForAllEarnersCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))` and selector `0x5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b`.
```solidity
event RewardsSubmissionForAllEarnersCreated(address indexed tokenHopper, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
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
        pub rewardsSubmission: <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            type DataTuple<'a> = (IRewardsCoordinatorTypes::RewardsSubmission,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "RewardsSubmissionForAllEarnersCreated(address,uint256,bytes32,((address,uint96)[],address,uint256,uint32,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                82u8,
                81u8,
                182u8,
                253u8,
                239u8,
                203u8,
                93u8,
                129u8,
                20u8,
                78u8,
                115u8,
                95u8,
                105u8,
                234u8,
                76u8,
                105u8,
                95u8,
                212u8,
                59u8,
                2u8,
                137u8,
                202u8,
                83u8,
                220u8,
                7u8,
                80u8,
                51u8,
                245u8,
                252u8,
                128u8,
                6u8,
                139u8,
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
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
        impl alloy_sol_types::private::IntoLogData
        for RewardsSubmissionForAllEarnersCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RewardsSubmissionForAllEarnersCreated>
        for alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RewardsUpdaterSet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                123u8,
                130u8,
                244u8,
                56u8,
                215u8,
                95u8,
                197u8,
                104u8,
                235u8,
                171u8,
                72u8,
                75u8,
                117u8,
                176u8,
                29u8,
                146u8,
                135u8,
                185u8,
                233u8,
                139u8,
                73u8,
                11u8,
                124u8,
                35u8,
                34u8,
                22u8,
                35u8,
                182u8,
                112u8,
                93u8,
                187u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
constructor(address _delegationManager, address _strategyManager, address _allocationManager, address _pauserRegistry, address _permissionController, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 _GENESIS_REWARDS_TIMESTAMP);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
        pub _strategyManager: alloy::sol_types::private::Address,
        pub _allocationManager: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub _permissionController: alloy::sol_types::private::Address,
        pub _CALCULATION_INTERVAL_SECONDS: u32,
        pub _MAX_REWARDS_DURATION: u32,
        pub _MAX_RETROACTIVE_LENGTH: u32,
        pub _MAX_FUTURE_LENGTH: u32,
        pub _GENESIS_REWARDS_TIMESTAMP: u32,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
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
                u32,
                u32,
                u32,
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
                        value._delegationManager,
                        value._strategyManager,
                        value._allocationManager,
                        value._pauserRegistry,
                        value._permissionController,
                        value._CALCULATION_INTERVAL_SECONDS,
                        value._MAX_REWARDS_DURATION,
                        value._MAX_RETROACTIVE_LENGTH,
                        value._MAX_FUTURE_LENGTH,
                        value._GENESIS_REWARDS_TIMESTAMP,
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
                        _allocationManager: tuple.2,
                        _pauserRegistry: tuple.3,
                        _permissionController: tuple.4,
                        _CALCULATION_INTERVAL_SECONDS: tuple.5,
                        _MAX_REWARDS_DURATION: tuple.6,
                        _MAX_RETROACTIVE_LENGTH: tuple.7,
                        _MAX_FUTURE_LENGTH: tuple.8,
                        _GENESIS_REWARDS_TIMESTAMP: tuple.9,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
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
                        &self._delegationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._strategyManager,
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._CALCULATION_INTERVAL_SECONDS,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_REWARDS_DURATION,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_RETROACTIVE_LENGTH,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._MAX_FUTURE_LENGTH),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._GENESIS_REWARDS_TIMESTAMP,
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
            impl ::core::convert::From<CALCULATION_INTERVAL_SECONDSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: CALCULATION_INTERVAL_SECONDSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CALCULATION_INTERVAL_SECONDSCall {
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
            impl ::core::convert::From<CALCULATION_INTERVAL_SECONDSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: CALCULATION_INTERVAL_SECONDSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CALCULATION_INTERVAL_SECONDSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CALCULATION_INTERVAL_SECONDSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CALCULATION_INTERVAL_SECONDSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<GENESIS_REWARDS_TIMESTAMPCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_REWARDS_TIMESTAMPCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GENESIS_REWARDS_TIMESTAMPCall {
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
            impl ::core::convert::From<GENESIS_REWARDS_TIMESTAMPReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_REWARDS_TIMESTAMPReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for GENESIS_REWARDS_TIMESTAMPReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENESIS_REWARDS_TIMESTAMPCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = GENESIS_REWARDS_TIMESTAMPReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<MAX_FUTURE_LENGTHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_FUTURE_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_FUTURE_LENGTHCall {
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
            impl ::core::convert::From<MAX_FUTURE_LENGTHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_FUTURE_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_FUTURE_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_FUTURE_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_FUTURE_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<MAX_RETROACTIVE_LENGTHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RETROACTIVE_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_RETROACTIVE_LENGTHCall {
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
            impl ::core::convert::From<MAX_RETROACTIVE_LENGTHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RETROACTIVE_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_RETROACTIVE_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_RETROACTIVE_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_RETROACTIVE_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<MAX_REWARDS_DURATIONCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_REWARDS_DURATIONCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_REWARDS_DURATIONCall {
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
            impl ::core::convert::From<MAX_REWARDS_DURATIONReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_REWARDS_DURATIONReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_REWARDS_DURATIONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_REWARDS_DURATIONCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_REWARDS_DURATIONReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<activationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: activationDelayReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for activationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activationDelayCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = activationDelayReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `calculateEarnerLeafHash((address,bytes32))` and selector `0x149bc872`.
```solidity
function calculateEarnerLeafHash(IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEarnerLeafHashCall {
        pub leaf: <IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateEarnerLeafHash((address,bytes32))`](calculateEarnerLeafHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateEarnerLeafHashReturn {
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
                IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateEarnerLeafHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateEarnerLeafHashCall) -> Self {
                    (value.leaf,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateEarnerLeafHashCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateEarnerLeafHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateEarnerLeafHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateEarnerLeafHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateEarnerLeafHashCall {
            type Parameters<'a> = (IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateEarnerLeafHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf as alloy_sol_types::SolType>::tokenize(
                        &self.leaf,
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
    /**Function with signature `calculateTokenLeafHash((address,uint256))` and selector `0xf8cd8448`.
```solidity
function calculateTokenLeafHash(IRewardsCoordinatorTypes.TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateTokenLeafHashCall {
        pub leaf: <IRewardsCoordinatorTypes::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`calculateTokenLeafHash((address,uint256))`](calculateTokenLeafHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateTokenLeafHashReturn {
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
                IRewardsCoordinatorTypes::TokenTreeMerkleLeaf,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<calculateTokenLeafHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateTokenLeafHashCall) -> Self {
                    (value.leaf,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateTokenLeafHashCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateTokenLeafHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateTokenLeafHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateTokenLeafHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateTokenLeafHashCall {
            type Parameters<'a> = (IRewardsCoordinatorTypes::TokenTreeMerkleLeaf,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateTokenLeafHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <IRewardsCoordinatorTypes::TokenTreeMerkleLeaf as alloy_sol_types::SolType>::tokenize(
                        &self.leaf,
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
    /**Function with signature `checkClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]))` and selector `0x5e9d8348`.
```solidity
function checkClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkClaimCall {
        pub claim: <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]))`](checkClaimCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkClaimReturn {
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
                IRewardsCoordinatorTypes::RewardsMerkleClaim,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Parameters<'a> = (IRewardsCoordinatorTypes::RewardsMerkleClaim,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkClaimReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy_sol_types::SolType>::tokenize(
                        &self.claim,
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
    /**Function with signature `claimerFor(address)` and selector `0x2b9f64a4`.
```solidity
function claimerFor(address earner) external view returns (address claimer);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForCall {
        pub earner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`claimerFor(address)`](claimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForReturn {
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
            impl ::core::convert::From<claimerForCall> for UnderlyingRustTuple<'_> {
                fn from(value: claimerForCall) -> Self {
                    (value.earner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimerForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { earner: tuple.0 }
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
            impl ::core::convert::From<claimerForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: claimerForReturn) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for claimerForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { claimer: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimerForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimerForReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.earner,
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
    /**Function with signature `createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xfce36c7d`.
```solidity
function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createAVSRewardsSubmissionCall {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createAVSRewardsSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createAVSRewardsSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [252u8, 227u8, 108u8, 125u8];
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
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
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
    /**Function with signature `createOperatorDirectedAVSRewardsSubmission(address,((address,uint96)[],address,(address,uint256)[],uint32,uint32,string)[])` and selector `0x9cb9a5fa`.
```solidity
function createOperatorDirectedAVSRewardsSubmission(address avs, IRewardsCoordinatorTypes.OperatorDirectedRewardsSubmission[] memory operatorDirectedRewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorDirectedAVSRewardsSubmissionCall {
        pub avs: alloy::sol_types::private::Address,
        pub operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorDirectedAVSRewardsSubmissionCall) -> Self {
                    (value.avs, value.operatorDirectedRewardsSubmissions)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorDirectedAVSRewardsSubmissionCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorDirectedAVSRewardsSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: createOperatorDirectedAVSRewardsSubmissionReturn,
                ) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorDirectedAVSRewardsSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for createOperatorDirectedAVSRewardsSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorDirectedAVSRewardsSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission,
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `createRewardsForAllEarners(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0xff9f6cce`.
```solidity
function createRewardsForAllEarners(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllEarnersCall {
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createRewardsForAllEarnersCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllEarnersCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createRewardsForAllEarnersCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createRewardsForAllEarnersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllEarnersReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createRewardsForAllEarnersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createRewardsForAllEarnersCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createRewardsForAllEarnersReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createRewardsForAllEarners(((address,uint96)[],address,uint256,uint32,uint32)[])";
            const SELECTOR: [u8; 4] = [255u8, 159u8, 108u8, 206u8];
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
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
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
    /**Function with signature `createRewardsForAllSubmission(((address,uint96)[],address,uint256,uint32,uint32)[])` and selector `0x36af41fa`.
```solidity
function createRewardsForAllSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createRewardsForAllSubmissionCall {
        pub rewardsSubmissions: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createRewardsForAllSubmissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllSubmissionCall) -> Self {
                    (value.rewardsSubmissions,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createRewardsForAllSubmissionCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createRewardsForAllSubmissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createRewardsForAllSubmissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createRewardsForAllSubmissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createRewardsForAllSubmissionCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsSubmission,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createRewardsForAllSubmissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (
                    <alloy::sol_types::sol_data::Array<
                        IRewardsCoordinatorTypes::RewardsSubmission,
                    > as alloy_sol_types::SolType>::tokenize(&self.rewardsSubmissions),
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
    /**Function with signature `cumulativeClaimed(address,address)` and selector `0x865c6953`.
```solidity
function cumulativeClaimed(address earner, address token) external view returns (uint256 totalClaimed);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedCall {
        pub earner: alloy::sol_types::private::Address,
        pub token: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeClaimed(address,address)`](cumulativeClaimedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedReturn {
        pub totalClaimed: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
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
            impl ::core::convert::From<cumulativeClaimedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedCall) -> Self {
                    (value.earner, value.token)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeClaimedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        earner: tuple.0,
                        token: tuple.1,
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
            impl ::core::convert::From<cumulativeClaimedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedReturn) -> Self {
                    (value.totalClaimed,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeClaimedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { totalClaimed: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cumulativeClaimedCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cumulativeClaimedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.earner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.token,
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
            impl ::core::convert::From<currRewardsCalculationEndTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currRewardsCalculationEndTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currRewardsCalculationEndTimestampCall {
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
            impl ::core::convert::From<currRewardsCalculationEndTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currRewardsCalculationEndTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currRewardsCalculationEndTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currRewardsCalculationEndTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currRewardsCalculationEndTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defaultOperatorSplitBipsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: defaultOperatorSplitBipsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defaultOperatorSplitBipsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<defaultOperatorSplitBipsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: defaultOperatorSplitBipsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for defaultOperatorSplitBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for defaultOperatorSplitBipsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = defaultOperatorSplitBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<delegationManagerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerCall {
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
            impl ::core::convert::From<delegationManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delegationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delegationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = disableRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.rootIndex),
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
    /**Function with signature `getCurrentClaimableDistributionRoot()` and selector `0x0e9a53cf`.
```solidity
function getCurrentClaimableDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentClaimableDistributionRootCall {}
    ///Container type for the return parameters of the [`getCurrentClaimableDistributionRoot()`](getCurrentClaimableDistributionRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentClaimableDistributionRootReturn {
        pub _0: <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getCurrentClaimableDistributionRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentClaimableDistributionRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentClaimableDistributionRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentClaimableDistributionRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentClaimableDistributionRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentClaimableDistributionRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentClaimableDistributionRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentClaimableDistributionRootReturn;
            type ReturnTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentDistributionRoot()` and selector `0x9be3d4e4`.
```solidity
function getCurrentDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDistributionRootCall {}
    ///Container type for the return parameters of the [`getCurrentDistributionRoot()`](getCurrentDistributionRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDistributionRootReturn {
        pub _0: <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getCurrentDistributionRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentDistributionRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentDistributionRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentDistributionRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentDistributionRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentDistributionRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentDistributionRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentDistributionRootReturn;
            type ReturnTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getDistributionRootAtIndex(uint256)` and selector `0xde02e503`.
```solidity
function getDistributionRootAtIndex(uint256 index) external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootAtIndexCall {
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getDistributionRootAtIndex(uint256)`](getDistributionRootAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getDistributionRootAtIndexReturn {
        pub _0: <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
    }
    #[allow(
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
            impl ::core::convert::From<getDistributionRootAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootAtIndexCall) -> Self {
                    (value.index,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDistributionRootAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { index: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::DistributionRoot as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getDistributionRootAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDistributionRootAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDistributionRootAtIndexCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDistributionRootAtIndexReturn;
            type ReturnTuple<'a> = (IRewardsCoordinatorTypes::DistributionRoot,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
            impl ::core::convert::From<getDistributionRootsLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootsLengthCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDistributionRootsLengthCall {
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
            impl ::core::convert::From<getDistributionRootsLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getDistributionRootsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getDistributionRootsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getDistributionRootsLengthCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getDistributionRootsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorAVSSplit(address,address)`](getOperatorAVSSplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorAVSSplitReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorAVSSplitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorAVSSplitCall) -> Self {
                    (value.operator, value.avs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorAVSSplitCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorAVSSplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorAVSSplitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorAVSSplitReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorAVSSplitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorPISplit(address)`](getOperatorPISplitCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorPISplitReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorPISplitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorPISplitCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorPISplitCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getOperatorPISplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorPISplitReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorPISplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorPISplitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorPISplitReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub rootHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`getRootIndexFromHash(bytes32)`](getRootIndexFromHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRootIndexFromHashReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRootIndexFromHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRootIndexFromHashCall) -> Self {
                    (value.rootHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRootIndexFromHashCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getRootIndexFromHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRootIndexFromHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRootIndexFromHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRootIndexFromHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRootIndexFromHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize(address,uint256,address,uint32,uint16)` and selector `0xf6efbb59`.
```solidity
function initialize(address initialOwner, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _defaultSplitBips) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _rewardsUpdater: alloy::sol_types::private::Address,
        pub _activationDelay: u32,
        pub _defaultSplitBips: u16,
    }
    ///Container type for the return parameters of the [`initialize(address,uint256,address,uint32,uint16)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Address,
                u32,
                u16,
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
                        initialPausedStatus: tuple.1,
                        _rewardsUpdater: tuple.2,
                        _activationDelay: tuple.3,
                        _defaultSplitBips: tuple.4,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address,uint256,address,uint32,uint16)";
            const SELECTOR: [u8; 4] = [246u8, 239u8, 187u8, 89u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._rewardsUpdater,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._activationDelay),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self._defaultSplitBips),
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
    /**Function with signature `isAVSRewardsSubmissionHash(address,bytes32)` and selector `0x6d21117e`.
```solidity
function isAVSRewardsSubmissionHash(address avs, bytes32 hash) external view returns (bool valid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashCall {
        pub avs: alloy::sol_types::private::Address,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isAVSRewardsSubmissionHash(address,bytes32)`](isAVSRewardsSubmissionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashReturn {
        pub valid: bool,
    }
    #[allow(
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashCall) -> Self {
                    (value.avs, value.hash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isAVSRewardsSubmissionHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        hash: tuple.1,
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashReturn) -> Self {
                    (value.valid,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isAVSRewardsSubmissionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { valid: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isAVSRewardsSubmissionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isAVSRewardsSubmissionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
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
    /**Function with signature `isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)` and selector `0xed71e6a2`.
```solidity
function isOperatorDirectedAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorDirectedAVSRewardsSubmissionHashCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)`](isOperatorDirectedAVSRewardsSubmissionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorDirectedAVSRewardsSubmissionHashReturn {
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
            impl ::core::convert::From<isOperatorDirectedAVSRewardsSubmissionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorDirectedAVSRewardsSubmissionHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorDirectedAVSRewardsSubmissionHashCall {
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
            impl ::core::convert::From<isOperatorDirectedAVSRewardsSubmissionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: isOperatorDirectedAVSRewardsSubmissionHashReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorDirectedAVSRewardsSubmissionHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for isOperatorDirectedAVSRewardsSubmissionHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorDirectedAVSRewardsSubmissionHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorDirectedAVSRewardsSubmissionHash(address,bytes32)";
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isRewardsForAllSubmitter(address)` and selector `0x0018572c`.
```solidity
function isRewardsForAllSubmitter(address submitter) external view returns (bool valid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterCall {
        pub submitter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isRewardsForAllSubmitter(address)`](isRewardsForAllSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterReturn {
        pub valid: bool,
    }
    #[allow(
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
            impl ::core::convert::From<isRewardsForAllSubmitterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterCall) -> Self {
                    (value.submitter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsForAllSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { submitter: tuple.0 }
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
            impl ::core::convert::From<isRewardsForAllSubmitterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterReturn) -> Self {
                    (value.valid,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsForAllSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { valid: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsForAllSubmitterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsForAllSubmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.submitter,
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
    /**Function with signature `isRewardsSubmissionForAllEarnersHash(address,bytes32)` and selector `0xaebd8bae`.
```solidity
function isRewardsSubmissionForAllEarnersHash(address avs, bytes32 hash) external view returns (bool valid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashCall {
        pub avs: alloy::sol_types::private::Address,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllEarnersHash(address,bytes32)`](isRewardsSubmissionForAllEarnersHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashReturn {
        pub valid: bool,
    }
    #[allow(
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashCall) -> Self {
                    (value.avs, value.hash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllEarnersHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        hash: tuple.1,
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashReturn) -> Self {
                    (value.valid,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllEarnersHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { valid: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsSubmissionForAllEarnersHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsSubmissionForAllEarnersHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
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
    /**Function with signature `isRewardsSubmissionForAllHash(address,bytes32)` and selector `0xc46db606`.
```solidity
function isRewardsSubmissionForAllHash(address avs, bytes32 hash) external view returns (bool valid);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashCall {
        pub avs: alloy::sol_types::private::Address,
        pub hash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllHash(address,bytes32)`](isRewardsSubmissionForAllHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashReturn {
        pub valid: bool,
    }
    #[allow(
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashCall) -> Self {
                    (value.avs, value.hash)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        hash: tuple.1,
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashReturn) -> Self {
                    (value.valid,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { valid: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isRewardsSubmissionForAllHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isRewardsSubmissionForAllHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hash),
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
    /**Function with signature `processClaim((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[]),address)` and selector `0x3ccc861d`.
```solidity
function processClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim, address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimCall {
        pub claim: <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
                IRewardsCoordinatorTypes::RewardsMerkleClaim,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
                IRewardsCoordinatorTypes::RewardsMerkleClaim,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processClaimReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy_sol_types::SolType>::tokenize(
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `processClaims((uint32,uint32,bytes,(address,bytes32),uint32[],bytes[],(address,uint256)[])[],address)` and selector `0x4596021c`.
```solidity
function processClaims(IRewardsCoordinatorTypes.RewardsMerkleClaim[] memory claims, address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processClaimsCall {
        pub claims: alloy::sol_types::private::Vec<
            <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
        >,
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
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsMerkleClaim,
                >,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
                alloy::sol_types::sol_data::Array<
                    IRewardsCoordinatorTypes::RewardsMerkleClaim,
                >,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processClaimsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        IRewardsCoordinatorTypes::RewardsMerkleClaim,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<rewardsUpdaterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: rewardsUpdaterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for rewardsUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for rewardsUpdaterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = rewardsUpdaterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setActivationDelayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setActivationDelayCall) -> Self {
                    (value._activationDelay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setActivationDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _activationDelay: tuple.0 }
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
            impl ::core::convert::From<setActivationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setActivationDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setActivationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setActivationDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setActivationDelayReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._activationDelay),
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
    /**Function with signature `setClaimerFor(address)` and selector `0xa0169ddd`.
```solidity
function setClaimerFor(address claimer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerFor_0Call {
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address)`](setClaimerFor_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerFor_0Return {}
    #[allow(
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
            impl ::core::convert::From<setClaimerFor_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerFor_0Call) -> Self {
                    (value.claimer,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerFor_0Call {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setClaimerFor_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerFor_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setClaimerFor_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerFor_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerFor_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setClaimerFor(address,address)` and selector `0xf22cef85`.
```solidity
function setClaimerFor(address earner, address claimer) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerFor_1Call {
        pub earner: alloy::sol_types::private::Address,
        pub claimer: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setClaimerFor(address,address)`](setClaimerFor_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setClaimerFor_1Return {}
    #[allow(
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
            impl ::core::convert::From<setClaimerFor_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerFor_1Call) -> Self {
                    (value.earner, value.claimer)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setClaimerFor_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        earner: tuple.0,
                        claimer: tuple.1,
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
            impl ::core::convert::From<setClaimerFor_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setClaimerFor_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setClaimerFor_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setClaimerFor_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerFor_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setClaimerFor(address,address)";
            const SELECTOR: [u8; 4] = [242u8, 44u8, 239u8, 133u8];
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
                        &self.earner,
                    ),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setDefaultOperatorSplitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDefaultOperatorSplitCall) -> Self {
                    (value.split,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDefaultOperatorSplitCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setDefaultOperatorSplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setDefaultOperatorSplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setDefaultOperatorSplitReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setDefaultOperatorSplitCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setDefaultOperatorSplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.split),
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
    /**Function with signature `setOperatorAVSSplit(address,address,uint16)` and selector `0xdcbb03b3`.
```solidity
function setOperatorAVSSplit(address operator, address avs, uint16 split) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorAVSSplitCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorAVSSplitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorAVSSplitCall) -> Self {
                    (value.operator, value.avs, value.split)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorAVSSplitCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorAVSSplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorAVSSplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorAVSSplitReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorAVSSplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.split),
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
    /**Function with signature `setOperatorPISplit(address,uint16)` and selector `0xb3dbb0e0`.
```solidity
function setOperatorPISplit(address operator, uint16 split) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setOperatorPISplitCall {
        pub operator: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorPISplitCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorPISplitCall) -> Self {
                    (value.operator, value.split)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorPISplitCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorPISplitReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorPISplitReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setOperatorPISplitReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorPISplitReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(&self.split),
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
    /**Function with signature `setRewardsForAllSubmitter(address,bool)` and selector `0x0eb38345`.
```solidity
function setRewardsForAllSubmitter(address _submitter, bool _newValue) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setRewardsForAllSubmitterCall {
        pub _submitter: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsForAllSubmitterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsForAllSubmitterCall) -> Self {
                    (value._submitter, value._newValue)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsForAllSubmitterCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsForAllSubmitterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsForAllSubmitterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsForAllSubmitterReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsForAllSubmitterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setRewardsUpdaterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsUpdaterCall) -> Self {
                    (value._rewardsUpdater,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsUpdaterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _rewardsUpdater: tuple.0 }
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
            impl ::core::convert::From<setRewardsUpdaterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setRewardsUpdaterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setRewardsUpdaterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setRewardsUpdaterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setRewardsUpdaterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `submissionNonce(address)` and selector `0xbb7e451f`.
```solidity
function submissionNonce(address avs) external view returns (uint256 nonce);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceCall {
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`submissionNonce(address)`](submissionNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceReturn {
        pub nonce: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(
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
            impl ::core::convert::From<submissionNonceCall> for UnderlyingRustTuple<'_> {
                fn from(value: submissionNonceCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for submissionNonceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { avs: tuple.0 }
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
            impl ::core::convert::From<submissionNonceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: submissionNonceReturn) -> Self {
                    (value.nonce,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submissionNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { nonce: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for submissionNonceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submissionNonceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                        &self.avs,
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
    /**Function with signature `submitRoot(bytes32,uint32)` and selector `0x3efe1db6`.
```solidity
function submitRoot(bytes32 root, uint32 rewardsCalculationEndTimestamp) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submitRootCall {
        pub root: alloy::sol_types::private::FixedBytes<32>,
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
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = submitRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    ///Container for all the [`RewardsCoordinator`](self) function calls.
    pub enum RewardsCoordinatorCalls {
        CALCULATION_INTERVAL_SECONDS(CALCULATION_INTERVAL_SECONDSCall),
        GENESIS_REWARDS_TIMESTAMP(GENESIS_REWARDS_TIMESTAMPCall),
        MAX_FUTURE_LENGTH(MAX_FUTURE_LENGTHCall),
        MAX_RETROACTIVE_LENGTH(MAX_RETROACTIVE_LENGTHCall),
        MAX_REWARDS_DURATION(MAX_REWARDS_DURATIONCall),
        activationDelay(activationDelayCall),
        allocationManager(allocationManagerCall),
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        calculateEarnerLeafHash(calculateEarnerLeafHashCall),
        calculateTokenLeafHash(calculateTokenLeafHashCall),
        checkClaim(checkClaimCall),
        claimerFor(claimerForCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createOperatorDirectedAVSRewardsSubmission(
            createOperatorDirectedAVSRewardsSubmissionCall,
        ),
        createRewardsForAllEarners(createRewardsForAllEarnersCall),
        createRewardsForAllSubmission(createRewardsForAllSubmissionCall),
        cumulativeClaimed(cumulativeClaimedCall),
        currRewardsCalculationEndTimestamp(currRewardsCalculationEndTimestampCall),
        defaultOperatorSplitBips(defaultOperatorSplitBipsCall),
        delegationManager(delegationManagerCall),
        disableRoot(disableRootCall),
        getCurrentClaimableDistributionRoot(getCurrentClaimableDistributionRootCall),
        getCurrentDistributionRoot(getCurrentDistributionRootCall),
        getDistributionRootAtIndex(getDistributionRootAtIndexCall),
        getDistributionRootsLength(getDistributionRootsLengthCall),
        getOperatorAVSSplit(getOperatorAVSSplitCall),
        getOperatorPISplit(getOperatorPISplitCall),
        getRootIndexFromHash(getRootIndexFromHashCall),
        initialize(initializeCall),
        isAVSRewardsSubmissionHash(isAVSRewardsSubmissionHashCall),
        isOperatorDirectedAVSRewardsSubmissionHash(
            isOperatorDirectedAVSRewardsSubmissionHashCall,
        ),
        isRewardsForAllSubmitter(isRewardsForAllSubmitterCall),
        isRewardsSubmissionForAllEarnersHash(isRewardsSubmissionForAllEarnersHashCall),
        isRewardsSubmissionForAllHash(isRewardsSubmissionForAllHashCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        permissionController(permissionControllerCall),
        processClaim(processClaimCall),
        processClaims(processClaimsCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsUpdater(rewardsUpdaterCall),
        setActivationDelay(setActivationDelayCall),
        setClaimerFor_0(setClaimerFor_0Call),
        setClaimerFor_1(setClaimerFor_1Call),
        setDefaultOperatorSplit(setDefaultOperatorSplitCall),
        setOperatorAVSSplit(setOperatorAVSSplitCall),
        setOperatorPISplit(setOperatorPISplitCall),
        setRewardsForAllSubmitter(setRewardsForAllSubmitterCall),
        setRewardsUpdater(setRewardsUpdaterCall),
        strategyManager(strategyManagerCall),
        submissionNonce(submissionNonceCall),
        submitRoot(submitRootCall),
        transferOwnership(transferOwnershipCall),
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
            [70u8, 87u8, 226u8, 106u8],
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
            [202u8, 138u8, 167u8, 199u8],
            [220u8, 187u8, 3u8, 179u8],
            [222u8, 2u8, 229u8, 3u8],
            [224u8, 99u8, 248u8, 31u8],
            [232u8, 16u8, 206u8, 33u8],
            [234u8, 77u8, 60u8, 155u8],
            [237u8, 113u8, 230u8, 162u8],
            [242u8, 44u8, 239u8, 133u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 239u8, 187u8, 89u8],
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
        const COUNT: usize = 58usize;
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
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setClaimerFor_0(_) => {
                    <setClaimerFor_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setClaimerFor_1(_) => {
                    <setClaimerFor_1Call as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<RewardsCoordinatorCalls>] = &[
                {
                    fn isRewardsForAllSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <isRewardsForAllSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::setRewardsForAllSubmitter)
                    }
                    setRewardsForAllSubmitter
                },
                {
                    fn GENESIS_REWARDS_TIMESTAMP(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <GENESIS_REWARDS_TIMESTAMPCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                        <claimerForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <submitRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::processClaims)
                    }
                    processClaims
                },
                {
                    fn permissionController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::permissionController)
                    }
                    permissionController
                },
                {
                    fn getOperatorPISplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <getOperatorPISplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn checkClaim(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <checkClaimCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                    fn setClaimerFor_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setClaimerFor_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::setClaimerFor_0)
                    }
                    setClaimerFor_0
                },
                {
                    fn setDefaultOperatorSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setDefaultOperatorSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn setOperatorAVSSplit(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setOperatorAVSSplitCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                    fn setClaimerFor_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setClaimerFor_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::setClaimerFor_1)
                    }
                    setClaimerFor_1
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::initialize)
                    }
                    initialize
                },
                {
                    fn calculateTokenLeafHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <calculateTokenLeafHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setClaimerFor_0(inner) => {
                    <setClaimerFor_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setClaimerFor_1(inner) => {
                    <setClaimerFor_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setClaimerFor_0(inner) => {
                    <setClaimerFor_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setClaimerFor_1(inner) => {
                    <setClaimerFor_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`RewardsCoordinator`](self) custom errors.
    pub enum RewardsCoordinatorErrors {
        AmountExceedsMax(AmountExceedsMax),
        AmountIsZero(AmountIsZero),
        CurrentlyPaused(CurrentlyPaused),
        DurationExceedsMax(DurationExceedsMax),
        EarningsNotGreaterThanClaimed(EarningsNotGreaterThanClaimed),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InputArrayLengthZero(InputArrayLengthZero),
        InvalidAddressZero(InvalidAddressZero),
        InvalidCalculationIntervalSecondsRemainder(
            InvalidCalculationIntervalSecondsRemainder,
        ),
        InvalidClaimProof(InvalidClaimProof),
        InvalidDurationRemainder(InvalidDurationRemainder),
        InvalidEarner(InvalidEarner),
        InvalidEarnerLeafIndex(InvalidEarnerLeafIndex),
        InvalidGenesisRewardsTimestampRemainder(InvalidGenesisRewardsTimestampRemainder),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidPermissions(InvalidPermissions),
        InvalidProofLength(InvalidProofLength),
        InvalidRoot(InvalidRoot),
        InvalidRootIndex(InvalidRootIndex),
        InvalidStartTimestampRemainder(InvalidStartTimestampRemainder),
        InvalidTokenLeafIndex(InvalidTokenLeafIndex),
        NewRootMustBeForNewCalculatedPeriod(NewRootMustBeForNewCalculatedPeriod),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorsNotInAscendingOrder(OperatorsNotInAscendingOrder),
        PreviousSplitPending(PreviousSplitPending),
        RewardsEndTimestampNotElapsed(RewardsEndTimestampNotElapsed),
        RootAlreadyActivated(RootAlreadyActivated),
        RootDisabled(RootDisabled),
        RootNotActivated(RootNotActivated),
        SplitExceedsMax(SplitExceedsMax),
        StartTimestampTooFarInFuture(StartTimestampTooFarInFuture),
        StartTimestampTooFarInPast(StartTimestampTooFarInPast),
        StrategiesNotInAscendingOrder(StrategiesNotInAscendingOrder),
        StrategyNotWhitelisted(StrategyNotWhitelisted),
        SubmissionNotRetroactive(SubmissionNotRetroactive),
        UnauthorizedCaller(UnauthorizedCaller),
    }
    #[automatically_derived]
    impl RewardsCoordinatorErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 53u8, 78u8, 174u8],
            [13u8, 42u8, 249u8, 34u8],
            [14u8, 6u8, 189u8, 49u8],
            [16u8, 199u8, 72u8, 166u8],
            [27u8, 20u8, 23u8, 75u8],
            [28u8, 45u8, 105u8, 188u8],
            [48u8, 219u8, 217u8, 148u8],
            [55u8, 66u8, 231u8, 212u8],
            [67u8, 113u8, 74u8, 253u8],
            [67u8, 173u8, 32u8, 252u8],
            [68u8, 120u8, 246u8, 114u8],
            [77u8, 197u8, 246u8, 164u8],
            [80u8, 69u8, 112u8, 227u8],
            [84u8, 13u8, 98u8, 132u8],
            [92u8, 66u8, 124u8, 217u8],
            [93u8, 251u8, 44u8, 162u8],
            [99u8, 97u8, 206u8, 128u8],
            [105u8, 202u8, 22u8, 201u8],
            [114u8, 159u8, 148u8, 44u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [121u8, 108u8, 197u8, 37u8],
            [123u8, 30u8, 37u8, 197u8],
            [126u8, 226u8, 180u8, 67u8],
            [132u8, 10u8, 72u8, 213u8],
            [135u8, 218u8, 63u8, 136u8],
            [137u8, 28u8, 99u8, 223u8],
            [147u8, 45u8, 148u8, 247u8],
            [148u8, 168u8, 211u8, 137u8],
            [161u8, 189u8, 21u8, 216u8],
            [169u8, 254u8, 155u8, 224u8],
            [170u8, 56u8, 94u8, 129u8],
            [198u8, 29u8, 202u8, 93u8],
            [223u8, 173u8, 156u8, 161u8],
            [238u8, 102u8, 71u8, 5u8],
            [240u8, 106u8, 83u8, 196u8],
            [251u8, 73u8, 78u8, 161u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RewardsCoordinatorErrors {
        const NAME: &'static str = "RewardsCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 38usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AmountExceedsMax(_) => {
                    <AmountExceedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::AmountIsZero(_) => {
                    <AmountIsZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DurationExceedsMax(_) => {
                    <DurationExceedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::EarningsNotGreaterThanClaimed(_) => {
                    <EarningsNotGreaterThanClaimed as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidAddressZero(_) => {
                    <InvalidAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCalculationIntervalSecondsRemainder(_) => {
                    <InvalidCalculationIntervalSecondsRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidClaimProof(_) => {
                    <InvalidClaimProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDurationRemainder(_) => {
                    <InvalidDurationRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEarner(_) => {
                    <InvalidEarner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEarnerLeafIndex(_) => {
                    <InvalidEarnerLeafIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidGenesisRewardsTimestampRemainder(_) => {
                    <InvalidGenesisRewardsTimestampRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRoot(_) => {
                    <InvalidRoot as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidRootIndex(_) => {
                    <InvalidRootIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidStartTimestampRemainder(_) => {
                    <InvalidStartTimestampRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTokenLeafIndex(_) => {
                    <InvalidTokenLeafIndex as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NewRootMustBeForNewCalculatedPeriod(_) => {
                    <NewRootMustBeForNewCalculatedPeriod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorsNotInAscendingOrder(_) => {
                    <OperatorsNotInAscendingOrder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PreviousSplitPending(_) => {
                    <PreviousSplitPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RewardsEndTimestampNotElapsed(_) => {
                    <RewardsEndTimestampNotElapsed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RootAlreadyActivated(_) => {
                    <RootAlreadyActivated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RootDisabled(_) => {
                    <RootDisabled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::RootNotActivated(_) => {
                    <RootNotActivated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SplitExceedsMax(_) => {
                    <SplitExceedsMax as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StartTimestampTooFarInFuture(_) => {
                    <StartTimestampTooFarInFuture as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StartTimestampTooFarInPast(_) => {
                    <StartTimestampTooFarInPast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategiesNotInAscendingOrder(_) => {
                    <StrategiesNotInAscendingOrder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyNotWhitelisted(_) => {
                    <StrategyNotWhitelisted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SubmissionNotRetroactive(_) => {
                    <SubmissionNotRetroactive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnauthorizedCaller(_) => {
                    <UnauthorizedCaller as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<RewardsCoordinatorErrors>] = &[
                {
                    fn StartTimestampTooFarInPast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <StartTimestampTooFarInPast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::StartTimestampTooFarInPast)
                    }
                    StartTimestampTooFarInPast
                },
                {
                    fn RewardsEndTimestampNotElapsed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <RewardsEndTimestampNotElapsed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::RewardsEndTimestampNotElapsed)
                    }
                    RewardsEndTimestampNotElapsed
                },
                {
                    fn InvalidGenesisRewardsTimestampRemainder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidGenesisRewardsTimestampRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorErrors::InvalidGenesisRewardsTimestampRemainder,
                            )
                    }
                    InvalidGenesisRewardsTimestampRemainder
                },
                {
                    fn InvalidAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidAddressZero)
                    }
                    InvalidAddressZero
                },
                {
                    fn RootDisabled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <RootDisabled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::RootDisabled)
                    }
                    RootDisabled
                },
                {
                    fn AmountExceedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <AmountExceedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::AmountExceedsMax)
                    }
                    AmountExceedsMax
                },
                {
                    fn RootAlreadyActivated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <RootAlreadyActivated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::RootAlreadyActivated)
                    }
                    RootAlreadyActivated
                },
                {
                    fn DurationExceedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <DurationExceedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::DurationExceedsMax)
                    }
                    DurationExceedsMax
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn AmountIsZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <AmountIsZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::AmountIsZero)
                    }
                    AmountIsZero
                },
                {
                    fn InvalidCalculationIntervalSecondsRemainder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidCalculationIntervalSecondsRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorErrors::InvalidCalculationIntervalSecondsRemainder,
                            )
                    }
                    InvalidCalculationIntervalSecondsRemainder
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn InvalidRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidRoot as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidRoot)
                    }
                    InvalidRoot
                },
                {
                    fn SubmissionNotRetroactive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <SubmissionNotRetroactive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::SubmissionNotRetroactive)
                    }
                    SubmissionNotRetroactive
                },
                {
                    fn UnauthorizedCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <UnauthorizedCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::UnauthorizedCaller)
                    }
                    UnauthorizedCaller
                },
                {
                    fn StrategyNotWhitelisted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <StrategyNotWhitelisted as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::StrategyNotWhitelisted)
                    }
                    StrategyNotWhitelisted
                },
                {
                    fn InvalidEarnerLeafIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidEarnerLeafIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidEarnerLeafIndex)
                    }
                    InvalidEarnerLeafIndex
                },
                {
                    fn InvalidClaimProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidClaimProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidClaimProof)
                    }
                    InvalidClaimProof
                },
                {
                    fn NewRootMustBeForNewCalculatedPeriod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <NewRootMustBeForNewCalculatedPeriod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorErrors::NewRootMustBeForNewCalculatedPeriod,
                            )
                    }
                    NewRootMustBeForNewCalculatedPeriod
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InputArrayLengthZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InputArrayLengthZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InputArrayLengthZero)
                    }
                    InputArrayLengthZero
                },
                {
                    fn PreviousSplitPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <PreviousSplitPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::PreviousSplitPending)
                    }
                    PreviousSplitPending
                },
                {
                    fn StartTimestampTooFarInFuture(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <StartTimestampTooFarInFuture as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::StartTimestampTooFarInFuture)
                    }
                    StartTimestampTooFarInFuture
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn OperatorsNotInAscendingOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <OperatorsNotInAscendingOrder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::OperatorsNotInAscendingOrder)
                    }
                    OperatorsNotInAscendingOrder
                },
                {
                    fn SplitExceedsMax(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <SplitExceedsMax as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::SplitExceedsMax)
                    }
                    SplitExceedsMax
                },
                {
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn InvalidRootIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidRootIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidRootIndex)
                    }
                    InvalidRootIndex
                },
                {
                    fn RootNotActivated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <RootNotActivated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::RootNotActivated)
                    }
                    RootNotActivated
                },
                {
                    fn InvalidTokenLeafIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidTokenLeafIndex as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidTokenLeafIndex)
                    }
                    InvalidTokenLeafIndex
                },
                {
                    fn EarningsNotGreaterThanClaimed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <EarningsNotGreaterThanClaimed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::EarningsNotGreaterThanClaimed)
                    }
                    EarningsNotGreaterThanClaimed
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn StrategiesNotInAscendingOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <StrategiesNotInAscendingOrder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::StrategiesNotInAscendingOrder)
                    }
                    StrategiesNotInAscendingOrder
                },
                {
                    fn InvalidDurationRemainder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidDurationRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidDurationRemainder)
                    }
                    InvalidDurationRemainder
                },
                {
                    fn InvalidStartTimestampRemainder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidStartTimestampRemainder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                RewardsCoordinatorErrors::InvalidStartTimestampRemainder,
                            )
                    }
                    InvalidStartTimestampRemainder
                },
                {
                    fn InvalidEarner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorErrors> {
                        <InvalidEarner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorErrors::InvalidEarner)
                    }
                    InvalidEarner
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
                Self::AmountExceedsMax(inner) => {
                    <AmountExceedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::AmountIsZero(inner) => {
                    <AmountIsZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DurationExceedsMax(inner) => {
                    <DurationExceedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::EarningsNotGreaterThanClaimed(inner) => {
                    <EarningsNotGreaterThanClaimed as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidAddressZero(inner) => {
                    <InvalidAddressZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCalculationIntervalSecondsRemainder(inner) => {
                    <InvalidCalculationIntervalSecondsRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidClaimProof(inner) => {
                    <InvalidClaimProof as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidDurationRemainder(inner) => {
                    <InvalidDurationRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEarner(inner) => {
                    <InvalidEarner as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidEarnerLeafIndex(inner) => {
                    <InvalidEarnerLeafIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidGenesisRewardsTimestampRemainder(inner) => {
                    <InvalidGenesisRewardsTimestampRemainder as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidRoot(inner) => {
                    <InvalidRoot as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidRootIndex(inner) => {
                    <InvalidRootIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidStartTimestampRemainder(inner) => {
                    <InvalidStartTimestampRemainder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTokenLeafIndex(inner) => {
                    <InvalidTokenLeafIndex as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NewRootMustBeForNewCalculatedPeriod(inner) => {
                    <NewRootMustBeForNewCalculatedPeriod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorsNotInAscendingOrder(inner) => {
                    <OperatorsNotInAscendingOrder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PreviousSplitPending(inner) => {
                    <PreviousSplitPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RewardsEndTimestampNotElapsed(inner) => {
                    <RewardsEndTimestampNotElapsed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RootAlreadyActivated(inner) => {
                    <RootAlreadyActivated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::RootNotActivated(inner) => {
                    <RootNotActivated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SplitExceedsMax(inner) => {
                    <SplitExceedsMax as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StartTimestampTooFarInFuture(inner) => {
                    <StartTimestampTooFarInFuture as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StartTimestampTooFarInPast(inner) => {
                    <StartTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategiesNotInAscendingOrder(inner) => {
                    <StrategiesNotInAscendingOrder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyNotWhitelisted(inner) => {
                    <StrategyNotWhitelisted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SubmissionNotRetroactive(inner) => {
                    <SubmissionNotRetroactive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnauthorizedCaller(inner) => {
                    <UnauthorizedCaller as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AmountExceedsMax(inner) => {
                    <AmountExceedsMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::AmountIsZero(inner) => {
                    <AmountIsZero as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::DurationExceedsMax(inner) => {
                    <DurationExceedsMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::EarningsNotGreaterThanClaimed(inner) => {
                    <EarningsNotGreaterThanClaimed as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidAddressZero(inner) => {
                    <InvalidAddressZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCalculationIntervalSecondsRemainder(inner) => {
                    <InvalidCalculationIntervalSecondsRemainder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidClaimProof(inner) => {
                    <InvalidClaimProof as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidDurationRemainder(inner) => {
                    <InvalidDurationRemainder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEarner(inner) => {
                    <InvalidEarner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEarnerLeafIndex(inner) => {
                    <InvalidEarnerLeafIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidGenesisRewardsTimestampRemainder(inner) => {
                    <InvalidGenesisRewardsTimestampRemainder as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRoot(inner) => {
                    <InvalidRoot as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidRootIndex(inner) => {
                    <InvalidRootIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidStartTimestampRemainder(inner) => {
                    <InvalidStartTimestampRemainder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidTokenLeafIndex(inner) => {
                    <InvalidTokenLeafIndex as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NewRootMustBeForNewCalculatedPeriod(inner) => {
                    <NewRootMustBeForNewCalculatedPeriod as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorsNotInAscendingOrder(inner) => {
                    <OperatorsNotInAscendingOrder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PreviousSplitPending(inner) => {
                    <PreviousSplitPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RewardsEndTimestampNotElapsed(inner) => {
                    <RewardsEndTimestampNotElapsed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RootAlreadyActivated(inner) => {
                    <RootAlreadyActivated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RootDisabled(inner) => {
                    <RootDisabled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::RootNotActivated(inner) => {
                    <RootNotActivated as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SplitExceedsMax(inner) => {
                    <SplitExceedsMax as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StartTimestampTooFarInFuture(inner) => {
                    <StartTimestampTooFarInFuture as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StartTimestampTooFarInPast(inner) => {
                    <StartTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategiesNotInAscendingOrder(inner) => {
                    <StrategiesNotInAscendingOrder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategyNotWhitelisted(inner) => {
                    <StrategyNotWhitelisted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SubmissionNotRetroactive(inner) => {
                    <SubmissionNotRetroactive as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`RewardsCoordinator`](self) events.
    pub enum RewardsCoordinatorEvents {
        AVSRewardsSubmissionCreated(AVSRewardsSubmissionCreated),
        ActivationDelaySet(ActivationDelaySet),
        ClaimerForSet(ClaimerForSet),
        DefaultOperatorSplitBipsSet(DefaultOperatorSplitBipsSet),
        DistributionRootDisabled(DistributionRootDisabled),
        DistributionRootSubmitted(DistributionRootSubmitted),
        Initialized(Initialized),
        OperatorAVSSplitBipsSet(OperatorAVSSplitBipsSet),
        OperatorDirectedAVSRewardsSubmissionCreated(
            OperatorDirectedAVSRewardsSubmissionCreated,
        ),
        OperatorPISplitBipsSet(OperatorPISplitBipsSet),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        RewardsClaimed(RewardsClaimed),
        RewardsForAllSubmitterSet(RewardsForAllSubmitterSet),
        RewardsSubmissionForAllCreated(RewardsSubmissionForAllCreated),
        RewardsSubmissionForAllEarnersCreated(RewardsSubmissionForAllEarnersCreated),
        RewardsUpdaterSet(RewardsUpdaterSet),
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
                35u8,
                123u8,
                130u8,
                244u8,
                56u8,
                215u8,
                95u8,
                197u8,
                104u8,
                235u8,
                171u8,
                72u8,
                75u8,
                117u8,
                176u8,
                29u8,
                146u8,
                135u8,
                185u8,
                233u8,
                139u8,
                73u8,
                11u8,
                124u8,
                35u8,
                34u8,
                22u8,
                35u8,
                182u8,
                112u8,
                93u8,
                187u8,
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
                69u8,
                10u8,
                54u8,
                122u8,
                56u8,
                12u8,
                78u8,
                51u8,
                158u8,
                90u8,
                231u8,
                52u8,
                12u8,
                132u8,
                100u8,
                239u8,
                39u8,
                175u8,
                119u8,
                129u8,
                173u8,
                153u8,
                69u8,
                207u8,
                232u8,
                171u8,
                216u8,
                40u8,
                248u8,
                158u8,
                98u8,
                129u8,
            ],
            [
                72u8,
                225u8,
                152u8,
                182u8,
                174u8,
                53u8,
                126u8,
                82u8,
                146u8,
                4u8,
                238u8,
                83u8,
                168u8,
                229u8,
                20u8,
                196u8,
                112u8,
                255u8,
                119u8,
                217u8,
                204u8,
                142u8,
                79u8,
                114u8,
                7u8,
                248u8,
                181u8,
                212u8,
                144u8,
                174u8,
                105u8,
                52u8,
            ],
            [
                77u8,
                230u8,
                41u8,
                62u8,
                102u8,
                141u8,
                241u8,
                57u8,
                132u8,
                34u8,
                225u8,
                222u8,
                241u8,
                33u8,
                24u8,
                5u8,
                44u8,
                21u8,
                57u8,
                160u8,
                60u8,
                191u8,
                237u8,
                193u8,
                69u8,
                137u8,
                93u8,
                72u8,
                215u8,
                104u8,
                95u8,
                28u8,
            ],
            [
                81u8,
                8u8,
                139u8,
                140u8,
                137u8,
                98u8,
                141u8,
                243u8,
                168u8,
                23u8,
                64u8,
                2u8,
                194u8,
                160u8,
                52u8,
                208u8,
                21u8,
                47u8,
                206u8,
                106u8,
                248u8,
                65u8,
                93u8,
                101u8,
                27u8,
                42u8,
                71u8,
                52u8,
                191u8,
                39u8,
                4u8,
                130u8,
            ],
            [
                82u8,
                81u8,
                182u8,
                253u8,
                239u8,
                203u8,
                93u8,
                129u8,
                20u8,
                78u8,
                115u8,
                95u8,
                105u8,
                234u8,
                76u8,
                105u8,
                95u8,
                212u8,
                59u8,
                2u8,
                137u8,
                202u8,
                83u8,
                220u8,
                7u8,
                80u8,
                51u8,
                245u8,
                252u8,
                128u8,
                6u8,
                139u8,
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
                149u8,
                67u8,
                219u8,
                213u8,
                85u8,
                128u8,
                132u8,
                37u8,
                134u8,
                169u8,
                81u8,
                240u8,
                56u8,
                110u8,
                36u8,
                214u8,
                138u8,
                93u8,
                249u8,
                154u8,
                226u8,
                158u8,
                59u8,
                33u8,
                101u8,
                136u8,
                180u8,
                95u8,
                214u8,
                132u8,
                206u8,
                49u8,
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
                175u8,
                85u8,
                124u8,
                108u8,
                2u8,
                194u8,
                8u8,
                121u8,
                72u8,
                23u8,
                167u8,
                5u8,
                96u8,
                156u8,
                250u8,
                147u8,
                95u8,
                130u8,
                115u8,
                18u8,
                161u8,
                173u8,
                253u8,
                210u8,
                100u8,
                148u8,
                182u8,
                185u8,
                93u8,
                210u8,
                180u8,
                179u8,
            ],
            [
                186u8,
                185u8,
                71u8,
                147u8,
                77u8,
                66u8,
                224u8,
                173u8,
                32u8,
                111u8,
                37u8,
                201u8,
                202u8,
                177u8,
                139u8,
                91u8,
                182u8,
                174u8,
                20u8,
                74u8,
                207u8,
                176u8,
                15u8,
                64u8,
                180u8,
                227u8,
                170u8,
                89u8,
                89u8,
                12u8,
                163u8,
                18u8,
            ],
            [
                209u8,
                224u8,
                40u8,
                189u8,
                102u8,
                68u8,
                134u8,
                164u8,
                106u8,
                210u8,
                96u8,
                64u8,
                233u8,
                153u8,
                205u8,
                45u8,
                34u8,
                225u8,
                233u8,
                160u8,
                148u8,
                238u8,
                106u8,
                254u8,
                25u8,
                252u8,
                246u8,
                70u8,
                120u8,
                241u8,
                111u8,
                116u8,
            ],
            [
                216u8,
                80u8,
                230u8,
                229u8,
                223u8,
                164u8,
                151u8,
                183u8,
                38u8,
                97u8,
                250u8,
                115u8,
                223u8,
                41u8,
                35u8,
                70u8,
                78u8,
                174u8,
                217u8,
                220u8,
                47u8,
                241u8,
                211u8,
                203u8,
                130u8,
                188u8,
                203u8,
                254u8,
                171u8,
                229u8,
                196u8,
                30u8,
            ],
            [
                230u8,
                205u8,
                78u8,
                223u8,
                220u8,
                193u8,
                246u8,
                209u8,
                48u8,
                171u8,
                53u8,
                247u8,
                61u8,
                114u8,
                55u8,
                143u8,
                58u8,
                100u8,
                41u8,
                68u8,
                251u8,
                78u8,
                229u8,
                189u8,
                132u8,
                183u8,
                128u8,
                122u8,
                129u8,
                234u8,
                28u8,
                78u8,
            ],
            [
                236u8,
                216u8,
                102u8,
                195u8,
                193u8,
                88u8,
                250u8,
                0u8,
                191u8,
                52u8,
                216u8,
                3u8,
                213u8,
                246u8,
                2u8,
                48u8,
                0u8,
                181u8,
                112u8,
                128u8,
                188u8,
                180u8,
                138u8,
                240u8,
                4u8,
                194u8,
                180u8,
                180u8,
                107u8,
                58u8,
                253u8,
                8u8,
            ],
            [
                252u8,
                136u8,
                136u8,
                191u8,
                253u8,
                113u8,
                29u8,
                166u8,
                11u8,
                197u8,
                9u8,
                43u8,
                51u8,
                246u8,
                119u8,
                216u8,
                24u8,
                150u8,
                254u8,
                128u8,
                236u8,
                198u8,
                119u8,
                184u8,
                76u8,
                250u8,
                184u8,
                24u8,
                68u8,
                98u8,
                182u8,
                224u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RewardsCoordinatorEvents {
        const NAME: &'static str = "RewardsCoordinatorEvents";
        const COUNT: usize = 18usize;
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
                Self::Paused(inner) => {
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
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
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
                Self::Paused(inner) => {
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
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _CALCULATION_INTERVAL_SECONDS: u32,
        _MAX_REWARDS_DURATION: u32,
        _MAX_RETROACTIVE_LENGTH: u32,
        _MAX_FUTURE_LENGTH: u32,
        _GENESIS_REWARDS_TIMESTAMP: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<RewardsCoordinatorInstance<T, P, N>>,
    > {
        RewardsCoordinatorInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _delegationManager,
            _strategyManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _CALCULATION_INTERVAL_SECONDS,
            _MAX_REWARDS_DURATION,
            _MAX_RETROACTIVE_LENGTH,
            _MAX_FUTURE_LENGTH,
            _GENESIS_REWARDS_TIMESTAMP,
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
        _allocationManager: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _permissionController: alloy::sol_types::private::Address,
        _CALCULATION_INTERVAL_SECONDS: u32,
        _MAX_REWARDS_DURATION: u32,
        _MAX_RETROACTIVE_LENGTH: u32,
        _MAX_FUTURE_LENGTH: u32,
        _GENESIS_REWARDS_TIMESTAMP: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        RewardsCoordinatorInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _delegationManager,
            _strategyManager,
            _allocationManager,
            _pauserRegistry,
            _permissionController,
            _CALCULATION_INTERVAL_SECONDS,
            _MAX_REWARDS_DURATION,
            _MAX_RETROACTIVE_LENGTH,
            _MAX_FUTURE_LENGTH,
            _GENESIS_REWARDS_TIMESTAMP,
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
            f.debug_tuple("RewardsCoordinatorInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > RewardsCoordinatorInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`RewardsCoordinator`](self) contract instance.

See the [wrapper's documentation](`RewardsCoordinatorInstance`) for more details.*/
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
            _delegationManager: alloy::sol_types::private::Address,
            _strategyManager: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _CALCULATION_INTERVAL_SECONDS: u32,
            _MAX_REWARDS_DURATION: u32,
            _MAX_RETROACTIVE_LENGTH: u32,
            _MAX_FUTURE_LENGTH: u32,
            _GENESIS_REWARDS_TIMESTAMP: u32,
        ) -> alloy_contract::Result<RewardsCoordinatorInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegationManager,
                _strategyManager,
                _allocationManager,
                _pauserRegistry,
                _permissionController,
                _CALCULATION_INTERVAL_SECONDS,
                _MAX_REWARDS_DURATION,
                _MAX_RETROACTIVE_LENGTH,
                _MAX_FUTURE_LENGTH,
                _GENESIS_REWARDS_TIMESTAMP,
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
            _allocationManager: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _permissionController: alloy::sol_types::private::Address,
            _CALCULATION_INTERVAL_SECONDS: u32,
            _MAX_REWARDS_DURATION: u32,
            _MAX_RETROACTIVE_LENGTH: u32,
            _MAX_FUTURE_LENGTH: u32,
            _GENESIS_REWARDS_TIMESTAMP: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegationManager,
                            _strategyManager,
                            _allocationManager,
                            _pauserRegistry,
                            _permissionController,
                            _CALCULATION_INTERVAL_SECONDS,
                            _MAX_REWARDS_DURATION,
                            _MAX_RETROACTIVE_LENGTH,
                            _MAX_FUTURE_LENGTH,
                            _GENESIS_REWARDS_TIMESTAMP,
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
    > RewardsCoordinatorInstance<T, P, N> {
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
            self.call_builder(
                &CALCULATION_INTERVAL_SECONDSCall {
                },
            )
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
        ///Creates a new call builder for the [`calculateEarnerLeafHash`] function.
        pub fn calculateEarnerLeafHash(
            &self,
            leaf: <IRewardsCoordinatorTypes::EarnerTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateEarnerLeafHashCall, N> {
            self.call_builder(
                &calculateEarnerLeafHashCall {
                    leaf,
                },
            )
        }
        ///Creates a new call builder for the [`calculateTokenLeafHash`] function.
        pub fn calculateTokenLeafHash(
            &self,
            leaf: <IRewardsCoordinatorTypes::TokenTreeMerkleLeaf as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateTokenLeafHashCall, N> {
            self.call_builder(&calculateTokenLeafHashCall { leaf })
        }
        ///Creates a new call builder for the [`checkClaim`] function.
        pub fn checkClaim(
            &self,
            claim: <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkClaimCall, N> {
            self.call_builder(&checkClaimCall { claim })
        }
        ///Creates a new call builder for the [`claimerFor`] function.
        pub fn claimerFor(
            &self,
            earner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimerForCall, N> {
            self.call_builder(&claimerForCall { earner })
        }
        ///Creates a new call builder for the [`createAVSRewardsSubmission`] function.
        pub fn createAVSRewardsSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createAVSRewardsSubmissionCall, N> {
            self.call_builder(
                &createAVSRewardsSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createOperatorDirectedAVSRewardsSubmission`] function.
        pub fn createOperatorDirectedAVSRewardsSubmission(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorDirectedRewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::OperatorDirectedRewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            createOperatorDirectedAVSRewardsSubmissionCall,
            N,
        > {
            self.call_builder(
                &createOperatorDirectedAVSRewardsSubmissionCall {
                    avs,
                    operatorDirectedRewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createRewardsForAllEarners`] function.
        pub fn createRewardsForAllEarners(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createRewardsForAllEarnersCall, N> {
            self.call_builder(
                &createRewardsForAllEarnersCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`createRewardsForAllSubmission`] function.
        pub fn createRewardsForAllSubmission(
            &self,
            rewardsSubmissions: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsSubmission as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            createRewardsForAllSubmissionCall,
            N,
        > {
            self.call_builder(
                &createRewardsForAllSubmissionCall {
                    rewardsSubmissions,
                },
            )
        }
        ///Creates a new call builder for the [`cumulativeClaimed`] function.
        pub fn cumulativeClaimed(
            &self,
            earner: alloy::sol_types::private::Address,
            token: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeClaimedCall, N> {
            self.call_builder(
                &cumulativeClaimedCall {
                    earner,
                    token,
                },
            )
        }
        ///Creates a new call builder for the [`currRewardsCalculationEndTimestamp`] function.
        pub fn currRewardsCalculationEndTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            currRewardsCalculationEndTimestampCall,
            N,
        > {
            self.call_builder(
                &currRewardsCalculationEndTimestampCall {
                },
            )
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
        ///Creates a new call builder for the [`getCurrentClaimableDistributionRoot`] function.
        pub fn getCurrentClaimableDistributionRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getCurrentClaimableDistributionRootCall,
            N,
        > {
            self.call_builder(
                &getCurrentClaimableDistributionRootCall {
                },
            )
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
            self.call_builder(
                &getDistributionRootAtIndexCall {
                    index,
                },
            )
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
            self.call_builder(
                &getOperatorAVSSplitCall {
                    operator,
                    avs,
                },
            )
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
            self.call_builder(
                &getRootIndexFromHashCall {
                    rootHash,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _rewardsUpdater: alloy::sol_types::private::Address,
            _activationDelay: u32,
            _defaultSplitBips: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    initialPausedStatus,
                    _rewardsUpdater,
                    _activationDelay,
                    _defaultSplitBips,
                },
            )
        }
        ///Creates a new call builder for the [`isAVSRewardsSubmissionHash`] function.
        pub fn isAVSRewardsSubmissionHash(
            &self,
            avs: alloy::sol_types::private::Address,
            hash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isAVSRewardsSubmissionHashCall, N> {
            self.call_builder(
                &isAVSRewardsSubmissionHashCall {
                    avs,
                    hash,
                },
            )
        }
        ///Creates a new call builder for the [`isOperatorDirectedAVSRewardsSubmissionHash`] function.
        pub fn isOperatorDirectedAVSRewardsSubmissionHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            isOperatorDirectedAVSRewardsSubmissionHashCall,
            N,
        > {
            self.call_builder(
                &isOperatorDirectedAVSRewardsSubmissionHashCall {
                    _0,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isRewardsForAllSubmitter`] function.
        pub fn isRewardsForAllSubmitter(
            &self,
            submitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isRewardsForAllSubmitterCall, N> {
            self.call_builder(
                &isRewardsForAllSubmitterCall {
                    submitter,
                },
            )
        }
        ///Creates a new call builder for the [`isRewardsSubmissionForAllEarnersHash`] function.
        pub fn isRewardsSubmissionForAllEarnersHash(
            &self,
            avs: alloy::sol_types::private::Address,
            hash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            isRewardsSubmissionForAllEarnersHashCall,
            N,
        > {
            self.call_builder(
                &isRewardsSubmissionForAllEarnersHashCall {
                    avs,
                    hash,
                },
            )
        }
        ///Creates a new call builder for the [`isRewardsSubmissionForAllHash`] function.
        pub fn isRewardsSubmissionForAllHash(
            &self,
            avs: alloy::sol_types::private::Address,
            hash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            isRewardsSubmissionForAllHashCall,
            N,
        > {
            self.call_builder(
                &isRewardsSubmissionForAllHashCall {
                    avs,
                    hash,
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
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall {})
        }
        ///Creates a new call builder for the [`processClaim`] function.
        pub fn processClaim(
            &self,
            claim: <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, processClaimCall, N> {
            self.call_builder(
                &processClaimCall {
                    claim,
                    recipient,
                },
            )
        }
        ///Creates a new call builder for the [`processClaims`] function.
        pub fn processClaims(
            &self,
            claims: alloy::sol_types::private::Vec<
                <IRewardsCoordinatorTypes::RewardsMerkleClaim as alloy::sol_types::SolType>::RustType,
            >,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, processClaimsCall, N> {
            self.call_builder(
                &processClaimsCall {
                    claims,
                    recipient,
                },
            )
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
            self.call_builder(
                &setActivationDelayCall {
                    _activationDelay,
                },
            )
        }
        ///Creates a new call builder for the [`setClaimerFor_0`] function.
        pub fn setClaimerFor_0(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerFor_0Call, N> {
            self.call_builder(&setClaimerFor_0Call { claimer })
        }
        ///Creates a new call builder for the [`setClaimerFor_1`] function.
        pub fn setClaimerFor_1(
            &self,
            earner: alloy::sol_types::private::Address,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerFor_1Call, N> {
            self.call_builder(
                &setClaimerFor_1Call {
                    earner,
                    claimer,
                },
            )
        }
        ///Creates a new call builder for the [`setDefaultOperatorSplit`] function.
        pub fn setDefaultOperatorSplit(
            &self,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setDefaultOperatorSplitCall, N> {
            self.call_builder(
                &setDefaultOperatorSplitCall {
                    split,
                },
            )
        }
        ///Creates a new call builder for the [`setOperatorAVSSplit`] function.
        pub fn setOperatorAVSSplit(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorAVSSplitCall, N> {
            self.call_builder(
                &setOperatorAVSSplitCall {
                    operator,
                    avs,
                    split,
                },
            )
        }
        ///Creates a new call builder for the [`setOperatorPISplit`] function.
        pub fn setOperatorPISplit(
            &self,
            operator: alloy::sol_types::private::Address,
            split: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorPISplitCall, N> {
            self.call_builder(
                &setOperatorPISplitCall {
                    operator,
                    split,
                },
            )
        }
        ///Creates a new call builder for the [`setRewardsForAllSubmitter`] function.
        pub fn setRewardsForAllSubmitter(
            &self,
            _submitter: alloy::sol_types::private::Address,
            _newValue: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsForAllSubmitterCall, N> {
            self.call_builder(
                &setRewardsForAllSubmitterCall {
                    _submitter,
                    _newValue,
                },
            )
        }
        ///Creates a new call builder for the [`setRewardsUpdater`] function.
        pub fn setRewardsUpdater(
            &self,
            _rewardsUpdater: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setRewardsUpdaterCall, N> {
            self.call_builder(
                &setRewardsUpdaterCall {
                    _rewardsUpdater,
                },
            )
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
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, submissionNonceCall, N> {
            self.call_builder(&submissionNonceCall { avs })
        }
        ///Creates a new call builder for the [`submitRoot`] function.
        pub fn submitRoot(
            &self,
            root: alloy::sol_types::private::FixedBytes<32>,
            rewardsCalculationEndTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, submitRootCall, N> {
            self.call_builder(
                &submitRootCall {
                    root,
                    rewardsCalculationEndTimestamp,
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
    > RewardsCoordinatorInstance<T, P, N> {
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
        pub fn ClaimerForSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ClaimerForSet, N> {
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
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
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
        ) -> alloy_contract::Event<
            T,
            &P,
            OperatorDirectedAVSRewardsSubmissionCreated,
            N,
        > {
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
        ///Creates a new event filter for the [`RewardsClaimed`] event.
        pub fn RewardsClaimed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RewardsClaimed, N> {
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
