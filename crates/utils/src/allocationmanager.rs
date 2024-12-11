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
    error InvalidAllocationDelay();
    error InvalidExpectedTotalMagnitude();
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
    event MaxMagnitudeUpdated(address operator, address strategy, uint64 totalMagnitude);
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
    function encumberedMagnitude(address, address) external view returns (uint64);
    function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
    function getAllocationDelay(address operator) external view returns (bool isSet, uint32 delay);
    function getAllocationInfo(OperatorSet memory operatorSet, address[] memory strategies, address[] memory operators) external view returns (IAllocationManagerTypes.MagnitudeInfo[][] memory);
    function getAllocationInfo(address operator, address strategy, OperatorSet[] memory operatorSets) external view returns (IAllocationManagerTypes.MagnitudeInfo[] memory);
    function getAllocationInfo(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.MagnitudeInfo[] memory);
    function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
    function getMaxMagnitudesAtTimestamp(address operator, address[] memory strategies, uint32 timestamp) external view returns (uint64[] memory);
    function getMinDelegatedAndSlashableOperatorShares(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 beforeTimestamp) external view returns (uint256[][] memory, uint256[][] memory);
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
    "name": "getMinDelegatedAndSlashableOperatorShares",
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
        "name": "totalMagnitude",
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
    "name": "InvalidAllocationDelay",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidExpectedTotalMagnitude",
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
    ///0x610100346101b357601f613dee38819003918201601f19168301916001600160401b038311848410176101b7578084926080946040528339810103126101b3578051906001600160a01b03821682036101b35760208101516001600160a01b03811681036101b35761007f6060610078604085016101cb565b93016101cb565b9260805260a05260c05260e0525f5460ff8160081c1661015e5760ff80821610610124575b604051613c1190816101dd8239608051818181610e1b015281816111dd01528181611315015281816117a50152818161181c01528181611e85015261200b015260a0518181816106a901528181610f91015281816114630152611a06015260c0518181816108810152610b21015260e051818181611c0f01526134d90152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100a4565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b519063ffffffff821682036101b35756fe60806040526004361015610011575f80fd5b5f3560e01c80630b002119146101e457806310d67a2f146101df578063136439dd146101da5780631637b60f146101d55780631794bb3c146101d05780632981eb77146101cb57806335af054a146101c65780634b5046ef146101c15780634d9dbde9146101bc578063547afb87146101b757806356c483e6146101b2578063595c6a67146101ad5780635ac86ab7146101a85780635c489bb5146101a35780635c975abb1461019e57806360db99a3146101995780636b3aa72e146101945780636cfb44811461018f578063715018a61461018a5780637bc1ef6114610185578063843b349f14610180578063886f11951461017b5780638da5cb5b14610176578063a984eb3a14610171578063b9fbaed11461016c578063df5cf72314610167578063e07d2b1214610162578063f2fde38b1461015d5763fabc1cbc14610158575f80fd5b612285565b6121f4565b611f5c565b611e70565b611e33565b611de6565b611dbe565b611d96565b611c33565b611bf3565b611b98565b611a35565b6119f1565b61137d565b611360565b6112dc565b6112a9565b611223565b6111b2565b6110eb565b610f4a565b610d77565b610cbc565b610b05565b6109f3565b6105dc565b610515565b610471565b6102e8565b60409060031901126101fa57600490565b5f80fd5b9181601f840112156101fa578235916001600160401b0383116101fa576020808501948460051b0101116101fa57565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061026057505050505090565b9091929394603f19828203018352855190602080835192838152019201905f905b8082106102a05750505060208060019297019301930191939290610251565b90919260206102de60019286519063ffffffff60406060936001600160401b0381511684526020810151600f0b602085015201511660408201520190565b9401920190610281565b346101fa5760803660031901126101fa57610302366101e9565b6044356001600160401b0381116101fa576103219036906004016101fe565b91906064356001600160401b0381116101fa576103429036906004016101fe565b9161034c83612321565b945f5b8481106103685760405180610364898261022e565b0390f35b5f5b82811061037a575060010161034f565b94600186610452816103ca888c6103c4879f9b9a999d6103b78f8f928d6103b16103ac8f946103ac956103bf9861237e565b612393565b9761237e565b923690610c09565b612cf1565b91612d89565b6104366103e160208301516001600160401b031690565b9161042961040460606103f86040850151600f0b90565b93015163ffffffff1690565b9161041f610410610bb5565b6001600160401b039096168652565b600f0b6020850152565b63ffffffff166040830152565b6104408b8a6123a8565b519061044c83836123a8565b526123a8565b50019095919293975061036a565b6001600160a01b038116036101fa57565b346101fa5760203660031901126101fa576004803561048f81610460565b60655460405163755b36bd60e11b81529260209184919082906001600160a01b03165afa918215610510576104df926104da915f916104e1575b506001600160a01b031633146123dc565b612efe565b005b610503915060203d602011610509575b6104fb8183610b94565b8101906123bc565b5f6104c9565b503d6104f1565b6123d1565b346101fa5760203660031901126101fa5760043560655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa80156105105761056c915f916105ad575b5061240a565b61057b60665482811614612420565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b6105cf915060203d6020116105d5575b6105c78183610b94565b8101906123f2565b5f610566565b503d6105bd565b346101fa5760203660031901126101fa576004356001600160401b0381116101fa5761060f6106389136906004016101fe565b9190610628610622600180606654161490565b15612436565b61063133612af0565b929061244c565b5f905b83821061064457005b61064f828583612462565b61067761065f6040830183612484565b905061066e60608401846124b9565b919050146124ee565b6106a560206106896040840184612484565b604051631062711b60e11b815293849283929060048401612534565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610510576106ea915f916109d5575b5061256f565b6106fc6106f682612393565b33612f7a565b335f9081526097602052604090206107349061072f9061071b84612393565b60018060a01b03165f5260205260405f2090565b613166565b9161076361075361074760208501612585565b6001600160401b031690565b6001600160401b03851614612599565b5f5b6107726040840184612484565b90508110156109c557807f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66107c16103bf6107bc6001956107b660408a018a612484565b906125af565b61239d565b6108ee6107d7826107d189612393565b33612d89565b80926107f96107f36107ed6040850151600f0b90565b600f0b90565b156125bf565b61084461083a8a61083461082f8a61082961081e60208a01516001600160401b031690565b9460608101906124b9565b9061237e565b612585565b906131a1565b600f0b6040840152565b61085f6108586107ed6040850151600f0b90565b15156125d5565b6108706107ed6040840151600f0b90565b5f81121561095a57506108b96108ac7f000000000000000000000000000000000000000000000000000000000000000063ffffffff42166125ff565b63ffffffff166060840152565b335f908152609a602052604090206108df9082906108da9061071b8d612393565b6131ed565b6108e889612393565b33613238565b6108ff836107b66040890189612484565b61095161090b88612393565b92610943606061093761092860208501516001600160401b031690565b6040850151600f0b5b906131d2565b92015163ffffffff1690565b90604051948594338661262f565b0390a101610765565b5f12156108df576109746108ac8d63ffffffff42166125ff565b61099861098b61092884516001600160401b031690565b6001600160401b03168352565b6109c06001600160401b038b166109b961074785516001600160401b031690565b1115612619565b6108df565b50600190930192915061063b9050565b6109ed915060203d81116105d5576105c78183610b94565b5f6106e4565b346101fa5760603660031901126101fa57600435610a1081610460565b610a6e602435610a1f81610460565b604435905f5493610a54610a3e610a3a8760ff9060081c1690565b1590565b80968197610aed575b8115610acd575b5061267e565b84610a65600160ff195f5416175f55565b610ab6576126e1565b610a7457005b610a8261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081015b0390a1005b610ac861010061ff00195f5416175f55565b6126e1565b303b15915081610adf575b505f610a4e565b60ff1660011490505f610ad8565b600160ff8216109150610a47565b5f9103126101fa57565b346101fa575f3660031901126101fa57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b03821117610b7457604052565b610b45565b606081019081106001600160401b03821117610b7457604052565b90601f801991011681019081106001600160401b03821117610b7457604052565b60405190610bc4606083610b94565b565b60405190610bc4604083610b94565b60405190610bc4608083610b94565b6001600160401b038111610b745760051b60200190565b63ffffffff8116036101fa57565b91908260409103126101fa57604051610c2181610b59565b60208082948035610c3181610460565b8452013591610c3f83610bfb565b0152565b90602080835192838152019201905f5b818110610c605750505090565b9091926020610c9e60019286519063ffffffff60406060936001600160401b0381511684526020810151600f0b602085015201511660408201520190565b9401929101610c53565b906020610cb9928181520190610c43565b90565b346101fa5760603660031901126101fa57600435610cd981610460565b602435610ce581610460565b604435906001600160401b0382116101fa57366023830112156101fa57816004013592610d1184610be4565b92610d1f6040519485610b94565b8484526024602085019560061b820101903682116101fa57602401945b818610610d5d57610364610d51868686612755565b60405191829182610ca8565b6020604091610d6c3689610c09565b815201950194610d3c565b346101fa5760603660031901126101fa57600435610d9481610460565b6024356001600160401b0381116101fa57610db39036906004016101fe565b6044929192356001600160401b0381116101fa57610dd59036906004016101fe565b9290610de8610622600180606654161490565b610df38484146124ee565b6040516336b87bd760e11b81526001600160a01b0383166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561051057610e5c915f91610e9b575b5061281b565b5f5b838110610e6757005b80610e95610e7b6103ac600194888b61237e565b610e8e610e89848a8861237e565b612831565b9086613070565b01610e5e565b610eb4915060203d6020116105d5576105c78183610b94565b5f610e56565b60409060031901126101fa57600435610ed281610460565b90602435610cb981610460565b9291604084019360408152825180955260206060820193015f955b808710610f16575050610cb99394506020818403910152610c43565b815180516001600160a01b0316865260209081015163ffffffff168187015291949091600191604001950196019590610efa565b346101fa57610f5836610eba565b6040516316ae76cb60e01b81526001600160a01b0380841660048301525f602483018190525f19604484015291939190849060649082907f0000000000000000000000000000000000000000000000000000000000000000165afa928315610510575f93610fdd575b50610fcd918391612755565b9061036460405192839283610edf565b92503d805f853e610fee8185610b94565b8301926020818503126101fa578051906001600160401b0382116101fa57019280601f850112156101fa5783519361102585610be4565b916110336040519384610b94565b85835260208084019660061b830101918183116101fa57602001955b82871061106657505050610fcd9293509291610fc1565b6040878303126101fa576020604091825161108081610b59565b895161108b81610460565b8152828a015161109a81610bfb565b8382015281520196019561104f565b60206040818301928281528451809452019201905f5b8181106110cc5750505090565b82516001600160401b03168452602093840193909201916001016110bf565b346101fa5760403660031901126101fa5760043561110881610460565b6024356001600160401b0381116101fa576111279036906004016101fe565b919061113283612840565b926001600160a01b03909216915f5b818110611156576040518061036487826110a9565b600190845f52609760205261119661072f60405f2061117684878961237e565b359061118182610460565b9060018060a01b03165f5260205260405f2090565b6001600160401b036111a883896123a8565b9116905201611141565b346101fa5760403660031901126101fa576004356111cf81610460565b6024356111db81610bfb565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303611214576104df91613437565b63f739589b60e01b5f5260045ffd5b346101fa575f3660031901126101fa5760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561051057611275915f916105ad575061240a565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b346101fa5760203660031901126101fa5760043560ff81168091036101fa5760016020911b806066541614604051908152f35b346101fa5760203660031901126101fa576004356112f981610bfb565b6040516336b87bd760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610510576104df9261135a915f91610e9b575061281b565b33613437565b346101fa575f3660031901126101fa576020606654604051908152f35b346101fa5760203660031901126101fa576004356001600160401b0381116101fa5760a060031982360301126101fa576113be610622600280606654161490565b606481013590811515806119df575b6113d690612872565b6113e260248201612888565b916113ff6113ee610bc6565b3381529363ffffffff166020850152565b61140883612cf1565b9161145f60208561141b84600401612393565b6040516309a961f360e11b81526001600160a01b0391821660048201528251909116602482015260209091015163ffffffff16604482015291829081906064820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610510576114a7915f916119c0575b50949394612892565b6114c16114ba60448301836004016124b9565b9050612840565b670de0b6b3a7640000949092905f5b6114e060448501856004016124b9565b905081101561195a57611516826114f986600401612393565b6115106103ac8561082960448b018b6004016124b9565b90612d89565b9061153861153161074760208501516001600160401b031690565b15156128a8565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf68561157d6107478761157861074760208901516001600160401b031690565b6137ec565b936115ae61159e8661159960208501516001600160401b031690565b6128be565b6001600160401b03166020830152565b6115d36115c68661159984516001600160401b031690565b6001600160401b03168252565b866115e26040830151600f0b90565b5f6115ed82600f0b90565b12611898575b5050611623818761160685600401612393565b61161d6103ac8961082960448a018a6004016124b9565b90613238565b61167461165e602061164f6103ac8861082961164189600401612393565b9860448101906004016124b9565b9301516001600160401b031690565b926040519384938d63ffffffff4216938661291b565b0390a16116bc61072f6116a561168c88600401612393565b6001600160a01b03165f90815260976020526040902090565b61071b6103ac8561082960448c018c6004016124b9565b916116c781846128be565b9061170a82846117058a61071b6103ac6116f76116e961168c85600401612393565b9360448101906004016124b9565b4263ffffffff16969161237e565b613a38565b50507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c61173988600401612393565b6117506103ac8661082960448d018d6004016124b9565b604080516001600160a01b0393841681529190921660208201526001600160401b03851691810191909152606090a161178b87600401612393565b916117a36103ac8561082960448c018c6004016124b9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163b156101fa5760405163a57ab10b60e01b81526001600160a01b039485166004820152931660248401526001600160401b0380861660448501521660648301529092905f8480608481010381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1918215610510576001600160401b03808c9260019761186d9661187e575b50169216613906565b61187782886123a8565b52016114d0565b8061188c5f61189293610b94565b80610afb565b5f611864565b6118d56118c66107476107476118ec956115786118ba6118ba6118e2986128de565b6001600160801b031690565b6001600160801b0316600f0b90565b6040840151600f0b6128f5565b600f0b6040830152565b828a6118fa84600401612393565b6119506119146103ac8961082960448a018a6004016124b9565b61193861192b60208801516001600160401b031690565b6040880151600f0b610931565b606087015163ffffffff16916040519586958661291b565b0390a1865f6115f3565b7f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe58686610ab16119b18861199081600401612393565b936119a160448301836004016124b9565b9390926084810190600401612964565b939092604051978897886129f9565b6119d9915060203d6020116105d5576105c78183610b94565b5f61149e565b50670de0b6b3a76400008211156113cd565b346101fa575f3660031901126101fa576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101fa57611a4336610eba565b611a71611a64826111818560018060a01b03165f52609860205260405f2090565b546001600160401b031690565b90611aa3611a93826111818660018060a01b03165f52609a60205260405f2090565b5480600f0b9060801d600f0b0390565b5f905b808210611af0575b610364611ad68561159961072f876111818b60018060a01b03165f52609760205260405f2090565b6040516001600160401b0390911681529081906020820190565b9092611b53611b4e611b1f86611b1a876111818b60018060a01b03165f52609a60205260405f2090565b613600565b6001600160a01b0388165f908152609960205260409020611b41908790611181565b905f5260205260405f2090565b612a8a565b611b70611b67604083015163ffffffff1690565b63ffffffff1690565b4210611b915760200151600191611b8991600f0b610931565b930190611aa6565b5092611aae565b346101fa575f3660031901126101fa57611bb06136be565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101fa575f3660031901126101fa57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101fa5760603660031901126101fa57600435611c5081610460565b6024356001600160401b0381116101fa57611c6f9036906004016101fe565b9160443591611c7d83610bfb565b611c8684612840565b936001600160a01b039092169263ffffffff16915f5b818110611cb1576040518061036488826110a9565b845f526097602052611ccb60405f2061117683858761237e565b8054905f5b828110611d425750600192919081611d0f575050611d09670de0b6b3a76400005b611cfb838a6123a8565b906001600160401b03169052565b01611c9c565b611d2d611d3d91611d22611d0994613158565b905f5260205f200190565b5460201c6001600160401b031690565b611cf1565b918083169080841860011c8201809211611d9157825f528763ffffffff611d738460205f200163ffffffff90541690565b161115611d81575091611cd0565b9250611d8c9061341c565b611cd0565b6125eb565b346101fa575f3660031901126101fa576065546040516001600160a01b039091168152602090f35b346101fa575f3660031901126101fa576033546040516001600160a01b039091168152602090f35b346101fa5760206001600160401b03611e29611e0136610eba565b6001600160a01b039182165f9081526098865260408082209290931681526020919091522090565b5416604051908152f35b346101fa5760203660031901126101fa57604063ffffffff611e5f600435611e5a81610460565b612af0565b835191151582529091166020820152f35b346101fa575f3660031901126101fa576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080602083519182815201916020808360051b8301019401925f915b838310611edf57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b808210611f1f5750505060208060019297019301930191939290611ed0565b90919260208060019286518152019401920190611f00565b9091611f4e610cb993604084526040840190611eb4565b916020818403910152611eb4565b346101fa5760a03660031901126101fa57611f76366101e9565b6044356001600160401b0381116101fa57611f959036906004016101fe565b6064929192356001600160401b0381116101fa57611fb79036906004016101fe565b611fe56103bf63ffffffff608495949535611fd181610bfb565b1695611fde428811612b90565b3690610c09565b9460405194637870733b60e11b86525f86806120078689898860048601612c94565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa958615610510575f966121d0575b5061204f849294612321565b96670de0b6b3a7640000925f5b81811061207257604051806103648c8c83611f37565b6120806103ac82848a61237e565b61208987612840565b612093838d6123a8565b5261209e828c6123a8565b505f8b87878d888c8f5b8188106120c05750505050505050505060010161205c565b8861218861074761072f6111816111818f978e9d9c60019f9b9c8c8b6121949f61211d61215e9f611b4e9061218e9f8f6121046103ac8961216d9d611b419561237e565b9c8d9160018060a01b03165f52609960205260405f2090565b80516001600160401b03169463ffffffff61213f604084015163ffffffff1690565b1611156121a2575b509161215e612164926001600160401b03946123a8565b516123a8565b51911690613906565b6001600160a01b039096165f90815260976020526040902090565b90613866565b946123a8565b52018b87878d888c8f6120a8565b612164926121c56001600160401b03959396610931602061215e950151600f0b90565b959294509250612147565b6121ed9196503d805f833e6121e58183610b94565b810190612ba6565b945f612043565b346101fa5760203660031901126101fa5760043561221181610460565b6122196136be565b6001600160a01b03811615612231576104df906133be565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346101fa5760203660031901126101fa5760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa8015610510576122de915f916104e157506001600160a01b031633146123dc565b6122ef606654198219811614612420565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b9061232b82610be4565b6123386040519182610b94565b8281526020819361234b601f1991610be4565b0191015f5b82811061235c57505050565b606082820152602001612350565b634e487b7160e01b5f52603260045260245ffd5b919081101561238e5760051b0190565b61236a565b35610cb981610460565b610cb9903690610c09565b805182101561238e5760209160051b010190565b908160209103126101fa5751610cb981610460565b6040513d5f823e3d90fd5b156123e357565b63794821ff60e01b5f5260045ffd5b908160209103126101fa575180151581036101fa5790565b1561241157565b631d77d47760e21b5f5260045ffd5b1561242757565b63c61dca5d60e01b5f5260045ffd5b1561243d57565b63840a48d560e01b5f5260045ffd5b1561245357565b63fa55fc8160e01b5f5260045ffd5b919081101561238e5760051b81013590607e19813603018212156101fa570190565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa57602001918160061b360383136101fa57565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa57602001918160051b360383136101fa57565b156124f557565b6343714afd60e01b5f5260045ffd5b63ffffffff60208092803561251881610460565b6001600160a01b03168552013561252e81610bfb565b16910152565b60208082528101839052604001915f5b8181106125515750505090565b9091926040808261256460019488612504565b019401929101612544565b1561257657565b631fb1705560e21b5f5260045ffd5b356001600160401b03811681036101fa5790565b156125a057565b633a3fa06360e21b5f5260045ffd5b919081101561238e5760061b0190565b156125c657565b630d8fcbe360e41b5f5260045ffd5b156125dc57565b634606179360e11b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b9063ffffffff8091169116019063ffffffff8211611d9157565b1561262057565b63329d4e5360e21b5f5260045ffd5b6001600160a01b03909116815260c081019594909360a09363ffffffff936001600160401b03929190612666906020890190612504565b600180871b0316606087015216608085015216910152565b1561268557565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b606554610bc493919261273e9290916001600160a01b03161580612743575b61270990612ee8565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2612efe565b6133be565b506001600160a01b0382161515612700565b9092919283519361276585610be4565b946127736040519687610b94565b808652612782601f1991610be4565b015f5b8181106127f25750505f5b81518110156127ec57806127b96127b26127ac600194866123a8565b51612cf1565b8587612d89565b6127d06103e160208301516001600160401b031690565b6127da82896123a8565b526127e581886123a8565b5001612790565b50505050565b60209060405161280181610b79565b5f81525f838201525f604082015282828a01015201612785565b1561282257565b6325ec6c1f60e01b5f5260045ffd5b3561ffff811681036101fa5790565b9061284a82610be4565b6128576040519182610b94565b8281528092612868601f1991610be4565b0190602036910137565b1561287957565b631353603160e01b5f5260045ffd5b35610cb981610bfb565b1561289957565b63ccea9e6f60e01b5f5260045ffd5b156128af57565b634e99e6cf60e01b5f5260045ffd5b906001600160401b03809116911603906001600160401b038211611d9157565b600f0b60016001607f1b03198114611d91575f0390565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b03831317611d9157565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff908116604083015260c082019594919360a0939192916001600160401b039190612666565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa576020019181360383136101fa57565b916020908281520191905f5b8181106129af5750505090565b90919260208060019286356129c381610460565b848060a01b0316815201940191019190916129a2565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff1660408201529594929391612a3e919060c0606089015260c0880191612996565b9480860360808201526020808551978881520194015f965b808810612a72575050610cb994955060a08185039101526129d9565b90946020806001928851815201960197019690612a56565b90604051612a9781610b79565b604063ffffffff8294546001600160401b038116845280831c600f0b602085015260c01c16910152565b90604051612ace81610b79565b604063ffffffff8294548181168452818160201c166020850152821c16910152565b60018060a01b03165f52609b6020526040805f20612b42825191612b1383610b79565b5463ffffffff81169283815263ffffffff808360201c1692836020840152861c16948591015263ffffffff1690565b9163ffffffff831615159081612b77575b5015612b695750905b63ffffffff821615159190565b63ffffffff16905090612b5c565b612b87915063ffffffff16611b67565b4210155f612b53565b15612b9757565b63b7d0949760e01b5f5260045ffd5b6020818303126101fa578051906001600160401b0382116101fa57019080601f830112156101fa57815190612bda82610be4565b92612be86040519485610b94565b82845260208085019360051b820101908282116101fa5760208101935b828510612c1457505050505090565b84516001600160401b0381116101fa57820184603f820112156101fa57602081015190612c4082610be4565b91612c4e6040519384610b94565b8083526020808085019260051b84010101918783116101fa57604001905b828210612c8457505050815260209485019401612c05565b8151815260209182019101612c6c565b949391949290928560408201604083525260608101935f965b808810612cc9575050610cb99495506020818503910152612996565b90946020806001928835612cdc81610460565b848060a01b0316815201960197019690612cad565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b16603482015260208152612d37604082610b94565b5190519060208110612d47575090565b5f199060200360031b1b1690565b60405190608082018281106001600160401b03821117610b74576040525f6060838281528260208201528260408201520152565b90611a64612ddc91611181612dc1611b4e612da2612d55565b97611b41856111818a60018060a01b03165f52609960205260405f2090565b6001600160a01b039095165f90815260986020526040902090565b90612dee604082015163ffffffff1690565b63ffffffff81164210612e79575080612e37612e276020612e19612e5695516001600160401b031690565b9301926109318451600f0b90565b6001600160401b03166020860152565b6001600160401b03831684525f60608501525f604085015251600f0b90565b905f612e6283600f0b90565b12612e6c57505090565b610cb9916115c6916131d2565b90610cb992935080612ed1612ea66020612e9d612edb95516001600160401b031690565b930151600f0b90565b91612ec1612eb2610bd5565b6001600160401b039098168852565b6001600160401b03166020870152565b600f0b6040850152565b63ffffffff166060830152565b15612eef57565b6339b190bb60e11b5f5260045ffd5b6001600160a01b0316612f12811515612ee8565b606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b5f198114611d915760010190565b8015611d91575f190190565b6001600160a01b038082165f908152609a6020908152604080832093861683529290529081209093929190612fae90611a93565b935b84151580613065575b1561305e57612fe4612fdf846111818560018060a01b03165f52609a60205260405f2090565b613716565b612fef818585612d89565b90613004611b67606084015163ffffffff1690565b4210613055579161301d6130499261304f948787613238565b61304361303e866111818760018060a01b03165f52609a60205260405f2090565b613757565b50612f60565b94612f6e565b93612fb0565b50505050509050565b5050509050565b5061ffff8110612fb9565b6001600160a01b038181165f908152609a6020908152604080832093861683529290529081209094906130a290611a93565b945b8515158061314b575b15613143576130d3612fdf856111818660018060a01b03165f52609a60205260405f2090565b6130de818686612d89565b906130f3611b67606084015163ffffffff1690565b4210613139579161310c61312d92613133948888613238565b61304361303e876111818860018060a01b03165f52609a60205260405f2090565b95612f6e565b946130a4565b5050509350505050565b509350505050565b5061ffff851681106130ad565b5f19810191908211611d9157565b80548061317b575050670de0b6b3a764000090565b805f19810111611d91576001600160401b03915f525f199060205f2001015460201c1690565b6001600160401b03809116600f0b9116600f0b0360016001607f1b03811360016001607f1b0319821217611d915790565b6001600160401b0391826131e99216600f0b6128f5565b1690565b90815460801d9061320c826001850190600f0b5f5260205260405f2090565b5581546001600160801b0316600190910160801b6fffffffffffffffffffffffffffffffff1916179055565b90916133867facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55946133286133b9936132c86001600160401b036020850151169163ffffffff6040860151600f0b81606088015116906040519561329a87610b79565b8652600f0b6020860152166040840152611b41896111818a60018060a01b03165f52609960205260405f2090565b81518154602084015160409485015163ffffffff60c01b60c09190911b1677ffffffffffffffffffffffffffffffff00000000000000009190951b166001600160e01b03199091166001600160401b039092169190911717919091179055565b61337961333c82516001600160401b031690565b6001600160a01b0386165f90815260986020526040902061335e908890611181565b906001600160401b03166001600160401b0319825416179055565b516001600160401b031690565b604080516001600160a01b0394851681529490931660208501526001600160401b03169183019190915281906060820190565b0390a1565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b1561340d57565b63dd18181560e01b5f5260045ffd5b9060018201809211611d9157565b91908201809211611d9157565b907f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9161346b63ffffffff83161515613406565b6133b961358961349361348e8460018060a01b03165f52609b60205260405f2090565b612ac1565b6134c985602083016134a9815163ffffffff1690565b63ffffffff81161515806135c8575b6135ba575b509063ffffffff169052565b61357f6134ff611b6763ffffffff7f0000000000000000000000000000000000000000000000000000000000000000164261342a565b63ffffffff1660408301908152916001600160a01b0386165f908152609b602052604090208151815463ffffffff191663ffffffff919091161781559060208181015183546040938401516bffffffffffffffff00000000199091169190921b67ffffffff000000001617911b6bffffffff000000000000000016179055565b5163ffffffff1690565b604080516001600160a01b03909416845263ffffffff94851660208501529316928201929092529081906060820190565b63ffffffff1684525f6134bd565b506135dd611b67604087015163ffffffff1690565b4210156134b8565b9190915f8382019384129112908015821691151617611d9157565b805490916001600160ff1b038111613668576107ed61362d6136286136349385600f0b6135e5565b613b58565b9260801d90565b81600f0b1215613659576001613655920190600f0b5f5260205260405f2090565b5490565b632d0483c560e21b5f5260045ffd5b60405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608490fd5b6033546001600160a01b031633036136d257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61372b815480600f0b9060801d600f0b131590565b613748578054600f0b5f9081526001909101602052604090205490565b631ed9509560e11b5f5260045ffd5b9061376d825480600f0b9060801d600f0b131590565b613748578154600f0b9160018101925f6137a882613796818890600f0b5f5260205260405f2090565b549690600f0b5f5260205260405f2090565b5560016001600160801b031983541691016001600160801b0316179055565b81156137d1570490565b634e487b7160e01b5f52601260045260245ffd5b156101fa57565b5f19828209828202918280831092039180830392146138555781670de0b6b3a764000011156101fa577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b5f19670de0b6b3a7640000820991670de0b6b3a76400008202918280851094039380850394146138fa57670de0b6b3a764000082916138a68684116137e5565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b5090610cb992506137c7565b90915f198383099280830292838086109503948086039514613930579082916138a68684116137e5565b505090610cb992506137c7565b90815468010000000000000000811015610b74576001810180845581101561238e5760206001600160401b0391610bc4945f52815f20019261399663ffffffff825116859063ffffffff1663ffffffff19825416179055565b015182546bffffffffffffffff000000001916911660201b6bffffffffffffffff0000000016179055565b906040516139ce81610b59565b60206001600160401b0382945463ffffffff81168452821c16910152565b156139f357565b60405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606490fd5b909291928382548015155f14613b2e57926020929184613a70613a6b613a60613af498613158565b855f5260205f200190565b6139c1565b9363ffffffff613a95613a87875163ffffffff1690565b8284169283911611156139ec565b613aa6611b67875163ffffffff1690565b03613af85750613ae692611d22613abc92613158565b906bffffffffffffffff0000000082549160201b16906bffffffffffffffff000000001916179055565b01516001600160401b031690565b9190565b915050613b2991613b16613b0a610bc6565b63ffffffff9093168352565b6001600160401b0388168286015261393d565b613ae6565b5050613b5391613b3f613b0a610bc6565b6001600160401b038516602083015261393d565b5f9190565b60016001607f1b031981121580613bca575b15613b7557600f0b90565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608490fd5b5060016001607f1b03811315613b6a56fea264697066735822122029a6b3d18fd735af7f32c8fcf7765c900ac2a6f8fc74b5d7ab041ccbea6e56ca64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x004a\x01\xB3W`\x1Fa=\xEE8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\xB7W\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\x01\xB3W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xB3W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xB3Wa\0\x7F``a\0x`@\x85\x01a\x01\xCBV[\x93\x01a\x01\xCBV[\x92`\x80R`\xA0R`\xC0R`\xE0R_T`\xFF\x81`\x08\x1C\x16a\x01^W`\xFF\x80\x82\x16\x10a\x01$W[`@Qa<\x11\x90\x81a\x01\xDD\x829`\x80Q\x81\x81\x81a\x0E\x1B\x01R\x81\x81a\x11\xDD\x01R\x81\x81a\x13\x15\x01R\x81\x81a\x17\xA5\x01R\x81\x81a\x18\x1C\x01R\x81\x81a\x1E\x85\x01Ra \x0B\x01R`\xA0Q\x81\x81\x81a\x06\xA9\x01R\x81\x81a\x0F\x91\x01R\x81\x81a\x14c\x01Ra\x1A\x06\x01R`\xC0Q\x81\x81\x81a\x08\x81\x01Ra\x0B!\x01R`\xE0Q\x81\x81\x81a\x1C\x0F\x01Ra4\xD9\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xA4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xB3WV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\0!\x19\x14a\x01\xE4W\x80c\x10\xD6z/\x14a\x01\xDFW\x80c\x13d9\xDD\x14a\x01\xDAW\x80c\x167\xB6\x0F\x14a\x01\xD5W\x80c\x17\x94\xBB<\x14a\x01\xD0W\x80c)\x81\xEBw\x14a\x01\xCBW\x80c5\xAF\x05J\x14a\x01\xC6W\x80cKPF\xEF\x14a\x01\xC1W\x80cM\x9D\xBD\xE9\x14a\x01\xBCW\x80cTz\xFB\x87\x14a\x01\xB7W\x80cV\xC4\x83\xE6\x14a\x01\xB2W\x80cY\\jg\x14a\x01\xADW\x80cZ\xC8j\xB7\x14a\x01\xA8W\x80c\\H\x9B\xB5\x14a\x01\xA3W\x80c\\\x97Z\xBB\x14a\x01\x9EW\x80c`\xDB\x99\xA3\x14a\x01\x99W\x80ck:\xA7.\x14a\x01\x94W\x80cl\xFBD\x81\x14a\x01\x8FW\x80cqP\x18\xA6\x14a\x01\x8AW\x80c{\xC1\xEFa\x14a\x01\x85W\x80c\x84;4\x9F\x14a\x01\x80W\x80c\x88o\x11\x95\x14a\x01{W\x80c\x8D\xA5\xCB[\x14a\x01vW\x80c\xA9\x84\xEB:\x14a\x01qW\x80c\xB9\xFB\xAE\xD1\x14a\x01lW\x80c\xDF\\\xF7#\x14a\x01gW\x80c\xE0}+\x12\x14a\x01bW\x80c\xF2\xFD\xE3\x8B\x14a\x01]Wc\xFA\xBC\x1C\xBC\x14a\x01XW_\x80\xFD[a\"\x85V[a!\xF4V[a\x1F\\V[a\x1EpV[a\x1E3V[a\x1D\xE6V[a\x1D\xBEV[a\x1D\x96V[a\x1C3V[a\x1B\xF3V[a\x1B\x98V[a\x1A5V[a\x19\xF1V[a\x13}V[a\x13`V[a\x12\xDCV[a\x12\xA9V[a\x12#V[a\x11\xB2V[a\x10\xEBV[a\x0FJV[a\rwV[a\x0C\xBCV[a\x0B\x05V[a\t\xF3V[a\x05\xDCV[a\x05\x15V[a\x04qV[a\x02\xE8V[`@\x90`\x03\x19\x01\x12a\x01\xFAW`\x04\x90V[_\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xFAW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xFAW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xFAWV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x02`WPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xA0WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x02QV[\x90\x91\x92` a\x02\xDE`\x01\x92\x86Q\x90c\xFF\xFF\xFF\xFF`@``\x93`\x01`\x01`@\x1B\x03\x81Q\x16\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R\x01Q\x16`@\x82\x01R\x01\x90V[\x94\x01\x92\x01\x90a\x02\x81V[4a\x01\xFAW`\x806`\x03\x19\x01\x12a\x01\xFAWa\x03\x026a\x01\xE9V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x03!\x906\x90`\x04\x01a\x01\xFEV[\x91\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x03B\x906\x90`\x04\x01a\x01\xFEV[\x91a\x03L\x83a#!V[\x94_[\x84\x81\x10a\x03hW`@Q\x80a\x03d\x89\x82a\x02.V[\x03\x90\xF3[_[\x82\x81\x10a\x03zWP`\x01\x01a\x03OV[\x94`\x01\x86a\x04R\x81a\x03\xCA\x88\x8Ca\x03\xC4\x87\x9F\x9B\x9A\x99\x9Da\x03\xB7\x8F\x8F\x92\x8Da\x03\xB1a\x03\xAC\x8F\x94a\x03\xAC\x95a\x03\xBF\x98a#~V[a#\x93V[\x97a#~V[\x926\x90a\x0C\tV[a,\xF1V[\x91a-\x89V[a\x046a\x03\xE1` \x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91a\x04)a\x04\x04``a\x03\xF8`@\x85\x01Q`\x0F\x0B\x90V[\x93\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x91a\x04\x1Fa\x04\x10a\x0B\xB5V[`\x01`\x01`@\x1B\x03\x90\x96\x16\x86RV[`\x0F\x0B` \x85\x01RV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01RV[a\x04@\x8B\x8Aa#\xA8V[Q\x90a\x04L\x83\x83a#\xA8V[Ra#\xA8V[P\x01\x90\x95\x91\x92\x93\x97Pa\x03jV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xFAWV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x04\x805a\x04\x8F\x81a\x04`V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x10Wa\x04\xDF\x92a\x04\xDA\x91_\x91a\x04\xE1W[P`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDCV[a.\xFEV[\0[a\x05\x03\x91P` =` \x11a\x05\tW[a\x04\xFB\x81\x83a\x0B\x94V[\x81\x01\x90a#\xBCV[_a\x04\xC9V[P=a\x04\xF1V[a#\xD1V[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x05l\x91_\x91a\x05\xADW[Pa$\nV[a\x05{`fT\x82\x81\x16\x14a$ V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x05\xCF\x91P` =` \x11a\x05\xD5W[a\x05\xC7\x81\x83a\x0B\x94V[\x81\x01\x90a#\xF2V[_a\x05fV[P=a\x05\xBDV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x06\x0Fa\x068\x916\x90`\x04\x01a\x01\xFEV[\x91\x90a\x06(a\x06\"`\x01\x80`fT\x16\x14\x90V[\x15a$6V[a\x0613a*\xF0V[\x92\x90a$LV[_\x90[\x83\x82\x10a\x06DW\0[a\x06O\x82\x85\x83a$bV[a\x06wa\x06_`@\x83\x01\x83a$\x84V[\x90Pa\x06n``\x84\x01\x84a$\xB9V[\x91\x90P\x14a$\xEEV[a\x06\xA5` a\x06\x89`@\x84\x01\x84a$\x84V[`@Qc\x10bq\x1B`\xE1\x1B\x81R\x93\x84\x92\x83\x92\x90`\x04\x84\x01a%4V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x06\xEA\x91_\x91a\t\xD5W[Pa%oV[a\x06\xFCa\x06\xF6\x82a#\x93V[3a/zV[3_\x90\x81R`\x97` R`@\x90 a\x074\x90a\x07/\x90a\x07\x1B\x84a#\x93V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a1fV[\x91a\x07ca\x07Sa\x07G` \x85\x01a%\x85V[`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x85\x16\x14a%\x99V[_[a\x07r`@\x84\x01\x84a$\x84V[\x90P\x81\x10\x15a\t\xC5W\x80\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x07\xC1a\x03\xBFa\x07\xBC`\x01\x95a\x07\xB6`@\x8A\x01\x8Aa$\x84V[\x90a%\xAFV[a#\x9DV[a\x08\xEEa\x07\xD7\x82a\x07\xD1\x89a#\x93V[3a-\x89V[\x80\x92a\x07\xF9a\x07\xF3a\x07\xED`@\x85\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x15a%\xBFV[a\x08Da\x08:\x8Aa\x084a\x08/\x8Aa\x08)a\x08\x1E` \x8A\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x94``\x81\x01\x90a$\xB9V[\x90a#~V[a%\x85V[\x90a1\xA1V[`\x0F\x0B`@\x84\x01RV[a\x08_a\x08Xa\x07\xED`@\x85\x01Q`\x0F\x0B\x90V[\x15\x15a%\xD5V[a\x08pa\x07\xED`@\x84\x01Q`\x0F\x0B\x90V[_\x81\x12\x15a\tZWPa\x08\xB9a\x08\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFB\x16a%\xFFV[c\xFF\xFF\xFF\xFF\x16``\x84\x01RV[3_\x90\x81R`\x9A` R`@\x90 a\x08\xDF\x90\x82\x90a\x08\xDA\x90a\x07\x1B\x8Da#\x93V[a1\xEDV[a\x08\xE8\x89a#\x93V[3a28V[a\x08\xFF\x83a\x07\xB6`@\x89\x01\x89a$\x84V[a\tQa\t\x0B\x88a#\x93V[\x92a\tC``a\t7a\t(` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x85\x01Q`\x0F\x0B[\x90a1\xD2V[\x92\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x94\x85\x943\x86a&/V[\x03\x90\xA1\x01a\x07eV[_\x12\x15a\x08\xDFWa\tta\x08\xAC\x8Dc\xFF\xFF\xFF\xFFB\x16a%\xFFV[a\t\x98a\t\x8Ba\t(\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x83RV[a\t\xC0`\x01`\x01`@\x1B\x03\x8B\x16a\t\xB9a\x07G\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x11\x15a&\x19V[a\x08\xDFV[P`\x01\x90\x93\x01\x92\x91Pa\x06;\x90PV[a\t\xED\x91P` =\x81\x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x06\xE4V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\n\x10\x81a\x04`V[a\nn`$5a\n\x1F\x81a\x04`V[`D5\x90_T\x93a\nTa\n>a\n:\x87`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x96\x81\x97a\n\xEDW[\x81\x15a\n\xCDW[Pa&~V[\x84a\ne`\x01`\xFF\x19_T\x16\x17_UV[a\n\xB6Wa&\xE1V[a\ntW\0[a\n\x82a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01[\x03\x90\xA1\0[a\n\xC8a\x01\0a\xFF\0\x19_T\x16\x17_UV[a&\xE1V[0;\x15\x91P\x81a\n\xDFW[P_a\nNV[`\xFF\x16`\x01\x14\x90P_a\n\xD8V[`\x01`\xFF\x82\x16\x10\x91Pa\nGV[_\x91\x03\x12a\x01\xFAWV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[a\x0BEV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[`@Q\x90a\x0B\xC4``\x83a\x0B\x94V[V[`@Q\x90a\x0B\xC4`@\x83a\x0B\x94V[`@Q\x90a\x0B\xC4`\x80\x83a\x0B\x94V[`\x01`\x01`@\x1B\x03\x81\x11a\x0BtW`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xFAWV[\x91\x90\x82`@\x91\x03\x12a\x01\xFAW`@Qa\x0C!\x81a\x0BYV[` \x80\x82\x94\x805a\x0C1\x81a\x04`V[\x84R\x015\x91a\x0C?\x83a\x0B\xFBV[\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0C`WPPP\x90V[\x90\x91\x92` a\x0C\x9E`\x01\x92\x86Q\x90c\xFF\xFF\xFF\xFF`@``\x93`\x01`\x01`@\x1B\x03\x81Q\x16\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R\x01Q\x16`@\x82\x01R\x01\x90V[\x94\x01\x92\x91\x01a\x0CSV[\x90` a\x0C\xB9\x92\x81\x81R\x01\x90a\x0CCV[\x90V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x0C\xD9\x81a\x04`V[`$5a\x0C\xE5\x81a\x04`V[`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW6`#\x83\x01\x12\x15a\x01\xFAW\x81`\x04\x015\x92a\r\x11\x84a\x0B\xE4V[\x92a\r\x1F`@Q\x94\x85a\x0B\x94V[\x84\x84R`$` \x85\x01\x95`\x06\x1B\x82\x01\x01\x906\x82\x11a\x01\xFAW`$\x01\x94[\x81\x86\x10a\r]Wa\x03da\rQ\x86\x86\x86a'UV[`@Q\x91\x82\x91\x82a\x0C\xA8V[` `@\x91a\rl6\x89a\x0C\tV[\x81R\x01\x95\x01\x94a\r<V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\r\x94\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\r\xB3\x906\x90`\x04\x01a\x01\xFEV[`D\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\r\xD5\x906\x90`\x04\x01a\x01\xFEV[\x92\x90a\r\xE8a\x06\"`\x01\x80`fT\x16\x14\x90V[a\r\xF3\x84\x84\x14a$\xEEV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x0E\\\x91_\x91a\x0E\x9BW[Pa(\x1BV[_[\x83\x81\x10a\x0EgW\0[\x80a\x0E\x95a\x0E{a\x03\xAC`\x01\x94\x88\x8Ba#~V[a\x0E\x8Ea\x0E\x89\x84\x8A\x88a#~V[a(1V[\x90\x86a0pV[\x01a\x0E^V[a\x0E\xB4\x91P` =` \x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x0EVV[`@\x90`\x03\x19\x01\x12a\x01\xFAW`\x045a\x0E\xD2\x81a\x04`V[\x90`$5a\x0C\xB9\x81a\x04`V[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R` ``\x82\x01\x93\x01_\x95[\x80\x87\x10a\x0F\x16WPPa\x0C\xB9\x93\x94P` \x81\x84\x03\x91\x01Ra\x0CCV[\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x87\x01R\x91\x94\x90\x91`\x01\x91`@\x01\x95\x01\x96\x01\x95\x90a\x0E\xFAV[4a\x01\xFAWa\x0FX6a\x0E\xBAV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R_`$\x83\x01\x81\x90R_\x19`D\x84\x01R\x91\x93\x91\x90\x84\x90`d\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x05\x10W_\x93a\x0F\xDDW[Pa\x0F\xCD\x91\x83\x91a'UV[\x90a\x03d`@Q\x92\x83\x92\x83a\x0E\xDFV[\x92P=\x80_\x85>a\x0F\xEE\x81\x85a\x0B\x94V[\x83\x01\x92` \x81\x85\x03\x12a\x01\xFAW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW\x01\x92\x80`\x1F\x85\x01\x12\x15a\x01\xFAW\x83Q\x93a\x10%\x85a\x0B\xE4V[\x91a\x103`@Q\x93\x84a\x0B\x94V[\x85\x83R` \x80\x84\x01\x96`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x01\xFAW` \x01\x95[\x82\x87\x10a\x10fWPPPa\x0F\xCD\x92\x93P\x92\x91a\x0F\xC1V[`@\x87\x83\x03\x12a\x01\xFAW` `@\x91\x82Qa\x10\x80\x81a\x0BYV[\x89Qa\x10\x8B\x81a\x04`V[\x81R\x82\x8A\x01Qa\x10\x9A\x81a\x0B\xFBV[\x83\x82\x01R\x81R\x01\x96\x01\x95a\x10OV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCCWPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBFV[4a\x01\xFAW`@6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x11\x08\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x11'\x906\x90`\x04\x01a\x01\xFEV[\x91\x90a\x112\x83a(@V[\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91_[\x81\x81\x10a\x11VW`@Q\x80a\x03d\x87\x82a\x10\xA9V[`\x01\x90\x84_R`\x97` Ra\x11\x96a\x07/`@_ a\x11v\x84\x87\x89a#~V[5\x90a\x11\x81\x82a\x04`V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01`\x01`@\x1B\x03a\x11\xA8\x83\x89a#\xA8V[\x91\x16\x90R\x01a\x11AV[4a\x01\xFAW`@6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x11\xCF\x81a\x04`V[`$5a\x11\xDB\x81a\x0B\xFBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x12\x14Wa\x04\xDF\x91a47V[c\xF79X\x9B`\xE0\x1B_R`\x04_\xFD[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x12u\x91_\x91a\x05\xADWPa$\nV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\xFF\x81\x16\x80\x91\x03a\x01\xFAW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x12\xF9\x81a\x0B\xFBV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x10Wa\x04\xDF\x92a\x13Z\x91_\x91a\x0E\x9BWPa(\x1BV[3a47V[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `fT`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAW`\xA0`\x03\x19\x826\x03\x01\x12a\x01\xFAWa\x13\xBEa\x06\"`\x02\x80`fT\x16\x14\x90V[`d\x81\x015\x90\x81\x15\x15\x80a\x19\xDFW[a\x13\xD6\x90a(rV[a\x13\xE2`$\x82\x01a(\x88V[\x91a\x13\xFFa\x13\xEEa\x0B\xC6V[3\x81R\x93c\xFF\xFF\xFF\xFF\x16` \x85\x01RV[a\x14\x08\x83a,\xF1V[\x91a\x14_` \x85a\x14\x1B\x84`\x04\x01a#\x93V[`@Qc\t\xA9a\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x82Q\x90\x91\x16`$\x82\x01R` \x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x91\x82\x90\x81\x90`d\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x14\xA7\x91_\x91a\x19\xC0W[P\x94\x93\x94a(\x92V[a\x14\xC1a\x14\xBA`D\x83\x01\x83`\x04\x01a$\xB9V[\x90Pa(@V[g\r\xE0\xB6\xB3\xA7d\0\0\x94\x90\x92\x90_[a\x14\xE0`D\x85\x01\x85`\x04\x01a$\xB9V[\x90P\x81\x10\x15a\x19ZWa\x15\x16\x82a\x14\xF9\x86`\x04\x01a#\x93V[a\x15\x10a\x03\xAC\x85a\x08)`D\x8B\x01\x8B`\x04\x01a$\xB9V[\x90a-\x89V[\x90a\x158a\x151a\x07G` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x15\x15a(\xA8V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6\x85a\x15}a\x07G\x87a\x15xa\x07G` \x89\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a7\xECV[\x93a\x15\xAEa\x15\x9E\x86a\x15\x99` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a(\xBEV[`\x01`\x01`@\x1B\x03\x16` \x83\x01RV[a\x15\xD3a\x15\xC6\x86a\x15\x99\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x82RV[\x86a\x15\xE2`@\x83\x01Q`\x0F\x0B\x90V[_a\x15\xED\x82`\x0F\x0B\x90V[\x12a\x18\x98W[PPa\x16#\x81\x87a\x16\x06\x85`\x04\x01a#\x93V[a\x16\x1Da\x03\xAC\x89a\x08)`D\x8A\x01\x8A`\x04\x01a$\xB9V[\x90a28V[a\x16ta\x16^` a\x16Oa\x03\xAC\x88a\x08)a\x16A\x89`\x04\x01a#\x93V[\x98`D\x81\x01\x90`\x04\x01a$\xB9V[\x93\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92`@Q\x93\x84\x93\x8Dc\xFF\xFF\xFF\xFFB\x16\x93\x86a)\x1BV[\x03\x90\xA1a\x16\xBCa\x07/a\x16\xA5a\x16\x8C\x88`\x04\x01a#\x93V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x97` R`@\x90 \x90V[a\x07\x1Ba\x03\xAC\x85a\x08)`D\x8C\x01\x8C`\x04\x01a$\xB9V[\x91a\x16\xC7\x81\x84a(\xBEV[\x90a\x17\n\x82\x84a\x17\x05\x8Aa\x07\x1Ba\x03\xACa\x16\xF7a\x16\xE9a\x16\x8C\x85`\x04\x01a#\x93V[\x93`D\x81\x01\x90`\x04\x01a$\xB9V[Bc\xFF\xFF\xFF\xFF\x16\x96\x91a#~V[a:8V[PP\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\a\x179\x88`\x04\x01a#\x93V[a\x17Pa\x03\xAC\x86a\x08)`D\x8D\x01\x8D`\x04\x01a$\xB9V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x85\x16\x91\x81\x01\x91\x90\x91R``\x90\xA1a\x17\x8B\x87`\x04\x01a#\x93V[\x91a\x17\xA3a\x03\xAC\x85a\x08)`D\x8C\x01\x8C`\x04\x01a$\xB9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16;\x15a\x01\xFAW`@Qc\xA5z\xB1\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x16`$\x84\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x85\x01R\x16`d\x83\x01R\x90\x92\x90_\x84\x80`\x84\x81\x01\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\x05\x10W`\x01`\x01`@\x1B\x03\x80\x8C\x92`\x01\x97a\x18m\x96a\x18~W[P\x16\x92\x16a9\x06V[a\x18w\x82\x88a#\xA8V[R\x01a\x14\xD0V[\x80a\x18\x8C_a\x18\x92\x93a\x0B\x94V[\x80a\n\xFBV[_a\x18dV[a\x18\xD5a\x18\xC6a\x07Ga\x07Ga\x18\xEC\x95a\x15xa\x18\xBAa\x18\xBAa\x18\xE2\x98a(\xDEV[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16`\x0F\x0B\x90V[`@\x84\x01Q`\x0F\x0Ba(\xF5V[`\x0F\x0B`@\x83\x01RV[\x82\x8Aa\x18\xFA\x84`\x04\x01a#\x93V[a\x19Pa\x19\x14a\x03\xAC\x89a\x08)`D\x8A\x01\x8A`\x04\x01a$\xB9V[a\x198a\x19+` \x88\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x88\x01Q`\x0F\x0Ba\t1V[``\x87\x01Qc\xFF\xFF\xFF\xFF\x16\x91`@Q\x95\x86\x95\x86a)\x1BV[\x03\x90\xA1\x86_a\x15\xF3V[\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5\x86\x86a\n\xB1a\x19\xB1\x88a\x19\x90\x81`\x04\x01a#\x93V[\x93a\x19\xA1`D\x83\x01\x83`\x04\x01a$\xB9V[\x93\x90\x92`\x84\x81\x01\x90`\x04\x01a)dV[\x93\x90\x92`@Q\x97\x88\x97\x88a)\xF9V[a\x19\xD9\x91P` =` \x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x14\x9EV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x13\xCDV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xFAWa\x1AC6a\x0E\xBAV[a\x1Aqa\x1Ad\x82a\x11\x81\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[T`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x1A\xA3a\x1A\x93\x82a\x11\x81\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x03\x90V[_\x90[\x80\x82\x10a\x1A\xF0W[a\x03da\x1A\xD6\x85a\x15\x99a\x07/\x87a\x11\x81\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x97` R`@_ \x90V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x90\x92a\x1BSa\x1BNa\x1B\x1F\x86a\x1B\x1A\x87a\x11\x81\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a6\0V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x99` R`@\x90 a\x1BA\x90\x87\x90a\x11\x81V[\x90_R` R`@_ \x90V[a*\x8AV[a\x1Bpa\x1Bg`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10a\x1B\x91W` \x01Q`\x01\x91a\x1B\x89\x91`\x0F\x0Ba\t1V[\x93\x01\x90a\x1A\xA6V[P\x92a\x1A\xAEV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAWa\x1B\xB0a6\xBEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x1CP\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1Co\x906\x90`\x04\x01a\x01\xFEV[\x91`D5\x91a\x1C}\x83a\x0B\xFBV[a\x1C\x86\x84a(@V[\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\xFF\xFF\xFF\xFF\x16\x91_[\x81\x81\x10a\x1C\xB1W`@Q\x80a\x03d\x88\x82a\x10\xA9V[\x84_R`\x97` Ra\x1C\xCB`@_ a\x11v\x83\x85\x87a#~V[\x80T\x90_[\x82\x81\x10a\x1DBWP`\x01\x92\x91\x90\x81a\x1D\x0FWPPa\x1D\tg\r\xE0\xB6\xB3\xA7d\0\0[a\x1C\xFB\x83\x8Aa#\xA8V[\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x01a\x1C\x9CV[a\x1D-a\x1D=\x91a\x1D\"a\x1D\t\x94a1XV[\x90_R` _ \x01\x90V[T` \x1C`\x01`\x01`@\x1B\x03\x16\x90V[a\x1C\xF1V[\x91\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x1D\x91W\x82_R\x87c\xFF\xFF\xFF\xFFa\x1Ds\x84` _ \x01c\xFF\xFF\xFF\xFF\x90T\x16\x90V[\x16\x11\x15a\x1D\x81WP\x91a\x1C\xD0V[\x92Pa\x1D\x8C\x90a4\x1CV[a\x1C\xD0V[a%\xEBV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW` `\x01`\x01`@\x1B\x03a\x1E)a\x1E\x016a\x0E\xBAV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`@c\xFF\xFF\xFF\xFFa\x1E_`\x045a\x1EZ\x81a\x04`V[a*\xF0V[\x83Q\x91\x15\x15\x82R\x90\x91\x16` \x82\x01R\xF3[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x1E\xDFWPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x1F\x1FWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x1E\xD0V[\x90\x91\x92` \x80`\x01\x92\x86Q\x81R\x01\x94\x01\x92\x01\x90a\x1F\0V[\x90\x91a\x1FNa\x0C\xB9\x93`@\x84R`@\x84\x01\x90a\x1E\xB4V[\x91` \x81\x84\x03\x91\x01Ra\x1E\xB4V[4a\x01\xFAW`\xA06`\x03\x19\x01\x12a\x01\xFAWa\x1Fv6a\x01\xE9V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1F\x95\x906\x90`\x04\x01a\x01\xFEV[`d\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1F\xB7\x906\x90`\x04\x01a\x01\xFEV[a\x1F\xE5a\x03\xBFc\xFF\xFF\xFF\xFF`\x84\x95\x94\x955a\x1F\xD1\x81a\x0B\xFBV[\x16\x95a\x1F\xDEB\x88\x11a+\x90V[6\x90a\x0C\tV[\x94`@Q\x94cxps;`\xE1\x1B\x86R_\x86\x80a \x07\x86\x89\x89\x88`\x04\x86\x01a,\x94V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05\x10W_\x96a!\xD0W[Pa O\x84\x92\x94a#!V[\x96g\r\xE0\xB6\xB3\xA7d\0\0\x92_[\x81\x81\x10a rW`@Q\x80a\x03d\x8C\x8C\x83a\x1F7V[a \x80a\x03\xAC\x82\x84\x8Aa#~V[a \x89\x87a(@V[a \x93\x83\x8Da#\xA8V[Ra \x9E\x82\x8Ca#\xA8V[P_\x8B\x87\x87\x8D\x88\x8C\x8F[\x81\x88\x10a \xC0WPPPPPPPPP`\x01\x01a \\V[\x88a!\x88a\x07Ga\x07/a\x11\x81a\x11\x81\x8F\x97\x8E\x9D\x9C`\x01\x9F\x9B\x9C\x8C\x8Ba!\x94\x9Fa!\x1Da!^\x9Fa\x1BN\x90a!\x8E\x9F\x8Fa!\x04a\x03\xAC\x89a!m\x9Da\x1BA\x95a#~V[\x9C\x8D\x91`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x80Q`\x01`\x01`@\x1B\x03\x16\x94c\xFF\xFF\xFF\xFFa!?`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15a!\xA2W[P\x91a!^a!d\x92`\x01`\x01`@\x1B\x03\x94a#\xA8V[Qa#\xA8V[Q\x91\x16\x90a9\x06V[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16_\x90\x81R`\x97` R`@\x90 \x90V[\x90a8fV[\x94a#\xA8V[R\x01\x8B\x87\x87\x8D\x88\x8C\x8Fa \xA8V[a!d\x92a!\xC5`\x01`\x01`@\x1B\x03\x95\x93\x96a\t1` a!^\x95\x01Q`\x0F\x0B\x90V[\x95\x92\x94P\x92Pa!GV[a!\xED\x91\x96P=\x80_\x83>a!\xE5\x81\x83a\x0B\x94V[\x81\x01\x90a+\xA6V[\x94_a CV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045a\"\x11\x81a\x04`V[a\"\x19a6\xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\"1Wa\x04\xDF\x90a3\xBEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\"\xDE\x91_\x91a\x04\xE1WP`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDCV[a\"\xEF`fT\x19\x82\x19\x81\x16\x14a$ V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[\x90a#+\x82a\x0B\xE4V[a#8`@Q\x91\x82a\x0B\x94V[\x82\x81R` \x81\x93a#K`\x1F\x19\x91a\x0B\xE4V[\x01\x91\x01_[\x82\x81\x10a#\\WPPPV[``\x82\x82\x01R` \x01a#PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x05\x1B\x01\x90V[a#jV[5a\x0C\xB9\x81a\x04`V[a\x0C\xB9\x906\x90a\x0C\tV[\x80Q\x82\x10\x15a#\x8EW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x01\xFAWQa\x0C\xB9\x81a\x04`V[`@Q=_\x82>=\x90\xFD[\x15a#\xE3WV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x01\xFAWQ\x80\x15\x15\x81\x03a\x01\xFAW\x90V[\x15a$\x11WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a$'WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[\x15a$=WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a$SWV[c\xFAU\xFC\x81`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x05\x1B\x81\x015\x90`~\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x01\xFAWV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xFAWV[\x15a$\xF5WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF` \x80\x92\x805a%\x18\x81a\x04`V[`\x01`\x01`\xA0\x1B\x03\x16\x85R\x015a%.\x81a\x0B\xFBV[\x16\x91\x01RV[` \x80\x82R\x81\x01\x83\x90R`@\x01\x91_[\x81\x81\x10a%QWPPP\x90V[\x90\x91\x92`@\x80\x82a%d`\x01\x94\x88a%\x04V[\x01\x94\x01\x92\x91\x01a%DV[\x15a%vWV[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[5`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01\xFAW\x90V[\x15a%\xA0WV[c:?\xA0c`\xE2\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x06\x1B\x01\x90V[\x15a%\xC6WV[c\r\x8F\xCB\xE3`\xE4\x1B_R`\x04_\xFD[\x15a%\xDCWV[cF\x06\x17\x93`\xE1\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x1D\x91WV[\x15a& WV[c2\x9DNS`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`\xC0\x81\x01\x95\x94\x90\x93`\xA0\x93c\xFF\xFF\xFF\xFF\x93`\x01`\x01`@\x1B\x03\x92\x91\x90a&f\x90` \x89\x01\x90a%\x04V[`\x01\x80\x87\x1B\x03\x16``\x87\x01R\x16`\x80\x85\x01R\x16\x91\x01RV[\x15a&\x85WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`eTa\x0B\xC4\x93\x91\x92a'>\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a'CW[a'\t\x90a.\xE8V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a.\xFEV[a3\xBEV[P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15a'\0V[\x90\x92\x91\x92\x83Q\x93a'e\x85a\x0B\xE4V[\x94a's`@Q\x96\x87a\x0B\x94V[\x80\x86Ra'\x82`\x1F\x19\x91a\x0B\xE4V[\x01_[\x81\x81\x10a'\xF2WPP_[\x81Q\x81\x10\x15a'\xECW\x80a'\xB9a'\xB2a'\xAC`\x01\x94\x86a#\xA8V[Qa,\xF1V[\x85\x87a-\x89V[a'\xD0a\x03\xE1` \x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a'\xDA\x82\x89a#\xA8V[Ra'\xE5\x81\x88a#\xA8V[P\x01a'\x90V[PPPPV[` \x90`@Qa(\x01\x81a\x0ByV[_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x8A\x01\x01R\x01a'\x85V[\x15a(\"WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[5a\xFF\xFF\x81\x16\x81\x03a\x01\xFAW\x90V[\x90a(J\x82a\x0B\xE4V[a(W`@Q\x91\x82a\x0B\x94V[\x82\x81R\x80\x92a(h`\x1F\x19\x91a\x0B\xE4V[\x01\x90` 6\x91\x017V[\x15a(yWV[c\x13S`1`\xE0\x1B_R`\x04_\xFD[5a\x0C\xB9\x81a\x0B\xFBV[\x15a(\x99WV[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[\x15a(\xAFWV[cN\x99\xE6\xCF`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x1D\x91WV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a\x1D\x91W_\x03\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a\x1D\x91WV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`@\x83\x01R`\xC0\x82\x01\x95\x94\x91\x93`\xA0\x93\x91\x92\x91`\x01`\x01`@\x1B\x03\x91\x90a&fV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x816\x03\x83\x13a\x01\xFAWV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a)\xAFWPPP\x90V[\x90\x91\x92` \x80`\x01\x92\x865a)\xC3\x81a\x04`V[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x94\x01\x91\x01\x91\x90\x91a)\xA2V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x95\x94\x92\x93\x91a*>\x91\x90`\xC0``\x89\x01R`\xC0\x88\x01\x91a)\x96V[\x94\x80\x86\x03`\x80\x82\x01R` \x80\x85Q\x97\x88\x81R\x01\x94\x01_\x96[\x80\x88\x10a*rWPPa\x0C\xB9\x94\x95P`\xA0\x81\x85\x03\x91\x01Ra)\xD9V[\x90\x94` \x80`\x01\x92\x88Q\x81R\x01\x96\x01\x97\x01\x96\x90a*VV[\x90`@Qa*\x97\x81a\x0ByV[`@c\xFF\xFF\xFF\xFF\x82\x94T`\x01`\x01`@\x1B\x03\x81\x16\x84R\x80\x83\x1C`\x0F\x0B` \x85\x01R`\xC0\x1C\x16\x91\x01RV[\x90`@Qa*\xCE\x81a\x0ByV[`@c\xFF\xFF\xFF\xFF\x82\x94T\x81\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R\x82\x1C\x16\x91\x01RV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@\x80_ a+B\x82Q\x91a+\x13\x83a\x0ByV[Tc\xFF\xFF\xFF\xFF\x81\x16\x92\x83\x81Rc\xFF\xFF\xFF\xFF\x80\x83` \x1C\x16\x92\x83` \x84\x01R\x86\x1C\x16\x94\x85\x91\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x91c\xFF\xFF\xFF\xFF\x83\x16\x15\x15\x90\x81a+wW[P\x15a+iWP\x90[c\xFF\xFF\xFF\xFF\x82\x16\x15\x15\x91\x90V[c\xFF\xFF\xFF\xFF\x16\x90P\x90a+\\V[a+\x87\x91Pc\xFF\xFF\xFF\xFF\x16a\x1BgV[B\x10\x15_a+SV[\x15a+\x97WV[c\xB7\xD0\x94\x97`\xE0\x1B_R`\x04_\xFD[` \x81\x83\x03\x12a\x01\xFAW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xFAW\x81Q\x90a+\xDA\x82a\x0B\xE4V[\x92a+\xE8`@Q\x94\x85a\x0B\x94V[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x90\x82\x82\x11a\x01\xFAW` \x81\x01\x93[\x82\x85\x10a,\x14WPPPPP\x90V[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAW\x82\x01\x84`?\x82\x01\x12\x15a\x01\xFAW` \x81\x01Q\x90a,@\x82a\x0B\xE4V[\x91a,N`@Q\x93\x84a\x0B\x94V[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x87\x83\x11a\x01\xFAW`@\x01\x90[\x82\x82\x10a,\x84WPPP\x81R` \x94\x85\x01\x94\x01a,\x05V[\x81Q\x81R` \x91\x82\x01\x91\x01a,lV[\x94\x93\x91\x94\x92\x90\x92\x85`@\x82\x01`@\x83RR``\x81\x01\x93_\x96[\x80\x88\x10a,\xC9WPPa\x0C\xB9\x94\x95P` \x81\x85\x03\x91\x01Ra)\x96V[\x90\x94` \x80`\x01\x92\x885a,\xDC\x81a\x04`V[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x96\x01\x97\x01\x96\x90a,\xADV[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra-7`@\x82a\x0B\x94V[Q\x90Q\x90` \x81\x10a-GWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@R_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x90a\x1Ada-\xDC\x91a\x11\x81a-\xC1a\x1BNa-\xA2a-UV[\x97a\x1BA\x85a\x11\x81\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16_\x90\x81R`\x98` R`@\x90 \x90V[\x90a-\xEE`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16B\x10a.yWP\x80a.7a.'` a.\x19a.V\x95Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93\x01\x92a\t1\x84Q`\x0F\x0B\x90V[`\x01`\x01`@\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x83\x16\x84R_``\x85\x01R_`@\x85\x01RQ`\x0F\x0B\x90V[\x90_a.b\x83`\x0F\x0B\x90V[\x12a.lWPP\x90V[a\x0C\xB9\x91a\x15\xC6\x91a1\xD2V[\x90a\x0C\xB9\x92\x93P\x80a.\xD1a.\xA6` a.\x9Da.\xDB\x95Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93\x01Q`\x0F\x0B\x90V[\x91a.\xC1a.\xB2a\x0B\xD5V[`\x01`\x01`@\x1B\x03\x90\x98\x16\x88RV[`\x01`\x01`@\x1B\x03\x16` \x87\x01RV[`\x0F\x0B`@\x85\x01RV[c\xFF\xFF\xFF\xFF\x16``\x83\x01RV[\x15a.\xEFWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16a/\x12\x81\x15\x15a.\xE8V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[_\x19\x81\x14a\x1D\x91W`\x01\x01\x90V[\x80\x15a\x1D\x91W_\x19\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x93\x92\x91\x90a/\xAE\x90a\x1A\x93V[\x93[\x84\x15\x15\x80a0eW[\x15a0^Wa/\xE4a/\xDF\x84a\x11\x81\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a7\x16V[a/\xEF\x81\x85\x85a-\x89V[\x90a0\x04a\x1Bg``\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10a0UW\x91a0\x1Da0I\x92a0O\x94\x87\x87a28V[a0Ca0>\x86a\x11\x81\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a7WV[Pa/`V[\x94a/nV[\x93a/\xB0V[PPPPP\x90PV[PPP\x90PV[Pa\xFF\xFF\x81\x10a/\xB9V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x94\x90a0\xA2\x90a\x1A\x93V[\x94[\x85\x15\x15\x80a1KW[\x15a1CWa0\xD3a/\xDF\x85a\x11\x81\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a0\xDE\x81\x86\x86a-\x89V[\x90a0\xF3a\x1Bg``\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10a19W\x91a1\x0Ca1-\x92a13\x94\x88\x88a28V[a0Ca0>\x87a\x11\x81\x88`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x95a/nV[\x94a0\xA4V[PPP\x93PPPPV[P\x93PPPPV[Pa\xFF\xFF\x85\x16\x81\x10a0\xADV[_\x19\x81\x01\x91\x90\x82\x11a\x1D\x91WV[\x80T\x80a1{WPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x80_\x19\x81\x01\x11a\x1D\x91W`\x01`\x01`@\x1B\x03\x91_R_\x19\x90` _ \x01\x01T` \x1C\x16\x90V[`\x01`\x01`@\x1B\x03\x80\x91\x16`\x0F\x0B\x91\x16`\x0F\x0B\x03`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17a\x1D\x91W\x90V[`\x01`\x01`@\x1B\x03\x91\x82a1\xE9\x92\x16`\x0F\x0Ba(\xF5V[\x16\x90V[\x90\x81T`\x80\x1D\x90a2\x0C\x82`\x01\x85\x01\x90`\x0F\x0B_R` R`@_ \x90V[U\x81T`\x01`\x01`\x80\x1B\x03\x16`\x01\x90\x91\x01`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[\x90\x91a3\x86\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x94a3(a3\xB9\x93a2\xC8`\x01`\x01`@\x1B\x03` \x85\x01Q\x16\x91c\xFF\xFF\xFF\xFF`@\x86\x01Q`\x0F\x0B\x81``\x88\x01Q\x16\x90`@Q\x95a2\x9A\x87a\x0ByV[\x86R`\x0F\x0B` \x86\x01R\x16`@\x84\x01Ra\x1BA\x89a\x11\x81\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x81Q\x81T` \x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x91\x90\x91\x1B\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x95\x1B\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x91\x90\x91\x17\x90UV[a3ya3<\x82Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x98` R`@\x90 a3^\x90\x88\x90a\x11\x81V[\x90`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x81\x90``\x82\x01\x90V[\x03\x90\xA1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x15a4\rWV[c\xDD\x18\x18\x15`\xE0\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x1D\x91WV[\x91\x90\x82\x01\x80\x92\x11a\x1D\x91WV[\x90\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91a4kc\xFF\xFF\xFF\xFF\x83\x16\x15\x15a4\x06V[a3\xB9a5\x89a4\x93a4\x8E\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90V[a*\xC1V[a4\xC9\x85` \x83\x01a4\xA9\x81Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x80a5\xC8W[a5\xBAW[P\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a5\x7Fa4\xFFa\x1Bgc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba4*V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01\x90\x81R\x91`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9B` R`@\x90 \x81Q\x81Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x81U\x90` \x81\x81\x01Q\x83T`@\x93\x84\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x91\x90\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x91\x1Bk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x85\x01R\x93\x16\x92\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[c\xFF\xFF\xFF\xFF\x16\x84R_a4\xBDV[Pa5\xDDa\x1Bg`@\x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a4\xB8V[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x1D\x91WV[\x80T\x90\x91`\x01`\x01`\xFF\x1B\x03\x81\x11a6hWa\x07\xEDa6-a6(a64\x93\x85`\x0F\x0Ba5\xE5V[a;XV[\x92`\x80\x1D\x90V[\x81`\x0F\x0B\x12\x15a6YW`\x01a6U\x92\x01\x90`\x0F\x0B_R` R`@_ \x90V[T\x90V[c-\x04\x83\xC5`\xE2\x1B_R`\x04_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a6\xD2WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a7+\x81T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[a7HW\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[c\x1E\xD9P\x95`\xE1\x1B_R`\x04_\xFD[\x90a7m\x82T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[a7HW\x81T`\x0F\x0B\x91`\x01\x81\x01\x92_a7\xA8\x82a7\x96\x81\x88\x90`\x0F\x0B_R` R`@_ \x90V[T\x96\x90`\x0F\x0B_R` R`@_ \x90V[U`\x01`\x01`\x01`\x80\x1B\x03\x19\x83T\x16\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x17\x90UV[\x81\x15a7\xD1W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\x01\xFAWV[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a8UW\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x01\xFAW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14a8\xFAWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91a8\xA6\x86\x84\x11a7\xE5V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x0C\xB9\x92Pa7\xC7V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a90W\x90\x82\x91a8\xA6\x86\x84\x11a7\xE5V[PP\x90a\x0C\xB9\x92Pa7\xC7V[\x90\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0BtW`\x01\x81\x01\x80\x84U\x81\x10\x15a#\x8EW` `\x01`\x01`@\x1B\x03\x91a\x0B\xC4\x94_R\x81_ \x01\x92a9\x96c\xFF\xFF\xFF\xFF\x82Q\x16\x85\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[\x01Q\x82Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x91\x16` \x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90UV[\x90`@Qa9\xCE\x81a\x0BYV[` `\x01`\x01`@\x1B\x03\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x82\x1C\x16\x91\x01RV[\x15a9\xF3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x92\x91\x92\x83\x82T\x80\x15\x15_\x14a;.W\x92` \x92\x91\x84a:pa:ka:`a:\xF4\x98a1XV[\x85_R` _ \x01\x90V[a9\xC1V[\x93c\xFF\xFF\xFF\xFFa:\x95a:\x87\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x82\x84\x16\x92\x83\x91\x16\x11\x15a9\xECV[a:\xA6a\x1Bg\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x03a:\xF8WPa:\xE6\x92a\x1D\"a:\xBC\x92a1XV[\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x90V[\x91PPa;)\x91a;\x16a;\na\x0B\xC6V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`@\x1B\x03\x88\x16\x82\x86\x01Ra9=V[a:\xE6V[PPa;S\x91a;?a;\na\x0B\xC6V[`\x01`\x01`@\x1B\x03\x85\x16` \x83\x01Ra9=V[_\x91\x90V[`\x01`\x01`\x7F\x1B\x03\x19\x81\x12\x15\x80a;\xCAW[\x15a;uW`\x0F\x0B\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[P`\x01`\x01`\x7F\x1B\x03\x81\x13\x15a;jV\xFE\xA2dipfsX\"\x12 )\xA6\xB3\xD1\x8F\xD75\xAF\x7F2\xC8\xFC\xF7v\\\x90\n\xC2\xA6\xF8\xFCt\xB5\xD7\xAB\x04\x1C\xCB\xEAnV\xCAdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c80630b002119146101e457806310d67a2f146101df578063136439dd146101da5780631637b60f146101d55780631794bb3c146101d05780632981eb77146101cb57806335af054a146101c65780634b5046ef146101c15780634d9dbde9146101bc578063547afb87146101b757806356c483e6146101b2578063595c6a67146101ad5780635ac86ab7146101a85780635c489bb5146101a35780635c975abb1461019e57806360db99a3146101995780636b3aa72e146101945780636cfb44811461018f578063715018a61461018a5780637bc1ef6114610185578063843b349f14610180578063886f11951461017b5780638da5cb5b14610176578063a984eb3a14610171578063b9fbaed11461016c578063df5cf72314610167578063e07d2b1214610162578063f2fde38b1461015d5763fabc1cbc14610158575f80fd5b612285565b6121f4565b611f5c565b611e70565b611e33565b611de6565b611dbe565b611d96565b611c33565b611bf3565b611b98565b611a35565b6119f1565b61137d565b611360565b6112dc565b6112a9565b611223565b6111b2565b6110eb565b610f4a565b610d77565b610cbc565b610b05565b6109f3565b6105dc565b610515565b610471565b6102e8565b60409060031901126101fa57600490565b5f80fd5b9181601f840112156101fa578235916001600160401b0383116101fa576020808501948460051b0101116101fa57565b602081016020825282518091526040820191602060408360051b8301019401925f915b83831061026057505050505090565b9091929394603f19828203018352855190602080835192838152019201905f905b8082106102a05750505060208060019297019301930191939290610251565b90919260206102de60019286519063ffffffff60406060936001600160401b0381511684526020810151600f0b602085015201511660408201520190565b9401920190610281565b346101fa5760803660031901126101fa57610302366101e9565b6044356001600160401b0381116101fa576103219036906004016101fe565b91906064356001600160401b0381116101fa576103429036906004016101fe565b9161034c83612321565b945f5b8481106103685760405180610364898261022e565b0390f35b5f5b82811061037a575060010161034f565b94600186610452816103ca888c6103c4879f9b9a999d6103b78f8f928d6103b16103ac8f946103ac956103bf9861237e565b612393565b9761237e565b923690610c09565b612cf1565b91612d89565b6104366103e160208301516001600160401b031690565b9161042961040460606103f86040850151600f0b90565b93015163ffffffff1690565b9161041f610410610bb5565b6001600160401b039096168652565b600f0b6020850152565b63ffffffff166040830152565b6104408b8a6123a8565b519061044c83836123a8565b526123a8565b50019095919293975061036a565b6001600160a01b038116036101fa57565b346101fa5760203660031901126101fa576004803561048f81610460565b60655460405163755b36bd60e11b81529260209184919082906001600160a01b03165afa918215610510576104df926104da915f916104e1575b506001600160a01b031633146123dc565b612efe565b005b610503915060203d602011610509575b6104fb8183610b94565b8101906123bc565b5f6104c9565b503d6104f1565b6123d1565b346101fa5760203660031901126101fa5760043560655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa80156105105761056c915f916105ad575b5061240a565b61057b60665482811614612420565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b6105cf915060203d6020116105d5575b6105c78183610b94565b8101906123f2565b5f610566565b503d6105bd565b346101fa5760203660031901126101fa576004356001600160401b0381116101fa5761060f6106389136906004016101fe565b9190610628610622600180606654161490565b15612436565b61063133612af0565b929061244c565b5f905b83821061064457005b61064f828583612462565b61067761065f6040830183612484565b905061066e60608401846124b9565b919050146124ee565b6106a560206106896040840184612484565b604051631062711b60e11b815293849283929060048401612534565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610510576106ea915f916109d5575b5061256f565b6106fc6106f682612393565b33612f7a565b335f9081526097602052604090206107349061072f9061071b84612393565b60018060a01b03165f5260205260405f2090565b613166565b9161076361075361074760208501612585565b6001600160401b031690565b6001600160401b03851614612599565b5f5b6107726040840184612484565b90508110156109c557807f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66107c16103bf6107bc6001956107b660408a018a612484565b906125af565b61239d565b6108ee6107d7826107d189612393565b33612d89565b80926107f96107f36107ed6040850151600f0b90565b600f0b90565b156125bf565b61084461083a8a61083461082f8a61082961081e60208a01516001600160401b031690565b9460608101906124b9565b9061237e565b612585565b906131a1565b600f0b6040840152565b61085f6108586107ed6040850151600f0b90565b15156125d5565b6108706107ed6040840151600f0b90565b5f81121561095a57506108b96108ac7f000000000000000000000000000000000000000000000000000000000000000063ffffffff42166125ff565b63ffffffff166060840152565b335f908152609a602052604090206108df9082906108da9061071b8d612393565b6131ed565b6108e889612393565b33613238565b6108ff836107b66040890189612484565b61095161090b88612393565b92610943606061093761092860208501516001600160401b031690565b6040850151600f0b5b906131d2565b92015163ffffffff1690565b90604051948594338661262f565b0390a101610765565b5f12156108df576109746108ac8d63ffffffff42166125ff565b61099861098b61092884516001600160401b031690565b6001600160401b03168352565b6109c06001600160401b038b166109b961074785516001600160401b031690565b1115612619565b6108df565b50600190930192915061063b9050565b6109ed915060203d81116105d5576105c78183610b94565b5f6106e4565b346101fa5760603660031901126101fa57600435610a1081610460565b610a6e602435610a1f81610460565b604435905f5493610a54610a3e610a3a8760ff9060081c1690565b1590565b80968197610aed575b8115610acd575b5061267e565b84610a65600160ff195f5416175f55565b610ab6576126e1565b610a7457005b610a8261ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081015b0390a1005b610ac861010061ff00195f5416175f55565b6126e1565b303b15915081610adf575b505f610a4e565b60ff1660011490505f610ad8565b600160ff8216109150610a47565b5f9103126101fa57565b346101fa575f3660031901126101fa57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b03821117610b7457604052565b610b45565b606081019081106001600160401b03821117610b7457604052565b90601f801991011681019081106001600160401b03821117610b7457604052565b60405190610bc4606083610b94565b565b60405190610bc4604083610b94565b60405190610bc4608083610b94565b6001600160401b038111610b745760051b60200190565b63ffffffff8116036101fa57565b91908260409103126101fa57604051610c2181610b59565b60208082948035610c3181610460565b8452013591610c3f83610bfb565b0152565b90602080835192838152019201905f5b818110610c605750505090565b9091926020610c9e60019286519063ffffffff60406060936001600160401b0381511684526020810151600f0b602085015201511660408201520190565b9401929101610c53565b906020610cb9928181520190610c43565b90565b346101fa5760603660031901126101fa57600435610cd981610460565b602435610ce581610460565b604435906001600160401b0382116101fa57366023830112156101fa57816004013592610d1184610be4565b92610d1f6040519485610b94565b8484526024602085019560061b820101903682116101fa57602401945b818610610d5d57610364610d51868686612755565b60405191829182610ca8565b6020604091610d6c3689610c09565b815201950194610d3c565b346101fa5760603660031901126101fa57600435610d9481610460565b6024356001600160401b0381116101fa57610db39036906004016101fe565b6044929192356001600160401b0381116101fa57610dd59036906004016101fe565b9290610de8610622600180606654161490565b610df38484146124ee565b6040516336b87bd760e11b81526001600160a01b0383166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561051057610e5c915f91610e9b575b5061281b565b5f5b838110610e6757005b80610e95610e7b6103ac600194888b61237e565b610e8e610e89848a8861237e565b612831565b9086613070565b01610e5e565b610eb4915060203d6020116105d5576105c78183610b94565b5f610e56565b60409060031901126101fa57600435610ed281610460565b90602435610cb981610460565b9291604084019360408152825180955260206060820193015f955b808710610f16575050610cb99394506020818403910152610c43565b815180516001600160a01b0316865260209081015163ffffffff168187015291949091600191604001950196019590610efa565b346101fa57610f5836610eba565b6040516316ae76cb60e01b81526001600160a01b0380841660048301525f602483018190525f19604484015291939190849060649082907f0000000000000000000000000000000000000000000000000000000000000000165afa928315610510575f93610fdd575b50610fcd918391612755565b9061036460405192839283610edf565b92503d805f853e610fee8185610b94565b8301926020818503126101fa578051906001600160401b0382116101fa57019280601f850112156101fa5783519361102585610be4565b916110336040519384610b94565b85835260208084019660061b830101918183116101fa57602001955b82871061106657505050610fcd9293509291610fc1565b6040878303126101fa576020604091825161108081610b59565b895161108b81610460565b8152828a015161109a81610bfb565b8382015281520196019561104f565b60206040818301928281528451809452019201905f5b8181106110cc5750505090565b82516001600160401b03168452602093840193909201916001016110bf565b346101fa5760403660031901126101fa5760043561110881610460565b6024356001600160401b0381116101fa576111279036906004016101fe565b919061113283612840565b926001600160a01b03909216915f5b818110611156576040518061036487826110a9565b600190845f52609760205261119661072f60405f2061117684878961237e565b359061118182610460565b9060018060a01b03165f5260205260405f2090565b6001600160401b036111a883896123a8565b9116905201611141565b346101fa5760403660031901126101fa576004356111cf81610460565b6024356111db81610bfb565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163303611214576104df91613437565b63f739589b60e01b5f5260045ffd5b346101fa575f3660031901126101fa5760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa801561051057611275915f916105ad575061240a565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b346101fa5760203660031901126101fa5760043560ff81168091036101fa5760016020911b806066541614604051908152f35b346101fa5760203660031901126101fa576004356112f981610bfb565b6040516336b87bd760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa918215610510576104df9261135a915f91610e9b575061281b565b33613437565b346101fa575f3660031901126101fa576020606654604051908152f35b346101fa5760203660031901126101fa576004356001600160401b0381116101fa5760a060031982360301126101fa576113be610622600280606654161490565b606481013590811515806119df575b6113d690612872565b6113e260248201612888565b916113ff6113ee610bc6565b3381529363ffffffff166020850152565b61140883612cf1565b9161145f60208561141b84600401612393565b6040516309a961f360e11b81526001600160a01b0391821660048201528251909116602482015260209091015163ffffffff16604482015291829081906064820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610510576114a7915f916119c0575b50949394612892565b6114c16114ba60448301836004016124b9565b9050612840565b670de0b6b3a7640000949092905f5b6114e060448501856004016124b9565b905081101561195a57611516826114f986600401612393565b6115106103ac8561082960448b018b6004016124b9565b90612d89565b9061153861153161074760208501516001600160401b031690565b15156128a8565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf68561157d6107478761157861074760208901516001600160401b031690565b6137ec565b936115ae61159e8661159960208501516001600160401b031690565b6128be565b6001600160401b03166020830152565b6115d36115c68661159984516001600160401b031690565b6001600160401b03168252565b866115e26040830151600f0b90565b5f6115ed82600f0b90565b12611898575b5050611623818761160685600401612393565b61161d6103ac8961082960448a018a6004016124b9565b90613238565b61167461165e602061164f6103ac8861082961164189600401612393565b9860448101906004016124b9565b9301516001600160401b031690565b926040519384938d63ffffffff4216938661291b565b0390a16116bc61072f6116a561168c88600401612393565b6001600160a01b03165f90815260976020526040902090565b61071b6103ac8561082960448c018c6004016124b9565b916116c781846128be565b9061170a82846117058a61071b6103ac6116f76116e961168c85600401612393565b9360448101906004016124b9565b4263ffffffff16969161237e565b613a38565b50507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c61173988600401612393565b6117506103ac8661082960448d018d6004016124b9565b604080516001600160a01b0393841681529190921660208201526001600160401b03851691810191909152606090a161178b87600401612393565b916117a36103ac8561082960448c018c6004016124b9565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03163b156101fa5760405163a57ab10b60e01b81526001600160a01b039485166004820152931660248401526001600160401b0380861660448501521660648301529092905f8480608481010381837f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165af1918215610510576001600160401b03808c9260019761186d9661187e575b50169216613906565b61187782886123a8565b52016114d0565b8061188c5f61189293610b94565b80610afb565b5f611864565b6118d56118c66107476107476118ec956115786118ba6118ba6118e2986128de565b6001600160801b031690565b6001600160801b0316600f0b90565b6040840151600f0b6128f5565b600f0b6040830152565b828a6118fa84600401612393565b6119506119146103ac8961082960448a018a6004016124b9565b61193861192b60208801516001600160401b031690565b6040880151600f0b610931565b606087015163ffffffff16916040519586958661291b565b0390a1865f6115f3565b7f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe58686610ab16119b18861199081600401612393565b936119a160448301836004016124b9565b9390926084810190600401612964565b939092604051978897886129f9565b6119d9915060203d6020116105d5576105c78183610b94565b5f61149e565b50670de0b6b3a76400008211156113cd565b346101fa575f3660031901126101fa576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101fa57611a4336610eba565b611a71611a64826111818560018060a01b03165f52609860205260405f2090565b546001600160401b031690565b90611aa3611a93826111818660018060a01b03165f52609a60205260405f2090565b5480600f0b9060801d600f0b0390565b5f905b808210611af0575b610364611ad68561159961072f876111818b60018060a01b03165f52609760205260405f2090565b6040516001600160401b0390911681529081906020820190565b9092611b53611b4e611b1f86611b1a876111818b60018060a01b03165f52609a60205260405f2090565b613600565b6001600160a01b0388165f908152609960205260409020611b41908790611181565b905f5260205260405f2090565b612a8a565b611b70611b67604083015163ffffffff1690565b63ffffffff1690565b4210611b915760200151600191611b8991600f0b610931565b930190611aa6565b5092611aae565b346101fa575f3660031901126101fa57611bb06136be565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b346101fa575f3660031901126101fa57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b346101fa5760603660031901126101fa57600435611c5081610460565b6024356001600160401b0381116101fa57611c6f9036906004016101fe565b9160443591611c7d83610bfb565b611c8684612840565b936001600160a01b039092169263ffffffff16915f5b818110611cb1576040518061036488826110a9565b845f526097602052611ccb60405f2061117683858761237e565b8054905f5b828110611d425750600192919081611d0f575050611d09670de0b6b3a76400005b611cfb838a6123a8565b906001600160401b03169052565b01611c9c565b611d2d611d3d91611d22611d0994613158565b905f5260205f200190565b5460201c6001600160401b031690565b611cf1565b918083169080841860011c8201809211611d9157825f528763ffffffff611d738460205f200163ffffffff90541690565b161115611d81575091611cd0565b9250611d8c9061341c565b611cd0565b6125eb565b346101fa575f3660031901126101fa576065546040516001600160a01b039091168152602090f35b346101fa575f3660031901126101fa576033546040516001600160a01b039091168152602090f35b346101fa5760206001600160401b03611e29611e0136610eba565b6001600160a01b039182165f9081526098865260408082209290931681526020919091522090565b5416604051908152f35b346101fa5760203660031901126101fa57604063ffffffff611e5f600435611e5a81610460565b612af0565b835191151582529091166020820152f35b346101fa575f3660031901126101fa576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080602083519182815201916020808360051b8301019401925f915b838310611edf57505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b808210611f1f5750505060208060019297019301930191939290611ed0565b90919260208060019286518152019401920190611f00565b9091611f4e610cb993604084526040840190611eb4565b916020818403910152611eb4565b346101fa5760a03660031901126101fa57611f76366101e9565b6044356001600160401b0381116101fa57611f959036906004016101fe565b6064929192356001600160401b0381116101fa57611fb79036906004016101fe565b611fe56103bf63ffffffff608495949535611fd181610bfb565b1695611fde428811612b90565b3690610c09565b9460405194637870733b60e11b86525f86806120078689898860048601612c94565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa958615610510575f966121d0575b5061204f849294612321565b96670de0b6b3a7640000925f5b81811061207257604051806103648c8c83611f37565b6120806103ac82848a61237e565b61208987612840565b612093838d6123a8565b5261209e828c6123a8565b505f8b87878d888c8f5b8188106120c05750505050505050505060010161205c565b8861218861074761072f6111816111818f978e9d9c60019f9b9c8c8b6121949f61211d61215e9f611b4e9061218e9f8f6121046103ac8961216d9d611b419561237e565b9c8d9160018060a01b03165f52609960205260405f2090565b80516001600160401b03169463ffffffff61213f604084015163ffffffff1690565b1611156121a2575b509161215e612164926001600160401b03946123a8565b516123a8565b51911690613906565b6001600160a01b039096165f90815260976020526040902090565b90613866565b946123a8565b52018b87878d888c8f6120a8565b612164926121c56001600160401b03959396610931602061215e950151600f0b90565b959294509250612147565b6121ed9196503d805f833e6121e58183610b94565b810190612ba6565b945f612043565b346101fa5760203660031901126101fa5760043561221181610460565b6122196136be565b6001600160a01b03811615612231576104df906133be565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b346101fa5760203660031901126101fa5760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa8015610510576122de915f916104e157506001600160a01b031633146123dc565b6122ef606654198219811614612420565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b9061232b82610be4565b6123386040519182610b94565b8281526020819361234b601f1991610be4565b0191015f5b82811061235c57505050565b606082820152602001612350565b634e487b7160e01b5f52603260045260245ffd5b919081101561238e5760051b0190565b61236a565b35610cb981610460565b610cb9903690610c09565b805182101561238e5760209160051b010190565b908160209103126101fa5751610cb981610460565b6040513d5f823e3d90fd5b156123e357565b63794821ff60e01b5f5260045ffd5b908160209103126101fa575180151581036101fa5790565b1561241157565b631d77d47760e21b5f5260045ffd5b1561242757565b63c61dca5d60e01b5f5260045ffd5b1561243d57565b63840a48d560e01b5f5260045ffd5b1561245357565b63fa55fc8160e01b5f5260045ffd5b919081101561238e5760051b81013590607e19813603018212156101fa570190565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa57602001918160061b360383136101fa57565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa57602001918160051b360383136101fa57565b156124f557565b6343714afd60e01b5f5260045ffd5b63ffffffff60208092803561251881610460565b6001600160a01b03168552013561252e81610bfb565b16910152565b60208082528101839052604001915f5b8181106125515750505090565b9091926040808261256460019488612504565b019401929101612544565b1561257657565b631fb1705560e21b5f5260045ffd5b356001600160401b03811681036101fa5790565b156125a057565b633a3fa06360e21b5f5260045ffd5b919081101561238e5760061b0190565b156125c657565b630d8fcbe360e41b5f5260045ffd5b156125dc57565b634606179360e11b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b9063ffffffff8091169116019063ffffffff8211611d9157565b1561262057565b63329d4e5360e21b5f5260045ffd5b6001600160a01b03909116815260c081019594909360a09363ffffffff936001600160401b03929190612666906020890190612504565b600180871b0316606087015216608085015216910152565b1561268557565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b606554610bc493919261273e9290916001600160a01b03161580612743575b61270990612ee8565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2612efe565b6133be565b506001600160a01b0382161515612700565b9092919283519361276585610be4565b946127736040519687610b94565b808652612782601f1991610be4565b015f5b8181106127f25750505f5b81518110156127ec57806127b96127b26127ac600194866123a8565b51612cf1565b8587612d89565b6127d06103e160208301516001600160401b031690565b6127da82896123a8565b526127e581886123a8565b5001612790565b50505050565b60209060405161280181610b79565b5f81525f838201525f604082015282828a01015201612785565b1561282257565b6325ec6c1f60e01b5f5260045ffd5b3561ffff811681036101fa5790565b9061284a82610be4565b6128576040519182610b94565b8281528092612868601f1991610be4565b0190602036910137565b1561287957565b631353603160e01b5f5260045ffd5b35610cb981610bfb565b1561289957565b63ccea9e6f60e01b5f5260045ffd5b156128af57565b634e99e6cf60e01b5f5260045ffd5b906001600160401b03809116911603906001600160401b038211611d9157565b600f0b60016001607f1b03198114611d91575f0390565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b03831317611d9157565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff908116604083015260c082019594919360a0939192916001600160401b039190612666565b903590601e19813603018212156101fa57018035906001600160401b0382116101fa576020019181360383136101fa57565b916020908281520191905f5b8181106129af5750505090565b90919260208060019286356129c381610460565b848060a01b0316815201940191019190916129a2565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff1660408201529594929391612a3e919060c0606089015260c0880191612996565b9480860360808201526020808551978881520194015f965b808810612a72575050610cb994955060a08185039101526129d9565b90946020806001928851815201960197019690612a56565b90604051612a9781610b79565b604063ffffffff8294546001600160401b038116845280831c600f0b602085015260c01c16910152565b90604051612ace81610b79565b604063ffffffff8294548181168452818160201c166020850152821c16910152565b60018060a01b03165f52609b6020526040805f20612b42825191612b1383610b79565b5463ffffffff81169283815263ffffffff808360201c1692836020840152861c16948591015263ffffffff1690565b9163ffffffff831615159081612b77575b5015612b695750905b63ffffffff821615159190565b63ffffffff16905090612b5c565b612b87915063ffffffff16611b67565b4210155f612b53565b15612b9757565b63b7d0949760e01b5f5260045ffd5b6020818303126101fa578051906001600160401b0382116101fa57019080601f830112156101fa57815190612bda82610be4565b92612be86040519485610b94565b82845260208085019360051b820101908282116101fa5760208101935b828510612c1457505050505090565b84516001600160401b0381116101fa57820184603f820112156101fa57602081015190612c4082610be4565b91612c4e6040519384610b94565b8083526020808085019260051b84010101918783116101fa57604001905b828210612c8457505050815260209485019401612c05565b8151815260209182019101612c6c565b949391949290928560408201604083525260608101935f965b808810612cc9575050610cb99495506020818503910152612996565b90946020806001928835612cdc81610460565b848060a01b0316815201960197019690612cad565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b16603482015260208152612d37604082610b94565b5190519060208110612d47575090565b5f199060200360031b1b1690565b60405190608082018281106001600160401b03821117610b74576040525f6060838281528260208201528260408201520152565b90611a64612ddc91611181612dc1611b4e612da2612d55565b97611b41856111818a60018060a01b03165f52609960205260405f2090565b6001600160a01b039095165f90815260986020526040902090565b90612dee604082015163ffffffff1690565b63ffffffff81164210612e79575080612e37612e276020612e19612e5695516001600160401b031690565b9301926109318451600f0b90565b6001600160401b03166020860152565b6001600160401b03831684525f60608501525f604085015251600f0b90565b905f612e6283600f0b90565b12612e6c57505090565b610cb9916115c6916131d2565b90610cb992935080612ed1612ea66020612e9d612edb95516001600160401b031690565b930151600f0b90565b91612ec1612eb2610bd5565b6001600160401b039098168852565b6001600160401b03166020870152565b600f0b6040850152565b63ffffffff166060830152565b15612eef57565b6339b190bb60e11b5f5260045ffd5b6001600160a01b0316612f12811515612ee8565b606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b5f198114611d915760010190565b8015611d91575f190190565b6001600160a01b038082165f908152609a6020908152604080832093861683529290529081209093929190612fae90611a93565b935b84151580613065575b1561305e57612fe4612fdf846111818560018060a01b03165f52609a60205260405f2090565b613716565b612fef818585612d89565b90613004611b67606084015163ffffffff1690565b4210613055579161301d6130499261304f948787613238565b61304361303e866111818760018060a01b03165f52609a60205260405f2090565b613757565b50612f60565b94612f6e565b93612fb0565b50505050509050565b5050509050565b5061ffff8110612fb9565b6001600160a01b038181165f908152609a6020908152604080832093861683529290529081209094906130a290611a93565b945b8515158061314b575b15613143576130d3612fdf856111818660018060a01b03165f52609a60205260405f2090565b6130de818686612d89565b906130f3611b67606084015163ffffffff1690565b4210613139579161310c61312d92613133948888613238565b61304361303e876111818860018060a01b03165f52609a60205260405f2090565b95612f6e565b946130a4565b5050509350505050565b509350505050565b5061ffff851681106130ad565b5f19810191908211611d9157565b80548061317b575050670de0b6b3a764000090565b805f19810111611d91576001600160401b03915f525f199060205f2001015460201c1690565b6001600160401b03809116600f0b9116600f0b0360016001607f1b03811360016001607f1b0319821217611d915790565b6001600160401b0391826131e99216600f0b6128f5565b1690565b90815460801d9061320c826001850190600f0b5f5260205260405f2090565b5581546001600160801b0316600190910160801b6fffffffffffffffffffffffffffffffff1916179055565b90916133867facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc55946133286133b9936132c86001600160401b036020850151169163ffffffff6040860151600f0b81606088015116906040519561329a87610b79565b8652600f0b6020860152166040840152611b41896111818a60018060a01b03165f52609960205260405f2090565b81518154602084015160409485015163ffffffff60c01b60c09190911b1677ffffffffffffffffffffffffffffffff00000000000000009190951b166001600160e01b03199091166001600160401b039092169190911717919091179055565b61337961333c82516001600160401b031690565b6001600160a01b0386165f90815260986020526040902061335e908890611181565b906001600160401b03166001600160401b0319825416179055565b516001600160401b031690565b604080516001600160a01b0394851681529490931660208501526001600160401b03169183019190915281906060820190565b0390a1565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b1561340d57565b63dd18181560e01b5f5260045ffd5b9060018201809211611d9157565b91908201809211611d9157565b907f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9161346b63ffffffff83161515613406565b6133b961358961349361348e8460018060a01b03165f52609b60205260405f2090565b612ac1565b6134c985602083016134a9815163ffffffff1690565b63ffffffff81161515806135c8575b6135ba575b509063ffffffff169052565b61357f6134ff611b6763ffffffff7f0000000000000000000000000000000000000000000000000000000000000000164261342a565b63ffffffff1660408301908152916001600160a01b0386165f908152609b602052604090208151815463ffffffff191663ffffffff919091161781559060208181015183546040938401516bffffffffffffffff00000000199091169190921b67ffffffff000000001617911b6bffffffff000000000000000016179055565b5163ffffffff1690565b604080516001600160a01b03909416845263ffffffff94851660208501529316928201929092529081906060820190565b63ffffffff1684525f6134bd565b506135dd611b67604087015163ffffffff1690565b4210156134b8565b9190915f8382019384129112908015821691151617611d9157565b805490916001600160ff1b038111613668576107ed61362d6136286136349385600f0b6135e5565b613b58565b9260801d90565b81600f0b1215613659576001613655920190600f0b5f5260205260405f2090565b5490565b632d0483c560e21b5f5260045ffd5b60405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608490fd5b6033546001600160a01b031633036136d257565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b61372b815480600f0b9060801d600f0b131590565b613748578054600f0b5f9081526001909101602052604090205490565b631ed9509560e11b5f5260045ffd5b9061376d825480600f0b9060801d600f0b131590565b613748578154600f0b9160018101925f6137a882613796818890600f0b5f5260205260405f2090565b549690600f0b5f5260205260405f2090565b5560016001600160801b031983541691016001600160801b0316179055565b81156137d1570490565b634e487b7160e01b5f52601260045260245ffd5b156101fa57565b5f19828209828202918280831092039180830392146138555781670de0b6b3a764000011156101fa577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b5f19670de0b6b3a7640000820991670de0b6b3a76400008202918280851094039380850394146138fa57670de0b6b3a764000082916138a68684116137e5565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b5090610cb992506137c7565b90915f198383099280830292838086109503948086039514613930579082916138a68684116137e5565b505090610cb992506137c7565b90815468010000000000000000811015610b74576001810180845581101561238e5760206001600160401b0391610bc4945f52815f20019261399663ffffffff825116859063ffffffff1663ffffffff19825416179055565b015182546bffffffffffffffff000000001916911660201b6bffffffffffffffff0000000016179055565b906040516139ce81610b59565b60206001600160401b0382945463ffffffff81168452821c16910152565b156139f357565b60405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606490fd5b909291928382548015155f14613b2e57926020929184613a70613a6b613a60613af498613158565b855f5260205f200190565b6139c1565b9363ffffffff613a95613a87875163ffffffff1690565b8284169283911611156139ec565b613aa6611b67875163ffffffff1690565b03613af85750613ae692611d22613abc92613158565b906bffffffffffffffff0000000082549160201b16906bffffffffffffffff000000001916179055565b01516001600160401b031690565b9190565b915050613b2991613b16613b0a610bc6565b63ffffffff9093168352565b6001600160401b0388168286015261393d565b613ae6565b5050613b5391613b3f613b0a610bc6565b6001600160401b038516602083015261393d565b5f9190565b60016001607f1b031981121580613bca575b15613b7557600f0b90565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608490fd5b5060016001607f1b03811315613b6a56fea264697066735822122029a6b3d18fd735af7f32c8fcf7765c900ac2a6f8fc74b5d7ab041ccbea6e56ca64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x0B\0!\x19\x14a\x01\xE4W\x80c\x10\xD6z/\x14a\x01\xDFW\x80c\x13d9\xDD\x14a\x01\xDAW\x80c\x167\xB6\x0F\x14a\x01\xD5W\x80c\x17\x94\xBB<\x14a\x01\xD0W\x80c)\x81\xEBw\x14a\x01\xCBW\x80c5\xAF\x05J\x14a\x01\xC6W\x80cKPF\xEF\x14a\x01\xC1W\x80cM\x9D\xBD\xE9\x14a\x01\xBCW\x80cTz\xFB\x87\x14a\x01\xB7W\x80cV\xC4\x83\xE6\x14a\x01\xB2W\x80cY\\jg\x14a\x01\xADW\x80cZ\xC8j\xB7\x14a\x01\xA8W\x80c\\H\x9B\xB5\x14a\x01\xA3W\x80c\\\x97Z\xBB\x14a\x01\x9EW\x80c`\xDB\x99\xA3\x14a\x01\x99W\x80ck:\xA7.\x14a\x01\x94W\x80cl\xFBD\x81\x14a\x01\x8FW\x80cqP\x18\xA6\x14a\x01\x8AW\x80c{\xC1\xEFa\x14a\x01\x85W\x80c\x84;4\x9F\x14a\x01\x80W\x80c\x88o\x11\x95\x14a\x01{W\x80c\x8D\xA5\xCB[\x14a\x01vW\x80c\xA9\x84\xEB:\x14a\x01qW\x80c\xB9\xFB\xAE\xD1\x14a\x01lW\x80c\xDF\\\xF7#\x14a\x01gW\x80c\xE0}+\x12\x14a\x01bW\x80c\xF2\xFD\xE3\x8B\x14a\x01]Wc\xFA\xBC\x1C\xBC\x14a\x01XW_\x80\xFD[a\"\x85V[a!\xF4V[a\x1F\\V[a\x1EpV[a\x1E3V[a\x1D\xE6V[a\x1D\xBEV[a\x1D\x96V[a\x1C3V[a\x1B\xF3V[a\x1B\x98V[a\x1A5V[a\x19\xF1V[a\x13}V[a\x13`V[a\x12\xDCV[a\x12\xA9V[a\x12#V[a\x11\xB2V[a\x10\xEBV[a\x0FJV[a\rwV[a\x0C\xBCV[a\x0B\x05V[a\t\xF3V[a\x05\xDCV[a\x05\x15V[a\x04qV[a\x02\xE8V[`@\x90`\x03\x19\x01\x12a\x01\xFAW`\x04\x90V[_\x80\xFD[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xFAW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xFAW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xFAWV[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x02`WPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x02\xA0WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x02QV[\x90\x91\x92` a\x02\xDE`\x01\x92\x86Q\x90c\xFF\xFF\xFF\xFF`@``\x93`\x01`\x01`@\x1B\x03\x81Q\x16\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R\x01Q\x16`@\x82\x01R\x01\x90V[\x94\x01\x92\x01\x90a\x02\x81V[4a\x01\xFAW`\x806`\x03\x19\x01\x12a\x01\xFAWa\x03\x026a\x01\xE9V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x03!\x906\x90`\x04\x01a\x01\xFEV[\x91\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x03B\x906\x90`\x04\x01a\x01\xFEV[\x91a\x03L\x83a#!V[\x94_[\x84\x81\x10a\x03hW`@Q\x80a\x03d\x89\x82a\x02.V[\x03\x90\xF3[_[\x82\x81\x10a\x03zWP`\x01\x01a\x03OV[\x94`\x01\x86a\x04R\x81a\x03\xCA\x88\x8Ca\x03\xC4\x87\x9F\x9B\x9A\x99\x9Da\x03\xB7\x8F\x8F\x92\x8Da\x03\xB1a\x03\xAC\x8F\x94a\x03\xAC\x95a\x03\xBF\x98a#~V[a#\x93V[\x97a#~V[\x926\x90a\x0C\tV[a,\xF1V[\x91a-\x89V[a\x046a\x03\xE1` \x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91a\x04)a\x04\x04``a\x03\xF8`@\x85\x01Q`\x0F\x0B\x90V[\x93\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x91a\x04\x1Fa\x04\x10a\x0B\xB5V[`\x01`\x01`@\x1B\x03\x90\x96\x16\x86RV[`\x0F\x0B` \x85\x01RV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01RV[a\x04@\x8B\x8Aa#\xA8V[Q\x90a\x04L\x83\x83a#\xA8V[Ra#\xA8V[P\x01\x90\x95\x91\x92\x93\x97Pa\x03jV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xFAWV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x04\x805a\x04\x8F\x81a\x04`V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x92` \x91\x84\x91\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x10Wa\x04\xDF\x92a\x04\xDA\x91_\x91a\x04\xE1W[P`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDCV[a.\xFEV[\0[a\x05\x03\x91P` =` \x11a\x05\tW[a\x04\xFB\x81\x83a\x0B\x94V[\x81\x01\x90a#\xBCV[_a\x04\xC9V[P=a\x04\xF1V[a#\xD1V[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x05l\x91_\x91a\x05\xADW[Pa$\nV[a\x05{`fT\x82\x81\x16\x14a$ V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x05\xCF\x91P` =` \x11a\x05\xD5W[a\x05\xC7\x81\x83a\x0B\x94V[\x81\x01\x90a#\xF2V[_a\x05fV[P=a\x05\xBDV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x06\x0Fa\x068\x916\x90`\x04\x01a\x01\xFEV[\x91\x90a\x06(a\x06\"`\x01\x80`fT\x16\x14\x90V[\x15a$6V[a\x0613a*\xF0V[\x92\x90a$LV[_\x90[\x83\x82\x10a\x06DW\0[a\x06O\x82\x85\x83a$bV[a\x06wa\x06_`@\x83\x01\x83a$\x84V[\x90Pa\x06n``\x84\x01\x84a$\xB9V[\x91\x90P\x14a$\xEEV[a\x06\xA5` a\x06\x89`@\x84\x01\x84a$\x84V[`@Qc\x10bq\x1B`\xE1\x1B\x81R\x93\x84\x92\x83\x92\x90`\x04\x84\x01a%4V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x06\xEA\x91_\x91a\t\xD5W[Pa%oV[a\x06\xFCa\x06\xF6\x82a#\x93V[3a/zV[3_\x90\x81R`\x97` R`@\x90 a\x074\x90a\x07/\x90a\x07\x1B\x84a#\x93V[`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a1fV[\x91a\x07ca\x07Sa\x07G` \x85\x01a%\x85V[`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x85\x16\x14a%\x99V[_[a\x07r`@\x84\x01\x84a$\x84V[\x90P\x81\x10\x15a\t\xC5W\x80\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x07\xC1a\x03\xBFa\x07\xBC`\x01\x95a\x07\xB6`@\x8A\x01\x8Aa$\x84V[\x90a%\xAFV[a#\x9DV[a\x08\xEEa\x07\xD7\x82a\x07\xD1\x89a#\x93V[3a-\x89V[\x80\x92a\x07\xF9a\x07\xF3a\x07\xED`@\x85\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x15a%\xBFV[a\x08Da\x08:\x8Aa\x084a\x08/\x8Aa\x08)a\x08\x1E` \x8A\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x94``\x81\x01\x90a$\xB9V[\x90a#~V[a%\x85V[\x90a1\xA1V[`\x0F\x0B`@\x84\x01RV[a\x08_a\x08Xa\x07\xED`@\x85\x01Q`\x0F\x0B\x90V[\x15\x15a%\xD5V[a\x08pa\x07\xED`@\x84\x01Q`\x0F\x0B\x90V[_\x81\x12\x15a\tZWPa\x08\xB9a\x08\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFB\x16a%\xFFV[c\xFF\xFF\xFF\xFF\x16``\x84\x01RV[3_\x90\x81R`\x9A` R`@\x90 a\x08\xDF\x90\x82\x90a\x08\xDA\x90a\x07\x1B\x8Da#\x93V[a1\xEDV[a\x08\xE8\x89a#\x93V[3a28V[a\x08\xFF\x83a\x07\xB6`@\x89\x01\x89a$\x84V[a\tQa\t\x0B\x88a#\x93V[\x92a\tC``a\t7a\t(` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x85\x01Q`\x0F\x0B[\x90a1\xD2V[\x92\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`@Q\x94\x85\x943\x86a&/V[\x03\x90\xA1\x01a\x07eV[_\x12\x15a\x08\xDFWa\tta\x08\xAC\x8Dc\xFF\xFF\xFF\xFFB\x16a%\xFFV[a\t\x98a\t\x8Ba\t(\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x83RV[a\t\xC0`\x01`\x01`@\x1B\x03\x8B\x16a\t\xB9a\x07G\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x11\x15a&\x19V[a\x08\xDFV[P`\x01\x90\x93\x01\x92\x91Pa\x06;\x90PV[a\t\xED\x91P` =\x81\x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x06\xE4V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\n\x10\x81a\x04`V[a\nn`$5a\n\x1F\x81a\x04`V[`D5\x90_T\x93a\nTa\n>a\n:\x87`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x96\x81\x97a\n\xEDW[\x81\x15a\n\xCDW[Pa&~V[\x84a\ne`\x01`\xFF\x19_T\x16\x17_UV[a\n\xB6Wa&\xE1V[a\ntW\0[a\n\x82a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01[\x03\x90\xA1\0[a\n\xC8a\x01\0a\xFF\0\x19_T\x16\x17_UV[a&\xE1V[0;\x15\x91P\x81a\n\xDFW[P_a\nNV[`\xFF\x16`\x01\x14\x90P_a\n\xD8V[`\x01`\xFF\x82\x16\x10\x91Pa\nGV[_\x91\x03\x12a\x01\xFAWV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[a\x0BEV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@RV[`@Q\x90a\x0B\xC4``\x83a\x0B\x94V[V[`@Q\x90a\x0B\xC4`@\x83a\x0B\x94V[`@Q\x90a\x0B\xC4`\x80\x83a\x0B\x94V[`\x01`\x01`@\x1B\x03\x81\x11a\x0BtW`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x01\xFAWV[\x91\x90\x82`@\x91\x03\x12a\x01\xFAW`@Qa\x0C!\x81a\x0BYV[` \x80\x82\x94\x805a\x0C1\x81a\x04`V[\x84R\x015\x91a\x0C?\x83a\x0B\xFBV[\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0C`WPPP\x90V[\x90\x91\x92` a\x0C\x9E`\x01\x92\x86Q\x90c\xFF\xFF\xFF\xFF`@``\x93`\x01`\x01`@\x1B\x03\x81Q\x16\x84R` \x81\x01Q`\x0F\x0B` \x85\x01R\x01Q\x16`@\x82\x01R\x01\x90V[\x94\x01\x92\x91\x01a\x0CSV[\x90` a\x0C\xB9\x92\x81\x81R\x01\x90a\x0CCV[\x90V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x0C\xD9\x81a\x04`V[`$5a\x0C\xE5\x81a\x04`V[`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW6`#\x83\x01\x12\x15a\x01\xFAW\x81`\x04\x015\x92a\r\x11\x84a\x0B\xE4V[\x92a\r\x1F`@Q\x94\x85a\x0B\x94V[\x84\x84R`$` \x85\x01\x95`\x06\x1B\x82\x01\x01\x906\x82\x11a\x01\xFAW`$\x01\x94[\x81\x86\x10a\r]Wa\x03da\rQ\x86\x86\x86a'UV[`@Q\x91\x82\x91\x82a\x0C\xA8V[` `@\x91a\rl6\x89a\x0C\tV[\x81R\x01\x95\x01\x94a\r<V[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\r\x94\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\r\xB3\x906\x90`\x04\x01a\x01\xFEV[`D\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\r\xD5\x906\x90`\x04\x01a\x01\xFEV[\x92\x90a\r\xE8a\x06\"`\x01\x80`fT\x16\x14\x90V[a\r\xF3\x84\x84\x14a$\xEEV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x0E\\\x91_\x91a\x0E\x9BW[Pa(\x1BV[_[\x83\x81\x10a\x0EgW\0[\x80a\x0E\x95a\x0E{a\x03\xAC`\x01\x94\x88\x8Ba#~V[a\x0E\x8Ea\x0E\x89\x84\x8A\x88a#~V[a(1V[\x90\x86a0pV[\x01a\x0E^V[a\x0E\xB4\x91P` =` \x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x0EVV[`@\x90`\x03\x19\x01\x12a\x01\xFAW`\x045a\x0E\xD2\x81a\x04`V[\x90`$5a\x0C\xB9\x81a\x04`V[\x92\x91`@\x84\x01\x93`@\x81R\x82Q\x80\x95R` ``\x82\x01\x93\x01_\x95[\x80\x87\x10a\x0F\x16WPPa\x0C\xB9\x93\x94P` \x81\x84\x03\x91\x01Ra\x0CCV[\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x87\x01R\x91\x94\x90\x91`\x01\x91`@\x01\x95\x01\x96\x01\x95\x90a\x0E\xFAV[4a\x01\xFAWa\x0FX6a\x0E\xBAV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R_`$\x83\x01\x81\x90R_\x19`D\x84\x01R\x91\x93\x91\x90\x84\x90`d\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x92\x83\x15a\x05\x10W_\x93a\x0F\xDDW[Pa\x0F\xCD\x91\x83\x91a'UV[\x90a\x03d`@Q\x92\x83\x92\x83a\x0E\xDFV[\x92P=\x80_\x85>a\x0F\xEE\x81\x85a\x0B\x94V[\x83\x01\x92` \x81\x85\x03\x12a\x01\xFAW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW\x01\x92\x80`\x1F\x85\x01\x12\x15a\x01\xFAW\x83Q\x93a\x10%\x85a\x0B\xE4V[\x91a\x103`@Q\x93\x84a\x0B\x94V[\x85\x83R` \x80\x84\x01\x96`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x01\xFAW` \x01\x95[\x82\x87\x10a\x10fWPPPa\x0F\xCD\x92\x93P\x92\x91a\x0F\xC1V[`@\x87\x83\x03\x12a\x01\xFAW` `@\x91\x82Qa\x10\x80\x81a\x0BYV[\x89Qa\x10\x8B\x81a\x04`V[\x81R\x82\x8A\x01Qa\x10\x9A\x81a\x0B\xFBV[\x83\x82\x01R\x81R\x01\x96\x01\x95a\x10OV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x10\xCCWPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xBFV[4a\x01\xFAW`@6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x11\x08\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x11'\x906\x90`\x04\x01a\x01\xFEV[\x91\x90a\x112\x83a(@V[\x92`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91_[\x81\x81\x10a\x11VW`@Q\x80a\x03d\x87\x82a\x10\xA9V[`\x01\x90\x84_R`\x97` Ra\x11\x96a\x07/`@_ a\x11v\x84\x87\x89a#~V[5\x90a\x11\x81\x82a\x04`V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[`\x01`\x01`@\x1B\x03a\x11\xA8\x83\x89a#\xA8V[\x91\x16\x90R\x01a\x11AV[4a\x01\xFAW`@6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x11\xCF\x81a\x04`V[`$5a\x11\xDB\x81a\x0B\xFBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x12\x14Wa\x04\xDF\x91a47V[c\xF79X\x9B`\xE0\x1B_R`\x04_\xFD[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x12u\x91_\x91a\x05\xADWPa$\nV[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\xFF\x81\x16\x80\x91\x03a\x01\xFAW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x12\xF9\x81a\x0B\xFBV[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\x10Wa\x04\xDF\x92a\x13Z\x91_\x91a\x0E\x9BWPa(\x1BV[3a47V[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `fT`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAW`\xA0`\x03\x19\x826\x03\x01\x12a\x01\xFAWa\x13\xBEa\x06\"`\x02\x80`fT\x16\x14\x90V[`d\x81\x015\x90\x81\x15\x15\x80a\x19\xDFW[a\x13\xD6\x90a(rV[a\x13\xE2`$\x82\x01a(\x88V[\x91a\x13\xFFa\x13\xEEa\x0B\xC6V[3\x81R\x93c\xFF\xFF\xFF\xFF\x16` \x85\x01RV[a\x14\x08\x83a,\xF1V[\x91a\x14_` \x85a\x14\x1B\x84`\x04\x01a#\x93V[`@Qc\t\xA9a\xF3`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x82Q\x90\x91\x16`$\x82\x01R` \x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`D\x82\x01R\x91\x82\x90\x81\x90`d\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\x14\xA7\x91_\x91a\x19\xC0W[P\x94\x93\x94a(\x92V[a\x14\xC1a\x14\xBA`D\x83\x01\x83`\x04\x01a$\xB9V[\x90Pa(@V[g\r\xE0\xB6\xB3\xA7d\0\0\x94\x90\x92\x90_[a\x14\xE0`D\x85\x01\x85`\x04\x01a$\xB9V[\x90P\x81\x10\x15a\x19ZWa\x15\x16\x82a\x14\xF9\x86`\x04\x01a#\x93V[a\x15\x10a\x03\xAC\x85a\x08)`D\x8B\x01\x8B`\x04\x01a$\xB9V[\x90a-\x89V[\x90a\x158a\x151a\x07G` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x15\x15a(\xA8V[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6\x85a\x15}a\x07G\x87a\x15xa\x07G` \x89\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a7\xECV[\x93a\x15\xAEa\x15\x9E\x86a\x15\x99` \x85\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a(\xBEV[`\x01`\x01`@\x1B\x03\x16` \x83\x01RV[a\x15\xD3a\x15\xC6\x86a\x15\x99\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x82RV[\x86a\x15\xE2`@\x83\x01Q`\x0F\x0B\x90V[_a\x15\xED\x82`\x0F\x0B\x90V[\x12a\x18\x98W[PPa\x16#\x81\x87a\x16\x06\x85`\x04\x01a#\x93V[a\x16\x1Da\x03\xAC\x89a\x08)`D\x8A\x01\x8A`\x04\x01a$\xB9V[\x90a28V[a\x16ta\x16^` a\x16Oa\x03\xAC\x88a\x08)a\x16A\x89`\x04\x01a#\x93V[\x98`D\x81\x01\x90`\x04\x01a$\xB9V[\x93\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92`@Q\x93\x84\x93\x8Dc\xFF\xFF\xFF\xFFB\x16\x93\x86a)\x1BV[\x03\x90\xA1a\x16\xBCa\x07/a\x16\xA5a\x16\x8C\x88`\x04\x01a#\x93V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x97` R`@\x90 \x90V[a\x07\x1Ba\x03\xAC\x85a\x08)`D\x8C\x01\x8C`\x04\x01a$\xB9V[\x91a\x16\xC7\x81\x84a(\xBEV[\x90a\x17\n\x82\x84a\x17\x05\x8Aa\x07\x1Ba\x03\xACa\x16\xF7a\x16\xE9a\x16\x8C\x85`\x04\x01a#\x93V[\x93`D\x81\x01\x90`\x04\x01a$\xB9V[Bc\xFF\xFF\xFF\xFF\x16\x96\x91a#~V[a:8V[PP\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\a\x179\x88`\x04\x01a#\x93V[a\x17Pa\x03\xAC\x86a\x08)`D\x8D\x01\x8D`\x04\x01a$\xB9V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x91\x90\x92\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x85\x16\x91\x81\x01\x91\x90\x91R``\x90\xA1a\x17\x8B\x87`\x04\x01a#\x93V[\x91a\x17\xA3a\x03\xAC\x85a\x08)`D\x8C\x01\x8C`\x04\x01a$\xB9V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16;\x15a\x01\xFAW`@Qc\xA5z\xB1\x0B`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16`\x04\x82\x01R\x93\x16`$\x84\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x85\x01R\x16`d\x83\x01R\x90\x92\x90_\x84\x80`\x84\x81\x01\x03\x81\x83\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xF1\x91\x82\x15a\x05\x10W`\x01`\x01`@\x1B\x03\x80\x8C\x92`\x01\x97a\x18m\x96a\x18~W[P\x16\x92\x16a9\x06V[a\x18w\x82\x88a#\xA8V[R\x01a\x14\xD0V[\x80a\x18\x8C_a\x18\x92\x93a\x0B\x94V[\x80a\n\xFBV[_a\x18dV[a\x18\xD5a\x18\xC6a\x07Ga\x07Ga\x18\xEC\x95a\x15xa\x18\xBAa\x18\xBAa\x18\xE2\x98a(\xDEV[`\x01`\x01`\x80\x1B\x03\x16\x90V[`\x01`\x01`\x80\x1B\x03\x16`\x0F\x0B\x90V[`@\x84\x01Q`\x0F\x0Ba(\xF5V[`\x0F\x0B`@\x83\x01RV[\x82\x8Aa\x18\xFA\x84`\x04\x01a#\x93V[a\x19Pa\x19\x14a\x03\xAC\x89a\x08)`D\x8A\x01\x8A`\x04\x01a$\xB9V[a\x198a\x19+` \x88\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x88\x01Q`\x0F\x0Ba\t1V[``\x87\x01Qc\xFF\xFF\xFF\xFF\x16\x91`@Q\x95\x86\x95\x86a)\x1BV[\x03\x90\xA1\x86_a\x15\xF3V[\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5\x86\x86a\n\xB1a\x19\xB1\x88a\x19\x90\x81`\x04\x01a#\x93V[\x93a\x19\xA1`D\x83\x01\x83`\x04\x01a$\xB9V[\x93\x90\x92`\x84\x81\x01\x90`\x04\x01a)dV[\x93\x90\x92`@Q\x97\x88\x97\x88a)\xF9V[a\x19\xD9\x91P` =` \x11a\x05\xD5Wa\x05\xC7\x81\x83a\x0B\x94V[_a\x14\x9EV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x82\x11\x15a\x13\xCDV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xFAWa\x1AC6a\x0E\xBAV[a\x1Aqa\x1Ad\x82a\x11\x81\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90V[T`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x1A\xA3a\x1A\x93\x82a\x11\x81\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x03\x90V[_\x90[\x80\x82\x10a\x1A\xF0W[a\x03da\x1A\xD6\x85a\x15\x99a\x07/\x87a\x11\x81\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x97` R`@_ \x90V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x90\x92a\x1BSa\x1BNa\x1B\x1F\x86a\x1B\x1A\x87a\x11\x81\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a6\0V[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x99` R`@\x90 a\x1BA\x90\x87\x90a\x11\x81V[\x90_R` R`@_ \x90V[a*\x8AV[a\x1Bpa\x1Bg`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[B\x10a\x1B\x91W` \x01Q`\x01\x91a\x1B\x89\x91`\x0F\x0Ba\t1V[\x93\x01\x90a\x1A\xA6V[P\x92a\x1A\xAEV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAWa\x1B\xB0a6\xBEV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x01\xFAW``6`\x03\x19\x01\x12a\x01\xFAW`\x045a\x1CP\x81a\x04`V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1Co\x906\x90`\x04\x01a\x01\xFEV[\x91`D5\x91a\x1C}\x83a\x0B\xFBV[a\x1C\x86\x84a(@V[\x93`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x92c\xFF\xFF\xFF\xFF\x16\x91_[\x81\x81\x10a\x1C\xB1W`@Q\x80a\x03d\x88\x82a\x10\xA9V[\x84_R`\x97` Ra\x1C\xCB`@_ a\x11v\x83\x85\x87a#~V[\x80T\x90_[\x82\x81\x10a\x1DBWP`\x01\x92\x91\x90\x81a\x1D\x0FWPPa\x1D\tg\r\xE0\xB6\xB3\xA7d\0\0[a\x1C\xFB\x83\x8Aa#\xA8V[\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x01a\x1C\x9CV[a\x1D-a\x1D=\x91a\x1D\"a\x1D\t\x94a1XV[\x90_R` _ \x01\x90V[T` \x1C`\x01`\x01`@\x1B\x03\x16\x90V[a\x1C\xF1V[\x91\x80\x83\x16\x90\x80\x84\x18`\x01\x1C\x82\x01\x80\x92\x11a\x1D\x91W\x82_R\x87c\xFF\xFF\xFF\xFFa\x1Ds\x84` _ \x01c\xFF\xFF\xFF\xFF\x90T\x16\x90V[\x16\x11\x15a\x1D\x81WP\x91a\x1C\xD0V[\x92Pa\x1D\x8C\x90a4\x1CV[a\x1C\xD0V[a%\xEBV[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW` `\x01`\x01`@\x1B\x03a\x1E)a\x1E\x016a\x0E\xBAV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\x98\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`@c\xFF\xFF\xFF\xFFa\x1E_`\x045a\x1EZ\x81a\x04`V[a*\xF0V[\x83Q\x91\x15\x15\x82R\x90\x91\x16` \x82\x01R\xF3[4a\x01\xFAW_6`\x03\x19\x01\x12a\x01\xFAW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x1E\xDFWPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x1F\x1FWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x1E\xD0V[\x90\x91\x92` \x80`\x01\x92\x86Q\x81R\x01\x94\x01\x92\x01\x90a\x1F\0V[\x90\x91a\x1FNa\x0C\xB9\x93`@\x84R`@\x84\x01\x90a\x1E\xB4V[\x91` \x81\x84\x03\x91\x01Ra\x1E\xB4V[4a\x01\xFAW`\xA06`\x03\x19\x01\x12a\x01\xFAWa\x1Fv6a\x01\xE9V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1F\x95\x906\x90`\x04\x01a\x01\xFEV[`d\x92\x91\x925`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAWa\x1F\xB7\x906\x90`\x04\x01a\x01\xFEV[a\x1F\xE5a\x03\xBFc\xFF\xFF\xFF\xFF`\x84\x95\x94\x955a\x1F\xD1\x81a\x0B\xFBV[\x16\x95a\x1F\xDEB\x88\x11a+\x90V[6\x90a\x0C\tV[\x94`@Q\x94cxps;`\xE1\x1B\x86R_\x86\x80a \x07\x86\x89\x89\x88`\x04\x86\x01a,\x94V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x95\x86\x15a\x05\x10W_\x96a!\xD0W[Pa O\x84\x92\x94a#!V[\x96g\r\xE0\xB6\xB3\xA7d\0\0\x92_[\x81\x81\x10a rW`@Q\x80a\x03d\x8C\x8C\x83a\x1F7V[a \x80a\x03\xAC\x82\x84\x8Aa#~V[a \x89\x87a(@V[a \x93\x83\x8Da#\xA8V[Ra \x9E\x82\x8Ca#\xA8V[P_\x8B\x87\x87\x8D\x88\x8C\x8F[\x81\x88\x10a \xC0WPPPPPPPPP`\x01\x01a \\V[\x88a!\x88a\x07Ga\x07/a\x11\x81a\x11\x81\x8F\x97\x8E\x9D\x9C`\x01\x9F\x9B\x9C\x8C\x8Ba!\x94\x9Fa!\x1Da!^\x9Fa\x1BN\x90a!\x8E\x9F\x8Fa!\x04a\x03\xAC\x89a!m\x9Da\x1BA\x95a#~V[\x9C\x8D\x91`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x80Q`\x01`\x01`@\x1B\x03\x16\x94c\xFF\xFF\xFF\xFFa!?`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15a!\xA2W[P\x91a!^a!d\x92`\x01`\x01`@\x1B\x03\x94a#\xA8V[Qa#\xA8V[Q\x91\x16\x90a9\x06V[`\x01`\x01`\xA0\x1B\x03\x90\x96\x16_\x90\x81R`\x97` R`@\x90 \x90V[\x90a8fV[\x94a#\xA8V[R\x01\x8B\x87\x87\x8D\x88\x8C\x8Fa \xA8V[a!d\x92a!\xC5`\x01`\x01`@\x1B\x03\x95\x93\x96a\t1` a!^\x95\x01Q`\x0F\x0B\x90V[\x95\x92\x94P\x92Pa!GV[a!\xED\x91\x96P=\x80_\x83>a!\xE5\x81\x83a\x0B\x94V[\x81\x01\x90a+\xA6V[\x94_a CV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045a\"\x11\x81a\x04`V[a\"\x19a6\xBEV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\"1Wa\x04\xDF\x90a3\xBEV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\x10Wa\"\xDE\x91_\x91a\x04\xE1WP`\x01`\x01`\xA0\x1B\x03\x163\x14a#\xDCV[a\"\xEF`fT\x19\x82\x19\x81\x16\x14a$ V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[\x90a#+\x82a\x0B\xE4V[a#8`@Q\x91\x82a\x0B\x94V[\x82\x81R` \x81\x93a#K`\x1F\x19\x91a\x0B\xE4V[\x01\x91\x01_[\x82\x81\x10a#\\WPPPV[``\x82\x82\x01R` \x01a#PV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x05\x1B\x01\x90V[a#jV[5a\x0C\xB9\x81a\x04`V[a\x0C\xB9\x906\x90a\x0C\tV[\x80Q\x82\x10\x15a#\x8EW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x01\xFAWQa\x0C\xB9\x81a\x04`V[`@Q=_\x82>=\x90\xFD[\x15a#\xE3WV[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x01\xFAWQ\x80\x15\x15\x81\x03a\x01\xFAW\x90V[\x15a$\x11WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a$'WV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[\x15a$=WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a$SWV[c\xFAU\xFC\x81`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x05\x1B\x81\x015\x90`~\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x81`\x06\x1B6\x03\x83\x13a\x01\xFAWV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xFAWV[\x15a$\xF5WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF` \x80\x92\x805a%\x18\x81a\x04`V[`\x01`\x01`\xA0\x1B\x03\x16\x85R\x015a%.\x81a\x0B\xFBV[\x16\x91\x01RV[` \x80\x82R\x81\x01\x83\x90R`@\x01\x91_[\x81\x81\x10a%QWPPP\x90V[\x90\x91\x92`@\x80\x82a%d`\x01\x94\x88a%\x04V[\x01\x94\x01\x92\x91\x01a%DV[\x15a%vWV[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[5`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x01\xFAW\x90V[\x15a%\xA0WV[c:?\xA0c`\xE2\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a#\x8EW`\x06\x1B\x01\x90V[\x15a%\xC6WV[c\r\x8F\xCB\xE3`\xE4\x1B_R`\x04_\xFD[\x15a%\xDCWV[cF\x06\x17\x93`\xE1\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a\x1D\x91WV[\x15a& WV[c2\x9DNS`\xE2\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`\xC0\x81\x01\x95\x94\x90\x93`\xA0\x93c\xFF\xFF\xFF\xFF\x93`\x01`\x01`@\x1B\x03\x92\x91\x90a&f\x90` \x89\x01\x90a%\x04V[`\x01\x80\x87\x1B\x03\x16``\x87\x01R\x16`\x80\x85\x01R\x16\x91\x01RV[\x15a&\x85WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`eTa\x0B\xC4\x93\x91\x92a'>\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a'CW[a'\t\x90a.\xE8V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a.\xFEV[a3\xBEV[P`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15a'\0V[\x90\x92\x91\x92\x83Q\x93a'e\x85a\x0B\xE4V[\x94a's`@Q\x96\x87a\x0B\x94V[\x80\x86Ra'\x82`\x1F\x19\x91a\x0B\xE4V[\x01_[\x81\x81\x10a'\xF2WPP_[\x81Q\x81\x10\x15a'\xECW\x80a'\xB9a'\xB2a'\xAC`\x01\x94\x86a#\xA8V[Qa,\xF1V[\x85\x87a-\x89V[a'\xD0a\x03\xE1` \x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[a'\xDA\x82\x89a#\xA8V[Ra'\xE5\x81\x88a#\xA8V[P\x01a'\x90V[PPPPV[` \x90`@Qa(\x01\x81a\x0ByV[_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x8A\x01\x01R\x01a'\x85V[\x15a(\"WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[5a\xFF\xFF\x81\x16\x81\x03a\x01\xFAW\x90V[\x90a(J\x82a\x0B\xE4V[a(W`@Q\x91\x82a\x0B\x94V[\x82\x81R\x80\x92a(h`\x1F\x19\x91a\x0B\xE4V[\x01\x90` 6\x91\x017V[\x15a(yWV[c\x13S`1`\xE0\x1B_R`\x04_\xFD[5a\x0C\xB9\x81a\x0B\xFBV[\x15a(\x99WV[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[\x15a(\xAFWV[cN\x99\xE6\xCF`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x1D\x91WV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a\x1D\x91W_\x03\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a\x1D\x91WV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`@\x83\x01R`\xC0\x82\x01\x95\x94\x91\x93`\xA0\x93\x91\x92\x91`\x01`\x01`@\x1B\x03\x91\x90a&fV[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xFAW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW` \x01\x91\x816\x03\x83\x13a\x01\xFAWV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a)\xAFWPPP\x90V[\x90\x91\x92` \x80`\x01\x92\x865a)\xC3\x81a\x04`V[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x94\x01\x91\x01\x91\x90\x91a)\xA2V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x95\x94\x92\x93\x91a*>\x91\x90`\xC0``\x89\x01R`\xC0\x88\x01\x91a)\x96V[\x94\x80\x86\x03`\x80\x82\x01R` \x80\x85Q\x97\x88\x81R\x01\x94\x01_\x96[\x80\x88\x10a*rWPPa\x0C\xB9\x94\x95P`\xA0\x81\x85\x03\x91\x01Ra)\xD9V[\x90\x94` \x80`\x01\x92\x88Q\x81R\x01\x96\x01\x97\x01\x96\x90a*VV[\x90`@Qa*\x97\x81a\x0ByV[`@c\xFF\xFF\xFF\xFF\x82\x94T`\x01`\x01`@\x1B\x03\x81\x16\x84R\x80\x83\x1C`\x0F\x0B` \x85\x01R`\xC0\x1C\x16\x91\x01RV[\x90`@Qa*\xCE\x81a\x0ByV[`@c\xFF\xFF\xFF\xFF\x82\x94T\x81\x81\x16\x84R\x81\x81` \x1C\x16` \x85\x01R\x82\x1C\x16\x91\x01RV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@\x80_ a+B\x82Q\x91a+\x13\x83a\x0ByV[Tc\xFF\xFF\xFF\xFF\x81\x16\x92\x83\x81Rc\xFF\xFF\xFF\xFF\x80\x83` \x1C\x16\x92\x83` \x84\x01R\x86\x1C\x16\x94\x85\x91\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x91c\xFF\xFF\xFF\xFF\x83\x16\x15\x15\x90\x81a+wW[P\x15a+iWP\x90[c\xFF\xFF\xFF\xFF\x82\x16\x15\x15\x91\x90V[c\xFF\xFF\xFF\xFF\x16\x90P\x90a+\\V[a+\x87\x91Pc\xFF\xFF\xFF\xFF\x16a\x1BgV[B\x10\x15_a+SV[\x15a+\x97WV[c\xB7\xD0\x94\x97`\xE0\x1B_R`\x04_\xFD[` \x81\x83\x03\x12a\x01\xFAW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xFAW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x01\xFAW\x81Q\x90a+\xDA\x82a\x0B\xE4V[\x92a+\xE8`@Q\x94\x85a\x0B\x94V[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x90\x82\x82\x11a\x01\xFAW` \x81\x01\x93[\x82\x85\x10a,\x14WPPPPP\x90V[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\x01\xFAW\x82\x01\x84`?\x82\x01\x12\x15a\x01\xFAW` \x81\x01Q\x90a,@\x82a\x0B\xE4V[\x91a,N`@Q\x93\x84a\x0B\x94V[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x87\x83\x11a\x01\xFAW`@\x01\x90[\x82\x82\x10a,\x84WPPP\x81R` \x94\x85\x01\x94\x01a,\x05V[\x81Q\x81R` \x91\x82\x01\x91\x01a,lV[\x94\x93\x91\x94\x92\x90\x92\x85`@\x82\x01`@\x83RR``\x81\x01\x93_\x96[\x80\x88\x10a,\xC9WPPa\x0C\xB9\x94\x95P` \x81\x85\x03\x91\x01Ra)\x96V[\x90\x94` \x80`\x01\x92\x885a,\xDC\x81a\x04`V[\x84\x80`\xA0\x1B\x03\x16\x81R\x01\x96\x01\x97\x01\x96\x90a,\xADV[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra-7`@\x82a\x0B\x94V[Q\x90Q\x90` \x81\x10a-GWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[`@Q\x90`\x80\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x0BtW`@R_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x90a\x1Ada-\xDC\x91a\x11\x81a-\xC1a\x1BNa-\xA2a-UV[\x97a\x1BA\x85a\x11\x81\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16_\x90\x81R`\x98` R`@\x90 \x90V[\x90a-\xEE`@\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16B\x10a.yWP\x80a.7a.'` a.\x19a.V\x95Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93\x01\x92a\t1\x84Q`\x0F\x0B\x90V[`\x01`\x01`@\x1B\x03\x16` \x86\x01RV[`\x01`\x01`@\x1B\x03\x83\x16\x84R_``\x85\x01R_`@\x85\x01RQ`\x0F\x0B\x90V[\x90_a.b\x83`\x0F\x0B\x90V[\x12a.lWPP\x90V[a\x0C\xB9\x91a\x15\xC6\x91a1\xD2V[\x90a\x0C\xB9\x92\x93P\x80a.\xD1a.\xA6` a.\x9Da.\xDB\x95Q`\x01`\x01`@\x1B\x03\x16\x90V[\x93\x01Q`\x0F\x0B\x90V[\x91a.\xC1a.\xB2a\x0B\xD5V[`\x01`\x01`@\x1B\x03\x90\x98\x16\x88RV[`\x01`\x01`@\x1B\x03\x16` \x87\x01RV[`\x0F\x0B`@\x85\x01RV[c\xFF\xFF\xFF\xFF\x16``\x83\x01RV[\x15a.\xEFWV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x16a/\x12\x81\x15\x15a.\xE8V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[_\x19\x81\x14a\x1D\x91W`\x01\x01\x90V[\x80\x15a\x1D\x91W_\x19\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x93\x92\x91\x90a/\xAE\x90a\x1A\x93V[\x93[\x84\x15\x15\x80a0eW[\x15a0^Wa/\xE4a/\xDF\x84a\x11\x81\x85`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a7\x16V[a/\xEF\x81\x85\x85a-\x89V[\x90a0\x04a\x1Bg``\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10a0UW\x91a0\x1Da0I\x92a0O\x94\x87\x87a28V[a0Ca0>\x86a\x11\x81\x87`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a7WV[Pa/`V[\x94a/nV[\x93a/\xB0V[PPPPP\x90PV[PPP\x90PV[Pa\xFF\xFF\x81\x10a/\xB9V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x94\x90a0\xA2\x90a\x1A\x93V[\x94[\x85\x15\x15\x80a1KW[\x15a1CWa0\xD3a/\xDF\x85a\x11\x81\x86`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[a0\xDE\x81\x86\x86a-\x89V[\x90a0\xF3a\x1Bg``\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10a19W\x91a1\x0Ca1-\x92a13\x94\x88\x88a28V[a0Ca0>\x87a\x11\x81\x88`\x01\x80`\xA0\x1B\x03\x16_R`\x9A` R`@_ \x90V[\x95a/nV[\x94a0\xA4V[PPP\x93PPPPV[P\x93PPPPV[Pa\xFF\xFF\x85\x16\x81\x10a0\xADV[_\x19\x81\x01\x91\x90\x82\x11a\x1D\x91WV[\x80T\x80a1{WPPg\r\xE0\xB6\xB3\xA7d\0\0\x90V[\x80_\x19\x81\x01\x11a\x1D\x91W`\x01`\x01`@\x1B\x03\x91_R_\x19\x90` _ \x01\x01T` \x1C\x16\x90V[`\x01`\x01`@\x1B\x03\x80\x91\x16`\x0F\x0B\x91\x16`\x0F\x0B\x03`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17a\x1D\x91W\x90V[`\x01`\x01`@\x1B\x03\x91\x82a1\xE9\x92\x16`\x0F\x0Ba(\xF5V[\x16\x90V[\x90\x81T`\x80\x1D\x90a2\x0C\x82`\x01\x85\x01\x90`\x0F\x0B_R` R`@_ \x90V[U\x81T`\x01`\x01`\x80\x1B\x03\x16`\x01\x90\x91\x01`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[\x90\x91a3\x86\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x94a3(a3\xB9\x93a2\xC8`\x01`\x01`@\x1B\x03` \x85\x01Q\x16\x91c\xFF\xFF\xFF\xFF`@\x86\x01Q`\x0F\x0B\x81``\x88\x01Q\x16\x90`@Q\x95a2\x9A\x87a\x0ByV[\x86R`\x0F\x0B` \x86\x01R\x16`@\x84\x01Ra\x1BA\x89a\x11\x81\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\x99` R`@_ \x90V[\x81Q\x81T` \x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x91\x90\x91\x1B\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x95\x1B\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x91\x90\x91\x17\x90UV[a3ya3<\x82Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x98` R`@\x90 a3^\x90\x88\x90a\x11\x81V[\x90`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[Q`\x01`\x01`@\x1B\x03\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x81\x90``\x82\x01\x90V[\x03\x90\xA1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x15a4\rWV[c\xDD\x18\x18\x15`\xE0\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x1D\x91WV[\x91\x90\x82\x01\x80\x92\x11a\x1D\x91WV[\x90\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91a4kc\xFF\xFF\xFF\xFF\x83\x16\x15\x15a4\x06V[a3\xB9a5\x89a4\x93a4\x8E\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90V[a*\xC1V[a4\xC9\x85` \x83\x01a4\xA9\x81Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x80a5\xC8W[a5\xBAW[P\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a5\x7Fa4\xFFa\x1Bgc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba4*V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01\x90\x81R\x91`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9B` R`@\x90 \x81Q\x81Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x81U\x90` \x81\x81\x01Q\x83T`@\x93\x84\x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x91\x16\x91\x90\x92\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x91\x1Bk\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x16\x17\x90UV[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x85\x01R\x93\x16\x92\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[c\xFF\xFF\xFF\xFF\x16\x84R_a4\xBDV[Pa5\xDDa\x1Bg`@\x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[B\x10\x15a4\xB8V[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x1D\x91WV[\x80T\x90\x91`\x01`\x01`\xFF\x1B\x03\x81\x11a6hWa\x07\xEDa6-a6(a64\x93\x85`\x0F\x0Ba5\xE5V[a;XV[\x92`\x80\x1D\x90V[\x81`\x0F\x0B\x12\x15a6YW`\x01a6U\x92\x01\x90`\x0F\x0B_R` R`@_ \x90V[T\x90V[c-\x04\x83\xC5`\xE2\x1B_R`\x04_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a6\xD2WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a7+\x81T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[a7HW\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[c\x1E\xD9P\x95`\xE1\x1B_R`\x04_\xFD[\x90a7m\x82T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[a7HW\x81T`\x0F\x0B\x91`\x01\x81\x01\x92_a7\xA8\x82a7\x96\x81\x88\x90`\x0F\x0B_R` R`@_ \x90V[T\x96\x90`\x0F\x0B_R` R`@_ \x90V[U`\x01`\x01`\x01`\x80\x1B\x03\x19\x83T\x16\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x17\x90UV[\x81\x15a7\xD1W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\x01\xFAWV[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14a8UW\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x01\xFAW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14a8\xFAWg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91a8\xA6\x86\x84\x11a7\xE5V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x0C\xB9\x92Pa7\xC7V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14a90W\x90\x82\x91a8\xA6\x86\x84\x11a7\xE5V[PP\x90a\x0C\xB9\x92Pa7\xC7V[\x90\x81Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0BtW`\x01\x81\x01\x80\x84U\x81\x10\x15a#\x8EW` `\x01`\x01`@\x1B\x03\x91a\x0B\xC4\x94_R\x81_ \x01\x92a9\x96c\xFF\xFF\xFF\xFF\x82Q\x16\x85\x90c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19\x82T\x16\x17\x90UV[\x01Q\x82Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x91\x16` \x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90UV[\x90`@Qa9\xCE\x81a\x0BYV[` `\x01`\x01`@\x1B\x03\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84R\x82\x1C\x16\x91\x01RV[\x15a9\xF3WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x92\x91\x92\x83\x82T\x80\x15\x15_\x14a;.W\x92` \x92\x91\x84a:pa:ka:`a:\xF4\x98a1XV[\x85_R` _ \x01\x90V[a9\xC1V[\x93c\xFF\xFF\xFF\xFFa:\x95a:\x87\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x82\x84\x16\x92\x83\x91\x16\x11\x15a9\xECV[a:\xA6a\x1Bg\x87Qc\xFF\xFF\xFF\xFF\x16\x90V[\x03a:\xF8WPa:\xE6\x92a\x1D\"a:\xBC\x92a1XV[\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x82T\x91` \x1B\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16\x17\x90UV[\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x90V[\x91PPa;)\x91a;\x16a;\na\x0B\xC6V[c\xFF\xFF\xFF\xFF\x90\x93\x16\x83RV[`\x01`\x01`@\x1B\x03\x88\x16\x82\x86\x01Ra9=V[a:\xE6V[PPa;S\x91a;?a;\na\x0B\xC6V[`\x01`\x01`@\x1B\x03\x85\x16` \x83\x01Ra9=V[_\x91\x90V[`\x01`\x01`\x7F\x1B\x03\x19\x81\x12\x15\x80a;\xCAW[\x15a;uW`\x0F\x0B\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[P`\x01`\x01`\x7F\x1B\x03\x81\x13\x15a;jV\xFE\xA2dipfsX\"\x12 )\xA6\xB3\xD1\x8F\xD75\xAF\x7F2\xC8\xFC\xF7v\\\x90\n\xC2\xA6\xF8\xFCt\xB5\xD7\xAB\x04\x1C\xCB\xEAnV\xCAdsolcC\0\x08\x1B\x003",
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
    /**Custom error with signature `InvalidAllocationDelay()` and selector `0xdd181815`.
```solidity
error InvalidAllocationDelay();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAllocationDelay {}
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
        impl ::core::convert::From<InvalidAllocationDelay> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAllocationDelay) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAllocationDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAllocationDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAllocationDelay()";
            const SELECTOR: [u8; 4] = [221u8, 24u8, 24u8, 21u8];
            #[inline]
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
    /**Custom error with signature `InvalidExpectedTotalMagnitude()` and selector `0xe8fe818c`.
```solidity
error InvalidExpectedTotalMagnitude();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidExpectedTotalMagnitude {}
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
        impl ::core::convert::From<InvalidExpectedTotalMagnitude>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidExpectedTotalMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidExpectedTotalMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidExpectedTotalMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidExpectedTotalMagnitude()";
            const SELECTOR: [u8; 4] = [232u8, 254u8, 129u8, 140u8];
            #[inline]
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
event MaxMagnitudeUpdated(address operator, address strategy, uint64 totalMagnitude);
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
        pub totalMagnitude: u64,
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
                    totalMagnitude: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.totalMagnitude),
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
function encumberedMagnitude(address, address) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encumberedMagnitudeCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
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
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for encumberedMagnitudeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
    /**Function with signature `getMinDelegatedAndSlashableOperatorShares((address,uint32),address[],address[],uint32)` and selector `0xe07d2b12`.
```solidity
function getMinDelegatedAndSlashableOperatorShares(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 beforeTimestamp) external view returns (uint256[][] memory, uint256[][] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelegatedAndSlashableOperatorSharesCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub beforeTimestamp: u32,
    }
    ///Container type for the return parameters of the [`getMinDelegatedAndSlashableOperatorShares((address,uint32),address[],address[],uint32)`](getMinDelegatedAndSlashableOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelegatedAndSlashableOperatorSharesReturn {
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
            impl ::core::convert::From<getMinDelegatedAndSlashableOperatorSharesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinDelegatedAndSlashableOperatorSharesCall) -> Self {
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
            for getMinDelegatedAndSlashableOperatorSharesCall {
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
            impl ::core::convert::From<getMinDelegatedAndSlashableOperatorSharesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinDelegatedAndSlashableOperatorSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinDelegatedAndSlashableOperatorSharesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMinDelegatedAndSlashableOperatorSharesCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinDelegatedAndSlashableOperatorSharesReturn;
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
            const SIGNATURE: &'static str = "getMinDelegatedAndSlashableOperatorShares((address,uint32),address[],address[],uint32)";
            const SELECTOR: [u8; 4] = [224u8, 125u8, 43u8, 18u8];
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
        getMaxMagnitudes(getMaxMagnitudesCall),
        getMaxMagnitudesAtTimestamp(getMaxMagnitudesAtTimestampCall),
        getMinDelegatedAndSlashableOperatorShares(
            getMinDelegatedAndSlashableOperatorSharesCall,
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
            [169u8, 132u8, 235u8, 58u8],
            [185u8, 251u8, 174u8, 209u8],
            [223u8, 92u8, 247u8, 35u8],
            [224u8, 125u8, 43u8, 18u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerCalls {
        const NAME: &'static str = "AllocationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 29usize;
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
                Self::getMaxMagnitudes(_) => {
                    <getMaxMagnitudesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudesAtTimestamp(_) => {
                    <getMaxMagnitudesAtTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMinDelegatedAndSlashableOperatorShares(_) => {
                    <getMinDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn getMinDelegatedAndSlashableOperatorShares(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMinDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerCalls::getMinDelegatedAndSlashableOperatorShares,
                            )
                    }
                    getMinDelegatedAndSlashableOperatorShares
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
                Self::getMinDelegatedAndSlashableOperatorShares(inner) => {
                    <getMinDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getMinDelegatedAndSlashableOperatorShares(inner) => {
                    <getMinDelegatedAndSlashableOperatorSharesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        InvalidAllocationDelay(InvalidAllocationDelay),
        InvalidExpectedTotalMagnitude(InvalidExpectedTotalMagnitude),
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
            [171u8, 69u8, 137u8, 35u8],
            [180u8, 18u8, 15u8, 20u8],
            [183u8, 208u8, 148u8, 151u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 117u8, 57u8, 76u8],
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 252u8, 190u8, 48u8],
            [221u8, 24u8, 24u8, 21u8],
            [232u8, 254u8, 129u8, 140u8],
            [247u8, 57u8, 88u8, 155u8],
            [250u8, 85u8, 252u8, 129u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerErrors {
        const NAME: &'static str = "AllocationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
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
                Self::InvalidAllocationDelay(_) => {
                    <InvalidAllocationDelay as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidExpectedTotalMagnitude(_) => {
                    <InvalidExpectedTotalMagnitude as alloy_sol_types::SolError>::SELECTOR
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
                    fn InvalidAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidAllocationDelay as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidAllocationDelay)
                    }
                    InvalidAllocationDelay
                },
                {
                    fn InvalidExpectedTotalMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidExpectedTotalMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerErrors::InvalidExpectedTotalMagnitude)
                    }
                    InvalidExpectedTotalMagnitude
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
                Self::InvalidAllocationDelay(inner) => {
                    <InvalidAllocationDelay as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidExpectedTotalMagnitude(inner) => {
                    <InvalidExpectedTotalMagnitude as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InvalidAllocationDelay(inner) => {
                    <InvalidAllocationDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidExpectedTotalMagnitude(inner) => {
                    <InvalidExpectedTotalMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
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
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, encumberedMagnitudeCall, N> {
            self.call_builder(&encumberedMagnitudeCall { _0, _1 })
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
        ///Creates a new call builder for the [`getMinDelegatedAndSlashableOperatorShares`] function.
        pub fn getMinDelegatedAndSlashableOperatorShares(
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
            getMinDelegatedAndSlashableOperatorSharesCall,
            N,
        > {
            self.call_builder(
                &getMinDelegatedAndSlashableOperatorSharesCall {
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
