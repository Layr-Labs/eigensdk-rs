///Module containing a contract's types and functions.
/**

```solidity
library IRewardsCoordinatorTypes {
    struct DistributionRoot { bytes32 root; uint32 rewardsCalculationEndTimestamp; uint32 activatedAt; bool disabled; }
    struct EarnerTreeMerkleLeaf { address earner; bytes32 earnerTokenRoot; }
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
    error InvalidCalculationIntervalSecondsRemainder();
    error InvalidClaimProof();
    error InvalidDurationRemainder();
    error InvalidEarnerLeafIndex();
    error InvalidGenesisRewardsTimestampRemainder();
    error InvalidNewPausedStatus();
    error InvalidProofLength();
    error InvalidRoot();
    error InvalidRootIndex();
    error InvalidStartTimestampRemainder();
    error InvalidTokenLeafIndex();
    error NewRootMustBeForNewCalculatedPeriod();
    error OnlyPauser();
    error OnlyUnpauser();
    error RewardsEndTimestampNotElapsed();
    error RootAlreadyActivated();
    error RootDisabled();
    error RootNotActivated();
    error StartTimestampTooFarInFuture();
    error StartTimestampTooFarInPast();
    error StrategiesNotInAscendingOrder();
    error StrategyNotWhitelisted();
    error UnauthorizedCaller();

    event AVSRewardsSubmissionCreated(address indexed avs, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event ActivationDelaySet(uint32 oldActivationDelay, uint32 newActivationDelay);
    event ClaimerForSet(address indexed earner, address indexed oldClaimer, address indexed claimer);
    event DistributionRootDisabled(uint32 indexed rootIndex);
    event DistributionRootSubmitted(uint32 indexed rootIndex, bytes32 indexed root, uint32 indexed rewardsCalculationEndTimestamp, uint32 activatedAt);
    event GlobalCommissionBipsSet(uint16 oldGlobalCommissionBips, uint16 newGlobalCommissionBips);
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event RewardsClaimed(bytes32 root, address indexed earner, address indexed claimer, address indexed recipient, address token, uint256 claimedAmount);
    event RewardsForAllSubmitterSet(address indexed rewardsForAllSubmitter, bool indexed oldValue, bool indexed newValue);
    event RewardsSubmissionForAllCreated(address indexed submitter, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event RewardsSubmissionForAllEarnersCreated(address indexed tokenHopper, uint256 indexed submissionNonce, bytes32 indexed rewardsSubmissionHash, IRewardsCoordinatorTypes.RewardsSubmission rewardsSubmission);
    event RewardsUpdaterSet(address indexed oldRewardsUpdater, address indexed newRewardsUpdater);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegationManager, address _strategyManager, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 _GENESIS_REWARDS_TIMESTAMP);

    function CALCULATION_INTERVAL_SECONDS() external view returns (uint32);
    function GENESIS_REWARDS_TIMESTAMP() external view returns (uint32);
    function MAX_FUTURE_LENGTH() external view returns (uint32);
    function MAX_RETROACTIVE_LENGTH() external view returns (uint32);
    function MAX_REWARDS_DURATION() external view returns (uint32);
    function activationDelay() external view returns (uint32);
    function beaconChainETHStrategy() external view returns (address);
    function calculateEarnerLeafHash(IRewardsCoordinatorTypes.EarnerTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function calculateTokenLeafHash(IRewardsCoordinatorTypes.TokenTreeMerkleLeaf memory leaf) external pure returns (bytes32);
    function checkClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim) external view returns (bool);
    function claimerFor(address) external view returns (address);
    function createAVSRewardsSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createRewardsForAllEarners(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function createRewardsForAllSubmission(IRewardsCoordinatorTypes.RewardsSubmission[] memory rewardsSubmissions) external;
    function cumulativeClaimed(address, address) external view returns (uint256);
    function currRewardsCalculationEndTimestamp() external view returns (uint32);
    function delegationManager() external view returns (address);
    function disableRoot(uint32 rootIndex) external;
    function getCurrentClaimableDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getCurrentDistributionRoot() external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getDistributionRootAtIndex(uint256 index) external view returns (IRewardsCoordinatorTypes.DistributionRoot memory);
    function getDistributionRootsLength() external view returns (uint256);
    function getRootIndexFromHash(bytes32 rootHash) external view returns (uint32);
    function globalOperatorCommissionBips() external view returns (uint16);
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _globalCommissionBips) external;
    function isAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
    function isRewardsForAllSubmitter(address) external view returns (bool);
    function isRewardsSubmissionForAllEarnersHash(address, bytes32) external view returns (bool);
    function isRewardsSubmissionForAllHash(address, bytes32) external view returns (bool);
    function operatorCommissionBips(address operator, address avs) external view returns (uint16);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function processClaim(IRewardsCoordinatorTypes.RewardsMerkleClaim memory claim, address recipient) external;
    function renounceOwnership() external;
    function rewardsUpdater() external view returns (address);
    function setActivationDelay(uint32 _activationDelay) external;
    function setClaimerFor(address claimer) external;
    function setGlobalOperatorCommission(uint16 _globalCommissionBips) external;
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
    "name": "globalOperatorCommissionBips",
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
        "name": "_globalCommissionBips",
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
    "name": "operatorCommissionBips",
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
    "name": "setGlobalOperatorCommission",
    "inputs": [
      {
        "name": "_globalCommissionBips",
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
    "name": "GlobalCommissionBipsSet",
    "inputs": [
      {
        "name": "oldGlobalCommissionBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
      },
      {
        "name": "newGlobalCommissionBips",
        "type": "uint16",
        "indexed": false,
        "internalType": "uint16"
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
    ///0x6101603461023e57601f612e9638819003918201601f19168301916001600160401b038311848410176102425780849260e09460405283398101031261023e578051906001600160a01b038216820361023e5760208101516001600160a01b038116810361023e5761007360408301610256565b61007f60608401610256565b9061008c60808501610256565b926100a560c061009e60a08801610256565b9601610256565b9563ffffffff8316801561022a5763ffffffff81818a16061661021b576201518063ffffffff91061661020c5760805260a05260c05260e0526101005261012052610140525f5460ff8160081c166101b75760ff8082161061017d575b604051612c2e9081610268823960805181611461015260a051818181610953015261240d015260c0518181816110e20152612332015260e05181818161121e01526122f701526101005181818161091a015261238b0152610120518181816103a901526123dc0152610140518181816105ac01526125310152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f610102565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b63223c7b3960e11b5f5260045ffd5b630e06bd3160e01b5f5260045ffd5b634e487b7160e01b5f52601260045260245ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b519063ffffffff8216820361023e5756fe60806040526004361015610011575f80fd5b5f3560e01c806218572c1461033357806304a0c5021461032e578063092db007146103295780630e9a53cf146103245780630eb383451461031f57806310d67a2f1461031a578063131433b414610315578063136439dd14610310578063149bc8721461030b57806322f19a64146103065780632b9f64a41461030157806336af41fa146102fc57806337838ed0146102f757806339b70e38146102f25780633a8c0786146102ed5780633ccc861d146102e85780633efe1db6146102e35780634d18cc35146102de57806358baaa3e146102d9578063595c6a67146102d45780635ac86ab7146102cf5780635c975abb146102ca5780635e9d8348146102c55780636d21117e146102c0578063715018a6146102bb5780637b8f8b05146102b6578063863cb9a9146102b1578063865c6953146102ac578063886f1195146102a75780638da5cb5b146102a25780639104c3191461029d5780639be3d4e4146102985780639d45c28114610293578063a0169ddd1461028e578063aebd8bae14610289578063bb7e451f14610284578063bf21a8aa1461027f578063c46db6061461027a578063d4540a5514610275578063de02e50314610270578063e221b2451461026b578063e810ce2114610266578063ea4d3c9b14610261578063f2fde38b1461025c578063f8cd844814610257578063f96abf2e14610252578063fabc1cbc1461024d578063fbf1e2c114610248578063fce36c7d146102435763ff9f6cce1461023e575f80fd5b6117db565b6116e3565b6116bb565b61161f565b611545565b611521565b611490565b61144c565b611420565b6113f0565b6113c5565b6112a1565b611242565b611202565b6111c7565b611179565b611106565b6110c6565b61108b565b61105d565b611035565b61100d565b610fae565b610f76565b610f59565b610efe565b610eb0565b610e5a565b610e3d565b610e0a565b610d84565b610d57565b610d31565b610bf4565b6109b7565b610982565b61093e565b6108fe565b6107a6565b61070f565b6106d4565b6106a8565b6105d0565b610590565b6104ec565b610459565b6103f1565b6103cd565b61038d565b61034d565b6001600160a01b0381160361034957565b5f80fd5b346103495760203660031901126103495760043561036a81610338565b60018060a01b03165f5260d1602052602060ff60405f2054166040519015158152f35b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610349575f36600319011261034957602061ffff60cb5460e01c16604051908152f35b34610349575f3660031901126103495761044b61040c611a3f565b6040519182918291909160608060808301948051845263ffffffff602082015116602085015263ffffffff604082015116604085015201511515910152565b0390f35b8015150361034957565b346103495760403660031901126103495760043561047681610338565b602435906104838261044f565b61048b6120bd565b60018060a01b0316805f5260d160205260ff60405f205416151582151590827f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c5f80a45f5260d160205260405f209060ff8019835416911515161790555f80f35b34610349576020366003190112610349576004803561050a81610338565b60655460405163755b36bd60e11b81529260209184919082906001600160a01b03165afa91821561058b5761055a92610555915f9161055c575b506001600160a01b03163314611af9565b61212b565b005b61057e915060203d602011610584575b6105768183611920565b810190611ad6565b5f610544565b503d61056c565b611aee565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760203660031901126103495760043560655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561058b57610627915f91610668575b50611b24565b61063660665482811614611b3a565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b61068a915060203d602011610690575b6106828183611920565b810190611b0f565b5f610621565b503d610678565b604090600319011261034957600490565b346103495760403660031901126103495760206106cc6106c736610697565b611b5a565b604051908152f35b34610349576040366003190112610349576106f0600435610338565b6106fb602435610338565b602061ffff60cb5460e01c16604051908152f35b346103495760203660031901126103495760043561072c81610338565b60018060a01b03165f5260cc602052602060018060a01b0360405f205416604051908152f35b9060206003198301126103495760043567ffffffffffffffff811161034957826023820112156103495780600401359267ffffffffffffffff84116103495760248460051b83010111610349576024019190565b34610349576107b436610752565b906107cc6107c6600280606654161490565b15611ba3565b335f5260d16020526107e460ff60405f205416611bb9565b6107f360026097541415611bcf565b60026097555f5b82811061080b5761055a6001609755565b806108f861081c6001938686611c1b565b335f90815260ce602052604090205460405160208101906108518161084386863387611d47565b03601f198101835282611920565b5190209061085e83612295565b335f90815260d060205260409020610891906108849084905b905f5260205260405f2090565b805460ff19166001179055565b61089a81611d6b565b335f90815260ce60205260409020556040517f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf2704823391806108da8782611d94565b0390a460406108eb60208301611b50565b910135903090339061255a565b016107fa565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610349575f366003190112610349576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610349575f36600319011261034957602063ffffffff60cb5460a01c16604051908152f35b90816101009103126103495790565b346103495760403660031901126103495760043567ffffffffffffffff8111610349576109e89036906004016109a8565b6024356109f481610338565b610a056107c6600480606654161490565b610a1460026097541415611bcf565b6002609755610a33610a2d610a2884611da5565b6119c4565b506119fe565b90610a3e828461262d565b610a4a60608401611b50565b6001600160a01b038181165f90815260cc6020526040902054168015610bd2575b9192916001600160a01b031690610a83338314611bb9565b60a08501936001600160a01b038181169490831692909160e08801915f5b610aab898b611daf565b9050811015610bc8576001818989897f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce318f89610bbd610b968e610b8f8f610b058f919e610aab9f610aff90610b779a611de5565b90611e1b565b95610b40610b238360018060a01b03165f5260cd60205260405f2090565b610b2c89611b50565b60018060a01b03165f5260205260405f2090565b54610b80610b5e60208a013592610b58818511611e2b565b836119a3565b998a9460018060a01b03165f5260cd60205260405f2090565b610b2c8a611b50565b55610b8a87611b50565b612752565b5192611b50565b604080519384526001600160a01b0390911660208401528201929092529081906060820190565b0390a4019050610aa1565b61055a6001609755565b5080610a6b565b63ffffffff81160361034957565b3590610bf282610bd9565b565b3461034957604036600319011261034957602435600435610c1482610bd9565b610c256107c6600880606654161490565b60cb5491610c3d336001600160a01b03851614611bb9565b63ffffffff81169263ffffffff8160c01c16841115610d225763ffffffff7fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd0891610c88428710611e41565b610d0c610cb5610cad610ca060ca5463ffffffff1690565b9360a01c63ffffffff1690565b844216611e57565b94610ce7610cc1611942565b88815263ffffffff8316602082015263ffffffff881660408201525f6060820152611e71565b60cb805463ffffffff60c01b191660c09290921b63ffffffff60c01b16919091179055565b60405163ffffffff9490941684521691602090a4005b631ca7e50b60e21b5f5260045ffd5b34610349575f36600319011261034957602063ffffffff60cb5460c01c16604051908152f35b346103495760203660031901126103495761055a600435610d7781610bd9565b610d7f6120bd565b61278d565b34610349575f3660031901126103495760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561058b57610dd6915f916106685750611b24565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b346103495760203660031901126103495760043560ff81168091036103495760016020911b806066541614604051908152f35b34610349575f366003190112610349576020606654604051908152f35b346103495760203660031901126103495760043567ffffffffffffffff811161034957610e8e610ea59136906004016109a8565b610e9f610a2d8235610a2881610bd9565b9061262d565b602060405160018152f35b3461034957604036600319011261034957600435610ecd81610338565b6024359060018060a01b03165f5260cf60205260405f20905f52602052602060ff60405f2054166040519015158152f35b34610349575f36600319011261034957610f166120bd565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610349575f36600319011261034957602060ca54604051908152f35b346103495760203660031901126103495761055a600435610f9681610338565b610f9e6120bd565b612838565b3590610bf282610338565b34610349576040366003190112610349576020611004600435610fd081610338565b60243590610fdd82610338565b60018060a01b03165f5260cd835260405f209060018060a01b03165f5260205260405f2090565b54604051908152f35b34610349575f366003190112610349576065546040516001600160a01b039091168152602090f35b34610349575f366003190112610349576033546040516001600160a01b039091168152602090f35b34610349575f36600319011261034957602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610349575f366003190112610349576110a3611951565b5060ca545f1981019081116110c15761040c610a2d61044b926119c4565b611975565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760203660031901126103495760043561112381610338565b335f81815260cc6020526040812080546001600160a01b039485166001600160a01b03198216811790925590931691907fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca3129080a4005b346103495760403660031901126103495760043561119681610338565b6024359060018060a01b03165f5260d260205260405f20905f52602052602060ff60405f2054166040519015158152f35b34610349576020366003190112610349576004356111e481610338565b60018060a01b03165f5260ce602052602060405f2054604051908152f35b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760403660031901126103495760043561125f81610338565b6024359060018060a01b03165f5260d060205260405f20905f52602052602060ff60405f2054166040519015158152f35b60a4359061ffff8216820361034957565b346103495760c0366003190112610349576004356112be81610338565b61133d6024356112cd81610338565b6044356064356112dc81610338565b608435916112e983610bd9565b6112f1611290565b935f549661132361130d6113098a60ff9060081c1690565b1590565b8099819a6113b7575b8115611397575b50611f13565b87611334600160ff195f5416175f55565b61138057611f76565b61134357005b61135161ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b61139261010061ff00195f5416175f55565b611f76565b303b159150816113a9575b505f61131d565b60ff1660011490505f6113a2565b600160ff8216109150611316565b346103495760203660031901126103495761044b61040c610a2d6004356113ea611951565b506119c4565b346103495760203660031901126103495760043561ffff811681036103495761055a9061141b6120bd565b61287f565b3461034957602036600319011261034957602061143e60043561200a565b63ffffffff60405191168152f35b34610349575f366003190112610349576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610349576020366003190112610349576004356114ad81610338565b6114b56120bd565b6001600160a01b038116156114cd5761055a906127f0565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346103495760403660031901126103495760206106cc61154036610697565b612060565b346103495760203660031901126103495760043561156281610bd9565b6115736107c6600880606654161490565b61158860018060a01b0360cb54163314611bb9565b60ca549063ffffffff811691821015611610576115a66001916119c4565b500163ffffffff81546115bf60ff8260401c16156120a7565b60201c1642101561160157805460ff60401b1916600160401b1790557fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e5f80a2005b630c36f66560e21b5f5260045ffd5b6394a8d38960e01b5f5260045ffd5b346103495760203660031901126103495760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa801561058b57611678915f9161055c57506001600160a01b03163314611af9565b611689606654198219811614611b3a565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b34610349575f3660031901126103495760cb546040516001600160a01b039091168152602090f35b34610349576116f136610752565b906117036107c6600180606654161490565b61171260026097541415611bcf565b60026097555f5b82811061172a5761055a6001609755565b806117d561173b6001938686611c1b565b335f90815260ce602052604090205460405160208101906117628161084386863387611d47565b5190209061176f83612295565b335f90815260cf6020526040902061178c90610884908490610877565b61179581611d6b565b335f90815260ce60205260409020556040517f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e62813391806108da8782611d94565b01611719565b34610349576117e936610752565b906117fb6107c6601080606654161490565b335f5260d160205261181360ff60405f205416611bb9565b61182260026097541415611bcf565b60026097555f5b82811061183a5761055a6001609755565b806118e561184b6001938686611c1b565b335f90815260ce602052604090205460405160208101906118728161084386863387611d47565b5190209061187f83612295565b335f90815260d26020526040902061189c90610884908490610877565b6118a581611d6b565b335f90815260ce60205260409020556040517f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b3391806108da8782611d94565b01611829565b634e487b7160e01b5f52604160045260245ffd5b6080810190811067ffffffffffffffff82111761191b57604052565b6118eb565b90601f8019910116810190811067ffffffffffffffff82111761191b57604052565b60405190610bf2608083611920565b6040519061195e826118ff565b5f6060838281528260208201528260408201520152565b634e487b7160e01b5f52601160045260245ffd5b80156110c1575f190190565b5f198101919082116110c157565b919082039182116110c157565b634e487b7160e01b5f52603260045260245ffd5b60ca548110156119e05760ca5f5260205f209060011b01905f90565b6119b0565b80548210156119e0575f5260205f209060011b01905f90565b90604051611a0b816118ff565b606060ff6001839580548552015463ffffffff8116602085015263ffffffff8160201c16604085015260401c161515910152565b611a47611951565b5060ca54805b611a725750611a5a611942565b5f81525f60208201525f60408201525f606082015290565b611a81610a2d610a2883611995565b90611a926113096060840151151590565b80611ab0575b611aac57611aa69150611989565b80611a4d565b5090565b50611ace611ac5604084015163ffffffff1690565b63ffffffff1690565b421015611a98565b908160209103126103495751611aeb81610338565b90565b6040513d5f823e3d90fd5b15611b0057565b63794821ff60e01b5f5260045ffd5b908160209103126103495751611aeb8161044f565b15611b2b57565b631d77d47760e21b5f5260045ffd5b15611b4157565b63c61dca5d60e01b5f5260045ffd5b35611aeb81610338565b6020813591611b6883610338565b01356040519060208201925f84526001600160601b03199060601b166021830152603582015260358152611b9d605582611920565b51902090565b15611baa57565b63840a48d560e01b5f5260045ffd5b15611bc057565b635c427cd960e01b5f5260045ffd5b15611bd657565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b91908110156119e05760051b81013590609e1981360301821215610349570190565b916020908281520191905f905b808210611c575750505090565b9091928335611c6581610338565b6001600160a01b0316815260208401356001600160601b038116919082900361034957604081600193602083940152019401920190611c4a565b908135601e19833603018112156103495782019060208235920167ffffffffffffffff8311610349578260061b3603811361034957611d3c6080611cf08193611aeb9660a0875260a0870191611c3d565b95611d10611d0060208301610fa3565b6001600160a01b03166020870152565b60408101356040860152611d36611d2960608301610be7565b63ffffffff166060870152565b01610be7565b63ffffffff16910152565b611aeb939260609260018060a01b0316825260208201528160408201520190611c9f565b90600182018092116110c157565b90602082018092116110c157565b919082018092116110c157565b906020611aeb928181520190611c9f565b35611aeb81610bd9565b903590601e1981360301821215610349570180359067ffffffffffffffff821161034957602001918160051b3603831361034957565b903590601e1981360301821215610349570180359067ffffffffffffffff821161034957602001918160061b3603831361034957565b91908110156119e05760061b0190565b15611e3257565b63aa385e8160e01b5f5260045ffd5b15611e4857565b6306957c9160e11b5f5260045ffd5b9063ffffffff8091169116019063ffffffff82116110c157565b60ca54600160401b81101561191b57806001611e92920160ca5560ca6119e5565b919091611f0057611ee160606001610bf29484518155019263ffffffff60208201511684549067ffffffff00000000604084015160201b169167ffffffffffffffff1916171784550151151590565b815460ff60401b191690151560401b68ff000000000000000016179055565b634e487b7160e01b5f525f60045260245ffd5b15611f1a57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b92610f9e61141b94611fdb610d7f94610bf299989660018060a01b03606554161580611fe0575b611fa690612115565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a261212b565b6127f0565b506001600160a01b0382161515611f9d565b63ffffffff5f199116019063ffffffff82116110c157565b63ffffffff60ca54165b63ffffffff811661202e5763504570e360e01b5f5260045ffd5b8161203b610a2883611ff2565b5054146120565763ffffffff1680156110c1575f1901612014565b611aeb9150611ff2565b602081359161206e83610338565b0135604051906020820192600160f81b84526001600160601b03199060601b166021830152603582015260358152611b9d605582611920565b156120ae57565b631b14174b60e01b5f5260045ffd5b6033546001600160a01b031633036120d157565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b1561211c57565b6339b190bb60e11b5f5260045ffd5b6001600160a01b031661213f811515612115565b606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b1561219457565b63796cc52560e01b5f5260045ffd5b156121aa57565b6310eb483f60e21b5f5260045ffd5b156121c057565b63070b5a6f60e21b5f5260045ffd5b156121d657565b630dd0b9f560e21b5f5260045ffd5b9063ffffffff169081156121fd5763ffffffff160690565b634e487b7160e01b5f52601260045260245ffd5b1561221857565b63ee66470560e01b5f5260045ffd5b1561222e57565b633c1a94f160e21b5f5260045ffd5b1561224457565b63041aa75760e11b5f5260045ffd5b1561225a57565b637ee2b44360e01b5f5260045ffd5b1561227057565b632efd965160e11b5f5260045ffd5b1561228657565b63dfad9ca160e01b5f5260045ffd5b6122ab6122a28280611de5565b9050151561218d565b6122d66f4b3b4ca85a86c47a098a223fffffffff60408301356122cf8115156121a3565b11156121b9565b61240b6123d1612325608084016123206122ef82611da5565b63ffffffff807f000000000000000000000000000000000000000000000000000000000000000016911611156121cf565b611da5565b61235f63ffffffff6123587f000000000000000000000000000000000000000000000000000000000000000080946121e5565b1615612211565b61238161237b611ac5606087019361237685611da5565b6121e5565b15612227565b6123b163ffffffff7f000000000000000000000000000000000000000000000000000000000000000016426119a3565b6123bd611ac583611da5565b10158061251a575b6123209095949561223d565b63ffffffff612402817f00000000000000000000000000000000000000000000000000000000000000001642611d87565b91161115612253565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f90815b6124448280611de5565b90508310156125135761246361245e84610aff8580611de5565b611b50565b60405163198f077960e21b81526001600160a01b03821660048201529091906020816024818a5afa92831561058b576001936124c7925f916124f5575b5080156124cf575b6124b190612269565b838060a01b03168092848060a01b03161061227f565b92019161243a565b5060a084901b849003811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0146124a8565b61250d915060203d8111610690576106828183611920565b5f6124a0565b5050509050565b5061232061252a611ac583611da5565b63ffffffff7f000000000000000000000000000000000000000000000000000000000000000016111590506123c5565b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152610bf29161259e608483611920565b612955565b156125aa57565b631437a2bb60e31b5f5260045ffd5b156125c057565b6343714afd60e01b5f5260045ffd5b903590601e1981360301821215610349570180359067ffffffffffffffff82116103495760200191813603831361034957565b91908110156119e05760051b0190565b908210156119e0576126299160051b8101906125cf565b9091565b9190916126486126436113096060860151151590565b6120a7565b61266761265f611ac5604086015163ffffffff1690565b4210156125a3565b60a08101906126768282611daf565b905061269360c083019161268a8385611daf565b919050146125b9565b6126da6126a08284611daf565b9690506126b560e085019761268a8987611de5565b516126c260208501611da5565b6126cf60408601866125cf565b916060870193612a73565b6080820135925f5b6126ec8285611daf565b9050811015612749578061274385610aff8a61273c856127338161272d8c8f61232060019d8f6127219061272794508d611daf565b90612602565b98611daf565b90612612565b9490938c611de5565b928a612abe565b016126e2565b50505050509050565b60405163a9059cbb60e01b60208201526001600160a01b039092166024830152604480830193909352918152610bf29161259e606483611920565b60cb54907faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b36040805163ffffffff8560a01c16815263ffffffff84166020820152a163ffffffff60a01b1990911660a09190911b63ffffffff60a01b161760cb55565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b60cb546001600160a01b0391821691829082167f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb5f80a36001600160a01b0319161760cb55565b60cb54907f8cdc428b0431b82d1619763f443a48197db344ba96905f3949643acd1c863a066040805161ffff8560e01c16815261ffff84166020820152a161ffff60e01b1990911660e09190911b61ffff60e01b161760cb55565b67ffffffffffffffff811161191b57601f01601f191660200190565b156128fd57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b60018060a01b0316906040519061296d604083611920565b602082527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020830152823b156129e2575f816129bd948260208195519301915af16129b7612b6e565b90612b9d565b8051806129c8575050565b816020806129dd93610bf29501019101611b0f565b6128f6565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b929192612a33826128da565b91612a416040519384611920565b829481845281830111610349578281602093845f960137010152565b15612a6457565b6369ca16c960e01b5f5260045ffd5b91929063ffffffff169160018260051c1b831015612ab057612aa6612aab94612a9e610bf297611b5a565b933691612a27565b612af8565b612a5d565b62c6c39d60e71b5f5260045ffd5b91929063ffffffff169160018260051c1b831015612ae957612aa6612aab94612a9e610bf297612060565b63054ff4df60e51b5f5260045ffd5b93909291601f855116612b5f5791906020925b85518411612b565760018316612b3c575f5282850151602052612b3560405f209260011c93611d79565b9291612b0b565b838601515f52602052612b3560405f209260011c93611d79565b92509350501490565b6313717da960e21b5f5260045ffd5b3d15612b98573d90612b7f826128da565b91612b8d6040519384611920565b82523d5f602084013e565b606090565b90919015612ba9575090565b815115612bb95750805190602001fd5b604460209160405192839162461bcd60e51b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fdfea2646970667358221220696395a65d09ad98bc426daca26262bb185cfdbf8cbc74db2f347f0f81cd036164736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01`4a\x02>W`\x1Fa.\x968\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x02BW\x80\x84\x92`\xE0\x94`@R\x839\x81\x01\x03\x12a\x02>W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02>W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02>Wa\0s`@\x83\x01a\x02VV[a\0\x7F``\x84\x01a\x02VV[\x90a\0\x8C`\x80\x85\x01a\x02VV[\x92a\0\xA5`\xC0a\0\x9E`\xA0\x88\x01a\x02VV[\x96\x01a\x02VV[\x95c\xFF\xFF\xFF\xFF\x83\x16\x80\x15a\x02*Wc\xFF\xFF\xFF\xFF\x81\x81\x8A\x16\x06\x16a\x02\x1BWb\x01Q\x80c\xFF\xFF\xFF\xFF\x91\x06\x16a\x02\x0CW`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0Ra\x01 Ra\x01@R_T`\xFF\x81`\x08\x1C\x16a\x01\xB7W`\xFF\x80\x82\x16\x10a\x01}W[`@Qa,.\x90\x81a\x02h\x829`\x80Q\x81a\x14a\x01R`\xA0Q\x81\x81\x81a\tS\x01Ra$\r\x01R`\xC0Q\x81\x81\x81a\x10\xE2\x01Ra#2\x01R`\xE0Q\x81\x81\x81a\x12\x1E\x01Ra\"\xF7\x01Ra\x01\0Q\x81\x81\x81a\t\x1A\x01Ra#\x8B\x01Ra\x01 Q\x81\x81\x81a\x03\xA9\x01Ra#\xDC\x01Ra\x01@Q\x81\x81\x81a\x05\xAC\x01Ra%1\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\x01\x02V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c\"<{9`\xE1\x1B_R`\x04_\xFD[c\x0E\x06\xBD1`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02>WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\x18W,\x14a\x033W\x80c\x04\xA0\xC5\x02\x14a\x03.W\x80c\t-\xB0\x07\x14a\x03)W\x80c\x0E\x9AS\xCF\x14a\x03$W\x80c\x0E\xB3\x83E\x14a\x03\x1FW\x80c\x10\xD6z/\x14a\x03\x1AW\x80c\x13\x143\xB4\x14a\x03\x15W\x80c\x13d9\xDD\x14a\x03\x10W\x80c\x14\x9B\xC8r\x14a\x03\x0BW\x80c\"\xF1\x9Ad\x14a\x03\x06W\x80c+\x9Fd\xA4\x14a\x03\x01W\x80c6\xAFA\xFA\x14a\x02\xFCW\x80c7\x83\x8E\xD0\x14a\x02\xF7W\x80c9\xB7\x0E8\x14a\x02\xF2W\x80c:\x8C\x07\x86\x14a\x02\xEDW\x80c<\xCC\x86\x1D\x14a\x02\xE8W\x80c>\xFE\x1D\xB6\x14a\x02\xE3W\x80cM\x18\xCC5\x14a\x02\xDEW\x80cX\xBA\xAA>\x14a\x02\xD9W\x80cY\\jg\x14a\x02\xD4W\x80cZ\xC8j\xB7\x14a\x02\xCFW\x80c\\\x97Z\xBB\x14a\x02\xCAW\x80c^\x9D\x83H\x14a\x02\xC5W\x80cm!\x11~\x14a\x02\xC0W\x80cqP\x18\xA6\x14a\x02\xBBW\x80c{\x8F\x8B\x05\x14a\x02\xB6W\x80c\x86<\xB9\xA9\x14a\x02\xB1W\x80c\x86\\iS\x14a\x02\xACW\x80c\x88o\x11\x95\x14a\x02\xA7W\x80c\x8D\xA5\xCB[\x14a\x02\xA2W\x80c\x91\x04\xC3\x19\x14a\x02\x9DW\x80c\x9B\xE3\xD4\xE4\x14a\x02\x98W\x80c\x9DE\xC2\x81\x14a\x02\x93W\x80c\xA0\x16\x9D\xDD\x14a\x02\x8EW\x80c\xAE\xBD\x8B\xAE\x14a\x02\x89W\x80c\xBB~E\x1F\x14a\x02\x84W\x80c\xBF!\xA8\xAA\x14a\x02\x7FW\x80c\xC4m\xB6\x06\x14a\x02zW\x80c\xD4T\nU\x14a\x02uW\x80c\xDE\x02\xE5\x03\x14a\x02pW\x80c\xE2!\xB2E\x14a\x02kW\x80c\xE8\x10\xCE!\x14a\x02fW\x80c\xEAM<\x9B\x14a\x02aW\x80c\xF2\xFD\xE3\x8B\x14a\x02\\W\x80c\xF8\xCD\x84H\x14a\x02WW\x80c\xF9j\xBF.\x14a\x02RW\x80c\xFA\xBC\x1C\xBC\x14a\x02MW\x80c\xFB\xF1\xE2\xC1\x14a\x02HW\x80c\xFC\xE3l}\x14a\x02CWc\xFF\x9Fl\xCE\x14a\x02>W_\x80\xFD[a\x17\xDBV[a\x16\xE3V[a\x16\xBBV[a\x16\x1FV[a\x15EV[a\x15!V[a\x14\x90V[a\x14LV[a\x14 V[a\x13\xF0V[a\x13\xC5V[a\x12\xA1V[a\x12BV[a\x12\x02V[a\x11\xC7V[a\x11yV[a\x11\x06V[a\x10\xC6V[a\x10\x8BV[a\x10]V[a\x105V[a\x10\rV[a\x0F\xAEV[a\x0FvV[a\x0FYV[a\x0E\xFEV[a\x0E\xB0V[a\x0EZV[a\x0E=V[a\x0E\nV[a\r\x84V[a\rWV[a\r1V[a\x0B\xF4V[a\t\xB7V[a\t\x82V[a\t>V[a\x08\xFEV[a\x07\xA6V[a\x07\x0FV[a\x06\xD4V[a\x06\xA8V[a\x05\xD0V[a\x05\x90V[a\x04\xECV[a\x04YV[a\x03\xF1V[a\x03\xCDV[a\x03\x8DV[a\x03MV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03IWV[_\x80\xFD[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x03j\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD1` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x04Ka\x04\x0Ca\x1A?V[`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x85\x01R\x01Q\x15\x15\x91\x01RV[\x03\x90\xF3[\x80\x15\x15\x03a\x03IWV[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x04v\x81a\x038V[`$5\x90a\x04\x83\x82a\x04OV[a\x04\x8Ba \xBDV[`\x01\x80`\xA0\x1B\x03\x16\x80_R`\xD1` R`\xFF`@_ T\x16\x15\x15\x82\x15\x15\x90\x82\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C_\x80\xA4_R`\xD1` R`@_ \x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90U_\x80\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x04\x805a\x05\n\x81a\x038V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x8BWa\x05Z\x92a\x05U\x91_\x91a\x05\\W[P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xF9V[a!+V[\0[a\x05~\x91P` =` \x11a\x05\x84W[a\x05v\x81\x83a\x19 V[\x81\x01\x90a\x1A\xD6V[_a\x05DV[P=a\x05lV[a\x1A\xEEV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\x06'\x91_\x91a\x06hW[Pa\x1B$V[a\x066`fT\x82\x81\x16\x14a\x1B:V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x06\x8A\x91P` =` \x11a\x06\x90W[a\x06\x82\x81\x83a\x19 V[\x81\x01\x90a\x1B\x0FV[_a\x06!V[P=a\x06xV[`@\x90`\x03\x19\x01\x12a\x03IW`\x04\x90V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x06\xCCa\x06\xC76a\x06\x97V[a\x1BZV[`@Q\x90\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IWa\x06\xF0`\x045a\x038V[a\x06\xFB`$5a\x038V[` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x07,\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[\x90` `\x03\x19\x83\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IW\x82`#\x82\x01\x12\x15a\x03IW\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\x03IW`$\x84`\x05\x1B\x83\x01\x01\x11a\x03IW`$\x01\x91\x90V[4a\x03IWa\x07\xB46a\x07RV[\x90a\x07\xCCa\x07\xC6`\x02\x80`fT\x16\x14\x90V[\x15a\x1B\xA3V[3_R`\xD1` Ra\x07\xE4`\xFF`@_ T\x16a\x1B\xB9V[a\x07\xF3`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x08\x0BWa\x05Z`\x01`\x97UV[\x80a\x08\xF8a\x08\x1C`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x08Q\x81a\x08C\x86\x863\x87a\x1DGV[\x03`\x1F\x19\x81\x01\x83R\x82a\x19 V[Q\x90 \x90a\x08^\x83a\"\x95V[3_\x90\x81R`\xD0` R`@\x90 a\x08\x91\x90a\x08\x84\x90\x84\x90[\x90_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x08\x9A\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x823\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x03\x90\xA4`@a\x08\xEB` \x83\x01a\x1BPV[\x91\x015\x900\x903\x90a%ZV[\x01a\x07\xFAV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16`@Q\x90\x81R\xF3[\x90\x81a\x01\0\x91\x03\x12a\x03IW\x90V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IWa\t\xE8\x906\x90`\x04\x01a\t\xA8V[`$5a\t\xF4\x81a\x038V[a\n\x05a\x07\xC6`\x04\x80`fT\x16\x14\x90V[a\n\x14`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97Ua\n3a\n-a\n(\x84a\x1D\xA5V[a\x19\xC4V[Pa\x19\xFEV[\x90a\n>\x82\x84a&-V[a\nJ``\x84\x01a\x1BPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xCC` R`@\x90 T\x16\x80\x15a\x0B\xD2W[\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90a\n\x833\x83\x14a\x1B\xB9V[`\xA0\x85\x01\x93`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x94\x90\x83\x16\x92\x90\x91`\xE0\x88\x01\x91_[a\n\xAB\x89\x8Ba\x1D\xAFV[\x90P\x81\x10\x15a\x0B\xC8W`\x01\x81\x89\x89\x89\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x8F\x89a\x0B\xBDa\x0B\x96\x8Ea\x0B\x8F\x8Fa\x0B\x05\x8F\x91\x9Ea\n\xAB\x9Fa\n\xFF\x90a\x0Bw\x9Aa\x1D\xE5V[\x90a\x1E\x1BV[\x95a\x0B@a\x0B#\x83`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a\x0B,\x89a\x1BPV[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta\x0B\x80a\x0B^` \x8A\x015\x92a\x0BX\x81\x85\x11a\x1E+V[\x83a\x19\xA3V[\x99\x8A\x94`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a\x0B,\x8Aa\x1BPV[Ua\x0B\x8A\x87a\x1BPV[a'RV[Q\x92a\x1BPV[`@\x80Q\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x84\x01R\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4\x01\x90Pa\n\xA1V[a\x05Z`\x01`\x97UV[P\x80a\nkV[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03IWV[5\x90a\x0B\xF2\x82a\x0B\xD9V[V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`$5`\x045a\x0C\x14\x82a\x0B\xD9V[a\x0C%a\x07\xC6`\x08\x80`fT\x16\x14\x90V[`\xCBT\x91a\x0C=3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x1B\xB9V[c\xFF\xFF\xFF\xFF\x81\x16\x92c\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16\x84\x11\x15a\r\"Wc\xFF\xFF\xFF\xFF\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91a\x0C\x88B\x87\x10a\x1EAV[a\r\x0Ca\x0C\xB5a\x0C\xADa\x0C\xA0`\xCATc\xFF\xFF\xFF\xFF\x16\x90V[\x93`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x84B\x16a\x1EWV[\x94a\x0C\xE7a\x0C\xC1a\x19BV[\x88\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`@\x82\x01R_``\x82\x01Ra\x1EqV[`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\xC0\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x91\x90\x91\x17\x90UV[`@Qc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R\x16\x91` \x90\xA4\0[c\x1C\xA7\xE5\x0B`\xE2\x1B_R`\x04_\xFD[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` c\xFF\xFF\xFF\xFF`\xCBT`\xC0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x05Z`\x045a\rw\x81a\x0B\xD9V[a\r\x7Fa \xBDV[a'\x8DV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\r\xD6\x91_\x91a\x06hWPa\x1B$V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045`\xFF\x81\x16\x80\x91\x03a\x03IW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `fT`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IWa\x0E\x8Ea\x0E\xA5\x916\x90`\x04\x01a\t\xA8V[a\x0E\x9Fa\n-\x825a\n(\x81a\x0B\xD9V[\x90a&-V[` `@Q`\x01\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x0E\xCD\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xCF` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x0F\x16a \xBDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `\xCAT`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x05Z`\x045a\x0F\x96\x81a\x038V[a\x0F\x9Ea \xBDV[a(8V[5\x90a\x0B\xF2\x82a\x038V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x10\x04`\x045a\x0F\xD0\x81a\x038V[`$5\x90a\x0F\xDD\x82a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCD\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x10\xA3a\x19QV[P`\xCAT_\x19\x81\x01\x90\x81\x11a\x10\xC1Wa\x04\x0Ca\n-a\x04K\x92a\x19\xC4V[a\x19uV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x11#\x81a\x038V[3_\x81\x81R`\xCC` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x93\x16\x91\x90\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x90\x80\xA4\0[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x11\x96\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD2` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x11\xE4\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCE` R` `@_ T`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x12_\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD0` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[`\xA45\x90a\xFF\xFF\x82\x16\x82\x03a\x03IWV[4a\x03IW`\xC06`\x03\x19\x01\x12a\x03IW`\x045a\x12\xBE\x81a\x038V[a\x13=`$5a\x12\xCD\x81a\x038V[`D5`d5a\x12\xDC\x81a\x038V[`\x845\x91a\x12\xE9\x83a\x0B\xD9V[a\x12\xF1a\x12\x90V[\x93_T\x96a\x13#a\x13\ra\x13\t\x8A`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x99\x81\x9Aa\x13\xB7W[\x81\x15a\x13\x97W[Pa\x1F\x13V[\x87a\x134`\x01`\xFF\x19_T\x16\x17_UV[a\x13\x80Wa\x1FvV[a\x13CW\0[a\x13Qa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x13\x92a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x1FvV[0;\x15\x91P\x81a\x13\xA9W[P_a\x13\x1DV[`\xFF\x16`\x01\x14\x90P_a\x13\xA2V[`\x01`\xFF\x82\x16\x10\x91Pa\x13\x16V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x04Ka\x04\x0Ca\n-`\x045a\x13\xEAa\x19QV[Pa\x19\xC4V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03IWa\x05Z\x90a\x14\x1Ba \xBDV[a(\x7FV[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW` a\x14>`\x045a \nV[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x14\xAD\x81a\x038V[a\x14\xB5a \xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x14\xCDWa\x05Z\x90a'\xF0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x06\xCCa\x15@6a\x06\x97V[a `V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x15b\x81a\x0B\xD9V[a\x15sa\x07\xC6`\x08\x80`fT\x16\x14\x90V[a\x15\x88`\x01\x80`\xA0\x1B\x03`\xCBT\x163\x14a\x1B\xB9V[`\xCAT\x90c\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x10\x15a\x16\x10Wa\x15\xA6`\x01\x91a\x19\xC4V[P\x01c\xFF\xFF\xFF\xFF\x81Ta\x15\xBF`\xFF\x82`@\x1C\x16\x15a \xA7V[` \x1C\x16B\x10\x15a\x16\x01W\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E_\x80\xA2\0[c\x0C6\xF6e`\xE2\x1B_R`\x04_\xFD[c\x94\xA8\xD3\x89`\xE0\x1B_R`\x04_\xFD[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\x16x\x91_\x91a\x05\\WP`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xF9V[a\x16\x89`fT\x19\x82\x19\x81\x16\x14a\x1B:V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IWa\x16\xF16a\x07RV[\x90a\x17\x03a\x07\xC6`\x01\x80`fT\x16\x14\x90V[a\x17\x12`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x17*Wa\x05Z`\x01`\x97UV[\x80a\x17\xD5a\x17;`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x17b\x81a\x08C\x86\x863\x87a\x1DGV[Q\x90 \x90a\x17o\x83a\"\x95V[3_\x90\x81R`\xCF` R`@\x90 a\x17\x8C\x90a\x08\x84\x90\x84\x90a\x08wV[a\x17\x95\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x813\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x01a\x17\x19V[4a\x03IWa\x17\xE96a\x07RV[\x90a\x17\xFBa\x07\xC6`\x10\x80`fT\x16\x14\x90V[3_R`\xD1` Ra\x18\x13`\xFF`@_ T\x16a\x1B\xB9V[a\x18\"`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x18:Wa\x05Z`\x01`\x97UV[\x80a\x18\xE5a\x18K`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x18r\x81a\x08C\x86\x863\x87a\x1DGV[Q\x90 \x90a\x18\x7F\x83a\"\x95V[3_\x90\x81R`\xD2` R`@\x90 a\x18\x9C\x90a\x08\x84\x90\x84\x90a\x08wV[a\x18\xA5\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B3\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x01a\x18)V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x19\x1BW`@RV[a\x18\xEBV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x19\x1BW`@RV[`@Q\x90a\x0B\xF2`\x80\x83a\x19 V[`@Q\x90a\x19^\x82a\x18\xFFV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x10\xC1W_\x19\x01\x90V[_\x19\x81\x01\x91\x90\x82\x11a\x10\xC1WV[\x91\x90\x82\x03\x91\x82\x11a\x10\xC1WV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xCAT\x81\x10\x15a\x19\xE0W`\xCA_R` _ \x90`\x01\x1B\x01\x90_\x90V[a\x19\xB0V[\x80T\x82\x10\x15a\x19\xE0W_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90`@Qa\x1A\x0B\x81a\x18\xFFV[```\xFF`\x01\x83\x95\x80T\x85R\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x85\x01Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16`@\x85\x01R`@\x1C\x16\x15\x15\x91\x01RV[a\x1AGa\x19QV[P`\xCAT\x80[a\x1ArWPa\x1AZa\x19BV[_\x81R_` \x82\x01R_`@\x82\x01R_``\x82\x01R\x90V[a\x1A\x81a\n-a\n(\x83a\x19\x95V[\x90a\x1A\x92a\x13\t``\x84\x01Q\x15\x15\x90V[\x80a\x1A\xB0W[a\x1A\xACWa\x1A\xA6\x91Pa\x19\x89V[\x80a\x1AMV[P\x90V[Pa\x1A\xCEa\x1A\xC5`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a\x1A\x98V[\x90\x81` \x91\x03\x12a\x03IWQa\x1A\xEB\x81a\x038V[\x90V[`@Q=_\x82>=\x90\xFD[\x15a\x1B\0WV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03IWQa\x1A\xEB\x81a\x04OV[\x15a\x1B+WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1BAWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[5a\x1A\xEB\x81a\x038V[` \x815\x91a\x1Bh\x83a\x038V[\x015`@Q\x90` \x82\x01\x92_\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra\x1B\x9D`U\x82a\x19 V[Q\x90 \x90V[\x15a\x1B\xAAWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xC0WV[c\\B|\xD9`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xD6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x19\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x90V[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\x1CWWPPP\x90V[\x90\x91\x92\x835a\x1Ce\x81a\x038V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x015`\x01`\x01``\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x03IW`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a\x1CJV[\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03IW\x82\x01\x90` \x825\x92\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03IW\x82`\x06\x1B6\x03\x81\x13a\x03IWa\x1D<`\x80a\x1C\xF0\x81\x93a\x1A\xEB\x96`\xA0\x87R`\xA0\x87\x01\x91a\x1C=V[\x95a\x1D\x10a\x1D\0` \x83\x01a\x0F\xA3V[`\x01`\x01`\xA0\x1B\x03\x16` \x87\x01RV[`@\x81\x015`@\x86\x01Ra\x1D6a\x1D)``\x83\x01a\x0B\xE7V[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x0B\xE7V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x1A\xEB\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x1C\x9FV[\x90`\x01\x82\x01\x80\x92\x11a\x10\xC1WV[\x90` \x82\x01\x80\x92\x11a\x10\xC1WV[\x91\x90\x82\x01\x80\x92\x11a\x10\xC1WV[\x90` a\x1A\xEB\x92\x81\x81R\x01\x90a\x1C\x9FV[5a\x1A\xEB\x81a\x0B\xD9V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03IWV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x03IWV[\x91\x90\x81\x10\x15a\x19\xE0W`\x06\x1B\x01\x90V[\x15a\x1E2WV[c\xAA8^\x81`\xE0\x1B_R`\x04_\xFD[\x15a\x1EHWV[c\x06\x95|\x91`\xE1\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\xC1WV[`\xCAT`\x01`@\x1B\x81\x10\x15a\x19\x1BW\x80`\x01a\x1E\x92\x92\x01`\xCAU`\xCAa\x19\xE5V[\x91\x90\x91a\x1F\0Wa\x1E\xE1```\x01a\x0B\xF2\x94\x84Q\x81U\x01\x92c\xFF\xFF\xFF\xFF` \x82\x01Q\x16\x84T\x90g\xFF\xFF\xFF\xFF\0\0\0\0`@\x84\x01Q` \x1B\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17\x84U\x01Q\x15\x15\x90V[\x81T`\xFF`@\x1B\x19\x16\x90\x15\x15`@\x1Bh\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x15a\x1F\x1AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x92a\x0F\x9Ea\x14\x1B\x94a\x1F\xDBa\r\x7F\x94a\x0B\xF2\x99\x98\x96`\x01\x80`\xA0\x1B\x03`eT\x16\x15\x80a\x1F\xE0W[a\x1F\xA6\x90a!\x15V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a!+V[a'\xF0V[P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15a\x1F\x9DV[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\xC1WV[c\xFF\xFF\xFF\xFF`\xCAT\x16[c\xFF\xFF\xFF\xFF\x81\x16a .WcPEp\xE3`\xE0\x1B_R`\x04_\xFD[\x81a ;a\n(\x83a\x1F\xF2V[PT\x14a VWc\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\xC1W_\x19\x01a \x14V[a\x1A\xEB\x91Pa\x1F\xF2V[` \x815\x91a n\x83a\x038V[\x015`@Q\x90` \x82\x01\x92`\x01`\xF8\x1B\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra\x1B\x9D`U\x82a\x19 V[\x15a \xAEWV[c\x1B\x14\x17K`\xE0\x1B_R`\x04_\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a \xD1WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x15a!\x1CWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16a!?\x81\x15\x15a!\x15V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[\x15a!\x94WV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a!\xAAWV[c\x10\xEBH?`\xE2\x1B_R`\x04_\xFD[\x15a!\xC0WV[c\x07\x0BZo`\xE2\x1B_R`\x04_\xFD[\x15a!\xD6WV[c\r\xD0\xB9\xF5`\xE2\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x16\x90\x81\x15a!\xFDWc\xFF\xFF\xFF\xFF\x16\x06\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\"\x18WV[c\xEEfG\x05`\xE0\x1B_R`\x04_\xFD[\x15a\".WV[c<\x1A\x94\xF1`\xE2\x1B_R`\x04_\xFD[\x15a\"DWV[c\x04\x1A\xA7W`\xE1\x1B_R`\x04_\xFD[\x15a\"ZWV[c~\xE2\xB4C`\xE0\x1B_R`\x04_\xFD[\x15a\"pWV[c.\xFD\x96Q`\xE1\x1B_R`\x04_\xFD[\x15a\"\x86WV[c\xDF\xAD\x9C\xA1`\xE0\x1B_R`\x04_\xFD[a\"\xABa\"\xA2\x82\x80a\x1D\xE5V[\x90P\x15\x15a!\x8DV[a\"\xD6oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF`@\x83\x015a\"\xCF\x81\x15\x15a!\xA3V[\x11\x15a!\xB9V[a$\x0Ba#\xD1a#%`\x80\x84\x01a# a\"\xEF\x82a\x1D\xA5V[c\xFF\xFF\xFF\xFF\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x16\x11\x15a!\xCFV[a\x1D\xA5V[a#_c\xFF\xFF\xFF\xFFa#X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x94a!\xE5V[\x16\x15a\"\x11V[a#\x81a#{a\x1A\xC5``\x87\x01\x93a#v\x85a\x1D\xA5V[a!\xE5V[\x15a\"'V[a#\xB1c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x19\xA3V[a#\xBDa\x1A\xC5\x83a\x1D\xA5V[\x10\x15\x80a%\x1AW[a# \x90\x95\x94\x95a\"=V[c\xFF\xFF\xFF\xFFa$\x02\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x1D\x87V[\x91\x16\x11\x15a\"SV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_\x90\x81[a$D\x82\x80a\x1D\xE5V[\x90P\x83\x10\x15a%\x13Wa$ca$^\x84a\n\xFF\x85\x80a\x1D\xE5V[a\x1BPV[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x8AZ\xFA\x92\x83\x15a\x05\x8BW`\x01\x93a$\xC7\x92_\x91a$\xF5W[P\x80\x15a$\xCFW[a$\xB1\x90a\"iV[\x83\x80`\xA0\x1B\x03\x16\x80\x92\x84\x80`\xA0\x1B\x03\x16\x10a\"\x7FV[\x92\x01\x91a$:V[P`\xA0\x84\x90\x1B\x84\x90\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a$\xA8V[a%\r\x91P` =\x81\x11a\x06\x90Wa\x06\x82\x81\x83a\x19 V[_a$\xA0V[PPP\x90PV[Pa# a%*a\x1A\xC5\x83a\x1D\xA5V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x11\x15\x90Pa#\xC5V[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x0B\xF2\x91a%\x9E`\x84\x83a\x19 V[a)UV[\x15a%\xAAWV[c\x147\xA2\xBB`\xE3\x1B_R`\x04_\xFD[\x15a%\xC0WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x816\x03\x83\x13a\x03IWV[\x91\x90\x81\x10\x15a\x19\xE0W`\x05\x1B\x01\x90V[\x90\x82\x10\x15a\x19\xE0Wa&)\x91`\x05\x1B\x81\x01\x90a%\xCFV[\x90\x91V[\x91\x90\x91a&Ha&Ca\x13\t``\x86\x01Q\x15\x15\x90V[a \xA7V[a&ga&_a\x1A\xC5`@\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a%\xA3V[`\xA0\x81\x01\x90a&v\x82\x82a\x1D\xAFV[\x90Pa&\x93`\xC0\x83\x01\x91a&\x8A\x83\x85a\x1D\xAFV[\x91\x90P\x14a%\xB9V[a&\xDAa&\xA0\x82\x84a\x1D\xAFV[\x96\x90Pa&\xB5`\xE0\x85\x01\x97a&\x8A\x89\x87a\x1D\xE5V[Qa&\xC2` \x85\x01a\x1D\xA5V[a&\xCF`@\x86\x01\x86a%\xCFV[\x91``\x87\x01\x93a*sV[`\x80\x82\x015\x92_[a&\xEC\x82\x85a\x1D\xAFV[\x90P\x81\x10\x15a'IW\x80a'C\x85a\n\xFF\x8Aa'<\x85a'3\x81a'-\x8C\x8Fa# `\x01\x9D\x8Fa'!\x90a''\x94P\x8Da\x1D\xAFV[\x90a&\x02V[\x98a\x1D\xAFV[\x90a&\x12V[\x94\x90\x93\x8Ca\x1D\xE5V[\x92\x8Aa*\xBEV[\x01a&\xE2V[PPPPP\x90PV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x0B\xF2\x91a%\x9E`d\x83a\x19 V[`\xCBT\x90\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3`@\x80Qc\xFF\xFF\xFF\xFF\x85`\xA0\x1C\x16\x81Rc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R\xA1c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17`\xCBUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x82\x90\x82\x16\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB_\x80\xA3`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`\xCBUV[`\xCBT\x90\x7F\x8C\xDCB\x8B\x041\xB8-\x16\x19v?D:H\x19}\xB3D\xBA\x96\x90_9Id:\xCD\x1C\x86:\x06`@\x80Qa\xFF\xFF\x85`\xE0\x1C\x16\x81Ra\xFF\xFF\x84\x16` \x82\x01R\xA1a\xFF\xFF`\xE0\x1B\x19\x90\x91\x16`\xE0\x91\x90\x91\x1Ba\xFF\xFF`\xE0\x1B\x16\x17`\xCBUV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x19\x1BW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x15a(\xFDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90`@Q\x90a)m`@\x83a\x19 V[` \x82R\x7FSafeERC20: low-level call failed` \x83\x01R\x82;\x15a)\xE2W_\x81a)\xBD\x94\x82` \x81\x95Q\x93\x01\x91Z\xF1a)\xB7a+nV[\x90a+\x9DV[\x80Q\x80a)\xC8WPPV[\x81` \x80a)\xDD\x93a\x0B\xF2\x95\x01\x01\x91\x01a\x1B\x0FV[a(\xF6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x92\x91\x92a*3\x82a(\xDAV[\x91a*A`@Q\x93\x84a\x19 V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03IW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x15a*dWV[ci\xCA\x16\xC9`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a*\xB0Wa*\xA6a*\xAB\x94a*\x9Ea\x0B\xF2\x97a\x1BZV[\x936\x91a*'V[a*\xF8V[a*]V[b\xC6\xC3\x9D`\xE7\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a*\xE9Wa*\xA6a*\xAB\x94a*\x9Ea\x0B\xF2\x97a `V[c\x05O\xF4\xDF`\xE5\x1B_R`\x04_\xFD[\x93\x90\x92\x91`\x1F\x85Q\x16a+_W\x91\x90` \x92[\x85Q\x84\x11a+VW`\x01\x83\x16a+<W_R\x82\x85\x01Q` Ra+5`@_ \x92`\x01\x1C\x93a\x1DyV[\x92\x91a+\x0BV[\x83\x86\x01Q_R` Ra+5`@_ \x92`\x01\x1C\x93a\x1DyV[\x92P\x93PP\x14\x90V[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[=\x15a+\x98W=\x90a+\x7F\x82a(\xDAV[\x91a+\x8D`@Q\x93\x84a\x19 V[\x82R=_` \x84\x01>V[``\x90V[\x90\x91\x90\x15a+\xA9WP\x90V[\x81Q\x15a+\xB9WP\x80Q\x90` \x01\xFD[`D` \x91`@Q\x92\x83\x91bF\x1B\xCD`\xE5\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD\xFE\xA2dipfsX\"\x12 ic\x95\xA6]\t\xAD\x98\xBCBm\xAC\xA2bb\xBB\x18\\\xFD\xBF\x8C\xBCt\xDB/4\x7F\x0F\x81\xCD\x03adsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806218572c1461033357806304a0c5021461032e578063092db007146103295780630e9a53cf146103245780630eb383451461031f57806310d67a2f1461031a578063131433b414610315578063136439dd14610310578063149bc8721461030b57806322f19a64146103065780632b9f64a41461030157806336af41fa146102fc57806337838ed0146102f757806339b70e38146102f25780633a8c0786146102ed5780633ccc861d146102e85780633efe1db6146102e35780634d18cc35146102de57806358baaa3e146102d9578063595c6a67146102d45780635ac86ab7146102cf5780635c975abb146102ca5780635e9d8348146102c55780636d21117e146102c0578063715018a6146102bb5780637b8f8b05146102b6578063863cb9a9146102b1578063865c6953146102ac578063886f1195146102a75780638da5cb5b146102a25780639104c3191461029d5780639be3d4e4146102985780639d45c28114610293578063a0169ddd1461028e578063aebd8bae14610289578063bb7e451f14610284578063bf21a8aa1461027f578063c46db6061461027a578063d4540a5514610275578063de02e50314610270578063e221b2451461026b578063e810ce2114610266578063ea4d3c9b14610261578063f2fde38b1461025c578063f8cd844814610257578063f96abf2e14610252578063fabc1cbc1461024d578063fbf1e2c114610248578063fce36c7d146102435763ff9f6cce1461023e575f80fd5b6117db565b6116e3565b6116bb565b61161f565b611545565b611521565b611490565b61144c565b611420565b6113f0565b6113c5565b6112a1565b611242565b611202565b6111c7565b611179565b611106565b6110c6565b61108b565b61105d565b611035565b61100d565b610fae565b610f76565b610f59565b610efe565b610eb0565b610e5a565b610e3d565b610e0a565b610d84565b610d57565b610d31565b610bf4565b6109b7565b610982565b61093e565b6108fe565b6107a6565b61070f565b6106d4565b6106a8565b6105d0565b610590565b6104ec565b610459565b6103f1565b6103cd565b61038d565b61034d565b6001600160a01b0381160361034957565b5f80fd5b346103495760203660031901126103495760043561036a81610338565b60018060a01b03165f5260d1602052602060ff60405f2054166040519015158152f35b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610349575f36600319011261034957602061ffff60cb5460e01c16604051908152f35b34610349575f3660031901126103495761044b61040c611a3f565b6040519182918291909160608060808301948051845263ffffffff602082015116602085015263ffffffff604082015116604085015201511515910152565b0390f35b8015150361034957565b346103495760403660031901126103495760043561047681610338565b602435906104838261044f565b61048b6120bd565b60018060a01b0316805f5260d160205260ff60405f205416151582151590827f4de6293e668df1398422e1def12118052c1539a03cbfedc145895d48d7685f1c5f80a45f5260d160205260405f209060ff8019835416911515161790555f80f35b34610349576020366003190112610349576004803561050a81610338565b60655460405163755b36bd60e11b81529260209184919082906001600160a01b03165afa91821561058b5761055a92610555915f9161055c575b506001600160a01b03163314611af9565b61212b565b005b61057e915060203d602011610584575b6105768183611920565b810190611ad6565b5f610544565b503d61056c565b611aee565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760203660031901126103495760043560655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561058b57610627915f91610668575b50611b24565b61063660665482811614611b3a565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b61068a915060203d602011610690575b6106828183611920565b810190611b0f565b5f610621565b503d610678565b604090600319011261034957600490565b346103495760403660031901126103495760206106cc6106c736610697565b611b5a565b604051908152f35b34610349576040366003190112610349576106f0600435610338565b6106fb602435610338565b602061ffff60cb5460e01c16604051908152f35b346103495760203660031901126103495760043561072c81610338565b60018060a01b03165f5260cc602052602060018060a01b0360405f205416604051908152f35b9060206003198301126103495760043567ffffffffffffffff811161034957826023820112156103495780600401359267ffffffffffffffff84116103495760248460051b83010111610349576024019190565b34610349576107b436610752565b906107cc6107c6600280606654161490565b15611ba3565b335f5260d16020526107e460ff60405f205416611bb9565b6107f360026097541415611bcf565b60026097555f5b82811061080b5761055a6001609755565b806108f861081c6001938686611c1b565b335f90815260ce602052604090205460405160208101906108518161084386863387611d47565b03601f198101835282611920565b5190209061085e83612295565b335f90815260d060205260409020610891906108849084905b905f5260205260405f2090565b805460ff19166001179055565b61089a81611d6b565b335f90815260ce60205260409020556040517f51088b8c89628df3a8174002c2a034d0152fce6af8415d651b2a4734bf2704823391806108da8782611d94565b0390a460406108eb60208301611b50565b910135903090339061255a565b016107fa565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610349575f366003190112610349576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610349575f36600319011261034957602063ffffffff60cb5460a01c16604051908152f35b90816101009103126103495790565b346103495760403660031901126103495760043567ffffffffffffffff8111610349576109e89036906004016109a8565b6024356109f481610338565b610a056107c6600480606654161490565b610a1460026097541415611bcf565b6002609755610a33610a2d610a2884611da5565b6119c4565b506119fe565b90610a3e828461262d565b610a4a60608401611b50565b6001600160a01b038181165f90815260cc6020526040902054168015610bd2575b9192916001600160a01b031690610a83338314611bb9565b60a08501936001600160a01b038181169490831692909160e08801915f5b610aab898b611daf565b9050811015610bc8576001818989897f9543dbd55580842586a951f0386e24d68a5df99ae29e3b216588b45fd684ce318f89610bbd610b968e610b8f8f610b058f919e610aab9f610aff90610b779a611de5565b90611e1b565b95610b40610b238360018060a01b03165f5260cd60205260405f2090565b610b2c89611b50565b60018060a01b03165f5260205260405f2090565b54610b80610b5e60208a013592610b58818511611e2b565b836119a3565b998a9460018060a01b03165f5260cd60205260405f2090565b610b2c8a611b50565b55610b8a87611b50565b612752565b5192611b50565b604080519384526001600160a01b0390911660208401528201929092529081906060820190565b0390a4019050610aa1565b61055a6001609755565b5080610a6b565b63ffffffff81160361034957565b3590610bf282610bd9565b565b3461034957604036600319011261034957602435600435610c1482610bd9565b610c256107c6600880606654161490565b60cb5491610c3d336001600160a01b03851614611bb9565b63ffffffff81169263ffffffff8160c01c16841115610d225763ffffffff7fecd866c3c158fa00bf34d803d5f6023000b57080bcb48af004c2b4b46b3afd0891610c88428710611e41565b610d0c610cb5610cad610ca060ca5463ffffffff1690565b9360a01c63ffffffff1690565b844216611e57565b94610ce7610cc1611942565b88815263ffffffff8316602082015263ffffffff881660408201525f6060820152611e71565b60cb805463ffffffff60c01b191660c09290921b63ffffffff60c01b16919091179055565b60405163ffffffff9490941684521691602090a4005b631ca7e50b60e21b5f5260045ffd5b34610349575f36600319011261034957602063ffffffff60cb5460c01c16604051908152f35b346103495760203660031901126103495761055a600435610d7781610bd9565b610d7f6120bd565b61278d565b34610349575f3660031901126103495760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561058b57610dd6915f916106685750611b24565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b346103495760203660031901126103495760043560ff81168091036103495760016020911b806066541614604051908152f35b34610349575f366003190112610349576020606654604051908152f35b346103495760203660031901126103495760043567ffffffffffffffff811161034957610e8e610ea59136906004016109a8565b610e9f610a2d8235610a2881610bd9565b9061262d565b602060405160018152f35b3461034957604036600319011261034957600435610ecd81610338565b6024359060018060a01b03165f5260cf60205260405f20905f52602052602060ff60405f2054166040519015158152f35b34610349575f36600319011261034957610f166120bd565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610349575f36600319011261034957602060ca54604051908152f35b346103495760203660031901126103495761055a600435610f9681610338565b610f9e6120bd565b612838565b3590610bf282610338565b34610349576040366003190112610349576020611004600435610fd081610338565b60243590610fdd82610338565b60018060a01b03165f5260cd835260405f209060018060a01b03165f5260205260405f2090565b54604051908152f35b34610349575f366003190112610349576065546040516001600160a01b039091168152602090f35b34610349575f366003190112610349576033546040516001600160a01b039091168152602090f35b34610349575f36600319011261034957602060405173beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac08152f35b34610349575f366003190112610349576110a3611951565b5060ca545f1981019081116110c15761040c610a2d61044b926119c4565b611975565b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760203660031901126103495760043561112381610338565b335f81815260cc6020526040812080546001600160a01b039485166001600160a01b03198216811790925590931691907fbab947934d42e0ad206f25c9cab18b5bb6ae144acfb00f40b4e3aa59590ca3129080a4005b346103495760403660031901126103495760043561119681610338565b6024359060018060a01b03165f5260d260205260405f20905f52602052602060ff60405f2054166040519015158152f35b34610349576020366003190112610349576004356111e481610338565b60018060a01b03165f5260ce602052602060405f2054604051908152f35b34610349575f36600319011261034957602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346103495760403660031901126103495760043561125f81610338565b6024359060018060a01b03165f5260d060205260405f20905f52602052602060ff60405f2054166040519015158152f35b60a4359061ffff8216820361034957565b346103495760c0366003190112610349576004356112be81610338565b61133d6024356112cd81610338565b6044356064356112dc81610338565b608435916112e983610bd9565b6112f1611290565b935f549661132361130d6113098a60ff9060081c1690565b1590565b8099819a6113b7575b8115611397575b50611f13565b87611334600160ff195f5416175f55565b61138057611f76565b61134357005b61135161ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890602090a1005b61139261010061ff00195f5416175f55565b611f76565b303b159150816113a9575b505f61131d565b60ff1660011490505f6113a2565b600160ff8216109150611316565b346103495760203660031901126103495761044b61040c610a2d6004356113ea611951565b506119c4565b346103495760203660031901126103495760043561ffff811681036103495761055a9061141b6120bd565b61287f565b3461034957602036600319011261034957602061143e60043561200a565b63ffffffff60405191168152f35b34610349575f366003190112610349576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610349576020366003190112610349576004356114ad81610338565b6114b56120bd565b6001600160a01b038116156114cd5761055a906127f0565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346103495760403660031901126103495760206106cc61154036610697565b612060565b346103495760203660031901126103495760043561156281610bd9565b6115736107c6600880606654161490565b61158860018060a01b0360cb54163314611bb9565b60ca549063ffffffff811691821015611610576115a66001916119c4565b500163ffffffff81546115bf60ff8260401c16156120a7565b60201c1642101561160157805460ff60401b1916600160401b1790557fd850e6e5dfa497b72661fa73df2923464eaed9dc2ff1d3cb82bccbfeabe5c41e5f80a2005b630c36f66560e21b5f5260045ffd5b6394a8d38960e01b5f5260045ffd5b346103495760203660031901126103495760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa801561058b57611678915f9161055c57506001600160a01b03163314611af9565b611689606654198219811614611b3a565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b34610349575f3660031901126103495760cb546040516001600160a01b039091168152602090f35b34610349576116f136610752565b906117036107c6600180606654161490565b61171260026097541415611bcf565b60026097555f5b82811061172a5761055a6001609755565b806117d561173b6001938686611c1b565b335f90815260ce602052604090205460405160208101906117628161084386863387611d47565b5190209061176f83612295565b335f90815260cf6020526040902061178c90610884908490610877565b61179581611d6b565b335f90815260ce60205260409020556040517f450a367a380c4e339e5ae7340c8464ef27af7781ad9945cfe8abd828f89e62813391806108da8782611d94565b01611719565b34610349576117e936610752565b906117fb6107c6601080606654161490565b335f5260d160205261181360ff60405f205416611bb9565b61182260026097541415611bcf565b60026097555f5b82811061183a5761055a6001609755565b806118e561184b6001938686611c1b565b335f90815260ce602052604090205460405160208101906118728161084386863387611d47565b5190209061187f83612295565b335f90815260d26020526040902061189c90610884908490610877565b6118a581611d6b565b335f90815260ce60205260409020556040517f5251b6fdefcb5d81144e735f69ea4c695fd43b0289ca53dc075033f5fc80068b3391806108da8782611d94565b01611829565b634e487b7160e01b5f52604160045260245ffd5b6080810190811067ffffffffffffffff82111761191b57604052565b6118eb565b90601f8019910116810190811067ffffffffffffffff82111761191b57604052565b60405190610bf2608083611920565b6040519061195e826118ff565b5f6060838281528260208201528260408201520152565b634e487b7160e01b5f52601160045260245ffd5b80156110c1575f190190565b5f198101919082116110c157565b919082039182116110c157565b634e487b7160e01b5f52603260045260245ffd5b60ca548110156119e05760ca5f5260205f209060011b01905f90565b6119b0565b80548210156119e0575f5260205f209060011b01905f90565b90604051611a0b816118ff565b606060ff6001839580548552015463ffffffff8116602085015263ffffffff8160201c16604085015260401c161515910152565b611a47611951565b5060ca54805b611a725750611a5a611942565b5f81525f60208201525f60408201525f606082015290565b611a81610a2d610a2883611995565b90611a926113096060840151151590565b80611ab0575b611aac57611aa69150611989565b80611a4d565b5090565b50611ace611ac5604084015163ffffffff1690565b63ffffffff1690565b421015611a98565b908160209103126103495751611aeb81610338565b90565b6040513d5f823e3d90fd5b15611b0057565b63794821ff60e01b5f5260045ffd5b908160209103126103495751611aeb8161044f565b15611b2b57565b631d77d47760e21b5f5260045ffd5b15611b4157565b63c61dca5d60e01b5f5260045ffd5b35611aeb81610338565b6020813591611b6883610338565b01356040519060208201925f84526001600160601b03199060601b166021830152603582015260358152611b9d605582611920565b51902090565b15611baa57565b63840a48d560e01b5f5260045ffd5b15611bc057565b635c427cd960e01b5f5260045ffd5b15611bd657565b60405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606490fd5b91908110156119e05760051b81013590609e1981360301821215610349570190565b916020908281520191905f905b808210611c575750505090565b9091928335611c6581610338565b6001600160a01b0316815260208401356001600160601b038116919082900361034957604081600193602083940152019401920190611c4a565b908135601e19833603018112156103495782019060208235920167ffffffffffffffff8311610349578260061b3603811361034957611d3c6080611cf08193611aeb9660a0875260a0870191611c3d565b95611d10611d0060208301610fa3565b6001600160a01b03166020870152565b60408101356040860152611d36611d2960608301610be7565b63ffffffff166060870152565b01610be7565b63ffffffff16910152565b611aeb939260609260018060a01b0316825260208201528160408201520190611c9f565b90600182018092116110c157565b90602082018092116110c157565b919082018092116110c157565b906020611aeb928181520190611c9f565b35611aeb81610bd9565b903590601e1981360301821215610349570180359067ffffffffffffffff821161034957602001918160051b3603831361034957565b903590601e1981360301821215610349570180359067ffffffffffffffff821161034957602001918160061b3603831361034957565b91908110156119e05760061b0190565b15611e3257565b63aa385e8160e01b5f5260045ffd5b15611e4857565b6306957c9160e11b5f5260045ffd5b9063ffffffff8091169116019063ffffffff82116110c157565b60ca54600160401b81101561191b57806001611e92920160ca5560ca6119e5565b919091611f0057611ee160606001610bf29484518155019263ffffffff60208201511684549067ffffffff00000000604084015160201b169167ffffffffffffffff1916171784550151151590565b815460ff60401b191690151560401b68ff000000000000000016179055565b634e487b7160e01b5f525f60045260245ffd5b15611f1a57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b92610f9e61141b94611fdb610d7f94610bf299989660018060a01b03606554161580611fe0575b611fa690612115565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a261212b565b6127f0565b506001600160a01b0382161515611f9d565b63ffffffff5f199116019063ffffffff82116110c157565b63ffffffff60ca54165b63ffffffff811661202e5763504570e360e01b5f5260045ffd5b8161203b610a2883611ff2565b5054146120565763ffffffff1680156110c1575f1901612014565b611aeb9150611ff2565b602081359161206e83610338565b0135604051906020820192600160f81b84526001600160601b03199060601b166021830152603582015260358152611b9d605582611920565b156120ae57565b631b14174b60e01b5f5260045ffd5b6033546001600160a01b031633036120d157565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b1561211c57565b6339b190bb60e11b5f5260045ffd5b6001600160a01b031661213f811515612115565b606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b1561219457565b63796cc52560e01b5f5260045ffd5b156121aa57565b6310eb483f60e21b5f5260045ffd5b156121c057565b63070b5a6f60e21b5f5260045ffd5b156121d657565b630dd0b9f560e21b5f5260045ffd5b9063ffffffff169081156121fd5763ffffffff160690565b634e487b7160e01b5f52601260045260245ffd5b1561221857565b63ee66470560e01b5f5260045ffd5b1561222e57565b633c1a94f160e21b5f5260045ffd5b1561224457565b63041aa75760e11b5f5260045ffd5b1561225a57565b637ee2b44360e01b5f5260045ffd5b1561227057565b632efd965160e11b5f5260045ffd5b1561228657565b63dfad9ca160e01b5f5260045ffd5b6122ab6122a28280611de5565b9050151561218d565b6122d66f4b3b4ca85a86c47a098a223fffffffff60408301356122cf8115156121a3565b11156121b9565b61240b6123d1612325608084016123206122ef82611da5565b63ffffffff807f000000000000000000000000000000000000000000000000000000000000000016911611156121cf565b611da5565b61235f63ffffffff6123587f000000000000000000000000000000000000000000000000000000000000000080946121e5565b1615612211565b61238161237b611ac5606087019361237685611da5565b6121e5565b15612227565b6123b163ffffffff7f000000000000000000000000000000000000000000000000000000000000000016426119a3565b6123bd611ac583611da5565b10158061251a575b6123209095949561223d565b63ffffffff612402817f00000000000000000000000000000000000000000000000000000000000000001642611d87565b91161115612253565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316915f90815b6124448280611de5565b90508310156125135761246361245e84610aff8580611de5565b611b50565b60405163198f077960e21b81526001600160a01b03821660048201529091906020816024818a5afa92831561058b576001936124c7925f916124f5575b5080156124cf575b6124b190612269565b838060a01b03168092848060a01b03161061227f565b92019161243a565b5060a084901b849003811673beac0eeeeeeeeeeeeeeeeeeeeeeeeeeeeeebeac0146124a8565b61250d915060203d8111610690576106828183611920565b5f6124a0565b5050509050565b5061232061252a611ac583611da5565b63ffffffff7f000000000000000000000000000000000000000000000000000000000000000016111590506123c5565b6040516323b872dd60e01b60208201526001600160a01b039283166024820152929091166044830152606480830193909352918152610bf29161259e608483611920565b612955565b156125aa57565b631437a2bb60e31b5f5260045ffd5b156125c057565b6343714afd60e01b5f5260045ffd5b903590601e1981360301821215610349570180359067ffffffffffffffff82116103495760200191813603831361034957565b91908110156119e05760051b0190565b908210156119e0576126299160051b8101906125cf565b9091565b9190916126486126436113096060860151151590565b6120a7565b61266761265f611ac5604086015163ffffffff1690565b4210156125a3565b60a08101906126768282611daf565b905061269360c083019161268a8385611daf565b919050146125b9565b6126da6126a08284611daf565b9690506126b560e085019761268a8987611de5565b516126c260208501611da5565b6126cf60408601866125cf565b916060870193612a73565b6080820135925f5b6126ec8285611daf565b9050811015612749578061274385610aff8a61273c856127338161272d8c8f61232060019d8f6127219061272794508d611daf565b90612602565b98611daf565b90612612565b9490938c611de5565b928a612abe565b016126e2565b50505050509050565b60405163a9059cbb60e01b60208201526001600160a01b039092166024830152604480830193909352918152610bf29161259e606483611920565b60cb54907faf557c6c02c208794817a705609cfa935f827312a1adfdd26494b6b95dd2b4b36040805163ffffffff8560a01c16815263ffffffff84166020820152a163ffffffff60a01b1990911660a09190911b63ffffffff60a01b161760cb55565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b60cb546001600160a01b0391821691829082167f237b82f438d75fc568ebab484b75b01d9287b9e98b490b7c23221623b6705dbb5f80a36001600160a01b0319161760cb55565b60cb54907f8cdc428b0431b82d1619763f443a48197db344ba96905f3949643acd1c863a066040805161ffff8560e01c16815261ffff84166020820152a161ffff60e01b1990911660e09190911b61ffff60e01b161760cb55565b67ffffffffffffffff811161191b57601f01601f191660200190565b156128fd57565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b60018060a01b0316906040519061296d604083611920565b602082527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020830152823b156129e2575f816129bd948260208195519301915af16129b7612b6e565b90612b9d565b8051806129c8575050565b816020806129dd93610bf29501019101611b0f565b6128f6565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b929192612a33826128da565b91612a416040519384611920565b829481845281830111610349578281602093845f960137010152565b15612a6457565b6369ca16c960e01b5f5260045ffd5b91929063ffffffff169160018260051c1b831015612ab057612aa6612aab94612a9e610bf297611b5a565b933691612a27565b612af8565b612a5d565b62c6c39d60e71b5f5260045ffd5b91929063ffffffff169160018260051c1b831015612ae957612aa6612aab94612a9e610bf297612060565b63054ff4df60e51b5f5260045ffd5b93909291601f855116612b5f5791906020925b85518411612b565760018316612b3c575f5282850151602052612b3560405f209260011c93611d79565b9291612b0b565b838601515f52602052612b3560405f209260011c93611d79565b92509350501490565b6313717da960e21b5f5260045ffd5b3d15612b98573d90612b7f826128da565b91612b8d6040519384611920565b82523d5f602084013e565b606090565b90919015612ba9575090565b815115612bb95750805190602001fd5b604460209160405192839162461bcd60e51b83528160048401528051918291826024860152018484015e5f828201840152601f01601f19168101030190fdfea2646970667358221220696395a65d09ad98bc426daca26262bb185cfdbf8cbc74db2f347f0f81cd036164736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80b\x18W,\x14a\x033W\x80c\x04\xA0\xC5\x02\x14a\x03.W\x80c\t-\xB0\x07\x14a\x03)W\x80c\x0E\x9AS\xCF\x14a\x03$W\x80c\x0E\xB3\x83E\x14a\x03\x1FW\x80c\x10\xD6z/\x14a\x03\x1AW\x80c\x13\x143\xB4\x14a\x03\x15W\x80c\x13d9\xDD\x14a\x03\x10W\x80c\x14\x9B\xC8r\x14a\x03\x0BW\x80c\"\xF1\x9Ad\x14a\x03\x06W\x80c+\x9Fd\xA4\x14a\x03\x01W\x80c6\xAFA\xFA\x14a\x02\xFCW\x80c7\x83\x8E\xD0\x14a\x02\xF7W\x80c9\xB7\x0E8\x14a\x02\xF2W\x80c:\x8C\x07\x86\x14a\x02\xEDW\x80c<\xCC\x86\x1D\x14a\x02\xE8W\x80c>\xFE\x1D\xB6\x14a\x02\xE3W\x80cM\x18\xCC5\x14a\x02\xDEW\x80cX\xBA\xAA>\x14a\x02\xD9W\x80cY\\jg\x14a\x02\xD4W\x80cZ\xC8j\xB7\x14a\x02\xCFW\x80c\\\x97Z\xBB\x14a\x02\xCAW\x80c^\x9D\x83H\x14a\x02\xC5W\x80cm!\x11~\x14a\x02\xC0W\x80cqP\x18\xA6\x14a\x02\xBBW\x80c{\x8F\x8B\x05\x14a\x02\xB6W\x80c\x86<\xB9\xA9\x14a\x02\xB1W\x80c\x86\\iS\x14a\x02\xACW\x80c\x88o\x11\x95\x14a\x02\xA7W\x80c\x8D\xA5\xCB[\x14a\x02\xA2W\x80c\x91\x04\xC3\x19\x14a\x02\x9DW\x80c\x9B\xE3\xD4\xE4\x14a\x02\x98W\x80c\x9DE\xC2\x81\x14a\x02\x93W\x80c\xA0\x16\x9D\xDD\x14a\x02\x8EW\x80c\xAE\xBD\x8B\xAE\x14a\x02\x89W\x80c\xBB~E\x1F\x14a\x02\x84W\x80c\xBF!\xA8\xAA\x14a\x02\x7FW\x80c\xC4m\xB6\x06\x14a\x02zW\x80c\xD4T\nU\x14a\x02uW\x80c\xDE\x02\xE5\x03\x14a\x02pW\x80c\xE2!\xB2E\x14a\x02kW\x80c\xE8\x10\xCE!\x14a\x02fW\x80c\xEAM<\x9B\x14a\x02aW\x80c\xF2\xFD\xE3\x8B\x14a\x02\\W\x80c\xF8\xCD\x84H\x14a\x02WW\x80c\xF9j\xBF.\x14a\x02RW\x80c\xFA\xBC\x1C\xBC\x14a\x02MW\x80c\xFB\xF1\xE2\xC1\x14a\x02HW\x80c\xFC\xE3l}\x14a\x02CWc\xFF\x9Fl\xCE\x14a\x02>W_\x80\xFD[a\x17\xDBV[a\x16\xE3V[a\x16\xBBV[a\x16\x1FV[a\x15EV[a\x15!V[a\x14\x90V[a\x14LV[a\x14 V[a\x13\xF0V[a\x13\xC5V[a\x12\xA1V[a\x12BV[a\x12\x02V[a\x11\xC7V[a\x11yV[a\x11\x06V[a\x10\xC6V[a\x10\x8BV[a\x10]V[a\x105V[a\x10\rV[a\x0F\xAEV[a\x0FvV[a\x0FYV[a\x0E\xFEV[a\x0E\xB0V[a\x0EZV[a\x0E=V[a\x0E\nV[a\r\x84V[a\rWV[a\r1V[a\x0B\xF4V[a\t\xB7V[a\t\x82V[a\t>V[a\x08\xFEV[a\x07\xA6V[a\x07\x0FV[a\x06\xD4V[a\x06\xA8V[a\x05\xD0V[a\x05\x90V[a\x04\xECV[a\x04YV[a\x03\xF1V[a\x03\xCDV[a\x03\x8DV[a\x03MV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03IWV[_\x80\xFD[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x03j\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xD1` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x04Ka\x04\x0Ca\x1A?V[`@Q\x91\x82\x91\x82\x91\x90\x91``\x80`\x80\x83\x01\x94\x80Q\x84Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01Rc\xFF\xFF\xFF\xFF`@\x82\x01Q\x16`@\x85\x01R\x01Q\x15\x15\x91\x01RV[\x03\x90\xF3[\x80\x15\x15\x03a\x03IWV[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x04v\x81a\x038V[`$5\x90a\x04\x83\x82a\x04OV[a\x04\x8Ba \xBDV[`\x01\x80`\xA0\x1B\x03\x16\x80_R`\xD1` R`\xFF`@_ T\x16\x15\x15\x82\x15\x15\x90\x82\x7FM\xE6)>f\x8D\xF19\x84\"\xE1\xDE\xF1!\x18\x05,\x159\xA0<\xBF\xED\xC1E\x89]H\xD7h_\x1C_\x80\xA4_R`\xD1` R`@_ \x90`\xFF\x80\x19\x83T\x16\x91\x15\x15\x16\x17\x90U_\x80\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x04\x805a\x05\n\x81a\x038V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x8BWa\x05Z\x92a\x05U\x91_\x91a\x05\\W[P`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xF9V[a!+V[\0[a\x05~\x91P` =` \x11a\x05\x84W[a\x05v\x81\x83a\x19 V[\x81\x01\x90a\x1A\xD6V[_a\x05DV[P=a\x05lV[a\x1A\xEEV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\x06'\x91_\x91a\x06hW[Pa\x1B$V[a\x066`fT\x82\x81\x16\x14a\x1B:V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x06\x8A\x91P` =` \x11a\x06\x90W[a\x06\x82\x81\x83a\x19 V[\x81\x01\x90a\x1B\x0FV[_a\x06!V[P=a\x06xV[`@\x90`\x03\x19\x01\x12a\x03IW`\x04\x90V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x06\xCCa\x06\xC76a\x06\x97V[a\x1BZV[`@Q\x90\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IWa\x06\xF0`\x045a\x038V[a\x06\xFB`$5a\x038V[` a\xFF\xFF`\xCBT`\xE0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x07,\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCC` R` `\x01\x80`\xA0\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[\x90` `\x03\x19\x83\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IW\x82`#\x82\x01\x12\x15a\x03IW\x80`\x04\x015\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11a\x03IW`$\x84`\x05\x1B\x83\x01\x01\x11a\x03IW`$\x01\x91\x90V[4a\x03IWa\x07\xB46a\x07RV[\x90a\x07\xCCa\x07\xC6`\x02\x80`fT\x16\x14\x90V[\x15a\x1B\xA3V[3_R`\xD1` Ra\x07\xE4`\xFF`@_ T\x16a\x1B\xB9V[a\x07\xF3`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x08\x0BWa\x05Z`\x01`\x97UV[\x80a\x08\xF8a\x08\x1C`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x08Q\x81a\x08C\x86\x863\x87a\x1DGV[\x03`\x1F\x19\x81\x01\x83R\x82a\x19 V[Q\x90 \x90a\x08^\x83a\"\x95V[3_\x90\x81R`\xD0` R`@\x90 a\x08\x91\x90a\x08\x84\x90\x84\x90[\x90_R` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[a\x08\x9A\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FQ\x08\x8B\x8C\x89b\x8D\xF3\xA8\x17@\x02\xC2\xA04\xD0\x15/\xCEj\xF8A]e\x1B*G4\xBF'\x04\x823\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x03\x90\xA4`@a\x08\xEB` \x83\x01a\x1BPV[\x91\x015\x900\x903\x90a%ZV[\x01a\x07\xFAV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` c\xFF\xFF\xFF\xFF`\xCBT`\xA0\x1C\x16`@Q\x90\x81R\xF3[\x90\x81a\x01\0\x91\x03\x12a\x03IW\x90V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IWa\t\xE8\x906\x90`\x04\x01a\t\xA8V[`$5a\t\xF4\x81a\x038V[a\n\x05a\x07\xC6`\x04\x80`fT\x16\x14\x90V[a\n\x14`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97Ua\n3a\n-a\n(\x84a\x1D\xA5V[a\x19\xC4V[Pa\x19\xFEV[\x90a\n>\x82\x84a&-V[a\nJ``\x84\x01a\x1BPV[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xCC` R`@\x90 T\x16\x80\x15a\x0B\xD2W[\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90a\n\x833\x83\x14a\x1B\xB9V[`\xA0\x85\x01\x93`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x94\x90\x83\x16\x92\x90\x91`\xE0\x88\x01\x91_[a\n\xAB\x89\x8Ba\x1D\xAFV[\x90P\x81\x10\x15a\x0B\xC8W`\x01\x81\x89\x89\x89\x7F\x95C\xDB\xD5U\x80\x84%\x86\xA9Q\xF08n$\xD6\x8A]\xF9\x9A\xE2\x9E;!e\x88\xB4_\xD6\x84\xCE1\x8F\x89a\x0B\xBDa\x0B\x96\x8Ea\x0B\x8F\x8Fa\x0B\x05\x8F\x91\x9Ea\n\xAB\x9Fa\n\xFF\x90a\x0Bw\x9Aa\x1D\xE5V[\x90a\x1E\x1BV[\x95a\x0B@a\x0B#\x83`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a\x0B,\x89a\x1BPV[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[Ta\x0B\x80a\x0B^` \x8A\x015\x92a\x0BX\x81\x85\x11a\x1E+V[\x83a\x19\xA3V[\x99\x8A\x94`\x01\x80`\xA0\x1B\x03\x16_R`\xCD` R`@_ \x90V[a\x0B,\x8Aa\x1BPV[Ua\x0B\x8A\x87a\x1BPV[a'RV[Q\x92a\x1BPV[`@\x80Q\x93\x84R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x84\x01R\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA4\x01\x90Pa\n\xA1V[a\x05Z`\x01`\x97UV[P\x80a\nkV[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03IWV[5\x90a\x0B\xF2\x82a\x0B\xD9V[V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`$5`\x045a\x0C\x14\x82a\x0B\xD9V[a\x0C%a\x07\xC6`\x08\x80`fT\x16\x14\x90V[`\xCBT\x91a\x0C=3`\x01`\x01`\xA0\x1B\x03\x85\x16\x14a\x1B\xB9V[c\xFF\xFF\xFF\xFF\x81\x16\x92c\xFF\xFF\xFF\xFF\x81`\xC0\x1C\x16\x84\x11\x15a\r\"Wc\xFF\xFF\xFF\xFF\x7F\xEC\xD8f\xC3\xC1X\xFA\0\xBF4\xD8\x03\xD5\xF6\x020\0\xB5p\x80\xBC\xB4\x8A\xF0\x04\xC2\xB4\xB4k:\xFD\x08\x91a\x0C\x88B\x87\x10a\x1EAV[a\r\x0Ca\x0C\xB5a\x0C\xADa\x0C\xA0`\xCATc\xFF\xFF\xFF\xFF\x16\x90V[\x93`\xA0\x1Cc\xFF\xFF\xFF\xFF\x16\x90V[\x84B\x16a\x1EWV[\x94a\x0C\xE7a\x0C\xC1a\x19BV[\x88\x81Rc\xFF\xFF\xFF\xFF\x83\x16` \x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`@\x82\x01R_``\x82\x01Ra\x1EqV[`\xCB\x80Tc\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\xC0\x92\x90\x92\x1Bc\xFF\xFF\xFF\xFF`\xC0\x1B\x16\x91\x90\x91\x17\x90UV[`@Qc\xFF\xFF\xFF\xFF\x94\x90\x94\x16\x84R\x16\x91` \x90\xA4\0[c\x1C\xA7\xE5\x0B`\xE2\x1B_R`\x04_\xFD[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` c\xFF\xFF\xFF\xFF`\xCBT`\xC0\x1C\x16`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x05Z`\x045a\rw\x81a\x0B\xD9V[a\r\x7Fa \xBDV[a'\x8DV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\r\xD6\x91_\x91a\x06hWPa\x1B$V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045`\xFF\x81\x16\x80\x91\x03a\x03IW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `fT`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x03IWa\x0E\x8Ea\x0E\xA5\x916\x90`\x04\x01a\t\xA8V[a\x0E\x9Fa\n-\x825a\n(\x81a\x0B\xD9V[\x90a&-V[` `@Q`\x01\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x0E\xCD\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xCF` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x0F\x16a \xBDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `\xCAT`@Q\x90\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x05Z`\x045a\x0F\x96\x81a\x038V[a\x0F\x9Ea \xBDV[a(8V[5\x90a\x0B\xF2\x82a\x038V[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x10\x04`\x045a\x0F\xD0\x81a\x038V[`$5\x90a\x0F\xDD\x82a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCD\x83R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[T`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qs\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IWa\x10\xA3a\x19QV[P`\xCAT_\x19\x81\x01\x90\x81\x11a\x10\xC1Wa\x04\x0Ca\n-a\x04K\x92a\x19\xC4V[a\x19uV[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x11#\x81a\x038V[3_\x81\x81R`\xCC` R`@\x81 \x80T`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x93\x16\x91\x90\x7F\xBA\xB9G\x93MB\xE0\xAD o%\xC9\xCA\xB1\x8B[\xB6\xAE\x14J\xCF\xB0\x0F@\xB4\xE3\xAAYY\x0C\xA3\x12\x90\x80\xA4\0[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x11\x96\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD2` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x11\xE4\x81a\x038V[`\x01\x80`\xA0\x1B\x03\x16_R`\xCE` R` `@_ T`@Q\x90\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW`\x045a\x12_\x81a\x038V[`$5\x90`\x01\x80`\xA0\x1B\x03\x16_R`\xD0` R`@_ \x90_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[`\xA45\x90a\xFF\xFF\x82\x16\x82\x03a\x03IWV[4a\x03IW`\xC06`\x03\x19\x01\x12a\x03IW`\x045a\x12\xBE\x81a\x038V[a\x13=`$5a\x12\xCD\x81a\x038V[`D5`d5a\x12\xDC\x81a\x038V[`\x845\x91a\x12\xE9\x83a\x0B\xD9V[a\x12\xF1a\x12\x90V[\x93_T\x96a\x13#a\x13\ra\x13\t\x8A`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x99\x81\x9Aa\x13\xB7W[\x81\x15a\x13\x97W[Pa\x1F\x13V[\x87a\x134`\x01`\xFF\x19_T\x16\x17_UV[a\x13\x80Wa\x1FvV[a\x13CW\0[a\x13Qa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x90\xA1\0[a\x13\x92a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x1FvV[0;\x15\x91P\x81a\x13\xA9W[P_a\x13\x1DV[`\xFF\x16`\x01\x14\x90P_a\x13\xA2V[`\x01`\xFF\x82\x16\x10\x91Pa\x13\x16V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IWa\x04Ka\x04\x0Ca\n-`\x045a\x13\xEAa\x19QV[Pa\x19\xC4V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\xFF\xFF\x81\x16\x81\x03a\x03IWa\x05Z\x90a\x14\x1Ba \xBDV[a(\x7FV[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW` a\x14>`\x045a \nV[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x14\xAD\x81a\x038V[a\x14\xB5a \xBDV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x14\xCDWa\x05Z\x90a'\xF0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03IW`@6`\x03\x19\x01\x12a\x03IW` a\x06\xCCa\x15@6a\x06\x97V[a `V[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`\x045a\x15b\x81a\x0B\xD9V[a\x15sa\x07\xC6`\x08\x80`fT\x16\x14\x90V[a\x15\x88`\x01\x80`\xA0\x1B\x03`\xCBT\x163\x14a\x1B\xB9V[`\xCAT\x90c\xFF\xFF\xFF\xFF\x81\x16\x91\x82\x10\x15a\x16\x10Wa\x15\xA6`\x01\x91a\x19\xC4V[P\x01c\xFF\xFF\xFF\xFF\x81Ta\x15\xBF`\xFF\x82`@\x1C\x16\x15a \xA7V[` \x1C\x16B\x10\x15a\x16\x01W\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U\x7F\xD8P\xE6\xE5\xDF\xA4\x97\xB7&a\xFAs\xDF)#FN\xAE\xD9\xDC/\xF1\xD3\xCB\x82\xBC\xCB\xFE\xAB\xE5\xC4\x1E_\x80\xA2\0[c\x0C6\xF6e`\xE2\x1B_R`\x04_\xFD[c\x94\xA8\xD3\x89`\xE0\x1B_R`\x04_\xFD[4a\x03IW` 6`\x03\x19\x01\x12a\x03IW`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x8BWa\x16x\x91_\x91a\x05\\WP`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1A\xF9V[a\x16\x89`fT\x19\x82\x19\x81\x16\x14a\x1B:V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[4a\x03IW_6`\x03\x19\x01\x12a\x03IW`\xCBT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03IWa\x16\xF16a\x07RV[\x90a\x17\x03a\x07\xC6`\x01\x80`fT\x16\x14\x90V[a\x17\x12`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x17*Wa\x05Z`\x01`\x97UV[\x80a\x17\xD5a\x17;`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x17b\x81a\x08C\x86\x863\x87a\x1DGV[Q\x90 \x90a\x17o\x83a\"\x95V[3_\x90\x81R`\xCF` R`@\x90 a\x17\x8C\x90a\x08\x84\x90\x84\x90a\x08wV[a\x17\x95\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FE\n6z8\x0CN3\x9EZ\xE74\x0C\x84d\xEF'\xAFw\x81\xAD\x99E\xCF\xE8\xAB\xD8(\xF8\x9Eb\x813\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x01a\x17\x19V[4a\x03IWa\x17\xE96a\x07RV[\x90a\x17\xFBa\x07\xC6`\x10\x80`fT\x16\x14\x90V[3_R`\xD1` Ra\x18\x13`\xFF`@_ T\x16a\x1B\xB9V[a\x18\"`\x02`\x97T\x14\x15a\x1B\xCFV[`\x02`\x97U_[\x82\x81\x10a\x18:Wa\x05Z`\x01`\x97UV[\x80a\x18\xE5a\x18K`\x01\x93\x86\x86a\x1C\x1BV[3_\x90\x81R`\xCE` R`@\x90 T`@Q` \x81\x01\x90a\x18r\x81a\x08C\x86\x863\x87a\x1DGV[Q\x90 \x90a\x18\x7F\x83a\"\x95V[3_\x90\x81R`\xD2` R`@\x90 a\x18\x9C\x90a\x08\x84\x90\x84\x90a\x08wV[a\x18\xA5\x81a\x1DkV[3_\x90\x81R`\xCE` R`@\x90 U`@Q\x7FRQ\xB6\xFD\xEF\xCB]\x81\x14Ns_i\xEALi_\xD4;\x02\x89\xCAS\xDC\x07P3\xF5\xFC\x80\x06\x8B3\x91\x80a\x08\xDA\x87\x82a\x1D\x94V[\x01a\x18)V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x19\x1BW`@RV[a\x18\xEBV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x19\x1BW`@RV[`@Q\x90a\x0B\xF2`\x80\x83a\x19 V[`@Q\x90a\x19^\x82a\x18\xFFV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x15a\x10\xC1W_\x19\x01\x90V[_\x19\x81\x01\x91\x90\x82\x11a\x10\xC1WV[\x91\x90\x82\x03\x91\x82\x11a\x10\xC1WV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[`\xCAT\x81\x10\x15a\x19\xE0W`\xCA_R` _ \x90`\x01\x1B\x01\x90_\x90V[a\x19\xB0V[\x80T\x82\x10\x15a\x19\xE0W_R` _ \x90`\x01\x1B\x01\x90_\x90V[\x90`@Qa\x1A\x0B\x81a\x18\xFFV[```\xFF`\x01\x83\x95\x80T\x85R\x01Tc\xFF\xFF\xFF\xFF\x81\x16` \x85\x01Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16`@\x85\x01R`@\x1C\x16\x15\x15\x91\x01RV[a\x1AGa\x19QV[P`\xCAT\x80[a\x1ArWPa\x1AZa\x19BV[_\x81R_` \x82\x01R_`@\x82\x01R_``\x82\x01R\x90V[a\x1A\x81a\n-a\n(\x83a\x19\x95V[\x90a\x1A\x92a\x13\t``\x84\x01Q\x15\x15\x90V[\x80a\x1A\xB0W[a\x1A\xACWa\x1A\xA6\x91Pa\x19\x89V[\x80a\x1AMV[P\x90V[Pa\x1A\xCEa\x1A\xC5`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a\x1A\x98V[\x90\x81` \x91\x03\x12a\x03IWQa\x1A\xEB\x81a\x038V[\x90V[`@Q=_\x82>=\x90\xFD[\x15a\x1B\0WV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03IWQa\x1A\xEB\x81a\x04OV[\x15a\x1B+WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a\x1BAWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[5a\x1A\xEB\x81a\x038V[` \x815\x91a\x1Bh\x83a\x038V[\x015`@Q\x90` \x82\x01\x92_\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra\x1B\x9D`U\x82a\x19 V[Q\x90 \x90V[\x15a\x1B\xAAWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xC0WV[c\\B|\xD9`\xE0\x1B_R`\x04_\xFD[\x15a\x1B\xD6WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x90\xFD[\x91\x90\x81\x10\x15a\x19\xE0W`\x05\x1B\x81\x015\x90`\x9E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x90V[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a\x1CWWPPP\x90V[\x90\x91\x92\x835a\x1Ce\x81a\x038V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x84\x015`\x01`\x01``\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\x03IW`@\x81`\x01\x93` \x83\x94\x01R\x01\x94\x01\x92\x01\x90a\x1CJV[\x90\x815`\x1E\x19\x836\x03\x01\x81\x12\x15a\x03IW\x82\x01\x90` \x825\x92\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x03IW\x82`\x06\x1B6\x03\x81\x13a\x03IWa\x1D<`\x80a\x1C\xF0\x81\x93a\x1A\xEB\x96`\xA0\x87R`\xA0\x87\x01\x91a\x1C=V[\x95a\x1D\x10a\x1D\0` \x83\x01a\x0F\xA3V[`\x01`\x01`\xA0\x1B\x03\x16` \x87\x01RV[`@\x81\x015`@\x86\x01Ra\x1D6a\x1D)``\x83\x01a\x0B\xE7V[c\xFF\xFF\xFF\xFF\x16``\x87\x01RV[\x01a\x0B\xE7V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[a\x1A\xEB\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x1C\x9FV[\x90`\x01\x82\x01\x80\x92\x11a\x10\xC1WV[\x90` \x82\x01\x80\x92\x11a\x10\xC1WV[\x91\x90\x82\x01\x80\x92\x11a\x10\xC1WV[\x90` a\x1A\xEB\x92\x81\x81R\x01\x90a\x1C\x9FV[5a\x1A\xEB\x81a\x0B\xD9V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03IWV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x03IWV[\x91\x90\x81\x10\x15a\x19\xE0W`\x06\x1B\x01\x90V[\x15a\x1E2WV[c\xAA8^\x81`\xE0\x1B_R`\x04_\xFD[\x15a\x1EHWV[c\x06\x95|\x91`\xE1\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\xC1WV[`\xCAT`\x01`@\x1B\x81\x10\x15a\x19\x1BW\x80`\x01a\x1E\x92\x92\x01`\xCAU`\xCAa\x19\xE5V[\x91\x90\x91a\x1F\0Wa\x1E\xE1```\x01a\x0B\xF2\x94\x84Q\x81U\x01\x92c\xFF\xFF\xFF\xFF` \x82\x01Q\x16\x84T\x90g\xFF\xFF\xFF\xFF\0\0\0\0`@\x84\x01Q` \x1B\x16\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17\x84U\x01Q\x15\x15\x90V[\x81T`\xFF`@\x1B\x19\x16\x90\x15\x15`@\x1Bh\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x15a\x1F\x1AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[\x92a\x0F\x9Ea\x14\x1B\x94a\x1F\xDBa\r\x7F\x94a\x0B\xF2\x99\x98\x96`\x01\x80`\xA0\x1B\x03`eT\x16\x15\x80a\x1F\xE0W[a\x1F\xA6\x90a!\x15V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a!+V[a'\xF0V[P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15a\x1F\x9DV[c\xFF\xFF\xFF\xFF_\x19\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x10\xC1WV[c\xFF\xFF\xFF\xFF`\xCAT\x16[c\xFF\xFF\xFF\xFF\x81\x16a .WcPEp\xE3`\xE0\x1B_R`\x04_\xFD[\x81a ;a\n(\x83a\x1F\xF2V[PT\x14a VWc\xFF\xFF\xFF\xFF\x16\x80\x15a\x10\xC1W_\x19\x01a \x14V[a\x1A\xEB\x91Pa\x1F\xF2V[` \x815\x91a n\x83a\x038V[\x015`@Q\x90` \x82\x01\x92`\x01`\xF8\x1B\x84R`\x01`\x01``\x1B\x03\x19\x90``\x1B\x16`!\x83\x01R`5\x82\x01R`5\x81Ra\x1B\x9D`U\x82a\x19 V[\x15a \xAEWV[c\x1B\x14\x17K`\xE0\x1B_R`\x04_\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a \xD1WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[\x15a!\x1CWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16a!?\x81\x15\x15a!\x15V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[\x15a!\x94WV[cyl\xC5%`\xE0\x1B_R`\x04_\xFD[\x15a!\xAAWV[c\x10\xEBH?`\xE2\x1B_R`\x04_\xFD[\x15a!\xC0WV[c\x07\x0BZo`\xE2\x1B_R`\x04_\xFD[\x15a!\xD6WV[c\r\xD0\xB9\xF5`\xE2\x1B_R`\x04_\xFD[\x90c\xFF\xFF\xFF\xFF\x16\x90\x81\x15a!\xFDWc\xFF\xFF\xFF\xFF\x16\x06\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\"\x18WV[c\xEEfG\x05`\xE0\x1B_R`\x04_\xFD[\x15a\".WV[c<\x1A\x94\xF1`\xE2\x1B_R`\x04_\xFD[\x15a\"DWV[c\x04\x1A\xA7W`\xE1\x1B_R`\x04_\xFD[\x15a\"ZWV[c~\xE2\xB4C`\xE0\x1B_R`\x04_\xFD[\x15a\"pWV[c.\xFD\x96Q`\xE1\x1B_R`\x04_\xFD[\x15a\"\x86WV[c\xDF\xAD\x9C\xA1`\xE0\x1B_R`\x04_\xFD[a\"\xABa\"\xA2\x82\x80a\x1D\xE5V[\x90P\x15\x15a!\x8DV[a\"\xD6oK;L\xA8Z\x86\xC4z\t\x8A\"?\xFF\xFF\xFF\xFF`@\x83\x015a\"\xCF\x81\x15\x15a!\xA3V[\x11\x15a!\xB9V[a$\x0Ba#\xD1a#%`\x80\x84\x01a# a\"\xEF\x82a\x1D\xA5V[c\xFF\xFF\xFF\xFF\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x91\x16\x11\x15a!\xCFV[a\x1D\xA5V[a#_c\xFF\xFF\xFF\xFFa#X\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x80\x94a!\xE5V[\x16\x15a\"\x11V[a#\x81a#{a\x1A\xC5``\x87\x01\x93a#v\x85a\x1D\xA5V[a!\xE5V[\x15a\"'V[a#\xB1c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x19\xA3V[a#\xBDa\x1A\xC5\x83a\x1D\xA5V[\x10\x15\x80a%\x1AW[a# \x90\x95\x94\x95a\"=V[c\xFF\xFF\xFF\xFFa$\x02\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba\x1D\x87V[\x91\x16\x11\x15a\"SV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91_\x90\x81[a$D\x82\x80a\x1D\xE5V[\x90P\x83\x10\x15a%\x13Wa$ca$^\x84a\n\xFF\x85\x80a\x1D\xE5V[a\x1BPV[`@Qc\x19\x8F\x07y`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R\x90\x91\x90` \x81`$\x81\x8AZ\xFA\x92\x83\x15a\x05\x8BW`\x01\x93a$\xC7\x92_\x91a$\xF5W[P\x80\x15a$\xCFW[a$\xB1\x90a\"iV[\x83\x80`\xA0\x1B\x03\x16\x80\x92\x84\x80`\xA0\x1B\x03\x16\x10a\"\x7FV[\x92\x01\x91a$:V[P`\xA0\x84\x90\x1B\x84\x90\x03\x81\x16s\xBE\xAC\x0E\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEE\xEB\xEA\xC0\x14a$\xA8V[a%\r\x91P` =\x81\x11a\x06\x90Wa\x06\x82\x81\x83a\x19 V[_a$\xA0V[PPP\x90PV[Pa# a%*a\x1A\xC5\x83a\x1D\xA5V[c\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x11\x15\x90Pa#\xC5V[`@Qc#\xB8r\xDD`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`$\x82\x01R\x92\x90\x91\x16`D\x83\x01R`d\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x0B\xF2\x91a%\x9E`\x84\x83a\x19 V[a)UV[\x15a%\xAAWV[c\x147\xA2\xBB`\xE3\x1B_R`\x04_\xFD[\x15a%\xC0WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03IW\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x03IW` \x01\x91\x816\x03\x83\x13a\x03IWV[\x91\x90\x81\x10\x15a\x19\xE0W`\x05\x1B\x01\x90V[\x90\x82\x10\x15a\x19\xE0Wa&)\x91`\x05\x1B\x81\x01\x90a%\xCFV[\x90\x91V[\x91\x90\x91a&Ha&Ca\x13\t``\x86\x01Q\x15\x15\x90V[a \xA7V[a&ga&_a\x1A\xC5`@\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a%\xA3V[`\xA0\x81\x01\x90a&v\x82\x82a\x1D\xAFV[\x90Pa&\x93`\xC0\x83\x01\x91a&\x8A\x83\x85a\x1D\xAFV[\x91\x90P\x14a%\xB9V[a&\xDAa&\xA0\x82\x84a\x1D\xAFV[\x96\x90Pa&\xB5`\xE0\x85\x01\x97a&\x8A\x89\x87a\x1D\xE5V[Qa&\xC2` \x85\x01a\x1D\xA5V[a&\xCF`@\x86\x01\x86a%\xCFV[\x91``\x87\x01\x93a*sV[`\x80\x82\x015\x92_[a&\xEC\x82\x85a\x1D\xAFV[\x90P\x81\x10\x15a'IW\x80a'C\x85a\n\xFF\x8Aa'<\x85a'3\x81a'-\x8C\x8Fa# `\x01\x9D\x8Fa'!\x90a''\x94P\x8Da\x1D\xAFV[\x90a&\x02V[\x98a\x1D\xAFV[\x90a&\x12V[\x94\x90\x93\x8Ca\x1D\xE5V[\x92\x8Aa*\xBEV[\x01a&\xE2V[PPPPP\x90PV[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x80\x83\x01\x93\x90\x93R\x91\x81Ra\x0B\xF2\x91a%\x9E`d\x83a\x19 V[`\xCBT\x90\x7F\xAFU|l\x02\xC2\x08yH\x17\xA7\x05`\x9C\xFA\x93_\x82s\x12\xA1\xAD\xFD\xD2d\x94\xB6\xB9]\xD2\xB4\xB3`@\x80Qc\xFF\xFF\xFF\xFF\x85`\xA0\x1C\x16\x81Rc\xFF\xFF\xFF\xFF\x84\x16` \x82\x01R\xA1c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x91\x16`\xA0\x91\x90\x91\x1Bc\xFF\xFF\xFF\xFF`\xA0\x1B\x16\x17`\xCBUV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[`\xCBT`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x91\x82\x90\x82\x16\x7F#{\x82\xF48\xD7_\xC5h\xEB\xABHKu\xB0\x1D\x92\x87\xB9\xE9\x8BI\x0B|#\"\x16#\xB6p]\xBB_\x80\xA3`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`\xCBUV[`\xCBT\x90\x7F\x8C\xDCB\x8B\x041\xB8-\x16\x19v?D:H\x19}\xB3D\xBA\x96\x90_9Id:\xCD\x1C\x86:\x06`@\x80Qa\xFF\xFF\x85`\xE0\x1C\x16\x81Ra\xFF\xFF\x84\x16` \x82\x01R\xA1a\xFF\xFF`\xE0\x1B\x19\x90\x91\x16`\xE0\x91\x90\x91\x1Ba\xFF\xFF`\xE0\x1B\x16\x17`\xCBUV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x19\x1BW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x15a(\xFDWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[`\x01\x80`\xA0\x1B\x03\x16\x90`@Q\x90a)m`@\x83a\x19 V[` \x82R\x7FSafeERC20: low-level call failed` \x83\x01R\x82;\x15a)\xE2W_\x81a)\xBD\x94\x82` \x81\x95Q\x93\x01\x91Z\xF1a)\xB7a+nV[\x90a+\x9DV[\x80Q\x80a)\xC8WPPV[\x81` \x80a)\xDD\x93a\x0B\xF2\x95\x01\x01\x91\x01a\x1B\x0FV[a(\xF6V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x92\x91\x92a*3\x82a(\xDAV[\x91a*A`@Q\x93\x84a\x19 V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03IW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x15a*dWV[ci\xCA\x16\xC9`\xE0\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a*\xB0Wa*\xA6a*\xAB\x94a*\x9Ea\x0B\xF2\x97a\x1BZV[\x936\x91a*'V[a*\xF8V[a*]V[b\xC6\xC3\x9D`\xE7\x1B_R`\x04_\xFD[\x91\x92\x90c\xFF\xFF\xFF\xFF\x16\x91`\x01\x82`\x05\x1C\x1B\x83\x10\x15a*\xE9Wa*\xA6a*\xAB\x94a*\x9Ea\x0B\xF2\x97a `V[c\x05O\xF4\xDF`\xE5\x1B_R`\x04_\xFD[\x93\x90\x92\x91`\x1F\x85Q\x16a+_W\x91\x90` \x92[\x85Q\x84\x11a+VW`\x01\x83\x16a+<W_R\x82\x85\x01Q` Ra+5`@_ \x92`\x01\x1C\x93a\x1DyV[\x92\x91a+\x0BV[\x83\x86\x01Q_R` Ra+5`@_ \x92`\x01\x1C\x93a\x1DyV[\x92P\x93PP\x14\x90V[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[=\x15a+\x98W=\x90a+\x7F\x82a(\xDAV[\x91a+\x8D`@Q\x93\x84a\x19 V[\x82R=_` \x84\x01>V[``\x90V[\x90\x91\x90\x15a+\xA9WP\x90V[\x81Q\x15a+\xB9WP\x80Q\x90` \x01\xFD[`D` \x91`@Q\x92\x83\x91bF\x1B\xCD`\xE5\x1B\x83R\x81`\x04\x84\x01R\x80Q\x91\x82\x91\x82`$\x86\x01R\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xFD\xFE\xA2dipfsX\"\x12 ic\x95\xA6]\t\xAD\x98\xBCBm\xAC\xA2bb\xBB\x18\\\xFD\xBF\x8C\xBCt\xDB/4\x7F\x0F\x81\xCD\x03adsolcC\0\x08\x1B\x003",
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
    /**Event with signature `GlobalCommissionBipsSet(uint16,uint16)` and selector `0x8cdc428b0431b82d1619763f443a48197db344ba96905f3949643acd1c863a06`.
```solidity
event GlobalCommissionBipsSet(uint16 oldGlobalCommissionBips, uint16 newGlobalCommissionBips);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct GlobalCommissionBipsSet {
        #[allow(missing_docs)]
        pub oldGlobalCommissionBips: u16,
        #[allow(missing_docs)]
        pub newGlobalCommissionBips: u16,
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
        impl alloy_sol_types::SolEvent for GlobalCommissionBipsSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<16>,
                alloy::sol_types::sol_data::Uint<16>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "GlobalCommissionBipsSet(uint16,uint16)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                140u8,
                220u8,
                66u8,
                139u8,
                4u8,
                49u8,
                184u8,
                45u8,
                22u8,
                25u8,
                118u8,
                63u8,
                68u8,
                58u8,
                72u8,
                25u8,
                125u8,
                179u8,
                68u8,
                186u8,
                150u8,
                144u8,
                95u8,
                57u8,
                73u8,
                100u8,
                58u8,
                205u8,
                28u8,
                134u8,
                58u8,
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
                    oldGlobalCommissionBips: data.0,
                    newGlobalCommissionBips: data.1,
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
                        &self.oldGlobalCommissionBips,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        16,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newGlobalCommissionBips,
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
        impl alloy_sol_types::private::IntoLogData for GlobalCommissionBipsSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&GlobalCommissionBipsSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &GlobalCommissionBipsSet,
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
constructor(address _delegationManager, address _strategyManager, uint32 _CALCULATION_INTERVAL_SECONDS, uint32 _MAX_REWARDS_DURATION, uint32 _MAX_RETROACTIVE_LENGTH, uint32 _MAX_FUTURE_LENGTH, uint32 _GENESIS_REWARDS_TIMESTAMP);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegationManager: alloy::sol_types::private::Address,
        pub _strategyManager: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        _CALCULATION_INTERVAL_SECONDS: tuple.2,
                        _MAX_REWARDS_DURATION: tuple.3,
                        _MAX_RETROACTIVE_LENGTH: tuple.4,
                        _MAX_FUTURE_LENGTH: tuple.5,
                        _GENESIS_REWARDS_TIMESTAMP: tuple.6,
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
function claimerFor(address) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`claimerFor(address)`](claimerForCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimerForReturn {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
function cumulativeClaimed(address, address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`cumulativeClaimed(address,address)`](cumulativeClaimedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cumulativeClaimedReturn {
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
            impl ::core::convert::From<cumulativeClaimedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeClaimedCall {
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
            impl ::core::convert::From<cumulativeClaimedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: cumulativeClaimedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for cumulativeClaimedReturn {
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
    /**Function with signature `globalOperatorCommissionBips()` and selector `0x092db007`.
```solidity
function globalOperatorCommissionBips() external view returns (uint16);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct globalOperatorCommissionBipsCall {}
    ///Container type for the return parameters of the [`globalOperatorCommissionBips()`](globalOperatorCommissionBipsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct globalOperatorCommissionBipsReturn {
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
            impl ::core::convert::From<globalOperatorCommissionBipsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: globalOperatorCommissionBipsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for globalOperatorCommissionBipsCall {
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
            impl ::core::convert::From<globalOperatorCommissionBipsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: globalOperatorCommissionBipsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for globalOperatorCommissionBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for globalOperatorCommissionBipsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = globalOperatorCommissionBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "globalOperatorCommissionBips()";
            const SELECTOR: [u8; 4] = [9u8, 45u8, 176u8, 7u8];
            #[inline]
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
    /**Function with signature `initialize(address,address,uint256,address,uint32,uint16)` and selector `0xd4540a55`.
```solidity
function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus, address _rewardsUpdater, uint32 _activationDelay, uint16 _globalCommissionBips) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _rewardsUpdater: alloy::sol_types::private::Address,
        pub _activationDelay: u32,
        pub _globalCommissionBips: u16,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        value._rewardsUpdater,
                        value._activationDelay,
                        value._globalCommissionBips,
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
                        _globalCommissionBips: tuple.5,
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
            const SIGNATURE: &'static str = "initialize(address,address,uint256,address,uint32,uint16)";
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
                    > as alloy_sol_types::SolType>::tokenize(&self._globalCommissionBips),
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
function isAVSRewardsSubmissionHash(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isAVSRewardsSubmissionHash(address,bytes32)`](isAVSRewardsSubmissionHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isAVSRewardsSubmissionHashReturn {
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isAVSRewardsSubmissionHashCall {
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
            impl ::core::convert::From<isAVSRewardsSubmissionHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isAVSRewardsSubmissionHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isAVSRewardsSubmissionHashReturn {
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
function isRewardsForAllSubmitter(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isRewardsForAllSubmitter(address)`](isRewardsForAllSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsForAllSubmitterReturn {
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
            impl ::core::convert::From<isRewardsForAllSubmitterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsForAllSubmitterCall {
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
            impl ::core::convert::From<isRewardsForAllSubmitterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsForAllSubmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsForAllSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
    /**Function with signature `isRewardsSubmissionForAllEarnersHash(address,bytes32)` and selector `0xaebd8bae`.
```solidity
function isRewardsSubmissionForAllEarnersHash(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllEarnersHash(address,bytes32)`](isRewardsSubmissionForAllEarnersHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllEarnersHashReturn {
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllEarnersHashCall {
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
            impl ::core::convert::From<isRewardsSubmissionForAllEarnersHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllEarnersHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllEarnersHashReturn {
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
    /**Function with signature `isRewardsSubmissionForAllHash(address,bytes32)` and selector `0xc46db606`.
```solidity
function isRewardsSubmissionForAllHash(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`isRewardsSubmissionForAllHash(address,bytes32)`](isRewardsSubmissionForAllHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isRewardsSubmissionForAllHashReturn {
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllHashCall {
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
            impl ::core::convert::From<isRewardsSubmissionForAllHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isRewardsSubmissionForAllHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isRewardsSubmissionForAllHashReturn {
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
    /**Function with signature `operatorCommissionBips(address,address)` and selector `0x22f19a64`.
```solidity
function operatorCommissionBips(address operator, address avs) external view returns (uint16);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorCommissionBipsCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`operatorCommissionBips(address,address)`](operatorCommissionBipsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorCommissionBipsReturn {
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
            impl ::core::convert::From<operatorCommissionBipsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorCommissionBipsCall) -> Self {
                    (value.operator, value.avs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorCommissionBipsCall {
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
            impl ::core::convert::From<operatorCommissionBipsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorCommissionBipsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorCommissionBipsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorCommissionBipsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorCommissionBipsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorCommissionBips(address,address)";
            const SELECTOR: [u8; 4] = [34u8, 241u8, 154u8, 100u8];
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
    pub struct setClaimerForCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setClaimerForReturn;
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
    /**Function with signature `setGlobalOperatorCommission(uint16)` and selector `0xe221b245`.
```solidity
function setGlobalOperatorCommission(uint16 _globalCommissionBips) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalOperatorCommissionCall {
        pub _globalCommissionBips: u16,
    }
    ///Container type for the return parameters of the [`setGlobalOperatorCommission(uint16)`](setGlobalOperatorCommissionCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setGlobalOperatorCommissionReturn {}
    #[allow(
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
            impl ::core::convert::From<setGlobalOperatorCommissionCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalOperatorCommissionCall) -> Self {
                    (value._globalCommissionBips,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setGlobalOperatorCommissionCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _globalCommissionBips: tuple.0,
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
            impl ::core::convert::From<setGlobalOperatorCommissionReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setGlobalOperatorCommissionReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setGlobalOperatorCommissionReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setGlobalOperatorCommissionCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<16>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setGlobalOperatorCommissionReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setGlobalOperatorCommission(uint16)";
            const SELECTOR: [u8; 4] = [226u8, 33u8, 178u8, 69u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._globalCommissionBips,
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
function submissionNonce(address) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`submissionNonce(address)`](submissionNonceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct submissionNonceReturn {
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
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for submissionNonceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
        beaconChainETHStrategy(beaconChainETHStrategyCall),
        calculateEarnerLeafHash(calculateEarnerLeafHashCall),
        calculateTokenLeafHash(calculateTokenLeafHashCall),
        checkClaim(checkClaimCall),
        claimerFor(claimerForCall),
        createAVSRewardsSubmission(createAVSRewardsSubmissionCall),
        createRewardsForAllEarners(createRewardsForAllEarnersCall),
        createRewardsForAllSubmission(createRewardsForAllSubmissionCall),
        cumulativeClaimed(cumulativeClaimedCall),
        currRewardsCalculationEndTimestamp(currRewardsCalculationEndTimestampCall),
        delegationManager(delegationManagerCall),
        disableRoot(disableRootCall),
        getCurrentClaimableDistributionRoot(getCurrentClaimableDistributionRootCall),
        getCurrentDistributionRoot(getCurrentDistributionRootCall),
        getDistributionRootAtIndex(getDistributionRootAtIndexCall),
        getDistributionRootsLength(getDistributionRootsLengthCall),
        getRootIndexFromHash(getRootIndexFromHashCall),
        globalOperatorCommissionBips(globalOperatorCommissionBipsCall),
        initialize(initializeCall),
        isAVSRewardsSubmissionHash(isAVSRewardsSubmissionHashCall),
        isRewardsForAllSubmitter(isRewardsForAllSubmitterCall),
        isRewardsSubmissionForAllEarnersHash(isRewardsSubmissionForAllEarnersHashCall),
        isRewardsSubmissionForAllHash(isRewardsSubmissionForAllHashCall),
        operatorCommissionBips(operatorCommissionBipsCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        processClaim(processClaimCall),
        renounceOwnership(renounceOwnershipCall),
        rewardsUpdater(rewardsUpdaterCall),
        setActivationDelay(setActivationDelayCall),
        setClaimerFor(setClaimerForCall),
        setGlobalOperatorCommission(setGlobalOperatorCommissionCall),
        setPauserRegistry(setPauserRegistryCall),
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
            [9u8, 45u8, 176u8, 7u8],
            [14u8, 154u8, 83u8, 207u8],
            [14u8, 179u8, 131u8, 69u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 20u8, 51u8, 180u8],
            [19u8, 100u8, 57u8, 221u8],
            [20u8, 155u8, 200u8, 114u8],
            [34u8, 241u8, 154u8, 100u8],
            [43u8, 159u8, 100u8, 164u8],
            [54u8, 175u8, 65u8, 250u8],
            [55u8, 131u8, 142u8, 208u8],
            [57u8, 183u8, 14u8, 56u8],
            [58u8, 140u8, 7u8, 134u8],
            [60u8, 204u8, 134u8, 29u8],
            [62u8, 254u8, 29u8, 182u8],
            [77u8, 24u8, 204u8, 53u8],
            [88u8, 186u8, 170u8, 62u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [94u8, 157u8, 131u8, 72u8],
            [109u8, 33u8, 17u8, 126u8],
            [113u8, 80u8, 24u8, 166u8],
            [123u8, 143u8, 139u8, 5u8],
            [134u8, 60u8, 185u8, 169u8],
            [134u8, 92u8, 105u8, 83u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [145u8, 4u8, 195u8, 25u8],
            [155u8, 227u8, 212u8, 228u8],
            [157u8, 69u8, 194u8, 129u8],
            [160u8, 22u8, 157u8, 221u8],
            [174u8, 189u8, 139u8, 174u8],
            [187u8, 126u8, 69u8, 31u8],
            [191u8, 33u8, 168u8, 170u8],
            [196u8, 109u8, 182u8, 6u8],
            [212u8, 84u8, 10u8, 85u8],
            [222u8, 2u8, 229u8, 3u8],
            [226u8, 33u8, 178u8, 69u8],
            [232u8, 16u8, 206u8, 33u8],
            [234u8, 77u8, 60u8, 155u8],
            [242u8, 253u8, 227u8, 139u8],
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
        const COUNT: usize = 50usize;
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
                Self::getRootIndexFromHash(_) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::globalOperatorCommissionBips(_) => {
                    <globalOperatorCommissionBipsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isAVSRewardsSubmissionHash(_) => {
                    <isAVSRewardsSubmissionHashCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::operatorCommissionBips(_) => {
                    <operatorCommissionBipsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setGlobalOperatorCommission(_) => {
                    <setGlobalOperatorCommissionCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn globalOperatorCommissionBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <globalOperatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::globalOperatorCommissionBips)
                    }
                    globalOperatorCommissionBips
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
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                    fn operatorCommissionBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <operatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::operatorCommissionBips)
                    }
                    operatorCommissionBips
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
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::setClaimerFor)
                    }
                    setClaimerFor
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
                    fn setGlobalOperatorCommission(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<RewardsCoordinatorCalls> {
                        <setGlobalOperatorCommissionCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(RewardsCoordinatorCalls::setGlobalOperatorCommission)
                    }
                    setGlobalOperatorCommission
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
                Self::getRootIndexFromHash(inner) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::globalOperatorCommissionBips(inner) => {
                    <globalOperatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::operatorCommissionBips(inner) => {
                    <operatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setGlobalOperatorCommission(inner) => {
                    <setGlobalOperatorCommissionCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getRootIndexFromHash(inner) => {
                    <getRootIndexFromHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::globalOperatorCommissionBips(inner) => {
                    <globalOperatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::operatorCommissionBips(inner) => {
                    <operatorCommissionBipsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setGlobalOperatorCommission(inner) => {
                    <setGlobalOperatorCommissionCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        InvalidCalculationIntervalSecondsRemainder(
            InvalidCalculationIntervalSecondsRemainder,
        ),
        InvalidClaimProof(InvalidClaimProof),
        InvalidDurationRemainder(InvalidDurationRemainder),
        InvalidEarnerLeafIndex(InvalidEarnerLeafIndex),
        InvalidGenesisRewardsTimestampRemainder(InvalidGenesisRewardsTimestampRemainder),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidProofLength(InvalidProofLength),
        InvalidRoot(InvalidRoot),
        InvalidRootIndex(InvalidRootIndex),
        InvalidStartTimestampRemainder(InvalidStartTimestampRemainder),
        InvalidTokenLeafIndex(InvalidTokenLeafIndex),
        NewRootMustBeForNewCalculatedPeriod(NewRootMustBeForNewCalculatedPeriod),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        RewardsEndTimestampNotElapsed(RewardsEndTimestampNotElapsed),
        RootAlreadyActivated(RootAlreadyActivated),
        RootDisabled(RootDisabled),
        RootNotActivated(RootNotActivated),
        StartTimestampTooFarInFuture(StartTimestampTooFarInFuture),
        StartTimestampTooFarInPast(StartTimestampTooFarInPast),
        StrategiesNotInAscendingOrder(StrategiesNotInAscendingOrder),
        StrategyNotWhitelisted(StrategyNotWhitelisted),
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
            [27u8, 20u8, 23u8, 75u8],
            [28u8, 45u8, 105u8, 188u8],
            [48u8, 219u8, 217u8, 148u8],
            [55u8, 66u8, 231u8, 212u8],
            [67u8, 113u8, 74u8, 253u8],
            [67u8, 173u8, 32u8, 252u8],
            [68u8, 120u8, 246u8, 114u8],
            [77u8, 197u8, 246u8, 164u8],
            [80u8, 69u8, 112u8, 227u8],
            [92u8, 66u8, 124u8, 217u8],
            [93u8, 251u8, 44u8, 162u8],
            [99u8, 97u8, 206u8, 128u8],
            [105u8, 202u8, 22u8, 201u8],
            [114u8, 159u8, 148u8, 44u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [121u8, 108u8, 197u8, 37u8],
            [126u8, 226u8, 180u8, 67u8],
            [132u8, 10u8, 72u8, 213u8],
            [148u8, 168u8, 211u8, 137u8],
            [161u8, 189u8, 21u8, 216u8],
            [169u8, 254u8, 155u8, 224u8],
            [170u8, 56u8, 94u8, 129u8],
            [198u8, 29u8, 202u8, 93u8],
            [223u8, 173u8, 156u8, 161u8],
            [238u8, 102u8, 71u8, 5u8],
            [240u8, 106u8, 83u8, 196u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for RewardsCoordinatorErrors {
        const NAME: &'static str = "RewardsCoordinatorErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
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
                Self::InvalidCalculationIntervalSecondsRemainder(_) => {
                    <InvalidCalculationIntervalSecondsRemainder as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidClaimProof(_) => {
                    <InvalidClaimProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidDurationRemainder(_) => {
                    <InvalidDurationRemainder as alloy_sol_types::SolError>::SELECTOR
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
        DistributionRootDisabled(DistributionRootDisabled),
        DistributionRootSubmitted(DistributionRootSubmitted),
        GlobalCommissionBipsSet(GlobalCommissionBipsSet),
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
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
                140u8,
                220u8,
                66u8,
                139u8,
                4u8,
                49u8,
                184u8,
                45u8,
                22u8,
                25u8,
                118u8,
                63u8,
                68u8,
                58u8,
                72u8,
                25u8,
                125u8,
                179u8,
                68u8,
                186u8,
                150u8,
                144u8,
                95u8,
                57u8,
                73u8,
                100u8,
                58u8,
                205u8,
                28u8,
                134u8,
                58u8,
                6u8,
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
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for RewardsCoordinatorEvents {
        const NAME: &'static str = "RewardsCoordinatorEvents";
        const COUNT: usize = 16usize;
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
                Some(
                    <GlobalCommissionBipsSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <GlobalCommissionBipsSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::GlobalCommissionBipsSet)
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
                Self::DistributionRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DistributionRootSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::GlobalCommissionBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
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
                Self::DistributionRootDisabled(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DistributionRootSubmitted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::GlobalCommissionBipsSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
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
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimerForCall, N> {
            self.call_builder(&claimerForCall { _0 })
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
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, cumulativeClaimedCall, N> {
            self.call_builder(&cumulativeClaimedCall { _0, _1 })
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
        ///Creates a new call builder for the [`globalOperatorCommissionBips`] function.
        pub fn globalOperatorCommissionBips(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, globalOperatorCommissionBipsCall, N> {
            self.call_builder(
                &globalOperatorCommissionBipsCall {
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _rewardsUpdater: alloy::sol_types::private::Address,
            _activationDelay: u32,
            _globalCommissionBips: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    _pauserRegistry,
                    initialPausedStatus,
                    _rewardsUpdater,
                    _activationDelay,
                    _globalCommissionBips,
                },
            )
        }
        ///Creates a new call builder for the [`isAVSRewardsSubmissionHash`] function.
        pub fn isAVSRewardsSubmissionHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, isAVSRewardsSubmissionHashCall, N> {
            self.call_builder(
                &isAVSRewardsSubmissionHashCall {
                    _0,
                    _1,
                },
            )
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
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            isRewardsSubmissionForAllEarnersHashCall,
            N,
        > {
            self.call_builder(
                &isRewardsSubmissionForAllEarnersHashCall {
                    _0,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`isRewardsSubmissionForAllHash`] function.
        pub fn isRewardsSubmissionForAllHash(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            isRewardsSubmissionForAllHashCall,
            N,
        > {
            self.call_builder(
                &isRewardsSubmissionForAllHashCall {
                    _0,
                    _1,
                },
            )
        }
        ///Creates a new call builder for the [`operatorCommissionBips`] function.
        pub fn operatorCommissionBips(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorCommissionBipsCall, N> {
            self.call_builder(
                &operatorCommissionBipsCall {
                    operator,
                    avs,
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
        ///Creates a new call builder for the [`setClaimerFor`] function.
        pub fn setClaimerFor(
            &self,
            claimer: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setClaimerForCall, N> {
            self.call_builder(&setClaimerForCall { claimer })
        }
        ///Creates a new call builder for the [`setGlobalOperatorCommission`] function.
        pub fn setGlobalOperatorCommission(
            &self,
            _globalCommissionBips: u16,
        ) -> alloy_contract::SolCallBuilder<T, &P, setGlobalOperatorCommissionCall, N> {
            self.call_builder(
                &setGlobalOperatorCommissionCall {
                    _globalCommissionBips,
                },
            )
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
        ///Creates a new event filter for the [`GlobalCommissionBipsSet`] event.
        pub fn GlobalCommissionBipsSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, GlobalCommissionBipsSet, N> {
            self.event_filter::<GlobalCommissionBipsSet>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
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
