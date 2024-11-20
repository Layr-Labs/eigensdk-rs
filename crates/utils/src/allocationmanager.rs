///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct MagnitudeAllocation { address strategy; uint64 expectedMaxMagnitude; OperatorSet[] operatorSets; uint64[] magnitudes; }
    struct MagnitudeInfo { uint64 currentMagnitude; int128 pendingDiff; uint32 effectTimestamp; }
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256 wadToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct MagnitudeAllocation { address strategy; uint64 expectedMaxMagnitude; OperatorSet[] operatorSets; uint64[] magnitudes; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MagnitudeAllocation {
        pub strategy: alloy::sol_types::private::Address,
        pub expectedMaxMagnitude: u64,
        pub operatorSets: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
        pub magnitudes: alloy::sol_types::private::Vec<u64>,
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
        impl ::core::convert::From<MagnitudeAllocation> for UnderlyingRustTuple<'_> {
            fn from(value: MagnitudeAllocation) -> Self {
                (
                    value.strategy,
                    value.expectedMaxMagnitude,
                    value.operatorSets,
                    value.magnitudes,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MagnitudeAllocation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    expectedMaxMagnitude: tuple.1,
                    operatorSets: tuple.2,
                    magnitudes: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MagnitudeAllocation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MagnitudeAllocation {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.expectedMaxMagnitude),
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSets),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::tokenize(&self.magnitudes),
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
        impl alloy_sol_types::SolType for MagnitudeAllocation {
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
        impl alloy_sol_types::SolStruct for MagnitudeAllocation {
            const NAME: &'static str = "MagnitudeAllocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MagnitudeAllocation(address strategy,uint64 expectedMaxMagnitude,OperatorSet[] operatorSets,uint64[] magnitudes)",
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
                            &self.expectedMaxMagnitude,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSets)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.magnitudes)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MagnitudeAllocation {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expectedMaxMagnitude,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSets,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.magnitudes,
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
                    &rust.expectedMaxMagnitude,
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
                    &rust.magnitudes,
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
struct MagnitudeInfo { uint64 currentMagnitude; int128 pendingDiff; uint32 effectTimestamp; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MagnitudeInfo {
        pub currentMagnitude: u64,
        pub pendingDiff: i128,
        pub effectTimestamp: u32,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Int<128>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u64, i128, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MagnitudeInfo> for UnderlyingRustTuple<'_> {
            fn from(value: MagnitudeInfo) -> Self {
                (value.currentMagnitude, value.pendingDiff, value.effectTimestamp)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MagnitudeInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentMagnitude: tuple.0,
                    pendingDiff: tuple.1,
                    effectTimestamp: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for MagnitudeInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for MagnitudeInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentMagnitude),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.pendingDiff),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectTimestamp),
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
        impl alloy_sol_types::SolType for MagnitudeInfo {
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
        impl alloy_sol_types::SolStruct for MagnitudeInfo {
            const NAME: &'static str = "MagnitudeInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MagnitudeInfo(uint64 currentMagnitude,int128 pendingDiff,uint32 effectTimestamp)",
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currentMagnitude,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pendingDiff)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.effectTimestamp,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for MagnitudeInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currentMagnitude,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pendingDiff,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.effectTimestamp,
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currentMagnitude,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.pendingDiff,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.effectTimestamp,
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
struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256 wadToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub wadToSlash: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<SlashingParams> for UnderlyingRustTuple<'_> {
            fn from(value: SlashingParams) -> Self {
                (
                    value.operator,
                    value.operatorSetId,
                    value.strategies,
                    value.wadToSlash,
                    value.description,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SlashingParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorSetId: tuple.1,
                    strategies: tuple.2,
                    wadToSlash: tuple.3,
                    description: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SlashingParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SlashingParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadToSlash),
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
        impl alloy_sol_types::SolType for SlashingParams {
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
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256 wadToSlash,string description)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadToSlash)
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
        impl alloy_sol_types::EventTopic for SlashingParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadToSlash,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operator,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadToSlash,
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAllocationManagerTypesInstance<T, P, N> {
        IAllocationManagerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAllocationManagerTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAllocationManagerTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAllocationManagerTypesInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAllocationManagerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAllocationManagerTypesInstance")
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
    > IAllocationManagerTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IAllocationManagerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAllocationManagerTypesInstance<T, P, N> {
            IAllocationManagerTypesInstance {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
    > IAllocationManagerTypesInstance<T, P, N> {
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
library IAllocationManagerTypes {
    struct MagnitudeAllocation {
        address strategy;
        uint64 expectedMaxMagnitude;
        OperatorSet[] operatorSets;
        uint64[] magnitudes;
    }
    struct MagnitudeInfo {
        uint64 currentMagnitude;
        int128 pendingDiff;
        uint32 effectTimestamp;
    }
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        address[] strategies;
        uint256 wadToSlash;
        string description;
    }
}

interface AllocationManager {
    struct OperatorSet {
        address avs;
        uint32 operatorSetId;
    }

    error AlreadySlashedForTimestamp();
    error CurrentlyPaused();
    error Empty();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InsufficientAllocatableMagnitude();
    error InvalidExpectedMaxMagnitude();
    error InvalidNewPausedStatus();
    error InvalidOperator();
    error InvalidOperatorSet();
    error InvalidSignature();
    error InvalidTimestamp();
    error InvalidWadToSlash();
    error ModificationAlreadyPending();
    error OnlyDelegationManager();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorNotAllocated();
    error OperatorNotRegistered();
    error OutOfBounds();
    error SaltSpent();
    error SameMagnitude();
    error SignatureExpired();
    error UninitializedAllocationDelay();

    event AllocationDelaySet(address operator, uint32 delay, uint32 effectTimestamp);
    event EncumberedMagnitudeUpdated(address operator, address strategy, uint64 encumberedMagnitude);
    event Initialized(uint8 version);
    event MaxMagnitudeUpdated(address operator, address strategy, uint64 maxMagnitude);
    event OperatorSetMagnitudeUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectTimestamp);
    event OperatorSlashed(address operator, OperatorSet operatorSet, address[] strategies, uint256[] wadSlashed, string description);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegation, address _avsDirectory, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);

    function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
    function DEALLOCATION_DELAY() external view returns (uint32);
    function avsDirectory() external view returns (address);
    function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
    function delegation() external view returns (address);
    function encumberedMagnitude(address operator, address strategy) external view returns (uint64);
    function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
    function getAllocationDelay(address operator) external view returns (bool isSet, uint32 delay);
    function getAllocationInfo(OperatorSet memory operatorSet, address[] memory strategies, address[] memory operators) external view returns (IAllocationManagerTypes.MagnitudeInfo[][] memory);
    function getAllocationInfo(address operator, address strategy, OperatorSet[] memory operatorSets) external view returns (IAllocationManagerTypes.MagnitudeInfo[] memory);
    function getAllocationInfo(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.MagnitudeInfo[] memory);
    function getCurrentDelegatedAndSlashableOperatorShares(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory, uint256[][] memory);
    function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
    function getMaxMagnitudesAtTimestamp(address operator, address[] memory strategies, uint32 timestamp) external view returns (uint64[] memory);
    function getMinDelegatedAndSlashableOperatorSharesBefore(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 beforeTimestamp) external view returns (uint256[][] memory, uint256[][] memory);
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
    function modifyAllocations(IAllocationManagerTypes.MagnitudeAllocation[] memory allocations) external;
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function renounceOwnership() external;
    function setAllocationDelay(address operator, uint32 delay) external;
    function setAllocationDelay(uint32 delay) external;
    function setPauserRegistry(address newPauserRegistry) external;
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
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
        "name": "_delegation",
        "type": "address",
        "internalType": "contract IDelegationManager"
      },
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_DEALLOCATION_DELAY",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "_ALLOCATION_CONFIGURATION_DELAY",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "ALLOCATION_CONFIGURATION_DELAY",
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
    "name": "DEALLOCATION_DELAY",
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
    "name": "clearDeallocationQueue",
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
      },
      {
        "name": "numToClear",
        "type": "uint16[]",
        "internalType": "uint16[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delegation",
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
    "name": "encumberedMagnitude",
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
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocatableMagnitude",
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
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocationDelay",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "isSet",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "delay",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocationInfo",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct IAllocationManagerTypes.MagnitudeInfo[][]",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectTimestamp",
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
    "name": "getAllocationInfo",
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
        "name": "operatorSets",
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.MagnitudeInfo[]",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectTimestamp",
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
    "name": "getAllocationInfo",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.MagnitudeInfo[]",
        "components": [
          {
            "name": "currentMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "pendingDiff",
            "type": "int128",
            "internalType": "int128"
          },
          {
            "name": "effectTimestamp",
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
    "name": "getCurrentDelegatedAndSlashableOperatorShares",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
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
      },
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
    "name": "getMaxMagnitudes",
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
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMaxMagnitudesAtTimestamp",
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
      },
      {
        "name": "timestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint64[]",
        "internalType": "uint64[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getMinDelegatedAndSlashableOperatorSharesBefore",
    "inputs": [
      {
        "name": "operatorSet",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "beforeTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      },
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
    "name": "modifyAllocations",
    "inputs": [
      {
        "name": "allocations",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.MagnitudeAllocation[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "expectedMaxMagnitude",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "operatorSets",
            "type": "tuple[]",
            "internalType": "struct OperatorSet[]",
            "components": [
              {
                "name": "avs",
                "type": "address",
                "internalType": "address"
              },
              {
                "name": "operatorSetId",
                "type": "uint32",
                "internalType": "uint32"
              }
            ]
          },
          {
            "name": "magnitudes",
            "type": "uint64[]",
            "internalType": "uint64[]"
          }
        ]
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
    "name": "renounceOwnership",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAllocationDelay",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "delay",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setAllocationDelay",
    "inputs": [
      {
        "name": "delay",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "slashOperator",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.SlashingParams",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          },
          {
            "name": "wadToSlash",
            "type": "uint256",
            "internalType": "uint256"
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
    "name": "AllocationDelaySet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "delay",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "effectTimestamp",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "EncumberedMagnitudeUpdated",
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
        "name": "encumberedMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
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
    "name": "MaxMagnitudeUpdated",
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
        "name": "maxMagnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSetMagnitudeUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "magnitude",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "effectTimestamp",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSlashed",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "operatorSet",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "strategies",
        "type": "address[]",
        "indexed": false,
        "internalType": "contract IStrategy[]"
      },
      {
        "name": "wadSlashed",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      },
      {
        "name": "description",
        "type": "string",
        "indexed": false,
        "internalType": "string"
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
    "name": "AlreadySlashedForTimestamp",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "Empty",
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
    "name": "InsufficientAllocatableMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidExpectedMaxMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperator",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidOperatorSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidTimestamp",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidWadToSlash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ModificationAlreadyPending",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyDelegationManager",
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
    "name": "OperatorNotAllocated",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfBounds",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaltSpent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SameMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
    "inputs": []
  },
  {
    "type": "error",
    "name": "UninitializedAllocationDelay",
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
pub mod AllocationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61010060405234801561001157600080fd5b506040516144f43803806144f483398101604081905261003091610154565b6001600160a01b03808516608052831660a05263ffffffff80831660c052811660e05261005b610064565b505050506101ad565b600054610100900460ff16156100d05760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811614610121576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b038116811461013857600080fd5b50565b805163ffffffff8116811461014f57600080fd5b919050565b6000806000806080858703121561016a57600080fd5b845161017581610123565b602086015190945061018681610123565b92506101946040860161013b565b91506101a26060860161013b565b905092959194509250565b60805160a05160c05160e0516142c861022c600039600081816104020152612a8d01526000818161025b0152610b78015260008181610390015281816109120152818161111201526114eb0152600081816104f701528181610fe2015281816112720152818161139b01528181611a2f0152611e7a01526142c86000f3fe608060405234801561001057600080fd5b50600436106101da5760003560e01c806360db99a3116101045780638da5cb5b116100a2578063b9fbaed111610071578063b9fbaed1146104c3578063df5cf723146104f2578063f2fde38b14610519578063fabc1cbc1461052c57600080fd5b80638da5cb5b1461044a57806393d7a72b1461045b57806394bd62a51461047c578063a984eb3a1461048f57600080fd5b8063715018a6116100de578063715018a6146103f55780637bc1ef61146103fd578063843b349f14610424578063886f11951461043757600080fd5b806360db99a3146103785780636b3aa72e1461038b5780636cfb4481146103ca57600080fd5b80634b5046ef1161017c578063595c6a671161014b578063595c6a67146103195780635ac86ab7146103215780635c489bb5146103545780635c975abb1461036757600080fd5b80634b5046ef146102b25780634d9dbde9146102c5578063547afb87146102e657806356c483e61461030657600080fd5b80631637b60f116101b85780631637b60f146102305780631794bb3c146102435780632981eb771461025657806335af054a1461029257600080fd5b80630b002119146101df57806310d67a2f14610208578063136439dd1461021d575b600080fd5b6101f26101ed36600461329d565b61053f565b6040516101ff9190613322565b60405180910390f35b61021b6102163660046133f1565b6106a4565b005b61021b61022b36600461340e565b610758565b61021b61023e366004613427565b610843565b61021b610251366004613468565b610d49565b61027d7f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016101ff565b6102a56102a036600461358e565b610e6e565b6040516101ff91906136c0565b61021b6102c03660046136d3565b610f7a565b6102d86102d336600461373c565b6110dd565b6040516101ff929190613775565b6102f96102f43660046137f1565b61119f565b6040516101ff9190613845565b61021b610314366004613891565b611267565b61021b6112be565b61034461032f3660046138bf565b606654600160ff9092169190911b9081161490565b60405190151581526020016101ff565b61021b6103623660046138e2565b611386565b6066546040519081526020016101ff565b61021b6103863660046138ff565b611435565b6103b27f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101ff565b6103dd6103d836600461373c565b611bbb565b6040516001600160401b0390911681526020016101ff565b61021b611d2e565b61027d7f000000000000000000000000000000000000000000000000000000000000000081565b6102f9610432366004613939565b611d42565b6065546103b2906001600160a01b031681565b6033546001600160a01b03166103b2565b61046e6104693660046139a0565b611e36565b6040516101ff929190613ac5565b61046e61048a36600461329d565b6121a5565b6103dd61049d36600461373c565b60986020908152600092835260408084209091529082529020546001600160401b031681565b6104d66104d13660046133f1565b6121c4565b60408051921515835263ffffffff9091166020830152016101ff565b6103b27f000000000000000000000000000000000000000000000000000000000000000081565b61021b6105273660046133f1565b612296565b61021b61053a36600461340e565b61230c565b60606000826001600160401b0381111561055b5761055b6134a9565b60405190808252806020026020018201604052801561058e57816020015b60608152602001906001900390816105795790505b50905060005b838110156106995760005b868110156106905760006106168787858181106105be576105be613aea565b90506020020160208101906105d391906133f1565b8a8a858181106105e5576105e5613aea565b90506020020160208101906105fa91906133f1565b61061161060c368f90038f018f613b00565b612414565b612474565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff1681525084848151811061066357610663613aea565b6020026020010151838151811061067c5761067c613aea565b60209081029190910101525060010161059f565b50600101610594565b509695505050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071b9190613b1c565b6001600160a01b0316336001600160a01b03161461074c5760405163794821ff60e01b815260040160405180910390fd5b610755816125dd565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156107a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107c49190613b39565b6107e157604051631d77d47760e21b815260040160405180910390fd5b606654818116146108055760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60665460009060019081160361086c5760405163840a48d560e01b815260040160405180910390fd5b600080610878336121c4565b915091508161089a5760405163fa55fc8160e01b815260040160405180910390fd5b60005b84811015610d4157368686838181106108b8576108b8613aea565b90506020028101906108ca9190613b5b565b90506108d96060820182613b7b565b90506108e86040830183613bc4565b905014610908576040516343714afd60e01b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166320c4e2366109446040840184613bc4565b6040518363ffffffff1660e01b8152600401610961929190613c42565b602060405180830381865afa15801561097e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109a29190613b39565b6109bf57604051631fb1705560e21b815260040160405180910390fd5b3360009081526097602090815260408220610a079183906109e2908601866133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002061266d565b9050610a196040830160208401613c7a565b6001600160401b0316816001600160401b031614610a4a5760405163982f66b360e01b815260040160405180910390fd5b610a6333610a5b60208501856133f1565b61ffff6126bb565b60005b610a736040840184613bc4565b9050811015610d33576000610ab4610a8e6040860186613bc4565b84818110610a9e57610a9e613aea565b90506040020180360381019061060c9190613b00565b90506000610acf33610ac960208801886133f1565b84612474565b90508060400151600f0b600014610af957604051630d8fcbe360e41b815260040160405180910390fd5b6020810151610b3990610b0f6060880188613b7b565b86818110610b1f57610b1f613aea565b9050602002016020810190610b349190613c7a565b6127c5565b600f0b60408201819052600003610b6357604051634606179360e11b815260040160405180910390fd5b60008160400151600f0b1215610c2657610b9d7f000000000000000000000000000000000000000000000000000000000000000042613cb9565b63ffffffff166060820152336000908152609a602090815260408220610c21928592610bcb908a018a6133f1565b6001600160a01b031681526020810191909152604001600020908154600160801b90819004600f0b6000818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b610c8c565b60008160400151600f0b1315610c8c57610c408742613cb9565b63ffffffff16606082015280516040820151610c5c91906127dd565b6001600160401b039081168083529085161015610c8c5760405163329d4e5360e21b815260040160405180910390fd5b610ca433610c9d60208801886133f1565b84846127f2565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf633610cd36040880188613bc4565b86818110610ce357610ce3613aea565b604002919091019050610cf960208901896133f1565b610d0b856020015186604001516127dd565b8560600151604051610d21959493929190613cd5565b60405180910390a15050600101610a66565b50505080600101905061089d565b505050505050565b600054610100900460ff1615808015610d695750600054600160ff909116105b80610d835750303b158015610d83575060005460ff166001145b610deb5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff191660011790558015610e0e576000805461ff0019166101001790555b610e188383612910565b610e2184612991565b8015610e68576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b6060600082516001600160401b03811115610e8b57610e8b6134a9565b604051908082528060200260200182016040528015610ed657816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610ea95790505b50905060005b8351811015610f6f576000610f0e8787610611888681518110610f0157610f01613aea565b6020026020010151612414565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250838381518110610f5b57610f5b613aea565b602090810291909101015250600101610edc565b5090505b9392505050565b606654600090600190811603610fa35760405163840a48d560e01b815260040160405180910390fd5b838214610fc3576040516343714afd60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0387811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015611029573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061104d9190613b39565b61106a576040516325ec6c1f60e01b815260040160405180910390fd5b60005b848110156110d4576110cc8787878481811061108b5761108b613aea565b90506020020160208101906110a091906133f1565b8686858181106110b2576110b2613aea565b90506020020160208101906110c79190613d26565b6126bb565b60010161106d565b50505050505050565b6040516316ae76cb60e01b81526001600160a01b038381166004830152600060248301819052600019604484015260609283927f000000000000000000000000000000000000000000000000000000000000000016906316ae76cb90606401600060405180830381865afa158015611159573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111819190810190613d4a565b90506000611190868684610e6e565b919350909150505b9250929050565b60606000826001600160401b038111156111bb576111bb6134a9565b6040519080825280602002602001820160405280156111e4578160200160208202803683370190505b50905060005b83811015610f6f576001600160a01b03861660009081526097602052604081206112359187878581811061122057611220613aea565b90506020020160208101906109e291906133f1565b82828151811061124757611247613aea565b6001600160401b03909216602092830291909101909101526001016111ea565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112b05760405163f739589b60e01b815260040160405180910390fd5b6112ba82826129e3565b5050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611306573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061132a9190613b39565b61134757604051631d77d47760e21b815260040160405180910390fd5b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa1580156113ea573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061140e9190613b39565b61142b576040516325ec6c1f60e01b815260040160405180910390fd5b61075533826129e3565b60665460019060029081160361145e5760405163840a48d560e01b815260040160405180910390fd5b8160600135600010801561147e5750670de0b6b3a7640000606083013511155b61149b57604051631353603160e01b815260040160405180910390fd5b60006040518060400160405280336001600160a01b031681526020018460200160208101906114ca91906138e2565b63ffffffff169052905060006114df82612414565b90506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631352c3e661151d60208701876133f1565b846040518363ffffffff1660e01b815260040161153b929190613e11565b602060405180830381865afa158015611558573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061157c9190613b39565b6115995760405163ccea9e6f60e01b815260040160405180910390fd5b60006115a86040860186613b7b565b90506001600160401b038111156115c1576115c16134a9565b6040519080825280602002602001820160405280156115ea578160200160208202803683370190505b50905060005b6115fd6040870187613b7b565b9050811015611b4e57600061165061161860208901896133f1565b61162560408a018a613b7b565b8581811061163557611635613aea565b905060200201602081019061164a91906133f1565b86612474565b9050600081602001516001600160401b03161161168057604051634e99e6cf60e01b815260040160405180910390fd5b602081015160009061169f906001600160401b031660608a0135612b91565b905080826020018181516116b39190613e47565b6001600160401b03169052508151819083906116d0908390613e47565b6001600160401b031690525060408201516000600f9190910b12156117cc5760006117168960600135846040015161170790613e66565b6001600160801b031690612b91565b9050806001600160401b0316836040018181516117339190613e8c565b600f0b9052507f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf661176760208b018b6133f1565b8861177560408d018d613b7b565b8881811061178557611785613aea565b905060200201602081019061179a91906133f1565b6117ac876020015188604001516127dd565b87606001516040516117c2959493929190613eb9565b60405180910390a1505b6118156117dc60208a018a6133f1565b6117e960408b018b613b7b565b868181106117f9576117f9613aea565b905060200201602081019061180e91906133f1565b87856127f2565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf661184360208a018a6133f1565b8761185160408c018c613b7b565b8781811061186157611861613aea565b905060200201602081019061187691906133f1565b85602001514260405161188d959493929190613eb9565b60405180910390a160006118ec6097826118aa60208d018d6133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002060008b80604001906118dc9190613b7b565b8881811061122057611220613aea565b905060006118fa8383613e47565b905061198f4282609760008e600001602081019061191891906133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002060008e806040019061194a9190613b7b565b8b81811061195a5761195a613aea565b905060200201602081019061196f91906133f1565b6001600160a01b0316815260208101919091526040016000209190612ba8565b507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c90506119c060208c018c6133f1565b6119cd60408d018d613b7b565b888181106119dd576119dd613aea565b90506020020160208101906119f291906133f1565b604080516001600160a01b0393841681529290911660208301526001600160401b0384169082015260600160405180910390a16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a57ab10b611a6160208d018d6133f1565b611a6e60408e018e613b7b565b89818110611a7e57611a7e613aea565b9050602002016020810190611a9391906133f1565b6040516001600160e01b031960e085901b1681526001600160a01b039283166004820152911660248201526001600160401b03808616604483015284166064820152608401600060405180830381600087803b158015611af257600080fd5b505af1158015611b06573d6000803e3d6000fd5b50611b21925050506001600160401b03848116908416612bc3565b868681518110611b3357611b33613aea565b602002602001018181525050505050508060010190506115f0565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5611b7d60208701876133f1565b84611b8b6040890189613b7b565b85611b9960808c018c613eef565b604051611bac9796959493929190613f75565b60405180910390a15050505050565b6001600160a01b03828116600081815260986020908152604080832094861680845294825280832054938352609a8252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611ce8576001600160a01b038087166000908152609a602090815260408083209389168352929052908120611c4f9083612bd8565b6001600160a01b038881166000908152609960209081526040808320938b168352928152828220848352815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250421015611ccb575050611ce8565b611cd98582602001516127dd565b94505050806001019050611c16565b506001600160a01b0380861660009081526097602090815260408083209388168352929052208290611d199061266d565b611d239190613e47565b925050505b92915050565b611d36612c49565b611d406000612991565b565b60606000836001600160401b03811115611d5e57611d5e6134a9565b604051908082528060200260200182016040528015611d87578160200160208202803683370190505b50905060005b84811015611e2c576001600160a01b0387166000908152609760205260408120611dfa91869190898986818110611dc657611dc6613aea565b9050602002016020810190611ddb91906133f1565b6001600160a01b03168152602081019190915260400160002090612ca3565b828281518110611e0c57611e0c613aea565b6001600160401b0390921660209283029190910190910152600101611d8d565b5095945050505050565b606080428363ffffffff161015611e605760405163b7d0949760e01b815260040160405180910390fd5b6000611e7461060c368b90038b018b613b00565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e6768a8a8a8a6040518563ffffffff1660e01b8152600401611eca9493929190614009565b600060405180830381865afa158015611ee7573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f0f919081019061406c565b90506000886001600160401b03811115611f2b57611f2b6134a9565b604051908082528060200260200182016040528015611f5e57816020015b6060815260200190600190039081611f495790505b50905060005b898110156121945760008b8b83818110611f8057611f80613aea565b9050602002016020810190611f9591906133f1565b9050886001600160401b03811115611faf57611faf6134a9565b604051908082528060200260200182016040528015611fd8578160200160208202803683370190505b50838381518110611feb57611feb613aea565b602002602001018190525060005b8981101561218a5760008b8b8381811061201557612015613aea565b905060200201602081019061202a91906133f1565b6001600160a01b03808516600090815260996020908152604080832093851683529281528282208b8352815290829020825160608101845290546001600160401b038116808352600160401b8204600f0b9383019390935263ffffffff600160c01b9091048116938201849052939450929091908d16106120b6576120b38183602001516127dd565b90505b6001600160a01b038086166000908152609760209081526040808320938716835292905220612145906120e89061266d565b6001600160401b031661213f836001600160401b03168b8a8151811061211057612110613aea565b6020026020010151888151811061212957612129613aea565b6020026020010151612cf490919063ffffffff16565b90612bc3565b87878151811061215757612157613aea565b6020026020010151858151811061217057612170613aea565b602002602001018181525050505050806001019050611ff9565b5050600101611f64565b50909a909950975050505050505050565b6060806121b6878787878742611e36565b915091509550959350505050565b6001600160a01b0381166000908152609b602090815260408083208151608081018352905463ffffffff808216835260ff600160201b830416151594830194909452650100000000008104841692820192909252600160481b90910490911660608201819052829190158015906122455750806060015163ffffffff164210155b15612256578060400151915061225b565b805191505b602081015115156001148061228e5750606081015163ffffffff161580159061228e5750806060015163ffffffff164210155b925050915091565b61229e612c49565b6001600160a01b0381166123035760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610de2565b61075581612991565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561235f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123839190613b1c565b6001600160a01b0316336001600160a01b0316146123b45760405163794821ff60e01b815260040160405180910390fd5b6066541981196066541916146123dd5760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610838565b60008160000151826020015163ffffffff1660405160200161245c92919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611d2890614175565b604080516080810182526000808252602080830182905282840182905260608084018390526001600160a01b0388811680855260998452868520918916808652918452868520888652845286852087519384018852546001600160401b038082168552600160401b8204600f0b8587015263ffffffff600160c01b9092048216858a019081529287526098865288872093875292909452959093205494519394909392169116421015612572576040518060800160405280826001600160401b0316815260200183600001516001600160401b031681526020018360200151600f0b8152602001836040015163ffffffff1681525092505050610f73565b612584826000015183602001516127dd565b6001600160401b0390811660208086019190915290821684526000606085018190526040850181905290830151600f0b12156125d4576125c88183602001516127dd565b6001600160401b031683525b50509392505050565b6001600160a01b038116612604576040516339b190bb60e11b815260040160405180910390fd5b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b805460009080156126ab5761269583612687600184614199565b600091825260209091200190565b54600160201b90046001600160401b0316610f73565b670de0b6b3a76400009392505050565b6001600160a01b038381166000908152609a60209081526040808320938616835292905290812054600f81810b600160801b909204900b035b60008111801561270757508261ffff1682105b156127be576001600160a01b038086166000908152609a60209081526040808320938816835292905290812061273c90612d09565b9050600061274b878784612474565b9050806060015163ffffffff164210156127665750506127be565b612772878784846127f2565b6001600160a01b038088166000908152609a60209081526040808320938a168352929052206127a090612d5d565b506127aa846141ac565b93506127b5836141c5565b925050506126f4565b5050505050565b6000610f736001600160401b038085169084166141dc565b6000610f73826001600160401b038516613e8c565b60408051606080820183526020848101516001600160401b03908116845285850151600f0b8285019081528684015163ffffffff9081168688019081526001600160a01b038c81166000818152609988528a8120928e168082529288528a81208d825288528a812099518a5496519451909516600160c01b0263ffffffff60c01b196001600160801b03909516600160401b026001600160c01b031990971695881695909517959095179290921692909217909655875186835260988552878320828452855291879020805492841667ffffffffffffffff1990931692909217909155865186519586529285015216928201929092527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559101610e5f565b6065546001600160a01b031615801561293157506001600160a01b03821615155b61294e576040516339b190bb60e11b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26112ba826125dd565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0382166000908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b909204166060820181905215801590612a605750806060015163ffffffff164210155b15612a7a57604081015163ffffffff168152600160208201525b63ffffffff8083166040830152612ab3907f00000000000000000000000000000000000000000000000000000000000000001642614209565b63ffffffff90811660608381019182526001600160a01b0386166000818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db910160405180910390a1505050565b6000610f738383670de0b6b3a76400006001612ddc565b600080612bb6858585612e39565b915091505b935093915050565b6000610f7383670de0b6b3a764000084613007565b600080612bfb612be7846130f1565b8554612bf69190600f0b61421c565b61315f565b8454909150600160801b9004600f90810b9082900b12612c2e57604051632d0483c560e21b815260040160405180910390fd5b600f0b60009081526001939093016020525050604090205490565b6033546001600160a01b03163314611d405760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610de2565b815460009081612cb5858583856131c8565b90508015612ce257612ccc85612687600184614199565b54600160201b90046001600160401b0316611d23565b50670de0b6b3a7640000949350505050565b6000610f738383670de0b6b3a7640000613007565b6000612d248254600f81810b600160801b909204900b131590565b15612d4257604051631ed9509560e11b815260040160405180910390fd5b508054600f0b60009081526001909101602052604090205490565b6000612d788254600f81810b600160801b909204900b131590565b15612d9657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b6000818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b600080612dea868686613007565b90506001836002811115612e0057612e00614244565b148015612e1d575060008480612e1857612e1861425a565b868809115b15612e3057612e2d600182614209565b90505b95945050505050565b825460009081908015612f9a576000612e5787612687600185614199565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b031660208401529192509087161015612ed85760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401610de2565b805163ffffffff808816911603612f295784612ef988612687600186614199565b80546001600160401b0392909216600160201b026bffffffffffffffff0000000019909216919091179055612f8a565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d5560008d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b602001519250839150612bbb9050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a5560008a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081612bbb565b6000808060001985870985870292508281108382030391505080600003613041578382816130375761303761425a565b0492505050610f73565b8084116130885760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401610de2565b60008486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091026000889003889004909101858311909403939093029303949094049190911702949350505050565b60006001600160ff1b0382111561315b5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401610de2565b5090565b80600f81900b81146131c35760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401610de2565b919050565b60005b8183101561321e5760006131df8484613226565b60008781526020902090915063ffffffff86169082015463ffffffff16111561320a57809250613218565b613215816001614209565b93505b506131cb565b509392505050565b60006132356002848418614270565b610f7390848416614209565b60006040828403121561325357600080fd5b50919050565b60008083601f84011261326b57600080fd5b5081356001600160401b0381111561328257600080fd5b6020830191508360208260051b850101111561119857600080fd5b6000806000806000608086880312156132b557600080fd5b6132bf8787613241565b945060408601356001600160401b038111156132da57600080fd5b6132e688828901613259565b90955093505060608601356001600160401b0381111561330557600080fd5b61331188828901613259565b969995985093965092949392505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156133d057868503603f190184528151805180875260209182019187019060005b818110156133b757835180516001600160401b03168452602080820151600f0b9085015260409081015163ffffffff16908401526060830160209490940193925060010161336e565b509096505050602093840193919091019060010161334a565b50929695505050505050565b6001600160a01b038116811461075557600080fd5b60006020828403121561340357600080fd5b8135610f73816133dc565b60006020828403121561342057600080fd5b5035919050565b6000806020838503121561343a57600080fd5b82356001600160401b0381111561345057600080fd5b61345c85828601613259565b90969095509350505050565b60008060006060848603121561347d57600080fd5b8335613488816133dc565b92506020840135613498816133dc565b929592945050506040919091013590565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156134e1576134e16134a9565b60405290565b604051601f8201601f191681016001600160401b038111828210171561350f5761350f6134a9565b604052919050565b60006001600160401b03821115613530576135306134a9565b5060051b60200190565b63ffffffff8116811461075557600080fd5b60006040828403121561355e57600080fd5b6135666134bf565b90508135613573816133dc565b815260208201356135838161353a565b602082015292915050565b6000806000606084860312156135a357600080fd5b83356135ae816133dc565b925060208401356135be816133dc565b915060408401356001600160401b038111156135d957600080fd5b8401601f810186136135ea57600080fd5b80356135fd6135f882613517565b6134e7565b8082825260208201915060208360061b85010192508883111561361f57600080fd5b6020840193505b8284101561364b57613638898561354c565b8252602082019150604084019350613626565b809450505050509250925092565b600081518084526020840193506020830160005b828110156136b657815180516001600160401b03168752602080820151600f0b9088015260409081015163ffffffff16908701526060860195506020919091019060010161366d565b5093949350505050565b602081526000610f736020830184613659565b6000806000806000606086880312156136eb57600080fd5b85356136f6816133dc565b945060208601356001600160401b0381111561371157600080fd5b61371d88828901613259565b90955093505060408601356001600160401b0381111561330557600080fd5b6000806040838503121561374f57600080fd5b823561375a816133dc565b9150602083013561376a816133dc565b809150509250929050565b6040808252835190820181905260009060208501906060840190835b818110156137d3576137bd83855180516001600160a01b0316825260209081015163ffffffff16910152565b6020939093019260409290920191600101613791565b505083810360208501526137e78186613659565b9695505050505050565b60008060006040848603121561380657600080fd5b8335613811816133dc565b925060208401356001600160401b0381111561382c57600080fd5b61383886828701613259565b9497909650939450505050565b602080825282518282018190526000918401906040840190835b818110156138865783516001600160401b031683526020938401939092019160010161385f565b509095945050505050565b600080604083850312156138a457600080fd5b82356138af816133dc565b9150602083013561376a8161353a565b6000602082840312156138d157600080fd5b813560ff81168114610f7357600080fd5b6000602082840312156138f457600080fd5b8135610f738161353a565b60006020828403121561391157600080fd5b81356001600160401b0381111561392757600080fd5b820160a08185031215610f7357600080fd5b6000806000806060858703121561394f57600080fd5b843561395a816133dc565b935060208501356001600160401b0381111561397557600080fd5b61398187828801613259565b90945092505060408501356139958161353a565b939692955090935050565b60008060008060008060a087890312156139b957600080fd5b6139c38888613241565b955060408701356001600160401b038111156139de57600080fd5b6139ea89828a01613259565b90965094505060608701356001600160401b03811115613a0957600080fd5b613a1589828a01613259565b9094509250506080870135613a298161353a565b809150509295509295509295565b600081518084526020840193506020830160005b828110156136b6578151865260209586019590910190600101613a4b565b600082825180855260208501945060208160051b8301016020850160005b83811015613ab957601f19858403018852613aa3838351613a37565b6020988901989093509190910190600101613a87565b50909695505050505050565b604081526000613ad86040830185613a69565b8281036020840152612e308185613a69565b634e487b7160e01b600052603260045260246000fd5b600060408284031215613b1257600080fd5b610f73838361354c565b600060208284031215613b2e57600080fd5b8151610f73816133dc565b600060208284031215613b4b57600080fd5b81518015158114610f7357600080fd5b60008235607e19833603018112613b7157600080fd5b9190910192915050565b6000808335601e19843603018112613b9257600080fd5b8301803591506001600160401b03821115613bac57600080fd5b6020019150600581901b360382131561119857600080fd5b6000808335601e19843603018112613bdb57600080fd5b8301803591506001600160401b03821115613bf557600080fd5b6020019150600681901b360382131561119857600080fd5b8035613c18816133dc565b6001600160a01b031682526020810135613c318161353a565b63ffffffff81166020840152505050565b6020808252810182905260008360408301825b85811015611e2c57613c678284613c0d565b6040928301929190910190600101613c55565b600060208284031215613c8c57600080fd5b81356001600160401b0381168114610f7357600080fd5b634e487b7160e01b600052601160045260246000fd5b63ffffffff8181168382160190811115611d2857611d28613ca3565b6001600160a01b038616815260c08101613cf26020830187613c0d565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b600060208284031215613d3857600080fd5b813561ffff81168114610f7357600080fd5b600060208284031215613d5c57600080fd5b81516001600160401b03811115613d7257600080fd5b8201601f81018413613d8357600080fd5b8051613d916135f882613517565b8082825260208201915060208360061b850101925086831115613db357600080fd5b6020840193505b828410156137e75760408488031215613dd257600080fd5b613dda6134bf565b8451613de5816133dc565b81526020850151613df58161353a565b8060208301525080835250602082019150604084019350613dba565b6001600160a01b038316815260608101610f73602083018480516001600160a01b0316825260209081015163ffffffff16910152565b6001600160401b038281168282160390811115611d2857611d28613ca3565b600081600f0b60016001607f1b03198103613e8357613e83613ca3565b60000392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715611d2857611d28613ca3565b6001600160a01b038616815260c08101613cf2602083018780516001600160a01b0316825260209081015163ffffffff16910152565b6000808335601e19843603018112613f0657600080fd5b8301803591506001600160401b03821115613f2057600080fd5b60200191503681900382131561119857600080fd5b81835260208301925060008160005b848110156136b6578135613f57816133dc565b6001600160a01b031686526020958601959190910190600101613f44565b6001600160a01b0388168152613fa7602082018880516001600160a01b0316825260209081015163ffffffff16910152565b60c060608201526000613fbe60c083018789613f35565b8281036080840152613fd08187613a37565b905082810360a0840152838152838560208301376000602085830101526020601f19601f86011682010191505098975050505050505050565b6040808252810184905260008560608301825b8781101561404c57823561402f816133dc565b6001600160a01b031682526020928301929091019060010161401c565b508381036020850152614060818688613f35565b98975050505050505050565b60006020828403121561407e57600080fd5b81516001600160401b0381111561409457600080fd5b8201601f810184136140a557600080fd5b80516140b36135f882613517565b8082825260208201915060208360051b8501019250868311156140d557600080fd5b602084015b838110156106995780516001600160401b038111156140f857600080fd5b8501603f8101891361410957600080fd5b602081015161411a6135f882613517565b808282526020820191506020808460051b8601010192508b83111561413e57600080fd5b6040840193505b82841015614160578351825260209384019390910190614145565b865250506020938401939190910190506140da565b805160208083015191908110156132535760001960209190910360031b1b16919050565b81810381811115611d2857611d28613ca3565b6000600182016141be576141be613ca3565b5060010190565b6000816141d4576141d4613ca3565b506000190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715611d2857611d28613ca3565b80820180821115611d2857611d28613ca3565b808201828112600083128015821682158216171561423c5761423c613ca3565b505092915050565b634e487b7160e01b600052602160045260246000fd5b634e487b7160e01b600052601260045260246000fd5b60008261428d57634e487b7160e01b600052601260045260246000fd5b50049056fea2646970667358221220ae4b42d4f75ff0ae3a57ba19d6450480878d0fa873221601aefa6342191d40e664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x11W`\0\x80\xFD[P`@QaD\xF48\x03\x80aD\xF4\x839\x81\x01`@\x81\x90Ra\x000\x91a\x01TV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x80R\x83\x16`\xA0Rc\xFF\xFF\xFF\xFF\x80\x83\x16`\xC0R\x81\x16`\xE0Ra\0[a\0dV[PPPPa\x01\xADV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xD0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\x01!W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x018W`\0\x80\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01OW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x01jW`\0\x80\xFD[\x84Qa\x01u\x81a\x01#V[` \x86\x01Q\x90\x94Pa\x01\x86\x81a\x01#V[\x92Pa\x01\x94`@\x86\x01a\x01;V[\x91Pa\x01\xA2``\x86\x01a\x01;V[\x90P\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0QaB\xC8a\x02,`\09`\0\x81\x81a\x04\x02\x01Ra*\x8D\x01R`\0\x81\x81a\x02[\x01Ra\x0Bx\x01R`\0\x81\x81a\x03\x90\x01R\x81\x81a\t\x12\x01R\x81\x81a\x11\x12\x01Ra\x14\xEB\x01R`\0\x81\x81a\x04\xF7\x01R\x81\x81a\x0F\xE2\x01R\x81\x81a\x12r\x01R\x81\x81a\x13\x9B\x01R\x81\x81a\x1A/\x01Ra\x1Ez\x01RaB\xC8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80c`\xDB\x99\xA3\x11a\x01\x04W\x80c\x8D\xA5\xCB[\x11a\0\xA2W\x80c\xB9\xFB\xAE\xD1\x11a\0qW\x80c\xB9\xFB\xAE\xD1\x14a\x04\xC3W\x80c\xDF\\\xF7#\x14a\x04\xF2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\x19W\x80c\xFA\xBC\x1C\xBC\x14a\x05,W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04JW\x80c\x93\xD7\xA7+\x14a\x04[W\x80c\x94\xBDb\xA5\x14a\x04|W\x80c\xA9\x84\xEB:\x14a\x04\x8FW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xDEW\x80cqP\x18\xA6\x14a\x03\xF5W\x80c{\xC1\xEFa\x14a\x03\xFDW\x80c\x84;4\x9F\x14a\x04$W\x80c\x88o\x11\x95\x14a\x047W`\0\x80\xFD[\x80c`\xDB\x99\xA3\x14a\x03xW\x80ck:\xA7.\x14a\x03\x8BW\x80cl\xFBD\x81\x14a\x03\xCAW`\0\x80\xFD[\x80cKPF\xEF\x11a\x01|W\x80cY\\jg\x11a\x01KW\x80cY\\jg\x14a\x03\x19W\x80cZ\xC8j\xB7\x14a\x03!W\x80c\\H\x9B\xB5\x14a\x03TW\x80c\\\x97Z\xBB\x14a\x03gW`\0\x80\xFD[\x80cKPF\xEF\x14a\x02\xB2W\x80cM\x9D\xBD\xE9\x14a\x02\xC5W\x80cTz\xFB\x87\x14a\x02\xE6W\x80cV\xC4\x83\xE6\x14a\x03\x06W`\0\x80\xFD[\x80c\x167\xB6\x0F\x11a\x01\xB8W\x80c\x167\xB6\x0F\x14a\x020W\x80c\x17\x94\xBB<\x14a\x02CW\x80c)\x81\xEBw\x14a\x02VW\x80c5\xAF\x05J\x14a\x02\x92W`\0\x80\xFD[\x80c\x0B\0!\x19\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02\x08W\x80c\x13d9\xDD\x14a\x02\x1DW[`\0\x80\xFD[a\x01\xF2a\x01\xED6`\x04a2\x9DV[a\x05?V[`@Qa\x01\xFF\x91\x90a3\"V[`@Q\x80\x91\x03\x90\xF3[a\x02\x1Ba\x02\x166`\x04a3\xF1V[a\x06\xA4V[\0[a\x02\x1Ba\x02+6`\x04a4\x0EV[a\x07XV[a\x02\x1Ba\x02>6`\x04a4'V[a\x08CV[a\x02\x1Ba\x02Q6`\x04a4hV[a\rIV[a\x02}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x02\xA5a\x02\xA06`\x04a5\x8EV[a\x0EnV[`@Qa\x01\xFF\x91\x90a6\xC0V[a\x02\x1Ba\x02\xC06`\x04a6\xD3V[a\x0FzV[a\x02\xD8a\x02\xD36`\x04a7<V[a\x10\xDDV[`@Qa\x01\xFF\x92\x91\x90a7uV[a\x02\xF9a\x02\xF46`\x04a7\xF1V[a\x11\x9FV[`@Qa\x01\xFF\x91\x90a8EV[a\x02\x1Ba\x03\x146`\x04a8\x91V[a\x12gV[a\x02\x1Ba\x12\xBEV[a\x03Da\x03/6`\x04a8\xBFV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x03b6`\x04a8\xE2V[a\x13\x86V[`fT`@Q\x90\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x03\x866`\x04a8\xFFV[a\x145V[a\x03\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x03\xDDa\x03\xD86`\x04a7<V[a\x1B\xBBV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x1D.V[a\x02}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xF9a\x0426`\x04a99V[a\x1DBV[`eTa\x03\xB2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xB2V[a\x04na\x04i6`\x04a9\xA0V[a\x1E6V[`@Qa\x01\xFF\x92\x91\x90a:\xC5V[a\x04na\x04\x8A6`\x04a2\x9DV[a!\xA5V[a\x03\xDDa\x04\x9D6`\x04a7<V[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\xD6a\x04\xD16`\x04a3\xF1V[a!\xC4V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xFFV[a\x03\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x1Ba\x05'6`\x04a3\xF1V[a\"\x96V[a\x02\x1Ba\x05:6`\x04a4\x0EV[a#\x0CV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05[Wa\x05[a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05yW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x06\x99W`\0[\x86\x81\x10\x15a\x06\x90W`\0a\x06\x16\x87\x87\x85\x81\x81\x10a\x05\xBEWa\x05\xBEa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x05\xD3\x91\x90a3\xF1V[\x8A\x8A\x85\x81\x81\x10a\x05\xE5Wa\x05\xE5a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x05\xFA\x91\x90a3\xF1V[a\x06\x11a\x06\x0C6\x8F\x90\x03\x8F\x01\x8Fa;\0V[a$\x14V[a$tV[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x84\x84\x81Q\x81\x10a\x06cWa\x06ca:\xEAV[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x06|Wa\x06|a:\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\x9FV[P`\x01\x01a\x05\x94V[P\x96\x95PPPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1B\x91\x90a;\x1CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07LW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U\x81a%\xDDV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC4\x91\x90a;9V[a\x07\xE1W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x14a\x08\x05W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT`\0\x90`\x01\x90\x81\x16\x03a\x08lW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x08x3a!\xC4V[\x91P\x91P\x81a\x08\x9AW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84\x81\x10\x15a\rAW6\x86\x86\x83\x81\x81\x10a\x08\xB8Wa\x08\xB8a:\xEAV[\x90P` \x02\x81\x01\x90a\x08\xCA\x91\x90a;[V[\x90Pa\x08\xD9``\x82\x01\x82a;{V[\x90Pa\x08\xE8`@\x83\x01\x83a;\xC4V[\x90P\x14a\t\x08W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c \xC4\xE26a\tD`@\x84\x01\x84a;\xC4V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\ta\x92\x91\x90a<BV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xA2\x91\x90a;9V[a\t\xBFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x97` \x90\x81R`@\x82 a\n\x07\x91\x83\x90a\t\xE2\x90\x86\x01\x86a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a&mV[\x90Pa\n\x19`@\x83\x01` \x84\x01a<zV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a\nJW`@Qc\x98/f\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nc3a\n[` \x85\x01\x85a3\xF1V[a\xFF\xFFa&\xBBV[`\0[a\ns`@\x84\x01\x84a;\xC4V[\x90P\x81\x10\x15a\r3W`\0a\n\xB4a\n\x8E`@\x86\x01\x86a;\xC4V[\x84\x81\x81\x10a\n\x9EWa\n\x9Ea:\xEAV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x06\x0C\x91\x90a;\0V[\x90P`\0a\n\xCF3a\n\xC9` \x88\x01\x88a3\xF1V[\x84a$tV[\x90P\x80`@\x01Q`\x0F\x0B`\0\x14a\n\xF9W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x0B9\x90a\x0B\x0F``\x88\x01\x88a;{V[\x86\x81\x81\x10a\x0B\x1FWa\x0B\x1Fa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x0B4\x91\x90a<zV[a'\xC5V[`\x0F\x0B`@\x82\x01\x81\x90R`\0\x03a\x0BcW`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@\x01Q`\x0F\x0B\x12\x15a\x0C&Wa\x0B\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba<\xB9V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R3`\0\x90\x81R`\x9A` \x90\x81R`@\x82 a\x0C!\x92\x85\x92a\x0B\xCB\x90\x8A\x01\x8Aa3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x0C\x8CV[`\0\x81`@\x01Q`\x0F\x0B\x13\x15a\x0C\x8CWa\x0C@\x87Ba<\xB9V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R\x80Q`@\x82\x01Qa\x0C\\\x91\x90a'\xDDV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x80\x83R\x90\x85\x16\x10\x15a\x0C\x8CW`@Qc2\x9DNS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xA43a\x0C\x9D` \x88\x01\x88a3\xF1V[\x84\x84a'\xF2V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF63a\x0C\xD3`@\x88\x01\x88a;\xC4V[\x86\x81\x81\x10a\x0C\xE3Wa\x0C\xE3a:\xEAV[`@\x02\x91\x90\x91\x01\x90Pa\x0C\xF9` \x89\x01\x89a3\xF1V[a\r\x0B\x85` \x01Q\x86`@\x01Qa'\xDDV[\x85``\x01Q`@Qa\r!\x95\x94\x93\x92\x91\x90a<\xD5V[`@Q\x80\x91\x03\x90\xA1PP`\x01\x01a\nfV[PPP\x80`\x01\x01\x90Pa\x08\x9DV[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\riWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\r\x83WP0;\x15\x80\x15a\r\x83WP`\0T`\xFF\x16`\x01\x14[a\r\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0E\x0EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0E\x18\x83\x83a)\x10V[a\x0E!\x84a)\x91V[\x80\x15a\x0EhW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x8BWa\x0E\x8Ba4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xD6W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0E\xA9W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0FoW`\0a\x0F\x0E\x87\x87a\x06\x11\x88\x86\x81Q\x81\x10a\x0F\x01Wa\x0F\x01a:\xEAV[` \x02` \x01\x01Qa$\x14V[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x83\x83\x81Q\x81\x10a\x0F[Wa\x0F[a:\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\xDCV[P\x90P[\x93\x92PPPV[`fT`\0\x90`\x01\x90\x81\x16\x03a\x0F\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x0F\xC3W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10M\x91\x90a;9V[a\x10jW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84\x81\x10\x15a\x10\xD4Wa\x10\xCC\x87\x87\x87\x84\x81\x81\x10a\x10\x8BWa\x10\x8Ba:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x10\xA0\x91\x90a3\xF1V[\x86\x86\x85\x81\x81\x10a\x10\xB2Wa\x10\xB2a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x10\xC7\x91\x90a=&V[a&\xBBV[`\x01\x01a\x10mV[PPPPPPPV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0`$\x83\x01\x81\x90R`\0\x19`D\x84\x01R``\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x16\xAEv\xCB\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x81\x91\x90\x81\x01\x90a=JV[\x90P`\0a\x11\x90\x86\x86\x84a\x0EnV[\x91\x93P\x90\x91PP[\x92P\x92\x90PV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xBBWa\x11\xBBa4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xE4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0FoW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x97` R`@\x81 a\x125\x91\x87\x87\x85\x81\x81\x10a\x12 Wa\x12 a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t\xE2\x91\x90a3\xF1V[\x82\x82\x81Q\x81\x10a\x12GWa\x12Ga:\xEAV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\xEAV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12\xB0W`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xBA\x82\x82a)\xE3V[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13*\x91\x90a;9V[a\x13GW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x0E\x91\x90a;9V[a\x14+W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U3\x82a)\xE3V[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x14^W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81``\x015`\0\x10\x80\x15a\x14~WPg\r\xE0\xB6\xB3\xA7d\0\0``\x83\x015\x11\x15[a\x14\x9BW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x14\xCA\x91\x90a8\xE2V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P`\0a\x14\xDF\x82a$\x14V[\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13R\xC3\xE6a\x15\x1D` \x87\x01\x87a3\xF1V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15;\x92\x91\x90a>\x11V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15|\x91\x90a;9V[a\x15\x99W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xA8`@\x86\x01\x86a;{V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xC1Wa\x15\xC1a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x15\xFD`@\x87\x01\x87a;{V[\x90P\x81\x10\x15a\x1BNW`\0a\x16Pa\x16\x18` \x89\x01\x89a3\xF1V[a\x16%`@\x8A\x01\x8Aa;{V[\x85\x81\x81\x10a\x165Wa\x165a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x16J\x91\x90a3\xF1V[\x86a$tV[\x90P`\0\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11a\x16\x80W`@QcN\x99\xE6\xCF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\0\x90a\x16\x9F\x90`\x01`\x01`@\x1B\x03\x16``\x8A\x015a+\x91V[\x90P\x80\x82` \x01\x81\x81Qa\x16\xB3\x91\x90a>GV[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Q\x81\x90\x83\x90a\x16\xD0\x90\x83\x90a>GV[`\x01`\x01`@\x1B\x03\x16\x90RP`@\x82\x01Q`\0`\x0F\x91\x90\x91\x0B\x12\x15a\x17\xCCW`\0a\x17\x16\x89``\x015\x84`@\x01Qa\x17\x07\x90a>fV[`\x01`\x01`\x80\x1B\x03\x16\x90a+\x91V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x83`@\x01\x81\x81Qa\x173\x91\x90a>\x8CV[`\x0F\x0B\x90RP\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x17g` \x8B\x01\x8Ba3\xF1V[\x88a\x17u`@\x8D\x01\x8Da;{V[\x88\x81\x81\x10a\x17\x85Wa\x17\x85a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x17\x9A\x91\x90a3\xF1V[a\x17\xAC\x87` \x01Q\x88`@\x01Qa'\xDDV[\x87``\x01Q`@Qa\x17\xC2\x95\x94\x93\x92\x91\x90a>\xB9V[`@Q\x80\x91\x03\x90\xA1P[a\x18\x15a\x17\xDC` \x8A\x01\x8Aa3\xF1V[a\x17\xE9`@\x8B\x01\x8Ba;{V[\x86\x81\x81\x10a\x17\xF9Wa\x17\xF9a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x18\x0E\x91\x90a3\xF1V[\x87\x85a'\xF2V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x18C` \x8A\x01\x8Aa3\xF1V[\x87a\x18Q`@\x8C\x01\x8Ca;{V[\x87\x81\x81\x10a\x18aWa\x18aa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x18v\x91\x90a3\xF1V[\x85` \x01QB`@Qa\x18\x8D\x95\x94\x93\x92\x91\x90a>\xB9V[`@Q\x80\x91\x03\x90\xA1`\0a\x18\xEC`\x97\x82a\x18\xAA` \x8D\x01\x8Da3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x80`@\x01\x90a\x18\xDC\x91\x90a;{V[\x88\x81\x81\x10a\x12 Wa\x12 a:\xEAV[\x90P`\0a\x18\xFA\x83\x83a>GV[\x90Pa\x19\x8FB\x82`\x97`\0\x8E`\0\x01` \x81\x01\x90a\x19\x18\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x80`@\x01\x90a\x19J\x91\x90a;{V[\x8B\x81\x81\x10a\x19ZWa\x19Za:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x19o\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x91\x90a+\xA8V[P\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90Pa\x19\xC0` \x8C\x01\x8Ca3\xF1V[a\x19\xCD`@\x8D\x01\x8Da;{V[\x88\x81\x81\x10a\x19\xDDWa\x19\xDDa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x19\xF2\x91\x90a3\xF1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R`\x01`\x01`@\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5z\xB1\x0Ba\x1Aa` \x8D\x01\x8Da3\xF1V[a\x1An`@\x8E\x01\x8Ea;{V[\x89\x81\x81\x10a\x1A~Wa\x1A~a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x93\x91\x90a3\xF1V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x83\x01R\x84\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x06W=`\0\x80>=`\0\xFD[Pa\x1B!\x92PPP`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x84\x16a+\xC3V[\x86\x86\x81Q\x81\x10a\x1B3Wa\x1B3a:\xEAV[` \x02` \x01\x01\x81\x81RPPPPPP\x80`\x01\x01\x90Pa\x15\xF0V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x1B}` \x87\x01\x87a3\xF1V[\x84a\x1B\x8B`@\x89\x01\x89a;{V[\x85a\x1B\x99`\x80\x8C\x01\x8Ca>\xEFV[`@Qa\x1B\xAC\x97\x96\x95\x94\x93\x92\x91\x90a?uV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x9A\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1C\xE8W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1CO\x90\x83a+\xD8V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PB\x10\x15a\x1C\xCBWPPa\x1C\xE8V[a\x1C\xD9\x85\x82` \x01Qa'\xDDV[\x94PPP\x80`\x01\x01\x90Pa\x1C\x16V[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1D\x19\x90a&mV[a\x1D#\x91\x90a>GV[\x92PPP[\x92\x91PPV[a\x1D6a,IV[a\x1D@`\0a)\x91V[V[```\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D^Wa\x1D^a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x87W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x1E,W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x97` R`@\x81 a\x1D\xFA\x91\x86\x91\x90\x89\x89\x86\x81\x81\x10a\x1D\xC6Wa\x1D\xC6a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1D\xDB\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x90a,\xA3V[\x82\x82\x81Q\x81\x10a\x1E\x0CWa\x1E\x0Ca:\xEAV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1D\x8DV[P\x95\x94PPPPPV[``\x80B\x83c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E`W`@Qc\xB7\xD0\x94\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1Eta\x06\x0C6\x8B\x90\x03\x8B\x01\x8Ba;\0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x8A\x8A\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xCA\x94\x93\x92\x91\x90a@\tV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x0F\x91\x90\x81\x01\x90a@lV[\x90P`\0\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F+Wa\x1F+a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F^W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FIW\x90P[P\x90P`\0[\x89\x81\x10\x15a!\x94W`\0\x8B\x8B\x83\x81\x81\x10a\x1F\x80Wa\x1F\x80a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1F\x95\x91\x90a3\xF1V[\x90P\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xAFWa\x1F\xAFa4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xD8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x1F\xEBWa\x1F\xEBa:\xEAV[` \x02` \x01\x01\x81\x90RP`\0[\x89\x81\x10\x15a!\x8AW`\0\x8B\x8B\x83\x81\x81\x10a \x15Wa \x15a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a *\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x8B\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\x01`@\x1B\x82\x04`\x0F\x0B\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x93\x82\x01\x84\x90R\x93\x94P\x92\x90\x91\x90\x8D\x16\x10a \xB6Wa \xB3\x81\x83` \x01Qa'\xDDV[\x90P[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a!E\x90a \xE8\x90a&mV[`\x01`\x01`@\x1B\x03\x16a!?\x83`\x01`\x01`@\x1B\x03\x16\x8B\x8A\x81Q\x81\x10a!\x10Wa!\x10a:\xEAV[` \x02` \x01\x01Q\x88\x81Q\x81\x10a!)Wa!)a:\xEAV[` \x02` \x01\x01Qa,\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a+\xC3V[\x87\x87\x81Q\x81\x10a!WWa!Wa:\xEAV[` \x02` \x01\x01Q\x85\x81Q\x81\x10a!pWa!pa:\xEAV[` \x02` \x01\x01\x81\x81RPPPPP\x80`\x01\x01\x90Pa\x1F\xF9V[PP`\x01\x01a\x1FdV[P\x90\x9A\x90\x99P\x97PPPPPPPPV[``\x80a!\xB6\x87\x87\x87\x87\x87Ba\x1E6V[\x91P\x91P\x95P\x95\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`H\x1B\x90\x91\x04\x90\x91\x16``\x82\x01\x81\x90R\x82\x91\x90\x15\x80\x15\x90a\"EWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\"VW\x80`@\x01Q\x91Pa\"[V[\x80Q\x91P[` \x81\x01Q\x15\x15`\x01\x14\x80a\"\x8EWP``\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\"\x8EWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x92PP\x91P\x91V[a\"\x9Ea,IV[`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\r\xE2V[a\x07U\x81a)\x91V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x83\x91\x90a;\x1CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xB4W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x19\x81\x19`fT\x19\x16\x14a#\xDDW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x088V[`\0\x81`\0\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a$\\\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1D(\x90aAuV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x80\x85R`\x99\x84R\x86\x85 \x91\x89\x16\x80\x86R\x91\x84R\x86\x85 \x88\x86R\x84R\x86\x85 \x87Q\x93\x84\x01\x88RT`\x01`\x01`@\x1B\x03\x80\x82\x16\x85R`\x01`@\x1B\x82\x04`\x0F\x0B\x85\x87\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x92\x04\x82\x16\x85\x8A\x01\x90\x81R\x92\x87R`\x98\x86R\x88\x87 \x93\x87R\x92\x90\x94R\x95\x90\x93 T\x94Q\x93\x94\x90\x93\x92\x16\x91\x16B\x10\x15a%rW`@Q\x80`\x80\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x92PPPa\x0FsV[a%\x84\x82`\0\x01Q\x83` \x01Qa'\xDDV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x86\x01\x91\x90\x91R\x90\x82\x16\x84R`\0``\x85\x01\x81\x90R`@\x85\x01\x81\x90R\x90\x83\x01Q`\x0F\x0B\x12\x15a%\xD4Wa%\xC8\x81\x83` \x01Qa'\xDDV[`\x01`\x01`@\x1B\x03\x16\x83R[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a&\x04W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80T`\0\x90\x80\x15a&\xABWa&\x95\x83a&\x87`\x01\x84aA\x99V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0FsV[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[`\0\x81\x11\x80\x15a'\x07WP\x82a\xFF\xFF\x16\x82\x10[\x15a'\xBEW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a'<\x90a-\tV[\x90P`\0a'K\x87\x87\x84a$tV[\x90P\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a'fWPPa'\xBEV[a'r\x87\x87\x84\x84a'\xF2V[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R a'\xA0\x90a-]V[Pa'\xAA\x84aA\xACV[\x93Pa'\xB5\x83aA\xC5V[\x92PPPa&\xF4V[PPPPPV[`\0a\x0Fs`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aA\xDCV[`\0a\x0Fs\x82`\x01`\x01`@\x1B\x03\x85\x16a>\x8CV[`@\x80Q``\x80\x82\x01\x83R` \x84\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x84R\x85\x85\x01Q`\x0F\x0B\x82\x85\x01\x90\x81R\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x86\x88\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16`\0\x81\x81R`\x99\x88R\x8A\x81 \x92\x8E\x16\x80\x82R\x92\x88R\x8A\x81 \x8D\x82R\x88R\x8A\x81 \x99Q\x8AT\x96Q\x94Q\x90\x95\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x90\x95\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x97\x16\x95\x88\x16\x95\x90\x95\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x90\x96U\x87Q\x86\x83R`\x98\x85R\x87\x83 \x82\x84R\x85R\x91\x87\x90 \x80T\x92\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x86Q\x86Q\x95\x86R\x92\x85\x01R\x16\x92\x82\x01\x92\x90\x92R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91\x01a\x0E_V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a)1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a)NW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x12\xBA\x82a%\xDDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a*`WP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a*zW`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x80\x83\x16`@\x83\x01Ra*\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16BaB\tV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x0Fs\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01a-\xDCV[`\0\x80a+\xB6\x85\x85\x85a.9V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x0Fs\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a0\x07V[`\0\x80a+\xFBa+\xE7\x84a0\xF1V[\x85Ta+\xF6\x91\x90`\x0F\x0BaB\x1CV[a1_V[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a,.W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B`\0\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\r\xE2V[\x81T`\0\x90\x81a,\xB5\x85\x85\x83\x85a1\xC8V[\x90P\x80\x15a,\xE2Wa,\xCC\x85a&\x87`\x01\x84aA\x99V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D#V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[`\0a\x0Fs\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\x07V[`\0a-$\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a-BW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[`\0a-x\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a-\x96W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`\0\x80a-\xEA\x86\x86\x86a0\x07V[\x90P`\x01\x83`\x02\x81\x11\x15a.\0Wa.\0aBDV[\x14\x80\x15a.\x1DWP`\0\x84\x80a.\x18Wa.\x18aBZV[\x86\x88\t\x11[\x15a.0Wa.-`\x01\x82aB\tV[\x90P[\x95\x94PPPPPV[\x82T`\0\x90\x81\x90\x80\x15a/\x9AW`\0a.W\x87a&\x87`\x01\x85aA\x99V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a.\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xE2V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a/)W\x84a.\xF9\x88a&\x87`\x01\x86aA\x99V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua/\x8AV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa+\xBB\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a+\xBBV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a0AW\x83\x82\x81a07Wa07aBZV[\x04\x92PPPa\x0FsV[\x80\x84\x11a0\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\r\xE2V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a1[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\r\xE2V[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a1\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\r\xE2V[\x91\x90PV[`\0[\x81\x83\x10\x15a2\x1EW`\0a1\xDF\x84\x84a2&V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a2\nW\x80\x92Pa2\x18V[a2\x15\x81`\x01aB\tV[\x93P[Pa1\xCBV[P\x93\x92PPPV[`\0a25`\x02\x84\x84\x18aBpV[a\x0Fs\x90\x84\x84\x16aB\tV[`\0`@\x82\x84\x03\x12\x15a2SW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a2kW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x82W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x11\x98W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2\xB5W`\0\x80\xFD[a2\xBF\x87\x87a2AV[\x94P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xDAW`\0\x80\xFD[a2\xE6\x88\x82\x89\x01a2YV[\x90\x95P\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[a3\x11\x88\x82\x89\x01a2YV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a3\xD0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87R` \x91\x82\x01\x91\x87\x01\x90`\0[\x81\x81\x10\x15a3\xB7W\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x84R` \x80\x82\x01Q`\x0F\x0B\x90\x85\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x84\x01R``\x83\x01` \x94\x90\x94\x01\x93\x92P`\x01\x01a3nV[P\x90\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a3JV[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4\x03W`\0\x80\xFD[\x815a\x0Fs\x81a3\xDCV[`\0` \x82\x84\x03\x12\x15a4 W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a4:W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a4PW`\0\x80\xFD[a4\\\x85\x82\x86\x01a2YV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a4}W`\0\x80\xFD[\x835a4\x88\x81a3\xDCV[\x92P` \x84\x015a4\x98\x81a3\xDCV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE1Wa4\xE1a4\xA9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x0FWa5\x0Fa4\xA9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a50Wa50a4\xA9V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07UW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a5^W`\0\x80\xFD[a5fa4\xBFV[\x90P\x815a5s\x81a3\xDCV[\x81R` \x82\x015a5\x83\x81a5:V[` \x82\x01R\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\xA3W`\0\x80\xFD[\x835a5\xAE\x81a3\xDCV[\x92P` \x84\x015a5\xBE\x81a3\xDCV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xD9W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a5\xEAW`\0\x80\xFD[\x805a5\xFDa5\xF8\x82a5\x17V[a4\xE7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a6\x1FW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6KWa68\x89\x85a5LV[\x82R` \x82\x01\x91P`@\x84\x01\x93Pa6&V[\x80\x94PPPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6\xB6W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x87R` \x80\x82\x01Q`\x0F\x0B\x90\x88\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R``\x86\x01\x95P` \x91\x90\x91\x01\x90`\x01\x01a6mV[P\x93\x94\x93PPPPV[` \x81R`\0a\x0Fs` \x83\x01\x84a6YV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a6\xEBW`\0\x80\xFD[\x855a6\xF6\x81a3\xDCV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x11W`\0\x80\xFD[a7\x1D\x88\x82\x89\x01a2YV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a7OW`\0\x80\xFD[\x825a7Z\x81a3\xDCV[\x91P` \x83\x015a7j\x81a3\xDCV[\x80\x91PP\x92P\x92\x90PV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\xD3Wa7\xBD\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a7\x91V[PP\x83\x81\x03` \x85\x01Ra7\xE7\x81\x86a6YV[\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8\x06W`\0\x80\xFD[\x835a8\x11\x81a3\xDCV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8,W`\0\x80\xFD[a88\x86\x82\x87\x01a2YV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a8\x86W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a8_V[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a8\xA4W`\0\x80\xFD[\x825a8\xAF\x81a3\xDCV[\x91P` \x83\x015a7j\x81a5:V[`\0` \x82\x84\x03\x12\x15a8\xD1W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xF4W`\0\x80\xFD[\x815a\x0Fs\x81a5:V[`\0` \x82\x84\x03\x12\x15a9\x11W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9'W`\0\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x0FsW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a9OW`\0\x80\xFD[\x845a9Z\x81a3\xDCV[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9uW`\0\x80\xFD[a9\x81\x87\x82\x88\x01a2YV[\x90\x94P\x92PP`@\x85\x015a9\x95\x81a5:V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a9\xB9W`\0\x80\xFD[a9\xC3\x88\x88a2AV[\x95P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xDEW`\0\x80\xFD[a9\xEA\x89\x82\x8A\x01a2YV[\x90\x96P\x94PP``\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\tW`\0\x80\xFD[a:\x15\x89\x82\x8A\x01a2YV[\x90\x94P\x92PP`\x80\x87\x015a:)\x81a5:V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6\xB6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a:KV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a:\xB9W`\x1F\x19\x85\x84\x03\x01\x88Ra:\xA3\x83\x83Qa:7V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a:\x87V[P\x90\x96\x95PPPPPPV[`@\x81R`\0a:\xD8`@\x83\x01\x85a:iV[\x82\x81\x03` \x84\x01Ra.0\x81\x85a:iV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a;\x12W`\0\x80\xFD[a\x0Fs\x83\x83a5LV[`\0` \x82\x84\x03\x12\x15a;.W`\0\x80\xFD[\x81Qa\x0Fs\x81a3\xDCV[`\0` \x82\x84\x03\x12\x15a;KW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0FsW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a;qW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;\x92W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xACW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xF5W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[\x805a<\x18\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a<1\x81a5:V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[` \x80\x82R\x81\x01\x82\x90R`\0\x83`@\x83\x01\x82[\x85\x81\x10\x15a\x1E,Wa<g\x82\x84a<\rV[`@\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a<UV[`\0` \x82\x84\x03\x12\x15a<\x8CW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a<\xF2` \x83\x01\x87a<\rV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=8W`\0\x80\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\\W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a=rW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a=\x83W`\0\x80\xFD[\x80Qa=\x91a5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a=\xB3W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a7\xE7W`@\x84\x88\x03\x12\x15a=\xD2W`\0\x80\xFD[a=\xDAa4\xBFV[\x84Qa=\xE5\x81a3\xDCV[\x81R` \x85\x01Qa=\xF5\x81a5:V[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa=\xBAV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0Fs` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a>\x83Wa>\x83a<\xA3V[`\0\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x1D(Wa\x1D(a<\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a<\xF2` \x83\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x06W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a? W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a6\xB6W\x815a?W\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a?DV[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81Ra?\xA7` \x82\x01\x88\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\xC0``\x82\x01R`\0a?\xBE`\xC0\x83\x01\x87\x89a?5V[\x82\x81\x03`\x80\x84\x01Ra?\xD0\x81\x87a:7V[\x90P\x82\x81\x03`\xA0\x84\x01R\x83\x81R\x83\x85` \x83\x017`\0` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x98\x97PPPPPPPPV[`@\x80\x82R\x81\x01\x84\x90R`\0\x85``\x83\x01\x82[\x87\x81\x10\x15a@LW\x825a@/\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a@\x1CV[P\x83\x81\x03` \x85\x01Ra@`\x81\x86\x88a?5V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a@~W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x94W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@\xA5W`\0\x80\xFD[\x80Qa@\xB3a5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a@\xD5W`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a\x06\x99W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF8W`\0\x80\xFD[\x85\x01`?\x81\x01\x89\x13aA\tW`\0\x80\xFD[` \x81\x01QaA\x1Aa5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aA>W`\0\x80\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aA`W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aAEV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa@\xDAV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a2SW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\0`\x01\x82\x01aA\xBEWaA\xBEa<\xA3V[P`\x01\x01\x90V[`\0\x81aA\xD4WaA\xD4a<\xA3V[P`\0\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x1D(Wa\x1D(a<\xA3V[\x80\x82\x01\x80\x82\x11\x15a\x1D(Wa\x1D(a<\xA3V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aB<WaB<a<\xA3V[PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aB\x8DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAEKB\xD4\xF7_\xF0\xAE:W\xBA\x19\xD6E\x04\x80\x87\x8D\x0F\xA8s\"\x16\x01\xAE\xFAcB\x19\x1D@\xE6dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101da5760003560e01c806360db99a3116101045780638da5cb5b116100a2578063b9fbaed111610071578063b9fbaed1146104c3578063df5cf723146104f2578063f2fde38b14610519578063fabc1cbc1461052c57600080fd5b80638da5cb5b1461044a57806393d7a72b1461045b57806394bd62a51461047c578063a984eb3a1461048f57600080fd5b8063715018a6116100de578063715018a6146103f55780637bc1ef61146103fd578063843b349f14610424578063886f11951461043757600080fd5b806360db99a3146103785780636b3aa72e1461038b5780636cfb4481146103ca57600080fd5b80634b5046ef1161017c578063595c6a671161014b578063595c6a67146103195780635ac86ab7146103215780635c489bb5146103545780635c975abb1461036757600080fd5b80634b5046ef146102b25780634d9dbde9146102c5578063547afb87146102e657806356c483e61461030657600080fd5b80631637b60f116101b85780631637b60f146102305780631794bb3c146102435780632981eb771461025657806335af054a1461029257600080fd5b80630b002119146101df57806310d67a2f14610208578063136439dd1461021d575b600080fd5b6101f26101ed36600461329d565b61053f565b6040516101ff9190613322565b60405180910390f35b61021b6102163660046133f1565b6106a4565b005b61021b61022b36600461340e565b610758565b61021b61023e366004613427565b610843565b61021b610251366004613468565b610d49565b61027d7f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016101ff565b6102a56102a036600461358e565b610e6e565b6040516101ff91906136c0565b61021b6102c03660046136d3565b610f7a565b6102d86102d336600461373c565b6110dd565b6040516101ff929190613775565b6102f96102f43660046137f1565b61119f565b6040516101ff9190613845565b61021b610314366004613891565b611267565b61021b6112be565b61034461032f3660046138bf565b606654600160ff9092169190911b9081161490565b60405190151581526020016101ff565b61021b6103623660046138e2565b611386565b6066546040519081526020016101ff565b61021b6103863660046138ff565b611435565b6103b27f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101ff565b6103dd6103d836600461373c565b611bbb565b6040516001600160401b0390911681526020016101ff565b61021b611d2e565b61027d7f000000000000000000000000000000000000000000000000000000000000000081565b6102f9610432366004613939565b611d42565b6065546103b2906001600160a01b031681565b6033546001600160a01b03166103b2565b61046e6104693660046139a0565b611e36565b6040516101ff929190613ac5565b61046e61048a36600461329d565b6121a5565b6103dd61049d36600461373c565b60986020908152600092835260408084209091529082529020546001600160401b031681565b6104d66104d13660046133f1565b6121c4565b60408051921515835263ffffffff9091166020830152016101ff565b6103b27f000000000000000000000000000000000000000000000000000000000000000081565b61021b6105273660046133f1565b612296565b61021b61053a36600461340e565b61230c565b60606000826001600160401b0381111561055b5761055b6134a9565b60405190808252806020026020018201604052801561058e57816020015b60608152602001906001900390816105795790505b50905060005b838110156106995760005b868110156106905760006106168787858181106105be576105be613aea565b90506020020160208101906105d391906133f1565b8a8a858181106105e5576105e5613aea565b90506020020160208101906105fa91906133f1565b61061161060c368f90038f018f613b00565b612414565b612474565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff1681525084848151811061066357610663613aea565b6020026020010151838151811061067c5761067c613aea565b60209081029190910101525060010161059f565b50600101610594565b509695505050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106f7573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061071b9190613b1c565b6001600160a01b0316336001600160a01b03161461074c5760405163794821ff60e01b815260040160405180910390fd5b610755816125dd565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156107a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107c49190613b39565b6107e157604051631d77d47760e21b815260040160405180910390fd5b606654818116146108055760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60665460009060019081160361086c5760405163840a48d560e01b815260040160405180910390fd5b600080610878336121c4565b915091508161089a5760405163fa55fc8160e01b815260040160405180910390fd5b60005b84811015610d4157368686838181106108b8576108b8613aea565b90506020028101906108ca9190613b5b565b90506108d96060820182613b7b565b90506108e86040830183613bc4565b905014610908576040516343714afd60e01b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166320c4e2366109446040840184613bc4565b6040518363ffffffff1660e01b8152600401610961929190613c42565b602060405180830381865afa15801561097e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906109a29190613b39565b6109bf57604051631fb1705560e21b815260040160405180910390fd5b3360009081526097602090815260408220610a079183906109e2908601866133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002061266d565b9050610a196040830160208401613c7a565b6001600160401b0316816001600160401b031614610a4a5760405163982f66b360e01b815260040160405180910390fd5b610a6333610a5b60208501856133f1565b61ffff6126bb565b60005b610a736040840184613bc4565b9050811015610d33576000610ab4610a8e6040860186613bc4565b84818110610a9e57610a9e613aea565b90506040020180360381019061060c9190613b00565b90506000610acf33610ac960208801886133f1565b84612474565b90508060400151600f0b600014610af957604051630d8fcbe360e41b815260040160405180910390fd5b6020810151610b3990610b0f6060880188613b7b565b86818110610b1f57610b1f613aea565b9050602002016020810190610b349190613c7a565b6127c5565b600f0b60408201819052600003610b6357604051634606179360e11b815260040160405180910390fd5b60008160400151600f0b1215610c2657610b9d7f000000000000000000000000000000000000000000000000000000000000000042613cb9565b63ffffffff166060820152336000908152609a602090815260408220610c21928592610bcb908a018a6133f1565b6001600160a01b031681526020810191909152604001600020908154600160801b90819004600f0b6000818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b610c8c565b60008160400151600f0b1315610c8c57610c408742613cb9565b63ffffffff16606082015280516040820151610c5c91906127dd565b6001600160401b039081168083529085161015610c8c5760405163329d4e5360e21b815260040160405180910390fd5b610ca433610c9d60208801886133f1565b84846127f2565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf633610cd36040880188613bc4565b86818110610ce357610ce3613aea565b604002919091019050610cf960208901896133f1565b610d0b856020015186604001516127dd565b8560600151604051610d21959493929190613cd5565b60405180910390a15050600101610a66565b50505080600101905061089d565b505050505050565b600054610100900460ff1615808015610d695750600054600160ff909116105b80610d835750303b158015610d83575060005460ff166001145b610deb5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff191660011790558015610e0e576000805461ff0019166101001790555b610e188383612910565b610e2184612991565b8015610e68576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b6060600082516001600160401b03811115610e8b57610e8b6134a9565b604051908082528060200260200182016040528015610ed657816020015b6040805160608101825260008082526020808301829052928201528252600019909201910181610ea95790505b50905060005b8351811015610f6f576000610f0e8787610611888681518110610f0157610f01613aea565b6020026020010151612414565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250838381518110610f5b57610f5b613aea565b602090810291909101015250600101610edc565b5090505b9392505050565b606654600090600190811603610fa35760405163840a48d560e01b815260040160405180910390fd5b838214610fc3576040516343714afd60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0387811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015611029573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061104d9190613b39565b61106a576040516325ec6c1f60e01b815260040160405180910390fd5b60005b848110156110d4576110cc8787878481811061108b5761108b613aea565b90506020020160208101906110a091906133f1565b8686858181106110b2576110b2613aea565b90506020020160208101906110c79190613d26565b6126bb565b60010161106d565b50505050505050565b6040516316ae76cb60e01b81526001600160a01b038381166004830152600060248301819052600019604484015260609283927f000000000000000000000000000000000000000000000000000000000000000016906316ae76cb90606401600060405180830381865afa158015611159573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526111819190810190613d4a565b90506000611190868684610e6e565b919350909150505b9250929050565b60606000826001600160401b038111156111bb576111bb6134a9565b6040519080825280602002602001820160405280156111e4578160200160208202803683370190505b50905060005b83811015610f6f576001600160a01b03861660009081526097602052604081206112359187878581811061122057611220613aea565b90506020020160208101906109e291906133f1565b82828151811061124757611247613aea565b6001600160401b03909216602092830291909101909101526001016111ea565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112b05760405163f739589b60e01b815260040160405180910390fd5b6112ba82826129e3565b5050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015611306573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061132a9190613b39565b61134757604051631d77d47760e21b815260040160405180910390fd5b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa1580156113ea573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061140e9190613b39565b61142b576040516325ec6c1f60e01b815260040160405180910390fd5b61075533826129e3565b60665460019060029081160361145e5760405163840a48d560e01b815260040160405180910390fd5b8160600135600010801561147e5750670de0b6b3a7640000606083013511155b61149b57604051631353603160e01b815260040160405180910390fd5b60006040518060400160405280336001600160a01b031681526020018460200160208101906114ca91906138e2565b63ffffffff169052905060006114df82612414565b90506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631352c3e661151d60208701876133f1565b846040518363ffffffff1660e01b815260040161153b929190613e11565b602060405180830381865afa158015611558573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061157c9190613b39565b6115995760405163ccea9e6f60e01b815260040160405180910390fd5b60006115a86040860186613b7b565b90506001600160401b038111156115c1576115c16134a9565b6040519080825280602002602001820160405280156115ea578160200160208202803683370190505b50905060005b6115fd6040870187613b7b565b9050811015611b4e57600061165061161860208901896133f1565b61162560408a018a613b7b565b8581811061163557611635613aea565b905060200201602081019061164a91906133f1565b86612474565b9050600081602001516001600160401b03161161168057604051634e99e6cf60e01b815260040160405180910390fd5b602081015160009061169f906001600160401b031660608a0135612b91565b905080826020018181516116b39190613e47565b6001600160401b03169052508151819083906116d0908390613e47565b6001600160401b031690525060408201516000600f9190910b12156117cc5760006117168960600135846040015161170790613e66565b6001600160801b031690612b91565b9050806001600160401b0316836040018181516117339190613e8c565b600f0b9052507f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf661176760208b018b6133f1565b8861177560408d018d613b7b565b8881811061178557611785613aea565b905060200201602081019061179a91906133f1565b6117ac876020015188604001516127dd565b87606001516040516117c2959493929190613eb9565b60405180910390a1505b6118156117dc60208a018a6133f1565b6117e960408b018b613b7b565b868181106117f9576117f9613aea565b905060200201602081019061180e91906133f1565b87856127f2565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf661184360208a018a6133f1565b8761185160408c018c613b7b565b8781811061186157611861613aea565b905060200201602081019061187691906133f1565b85602001514260405161188d959493929190613eb9565b60405180910390a160006118ec6097826118aa60208d018d6133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002060008b80604001906118dc9190613b7b565b8881811061122057611220613aea565b905060006118fa8383613e47565b905061198f4282609760008e600001602081019061191891906133f1565b6001600160a01b03166001600160a01b0316815260200190815260200160002060008e806040019061194a9190613b7b565b8b81811061195a5761195a613aea565b905060200201602081019061196f91906133f1565b6001600160a01b0316815260208101919091526040016000209190612ba8565b507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c90506119c060208c018c6133f1565b6119cd60408d018d613b7b565b888181106119dd576119dd613aea565b90506020020160208101906119f291906133f1565b604080516001600160a01b0393841681529290911660208301526001600160401b0384169082015260600160405180910390a16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a57ab10b611a6160208d018d6133f1565b611a6e60408e018e613b7b565b89818110611a7e57611a7e613aea565b9050602002016020810190611a9391906133f1565b6040516001600160e01b031960e085901b1681526001600160a01b039283166004820152911660248201526001600160401b03808616604483015284166064820152608401600060405180830381600087803b158015611af257600080fd5b505af1158015611b06573d6000803e3d6000fd5b50611b21925050506001600160401b03848116908416612bc3565b868681518110611b3357611b33613aea565b602002602001018181525050505050508060010190506115f0565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5611b7d60208701876133f1565b84611b8b6040890189613b7b565b85611b9960808c018c613eef565b604051611bac9796959493929190613f75565b60405180910390a15050505050565b6001600160a01b03828116600081815260986020908152604080832094861680845294825280832054938352609a8252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611ce8576001600160a01b038087166000908152609a602090815260408083209389168352929052908120611c4f9083612bd8565b6001600160a01b038881166000908152609960209081526040808320938b168352928152828220848352815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250421015611ccb575050611ce8565b611cd98582602001516127dd565b94505050806001019050611c16565b506001600160a01b0380861660009081526097602090815260408083209388168352929052208290611d199061266d565b611d239190613e47565b925050505b92915050565b611d36612c49565b611d406000612991565b565b60606000836001600160401b03811115611d5e57611d5e6134a9565b604051908082528060200260200182016040528015611d87578160200160208202803683370190505b50905060005b84811015611e2c576001600160a01b0387166000908152609760205260408120611dfa91869190898986818110611dc657611dc6613aea565b9050602002016020810190611ddb91906133f1565b6001600160a01b03168152602081019190915260400160002090612ca3565b828281518110611e0c57611e0c613aea565b6001600160401b0390921660209283029190910190910152600101611d8d565b5095945050505050565b606080428363ffffffff161015611e605760405163b7d0949760e01b815260040160405180910390fd5b6000611e7461060c368b90038b018b613b00565b905060007f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e6768a8a8a8a6040518563ffffffff1660e01b8152600401611eca9493929190614009565b600060405180830381865afa158015611ee7573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f0f919081019061406c565b90506000886001600160401b03811115611f2b57611f2b6134a9565b604051908082528060200260200182016040528015611f5e57816020015b6060815260200190600190039081611f495790505b50905060005b898110156121945760008b8b83818110611f8057611f80613aea565b9050602002016020810190611f9591906133f1565b9050886001600160401b03811115611faf57611faf6134a9565b604051908082528060200260200182016040528015611fd8578160200160208202803683370190505b50838381518110611feb57611feb613aea565b602002602001018190525060005b8981101561218a5760008b8b8381811061201557612015613aea565b905060200201602081019061202a91906133f1565b6001600160a01b03808516600090815260996020908152604080832093851683529281528282208b8352815290829020825160608101845290546001600160401b038116808352600160401b8204600f0b9383019390935263ffffffff600160c01b9091048116938201849052939450929091908d16106120b6576120b38183602001516127dd565b90505b6001600160a01b038086166000908152609760209081526040808320938716835292905220612145906120e89061266d565b6001600160401b031661213f836001600160401b03168b8a8151811061211057612110613aea565b6020026020010151888151811061212957612129613aea565b6020026020010151612cf490919063ffffffff16565b90612bc3565b87878151811061215757612157613aea565b6020026020010151858151811061217057612170613aea565b602002602001018181525050505050806001019050611ff9565b5050600101611f64565b50909a909950975050505050505050565b6060806121b6878787878742611e36565b915091509550959350505050565b6001600160a01b0381166000908152609b602090815260408083208151608081018352905463ffffffff808216835260ff600160201b830416151594830194909452650100000000008104841692820192909252600160481b90910490911660608201819052829190158015906122455750806060015163ffffffff164210155b15612256578060400151915061225b565b805191505b602081015115156001148061228e5750606081015163ffffffff161580159061228e5750806060015163ffffffff164210155b925050915091565b61229e612c49565b6001600160a01b0381166123035760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610de2565b61075581612991565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561235f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123839190613b1c565b6001600160a01b0316336001600160a01b0316146123b45760405163794821ff60e01b815260040160405180910390fd5b6066541981196066541916146123dd5760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610838565b60008160000151826020015163ffffffff1660405160200161245c92919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611d2890614175565b604080516080810182526000808252602080830182905282840182905260608084018390526001600160a01b0388811680855260998452868520918916808652918452868520888652845286852087519384018852546001600160401b038082168552600160401b8204600f0b8587015263ffffffff600160c01b9092048216858a019081529287526098865288872093875292909452959093205494519394909392169116421015612572576040518060800160405280826001600160401b0316815260200183600001516001600160401b031681526020018360200151600f0b8152602001836040015163ffffffff1681525092505050610f73565b612584826000015183602001516127dd565b6001600160401b0390811660208086019190915290821684526000606085018190526040850181905290830151600f0b12156125d4576125c88183602001516127dd565b6001600160401b031683525b50509392505050565b6001600160a01b038116612604576040516339b190bb60e11b815260040160405180910390fd5b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b805460009080156126ab5761269583612687600184614199565b600091825260209091200190565b54600160201b90046001600160401b0316610f73565b670de0b6b3a76400009392505050565b6001600160a01b038381166000908152609a60209081526040808320938616835292905290812054600f81810b600160801b909204900b035b60008111801561270757508261ffff1682105b156127be576001600160a01b038086166000908152609a60209081526040808320938816835292905290812061273c90612d09565b9050600061274b878784612474565b9050806060015163ffffffff164210156127665750506127be565b612772878784846127f2565b6001600160a01b038088166000908152609a60209081526040808320938a168352929052206127a090612d5d565b506127aa846141ac565b93506127b5836141c5565b925050506126f4565b5050505050565b6000610f736001600160401b038085169084166141dc565b6000610f73826001600160401b038516613e8c565b60408051606080820183526020848101516001600160401b03908116845285850151600f0b8285019081528684015163ffffffff9081168688019081526001600160a01b038c81166000818152609988528a8120928e168082529288528a81208d825288528a812099518a5496519451909516600160c01b0263ffffffff60c01b196001600160801b03909516600160401b026001600160c01b031990971695881695909517959095179290921692909217909655875186835260988552878320828452855291879020805492841667ffffffffffffffff1990931692909217909155865186519586529285015216928201929092527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559101610e5f565b6065546001600160a01b031615801561293157506001600160a01b03821615155b61294e576040516339b190bb60e11b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26112ba826125dd565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b0382166000908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b909204166060820181905215801590612a605750806060015163ffffffff164210155b15612a7a57604081015163ffffffff168152600160208201525b63ffffffff8083166040830152612ab3907f00000000000000000000000000000000000000000000000000000000000000001642614209565b63ffffffff90811660608381019182526001600160a01b0386166000818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db910160405180910390a1505050565b6000610f738383670de0b6b3a76400006001612ddc565b600080612bb6858585612e39565b915091505b935093915050565b6000610f7383670de0b6b3a764000084613007565b600080612bfb612be7846130f1565b8554612bf69190600f0b61421c565b61315f565b8454909150600160801b9004600f90810b9082900b12612c2e57604051632d0483c560e21b815260040160405180910390fd5b600f0b60009081526001939093016020525050604090205490565b6033546001600160a01b03163314611d405760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610de2565b815460009081612cb5858583856131c8565b90508015612ce257612ccc85612687600184614199565b54600160201b90046001600160401b0316611d23565b50670de0b6b3a7640000949350505050565b6000610f738383670de0b6b3a7640000613007565b6000612d248254600f81810b600160801b909204900b131590565b15612d4257604051631ed9509560e11b815260040160405180910390fd5b508054600f0b60009081526001909101602052604090205490565b6000612d788254600f81810b600160801b909204900b131590565b15612d9657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b6000818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b600080612dea868686613007565b90506001836002811115612e0057612e00614244565b148015612e1d575060008480612e1857612e1861425a565b868809115b15612e3057612e2d600182614209565b90505b95945050505050565b825460009081908015612f9a576000612e5787612687600185614199565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b031660208401529192509087161015612ed85760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401610de2565b805163ffffffff808816911603612f295784612ef988612687600186614199565b80546001600160401b0392909216600160201b026bffffffffffffffff0000000019909216919091179055612f8a565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d5560008d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b602001519250839150612bbb9050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a5560008a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081612bbb565b6000808060001985870985870292508281108382030391505080600003613041578382816130375761303761425a565b0492505050610f73565b8084116130885760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401610de2565b60008486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091026000889003889004909101858311909403939093029303949094049190911702949350505050565b60006001600160ff1b0382111561315b5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401610de2565b5090565b80600f81900b81146131c35760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401610de2565b919050565b60005b8183101561321e5760006131df8484613226565b60008781526020902090915063ffffffff86169082015463ffffffff16111561320a57809250613218565b613215816001614209565b93505b506131cb565b509392505050565b60006132356002848418614270565b610f7390848416614209565b60006040828403121561325357600080fd5b50919050565b60008083601f84011261326b57600080fd5b5081356001600160401b0381111561328257600080fd5b6020830191508360208260051b850101111561119857600080fd5b6000806000806000608086880312156132b557600080fd5b6132bf8787613241565b945060408601356001600160401b038111156132da57600080fd5b6132e688828901613259565b90955093505060608601356001600160401b0381111561330557600080fd5b61331188828901613259565b969995985093965092949392505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156133d057868503603f190184528151805180875260209182019187019060005b818110156133b757835180516001600160401b03168452602080820151600f0b9085015260409081015163ffffffff16908401526060830160209490940193925060010161336e565b509096505050602093840193919091019060010161334a565b50929695505050505050565b6001600160a01b038116811461075557600080fd5b60006020828403121561340357600080fd5b8135610f73816133dc565b60006020828403121561342057600080fd5b5035919050565b6000806020838503121561343a57600080fd5b82356001600160401b0381111561345057600080fd5b61345c85828601613259565b90969095509350505050565b60008060006060848603121561347d57600080fd5b8335613488816133dc565b92506020840135613498816133dc565b929592945050506040919091013590565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156134e1576134e16134a9565b60405290565b604051601f8201601f191681016001600160401b038111828210171561350f5761350f6134a9565b604052919050565b60006001600160401b03821115613530576135306134a9565b5060051b60200190565b63ffffffff8116811461075557600080fd5b60006040828403121561355e57600080fd5b6135666134bf565b90508135613573816133dc565b815260208201356135838161353a565b602082015292915050565b6000806000606084860312156135a357600080fd5b83356135ae816133dc565b925060208401356135be816133dc565b915060408401356001600160401b038111156135d957600080fd5b8401601f810186136135ea57600080fd5b80356135fd6135f882613517565b6134e7565b8082825260208201915060208360061b85010192508883111561361f57600080fd5b6020840193505b8284101561364b57613638898561354c565b8252602082019150604084019350613626565b809450505050509250925092565b600081518084526020840193506020830160005b828110156136b657815180516001600160401b03168752602080820151600f0b9088015260409081015163ffffffff16908701526060860195506020919091019060010161366d565b5093949350505050565b602081526000610f736020830184613659565b6000806000806000606086880312156136eb57600080fd5b85356136f6816133dc565b945060208601356001600160401b0381111561371157600080fd5b61371d88828901613259565b90955093505060408601356001600160401b0381111561330557600080fd5b6000806040838503121561374f57600080fd5b823561375a816133dc565b9150602083013561376a816133dc565b809150509250929050565b6040808252835190820181905260009060208501906060840190835b818110156137d3576137bd83855180516001600160a01b0316825260209081015163ffffffff16910152565b6020939093019260409290920191600101613791565b505083810360208501526137e78186613659565b9695505050505050565b60008060006040848603121561380657600080fd5b8335613811816133dc565b925060208401356001600160401b0381111561382c57600080fd5b61383886828701613259565b9497909650939450505050565b602080825282518282018190526000918401906040840190835b818110156138865783516001600160401b031683526020938401939092019160010161385f565b509095945050505050565b600080604083850312156138a457600080fd5b82356138af816133dc565b9150602083013561376a8161353a565b6000602082840312156138d157600080fd5b813560ff81168114610f7357600080fd5b6000602082840312156138f457600080fd5b8135610f738161353a565b60006020828403121561391157600080fd5b81356001600160401b0381111561392757600080fd5b820160a08185031215610f7357600080fd5b6000806000806060858703121561394f57600080fd5b843561395a816133dc565b935060208501356001600160401b0381111561397557600080fd5b61398187828801613259565b90945092505060408501356139958161353a565b939692955090935050565b60008060008060008060a087890312156139b957600080fd5b6139c38888613241565b955060408701356001600160401b038111156139de57600080fd5b6139ea89828a01613259565b90965094505060608701356001600160401b03811115613a0957600080fd5b613a1589828a01613259565b9094509250506080870135613a298161353a565b809150509295509295509295565b600081518084526020840193506020830160005b828110156136b6578151865260209586019590910190600101613a4b565b600082825180855260208501945060208160051b8301016020850160005b83811015613ab957601f19858403018852613aa3838351613a37565b6020988901989093509190910190600101613a87565b50909695505050505050565b604081526000613ad86040830185613a69565b8281036020840152612e308185613a69565b634e487b7160e01b600052603260045260246000fd5b600060408284031215613b1257600080fd5b610f73838361354c565b600060208284031215613b2e57600080fd5b8151610f73816133dc565b600060208284031215613b4b57600080fd5b81518015158114610f7357600080fd5b60008235607e19833603018112613b7157600080fd5b9190910192915050565b6000808335601e19843603018112613b9257600080fd5b8301803591506001600160401b03821115613bac57600080fd5b6020019150600581901b360382131561119857600080fd5b6000808335601e19843603018112613bdb57600080fd5b8301803591506001600160401b03821115613bf557600080fd5b6020019150600681901b360382131561119857600080fd5b8035613c18816133dc565b6001600160a01b031682526020810135613c318161353a565b63ffffffff81166020840152505050565b6020808252810182905260008360408301825b85811015611e2c57613c678284613c0d565b6040928301929190910190600101613c55565b600060208284031215613c8c57600080fd5b81356001600160401b0381168114610f7357600080fd5b634e487b7160e01b600052601160045260246000fd5b63ffffffff8181168382160190811115611d2857611d28613ca3565b6001600160a01b038616815260c08101613cf26020830187613c0d565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b600060208284031215613d3857600080fd5b813561ffff81168114610f7357600080fd5b600060208284031215613d5c57600080fd5b81516001600160401b03811115613d7257600080fd5b8201601f81018413613d8357600080fd5b8051613d916135f882613517565b8082825260208201915060208360061b850101925086831115613db357600080fd5b6020840193505b828410156137e75760408488031215613dd257600080fd5b613dda6134bf565b8451613de5816133dc565b81526020850151613df58161353a565b8060208301525080835250602082019150604084019350613dba565b6001600160a01b038316815260608101610f73602083018480516001600160a01b0316825260209081015163ffffffff16910152565b6001600160401b038281168282160390811115611d2857611d28613ca3565b600081600f0b60016001607f1b03198103613e8357613e83613ca3565b60000392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715611d2857611d28613ca3565b6001600160a01b038616815260c08101613cf2602083018780516001600160a01b0316825260209081015163ffffffff16910152565b6000808335601e19843603018112613f0657600080fd5b8301803591506001600160401b03821115613f2057600080fd5b60200191503681900382131561119857600080fd5b81835260208301925060008160005b848110156136b6578135613f57816133dc565b6001600160a01b031686526020958601959190910190600101613f44565b6001600160a01b0388168152613fa7602082018880516001600160a01b0316825260209081015163ffffffff16910152565b60c060608201526000613fbe60c083018789613f35565b8281036080840152613fd08187613a37565b905082810360a0840152838152838560208301376000602085830101526020601f19601f86011682010191505098975050505050505050565b6040808252810184905260008560608301825b8781101561404c57823561402f816133dc565b6001600160a01b031682526020928301929091019060010161401c565b508381036020850152614060818688613f35565b98975050505050505050565b60006020828403121561407e57600080fd5b81516001600160401b0381111561409457600080fd5b8201601f810184136140a557600080fd5b80516140b36135f882613517565b8082825260208201915060208360051b8501019250868311156140d557600080fd5b602084015b838110156106995780516001600160401b038111156140f857600080fd5b8501603f8101891361410957600080fd5b602081015161411a6135f882613517565b808282526020820191506020808460051b8601010192508b83111561413e57600080fd5b6040840193505b82841015614160578351825260209384019390910190614145565b865250506020938401939190910190506140da565b805160208083015191908110156132535760001960209190910360031b1b16919050565b81810381811115611d2857611d28613ca3565b6000600182016141be576141be613ca3565b5060010190565b6000816141d4576141d4613ca3565b506000190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715611d2857611d28613ca3565b80820180821115611d2857611d28613ca3565b808201828112600083128015821682158216171561423c5761423c613ca3565b505092915050565b634e487b7160e01b600052602160045260246000fd5b634e487b7160e01b600052601260045260246000fd5b60008261428d57634e487b7160e01b600052601260045260246000fd5b50049056fea2646970667358221220ae4b42d4f75ff0ae3a57ba19d6450480878d0fa873221601aefa6342191d40e664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80c`\xDB\x99\xA3\x11a\x01\x04W\x80c\x8D\xA5\xCB[\x11a\0\xA2W\x80c\xB9\xFB\xAE\xD1\x11a\0qW\x80c\xB9\xFB\xAE\xD1\x14a\x04\xC3W\x80c\xDF\\\xF7#\x14a\x04\xF2W\x80c\xF2\xFD\xE3\x8B\x14a\x05\x19W\x80c\xFA\xBC\x1C\xBC\x14a\x05,W`\0\x80\xFD[\x80c\x8D\xA5\xCB[\x14a\x04JW\x80c\x93\xD7\xA7+\x14a\x04[W\x80c\x94\xBDb\xA5\x14a\x04|W\x80c\xA9\x84\xEB:\x14a\x04\x8FW`\0\x80\xFD[\x80cqP\x18\xA6\x11a\0\xDEW\x80cqP\x18\xA6\x14a\x03\xF5W\x80c{\xC1\xEFa\x14a\x03\xFDW\x80c\x84;4\x9F\x14a\x04$W\x80c\x88o\x11\x95\x14a\x047W`\0\x80\xFD[\x80c`\xDB\x99\xA3\x14a\x03xW\x80ck:\xA7.\x14a\x03\x8BW\x80cl\xFBD\x81\x14a\x03\xCAW`\0\x80\xFD[\x80cKPF\xEF\x11a\x01|W\x80cY\\jg\x11a\x01KW\x80cY\\jg\x14a\x03\x19W\x80cZ\xC8j\xB7\x14a\x03!W\x80c\\H\x9B\xB5\x14a\x03TW\x80c\\\x97Z\xBB\x14a\x03gW`\0\x80\xFD[\x80cKPF\xEF\x14a\x02\xB2W\x80cM\x9D\xBD\xE9\x14a\x02\xC5W\x80cTz\xFB\x87\x14a\x02\xE6W\x80cV\xC4\x83\xE6\x14a\x03\x06W`\0\x80\xFD[\x80c\x167\xB6\x0F\x11a\x01\xB8W\x80c\x167\xB6\x0F\x14a\x020W\x80c\x17\x94\xBB<\x14a\x02CW\x80c)\x81\xEBw\x14a\x02VW\x80c5\xAF\x05J\x14a\x02\x92W`\0\x80\xFD[\x80c\x0B\0!\x19\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02\x08W\x80c\x13d9\xDD\x14a\x02\x1DW[`\0\x80\xFD[a\x01\xF2a\x01\xED6`\x04a2\x9DV[a\x05?V[`@Qa\x01\xFF\x91\x90a3\"V[`@Q\x80\x91\x03\x90\xF3[a\x02\x1Ba\x02\x166`\x04a3\xF1V[a\x06\xA4V[\0[a\x02\x1Ba\x02+6`\x04a4\x0EV[a\x07XV[a\x02\x1Ba\x02>6`\x04a4'V[a\x08CV[a\x02\x1Ba\x02Q6`\x04a4hV[a\rIV[a\x02}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x02\xA5a\x02\xA06`\x04a5\x8EV[a\x0EnV[`@Qa\x01\xFF\x91\x90a6\xC0V[a\x02\x1Ba\x02\xC06`\x04a6\xD3V[a\x0FzV[a\x02\xD8a\x02\xD36`\x04a7<V[a\x10\xDDV[`@Qa\x01\xFF\x92\x91\x90a7uV[a\x02\xF9a\x02\xF46`\x04a7\xF1V[a\x11\x9FV[`@Qa\x01\xFF\x91\x90a8EV[a\x02\x1Ba\x03\x146`\x04a8\x91V[a\x12gV[a\x02\x1Ba\x12\xBEV[a\x03Da\x03/6`\x04a8\xBFV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x03b6`\x04a8\xE2V[a\x13\x86V[`fT`@Q\x90\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x03\x866`\x04a8\xFFV[a\x145V[a\x03\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x03\xDDa\x03\xD86`\x04a7<V[a\x1B\xBBV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[a\x02\x1Ba\x1D.V[a\x02}\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xF9a\x0426`\x04a99V[a\x1DBV[`eTa\x03\xB2\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xB2V[a\x04na\x04i6`\x04a9\xA0V[a\x1E6V[`@Qa\x01\xFF\x92\x91\x90a:\xC5V[a\x04na\x04\x8A6`\x04a2\x9DV[a!\xA5V[a\x03\xDDa\x04\x9D6`\x04a7<V[`\x98` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\xD6a\x04\xD16`\x04a3\xF1V[a!\xC4V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xFFV[a\x03\xB2\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x1Ba\x05'6`\x04a3\xF1V[a\"\x96V[a\x02\x1Ba\x05:6`\x04a4\x0EV[a#\x0CV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x05[Wa\x05[a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05\x8EW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05yW\x90P[P\x90P`\0[\x83\x81\x10\x15a\x06\x99W`\0[\x86\x81\x10\x15a\x06\x90W`\0a\x06\x16\x87\x87\x85\x81\x81\x10a\x05\xBEWa\x05\xBEa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x05\xD3\x91\x90a3\xF1V[\x8A\x8A\x85\x81\x81\x10a\x05\xE5Wa\x05\xE5a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x05\xFA\x91\x90a3\xF1V[a\x06\x11a\x06\x0C6\x8F\x90\x03\x8F\x01\x8Fa;\0V[a$\x14V[a$tV[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x84\x84\x81Q\x81\x10a\x06cWa\x06ca:\xEAV[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x06|Wa\x06|a:\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05\x9FV[P`\x01\x01a\x05\x94V[P\x96\x95PPPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xF7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x1B\x91\x90a;\x1CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07LW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U\x81a%\xDDV[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\xC4\x91\x90a;9V[a\x07\xE1W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x14a\x08\x05W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT`\0\x90`\x01\x90\x81\x16\x03a\x08lW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x08x3a!\xC4V[\x91P\x91P\x81a\x08\x9AW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84\x81\x10\x15a\rAW6\x86\x86\x83\x81\x81\x10a\x08\xB8Wa\x08\xB8a:\xEAV[\x90P` \x02\x81\x01\x90a\x08\xCA\x91\x90a;[V[\x90Pa\x08\xD9``\x82\x01\x82a;{V[\x90Pa\x08\xE8`@\x83\x01\x83a;\xC4V[\x90P\x14a\t\x08W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c \xC4\xE26a\tD`@\x84\x01\x84a;\xC4V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\ta\x92\x91\x90a<BV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t~W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xA2\x91\x90a;9V[a\t\xBFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[3`\0\x90\x81R`\x97` \x90\x81R`@\x82 a\n\x07\x91\x83\x90a\t\xE2\x90\x86\x01\x86a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 a&mV[\x90Pa\n\x19`@\x83\x01` \x84\x01a<zV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a\nJW`@Qc\x98/f\xB3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\nc3a\n[` \x85\x01\x85a3\xF1V[a\xFF\xFFa&\xBBV[`\0[a\ns`@\x84\x01\x84a;\xC4V[\x90P\x81\x10\x15a\r3W`\0a\n\xB4a\n\x8E`@\x86\x01\x86a;\xC4V[\x84\x81\x81\x10a\n\x9EWa\n\x9Ea:\xEAV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x06\x0C\x91\x90a;\0V[\x90P`\0a\n\xCF3a\n\xC9` \x88\x01\x88a3\xF1V[\x84a$tV[\x90P\x80`@\x01Q`\x0F\x0B`\0\x14a\n\xF9W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\x0B9\x90a\x0B\x0F``\x88\x01\x88a;{V[\x86\x81\x81\x10a\x0B\x1FWa\x0B\x1Fa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x0B4\x91\x90a<zV[a'\xC5V[`\x0F\x0B`@\x82\x01\x81\x90R`\0\x03a\x0BcW`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x81`@\x01Q`\x0F\x0B\x12\x15a\x0C&Wa\x0B\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba<\xB9V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R3`\0\x90\x81R`\x9A` \x90\x81R`@\x82 a\x0C!\x92\x85\x92a\x0B\xCB\x90\x8A\x01\x8Aa3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B`\0\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x0C\x8CV[`\0\x81`@\x01Q`\x0F\x0B\x13\x15a\x0C\x8CWa\x0C@\x87Ba<\xB9V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R\x80Q`@\x82\x01Qa\x0C\\\x91\x90a'\xDDV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x80\x83R\x90\x85\x16\x10\x15a\x0C\x8CW`@Qc2\x9DNS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0C\xA43a\x0C\x9D` \x88\x01\x88a3\xF1V[\x84\x84a'\xF2V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF63a\x0C\xD3`@\x88\x01\x88a;\xC4V[\x86\x81\x81\x10a\x0C\xE3Wa\x0C\xE3a:\xEAV[`@\x02\x91\x90\x91\x01\x90Pa\x0C\xF9` \x89\x01\x89a3\xF1V[a\r\x0B\x85` \x01Q\x86`@\x01Qa'\xDDV[\x85``\x01Q`@Qa\r!\x95\x94\x93\x92\x91\x90a<\xD5V[`@Q\x80\x91\x03\x90\xA1PP`\x01\x01a\nfV[PPP\x80`\x01\x01\x90Pa\x08\x9DV[PPPPPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\riWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\r\x83WP0;\x15\x80\x15a\r\x83WP`\0T`\xFF\x16`\x01\x14[a\r\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x0E\x0EW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x0E\x18\x83\x83a)\x10V[a\x0E!\x84a)\x91V[\x80\x15a\x0EhW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E\x8BWa\x0E\x8Ba4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xD6W\x81` \x01[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R`\0\x19\x90\x92\x01\x91\x01\x81a\x0E\xA9W\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x0FoW`\0a\x0F\x0E\x87\x87a\x06\x11\x88\x86\x81Q\x81\x10a\x0F\x01Wa\x0F\x01a:\xEAV[` \x02` \x01\x01Qa$\x14V[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x83\x83\x81Q\x81\x10a\x0F[Wa\x0F[a:\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\xDCV[P\x90P[\x93\x92PPPV[`fT`\0\x90`\x01\x90\x81\x16\x03a\x0F\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x0F\xC3W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10)W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10M\x91\x90a;9V[a\x10jW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84\x81\x10\x15a\x10\xD4Wa\x10\xCC\x87\x87\x87\x84\x81\x81\x10a\x10\x8BWa\x10\x8Ba:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x10\xA0\x91\x90a3\xF1V[\x86\x86\x85\x81\x81\x10a\x10\xB2Wa\x10\xB2a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x10\xC7\x91\x90a=&V[a&\xBBV[`\x01\x01a\x10mV[PPPPPPPV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R`\0`$\x83\x01\x81\x90R`\0\x19`D\x84\x01R``\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x16\xAEv\xCB\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11\x81\x91\x90\x81\x01\x90a=JV[\x90P`\0a\x11\x90\x86\x86\x84a\x0EnV[\x91\x93P\x90\x91PP[\x92P\x92\x90PV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\xBBWa\x11\xBBa4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\xE4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x0FoW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x97` R`@\x81 a\x125\x91\x87\x87\x85\x81\x81\x10a\x12 Wa\x12 a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t\xE2\x91\x90a3\xF1V[\x82\x82\x81Q\x81\x10a\x12GWa\x12Ga:\xEAV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\xEAV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12\xB0W`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xBA\x82\x82a)\xE3V[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x06W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13*\x91\x90a;9V[a\x13GW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x0E\x91\x90a;9V[a\x14+W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07U3\x82a)\xE3V[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x14^W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81``\x015`\0\x10\x80\x15a\x14~WPg\r\xE0\xB6\xB3\xA7d\0\0``\x83\x015\x11\x15[a\x14\x9BW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x14\xCA\x91\x90a8\xE2V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P`\0a\x14\xDF\x82a$\x14V[\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13R\xC3\xE6a\x15\x1D` \x87\x01\x87a3\xF1V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x15;\x92\x91\x90a>\x11V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15|\x91\x90a;9V[a\x15\x99W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15\xA8`@\x86\x01\x86a;{V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xC1Wa\x15\xC1a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[a\x15\xFD`@\x87\x01\x87a;{V[\x90P\x81\x10\x15a\x1BNW`\0a\x16Pa\x16\x18` \x89\x01\x89a3\xF1V[a\x16%`@\x8A\x01\x8Aa;{V[\x85\x81\x81\x10a\x165Wa\x165a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x16J\x91\x90a3\xF1V[\x86a$tV[\x90P`\0\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11a\x16\x80W`@QcN\x99\xE6\xCF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q`\0\x90a\x16\x9F\x90`\x01`\x01`@\x1B\x03\x16``\x8A\x015a+\x91V[\x90P\x80\x82` \x01\x81\x81Qa\x16\xB3\x91\x90a>GV[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Q\x81\x90\x83\x90a\x16\xD0\x90\x83\x90a>GV[`\x01`\x01`@\x1B\x03\x16\x90RP`@\x82\x01Q`\0`\x0F\x91\x90\x91\x0B\x12\x15a\x17\xCCW`\0a\x17\x16\x89``\x015\x84`@\x01Qa\x17\x07\x90a>fV[`\x01`\x01`\x80\x1B\x03\x16\x90a+\x91V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x83`@\x01\x81\x81Qa\x173\x91\x90a>\x8CV[`\x0F\x0B\x90RP\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x17g` \x8B\x01\x8Ba3\xF1V[\x88a\x17u`@\x8D\x01\x8Da;{V[\x88\x81\x81\x10a\x17\x85Wa\x17\x85a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x17\x9A\x91\x90a3\xF1V[a\x17\xAC\x87` \x01Q\x88`@\x01Qa'\xDDV[\x87``\x01Q`@Qa\x17\xC2\x95\x94\x93\x92\x91\x90a>\xB9V[`@Q\x80\x91\x03\x90\xA1P[a\x18\x15a\x17\xDC` \x8A\x01\x8Aa3\xF1V[a\x17\xE9`@\x8B\x01\x8Ba;{V[\x86\x81\x81\x10a\x17\xF9Wa\x17\xF9a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x18\x0E\x91\x90a3\xF1V[\x87\x85a'\xF2V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x18C` \x8A\x01\x8Aa3\xF1V[\x87a\x18Q`@\x8C\x01\x8Ca;{V[\x87\x81\x81\x10a\x18aWa\x18aa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x18v\x91\x90a3\xF1V[\x85` \x01QB`@Qa\x18\x8D\x95\x94\x93\x92\x91\x90a>\xB9V[`@Q\x80\x91\x03\x90\xA1`\0a\x18\xEC`\x97\x82a\x18\xAA` \x8D\x01\x8Da3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8B\x80`@\x01\x90a\x18\xDC\x91\x90a;{V[\x88\x81\x81\x10a\x12 Wa\x12 a:\xEAV[\x90P`\0a\x18\xFA\x83\x83a>GV[\x90Pa\x19\x8FB\x82`\x97`\0\x8E`\0\x01` \x81\x01\x90a\x19\x18\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0\x8E\x80`@\x01\x90a\x19J\x91\x90a;{V[\x8B\x81\x81\x10a\x19ZWa\x19Za:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x19o\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x91\x90a+\xA8V[P\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90Pa\x19\xC0` \x8C\x01\x8Ca3\xF1V[a\x19\xCD`@\x8D\x01\x8Da;{V[\x88\x81\x81\x10a\x19\xDDWa\x19\xDDa:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x19\xF2\x91\x90a3\xF1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R`\x01`\x01`@\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5z\xB1\x0Ba\x1Aa` \x8D\x01\x8Da3\xF1V[a\x1An`@\x8E\x01\x8Ea;{V[\x89\x81\x81\x10a\x1A~Wa\x1A~a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x93\x91\x90a3\xF1V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x83\x01R\x84\x16`d\x82\x01R`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xF2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x06W=`\0\x80>=`\0\xFD[Pa\x1B!\x92PPP`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x84\x16a+\xC3V[\x86\x86\x81Q\x81\x10a\x1B3Wa\x1B3a:\xEAV[` \x02` \x01\x01\x81\x81RPPPPPP\x80`\x01\x01\x90Pa\x15\xF0V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x1B}` \x87\x01\x87a3\xF1V[\x84a\x1B\x8B`@\x89\x01\x89a;{V[\x85a\x1B\x99`\x80\x8C\x01\x8Ca>\xEFV[`@Qa\x1B\xAC\x97\x96\x95\x94\x93\x92\x91\x90a?uV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\0\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x9A\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1C\xE8W`\x01`\x01`\xA0\x1B\x03\x80\x87\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1CO\x90\x83a+\xD8V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PB\x10\x15a\x1C\xCBWPPa\x1C\xE8V[a\x1C\xD9\x85\x82` \x01Qa'\xDDV[\x94PPP\x80`\x01\x01\x90Pa\x1C\x16V[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1D\x19\x90a&mV[a\x1D#\x91\x90a>GV[\x92PPP[\x92\x91PPV[a\x1D6a,IV[a\x1D@`\0a)\x91V[V[```\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D^Wa\x1D^a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x87W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x84\x81\x10\x15a\x1E,W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\x97` R`@\x81 a\x1D\xFA\x91\x86\x91\x90\x89\x89\x86\x81\x81\x10a\x1D\xC6Wa\x1D\xC6a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1D\xDB\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x90a,\xA3V[\x82\x82\x81Q\x81\x10a\x1E\x0CWa\x1E\x0Ca:\xEAV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1D\x8DV[P\x95\x94PPPPPV[``\x80B\x83c\xFF\xFF\xFF\xFF\x16\x10\x15a\x1E`W`@Qc\xB7\xD0\x94\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1Eta\x06\x0C6\x8B\x90\x03\x8B\x01\x8Ba;\0V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x8A\x8A\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xCA\x94\x93\x92\x91\x90a@\tV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xE7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x0F\x91\x90\x81\x01\x90a@lV[\x90P`\0\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F+Wa\x1F+a4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F^W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FIW\x90P[P\x90P`\0[\x89\x81\x10\x15a!\x94W`\0\x8B\x8B\x83\x81\x81\x10a\x1F\x80Wa\x1F\x80a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x1F\x95\x91\x90a3\xF1V[\x90P\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xAFWa\x1F\xAFa4\xA9V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xD8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x1F\xEBWa\x1F\xEBa:\xEAV[` \x02` \x01\x01\x81\x90RP`\0[\x89\x81\x10\x15a!\x8AW`\0\x8B\x8B\x83\x81\x81\x10a \x15Wa \x15a:\xEAV[\x90P` \x02\x01` \x81\x01\x90a *\x91\x90a3\xF1V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x8B\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\x01`@\x1B\x82\x04`\x0F\x0B\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x93\x82\x01\x84\x90R\x93\x94P\x92\x90\x91\x90\x8D\x16\x10a \xB6Wa \xB3\x81\x83` \x01Qa'\xDDV[\x90P[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a!E\x90a \xE8\x90a&mV[`\x01`\x01`@\x1B\x03\x16a!?\x83`\x01`\x01`@\x1B\x03\x16\x8B\x8A\x81Q\x81\x10a!\x10Wa!\x10a:\xEAV[` \x02` \x01\x01Q\x88\x81Q\x81\x10a!)Wa!)a:\xEAV[` \x02` \x01\x01Qa,\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a+\xC3V[\x87\x87\x81Q\x81\x10a!WWa!Wa:\xEAV[` \x02` \x01\x01Q\x85\x81Q\x81\x10a!pWa!pa:\xEAV[` \x02` \x01\x01\x81\x81RPPPPP\x80`\x01\x01\x90Pa\x1F\xF9V[PP`\x01\x01a\x1FdV[P\x90\x9A\x90\x99P\x97PPPPPPPPV[``\x80a!\xB6\x87\x87\x87\x87\x87Ba\x1E6V[\x91P\x91P\x95P\x95\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x94\x83\x01\x94\x90\x94Re\x01\0\0\0\0\0\x81\x04\x84\x16\x92\x82\x01\x92\x90\x92R`\x01`H\x1B\x90\x91\x04\x90\x91\x16``\x82\x01\x81\x90R\x82\x91\x90\x15\x80\x15\x90a\"EWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\"VW\x80`@\x01Q\x91Pa\"[V[\x80Q\x91P[` \x81\x01Q\x15\x15`\x01\x14\x80a\"\x8EWP``\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80\x15\x90a\"\x8EWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x92PP\x91P\x91V[a\"\x9Ea,IV[`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x03W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\r\xE2V[a\x07U\x81a)\x91V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\x83\x91\x90a;\x1CV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a#\xB4W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x19\x81\x19`fT\x19\x16\x14a#\xDDW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x088V[`\0\x81`\0\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a$\\\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1D(\x90aAuV[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x80\x85R`\x99\x84R\x86\x85 \x91\x89\x16\x80\x86R\x91\x84R\x86\x85 \x88\x86R\x84R\x86\x85 \x87Q\x93\x84\x01\x88RT`\x01`\x01`@\x1B\x03\x80\x82\x16\x85R`\x01`@\x1B\x82\x04`\x0F\x0B\x85\x87\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x92\x04\x82\x16\x85\x8A\x01\x90\x81R\x92\x87R`\x98\x86R\x88\x87 \x93\x87R\x92\x90\x94R\x95\x90\x93 T\x94Q\x93\x94\x90\x93\x92\x16\x91\x16B\x10\x15a%rW`@Q\x80`\x80\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83`\0\x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x92PPPa\x0FsV[a%\x84\x82`\0\x01Q\x83` \x01Qa'\xDDV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x86\x01\x91\x90\x91R\x90\x82\x16\x84R`\0``\x85\x01\x81\x90R`@\x85\x01\x81\x90R\x90\x83\x01Q`\x0F\x0B\x12\x15a%\xD4Wa%\xC8\x81\x83` \x01Qa'\xDDV[`\x01`\x01`@\x1B\x03\x16\x83R[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a&\x04W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[\x80T`\0\x90\x80\x15a&\xABWa&\x95\x83a&\x87`\x01\x84aA\x99V[`\0\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0FsV[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[`\0\x81\x11\x80\x15a'\x07WP\x82a\xFF\xFF\x16\x82\x10[\x15a'\xBEW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a'<\x90a-\tV[\x90P`\0a'K\x87\x87\x84a$tV[\x90P\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a'fWPPa'\xBEV[a'r\x87\x87\x84\x84a'\xF2V[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16`\0\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R a'\xA0\x90a-]V[Pa'\xAA\x84aA\xACV[\x93Pa'\xB5\x83aA\xC5V[\x92PPPa&\xF4V[PPPPPV[`\0a\x0Fs`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aA\xDCV[`\0a\x0Fs\x82`\x01`\x01`@\x1B\x03\x85\x16a>\x8CV[`@\x80Q``\x80\x82\x01\x83R` \x84\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x84R\x85\x85\x01Q`\x0F\x0B\x82\x85\x01\x90\x81R\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x86\x88\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16`\0\x81\x81R`\x99\x88R\x8A\x81 \x92\x8E\x16\x80\x82R\x92\x88R\x8A\x81 \x8D\x82R\x88R\x8A\x81 \x99Q\x8AT\x96Q\x94Q\x90\x95\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x90\x95\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x97\x16\x95\x88\x16\x95\x90\x95\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x90\x96U\x87Q\x86\x83R`\x98\x85R\x87\x83 \x82\x84R\x85R\x91\x87\x90 \x80T\x92\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x86Q\x86Q\x95\x86R\x92\x85\x01R\x16\x92\x82\x01\x92\x90\x92R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91\x01a\x0E_V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a)1WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a)NW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x12\xBA\x82a%\xDDV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a*`WP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a*zW`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x80\x83\x16`@\x83\x01Ra*\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16BaB\tV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[`\0a\x0Fs\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01a-\xDCV[`\0\x80a+\xB6\x85\x85\x85a.9V[\x91P\x91P[\x93P\x93\x91PPV[`\0a\x0Fs\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a0\x07V[`\0\x80a+\xFBa+\xE7\x84a0\xF1V[\x85Ta+\xF6\x91\x90`\x0F\x0BaB\x1CV[a1_V[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a,.W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B`\0\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1D@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\r\xE2V[\x81T`\0\x90\x81a,\xB5\x85\x85\x83\x85a1\xC8V[\x90P\x80\x15a,\xE2Wa,\xCC\x85a&\x87`\x01\x84aA\x99V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D#V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[`\0a\x0Fs\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a0\x07V[`\0a-$\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a-BW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[`\0a-x\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a-\x96W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B`\0\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`\0\x80a-\xEA\x86\x86\x86a0\x07V[\x90P`\x01\x83`\x02\x81\x11\x15a.\0Wa.\0aBDV[\x14\x80\x15a.\x1DWP`\0\x84\x80a.\x18Wa.\x18aBZV[\x86\x88\t\x11[\x15a.0Wa.-`\x01\x82aB\tV[\x90P[\x95\x94PPPPPV[\x82T`\0\x90\x81\x90\x80\x15a/\x9AW`\0a.W\x87a&\x87`\x01\x85aA\x99V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a.\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\xE2V[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a/)W\x84a.\xF9\x88a&\x87`\x01\x86aA\x99V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua/\x8AV[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU`\0\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa+\xBB\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU`\0\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a+\xBBV[`\0\x80\x80`\0\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80`\0\x03a0AW\x83\x82\x81a07Wa07aBZV[\x04\x92PPPa\x0FsV[\x80\x84\x11a0\x88W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\r\xE2V[`\0\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02`\0\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[`\0`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a1[W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\r\xE2V[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a1\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\r\xE2V[\x91\x90PV[`\0[\x81\x83\x10\x15a2\x1EW`\0a1\xDF\x84\x84a2&V[`\0\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a2\nW\x80\x92Pa2\x18V[a2\x15\x81`\x01aB\tV[\x93P[Pa1\xCBV[P\x93\x92PPPV[`\0a25`\x02\x84\x84\x18aBpV[a\x0Fs\x90\x84\x84\x16aB\tV[`\0`@\x82\x84\x03\x12\x15a2SW`\0\x80\xFD[P\x91\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a2kW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x82W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x11\x98W`\0\x80\xFD[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a2\xB5W`\0\x80\xFD[a2\xBF\x87\x87a2AV[\x94P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xDAW`\0\x80\xFD[a2\xE6\x88\x82\x89\x01a2YV[\x90\x95P\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[a3\x11\x88\x82\x89\x01a2YV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15a3\xD0W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87R` \x91\x82\x01\x91\x87\x01\x90`\0[\x81\x81\x10\x15a3\xB7W\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x84R` \x80\x82\x01Q`\x0F\x0B\x90\x85\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x84\x01R``\x83\x01` \x94\x90\x94\x01\x93\x92P`\x01\x01a3nV[P\x90\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a3JV[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07UW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4\x03W`\0\x80\xFD[\x815a\x0Fs\x81a3\xDCV[`\0` \x82\x84\x03\x12\x15a4 W`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a4:W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a4PW`\0\x80\xFD[a4\\\x85\x82\x86\x01a2YV[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a4}W`\0\x80\xFD[\x835a4\x88\x81a3\xDCV[\x92P` \x84\x015a4\x98\x81a3\xDCV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a4\xE1Wa4\xE1a4\xA9V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x0FWa5\x0Fa4\xA9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a50Wa50a4\xA9V[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07UW`\0\x80\xFD[`\0`@\x82\x84\x03\x12\x15a5^W`\0\x80\xFD[a5fa4\xBFV[\x90P\x815a5s\x81a3\xDCV[\x81R` \x82\x015a5\x83\x81a5:V[` \x82\x01R\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a5\xA3W`\0\x80\xFD[\x835a5\xAE\x81a3\xDCV[\x92P` \x84\x015a5\xBE\x81a3\xDCV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5\xD9W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a5\xEAW`\0\x80\xFD[\x805a5\xFDa5\xF8\x82a5\x17V[a4\xE7V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a6\x1FW`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6KWa68\x89\x85a5LV[\x82R` \x82\x01\x91P`@\x84\x01\x93Pa6&V[\x80\x94PPPPP\x92P\x92P\x92V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6\xB6W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x87R` \x80\x82\x01Q`\x0F\x0B\x90\x88\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R``\x86\x01\x95P` \x91\x90\x91\x01\x90`\x01\x01a6mV[P\x93\x94\x93PPPPV[` \x81R`\0a\x0Fs` \x83\x01\x84a6YV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a6\xEBW`\0\x80\xFD[\x855a6\xF6\x81a3\xDCV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x11W`\0\x80\xFD[a7\x1D\x88\x82\x89\x01a2YV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\x05W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a7OW`\0\x80\xFD[\x825a7Z\x81a3\xDCV[\x91P` \x83\x015a7j\x81a3\xDCV[\x80\x91PP\x92P\x92\x90PV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R`\0\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a7\xD3Wa7\xBD\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a7\x91V[PP\x83\x81\x03` \x85\x01Ra7\xE7\x81\x86a6YV[\x96\x95PPPPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a8\x06W`\0\x80\xFD[\x835a8\x11\x81a3\xDCV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8,W`\0\x80\xFD[a88\x86\x82\x87\x01a2YV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a8\x86W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a8_V[P\x90\x95\x94PPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a8\xA4W`\0\x80\xFD[\x825a8\xAF\x81a3\xDCV[\x91P` \x83\x015a7j\x81a5:V[`\0` \x82\x84\x03\x12\x15a8\xD1W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a8\xF4W`\0\x80\xFD[\x815a\x0Fs\x81a5:V[`\0` \x82\x84\x03\x12\x15a9\x11W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a9'W`\0\x80\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x0FsW`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a9OW`\0\x80\xFD[\x845a9Z\x81a3\xDCV[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9uW`\0\x80\xFD[a9\x81\x87\x82\x88\x01a2YV[\x90\x94P\x92PP`@\x85\x015a9\x95\x81a5:V[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\xA0\x87\x89\x03\x12\x15a9\xB9W`\0\x80\xFD[a9\xC3\x88\x88a2AV[\x95P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xDEW`\0\x80\xFD[a9\xEA\x89\x82\x8A\x01a2YV[\x90\x96P\x94PP``\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a:\tW`\0\x80\xFD[a:\x15\x89\x82\x8A\x01a2YV[\x90\x94P\x92PP`\x80\x87\x015a:)\x81a5:V[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15a6\xB6W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a:KV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15a:\xB9W`\x1F\x19\x85\x84\x03\x01\x88Ra:\xA3\x83\x83Qa:7V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a:\x87V[P\x90\x96\x95PPPPPPV[`@\x81R`\0a:\xD8`@\x83\x01\x85a:iV[\x82\x81\x03` \x84\x01Ra.0\x81\x85a:iV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`@\x82\x84\x03\x12\x15a;\x12W`\0\x80\xFD[a\x0Fs\x83\x83a5LV[`\0` \x82\x84\x03\x12\x15a;.W`\0\x80\xFD[\x81Qa\x0Fs\x81a3\xDCV[`\0` \x82\x84\x03\x12\x15a;KW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x0FsW`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a;qW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;\x92W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xACW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a;\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a;\xF5W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[\x805a<\x18\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a<1\x81a5:V[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[` \x80\x82R\x81\x01\x82\x90R`\0\x83`@\x83\x01\x82[\x85\x81\x10\x15a\x1E,Wa<g\x82\x84a<\rV[`@\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a<UV[`\0` \x82\x84\x03\x12\x15a<\x8CW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a<\xF2` \x83\x01\x87a<\rV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a=8W`\0\x80\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0FsW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a=\\W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a=rW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a=\x83W`\0\x80\xFD[\x80Qa=\x91a5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a=\xB3W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a7\xE7W`@\x84\x88\x03\x12\x15a=\xD2W`\0\x80\xFD[a=\xDAa4\xBFV[\x84Qa=\xE5\x81a3\xDCV[\x81R` \x85\x01Qa=\xF5\x81a5:V[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa=\xBAV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0Fs` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a>\x83Wa>\x83a<\xA3V[`\0\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x1D(Wa\x1D(a<\xA3V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a<\xF2` \x83\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\x06W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a? W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x11\x98W`\0\x80\xFD[\x81\x83R` \x83\x01\x92P`\0\x81`\0[\x84\x81\x10\x15a6\xB6W\x815a?W\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a?DV[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81Ra?\xA7` \x82\x01\x88\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\xC0``\x82\x01R`\0a?\xBE`\xC0\x83\x01\x87\x89a?5V[\x82\x81\x03`\x80\x84\x01Ra?\xD0\x81\x87a:7V[\x90P\x82\x81\x03`\xA0\x84\x01R\x83\x81R\x83\x85` \x83\x017`\0` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x98\x97PPPPPPPPV[`@\x80\x82R\x81\x01\x84\x90R`\0\x85``\x83\x01\x82[\x87\x81\x10\x15a@LW\x825a@/\x81a3\xDCV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a@\x1CV[P\x83\x81\x03` \x85\x01Ra@`\x81\x86\x88a?5V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a@~W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@\x94W`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13a@\xA5W`\0\x80\xFD[\x80Qa@\xB3a5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a@\xD5W`\0\x80\xFD[` \x84\x01[\x83\x81\x10\x15a\x06\x99W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a@\xF8W`\0\x80\xFD[\x85\x01`?\x81\x01\x89\x13aA\tW`\0\x80\xFD[` \x81\x01QaA\x1Aa5\xF8\x82a5\x17V[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aA>W`\0\x80\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aA`W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aAEV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa@\xDAV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a2SW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[\x81\x81\x03\x81\x81\x11\x15a\x1D(Wa\x1D(a<\xA3V[`\0`\x01\x82\x01aA\xBEWaA\xBEa<\xA3V[P`\x01\x01\x90V[`\0\x81aA\xD4WaA\xD4a<\xA3V[P`\0\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x1D(Wa\x1D(a<\xA3V[\x80\x82\x01\x80\x82\x11\x15a\x1D(Wa\x1D(a<\xA3V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aB<WaB<a<\xA3V[PP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aB\x8DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xAEKB\xD4\xF7_\xF0\xAE:W\xBA\x19\xD6E\x04\x80\x87\x8D\x0F\xA8s\"\x16\x01\xAE\xFAcB\x19\x1D@\xE6dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct OperatorSet { address avs; uint32 operatorSetId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
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
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                (value.avs, value.operatorSetId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    operatorSetId: tuple.1,
                }
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
                    "OperatorSet(address avs,uint32 operatorSetId)",
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
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
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
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
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
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
    /**Custom error with signature `AlreadySlashedForTimestamp()` and selector `0xab458923`.
```solidity
error AlreadySlashedForTimestamp();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadySlashedForTimestamp {}
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
        impl ::core::convert::From<AlreadySlashedForTimestamp>
        for UnderlyingRustTuple<'_> {
            fn from(value: AlreadySlashedForTimestamp) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AlreadySlashedForTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadySlashedForTimestamp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadySlashedForTimestamp()";
            const SELECTOR: [u8; 4] = [171u8, 69u8, 137u8, 35u8];
            #[inline]
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
    /**Custom error with signature `Empty()` and selector `0x3db2a12a`.
```solidity
error Empty();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Empty {}
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
        impl ::core::convert::From<Empty> for UnderlyingRustTuple<'_> {
            fn from(value: Empty) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Empty {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for Empty {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "Empty()";
            const SELECTOR: [u8; 4] = [61u8, 178u8, 161u8, 42u8];
            #[inline]
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
    /**Custom error with signature `InsufficientAllocatableMagnitude()` and selector `0xca75394c`.
```solidity
error InsufficientAllocatableMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientAllocatableMagnitude {}
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
        impl ::core::convert::From<InsufficientAllocatableMagnitude>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAllocatableMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientAllocatableMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAllocatableMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientAllocatableMagnitude()";
            const SELECTOR: [u8; 4] = [202u8, 117u8, 57u8, 76u8];
            #[inline]
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
    /**Custom error with signature `InvalidExpectedMaxMagnitude()` and selector `0x982f66b3`.
```solidity
error InvalidExpectedMaxMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidExpectedMaxMagnitude {}
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
        impl ::core::convert::From<InvalidExpectedMaxMagnitude>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidExpectedMaxMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidExpectedMaxMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidExpectedMaxMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidExpectedMaxMagnitude()";
            const SELECTOR: [u8; 4] = [152u8, 47u8, 102u8, 179u8];
            #[inline]
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
    /**Custom error with signature `InvalidOperator()` and selector `0xccea9e6f`.
```solidity
error InvalidOperator();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperator {}
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
        impl ::core::convert::From<InvalidOperator> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperator) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperator {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperator()";
            const SELECTOR: [u8; 4] = [204u8, 234u8, 158u8, 111u8];
            #[inline]
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
    /**Custom error with signature `InvalidOperatorSet()` and selector `0x7ec5c154`.
```solidity
error InvalidOperatorSet();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidOperatorSet {}
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
        impl ::core::convert::From<InvalidOperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidOperatorSet()";
            const SELECTOR: [u8; 4] = [126u8, 197u8, 193u8, 84u8];
            #[inline]
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
    /**Custom error with signature `InvalidTimestamp()` and selector `0xb7d09497`.
```solidity
error InvalidTimestamp();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidTimestamp {}
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
        impl ::core::convert::From<InvalidTimestamp> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidTimestamp) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidTimestamp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidTimestamp()";
            const SELECTOR: [u8; 4] = [183u8, 208u8, 148u8, 151u8];
            #[inline]
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
    /**Custom error with signature `InvalidWadToSlash()` and selector `0x13536031`.
```solidity
error InvalidWadToSlash();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidWadToSlash {}
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
        impl ::core::convert::From<InvalidWadToSlash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidWadToSlash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidWadToSlash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidWadToSlash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidWadToSlash()";
            const SELECTOR: [u8; 4] = [19u8, 83u8, 96u8, 49u8];
            #[inline]
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
    /**Custom error with signature `ModificationAlreadyPending()` and selector `0xd8fcbe30`.
```solidity
error ModificationAlreadyPending();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ModificationAlreadyPending {}
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
        impl ::core::convert::From<ModificationAlreadyPending>
        for UnderlyingRustTuple<'_> {
            fn from(value: ModificationAlreadyPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ModificationAlreadyPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ModificationAlreadyPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ModificationAlreadyPending()";
            const SELECTOR: [u8; 4] = [216u8, 252u8, 190u8, 48u8];
            #[inline]
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
    /**Custom error with signature `OnlyDelegationManager()` and selector `0xf739589b`.
```solidity
error OnlyDelegationManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyDelegationManager {}
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
        impl ::core::convert::From<OnlyDelegationManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyDelegationManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyDelegationManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyDelegationManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyDelegationManager()";
            const SELECTOR: [u8; 4] = [247u8, 57u8, 88u8, 155u8];
            #[inline]
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
    /**Custom error with signature `OperatorNotAllocated()` and selector `0x4e99e6cf`.
```solidity
error OperatorNotAllocated();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotAllocated {}
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
        impl ::core::convert::From<OperatorNotAllocated> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotAllocated) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotAllocated {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotAllocated {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotAllocated()";
            const SELECTOR: [u8; 4] = [78u8, 153u8, 230u8, 207u8];
            #[inline]
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
    /**Custom error with signature `OutOfBounds()` and selector `0xb4120f14`.
```solidity
error OutOfBounds();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OutOfBounds {}
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
        impl ::core::convert::From<OutOfBounds> for UnderlyingRustTuple<'_> {
            fn from(value: OutOfBounds) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OutOfBounds {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OutOfBounds {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OutOfBounds()";
            const SELECTOR: [u8; 4] = [180u8, 18u8, 15u8, 20u8];
            #[inline]
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
    /**Custom error with signature `SameMagnitude()` and selector `0x8c0c2f26`.
```solidity
error SameMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SameMagnitude {}
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
        impl ::core::convert::From<SameMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: SameMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SameMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SameMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SameMagnitude()";
            const SELECTOR: [u8; 4] = [140u8, 12u8, 47u8, 38u8];
            #[inline]
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
    /**Custom error with signature `UninitializedAllocationDelay()` and selector `0xfa55fc81`.
```solidity
error UninitializedAllocationDelay();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UninitializedAllocationDelay {}
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
        impl ::core::convert::From<UninitializedAllocationDelay>
        for UnderlyingRustTuple<'_> {
            fn from(value: UninitializedAllocationDelay) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UninitializedAllocationDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UninitializedAllocationDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UninitializedAllocationDelay()";
            const SELECTOR: [u8; 4] = [250u8, 85u8, 252u8, 129u8];
            #[inline]
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
    /**Event with signature `AllocationDelaySet(address,uint32,uint32)` and selector `0x4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db`.
```solidity
event AllocationDelaySet(address operator, uint32 delay, uint32 effectTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AllocationDelaySet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delay: u32,
        #[allow(missing_docs)]
        pub effectTimestamp: u32,
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
        impl alloy_sol_types::SolEvent for AllocationDelaySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AllocationDelaySet(address,uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                78u8,
                133u8,
                117u8,
                29u8,
                99u8,
                49u8,
                80u8,
                108u8,
                108u8,
                98u8,
                51u8,
                95u8,
                32u8,
                126u8,
                179u8,
                31u8,
                18u8,
                166u8,
                30u8,
                87u8,
                15u8,
                52u8,
                245u8,
                193u8,
                118u8,
                64u8,
                48u8,
                135u8,
                133u8,
                198u8,
                212u8,
                219u8,
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
                    delay: data.1,
                    effectTimestamp: data.2,
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectTimestamp),
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
        impl alloy_sol_types::private::IntoLogData for AllocationDelaySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AllocationDelaySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AllocationDelaySet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `EncumberedMagnitudeUpdated(address,address,uint64)` and selector `0xacf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55`.
```solidity
event EncumberedMagnitudeUpdated(address operator, address strategy, uint64 encumberedMagnitude);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EncumberedMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub encumberedMagnitude: u64,
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
        impl alloy_sol_types::SolEvent for EncumberedMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EncumberedMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                172u8,
                249u8,
                9u8,
                95u8,
                235u8,
                58u8,
                55u8,
                12u8,
                156u8,
                246u8,
                146u8,
                66u8,
                28u8,
                105u8,
                239u8,
                50u8,
                13u8,
                77u8,
                181u8,
                198u8,
                110u8,
                106u8,
                125u8,
                41u8,
                199u8,
                105u8,
                78u8,
                176u8,
                35u8,
                100u8,
                252u8,
                85u8,
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
                    encumberedMagnitude: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.encumberedMagnitude),
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
        impl alloy_sol_types::private::IntoLogData for EncumberedMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EncumberedMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &EncumberedMagnitudeUpdated,
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
    /**Event with signature `MaxMagnitudeUpdated(address,address,uint64)` and selector `0x1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c`.
```solidity
event MaxMagnitudeUpdated(address operator, address strategy, uint64 maxMagnitude);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MaxMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub maxMagnitude: u64,
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
        impl alloy_sol_types::SolEvent for MaxMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaxMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                28u8,
                100u8,
                88u8,
                7u8,
                154u8,
                65u8,
                7u8,
                125u8,
                0u8,
                60u8,
                17u8,
                250u8,
                249u8,
                191u8,
                9u8,
                126u8,
                105u8,
                59u8,
                214u8,
                121u8,
                121u8,
                228u8,
                230u8,
                80u8,
                11u8,
                172u8,
                123u8,
                41u8,
                219u8,
                119u8,
                155u8,
                92u8,
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
                    maxMagnitude: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.maxMagnitude),
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
        impl alloy_sol_types::private::IntoLogData for MaxMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MaxMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &MaxMagnitudeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetMagnitudeUpdated(address,(address,uint32),address,uint64,uint32)` and selector `0x8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf6`.
```solidity
event OperatorSetMagnitudeUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectTimestamp);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetMagnitudeUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub magnitude: u64,
        #[allow(missing_docs)]
        pub effectTimestamp: u32,
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
        impl alloy_sol_types::SolEvent for OperatorSetMagnitudeUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetMagnitudeUpdated(address,(address,uint32),address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                153u8,
                126u8,
                83u8,
                215u8,
                185u8,
                229u8,
                217u8,
                35u8,
                208u8,
                162u8,
                28u8,
                96u8,
                223u8,
                129u8,
                225u8,
                116u8,
                8u8,
                96u8,
                209u8,
                168u8,
                198u8,
                107u8,
                140u8,
                99u8,
                197u8,
                4u8,
                122u8,
                226u8,
                14u8,
                170u8,
                246u8,
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
                    operatorSet: data.1,
                    strategy: data.2,
                    magnitude: data.3,
                    effectTimestamp: data.4,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.magnitude),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectTimestamp),
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetMagnitudeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetMagnitudeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorSetMagnitudeUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSlashed(address,(address,uint32),address[],uint256[],string)` and selector `0x80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5`.
```solidity
event OperatorSlashed(address operator, OperatorSet operatorSet, address[] strategies, uint256[] wadSlashed, string description);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSlashed {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        #[allow(missing_docs)]
        pub wadSlashed: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSlashed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSlashed(address,(address,uint32),address[],uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                128u8,
                150u8,
                154u8,
                210u8,
                148u8,
                40u8,
                214u8,
                121u8,
                126u8,
                231u8,
                170u8,
                208u8,
                132u8,
                249u8,
                228u8,
                164u8,
                42u8,
                130u8,
                252u8,
                80u8,
                109u8,
                205u8,
                44u8,
                163u8,
                182u8,
                251u8,
                67u8,
                31u8,
                133u8,
                204u8,
                235u8,
                229u8,
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
                    operatorSet: data.1,
                    strategies: data.2,
                    wadSlashed: data.3,
                    description: data.4,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadSlashed),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.description,
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
        impl alloy_sol_types::private::IntoLogData for OperatorSlashed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSlashed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSlashed) -> alloy_sol_types::private::LogData {
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
constructor(address _delegation, address _avsDirectory, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegation: alloy::sol_types::private::Address,
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _DEALLOCATION_DELAY: u32,
        pub _ALLOCATION_CONFIGURATION_DELAY: u32,
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
                        value._delegation,
                        value._avsDirectory,
                        value._DEALLOCATION_DELAY,
                        value._ALLOCATION_CONFIGURATION_DELAY,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegation: tuple.0,
                        _avsDirectory: tuple.1,
                        _DEALLOCATION_DELAY: tuple.2,
                        _ALLOCATION_CONFIGURATION_DELAY: tuple.3,
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
                        &self._delegation,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._DEALLOCATION_DELAY),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._ALLOCATION_CONFIGURATION_DELAY,
                    ),
                )
            }
        }
    };
    /**Function with signature `ALLOCATION_CONFIGURATION_DELAY()` and selector `0x7bc1ef61`.
```solidity
function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ALLOCATION_CONFIGURATION_DELAYCall {}
    ///Container type for the return parameters of the [`ALLOCATION_CONFIGURATION_DELAY()`](ALLOCATION_CONFIGURATION_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ALLOCATION_CONFIGURATION_DELAYReturn {
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
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ALLOCATION_CONFIGURATION_DELAYCall {
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
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for ALLOCATION_CONFIGURATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ALLOCATION_CONFIGURATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ALLOCATION_CONFIGURATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ALLOCATION_CONFIGURATION_DELAY()";
            const SELECTOR: [u8; 4] = [123u8, 193u8, 239u8, 97u8];
            #[inline]
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
    /**Function with signature `DEALLOCATION_DELAY()` and selector `0x2981eb77`.
```solidity
function DEALLOCATION_DELAY() external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEALLOCATION_DELAYCall {}
    ///Container type for the return parameters of the [`DEALLOCATION_DELAY()`](DEALLOCATION_DELAYCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DEALLOCATION_DELAYReturn {
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
            impl ::core::convert::From<DEALLOCATION_DELAYCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEALLOCATION_DELAYCall {
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
            impl ::core::convert::From<DEALLOCATION_DELAYReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for DEALLOCATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEALLOCATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEALLOCATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DEALLOCATION_DELAY()";
            const SELECTOR: [u8; 4] = [41u8, 129u8, 235u8, 119u8];
            #[inline]
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
    /**Function with signature `clearDeallocationQueue(address,address[],uint16[])` and selector `0x4b5046ef`.
```solidity
function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearDeallocationQueueCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub numToClear: alloy::sol_types::private::Vec<u16>,
    }
    ///Container type for the return parameters of the [`clearDeallocationQueue(address,address[],uint16[])`](clearDeallocationQueueCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearDeallocationQueueReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<u16>,
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
            impl ::core::convert::From<clearDeallocationQueueCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueCall) -> Self {
                    (value.operator, value.strategies, value.numToClear)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearDeallocationQueueCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        numToClear: tuple.2,
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
            impl ::core::convert::From<clearDeallocationQueueReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for clearDeallocationQueueReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearDeallocationQueueCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<16>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearDeallocationQueueReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clearDeallocationQueue(address,address[],uint16[])";
            const SELECTOR: [u8; 4] = [75u8, 80u8, 70u8, 239u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<16>,
                    > as alloy_sol_types::SolType>::tokenize(&self.numToClear),
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
    /**Function with signature `delegation()` and selector `0xdf5cf723`.
```solidity
function delegation() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationCall {}
    ///Container type for the return parameters of the [`delegation()`](delegationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delegationReturn {
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
            impl ::core::convert::From<delegationCall> for UnderlyingRustTuple<'_> {
                fn from(value: delegationCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationCall {
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
            impl ::core::convert::From<delegationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: delegationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for delegationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delegationCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delegation()";
            const SELECTOR: [u8; 4] = [223u8, 92u8, 247u8, 35u8];
            #[inline]
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
    /**Function with signature `encumberedMagnitude(address,address)` and selector `0xa984eb3a`.
```solidity
function encumberedMagnitude(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encumberedMagnitudeCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`encumberedMagnitude(address,address)`](encumberedMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encumberedMagnitudeReturn {
        pub _0: u64,
    }
    #[allow(
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
            impl ::core::convert::From<encumberedMagnitudeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: encumberedMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encumberedMagnitudeCall {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<encumberedMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: encumberedMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encumberedMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for encumberedMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = encumberedMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "encumberedMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [169u8, 132u8, 235u8, 58u8];
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
    /**Function with signature `getAllocatableMagnitude(address,address)` and selector `0x6cfb4481`.
```solidity
function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatableMagnitudeCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatableMagnitude(address,address)`](getAllocatableMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatableMagnitudeReturn {
        pub _0: u64,
    }
    #[allow(
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
            impl ::core::convert::From<getAllocatableMagnitudeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatableMagnitudeCall {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocatableMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatableMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatableMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatableMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatableMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [108u8, 251u8, 68u8, 129u8];
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
    /**Function with signature `getAllocationDelay(address)` and selector `0xb9fbaed1`.
```solidity
function getAllocationDelay(address operator) external view returns (bool isSet, uint32 delay);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationDelayCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocationDelay(address)`](getAllocationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationDelayReturn {
        pub isSet: bool,
        pub delay: u32,
    }
    #[allow(
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
            impl ::core::convert::From<getAllocationDelayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationDelayCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayReturn) -> Self {
                    (value.isSet, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        isSet: tuple.0,
                        delay: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationDelayCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationDelayReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocationDelay(address)";
            const SELECTOR: [u8; 4] = [185u8, 251u8, 174u8, 209u8];
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
    /**Function with signature `getAllocationInfo((address,uint32),address[],address[])` and selector `0x0b002119`.
```solidity
function getAllocationInfo(OperatorSet memory operatorSet, address[] memory strategies, address[] memory operators) external view returns (IAllocationManagerTypes.MagnitudeInfo[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_0Call {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getAllocationInfo((address,uint32),address[],address[])`](getAllocationInfo_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_0Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_0Call) -> Self {
                    (value.operatorSet, value.strategies, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        strategies: tuple.1,
                        operators: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        IAllocationManagerTypes::MagnitudeInfo,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationInfo_0Call {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        IAllocationManagerTypes::MagnitudeInfo,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocationInfo((address,uint32),address[],address[])";
            const SELECTOR: [u8; 4] = [11u8, 0u8, 33u8, 25u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
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
    /**Function with signature `getAllocationInfo(address,address,(address,uint32)[])` and selector `0x35af054a`.
```solidity
function getAllocationInfo(address operator, address strategy, OperatorSet[] memory operatorSets) external view returns (IAllocationManagerTypes.MagnitudeInfo[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_1Call {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
        pub operatorSets: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`getAllocationInfo(address,address,(address,uint32)[])`](getAllocationInfo_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_1Return {
        pub _0: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_1Call) -> Self {
                    (value.operator, value.strategy, value.operatorSets)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategy: tuple.1,
                        operatorSets: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::MagnitudeInfo,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationInfo_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::MagnitudeInfo,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocationInfo(address,address,(address,uint32)[])";
            const SELECTOR: [u8; 4] = [53u8, 175u8, 5u8, 74u8];
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
                    <alloy::sol_types::sol_data::Array<
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSets),
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
    /**Function with signature `getAllocationInfo(address,address)` and selector `0x4d9dbde9`.
```solidity
function getAllocationInfo(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.MagnitudeInfo[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_2Call {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocationInfo(address,address)`](getAllocationInfo_2Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationInfo_2Return {
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
        pub _1: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_2Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_2Call) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_2Call {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_2Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_2Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationInfo_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationInfo_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocationInfo(address,address)";
            const SELECTOR: [u8; 4] = [77u8, 157u8, 189u8, 233u8];
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
    /**Function with signature `getCurrentDelegatedAndSlashableOperatorShares((address,uint32),address[],address[])` and selector `0x94bd62a5`.
```solidity
function getCurrentDelegatedAndSlashableOperatorShares(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies) external view returns (uint256[][] memory, uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDelegatedAndSlashableOperatorSharesCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getCurrentDelegatedAndSlashableOperatorShares((address,uint32),address[],address[])`](getCurrentDelegatedAndSlashableOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentDelegatedAndSlashableOperatorSharesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        >,
        pub _1: alloy::sol_types::private::Vec<
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
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCurrentDelegatedAndSlashableOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: getCurrentDelegatedAndSlashableOperatorSharesCall,
                ) -> Self {
                    (value.operatorSet, value.operators, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentDelegatedAndSlashableOperatorSharesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operators: tuple.1,
                        strategies: tuple.2,
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
            impl ::core::convert::From<
                getCurrentDelegatedAndSlashableOperatorSharesReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: getCurrentDelegatedAndSlashableOperatorSharesReturn,
                ) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentDelegatedAndSlashableOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for getCurrentDelegatedAndSlashableOperatorSharesCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentDelegatedAndSlashableOperatorSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentDelegatedAndSlashableOperatorShares((address,uint32),address[],address[])";
            const SELECTOR: [u8; 4] = [148u8, 189u8, 98u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
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
    /**Function with signature `getMaxMagnitudes(address,address[])` and selector `0x547afb87`.
```solidity
function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address,address[])`](getMaxMagnitudesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesReturn {
        pub _0: alloy::sol_types::private::Vec<u64>,
    }
    #[allow(
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
            impl ::core::convert::From<getMaxMagnitudesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesCall {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u64>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudes(address,address[])";
            const SELECTOR: [u8; 4] = [84u8, 122u8, 251u8, 135u8];
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
    /**Function with signature `getMaxMagnitudesAtTimestamp(address,address[],uint32)` and selector `0x843b349f`.
```solidity
function getMaxMagnitudesAtTimestamp(address operator, address[] memory strategies, uint32 timestamp) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtTimestampCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub timestamp: u32,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudesAtTimestamp(address,address[],uint32)`](getMaxMagnitudesAtTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtTimestampReturn {
        pub _0: alloy::sol_types::private::Vec<u64>,
    }
    #[allow(
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
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getMaxMagnitudesAtTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtTimestampCall) -> Self {
                    (value.operator, value.strategies, value.timestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        timestamp: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u64>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesAtTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudesAtTimestampCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesAtTimestampReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudesAtTimestamp(address,address[],uint32)";
            const SELECTOR: [u8; 4] = [132u8, 59u8, 52u8, 159u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestamp),
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
    /**Function with signature `getMinDelegatedAndSlashableOperatorSharesBefore((address,uint32),address[],address[],uint32)` and selector `0x93d7a72b`.
```solidity
function getMinDelegatedAndSlashableOperatorSharesBefore(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 beforeTimestamp) external view returns (uint256[][] memory, uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelegatedAndSlashableOperatorSharesBeforeCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub beforeTimestamp: u32,
    }
    ///Container type for the return parameters of the [`getMinDelegatedAndSlashableOperatorSharesBefore((address,uint32),address[],address[],uint32)`](getMinDelegatedAndSlashableOperatorSharesBeforeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelegatedAndSlashableOperatorSharesBeforeReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        >,
        pub _1: alloy::sol_types::private::Vec<
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
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<
                getMinDelegatedAndSlashableOperatorSharesBeforeCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: getMinDelegatedAndSlashableOperatorSharesBeforeCall,
                ) -> Self {
                    (
                        value.operatorSet,
                        value.operators,
                        value.strategies,
                        value.beforeTimestamp,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinDelegatedAndSlashableOperatorSharesBeforeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operators: tuple.1,
                        strategies: tuple.2,
                        beforeTimestamp: tuple.3,
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
            impl ::core::convert::From<
                getMinDelegatedAndSlashableOperatorSharesBeforeReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: getMinDelegatedAndSlashableOperatorSharesBeforeReturn,
                ) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinDelegatedAndSlashableOperatorSharesBeforeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for getMinDelegatedAndSlashableOperatorSharesBeforeCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinDelegatedAndSlashableOperatorSharesBeforeReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    >,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMinDelegatedAndSlashableOperatorSharesBefore((address,uint32),address[],address[],uint32)";
            const SELECTOR: [u8; 4] = [147u8, 215u8, 167u8, 43u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beforeTimestamp),
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
    /**Function with signature `modifyAllocations((address,uint64,(address,uint32)[],uint64[])[])` and selector `0x1637b60f`.
```solidity
function modifyAllocations(IAllocationManagerTypes.MagnitudeAllocation[] memory allocations) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsCall {
        pub allocations: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::MagnitudeAllocation as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`modifyAllocations((address,uint64,(address,uint32)[],uint64[])[])`](modifyAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsReturn {}
    #[allow(
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
                    IAllocationManagerTypes::MagnitudeAllocation,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::MagnitudeAllocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<modifyAllocationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsCall) -> Self {
                    (value.allocations,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { allocations: tuple.0 }
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
            impl ::core::convert::From<modifyAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::MagnitudeAllocation,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyAllocationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyAllocations((address,uint64,(address,uint32)[],uint64[])[])";
            const SELECTOR: [u8; 4] = [22u8, 55u8, 182u8, 15u8];
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
                        IAllocationManagerTypes::MagnitudeAllocation,
                    > as alloy_sol_types::SolType>::tokenize(&self.allocations),
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
    /**Function with signature `setAllocationDelay(address,uint32)` and selector `0x56c483e6`.
```solidity
function setAllocationDelay(address operator, uint32 delay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelay_0Call {
        pub operator: alloy::sol_types::private::Address,
        pub delay: u32,
    }
    ///Container type for the return parameters of the [`setAllocationDelay(address,uint32)`](setAllocationDelay_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelay_0Return {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelay_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_0Call) -> Self {
                    (value.operator, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelay_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        delay: tuple.1,
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
            impl ::core::convert::From<setAllocationDelay_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelay_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllocationDelay_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelay_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllocationDelay(address,uint32)";
            const SELECTOR: [u8; 4] = [86u8, 196u8, 131u8, 230u8];
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
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
    /**Function with signature `setAllocationDelay(uint32)` and selector `0x5c489bb5`.
```solidity
function setAllocationDelay(uint32 delay) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelay_1Call {
        pub delay: u32,
    }
    ///Container type for the return parameters of the [`setAllocationDelay(uint32)`](setAllocationDelay_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelay_1Return {}
    #[allow(
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
            impl ::core::convert::From<setAllocationDelay_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_1Call) -> Self {
                    (value.delay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelay_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delay: tuple.0 }
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
            impl ::core::convert::From<setAllocationDelay_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelay_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllocationDelay_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelay_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAllocationDelay(uint32)";
            const SELECTOR: [u8; 4] = [92u8, 72u8, 155u8, 181u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.delay),
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
    /**Function with signature `slashOperator((address,uint32,address[],uint256,string))` and selector `0x60db99a3`.
```solidity
function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,address[],uint256,string))`](slashOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::SlashingParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<slashOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { params: tuple.0 }
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
            impl ::core::convert::From<slashOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashOperatorCall {
            type Parameters<'a> = (IAllocationManagerTypes::SlashingParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator((address,uint32,address[],uint256,string))";
            const SELECTOR: [u8; 4] = [96u8, 219u8, 153u8, 163u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::SlashingParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
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
    ///Container for all the [`AllocationManager`](self) function calls.
    pub enum AllocationManagerCalls {
        ALLOCATION_CONFIGURATION_DELAY(ALLOCATION_CONFIGURATION_DELAYCall),
        DEALLOCATION_DELAY(DEALLOCATION_DELAYCall),
        avsDirectory(avsDirectoryCall),
        clearDeallocationQueue(clearDeallocationQueueCall),
        delegation(delegationCall),
        encumberedMagnitude(encumberedMagnitudeCall),
        getAllocatableMagnitude(getAllocatableMagnitudeCall),
        getAllocationDelay(getAllocationDelayCall),
        getAllocationInfo_0(getAllocationInfo_0Call),
        getAllocationInfo_1(getAllocationInfo_1Call),
        getAllocationInfo_2(getAllocationInfo_2Call),
        getCurrentDelegatedAndSlashableOperatorShares(
            getCurrentDelegatedAndSlashableOperatorSharesCall,
        ),
        getMaxMagnitudes(getMaxMagnitudesCall),
        getMaxMagnitudesAtTimestamp(getMaxMagnitudesAtTimestampCall),
        getMinDelegatedAndSlashableOperatorSharesBefore(
            getMinDelegatedAndSlashableOperatorSharesBeforeCall,
        ),
        initialize(initializeCall),
        modifyAllocations(modifyAllocationsCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        renounceOwnership(renounceOwnershipCall),
        setAllocationDelay_0(setAllocationDelay_0Call),
        setAllocationDelay_1(setAllocationDelay_1Call),
        setPauserRegistry(setPauserRegistryCall),
        slashOperator(slashOperatorCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl AllocationManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 0u8, 33u8, 25u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 100u8, 57u8, 221u8],
            [22u8, 55u8, 182u8, 15u8],
            [23u8, 148u8, 187u8, 60u8],
            [41u8, 129u8, 235u8, 119u8],
            [53u8, 175u8, 5u8, 74u8],
            [75u8, 80u8, 70u8, 239u8],
            [77u8, 157u8, 189u8, 233u8],
            [84u8, 122u8, 251u8, 135u8],
            [86u8, 196u8, 131u8, 230u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 72u8, 155u8, 181u8],
            [92u8, 151u8, 90u8, 187u8],
            [96u8, 219u8, 153u8, 163u8],
            [107u8, 58u8, 167u8, 46u8],
            [108u8, 251u8, 68u8, 129u8],
            [113u8, 80u8, 24u8, 166u8],
            [123u8, 193u8, 239u8, 97u8],
            [132u8, 59u8, 52u8, 159u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [147u8, 215u8, 167u8, 43u8],
            [148u8, 189u8, 98u8, 165u8],
            [169u8, 132u8, 235u8, 58u8],
            [185u8, 251u8, 174u8, 209u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerCalls {
        const NAME: &'static str = "AllocationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 30usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ALLOCATION_CONFIGURATION_DELAY(_) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEALLOCATION_DELAY(_) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clearDeallocationQueue(_) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::encumberedMagnitude(_) => {
                    <encumberedMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatableMagnitude(_) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocationDelay(_) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocationInfo_0(_) => {
                    <getAllocationInfo_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocationInfo_1(_) => {
                    <getAllocationInfo_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocationInfo_2(_) => {
                    <getAllocationInfo_2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentDelegatedAndSlashableOperatorShares(_) => {
                    <getCurrentDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudes(_) => {
                    <getMaxMagnitudesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudesAtTimestamp(_) => {
                    <getMaxMagnitudesAtTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMinDelegatedAndSlashableOperatorSharesBefore(_) => {
                    <getMinDelegatedAndSlashableOperatorSharesBeforeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyAllocations(_) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllocationDelay_0(_) => {
                    <setAllocationDelay_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllocationDelay_1(_) => {
                    <setAllocationDelay_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<AllocationManagerCalls>] = &[
                {
                    fn getAllocationInfo_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationInfo_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocationInfo_0)
                    }
                    getAllocationInfo_0
                },
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn modifyAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::modifyAllocations)
                    }
                    modifyAllocations
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn DEALLOCATION_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::DEALLOCATION_DELAY)
                    }
                    DEALLOCATION_DELAY
                },
                {
                    fn getAllocationInfo_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationInfo_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocationInfo_1)
                    }
                    getAllocationInfo_1
                },
                {
                    fn clearDeallocationQueue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::clearDeallocationQueue)
                    }
                    clearDeallocationQueue
                },
                {
                    fn getAllocationInfo_2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationInfo_2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocationInfo_2)
                    }
                    getAllocationInfo_2
                },
                {
                    fn getMaxMagnitudes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitudes)
                    }
                    getMaxMagnitudes
                },
                {
                    fn setAllocationDelay_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAllocationDelay_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::setAllocationDelay_0)
                    }
                    setAllocationDelay_0
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn setAllocationDelay_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAllocationDelay_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::setAllocationDelay_1)
                    }
                    setAllocationDelay_1
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn slashOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <slashOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::slashOperator)
                    }
                    slashOperator
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn getAllocatableMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocatableMagnitude)
                    }
                    getAllocatableMagnitude
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn ALLOCATION_CONFIGURATION_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::ALLOCATION_CONFIGURATION_DELAY)
                    }
                    ALLOCATION_CONFIGURATION_DELAY
                },
                {
                    fn getMaxMagnitudesAtTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudesAtTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getMaxMagnitudesAtTimestamp)
                    }
                    getMaxMagnitudesAtTimestamp
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn getMinDelegatedAndSlashableOperatorSharesBefore(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMinDelegatedAndSlashableOperatorSharesBeforeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerCalls::getMinDelegatedAndSlashableOperatorSharesBefore,
                            )
                    }
                    getMinDelegatedAndSlashableOperatorSharesBefore
                },
                {
                    fn getCurrentDelegatedAndSlashableOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getCurrentDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerCalls::getCurrentDelegatedAndSlashableOperatorShares,
                            )
                    }
                    getCurrentDelegatedAndSlashableOperatorShares
                },
                {
                    fn encumberedMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <encumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::encumberedMagnitude)
                    }
                    encumberedMagnitude
                },
                {
                    fn getAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getAllocationDelay)
                    }
                    getAllocationDelay
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::unpause)
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
                Self::ALLOCATION_CONFIGURATION_DELAY(inner) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clearDeallocationQueue(inner) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::encumberedMagnitude(inner) => {
                    <encumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatableMagnitude(inner) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationDelay(inner) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationInfo_0(inner) => {
                    <getAllocationInfo_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationInfo_1(inner) => {
                    <getAllocationInfo_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationInfo_2(inner) => {
                    <getAllocationInfo_2Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentDelegatedAndSlashableOperatorShares(inner) => {
                    <getCurrentDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudes(inner) => {
                    <getMaxMagnitudesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudesAtTimestamp(inner) => {
                    <getMaxMagnitudesAtTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMinDelegatedAndSlashableOperatorSharesBefore(inner) => {
                    <getMinDelegatedAndSlashableOperatorSharesBeforeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::modifyAllocations(inner) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllocationDelay_0(inner) => {
                    <setAllocationDelay_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAllocationDelay_1(inner) => {
                    <setAllocationDelay_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::ALLOCATION_CONFIGURATION_DELAY(inner) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::clearDeallocationQueue(inner) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::encumberedMagnitude(inner) => {
                    <encumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatableMagnitude(inner) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocationDelay(inner) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocationInfo_0(inner) => {
                    <getAllocationInfo_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocationInfo_1(inner) => {
                    <getAllocationInfo_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocationInfo_2(inner) => {
                    <getAllocationInfo_2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentDelegatedAndSlashableOperatorShares(inner) => {
                    <getCurrentDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudes(inner) => {
                    <getMaxMagnitudesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudesAtTimestamp(inner) => {
                    <getMaxMagnitudesAtTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMinDelegatedAndSlashableOperatorSharesBefore(inner) => {
                    <getMinDelegatedAndSlashableOperatorSharesBeforeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::modifyAllocations(inner) => {
                    <modifyAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllocationDelay_0(inner) => {
                    <setAllocationDelay_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setAllocationDelay_1(inner) => {
                    <setAllocationDelay_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::slashOperator(inner) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`AllocationManager`](self) custom errors.
    pub enum AllocationManagerErrors {
        AlreadySlashedForTimestamp(AlreadySlashedForTimestamp),
        CurrentlyPaused(CurrentlyPaused),
        Empty(Empty),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InsufficientAllocatableMagnitude(InsufficientAllocatableMagnitude),
        InvalidExpectedMaxMagnitude(InvalidExpectedMaxMagnitude),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidOperator(InvalidOperator),
        InvalidOperatorSet(InvalidOperatorSet),
        InvalidSignature(InvalidSignature),
        InvalidTimestamp(InvalidTimestamp),
        InvalidWadToSlash(InvalidWadToSlash),
        ModificationAlreadyPending(ModificationAlreadyPending),
        OnlyDelegationManager(OnlyDelegationManager),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorNotAllocated(OperatorNotAllocated),
        OperatorNotRegistered(OperatorNotRegistered),
        OutOfBounds(OutOfBounds),
        SaltSpent(SaltSpent),
        SameMagnitude(SameMagnitude),
        SignatureExpired(SignatureExpired),
        UninitializedAllocationDelay(UninitializedAllocationDelay),
    }
    #[automatically_derived]
    impl AllocationManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [19u8, 83u8, 96u8, 49u8],
            [37u8, 236u8, 108u8, 31u8],
            [53u8, 49u8, 50u8, 68u8],
            [61u8, 178u8, 161u8, 42u8],
            [67u8, 113u8, 74u8, 253u8],
            [78u8, 153u8, 230u8, 207u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 12u8, 47u8, 38u8],
            [152u8, 47u8, 102u8, 179u8],
            [171u8, 69u8, 137u8, 35u8],
            [180u8, 18u8, 15u8, 20u8],
            [183u8, 208u8, 148u8, 151u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 117u8, 57u8, 76u8],
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 252u8, 190u8, 48u8],
            [247u8, 57u8, 88u8, 155u8],
            [250u8, 85u8, 252u8, 129u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerErrors {
        const NAME: &'static str = "AllocationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadySlashedForTimestamp(_) => {
                    <AlreadySlashedForTimestamp as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::Empty(_) => <Empty as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientAllocatableMagnitude(_) => {
                    <InsufficientAllocatableMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidExpectedMaxMagnitude(_) => {
                    <InvalidExpectedMaxMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperator(_) => {
                    <InvalidOperator as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidOperatorSet(_) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidTimestamp(_) => {
                    <InvalidTimestamp as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWadToSlash(_) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ModificationAlreadyPending(_) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyDelegationManager(_) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotAllocated(_) => {
                    <OperatorNotAllocated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => {
                    <OutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaltSpent(_) => <SaltSpent as alloy_sol_types::SolError>::SELECTOR,
                Self::SameMagnitude(_) => {
                    <SameMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UninitializedAllocationDelay(_) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<AllocationManagerErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn InvalidWadToSlash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidWadToSlash as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidWadToSlash)
                    }
                    InvalidWadToSlash
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn SaltSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::SaltSpent)
                    }
                    SaltSpent
                },
                {
                    fn Empty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <Empty as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::Empty)
                    }
                    Empty
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn OperatorNotAllocated(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OperatorNotAllocated as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OperatorNotAllocated)
                    }
                    OperatorNotAllocated
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn SameMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <SameMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::SameMagnitude)
                    }
                    SameMagnitude
                },
                {
                    fn InvalidExpectedMaxMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidExpectedMaxMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidExpectedMaxMagnitude)
                    }
                    InvalidExpectedMaxMagnitude
                },
                {
                    fn AlreadySlashedForTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <AlreadySlashedForTimestamp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::AlreadySlashedForTimestamp)
                    }
                    AlreadySlashedForTimestamp
                },
                {
                    fn OutOfBounds(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OutOfBounds)
                    }
                    OutOfBounds
                },
                {
                    fn InvalidTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidTimestamp as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidTimestamp)
                    }
                    InvalidTimestamp
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn InsufficientAllocatableMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InsufficientAllocatableMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerErrors::InsufficientAllocatableMagnitude,
                            )
                    }
                    InsufficientAllocatableMagnitude
                },
                {
                    fn InvalidOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidOperator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidOperator)
                    }
                    InvalidOperator
                },
                {
                    fn ModificationAlreadyPending(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::ModificationAlreadyPending)
                    }
                    ModificationAlreadyPending
                },
                {
                    fn OnlyDelegationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyDelegationManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::OnlyDelegationManager)
                    }
                    OnlyDelegationManager
                },
                {
                    fn UninitializedAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::UninitializedAllocationDelay)
                    }
                    UninitializedAllocationDelay
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
                Self::AlreadySlashedForTimestamp(inner) => {
                    <AlreadySlashedForTimestamp as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::Empty(inner) => {
                    <Empty as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InsufficientAllocatableMagnitude(inner) => {
                    <InsufficientAllocatableMagnitude as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidExpectedMaxMagnitude(inner) => {
                    <InvalidExpectedMaxMagnitude as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidTimestamp(inner) => {
                    <InvalidTimestamp as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyDelegationManager(inner) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotAllocated(inner) => {
                    <OperatorNotAllocated as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UninitializedAllocationDelay(inner) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadySlashedForTimestamp(inner) => {
                    <AlreadySlashedForTimestamp as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::Empty(inner) => {
                    <Empty as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InsufficientAllocatableMagnitude(inner) => {
                    <InsufficientAllocatableMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidExpectedMaxMagnitude(inner) => {
                    <InvalidExpectedMaxMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidTimestamp(inner) => {
                    <InvalidTimestamp as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyDelegationManager(inner) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorNotAllocated(inner) => {
                    <OperatorNotAllocated as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UninitializedAllocationDelay(inner) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AllocationManager`](self) events.
    pub enum AllocationManagerEvents {
        AllocationDelaySet(AllocationDelaySet),
        EncumberedMagnitudeUpdated(EncumberedMagnitudeUpdated),
        Initialized(Initialized),
        MaxMagnitudeUpdated(MaxMagnitudeUpdated),
        OperatorSetMagnitudeUpdated(OperatorSetMagnitudeUpdated),
        OperatorSlashed(OperatorSlashed),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl AllocationManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                28u8,
                100u8,
                88u8,
                7u8,
                154u8,
                65u8,
                7u8,
                125u8,
                0u8,
                60u8,
                17u8,
                250u8,
                249u8,
                191u8,
                9u8,
                126u8,
                105u8,
                59u8,
                214u8,
                121u8,
                121u8,
                228u8,
                230u8,
                80u8,
                11u8,
                172u8,
                123u8,
                41u8,
                219u8,
                119u8,
                155u8,
                92u8,
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
                78u8,
                133u8,
                117u8,
                29u8,
                99u8,
                49u8,
                80u8,
                108u8,
                108u8,
                98u8,
                51u8,
                95u8,
                32u8,
                126u8,
                179u8,
                31u8,
                18u8,
                166u8,
                30u8,
                87u8,
                15u8,
                52u8,
                245u8,
                193u8,
                118u8,
                64u8,
                48u8,
                135u8,
                133u8,
                198u8,
                212u8,
                219u8,
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
                128u8,
                150u8,
                154u8,
                210u8,
                148u8,
                40u8,
                214u8,
                121u8,
                126u8,
                231u8,
                170u8,
                208u8,
                132u8,
                249u8,
                228u8,
                164u8,
                42u8,
                130u8,
                252u8,
                80u8,
                109u8,
                205u8,
                44u8,
                163u8,
                182u8,
                251u8,
                67u8,
                31u8,
                133u8,
                204u8,
                235u8,
                229u8,
            ],
            [
                139u8,
                153u8,
                126u8,
                83u8,
                215u8,
                185u8,
                229u8,
                217u8,
                35u8,
                208u8,
                162u8,
                28u8,
                96u8,
                223u8,
                129u8,
                225u8,
                116u8,
                8u8,
                96u8,
                209u8,
                168u8,
                198u8,
                107u8,
                140u8,
                99u8,
                197u8,
                4u8,
                122u8,
                226u8,
                14u8,
                170u8,
                246u8,
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
                172u8,
                249u8,
                9u8,
                95u8,
                235u8,
                58u8,
                55u8,
                12u8,
                156u8,
                246u8,
                146u8,
                66u8,
                28u8,
                105u8,
                239u8,
                50u8,
                13u8,
                77u8,
                181u8,
                198u8,
                110u8,
                106u8,
                125u8,
                41u8,
                199u8,
                105u8,
                78u8,
                176u8,
                35u8,
                100u8,
                252u8,
                85u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AllocationManagerEvents {
        const NAME: &'static str = "AllocationManagerEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AllocationDelaySet)
                }
                Some(
                    <EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EncumberedMagnitudeUpdated)
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
                    <MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MaxMagnitudeUpdated)
                }
                Some(
                    <OperatorSetMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSetMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSetMagnitudeUpdated)
                }
                Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSlashed)
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
    impl alloy_sol_types::private::IntoLogData for AllocationManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EncumberedMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MaxMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
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
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EncumberedMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MaxMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetMagnitudeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
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
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AllocationManager`](self) contract instance.

See the [wrapper's documentation](`AllocationManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AllocationManagerInstance<T, P, N> {
        AllocationManagerInstance::<T, P, N>::new(address, provider)
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
        _delegation: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AllocationManagerInstance<T, P, N>>,
    > {
        AllocationManagerInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _delegation,
            _avsDirectory,
            _DEALLOCATION_DELAY,
            _ALLOCATION_CONFIGURATION_DELAY,
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
        _delegation: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AllocationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _delegation,
            _avsDirectory,
            _DEALLOCATION_DELAY,
            _ALLOCATION_CONFIGURATION_DELAY,
        )
    }
    /**A [`AllocationManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AllocationManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AllocationManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AllocationManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AllocationManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AllocationManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`AllocationManager`](self) contract instance.

See the [wrapper's documentation](`AllocationManagerInstance`) for more details.*/
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
            _delegation: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
        ) -> alloy_contract::Result<AllocationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegation,
                _avsDirectory,
                _DEALLOCATION_DELAY,
                _ALLOCATION_CONFIGURATION_DELAY,
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
            _delegation: alloy::sol_types::private::Address,
            _avsDirectory: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegation,
                            _avsDirectory,
                            _DEALLOCATION_DELAY,
                            _ALLOCATION_CONFIGURATION_DELAY,
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
    impl<T, P: ::core::clone::Clone, N> AllocationManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AllocationManagerInstance<T, P, N> {
            AllocationManagerInstance {
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
    > AllocationManagerInstance<T, P, N> {
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
        ///Creates a new call builder for the [`ALLOCATION_CONFIGURATION_DELAY`] function.
        pub fn ALLOCATION_CONFIGURATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            ALLOCATION_CONFIGURATION_DELAYCall,
            N,
        > {
            self.call_builder(
                &ALLOCATION_CONFIGURATION_DELAYCall {
                },
            )
        }
        ///Creates a new call builder for the [`DEALLOCATION_DELAY`] function.
        pub fn DEALLOCATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEALLOCATION_DELAYCall, N> {
            self.call_builder(&DEALLOCATION_DELAYCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`clearDeallocationQueue`] function.
        pub fn clearDeallocationQueue(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            numToClear: alloy::sol_types::private::Vec<u16>,
        ) -> alloy_contract::SolCallBuilder<T, &P, clearDeallocationQueueCall, N> {
            self.call_builder(
                &clearDeallocationQueueCall {
                    operator,
                    strategies,
                    numToClear,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`encumberedMagnitude`] function.
        pub fn encumberedMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, encumberedMagnitudeCall, N> {
            self.call_builder(
                &encumberedMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocatableMagnitude`] function.
        pub fn getAllocatableMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatableMagnitudeCall, N> {
            self.call_builder(
                &getAllocatableMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocationDelay`] function.
        pub fn getAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationDelayCall, N> {
            self.call_builder(&getAllocationDelayCall { operator })
        }
        ///Creates a new call builder for the [`getAllocationInfo_0`] function.
        pub fn getAllocationInfo_0(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationInfo_0Call, N> {
            self.call_builder(
                &getAllocationInfo_0Call {
                    operatorSet,
                    strategies,
                    operators,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocationInfo_1`] function.
        pub fn getAllocationInfo_1(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
            operatorSets: alloy::sol_types::private::Vec<
                <OperatorSet as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationInfo_1Call, N> {
            self.call_builder(
                &getAllocationInfo_1Call {
                    operator,
                    strategy,
                    operatorSets,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocationInfo_2`] function.
        pub fn getAllocationInfo_2(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationInfo_2Call, N> {
            self.call_builder(
                &getAllocationInfo_2Call {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentDelegatedAndSlashableOperatorShares`] function.
        pub fn getCurrentDelegatedAndSlashableOperatorShares(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getCurrentDelegatedAndSlashableOperatorSharesCall,
            N,
        > {
            self.call_builder(
                &getCurrentDelegatedAndSlashableOperatorSharesCall {
                    operatorSet,
                    operators,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudes`] function.
        pub fn getMaxMagnitudes(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesCall, N> {
            self.call_builder(
                &getMaxMagnitudesCall {
                    operator,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudesAtTimestamp`] function.
        pub fn getMaxMagnitudesAtTimestamp(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            timestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesAtTimestampCall, N> {
            self.call_builder(
                &getMaxMagnitudesAtTimestampCall {
                    operator,
                    strategies,
                    timestamp,
                },
            )
        }
        ///Creates a new call builder for the [`getMinDelegatedAndSlashableOperatorSharesBefore`] function.
        pub fn getMinDelegatedAndSlashableOperatorSharesBefore(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            beforeTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getMinDelegatedAndSlashableOperatorSharesBeforeCall,
            N,
        > {
            self.call_builder(
                &getMinDelegatedAndSlashableOperatorSharesBeforeCall {
                    operatorSet,
                    operators,
                    strategies,
                    beforeTimestamp,
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
        ///Creates a new call builder for the [`modifyAllocations`] function.
        pub fn modifyAllocations(
            &self,
            allocations: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::MagnitudeAllocation as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyAllocationsCall, N> {
            self.call_builder(
                &modifyAllocationsCall {
                    allocations,
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
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAllocationDelay_0`] function.
        pub fn setAllocationDelay_0(
            &self,
            operator: alloy::sol_types::private::Address,
            delay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllocationDelay_0Call, N> {
            self.call_builder(
                &setAllocationDelay_0Call {
                    operator,
                    delay,
                },
            )
        }
        ///Creates a new call builder for the [`setAllocationDelay_1`] function.
        pub fn setAllocationDelay_1(
            &self,
            delay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllocationDelay_1Call, N> {
            self.call_builder(&setAllocationDelay_1Call { delay })
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
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall { params })
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
    > AllocationManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`AllocationDelaySet`] event.
        pub fn AllocationDelaySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AllocationDelaySet, N> {
            self.event_filter::<AllocationDelaySet>()
        }
        ///Creates a new event filter for the [`EncumberedMagnitudeUpdated`] event.
        pub fn EncumberedMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EncumberedMagnitudeUpdated, N> {
            self.event_filter::<EncumberedMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`MaxMagnitudeUpdated`] event.
        pub fn MaxMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MaxMagnitudeUpdated, N> {
            self.event_filter::<MaxMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSetMagnitudeUpdated`] event.
        pub fn OperatorSetMagnitudeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetMagnitudeUpdated, N> {
            self.event_filter::<OperatorSetMagnitudeUpdated>()
        }
        ///Creates a new event filter for the [`OperatorSlashed`] event.
        pub fn OperatorSlashed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSlashed, N> {
            self.event_filter::<OperatorSlashed>()
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
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
