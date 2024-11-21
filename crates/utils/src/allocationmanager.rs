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
use crate::iavsdirectory::IAVSDirectory::OperatorSet;

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
        pub operatorSets:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<u64>,
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
        impl alloy_sol_types::SolType for MagnitudeAllocation {
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
        impl alloy_sol_types::SolStruct for MagnitudeAllocation {
            const NAME: &'static str = "MagnitudeAllocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MagnitudeAllocation(address strategy,uint64 expectedMaxMagnitude,OperatorSet[] operatorSets,uint64[] magnitudes)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<OperatorSet as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<OperatorSet as alloy_sol_types::SolStruct>::eip712_components());
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                (
                    value.currentMagnitude,
                    value.pendingDiff,
                    value.effectTimestamp,
                )
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.currentMagnitude,
                    ),
                    <alloy::sol_types::sol_data::Int<128> as alloy_sol_types::SolType>::tokenize(
                        &self.pendingDiff,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.effectTimestamp,
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
        impl alloy_sol_types::SolType for MagnitudeInfo {
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
        impl alloy_sol_types::SolStruct for MagnitudeInfo {
            const NAME: &'static str = "MagnitudeInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "MagnitudeInfo(uint64 currentMagnitude,int128 pendingDiff,uint32 effectTimestamp)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for SlashingParams {
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
        impl alloy_sol_types::SolStruct for SlashingParams {
            const NAME: &'static str = "SlashingParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256 wadToSlash,string description)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
    pub struct IAllocationManagerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
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
        > IAllocationManagerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IAllocationManagerTypes`](self) contract instance.

        See the [wrapper's documentation](`IAllocationManagerTypesInstance`) for more details.*/
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
        > IAllocationManagerTypesInstance<T, P, N>
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
        > IAllocationManagerTypesInstance<T, P, N>
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
    ///0x610100604052348015610010575f5ffd5b5060405161429338038061429383398101604081905261002f9161014e565b6001600160a01b03808516608052831660a05263ffffffff80831660c052811660e05261005a610063565b505050506101a4565b5f54610100900460ff16156100ce5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161461011d575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610133575f5ffd5b50565b805163ffffffff81168114610149575f5ffd5b919050565b5f5f5f5f60808587031215610161575f5ffd5b845161016c8161011f565b602086015190945061017d8161011f565b925061018b60408601610136565b915061019960608601610136565b905092959194509250565b60805160a05160c05160e05161407661021d5f395f81816103ed015261298d01525f81816102460152610b3901525f818161037b015281816108dd015281816110be015261148701525f81816104ad01528181610f93015281816112160152818161133c015281816119b90152611e8201526140765ff3fe608060405234801561000f575f5ffd5b50600436106101c6575f3560e01c80635c975abb116100fe578063886f11951161009e578063df5cf7231161006e578063df5cf723146104a8578063e07d2b12146104cf578063f2fde38b146104f0578063fabc1cbc14610503575f5ffd5b8063886f1195146104225780638da5cb5b14610435578063a984eb3a14610446578063b9fbaed114610479575f5ffd5b80636cfb4481116100d95780636cfb4481146103b5578063715018a6146103e05780637bc1ef61146103e8578063843b349f1461040f575f5ffd5b80635c975abb1461035257806360db99a3146103635780636b3aa72e14610376575f5ffd5b80634b5046ef1161016957806356c483e61161014457806356c483e6146102f1578063595c6a67146103045780635ac86ab71461030c5780635c489bb51461033f575f5ffd5b80634b5046ef1461029d5780634d9dbde9146102b0578063547afb87146102d1575f5ffd5b80631637b60f116101a45780631637b60f1461021b5780631794bb3c1461022e5780632981eb771461024157806335af054a1461027d575f5ffd5b80630b002119146101ca57806310d67a2f146101f3578063136439dd14610208575b5f5ffd5b6101dd6101d83660046130e2565b610516565b6040516101ea9190613161565b60405180910390f35b61020661020136600461322c565b610677565b005b610206610216366004613247565b610728565b61020661022936600461325e565b610811565b61020661023c36600461329c565b610d06565b6102687f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016101ea565b61029061028b3660046133b9565b610e25565b6040516101ea91906134e3565b6102066102ab3660046134f5565b610f2c565b6102c36102be366004613558565b61108b565b6040516101ea92919061358f565b6102e46102df36600461360a565b611146565b6040516101ea919061365a565b6102066102ff3660046136a5565b61120b565b610206611262565b61032f61031a3660046136d1565b606654600160ff9092169190911b9081161490565b60405190151581526020016101ea565b61020661034d3660046136f1565b611327565b6066546040519081526020016101ea565b61020661037136600461370c565b6113d4565b61039d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101ea565b6103c86103c3366004613558565b611b40565b6040516001600160401b0390911681526020016101ea565b610206611caf565b6102687f000000000000000000000000000000000000000000000000000000000000000081565b6102e461041d366004613742565b611cc2565b60655461039d906001600160a01b031681565b6033546001600160a01b031661039d565b6103c8610454366004613558565b609860209081525f92835260408084209091529082529020546001600160401b031681565b61048c61048736600461322c565b611db2565b60408051921515835263ffffffff9091166020830152016101ea565b61039d7f000000000000000000000000000000000000000000000000000000000000000081565b6104e26104dd3660046137a5565b611e41565b6040516101ea9291906138c0565b6102066104fe36600461322c565b6121a2565b610206610511366004613247565b612218565b60605f826001600160401b03811115610531576105316132da565b60405190808252806020026020018201604052801561056457816020015b606081526020019060019003908161054f5790505b5090505f5b8381101561066c575f5b86811015610663575f6105e9878785818110610591576105916138ed565b90506020020160208101906105a6919061322c565b8a8a858181106105b8576105b86138ed565b90506020020160208101906105cd919061322c565b6105e46105df368f90038f018f613901565b61231d565b61237b565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250848481518110610636576106366138ed565b6020026020010151838151811061064f5761064f6138ed565b602090810291909101015250600101610573565b50600101610569565b509695505050505050565b60655f9054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106eb919061391b565b6001600160a01b0316336001600160a01b03161461071c5760405163794821ff60e01b815260040160405180910390fd5b610725816124e0565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561076e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107929190613936565b6107af57604051631d77d47760e21b815260040160405180910390fd5b606654818116146107d35760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6066545f906001908116036108395760405163840a48d560e01b815260040160405180910390fd5b5f5f61084433611db2565b91509150816108665760405163fa55fc8160e01b815260040160405180910390fd5b5f5b84811015610cfe5736868683818110610883576108836138ed565b90506020028101906108959190613955565b90506108a46060820182613973565b90506108b360408301836139b8565b9050146108d3576040516343714afd60e01b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166320c4e23661090f60408401846139b8565b6040518363ffffffff1660e01b815260040161092c929190613a32565b602060405180830381865afa158015610947573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061096b9190613936565b61098857604051631fb1705560e21b815260040160405180910390fd5b6109a133610999602084018461322c565b61ffff612570565b335f90815260976020908152604082206109e79183906109c39086018661322c565b6001600160a01b03166001600160a01b031681526020019081526020015f20612675565b90506109f96040830160208401613a69565b6001600160401b0316816001600160401b031614610a2a57604051633a3fa06360e21b815260040160405180910390fd5b5f5b610a3960408401846139b8565b9050811015610cf0575f610a79610a5360408601866139b8565b84818110610a6357610a636138ed565b9050604002018036038101906105df9190613901565b90505f610a9333610a8d602088018861322c565b8461237b565b90508060400151600f0b5f14610abc57604051630d8fcbe360e41b815260040160405180910390fd5b6020810151610afc90610ad26060880188613973565b86818110610ae257610ae26138ed565b9050602002016020810190610af79190613a69565b6126c1565b600f0b604082018190525f03610b2557604051634606179360e11b815260040160405180910390fd5b5f8160400151600f0b1215610be457610b5e7f000000000000000000000000000000000000000000000000000000000000000042613aa3565b63ffffffff166060820152335f908152609a602090815260408220610bdf928592610b8b908a018a61322c565b6001600160a01b0316815260208101919091526040015f20908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b610c49565b5f8160400151600f0b1315610c4957610bfd8742613aa3565b63ffffffff16606082015280516040820151610c1991906126d8565b6001600160401b039081168083529085161015610c495760405163329d4e5360e21b815260040160405180910390fd5b610c6133610c5a602088018861322c565b84846126ec565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf633610c9060408801886139b8565b86818110610ca057610ca06138ed565b604002919091019050610cb6602089018961322c565b610cc8856020015186604001516126d8565b8560600151604051610cde959493929190613abf565b60405180910390a15050600101610a2c565b505050806001019050610868565b505050505050565b5f54610100900460ff1615808015610d2457505f54600160ff909116105b80610d3d5750303b158015610d3d57505f5460ff166001145b610da55760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015610dc6575f805461ff0019166101001790555b610dd08383612809565b610dd98461288a565b8015610e1f575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b60605f82516001600160401b03811115610e4157610e416132da565b604051908082528060200260200182016040528015610e8a57816020015b604080516060810182525f80825260208083018290529282015282525f19909201910181610e5f5790505b5090505f5b8351811015610f21575f610ec087876105e4888681518110610eb357610eb36138ed565b602002602001015161231d565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250838381518110610f0d57610f0d6138ed565b602090810291909101015250600101610e8f565b5090505b9392505050565b6066545f90600190811603610f545760405163840a48d560e01b815260040160405180910390fd5b838214610f74576040516343714afd60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0387811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015610fd8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ffc9190613936565b611019576040516325ec6c1f60e01b815260040160405180910390fd5b5f5b848110156110825761107a87878784818110611039576110396138ed565b905060200201602081019061104e919061322c565b868685818110611060576110606138ed565b90506020020160208101906110759190613b10565b612570565b60010161101b565b50505050505050565b6040516316ae76cb60e01b81526001600160a01b0383811660048301525f602483018190525f19604484015260609283927f000000000000000000000000000000000000000000000000000000000000000016906316ae76cb906064015f60405180830381865afa158015611102573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526111299190810190613b31565b90505f611137868684610e25565b919350909150505b9250929050565b60605f826001600160401b03811115611161576111616132da565b60405190808252806020026020018201604052801561118a578160200160208202803683370190505b5090505f5b83811015610f21576001600160a01b0386165f9081526097602052604081206111d9918787858181106111c4576111c46138ed565b90506020020160208101906109c3919061322c565b8282815181106111eb576111eb6138ed565b6001600160401b039092166020928302919091019091015260010161118f565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112545760405163f739589b60e01b815260040160405180910390fd5b61125e82826128db565b5050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156112a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112cc9190613936565b6112e957604051631d77d47760e21b815260040160405180910390fd5b5f19606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa158015611389573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113ad9190613936565b6113ca576040516325ec6c1f60e01b815260040160405180910390fd5b61072533826128db565b6066546001906002908116036113fd5760405163840a48d560e01b815260040160405180910390fd5b81606001355f10801561141c5750670de0b6b3a7640000606083013511155b61143957604051631353603160e01b815260040160405180910390fd5b5f6040518060400160405280336001600160a01b0316815260200184602001602081019061146791906136f1565b63ffffffff16905290505f61147b8261231d565b90506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631352c3e66114b9602087018761322c565b846040518363ffffffff1660e01b81526004016114d7929190613bf2565b602060405180830381865afa1580156114f2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115169190613936565b6115335760405163ccea9e6f60e01b815260040160405180910390fd5b5f6115416040860186613973565b90506001600160401b0381111561155a5761155a6132da565b604051908082528060200260200182016040528015611583578160200160208202803683370190505b5090505f5b6115956040870187613973565b9050811015611ad3575f6115e76115af602089018961322c565b6115bc60408a018a613973565b858181106115cc576115cc6138ed565b90506020020160208101906115e1919061322c565b8661237b565b90505f81602001516001600160401b03161161161657604051634e99e6cf60e01b815260040160405180910390fd5b60208101515f90611634906001600160401b031660608a0135612a68565b905080826020018181516116489190613c28565b6001600160401b0316905250815181908390611665908390613c28565b6001600160401b031690525060408201515f600f9190910b121561175f575f6116a98960600135846040015161169a90613c47565b6001600160801b031690612a68565b9050806001600160401b0316836040018181516116c69190613c6b565b600f0b9052507f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66116fa60208b018b61322c565b8861170860408d018d613973565b88818110611718576117186138ed565b905060200201602081019061172d919061322c565b61173f876020015188604001516126d8565b8760600151604051611755959493929190613c98565b60405180910390a1505b6117a861176f60208a018a61322c565b61177c60408b018b613973565b8681811061178c5761178c6138ed565b90506020020160208101906117a1919061322c565b87856126ec565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66117d660208a018a61322c565b876117e460408c018c613973565b878181106117f4576117f46138ed565b9050602002016020810190611809919061322c565b856020015142604051611820959493929190613c98565b60405180910390a15f61187c60978261183c60208d018d61322c565b6001600160a01b03166001600160a01b031681526020019081526020015f205f8b806040019061186c9190613973565b888181106111c4576111c46138ed565b90505f6118898383613c28565b9050611919428260975f8e5f0160208101906118a5919061322c565b6001600160a01b03166001600160a01b031681526020019081526020015f205f8e80604001906118d59190613973565b8b8181106118e5576118e56138ed565b90506020020160208101906118fa919061322c565b6001600160a01b0316815260208101919091526040015f209190612a7c565b507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c905061194a60208c018c61322c565b61195760408d018d613973565b88818110611967576119676138ed565b905060200201602081019061197c919061322c565b604080516001600160a01b0393841681529290911660208301526001600160401b0384169082015260600160405180910390a16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a57ab10b6119eb60208d018d61322c565b6119f860408e018e613973565b89818110611a0857611a086138ed565b9050602002016020810190611a1d919061322c565b6040516001600160e01b031960e085901b1681526001600160a01b039283166004820152911660248201526001600160401b038086166044830152841660648201526084015f604051808303815f87803b158015611a79575f5ffd5b505af1158015611a8b573d5f5f3e3d5ffd5b50611aa6925050506001600160401b03848116908416612a96565b868681518110611ab857611ab86138ed565b60200260200101818152505050505050806001019050611588565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5611b02602087018761322c565b84611b106040890189613973565b85611b1e60808c018c613cce565b604051611b319796959493929190613d4e565b60405180910390a15050505050565b6001600160a01b038281165f81815260986020908152604080832094861680845294825280832054938352609a8252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611c6a576001600160a01b038087165f908152609a602090815260408083209389168352929052908120611bd29083612aaa565b6001600160a01b038881165f908152609960209081526040808320938b168352928152828220848352815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250421015611c4d575050611c6a565b611c5b8582602001516126d8565b94505050806001019050611b9a565b506001600160a01b038086165f9081526097602090815260408083209388168352929052208290611c9a90612675565b611ca49190613c28565b925050505b92915050565b611cb7612b19565b611cc05f61288a565b565b60605f836001600160401b03811115611cdd57611cdd6132da565b604051908082528060200260200182016040528015611d06578160200160208202803683370190505b5090505f5b84811015611da8576001600160a01b0387165f908152609760205260408120611d7691869190898986818110611d4357611d436138ed565b9050602002016020810190611d58919061322c565b6001600160a01b0316815260208101919091526040015f2090612b73565b828281518110611d8857611d886138ed565b6001600160401b0390921660209283029190910190910152600101611d0b565b5095945050505050565b6001600160a01b0381165f908152609b602090815260408083208151606081018352905463ffffffff8082168352600160201b82048116948301859052600160401b9091041691810191909152829115801590611e195750806040015163ffffffff164210155b15611e2a5780602001519150611e2f565b805191505b8163ffffffff165f1415925050915091565b606080428363ffffffff1611611e6a5760405163b7d0949760e01b815260040160405180910390fd5b5f611e7d6105df368b90038b018b613901565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e6768a8a8a8a6040518563ffffffff1660e01b8152600401611ed29493929190613de0565b5f60405180830381865afa158015611eec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611f139190810190613e42565b90505f886001600160401b03811115611f2e57611f2e6132da565b604051908082528060200260200182016040528015611f6157816020015b6060815260200190600190039081611f4c5790505b5090505f5b89811015612191575f8b8b83818110611f8157611f816138ed565b9050602002016020810190611f96919061322c565b9050886001600160401b03811115611fb057611fb06132da565b604051908082528060200260200182016040528015611fd9578160200160208202803683370190505b50838381518110611fec57611fec6138ed565b60209081029190910101525f5b89811015612187575f8b8b83818110612014576120146138ed565b9050602002016020810190612029919061322c565b6001600160a01b038085165f90815260996020908152604080832093851683529281528282208b8352815290829020825160608101845290546001600160401b038116808352600160401b8204600f0b9383019390935263ffffffff600160c01b9091048116938201849052939450929091908d16106120b4576120b18183602001516126d8565b90505b6001600160a01b038086165f908152609760209081526040808320938716835292905220612142906120e590612675565b6001600160401b031661213c836001600160401b03168b8a8151811061210d5761210d6138ed565b60200260200101518881518110612126576121266138ed565b6020026020010151612a6890919063ffffffff16565b90612a96565b878781518110612154576121546138ed565b6020026020010151858151811061216d5761216d6138ed565b602002602001018181525050505050806001019050611ff9565b5050600101611f66565b50909a909950975050505050505050565b6121aa612b19565b6001600160a01b03811661220f5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610d9c565b6107258161288a565b60655f9054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612268573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061228c919061391b565b6001600160a01b0316336001600160a01b0316146122bd5760405163794821ff60e01b815260040160405180910390fd5b6066541981196066541916146122e65760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610806565b5f815f0151826020015163ffffffff1660405160200161236392919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611ca990613f43565b604080516080810182525f808252602080830182905282840182905260608084018390526001600160a01b0388811680855260998452868520918916808652918452868520888652845286852087519384018852546001600160401b038082168552600160401b8204600f0b8587015263ffffffff600160c01b9092048216858a019081529287526098865288872093875292909452959093205494519394909392169116421015612477576040518060800160405280826001600160401b03168152602001835f01516001600160401b031681526020018360200151600f0b8152602001836040015163ffffffff1681525092505050610f25565b612488825f015183602001516126d8565b6001600160401b0390811660208086019190915290821684525f606085018190526040850181905290830151600f0b12156124d7576124cb8183602001516126d8565b6001600160401b031683525b50509392505050565b6001600160a01b038116612507576040516339b190bb60e11b815260040160405180910390fd5b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038381165f908152609a60209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f811180156125ba57508261ffff1682105b1561266e576001600160a01b038086165f908152609a6020908152604080832093881683529290529081206125ee90612bc3565b90505f6125fc87878461237b565b9050806060015163ffffffff1642101561261757505061266e565b612623878784846126ec565b6001600160a01b038088165f908152609a60209081526040808320938a1683529290522061265090612c15565b5061265a84613f66565b935061266583613f7e565b925050506125a8565b5050505050565b80545f9080156126b15761269b8361268e600184613f93565b5f91825260209091200190565b54600160201b90046001600160401b0316610f25565b670de0b6b3a76400009392505050565b5f610f256001600160401b03808516908416613fa6565b5f610f25826001600160401b038516613c6b565b60408051606080820183526020848101516001600160401b03908116845285850151600f0b8285019081528684015163ffffffff9081168688019081526001600160a01b038c81165f818152609988528a8120928e168082529288528a81208d825288528a812099518a5496519451909516600160c01b0263ffffffff60c01b196001600160801b03909516600160401b026001600160c01b031990971695881695909517959095179290921692909217909655875186835260988552878320828452855291879020805492841667ffffffffffffffff1990931692909217909155865186519586529285015216928201929092527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559101610e16565b6065546001600160a01b031615801561282a57506001600160a01b03821615155b612847576040516339b190bb60e11b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261125e826124e0565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b8063ffffffff165f036129015760405163dd18181560e01b815260040160405180910390fd5b6001600160a01b0382165f908152609b60209081526040918290208251606081018452905463ffffffff8082168352600160201b82048116938301849052600160401b9091041692810192909252158015906129675750806040015163ffffffff164210155b1561297a57602081015163ffffffff1681525b63ffffffff80831660208301526129b3907f00000000000000000000000000000000000000000000000000000000000000001642613fd3565b63ffffffff90811660408381019182526001600160a01b0386165f818152609b6020908152908390208651815483890151965191881667ffffffffffffffff1990911617600160201b96881696909602959095176bffffffff00000000000000001916600160401b9587169586021790558251918252938616938101939093528201527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9060600160405180910390a1505050565b5f610f258383670de0b6b3a7640000612c92565b5f80612a89858585612d77565b915091505b935093915050565b5f610f2583670de0b6b3a764000084612c92565b5f5f612acc612ab884612f41565b8554612ac79190600f0b613fe6565b612fae565b8454909150600160801b9004600f90810b9082900b12612aff57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b6033546001600160a01b03163314611cc05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610d9c565b81545f9081612b8485858385613017565b90508015612bb157612b9b8561268e600184613f93565b54600160201b90046001600160401b0316611ca4565b50670de0b6b3a7640000949350505050565b5f612bdd8254600f81810b600160801b909204900b131590565b15612bfb57604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f612c2f8254600f81810b600160801b909204900b131590565b15612c4d57604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b5f80805f19858709858702925082811083820303915050805f03612cc957838281612cbf57612cbf61400d565b0492505050610f25565b808411612d105760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401610d9c565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b82545f9081908015612ed5575f612d938761268e600185613f93565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b031660208401529192509087161015612e145760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401610d9c565b805163ffffffff808816911603612e655784612e358861268e600186613f93565b80546001600160401b0392909216600160201b026bffffffffffffffff0000000019909216919091179055612ec5565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d555f8d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b602001519250839150612a8e9050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a555f8a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081612a8e565b5f6001600160ff1b03821115612faa5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401610d9c565b5090565b80600f81900b81146130125760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401610d9c565b919050565b5f5b8183101561306a575f61302c8484613072565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561305657809250613064565b613061816001613fd3565b93505b50613019565b509392505050565b5f6130806002848418614021565b610f2590848416613fd3565b5f6040828403121561309c575f5ffd5b50919050565b5f5f83601f8401126130b2575f5ffd5b5081356001600160401b038111156130c8575f5ffd5b6020830191508360208260051b850101111561113f575f5ffd5b5f5f5f5f5f608086880312156130f6575f5ffd5b613100878761308c565b945060408601356001600160401b0381111561311a575f5ffd5b613126888289016130a2565b90955093505060608601356001600160401b03811115613144575f5ffd5b613150888289016130a2565b969995985093965092949392505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561320c57868503603f19018452815180518087526020918201918701905f5b818110156131f357835180516001600160401b03168452602080820151600f0b9085015260409081015163ffffffff1690840152606083016020949094019392506001016131aa565b5090965050506020938401939190910190600101613187565b50929695505050505050565b6001600160a01b0381168114610725575f5ffd5b5f6020828403121561323c575f5ffd5b8135610f2581613218565b5f60208284031215613257575f5ffd5b5035919050565b5f5f6020838503121561326f575f5ffd5b82356001600160401b03811115613284575f5ffd5b613290858286016130a2565b90969095509350505050565b5f5f5f606084860312156132ae575f5ffd5b83356132b981613218565b925060208401356132c981613218565b929592945050506040919091013590565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715613310576133106132da565b60405290565b604051601f8201601f191681016001600160401b038111828210171561333e5761333e6132da565b604052919050565b5f6001600160401b0382111561335e5761335e6132da565b5060051b60200190565b63ffffffff81168114610725575f5ffd5b5f60408284031215613389575f5ffd5b6133916132ee565b9050813561339e81613218565b815260208201356133ae81613368565b602082015292915050565b5f5f5f606084860312156133cb575f5ffd5b83356133d681613218565b925060208401356133e681613218565b915060408401356001600160401b03811115613400575f5ffd5b8401601f81018613613410575f5ffd5b803561342361341e82613346565b613316565b8082825260208201915060208360061b850101925088831115613444575f5ffd5b6020840193505b828410156134705761345d8985613379565b825260208201915060408401935061344b565b809450505050509250925092565b5f8151808452602084019350602083015f5b828110156134d957815180516001600160401b03168752602080820151600f0b9088015260409081015163ffffffff169087015260608601955060209190910190600101613490565b5093949350505050565b602081525f610f25602083018461347e565b5f5f5f5f5f60608688031215613509575f5ffd5b853561351481613218565b945060208601356001600160401b0381111561352e575f5ffd5b61353a888289016130a2565b90955093505060408601356001600160401b03811115613144575f5ffd5b5f5f60408385031215613569575f5ffd5b823561357481613218565b9150602083013561358481613218565b809150509250929050565b604080825283519082018190525f9060208501906060840190835b818110156135ec576135d683855180516001600160a01b0316825260209081015163ffffffff16910152565b60209390930192604092909201916001016135aa565b50508381036020850152613600818661347e565b9695505050505050565b5f5f5f6040848603121561361c575f5ffd5b833561362781613218565b925060208401356001600160401b03811115613641575f5ffd5b61364d868287016130a2565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b8181101561369a5783516001600160401b0316835260209384019390920191600101613673565b509095945050505050565b5f5f604083850312156136b6575f5ffd5b82356136c181613218565b9150602083013561358481613368565b5f602082840312156136e1575f5ffd5b813560ff81168114610f25575f5ffd5b5f60208284031215613701575f5ffd5b8135610f2581613368565b5f6020828403121561371c575f5ffd5b81356001600160401b03811115613731575f5ffd5b820160a08185031215610f25575f5ffd5b5f5f5f5f60608587031215613755575f5ffd5b843561376081613218565b935060208501356001600160401b0381111561377a575f5ffd5b613786878288016130a2565b909450925050604085013561379a81613368565b939692955090935050565b5f5f5f5f5f5f60a087890312156137ba575f5ffd5b6137c4888861308c565b955060408701356001600160401b038111156137de575f5ffd5b6137ea89828a016130a2565b90965094505060608701356001600160401b03811115613808575f5ffd5b61381489828a016130a2565b909450925050608087013561382881613368565b809150509295509295509295565b5f8151808452602084019350602083015f5b828110156134d9578151865260209586019590910190600101613848565b5f82825180855260208501945060208160051b830101602085015f5b838110156138b457601f1985840301885261389e838351613836565b6020988901989093509190910190600101613882565b50909695505050505050565b604081525f6138d26040830185613866565b82810360208401526138e48185613866565b95945050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60408284031215613911575f5ffd5b610f258383613379565b5f6020828403121561392b575f5ffd5b8151610f2581613218565b5f60208284031215613946575f5ffd5b81518015158114610f25575f5ffd5b5f8235607e19833603018112613969575f5ffd5b9190910192915050565b5f5f8335601e19843603018112613988575f5ffd5b8301803591506001600160401b038211156139a1575f5ffd5b6020019150600581901b360382131561113f575f5ffd5b5f5f8335601e198436030181126139cd575f5ffd5b8301803591506001600160401b038211156139e6575f5ffd5b6020019150600681901b360382131561113f575f5ffd5b8035613a0881613218565b6001600160a01b031682526020810135613a2181613368565b63ffffffff81166020840152505050565b602080825281018290525f8360408301825b85811015611da857613a5682846139fd565b6040928301929190910190600101613a44565b5f60208284031215613a79575f5ffd5b81356001600160401b0381168114610f25575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b63ffffffff8181168382160190811115611ca957611ca9613a8f565b6001600160a01b038616815260c08101613adc60208301876139fd565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f60208284031215613b20575f5ffd5b813561ffff81168114610f25575f5ffd5b5f60208284031215613b41575f5ffd5b81516001600160401b03811115613b56575f5ffd5b8201601f81018413613b66575f5ffd5b8051613b7461341e82613346565b8082825260208201915060208360061b850101925086831115613b95575f5ffd5b6020840193505b828410156136005760408488031215613bb3575f5ffd5b613bbb6132ee565b8451613bc681613218565b81526020850151613bd681613368565b8060208301525080835250602082019150604084019350613b9c565b6001600160a01b038316815260608101610f25602083018480516001600160a01b0316825260209081015163ffffffff16910152565b6001600160401b038281168282160390811115611ca957611ca9613a8f565b5f81600f0b60016001607f1b03198103613c6357613c63613a8f565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715611ca957611ca9613a8f565b6001600160a01b038616815260c08101613adc602083018780516001600160a01b0316825260209081015163ffffffff16910152565b5f5f8335601e19843603018112613ce3575f5ffd5b8301803591506001600160401b03821115613cfc575f5ffd5b60200191503681900382131561113f575f5ffd5b8183526020830192505f815f5b848110156134d9578135613d3081613218565b6001600160a01b031686526020958601959190910190600101613d1d565b6001600160a01b0388168152613d80602082018880516001600160a01b0316825260209081015163ffffffff16910152565b60c060608201525f613d9660c083018789613d10565b8281036080840152613da88187613836565b905082810360a0840152838152838560208301375f602085830101526020601f19601f86011682010191505098975050505050505050565b604080825281018490525f8560608301825b87811015613e22578235613e0581613218565b6001600160a01b0316825260209283019290910190600101613df2565b508381036020850152613e36818688613d10565b98975050505050505050565b5f60208284031215613e52575f5ffd5b81516001600160401b03811115613e67575f5ffd5b8201601f81018413613e77575f5ffd5b8051613e8561341e82613346565b8082825260208201915060208360051b850101925086831115613ea6575f5ffd5b602084015b8381101561066c5780516001600160401b03811115613ec8575f5ffd5b8501603f81018913613ed8575f5ffd5b6020810151613ee961341e82613346565b808282526020820191506020808460051b8601010192508b831115613f0c575f5ffd5b6040840193505b82841015613f2e578351825260209384019390910190613f13565b86525050602093840193919091019050613eab565b8051602080830151919081101561309c575f1960209190910360031b1b16919050565b5f60018201613f7757613f77613a8f565b5060010190565b5f81613f8c57613f8c613a8f565b505f190190565b81810381811115611ca957611ca9613a8f565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715611ca957611ca9613a8f565b80820180821115611ca957611ca9613a8f565b8082018281125f83128015821682158216171561400557614005613a8f565b505092915050565b634e487b7160e01b5f52601260045260245ffd5b5f8261403b57634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220d7e22179daab227b85b04f5edb1f410400add9eae20f95a0a50c568e4fe4eb2764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@QaB\x938\x03\x80aB\x93\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01NV[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\x80R\x83\x16`\xA0Rc\xFF\xFF\xFF\xFF\x80\x83\x16`\xC0R\x81\x16`\xE0Ra\0Za\0cV[PPPPa\x01\xA4V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01\x1DW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x013W__\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01IW__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x01aW__\xFD[\x84Qa\x01l\x81a\x01\x1FV[` \x86\x01Q\x90\x94Pa\x01}\x81a\x01\x1FV[\x92Pa\x01\x8B`@\x86\x01a\x016V[\x91Pa\x01\x99``\x86\x01a\x016V[\x90P\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa@va\x02\x1D_9_\x81\x81a\x03\xED\x01Ra)\x8D\x01R_\x81\x81a\x02F\x01Ra\x0B9\x01R_\x81\x81a\x03{\x01R\x81\x81a\x08\xDD\x01R\x81\x81a\x10\xBE\x01Ra\x14\x87\x01R_\x81\x81a\x04\xAD\x01R\x81\x81a\x0F\x93\x01R\x81\x81a\x12\x16\x01R\x81\x81a\x13<\x01R\x81\x81a\x19\xB9\x01Ra\x1E\x82\x01Ra@v_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC6W_5`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xFEW\x80c\x88o\x11\x95\x11a\0\x9EW\x80c\xDF\\\xF7#\x11a\0nW\x80c\xDF\\\xF7#\x14a\x04\xA8W\x80c\xE0}+\x12\x14a\x04\xCFW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xF0W\x80c\xFA\xBC\x1C\xBC\x14a\x05\x03W__\xFD[\x80c\x88o\x11\x95\x14a\x04\"W\x80c\x8D\xA5\xCB[\x14a\x045W\x80c\xA9\x84\xEB:\x14a\x04FW\x80c\xB9\xFB\xAE\xD1\x14a\x04yW__\xFD[\x80cl\xFBD\x81\x11a\0\xD9W\x80cl\xFBD\x81\x14a\x03\xB5W\x80cqP\x18\xA6\x14a\x03\xE0W\x80c{\xC1\xEFa\x14a\x03\xE8W\x80c\x84;4\x9F\x14a\x04\x0FW__\xFD[\x80c\\\x97Z\xBB\x14a\x03RW\x80c`\xDB\x99\xA3\x14a\x03cW\x80ck:\xA7.\x14a\x03vW__\xFD[\x80cKPF\xEF\x11a\x01iW\x80cV\xC4\x83\xE6\x11a\x01DW\x80cV\xC4\x83\xE6\x14a\x02\xF1W\x80cY\\jg\x14a\x03\x04W\x80cZ\xC8j\xB7\x14a\x03\x0CW\x80c\\H\x9B\xB5\x14a\x03?W__\xFD[\x80cKPF\xEF\x14a\x02\x9DW\x80cM\x9D\xBD\xE9\x14a\x02\xB0W\x80cTz\xFB\x87\x14a\x02\xD1W__\xFD[\x80c\x167\xB6\x0F\x11a\x01\xA4W\x80c\x167\xB6\x0F\x14a\x02\x1BW\x80c\x17\x94\xBB<\x14a\x02.W\x80c)\x81\xEBw\x14a\x02AW\x80c5\xAF\x05J\x14a\x02}W__\xFD[\x80c\x0B\0!\x19\x14a\x01\xCAW\x80c\x10\xD6z/\x14a\x01\xF3W\x80c\x13d9\xDD\x14a\x02\x08W[__\xFD[a\x01\xDDa\x01\xD86`\x04a0\xE2V[a\x05\x16V[`@Qa\x01\xEA\x91\x90a1aV[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x02\x016`\x04a2,V[a\x06wV[\0[a\x02\x06a\x02\x166`\x04a2GV[a\x07(V[a\x02\x06a\x02)6`\x04a2^V[a\x08\x11V[a\x02\x06a\x02<6`\x04a2\x9CV[a\r\x06V[a\x02h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x02\x90a\x02\x8B6`\x04a3\xB9V[a\x0E%V[`@Qa\x01\xEA\x91\x90a4\xE3V[a\x02\x06a\x02\xAB6`\x04a4\xF5V[a\x0F,V[a\x02\xC3a\x02\xBE6`\x04a5XV[a\x10\x8BV[`@Qa\x01\xEA\x92\x91\x90a5\x8FV[a\x02\xE4a\x02\xDF6`\x04a6\nV[a\x11FV[`@Qa\x01\xEA\x91\x90a6ZV[a\x02\x06a\x02\xFF6`\x04a6\xA5V[a\x12\x0BV[a\x02\x06a\x12bV[a\x03/a\x03\x1A6`\x04a6\xD1V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xEAV[a\x02\x06a\x03M6`\x04a6\xF1V[a\x13'V[`fT`@Q\x90\x81R` \x01a\x01\xEAV[a\x02\x06a\x03q6`\x04a7\x0CV[a\x13\xD4V[a\x03\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x03\xC8a\x03\xC36`\x04a5XV[a\x1B@V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x02\x06a\x1C\xAFV[a\x02h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE4a\x04\x1D6`\x04a7BV[a\x1C\xC2V[`eTa\x03\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9DV[a\x03\xC8a\x04T6`\x04a5XV[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\x8Ca\x04\x876`\x04a2,V[a\x1D\xB2V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xEAV[a\x03\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xE2a\x04\xDD6`\x04a7\xA5V[a\x1EAV[`@Qa\x01\xEA\x92\x91\x90a8\xC0V[a\x02\x06a\x04\xFE6`\x04a2,V[a!\xA2V[a\x02\x06a\x05\x116`\x04a2GV[a\"\x18V[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x051Wa\x051a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05dW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05OW\x90P[P\x90P_[\x83\x81\x10\x15a\x06lW_[\x86\x81\x10\x15a\x06cW_a\x05\xE9\x87\x87\x85\x81\x81\x10a\x05\x91Wa\x05\x91a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x05\xA6\x91\x90a2,V[\x8A\x8A\x85\x81\x81\x10a\x05\xB8Wa\x05\xB8a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x05\xCD\x91\x90a2,V[a\x05\xE4a\x05\xDF6\x8F\x90\x03\x8F\x01\x8Fa9\x01V[a#\x1DV[a#{V[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x84\x84\x81Q\x81\x10a\x066Wa\x066a8\xEDV[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x06OWa\x06Oa8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05sV[P`\x01\x01a\x05iV[P\x96\x95PPPPPPV[`e_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xEB\x91\x90a9\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\x1CW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07%\x81a$\xE0V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x92\x91\x90a96V[a\x07\xAFW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x14a\x07\xD3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT_\x90`\x01\x90\x81\x16\x03a\x089W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x08D3a\x1D\xB2V[\x91P\x91P\x81a\x08fW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x0C\xFEW6\x86\x86\x83\x81\x81\x10a\x08\x83Wa\x08\x83a8\xEDV[\x90P` \x02\x81\x01\x90a\x08\x95\x91\x90a9UV[\x90Pa\x08\xA4``\x82\x01\x82a9sV[\x90Pa\x08\xB3`@\x83\x01\x83a9\xB8V[\x90P\x14a\x08\xD3W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c \xC4\xE26a\t\x0F`@\x84\x01\x84a9\xB8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t,\x92\x91\x90a:2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tk\x91\x90a96V[a\t\x88W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xA13a\t\x99` \x84\x01\x84a2,V[a\xFF\xFFa%pV[3_\x90\x81R`\x97` \x90\x81R`@\x82 a\t\xE7\x91\x83\x90a\t\xC3\x90\x86\x01\x86a2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a&uV[\x90Pa\t\xF9`@\x83\x01` \x84\x01a:iV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a\n*W`@Qc:?\xA0c`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\n9`@\x84\x01\x84a9\xB8V[\x90P\x81\x10\x15a\x0C\xF0W_a\nya\nS`@\x86\x01\x86a9\xB8V[\x84\x81\x81\x10a\ncWa\nca8\xEDV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\xDF\x91\x90a9\x01V[\x90P_a\n\x933a\n\x8D` \x88\x01\x88a2,V[\x84a#{V[\x90P\x80`@\x01Q`\x0F\x0B_\x14a\n\xBCW`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\n\xFC\x90a\n\xD2``\x88\x01\x88a9sV[\x86\x81\x81\x10a\n\xE2Wa\n\xE2a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\n\xF7\x91\x90a:iV[a&\xC1V[`\x0F\x0B`@\x82\x01\x81\x90R_\x03a\x0B%W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Q`\x0F\x0B\x12\x15a\x0B\xE4Wa\x0B^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba:\xA3V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R3_\x90\x81R`\x9A` \x90\x81R`@\x82 a\x0B\xDF\x92\x85\x92a\x0B\x8B\x90\x8A\x01\x8Aa2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x0CIV[_\x81`@\x01Q`\x0F\x0B\x13\x15a\x0CIWa\x0B\xFD\x87Ba:\xA3V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R\x80Q`@\x82\x01Qa\x0C\x19\x91\x90a&\xD8V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x80\x83R\x90\x85\x16\x10\x15a\x0CIW`@Qc2\x9DNS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ca3a\x0CZ` \x88\x01\x88a2,V[\x84\x84a&\xECV[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF63a\x0C\x90`@\x88\x01\x88a9\xB8V[\x86\x81\x81\x10a\x0C\xA0Wa\x0C\xA0a8\xEDV[`@\x02\x91\x90\x91\x01\x90Pa\x0C\xB6` \x89\x01\x89a2,V[a\x0C\xC8\x85` \x01Q\x86`@\x01Qa&\xD8V[\x85``\x01Q`@Qa\x0C\xDE\x95\x94\x93\x92\x91\x90a:\xBFV[`@Q\x80\x91\x03\x90\xA1PP`\x01\x01a\n,V[PPP\x80`\x01\x01\x90Pa\x08hV[PPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r$WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\r=WP0;\x15\x80\x15a\r=WP_T`\xFF\x16`\x01\x14[a\r\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\xC6W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r\xD0\x83\x83a(\tV[a\r\xD9\x84a(\x8AV[\x80\x15a\x0E\x1FW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EAWa\x0EAa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8AW\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x0E_W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x0F!W_a\x0E\xC0\x87\x87a\x05\xE4\x88\x86\x81Q\x81\x10a\x0E\xB3Wa\x0E\xB3a8\xEDV[` \x02` \x01\x01Qa#\x1DV[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x83\x83\x81Q\x81\x10a\x0F\rWa\x0F\ra8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\x8FV[P\x90P[\x93\x92PPPV[`fT_\x90`\x01\x90\x81\x16\x03a\x0FTW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x0FtW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFC\x91\x90a96V[a\x10\x19W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x10\x82Wa\x10z\x87\x87\x87\x84\x81\x81\x10a\x109Wa\x109a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x10N\x91\x90a2,V[\x86\x86\x85\x81\x81\x10a\x10`Wa\x10`a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x10u\x91\x90a;\x10V[a%pV[`\x01\x01a\x10\x1BV[PPPPPPPV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_`$\x83\x01\x81\x90R_\x19`D\x84\x01R``\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x16\xAEv\xCB\x90`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x02W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11)\x91\x90\x81\x01\x90a;1V[\x90P_a\x117\x86\x86\x84a\x0E%V[\x91\x93P\x90\x91PP[\x92P\x92\x90PV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11aWa\x11aa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0F!W`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x97` R`@\x81 a\x11\xD9\x91\x87\x87\x85\x81\x81\x10a\x11\xC4Wa\x11\xC4a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\t\xC3\x91\x90a2,V[\x82\x82\x81Q\x81\x10a\x11\xEBWa\x11\xEBa8\xEDV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\x8FV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12TW`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12^\x82\x82a(\xDBV[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCC\x91\x90a96V[a\x12\xE9W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xAD\x91\x90a96V[a\x13\xCAW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07%3\x82a(\xDBV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x13\xFDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81``\x015_\x10\x80\x15a\x14\x1CWPg\r\xE0\xB6\xB3\xA7d\0\0``\x83\x015\x11\x15[a\x149W`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x14g\x91\x90a6\xF1V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x14{\x82a#\x1DV[\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13R\xC3\xE6a\x14\xB9` \x87\x01\x87a2,V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD7\x92\x91\x90a;\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x16\x91\x90a96V[a\x153W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15A`@\x86\x01\x86a9sV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15ZWa\x15Za2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\x83W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x15\x95`@\x87\x01\x87a9sV[\x90P\x81\x10\x15a\x1A\xD3W_a\x15\xE7a\x15\xAF` \x89\x01\x89a2,V[a\x15\xBC`@\x8A\x01\x8Aa9sV[\x85\x81\x81\x10a\x15\xCCWa\x15\xCCa8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x15\xE1\x91\x90a2,V[\x86a#{V[\x90P_\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11a\x16\x16W`@QcN\x99\xE6\xCF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a\x164\x90`\x01`\x01`@\x1B\x03\x16``\x8A\x015a*hV[\x90P\x80\x82` \x01\x81\x81Qa\x16H\x91\x90a<(V[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Q\x81\x90\x83\x90a\x16e\x90\x83\x90a<(V[`\x01`\x01`@\x1B\x03\x16\x90RP`@\x82\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x17_W_a\x16\xA9\x89``\x015\x84`@\x01Qa\x16\x9A\x90a<GV[`\x01`\x01`\x80\x1B\x03\x16\x90a*hV[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x83`@\x01\x81\x81Qa\x16\xC6\x91\x90a<kV[`\x0F\x0B\x90RP\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x16\xFA` \x8B\x01\x8Ba2,V[\x88a\x17\x08`@\x8D\x01\x8Da9sV[\x88\x81\x81\x10a\x17\x18Wa\x17\x18a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x17-\x91\x90a2,V[a\x17?\x87` \x01Q\x88`@\x01Qa&\xD8V[\x87``\x01Q`@Qa\x17U\x95\x94\x93\x92\x91\x90a<\x98V[`@Q\x80\x91\x03\x90\xA1P[a\x17\xA8a\x17o` \x8A\x01\x8Aa2,V[a\x17|`@\x8B\x01\x8Ba9sV[\x86\x81\x81\x10a\x17\x8CWa\x17\x8Ca8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x17\xA1\x91\x90a2,V[\x87\x85a&\xECV[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x17\xD6` \x8A\x01\x8Aa2,V[\x87a\x17\xE4`@\x8C\x01\x8Ca9sV[\x87\x81\x81\x10a\x17\xF4Wa\x17\xF4a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x18\t\x91\x90a2,V[\x85` \x01QB`@Qa\x18 \x95\x94\x93\x92\x91\x90a<\x98V[`@Q\x80\x91\x03\x90\xA1_a\x18|`\x97\x82a\x18<` \x8D\x01\x8Da2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8B\x80`@\x01\x90a\x18l\x91\x90a9sV[\x88\x81\x81\x10a\x11\xC4Wa\x11\xC4a8\xEDV[\x90P_a\x18\x89\x83\x83a<(V[\x90Pa\x19\x19B\x82`\x97_\x8E_\x01` \x81\x01\x90a\x18\xA5\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8E\x80`@\x01\x90a\x18\xD5\x91\x90a9sV[\x8B\x81\x81\x10a\x18\xE5Wa\x18\xE5a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x18\xFA\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x91\x90a*|V[P\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90Pa\x19J` \x8C\x01\x8Ca2,V[a\x19W`@\x8D\x01\x8Da9sV[\x88\x81\x81\x10a\x19gWa\x19ga8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x19|\x91\x90a2,V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R`\x01`\x01`@\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5z\xB1\x0Ba\x19\xEB` \x8D\x01\x8Da2,V[a\x19\xF8`@\x8E\x01\x8Ea9sV[\x89\x81\x81\x10a\x1A\x08Wa\x1A\x08a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x1D\x91\x90a2,V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x83\x01R\x84\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AyW__\xFD[PZ\xF1\x15\x80\x15a\x1A\x8BW=__>=_\xFD[Pa\x1A\xA6\x92PPP`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x84\x16a*\x96V[\x86\x86\x81Q\x81\x10a\x1A\xB8Wa\x1A\xB8a8\xEDV[` \x02` \x01\x01\x81\x81RPPPPPP\x80`\x01\x01\x90Pa\x15\x88V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x1B\x02` \x87\x01\x87a2,V[\x84a\x1B\x10`@\x89\x01\x89a9sV[\x85a\x1B\x1E`\x80\x8C\x01\x8Ca<\xCEV[`@Qa\x1B1\x97\x96\x95\x94\x93\x92\x91\x90a=NV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x9A\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1CjW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1B\xD2\x90\x83a*\xAAV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PB\x10\x15a\x1CMWPPa\x1CjV[a\x1C[\x85\x82` \x01Qa&\xD8V[\x94PPP\x80`\x01\x01\x90Pa\x1B\x9AV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1C\x9A\x90a&uV[a\x1C\xA4\x91\x90a<(V[\x92PPP[\x92\x91PPV[a\x1C\xB7a+\x19V[a\x1C\xC0_a(\x8AV[V[``_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xDDWa\x1C\xDDa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x06W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x1D\xA8W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x97` R`@\x81 a\x1Dv\x91\x86\x91\x90\x89\x89\x86\x81\x81\x10a\x1DCWa\x1DCa8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1DX\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a+sV[\x82\x82\x81Q\x81\x10a\x1D\x88Wa\x1D\x88a8\xEDV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1D\x0BV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`@\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x82\x91\x15\x80\x15\x90a\x1E\x19WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\x1E*W\x80` \x01Q\x91Pa\x1E/V[\x80Q\x91P[\x81c\xFF\xFF\xFF\xFF\x16_\x14\x15\x92PP\x91P\x91V[``\x80B\x83c\xFF\xFF\xFF\xFF\x16\x11a\x1EjW`@Qc\xB7\xD0\x94\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1E}a\x05\xDF6\x8B\x90\x03\x8B\x01\x8Ba9\x01V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x8A\x8A\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xD2\x94\x93\x92\x91\x90a=\xE0V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x13\x91\x90\x81\x01\x90a>BV[\x90P_\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F.Wa\x1F.a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FaW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FLW\x90P[P\x90P_[\x89\x81\x10\x15a!\x91W_\x8B\x8B\x83\x81\x81\x10a\x1F\x81Wa\x1F\x81a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1F\x96\x91\x90a2,V[\x90P\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB0Wa\x1F\xB0a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xD9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x1F\xECWa\x1F\xECa8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x89\x81\x10\x15a!\x87W_\x8B\x8B\x83\x81\x81\x10a \x14Wa \x14a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a )\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x8B\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\x01`@\x1B\x82\x04`\x0F\x0B\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x93\x82\x01\x84\x90R\x93\x94P\x92\x90\x91\x90\x8D\x16\x10a \xB4Wa \xB1\x81\x83` \x01Qa&\xD8V[\x90P[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a!B\x90a \xE5\x90a&uV[`\x01`\x01`@\x1B\x03\x16a!<\x83`\x01`\x01`@\x1B\x03\x16\x8B\x8A\x81Q\x81\x10a!\rWa!\ra8\xEDV[` \x02` \x01\x01Q\x88\x81Q\x81\x10a!&Wa!&a8\xEDV[` \x02` \x01\x01Qa*h\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a*\x96V[\x87\x87\x81Q\x81\x10a!TWa!Ta8\xEDV[` \x02` \x01\x01Q\x85\x81Q\x81\x10a!mWa!ma8\xEDV[` \x02` \x01\x01\x81\x81RPPPPP\x80`\x01\x01\x90Pa\x1F\xF9V[PP`\x01\x01a\x1FfV[P\x90\x9A\x90\x99P\x97PPPPPPPPV[a!\xAAa+\x19V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\r\x9CV[a\x07%\x81a(\x8AV[`e_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x8C\x91\x90a9\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\xBDW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x19\x81\x19`fT\x19\x16\x14a\"\xE6W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x08\x06V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a#c\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C\xA9\x90a?CV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x80\x85R`\x99\x84R\x86\x85 \x91\x89\x16\x80\x86R\x91\x84R\x86\x85 \x88\x86R\x84R\x86\x85 \x87Q\x93\x84\x01\x88RT`\x01`\x01`@\x1B\x03\x80\x82\x16\x85R`\x01`@\x1B\x82\x04`\x0F\x0B\x85\x87\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x92\x04\x82\x16\x85\x8A\x01\x90\x81R\x92\x87R`\x98\x86R\x88\x87 \x93\x87R\x92\x90\x94R\x95\x90\x93 T\x94Q\x93\x94\x90\x93\x92\x16\x91\x16B\x10\x15a$wW`@Q\x80`\x80\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83_\x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x92PPPa\x0F%V[a$\x88\x82_\x01Q\x83` \x01Qa&\xD8V[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x86\x01\x91\x90\x91R\x90\x82\x16\x84R_``\x85\x01\x81\x90R`@\x85\x01\x81\x90R\x90\x83\x01Q`\x0F\x0B\x12\x15a$\xD7Wa$\xCB\x81\x83` \x01Qa&\xD8V[`\x01`\x01`@\x1B\x03\x16\x83R[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x07W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a%\xBAWP\x82a\xFF\xFF\x16\x82\x10[\x15a&nW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a%\xEE\x90a+\xC3V[\x90P_a%\xFC\x87\x87\x84a#{V[\x90P\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a&\x17WPPa&nV[a&#\x87\x87\x84\x84a&\xECV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R a&P\x90a,\x15V[Pa&Z\x84a?fV[\x93Pa&e\x83a?~V[\x92PPPa%\xA8V[PPPPPV[\x80T_\x90\x80\x15a&\xB1Wa&\x9B\x83a&\x8E`\x01\x84a?\x93V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0F%V[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[_a\x0F%`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16a?\xA6V[_a\x0F%\x82`\x01`\x01`@\x1B\x03\x85\x16a<kV[`@\x80Q``\x80\x82\x01\x83R` \x84\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x84R\x85\x85\x01Q`\x0F\x0B\x82\x85\x01\x90\x81R\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x86\x88\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16_\x81\x81R`\x99\x88R\x8A\x81 \x92\x8E\x16\x80\x82R\x92\x88R\x8A\x81 \x8D\x82R\x88R\x8A\x81 \x99Q\x8AT\x96Q\x94Q\x90\x95\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x90\x95\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x97\x16\x95\x88\x16\x95\x90\x95\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x90\x96U\x87Q\x86\x83R`\x98\x85R\x87\x83 \x82\x84R\x85R\x91\x87\x90 \x80T\x92\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x86Q\x86Q\x95\x86R\x92\x85\x01R\x16\x92\x82\x01\x92\x90\x92R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91\x01a\x0E\x16V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a(*WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a(GW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x12^\x82a$\xE0V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[\x80c\xFF\xFF\xFF\xFF\x16_\x03a)\x01W`@Qc\xDD\x18\x18\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x81\x16\x93\x83\x01\x84\x90R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R\x15\x80\x15\x90a)gWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a)zW` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R[c\xFF\xFF\xFF\xFF\x80\x83\x16` \x83\x01Ra)\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba?\xD3V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R\x90\x83\x90 \x86Q\x81T\x83\x89\x01Q\x96Q\x91\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01` \x1B\x96\x88\x16\x96\x90\x96\x02\x95\x90\x95\x17k\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x95\x87\x16\x95\x86\x02\x17\x90U\x82Q\x91\x82R\x93\x86\x16\x93\x81\x01\x93\x90\x93R\x82\x01R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[_a\x0F%\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\x92V[_\x80a*\x89\x85\x85\x85a-wV[\x91P\x91P[\x93P\x93\x91PPV[_a\x0F%\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a,\x92V[__a*\xCCa*\xB8\x84a/AV[\x85Ta*\xC7\x91\x90`\x0F\x0Ba?\xE6V[a/\xAEV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a*\xFFW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\r\x9CV[\x81T_\x90\x81a+\x84\x85\x85\x83\x85a0\x17V[\x90P\x80\x15a+\xB1Wa+\x9B\x85a&\x8E`\x01\x84a?\x93V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1C\xA4V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[_a+\xDD\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a+\xFBW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_a,/\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a,MW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a,\xC9W\x83\x82\x81a,\xBFWa,\xBFa@\rV[\x04\x92PPPa\x0F%V[\x80\x84\x11a-\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\r\x9CV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[\x82T_\x90\x81\x90\x80\x15a.\xD5W_a-\x93\x87a&\x8E`\x01\x85a?\x93V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a.\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\x9CV[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a.eW\x84a.5\x88a&\x8E`\x01\x86a?\x93V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua.\xC5V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa*\x8E\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a*\x8EV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a/\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\r\x9CV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a0\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\r\x9CV[\x91\x90PV[_[\x81\x83\x10\x15a0jW_a0,\x84\x84a0rV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a0VW\x80\x92Pa0dV[a0a\x81`\x01a?\xD3V[\x93P[Pa0\x19V[P\x93\x92PPPV[_a0\x80`\x02\x84\x84\x18a@!V[a\x0F%\x90\x84\x84\x16a?\xD3V[_`@\x82\x84\x03\x12\x15a0\x9CW__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a0\xB2W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC8W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x11?W__\xFD[_____`\x80\x86\x88\x03\x12\x15a0\xF6W__\xFD[a1\0\x87\x87a0\x8CV[\x94P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x1AW__\xFD[a1&\x88\x82\x89\x01a0\xA2V[\x90\x95P\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1DW__\xFD[a1P\x88\x82\x89\x01a0\xA2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a2\x0CW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87R` \x91\x82\x01\x91\x87\x01\x90_[\x81\x81\x10\x15a1\xF3W\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x84R` \x80\x82\x01Q`\x0F\x0B\x90\x85\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x84\x01R``\x83\x01` \x94\x90\x94\x01\x93\x92P`\x01\x01a1\xAAV[P\x90\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a1\x87V[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07%W__\xFD[_` \x82\x84\x03\x12\x15a2<W__\xFD[\x815a\x0F%\x81a2\x18V[_` \x82\x84\x03\x12\x15a2WW__\xFD[P5\x91\x90PV[__` \x83\x85\x03\x12\x15a2oW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x84W__\xFD[a2\x90\x85\x82\x86\x01a0\xA2V[\x90\x96\x90\x95P\x93PPPPV[___``\x84\x86\x03\x12\x15a2\xAEW__\xFD[\x835a2\xB9\x81a2\x18V[\x92P` \x84\x015a2\xC9\x81a2\x18V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3\x10Wa3\x10a2\xDAV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3>Wa3>a2\xDAV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a3^Wa3^a2\xDAV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07%W__\xFD[_`@\x82\x84\x03\x12\x15a3\x89W__\xFD[a3\x91a2\xEEV[\x90P\x815a3\x9E\x81a2\x18V[\x81R` \x82\x015a3\xAE\x81a3hV[` \x82\x01R\x92\x91PPV[___``\x84\x86\x03\x12\x15a3\xCBW__\xFD[\x835a3\xD6\x81a2\x18V[\x92P` \x84\x015a3\xE6\x81a2\x18V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a4\x10W__\xFD[\x805a4#a4\x1E\x82a3FV[a3\x16V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a4DW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a4pWa4]\x89\x85a3yV[\x82R` \x82\x01\x91P`@\x84\x01\x93Pa4KV[\x80\x94PPPPP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a4\xD9W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x87R` \x80\x82\x01Q`\x0F\x0B\x90\x88\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R``\x86\x01\x95P` \x91\x90\x91\x01\x90`\x01\x01a4\x90V[P\x93\x94\x93PPPPV[` \x81R_a\x0F%` \x83\x01\x84a4~V[_____``\x86\x88\x03\x12\x15a5\tW__\xFD[\x855a5\x14\x81a2\x18V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5.W__\xFD[a5:\x88\x82\x89\x01a0\xA2V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1DW__\xFD[__`@\x83\x85\x03\x12\x15a5iW__\xFD[\x825a5t\x81a2\x18V[\x91P` \x83\x015a5\x84\x81a2\x18V[\x80\x91PP\x92P\x92\x90PV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a5\xECWa5\xD6\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a5\xAAV[PP\x83\x81\x03` \x85\x01Ra6\0\x81\x86a4~V[\x96\x95PPPPPPV[___`@\x84\x86\x03\x12\x15a6\x1CW__\xFD[\x835a6'\x81a2\x18V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6AW__\xFD[a6M\x86\x82\x87\x01a0\xA2V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a6\x9AW\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6sV[P\x90\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a6\xB6W__\xFD[\x825a6\xC1\x81a2\x18V[\x91P` \x83\x015a5\x84\x81a3hV[_` \x82\x84\x03\x12\x15a6\xE1W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x0F%W__\xFD[_` \x82\x84\x03\x12\x15a7\x01W__\xFD[\x815a\x0F%\x81a3hV[_` \x82\x84\x03\x12\x15a7\x1CW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a71W__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x0F%W__\xFD[____``\x85\x87\x03\x12\x15a7UW__\xFD[\x845a7`\x81a2\x18V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7zW__\xFD[a7\x86\x87\x82\x88\x01a0\xA2V[\x90\x94P\x92PP`@\x85\x015a7\x9A\x81a3hV[\x93\x96\x92\x95P\x90\x93PPV[______`\xA0\x87\x89\x03\x12\x15a7\xBAW__\xFD[a7\xC4\x88\x88a0\x8CV[\x95P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xDEW__\xFD[a7\xEA\x89\x82\x8A\x01a0\xA2V[\x90\x96P\x94PP``\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x08W__\xFD[a8\x14\x89\x82\x8A\x01a0\xA2V[\x90\x94P\x92PP`\x80\x87\x015a8(\x81a3hV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a4\xD9W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a8HV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a8\xB4W`\x1F\x19\x85\x84\x03\x01\x88Ra8\x9E\x83\x83Qa86V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a8\x82V[P\x90\x96\x95PPPPPPV[`@\x81R_a8\xD2`@\x83\x01\x85a8fV[\x82\x81\x03` \x84\x01Ra8\xE4\x81\x85a8fV[\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x15a9\x11W__\xFD[a\x0F%\x83\x83a3yV[_` \x82\x84\x03\x12\x15a9+W__\xFD[\x81Qa\x0F%\x81a2\x18V[_` \x82\x84\x03\x12\x15a9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F%W__\xFD[_\x825`~\x19\x836\x03\x01\x81\x12a9iW__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a9\x88W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xA1W__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x11?W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a9\xCDW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xE6W__\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x11?W__\xFD[\x805a:\x08\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a:!\x81a3hV[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[` \x80\x82R\x81\x01\x82\x90R_\x83`@\x83\x01\x82[\x85\x81\x10\x15a\x1D\xA8Wa:V\x82\x84a9\xFDV[`@\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a:DV[_` \x82\x84\x03\x12\x15a:yW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F%W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a:\xDC` \x83\x01\x87a9\xFDV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a; W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0F%W__\xFD[_` \x82\x84\x03\x12\x15a;AW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;VW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a;fW__\xFD[\x80Qa;ta4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a;\x95W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6\0W`@\x84\x88\x03\x12\x15a;\xB3W__\xFD[a;\xBBa2\xEEV[\x84Qa;\xC6\x81a2\x18V[\x81R` \x85\x01Qa;\xD6\x81a3hV[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa;\x9CV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0F%` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a<cWa<ca:\x8FV[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a:\xDC` \x83\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<\xE3W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a<\xFCW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x11?W__\xFD[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15a4\xD9W\x815a=0\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a=\x1DV[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81Ra=\x80` \x82\x01\x88\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\xC0``\x82\x01R_a=\x96`\xC0\x83\x01\x87\x89a=\x10V[\x82\x81\x03`\x80\x84\x01Ra=\xA8\x81\x87a86V[\x90P\x82\x81\x03`\xA0\x84\x01R\x83\x81R\x83\x85` \x83\x017_` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x98\x97PPPPPPPPV[`@\x80\x82R\x81\x01\x84\x90R_\x85``\x83\x01\x82[\x87\x81\x10\x15a>\"W\x825a>\x05\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a=\xF2V[P\x83\x81\x03` \x85\x01Ra>6\x81\x86\x88a=\x10V[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a>RW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>gW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>wW__\xFD[\x80Qa>\x85a4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a>\xA6W__\xFD[` \x84\x01[\x83\x81\x10\x15a\x06lW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xC8W__\xFD[\x85\x01`?\x81\x01\x89\x13a>\xD8W__\xFD[` \x81\x01Qa>\xE9a4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15a?\x0CW__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15a?.W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a?\x13V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa>\xABV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a0\x9CW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01a?wWa?wa:\x8FV[P`\x01\x01\x90V[_\x81a?\x8CWa?\x8Ca:\x8FV[P_\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[\x80\x82\x01\x80\x82\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a@\x05Wa@\x05a:\x8FV[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a@;WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xD7\xE2!y\xDA\xAB\"{\x85\xB0O^\xDB\x1FA\x04\0\xAD\xD9\xEA\xE2\x0F\x95\xA0\xA5\x0CV\x8EO\xE4\xEB'dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b50600436106101c6575f3560e01c80635c975abb116100fe578063886f11951161009e578063df5cf7231161006e578063df5cf723146104a8578063e07d2b12146104cf578063f2fde38b146104f0578063fabc1cbc14610503575f5ffd5b8063886f1195146104225780638da5cb5b14610435578063a984eb3a14610446578063b9fbaed114610479575f5ffd5b80636cfb4481116100d95780636cfb4481146103b5578063715018a6146103e05780637bc1ef61146103e8578063843b349f1461040f575f5ffd5b80635c975abb1461035257806360db99a3146103635780636b3aa72e14610376575f5ffd5b80634b5046ef1161016957806356c483e61161014457806356c483e6146102f1578063595c6a67146103045780635ac86ab71461030c5780635c489bb51461033f575f5ffd5b80634b5046ef1461029d5780634d9dbde9146102b0578063547afb87146102d1575f5ffd5b80631637b60f116101a45780631637b60f1461021b5780631794bb3c1461022e5780632981eb771461024157806335af054a1461027d575f5ffd5b80630b002119146101ca57806310d67a2f146101f3578063136439dd14610208575b5f5ffd5b6101dd6101d83660046130e2565b610516565b6040516101ea9190613161565b60405180910390f35b61020661020136600461322c565b610677565b005b610206610216366004613247565b610728565b61020661022936600461325e565b610811565b61020661023c36600461329c565b610d06565b6102687f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016101ea565b61029061028b3660046133b9565b610e25565b6040516101ea91906134e3565b6102066102ab3660046134f5565b610f2c565b6102c36102be366004613558565b61108b565b6040516101ea92919061358f565b6102e46102df36600461360a565b611146565b6040516101ea919061365a565b6102066102ff3660046136a5565b61120b565b610206611262565b61032f61031a3660046136d1565b606654600160ff9092169190911b9081161490565b60405190151581526020016101ea565b61020661034d3660046136f1565b611327565b6066546040519081526020016101ea565b61020661037136600461370c565b6113d4565b61039d7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101ea565b6103c86103c3366004613558565b611b40565b6040516001600160401b0390911681526020016101ea565b610206611caf565b6102687f000000000000000000000000000000000000000000000000000000000000000081565b6102e461041d366004613742565b611cc2565b60655461039d906001600160a01b031681565b6033546001600160a01b031661039d565b6103c8610454366004613558565b609860209081525f92835260408084209091529082529020546001600160401b031681565b61048c61048736600461322c565b611db2565b60408051921515835263ffffffff9091166020830152016101ea565b61039d7f000000000000000000000000000000000000000000000000000000000000000081565b6104e26104dd3660046137a5565b611e41565b6040516101ea9291906138c0565b6102066104fe36600461322c565b6121a2565b610206610511366004613247565b612218565b60605f826001600160401b03811115610531576105316132da565b60405190808252806020026020018201604052801561056457816020015b606081526020019060019003908161054f5790505b5090505f5b8381101561066c575f5b86811015610663575f6105e9878785818110610591576105916138ed565b90506020020160208101906105a6919061322c565b8a8a858181106105b8576105b86138ed565b90506020020160208101906105cd919061322c565b6105e46105df368f90038f018f613901565b61231d565b61237b565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250848481518110610636576106366138ed565b6020026020010151838151811061064f5761064f6138ed565b602090810291909101015250600101610573565b50600101610569565b509695505050505050565b60655f9054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156106c7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906106eb919061391b565b6001600160a01b0316336001600160a01b03161461071c5760405163794821ff60e01b815260040160405180910390fd5b610725816124e0565b50565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa15801561076e573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107929190613936565b6107af57604051631d77d47760e21b815260040160405180910390fd5b606654818116146107d35760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6066545f906001908116036108395760405163840a48d560e01b815260040160405180910390fd5b5f5f61084433611db2565b91509150816108665760405163fa55fc8160e01b815260040160405180910390fd5b5f5b84811015610cfe5736868683818110610883576108836138ed565b90506020028101906108959190613955565b90506108a46060820182613973565b90506108b360408301836139b8565b9050146108d3576040516343714afd60e01b815260040160405180910390fd5b6001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000166320c4e23661090f60408401846139b8565b6040518363ffffffff1660e01b815260040161092c929190613a32565b602060405180830381865afa158015610947573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061096b9190613936565b61098857604051631fb1705560e21b815260040160405180910390fd5b6109a133610999602084018461322c565b61ffff612570565b335f90815260976020908152604082206109e79183906109c39086018661322c565b6001600160a01b03166001600160a01b031681526020019081526020015f20612675565b90506109f96040830160208401613a69565b6001600160401b0316816001600160401b031614610a2a57604051633a3fa06360e21b815260040160405180910390fd5b5f5b610a3960408401846139b8565b9050811015610cf0575f610a79610a5360408601866139b8565b84818110610a6357610a636138ed565b9050604002018036038101906105df9190613901565b90505f610a9333610a8d602088018861322c565b8461237b565b90508060400151600f0b5f14610abc57604051630d8fcbe360e41b815260040160405180910390fd5b6020810151610afc90610ad26060880188613973565b86818110610ae257610ae26138ed565b9050602002016020810190610af79190613a69565b6126c1565b600f0b604082018190525f03610b2557604051634606179360e11b815260040160405180910390fd5b5f8160400151600f0b1215610be457610b5e7f000000000000000000000000000000000000000000000000000000000000000042613aa3565b63ffffffff166060820152335f908152609a602090815260408220610bdf928592610b8b908a018a61322c565b6001600160a01b0316815260208101919091526040015f20908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b610c49565b5f8160400151600f0b1315610c4957610bfd8742613aa3565b63ffffffff16606082015280516040820151610c1991906126d8565b6001600160401b039081168083529085161015610c495760405163329d4e5360e21b815260040160405180910390fd5b610c6133610c5a602088018861322c565b84846126ec565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf633610c9060408801886139b8565b86818110610ca057610ca06138ed565b604002919091019050610cb6602089018961322c565b610cc8856020015186604001516126d8565b8560600151604051610cde959493929190613abf565b60405180910390a15050600101610a2c565b505050806001019050610868565b505050505050565b5f54610100900460ff1615808015610d2457505f54600160ff909116105b80610d3d5750303b158015610d3d57505f5460ff166001145b610da55760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015610dc6575f805461ff0019166101001790555b610dd08383612809565b610dd98461288a565b8015610e1f575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b50505050565b60605f82516001600160401b03811115610e4157610e416132da565b604051908082528060200260200182016040528015610e8a57816020015b604080516060810182525f80825260208083018290529282015282525f19909201910181610e5f5790505b5090505f5b8351811015610f21575f610ec087876105e4888681518110610eb357610eb36138ed565b602002602001015161231d565b9050604051806060016040528082602001516001600160401b031681526020018260400151600f0b8152602001826060015163ffffffff16815250838381518110610f0d57610f0d6138ed565b602090810291909101015250600101610e8f565b5090505b9392505050565b6066545f90600190811603610f545760405163840a48d560e01b815260040160405180910390fd5b838214610f74576040516343714afd60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0387811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015610fd8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610ffc9190613936565b611019576040516325ec6c1f60e01b815260040160405180910390fd5b5f5b848110156110825761107a87878784818110611039576110396138ed565b905060200201602081019061104e919061322c565b868685818110611060576110606138ed565b90506020020160208101906110759190613b10565b612570565b60010161101b565b50505050505050565b6040516316ae76cb60e01b81526001600160a01b0383811660048301525f602483018190525f19604484015260609283927f000000000000000000000000000000000000000000000000000000000000000016906316ae76cb906064015f60405180830381865afa158015611102573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526111299190810190613b31565b90505f611137868684610e25565b919350909150505b9250929050565b60605f826001600160401b03811115611161576111616132da565b60405190808252806020026020018201604052801561118a578160200160208202803683370190505b5090505f5b83811015610f21576001600160a01b0386165f9081526097602052604081206111d9918787858181106111c4576111c46138ed565b90506020020160208101906109c3919061322c565b8282815181106111eb576111eb6138ed565b6001600160401b039092166020928302919091019091015260010161118f565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146112545760405163f739589b60e01b815260040160405180910390fd5b61125e82826128db565b5050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156112a8573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906112cc9190613936565b6112e957604051631d77d47760e21b815260040160405180910390fd5b5f19606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa158015611389573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906113ad9190613936565b6113ca576040516325ec6c1f60e01b815260040160405180910390fd5b61072533826128db565b6066546001906002908116036113fd5760405163840a48d560e01b815260040160405180910390fd5b81606001355f10801561141c5750670de0b6b3a7640000606083013511155b61143957604051631353603160e01b815260040160405180910390fd5b5f6040518060400160405280336001600160a01b0316815260200184602001602081019061146791906136f1565b63ffffffff16905290505f61147b8261231d565b90506001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016631352c3e66114b9602087018761322c565b846040518363ffffffff1660e01b81526004016114d7929190613bf2565b602060405180830381865afa1580156114f2573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906115169190613936565b6115335760405163ccea9e6f60e01b815260040160405180910390fd5b5f6115416040860186613973565b90506001600160401b0381111561155a5761155a6132da565b604051908082528060200260200182016040528015611583578160200160208202803683370190505b5090505f5b6115956040870187613973565b9050811015611ad3575f6115e76115af602089018961322c565b6115bc60408a018a613973565b858181106115cc576115cc6138ed565b90506020020160208101906115e1919061322c565b8661237b565b90505f81602001516001600160401b03161161161657604051634e99e6cf60e01b815260040160405180910390fd5b60208101515f90611634906001600160401b031660608a0135612a68565b905080826020018181516116489190613c28565b6001600160401b0316905250815181908390611665908390613c28565b6001600160401b031690525060408201515f600f9190910b121561175f575f6116a98960600135846040015161169a90613c47565b6001600160801b031690612a68565b9050806001600160401b0316836040018181516116c69190613c6b565b600f0b9052507f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66116fa60208b018b61322c565b8861170860408d018d613973565b88818110611718576117186138ed565b905060200201602081019061172d919061322c565b61173f876020015188604001516126d8565b8760600151604051611755959493929190613c98565b60405180910390a1505b6117a861176f60208a018a61322c565b61177c60408b018b613973565b8681811061178c5761178c6138ed565b90506020020160208101906117a1919061322c565b87856126ec565b7f8b997e53d7b9e5d923d0a21c60df81e1740860d1a8c66b8c63c5047ae20eaaf66117d660208a018a61322c565b876117e460408c018c613973565b878181106117f4576117f46138ed565b9050602002016020810190611809919061322c565b856020015142604051611820959493929190613c98565b60405180910390a15f61187c60978261183c60208d018d61322c565b6001600160a01b03166001600160a01b031681526020019081526020015f205f8b806040019061186c9190613973565b888181106111c4576111c46138ed565b90505f6118898383613c28565b9050611919428260975f8e5f0160208101906118a5919061322c565b6001600160a01b03166001600160a01b031681526020019081526020015f205f8e80604001906118d59190613973565b8b8181106118e5576118e56138ed565b90506020020160208101906118fa919061322c565b6001600160a01b0316815260208101919091526040015f209190612a7c565b507f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c905061194a60208c018c61322c565b61195760408d018d613973565b88818110611967576119676138ed565b905060200201602081019061197c919061322c565b604080516001600160a01b0393841681529290911660208301526001600160401b0384169082015260600160405180910390a16001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663a57ab10b6119eb60208d018d61322c565b6119f860408e018e613973565b89818110611a0857611a086138ed565b9050602002016020810190611a1d919061322c565b6040516001600160e01b031960e085901b1681526001600160a01b039283166004820152911660248201526001600160401b038086166044830152841660648201526084015f604051808303815f87803b158015611a79575f5ffd5b505af1158015611a8b573d5f5f3e3d5ffd5b50611aa6925050506001600160401b03848116908416612a96565b868681518110611ab857611ab86138ed565b60200260200101818152505050505050806001019050611588565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5611b02602087018761322c565b84611b106040890189613973565b85611b1e60808c018c613cce565b604051611b319796959493929190613d4e565b60405180910390a15050505050565b6001600160a01b038281165f81815260986020908152604080832094861680845294825280832054938352609a8252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611c6a576001600160a01b038087165f908152609a602090815260408083209389168352929052908120611bd29083612aaa565b6001600160a01b038881165f908152609960209081526040808320938b168352928152828220848352815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250421015611c4d575050611c6a565b611c5b8582602001516126d8565b94505050806001019050611b9a565b506001600160a01b038086165f9081526097602090815260408083209388168352929052208290611c9a90612675565b611ca49190613c28565b925050505b92915050565b611cb7612b19565b611cc05f61288a565b565b60605f836001600160401b03811115611cdd57611cdd6132da565b604051908082528060200260200182016040528015611d06578160200160208202803683370190505b5090505f5b84811015611da8576001600160a01b0387165f908152609760205260408120611d7691869190898986818110611d4357611d436138ed565b9050602002016020810190611d58919061322c565b6001600160a01b0316815260208101919091526040015f2090612b73565b828281518110611d8857611d886138ed565b6001600160401b0390921660209283029190910190910152600101611d0b565b5095945050505050565b6001600160a01b0381165f908152609b602090815260408083208151606081018352905463ffffffff8082168352600160201b82048116948301859052600160401b9091041691810191909152829115801590611e195750806040015163ffffffff164210155b15611e2a5780602001519150611e2f565b805191505b8163ffffffff165f1415925050915091565b606080428363ffffffff1611611e6a5760405163b7d0949760e01b815260040160405180910390fd5b5f611e7d6105df368b90038b018b613901565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e6768a8a8a8a6040518563ffffffff1660e01b8152600401611ed29493929190613de0565b5f60405180830381865afa158015611eec573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611f139190810190613e42565b90505f886001600160401b03811115611f2e57611f2e6132da565b604051908082528060200260200182016040528015611f6157816020015b6060815260200190600190039081611f4c5790505b5090505f5b89811015612191575f8b8b83818110611f8157611f816138ed565b9050602002016020810190611f96919061322c565b9050886001600160401b03811115611fb057611fb06132da565b604051908082528060200260200182016040528015611fd9578160200160208202803683370190505b50838381518110611fec57611fec6138ed565b60209081029190910101525f5b89811015612187575f8b8b83818110612014576120146138ed565b9050602002016020810190612029919061322c565b6001600160a01b038085165f90815260996020908152604080832093851683529281528282208b8352815290829020825160608101845290546001600160401b038116808352600160401b8204600f0b9383019390935263ffffffff600160c01b9091048116938201849052939450929091908d16106120b4576120b18183602001516126d8565b90505b6001600160a01b038086165f908152609760209081526040808320938716835292905220612142906120e590612675565b6001600160401b031661213c836001600160401b03168b8a8151811061210d5761210d6138ed565b60200260200101518881518110612126576121266138ed565b6020026020010151612a6890919063ffffffff16565b90612a96565b878781518110612154576121546138ed565b6020026020010151858151811061216d5761216d6138ed565b602002602001018181525050505050806001019050611ff9565b5050600101611f66565b50909a909950975050505050505050565b6121aa612b19565b6001600160a01b03811661220f5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610d9c565b6107258161288a565b60655f9054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612268573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061228c919061391b565b6001600160a01b0316336001600160a01b0316146122bd5760405163794821ff60e01b815260040160405180910390fd5b6066541981196066541916146122e65760405163c61dca5d60e01b815260040160405180910390fd5b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c90602001610806565b5f815f0151826020015163ffffffff1660405160200161236392919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b604051602081830303815290604052611ca990613f43565b604080516080810182525f808252602080830182905282840182905260608084018390526001600160a01b0388811680855260998452868520918916808652918452868520888652845286852087519384018852546001600160401b038082168552600160401b8204600f0b8587015263ffffffff600160c01b9092048216858a019081529287526098865288872093875292909452959093205494519394909392169116421015612477576040518060800160405280826001600160401b03168152602001835f01516001600160401b031681526020018360200151600f0b8152602001836040015163ffffffff1681525092505050610f25565b612488825f015183602001516126d8565b6001600160401b0390811660208086019190915290821684525f606085018190526040850181905290830151600f0b12156124d7576124cb8183602001516126d8565b6001600160401b031683525b50509392505050565b6001600160a01b038116612507576040516339b190bb60e11b815260040160405180910390fd5b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6001600160a01b038381165f908152609a60209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f811180156125ba57508261ffff1682105b1561266e576001600160a01b038086165f908152609a6020908152604080832093881683529290529081206125ee90612bc3565b90505f6125fc87878461237b565b9050806060015163ffffffff1642101561261757505061266e565b612623878784846126ec565b6001600160a01b038088165f908152609a60209081526040808320938a1683529290522061265090612c15565b5061265a84613f66565b935061266583613f7e565b925050506125a8565b5050505050565b80545f9080156126b15761269b8361268e600184613f93565b5f91825260209091200190565b54600160201b90046001600160401b0316610f25565b670de0b6b3a76400009392505050565b5f610f256001600160401b03808516908416613fa6565b5f610f25826001600160401b038516613c6b565b60408051606080820183526020848101516001600160401b03908116845285850151600f0b8285019081528684015163ffffffff9081168688019081526001600160a01b038c81165f818152609988528a8120928e168082529288528a81208d825288528a812099518a5496519451909516600160c01b0263ffffffff60c01b196001600160801b03909516600160401b026001600160c01b031990971695881695909517959095179290921692909217909655875186835260988552878320828452855291879020805492841667ffffffffffffffff1990931692909217909155865186519586529285015216928201929092527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559101610e16565b6065546001600160a01b031615801561282a57506001600160a01b03821615155b612847576040516339b190bb60e11b815260040160405180910390fd5b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a261125e826124e0565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b8063ffffffff165f036129015760405163dd18181560e01b815260040160405180910390fd5b6001600160a01b0382165f908152609b60209081526040918290208251606081018452905463ffffffff8082168352600160201b82048116938301849052600160401b9091041692810192909252158015906129675750806040015163ffffffff164210155b1561297a57602081015163ffffffff1681525b63ffffffff80831660208301526129b3907f00000000000000000000000000000000000000000000000000000000000000001642613fd3565b63ffffffff90811660408381019182526001600160a01b0386165f818152609b6020908152908390208651815483890151965191881667ffffffffffffffff1990911617600160201b96881696909602959095176bffffffff00000000000000001916600160401b9587169586021790558251918252938616938101939093528201527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9060600160405180910390a1505050565b5f610f258383670de0b6b3a7640000612c92565b5f80612a89858585612d77565b915091505b935093915050565b5f610f2583670de0b6b3a764000084612c92565b5f5f612acc612ab884612f41565b8554612ac79190600f0b613fe6565b612fae565b8454909150600160801b9004600f90810b9082900b12612aff57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b6033546001600160a01b03163314611cc05760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610d9c565b81545f9081612b8485858385613017565b90508015612bb157612b9b8561268e600184613f93565b54600160201b90046001600160401b0316611ca4565b50670de0b6b3a7640000949350505050565b5f612bdd8254600f81810b600160801b909204900b131590565b15612bfb57604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f612c2f8254600f81810b600160801b909204900b131590565b15612c4d57604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b5f80805f19858709858702925082811083820303915050805f03612cc957838281612cbf57612cbf61400d565b0492505050610f25565b808411612d105760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401610d9c565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b82545f9081908015612ed5575f612d938761268e600185613f93565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b031660208401529192509087161015612e145760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401610d9c565b805163ffffffff808816911603612e655784612e358861268e600186613f93565b80546001600160401b0392909216600160201b026bffffffffffffffff0000000019909216919091179055612ec5565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d555f8d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b602001519250839150612a8e9050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a555f8a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081612a8e565b5f6001600160ff1b03821115612faa5760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401610d9c565b5090565b80600f81900b81146130125760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401610d9c565b919050565b5f5b8183101561306a575f61302c8484613072565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561305657809250613064565b613061816001613fd3565b93505b50613019565b509392505050565b5f6130806002848418614021565b610f2590848416613fd3565b5f6040828403121561309c575f5ffd5b50919050565b5f5f83601f8401126130b2575f5ffd5b5081356001600160401b038111156130c8575f5ffd5b6020830191508360208260051b850101111561113f575f5ffd5b5f5f5f5f5f608086880312156130f6575f5ffd5b613100878761308c565b945060408601356001600160401b0381111561311a575f5ffd5b613126888289016130a2565b90955093505060608601356001600160401b03811115613144575f5ffd5b613150888289016130a2565b969995985093965092949392505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561320c57868503603f19018452815180518087526020918201918701905f5b818110156131f357835180516001600160401b03168452602080820151600f0b9085015260409081015163ffffffff1690840152606083016020949094019392506001016131aa565b5090965050506020938401939190910190600101613187565b50929695505050505050565b6001600160a01b0381168114610725575f5ffd5b5f6020828403121561323c575f5ffd5b8135610f2581613218565b5f60208284031215613257575f5ffd5b5035919050565b5f5f6020838503121561326f575f5ffd5b82356001600160401b03811115613284575f5ffd5b613290858286016130a2565b90969095509350505050565b5f5f5f606084860312156132ae575f5ffd5b83356132b981613218565b925060208401356132c981613218565b929592945050506040919091013590565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b0381118282101715613310576133106132da565b60405290565b604051601f8201601f191681016001600160401b038111828210171561333e5761333e6132da565b604052919050565b5f6001600160401b0382111561335e5761335e6132da565b5060051b60200190565b63ffffffff81168114610725575f5ffd5b5f60408284031215613389575f5ffd5b6133916132ee565b9050813561339e81613218565b815260208201356133ae81613368565b602082015292915050565b5f5f5f606084860312156133cb575f5ffd5b83356133d681613218565b925060208401356133e681613218565b915060408401356001600160401b03811115613400575f5ffd5b8401601f81018613613410575f5ffd5b803561342361341e82613346565b613316565b8082825260208201915060208360061b850101925088831115613444575f5ffd5b6020840193505b828410156134705761345d8985613379565b825260208201915060408401935061344b565b809450505050509250925092565b5f8151808452602084019350602083015f5b828110156134d957815180516001600160401b03168752602080820151600f0b9088015260409081015163ffffffff169087015260608601955060209190910190600101613490565b5093949350505050565b602081525f610f25602083018461347e565b5f5f5f5f5f60608688031215613509575f5ffd5b853561351481613218565b945060208601356001600160401b0381111561352e575f5ffd5b61353a888289016130a2565b90955093505060408601356001600160401b03811115613144575f5ffd5b5f5f60408385031215613569575f5ffd5b823561357481613218565b9150602083013561358481613218565b809150509250929050565b604080825283519082018190525f9060208501906060840190835b818110156135ec576135d683855180516001600160a01b0316825260209081015163ffffffff16910152565b60209390930192604092909201916001016135aa565b50508381036020850152613600818661347e565b9695505050505050565b5f5f5f6040848603121561361c575f5ffd5b833561362781613218565b925060208401356001600160401b03811115613641575f5ffd5b61364d868287016130a2565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b8181101561369a5783516001600160401b0316835260209384019390920191600101613673565b509095945050505050565b5f5f604083850312156136b6575f5ffd5b82356136c181613218565b9150602083013561358481613368565b5f602082840312156136e1575f5ffd5b813560ff81168114610f25575f5ffd5b5f60208284031215613701575f5ffd5b8135610f2581613368565b5f6020828403121561371c575f5ffd5b81356001600160401b03811115613731575f5ffd5b820160a08185031215610f25575f5ffd5b5f5f5f5f60608587031215613755575f5ffd5b843561376081613218565b935060208501356001600160401b0381111561377a575f5ffd5b613786878288016130a2565b909450925050604085013561379a81613368565b939692955090935050565b5f5f5f5f5f5f60a087890312156137ba575f5ffd5b6137c4888861308c565b955060408701356001600160401b038111156137de575f5ffd5b6137ea89828a016130a2565b90965094505060608701356001600160401b03811115613808575f5ffd5b61381489828a016130a2565b909450925050608087013561382881613368565b809150509295509295509295565b5f8151808452602084019350602083015f5b828110156134d9578151865260209586019590910190600101613848565b5f82825180855260208501945060208160051b830101602085015f5b838110156138b457601f1985840301885261389e838351613836565b6020988901989093509190910190600101613882565b50909695505050505050565b604081525f6138d26040830185613866565b82810360208401526138e48185613866565b95945050505050565b634e487b7160e01b5f52603260045260245ffd5b5f60408284031215613911575f5ffd5b610f258383613379565b5f6020828403121561392b575f5ffd5b8151610f2581613218565b5f60208284031215613946575f5ffd5b81518015158114610f25575f5ffd5b5f8235607e19833603018112613969575f5ffd5b9190910192915050565b5f5f8335601e19843603018112613988575f5ffd5b8301803591506001600160401b038211156139a1575f5ffd5b6020019150600581901b360382131561113f575f5ffd5b5f5f8335601e198436030181126139cd575f5ffd5b8301803591506001600160401b038211156139e6575f5ffd5b6020019150600681901b360382131561113f575f5ffd5b8035613a0881613218565b6001600160a01b031682526020810135613a2181613368565b63ffffffff81166020840152505050565b602080825281018290525f8360408301825b85811015611da857613a5682846139fd565b6040928301929190910190600101613a44565b5f60208284031215613a79575f5ffd5b81356001600160401b0381168114610f25575f5ffd5b634e487b7160e01b5f52601160045260245ffd5b63ffffffff8181168382160190811115611ca957611ca9613a8f565b6001600160a01b038616815260c08101613adc60208301876139fd565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f60208284031215613b20575f5ffd5b813561ffff81168114610f25575f5ffd5b5f60208284031215613b41575f5ffd5b81516001600160401b03811115613b56575f5ffd5b8201601f81018413613b66575f5ffd5b8051613b7461341e82613346565b8082825260208201915060208360061b850101925086831115613b95575f5ffd5b6020840193505b828410156136005760408488031215613bb3575f5ffd5b613bbb6132ee565b8451613bc681613218565b81526020850151613bd681613368565b8060208301525080835250602082019150604084019350613b9c565b6001600160a01b038316815260608101610f25602083018480516001600160a01b0316825260209081015163ffffffff16910152565b6001600160401b038281168282160390811115611ca957611ca9613a8f565b5f81600f0b60016001607f1b03198103613c6357613c63613a8f565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b031982121715611ca957611ca9613a8f565b6001600160a01b038616815260c08101613adc602083018780516001600160a01b0316825260209081015163ffffffff16910152565b5f5f8335601e19843603018112613ce3575f5ffd5b8301803591506001600160401b03821115613cfc575f5ffd5b60200191503681900382131561113f575f5ffd5b8183526020830192505f815f5b848110156134d9578135613d3081613218565b6001600160a01b031686526020958601959190910190600101613d1d565b6001600160a01b0388168152613d80602082018880516001600160a01b0316825260209081015163ffffffff16910152565b60c060608201525f613d9660c083018789613d10565b8281036080840152613da88187613836565b905082810360a0840152838152838560208301375f602085830101526020601f19601f86011682010191505098975050505050505050565b604080825281018490525f8560608301825b87811015613e22578235613e0581613218565b6001600160a01b0316825260209283019290910190600101613df2565b508381036020850152613e36818688613d10565b98975050505050505050565b5f60208284031215613e52575f5ffd5b81516001600160401b03811115613e67575f5ffd5b8201601f81018413613e77575f5ffd5b8051613e8561341e82613346565b8082825260208201915060208360051b850101925086831115613ea6575f5ffd5b602084015b8381101561066c5780516001600160401b03811115613ec8575f5ffd5b8501603f81018913613ed8575f5ffd5b6020810151613ee961341e82613346565b808282526020820191506020808460051b8601010192508b831115613f0c575f5ffd5b6040840193505b82841015613f2e578351825260209384019390910190613f13565b86525050602093840193919091019050613eab565b8051602080830151919081101561309c575f1960209190910360031b1b16919050565b5f60018201613f7757613f77613a8f565b5060010190565b5f81613f8c57613f8c613a8f565b505f190190565b81810381811115611ca957611ca9613a8f565b600f82810b9082900b0360016001607f1b0319811260016001607f1b0382131715611ca957611ca9613a8f565b80820180821115611ca957611ca9613a8f565b8082018281125f83128015821682158216171561400557614005613a8f565b505092915050565b634e487b7160e01b5f52601260045260245ffd5b5f8261403b57634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220d7e22179daab227b85b04f5edb1f410400add9eae20f95a0a50c568e4fe4eb2764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x01\xC6W_5`\xE0\x1C\x80c\\\x97Z\xBB\x11a\0\xFEW\x80c\x88o\x11\x95\x11a\0\x9EW\x80c\xDF\\\xF7#\x11a\0nW\x80c\xDF\\\xF7#\x14a\x04\xA8W\x80c\xE0}+\x12\x14a\x04\xCFW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xF0W\x80c\xFA\xBC\x1C\xBC\x14a\x05\x03W__\xFD[\x80c\x88o\x11\x95\x14a\x04\"W\x80c\x8D\xA5\xCB[\x14a\x045W\x80c\xA9\x84\xEB:\x14a\x04FW\x80c\xB9\xFB\xAE\xD1\x14a\x04yW__\xFD[\x80cl\xFBD\x81\x11a\0\xD9W\x80cl\xFBD\x81\x14a\x03\xB5W\x80cqP\x18\xA6\x14a\x03\xE0W\x80c{\xC1\xEFa\x14a\x03\xE8W\x80c\x84;4\x9F\x14a\x04\x0FW__\xFD[\x80c\\\x97Z\xBB\x14a\x03RW\x80c`\xDB\x99\xA3\x14a\x03cW\x80ck:\xA7.\x14a\x03vW__\xFD[\x80cKPF\xEF\x11a\x01iW\x80cV\xC4\x83\xE6\x11a\x01DW\x80cV\xC4\x83\xE6\x14a\x02\xF1W\x80cY\\jg\x14a\x03\x04W\x80cZ\xC8j\xB7\x14a\x03\x0CW\x80c\\H\x9B\xB5\x14a\x03?W__\xFD[\x80cKPF\xEF\x14a\x02\x9DW\x80cM\x9D\xBD\xE9\x14a\x02\xB0W\x80cTz\xFB\x87\x14a\x02\xD1W__\xFD[\x80c\x167\xB6\x0F\x11a\x01\xA4W\x80c\x167\xB6\x0F\x14a\x02\x1BW\x80c\x17\x94\xBB<\x14a\x02.W\x80c)\x81\xEBw\x14a\x02AW\x80c5\xAF\x05J\x14a\x02}W__\xFD[\x80c\x0B\0!\x19\x14a\x01\xCAW\x80c\x10\xD6z/\x14a\x01\xF3W\x80c\x13d9\xDD\x14a\x02\x08W[__\xFD[a\x01\xDDa\x01\xD86`\x04a0\xE2V[a\x05\x16V[`@Qa\x01\xEA\x91\x90a1aV[`@Q\x80\x91\x03\x90\xF3[a\x02\x06a\x02\x016`\x04a2,V[a\x06wV[\0[a\x02\x06a\x02\x166`\x04a2GV[a\x07(V[a\x02\x06a\x02)6`\x04a2^V[a\x08\x11V[a\x02\x06a\x02<6`\x04a2\x9CV[a\r\x06V[a\x02h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x02\x90a\x02\x8B6`\x04a3\xB9V[a\x0E%V[`@Qa\x01\xEA\x91\x90a4\xE3V[a\x02\x06a\x02\xAB6`\x04a4\xF5V[a\x0F,V[a\x02\xC3a\x02\xBE6`\x04a5XV[a\x10\x8BV[`@Qa\x01\xEA\x92\x91\x90a5\x8FV[a\x02\xE4a\x02\xDF6`\x04a6\nV[a\x11FV[`@Qa\x01\xEA\x91\x90a6ZV[a\x02\x06a\x02\xFF6`\x04a6\xA5V[a\x12\x0BV[a\x02\x06a\x12bV[a\x03/a\x03\x1A6`\x04a6\xD1V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xEAV[a\x02\x06a\x03M6`\x04a6\xF1V[a\x13'V[`fT`@Q\x90\x81R` \x01a\x01\xEAV[a\x02\x06a\x03q6`\x04a7\x0CV[a\x13\xD4V[a\x03\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x03\xC8a\x03\xC36`\x04a5XV[a\x1B@V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xEAV[a\x02\x06a\x1C\xAFV[a\x02h\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xE4a\x04\x1D6`\x04a7BV[a\x1C\xC2V[`eTa\x03\x9D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x9DV[a\x03\xC8a\x04T6`\x04a5XV[`\x98` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x04\x8Ca\x04\x876`\x04a2,V[a\x1D\xB2V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x01\xEAV[a\x03\x9D\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04\xE2a\x04\xDD6`\x04a7\xA5V[a\x1EAV[`@Qa\x01\xEA\x92\x91\x90a8\xC0V[a\x02\x06a\x04\xFE6`\x04a2,V[a!\xA2V[a\x02\x06a\x05\x116`\x04a2GV[a\"\x18V[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x051Wa\x051a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x05dW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x05OW\x90P[P\x90P_[\x83\x81\x10\x15a\x06lW_[\x86\x81\x10\x15a\x06cW_a\x05\xE9\x87\x87\x85\x81\x81\x10a\x05\x91Wa\x05\x91a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x05\xA6\x91\x90a2,V[\x8A\x8A\x85\x81\x81\x10a\x05\xB8Wa\x05\xB8a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x05\xCD\x91\x90a2,V[a\x05\xE4a\x05\xDF6\x8F\x90\x03\x8F\x01\x8Fa9\x01V[a#\x1DV[a#{V[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x84\x84\x81Q\x81\x10a\x066Wa\x066a8\xEDV[` \x02` \x01\x01Q\x83\x81Q\x81\x10a\x06OWa\x06Oa8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x05sV[P`\x01\x01a\x05iV[P\x96\x95PPPPPPV[`e_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xC7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xEB\x91\x90a9\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07\x1CW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07%\x81a$\xE0V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07nW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x92\x91\x90a96V[a\x07\xAFW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x14a\x07\xD3W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`fT_\x90`\x01\x90\x81\x16\x03a\x089W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\x08D3a\x1D\xB2V[\x91P\x91P\x81a\x08fW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x0C\xFEW6\x86\x86\x83\x81\x81\x10a\x08\x83Wa\x08\x83a8\xEDV[\x90P` \x02\x81\x01\x90a\x08\x95\x91\x90a9UV[\x90Pa\x08\xA4``\x82\x01\x82a9sV[\x90Pa\x08\xB3`@\x83\x01\x83a9\xB8V[\x90P\x14a\x08\xD3W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c \xC4\xE26a\t\x0F`@\x84\x01\x84a9\xB8V[`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t,\x92\x91\x90a:2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tGW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tk\x91\x90a96V[a\t\x88W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\xA13a\t\x99` \x84\x01\x84a2,V[a\xFF\xFFa%pV[3_\x90\x81R`\x97` \x90\x81R`@\x82 a\t\xE7\x91\x83\x90a\t\xC3\x90\x86\x01\x86a2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a&uV[\x90Pa\t\xF9`@\x83\x01` \x84\x01a:iV[`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a\n*W`@Qc:?\xA0c`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\n9`@\x84\x01\x84a9\xB8V[\x90P\x81\x10\x15a\x0C\xF0W_a\nya\nS`@\x86\x01\x86a9\xB8V[\x84\x81\x81\x10a\ncWa\nca8\xEDV[\x90P`@\x02\x01\x806\x03\x81\x01\x90a\x05\xDF\x91\x90a9\x01V[\x90P_a\n\x933a\n\x8D` \x88\x01\x88a2,V[\x84a#{V[\x90P\x80`@\x01Q`\x0F\x0B_\x14a\n\xBCW`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Qa\n\xFC\x90a\n\xD2``\x88\x01\x88a9sV[\x86\x81\x81\x10a\n\xE2Wa\n\xE2a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\n\xF7\x91\x90a:iV[a&\xC1V[`\x0F\x0B`@\x82\x01\x81\x90R_\x03a\x0B%W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x81`@\x01Q`\x0F\x0B\x12\x15a\x0B\xE4Wa\x0B^\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0Ba:\xA3V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R3_\x90\x81R`\x9A` \x90\x81R`@\x82 a\x0B\xDF\x92\x85\x92a\x0B\x8B\x90\x8A\x01\x8Aa2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x0CIV[_\x81`@\x01Q`\x0F\x0B\x13\x15a\x0CIWa\x0B\xFD\x87Ba:\xA3V[c\xFF\xFF\xFF\xFF\x16``\x82\x01R\x80Q`@\x82\x01Qa\x0C\x19\x91\x90a&\xD8V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x80\x83R\x90\x85\x16\x10\x15a\x0CIW`@Qc2\x9DNS`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ca3a\x0CZ` \x88\x01\x88a2,V[\x84\x84a&\xECV[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF63a\x0C\x90`@\x88\x01\x88a9\xB8V[\x86\x81\x81\x10a\x0C\xA0Wa\x0C\xA0a8\xEDV[`@\x02\x91\x90\x91\x01\x90Pa\x0C\xB6` \x89\x01\x89a2,V[a\x0C\xC8\x85` \x01Q\x86`@\x01Qa&\xD8V[\x85``\x01Q`@Qa\x0C\xDE\x95\x94\x93\x92\x91\x90a:\xBFV[`@Q\x80\x91\x03\x90\xA1PP`\x01\x01a\n,V[PPP\x80`\x01\x01\x90Pa\x08hV[PPPPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\r$WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a\r=WP0;\x15\x80\x15a\r=WP_T`\xFF\x16`\x01\x14[a\r\xA5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\r\xC6W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\r\xD0\x83\x83a(\tV[a\r\xD9\x84a(\x8AV[\x80\x15a\x0E\x1FW_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EAWa\x0EAa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\x8AW\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x0E_W\x90P[P\x90P_[\x83Q\x81\x10\x15a\x0F!W_a\x0E\xC0\x87\x87a\x05\xE4\x88\x86\x81Q\x81\x10a\x0E\xB3Wa\x0E\xB3a8\xEDV[` \x02` \x01\x01Qa#\x1DV[\x90P`@Q\x80``\x01`@R\x80\x82` \x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x82`@\x01Q`\x0F\x0B\x81R` \x01\x82``\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x83\x83\x81Q\x81\x10a\x0F\rWa\x0F\ra8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\x8FV[P\x90P[\x93\x92PPPV[`fT_\x90`\x01\x90\x81\x16\x03a\x0FTW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x0FtW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\xD8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F\xFC\x91\x90a96V[a\x10\x19W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x10\x82Wa\x10z\x87\x87\x87\x84\x81\x81\x10a\x109Wa\x109a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x10N\x91\x90a2,V[\x86\x86\x85\x81\x81\x10a\x10`Wa\x10`a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x10u\x91\x90a;\x10V[a%pV[`\x01\x01a\x10\x1BV[PPPPPPPV[`@Qc\x16\xAEv\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R_`$\x83\x01\x81\x90R_\x19`D\x84\x01R``\x92\x83\x92\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x16\xAEv\xCB\x90`d\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\x02W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11)\x91\x90\x81\x01\x90a;1V[\x90P_a\x117\x86\x86\x84a\x0E%V[\x91\x93P\x90\x91PP[\x92P\x92\x90PV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11aWa\x11aa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11\x8AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0F!W`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x97` R`@\x81 a\x11\xD9\x91\x87\x87\x85\x81\x81\x10a\x11\xC4Wa\x11\xC4a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\t\xC3\x91\x90a2,V[\x82\x82\x81Q\x81\x10a\x11\xEBWa\x11\xEBa8\xEDV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x11\x8FV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12TW`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12^\x82\x82a(\xDBV[PPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12\xA8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\xCC\x91\x90a96V[a\x12\xE9W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\x89W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\xAD\x91\x90a96V[a\x13\xCAW`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07%3\x82a(\xDBV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x13\xFDW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81``\x015_\x10\x80\x15a\x14\x1CWPg\r\xE0\xB6\xB3\xA7d\0\0``\x83\x015\x11\x15[a\x149W`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x14g\x91\x90a6\xF1V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x14{\x82a#\x1DV[\x90P`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\x13R\xC3\xE6a\x14\xB9` \x87\x01\x87a2,V[\x84`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14\xD7\x92\x91\x90a;\xF2V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\x16\x91\x90a96V[a\x153W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x15A`@\x86\x01\x86a9sV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15ZWa\x15Za2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\x83W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x15\x95`@\x87\x01\x87a9sV[\x90P\x81\x10\x15a\x1A\xD3W_a\x15\xE7a\x15\xAF` \x89\x01\x89a2,V[a\x15\xBC`@\x8A\x01\x8Aa9sV[\x85\x81\x81\x10a\x15\xCCWa\x15\xCCa8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x15\xE1\x91\x90a2,V[\x86a#{V[\x90P_\x81` \x01Q`\x01`\x01`@\x1B\x03\x16\x11a\x16\x16W`@QcN\x99\xE6\xCF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x81\x01Q_\x90a\x164\x90`\x01`\x01`@\x1B\x03\x16``\x8A\x015a*hV[\x90P\x80\x82` \x01\x81\x81Qa\x16H\x91\x90a<(V[`\x01`\x01`@\x1B\x03\x16\x90RP\x81Q\x81\x90\x83\x90a\x16e\x90\x83\x90a<(V[`\x01`\x01`@\x1B\x03\x16\x90RP`@\x82\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x17_W_a\x16\xA9\x89``\x015\x84`@\x01Qa\x16\x9A\x90a<GV[`\x01`\x01`\x80\x1B\x03\x16\x90a*hV[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x83`@\x01\x81\x81Qa\x16\xC6\x91\x90a<kV[`\x0F\x0B\x90RP\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x16\xFA` \x8B\x01\x8Ba2,V[\x88a\x17\x08`@\x8D\x01\x8Da9sV[\x88\x81\x81\x10a\x17\x18Wa\x17\x18a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x17-\x91\x90a2,V[a\x17?\x87` \x01Q\x88`@\x01Qa&\xD8V[\x87``\x01Q`@Qa\x17U\x95\x94\x93\x92\x91\x90a<\x98V[`@Q\x80\x91\x03\x90\xA1P[a\x17\xA8a\x17o` \x8A\x01\x8Aa2,V[a\x17|`@\x8B\x01\x8Ba9sV[\x86\x81\x81\x10a\x17\x8CWa\x17\x8Ca8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x17\xA1\x91\x90a2,V[\x87\x85a&\xECV[\x7F\x8B\x99~S\xD7\xB9\xE5\xD9#\xD0\xA2\x1C`\xDF\x81\xE1t\x08`\xD1\xA8\xC6k\x8Cc\xC5\x04z\xE2\x0E\xAA\xF6a\x17\xD6` \x8A\x01\x8Aa2,V[\x87a\x17\xE4`@\x8C\x01\x8Ca9sV[\x87\x81\x81\x10a\x17\xF4Wa\x17\xF4a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x18\t\x91\x90a2,V[\x85` \x01QB`@Qa\x18 \x95\x94\x93\x92\x91\x90a<\x98V[`@Q\x80\x91\x03\x90\xA1_a\x18|`\x97\x82a\x18<` \x8D\x01\x8Da2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8B\x80`@\x01\x90a\x18l\x91\x90a9sV[\x88\x81\x81\x10a\x11\xC4Wa\x11\xC4a8\xEDV[\x90P_a\x18\x89\x83\x83a<(V[\x90Pa\x19\x19B\x82`\x97_\x8E_\x01` \x81\x01\x90a\x18\xA5\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _\x8E\x80`@\x01\x90a\x18\xD5\x91\x90a9sV[\x8B\x81\x81\x10a\x18\xE5Wa\x18\xE5a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x18\xFA\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x91\x90a*|V[P\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90Pa\x19J` \x8C\x01\x8Ca2,V[a\x19W`@\x8D\x01\x8Da9sV[\x88\x81\x81\x10a\x19gWa\x19ga8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x19|\x91\x90a2,V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R`\x01`\x01`@\x1B\x03\x84\x16\x90\x82\x01R``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xA5z\xB1\x0Ba\x19\xEB` \x8D\x01\x8Da2,V[a\x19\xF8`@\x8E\x01\x8Ea9sV[\x89\x81\x81\x10a\x1A\x08Wa\x1A\x08a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1A\x1D\x91\x90a2,V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x83\x01R\x84\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x1AyW__\xFD[PZ\xF1\x15\x80\x15a\x1A\x8BW=__>=_\xFD[Pa\x1A\xA6\x92PPP`\x01`\x01`@\x1B\x03\x84\x81\x16\x90\x84\x16a*\x96V[\x86\x86\x81Q\x81\x10a\x1A\xB8Wa\x1A\xB8a8\xEDV[` \x02` \x01\x01\x81\x81RPPPPPP\x80`\x01\x01\x90Pa\x15\x88V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x1B\x02` \x87\x01\x87a2,V[\x84a\x1B\x10`@\x89\x01\x89a9sV[\x85a\x1B\x1E`\x80\x8C\x01\x8Ca<\xCEV[`@Qa\x1B1\x97\x96\x95\x94\x93\x92\x91\x90a=NV[`@Q\x80\x91\x03\x90\xA1PPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\x98` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\x9A\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1CjW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1B\xD2\x90\x83a*\xAAV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x82\x82 \x84\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PB\x10\x15a\x1CMWPPa\x1CjV[a\x1C[\x85\x82` \x01Qa&\xD8V[\x94PPP\x80`\x01\x01\x90Pa\x1B\x9AV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1C\x9A\x90a&uV[a\x1C\xA4\x91\x90a<(V[\x92PPP[\x92\x91PPV[a\x1C\xB7a+\x19V[a\x1C\xC0_a(\x8AV[V[``_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xDDWa\x1C\xDDa2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1D\x06W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84\x81\x10\x15a\x1D\xA8W`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x97` R`@\x81 a\x1Dv\x91\x86\x91\x90\x89\x89\x86\x81\x81\x10a\x1DCWa\x1DCa8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1DX\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a+sV[\x82\x82\x81Q\x81\x10a\x1D\x88Wa\x1D\x88a8\xEDV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1D\x0BV[P\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q``\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x81\x16\x94\x83\x01\x85\x90R`\x01`@\x1B\x90\x91\x04\x16\x91\x81\x01\x91\x90\x91R\x82\x91\x15\x80\x15\x90a\x1E\x19WP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a\x1E*W\x80` \x01Q\x91Pa\x1E/V[\x80Q\x91P[\x81c\xFF\xFF\xFF\xFF\x16_\x14\x15\x92PP\x91P\x91V[``\x80B\x83c\xFF\xFF\xFF\xFF\x16\x11a\x1EjW`@Qc\xB7\xD0\x94\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x1E}a\x05\xDF6\x8B\x90\x03\x8B\x01\x8Ba9\x01V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x8A\x8A\x8A\x8A`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x1E\xD2\x94\x93\x92\x91\x90a=\xE0V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xECW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F\x13\x91\x90\x81\x01\x90a>BV[\x90P_\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F.Wa\x1F.a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1FaW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1FLW\x90P[P\x90P_[\x89\x81\x10\x15a!\x91W_\x8B\x8B\x83\x81\x81\x10a\x1F\x81Wa\x1F\x81a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a\x1F\x96\x91\x90a2,V[\x90P\x88`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F\xB0Wa\x1F\xB0a2\xDAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\xD9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x83\x83\x81Q\x81\x10a\x1F\xECWa\x1F\xECa8\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x89\x81\x10\x15a!\x87W_\x8B\x8B\x83\x81\x81\x10a \x14Wa \x14a8\xEDV[\x90P` \x02\x01` \x81\x01\x90a )\x91\x90a2,V[`\x01`\x01`\xA0\x1B\x03\x80\x85\x16_\x90\x81R`\x99` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x8B\x83R\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x80\x83R`\x01`@\x1B\x82\x04`\x0F\x0B\x93\x83\x01\x93\x90\x93Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x91\x04\x81\x16\x93\x82\x01\x84\x90R\x93\x94P\x92\x90\x91\x90\x8D\x16\x10a \xB4Wa \xB1\x81\x83` \x01Qa&\xD8V[\x90P[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x97` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x90R a!B\x90a \xE5\x90a&uV[`\x01`\x01`@\x1B\x03\x16a!<\x83`\x01`\x01`@\x1B\x03\x16\x8B\x8A\x81Q\x81\x10a!\rWa!\ra8\xEDV[` \x02` \x01\x01Q\x88\x81Q\x81\x10a!&Wa!&a8\xEDV[` \x02` \x01\x01Qa*h\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a*\x96V[\x87\x87\x81Q\x81\x10a!TWa!Ta8\xEDV[` \x02` \x01\x01Q\x85\x81Q\x81\x10a!mWa!ma8\xEDV[` \x02` \x01\x01\x81\x81RPPPPP\x80`\x01\x01\x90Pa\x1F\xF9V[PP`\x01\x01a\x1FfV[P\x90\x9A\x90\x99P\x97PPPPPPPPV[a!\xAAa+\x19V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\"\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\r\x9CV[a\x07%\x81a(\x8AV[`e_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"hW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"\x8C\x91\x90a9\x1BV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\"\xBDW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x19\x81\x19`fT\x19\x16\x14a\"\xE6W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x08\x06V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a#c\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x1C\xA9\x90a?CV[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R``\x80\x84\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x88\x81\x16\x80\x85R`\x99\x84R\x86\x85 \x91\x89\x16\x80\x86R\x91\x84R\x86\x85 \x88\x86R\x84R\x86\x85 \x87Q\x93\x84\x01\x88RT`\x01`\x01`@\x1B\x03\x80\x82\x16\x85R`\x01`@\x1B\x82\x04`\x0F\x0B\x85\x87\x01Rc\xFF\xFF\xFF\xFF`\x01`\xC0\x1B\x90\x92\x04\x82\x16\x85\x8A\x01\x90\x81R\x92\x87R`\x98\x86R\x88\x87 \x93\x87R\x92\x90\x94R\x95\x90\x93 T\x94Q\x93\x94\x90\x93\x92\x16\x91\x16B\x10\x15a$wW`@Q\x80`\x80\x01`@R\x80\x82`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83_\x01Q`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x83` \x01Q`\x0F\x0B\x81R` \x01\x83`@\x01Qc\xFF\xFF\xFF\xFF\x16\x81RP\x92PPPa\x0F%V[a$\x88\x82_\x01Q\x83` \x01Qa&\xD8V[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x80\x86\x01\x91\x90\x91R\x90\x82\x16\x84R_``\x85\x01\x81\x90R`@\x85\x01\x81\x90R\x90\x83\x01Q`\x0F\x0B\x12\x15a$\xD7Wa$\xCB\x81\x83` \x01Qa&\xD8V[`\x01`\x01`@\x1B\x03\x16\x83R[PP\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x07W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a%\xBAWP\x82a\xFF\xFF\x16\x82\x10[\x15a&nW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a%\xEE\x90a+\xC3V[\x90P_a%\xFC\x87\x87\x84a#{V[\x90P\x80``\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15a&\x17WPPa&nV[a&#\x87\x87\x84\x84a&\xECV[`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\x9A` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R a&P\x90a,\x15V[Pa&Z\x84a?fV[\x93Pa&e\x83a?~V[\x92PPPa%\xA8V[PPPPPV[\x80T_\x90\x80\x15a&\xB1Wa&\x9B\x83a&\x8E`\x01\x84a?\x93V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0F%V[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[_a\x0F%`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16a?\xA6V[_a\x0F%\x82`\x01`\x01`@\x1B\x03\x85\x16a<kV[`@\x80Q``\x80\x82\x01\x83R` \x84\x81\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x84R\x85\x85\x01Q`\x0F\x0B\x82\x85\x01\x90\x81R\x86\x84\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x86\x88\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8C\x81\x16_\x81\x81R`\x99\x88R\x8A\x81 \x92\x8E\x16\x80\x82R\x92\x88R\x8A\x81 \x8D\x82R\x88R\x8A\x81 \x99Q\x8AT\x96Q\x94Q\x90\x95\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x90\x95\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x97\x16\x95\x88\x16\x95\x90\x95\x17\x95\x90\x95\x17\x92\x90\x92\x16\x92\x90\x92\x17\x90\x96U\x87Q\x86\x83R`\x98\x85R\x87\x83 \x82\x84R\x85R\x91\x87\x90 \x80T\x92\x84\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91U\x86Q\x86Q\x95\x86R\x92\x85\x01R\x16\x92\x82\x01\x92\x90\x92R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x91\x01a\x0E\x16V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a(*WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a(GW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x12^\x82a$\xE0V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[\x80c\xFF\xFF\xFF\xFF\x16_\x03a)\x01W`@Qc\xDD\x18\x18\x15`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q``\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x81\x16\x93\x83\x01\x84\x90R`\x01`@\x1B\x90\x91\x04\x16\x92\x81\x01\x92\x90\x92R\x15\x80\x15\x90a)gWP\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16B\x10\x15[\x15a)zW` \x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R[c\xFF\xFF\xFF\xFF\x80\x83\x16` \x83\x01Ra)\xB3\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Ba?\xD3V[c\xFF\xFF\xFF\xFF\x90\x81\x16`@\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R\x90\x83\x90 \x86Q\x81T\x83\x89\x01Q\x96Q\x91\x88\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17`\x01` \x1B\x96\x88\x16\x96\x90\x96\x02\x95\x90\x95\x17k\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16`\x01`@\x1B\x95\x87\x16\x95\x86\x02\x17\x90U\x82Q\x91\x82R\x93\x86\x16\x93\x81\x01\x93\x90\x93R\x82\x01R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x90``\x01`@Q\x80\x91\x03\x90\xA1PPPV[_a\x0F%\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\x92V[_\x80a*\x89\x85\x85\x85a-wV[\x91P\x91P[\x93P\x93\x91PPV[_a\x0F%\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a,\x92V[__a*\xCCa*\xB8\x84a/AV[\x85Ta*\xC7\x91\x90`\x0F\x0Ba?\xE6V[a/\xAEV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a*\xFFW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1C\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\r\x9CV[\x81T_\x90\x81a+\x84\x85\x85\x83\x85a0\x17V[\x90P\x80\x15a+\xB1Wa+\x9B\x85a&\x8E`\x01\x84a?\x93V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1C\xA4V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[_a+\xDD\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a+\xFBW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_a,/\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a,MW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a,\xC9W\x83\x82\x81a,\xBFWa,\xBFa@\rV[\x04\x92PPPa\x0F%V[\x80\x84\x11a-\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a\r\x9CV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[\x82T_\x90\x81\x90\x80\x15a.\xD5W_a-\x93\x87a&\x8E`\x01\x85a?\x93V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a.\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\r\x9CV[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03a.eW\x84a.5\x88a&\x8E`\x01\x86a?\x93V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua.\xC5V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa*\x8E\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a*\x8EV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a/\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a\r\x9CV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a0\x12W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a\r\x9CV[\x91\x90PV[_[\x81\x83\x10\x15a0jW_a0,\x84\x84a0rV[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a0VW\x80\x92Pa0dV[a0a\x81`\x01a?\xD3V[\x93P[Pa0\x19V[P\x93\x92PPPV[_a0\x80`\x02\x84\x84\x18a@!V[a\x0F%\x90\x84\x84\x16a?\xD3V[_`@\x82\x84\x03\x12\x15a0\x9CW__\xFD[P\x91\x90PV[__\x83`\x1F\x84\x01\x12a0\xB2W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xC8W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x11?W__\xFD[_____`\x80\x86\x88\x03\x12\x15a0\xF6W__\xFD[a1\0\x87\x87a0\x8CV[\x94P`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x1AW__\xFD[a1&\x88\x82\x89\x01a0\xA2V[\x90\x95P\x93PP``\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1DW__\xFD[a1P\x88\x82\x89\x01a0\xA2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a2\x0CW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q\x80\x87R` \x91\x82\x01\x91\x87\x01\x90_[\x81\x81\x10\x15a1\xF3W\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x84R` \x80\x82\x01Q`\x0F\x0B\x90\x85\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x84\x01R``\x83\x01` \x94\x90\x94\x01\x93\x92P`\x01\x01a1\xAAV[P\x90\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a1\x87V[P\x92\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x07%W__\xFD[_` \x82\x84\x03\x12\x15a2<W__\xFD[\x815a\x0F%\x81a2\x18V[_` \x82\x84\x03\x12\x15a2WW__\xFD[P5\x91\x90PV[__` \x83\x85\x03\x12\x15a2oW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x84W__\xFD[a2\x90\x85\x82\x86\x01a0\xA2V[\x90\x96\x90\x95P\x93PPPPV[___``\x84\x86\x03\x12\x15a2\xAEW__\xFD[\x835a2\xB9\x81a2\x18V[\x92P` \x84\x015a2\xC9\x81a2\x18V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3\x10Wa3\x10a2\xDAV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a3>Wa3>a2\xDAV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a3^Wa3^a2\xDAV[P`\x05\x1B` \x01\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x07%W__\xFD[_`@\x82\x84\x03\x12\x15a3\x89W__\xFD[a3\x91a2\xEEV[\x90P\x815a3\x9E\x81a2\x18V[\x81R` \x82\x015a3\xAE\x81a3hV[` \x82\x01R\x92\x91PPV[___``\x84\x86\x03\x12\x15a3\xCBW__\xFD[\x835a3\xD6\x81a2\x18V[\x92P` \x84\x015a3\xE6\x81a2\x18V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13a4\x10W__\xFD[\x805a4#a4\x1E\x82a3FV[a3\x16V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a4DW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a4pWa4]\x89\x85a3yV[\x82R` \x82\x01\x91P`@\x84\x01\x93Pa4KV[\x80\x94PPPPP\x92P\x92P\x92V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a4\xD9W\x81Q\x80Q`\x01`\x01`@\x1B\x03\x16\x87R` \x80\x82\x01Q`\x0F\x0B\x90\x88\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x87\x01R``\x86\x01\x95P` \x91\x90\x91\x01\x90`\x01\x01a4\x90V[P\x93\x94\x93PPPPV[` \x81R_a\x0F%` \x83\x01\x84a4~V[_____``\x86\x88\x03\x12\x15a5\tW__\xFD[\x855a5\x14\x81a2\x18V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5.W__\xFD[a5:\x88\x82\x89\x01a0\xA2V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1DW__\xFD[__`@\x83\x85\x03\x12\x15a5iW__\xFD[\x825a5t\x81a2\x18V[\x91P` \x83\x015a5\x84\x81a2\x18V[\x80\x91PP\x92P\x92\x90PV[`@\x80\x82R\x83Q\x90\x82\x01\x81\x90R_\x90` \x85\x01\x90``\x84\x01\x90\x83[\x81\x81\x10\x15a5\xECWa5\xD6\x83\x85Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x93\x90\x93\x01\x92`@\x92\x90\x92\x01\x91`\x01\x01a5\xAAV[PP\x83\x81\x03` \x85\x01Ra6\0\x81\x86a4~V[\x96\x95PPPPPPV[___`@\x84\x86\x03\x12\x15a6\x1CW__\xFD[\x835a6'\x81a2\x18V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6AW__\xFD[a6M\x86\x82\x87\x01a0\xA2V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a6\x9AW\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a6sV[P\x90\x95\x94PPPPPV[__`@\x83\x85\x03\x12\x15a6\xB6W__\xFD[\x825a6\xC1\x81a2\x18V[\x91P` \x83\x015a5\x84\x81a3hV[_` \x82\x84\x03\x12\x15a6\xE1W__\xFD[\x815`\xFF\x81\x16\x81\x14a\x0F%W__\xFD[_` \x82\x84\x03\x12\x15a7\x01W__\xFD[\x815a\x0F%\x81a3hV[_` \x82\x84\x03\x12\x15a7\x1CW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a71W__\xFD[\x82\x01`\xA0\x81\x85\x03\x12\x15a\x0F%W__\xFD[____``\x85\x87\x03\x12\x15a7UW__\xFD[\x845a7`\x81a2\x18V[\x93P` \x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7zW__\xFD[a7\x86\x87\x82\x88\x01a0\xA2V[\x90\x94P\x92PP`@\x85\x015a7\x9A\x81a3hV[\x93\x96\x92\x95P\x90\x93PPV[______`\xA0\x87\x89\x03\x12\x15a7\xBAW__\xFD[a7\xC4\x88\x88a0\x8CV[\x95P`@\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7\xDEW__\xFD[a7\xEA\x89\x82\x8A\x01a0\xA2V[\x90\x96P\x94PP``\x87\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a8\x08W__\xFD[a8\x14\x89\x82\x8A\x01a0\xA2V[\x90\x94P\x92PP`\x80\x87\x015a8(\x81a3hV[\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a4\xD9W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a8HV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a8\xB4W`\x1F\x19\x85\x84\x03\x01\x88Ra8\x9E\x83\x83Qa86V[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a8\x82V[P\x90\x96\x95PPPPPPV[`@\x81R_a8\xD2`@\x83\x01\x85a8fV[\x82\x81\x03` \x84\x01Ra8\xE4\x81\x85a8fV[\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_`@\x82\x84\x03\x12\x15a9\x11W__\xFD[a\x0F%\x83\x83a3yV[_` \x82\x84\x03\x12\x15a9+W__\xFD[\x81Qa\x0F%\x81a2\x18V[_` \x82\x84\x03\x12\x15a9FW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0F%W__\xFD[_\x825`~\x19\x836\x03\x01\x81\x12a9iW__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a9\x88W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xA1W__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x11?W__\xFD[__\x835`\x1E\x19\x846\x03\x01\x81\x12a9\xCDW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a9\xE6W__\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a\x11?W__\xFD[\x805a:\x08\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x81\x015a:!\x81a3hV[c\xFF\xFF\xFF\xFF\x81\x16` \x84\x01RPPPV[` \x80\x82R\x81\x01\x82\x90R_\x83`@\x83\x01\x82[\x85\x81\x10\x15a\x1D\xA8Wa:V\x82\x84a9\xFDV[`@\x92\x83\x01\x92\x91\x90\x91\x01\x90`\x01\x01a:DV[_` \x82\x84\x03\x12\x15a:yW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0F%W__\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a:\xDC` \x83\x01\x87a9\xFDV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[_` \x82\x84\x03\x12\x15a; W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0F%W__\xFD[_` \x82\x84\x03\x12\x15a;AW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a;VW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a;fW__\xFD[\x80Qa;ta4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a;\x95W__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6\0W`@\x84\x88\x03\x12\x15a;\xB3W__\xFD[a;\xBBa2\xEEV[\x84Qa;\xC6\x81a2\x18V[\x81R` \x85\x01Qa;\xD6\x81a3hV[\x80` \x83\x01RP\x80\x83RP` \x82\x01\x91P`@\x84\x01\x93Pa;\x9CV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R``\x81\x01a\x0F%` \x83\x01\x84\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03a<cWa<ca:\x8FV[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01a:\xDC` \x83\x01\x87\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[__\x835`\x1E\x19\x846\x03\x01\x81\x12a<\xE3W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a<\xFCW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x11?W__\xFD[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15a4\xD9W\x815a=0\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01a=\x1DV[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81Ra=\x80` \x82\x01\x88\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[`\xC0``\x82\x01R_a=\x96`\xC0\x83\x01\x87\x89a=\x10V[\x82\x81\x03`\x80\x84\x01Ra=\xA8\x81\x87a86V[\x90P\x82\x81\x03`\xA0\x84\x01R\x83\x81R\x83\x85` \x83\x017_` \x85\x83\x01\x01R` `\x1F\x19`\x1F\x86\x01\x16\x82\x01\x01\x91PP\x98\x97PPPPPPPPV[`@\x80\x82R\x81\x01\x84\x90R_\x85``\x83\x01\x82[\x87\x81\x10\x15a>\"W\x825a>\x05\x81a2\x18V[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a=\xF2V[P\x83\x81\x03` \x85\x01Ra>6\x81\x86\x88a=\x10V[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15a>RW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>gW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a>wW__\xFD[\x80Qa>\x85a4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a>\xA6W__\xFD[` \x84\x01[\x83\x81\x10\x15a\x06lW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a>\xC8W__\xFD[\x85\x01`?\x81\x01\x89\x13a>\xD8W__\xFD[` \x81\x01Qa>\xE9a4\x1E\x82a3FV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15a?\x0CW__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15a?.W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a?\x13V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90Pa>\xABV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a0\x9CW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01a?wWa?wa:\x8FV[P`\x01\x01\x90V[_\x81a?\x8CWa?\x8Ca:\x8FV[P_\x19\x01\x90V[\x81\x81\x03\x81\x81\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[\x80\x82\x01\x80\x82\x11\x15a\x1C\xA9Wa\x1C\xA9a:\x8FV[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a@\x05Wa@\x05a:\x8FV[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82a@;WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xD7\xE2!y\xDA\xAB\"{\x85\xB0O^\xDB\x1FA\x04\0\xAD\xD9\xEA\xE2\x0F\x95\xA0\xA5\x0CV\x8EO\xE4\xEB'dsolcC\0\x08\x1B\x003",
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetId,
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
        impl alloy_sol_types::SolType for OperatorSet {
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
        impl alloy_sol_types::SolStruct for OperatorSet {
            const NAME: &'static str = "OperatorSet";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "OperatorSet(address avs,uint32 operatorSetId)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadySlashedForTimestamp> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadySlashedForTimestamp) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadySlashedForTimestamp {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadySlashedForTimestamp {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InsufficientAllocatableMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientAllocatableMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientAllocatableMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientAllocatableMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidExpectedTotalMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidExpectedTotalMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidExpectedTotalMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidExpectedTotalMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ModificationAlreadyPending> for UnderlyingRustTuple<'_> {
            fn from(value: ModificationAlreadyPending) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ModificationAlreadyPending {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ModificationAlreadyPending {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UninitializedAllocationDelay> for UnderlyingRustTuple<'_> {
            fn from(value: UninitializedAllocationDelay) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UninitializedAllocationDelay {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UninitializedAllocationDelay {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AllocationDelaySet(address,uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    78u8, 133u8, 117u8, 29u8, 99u8, 49u8, 80u8, 108u8, 108u8, 98u8, 51u8, 95u8,
                    32u8, 126u8, 179u8, 31u8, 18u8, 166u8, 30u8, 87u8, 15u8, 52u8, 245u8, 193u8,
                    118u8, 64u8, 48u8, 135u8, 133u8, 198u8, 212u8, 219u8,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.delay,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.effectTimestamp,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EncumberedMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    172u8, 249u8, 9u8, 95u8, 235u8, 58u8, 55u8, 12u8, 156u8, 246u8, 146u8, 66u8,
                    28u8, 105u8, 239u8, 50u8, 13u8, 77u8, 181u8, 198u8, 110u8, 106u8, 125u8, 41u8,
                    199u8, 105u8, 78u8, 176u8, 35u8, 100u8, 252u8, 85u8,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.encumberedMagnitude,
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
            fn from(this: &EncumberedMagnitudeUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "MaxMagnitudeUpdated(address,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    28u8, 100u8, 88u8, 7u8, 154u8, 65u8, 7u8, 125u8, 0u8, 60u8, 17u8, 250u8, 249u8,
                    191u8, 9u8, 126u8, 105u8, 59u8, 214u8, 121u8, 121u8, 228u8, 230u8, 80u8, 11u8,
                    172u8, 123u8, 41u8, 219u8, 119u8, 155u8, 92u8,
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
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.totalMagnitude,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "OperatorSetMagnitudeUpdated(address,(address,uint32),address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    139u8, 153u8, 126u8, 83u8, 215u8, 185u8, 229u8, 217u8, 35u8, 208u8, 162u8,
                    28u8, 96u8, 223u8, 129u8, 225u8, 116u8, 8u8, 96u8, 209u8, 168u8, 198u8, 107u8,
                    140u8, 99u8, 197u8, 4u8, 122u8, 226u8, 14u8, 170u8, 246u8,
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
                        &self.operator,
                    ),
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.magnitude,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.effectTimestamp,
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
            fn from(this: &OperatorSetMagnitudeUpdated) -> alloy_sol_types::private::LogData {
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub wadSlashed:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "OperatorSlashed(address,(address,uint32),address[],uint256[],string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    128u8, 150u8, 154u8, 210u8, 148u8, 40u8, 214u8, 121u8, 126u8, 231u8, 170u8,
                    208u8, 132u8, 249u8, 228u8, 164u8, 42u8, 130u8, 252u8, 80u8, 109u8, 205u8,
                    44u8, 163u8, 182u8, 251u8, 67u8, 31u8, 133u8, 204u8, 235u8, 229u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
                        &self._delegation,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._DEALLOCATION_DELAY,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYCall> for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ALLOCATION_CONFIGURATION_DELAYCall {
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
            impl ::core::convert::From<ALLOCATION_CONFIGURATION_DELAYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ALLOCATION_CONFIGURATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ALLOCATION_CONFIGURATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ALLOCATION_CONFIGURATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ALLOCATION_CONFIGURATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<DEALLOCATION_DELAYCall> for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEALLOCATION_DELAYCall {
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
            impl ::core::convert::From<DEALLOCATION_DELAYReturn> for UnderlyingRustTuple<'_> {
                fn from(value: DEALLOCATION_DELAYReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for DEALLOCATION_DELAYReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for DEALLOCATION_DELAYCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = DEALLOCATION_DELAYReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `clearDeallocationQueue(address,address[],uint16[])` and selector `0x4b5046ef`.
    ```solidity
    function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct clearDeallocationQueueCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clearDeallocationQueueCall> for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueCall) -> Self {
                    (value.operator, value.strategies, value.numToClear)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearDeallocationQueueCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clearDeallocationQueueReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clearDeallocationQueueReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearDeallocationQueueReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearDeallocationQueueReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = delegationReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<encumberedMagnitudeCall> for UnderlyingRustTuple<'_> {
                fn from(value: encumberedMagnitudeCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encumberedMagnitudeCall {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u64,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<encumberedMagnitudeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: encumberedMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encumberedMagnitudeReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = encumberedMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocatableMagnitudeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatableMagnitudeCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocatableMagnitudeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatableMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatableMagnitudeReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatableMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationDelayCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationDelayReturn) -> Self {
                    (value.isSet, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationDelayReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationDelayReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationInfo_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_0Call) -> Self {
                    (value.operatorSet, value.strategies, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_0Call {
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
                    alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationInfo_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_0Return {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getAllocationInfo((address,uint32),address[],address[])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operatorSets:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationInfo_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_1Call) -> Self {
                    (value.operator, value.strategy, value.operatorSets)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_1Call {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::MagnitudeInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationInfo_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_1Return {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_1Return;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationInfo_2Call> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_2Call) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_2Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationInfo_2Return> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationInfo_2Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationInfo_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationInfo_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationInfo_2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeInfo>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesCall) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesCall {
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
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u64>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesAtTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtTimestampCall) -> Self {
                    (value.operator, value.strategies, value.timestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesAtTimestampCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u64>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesAtTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesAtTimestampReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesAtTimestampReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub beforeTimestamp: u32,
    }
    ///Container type for the return parameters of the [`getMinDelegatedAndSlashableOperatorShares((address,uint32),address[],address[],uint32)`](getMinDelegatedAndSlashableOperatorSharesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinDelegatedAndSlashableOperatorSharesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        >,
        pub _1: alloy::sol_types::private::Vec<
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMinDelegatedAndSlashableOperatorSharesCall>
                for UnderlyingRustTuple<'_>
            {
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
                for getMinDelegatedAndSlashableOperatorSharesCall
            {
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
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
            impl ::core::convert::From<getMinDelegatedAndSlashableOperatorSharesReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: getMinDelegatedAndSlashableOperatorSharesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for getMinDelegatedAndSlashableOperatorSharesReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinDelegatedAndSlashableOperatorSharesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeAllocation>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::MagnitudeAllocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<modifyAllocationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsCall) -> Self {
                    (value.allocations,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        allocations: tuple.0,
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
            impl ::core::convert::From<modifyAllocationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: modifyAllocationsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyAllocationsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::MagnitudeAllocation>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyAllocationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "modifyAllocations((address,uint64,(address,uint32)[],uint64[])[])";
            const SELECTOR: [u8; 4] = [22u8, 55u8, 182u8, 15u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::MagnitudeAllocation,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.allocations
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelay_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_0Call) -> Self {
                    (value.operator, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelay_0Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelay_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelay_0Return {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelay_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.delay,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelay_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_1Call) -> Self {
                    (value.delay,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelay_1Call {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelay_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelay_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelay_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllocationDelay_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelay_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.delay,
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
    /**Function with signature `slashOperator((address,uint32,address[],uint256,string))` and selector `0x60db99a3`.
    ```solidity
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params:
            <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "slashOperator((address,uint32,address[],uint256,string))";
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
        getMinDelegatedAndSlashableOperatorShares(getMinDelegatedAndSlashableOperatorSharesCall),
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
            )
                -> alloy_sol_types::Result<AllocationManagerCalls>] = &[
                {
                    fn getAllocationInfo_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationInfo_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AllocationManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AllocationManagerCalls::unpause)
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
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotAllocated(_) => {
                    <OperatorNotAllocated as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => <OutOfBounds as alloy_sol_types::SolError>::SELECTOR,
                Self::SaltSpent(_) => <SaltSpent as alloy_sol_types::SolError>::SELECTOR,
                Self::SameMagnitude(_) => <SameMagnitude as alloy_sol_types::SolError>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<AllocationManagerErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AllocationManagerErrors::SaltSpent)
                    }
                    SaltSpent
                },
                {
                    fn Empty(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <Empty as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AllocationManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <SameMagnitude as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <OutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(AllocationManagerErrors::UninitializedAllocationDelay)
                    }
                    UninitializedAllocationDelay
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
                        inner, out,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::Empty(inner) => {
                    <Empty as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InsufficientAllocatableMagnitude(inner) => {
                    <InsufficientAllocatableMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidAllocationDelay(inner) => {
                    <InvalidAllocationDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidExpectedTotalMagnitude(inner) => {
                    <InvalidExpectedTotalMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidTimestamp(inner) => {
                    <InvalidTimestamp as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyDelegationManager(inner) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorNotAllocated(inner) => {
                    <OperatorNotAllocated as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::UninitializedAllocationDelay(inner) => {
                    <UninitializedAllocationDelay as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
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
                28u8, 100u8, 88u8, 7u8, 154u8, 65u8, 7u8, 125u8, 0u8, 60u8, 17u8, 250u8, 249u8,
                191u8, 9u8, 126u8, 105u8, 59u8, 214u8, 121u8, 121u8, 228u8, 230u8, 80u8, 11u8,
                172u8, 123u8, 41u8, 219u8, 119u8, 155u8, 92u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                78u8, 133u8, 117u8, 29u8, 99u8, 49u8, 80u8, 108u8, 108u8, 98u8, 51u8, 95u8, 32u8,
                126u8, 179u8, 31u8, 18u8, 166u8, 30u8, 87u8, 15u8, 52u8, 245u8, 193u8, 118u8, 64u8,
                48u8, 135u8, 133u8, 198u8, 212u8, 219u8,
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
                128u8, 150u8, 154u8, 210u8, 148u8, 40u8, 214u8, 121u8, 126u8, 231u8, 170u8, 208u8,
                132u8, 249u8, 228u8, 164u8, 42u8, 130u8, 252u8, 80u8, 109u8, 205u8, 44u8, 163u8,
                182u8, 251u8, 67u8, 31u8, 133u8, 204u8, 235u8, 229u8,
            ],
            [
                139u8, 153u8, 126u8, 83u8, 215u8, 185u8, 229u8, 217u8, 35u8, 208u8, 162u8, 28u8,
                96u8, 223u8, 129u8, 225u8, 116u8, 8u8, 96u8, 209u8, 168u8, 198u8, 107u8, 140u8,
                99u8, 197u8, 4u8, 122u8, 226u8, 14u8, 170u8, 246u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                172u8, 249u8, 9u8, 95u8, 235u8, 58u8, 55u8, 12u8, 156u8, 246u8, 146u8, 66u8, 28u8,
                105u8, 239u8, 50u8, 13u8, 77u8, 181u8, 198u8, 110u8, 106u8, 125u8, 41u8, 199u8,
                105u8, 78u8, 176u8, 35u8, 100u8, 252u8, 85u8,
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
                Some(<AllocationDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AllocationDelaySet)
                }
                Some(<EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EncumberedMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EncumberedMagnitudeUpdated)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <MaxMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::MaxMagnitudeUpdated)
                }
                Some(
                    <OperatorSetMagnitudeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <OperatorSetMagnitudeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::OperatorSetMagnitudeUpdated),
                Some(<OperatorSlashed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSlashed as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSlashed)
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
                Some(<PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::PauserRegistrySet)
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::PauserRegistrySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<AllocationManagerInstance<T, P, N>>>
    {
        AllocationManagerInstance::<T, P, N>::deploy(
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
        AllocationManagerInstance::<T, P, N>::deploy_builder(
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
            f.debug_tuple("AllocationManagerInstance")
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
        > AllocationManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`AllocationManager`](self) contract instance.

        See the [wrapper's documentation](`AllocationManagerInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegation,
                        _avsDirectory,
                        _DEALLOCATION_DELAY,
                        _ALLOCATION_CONFIGURATION_DELAY,
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
        > AllocationManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`ALLOCATION_CONFIGURATION_DELAY`] function.
        pub fn ALLOCATION_CONFIGURATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, ALLOCATION_CONFIGURATION_DELAYCall, N> {
            self.call_builder(&ALLOCATION_CONFIGURATION_DELAYCall {})
        }
        ///Creates a new call builder for the [`DEALLOCATION_DELAY`] function.
        pub fn DEALLOCATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEALLOCATION_DELAYCall, N> {
            self.call_builder(&DEALLOCATION_DELAYCall {})
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`clearDeallocationQueue`] function.
        pub fn clearDeallocationQueue(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            numToClear: alloy::sol_types::private::Vec<u16>,
        ) -> alloy_contract::SolCallBuilder<T, &P, clearDeallocationQueueCall, N> {
            self.call_builder(&clearDeallocationQueueCall {
                operator,
                strategies,
                numToClear,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
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
            self.call_builder(&getAllocatableMagnitudeCall { operator, strategy })
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
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationInfo_0Call, N> {
            self.call_builder(&getAllocationInfo_0Call {
                operatorSet,
                strategies,
                operators,
            })
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
            self.call_builder(&getAllocationInfo_1Call {
                operator,
                strategy,
                operatorSets,
            })
        }
        ///Creates a new call builder for the [`getAllocationInfo_2`] function.
        pub fn getAllocationInfo_2(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationInfo_2Call, N> {
            self.call_builder(&getAllocationInfo_2Call { operator, strategy })
        }
        ///Creates a new call builder for the [`getMaxMagnitudes`] function.
        pub fn getMaxMagnitudes(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesCall, N> {
            self.call_builder(&getMaxMagnitudesCall {
                operator,
                strategies,
            })
        }
        ///Creates a new call builder for the [`getMaxMagnitudesAtTimestamp`] function.
        pub fn getMaxMagnitudesAtTimestamp(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            timestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesAtTimestampCall, N> {
            self.call_builder(&getMaxMagnitudesAtTimestampCall {
                operator,
                strategies,
                timestamp,
            })
        }
        ///Creates a new call builder for the [`getMinDelegatedAndSlashableOperatorShares`] function.
        pub fn getMinDelegatedAndSlashableOperatorShares(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            beforeTimestamp: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinDelegatedAndSlashableOperatorSharesCall, N>
        {
            self.call_builder(&getMinDelegatedAndSlashableOperatorSharesCall {
                operatorSet,
                operators,
                strategies,
                beforeTimestamp,
            })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _pauserRegistry,
                initialPausedStatus,
            })
        }
        ///Creates a new call builder for the [`modifyAllocations`] function.
        pub fn modifyAllocations(
            &self,
            allocations: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::MagnitudeAllocation as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyAllocationsCall, N> {
            self.call_builder(&modifyAllocationsCall { allocations })
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
            self.call_builder(&setAllocationDelay_0Call { operator, delay })
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
            self.call_builder(&setPauserRegistryCall { newPauserRegistry })
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
        > AllocationManagerInstance<T, P, N>
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
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
        pub fn OperatorSlashed_filter(&self) -> alloy_contract::Event<T, &P, OperatorSlashed, N> {
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
