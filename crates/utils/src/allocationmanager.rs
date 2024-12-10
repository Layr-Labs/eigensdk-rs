///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
    struct Allocation { uint64 currentMagnitude; int128 pendingDiff; uint32 effectBlock; }
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct DeregisterParams { address operator; address avs; uint32[] operatorSetIds; }
    struct RegisterParams { address avs; uint32[] operatorSetIds; bytes data; }
    struct SlashingParams { address operator; uint32 operatorSetId; uint256 wadToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
use crate::allocationmanager::AllocationManager::OperatorSet;

#[allow(non_snake_case)]
pub mod IAllocationManagerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocateParams {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub newMagnitudes: alloy::sol_types::private::Vec<u64>,
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
            OperatorSet,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <OperatorSet as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl ::core::convert::From<AllocateParams> for UnderlyingRustTuple<'_> {
            fn from(value: AllocateParams) -> Self {
                (value.operatorSet, value.strategies, value.newMagnitudes)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AllocateParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSet: tuple.0,
                    strategies: tuple.1,
                    newMagnitudes: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for AllocateParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for AllocateParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMagnitudes),
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
        impl alloy_sol_types::SolType for AllocateParams {
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
        impl alloy_sol_types::SolStruct for AllocateParams {
            const NAME: &'static str = "AllocateParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AllocateParams(OperatorSet operatorSet,address[] strategies,uint64[] newMagnitudes)",
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
                    <OperatorSet as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSet,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.newMagnitudes)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for AllocateParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <OperatorSet as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSet,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategies,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<64>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.newMagnitudes,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <OperatorSet as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSet,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<64>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.newMagnitudes,
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
    struct Allocation { uint64 currentMagnitude; int128 pendingDiff; uint32 effectBlock; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Allocation {
        pub currentMagnitude: u64,
        pub pendingDiff: i128,
        pub effectBlock: u32,
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
        impl ::core::convert::From<Allocation> for UnderlyingRustTuple<'_> {
            fn from(value: Allocation) -> Self {
                (value.currentMagnitude, value.pendingDiff, value.effectBlock)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Allocation {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currentMagnitude: tuple.0,
                    pendingDiff: tuple.1,
                    effectBlock: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Allocation {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Allocation {
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
                        &self.effectBlock,
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
        impl alloy_sol_types::SolType for Allocation {
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
        impl alloy_sol_types::SolStruct for Allocation {
            const NAME: &'static str = "Allocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Allocation(uint64 currentMagnitude,int128 pendingDiff,uint32 effectBlock)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.effectBlock)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Allocation {
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
                        &rust.effectBlock,
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
                    &rust.effectBlock,
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
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreateSetParams {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl ::core::convert::From<CreateSetParams> for UnderlyingRustTuple<'_> {
            fn from(value: CreateSetParams) -> Self {
                (value.operatorSetId, value.strategies)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CreateSetParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operatorSetId: tuple.0,
                    strategies: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CreateSetParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CreateSetParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategies),
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
        impl alloy_sol_types::SolType for CreateSetParams {
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
        impl alloy_sol_types::SolStruct for CreateSetParams {
            const NAME: &'static str = "CreateSetParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreateSetParams(uint32 operatorSetId,address[] strategies)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.strategies)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CreateSetParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
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
                    &rust.operatorSetId,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.strategies,
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
    struct DeregisterParams { address operator; address avs; uint32[] operatorSetIds; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DeregisterParams {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
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
        impl ::core::convert::From<DeregisterParams> for UnderlyingRustTuple<'_> {
            fn from(value: DeregisterParams) -> Self {
                (value.operator, value.avs, value.operatorSetIds)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DeregisterParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    avs: tuple.1,
                    operatorSetIds: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DeregisterParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DeregisterParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
        impl alloy_sol_types::SolType for DeregisterParams {
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
        impl alloy_sol_types::SolStruct for DeregisterParams {
            const NAME: &'static str = "DeregisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeregisterParams(address operator,address avs,uint32[] operatorSetIds)",
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.avs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSetIds,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DeregisterParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetIds,
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
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetIds,
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
    struct RegisterParams { address avs; uint32[] operatorSetIds; bytes data; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct RegisterParams {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub data: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<RegisterParams> for UnderlyingRustTuple<'_> {
            fn from(value: RegisterParams) -> Self {
                (value.avs, value.operatorSetIds, value.data)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for RegisterParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    operatorSetIds: tuple.1,
                    data: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for RegisterParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for RegisterParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
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
        impl alloy_sol_types::SolType for RegisterParams {
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
        impl alloy_sol_types::SolStruct for RegisterParams {
            const NAME: &'static str = "RegisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterParams(address avs,uint32[] operatorSetIds,bytes data)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.operatorSetIds,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.data,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for RegisterParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.avs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetIds,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.data,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetIds,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
    struct SlashingParams { address operator; uint32 operatorSetId; uint256 wadToSlash; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
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
                    wadToSlash: tuple.2,
                    description: tuple.3,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSetId,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.wadToSlash,
                    ),
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
                    "SlashingParams(address operator,uint32 operatorSetId,uint256 wadToSlash,string description)",
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
    struct AllocateParams {
        OperatorSet operatorSet;
        address[] strategies;
        uint64[] newMagnitudes;
    }
    struct Allocation {
        uint64 currentMagnitude;
        int128 pendingDiff;
        uint32 effectBlock;
    }
    struct CreateSetParams {
        uint32 operatorSetId;
        address[] strategies;
    }
    struct DeregisterParams {
        address operator;
        address avs;
        uint32[] operatorSetIds;
    }
    struct RegisterParams {
        address avs;
        uint32[] operatorSetIds;
        bytes data;
    }
    struct SlashingParams {
        address operator;
        uint32 operatorSetId;
        uint256 wadToSlash;
        string description;
    }
}

interface AllocationManager {
    struct OperatorSet {
        address avs;
        uint32 id;
    }

    error AlreadyMemberOfSet();
    error CurrentlyPaused();
    error Empty();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InsufficientMagnitude();
    error InvalidBlockNumber();
    error InvalidCaller();
    error InvalidNewPausedStatus();
    error InvalidOperator();
    error InvalidOperatorSet();
    error InvalidSignature();
    error InvalidStrategy();
    error InvalidWadToSlash();
    error ModificationAlreadyPending();
    error NotMemberOfSet();
    error OnlyDelegationManager();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error OperatorNotSlashable();
    error OutOfBounds();
    error SameMagnitude();
    error SignatureExpired();
    error StrategyAlreadyInOperatorSet();
    error StrategyNotInOperatorSet();
    error UninitializedAllocationDelay();

    event AVSMetadataURIUpdated(address indexed avs, string metadataURI);
    event AVSRegistrarSet(address avs, address registrar);
    event AllocationDelaySet(address operator, uint32 delay, uint32 effectBlock);
    event AllocationUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectBlock);
    event EncumberedMagnitudeUpdated(address operator, address strategy, uint64 encumberedMagnitude);
    event Initialized(uint8 version);
    event MaxMagnitudeUpdated(address operator, address strategy, uint64 maxMagnitude);
    event OperatorAddedToOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorRemovedFromOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorSetCreated(OperatorSet operatorSet);
    event OperatorSlashed(address operator, OperatorSet operatorSet, address[] strategies, uint256[] wadSlashed, string description);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StrategyAddedToOperatorSet(OperatorSet operatorSet, address strategy);
    event StrategyRemovedFromOperatorSet(OperatorSet operatorSet, address strategy);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegation, address _pauserRegistry, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);

    function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
    function DEALLOCATION_DELAY() external view returns (uint32);
    function addStrategiesToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
    function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
    function delegation() external view returns (address);
    function deregisterFromOperatorSets(IAllocationManagerTypes.DeregisterParams memory params) external;
    function encumberedMagnitude(address operator, address strategy) external view returns (uint64);
    function getAVSRegistrar(address avs) external view returns (address);
    function getAllocatableMagnitude(address operator, address strategy) external view returns (uint64);
    function getAllocatedSets(address operator) external view returns (OperatorSet[] memory);
    function getAllocatedStrategies(address operator, OperatorSet memory operatorSet) external view returns (address[] memory);
    function getAllocation(address operator, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation memory);
    function getAllocationDelay(address operator) external view returns (bool, uint32);
    function getAllocations(address[] memory operators, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation[] memory);
    function getMaxMagnitude(address operator, address strategy) external view returns (uint64);
    function getMaxMagnitudes(address[] memory operators, address strategy) external view returns (uint64[] memory);
    function getMaxMagnitudes(address operator, address[] memory strategies) external view returns (uint64[] memory);
    function getMaxMagnitudesAtBlock(address operator, address[] memory strategies, uint32 blockNumber) external view returns (uint64[] memory);
    function getMemberCount(OperatorSet memory operatorSet) external view returns (uint256);
    function getMembers(OperatorSet memory operatorSet) external view returns (address[] memory);
    function getMinimumSlashableStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 futureBlock) external view returns (uint256[][] memory slashableStake);
    function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
    function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory);
    function getStrategyAllocations(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.Allocation[] memory);
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
    function modifyAllocations(IAllocationManagerTypes.AllocateParams[] memory params) external;
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function registerForOperatorSets(IAllocationManagerTypes.RegisterParams memory params) external;
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function setAVSRegistrar(address registrar) external;
    function setAllocationDelay(address operator, uint32 delay) external;
    function setAllocationDelay(uint32 delay) external;
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateAVSMetadataURI(string memory metadataURI) external;
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
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
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
    "name": "addStrategiesToOperatorSet",
    "inputs": [
      {
        "name": "operatorSetId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "createOperatorSets",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.CreateSetParams[]",
        "components": [
          {
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "strategies",
            "type": "address[]",
            "internalType": "contract IStrategy[]"
          }
        ]
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
    "name": "deregisterFromOperatorSets",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.DeregisterParams",
        "components": [
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
            "name": "operatorSetIds",
            "type": "uint32[]",
            "internalType": "uint32[]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
    "name": "getAVSRegistrar",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
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
    "name": "getAllocatedSets",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
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
    "name": "getAllocatedStrategies",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getAllocation",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.Allocation",
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
            "name": "effectBlock",
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
        "name": "",
        "type": "bool",
        "internalType": "bool"
      },
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
    "name": "getAllocations",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
        "internalType": "struct IAllocationManagerTypes.Allocation[]",
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
            "name": "effectBlock",
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
    "name": "getMaxMagnitude",
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
    "name": "getMaxMagnitudes",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
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
        "type": "uint64[]",
        "internalType": "uint64[]"
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
    "name": "getMaxMagnitudesAtBlock",
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
        "name": "blockNumber",
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
    "name": "getMemberCount",
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "getMembers",
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
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
    "name": "getMinimumSlashableStake",
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
            "name": "id",
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
        "name": "futureBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "slashableStake",
        "type": "uint256[][]",
        "internalType": "uint256[][]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getRegisteredSets",
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
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "id",
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
    "name": "getStrategiesInOperatorSet",
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStrategyAllocations",
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.Allocation[]",
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
            "name": "effectBlock",
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
    "name": "isOperatorSet",
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
            "name": "id",
            "type": "uint32",
            "internalType": "uint32"
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
    "name": "modifyAllocations",
    "inputs": [
      {
        "name": "params",
        "type": "tuple[]",
        "internalType": "struct IAllocationManagerTypes.AllocateParams[]",
        "components": [
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
                "name": "id",
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
            "name": "newMagnitudes",
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
    "name": "registerForOperatorSets",
    "inputs": [
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IAllocationManagerTypes.RegisterParams",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetIds",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "data",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "removeStrategiesFromOperatorSet",
    "inputs": [
      {
        "name": "operatorSetId",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
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
    "name": "setAVSRegistrar",
    "inputs": [
      {
        "name": "registrar",
        "type": "address",
        "internalType": "contract IAVSRegistrar"
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
    "type": "function",
    "name": "updateAVSMetadataURI",
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
    "name": "AVSMetadataURIUpdated",
    "inputs": [
      {
        "name": "avs",
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
    "name": "AVSRegistrarSet",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "registrar",
        "type": "address",
        "indexed": false,
        "internalType": "contract IAVSRegistrar"
      }
    ],
    "anonymous": false
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
        "name": "effectBlock",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "AllocationUpdated",
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
            "name": "id",
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
        "name": "effectBlock",
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
    "name": "OperatorAddedToOperatorSet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
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
            "name": "id",
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
    "name": "OperatorRemovedFromOperatorSet",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
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
            "name": "id",
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
    "name": "OperatorSetCreated",
    "inputs": [
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
            "name": "id",
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
            "name": "id",
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
    "name": "StrategyAddedToOperatorSet",
    "inputs": [
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
            "name": "id",
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
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyRemovedFromOperatorSet",
    "inputs": [
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
            "name": "id",
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
    "name": "AlreadyMemberOfSet",
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
    "name": "InsufficientMagnitude",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBlockNumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCaller",
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
    "name": "InvalidStrategy",
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
    "name": "NotMemberOfSet",
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
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotSlashable",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OutOfBounds",
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
    "name": "StrategyAlreadyInOperatorSet",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StrategyNotInOperatorSet",
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
    ///0x610100604052348015610010575f5ffd5b506040516153d43803806153d483398101604081905261002f9161017a565b838282856001600160a01b03811661005a576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b039081166080529290921660a05263ffffffff90811660c0521660e05261008661008f565b505050506101d0565b5f54610100900460ff16156100fa5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff90811614610149575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b038116811461015f575f5ffd5b50565b805163ffffffff81168114610175575f5ffd5b919050565b5f5f5f5f6080858703121561018d575f5ffd5b84516101988161014b565b60208601519094506101a98161014b565b92506101b760408601610162565b91506101c560608601610162565b905092959194509250565b60805160a05160c05160e0516151846102505f395f818161050b015261386f01525f818161031b015281816110e8015261201e01525f818161069701528181610b1f015281816113fa01528181611a7601528181611b82015261282201525f818161054501528181610c7b01528181611ad30152612ef901526151845ff3fe608060405234801561000f575f5ffd5b506004361061026b575f3560e01c8063715018a61161014b578063a98fb355116100bf578063cd6dc68711610084578063cd6dc6871461066c578063ce7b5e4b1461067f578063df5cf72314610692578063f25f1610146106b9578063f2fde38b146106cc578063fabc1cbc146106df575f5ffd5b8063a98fb355146105f1578063b2447af714610604578063b92f60a514610617578063b9fbaed11461062a578063c221d8ae14610659575f5ffd5b8063886f119511610110578063886f1195146105405780638ce64854146105675780638da5cb5b1461058757806394d7d00c14610598578063a9333ec8146105ab578063a984eb3a146105be575f5ffd5b8063715018a6146104d857806376999342146104e057806379ae50cd146104f35780637bc1ef6114610506578063847d634f1461052d575f5ffd5b80634a10ffe5116101e25780635ac86ab7116101a75780635ac86ab71461043f5780635c489bb5146104625780635c975abb146104755780636cfb4481146104875780636e3492b5146104b25780636e875dba146104c5575f5ffd5b80634a10ffe5146103de5780634b5046ef146103fe578063547afb871461041157806356c483e614610424578063595c6a6714610437575f5ffd5b8063260dc75811610233578063260dc758146102f35780632981eb77146103165780632bab2c4a14610352578063304c10cd1461037257806340120dab1461039d5780634177a87c146103be575f5ffd5b80630ea43e431461026f57806310e1b9b814610284578063136439dd146102ad57806315ea7917146102c057806315fe5028146102d3575b5f5ffd5b61028261027d366004614213565b6106f2565b005b610297610292366004614311565b610c31565b6040516102a49190614358565b60405180910390f35b6102826102bb36600461438b565b610c66565b6102826102ce3660046143e2565b610d3b565b6102e66102e1366004614420565b611258565b6040516102a4919061449e565b6103066103013660046144b0565b61136f565b60405190151581526020016102a4565b61033d7f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102a4565b61036561036036600461455f565b6113a6565b6040516102a49190614613565b610385610380366004614420565b611693565b6040516001600160a01b0390911681526020016102a4565b6103b06103ab366004614676565b6116c2565b6040516102a492919061470e565b6103d16103cc3660046144b0565b61183d565b6040516102a4919061476b565b6103f16103ec36600461477d565b611861565b6040516102a491906147c0565b61028261040c36600461480b565b611909565b6103f161041f36600461488b565b6119c3565b6102826104323660046148d7565b611a6b565b610282611abe565b61030661044d36600461490a565b606654600160ff9092169190911b9081161490565b61028261047036600461492a565b611b6d565b6066545b6040519081526020016102a4565b61049a610495366004614676565b611c1d565b6040516001600160401b0390911681526020016102a4565b6102826104c0366004614959565b611d8a565b6103d16104d33660046144b0565b61215a565b61028261216b565b6102826104ee36600461498a565b61217c565b6102e6610501366004614420565b6122a6565b61033d7f000000000000000000000000000000000000000000000000000000000000000081565b61028261053b3660046143e2565b612380565b6103857f000000000000000000000000000000000000000000000000000000000000000081565b61057a6105753660046149d8565b6125ad565b6040516102a49190614a1b565b6033546001600160a01b0316610385565b6103f16105a6366004614a2d565b612672565b61049a6105b9366004614676565b61275e565b61049a6105cc366004614676565b60a260209081525f92835260408084209091529082529020546001600160401b031681565b6102826105ff366004614a88565b61278d565b6104796106123660046144b0565b6127d4565b610282610625366004614959565b6127e4565b61063d610638366004614420565b612af8565b60408051921515835263ffffffff9091166020830152016102a4565b6103d1610667366004614af4565b612b92565b61028261067a366004614b1f565b612bc3565b61028261068d36600461498a565b612cdf565b6103857f000000000000000000000000000000000000000000000000000000000000000081565b6102826106c7366004614420565b612e09565b6102826106da366004614420565b612e81565b6102826106ed36600461438b565b612ef7565b60665460019060029081160361071b5760405163840a48d560e01b815260040160405180910390fd5b81604001355f10801561073a5750670de0b6b3a7640000604083013511155b61075757604051631353603160e01b815260040160405180910390fd5b5f6040518060400160405280336001600160a01b03168152602001846020016020810190610785919061492a565b63ffffffff16905290505f6107a66107a06020860186614420565b83613006565b60208084015184516001600160a01b03165f908152609890925260409091209192506107dc919063ffffffff9081169061307b16565b6107f957604051631fb1705560e21b815260040160405180910390fd5b80610817576040516325131d4f60e01b815260040160405180910390fd5b5f61083960995f61082786613092565b81526020019081526020015f206130f0565b90505f816001600160401b038111156108545761085461425d565b60405190808252806020026020018201604052801561087d578160200160208202803683370190505b5090505f826001600160401b038111156108995761089961425d565b6040519080825280602002602001820160405280156108c2578160200160208202803683370190505b5090505f5b83811015610bcf575f6108fb8260995f6108e08b613092565b81526020019081526020015f206130f990919063ffffffff16565b90505f8061091e61090f60208d018d614420565b6109188b613092565b85613104565b915091508286858151811061093557610935614b49565b6001600160a01b039092166020928302919091019091015280516001600160401b03165f0361096657505050610bc7565b80515f90610981906001600160401b031660408e0135613270565b83519091505f9061099e906001600160401b038085169116613286565b9050808787815181106109b3576109b3614b49565b60200260200101818152505081835f018181516109d09190614b71565b6001600160401b03169052508351829085906109ed908390614b71565b6001600160401b0316905250602084018051839190610a0d908390614b71565b6001600160401b031690525060208301515f600f9190910b1215610ae3575f610a518e604001358560200151610a4290614b90565b6001600160801b031690613270565b9050806001600160401b031684602001818151610a6e9190614bb4565b915090600f0b9081600f0b815250507f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd8e5f016020810190610ab09190614420565b8d88610ac3885f0151896020015161329a565b8860400151604051610ad9959493929190614be1565b60405180910390a1505b610b04610af360208f018f614420565b610afc8d613092565b8787876132ae565b610b1d610b1460208f018f614420565b8551879061351a565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b6f73bdf8e5f016020810190610b5e9190614420565b6040516001600160e01b031960e084901b1681526001600160a01b0391821660048201529088166024820152604481018490526064015f604051808303815f87803b158015610bab575f5ffd5b505af1158015610bbd573d5f5f3e3d5ffd5b5050505050505050505b6001016108c7565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5610bfe6020890189614420565b868484610c0e60608d018d614c32565b604051610c2096959493929190614c9c565b60405180910390a150505050505050565b604080516060810182525f8082526020820181905291810182905290610c5a8561091886613092565b925050505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610cc8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cec9190614cfe565b610d0957604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610d2e5760405163c61dca5d60e01b815260040160405180910390fd5b610d378261359e565b5050565b6066545f90600190811603610d635760405163840a48d560e01b815260040160405180910390fd5b5f5f610d6e33612af8565b9150915081610d905760405163fa55fc8160e01b815260040160405180910390fd5b5f5b8481101561125057858582818110610dac57610dac614b49565b9050602002810190610dbe9190614d1d565b610dcc906060810190614d3b565b9050868683818110610de057610de0614b49565b9050602002810190610df29190614d1d565b610e00906040810190614d3b565b905014610e20576040516343714afd60e01b815260040160405180910390fd5b36868683818110610e3357610e33614b49565b9050602002810190610e459190614d1d565b90505f610e6033610e5b368590038501856144b0565b613006565b9050610ea9610e75604084016020850161492a565b63ffffffff1660985f610e8b6020870187614420565b6001600160a01b0316815260208101919091526040015f209061307b565b610ec657604051631fb1705560e21b815260040160405180910390fd5b5f5b888885818110610eda57610eda614b49565b9050602002810190610eec9190614d1d565b610efa906040810190614d3b565b9050811015611245575f898986818110610f1657610f16614b49565b9050602002810190610f289190614d1d565b610f36906040810190614d3b565b83818110610f4657610f46614b49565b9050602002016020810190610f5b9190614420565b9050610f6a338261ffff6135db565b5f80610f8733610918610f82368a90038a018a6144b0565b613092565b915091508060200151600f0b5f14610fb257604051630d8fcbe360e41b815260040160405180910390fd5b5f610fcd610fc5368990038901896144b0565b8584896136df565b9050611032825f01518e8e8b818110610fe857610fe8614b49565b9050602002810190610ffa9190614d1d565b611008906060810190614d3b565b8881811061101857611018614b49565b905060200201602081019061102d9190614d80565b613745565b600f0b602083018190525f0361105b57604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b12156111aa57801561111d576110e3611085610f82368a90038a018a6144b0565b335f90815260a3602090815260408083206001600160a01b038a1684529091529020908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b61110d7f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff166040830152611217565b61112f8360200151836020015161329a565b6001600160401b031660208401528c8c8981811061114f5761114f614b49565b90506020028101906111619190614d1d565b61116f906060810190614d3b565b8681811061117f5761117f614b49565b90506020020160208101906111949190614d80565b6001600160401b031682525f6020830152611217565b5f8260200151600f0b1315611217576111cb8360200151836020015161329a565b6001600160401b03908116602085018190528451909116101561120157604051636c9be0bf60e01b815260040160405180910390fd5b61120b8943614da6565b63ffffffff1660408301525b6112353361122d610f82368b90038b018b6144b0565b8686866132ae565b505060019092019150610ec89050565b505050600101610d92565b505050505050565b6001600160a01b0381165f908152609d602052604081206060919061127c906130f0565b90505f816001600160401b038111156112975761129761425d565b6040519080825280602002602001820160405280156112db57816020015b604080518082019091525f80825260208201528152602001906001900390816112b55790505b5090505f5b82811015611367576001600160a01b0385165f908152609d602052604090206113429061130d90836130f9565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b82828151811061135457611354614b49565b60209081029190910101526001016112e0565b509392505050565b60208082015182516001600160a01b03165f9081526098909252604082206113a09163ffffffff9081169061307b16565b92915050565b606083516001600160401b038111156113c1576113c161425d565b6040519080825280602002602001820160405280156113f457816020015b60608152602001906001900390816113df5790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b8152600401611446929190614dc2565b5f60405180830381865afa158015611460573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526114879190810190614de6565b90505f5b8551811015611689575f8682815181106114a7576114a7614b49565b6020026020010151905085516001600160401b038111156114ca576114ca61425d565b6040519080825280602002602001820160405280156114f3578160200160208202803683370190505b5084838151811061150657611506614b49565b60209081029190910101525f5b865181101561167f575f87828151811061152f5761152f614b49565b6020908102919091018101516001600160a01b038086165f90815260a184526040808220928416825291909352822090925061156a9061375c565b9050806001600160401b03165f03611583575050611677565b5f61158f858d85610c31565b90508863ffffffff16816040015163ffffffff16111580156115b757505f8160200151600f0b125b156115d9576115cd815f0151826020015161329a565b6001600160401b031681525b80515f906115f4906001600160401b03908116908516613286565b905061163b8189898151811061160c5761160c614b49565b6020026020010151878151811061162557611625614b49565b60200260200101516137a890919063ffffffff16565b89888151811061164d5761164d614b49565b6020026020010151868151811061166657611666614b49565b602002602001018181525050505050505b600101611513565b505060010161148b565b5050949350505050565b6001600160a01b038082165f9081526097602052604081205490911680156116bb5780610c5f565b5090919050565b6001600160a01b0382165f908152609d6020526040812060609182916116e7906130f0565b90505f816001600160401b038111156117025761170261425d565b60405190808252806020026020018201604052801561174657816020015b604080518082019091525f80825260208201528152602001906001900390816117205790505b5090505f826001600160401b038111156117625761176261425d565b6040519080825280602002602001820160405280156117ab57816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816117805790505b5090505f5b8381101561182e576001600160a01b0388165f908152609d602052604081206117dd9061130d90846130f9565b9050808483815181106117f2576117f2614b49565b602002602001018190525061180889828a610c31565b83838151811061181a5761181a614b49565b6020908102919091010152506001016117b0565b509093509150505b9250929050565b60605f610c5f60995f61184f86613092565b81526020019081526020015f206137bc565b60605f83516001600160401b0381111561187d5761187d61425d565b6040519080825280602002602001820160405280156118a6578160200160208202803683370190505b5090505f5b8451811015611367576118d78582815181106118c9576118c9614b49565b60200260200101518561275e565b8282815181106118e9576118e9614b49565b6001600160401b03909216602092830291909101909101526001016118ab565b6066545f906001908116036119315760405163840a48d560e01b815260040160405180910390fd5b838214611951576040516343714afd60e01b815260040160405180910390fd5b5f5b848110156119ba576119b28787878481811061197157611971614b49565b90506020020160208101906119869190614420565b86868581811061199857611998614b49565b90506020020160208101906119ad9190614ef2565b6135db565b600101611953565b50505050505050565b60605f82516001600160401b038111156119df576119df61425d565b604051908082528060200260200182016040528015611a08578160200160208202803683370190505b5090505f5b835181101561136757611a3985858381518110611a2c57611a2c614b49565b602002602001015161275e565b828281518110611a4b57611a4b614b49565b6001600160401b0390921660209283029190910190910152600101611a0d565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611ab45760405163f739589b60e01b815260040160405180910390fd5b610d3782826137c8565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611b20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b449190614cfe565b611b6157604051631d77d47760e21b815260040160405180910390fd5b611b6b5f1961359e565b565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa158015611bcf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf39190614cfe565b611c10576040516325ec6c1f60e01b815260040160405180910390fd5b611c1a33826137c8565b50565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611d47576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120611caf9083613969565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015611d2a575050611d47565b611d3885826020015161329a565b94505050806001019050611c77565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208290611d779061375c565b611d819190614b71565b95945050505050565b606654600290600490811603611db35760405163840a48d560e01b815260040160405180910390fd5b611dc06020830183614420565b6001600160a01b0316336001600160a01b03161480611dff5750611dea6040830160208401614420565b6001600160a01b0316336001600160a01b0316145b611e1c576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b611e2b6040840184614d3b565b90508110156120cf575f6040518060400160405280856020016020810190611e539190614420565b6001600160a01b03168152602001611e6e6040870187614d3b565b85818110611e7e57611e7e614b49565b9050602002016020810190611e93919061492a565b63ffffffff168152509050611ec2816020015163ffffffff1660985f876020016020810190610e8b9190614420565b611edf57604051631fb1705560e21b815260040160405180910390fd5b609e5f611eef6020870187614420565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611f1883613092565b815260208101919091526040015f205460ff16611f48576040516325131d4f60e01b815260040160405180910390fd5b611f82611f5482613092565b609c5f611f646020890189614420565b6001600160a01b0316815260208101919091526040015f20906139d8565b50611fba611f936020860186614420565b609a5f611f9f85613092565b81526020019081526020015f206139e390919063ffffffff16565b50611fc86020850185614420565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe826040516120009190614f13565b60405180910390a2604080518082019091525f8152602081016120437f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff169052609e5f61205b6020880188614420565b6001600160a01b03166001600160a01b031681526020019081526020015f205f61208484613092565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101611e1e565b506120e36103806040840160208501614420565b6001600160a01b0316639d8e0c236120fe6020850185614420565b61210b6040860186614d3b565b6040518463ffffffff1660e01b815260040161212993929190614f5a565b5f604051808303815f87803b158015612140575f5ffd5b505af1925050508015612151575060015b15610d37575050565b60606113a0609a5f61184f85613092565b6121736139f7565b611b6b5f613a51565b6040805180820182523380825263ffffffff80871660208085018290525f938452609890529390912091926121b2929161307b16565b6121cf57604051631fb1705560e21b815260040160405180910390fd5b5f6121d982613092565b90505f5b83811015611250576122228585838181106121fa576121fa614b49565b905060200201602081019061220f9190614420565b5f84815260996020526040902090613aa2565b61223f5760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b8386868481811061227357612273614b49565b90506020020160208101906122889190614420565b604051612296929190614f7e565b60405180910390a16001016121dd565b6001600160a01b0381165f908152609c60205260408120606091906122ca906130f0565b90505f816001600160401b038111156122e5576122e561425d565b60405190808252806020026020018201604052801561232957816020015b604080518082019091525f80825260208201528152602001906001900390816123035790505b5090505f5b82811015611367576001600160a01b0385165f908152609c6020526040902061235b9061130d90836130f9565b82828151811061236d5761236d614b49565b602090810291909101015260010161232e565b5f5b818110156125a8575f6040518060400160405280336001600160a01b031681526020018585858181106123b7576123b7614b49565b90506020028101906123c99190614fa4565b6123d790602081019061492a565b63ffffffff908116909152602080830151335f90815260989092526040909120929350612409929190811690613ab616565b61242657604051631fb1705560e21b815260040160405180910390fd5b60408051808201825233815260208381015163ffffffff169082015290517f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c9161246f91614f13565b60405180910390a15f61248182613092565b90505f5b85858581811061249757612497614b49565b90506020028101906124a99190614fa4565b6124b7906020810190614d3b565b905081101561259d576125058686868181106124d5576124d5614b49565b90506020028101906124e79190614fa4565b6124f5906020810190614d3b565b838181106121fa576121fa614b49565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b8387878781811061253a5761253a614b49565b905060200281019061254c9190614fa4565b61255a906020810190614d3b565b8481811061256a5761256a614b49565b905060200201602081019061257f9190614420565b60405161258d929190614f7e565b60405180910390a1600101612485565b505050600101612382565b505050565b60605f84516001600160401b038111156125c9576125c961425d565b60405190808252806020026020018201604052801561261257816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816125e75790505b5090505f5b85518110156126695761264486828151811061263557612635614b49565b60200260200101518686610c31565b82828151811061265657612656614b49565b6020908102919091010152600101612617565b50949350505050565b60605f83516001600160401b0381111561268e5761268e61425d565b6040519080825280602002602001820160405280156126b7578160200160208202803683370190505b5090505f5b8451811015612669576001600160a01b0386165f90815260a160205260408120865161272c928792918990869081106126f7576126f7614b49565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f20613ac190919063ffffffff16565b82828151811061273e5761273e614b49565b6001600160401b03909216602092830291909101909101526001016126bc565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610c5f9061375c565b336001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371383836040516127c8929190614fb8565b60405180910390a25050565b5f6113a0609a5f61082785613092565b60665460029060049081160361280d5760405163840a48d560e01b815260040160405180910390fd5b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa15801561286f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128939190614cfe565b6128b05760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b6128bf6020840184614d3b565b9050811015612a7557604080518082019091525f90806128e26020870187614420565b6001600160a01b031681526020018580602001906129009190614d3b565b8581811061291057612910614b49565b9050602002016020810190612925919061492a565b63ffffffff90811690915260208083015183516001600160a01b03165f9081526098909252604090912092935061296192919081169061307b16565b61297e57604051631fb1705560e21b815260040160405180910390fd5b6129883382613006565b156129a657604051636c6c6e2760e11b815260040160405180910390fd5b6129c66129b282613092565b335f908152609c6020526040902090613ab6565b506129f233609a5f6129d785613092565b81526020019081526020015f20613aa290919063ffffffff16565b50336001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612a2c9190614f13565b60405180910390a2335f908152609e60205260408120600191612a4e84613092565b815260208101919091526040015f20805460ff1916911515919091179055506001016128b2565b50612a866103806020840184614420565b6001600160a01b031663adcf73f733612aa26020860186614d3b565b612aaf6040880188614c32565b6040518663ffffffff1660e01b8152600401612acf959493929190614fcb565b5f604051808303815f87803b158015612ae6575f5ffd5b505af1158015611250573d5f5f3e3d5ffd5b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052650100000000008304821694840194909452600160481b909104166060820181905284939192919015801590612b795750826060015163ffffffff164310155b15612b88575050604081015160015b9590945092505050565b6001600160a01b0382165f908152609f6020526040812060609190612bbb908261184f86613092565b949350505050565b5f54610100900460ff1615808015612be157505f54600160ff909116105b80612bfa5750303b158015612bfa57505f5460ff166001145b612c625760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015612c83575f805461ff0019166101001790555b612c8c8261359e565b612c9583613a51565b80156125a8575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a1505050565b6040805180820182523380825263ffffffff80871660208085018290525f93845260989052939091209192612d15929161307b16565b612d3257604051631fb1705560e21b815260040160405180910390fd5b5f612d3c82613092565b90505f5b8381101561125057612d85858583818110612d5d57612d5d614b49565b9050602002016020810190612d729190614420565b5f848152609960205260409020906139e3565b612da2576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee83868684818110612dd657612dd6614b49565b9050602002016020810190612deb9190614420565b604051612df9929190614f7e565b60405180910390a1600101612d40565b335f81815260976020526040902080546001600160a01b0319166001600160a01b0384161790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf8590612e5b81611693565b604080516001600160a01b0393841681529290911660208301520160405180910390a150565b612e896139f7565b6001600160a01b038116612eee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401612c59565b611c1a81613a51565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f77919061500e565b6001600160a01b0316336001600160a01b031614612fa85760405163794821ff60e01b815260040160405180910390fd5b60665480198219811614612fcf5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016127c8565b6001600160a01b0382165f908152609e6020526040812081908161302985613092565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff1692820192909252915080612bbb57506020015163ffffffff1643109392505050565b5f8181526001830160205260408120541515610c5f565b5f815f0151826020015163ffffffff166040516020016130d892919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526113a090615029565b5f6113a0825490565b5f610c5f8383613b11565b6040805180820182525f80825260208083018290528351606081018552828152808201839052808501839052845180860186526001600160a01b03898116855260a18452868520908816855290925293822092939281906131649061375c565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff16918101829052919250431015613206579092509050613268565b613217815f0151826020015161329a565b6001600160401b0316815260208101515f600f9190910b1215613255576132468260200151826020015161329a565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b5f610c5f8383670de0b6b3a76400006001613b37565b5f610c5f83670de0b6b3a764000084613b90565b5f610c5f826001600160401b038516614bb4565b602082810180516001600160a01b038881165f81815260a286526040808220938a1680835293875290819020805467ffffffffffffffff19166001600160401b0395861617905593518451918252948101919091529216908201527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559060600160405180910390a16001600160a01b038581165f90815260a060209081526040808320888452825280832093871683529281528282208451815486840151878701516001600160401b039093166001600160c01b031990921691909117600160401b6001600160801b03909216919091021763ffffffff60c01b1916600160c01b63ffffffff9283160217909155835180850185528381528201929092528251808401909352606087901c8352908616908201527f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd908690835160405161341a93929188914390614be1565b60405180910390a16020810151600f0b15613483576001600160a01b0385165f908152609f60209081526040808320878452909152902061345b9084613aa2565b506001600160a01b0385165f908152609d6020526040902061347d9085613ab6565b50613513565b80516001600160401b03165f03613513576001600160a01b0385165f908152609f6020908152604080832087845290915290206134c090846139e3565b506001600160a01b0385165f908152609f6020908152604080832087845290915290206134ec906130f0565b5f03613513576001600160a01b0385165f908152609d6020526040902061125090856139d8565b5050505050565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613549904383613c75565b5050604080516001600160a01b038086168252841660208201526001600160401b038316918101919091527f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c90606001612cd2565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f8111801561362557508261ffff1682105b15613513576001600160a01b038086165f90815260a360209081526040808320938816835292905290812061365990613c8e565b90505f5f613668338489613104565b91509150806040015163ffffffff1643101561368657505050613513565b61369388848985856132ae565b6001600160a01b038089165f90815260a360209081526040808320938b168352929052206136c090613ce0565b506136ca8561504c565b94506136d584615064565b9350505050613613565b5f61370b8460995f6136f089613092565b81526020019081526020015f20613d5d90919063ffffffff16565b61371657505f612bbb565b8161372257505f612bbb565b82516001600160401b03165f0361373a57505f612bbb565b506001949350505050565b5f610c5f6001600160401b03808516908416615079565b80545f90801561379857613782836137756001846150a6565b5f91825260209091200190565b54600160201b90046001600160401b0316610c5f565b670de0b6b3a76400009392505050565b5f610c5f8383670de0b6b3a7640000613b90565b60605f610c5f83613d7e565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b9092041660608201819052158015906138445750806060015163ffffffff164310155b1561385e57604081015163ffffffff168152600160208201525b63ffffffff821660408201526138947f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9101612cd2565b5f5f61398b61397784613dd7565b85546139869190600f0b6150b9565b613e44565b8454909150600160801b9004600f90810b9082900b126139be57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f610c5f8383613ead565b5f610c5f836001600160a01b038416613ead565b6033546001600160a01b03163314611b6b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401612c59565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f610c5f836001600160a01b038416613f90565b5f610c5f8383613f90565b81545f9081613ad285858385613fdc565b90508015613aff57613ae9856137756001846150a6565b54600160201b90046001600160401b0316611d81565b50670de0b6b3a7640000949350505050565b5f825f018281548110613b2657613b26614b49565b905f5260205f200154905092915050565b5f5f613b44868686613b90565b90506001836002811115613b5a57613b5a6150e0565b148015613b7657505f8480613b7157613b716150f4565b868809115b15611d8157613b86600182615108565b9695505050505050565b5f80805f19858709858702925082811083820303915050805f03613bc757838281613bbd57613bbd6150f4565b0492505050610c5f565b808411613c0e5760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401612c59565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f80613c8285858561402f565b91509150935093915050565b5f613ca88254600f81810b600160801b909204900b131590565b15613cc657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f613cfa8254600f81810b600160801b909204900b131590565b15613d1857604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b6001600160a01b0381165f9081526001830160205260408120541515610c5f565b6060815f01805480602002602001604051908101604052809291908181526020018280548015613dcb57602002820191905f5260205f20905b815481526020019060010190808311613db7575b50505050509050919050565b5f6001600160ff1b03821115613e405760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401612c59565b5090565b80600f81900b8114613ea85760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401612c59565b919050565b5f8181526001830160205260408120548015613f87575f613ecf6001836150a6565b85549091505f90613ee2906001906150a6565b9050818114613f41575f865f018281548110613f0057613f00614b49565b905f5260205f200154905080875f018481548110613f2057613f20614b49565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080613f5257613f5261511b565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506113a0565b5f9150506113a0565b5f818152600183016020526040812054613fd557508154600181810184555f8481526020808220909301849055845484825282860190935260409020919091556113a0565b505f6113a0565b5f5b81831015611367575f613ff184846141f9565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561401b57809250614029565b614026816001615108565b93505b50613fde565b82545f908190801561418d575f61404b876137756001856150a6565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b0316602084015291925090871610156140cc5760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401612c59565b805163ffffffff80881691160361411d57846140ed886137756001866150a6565b80546001600160401b0392909216600160201b026bffffffffffffffff000000001990921691909117905561417d565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d555f8d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b6020015192508391506132689050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a555f8a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081613268565b5f614207600284841861512f565b610c5f90848416615108565b5f60208284031215614223575f5ffd5b81356001600160401b03811115614238575f5ffd5b820160808185031215610c5f575f5ffd5b6001600160a01b0381168114611c1a575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156142995761429961425d565b604052919050565b803563ffffffff81168114613ea8575f5ffd5b5f604082840312156142c4575f5ffd5b604080519081016001600160401b03811182821017156142e6576142e661425d565b60405290508082356142f781614249565b8152614305602084016142a1565b60208201525092915050565b5f5f5f60808486031215614323575f5ffd5b833561432e81614249565b925061433d85602086016142b4565b9150606084013561434d81614249565b809150509250925092565b81516001600160401b03168152602080830151600f0b9082015260408083015163ffffffff1690820152606081016113a0565b5f6020828403121561439b575f5ffd5b5035919050565b5f5f83601f8401126143b2575f5ffd5b5081356001600160401b038111156143c8575f5ffd5b6020830191508360208260051b8501011115611836575f5ffd5b5f5f602083850312156143f3575f5ffd5b82356001600160401b03811115614408575f5ffd5b614414858286016143a2565b90969095509350505050565b5f60208284031215614430575f5ffd5b8135610c5f81614249565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b828110156144945761447e86835161443b565b604095909501946020919091019060010161446b565b5093949350505050565b602081525f610c5f6020830184614459565b5f604082840312156144c0575f5ffd5b610c5f83836142b4565b5f6001600160401b038211156144e2576144e261425d565b5060051b60200190565b5f82601f8301126144fb575f5ffd5b813561450e614509826144ca565b614271565b8082825260208201915060208360051b86010192508583111561452f575f5ffd5b602085015b8381101561455557803561454781614249565b835260209283019201614534565b5095945050505050565b5f5f5f5f60a08587031215614572575f5ffd5b61457c86866142b4565b935060408501356001600160401b03811115614596575f5ffd5b6145a2878288016144ec565b93505060608501356001600160401b038111156145bd575f5ffd5b6145c9878288016144ec565b9250506145d8608086016142a1565b905092959194509250565b5f8151808452602084019350602083015f5b828110156144945781518652602095860195909101906001016145f5565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561466a57603f198786030184526146558583516145e3565b94506020938401939190910190600101614639565b50929695505050505050565b5f5f60408385031215614687575f5ffd5b823561469281614249565b915060208301356146a281614249565b809150509250929050565b5f8151808452602084019350602083015f5b82811015614494576146f886835180516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b60609590950194602091909101906001016146bf565b604081525f6147206040830185614459565b8281036020840152611d8181856146ad565b5f8151808452602084019350602083015f5b828110156144945781516001600160a01b0316865260209586019590910190600101614744565b602081525f610c5f6020830184614732565b5f5f6040838503121561478e575f5ffd5b82356001600160401b038111156147a3575f5ffd5b6147af858286016144ec565b92505060208301356146a281614249565b602080825282518282018190525f918401906040840190835b818110156148005783516001600160401b03168352602093840193909201916001016147d9565b509095945050505050565b5f5f5f5f5f6060868803121561481f575f5ffd5b853561482a81614249565b945060208601356001600160401b03811115614844575f5ffd5b614850888289016143a2565b90955093505060408601356001600160401b0381111561486e575f5ffd5b61487a888289016143a2565b969995985093965092949392505050565b5f5f6040838503121561489c575f5ffd5b82356148a781614249565b915060208301356001600160401b038111156148c1575f5ffd5b6148cd858286016144ec565b9150509250929050565b5f5f604083850312156148e8575f5ffd5b82356148f381614249565b9150614901602084016142a1565b90509250929050565b5f6020828403121561491a575f5ffd5b813560ff81168114610c5f575f5ffd5b5f6020828403121561493a575f5ffd5b610c5f826142a1565b5f60608284031215614953575f5ffd5b50919050565b5f60208284031215614969575f5ffd5b81356001600160401b0381111561497e575f5ffd5b612bbb84828501614943565b5f5f5f6040848603121561499c575f5ffd5b6149a5846142a1565b925060208401356001600160401b038111156149bf575f5ffd5b6149cb868287016143a2565b9497909650939450505050565b5f5f5f608084860312156149ea575f5ffd5b83356001600160401b038111156149ff575f5ffd5b614a0b868287016144ec565b93505061433d85602086016142b4565b602081525f610c5f60208301846146ad565b5f5f5f60608486031215614a3f575f5ffd5b8335614a4a81614249565b925060208401356001600160401b03811115614a64575f5ffd5b614a70868287016144ec565b925050614a7f604085016142a1565b90509250925092565b5f5f60208385031215614a99575f5ffd5b82356001600160401b03811115614aae575f5ffd5b8301601f81018513614abe575f5ffd5b80356001600160401b03811115614ad3575f5ffd5b856020828401011115614ae4575f5ffd5b6020919091019590945092505050565b5f5f60608385031215614b05575f5ffd5b8235614b1081614249565b915061490184602085016142b4565b5f5f60408385031215614b30575f5ffd5b8235614b3b81614249565b946020939093013593505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b6001600160401b0382811682821603908111156113a0576113a0614b5d565b5f81600f0b60016001607f1b03198103614bac57614bac614b5d565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b0319821217156113a0576113a0614b5d565b6001600160a01b038616815260c08101614bfe602083018761443b565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e19843603018112614c47575f5ffd5b8301803591506001600160401b03821115614c60575f5ffd5b602001915036819003821315611836575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b0387168152614cb5602082018761443b565b60c060608201525f614cca60c0830187614732565b8281036080840152614cdc81876145e3565b905082810360a0840152614cf1818587614c74565b9998505050505050505050565b5f60208284031215614d0e575f5ffd5b81518015158114610c5f575f5ffd5b5f8235607e19833603018112614d31575f5ffd5b9190910192915050565b5f5f8335601e19843603018112614d50575f5ffd5b8301803591506001600160401b03821115614d69575f5ffd5b6020019150600581901b3603821315611836575f5ffd5b5f60208284031215614d90575f5ffd5b81356001600160401b0381168114610c5f575f5ffd5b63ffffffff81811683821601908111156113a0576113a0614b5d565b604081525f614dd46040830185614732565b8281036020840152611d818185614732565b5f60208284031215614df6575f5ffd5b81516001600160401b03811115614e0b575f5ffd5b8201601f81018413614e1b575f5ffd5b8051614e29614509826144ca565b8082825260208201915060208360051b850101925086831115614e4a575f5ffd5b602084015b83811015614ee75780516001600160401b03811115614e6c575f5ffd5b8501603f81018913614e7c575f5ffd5b6020810151614e8d614509826144ca565b808282526020820191506020808460051b8601010192508b831115614eb0575f5ffd5b6040840193505b82841015614ed2578351825260209384019390910190614eb7565b86525050602093840193919091019050614e4f565b509695505050505050565b5f60208284031215614f02575f5ffd5b813561ffff81168114610c5f575f5ffd5b604081016113a0828461443b565b8183526020830192505f815f5b848110156144945763ffffffff614f44836142a1565b1686526020958601959190910190600101614f2e565b6001600160a01b03841681526040602082018190525f90611d819083018486614f21565b60608101614f8c828561443b565b6001600160a01b039290921660409190910152919050565b5f8235603e19833603018112614d31575f5ffd5b602081525f612bbb602083018486614c74565b6001600160a01b03861681526060602082018190525f90614fef9083018688614f21565b8281036040840152615002818587614c74565b98975050505050505050565b5f6020828403121561501e575f5ffd5b8151610c5f81614249565b80516020808301519190811015614953575f1960209190910360031b1b16919050565b5f6001820161505d5761505d614b5d565b5060010190565b5f8161507257615072614b5d565b505f190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b03821317156113a0576113a0614b5d565b818103818111156113a0576113a0614b5d565b8082018281125f8312801582168215821617156150d8576150d8614b5d565b505092915050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b808201808211156113a0576113a0614b5d565b634e487b7160e01b5f52603160045260245ffd5b5f8261514957634e487b7160e01b5f52601260045260245ffd5b50049056fea26469706673582212209d78270e88d1507a8a852df378661efd73b7f84e362d47eeb6c9a8da7b9ae33064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@QaS\xD48\x03\x80aS\xD4\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01zV[\x83\x82\x82\x85`\x01`\x01`\xA0\x1B\x03\x81\x16a\0ZW`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x92\x90\x92\x16`\xA0Rc\xFF\xFF\xFF\xFF\x90\x81\x16`\xC0R\x16`\xE0Ra\0\x86a\0\x8FV[PPPPa\x01\xD0V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xFAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01IW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01_W__\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01uW__\xFD[\x91\x90PV[____`\x80\x85\x87\x03\x12\x15a\x01\x8DW__\xFD[\x84Qa\x01\x98\x81a\x01KV[` \x86\x01Q\x90\x94Pa\x01\xA9\x81a\x01KV[\x92Pa\x01\xB7`@\x86\x01a\x01bV[\x91Pa\x01\xC5``\x86\x01a\x01bV[\x90P\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0QaQ\x84a\x02P_9_\x81\x81a\x05\x0B\x01Ra8o\x01R_\x81\x81a\x03\x1B\x01R\x81\x81a\x10\xE8\x01Ra \x1E\x01R_\x81\x81a\x06\x97\x01R\x81\x81a\x0B\x1F\x01R\x81\x81a\x13\xFA\x01R\x81\x81a\x1Av\x01R\x81\x81a\x1B\x82\x01Ra(\"\x01R_\x81\x81a\x05E\x01R\x81\x81a\x0C{\x01R\x81\x81a\x1A\xD3\x01Ra.\xF9\x01RaQ\x84_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02kW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01KW\x80c\xA9\x8F\xB3U\x11a\0\xBFW\x80c\xCDm\xC6\x87\x11a\0\x84W\x80c\xCDm\xC6\x87\x14a\x06lW\x80c\xCE{^K\x14a\x06\x7FW\x80c\xDF\\\xF7#\x14a\x06\x92W\x80c\xF2_\x16\x10\x14a\x06\xB9W\x80c\xF2\xFD\xE3\x8B\x14a\x06\xCCW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xDFW__\xFD[\x80c\xA9\x8F\xB3U\x14a\x05\xF1W\x80c\xB2Dz\xF7\x14a\x06\x04W\x80c\xB9/`\xA5\x14a\x06\x17W\x80c\xB9\xFB\xAE\xD1\x14a\x06*W\x80c\xC2!\xD8\xAE\x14a\x06YW__\xFD[\x80c\x88o\x11\x95\x11a\x01\x10W\x80c\x88o\x11\x95\x14a\x05@W\x80c\x8C\xE6HT\x14a\x05gW\x80c\x8D\xA5\xCB[\x14a\x05\x87W\x80c\x94\xD7\xD0\x0C\x14a\x05\x98W\x80c\xA93>\xC8\x14a\x05\xABW\x80c\xA9\x84\xEB:\x14a\x05\xBEW__\xFD[\x80cqP\x18\xA6\x14a\x04\xD8W\x80cv\x99\x93B\x14a\x04\xE0W\x80cy\xAEP\xCD\x14a\x04\xF3W\x80c{\xC1\xEFa\x14a\x05\x06W\x80c\x84}cO\x14a\x05-W__\xFD[\x80cJ\x10\xFF\xE5\x11a\x01\xE2W\x80cZ\xC8j\xB7\x11a\x01\xA7W\x80cZ\xC8j\xB7\x14a\x04?W\x80c\\H\x9B\xB5\x14a\x04bW\x80c\\\x97Z\xBB\x14a\x04uW\x80cl\xFBD\x81\x14a\x04\x87W\x80cn4\x92\xB5\x14a\x04\xB2W\x80cn\x87]\xBA\x14a\x04\xC5W__\xFD[\x80cJ\x10\xFF\xE5\x14a\x03\xDEW\x80cKPF\xEF\x14a\x03\xFEW\x80cTz\xFB\x87\x14a\x04\x11W\x80cV\xC4\x83\xE6\x14a\x04$W\x80cY\\jg\x14a\x047W__\xFD[\x80c&\r\xC7X\x11a\x023W\x80c&\r\xC7X\x14a\x02\xF3W\x80c)\x81\xEBw\x14a\x03\x16W\x80c+\xAB,J\x14a\x03RW\x80c0L\x10\xCD\x14a\x03rW\x80c@\x12\r\xAB\x14a\x03\x9DW\x80cAw\xA8|\x14a\x03\xBEW__\xFD[\x80c\x0E\xA4>C\x14a\x02oW\x80c\x10\xE1\xB9\xB8\x14a\x02\x84W\x80c\x13d9\xDD\x14a\x02\xADW\x80c\x15\xEAy\x17\x14a\x02\xC0W\x80c\x15\xFEP(\x14a\x02\xD3W[__\xFD[a\x02\x82a\x02}6`\x04aB\x13V[a\x06\xF2V[\0[a\x02\x97a\x02\x926`\x04aC\x11V[a\x0C1V[`@Qa\x02\xA4\x91\x90aCXV[`@Q\x80\x91\x03\x90\xF3[a\x02\x82a\x02\xBB6`\x04aC\x8BV[a\x0CfV[a\x02\x82a\x02\xCE6`\x04aC\xE2V[a\r;V[a\x02\xE6a\x02\xE16`\x04aD V[a\x12XV[`@Qa\x02\xA4\x91\x90aD\x9EV[a\x03\x06a\x03\x016`\x04aD\xB0V[a\x13oV[`@Q\x90\x15\x15\x81R` \x01a\x02\xA4V[a\x03=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x03ea\x03`6`\x04aE_V[a\x13\xA6V[`@Qa\x02\xA4\x91\x90aF\x13V[a\x03\x85a\x03\x806`\x04aD V[a\x16\x93V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x03\xB0a\x03\xAB6`\x04aFvV[a\x16\xC2V[`@Qa\x02\xA4\x92\x91\x90aG\x0EV[a\x03\xD1a\x03\xCC6`\x04aD\xB0V[a\x18=V[`@Qa\x02\xA4\x91\x90aGkV[a\x03\xF1a\x03\xEC6`\x04aG}V[a\x18aV[`@Qa\x02\xA4\x91\x90aG\xC0V[a\x02\x82a\x04\x0C6`\x04aH\x0BV[a\x19\tV[a\x03\xF1a\x04\x1F6`\x04aH\x8BV[a\x19\xC3V[a\x02\x82a\x0426`\x04aH\xD7V[a\x1AkV[a\x02\x82a\x1A\xBEV[a\x03\x06a\x04M6`\x04aI\nV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x02\x82a\x04p6`\x04aI*V[a\x1BmV[`fT[`@Q\x90\x81R` \x01a\x02\xA4V[a\x04\x9Aa\x04\x956`\x04aFvV[a\x1C\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x02\x82a\x04\xC06`\x04aIYV[a\x1D\x8AV[a\x03\xD1a\x04\xD36`\x04aD\xB0V[a!ZV[a\x02\x82a!kV[a\x02\x82a\x04\xEE6`\x04aI\x8AV[a!|V[a\x02\xE6a\x05\x016`\x04aD V[a\"\xA6V[a\x03=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x82a\x05;6`\x04aC\xE2V[a#\x80V[a\x03\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05za\x05u6`\x04aI\xD8V[a%\xADV[`@Qa\x02\xA4\x91\x90aJ\x1BV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x85V[a\x03\xF1a\x05\xA66`\x04aJ-V[a&rV[a\x04\x9Aa\x05\xB96`\x04aFvV[a'^V[a\x04\x9Aa\x05\xCC6`\x04aFvV[`\xA2` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\x82a\x05\xFF6`\x04aJ\x88V[a'\x8DV[a\x04ya\x06\x126`\x04aD\xB0V[a'\xD4V[a\x02\x82a\x06%6`\x04aIYV[a'\xE4V[a\x06=a\x0686`\x04aD V[a*\xF8V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xA4V[a\x03\xD1a\x06g6`\x04aJ\xF4V[a+\x92V[a\x02\x82a\x06z6`\x04aK\x1FV[a+\xC3V[a\x02\x82a\x06\x8D6`\x04aI\x8AV[a,\xDFV[a\x03\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x82a\x06\xC76`\x04aD V[a.\tV[a\x02\x82a\x06\xDA6`\x04aD V[a.\x81V[a\x02\x82a\x06\xED6`\x04aC\x8BV[a.\xF7V[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x07\x1BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x015_\x10\x80\x15a\x07:WPg\r\xE0\xB6\xB3\xA7d\0\0`@\x83\x015\x11\x15[a\x07WW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x07\x85\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x07\xA6a\x07\xA0` \x86\x01\x86aD V[\x83a0\x06V[` \x80\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x91\x92Pa\x07\xDC\x91\x90c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a0{\x16V[a\x07\xF9W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x08\x17W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x089`\x99_a\x08'\x86a0\x92V[\x81R` \x01\x90\x81R` \x01_ a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08TWa\x08TaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x99Wa\x08\x99aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xC2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0B\xCFW_a\x08\xFB\x82`\x99_a\x08\xE0\x8Ba0\x92V[\x81R` \x01\x90\x81R` \x01_ a0\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80a\t\x1Ea\t\x0F` \x8D\x01\x8DaD V[a\t\x18\x8Ba0\x92V[\x85a1\x04V[\x91P\x91P\x82\x86\x85\x81Q\x81\x10a\t5Wa\t5aKIV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\tfWPPPa\x0B\xC7V[\x80Q_\x90a\t\x81\x90`\x01`\x01`@\x1B\x03\x16`@\x8E\x015a2pV[\x83Q\x90\x91P_\x90a\t\x9E\x90`\x01`\x01`@\x1B\x03\x80\x85\x16\x91\x16a2\x86V[\x90P\x80\x87\x87\x81Q\x81\x10a\t\xB3Wa\t\xB3aKIV[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\t\xD0\x91\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\t\xED\x90\x83\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\n\r\x90\x83\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\n\xE3W_a\nQ\x8E`@\x015\x85` \x01Qa\nB\x90aK\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90a2pV[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\nn\x91\x90aK\xB4V[\x91P\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x8E_\x01` \x81\x01\x90a\n\xB0\x91\x90aD V[\x8D\x88a\n\xC3\x88_\x01Q\x89` \x01Qa2\x9AV[\x88`@\x01Q`@Qa\n\xD9\x95\x94\x93\x92\x91\x90aK\xE1V[`@Q\x80\x91\x03\x90\xA1P[a\x0B\x04a\n\xF3` \x8F\x01\x8FaD V[a\n\xFC\x8Da0\x92V[\x87\x87\x87a2\xAEV[a\x0B\x1Da\x0B\x14` \x8F\x01\x8FaD V[\x85Q\x87\x90a5\x1AV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB6\xF7;\xDF\x8E_\x01` \x81\x01\x90a\x0B^\x91\x90aD V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x88\x16`$\x82\x01R`D\x81\x01\x84\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xABW__\xFD[PZ\xF1\x15\x80\x15a\x0B\xBDW=__>=_\xFD[PPPPPPPPP[`\x01\x01a\x08\xC7V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x0B\xFE` \x89\x01\x89aD V[\x86\x84\x84a\x0C\x0E``\x8D\x01\x8DaL2V[`@Qa\x0C \x96\x95\x94\x93\x92\x91\x90aL\x9CV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x0CZ\x85a\t\x18\x86a0\x92V[\x92PPP[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90aL\xFEV[a\r\tW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\r.W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r7\x82a5\x9EV[PPV[`fT_\x90`\x01\x90\x81\x16\x03a\rcW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\rn3a*\xF8V[\x91P\x91P\x81a\r\x90W`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x12PW\x85\x85\x82\x81\x81\x10a\r\xACWa\r\xACaKIV[\x90P` \x02\x81\x01\x90a\r\xBE\x91\x90aM\x1DV[a\r\xCC\x90``\x81\x01\x90aM;V[\x90P\x86\x86\x83\x81\x81\x10a\r\xE0Wa\r\xE0aKIV[\x90P` \x02\x81\x01\x90a\r\xF2\x91\x90aM\x1DV[a\x0E\0\x90`@\x81\x01\x90aM;V[\x90P\x14a\x0E W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x86\x86\x83\x81\x81\x10a\x0E3Wa\x0E3aKIV[\x90P` \x02\x81\x01\x90a\x0EE\x91\x90aM\x1DV[\x90P_a\x0E`3a\x0E[6\x85\x90\x03\x85\x01\x85aD\xB0V[a0\x06V[\x90Pa\x0E\xA9a\x0Eu`@\x84\x01` \x85\x01aI*V[c\xFF\xFF\xFF\xFF\x16`\x98_a\x0E\x8B` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a0{V[a\x0E\xC6W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x88\x88\x85\x81\x81\x10a\x0E\xDAWa\x0E\xDAaKIV[\x90P` \x02\x81\x01\x90a\x0E\xEC\x91\x90aM\x1DV[a\x0E\xFA\x90`@\x81\x01\x90aM;V[\x90P\x81\x10\x15a\x12EW_\x89\x89\x86\x81\x81\x10a\x0F\x16Wa\x0F\x16aKIV[\x90P` \x02\x81\x01\x90a\x0F(\x91\x90aM\x1DV[a\x0F6\x90`@\x81\x01\x90aM;V[\x83\x81\x81\x10a\x0FFWa\x0FFaKIV[\x90P` \x02\x01` \x81\x01\x90a\x0F[\x91\x90aD V[\x90Pa\x0Fj3\x82a\xFF\xFFa5\xDBV[_\x80a\x0F\x873a\t\x18a\x0F\x826\x8A\x90\x03\x8A\x01\x8AaD\xB0V[a0\x92V[\x91P\x91P\x80` \x01Q`\x0F\x0B_\x14a\x0F\xB2W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\xCDa\x0F\xC56\x89\x90\x03\x89\x01\x89aD\xB0V[\x85\x84\x89a6\xDFV[\x90Pa\x102\x82_\x01Q\x8E\x8E\x8B\x81\x81\x10a\x0F\xE8Wa\x0F\xE8aKIV[\x90P` \x02\x81\x01\x90a\x0F\xFA\x91\x90aM\x1DV[a\x10\x08\x90``\x81\x01\x90aM;V[\x88\x81\x81\x10a\x10\x18Wa\x10\x18aKIV[\x90P` \x02\x01` \x81\x01\x90a\x10-\x91\x90aM\x80V[a7EV[`\x0F\x0B` \x83\x01\x81\x90R_\x03a\x10[W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a\x11\xAAW\x80\x15a\x11\x1DWa\x10\xE3a\x10\x85a\x0F\x826\x8A\x90\x03\x8A\x01\x8AaD\xB0V[3_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x84R\x90\x91R\x90 \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x11\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra\x12\x17V[a\x11/\x83` \x01Q\x83` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8C\x8C\x89\x81\x81\x10a\x11OWa\x11OaKIV[\x90P` \x02\x81\x01\x90a\x11a\x91\x90aM\x1DV[a\x11o\x90``\x81\x01\x90aM;V[\x86\x81\x81\x10a\x11\x7FWa\x11\x7FaKIV[\x90P` \x02\x01` \x81\x01\x90a\x11\x94\x91\x90aM\x80V[`\x01`\x01`@\x1B\x03\x16\x82R_` \x83\x01Ra\x12\x17V[_\x82` \x01Q`\x0F\x0B\x13\x15a\x12\x17Wa\x11\xCB\x83` \x01Q\x83` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a\x12\x01W`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\x0B\x89CaM\xA6V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a\x1253a\x12-a\x0F\x826\x8B\x90\x03\x8B\x01\x8BaD\xB0V[\x86\x86\x86a2\xAEV[PP`\x01\x90\x92\x01\x91Pa\x0E\xC8\x90PV[PPP`\x01\x01a\r\x92V[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\x12|\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x97Wa\x12\x97aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xDBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xB5W\x90P[P\x90P_[\x82\x81\x10\x15a\x13gW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\x13B\x90a\x13\r\x90\x83a0\xF9V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\x13TWa\x13TaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x12\xE0V[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\x13\xA0\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a0{\x16V[\x92\x91PPV[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xC1Wa\x13\xC1aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xF4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\xDFW\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14F\x92\x91\x90aM\xC2V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14`W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\x87\x91\x90\x81\x01\x90aM\xE6V[\x90P_[\x85Q\x81\x10\x15a\x16\x89W_\x86\x82\x81Q\x81\x10a\x14\xA7Wa\x14\xA7aKIV[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xCAWa\x14\xCAaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a\x15\x06Wa\x15\x06aKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a\x16\x7FW_\x87\x82\x81Q\x81\x10a\x15/Wa\x15/aKIV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa\x15j\x90a7\\V[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x15\x83WPPa\x16wV[_a\x15\x8F\x85\x8D\x85a\x0C1V[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a\x15\xB7WP_\x81` \x01Q`\x0F\x0B\x12[\x15a\x15\xD9Wa\x15\xCD\x81_\x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a\x15\xF4\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a2\x86V[\x90Pa\x16;\x81\x89\x89\x81Q\x81\x10a\x16\x0CWa\x16\x0CaKIV[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x16%Wa\x16%aKIV[` \x02` \x01\x01Qa7\xA8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a\x16MWa\x16MaKIV[` \x02` \x01\x01Q\x86\x81Q\x81\x10a\x16fWa\x16faKIV[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a\x15\x13V[PP`\x01\x01a\x14\x8BV[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\x16\xBBW\x80a\x0C_V[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x16\xE7\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x02Wa\x17\x02aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17FW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17 W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17bWa\x17baB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xABW\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x17\x80W\x90P[P\x90P_[\x83\x81\x10\x15a\x18.W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x17\xDD\x90a\x13\r\x90\x84a0\xF9V[\x90P\x80\x84\x83\x81Q\x81\x10a\x17\xF2Wa\x17\xF2aKIV[` \x02` \x01\x01\x81\x90RPa\x18\x08\x89\x82\x8Aa\x0C1V[\x83\x83\x81Q\x81\x10a\x18\x1AWa\x18\x1AaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\xB0V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x0C_`\x99_a\x18O\x86a0\x92V[\x81R` \x01\x90\x81R` \x01_ a7\xBCV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18}Wa\x18}aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\x13gWa\x18\xD7\x85\x82\x81Q\x81\x10a\x18\xC9Wa\x18\xC9aKIV[` \x02` \x01\x01Q\x85a'^V[\x82\x82\x81Q\x81\x10a\x18\xE9Wa\x18\xE9aKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x18\xABV[`fT_\x90`\x01\x90\x81\x16\x03a\x191W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x19QW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x19\xBAWa\x19\xB2\x87\x87\x87\x84\x81\x81\x10a\x19qWa\x19qaKIV[\x90P` \x02\x01` \x81\x01\x90a\x19\x86\x91\x90aD V[\x86\x86\x85\x81\x81\x10a\x19\x98Wa\x19\x98aKIV[\x90P` \x02\x01` \x81\x01\x90a\x19\xAD\x91\x90aN\xF2V[a5\xDBV[`\x01\x01a\x19SV[PPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDFWa\x19\xDFaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x13gWa\x1A9\x85\x85\x83\x81Q\x81\x10a\x1A,Wa\x1A,aKIV[` \x02` \x01\x01Qa'^V[\x82\x82\x81Q\x81\x10a\x1AKWa\x1AKaKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1A\rV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1A\xB4W`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r7\x82\x82a7\xC8V[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BD\x91\x90aL\xFEV[a\x1BaW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Bk_\x19a5\x9EV[V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF3\x91\x90aL\xFEV[a\x1C\x10W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x1A3\x82a7\xC8V[PV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1DGW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1C\xAF\x90\x83a9iV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a\x1D*WPPa\x1DGV[a\x1D8\x85\x82` \x01Qa2\x9AV[\x94PPP\x80`\x01\x01\x90Pa\x1CwV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1Dw\x90a7\\V[a\x1D\x81\x91\x90aKqV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1D\xB3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xC0` \x83\x01\x83aD V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x1D\xFFWPa\x1D\xEA`@\x83\x01` \x84\x01aD V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x1E\x1CW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\x1E+`@\x84\x01\x84aM;V[\x90P\x81\x10\x15a \xCFW_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a\x1ES\x91\x90aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1En`@\x87\x01\x87aM;V[\x85\x81\x81\x10a\x1E~Wa\x1E~aKIV[\x90P` \x02\x01` \x81\x01\x90a\x1E\x93\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x1E\xC2\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a\x0E\x8B\x91\x90aD V[a\x1E\xDFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a\x1E\xEF` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1F\x18\x83a0\x92V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x1FHW`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x82a\x1FT\x82a0\x92V[`\x9C_a\x1Fd` \x89\x01\x89aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a9\xD8V[Pa\x1F\xBAa\x1F\x93` \x86\x01\x86aD V[`\x9A_a\x1F\x9F\x85a0\x92V[\x81R` \x01\x90\x81R` \x01_ a9\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\x1F\xC8` \x85\x01\x85aD V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa \0\x91\x90aO\x13V[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a [` \x88\x01\x88aD V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a \x84\x84a0\x92V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a\x1E\x1EV[Pa \xE3a\x03\x80`@\x84\x01` \x85\x01aD V[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\x8E\x0C#a \xFE` \x85\x01\x85aD V[a!\x0B`@\x86\x01\x86aM;V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!)\x93\x92\x91\x90aOZV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!@W__\xFD[PZ\xF1\x92PPP\x80\x15a!QWP`\x01[\x15a\r7WPPV[``a\x13\xA0`\x9A_a\x18O\x85a0\x92V[a!sa9\xF7V[a\x1Bk_a:QV[`@\x80Q\x80\x82\x01\x82R3\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x87\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a!\xB2\x92\x91a0{\x16V[a!\xCFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a!\xD9\x82a0\x92V[\x90P_[\x83\x81\x10\x15a\x12PWa\"\"\x85\x85\x83\x81\x81\x10a!\xFAWa!\xFAaKIV[\x90P` \x02\x01` \x81\x01\x90a\"\x0F\x91\x90aD V[_\x84\x81R`\x99` R`@\x90 \x90a:\xA2V[a\"?W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x86\x86\x84\x81\x81\x10a\"sWa\"saKIV[\x90P` \x02\x01` \x81\x01\x90a\"\x88\x91\x90aD V[`@Qa\"\x96\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a!\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a\"\xCA\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE5Wa\"\xE5aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#)W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\x03W\x90P[P\x90P_[\x82\x81\x10\x15a\x13gW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a#[\x90a\x13\r\x90\x83a0\xF9V[\x82\x82\x81Q\x81\x10a#mWa#maKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#.V[_[\x81\x81\x10\x15a%\xA8W_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a#\xB7Wa#\xB7aKIV[\x90P` \x02\x81\x01\x90a#\xC9\x91\x90aO\xA4V[a#\xD7\x90` \x81\x01\x90aI*V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q3_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa$\t\x92\x91\x90\x81\x16\x90a:\xB6\x16V[a$&W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R3\x81R` \x83\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90Q\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l\x91a$o\x91aO\x13V[`@Q\x80\x91\x03\x90\xA1_a$\x81\x82a0\x92V[\x90P_[\x85\x85\x85\x81\x81\x10a$\x97Wa$\x97aKIV[\x90P` \x02\x81\x01\x90a$\xA9\x91\x90aO\xA4V[a$\xB7\x90` \x81\x01\x90aM;V[\x90P\x81\x10\x15a%\x9DWa%\x05\x86\x86\x86\x81\x81\x10a$\xD5Wa$\xD5aKIV[\x90P` \x02\x81\x01\x90a$\xE7\x91\x90aO\xA4V[a$\xF5\x90` \x81\x01\x90aM;V[\x83\x81\x81\x10a!\xFAWa!\xFAaKIV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x87\x81\x81\x10a%:Wa%:aKIV[\x90P` \x02\x81\x01\x90a%L\x91\x90aO\xA4V[a%Z\x90` \x81\x01\x90aM;V[\x84\x81\x81\x10a%jWa%jaKIV[\x90P` \x02\x01` \x81\x01\x90a%\x7F\x91\x90aD V[`@Qa%\x8D\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a$\x85V[PPP`\x01\x01a#\x82V[PPPV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xC9Wa%\xC9aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\x12W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a%\xE7W\x90P[P\x90P_[\x85Q\x81\x10\x15a&iWa&D\x86\x82\x81Q\x81\x10a&5Wa&5aKIV[` \x02` \x01\x01Q\x86\x86a\x0C1V[\x82\x82\x81Q\x81\x10a&VWa&VaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a&\x17V[P\x94\x93PPPPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\x8EWa&\x8EaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a&iW`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa',\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a&\xF7Wa&\xF7aKIV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a:\xC1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a'>Wa'>aKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a&\xBCV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x0C_\x90a7\\V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa'\xC8\x92\x91\x90aO\xB8V[`@Q\x80\x91\x03\x90\xA2PPV[_a\x13\xA0`\x9A_a\x08'\x85a0\x92V[`fT`\x02\x90`\x04\x90\x81\x16\x03a(\rW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x93\x91\x90aL\xFEV[a(\xB0W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a(\xBF` \x84\x01\x84aM;V[\x90P\x81\x10\x15a*uW`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a(\xE2` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x80` \x01\x90a)\0\x91\x90aM;V[\x85\x81\x81\x10a)\x10Wa)\x10aKIV[\x90P` \x02\x01` \x81\x01\x90a)%\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa)a\x92\x91\x90\x81\x16\x90a0{\x16V[a)~W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\x883\x82a0\x06V[\x15a)\xA6W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\xC6a)\xB2\x82a0\x92V[3_\x90\x81R`\x9C` R`@\x90 \x90a:\xB6V[Pa)\xF23`\x9A_a)\xD7\x85a0\x92V[\x81R` \x01\x90\x81R` \x01_ a:\xA2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa*,\x91\x90aO\x13V[`@Q\x80\x91\x03\x90\xA23_\x90\x81R`\x9E` R`@\x81 `\x01\x91a*N\x84a0\x92V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a(\xB2V[Pa*\x86a\x03\x80` \x84\x01\x84aD V[`\x01`\x01`\xA0\x1B\x03\x16c\xAD\xCFs\xF73a*\xA2` \x86\x01\x86aM;V[a*\xAF`@\x88\x01\x88aL2V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xCF\x95\x94\x93\x92\x91\x90aO\xCBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a*\xE6W__\xFD[PZ\xF1\x15\x80\x15a\x12PW=__>=_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90Re\x01\0\0\0\0\0\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a+yWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a+\x88WPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a+\xBB\x90\x82a\x18O\x86a0\x92V[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a+\xE1WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a+\xFAWP0;\x15\x80\x15a+\xFAWP_T`\xFF\x16`\x01\x14[a,bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a,\x83W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a,\x8C\x82a5\x9EV[a,\x95\x83a:QV[\x80\x15a%\xA8W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x82R3\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x87\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a-\x15\x92\x91a0{\x16V[a-2W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a-<\x82a0\x92V[\x90P_[\x83\x81\x10\x15a\x12PWa-\x85\x85\x85\x83\x81\x81\x10a-]Wa-]aKIV[\x90P` \x02\x01` \x81\x01\x90a-r\x91\x90aD V[_\x84\x81R`\x99` R`@\x90 \x90a9\xE3V[a-\xA2W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x86\x86\x84\x81\x81\x10a-\xD6Wa-\xD6aKIV[\x90P` \x02\x01` \x81\x01\x90a-\xEB\x91\x90aD V[`@Qa-\xF9\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a-@V[3_\x81\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x90a.[\x81a\x16\x93V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PV[a.\x89a9\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a.\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a,YV[a\x1C\x1A\x81a:QV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/w\x91\x90aP\x0EV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a/\xA8W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a/\xCFW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a'\xC8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a0)\x85a0\x92V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a+\xBBWP` \x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x93\x92PPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0C_V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a0\xD8\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x13\xA0\x90aP)V[_a\x13\xA0\x82T\x90V[_a\x0C_\x83\x83a;\x11V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q``\x81\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x80\x85\x01\x83\x90R\x84Q\x80\x86\x01\x86R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x85R`\xA1\x84R\x86\x85 \x90\x88\x16\x85R\x90\x92R\x93\x82 \x92\x93\x92\x81\x90a1d\x90a7\\V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a2\x06W\x90\x92P\x90Pa2hV[a2\x17\x81_\x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a2UWa2F\x82` \x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[_a\x0C_\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01a;7V[_a\x0C_\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a;\x90V[_a\x0C_\x82`\x01`\x01`@\x1B\x03\x85\x16aK\xB4V[` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x81\x81R`\xA2\x86R`@\x80\x82 \x93\x8A\x16\x80\x83R\x93\x87R\x90\x81\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x95\x86\x16\x17\x90U\x93Q\x84Q\x91\x82R\x94\x81\x01\x91\x90\x91R\x92\x16\x90\x82\x01R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x90``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x84Q\x81T\x86\x84\x01Q\x87\x87\x01Q`\x01`\x01`@\x1B\x03\x90\x93\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x02\x17c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x90\x91U\x83Q\x80\x85\x01\x85R\x83\x81R\x82\x01\x92\x90\x92R\x82Q\x80\x84\x01\x90\x93R``\x87\x90\x1C\x83R\x90\x86\x16\x90\x82\x01R\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x90\x86\x90\x83Q`@Qa4\x1A\x93\x92\x91\x88\x91C\x90aK\xE1V[`@Q\x80\x91\x03\x90\xA1` \x81\x01Q`\x0F\x0B\x15a4\x83W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4[\x90\x84a:\xA2V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a4}\x90\x85a:\xB6V[Pa5\x13V[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a5\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4\xC0\x90\x84a9\xE3V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4\xEC\x90a0\xF0V[_\x03a5\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\x12P\x90\x85a9\xD8V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a5I\x90C\x83a<uV[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90``\x01a,\xD2V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a6%WP\x82a\xFF\xFF\x16\x82\x10[\x15a5\x13W`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a6Y\x90a<\x8EV[\x90P__a6h3\x84\x89a1\x04V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a6\x86WPPPa5\x13V[a6\x93\x88\x84\x89\x85\x85a2\xAEV[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a6\xC0\x90a<\xE0V[Pa6\xCA\x85aPLV[\x94Pa6\xD5\x84aPdV[\x93PPPPa6\x13V[_a7\x0B\x84`\x99_a6\xF0\x89a0\x92V[\x81R` \x01\x90\x81R` \x01_ a=]\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a7\x16WP_a+\xBBV[\x81a7\"WP_a+\xBBV[\x82Q`\x01`\x01`@\x1B\x03\x16_\x03a7:WP_a+\xBBV[P`\x01\x94\x93PPPPV[_a\x0C_`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aPyV[\x80T_\x90\x80\x15a7\x98Wa7\x82\x83a7u`\x01\x84aP\xA6V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0C_V[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[_a\x0C_\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a;\x90V[``_a\x0C_\x83a=~V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a8DWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a8^W`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra8\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a,\xD2V[__a9\x8Ba9w\x84a=\xD7V[\x85Ta9\x86\x91\x90`\x0F\x0BaP\xB9V[a>DV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a9\xBEW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_a\x0C_\x83\x83a>\xADV[_a\x0C_\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a>\xADV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a,YV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x0C_\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a?\x90V[_a\x0C_\x83\x83a?\x90V[\x81T_\x90\x81a:\xD2\x85\x85\x83\x85a?\xDCV[\x90P\x80\x15a:\xFFWa:\xE9\x85a7u`\x01\x84aP\xA6V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D\x81V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[_\x82_\x01\x82\x81T\x81\x10a;&Wa;&aKIV[\x90_R` _ \x01T\x90P\x92\x91PPV[__a;D\x86\x86\x86a;\x90V[\x90P`\x01\x83`\x02\x81\x11\x15a;ZWa;ZaP\xE0V[\x14\x80\x15a;vWP_\x84\x80a;qWa;qaP\xF4V[\x86\x88\t\x11[\x15a\x1D\x81Wa;\x86`\x01\x82aQ\x08V[\x96\x95PPPPPPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a;\xC7W\x83\x82\x81a;\xBDWa;\xBDaP\xF4V[\x04\x92PPPa\x0C_V[\x80\x84\x11a<\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a,YV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_\x80a<\x82\x85\x85\x85a@/V[\x91P\x91P\x93P\x93\x91PPV[_a<\xA8\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a<\xC6W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_a<\xFA\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a=\x18W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0C_V[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a=\xCBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a=\xB7W[PPPPP\x90P\x91\x90PV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a>@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a,YV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a>\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a,YV[\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a?\x87W_a>\xCF`\x01\x83aP\xA6V[\x85T\x90\x91P_\x90a>\xE2\x90`\x01\x90aP\xA6V[\x90P\x81\x81\x14a?AW_\x86_\x01\x82\x81T\x81\x10a?\0Wa?\0aKIV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a? Wa? aKIV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a?RWa?RaQ\x1BV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x13\xA0V[_\x91PPa\x13\xA0V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta?\xD5WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x13\xA0V[P_a\x13\xA0V[_[\x81\x83\x10\x15a\x13gW_a?\xF1\x84\x84aA\xF9V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a@\x1BW\x80\x92Pa@)V[a@&\x81`\x01aQ\x08V[\x93P[Pa?\xDEV[\x82T_\x90\x81\x90\x80\x15aA\x8DW_a@K\x87a7u`\x01\x85aP\xA6V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a@\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a,YV[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03aA\x1DW\x84a@\xED\x88a7u`\x01\x86aP\xA6V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UaA}V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa2h\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a2hV[_aB\x07`\x02\x84\x84\x18aQ/V[a\x0C_\x90\x84\x84\x16aQ\x08V[_` \x82\x84\x03\x12\x15aB#W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB8W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x0C_W__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\x1AW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\x99WaB\x99aB]V[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a>\xA8W__\xFD[_`@\x82\x84\x03\x12\x15aB\xC4W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xE6WaB\xE6aB]V[`@R\x90P\x80\x825aB\xF7\x81aBIV[\x81RaC\x05` \x84\x01aB\xA1V[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aC#W__\xFD[\x835aC.\x81aBIV[\x92PaC=\x85` \x86\x01aB\xB4V[\x91P``\x84\x015aCM\x81aBIV[\x80\x91PP\x92P\x92P\x92V[\x81Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\x13\xA0V[_` \x82\x84\x03\x12\x15aC\x9BW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12aC\xB2W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xC8W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x186W__\xFD[__` \x83\x85\x03\x12\x15aC\xF3W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x08W__\xFD[aD\x14\x85\x82\x86\x01aC\xA2V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aD0W__\xFD[\x815a\x0C_\x81aBIV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94WaD~\x86\x83QaD;V[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aDkV[P\x93\x94\x93PPPPV[` \x81R_a\x0C_` \x83\x01\x84aDYV[_`@\x82\x84\x03\x12\x15aD\xC0W__\xFD[a\x0C_\x83\x83aB\xB4V[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xE2WaD\xE2aB]V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aD\xFBW__\xFD[\x815aE\x0EaE\t\x82aD\xCAV[aBqV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aE/W__\xFD[` \x85\x01[\x83\x81\x10\x15aEUW\x805aEG\x81aBIV[\x83R` \x92\x83\x01\x92\x01aE4V[P\x95\x94PPPPPV[____`\xA0\x85\x87\x03\x12\x15aErW__\xFD[aE|\x86\x86aB\xB4V[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x96W__\xFD[aE\xA2\x87\x82\x88\x01aD\xECV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xBDW__\xFD[aE\xC9\x87\x82\x88\x01aD\xECV[\x92PPaE\xD8`\x80\x86\x01aB\xA1V[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aE\xF5V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aFjW`?\x19\x87\x86\x03\x01\x84RaFU\x85\x83QaE\xE3V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF9V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aF\x87W__\xFD[\x825aF\x92\x81aBIV[\x91P` \x83\x015aF\xA2\x81aBIV[\x80\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94WaF\xF8\x86\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aF\xBFV[`@\x81R_aG `@\x83\x01\x85aDYV[\x82\x81\x03` \x84\x01Ra\x1D\x81\x81\x85aF\xADV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aGDV[` \x81R_a\x0C_` \x83\x01\x84aG2V[__`@\x83\x85\x03\x12\x15aG\x8EW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xA3W__\xFD[aG\xAF\x85\x82\x86\x01aD\xECV[\x92PP` \x83\x015aF\xA2\x81aBIV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aH\0W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aG\xD9V[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aH\x1FW__\xFD[\x855aH*\x81aBIV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHDW__\xFD[aHP\x88\x82\x89\x01aC\xA2V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHnW__\xFD[aHz\x88\x82\x89\x01aC\xA2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aH\x9CW__\xFD[\x825aH\xA7\x81aBIV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xC1W__\xFD[aH\xCD\x85\x82\x86\x01aD\xECV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aH\xE8W__\xFD[\x825aH\xF3\x81aBIV[\x91PaI\x01` \x84\x01aB\xA1V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aI\x1AW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x0C_W__\xFD[_` \x82\x84\x03\x12\x15aI:W__\xFD[a\x0C_\x82aB\xA1V[_``\x82\x84\x03\x12\x15aISW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aIiW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI~W__\xFD[a+\xBB\x84\x82\x85\x01aICV[___`@\x84\x86\x03\x12\x15aI\x9CW__\xFD[aI\xA5\x84aB\xA1V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xBFW__\xFD[aI\xCB\x86\x82\x87\x01aC\xA2V[\x94\x97\x90\x96P\x93\x94PPPPV[___`\x80\x84\x86\x03\x12\x15aI\xEAW__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xFFW__\xFD[aJ\x0B\x86\x82\x87\x01aD\xECV[\x93PPaC=\x85` \x86\x01aB\xB4V[` \x81R_a\x0C_` \x83\x01\x84aF\xADV[___``\x84\x86\x03\x12\x15aJ?W__\xFD[\x835aJJ\x81aBIV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJdW__\xFD[aJp\x86\x82\x87\x01aD\xECV[\x92PPaJ\x7F`@\x85\x01aB\xA1V[\x90P\x92P\x92P\x92V[__` \x83\x85\x03\x12\x15aJ\x99W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAEW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aJ\xBEW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD3W__\xFD[\x85` \x82\x84\x01\x01\x11\x15aJ\xE4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__``\x83\x85\x03\x12\x15aK\x05W__\xFD[\x825aK\x10\x81aBIV[\x91PaI\x01\x84` \x85\x01aB\xB4V[__`@\x83\x85\x03\x12\x15aK0W__\xFD[\x825aK;\x81aBIV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aK\xACWaK\xACaK]V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x13\xA0Wa\x13\xA0aK]V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aK\xFE` \x83\x01\x87aD;V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aLGW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL`W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x186W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81RaL\xB5` \x82\x01\x87aD;V[`\xC0``\x82\x01R_aL\xCA`\xC0\x83\x01\x87aG2V[\x82\x81\x03`\x80\x84\x01RaL\xDC\x81\x87aE\xE3V[\x90P\x82\x81\x03`\xA0\x84\x01RaL\xF1\x81\x85\x87aLtV[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aM\x0EW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C_W__\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aM1W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aMPW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aMiW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x186W__\xFD[_` \x82\x84\x03\x12\x15aM\x90W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0C_W__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[`@\x81R_aM\xD4`@\x83\x01\x85aG2V[\x82\x81\x03` \x84\x01Ra\x1D\x81\x81\x85aG2V[_` \x82\x84\x03\x12\x15aM\xF6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x0BW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN\x1BW__\xFD[\x80QaN)aE\t\x82aD\xCAV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aNJW__\xFD[` \x84\x01[\x83\x81\x10\x15aN\xE7W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aNlW__\xFD[\x85\x01`?\x81\x01\x89\x13aN|W__\xFD[` \x81\x01QaN\x8DaE\t\x82aD\xCAV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aN\xB0W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aN\xD2W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aN\xB7V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaNOV[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aO\x02W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0C_W__\xFD[`@\x81\x01a\x13\xA0\x82\x84aD;V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aD\x94Wc\xFF\xFF\xFF\xFFaOD\x83aB\xA1V[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aO.V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x1D\x81\x90\x83\x01\x84\x86aO!V[``\x81\x01aO\x8C\x82\x85aD;V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[_\x825`>\x19\x836\x03\x01\x81\x12aM1W__\xFD[` \x81R_a+\xBB` \x83\x01\x84\x86aLtV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R``` \x82\x01\x81\x90R_\x90aO\xEF\x90\x83\x01\x86\x88aO!V[\x82\x81\x03`@\x84\x01RaP\x02\x81\x85\x87aLtV[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15aP\x1EW__\xFD[\x81Qa\x0C_\x81aBIV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aISW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01aP]WaP]aK]V[P`\x01\x01\x90V[_\x81aPrWaPraK]V[P_\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x13\xA0Wa\x13\xA0aK]V[\x81\x81\x03\x81\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aP\xD8WaP\xD8aK]V[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x13\xA0Wa\x13\xA0aK]V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82aQIWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9Dx'\x0E\x88\xD1Pz\x8A\x85-\xF3xf\x1E\xFDs\xB7\xF8N6-G\xEE\xB6\xC9\xA8\xDA{\x9A\xE30dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061026b575f3560e01c8063715018a61161014b578063a98fb355116100bf578063cd6dc68711610084578063cd6dc6871461066c578063ce7b5e4b1461067f578063df5cf72314610692578063f25f1610146106b9578063f2fde38b146106cc578063fabc1cbc146106df575f5ffd5b8063a98fb355146105f1578063b2447af714610604578063b92f60a514610617578063b9fbaed11461062a578063c221d8ae14610659575f5ffd5b8063886f119511610110578063886f1195146105405780638ce64854146105675780638da5cb5b1461058757806394d7d00c14610598578063a9333ec8146105ab578063a984eb3a146105be575f5ffd5b8063715018a6146104d857806376999342146104e057806379ae50cd146104f35780637bc1ef6114610506578063847d634f1461052d575f5ffd5b80634a10ffe5116101e25780635ac86ab7116101a75780635ac86ab71461043f5780635c489bb5146104625780635c975abb146104755780636cfb4481146104875780636e3492b5146104b25780636e875dba146104c5575f5ffd5b80634a10ffe5146103de5780634b5046ef146103fe578063547afb871461041157806356c483e614610424578063595c6a6714610437575f5ffd5b8063260dc75811610233578063260dc758146102f35780632981eb77146103165780632bab2c4a14610352578063304c10cd1461037257806340120dab1461039d5780634177a87c146103be575f5ffd5b80630ea43e431461026f57806310e1b9b814610284578063136439dd146102ad57806315ea7917146102c057806315fe5028146102d3575b5f5ffd5b61028261027d366004614213565b6106f2565b005b610297610292366004614311565b610c31565b6040516102a49190614358565b60405180910390f35b6102826102bb36600461438b565b610c66565b6102826102ce3660046143e2565b610d3b565b6102e66102e1366004614420565b611258565b6040516102a4919061449e565b6103066103013660046144b0565b61136f565b60405190151581526020016102a4565b61033d7f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102a4565b61036561036036600461455f565b6113a6565b6040516102a49190614613565b610385610380366004614420565b611693565b6040516001600160a01b0390911681526020016102a4565b6103b06103ab366004614676565b6116c2565b6040516102a492919061470e565b6103d16103cc3660046144b0565b61183d565b6040516102a4919061476b565b6103f16103ec36600461477d565b611861565b6040516102a491906147c0565b61028261040c36600461480b565b611909565b6103f161041f36600461488b565b6119c3565b6102826104323660046148d7565b611a6b565b610282611abe565b61030661044d36600461490a565b606654600160ff9092169190911b9081161490565b61028261047036600461492a565b611b6d565b6066545b6040519081526020016102a4565b61049a610495366004614676565b611c1d565b6040516001600160401b0390911681526020016102a4565b6102826104c0366004614959565b611d8a565b6103d16104d33660046144b0565b61215a565b61028261216b565b6102826104ee36600461498a565b61217c565b6102e6610501366004614420565b6122a6565b61033d7f000000000000000000000000000000000000000000000000000000000000000081565b61028261053b3660046143e2565b612380565b6103857f000000000000000000000000000000000000000000000000000000000000000081565b61057a6105753660046149d8565b6125ad565b6040516102a49190614a1b565b6033546001600160a01b0316610385565b6103f16105a6366004614a2d565b612672565b61049a6105b9366004614676565b61275e565b61049a6105cc366004614676565b60a260209081525f92835260408084209091529082529020546001600160401b031681565b6102826105ff366004614a88565b61278d565b6104796106123660046144b0565b6127d4565b610282610625366004614959565b6127e4565b61063d610638366004614420565b612af8565b60408051921515835263ffffffff9091166020830152016102a4565b6103d1610667366004614af4565b612b92565b61028261067a366004614b1f565b612bc3565b61028261068d36600461498a565b612cdf565b6103857f000000000000000000000000000000000000000000000000000000000000000081565b6102826106c7366004614420565b612e09565b6102826106da366004614420565b612e81565b6102826106ed36600461438b565b612ef7565b60665460019060029081160361071b5760405163840a48d560e01b815260040160405180910390fd5b81604001355f10801561073a5750670de0b6b3a7640000604083013511155b61075757604051631353603160e01b815260040160405180910390fd5b5f6040518060400160405280336001600160a01b03168152602001846020016020810190610785919061492a565b63ffffffff16905290505f6107a66107a06020860186614420565b83613006565b60208084015184516001600160a01b03165f908152609890925260409091209192506107dc919063ffffffff9081169061307b16565b6107f957604051631fb1705560e21b815260040160405180910390fd5b80610817576040516325131d4f60e01b815260040160405180910390fd5b5f61083960995f61082786613092565b81526020019081526020015f206130f0565b90505f816001600160401b038111156108545761085461425d565b60405190808252806020026020018201604052801561087d578160200160208202803683370190505b5090505f826001600160401b038111156108995761089961425d565b6040519080825280602002602001820160405280156108c2578160200160208202803683370190505b5090505f5b83811015610bcf575f6108fb8260995f6108e08b613092565b81526020019081526020015f206130f990919063ffffffff16565b90505f8061091e61090f60208d018d614420565b6109188b613092565b85613104565b915091508286858151811061093557610935614b49565b6001600160a01b039092166020928302919091019091015280516001600160401b03165f0361096657505050610bc7565b80515f90610981906001600160401b031660408e0135613270565b83519091505f9061099e906001600160401b038085169116613286565b9050808787815181106109b3576109b3614b49565b60200260200101818152505081835f018181516109d09190614b71565b6001600160401b03169052508351829085906109ed908390614b71565b6001600160401b0316905250602084018051839190610a0d908390614b71565b6001600160401b031690525060208301515f600f9190910b1215610ae3575f610a518e604001358560200151610a4290614b90565b6001600160801b031690613270565b9050806001600160401b031684602001818151610a6e9190614bb4565b915090600f0b9081600f0b815250507f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd8e5f016020810190610ab09190614420565b8d88610ac3885f0151896020015161329a565b8860400151604051610ad9959493929190614be1565b60405180910390a1505b610b04610af360208f018f614420565b610afc8d613092565b8787876132ae565b610b1d610b1460208f018f614420565b8551879061351a565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663b6f73bdf8e5f016020810190610b5e9190614420565b6040516001600160e01b031960e084901b1681526001600160a01b0391821660048201529088166024820152604481018490526064015f604051808303815f87803b158015610bab575f5ffd5b505af1158015610bbd573d5f5f3e3d5ffd5b5050505050505050505b6001016108c7565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5610bfe6020890189614420565b868484610c0e60608d018d614c32565b604051610c2096959493929190614c9c565b60405180910390a150505050505050565b604080516060810182525f8082526020820181905291810182905290610c5a8561091886613092565b925050505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015610cc8573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190610cec9190614cfe565b610d0957604051631d77d47760e21b815260040160405180910390fd5b6066548181168114610d2e5760405163c61dca5d60e01b815260040160405180910390fd5b610d378261359e565b5050565b6066545f90600190811603610d635760405163840a48d560e01b815260040160405180910390fd5b5f5f610d6e33612af8565b9150915081610d905760405163fa55fc8160e01b815260040160405180910390fd5b5f5b8481101561125057858582818110610dac57610dac614b49565b9050602002810190610dbe9190614d1d565b610dcc906060810190614d3b565b9050868683818110610de057610de0614b49565b9050602002810190610df29190614d1d565b610e00906040810190614d3b565b905014610e20576040516343714afd60e01b815260040160405180910390fd5b36868683818110610e3357610e33614b49565b9050602002810190610e459190614d1d565b90505f610e6033610e5b368590038501856144b0565b613006565b9050610ea9610e75604084016020850161492a565b63ffffffff1660985f610e8b6020870187614420565b6001600160a01b0316815260208101919091526040015f209061307b565b610ec657604051631fb1705560e21b815260040160405180910390fd5b5f5b888885818110610eda57610eda614b49565b9050602002810190610eec9190614d1d565b610efa906040810190614d3b565b9050811015611245575f898986818110610f1657610f16614b49565b9050602002810190610f289190614d1d565b610f36906040810190614d3b565b83818110610f4657610f46614b49565b9050602002016020810190610f5b9190614420565b9050610f6a338261ffff6135db565b5f80610f8733610918610f82368a90038a018a6144b0565b613092565b915091508060200151600f0b5f14610fb257604051630d8fcbe360e41b815260040160405180910390fd5b5f610fcd610fc5368990038901896144b0565b8584896136df565b9050611032825f01518e8e8b818110610fe857610fe8614b49565b9050602002810190610ffa9190614d1d565b611008906060810190614d3b565b8881811061101857611018614b49565b905060200201602081019061102d9190614d80565b613745565b600f0b602083018190525f0361105b57604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b12156111aa57801561111d576110e3611085610f82368a90038a018a6144b0565b335f90815260a3602090815260408083206001600160a01b038a1684529091529020908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b61110d7f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff166040830152611217565b61112f8360200151836020015161329a565b6001600160401b031660208401528c8c8981811061114f5761114f614b49565b90506020028101906111619190614d1d565b61116f906060810190614d3b565b8681811061117f5761117f614b49565b90506020020160208101906111949190614d80565b6001600160401b031682525f6020830152611217565b5f8260200151600f0b1315611217576111cb8360200151836020015161329a565b6001600160401b03908116602085018190528451909116101561120157604051636c9be0bf60e01b815260040160405180910390fd5b61120b8943614da6565b63ffffffff1660408301525b6112353361122d610f82368b90038b018b6144b0565b8686866132ae565b505060019092019150610ec89050565b505050600101610d92565b505050505050565b6001600160a01b0381165f908152609d602052604081206060919061127c906130f0565b90505f816001600160401b038111156112975761129761425d565b6040519080825280602002602001820160405280156112db57816020015b604080518082019091525f80825260208201528152602001906001900390816112b55790505b5090505f5b82811015611367576001600160a01b0385165f908152609d602052604090206113429061130d90836130f9565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b82828151811061135457611354614b49565b60209081029190910101526001016112e0565b509392505050565b60208082015182516001600160a01b03165f9081526098909252604082206113a09163ffffffff9081169061307b16565b92915050565b606083516001600160401b038111156113c1576113c161425d565b6040519080825280602002602001820160405280156113f457816020015b60608152602001906001900390816113df5790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b8152600401611446929190614dc2565b5f60405180830381865afa158015611460573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526114879190810190614de6565b90505f5b8551811015611689575f8682815181106114a7576114a7614b49565b6020026020010151905085516001600160401b038111156114ca576114ca61425d565b6040519080825280602002602001820160405280156114f3578160200160208202803683370190505b5084838151811061150657611506614b49565b60209081029190910101525f5b865181101561167f575f87828151811061152f5761152f614b49565b6020908102919091018101516001600160a01b038086165f90815260a184526040808220928416825291909352822090925061156a9061375c565b9050806001600160401b03165f03611583575050611677565b5f61158f858d85610c31565b90508863ffffffff16816040015163ffffffff16111580156115b757505f8160200151600f0b125b156115d9576115cd815f0151826020015161329a565b6001600160401b031681525b80515f906115f4906001600160401b03908116908516613286565b905061163b8189898151811061160c5761160c614b49565b6020026020010151878151811061162557611625614b49565b60200260200101516137a890919063ffffffff16565b89888151811061164d5761164d614b49565b6020026020010151868151811061166657611666614b49565b602002602001018181525050505050505b600101611513565b505060010161148b565b5050949350505050565b6001600160a01b038082165f9081526097602052604081205490911680156116bb5780610c5f565b5090919050565b6001600160a01b0382165f908152609d6020526040812060609182916116e7906130f0565b90505f816001600160401b038111156117025761170261425d565b60405190808252806020026020018201604052801561174657816020015b604080518082019091525f80825260208201528152602001906001900390816117205790505b5090505f826001600160401b038111156117625761176261425d565b6040519080825280602002602001820160405280156117ab57816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816117805790505b5090505f5b8381101561182e576001600160a01b0388165f908152609d602052604081206117dd9061130d90846130f9565b9050808483815181106117f2576117f2614b49565b602002602001018190525061180889828a610c31565b83838151811061181a5761181a614b49565b6020908102919091010152506001016117b0565b509093509150505b9250929050565b60605f610c5f60995f61184f86613092565b81526020019081526020015f206137bc565b60605f83516001600160401b0381111561187d5761187d61425d565b6040519080825280602002602001820160405280156118a6578160200160208202803683370190505b5090505f5b8451811015611367576118d78582815181106118c9576118c9614b49565b60200260200101518561275e565b8282815181106118e9576118e9614b49565b6001600160401b03909216602092830291909101909101526001016118ab565b6066545f906001908116036119315760405163840a48d560e01b815260040160405180910390fd5b838214611951576040516343714afd60e01b815260040160405180910390fd5b5f5b848110156119ba576119b28787878481811061197157611971614b49565b90506020020160208101906119869190614420565b86868581811061199857611998614b49565b90506020020160208101906119ad9190614ef2565b6135db565b600101611953565b50505050505050565b60605f82516001600160401b038111156119df576119df61425d565b604051908082528060200260200182016040528015611a08578160200160208202803683370190505b5090505f5b835181101561136757611a3985858381518110611a2c57611a2c614b49565b602002602001015161275e565b828281518110611a4b57611a4b614b49565b6001600160401b0390921660209283029190910190910152600101611a0d565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611ab45760405163f739589b60e01b815260040160405180910390fd5b610d3782826137c8565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611b20573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611b449190614cfe565b611b6157604051631d77d47760e21b815260040160405180910390fd5b611b6b5f1961359e565b565b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa158015611bcf573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611bf39190614cfe565b611c10576040516325ec6c1f60e01b815260040160405180910390fd5b611c1a33826137c8565b50565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611d47576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120611caf9083613969565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015611d2a575050611d47565b611d3885826020015161329a565b94505050806001019050611c77565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208290611d779061375c565b611d819190614b71565b95945050505050565b606654600290600490811603611db35760405163840a48d560e01b815260040160405180910390fd5b611dc06020830183614420565b6001600160a01b0316336001600160a01b03161480611dff5750611dea6040830160208401614420565b6001600160a01b0316336001600160a01b0316145b611e1c576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b611e2b6040840184614d3b565b90508110156120cf575f6040518060400160405280856020016020810190611e539190614420565b6001600160a01b03168152602001611e6e6040870187614d3b565b85818110611e7e57611e7e614b49565b9050602002016020810190611e93919061492a565b63ffffffff168152509050611ec2816020015163ffffffff1660985f876020016020810190610e8b9190614420565b611edf57604051631fb1705560e21b815260040160405180910390fd5b609e5f611eef6020870187614420565b6001600160a01b03166001600160a01b031681526020019081526020015f205f611f1883613092565b815260208101919091526040015f205460ff16611f48576040516325131d4f60e01b815260040160405180910390fd5b611f82611f5482613092565b609c5f611f646020890189614420565b6001600160a01b0316815260208101919091526040015f20906139d8565b50611fba611f936020860186614420565b609a5f611f9f85613092565b81526020019081526020015f206139e390919063ffffffff16565b50611fc86020850185614420565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe826040516120009190614f13565b60405180910390a2604080518082019091525f8152602081016120437f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff169052609e5f61205b6020880188614420565b6001600160a01b03166001600160a01b031681526020019081526020015f205f61208484613092565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101611e1e565b506120e36103806040840160208501614420565b6001600160a01b0316639d8e0c236120fe6020850185614420565b61210b6040860186614d3b565b6040518463ffffffff1660e01b815260040161212993929190614f5a565b5f604051808303815f87803b158015612140575f5ffd5b505af1925050508015612151575060015b15610d37575050565b60606113a0609a5f61184f85613092565b6121736139f7565b611b6b5f613a51565b6040805180820182523380825263ffffffff80871660208085018290525f938452609890529390912091926121b2929161307b16565b6121cf57604051631fb1705560e21b815260040160405180910390fd5b5f6121d982613092565b90505f5b83811015611250576122228585838181106121fa576121fa614b49565b905060200201602081019061220f9190614420565b5f84815260996020526040902090613aa2565b61223f5760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b8386868481811061227357612273614b49565b90506020020160208101906122889190614420565b604051612296929190614f7e565b60405180910390a16001016121dd565b6001600160a01b0381165f908152609c60205260408120606091906122ca906130f0565b90505f816001600160401b038111156122e5576122e561425d565b60405190808252806020026020018201604052801561232957816020015b604080518082019091525f80825260208201528152602001906001900390816123035790505b5090505f5b82811015611367576001600160a01b0385165f908152609c6020526040902061235b9061130d90836130f9565b82828151811061236d5761236d614b49565b602090810291909101015260010161232e565b5f5b818110156125a8575f6040518060400160405280336001600160a01b031681526020018585858181106123b7576123b7614b49565b90506020028101906123c99190614fa4565b6123d790602081019061492a565b63ffffffff908116909152602080830151335f90815260989092526040909120929350612409929190811690613ab616565b61242657604051631fb1705560e21b815260040160405180910390fd5b60408051808201825233815260208381015163ffffffff169082015290517f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c9161246f91614f13565b60405180910390a15f61248182613092565b90505f5b85858581811061249757612497614b49565b90506020028101906124a99190614fa4565b6124b7906020810190614d3b565b905081101561259d576125058686868181106124d5576124d5614b49565b90506020028101906124e79190614fa4565b6124f5906020810190614d3b565b838181106121fa576121fa614b49565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b8387878781811061253a5761253a614b49565b905060200281019061254c9190614fa4565b61255a906020810190614d3b565b8481811061256a5761256a614b49565b905060200201602081019061257f9190614420565b60405161258d929190614f7e565b60405180910390a1600101612485565b505050600101612382565b505050565b60605f84516001600160401b038111156125c9576125c961425d565b60405190808252806020026020018201604052801561261257816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816125e75790505b5090505f5b85518110156126695761264486828151811061263557612635614b49565b60200260200101518686610c31565b82828151811061265657612656614b49565b6020908102919091010152600101612617565b50949350505050565b60605f83516001600160401b0381111561268e5761268e61425d565b6040519080825280602002602001820160405280156126b7578160200160208202803683370190505b5090505f5b8451811015612669576001600160a01b0386165f90815260a160205260408120865161272c928792918990869081106126f7576126f7614b49565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f20613ac190919063ffffffff16565b82828151811061273e5761273e614b49565b6001600160401b03909216602092830291909101909101526001016126bc565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610c5f9061375c565b336001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371383836040516127c8929190614fb8565b60405180910390a25050565b5f6113a0609a5f61082785613092565b60665460029060049081160361280d5760405163840a48d560e01b815260040160405180910390fd5b6040516336b87bd760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690636d70f7ae90602401602060405180830381865afa15801561286f573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906128939190614cfe565b6128b05760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b6128bf6020840184614d3b565b9050811015612a7557604080518082019091525f90806128e26020870187614420565b6001600160a01b031681526020018580602001906129009190614d3b565b8581811061291057612910614b49565b9050602002016020810190612925919061492a565b63ffffffff90811690915260208083015183516001600160a01b03165f9081526098909252604090912092935061296192919081169061307b16565b61297e57604051631fb1705560e21b815260040160405180910390fd5b6129883382613006565b156129a657604051636c6c6e2760e11b815260040160405180910390fd5b6129c66129b282613092565b335f908152609c6020526040902090613ab6565b506129f233609a5f6129d785613092565b81526020019081526020015f20613aa290919063ffffffff16565b50336001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612a2c9190614f13565b60405180910390a2335f908152609e60205260408120600191612a4e84613092565b815260208101919091526040015f20805460ff1916911515919091179055506001016128b2565b50612a866103806020840184614420565b6001600160a01b031663adcf73f733612aa26020860186614d3b565b612aaf6040880188614c32565b6040518663ffffffff1660e01b8152600401612acf959493929190614fcb565b5f604051808303815f87803b158015612ae6575f5ffd5b505af1158015611250573d5f5f3e3d5ffd5b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052650100000000008304821694840194909452600160481b909104166060820181905284939192919015801590612b795750826060015163ffffffff164310155b15612b88575050604081015160015b9590945092505050565b6001600160a01b0382165f908152609f6020526040812060609190612bbb908261184f86613092565b949350505050565b5f54610100900460ff1615808015612be157505f54600160ff909116105b80612bfa5750303b158015612bfa57505f5460ff166001145b612c625760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff191660011790558015612c83575f805461ff0019166101001790555b612c8c8261359e565b612c9583613a51565b80156125a8575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a1505050565b6040805180820182523380825263ffffffff80871660208085018290525f93845260989052939091209192612d15929161307b16565b612d3257604051631fb1705560e21b815260040160405180910390fd5b5f612d3c82613092565b90505f5b8381101561125057612d85858583818110612d5d57612d5d614b49565b9050602002016020810190612d729190614420565b5f848152609960205260409020906139e3565b612da2576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee83868684818110612dd657612dd6614b49565b9050602002016020810190612deb9190614420565b604051612df9929190614f7e565b60405180910390a1600101612d40565b335f81815260976020526040902080546001600160a01b0319166001600160a01b0384161790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf8590612e5b81611693565b604080516001600160a01b0393841681529290911660208301520160405180910390a150565b612e896139f7565b6001600160a01b038116612eee5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401612c59565b611c1a81613a51565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612f53573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612f77919061500e565b6001600160a01b0316336001600160a01b031614612fa85760405163794821ff60e01b815260040160405180910390fd5b60665480198219811614612fcf5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016127c8565b6001600160a01b0382165f908152609e6020526040812081908161302985613092565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff1692820192909252915080612bbb57506020015163ffffffff1643109392505050565b5f8181526001830160205260408120541515610c5f565b5f815f0151826020015163ffffffff166040516020016130d892919060609290921b6001600160601b031916825260a01b6001600160a01b031916601482015260200190565b6040516020818303038152906040526113a090615029565b5f6113a0825490565b5f610c5f8383613b11565b6040805180820182525f80825260208083018290528351606081018552828152808201839052808501839052845180860186526001600160a01b03898116855260a18452868520908816855290925293822092939281906131649061375c565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff16918101829052919250431015613206579092509050613268565b613217815f0151826020015161329a565b6001600160401b0316815260208101515f600f9190910b1215613255576132468260200151826020015161329a565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b5f610c5f8383670de0b6b3a76400006001613b37565b5f610c5f83670de0b6b3a764000084613b90565b5f610c5f826001600160401b038516614bb4565b602082810180516001600160a01b038881165f81815260a286526040808220938a1680835293875290819020805467ffffffffffffffff19166001600160401b0395861617905593518451918252948101919091529216908201527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559060600160405180910390a16001600160a01b038581165f90815260a060209081526040808320888452825280832093871683529281528282208451815486840151878701516001600160401b039093166001600160c01b031990921691909117600160401b6001600160801b03909216919091021763ffffffff60c01b1916600160c01b63ffffffff9283160217909155835180850185528381528201929092528251808401909352606087901c8352908616908201527f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd908690835160405161341a93929188914390614be1565b60405180910390a16020810151600f0b15613483576001600160a01b0385165f908152609f60209081526040808320878452909152902061345b9084613aa2565b506001600160a01b0385165f908152609d6020526040902061347d9085613ab6565b50613513565b80516001600160401b03165f03613513576001600160a01b0385165f908152609f6020908152604080832087845290915290206134c090846139e3565b506001600160a01b0385165f908152609f6020908152604080832087845290915290206134ec906130f0565b5f03613513576001600160a01b0385165f908152609d6020526040902061125090856139d8565b5050505050565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613549904383613c75565b5050604080516001600160a01b038086168252841660208201526001600160401b038316918101919091527f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c90606001612cd2565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f8111801561362557508261ffff1682105b15613513576001600160a01b038086165f90815260a360209081526040808320938816835292905290812061365990613c8e565b90505f5f613668338489613104565b91509150806040015163ffffffff1643101561368657505050613513565b61369388848985856132ae565b6001600160a01b038089165f90815260a360209081526040808320938b168352929052206136c090613ce0565b506136ca8561504c565b94506136d584615064565b9350505050613613565b5f61370b8460995f6136f089613092565b81526020019081526020015f20613d5d90919063ffffffff16565b61371657505f612bbb565b8161372257505f612bbb565b82516001600160401b03165f0361373a57505f612bbb565b506001949350505050565b5f610c5f6001600160401b03808516908416615079565b80545f90801561379857613782836137756001846150a6565b5f91825260209091200190565b54600160201b90046001600160401b0316610c5f565b670de0b6b3a76400009392505050565b5f610c5f8383670de0b6b3a7640000613b90565b60605f610c5f83613d7e565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b9092041660608201819052158015906138445750806060015163ffffffff164310155b1561385e57604081015163ffffffff168152600160208201525b63ffffffff821660408201526138947f000000000000000000000000000000000000000000000000000000000000000043614da6565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9101612cd2565b5f5f61398b61397784613dd7565b85546139869190600f0b6150b9565b613e44565b8454909150600160801b9004600f90810b9082900b126139be57604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f610c5f8383613ead565b5f610c5f836001600160a01b038416613ead565b6033546001600160a01b03163314611b6b5760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401612c59565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f610c5f836001600160a01b038416613f90565b5f610c5f8383613f90565b81545f9081613ad285858385613fdc565b90508015613aff57613ae9856137756001846150a6565b54600160201b90046001600160401b0316611d81565b50670de0b6b3a7640000949350505050565b5f825f018281548110613b2657613b26614b49565b905f5260205f200154905092915050565b5f5f613b44868686613b90565b90506001836002811115613b5a57613b5a6150e0565b148015613b7657505f8480613b7157613b716150f4565b868809115b15611d8157613b86600182615108565b9695505050505050565b5f80805f19858709858702925082811083820303915050805f03613bc757838281613bbd57613bbd6150f4565b0492505050610c5f565b808411613c0e5760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401612c59565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f80613c8285858561402f565b91509150935093915050565b5f613ca88254600f81810b600160801b909204900b131590565b15613cc657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f613cfa8254600f81810b600160801b909204900b131590565b15613d1857604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b6001600160a01b0381165f9081526001830160205260408120541515610c5f565b6060815f01805480602002602001604051908101604052809291908181526020018280548015613dcb57602002820191905f5260205f20905b815481526020019060010190808311613db7575b50505050509050919050565b5f6001600160ff1b03821115613e405760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401612c59565b5090565b80600f81900b8114613ea85760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401612c59565b919050565b5f8181526001830160205260408120548015613f87575f613ecf6001836150a6565b85549091505f90613ee2906001906150a6565b9050818114613f41575f865f018281548110613f0057613f00614b49565b905f5260205f200154905080875f018481548110613f2057613f20614b49565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080613f5257613f5261511b565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f9055600193505050506113a0565b5f9150506113a0565b5f818152600183016020526040812054613fd557508154600181810184555f8481526020808220909301849055845484825282860190935260409020919091556113a0565b505f6113a0565b5f5b81831015611367575f613ff184846141f9565b5f8781526020902090915063ffffffff86169082015463ffffffff16111561401b57809250614029565b614026816001615108565b93505b50613fde565b82545f908190801561418d575f61404b876137756001856150a6565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160401b0316602084015291925090871610156140cc5760405162461bcd60e51b815260206004820152601960248201527f536e617073686f743a2064656372656173696e67206b657973000000000000006044820152606401612c59565b805163ffffffff80881691160361411d57846140ed886137756001866150a6565b80546001600160401b0392909216600160201b026bffffffffffffffff000000001990921691909117905561417d565b6040805180820190915263ffffffff80881682526001600160401b0380881660208085019182528b54600181018d555f8d8152919091209451940180549151909216600160201b026001600160601b031990911693909216929092171790555b6020015192508391506132689050565b50506040805180820190915263ffffffff80851682526001600160401b0380851660208085019182528854600181018a555f8a81529182209551950180549251909316600160201b026001600160601b0319909216949093169390931792909217909155905081613268565b5f614207600284841861512f565b610c5f90848416615108565b5f60208284031215614223575f5ffd5b81356001600160401b03811115614238575f5ffd5b820160808185031215610c5f575f5ffd5b6001600160a01b0381168114611c1a575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156142995761429961425d565b604052919050565b803563ffffffff81168114613ea8575f5ffd5b5f604082840312156142c4575f5ffd5b604080519081016001600160401b03811182821017156142e6576142e661425d565b60405290508082356142f781614249565b8152614305602084016142a1565b60208201525092915050565b5f5f5f60808486031215614323575f5ffd5b833561432e81614249565b925061433d85602086016142b4565b9150606084013561434d81614249565b809150509250925092565b81516001600160401b03168152602080830151600f0b9082015260408083015163ffffffff1690820152606081016113a0565b5f6020828403121561439b575f5ffd5b5035919050565b5f5f83601f8401126143b2575f5ffd5b5081356001600160401b038111156143c8575f5ffd5b6020830191508360208260051b8501011115611836575f5ffd5b5f5f602083850312156143f3575f5ffd5b82356001600160401b03811115614408575f5ffd5b614414858286016143a2565b90969095509350505050565b5f60208284031215614430575f5ffd5b8135610c5f81614249565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b828110156144945761447e86835161443b565b604095909501946020919091019060010161446b565b5093949350505050565b602081525f610c5f6020830184614459565b5f604082840312156144c0575f5ffd5b610c5f83836142b4565b5f6001600160401b038211156144e2576144e261425d565b5060051b60200190565b5f82601f8301126144fb575f5ffd5b813561450e614509826144ca565b614271565b8082825260208201915060208360051b86010192508583111561452f575f5ffd5b602085015b8381101561455557803561454781614249565b835260209283019201614534565b5095945050505050565b5f5f5f5f60a08587031215614572575f5ffd5b61457c86866142b4565b935060408501356001600160401b03811115614596575f5ffd5b6145a2878288016144ec565b93505060608501356001600160401b038111156145bd575f5ffd5b6145c9878288016144ec565b9250506145d8608086016142a1565b905092959194509250565b5f8151808452602084019350602083015f5b828110156144945781518652602095860195909101906001016145f5565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b8281101561466a57603f198786030184526146558583516145e3565b94506020938401939190910190600101614639565b50929695505050505050565b5f5f60408385031215614687575f5ffd5b823561469281614249565b915060208301356146a281614249565b809150509250929050565b5f8151808452602084019350602083015f5b82811015614494576146f886835180516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b60609590950194602091909101906001016146bf565b604081525f6147206040830185614459565b8281036020840152611d8181856146ad565b5f8151808452602084019350602083015f5b828110156144945781516001600160a01b0316865260209586019590910190600101614744565b602081525f610c5f6020830184614732565b5f5f6040838503121561478e575f5ffd5b82356001600160401b038111156147a3575f5ffd5b6147af858286016144ec565b92505060208301356146a281614249565b602080825282518282018190525f918401906040840190835b818110156148005783516001600160401b03168352602093840193909201916001016147d9565b509095945050505050565b5f5f5f5f5f6060868803121561481f575f5ffd5b853561482a81614249565b945060208601356001600160401b03811115614844575f5ffd5b614850888289016143a2565b90955093505060408601356001600160401b0381111561486e575f5ffd5b61487a888289016143a2565b969995985093965092949392505050565b5f5f6040838503121561489c575f5ffd5b82356148a781614249565b915060208301356001600160401b038111156148c1575f5ffd5b6148cd858286016144ec565b9150509250929050565b5f5f604083850312156148e8575f5ffd5b82356148f381614249565b9150614901602084016142a1565b90509250929050565b5f6020828403121561491a575f5ffd5b813560ff81168114610c5f575f5ffd5b5f6020828403121561493a575f5ffd5b610c5f826142a1565b5f60608284031215614953575f5ffd5b50919050565b5f60208284031215614969575f5ffd5b81356001600160401b0381111561497e575f5ffd5b612bbb84828501614943565b5f5f5f6040848603121561499c575f5ffd5b6149a5846142a1565b925060208401356001600160401b038111156149bf575f5ffd5b6149cb868287016143a2565b9497909650939450505050565b5f5f5f608084860312156149ea575f5ffd5b83356001600160401b038111156149ff575f5ffd5b614a0b868287016144ec565b93505061433d85602086016142b4565b602081525f610c5f60208301846146ad565b5f5f5f60608486031215614a3f575f5ffd5b8335614a4a81614249565b925060208401356001600160401b03811115614a64575f5ffd5b614a70868287016144ec565b925050614a7f604085016142a1565b90509250925092565b5f5f60208385031215614a99575f5ffd5b82356001600160401b03811115614aae575f5ffd5b8301601f81018513614abe575f5ffd5b80356001600160401b03811115614ad3575f5ffd5b856020828401011115614ae4575f5ffd5b6020919091019590945092505050565b5f5f60608385031215614b05575f5ffd5b8235614b1081614249565b915061490184602085016142b4565b5f5f60408385031215614b30575f5ffd5b8235614b3b81614249565b946020939093013593505050565b634e487b7160e01b5f52603260045260245ffd5b634e487b7160e01b5f52601160045260245ffd5b6001600160401b0382811682821603908111156113a0576113a0614b5d565b5f81600f0b60016001607f1b03198103614bac57614bac614b5d565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b0319821217156113a0576113a0614b5d565b6001600160a01b038616815260c08101614bfe602083018761443b565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e19843603018112614c47575f5ffd5b8301803591506001600160401b03821115614c60575f5ffd5b602001915036819003821315611836575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b0387168152614cb5602082018761443b565b60c060608201525f614cca60c0830187614732565b8281036080840152614cdc81876145e3565b905082810360a0840152614cf1818587614c74565b9998505050505050505050565b5f60208284031215614d0e575f5ffd5b81518015158114610c5f575f5ffd5b5f8235607e19833603018112614d31575f5ffd5b9190910192915050565b5f5f8335601e19843603018112614d50575f5ffd5b8301803591506001600160401b03821115614d69575f5ffd5b6020019150600581901b3603821315611836575f5ffd5b5f60208284031215614d90575f5ffd5b81356001600160401b0381168114610c5f575f5ffd5b63ffffffff81811683821601908111156113a0576113a0614b5d565b604081525f614dd46040830185614732565b8281036020840152611d818185614732565b5f60208284031215614df6575f5ffd5b81516001600160401b03811115614e0b575f5ffd5b8201601f81018413614e1b575f5ffd5b8051614e29614509826144ca565b8082825260208201915060208360051b850101925086831115614e4a575f5ffd5b602084015b83811015614ee75780516001600160401b03811115614e6c575f5ffd5b8501603f81018913614e7c575f5ffd5b6020810151614e8d614509826144ca565b808282526020820191506020808460051b8601010192508b831115614eb0575f5ffd5b6040840193505b82841015614ed2578351825260209384019390910190614eb7565b86525050602093840193919091019050614e4f565b509695505050505050565b5f60208284031215614f02575f5ffd5b813561ffff81168114610c5f575f5ffd5b604081016113a0828461443b565b8183526020830192505f815f5b848110156144945763ffffffff614f44836142a1565b1686526020958601959190910190600101614f2e565b6001600160a01b03841681526040602082018190525f90611d819083018486614f21565b60608101614f8c828561443b565b6001600160a01b039290921660409190910152919050565b5f8235603e19833603018112614d31575f5ffd5b602081525f612bbb602083018486614c74565b6001600160a01b03861681526060602082018190525f90614fef9083018688614f21565b8281036040840152615002818587614c74565b98975050505050505050565b5f6020828403121561501e575f5ffd5b8151610c5f81614249565b80516020808301519190811015614953575f1960209190910360031b1b16919050565b5f6001820161505d5761505d614b5d565b5060010190565b5f8161507257615072614b5d565b505f190190565b600f82810b9082900b0360016001607f1b0319811260016001607f1b03821317156113a0576113a0614b5d565b818103818111156113a0576113a0614b5d565b8082018281125f8312801582168215821617156150d8576150d8614b5d565b505092915050565b634e487b7160e01b5f52602160045260245ffd5b634e487b7160e01b5f52601260045260245ffd5b808201808211156113a0576113a0614b5d565b634e487b7160e01b5f52603160045260245ffd5b5f8261514957634e487b7160e01b5f52601260045260245ffd5b50049056fea26469706673582212209d78270e88d1507a8a852df378661efd73b7f84e362d47eeb6c9a8da7b9ae33064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02kW_5`\xE0\x1C\x80cqP\x18\xA6\x11a\x01KW\x80c\xA9\x8F\xB3U\x11a\0\xBFW\x80c\xCDm\xC6\x87\x11a\0\x84W\x80c\xCDm\xC6\x87\x14a\x06lW\x80c\xCE{^K\x14a\x06\x7FW\x80c\xDF\\\xF7#\x14a\x06\x92W\x80c\xF2_\x16\x10\x14a\x06\xB9W\x80c\xF2\xFD\xE3\x8B\x14a\x06\xCCW\x80c\xFA\xBC\x1C\xBC\x14a\x06\xDFW__\xFD[\x80c\xA9\x8F\xB3U\x14a\x05\xF1W\x80c\xB2Dz\xF7\x14a\x06\x04W\x80c\xB9/`\xA5\x14a\x06\x17W\x80c\xB9\xFB\xAE\xD1\x14a\x06*W\x80c\xC2!\xD8\xAE\x14a\x06YW__\xFD[\x80c\x88o\x11\x95\x11a\x01\x10W\x80c\x88o\x11\x95\x14a\x05@W\x80c\x8C\xE6HT\x14a\x05gW\x80c\x8D\xA5\xCB[\x14a\x05\x87W\x80c\x94\xD7\xD0\x0C\x14a\x05\x98W\x80c\xA93>\xC8\x14a\x05\xABW\x80c\xA9\x84\xEB:\x14a\x05\xBEW__\xFD[\x80cqP\x18\xA6\x14a\x04\xD8W\x80cv\x99\x93B\x14a\x04\xE0W\x80cy\xAEP\xCD\x14a\x04\xF3W\x80c{\xC1\xEFa\x14a\x05\x06W\x80c\x84}cO\x14a\x05-W__\xFD[\x80cJ\x10\xFF\xE5\x11a\x01\xE2W\x80cZ\xC8j\xB7\x11a\x01\xA7W\x80cZ\xC8j\xB7\x14a\x04?W\x80c\\H\x9B\xB5\x14a\x04bW\x80c\\\x97Z\xBB\x14a\x04uW\x80cl\xFBD\x81\x14a\x04\x87W\x80cn4\x92\xB5\x14a\x04\xB2W\x80cn\x87]\xBA\x14a\x04\xC5W__\xFD[\x80cJ\x10\xFF\xE5\x14a\x03\xDEW\x80cKPF\xEF\x14a\x03\xFEW\x80cTz\xFB\x87\x14a\x04\x11W\x80cV\xC4\x83\xE6\x14a\x04$W\x80cY\\jg\x14a\x047W__\xFD[\x80c&\r\xC7X\x11a\x023W\x80c&\r\xC7X\x14a\x02\xF3W\x80c)\x81\xEBw\x14a\x03\x16W\x80c+\xAB,J\x14a\x03RW\x80c0L\x10\xCD\x14a\x03rW\x80c@\x12\r\xAB\x14a\x03\x9DW\x80cAw\xA8|\x14a\x03\xBEW__\xFD[\x80c\x0E\xA4>C\x14a\x02oW\x80c\x10\xE1\xB9\xB8\x14a\x02\x84W\x80c\x13d9\xDD\x14a\x02\xADW\x80c\x15\xEAy\x17\x14a\x02\xC0W\x80c\x15\xFEP(\x14a\x02\xD3W[__\xFD[a\x02\x82a\x02}6`\x04aB\x13V[a\x06\xF2V[\0[a\x02\x97a\x02\x926`\x04aC\x11V[a\x0C1V[`@Qa\x02\xA4\x91\x90aCXV[`@Q\x80\x91\x03\x90\xF3[a\x02\x82a\x02\xBB6`\x04aC\x8BV[a\x0CfV[a\x02\x82a\x02\xCE6`\x04aC\xE2V[a\r;V[a\x02\xE6a\x02\xE16`\x04aD V[a\x12XV[`@Qa\x02\xA4\x91\x90aD\x9EV[a\x03\x06a\x03\x016`\x04aD\xB0V[a\x13oV[`@Q\x90\x15\x15\x81R` \x01a\x02\xA4V[a\x03=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x03ea\x03`6`\x04aE_V[a\x13\xA6V[`@Qa\x02\xA4\x91\x90aF\x13V[a\x03\x85a\x03\x806`\x04aD V[a\x16\x93V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x03\xB0a\x03\xAB6`\x04aFvV[a\x16\xC2V[`@Qa\x02\xA4\x92\x91\x90aG\x0EV[a\x03\xD1a\x03\xCC6`\x04aD\xB0V[a\x18=V[`@Qa\x02\xA4\x91\x90aGkV[a\x03\xF1a\x03\xEC6`\x04aG}V[a\x18aV[`@Qa\x02\xA4\x91\x90aG\xC0V[a\x02\x82a\x04\x0C6`\x04aH\x0BV[a\x19\tV[a\x03\xF1a\x04\x1F6`\x04aH\x8BV[a\x19\xC3V[a\x02\x82a\x0426`\x04aH\xD7V[a\x1AkV[a\x02\x82a\x1A\xBEV[a\x03\x06a\x04M6`\x04aI\nV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[a\x02\x82a\x04p6`\x04aI*V[a\x1BmV[`fT[`@Q\x90\x81R` \x01a\x02\xA4V[a\x04\x9Aa\x04\x956`\x04aFvV[a\x1C\x1DV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA4V[a\x02\x82a\x04\xC06`\x04aIYV[a\x1D\x8AV[a\x03\xD1a\x04\xD36`\x04aD\xB0V[a!ZV[a\x02\x82a!kV[a\x02\x82a\x04\xEE6`\x04aI\x8AV[a!|V[a\x02\xE6a\x05\x016`\x04aD V[a\"\xA6V[a\x03=\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x82a\x05;6`\x04aC\xE2V[a#\x80V[a\x03\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05za\x05u6`\x04aI\xD8V[a%\xADV[`@Qa\x02\xA4\x91\x90aJ\x1BV[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x85V[a\x03\xF1a\x05\xA66`\x04aJ-V[a&rV[a\x04\x9Aa\x05\xB96`\x04aFvV[a'^V[a\x04\x9Aa\x05\xCC6`\x04aFvV[`\xA2` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\x82a\x05\xFF6`\x04aJ\x88V[a'\x8DV[a\x04ya\x06\x126`\x04aD\xB0V[a'\xD4V[a\x02\x82a\x06%6`\x04aIYV[a'\xE4V[a\x06=a\x0686`\x04aD V[a*\xF8V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xA4V[a\x03\xD1a\x06g6`\x04aJ\xF4V[a+\x92V[a\x02\x82a\x06z6`\x04aK\x1FV[a+\xC3V[a\x02\x82a\x06\x8D6`\x04aI\x8AV[a,\xDFV[a\x03\x85\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x82a\x06\xC76`\x04aD V[a.\tV[a\x02\x82a\x06\xDA6`\x04aD V[a.\x81V[a\x02\x82a\x06\xED6`\x04aC\x8BV[a.\xF7V[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x07\x1BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`@\x015_\x10\x80\x15a\x07:WPg\r\xE0\xB6\xB3\xA7d\0\0`@\x83\x015\x11\x15[a\x07WW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x84` \x01` \x81\x01\x90a\x07\x85\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x07\xA6a\x07\xA0` \x86\x01\x86aD V[\x83a0\x06V[` \x80\x84\x01Q\x84Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x91\x92Pa\x07\xDC\x91\x90c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a0{\x16V[a\x07\xF9W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x08\x17W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x089`\x99_a\x08'\x86a0\x92V[\x81R` \x01\x90\x81R` \x01_ a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08TWa\x08TaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08}W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x99Wa\x08\x99aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xC2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x0B\xCFW_a\x08\xFB\x82`\x99_a\x08\xE0\x8Ba0\x92V[\x81R` \x01\x90\x81R` \x01_ a0\xF9\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P_\x80a\t\x1Ea\t\x0F` \x8D\x01\x8DaD V[a\t\x18\x8Ba0\x92V[\x85a1\x04V[\x91P\x91P\x82\x86\x85\x81Q\x81\x10a\t5Wa\t5aKIV[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\tfWPPPa\x0B\xC7V[\x80Q_\x90a\t\x81\x90`\x01`\x01`@\x1B\x03\x16`@\x8E\x015a2pV[\x83Q\x90\x91P_\x90a\t\x9E\x90`\x01`\x01`@\x1B\x03\x80\x85\x16\x91\x16a2\x86V[\x90P\x80\x87\x87\x81Q\x81\x10a\t\xB3Wa\t\xB3aKIV[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\t\xD0\x91\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\t\xED\x90\x83\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\n\r\x90\x83\x90aKqV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\n\xE3W_a\nQ\x8E`@\x015\x85` \x01Qa\nB\x90aK\x90V[`\x01`\x01`\x80\x1B\x03\x16\x90a2pV[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\nn\x91\x90aK\xB4V[\x91P\x90`\x0F\x0B\x90\x81`\x0F\x0B\x81RPP\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x8E_\x01` \x81\x01\x90a\n\xB0\x91\x90aD V[\x8D\x88a\n\xC3\x88_\x01Q\x89` \x01Qa2\x9AV[\x88`@\x01Q`@Qa\n\xD9\x95\x94\x93\x92\x91\x90aK\xE1V[`@Q\x80\x91\x03\x90\xA1P[a\x0B\x04a\n\xF3` \x8F\x01\x8FaD V[a\n\xFC\x8Da0\x92V[\x87\x87\x87a2\xAEV[a\x0B\x1Da\x0B\x14` \x8F\x01\x8FaD V[\x85Q\x87\x90a5\x1AV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xB6\xF7;\xDF\x8E_\x01` \x81\x01\x90a\x0B^\x91\x90aD V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x90\x88\x16`$\x82\x01R`D\x81\x01\x84\x90R`d\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0B\xABW__\xFD[PZ\xF1\x15\x80\x15a\x0B\xBDW=__>=_\xFD[PPPPPPPPP[`\x01\x01a\x08\xC7V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x0B\xFE` \x89\x01\x89aD V[\x86\x84\x84a\x0C\x0E``\x8D\x01\x8DaL2V[`@Qa\x0C \x96\x95\x94\x93\x92\x91\x90aL\x9CV[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x0CZ\x85a\t\x18\x86a0\x92V[\x92PPP[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xC8W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xEC\x91\x90aL\xFEV[a\r\tW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\r.W`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r7\x82a5\x9EV[PPV[`fT_\x90`\x01\x90\x81\x16\x03a\rcW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[__a\rn3a*\xF8V[\x91P\x91P\x81a\r\x90W`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x12PW\x85\x85\x82\x81\x81\x10a\r\xACWa\r\xACaKIV[\x90P` \x02\x81\x01\x90a\r\xBE\x91\x90aM\x1DV[a\r\xCC\x90``\x81\x01\x90aM;V[\x90P\x86\x86\x83\x81\x81\x10a\r\xE0Wa\r\xE0aKIV[\x90P` \x02\x81\x01\x90a\r\xF2\x91\x90aM\x1DV[a\x0E\0\x90`@\x81\x01\x90aM;V[\x90P\x14a\x0E W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[6\x86\x86\x83\x81\x81\x10a\x0E3Wa\x0E3aKIV[\x90P` \x02\x81\x01\x90a\x0EE\x91\x90aM\x1DV[\x90P_a\x0E`3a\x0E[6\x85\x90\x03\x85\x01\x85aD\xB0V[a0\x06V[\x90Pa\x0E\xA9a\x0Eu`@\x84\x01` \x85\x01aI*V[c\xFF\xFF\xFF\xFF\x16`\x98_a\x0E\x8B` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a0{V[a\x0E\xC6W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x88\x88\x85\x81\x81\x10a\x0E\xDAWa\x0E\xDAaKIV[\x90P` \x02\x81\x01\x90a\x0E\xEC\x91\x90aM\x1DV[a\x0E\xFA\x90`@\x81\x01\x90aM;V[\x90P\x81\x10\x15a\x12EW_\x89\x89\x86\x81\x81\x10a\x0F\x16Wa\x0F\x16aKIV[\x90P` \x02\x81\x01\x90a\x0F(\x91\x90aM\x1DV[a\x0F6\x90`@\x81\x01\x90aM;V[\x83\x81\x81\x10a\x0FFWa\x0FFaKIV[\x90P` \x02\x01` \x81\x01\x90a\x0F[\x91\x90aD V[\x90Pa\x0Fj3\x82a\xFF\xFFa5\xDBV[_\x80a\x0F\x873a\t\x18a\x0F\x826\x8A\x90\x03\x8A\x01\x8AaD\xB0V[a0\x92V[\x91P\x91P\x80` \x01Q`\x0F\x0B_\x14a\x0F\xB2W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x0F\xCDa\x0F\xC56\x89\x90\x03\x89\x01\x89aD\xB0V[\x85\x84\x89a6\xDFV[\x90Pa\x102\x82_\x01Q\x8E\x8E\x8B\x81\x81\x10a\x0F\xE8Wa\x0F\xE8aKIV[\x90P` \x02\x81\x01\x90a\x0F\xFA\x91\x90aM\x1DV[a\x10\x08\x90``\x81\x01\x90aM;V[\x88\x81\x81\x10a\x10\x18Wa\x10\x18aKIV[\x90P` \x02\x01` \x81\x01\x90a\x10-\x91\x90aM\x80V[a7EV[`\x0F\x0B` \x83\x01\x81\x90R_\x03a\x10[W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a\x11\xAAW\x80\x15a\x11\x1DWa\x10\xE3a\x10\x85a\x0F\x826\x8A\x90\x03\x8A\x01\x8AaD\xB0V[3_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x8A\x16\x84R\x90\x91R\x90 \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a\x11\r\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra\x12\x17V[a\x11/\x83` \x01Q\x83` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8C\x8C\x89\x81\x81\x10a\x11OWa\x11OaKIV[\x90P` \x02\x81\x01\x90a\x11a\x91\x90aM\x1DV[a\x11o\x90``\x81\x01\x90aM;V[\x86\x81\x81\x10a\x11\x7FWa\x11\x7FaKIV[\x90P` \x02\x01` \x81\x01\x90a\x11\x94\x91\x90aM\x80V[`\x01`\x01`@\x1B\x03\x16\x82R_` \x83\x01Ra\x12\x17V[_\x82` \x01Q`\x0F\x0B\x13\x15a\x12\x17Wa\x11\xCB\x83` \x01Q\x83` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a\x12\x01W`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\x0B\x89CaM\xA6V[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a\x1253a\x12-a\x0F\x826\x8B\x90\x03\x8B\x01\x8BaD\xB0V[\x86\x86\x86a2\xAEV[PP`\x01\x90\x92\x01\x91Pa\x0E\xC8\x90PV[PPP`\x01\x01a\r\x92V[PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\x12|\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12\x97Wa\x12\x97aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xDBW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x12\xB5W\x90P[P\x90P_[\x82\x81\x10\x15a\x13gW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\x13B\x90a\x13\r\x90\x83a0\xF9V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\x13TWa\x13TaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x12\xE0V[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\x13\xA0\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a0{\x16V[\x92\x91PPV[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13\xC1Wa\x13\xC1aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13\xF4W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x13\xDFW\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14F\x92\x91\x90aM\xC2V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14`W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\x87\x91\x90\x81\x01\x90aM\xE6V[\x90P_[\x85Q\x81\x10\x15a\x16\x89W_\x86\x82\x81Q\x81\x10a\x14\xA7Wa\x14\xA7aKIV[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x14\xCAWa\x14\xCAaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x14\xF3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a\x15\x06Wa\x15\x06aKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a\x16\x7FW_\x87\x82\x81Q\x81\x10a\x15/Wa\x15/aKIV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa\x15j\x90a7\\V[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x15\x83WPPa\x16wV[_a\x15\x8F\x85\x8D\x85a\x0C1V[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a\x15\xB7WP_\x81` \x01Q`\x0F\x0B\x12[\x15a\x15\xD9Wa\x15\xCD\x81_\x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a\x15\xF4\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a2\x86V[\x90Pa\x16;\x81\x89\x89\x81Q\x81\x10a\x16\x0CWa\x16\x0CaKIV[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x16%Wa\x16%aKIV[` \x02` \x01\x01Qa7\xA8\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a\x16MWa\x16MaKIV[` \x02` \x01\x01Q\x86\x81Q\x81\x10a\x16fWa\x16faKIV[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a\x15\x13V[PP`\x01\x01a\x14\x8BV[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\x16\xBBW\x80a\x0C_V[P\x90\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x16\xE7\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\x02Wa\x17\x02aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17FW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17 W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17bWa\x17baB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\xABW\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x17\x80W\x90P[P\x90P_[\x83\x81\x10\x15a\x18.W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x17\xDD\x90a\x13\r\x90\x84a0\xF9V[\x90P\x80\x84\x83\x81Q\x81\x10a\x17\xF2Wa\x17\xF2aKIV[` \x02` \x01\x01\x81\x90RPa\x18\x08\x89\x82\x8Aa\x0C1V[\x83\x83\x81Q\x81\x10a\x18\x1AWa\x18\x1AaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\xB0V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x0C_`\x99_a\x18O\x86a0\x92V[\x81R` \x01\x90\x81R` \x01_ a7\xBCV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18}Wa\x18}aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\x13gWa\x18\xD7\x85\x82\x81Q\x81\x10a\x18\xC9Wa\x18\xC9aKIV[` \x02` \x01\x01Q\x85a'^V[\x82\x82\x81Q\x81\x10a\x18\xE9Wa\x18\xE9aKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x18\xABV[`fT_\x90`\x01\x90\x81\x16\x03a\x191W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x19QW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x19\xBAWa\x19\xB2\x87\x87\x87\x84\x81\x81\x10a\x19qWa\x19qaKIV[\x90P` \x02\x01` \x81\x01\x90a\x19\x86\x91\x90aD V[\x86\x86\x85\x81\x81\x10a\x19\x98Wa\x19\x98aKIV[\x90P` \x02\x01` \x81\x01\x90a\x19\xAD\x91\x90aN\xF2V[a5\xDBV[`\x01\x01a\x19SV[PPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19\xDFWa\x19\xDFaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1A\x08W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x13gWa\x1A9\x85\x85\x83\x81Q\x81\x10a\x1A,Wa\x1A,aKIV[` \x02` \x01\x01Qa'^V[\x82\x82\x81Q\x81\x10a\x1AKWa\x1AKaKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1A\rV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1A\xB4W`@Qc\xF79X\x9B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r7\x82\x82a7\xC8V[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1BD\x91\x90aL\xFEV[a\x1BaW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Bk_\x19a5\x9EV[V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xCFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xF3\x91\x90aL\xFEV[a\x1C\x10W`@Qc%\xECl\x1F`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1C\x1A3\x82a7\xC8V[PV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1DGW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1C\xAF\x90\x83a9iV[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a\x1D*WPPa\x1DGV[a\x1D8\x85\x82` \x01Qa2\x9AV[\x94PPP\x80`\x01\x01\x90Pa\x1CwV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1Dw\x90a7\\V[a\x1D\x81\x91\x90aKqV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a\x1D\xB3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1D\xC0` \x83\x01\x83aD V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14\x80a\x1D\xFFWPa\x1D\xEA`@\x83\x01` \x84\x01aD V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x1E\x1CW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a\x1E+`@\x84\x01\x84aM;V[\x90P\x81\x10\x15a \xCFW_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a\x1ES\x91\x90aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x1En`@\x87\x01\x87aM;V[\x85\x81\x81\x10a\x1E~Wa\x1E~aKIV[\x90P` \x02\x01` \x81\x01\x90a\x1E\x93\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\x1E\xC2\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a\x0E\x8B\x91\x90aD V[a\x1E\xDFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a\x1E\xEF` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\x1F\x18\x83a0\x92V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a\x1FHW`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\x82a\x1FT\x82a0\x92V[`\x9C_a\x1Fd` \x89\x01\x89aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a9\xD8V[Pa\x1F\xBAa\x1F\x93` \x86\x01\x86aD V[`\x9A_a\x1F\x9F\x85a0\x92V[\x81R` \x01\x90\x81R` \x01_ a9\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\x1F\xC8` \x85\x01\x85aD V[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa \0\x91\x90aO\x13V[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a [` \x88\x01\x88aD V[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a \x84\x84a0\x92V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a\x1E\x1EV[Pa \xE3a\x03\x80`@\x84\x01` \x85\x01aD V[`\x01`\x01`\xA0\x1B\x03\x16c\x9D\x8E\x0C#a \xFE` \x85\x01\x85aD V[a!\x0B`@\x86\x01\x86aM;V[`@Q\x84c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a!)\x93\x92\x91\x90aOZV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a!@W__\xFD[PZ\xF1\x92PPP\x80\x15a!QWP`\x01[\x15a\r7WPPV[``a\x13\xA0`\x9A_a\x18O\x85a0\x92V[a!sa9\xF7V[a\x1Bk_a:QV[`@\x80Q\x80\x82\x01\x82R3\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x87\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a!\xB2\x92\x91a0{\x16V[a!\xCFW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a!\xD9\x82a0\x92V[\x90P_[\x83\x81\x10\x15a\x12PWa\"\"\x85\x85\x83\x81\x81\x10a!\xFAWa!\xFAaKIV[\x90P` \x02\x01` \x81\x01\x90a\"\x0F\x91\x90aD V[_\x84\x81R`\x99` R`@\x90 \x90a:\xA2V[a\"?W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x86\x86\x84\x81\x81\x10a\"sWa\"saKIV[\x90P` \x02\x01` \x81\x01\x90a\"\x88\x91\x90aD V[`@Qa\"\x96\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a!\xDDV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a\"\xCA\x90a0\xF0V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xE5Wa\"\xE5aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a#)W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a#\x03W\x90P[P\x90P_[\x82\x81\x10\x15a\x13gW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a#[\x90a\x13\r\x90\x83a0\xF9V[\x82\x82\x81Q\x81\x10a#mWa#maKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a#.V[_[\x81\x81\x10\x15a%\xA8W_`@Q\x80`@\x01`@R\x803`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x85\x85\x81\x81\x10a#\xB7Wa#\xB7aKIV[\x90P` \x02\x81\x01\x90a#\xC9\x91\x90aO\xA4V[a#\xD7\x90` \x81\x01\x90aI*V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q3_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa$\t\x92\x91\x90\x81\x16\x90a:\xB6\x16V[a$&W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R3\x81R` \x83\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R\x90Q\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l\x91a$o\x91aO\x13V[`@Q\x80\x91\x03\x90\xA1_a$\x81\x82a0\x92V[\x90P_[\x85\x85\x85\x81\x81\x10a$\x97Wa$\x97aKIV[\x90P` \x02\x81\x01\x90a$\xA9\x91\x90aO\xA4V[a$\xB7\x90` \x81\x01\x90aM;V[\x90P\x81\x10\x15a%\x9DWa%\x05\x86\x86\x86\x81\x81\x10a$\xD5Wa$\xD5aKIV[\x90P` \x02\x81\x01\x90a$\xE7\x91\x90aO\xA4V[a$\xF5\x90` \x81\x01\x90aM;V[\x83\x81\x81\x10a!\xFAWa!\xFAaKIV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x87\x81\x81\x10a%:Wa%:aKIV[\x90P` \x02\x81\x01\x90a%L\x91\x90aO\xA4V[a%Z\x90` \x81\x01\x90aM;V[\x84\x81\x81\x10a%jWa%jaKIV[\x90P` \x02\x01` \x81\x01\x90a%\x7F\x91\x90aD V[`@Qa%\x8D\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a$\x85V[PPP`\x01\x01a#\x82V[PPPV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xC9Wa%\xC9aB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\x12W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a%\xE7W\x90P[P\x90P_[\x85Q\x81\x10\x15a&iWa&D\x86\x82\x81Q\x81\x10a&5Wa&5aKIV[` \x02` \x01\x01Q\x86\x86a\x0C1V[\x82\x82\x81Q\x81\x10a&VWa&VaKIV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a&\x17V[P\x94\x93PPPPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a&\x8EWa&\x8EaB]V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a&\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a&iW`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa',\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a&\xF7Wa&\xF7aKIV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a:\xC1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a'>Wa'>aKIV[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a&\xBCV[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x0C_\x90a7\\V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x83\x83`@Qa'\xC8\x92\x91\x90aO\xB8V[`@Q\x80\x91\x03\x90\xA2PPV[_a\x13\xA0`\x9A_a\x08'\x85a0\x92V[`fT`\x02\x90`\x04\x90\x81\x16\x03a(\rW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a(oW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\x93\x91\x90aL\xFEV[a(\xB0W`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a(\xBF` \x84\x01\x84aM;V[\x90P\x81\x10\x15a*uW`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a(\xE2` \x87\x01\x87aD V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85\x80` \x01\x90a)\0\x91\x90aM;V[\x85\x81\x81\x10a)\x10Wa)\x10aKIV[\x90P` \x02\x01` \x81\x01\x90a)%\x91\x90aI*V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa)a\x92\x91\x90\x81\x16\x90a0{\x16V[a)~W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\x883\x82a0\x06V[\x15a)\xA6W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a)\xC6a)\xB2\x82a0\x92V[3_\x90\x81R`\x9C` R`@\x90 \x90a:\xB6V[Pa)\xF23`\x9A_a)\xD7\x85a0\x92V[\x81R` \x01\x90\x81R` \x01_ a:\xA2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P3`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa*,\x91\x90aO\x13V[`@Q\x80\x91\x03\x90\xA23_\x90\x81R`\x9E` R`@\x81 `\x01\x91a*N\x84a0\x92V[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a(\xB2V[Pa*\x86a\x03\x80` \x84\x01\x84aD V[`\x01`\x01`\xA0\x1B\x03\x16c\xAD\xCFs\xF73a*\xA2` \x86\x01\x86aM;V[a*\xAF`@\x88\x01\x88aL2V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a*\xCF\x95\x94\x93\x92\x91\x90aO\xCBV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a*\xE6W__\xFD[PZ\xF1\x15\x80\x15a\x12PW=__>=_\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90Re\x01\0\0\0\0\0\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a+yWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a+\x88WPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a+\xBB\x90\x82a\x18O\x86a0\x92V[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a+\xE1WP_T`\x01`\xFF\x90\x91\x16\x10[\x80a+\xFAWP0;\x15\x80\x15a+\xFAWP_T`\xFF\x16`\x01\x14[a,bW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a,\x83W_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a,\x8C\x82a5\x9EV[a,\x95\x83a:QV[\x80\x15a%\xA8W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x82R3\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x87\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a-\x15\x92\x91a0{\x16V[a-2W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a-<\x82a0\x92V[\x90P_[\x83\x81\x10\x15a\x12PWa-\x85\x85\x85\x83\x81\x81\x10a-]Wa-]aKIV[\x90P` \x02\x01` \x81\x01\x90a-r\x91\x90aD V[_\x84\x81R`\x99` R`@\x90 \x90a9\xE3V[a-\xA2W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x86\x86\x84\x81\x81\x10a-\xD6Wa-\xD6aKIV[\x90P` \x02\x01` \x81\x01\x90a-\xEB\x91\x90aD V[`@Qa-\xF9\x92\x91\x90aO~V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a-@V[3_\x81\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x90a.[\x81a\x16\x93V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PV[a.\x89a9\xF7V[`\x01`\x01`\xA0\x1B\x03\x81\x16a.\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a,YV[a\x1C\x1A\x81a:QV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a/SW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a/w\x91\x90aP\x0EV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a/\xA8W`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a/\xCFW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a'\xC8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a0)\x85a0\x92V[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a+\xBBWP` \x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x93\x92PPPV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0C_V[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a0\xD8\x92\x91\x90``\x92\x90\x92\x1B`\x01`\x01``\x1B\x03\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x13\xA0\x90aP)V[_a\x13\xA0\x82T\x90V[_a\x0C_\x83\x83a;\x11V[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q``\x81\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x80\x85\x01\x83\x90R\x84Q\x80\x86\x01\x86R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x85R`\xA1\x84R\x86\x85 \x90\x88\x16\x85R\x90\x92R\x93\x82 \x92\x93\x92\x81\x90a1d\x90a7\\V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a2\x06W\x90\x92P\x90Pa2hV[a2\x17\x81_\x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a2UWa2F\x82` \x01Q\x82` \x01Qa2\x9AV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[_a\x0C_\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01a;7V[_a\x0C_\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a;\x90V[_a\x0C_\x82`\x01`\x01`@\x1B\x03\x85\x16aK\xB4V[` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x81\x81R`\xA2\x86R`@\x80\x82 \x93\x8A\x16\x80\x83R\x93\x87R\x90\x81\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x95\x86\x16\x17\x90U\x93Q\x84Q\x91\x82R\x94\x81\x01\x91\x90\x91R\x92\x16\x90\x82\x01R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x90``\x01`@Q\x80\x91\x03\x90\xA1`\x01`\x01`\xA0\x1B\x03\x85\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x84Q\x81T\x86\x84\x01Q\x87\x87\x01Q`\x01`\x01`@\x1B\x03\x90\x93\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17`\x01`@\x1B`\x01`\x01`\x80\x1B\x03\x90\x92\x16\x91\x90\x91\x02\x17c\xFF\xFF\xFF\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1Bc\xFF\xFF\xFF\xFF\x92\x83\x16\x02\x17\x90\x91U\x83Q\x80\x85\x01\x85R\x83\x81R\x82\x01\x92\x90\x92R\x82Q\x80\x84\x01\x90\x93R``\x87\x90\x1C\x83R\x90\x86\x16\x90\x82\x01R\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x90\x86\x90\x83Q`@Qa4\x1A\x93\x92\x91\x88\x91C\x90aK\xE1V[`@Q\x80\x91\x03\x90\xA1` \x81\x01Q`\x0F\x0B\x15a4\x83W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4[\x90\x84a:\xA2V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a4}\x90\x85a:\xB6V[Pa5\x13V[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a5\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4\xC0\x90\x84a9\xE3V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a4\xEC\x90a0\xF0V[_\x03a5\x13W`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\x12P\x90\x85a9\xD8V[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a5I\x90C\x83a<uV[PP`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90``\x01a,\xD2V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a6%WP\x82a\xFF\xFF\x16\x82\x10[\x15a5\x13W`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a6Y\x90a<\x8EV[\x90P__a6h3\x84\x89a1\x04V[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a6\x86WPPPa5\x13V[a6\x93\x88\x84\x89\x85\x85a2\xAEV[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a6\xC0\x90a<\xE0V[Pa6\xCA\x85aPLV[\x94Pa6\xD5\x84aPdV[\x93PPPPa6\x13V[_a7\x0B\x84`\x99_a6\xF0\x89a0\x92V[\x81R` \x01\x90\x81R` \x01_ a=]\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a7\x16WP_a+\xBBV[\x81a7\"WP_a+\xBBV[\x82Q`\x01`\x01`@\x1B\x03\x16_\x03a7:WP_a+\xBBV[P`\x01\x94\x93PPPPV[_a\x0C_`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aPyV[\x80T_\x90\x80\x15a7\x98Wa7\x82\x83a7u`\x01\x84aP\xA6V[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x0C_V[g\r\xE0\xB6\xB3\xA7d\0\0\x93\x92PPPV[_a\x0C_\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a;\x90V[``_a\x0C_\x83a=~V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a8DWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a8^W`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra8\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaM\xA6V[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a,\xD2V[__a9\x8Ba9w\x84a=\xD7V[\x85Ta9\x86\x91\x90`\x0F\x0BaP\xB9V[a>DV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a9\xBEW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_a\x0C_\x83\x83a>\xADV[_a\x0C_\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a>\xADV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a,YV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x0C_\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a?\x90V[_a\x0C_\x83\x83a?\x90V[\x81T_\x90\x81a:\xD2\x85\x85\x83\x85a?\xDCV[\x90P\x80\x15a:\xFFWa:\xE9\x85a7u`\x01\x84aP\xA6V[T`\x01` \x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a\x1D\x81V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x94\x93PPPPV[_\x82_\x01\x82\x81T\x81\x10a;&Wa;&aKIV[\x90_R` _ \x01T\x90P\x92\x91PPV[__a;D\x86\x86\x86a;\x90V[\x90P`\x01\x83`\x02\x81\x11\x15a;ZWa;ZaP\xE0V[\x14\x80\x15a;vWP_\x84\x80a;qWa;qaP\xF4V[\x86\x88\t\x11[\x15a\x1D\x81Wa;\x86`\x01\x82aQ\x08V[\x96\x95PPPPPPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03a;\xC7W\x83\x82\x81a;\xBDWa;\xBDaP\xF4V[\x04\x92PPPa\x0C_V[\x80\x84\x11a<\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a,YV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[_\x80a<\x82\x85\x85\x85a@/V[\x91P\x91P\x93P\x93\x91PPV[_a<\xA8\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a<\xC6W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_a<\xFA\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15a=\x18W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x0C_V[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a=\xCBW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a=\xB7W[PPPPP\x90P\x91\x90PV[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15a>@W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a,YV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14a>\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a,YV[\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15a?\x87W_a>\xCF`\x01\x83aP\xA6V[\x85T\x90\x91P_\x90a>\xE2\x90`\x01\x90aP\xA6V[\x90P\x81\x81\x14a?AW_\x86_\x01\x82\x81T\x81\x10a?\0Wa?\0aKIV[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10a? Wa? aKIV[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80a?RWa?RaQ\x1BV[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\x13\xA0V[_\x91PPa\x13\xA0V[_\x81\x81R`\x01\x83\x01` R`@\x81 Ta?\xD5WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\x13\xA0V[P_a\x13\xA0V[_[\x81\x83\x10\x15a\x13gW_a?\xF1\x84\x84aA\xF9V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a@\x1BW\x80\x92Pa@)V[a@&\x81`\x01aQ\x08V[\x93P[Pa?\xDEV[\x82T_\x90\x81\x90\x80\x15aA\x8DW_a@K\x87a7u`\x01\x85aP\xA6V[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x87\x16\x10\x15a@\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x19`$\x82\x01R\x7FSnapshot: decreasing keys\0\0\0\0\0\0\0`D\x82\x01R`d\x01a,YV[\x80Qc\xFF\xFF\xFF\xFF\x80\x88\x16\x91\x16\x03aA\x1DW\x84a@\xED\x88a7u`\x01\x86aP\xA6V[\x80T`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UaA}V[`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x88\x16\x82R`\x01`\x01`@\x1B\x03\x80\x88\x16` \x80\x85\x01\x91\x82R\x8BT`\x01\x81\x01\x8DU_\x8D\x81R\x91\x90\x91 \x94Q\x94\x01\x80T\x91Q\x90\x92\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x91\x16\x93\x90\x92\x16\x92\x90\x92\x17\x17\x90U[` \x01Q\x92P\x83\x91Pa2h\x90PV[PP`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x80\x85\x16\x82R`\x01`\x01`@\x1B\x03\x80\x85\x16` \x80\x85\x01\x91\x82R\x88T`\x01\x81\x01\x8AU_\x8A\x81R\x91\x82 \x95Q\x95\x01\x80T\x92Q\x90\x93\x16`\x01` \x1B\x02`\x01`\x01``\x1B\x03\x19\x90\x92\x16\x94\x90\x93\x16\x93\x90\x93\x17\x92\x90\x92\x17\x90\x91U\x90P\x81a2hV[_aB\x07`\x02\x84\x84\x18aQ/V[a\x0C_\x90\x84\x84\x16aQ\x08V[_` \x82\x84\x03\x12\x15aB#W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB8W__\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x0C_W__\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1C\x1AW__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\x99WaB\x99aB]V[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a>\xA8W__\xFD[_`@\x82\x84\x03\x12\x15aB\xC4W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\xE6WaB\xE6aB]V[`@R\x90P\x80\x825aB\xF7\x81aBIV[\x81RaC\x05` \x84\x01aB\xA1V[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aC#W__\xFD[\x835aC.\x81aBIV[\x92PaC=\x85` \x86\x01aB\xB4V[\x91P``\x84\x015aCM\x81aBIV[\x80\x91PP\x92P\x92P\x92V[\x81Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\x13\xA0V[_` \x82\x84\x03\x12\x15aC\x9BW__\xFD[P5\x91\x90PV[__\x83`\x1F\x84\x01\x12aC\xB2W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aC\xC8W__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x186W__\xFD[__` \x83\x85\x03\x12\x15aC\xF3W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aD\x08W__\xFD[aD\x14\x85\x82\x86\x01aC\xA2V[\x90\x96\x90\x95P\x93PPPPV[_` \x82\x84\x03\x12\x15aD0W__\xFD[\x815a\x0C_\x81aBIV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94WaD~\x86\x83QaD;V[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aDkV[P\x93\x94\x93PPPPV[` \x81R_a\x0C_` \x83\x01\x84aDYV[_`@\x82\x84\x03\x12\x15aD\xC0W__\xFD[a\x0C_\x83\x83aB\xB4V[_`\x01`\x01`@\x1B\x03\x82\x11\x15aD\xE2WaD\xE2aB]V[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aD\xFBW__\xFD[\x815aE\x0EaE\t\x82aD\xCAV[aBqV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aE/W__\xFD[` \x85\x01[\x83\x81\x10\x15aEUW\x805aEG\x81aBIV[\x83R` \x92\x83\x01\x92\x01aE4V[P\x95\x94PPPPPV[____`\xA0\x85\x87\x03\x12\x15aErW__\xFD[aE|\x86\x86aB\xB4V[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\x96W__\xFD[aE\xA2\x87\x82\x88\x01aD\xECV[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aE\xBDW__\xFD[aE\xC9\x87\x82\x88\x01aD\xECV[\x92PPaE\xD8`\x80\x86\x01aB\xA1V[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aE\xF5V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aFjW`?\x19\x87\x86\x03\x01\x84RaFU\x85\x83QaE\xE3V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF9V[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aF\x87W__\xFD[\x825aF\x92\x81aBIV[\x91P` \x83\x015aF\xA2\x81aBIV[\x80\x91PP\x92P\x92\x90PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94WaF\xF8\x86\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aF\xBFV[`@\x81R_aG `@\x83\x01\x85aDYV[\x82\x81\x03` \x84\x01Ra\x1D\x81\x81\x85aF\xADV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aD\x94W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aGDV[` \x81R_a\x0C_` \x83\x01\x84aG2V[__`@\x83\x85\x03\x12\x15aG\x8EW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG\xA3W__\xFD[aG\xAF\x85\x82\x86\x01aD\xECV[\x92PP` \x83\x015aF\xA2\x81aBIV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aH\0W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aG\xD9V[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aH\x1FW__\xFD[\x855aH*\x81aBIV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHDW__\xFD[aHP\x88\x82\x89\x01aC\xA2V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aHnW__\xFD[aHz\x88\x82\x89\x01aC\xA2V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__`@\x83\x85\x03\x12\x15aH\x9CW__\xFD[\x825aH\xA7\x81aBIV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xC1W__\xFD[aH\xCD\x85\x82\x86\x01aD\xECV[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aH\xE8W__\xFD[\x825aH\xF3\x81aBIV[\x91PaI\x01` \x84\x01aB\xA1V[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aI\x1AW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x0C_W__\xFD[_` \x82\x84\x03\x12\x15aI:W__\xFD[a\x0C_\x82aB\xA1V[_``\x82\x84\x03\x12\x15aISW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aIiW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI~W__\xFD[a+\xBB\x84\x82\x85\x01aICV[___`@\x84\x86\x03\x12\x15aI\x9CW__\xFD[aI\xA5\x84aB\xA1V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xBFW__\xFD[aI\xCB\x86\x82\x87\x01aC\xA2V[\x94\x97\x90\x96P\x93\x94PPPPV[___`\x80\x84\x86\x03\x12\x15aI\xEAW__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xFFW__\xFD[aJ\x0B\x86\x82\x87\x01aD\xECV[\x93PPaC=\x85` \x86\x01aB\xB4V[` \x81R_a\x0C_` \x83\x01\x84aF\xADV[___``\x84\x86\x03\x12\x15aJ?W__\xFD[\x835aJJ\x81aBIV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJdW__\xFD[aJp\x86\x82\x87\x01aD\xECV[\x92PPaJ\x7F`@\x85\x01aB\xA1V[\x90P\x92P\x92P\x92V[__` \x83\x85\x03\x12\x15aJ\x99W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xAEW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aJ\xBEW__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD3W__\xFD[\x85` \x82\x84\x01\x01\x11\x15aJ\xE4W__\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[__``\x83\x85\x03\x12\x15aK\x05W__\xFD[\x825aK\x10\x81aBIV[\x91PaI\x01\x84` \x85\x01aB\xB4V[__`@\x83\x85\x03\x12\x15aK0W__\xFD[\x825aK;\x81aBIV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aK\xACWaK\xACaK]V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\x13\xA0Wa\x13\xA0aK]V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aK\xFE` \x83\x01\x87aD;V[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aLGW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL`W__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x186W__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x87\x16\x81RaL\xB5` \x82\x01\x87aD;V[`\xC0``\x82\x01R_aL\xCA`\xC0\x83\x01\x87aG2V[\x82\x81\x03`\x80\x84\x01RaL\xDC\x81\x87aE\xE3V[\x90P\x82\x81\x03`\xA0\x84\x01RaL\xF1\x81\x85\x87aLtV[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aM\x0EW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x0C_W__\xFD[_\x825`~\x19\x836\x03\x01\x81\x12aM1W__\xFD[\x91\x90\x91\x01\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aMPW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aMiW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x186W__\xFD[_` \x82\x84\x03\x12\x15aM\x90W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x0C_W__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[`@\x81R_aM\xD4`@\x83\x01\x85aG2V[\x82\x81\x03` \x84\x01Ra\x1D\x81\x81\x85aG2V[_` \x82\x84\x03\x12\x15aM\xF6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x0BW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aN\x1BW__\xFD[\x80QaN)aE\t\x82aD\xCAV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aNJW__\xFD[` \x84\x01[\x83\x81\x10\x15aN\xE7W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aNlW__\xFD[\x85\x01`?\x81\x01\x89\x13aN|W__\xFD[` \x81\x01QaN\x8DaE\t\x82aD\xCAV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aN\xB0W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aN\xD2W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aN\xB7V[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaNOV[P\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aO\x02W__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x0C_W__\xFD[`@\x81\x01a\x13\xA0\x82\x84aD;V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aD\x94Wc\xFF\xFF\xFF\xFFaOD\x83aB\xA1V[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aO.V[`\x01`\x01`\xA0\x1B\x03\x84\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x1D\x81\x90\x83\x01\x84\x86aO!V[``\x81\x01aO\x8C\x82\x85aD;V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[_\x825`>\x19\x836\x03\x01\x81\x12aM1W__\xFD[` \x81R_a+\xBB` \x83\x01\x84\x86aLtV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R``` \x82\x01\x81\x90R_\x90aO\xEF\x90\x83\x01\x86\x88aO!V[\x82\x81\x03`@\x84\x01RaP\x02\x81\x85\x87aLtV[\x98\x97PPPPPPPPV[_` \x82\x84\x03\x12\x15aP\x1EW__\xFD[\x81Qa\x0C_\x81aBIV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aISW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01aP]WaP]aK]V[P`\x01\x01\x90V[_\x81aPrWaPraK]V[P_\x19\x01\x90V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\x13\xA0Wa\x13\xA0aK]V[\x81\x81\x03\x81\x81\x11\x15a\x13\xA0Wa\x13\xA0aK]V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aP\xD8WaP\xD8aK]V[PP\x92\x91PPV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\x13\xA0Wa\x13\xA0aK]V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82aQIWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \x9Dx'\x0E\x88\xD1Pz\x8A\x85-\xF3xf\x1E\xFDs\xB7\xF8N6-G\xEE\xB6\xC9\xA8\xDA{\x9A\xE30dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
    struct OperatorSet { address avs; uint32 id; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        pub avs: alloy::sol_types::private::Address,
        pub id: u32,
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
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    id: tuple.1,
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
                        &self.id,
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
                alloy_sol_types::private::Cow::Borrowed("OperatorSet(address avs,uint32 id)")
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.id)
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
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.id)
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
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
            }
            #[inline]
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**Custom error with signature `AlreadyMemberOfSet()` and selector `0xd8d8dc4e`.
    ```solidity
    error AlreadyMemberOfSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AlreadyMemberOfSet {}
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
        impl ::core::convert::From<AlreadyMemberOfSet> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyMemberOfSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyMemberOfSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyMemberOfSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyMemberOfSet()";
            const SELECTOR: [u8; 4] = [216u8, 216u8, 220u8, 78u8];
            #[inline]
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
    /**Custom error with signature `InsufficientMagnitude()` and selector `0x6c9be0bf`.
    ```solidity
    error InsufficientMagnitude();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientMagnitude {}
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
        impl ::core::convert::From<InsufficientMagnitude> for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientMagnitude) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InsufficientMagnitude {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientMagnitude {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientMagnitude()";
            const SELECTOR: [u8; 4] = [108u8, 155u8, 224u8, 191u8];
            #[inline]
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
    /**Custom error with signature `InvalidBlockNumber()` and selector `0x4e47846c`.
    ```solidity
    error InvalidBlockNumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBlockNumber {}
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
        impl ::core::convert::From<InvalidBlockNumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBlockNumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBlockNumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBlockNumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBlockNumber()";
            const SELECTOR: [u8; 4] = [78u8, 71u8, 132u8, 108u8];
            #[inline]
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
    /**Custom error with signature `InvalidCaller()` and selector `0x48f5c3ed`.
    ```solidity
    error InvalidCaller();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidCaller {}
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
        impl ::core::convert::From<InvalidCaller> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCaller()";
            const SELECTOR: [u8; 4] = [72u8, 245u8, 195u8, 237u8];
            #[inline]
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
    /**Custom error with signature `InvalidStrategy()` and selector `0x4e236e9a`.
    ```solidity
    error InvalidStrategy();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidStrategy {}
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
        impl ::core::convert::From<InvalidStrategy> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidStrategy) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidStrategy {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidStrategy {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidStrategy()";
            const SELECTOR: [u8; 4] = [78u8, 35u8, 110u8, 154u8];
            #[inline]
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
    /**Custom error with signature `NotMemberOfSet()` and selector `0x25131d4f`.
    ```solidity
    error NotMemberOfSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NotMemberOfSet {}
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
        impl ::core::convert::From<NotMemberOfSet> for UnderlyingRustTuple<'_> {
            fn from(value: NotMemberOfSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NotMemberOfSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NotMemberOfSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NotMemberOfSet()";
            const SELECTOR: [u8; 4] = [37u8, 19u8, 29u8, 79u8];
            #[inline]
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
    /**Custom error with signature `OperatorNotSlashable()` and selector `0xebbff497`.
    ```solidity
    error OperatorNotSlashable();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotSlashable {}
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
        impl ::core::convert::From<OperatorNotSlashable> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotSlashable) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotSlashable {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotSlashable {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotSlashable()";
            const SELECTOR: [u8; 4] = [235u8, 191u8, 244u8, 151u8];
            #[inline]
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
    /**Custom error with signature `StrategyAlreadyInOperatorSet()` and selector `0x585cfb2f`.
    ```solidity
    error StrategyAlreadyInOperatorSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyAlreadyInOperatorSet {}
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
        impl ::core::convert::From<StrategyAlreadyInOperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAlreadyInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyAlreadyInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyAlreadyInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategyAlreadyInOperatorSet()";
            const SELECTOR: [u8; 4] = [88u8, 92u8, 251u8, 47u8];
            #[inline]
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
    /**Custom error with signature `StrategyNotInOperatorSet()` and selector `0x6378684e`.
    ```solidity
    error StrategyNotInOperatorSet();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyNotInOperatorSet {}
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
        impl ::core::convert::From<StrategyNotInOperatorSet> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyNotInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyNotInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyNotInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategyNotInOperatorSet()";
            const SELECTOR: [u8; 4] = [99u8, 120u8, 104u8, 78u8];
            #[inline]
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
    /**Event with signature `AVSMetadataURIUpdated(address,string)` and selector `0xa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c943713`.
    ```solidity
    event AVSMetadataURIUpdated(address indexed avs, string metadataURI);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSMetadataURIUpdated {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSMetadataURIUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AVSMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    168u8, 156u8, 29u8, 194u8, 67u8, 216u8, 144u8, 138u8, 150u8, 221u8, 132u8,
                    148u8, 75u8, 204u8, 151u8, 214u8, 188u8, 106u8, 192u8, 13u8, 215u8, 142u8,
                    32u8, 98u8, 21u8, 118u8, 190u8, 106u8, 60u8, 148u8, 55u8, 19u8,
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
                    metadataURI: data.0,
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.metadataURI,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.avs.clone())
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
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for AVSMetadataURIUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSMetadataURIUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSMetadataURIUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AVSRegistrarSet(address,address)` and selector `0x2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85`.
    ```solidity
    event AVSRegistrarSet(address avs, address registrar);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSRegistrarSet {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registrar: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSRegistrarSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AVSRegistrarSet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    42u8, 233u8, 69u8, 196u8, 12u8, 68u8, 220u8, 14u8, 194u8, 99u8, 249u8, 86u8,
                    9u8, 195u8, 253u8, 198u8, 149u8, 46u8, 10u8, 239u8, 162u8, 45u8, 99u8, 116u8,
                    228u8, 79u8, 44u8, 153u8, 122u8, 206u8, 223u8, 133u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    avs: data.0,
                    registrar: data.1,
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.registrar,
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
        impl alloy_sol_types::private::IntoLogData for AVSRegistrarSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSRegistrarSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AVSRegistrarSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `AllocationDelaySet(address,uint32,uint32)` and selector `0x4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db`.
    ```solidity
    event AllocationDelaySet(address operator, uint32 delay, uint32 effectBlock);
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
        pub effectBlock: u32,
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
                    effectBlock: data.2,
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
                        &self.effectBlock,
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
    /**Event with signature `AllocationUpdated(address,(address,uint32),address,uint64,uint32)` and selector `0x1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd`.
    ```solidity
    event AllocationUpdated(address operator, OperatorSet operatorSet, address strategy, uint64 magnitude, uint32 effectBlock);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AllocationUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub magnitude: u64,
        #[allow(missing_docs)]
        pub effectBlock: u32,
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
        impl alloy_sol_types::SolEvent for AllocationUpdated {
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
                "AllocationUpdated(address,(address,uint32),address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    20u8, 135u8, 175u8, 84u8, 24u8, 196u8, 126u8, 229u8, 234u8, 69u8, 239u8, 74u8,
                    147u8, 57u8, 134u8, 104u8, 18u8, 8u8, 144u8, 119u8, 74u8, 158u8, 19u8, 72u8,
                    126u8, 97u8, 233u8, 220u8, 59u8, 175u8, 118u8, 221u8,
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
                    effectBlock: data.4,
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
                        &self.effectBlock,
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
        impl alloy_sol_types::private::IntoLogData for AllocationUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AllocationUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &AllocationUpdated) -> alloy_sol_types::private::LogData {
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
                    maxMagnitude: data.2,
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
                        &self.maxMagnitude,
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
    /**Event with signature `OperatorAddedToOperatorSet(address,(address,uint32))` and selector `0x43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e`.
    ```solidity
    event OperatorAddedToOperatorSet(address indexed operator, OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorAddedToOperatorSet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorAddedToOperatorSet {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorAddedToOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    67u8, 35u8, 46u8, 223u8, 144u8, 113u8, 117u8, 61u8, 35u8, 33u8, 229u8, 250u8,
                    126u8, 1u8, 131u8, 99u8, 238u8, 36u8, 142u8, 95u8, 33u8, 66u8, 230u8, 192u8,
                    142u8, 221u8, 50u8, 101u8, 191u8, 180u8, 137u8, 94u8,
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
                    operatorSet: data.0,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorAddedToOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorAddedToOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorAddedToOperatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorRemovedFromOperatorSet(address,(address,uint32))` and selector `0xad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe`.
    ```solidity
    event OperatorRemovedFromOperatorSet(address indexed operator, OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorRemovedFromOperatorSet {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorRemovedFromOperatorSet {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str =
                "OperatorRemovedFromOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    173u8, 52u8, 195u8, 7u8, 11u8, 225u8, 223u8, 251u8, 202u8, 164u8, 153u8, 208u8,
                    0u8, 186u8, 43u8, 141u8, 152u8, 72u8, 174u8, 252u8, 172u8, 48u8, 89u8, 223u8,
                    36u8, 93u8, 217u8, 92u8, 78u8, 206u8, 20u8, 254u8,
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
                    operatorSet: data.0,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorRemovedFromOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRemovedFromOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorRemovedFromOperatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSetCreated((address,uint32))` and selector `0x31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c`.
    ```solidity
    event OperatorSetCreated(OperatorSet operatorSet);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorSetCreated {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorSetCreated {
            type DataTuple<'a> = (OperatorSet,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetCreated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 98u8, 146u8, 133u8, 234u8, 210u8, 51u8, 90u8, 224u8, 147u8, 63u8, 134u8,
                    237u8, 42u8, 230u8, 51u8, 33u8, 247u8, 175u8, 119u8, 180u8, 230u8, 234u8,
                    171u8, 196u8, 44u8, 5u8, 120u8, 128u8, 151u8, 126u8, 108u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
                ),)
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
        impl alloy_sol_types::private::IntoLogData for OperatorSetCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSetCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSetCreated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `StrategyAddedToOperatorSet((address,uint32),address)` and selector `0x7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b`.
    ```solidity
    event StrategyAddedToOperatorSet(OperatorSet operatorSet, address strategy);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyAddedToOperatorSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyAddedToOperatorSet {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StrategyAddedToOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    122u8, 178u8, 96u8, 254u8, 10u8, 241u8, 147u8, 219u8, 95u8, 73u8, 134u8, 119u8,
                    13u8, 131u8, 27u8, 218u8, 78u8, 164u8, 96u8, 153u8, 220u8, 129u8, 126u8, 139u8,
                    103u8, 22u8, 220u8, 174u8, 138u8, 248u8, 232u8, 139u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    strategy: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
        impl alloy_sol_types::private::IntoLogData for StrategyAddedToOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyAddedToOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyAddedToOperatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyRemovedFromOperatorSet((address,uint32),address)` and selector `0x7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee`.
    ```solidity
    event StrategyRemovedFromOperatorSet(OperatorSet operatorSet, address strategy);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyRemovedFromOperatorSet {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for StrategyRemovedFromOperatorSet {
            type DataTuple<'a> = (OperatorSet, alloy::sol_types::sol_data::Address);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str =
                "StrategyRemovedFromOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    123u8, 75u8, 7u8, 61u8, 128u8, 220u8, 172u8, 85u8, 161u8, 17u8, 119u8, 216u8,
                    69u8, 154u8, 217u8, 246u8, 100u8, 206u8, 235u8, 145u8, 247u8, 31u8, 39u8, 22u8,
                    123u8, 177u8, 79u8, 129u8, 82u8, 167u8, 238u8, 238u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorSet: data.0,
                    strategy: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
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
        impl alloy_sol_types::private::IntoLogData for StrategyRemovedFromOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyRemovedFromOperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyRemovedFromOperatorSet) -> alloy_sol_types::private::LogData {
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
    constructor(address _delegation, address _pauserRegistry, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegation: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
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
                        value._pauserRegistry,
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
                        _pauserRegistry: tuple.1,
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
                        &self._pauserRegistry,
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
    /**Function with signature `addStrategiesToOperatorSet(uint32,address[])` and selector `0x76999342`.
    ```solidity
    function addStrategiesToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`addStrategiesToOperatorSet(uint32,address[])`](addStrategiesToOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<addStrategiesToOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            impl ::core::convert::From<addStrategiesToOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesToOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategiesToOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategiesToOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [118u8, 153u8, 147u8, 66u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
    /**Function with signature `createOperatorSets((uint32,address[])[])` and selector `0x847d634f`.
    ```solidity
    function createOperatorSets(IAllocationManagerTypes.CreateSetParams[] memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSets((uint32,address[])[])`](createOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsReturn {}
    #[allow(
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
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<createOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createOperatorSetsCall {
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
            impl ::core::convert::From<createOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets((uint32,address[])[])";
            const SELECTOR: [u8; 4] = [132u8, 125u8, 99u8, 79u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.params
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
    /**Function with signature `deregisterFromOperatorSets((address,address,uint32[]))` and selector `0x6e3492b5`.
    ```solidity
    function deregisterFromOperatorSets(IAllocationManagerTypes.DeregisterParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterFromOperatorSetsCall {
        pub params:
            <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`deregisterFromOperatorSets((address,address,uint32[]))`](deregisterFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterFromOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::DeregisterParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<deregisterFromOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterFromOperatorSetsCall {
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
            impl ::core::convert::From<deregisterFromOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterFromOperatorSetsCall {
            type Parameters<'a> = (IAllocationManagerTypes::DeregisterParams,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "deregisterFromOperatorSets((address,address,uint32[]))";
            const SELECTOR: [u8; 4] = [110u8, 52u8, 146u8, 181u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::DeregisterParams as alloy_sol_types::SolType>::tokenize(
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
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for encumberedMagnitudeCall {
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
    /**Function with signature `getAVSRegistrar(address)` and selector `0x304c10cd`.
    ```solidity
    function getAVSRegistrar(address avs) external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAVSRegistrarCall {
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAVSRegistrar(address)`](getAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAVSRegistrarReturn {
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
            impl ::core::convert::From<getAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAVSRegistrarCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { avs: tuple.0 }
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
            impl ::core::convert::From<getAVSRegistrarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAVSRegistrarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAVSRegistrarReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [48u8, 76u8, 16u8, 205u8];
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
    /**Function with signature `getAllocatedSets(address)` and selector `0x15fe5028`.
    ```solidity
    function getAllocatedSets(address operator) external view returns (OperatorSet[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedSetsCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatedSets(address)`](getAllocatedSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedSetsReturn {
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            impl ::core::convert::From<getAllocatedSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatedSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getAllocatedSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatedSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatedSets(address)";
            const SELECTOR: [u8; 4] = [21u8, 254u8, 80u8, 40u8];
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
    /**Function with signature `getAllocatedStrategies(address,(address,uint32))` and selector `0xc221d8ae`.
    ```solidity
    function getAllocatedStrategies(address operator, OperatorSet memory operatorSet) external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStrategiesCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getAllocatedStrategies(address,(address,uint32))`](getAllocatedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStrategiesReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatedStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                    }
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
            impl ::core::convert::From<getAllocatedStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocatedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedStrategiesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocatedStrategies(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [194u8, 33u8, 216u8, 174u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
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
    /**Function with signature `getAllocation(address,(address,uint32),address)` and selector `0x10e1b9b8`.
    ```solidity
    function getAllocation(address operator, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocation(address,(address,uint32),address)`](getAllocationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationReturn {
        pub _0: <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationCall) -> Self {
                    (value.operator, value.operatorSet, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSet: tuple.1,
                        strategy: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::Allocation,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAllocationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationReturn;
            type ReturnTuple<'a> = (IAllocationManagerTypes::Allocation,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocation(address,(address,uint32),address)";
            const SELECTOR: [u8; 4] = [16u8, 225u8, 185u8, 184u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),
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
    function getAllocationDelay(address operator) external view returns (bool, uint32);
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
        pub _0: bool,
        pub _1: u32,
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
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
    /**Function with signature `getAllocations(address[],(address,uint32),address)` and selector `0x8ce64854`.
    ```solidity
    function getAllocations(address[] memory operators, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsCall {
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocations(address[],(address,uint32),address)`](getAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationsCall) -> Self {
                    (value.operators, value.operatorSet, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        operatorSet: tuple.1,
                        strategy: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                OperatorSet,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getAllocations(address[],(address,uint32),address)";
            const SELECTOR: [u8; 4] = [140u8, 230u8, 72u8, 84u8];
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
    /**Function with signature `getMaxMagnitude(address,address)` and selector `0xa9333ec8`.
    ```solidity
    function getMaxMagnitude(address operator, address strategy) external view returns (uint64);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudeCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitude(address,address)`](getMaxMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudeReturn {
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
            impl ::core::convert::From<getMaxMagnitudeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudeCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudeCall {
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
            impl ::core::convert::From<getMaxMagnitudeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitude(address,address)";
            const SELECTOR: [u8; 4] = [169u8, 51u8, 62u8, 200u8];
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
    /**Function with signature `getMaxMagnitudes(address[],address)` and selector `0x4a10ffe5`.
    ```solidity
    function getMaxMagnitudes(address[] memory operators, address strategy) external view returns (uint64[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_0Call {
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address[],address)`](getMaxMagnitudes_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_0Return {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<getMaxMagnitudes_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Call) -> Self {
                    (value.operators, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudes_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        strategy: tuple.1,
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
            impl ::core::convert::From<getMaxMagnitudes_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudes_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudes_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_0Return;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudes(address[],address)";
            const SELECTOR: [u8; 4] = [74u8, 16u8, 255u8, 229u8];
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
    pub struct getMaxMagnitudes_1Call {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address,address[])`](getMaxMagnitudes_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_1Return {
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
            impl ::core::convert::From<getMaxMagnitudes_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Call) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudes_1Call {
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
            impl ::core::convert::From<getMaxMagnitudes_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudes_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudes_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_1Return;
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
    /**Function with signature `getMaxMagnitudesAtBlock(address,address[],uint32)` and selector `0x94d7d00c`.
    ```solidity
    function getMaxMagnitudesAtBlock(address operator, address[] memory strategies, uint32 blockNumber) external view returns (uint64[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudesAtBlock(address,address[],uint32)`](getMaxMagnitudesAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockReturn {
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
            impl ::core::convert::From<getMaxMagnitudesAtBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockCall) -> Self {
                    (value.operator, value.strategies, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesAtBlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        strategies: tuple.1,
                        blockNumber: tuple.2,
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
            impl ::core::convert::From<getMaxMagnitudesAtBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMaxMagnitudesAtBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMaxMagnitudesAtBlockCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesAtBlockReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMaxMagnitudesAtBlock(address,address[],uint32)";
            const SELECTOR: [u8; 4] = [148u8, 215u8, 208u8, 12u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getMemberCount((address,uint32))` and selector `0xb2447af7`.
    ```solidity
    function getMemberCount(OperatorSet memory operatorSet) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMemberCountCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMemberCount((address,uint32))`](getMemberCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMemberCountReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMemberCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMemberCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<getMemberCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMemberCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMemberCountCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMemberCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMemberCount((address,uint32))";
            const SELECTOR: [u8; 4] = [178u8, 68u8, 122u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
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
    /**Function with signature `getMembers((address,uint32))` and selector `0x6e875dba`.
    ```solidity
    function getMembers(OperatorSet memory operatorSet) external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMembersCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMembers((address,uint32))`](getMembersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMembersReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMembersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMembersCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMembersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
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
            impl ::core::convert::From<getMembersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMembersReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMembersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMembersCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMembersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getMembers((address,uint32))";
            const SELECTOR: [u8; 4] = [110u8, 135u8, 93u8, 186u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
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
    /**Function with signature `getMinimumSlashableStake((address,uint32),address[],address[],uint32)` and selector `0x2bab2c4a`.
    ```solidity
    function getMinimumSlashableStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 futureBlock) external view returns (uint256[][] memory slashableStake);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        pub futureBlock: u32,
    }
    ///Container type for the return parameters of the [`getMinimumSlashableStake((address,uint32),address[],address[],uint32)`](getMinimumSlashableStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeReturn {
        pub slashableStake: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getMinimumSlashableStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumSlashableStakeCall) -> Self {
                    (
                        value.operatorSet,
                        value.operators,
                        value.strategies,
                        value.futureBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMinimumSlashableStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        operators: tuple.1,
                        strategies: tuple.2,
                        futureBlock: tuple.3,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMinimumSlashableStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumSlashableStakeReturn) -> Self {
                    (value.slashableStake,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMinimumSlashableStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        slashableStake: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMinimumSlashableStakeCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinimumSlashableStakeReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getMinimumSlashableStake((address,uint32),address[],address[],uint32)";
            const SELECTOR: [u8; 4] = [43u8, 171u8, 44u8, 74u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.futureBlock),
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
    /**Function with signature `getRegisteredSets(address)` and selector `0x79ae50cd`.
    ```solidity
    function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getRegisteredSets(address)`](getRegisteredSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsReturn {
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
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
            impl ::core::convert::From<getRegisteredSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegisteredSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getRegisteredSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getRegisteredSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRegisteredSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRegisteredSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getRegisteredSets(address)";
            const SELECTOR: [u8; 4] = [121u8, 174u8, 80u8, 205u8];
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
    /**Function with signature `getStrategiesInOperatorSet((address,uint32))` and selector `0x4177a87c`.
    ```solidity
    function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategiesInOperatorSetCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getStrategiesInOperatorSet((address,uint32))`](getStrategiesInOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategiesInOperatorSetReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStrategiesInOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStrategiesInOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                    }
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
            impl ::core::convert::From<getStrategiesInOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStrategiesInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategiesInOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategiesInOperatorSetReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStrategiesInOperatorSet((address,uint32))";
            const SELECTOR: [u8; 4] = [65u8, 119u8, 168u8, 124u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
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
    /**Function with signature `getStrategyAllocations(address,address)` and selector `0x40120dab`.
    ```solidity
    function getStrategyAllocations(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.Allocation[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategyAllocationsCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getStrategyAllocations(address,address)`](getStrategyAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategyAllocationsReturn {
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
        pub _1: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategyAllocationsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStrategyAllocationsCall {
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
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <OperatorSet as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategyAllocationsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStrategyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategyAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategyAllocationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStrategyAllocations(address,address)";
            const SELECTOR: [u8; 4] = [64u8, 18u8, 13u8, 171u8];
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `isOperatorSet((address,uint32))` and selector `0x260dc758`.
    ```solidity
    function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isOperatorSet((address,uint32))`](isOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetReturn {
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
            type UnderlyingSolTuple<'a> = (OperatorSet,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<OperatorSet as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
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
            impl ::core::convert::From<isOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSet((address,uint32))";
            const SELECTOR: [u8; 4] = [38u8, 13u8, 199u8, 88u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(
                    &self.operatorSet,
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
    /**Function with signature `modifyAllocations(((address,uint32),address[],uint64[])[])` and selector `0x15ea7917`.
    ```solidity
    function modifyAllocations(IAllocationManagerTypes.AllocateParams[] memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsCall {
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`modifyAllocations(((address,uint32),address[],uint64[])[])`](modifyAllocationsCall) function.
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
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::AllocateParams>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
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
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyAllocationsCall {
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
                (alloy::sol_types::sol_data::Array<IAllocationManagerTypes::AllocateParams>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyAllocationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "modifyAllocations(((address,uint32),address[],uint64[])[])";
            const SELECTOR: [u8; 4] = [21u8, 234u8, 121u8, 23u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::AllocateParams,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.params
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
    /**Function with signature `registerForOperatorSets((address,uint32[],bytes))` and selector `0xb92f60a5`.
    ```solidity
    function registerForOperatorSets(IAllocationManagerTypes.RegisterParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsCall {
        pub params:
            <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerForOperatorSets((address,uint32[],bytes))`](registerForOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IAllocationManagerTypes::RegisterParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerForOperatorSetsCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerForOperatorSetsCall {
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
            impl ::core::convert::From<registerForOperatorSetsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerForOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerForOperatorSetsCall {
            type Parameters<'a> = (IAllocationManagerTypes::RegisterParams,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerForOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerForOperatorSets((address,uint32[],bytes))";
            const SELECTOR: [u8; 4] = [185u8, 47u8, 96u8, 165u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAllocationManagerTypes::RegisterParams as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `removeStrategiesFromOperatorSet(uint32,address[])` and selector `0xce7b5e4b`.
    ```solidity
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u32,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesFromOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesFromOperatorSetCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [206u8, 123u8, 94u8, 75u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
    /**Function with signature `setAVSRegistrar(address)` and selector `0xf25f1610`.
    ```solidity
    function setAVSRegistrar(address registrar) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address)`](setAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarReturn {}
    #[allow(
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
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.registrar,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { registrar: tuple.0 }
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
            impl ::core::convert::From<setAVSRegistrarReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address)";
            const SELECTOR: [u8; 4] = [242u8, 95u8, 22u8, 16u8];
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
                        &self.registrar,
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
    /**Function with signature `slashOperator((address,uint32,uint256,string))` and selector `0x0ea43e43`.
    ```solidity
    function slashOperator(IAllocationManagerTypes.SlashingParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub params:
            <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator((address,uint32,uint256,string))`](slashOperatorCall) function.
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
            const SIGNATURE: &'static str = "slashOperator((address,uint32,uint256,string))";
            const SELECTOR: [u8; 4] = [14u8, 164u8, 62u8, 67u8];
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
    ```solidity
    function updateAVSMetadataURI(string memory metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub metadataURI: alloy::sol_types::private::String,
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
                    (value.metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        metadataURI: tuple.0,
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
                        &self.metadataURI,
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
        addStrategiesToOperatorSet(addStrategiesToOperatorSetCall),
        clearDeallocationQueue(clearDeallocationQueueCall),
        createOperatorSets(createOperatorSetsCall),
        delegation(delegationCall),
        deregisterFromOperatorSets(deregisterFromOperatorSetsCall),
        encumberedMagnitude(encumberedMagnitudeCall),
        getAVSRegistrar(getAVSRegistrarCall),
        getAllocatableMagnitude(getAllocatableMagnitudeCall),
        getAllocatedSets(getAllocatedSetsCall),
        getAllocatedStrategies(getAllocatedStrategiesCall),
        getAllocation(getAllocationCall),
        getAllocationDelay(getAllocationDelayCall),
        getAllocations(getAllocationsCall),
        getMaxMagnitude(getMaxMagnitudeCall),
        getMaxMagnitudes_0(getMaxMagnitudes_0Call),
        getMaxMagnitudes_1(getMaxMagnitudes_1Call),
        getMaxMagnitudesAtBlock(getMaxMagnitudesAtBlockCall),
        getMemberCount(getMemberCountCall),
        getMembers(getMembersCall),
        getMinimumSlashableStake(getMinimumSlashableStakeCall),
        getRegisteredSets(getRegisteredSetsCall),
        getStrategiesInOperatorSet(getStrategiesInOperatorSetCall),
        getStrategyAllocations(getStrategyAllocationsCall),
        initialize(initializeCall),
        isOperatorSet(isOperatorSetCall),
        modifyAllocations(modifyAllocationsCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        registerForOperatorSets(registerForOperatorSetsCall),
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        renounceOwnership(renounceOwnershipCall),
        setAVSRegistrar(setAVSRegistrarCall),
        setAllocationDelay_0(setAllocationDelay_0Call),
        setAllocationDelay_1(setAllocationDelay_1Call),
        slashOperator(slashOperatorCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        updateAVSMetadataURI(updateAVSMetadataURICall),
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
            [14u8, 164u8, 62u8, 67u8],
            [16u8, 225u8, 185u8, 184u8],
            [19u8, 100u8, 57u8, 221u8],
            [21u8, 234u8, 121u8, 23u8],
            [21u8, 254u8, 80u8, 40u8],
            [38u8, 13u8, 199u8, 88u8],
            [41u8, 129u8, 235u8, 119u8],
            [43u8, 171u8, 44u8, 74u8],
            [48u8, 76u8, 16u8, 205u8],
            [64u8, 18u8, 13u8, 171u8],
            [65u8, 119u8, 168u8, 124u8],
            [74u8, 16u8, 255u8, 229u8],
            [75u8, 80u8, 70u8, 239u8],
            [84u8, 122u8, 251u8, 135u8],
            [86u8, 196u8, 131u8, 230u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 72u8, 155u8, 181u8],
            [92u8, 151u8, 90u8, 187u8],
            [108u8, 251u8, 68u8, 129u8],
            [110u8, 52u8, 146u8, 181u8],
            [110u8, 135u8, 93u8, 186u8],
            [113u8, 80u8, 24u8, 166u8],
            [118u8, 153u8, 147u8, 66u8],
            [121u8, 174u8, 80u8, 205u8],
            [123u8, 193u8, 239u8, 97u8],
            [132u8, 125u8, 99u8, 79u8],
            [136u8, 111u8, 17u8, 149u8],
            [140u8, 230u8, 72u8, 84u8],
            [141u8, 165u8, 203u8, 91u8],
            [148u8, 215u8, 208u8, 12u8],
            [169u8, 51u8, 62u8, 200u8],
            [169u8, 132u8, 235u8, 58u8],
            [169u8, 143u8, 179u8, 85u8],
            [178u8, 68u8, 122u8, 247u8],
            [185u8, 47u8, 96u8, 165u8],
            [185u8, 251u8, 174u8, 209u8],
            [194u8, 33u8, 216u8, 174u8],
            [205u8, 109u8, 198u8, 135u8],
            [206u8, 123u8, 94u8, 75u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 95u8, 22u8, 16u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerCalls {
        const NAME: &'static str = "AllocationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 44usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::ALLOCATION_CONFIGURATION_DELAY(_) => {
                    <ALLOCATION_CONFIGURATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::DEALLOCATION_DELAY(_) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategiesToOperatorSet(_) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::clearDeallocationQueue(_) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorSets(_) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::deregisterFromOperatorSets(_) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::encumberedMagnitude(_) => {
                    <encumberedMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAVSRegistrar(_) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatableMagnitude(_) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatedSets(_) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocatedStrategies(_) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocation(_) => <getAllocationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getAllocationDelay(_) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getAllocations(_) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitude(_) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudes_0(_) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudes_1(_) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMaxMagnitudesAtBlock(_) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMemberCount(_) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getMembers(_) => <getMembersCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getMinimumSlashableStake(_) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getRegisteredSets(_) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStrategiesInOperatorSet(_) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStrategyAllocations(_) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isOperatorSet(_) => <isOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::registerForOperatorSets(_) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategiesFromOperatorSet(_) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAVSRegistrar(_) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllocationDelay_0(_) => {
                    <setAllocationDelay_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setAllocationDelay_1(_) => {
                    <setAllocationDelay_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashOperator(_) => <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
                    fn getAllocation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getAllocation)
                    }
                    getAllocation
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
                    fn getAllocatedSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getAllocatedSets)
                    }
                    getAllocatedSets
                },
                {
                    fn isOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::isOperatorSet)
                    }
                    isOperatorSet
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
                    fn getMinimumSlashableStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMinimumSlashableStake)
                    }
                    getMinimumSlashableStake
                },
                {
                    fn getAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getAVSRegistrar)
                    }
                    getAVSRegistrar
                },
                {
                    fn getStrategyAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getStrategyAllocations)
                    }
                    getStrategyAllocations
                },
                {
                    fn getStrategiesInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::getStrategiesInOperatorSet)
                    }
                    getStrategiesInOperatorSet
                },
                {
                    fn getMaxMagnitudes_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMaxMagnitudes_0)
                    }
                    getMaxMagnitudes_0
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
                    fn getMaxMagnitudes_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMaxMagnitudes_1)
                    }
                    getMaxMagnitudes_1
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
                    fn deregisterFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::deregisterFromOperatorSets)
                    }
                    deregisterFromOperatorSets
                },
                {
                    fn getMembers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AllocationManagerCalls::getMembers)
                    }
                    getMembers
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
                    fn addStrategiesToOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::addStrategiesToOperatorSet)
                    }
                    addStrategiesToOperatorSet
                },
                {
                    fn getRegisteredSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getRegisteredSets)
                    }
                    getRegisteredSets
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
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::createOperatorSets)
                    }
                    createOperatorSets
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
                    fn getAllocations(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getAllocations)
                    }
                    getAllocations
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
                    fn getMaxMagnitudesAtBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMaxMagnitudesAtBlock)
                    }
                    getMaxMagnitudesAtBlock
                },
                {
                    fn getMaxMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMaxMagnitude)
                    }
                    getMaxMagnitude
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
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn getMemberCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getMemberCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getMemberCount)
                    }
                    getMemberCount
                },
                {
                    fn registerForOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::registerForOperatorSets)
                    }
                    registerForOperatorSets
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
                    fn getAllocatedStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getAllocatedStrategies)
                    }
                    getAllocatedStrategies
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
                    fn removeStrategiesFromOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AllocationManagerCalls::removeStrategiesFromOperatorSet)
                    }
                    removeStrategiesFromOperatorSet
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
                    fn setAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::setAVSRegistrar)
                    }
                    setAVSRegistrar
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
                Self::addStrategiesToOperatorSet(inner) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::clearDeallocationQueue(inner) => {
                    <clearDeallocationQueueCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterFromOperatorSets(inner) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::encumberedMagnitude(inner) => {
                    <encumberedMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAVSRegistrar(inner) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatableMagnitude(inner) => {
                    <getAllocatableMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatedSets(inner) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocatedStrategies(inner) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocation(inner) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocationDelay(inner) => {
                    <getAllocationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getAllocations(inner) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitude(inner) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudes_0(inner) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudes_1(inner) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMaxMagnitudesAtBlock(inner) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMemberCount(inner) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getMembers(inner) => {
                    <getMembersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getMinimumSlashableStake(inner) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getRegisteredSets(inner) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStrategiesInOperatorSet(inner) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStrategyAllocations(inner) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOperatorSet(inner) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::registerForOperatorSets(inner) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::addStrategiesToOperatorSet(inner) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deregisterFromOperatorSets(inner) => {
                    <deregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getAVSRegistrar(inner) => {
                    <getAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getAllocatedSets(inner) => {
                    <getAllocatedSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocatedStrategies(inner) => {
                    <getAllocatedStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getAllocation(inner) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getAllocations(inner) => {
                    <getAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitude(inner) => {
                    <getMaxMagnitudeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudes_0(inner) => {
                    <getMaxMagnitudes_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudes_1(inner) => {
                    <getMaxMagnitudes_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMaxMagnitudesAtBlock(inner) => {
                    <getMaxMagnitudesAtBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMemberCount(inner) => {
                    <getMemberCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMembers(inner) => {
                    <getMembersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getMinimumSlashableStake(inner) => {
                    <getMinimumSlashableStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getRegisteredSets(inner) => {
                    <getRegisteredSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStrategiesInOperatorSet(inner) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStrategyAllocations(inner) => {
                    <getStrategyAllocationsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isOperatorSet(inner) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::registerForOperatorSets(inner) => {
                    <registerForOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeStrategiesFromOperatorSet(inner) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAVSRegistrar(inner) => {
                    <setAVSRegistrarCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::updateAVSMetadataURI(inner) => {
                    <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`AllocationManager`](self) custom errors.
    pub enum AllocationManagerErrors {
        AlreadyMemberOfSet(AlreadyMemberOfSet),
        CurrentlyPaused(CurrentlyPaused),
        Empty(Empty),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InsufficientMagnitude(InsufficientMagnitude),
        InvalidBlockNumber(InvalidBlockNumber),
        InvalidCaller(InvalidCaller),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidOperator(InvalidOperator),
        InvalidOperatorSet(InvalidOperatorSet),
        InvalidSignature(InvalidSignature),
        InvalidStrategy(InvalidStrategy),
        InvalidWadToSlash(InvalidWadToSlash),
        ModificationAlreadyPending(ModificationAlreadyPending),
        NotMemberOfSet(NotMemberOfSet),
        OnlyDelegationManager(OnlyDelegationManager),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorNotRegistered(OperatorNotRegistered),
        OperatorNotSlashable(OperatorNotSlashable),
        OutOfBounds(OutOfBounds),
        SameMagnitude(SameMagnitude),
        SignatureExpired(SignatureExpired),
        StrategyAlreadyInOperatorSet(StrategyAlreadyInOperatorSet),
        StrategyNotInOperatorSet(StrategyNotInOperatorSet),
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
            [37u8, 19u8, 29u8, 79u8],
            [37u8, 236u8, 108u8, 31u8],
            [61u8, 178u8, 161u8, 42u8],
            [67u8, 113u8, 74u8, 253u8],
            [72u8, 245u8, 195u8, 237u8],
            [78u8, 35u8, 110u8, 154u8],
            [78u8, 71u8, 132u8, 108u8],
            [88u8, 92u8, 251u8, 47u8],
            [99u8, 120u8, 104u8, 78u8],
            [108u8, 155u8, 224u8, 191u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [140u8, 12u8, 47u8, 38u8],
            [180u8, 18u8, 15u8, 20u8],
            [198u8, 29u8, 202u8, 93u8],
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 216u8, 220u8, 78u8],
            [216u8, 252u8, 190u8, 48u8],
            [235u8, 191u8, 244u8, 151u8],
            [247u8, 57u8, 88u8, 155u8],
            [250u8, 85u8, 252u8, 129u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerErrors {
        const NAME: &'static str = "AllocationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 27usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyMemberOfSet(_) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::SELECTOR
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
                Self::InsufficientMagnitude(_) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBlockNumber(_) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCaller(_) => <InvalidCaller as alloy_sol_types::SolError>::SELECTOR,
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
                Self::InvalidStrategy(_) => {
                    <InvalidStrategy as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWadToSlash(_) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ModificationAlreadyPending(_) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotMemberOfSet(_) => <NotMemberOfSet as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyDelegationManager(_) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotSlashable(_) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => <OutOfBounds as alloy_sol_types::SolError>::SELECTOR,
                Self::SameMagnitude(_) => <SameMagnitude as alloy_sol_types::SolError>::SELECTOR,
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyAlreadyInOperatorSet(_) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyNotInOperatorSet(_) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::SELECTOR
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
                    fn NotMemberOfSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <NotMemberOfSet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::NotMemberOfSet)
                    }
                    NotMemberOfSet
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
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AllocationManagerErrors::InvalidCaller)
                    }
                    InvalidCaller
                },
                {
                    fn InvalidStrategy(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidStrategy as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InvalidStrategy)
                    }
                    InvalidStrategy
                },
                {
                    fn InvalidBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidBlockNumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InvalidBlockNumber)
                    }
                    InvalidBlockNumber
                },
                {
                    fn StrategyAlreadyInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::StrategyAlreadyInOperatorSet)
                    }
                    StrategyAlreadyInOperatorSet
                },
                {
                    fn StrategyNotInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::StrategyNotInOperatorSet)
                    }
                    StrategyNotInOperatorSet
                },
                {
                    fn InsufficientMagnitude(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InsufficientMagnitude as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InsufficientMagnitude)
                    }
                    InsufficientMagnitude
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
                    fn AlreadyMemberOfSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::AlreadyMemberOfSet)
                    }
                    AlreadyMemberOfSet
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
                    fn OperatorNotSlashable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <OperatorNotSlashable as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::OperatorNotSlashable)
                    }
                    OperatorNotSlashable
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
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::Empty(inner) => <Empty as alloy_sol_types::SolError>::abi_encoded_size(inner),
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidOperator(inner) => {
                    <InvalidOperator as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidOperatorSet(inner) => {
                    <InvalidOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidStrategy(inner) => {
                    <InvalidStrategy as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyDelegationManager(inner) => {
                    <OnlyDelegationManager as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBlockNumber(inner) => {
                    <InvalidBlockNumber as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InvalidStrategy(inner) => {
                    <InvalidStrategy as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidWadToSlash(inner) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ModificationAlreadyPending(inner) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
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
        AVSMetadataURIUpdated(AVSMetadataURIUpdated),
        AVSRegistrarSet(AVSRegistrarSet),
        AllocationDelaySet(AllocationDelaySet),
        AllocationUpdated(AllocationUpdated),
        EncumberedMagnitudeUpdated(EncumberedMagnitudeUpdated),
        Initialized(Initialized),
        MaxMagnitudeUpdated(MaxMagnitudeUpdated),
        OperatorAddedToOperatorSet(OperatorAddedToOperatorSet),
        OperatorRemovedFromOperatorSet(OperatorRemovedFromOperatorSet),
        OperatorSetCreated(OperatorSetCreated),
        OperatorSlashed(OperatorSlashed),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        StrategyAddedToOperatorSet(StrategyAddedToOperatorSet),
        StrategyRemovedFromOperatorSet(StrategyRemovedFromOperatorSet),
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
                20u8, 135u8, 175u8, 84u8, 24u8, 196u8, 126u8, 229u8, 234u8, 69u8, 239u8, 74u8,
                147u8, 57u8, 134u8, 104u8, 18u8, 8u8, 144u8, 119u8, 74u8, 158u8, 19u8, 72u8, 126u8,
                97u8, 233u8, 220u8, 59u8, 175u8, 118u8, 221u8,
            ],
            [
                28u8, 100u8, 88u8, 7u8, 154u8, 65u8, 7u8, 125u8, 0u8, 60u8, 17u8, 250u8, 249u8,
                191u8, 9u8, 126u8, 105u8, 59u8, 214u8, 121u8, 121u8, 228u8, 230u8, 80u8, 11u8,
                172u8, 123u8, 41u8, 219u8, 119u8, 155u8, 92u8,
            ],
            [
                42u8, 233u8, 69u8, 196u8, 12u8, 68u8, 220u8, 14u8, 194u8, 99u8, 249u8, 86u8, 9u8,
                195u8, 253u8, 198u8, 149u8, 46u8, 10u8, 239u8, 162u8, 45u8, 99u8, 116u8, 228u8,
                79u8, 44u8, 153u8, 122u8, 206u8, 223u8, 133u8,
            ],
            [
                49u8, 98u8, 146u8, 133u8, 234u8, 210u8, 51u8, 90u8, 224u8, 147u8, 63u8, 134u8,
                237u8, 42u8, 230u8, 51u8, 33u8, 247u8, 175u8, 119u8, 180u8, 230u8, 234u8, 171u8,
                196u8, 44u8, 5u8, 120u8, 128u8, 151u8, 126u8, 108u8,
            ],
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                67u8, 35u8, 46u8, 223u8, 144u8, 113u8, 117u8, 61u8, 35u8, 33u8, 229u8, 250u8,
                126u8, 1u8, 131u8, 99u8, 238u8, 36u8, 142u8, 95u8, 33u8, 66u8, 230u8, 192u8, 142u8,
                221u8, 50u8, 101u8, 191u8, 180u8, 137u8, 94u8,
            ],
            [
                78u8, 133u8, 117u8, 29u8, 99u8, 49u8, 80u8, 108u8, 108u8, 98u8, 51u8, 95u8, 32u8,
                126u8, 179u8, 31u8, 18u8, 166u8, 30u8, 87u8, 15u8, 52u8, 245u8, 193u8, 118u8, 64u8,
                48u8, 135u8, 133u8, 198u8, 212u8, 219u8,
            ],
            [
                122u8, 178u8, 96u8, 254u8, 10u8, 241u8, 147u8, 219u8, 95u8, 73u8, 134u8, 119u8,
                13u8, 131u8, 27u8, 218u8, 78u8, 164u8, 96u8, 153u8, 220u8, 129u8, 126u8, 139u8,
                103u8, 22u8, 220u8, 174u8, 138u8, 248u8, 232u8, 139u8,
            ],
            [
                123u8, 75u8, 7u8, 61u8, 128u8, 220u8, 172u8, 85u8, 161u8, 17u8, 119u8, 216u8, 69u8,
                154u8, 217u8, 246u8, 100u8, 206u8, 235u8, 145u8, 247u8, 31u8, 39u8, 22u8, 123u8,
                177u8, 79u8, 129u8, 82u8, 167u8, 238u8, 238u8,
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
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                168u8, 156u8, 29u8, 194u8, 67u8, 216u8, 144u8, 138u8, 150u8, 221u8, 132u8, 148u8,
                75u8, 204u8, 151u8, 214u8, 188u8, 106u8, 192u8, 13u8, 215u8, 142u8, 32u8, 98u8,
                21u8, 118u8, 190u8, 106u8, 60u8, 148u8, 55u8, 19u8,
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
            [
                173u8, 52u8, 195u8, 7u8, 11u8, 225u8, 223u8, 251u8, 202u8, 164u8, 153u8, 208u8,
                0u8, 186u8, 43u8, 141u8, 152u8, 72u8, 174u8, 252u8, 172u8, 48u8, 89u8, 223u8, 36u8,
                93u8, 217u8, 92u8, 78u8, 206u8, 20u8, 254u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AllocationManagerEvents {
        const NAME: &'static str = "AllocationManagerEvents";
        const COUNT: usize = 16usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AVSMetadataURIUpdated)
                }
                Some(<AVSRegistrarSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AVSRegistrarSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AVSRegistrarSet)
                }
                Some(<AllocationDelaySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AllocationDelaySet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AllocationDelaySet)
                }
                Some(<AllocationUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AllocationUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::AllocationUpdated)
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
                Some(<OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorAddedToOperatorSet)
                }
                Some(
                    <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::OperatorRemovedFromOperatorSet),
                Some(<OperatorSetCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSetCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorSetCreated)
                }
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
                Some(<StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyAddedToOperatorSet)
                }
                Some(
                    <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::StrategyRemovedFromOperatorSet),
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
                Self::AVSMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AVSRegistrarSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AllocationUpdated(inner) => {
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
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::StrategyAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AVSRegistrarSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AllocationDelaySet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::AllocationUpdated(inner) => {
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
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSlashed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::StrategyAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyRemovedFromOperatorSet(inner) => {
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
        _pauserRegistry: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<AllocationManagerInstance<T, P, N>>>
    {
        AllocationManagerInstance::<T, P, N>::deploy(
            provider,
            _delegation,
            _pauserRegistry,
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
        _pauserRegistry: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AllocationManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _delegation,
            _pauserRegistry,
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
            _pauserRegistry: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
        ) -> alloy_contract::Result<AllocationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegation,
                _pauserRegistry,
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
            _pauserRegistry: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _delegation,
                        _pauserRegistry,
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
        ///Creates a new call builder for the [`addStrategiesToOperatorSet`] function.
        pub fn addStrategiesToOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesToOperatorSetCall, N> {
            self.call_builder(&addStrategiesToOperatorSetCall {
                operatorSetId,
                strategies,
            })
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
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(&createOperatorSetsCall { params })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterFromOperatorSets`] function.
        pub fn deregisterFromOperatorSets(
            &self,
            params: <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterFromOperatorSetsCall, N> {
            self.call_builder(&deregisterFromOperatorSetsCall { params })
        }
        ///Creates a new call builder for the [`encumberedMagnitude`] function.
        pub fn encumberedMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, encumberedMagnitudeCall, N> {
            self.call_builder(&encumberedMagnitudeCall { operator, strategy })
        }
        ///Creates a new call builder for the [`getAVSRegistrar`] function.
        pub fn getAVSRegistrar(
            &self,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAVSRegistrarCall, N> {
            self.call_builder(&getAVSRegistrarCall { avs })
        }
        ///Creates a new call builder for the [`getAllocatableMagnitude`] function.
        pub fn getAllocatableMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatableMagnitudeCall, N> {
            self.call_builder(&getAllocatableMagnitudeCall { operator, strategy })
        }
        ///Creates a new call builder for the [`getAllocatedSets`] function.
        pub fn getAllocatedSets(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatedSetsCall, N> {
            self.call_builder(&getAllocatedSetsCall { operator })
        }
        ///Creates a new call builder for the [`getAllocatedStrategies`] function.
        pub fn getAllocatedStrategies(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocatedStrategiesCall, N> {
            self.call_builder(&getAllocatedStrategiesCall {
                operator,
                operatorSet,
            })
        }
        ///Creates a new call builder for the [`getAllocation`] function.
        pub fn getAllocation(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationCall, N> {
            self.call_builder(&getAllocationCall {
                operator,
                operatorSet,
                strategy,
            })
        }
        ///Creates a new call builder for the [`getAllocationDelay`] function.
        pub fn getAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationDelayCall, N> {
            self.call_builder(&getAllocationDelayCall { operator })
        }
        ///Creates a new call builder for the [`getAllocations`] function.
        pub fn getAllocations(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationsCall, N> {
            self.call_builder(&getAllocationsCall {
                operators,
                operatorSet,
                strategy,
            })
        }
        ///Creates a new call builder for the [`getMaxMagnitude`] function.
        pub fn getMaxMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudeCall, N> {
            self.call_builder(&getMaxMagnitudeCall { operator, strategy })
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_0`] function.
        pub fn getMaxMagnitudes_0(
            &self,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_0Call, N> {
            self.call_builder(&getMaxMagnitudes_0Call {
                operators,
                strategy,
            })
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_1`] function.
        pub fn getMaxMagnitudes_1(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_1Call, N> {
            self.call_builder(&getMaxMagnitudes_1Call {
                operator,
                strategies,
            })
        }
        ///Creates a new call builder for the [`getMaxMagnitudesAtBlock`] function.
        pub fn getMaxMagnitudesAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesAtBlockCall, N> {
            self.call_builder(&getMaxMagnitudesAtBlockCall {
                operator,
                strategies,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getMemberCount`] function.
        pub fn getMemberCount(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMemberCountCall, N> {
            self.call_builder(&getMemberCountCall { operatorSet })
        }
        ///Creates a new call builder for the [`getMembers`] function.
        pub fn getMembers(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMembersCall, N> {
            self.call_builder(&getMembersCall { operatorSet })
        }
        ///Creates a new call builder for the [`getMinimumSlashableStake`] function.
        pub fn getMinimumSlashableStake(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            futureBlock: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinimumSlashableStakeCall, N> {
            self.call_builder(&getMinimumSlashableStakeCall {
                operatorSet,
                operators,
                strategies,
                futureBlock,
            })
        }
        ///Creates a new call builder for the [`getRegisteredSets`] function.
        pub fn getRegisteredSets(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getRegisteredSetsCall, N> {
            self.call_builder(&getRegisteredSetsCall { operator })
        }
        ///Creates a new call builder for the [`getStrategiesInOperatorSet`] function.
        pub fn getStrategiesInOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStrategiesInOperatorSetCall, N> {
            self.call_builder(&getStrategiesInOperatorSetCall { operatorSet })
        }
        ///Creates a new call builder for the [`getStrategyAllocations`] function.
        pub fn getStrategyAllocations(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStrategyAllocationsCall, N> {
            self.call_builder(&getStrategyAllocationsCall { operator, strategy })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                initialPausedStatus,
            })
        }
        ///Creates a new call builder for the [`isOperatorSet`] function.
        pub fn isOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetCall, N> {
            self.call_builder(&isOperatorSetCall { operatorSet })
        }
        ///Creates a new call builder for the [`modifyAllocations`] function.
        pub fn modifyAllocations(
            &self,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyAllocationsCall, N> {
            self.call_builder(&modifyAllocationsCall { params })
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
        ///Creates a new call builder for the [`registerForOperatorSets`] function.
        pub fn registerForOperatorSets(
            &self,
            params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerForOperatorSetsCall, N> {
            self.call_builder(&registerForOperatorSetsCall { params })
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeStrategiesFromOperatorSetCall, N> {
            self.call_builder(&removeStrategiesFromOperatorSetCall {
                operatorSetId,
                strategies,
            })
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`setAVSRegistrar`] function.
        pub fn setAVSRegistrar(
            &self,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(&setAVSRegistrarCall { registrar })
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
        ///Creates a new call builder for the [`updateAVSMetadataURI`] function.
        pub fn updateAVSMetadataURI(
            &self,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(&updateAVSMetadataURICall { metadataURI })
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
        ///Creates a new event filter for the [`AVSMetadataURIUpdated`] event.
        pub fn AVSMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSMetadataURIUpdated, N> {
            self.event_filter::<AVSMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`AVSRegistrarSet`] event.
        pub fn AVSRegistrarSet_filter(&self) -> alloy_contract::Event<T, &P, AVSRegistrarSet, N> {
            self.event_filter::<AVSRegistrarSet>()
        }
        ///Creates a new event filter for the [`AllocationDelaySet`] event.
        pub fn AllocationDelaySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AllocationDelaySet, N> {
            self.event_filter::<AllocationDelaySet>()
        }
        ///Creates a new event filter for the [`AllocationUpdated`] event.
        pub fn AllocationUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AllocationUpdated, N> {
            self.event_filter::<AllocationUpdated>()
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
        ///Creates a new event filter for the [`OperatorAddedToOperatorSet`] event.
        pub fn OperatorAddedToOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAddedToOperatorSet, N> {
            self.event_filter::<OperatorAddedToOperatorSet>()
        }
        ///Creates a new event filter for the [`OperatorRemovedFromOperatorSet`] event.
        pub fn OperatorRemovedFromOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorRemovedFromOperatorSet, N> {
            self.event_filter::<OperatorRemovedFromOperatorSet>()
        }
        ///Creates a new event filter for the [`OperatorSetCreated`] event.
        pub fn OperatorSetCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSetCreated, N> {
            self.event_filter::<OperatorSetCreated>()
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
        ///Creates a new event filter for the [`StrategyAddedToOperatorSet`] event.
        pub fn StrategyAddedToOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyAddedToOperatorSet, N> {
            self.event_filter::<StrategyAddedToOperatorSet>()
        }
        ///Creates a new event filter for the [`StrategyRemovedFromOperatorSet`] event.
        pub fn StrategyRemovedFromOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyRemovedFromOperatorSet, N> {
            self.event_filter::<StrategyRemovedFromOperatorSet>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
