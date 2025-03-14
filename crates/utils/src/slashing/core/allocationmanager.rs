///Module containing a contract's types and functions.
/**

```solidity
library IAllocationManagerTypes {
    struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
    struct Allocation { uint64 currentMagnitude; int128 pendingDiff; uint32 effectBlock; }
    struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
    struct DeregisterParams { address operator; address avs; uint32[] operatorSetIds; }
    struct RegisterParams { address avs; uint32[] operatorSetIds; bytes data; }
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAllocationManagerTypes {
    use crate::slashing::core::allocationmanager::AllocationManager::OperatorSet;

    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocateParams {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub currentMagnitude: u64,
        #[allow(missing_docs)]
        pub pendingDiff: i128,
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
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
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
    struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub wadsToSlash:
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
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            u32,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
                    value.wadsToSlash,
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
                    wadsToSlash: tuple.3,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.wadsToSlash),
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
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256[] wadsToSlash,string description)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.wadsToSlash)
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
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.wadsToSlash,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.wadsToSlash,
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
        address[] strategies;
        uint256[] wadsToSlash;
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
    error InvalidAVSRegistrar();
    error InvalidAVSWithNoMetadataRegistered();
    error InvalidCaller();
    error InvalidNewPausedStatus();
    error InvalidOperator();
    error InvalidOperatorSet();
    error InvalidPermissions();
    error InvalidSnapshotOrdering();
    error InvalidWadToSlash();
    error ModificationAlreadyPending();
    error NotMemberOfSet();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorNotSlashable();
    error OutOfBounds();
    error SameMagnitude();
    error StrategiesMustBeInAscendingOrder();
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

    constructor(address _delegation, address _pauserRegistry, address _permissionController, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);

    function ALLOCATION_CONFIGURATION_DELAY() external view returns (uint32);
    function DEALLOCATION_DELAY() external view returns (uint32);
    function addStrategiesToOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    function clearDeallocationQueue(address operator, address[] memory strategies, uint16[] memory numToClear) external;
    function createOperatorSets(address avs, IAllocationManagerTypes.CreateSetParams[] memory params) external;
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
    function getOperatorSetCount(address avs) external view returns (uint256);
    function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
    function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory);
    function getStrategyAllocations(address operator, address strategy) external view returns (OperatorSet[] memory, IAllocationManagerTypes.Allocation[] memory);
    function initialize(address initialOwner, uint256 initialPausedStatus) external;
    function isMemberOfOperatorSet(address operator, OperatorSet memory operatorSet) external view returns (bool);
    function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
    function modifyAllocations(address operator, IAllocationManagerTypes.AllocateParams[] memory params) external;
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function permissionController() external view returns (address);
    function registerForOperatorSets(address operator, IAllocationManagerTypes.RegisterParams memory params) external;
    function removeStrategiesFromOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function setAVSRegistrar(address avs, address registrar) external;
    function setAllocationDelay(address operator, uint32 delay) external;
    function slashOperator(address avs, IAllocationManagerTypes.SlashingParams memory params) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateAVSMetadataURI(address avs, string memory metadataURI) external;
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
        "name": "_permissionController",
        "type": "address",
        "internalType": "contract IPermissionController"
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
        "name": "avs",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "getOperatorSetCount",
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
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "isMemberOfOperatorSet",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
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
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "registerForOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
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
        "name": "avs",
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "slashOperator",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "internalType": "address"
      },
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
            "name": "wadsToSlash",
            "type": "uint256[]",
            "internalType": "uint256[]"
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
        "name": "avs",
        "type": "address",
        "internalType": "address"
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
    "name": "InvalidAVSRegistrar",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidAVSWithNoMetadataRegistered",
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
    "name": "InvalidPermissions",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidSnapshotOrdering",
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
    "name": "StrategiesMustBeInAscendingOrder",
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod AllocationManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610120604052348015610010575f5ffd5b50604051615b9c380380615b9c83398101604081905261002f91610180565b82858383876001600160a01b03811661005b576040516339b190bb60e11b815260040160405180910390fd5b6001600160a01b0390811660805292831660a05263ffffffff91821660c0521660e052166101005261008b610095565b50505050506101e9565b5f54610100900460ff16156101005760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b5f5460ff9081161461014f575f805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b0381168114610165575f5ffd5b50565b805163ffffffff8116811461017b575f5ffd5b919050565b5f5f5f5f5f60a08688031215610194575f5ffd5b855161019f81610151565b60208701519095506101b081610151565b60408701519094506101c181610151565b92506101cf60608701610168565b91506101dd60808701610168565b90509295509295909350565b60805160a05160c05160e0516101005161592261027a5f395f81816103f9015261382101525f81816105480152613e1901525f818161031e01528181612276015261297e01525f81816106fa01528181610ccf0152818161165401528181611cbb01528181611d250152612c8701525f818161056f0152818161079201528181611dca015261349801526159225ff3fe608060405234801561000f575f5ffd5b5060043610610281575f3560e01c80636e875dba11610156578063a984eb3a116100ca578063c221d8ae11610084578063c221d8ae146106bc578063cd6dc687146106cf578063d3d96ff4146106e2578063df5cf723146106f5578063f2fde38b1461071c578063fabc1cbc1461072f575f5ffd5b8063a984eb3a1461060e578063adc2e3d914610641578063b2447af714610654578063b66bd98914610667578063b9fbaed11461067a578063ba1a84e5146106a9575f5ffd5b80638ce648541161011b5780638ce64854146105915780638da5cb5b146105b157806394d7d00c146105c2578063952899ee146105d5578063a9333ec8146105e8578063a9821821146105fb575f5ffd5b80636e875dba14610515578063715018a61461052857806379ae50cd146105305780637bc1ef6114610543578063886f11951461056a575f5ffd5b80634657e26a116101f8578063595c6a67116101b2578063595c6a67146104875780635ac86ab71461048f5780635c975abb146104b2578063670d3ba2146104c45780636cfb4481146104d75780636e3492b514610502575f5ffd5b80634657e26a146103f45780634a10ffe51461041b5780634b5046ef1461043b57806350feea201461044e578063547afb871461046157806356c483e614610474575f5ffd5b80632981eb77116102495780632981eb77146103195780632bab2c4a14610355578063304c10cd1461037557806336352057146103a057806340120dab146103b35780634177a87c146103d4575f5ffd5b806310e1b9b814610285578063136439dd146102ae57806315fe5028146102c3578063260dc758146102e3578063261f84e014610306575b5f5ffd5b61029861029336600461483e565b610742565b6040516102a59190614885565b60405180910390f35b6102c16102bc3660046148b8565b61077d565b005b6102d66102d13660046148cf565b610852565b6040516102a5919061494d565b6102f66102f136600461495f565b610969565b60405190151581526020016102a5565b6102c16103143660046149b9565b6109a0565b6103407f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102a5565b610368610363366004614a9e565b610c7b565b6040516102a59190614b52565b6103886103833660046148cf565b610f68565b6040516001600160a01b0390911681526020016102a5565b6102c16103ae366004614bb5565b610f97565b6103c66103c1366004614c07565b6117a9565b6040516102a5929190614c94565b6103e76103e236600461495f565b611924565b6040516102a59190614cf1565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b61042e610429366004614d03565b611948565b6040516102a59190614d46565b6102c1610449366004614d91565b6119f0565b6102c161045c366004614e11565b611aaa565b61042e61046f366004614e6f565b611c08565b6102c1610482366004614ebb565b611cb0565b6102c1611db5565b6102f661049d366004614eee565b606654600160ff9092169190911b9081161490565b6066545b6040519081526020016102a5565b6102f66104d2366004614f0e565b611e64565b6104ea6104e5366004614c07565b611e75565b6040516001600160401b0390911681526020016102a5565b6102c1610510366004614f4f565b611fe2565b6103e761052336600461495f565b6123c3565b6102c16123d4565b6102d661053e3660046148cf565b6123e5565b6103407f000000000000000000000000000000000000000000000000000000000000000081565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b6105a461059f366004614f80565b6124bf565b6040516102a59190614fc3565b6033546001600160a01b0316610388565b61042e6105d0366004614fd5565b612584565b6102c16105e3366004615030565b612670565b6104ea6105f6366004614c07565b612b37565b6102c16106093660046151d9565b612b66565b6104ea61061c366004614c07565b60a260209081525f92835260408084209091529082529020546001600160401b031681565b6102c161064f366004615257565b612c18565b6104b661066236600461495f565b612f75565b6102c1610675366004614e11565b612f97565b61068d6106883660046148cf565b6130f1565b60408051921515835263ffffffff9091166020830152016102a5565b6104b66106b73660046148cf565b61318b565b6103e76106ca366004614f0e565b6131ab565b6102c16106dd366004615299565b6131dc565b6102c16106f0366004614c07565b6132f9565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b6102c161072a3660046148cf565b61341d565b6102c161073d3660046148b8565b613496565b604080516060810182525f80825260208201819052918101829052906107718561076b866135ac565b8561360f565b925050505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156107df573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061080391906152c3565b61082057604051631d77d47760e21b815260040160405180910390fd5b60665481811681146108455760405163c61dca5d60e01b815260040160405180910390fd5b61084e8261377b565b5050565b6001600160a01b0381165f908152609d6020526040812060609190610876906137b8565b90505f816001600160401b0381111561089157610891614762565b6040519080825280602002602001820160405280156108d557816020015b604080518082019091525f80825260208201528152602001906001900390816108af5790505b5090505f5b82811015610961576001600160a01b0385165f908152609d6020526040902061093c9061090790836137c1565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b82828151811061094e5761094e6152e2565b60209081029190910101526001016108da565b509392505050565b60208082015182516001600160a01b03165f90815260989092526040822061099a9163ffffffff908116906137cc16565b92915050565b826109aa816137e3565b6109c75760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff166109ff5760405163caf9756160e01b815260040160405180910390fd5b5f5b82811015610c74575f6040518060400160405280876001600160a01b03168152602001868685818110610a3657610a366152e2565b9050602002810190610a4891906152f6565b610a56906020810190615314565b63ffffffff168152509050610aa0816020015163ffffffff1660985f896001600160a01b03166001600160a01b031681526020019081526020015f2061388d90919063ffffffff16565b610abd57604051631fb1705560e21b815260040160405180910390fd5b7f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040518060400160405280886001600160a01b03168152602001836020015163ffffffff16815250604051610b13919061532d565b60405180910390a15f610b25826135ac565b90505f5b868685818110610b3b57610b3b6152e2565b9050602002810190610b4d91906152f6565b610b5b90602081019061533b565b9050811015610c6957610bd1878786818110610b7957610b796152e2565b9050602002810190610b8b91906152f6565b610b9990602081019061533b565b83818110610ba957610ba96152e2565b9050602002016020810190610bbe91906148cf565b5f84815260996020526040902090613898565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83888887818110610c0657610c066152e2565b9050602002810190610c1891906152f6565b610c2690602081019061533b565b84818110610c3657610c366152e2565b9050602002016020810190610c4b91906148cf565b604051610c59929190615380565b60405180910390a1600101610b29565b505050600101610a01565b5050505050565b606083516001600160401b03811115610c9657610c96614762565b604051908082528060200260200182016040528015610cc957816020015b6060815260200190600190039081610cb45790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b8152600401610d1b9291906153a6565b5f60405180830381865afa158015610d35573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610d5c91908101906153ca565b90505f5b8551811015610f5e575f868281518110610d7c57610d7c6152e2565b6020026020010151905085516001600160401b03811115610d9f57610d9f614762565b604051908082528060200260200182016040528015610dc8578160200160208202803683370190505b50848381518110610ddb57610ddb6152e2565b60209081029190910101525f5b8651811015610f54575f878281518110610e0457610e046152e2565b6020908102919091018101516001600160a01b038086165f90815260a1845260408082209284168252919093528220909250610e3f906138ac565b9050806001600160401b03165f03610e58575050610f4c565b5f610e64858d85610742565b90508863ffffffff16816040015163ffffffff1611158015610e8c57505f8160200151600f0b125b15610eae57610ea2815f015182602001516138bf565b6001600160401b031681525b80515f90610ec9906001600160401b039081169085166138d3565b9050610f1081898981518110610ee157610ee16152e2565b60200260200101518781518110610efa57610efa6152e2565b60200260200101516138e790919063ffffffff16565b898881518110610f2257610f226152e2565b60200260200101518681518110610f3b57610f3b6152e2565b602002602001018181525050505050505b600101610de8565b5050600101610d60565b5050949350505050565b6001600160a01b038082165f908152609760205260408120549091168015610f905780610776565b5090919050565b606654600190600290811603610fc05760405163840a48d560e01b815260040160405180910390fd5b82610fca816137e3565b610fe75760405163932d94f760e01b815260040160405180910390fd5b5f6040518060400160405280866001600160a01b031681526020018560200160208101906110159190615314565b63ffffffff16905290505f61103661103060208701876148cf565b836138fb565b9050611045606086018661533b565b9050611054604087018761533b565b905014611074576040516343714afd60e01b815260040160405180910390fd5b60208083015183516001600160a01b03165f908152609890925260409091206110a69163ffffffff908116906137cc16565b6110c357604051631fb1705560e21b815260040160405180910390fd5b806110e15760405163ebbff49760e01b815260040160405180910390fd5b5f6110ef604087018761533b565b90506001600160401b0381111561110857611108614762565b604051908082528060200260200182016040528015611131578160200160208202803683370190505b5090505f5b611143604088018861533b565b905081101561173a578015806111d65750611161604088018861533b565b61116c6001846154ea565b81811061117b5761117b6152e2565b905060200201602081019061119091906148cf565b6001600160a01b03166111a6604089018961533b565b838181106111b6576111b66152e2565b90506020020160208101906111cb91906148cf565b6001600160a01b0316115b6111f357604051639f1c805360e01b815260040160405180910390fd5b611200606088018861533b565b82818110611210576112106152e2565b905060200201355f1080156112505750670de0b6b3a7640000611236606089018961533b565b83818110611246576112466152e2565b9050602002013511155b61126d57604051631353603160e01b815260040160405180910390fd5b6112c961127d604089018961533b565b8381811061128d5761128d6152e2565b90506020020160208101906112a291906148cf565b60995f6112ae886135ac565b81526020019081526020015f2061397190919063ffffffff16565b6112e6576040516331bc342760e11b815260040160405180910390fd5b5f806113386112f860208b018b6148cf565b611301886135ac565b61130e60408d018d61533b565b8781811061131e5761131e6152e2565b905060200201602081019061133391906148cf565b61360f565b805191935091506001600160401b03165f03611355575050611732565b5f61139061136660608c018c61533b565b86818110611376576113766152e2565b85516001600160401b031692602090910201359050613992565b83519091506113ab6001600160401b038084169083166138d3565b8686815181106113bd576113bd6152e2565b60200260200101818152505081835f018181516113da91906154fd565b6001600160401b03169052508351829085906113f79083906154fd565b6001600160401b03169052506020840180518391906114179083906154fd565b6001600160401b031690525060208301515f600f9190910b1215611532575f61147a61144660608e018e61533b565b88818110611456576114566152e2565b90506020020135856020015161146b9061551c565b6001600160801b031690613992565b9050806001600160401b0316846020018181516114979190615540565b600f0b9052507f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd6114cb60208e018e6148cf565b8a8e80604001906114dc919061533b565b8a8181106114ec576114ec6152e2565b905060200201602081019061150191906148cf565b611512885f015189602001516138bf565b886040015160405161152895949392919061556d565b60405180910390a1505b61158461154260208d018d6148cf565b61154b8a6135ac565b61155860408f018f61533b565b89818110611568576115686152e2565b905060200201602081019061157d91906148cf565b87876139a8565b7f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd6115b260208d018d6148cf565b896115c060408f018f61533b565b898181106115d0576115d06152e2565b90506020020160208101906115e591906148cf565b86516040516115f99493929190439061556d565b60405180910390a161164a61161160208d018d6148cf565b61161e60408e018e61533b565b8881811061162e5761162e6152e2565b905060200201602081019061164391906148cf565b8651613be0565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663ee74937f61168660208e018e6148cf565b61169360408f018f61533b565b898181106116a3576116a36152e2565b90506020020160208101906116b891906148cf565b875160405160e085901b6001600160e01b03191681526001600160a01b0393841660048201529290911660248301526001600160401b0380861660448401521660648201526084015f604051808303815f87803b158015611717575f5ffd5b505af1158015611729573d5f5f3e3d5ffd5b50505050505050505b600101611136565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe561176960208801886148cf565b8461177760408a018a61533b565b8561178560808d018d6155be565b6040516117989796959493929190615628565b60405180910390a150505050505050565b6001600160a01b0382165f908152609d6020526040812060609182916117ce906137b8565b90505f816001600160401b038111156117e9576117e9614762565b60405190808252806020026020018201604052801561182d57816020015b604080518082019091525f80825260208201528152602001906001900390816118075790505b5090505f826001600160401b0381111561184957611849614762565b60405190808252806020026020018201604052801561189257816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816118675790505b5090505f5b83811015611915576001600160a01b0388165f908152609d602052604081206118c49061090790846137c1565b9050808483815181106118d9576118d96152e2565b60200260200101819052506118ef89828a610742565b838381518110611901576119016152e2565b602090810291909101015250600101611897565b509093509150505b9250929050565b60605f61077660995f611936866135ac565b81526020019081526020015f20613c62565b60605f83516001600160401b0381111561196457611964614762565b60405190808252806020026020018201604052801561198d578160200160208202803683370190505b5090505f5b8451811015610961576119be8582815181106119b0576119b06152e2565b602002602001015185612b37565b8282815181106119d0576119d06152e2565b6001600160401b0390921660209283029190910190910152600101611992565b6066545f90600190811603611a185760405163840a48d560e01b815260040160405180910390fd5b838214611a38576040516343714afd60e01b815260040160405180910390fd5b5f5b84811015611aa157611a9987878784818110611a5857611a586152e2565b9050602002016020810190611a6d91906148cf565b868685818110611a7f57611a7f6152e2565b9050602002016020810190611a9491906156be565b613c6e565b600101611a3a565b50505050505050565b83611ab4816137e3565b611ad15760405163932d94f760e01b815260040160405180910390fd5b604080518082019091526001600160a01b038616815263ffffffff851660208201525f611afd826135ac565b9050611b3e826020015163ffffffff1660985f8a6001600160a01b03166001600160a01b031681526020019081526020015f206137cc90919063ffffffff16565b611b5b57604051631fb1705560e21b815260040160405180910390fd5b5f5b84811015611bfe57611b7a868683818110610ba957610ba96152e2565b611b975760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83878784818110611bcb57611bcb6152e2565b9050602002016020810190611be091906148cf565b604051611bee929190615380565b60405180910390a1600101611b5d565b5050505050505050565b60605f82516001600160401b03811115611c2457611c24614762565b604051908082528060200260200182016040528015611c4d578160200160208202803683370190505b5090505f5b835181101561096157611c7e85858381518110611c7157611c716152e2565b6020026020010151612b37565b828281518110611c9057611c906152e2565b6001600160401b0390921660209283029190910190910152600101611c52565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611dab57611ce9826137e3565b611d06576040516348f5c3ed60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0383811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015611d6a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d8e91906152c3565b611dab5760405163ccea9e6f60e01b815260040160405180910390fd5b61084e8282613d72565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611e17573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e3b91906152c3565b611e5857604051631d77d47760e21b815260040160405180910390fd5b611e625f1961377b565b565b5f61077683609a5f6112ae866135ac565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611f9f576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120611f079083613f13565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015611f82575050611f9f565b611f908582602001516138bf565b94505050806001019050611ecf565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208290611fcf906138ac565b611fd991906154fd565b95945050505050565b60665460029060049081160361200b5760405163840a48d560e01b815260040160405180910390fd5b61202061201b60208401846148cf565b6137e3565b80612039575061203961201b60408401602085016148cf565b612056576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b612065604084018461533b565b9050811015612327575f604051806040016040528085602001602081019061208d91906148cf565b6001600160a01b031681526020016120a8604087018761533b565b858181106120b8576120b86152e2565b90506020020160208101906120cd9190615314565b63ffffffff16815250905061211a816020015163ffffffff1660985f8760200160208101906120fc91906148cf565b6001600160a01b0316815260208101919091526040015f20906137cc565b61213757604051631fb1705560e21b815260040160405180910390fd5b609e5f61214760208701876148cf565b6001600160a01b03166001600160a01b031681526020019081526020015f205f612170836135ac565b815260208101919091526040015f205460ff166121a0576040516325131d4f60e01b815260040160405180910390fd5b6121da6121ac826135ac565b609c5f6121bc60208901896148cf565b6001600160a01b0316815260208101919091526040015f2090613f82565b506122126121eb60208601866148cf565b609a5f6121f7856135ac565b81526020019081526020015f20613f8d90919063ffffffff16565b5061222060208501856148cf565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe82604051612258919061532d565b60405180910390a2604080518082019091525f81526020810161229b7f0000000000000000000000000000000000000000000000000000000000000000436156df565b63ffffffff169052609e5f6122b360208801886148cf565b6001600160a01b03166001600160a01b031681526020019081526020015f205f6122dc846135ac565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101612058565b5061233b61038360408401602085016148cf565b6001600160a01b031663303ca95661235660208501856148cf565b61236660408601602087016148cf565b612373604087018761533b565b6040518563ffffffff1660e01b81526004016123929493929190615734565b5f604051808303815f87803b1580156123a9575f5ffd5b505af11580156123bb573d5f5f3e3d5ffd5b505050505050565b606061099a609a5f611936856135ac565b6123dc613fa1565b611e625f613ffb565b6001600160a01b0381165f908152609c6020526040812060609190612409906137b8565b90505f816001600160401b0381111561242457612424614762565b60405190808252806020026020018201604052801561246857816020015b604080518082019091525f80825260208201528152602001906001900390816124425790505b5090505f5b82811015610961576001600160a01b0385165f908152609c6020526040902061249a9061090790836137c1565b8282815181106124ac576124ac6152e2565b602090810291909101015260010161246d565b60605f84516001600160401b038111156124db576124db614762565b60405190808252806020026020018201604052801561252457816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816124f95790505b5090505f5b855181101561257b57612556868281518110612547576125476152e2565b60200260200101518686610742565b828281518110612568576125686152e2565b6020908102919091010152600101612529565b50949350505050565b60605f83516001600160401b038111156125a0576125a0614762565b6040519080825280602002602001820160405280156125c9578160200160208202803683370190505b5090505f5b845181101561257b576001600160a01b0386165f90815260a160205260408120865161263e92879291899086908110612609576126096152e2565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2061404c90919063ffffffff16565b828281518110612650576126506152e2565b6001600160401b03909216602092830291909101909101526001016125ce565b6066545f906001908116036126985760405163840a48d560e01b815260040160405180910390fd5b6126a1836137e3565b6126be576040516348f5c3ed60e01b815260040160405180910390fd5b5f5f5f6126ca866130f1565b91509150816126ec5760405163fa55fc8160e01b815260040160405180910390fd5b91505f90505b8351811015610c745783818151811061270d5761270d6152e2565b6020026020010151604001515184828151811061272c5761272c6152e2565b6020026020010151602001515114612757576040516343714afd60e01b815260040160405180910390fd5b5f84828151811061276a5761276a6152e2565b602090810291909101810151518082015181516001600160a01b03165f908152609890935260409092209092506127aa9163ffffffff908116906137cc16565b6127c757604051631fb1705560e21b815260040160405180910390fd5b5f6127d287836138fb565b90505f5b8684815181106127e8576127e86152e2565b60200260200101516020015151811015612b2c575f87858151811061280f5761280f6152e2565b602002602001015160200151828151811061282c5761282c6152e2565b60200260200101519050612843898261ffff613c6e565b5f5f6128528b61076b886135ac565b915091508060200151600f0b5f1461287d57604051630d8fcbe360e41b815260040160405180910390fd5b5f61288a87858489614060565b90506128cf825f01518c8a815181106128a5576128a56152e2565b60200260200101516040015187815181106128c2576128c26152e2565b6020026020010151614096565b600f0b602083018190525f036128f857604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b1215612a3c5780156129be57612979612919886135ac565b6001600160a01b03808f165f90815260a360209081526040808320938a16835292905220908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b6129a37f0000000000000000000000000000000000000000000000000000000000000000436156df565b6129ae9060016156df565b63ffffffff166040830152612aa9565b6129d0836020015183602001516138bf565b6001600160401b031660208401528a518b90899081106129f2576129f26152e2565b6020026020010151604001518581518110612a0f57612a0f6152e2565b6020908102919091018101516001600160401b031683525f9083015263ffffffff43166040830152612aa9565b5f8260200151600f0b1315612aa957612a5d836020015183602001516138bf565b6001600160401b039081166020850181905284519091161015612a9357604051636c9be0bf60e01b815260040160405180910390fd5b612a9d89436156df565b63ffffffff1660408301525b612abe8c612ab6896135ac565b8686866139a8565b7f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd8c612aec6109078a6135ac565b86612afe865f015187602001516138bf565b8660400151604051612b1495949392919061556d565b60405180910390a15050600190920191506127d69050565b5050506001016126f2565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610776906138ac565b82612b70816137e3565b612b8d5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff16612bcf576001600160a01b0384165f90815260a460205260409020805460ff191660011790555b836001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c9437138484604051612c0a929190615760565b60405180910390a250505050565b606654600290600490811603612c415760405163840a48d560e01b815260040160405180910390fd5b82612c4b816137e3565b612c685760405163932d94f760e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0385811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015612ccc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf091906152c3565b612d0d5760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b612d1c602085018561533b565b9050811015612ee457604080518082019091525f9080612d3f60208801886148cf565b6001600160a01b03168152602001868060200190612d5d919061533b565b85818110612d6d57612d6d6152e2565b9050602002016020810190612d829190615314565b63ffffffff90811690915260208083015183516001600160a01b03165f90815260989092526040909120929350612dbe9291908116906137cc16565b612ddb57604051631fb1705560e21b815260040160405180910390fd5b612de586826138fb565b15612e0357604051636c6c6e2760e11b815260040160405180910390fd5b612e2c612e0f826135ac565b6001600160a01b0388165f908152609c602052604090209061388d565b50612e5886609a5f612e3d856135ac565b81526020019081526020015f2061389890919063ffffffff16565b50856001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612e92919061532d565b60405180910390a26001600160a01b0386165f908152609e60205260408120600191612ebd846135ac565b815260208101919091526040015f20805460ff191691151591909117905550600101612d0f565b50612ef561038360208501856148cf565b6001600160a01b031663c63fd50285612f1160208701876148cf565b612f1e602088018861533b565b612f2b60408a018a6155be565b6040518763ffffffff1660e01b8152600401612f4c96959493929190615773565b5f604051808303815f87803b158015612f63575f5ffd5b505af1158015611bfe573d5f5f3e3d5ffd5b5f61099a609a5f612f85856135ac565b81526020019081526020015f206137b8565b83612fa1816137e3565b612fbe5760405163932d94f760e01b815260040160405180910390fd5b6040805180820182526001600160a01b03871680825263ffffffff80881660208085018290525f93845260989052939091209192612ffd92916137cc16565b61301a57604051631fb1705560e21b815260040160405180910390fd5b5f613024826135ac565b90505f5b84811015611bfe5761306d868683818110613045576130456152e2565b905060200201602081019061305a91906148cf565b5f84815260996020526040902090613f8d565b61308a576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee838787848181106130be576130be6152e2565b90506020020160208101906130d391906148cf565b6040516130e1929190615380565b60405180910390a1600101613028565b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052650100000000008304821694840194909452600160481b9091041660608201819052849391929190158015906131725750826060015163ffffffff164310155b15613181575050604081015160015b9590945092505050565b6001600160a01b0381165f90815260986020526040812061099a906137b8565b6001600160a01b0382165f908152609f60205260408120606091906131d49082611936866135ac565b949350505050565b5f54610100900460ff16158080156131fa57505f54600160ff909116105b806132135750303b15801561321357505f5460ff166001145b61327b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561329c575f805461ff0019166101001790555b6132a58261377b565b6132ae83613ffb565b80156132f4575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b505050565b81613303816137e3565b6133205760405163932d94f760e01b815260040160405180910390fd5b60405163b526578760e01b81526001600160a01b03848116600483015283169063b526578790602401602060405180830381865afa158015613364573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061338891906152c3565b6133a557604051631d0b13c160e31b815260040160405180910390fd5b6001600160a01b038381165f90815260976020526040902080546001600160a01b0319169184169190911790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85836133fd81610f68565b604080516001600160a01b039384168152929091166020830152016132eb565b613425613fa1565b6001600160a01b03811661348a5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401613272565b61349381613ffb565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061351691906157bf565b6001600160a01b0316336001600160a01b0316146135475760405163794821ff60e01b815260040160405180910390fd5b6066548019821981161461356e5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f815f0151826020015163ffffffff166040516020016135f792919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b60405160208183030381529060405261099a906157da565b6040805180820182525f80825260208083018290528351606081018552828152808201839052808501839052845180860186526001600160a01b03898116855260a184528685209088168552909252938220929392819061366f906138ac565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff16918101829052919250431015613711579092509050613773565b613722815f015182602001516138bf565b6001600160401b0316815260208101515f600f9190910b121561376057613751826020015182602001516138bf565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f61099a825490565b5f61077683836140ad565b5f8181526001830160205260408120541515610776565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015613869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061099a91906152c3565b5f61077683836140d3565b5f610776836001600160a01b0384166140d3565b5f61099a82670de0b6b3a764000061411f565b5f610776826001600160401b038516615540565b5f61077683670de0b6b3a764000084614163565b5f6107768383670de0b6b3a7640000614163565b6001600160a01b0382165f908152609e6020526040812081908161391e856135ac565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff16928201929092529150806131d457506020015163ffffffff164311159392505050565b6001600160a01b0381165f9081526001830160205260408120541515610776565b5f6107768383670de0b6b3a76400006001614248565b6020808301516001600160a01b038088165f90815260a284526040808220928816825291909352909120546001600160401b03908116911614613a6e57602082810180516001600160a01b038881165f81815260a286526040808220938a1680835293875290819020805467ffffffffffffffff19166001600160401b0395861617905593518451918252948101919091529216908201527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559060600160405180910390a15b6001600160a01b038086165f90815260a060209081526040808320888452825280832093871683529281529082902083518154928501519385015163ffffffff16600160c01b0263ffffffff60c01b196001600160801b038616600160401b026001600160c01b03199095166001600160401b03909316929092179390931716919091179055600f0b15613b50576001600160a01b0385165f908152609f602090815260408083208784529091529020613b289084613898565b506001600160a01b0385165f908152609d60205260409020613b4a908561388d565b50610c74565b80516001600160401b03165f03610c74576001600160a01b0385165f908152609f602090815260408083208784529091529020613b8d9084613f8d565b506001600160a01b0385165f908152609f602090815260408083208784529091529020613bb9906137b8565b5f03610c74576001600160a01b0385165f908152609d602052604090206123bb9085613f82565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613c0f9043836142a1565b604080516001600160a01b038086168252841660208201526001600160401b038316918101919091527f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c906060016132eb565b60605f610776836142b5565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f81118015613cb857508261ffff1682105b15610c74576001600160a01b038086165f90815260a3602090815260408083209388168352929052908120613cec9061430e565b90505f5f613cfb88848961360f565b91509150806040015163ffffffff16431015613d1957505050610c74565b613d2688848985856139a8565b6001600160a01b038089165f90815260a360209081526040808320938b16835292905220613d5390614360565b50613d5d856157fd565b9450613d6884615815565b9350505050613ca6565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b909204166060820181905215801590613dee5750806060015163ffffffff164310155b15613e0857604081015163ffffffff168152600160208201525b63ffffffff82166040820152613e3e7f0000000000000000000000000000000000000000000000000000000000000000436156df565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db91016132eb565b5f5f613f35613f21846143dd565b8554613f309190600f0b61582a565b61444a565b8454909150600160801b9004600f90810b9082900b12613f6857604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f61077683836144b3565b5f610776836001600160a01b0384166144b3565b6033546001600160a01b03163314611e625760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401613272565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f6107768383670de0b6b3a7640000614596565b5f6140718460995f6112ae896135ac565b801561407a5750815b8015611fd957505090516001600160401b031615159392505050565b5f6107766001600160401b03808516908416615851565b5f825f0182815481106140c2576140c26152e2565b905f5260205f200154905092915050565b5f81815260018301602052604081205461411857508154600181810184555f84815260208082209093018490558454848252828601909352604090209190915561099a565b505f61099a565b81545f90801561415b57614145846141386001846154ea565b5f91825260209091200190565b54600160201b90046001600160e01b03166131d4565b509092915050565b5f80805f19858709858702925082811083820303915050805f0361419a578382816141905761419061587e565b0492505050610776565b8084116141e15760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401613272565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f5f614255868686614163565b9050600183600281111561426b5761426b615892565b14801561428757505f84806142825761428261587e565b868809115b15611fd9576142976001826158a6565b9695505050505050565b6132f483836001600160401b0384166145de565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561430257602002820191905f5260205f20905b8154815260200190600101908083116142ee575b50505050509050919050565b5f6143288254600f81810b600160801b909204900b131590565b1561434657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f61437a8254600f81810b600160801b909204900b131590565b1561439857604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b5f6001600160ff1b038211156144465760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401613272565b5090565b80600f81900b81146144ae5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401613272565b919050565b5f818152600183016020526040812054801561458d575f6144d56001836154ea565b85549091505f906144e8906001906154ea565b9050818114614547575f865f018281548110614506576145066152e2565b905f5260205f200154905080875f018481548110614526576145266152e2565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080614558576145586158b9565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061099a565b5f91505061099a565b82545f90816145a7868683856146e1565b905080156145d4576145be866141386001846154ea565b54600160201b90046001600160e01b0316610771565b5091949350505050565b82548015614694575f6145f6856141386001856154ea565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156146485760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff8086169116036146925782614669866141386001866154ea565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f5b81831015610961575f6146f68484614734565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156147205780925061472e565b61472b8160016158a6565b93505b506146e3565b5f61474260028484186158cd565b610776908484166158a6565b6001600160a01b0381168114613493575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561479857614798614762565b60405290565b604051601f8201601f191681016001600160401b03811182821017156147c6576147c6614762565b604052919050565b803563ffffffff811681146144ae575f5ffd5b5f604082840312156147f1575f5ffd5b604080519081016001600160401b038111828210171561481357614813614762565b60405290508082356148248161474e565b8152614832602084016147ce565b60208201525092915050565b5f5f5f60808486031215614850575f5ffd5b833561485b8161474e565b925061486a85602086016147e1565b9150606084013561487a8161474e565b809150509250925092565b81516001600160401b03168152602080830151600f0b9082015260408083015163ffffffff16908201526060810161099a565b5f602082840312156148c8575f5ffd5b5035919050565b5f602082840312156148df575f5ffd5b81356107768161474e565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b828110156149435761492d8683516148ea565b604095909501946020919091019060010161491a565b5093949350505050565b602081525f6107766020830184614908565b5f6040828403121561496f575f5ffd5b61077683836147e1565b5f5f83601f840112614989575f5ffd5b5081356001600160401b0381111561499f575f5ffd5b6020830191508360208260051b850101111561191d575f5ffd5b5f5f5f604084860312156149cb575f5ffd5b83356149d68161474e565b925060208401356001600160401b038111156149f0575f5ffd5b6149fc86828701614979565b9497909650939450505050565b5f6001600160401b03821115614a2157614a21614762565b5060051b60200190565b5f82601f830112614a3a575f5ffd5b8135614a4d614a4882614a09565b61479e565b8082825260208201915060208360051b860101925085831115614a6e575f5ffd5b602085015b83811015614a94578035614a868161474e565b835260209283019201614a73565b5095945050505050565b5f5f5f5f60a08587031215614ab1575f5ffd5b614abb86866147e1565b935060408501356001600160401b03811115614ad5575f5ffd5b614ae187828801614a2b565b93505060608501356001600160401b03811115614afc575f5ffd5b614b0887828801614a2b565b925050614b17608086016147ce565b905092959194509250565b5f8151808452602084019350602083015f5b82811015614943578151865260209586019590910190600101614b34565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015614ba957603f19878603018452614b94858351614b22565b94506020938401939190910190600101614b78565b50929695505050505050565b5f5f60408385031215614bc6575f5ffd5b8235614bd18161474e565b915060208301356001600160401b03811115614beb575f5ffd5b830160a08186031215614bfc575f5ffd5b809150509250929050565b5f5f60408385031215614c18575f5ffd5b8235614c238161474e565b91506020830135614bfc8161474e565b5f8151808452602084019350602083015f5b8281101561494357614c7e86835180516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b6060959095019460209190910190600101614c45565b604081525f614ca66040830185614908565b8281036020840152611fd98185614c33565b5f8151808452602084019350602083015f5b828110156149435781516001600160a01b0316865260209586019590910190600101614cca565b602081525f6107766020830184614cb8565b5f5f60408385031215614d14575f5ffd5b82356001600160401b03811115614d29575f5ffd5b614d3585828601614a2b565b9250506020830135614bfc8161474e565b602080825282518282018190525f918401906040840190835b81811015614d865783516001600160401b0316835260209384019390920191600101614d5f565b509095945050505050565b5f5f5f5f5f60608688031215614da5575f5ffd5b8535614db08161474e565b945060208601356001600160401b03811115614dca575f5ffd5b614dd688828901614979565b90955093505060408601356001600160401b03811115614df4575f5ffd5b614e0088828901614979565b969995985093965092949392505050565b5f5f5f5f60608587031215614e24575f5ffd5b8435614e2f8161474e565b9350614e3d602086016147ce565b925060408501356001600160401b03811115614e57575f5ffd5b614e6387828801614979565b95989497509550505050565b5f5f60408385031215614e80575f5ffd5b8235614e8b8161474e565b915060208301356001600160401b03811115614ea5575f5ffd5b614eb185828601614a2b565b9150509250929050565b5f5f60408385031215614ecc575f5ffd5b8235614ed78161474e565b9150614ee5602084016147ce565b90509250929050565b5f60208284031215614efe575f5ffd5b813560ff81168114610776575f5ffd5b5f5f60608385031215614f1f575f5ffd5b8235614f2a8161474e565b9150614ee584602085016147e1565b5f60608284031215614f49575f5ffd5b50919050565b5f60208284031215614f5f575f5ffd5b81356001600160401b03811115614f74575f5ffd5b6131d484828501614f39565b5f5f5f60808486031215614f92575f5ffd5b83356001600160401b03811115614fa7575f5ffd5b614fb386828701614a2b565b93505061486a85602086016147e1565b602081525f6107766020830184614c33565b5f5f5f60608486031215614fe7575f5ffd5b8335614ff28161474e565b925060208401356001600160401b0381111561500c575f5ffd5b61501886828701614a2b565b925050615027604085016147ce565b90509250925092565b5f5f60408385031215615041575f5ffd5b823561504c8161474e565b915060208301356001600160401b03811115615066575f5ffd5b8301601f81018513615076575f5ffd5b8035615084614a4882614a09565b8082825260208201915060208360051b8501019250878311156150a5575f5ffd5b602084015b838110156151ca5780356001600160401b038111156150c7575f5ffd5b85016080818b03601f190112156150dc575f5ffd5b6150e4614776565b6150f18b602084016147e1565b815260608201356001600160401b0381111561510b575f5ffd5b61511a8c602083860101614a2b565b60208301525060808201356001600160401b03811115615138575f5ffd5b6020818401019250508a601f83011261514f575f5ffd5b813561515d614a4882614a09565b8082825260208201915060208360051b86010192508d83111561517e575f5ffd5b6020850194505b828510156151b45784356001600160401b03811681146151a3575f5ffd5b825260209485019490910190615185565b60408401525050845250602092830192016150aa565b50809450505050509250929050565b5f5f5f604084860312156151eb575f5ffd5b83356151f68161474e565b925060208401356001600160401b03811115615210575f5ffd5b8401601f81018613615220575f5ffd5b80356001600160401b03811115615235575f5ffd5b866020828401011115615246575f5ffd5b939660209190910195509293505050565b5f5f60408385031215615268575f5ffd5b82356152738161474e565b915060208301356001600160401b0381111561528d575f5ffd5b614eb185828601614f39565b5f5f604083850312156152aa575f5ffd5b82356152b58161474e565b946020939093013593505050565b5f602082840312156152d3575f5ffd5b81518015158114610776575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261530a575f5ffd5b9190910192915050565b5f60208284031215615324575f5ffd5b610776826147ce565b6040810161099a82846148ea565b5f5f8335601e19843603018112615350575f5ffd5b8301803591506001600160401b03821115615369575f5ffd5b6020019150600581901b360382131561191d575f5ffd5b6060810161538e82856148ea565b6001600160a01b039290921660409190910152919050565b604081525f6153b86040830185614cb8565b8281036020840152611fd98185614cb8565b5f602082840312156153da575f5ffd5b81516001600160401b038111156153ef575f5ffd5b8201601f810184136153ff575f5ffd5b805161540d614a4882614a09565b8082825260208201915060208360051b85010192508683111561542e575f5ffd5b602084015b838110156154cb5780516001600160401b03811115615450575f5ffd5b8501603f81018913615460575f5ffd5b6020810151615471614a4882614a09565b808282526020820191506020808460051b8601010192508b831115615494575f5ffd5b6040840193505b828410156154b657835182526020938401939091019061549b565b86525050602093840193919091019050615433565b509695505050505050565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561099a5761099a6154d6565b6001600160401b03828116828216039081111561099a5761099a6154d6565b5f81600f0b60016001607f1b03198103615538576155386154d6565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b03198212171561099a5761099a6154d6565b6001600160a01b038616815260c0810161558a60208301876148ea565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e198436030181126155d3575f5ffd5b8301803591506001600160401b038211156155ec575f5ffd5b60200191503681900382131561191d575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b03881681525f60c08201615646602084018a6148ea565b60c060608401528690528660e083015f5b8881101561568757823561566a8161474e565b6001600160a01b0316825260209283019290910190600101615657565b50838103608085015261569a8188614b22565b91505082810360a08401526156b0818587615600565b9a9950505050505050505050565b5f602082840312156156ce575f5ffd5b813561ffff81168114610776575f5ffd5b63ffffffff818116838216019081111561099a5761099a6154d6565b8183526020830192505f815f5b848110156149435763ffffffff61571e836147ce565b1686526020958601959190910190600101615708565b6001600160a01b038581168252841660208201526060604082018190525f9061429790830184866156fb565b602081525f6131d4602083018486615600565b6001600160a01b038781168252861660208201526080604082018190525f9061579f90830186886156fb565b82810360608401526157b2818587615600565b9998505050505050505050565b5f602082840312156157cf575f5ffd5b81516107768161474e565b80516020808301519190811015614f49575f1960209190910360031b1b16919050565b5f6001820161580e5761580e6154d6565b5060010190565b5f81615823576158236154d6565b505f190190565b8082018281125f831280158216821582161715615849576158496154d6565b505092915050565b600f82810b9082900b0360016001607f1b0319811260016001607f1b038213171561099a5761099a6154d6565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b8082018082111561099a5761099a6154d6565b634e487b7160e01b5f52603160045260245ffd5b5f826158e757634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220c3ba315211dbc6ce0ca79751329dee4d7a411ed4af89203aa0cc2a16f56ef31764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R4\x80\x15a\0\x10W__\xFD[P`@Qa[\x9C8\x03\x80a[\x9C\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\x80V[\x82\x85\x83\x83\x87`\x01`\x01`\xA0\x1B\x03\x81\x16a\0[W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\x80R\x92\x83\x16`\xA0Rc\xFF\xFF\xFF\xFF\x91\x82\x16`\xC0R\x16`\xE0R\x16a\x01\0Ra\0\x8Ba\0\x95V[PPPPPa\x01\xE9V[_Ta\x01\0\x90\x04`\xFF\x16\x15a\x01\0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[_T`\xFF\x90\x81\x16\x14a\x01OW_\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01eW__\xFD[PV[\x80Qc\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01{W__\xFD[\x91\x90PV[_____`\xA0\x86\x88\x03\x12\x15a\x01\x94W__\xFD[\x85Qa\x01\x9F\x81a\x01QV[` \x87\x01Q\x90\x95Pa\x01\xB0\x81a\x01QV[`@\x87\x01Q\x90\x94Pa\x01\xC1\x81a\x01QV[\x92Pa\x01\xCF``\x87\x01a\x01hV[\x91Pa\x01\xDD`\x80\x87\x01a\x01hV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0QaY\"a\x02z_9_\x81\x81a\x03\xF9\x01Ra8!\x01R_\x81\x81a\x05H\x01Ra>\x19\x01R_\x81\x81a\x03\x1E\x01R\x81\x81a\"v\x01Ra)~\x01R_\x81\x81a\x06\xFA\x01R\x81\x81a\x0C\xCF\x01R\x81\x81a\x16T\x01R\x81\x81a\x1C\xBB\x01R\x81\x81a\x1D%\x01Ra,\x87\x01R_\x81\x81a\x05o\x01R\x81\x81a\x07\x92\x01R\x81\x81a\x1D\xCA\x01Ra4\x98\x01RaY\"_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x81W_5`\xE0\x1C\x80cn\x87]\xBA\x11a\x01VW\x80c\xA9\x84\xEB:\x11a\0\xCAW\x80c\xC2!\xD8\xAE\x11a\0\x84W\x80c\xC2!\xD8\xAE\x14a\x06\xBCW\x80c\xCDm\xC6\x87\x14a\x06\xCFW\x80c\xD3\xD9o\xF4\x14a\x06\xE2W\x80c\xDF\\\xF7#\x14a\x06\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x07\x1CW\x80c\xFA\xBC\x1C\xBC\x14a\x07/W__\xFD[\x80c\xA9\x84\xEB:\x14a\x06\x0EW\x80c\xAD\xC2\xE3\xD9\x14a\x06AW\x80c\xB2Dz\xF7\x14a\x06TW\x80c\xB6k\xD9\x89\x14a\x06gW\x80c\xB9\xFB\xAE\xD1\x14a\x06zW\x80c\xBA\x1A\x84\xE5\x14a\x06\xA9W__\xFD[\x80c\x8C\xE6HT\x11a\x01\x1BW\x80c\x8C\xE6HT\x14a\x05\x91W\x80c\x8D\xA5\xCB[\x14a\x05\xB1W\x80c\x94\xD7\xD0\x0C\x14a\x05\xC2W\x80c\x95(\x99\xEE\x14a\x05\xD5W\x80c\xA93>\xC8\x14a\x05\xE8W\x80c\xA9\x82\x18!\x14a\x05\xFBW__\xFD[\x80cn\x87]\xBA\x14a\x05\x15W\x80cqP\x18\xA6\x14a\x05(W\x80cy\xAEP\xCD\x14a\x050W\x80c{\xC1\xEFa\x14a\x05CW\x80c\x88o\x11\x95\x14a\x05jW__\xFD[\x80cFW\xE2j\x11a\x01\xF8W\x80cY\\jg\x11a\x01\xB2W\x80cY\\jg\x14a\x04\x87W\x80cZ\xC8j\xB7\x14a\x04\x8FW\x80c\\\x97Z\xBB\x14a\x04\xB2W\x80cg\r;\xA2\x14a\x04\xC4W\x80cl\xFBD\x81\x14a\x04\xD7W\x80cn4\x92\xB5\x14a\x05\x02W__\xFD[\x80cFW\xE2j\x14a\x03\xF4W\x80cJ\x10\xFF\xE5\x14a\x04\x1BW\x80cKPF\xEF\x14a\x04;W\x80cP\xFE\xEA \x14a\x04NW\x80cTz\xFB\x87\x14a\x04aW\x80cV\xC4\x83\xE6\x14a\x04tW__\xFD[\x80c)\x81\xEBw\x11a\x02IW\x80c)\x81\xEBw\x14a\x03\x19W\x80c+\xAB,J\x14a\x03UW\x80c0L\x10\xCD\x14a\x03uW\x80c65 W\x14a\x03\xA0W\x80c@\x12\r\xAB\x14a\x03\xB3W\x80cAw\xA8|\x14a\x03\xD4W__\xFD[\x80c\x10\xE1\xB9\xB8\x14a\x02\x85W\x80c\x13d9\xDD\x14a\x02\xAEW\x80c\x15\xFEP(\x14a\x02\xC3W\x80c&\r\xC7X\x14a\x02\xE3W\x80c&\x1F\x84\xE0\x14a\x03\x06W[__\xFD[a\x02\x98a\x02\x936`\x04aH>V[a\x07BV[`@Qa\x02\xA5\x91\x90aH\x85V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x02\xBC6`\x04aH\xB8V[a\x07}V[\0[a\x02\xD6a\x02\xD16`\x04aH\xCFV[a\x08RV[`@Qa\x02\xA5\x91\x90aIMV[a\x02\xF6a\x02\xF16`\x04aI_V[a\tiV[`@Q\x90\x15\x15\x81R` \x01a\x02\xA5V[a\x02\xC1a\x03\x146`\x04aI\xB9V[a\t\xA0V[a\x03@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x03ha\x03c6`\x04aJ\x9EV[a\x0C{V[`@Qa\x02\xA5\x91\x90aKRV[a\x03\x88a\x03\x836`\x04aH\xCFV[a\x0FhV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x02\xC1a\x03\xAE6`\x04aK\xB5V[a\x0F\x97V[a\x03\xC6a\x03\xC16`\x04aL\x07V[a\x17\xA9V[`@Qa\x02\xA5\x92\x91\x90aL\x94V[a\x03\xE7a\x03\xE26`\x04aI_V[a\x19$V[`@Qa\x02\xA5\x91\x90aL\xF1V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04.a\x04)6`\x04aM\x03V[a\x19HV[`@Qa\x02\xA5\x91\x90aMFV[a\x02\xC1a\x04I6`\x04aM\x91V[a\x19\xF0V[a\x02\xC1a\x04\\6`\x04aN\x11V[a\x1A\xAAV[a\x04.a\x04o6`\x04aNoV[a\x1C\x08V[a\x02\xC1a\x04\x826`\x04aN\xBBV[a\x1C\xB0V[a\x02\xC1a\x1D\xB5V[a\x02\xF6a\x04\x9D6`\x04aN\xEEV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02\xA5V[a\x02\xF6a\x04\xD26`\x04aO\x0EV[a\x1EdV[a\x04\xEAa\x04\xE56`\x04aL\x07V[a\x1EuV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x02\xC1a\x05\x106`\x04aOOV[a\x1F\xE2V[a\x03\xE7a\x05#6`\x04aI_V[a#\xC3V[a\x02\xC1a#\xD4V[a\x02\xD6a\x05>6`\x04aH\xCFV[a#\xE5V[a\x03@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xA4a\x05\x9F6`\x04aO\x80V[a$\xBFV[`@Qa\x02\xA5\x91\x90aO\xC3V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x88V[a\x04.a\x05\xD06`\x04aO\xD5V[a%\x84V[a\x02\xC1a\x05\xE36`\x04aP0V[a&pV[a\x04\xEAa\x05\xF66`\x04aL\x07V[a+7V[a\x02\xC1a\x06\t6`\x04aQ\xD9V[a+fV[a\x04\xEAa\x06\x1C6`\x04aL\x07V[`\xA2` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\xC1a\x06O6`\x04aRWV[a,\x18V[a\x04\xB6a\x06b6`\x04aI_V[a/uV[a\x02\xC1a\x06u6`\x04aN\x11V[a/\x97V[a\x06\x8Da\x06\x886`\x04aH\xCFV[a0\xF1V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xA5V[a\x04\xB6a\x06\xB76`\x04aH\xCFV[a1\x8BV[a\x03\xE7a\x06\xCA6`\x04aO\x0EV[a1\xABV[a\x02\xC1a\x06\xDD6`\x04aR\x99V[a1\xDCV[a\x02\xC1a\x06\xF06`\x04aL\x07V[a2\xF9V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x07*6`\x04aH\xCFV[a4\x1DV[a\x02\xC1a\x07=6`\x04aH\xB8V[a4\x96V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x07q\x85a\x07k\x86a5\xACV[\x85a6\x0FV[\x92PPP[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x03\x91\x90aR\xC3V[a\x08 W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x08EW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08N\x82a7{V[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\x08v\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x91Wa\x08\x91aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xAFW\x90P[P\x90P_[\x82\x81\x10\x15a\taW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\t<\x90a\t\x07\x90\x83a7\xC1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\tNWa\tNaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x08\xDAV[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\t\x9A\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[\x92\x91PPV[\x82a\t\xAA\x81a7\xE3V[a\t\xC7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a\t\xFFW`@Qc\xCA\xF9ua`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x0CtW_`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x85\x81\x81\x10a\n6Wa\n6aR\xE2V[\x90P` \x02\x81\x01\x90a\nH\x91\x90aR\xF6V[a\nV\x90` \x81\x01\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\n\xA0\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a8\x8D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xBDW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x81RP`@Qa\x0B\x13\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA1_a\x0B%\x82a5\xACV[\x90P_[\x86\x86\x85\x81\x81\x10a\x0B;Wa\x0B;aR\xE2V[\x90P` \x02\x81\x01\x90a\x0BM\x91\x90aR\xF6V[a\x0B[\x90` \x81\x01\x90aS;V[\x90P\x81\x10\x15a\x0CiWa\x0B\xD1\x87\x87\x86\x81\x81\x10a\x0ByWa\x0ByaR\xE2V[\x90P` \x02\x81\x01\x90a\x0B\x8B\x91\x90aR\xF6V[a\x0B\x99\x90` \x81\x01\x90aS;V[\x83\x81\x81\x10a\x0B\xA9Wa\x0B\xA9aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xBE\x91\x90aH\xCFV[_\x84\x81R`\x99` R`@\x90 \x90a8\x98V[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x88\x88\x87\x81\x81\x10a\x0C\x06Wa\x0C\x06aR\xE2V[\x90P` \x02\x81\x01\x90a\x0C\x18\x91\x90aR\xF6V[a\x0C&\x90` \x81\x01\x90aS;V[\x84\x81\x81\x10a\x0C6Wa\x0C6aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x0CK\x91\x90aH\xCFV[`@Qa\x0CY\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x0B)V[PPP`\x01\x01a\n\x01V[PPPPPV[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x96Wa\x0C\x96aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xC9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xB4W\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x1B\x92\x91\x90aS\xA6V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\\\x91\x90\x81\x01\x90aS\xCAV[\x90P_[\x85Q\x81\x10\x15a\x0F^W_\x86\x82\x81Q\x81\x10a\r|Wa\r|aR\xE2V[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\x9FWa\r\x9FaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a\r\xDBWa\r\xDBaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a\x0FTW_\x87\x82\x81Q\x81\x10a\x0E\x04Wa\x0E\x04aR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa\x0E?\x90a8\xACV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x0EXWPPa\x0FLV[_a\x0Ed\x85\x8D\x85a\x07BV[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a\x0E\x8CWP_\x81` \x01Q`\x0F\x0B\x12[\x15a\x0E\xAEWa\x0E\xA2\x81_\x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a\x0E\xC9\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a8\xD3V[\x90Pa\x0F\x10\x81\x89\x89\x81Q\x81\x10a\x0E\xE1Wa\x0E\xE1aR\xE2V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x0E\xFAWa\x0E\xFAaR\xE2V[` \x02` \x01\x01Qa8\xE7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a\x0F\"Wa\x0F\"aR\xE2V[` \x02` \x01\x01Q\x86\x81Q\x81\x10a\x0F;Wa\x0F;aR\xE2V[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a\r\xE8V[PP`\x01\x01a\r`V[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\x0F\x90W\x80a\x07vV[P\x90\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x0F\xC0W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0F\xCA\x81a7\xE3V[a\x0F\xE7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01` \x81\x01\x90a\x10\x15\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x106a\x100` \x87\x01\x87aH\xCFV[\x83a8\xFBV[\x90Pa\x10E``\x86\x01\x86aS;V[\x90Pa\x10T`@\x87\x01\x87aS;V[\x90P\x14a\x10tW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 a\x10\xA6\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[a\x10\xC3W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x10\xE1W`@Qc\xEB\xBF\xF4\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x10\xEF`@\x87\x01\x87aS;V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x08Wa\x11\x08aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x111W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x11C`@\x88\x01\x88aS;V[\x90P\x81\x10\x15a\x17:W\x80\x15\x80a\x11\xD6WPa\x11a`@\x88\x01\x88aS;V[a\x11l`\x01\x84aT\xEAV[\x81\x81\x10a\x11{Wa\x11{aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x11\x90\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16a\x11\xA6`@\x89\x01\x89aS;V[\x83\x81\x81\x10a\x11\xB6Wa\x11\xB6aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x11\xCB\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x11\xF3W`@Qc\x9F\x1C\x80S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\0``\x88\x01\x88aS;V[\x82\x81\x81\x10a\x12\x10Wa\x12\x10aR\xE2V[\x90P` \x02\x015_\x10\x80\x15a\x12PWPg\r\xE0\xB6\xB3\xA7d\0\0a\x126``\x89\x01\x89aS;V[\x83\x81\x81\x10a\x12FWa\x12FaR\xE2V[\x90P` \x02\x015\x11\x15[a\x12mW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xC9a\x12}`@\x89\x01\x89aS;V[\x83\x81\x81\x10a\x12\x8DWa\x12\x8DaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x12\xA2\x91\x90aH\xCFV[`\x99_a\x12\xAE\x88a5\xACV[\x81R` \x01\x90\x81R` \x01_ a9q\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xE6W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x138a\x12\xF8` \x8B\x01\x8BaH\xCFV[a\x13\x01\x88a5\xACV[a\x13\x0E`@\x8D\x01\x8DaS;V[\x87\x81\x81\x10a\x13\x1EWa\x13\x1EaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x133\x91\x90aH\xCFV[a6\x0FV[\x80Q\x91\x93P\x91P`\x01`\x01`@\x1B\x03\x16_\x03a\x13UWPPa\x172V[_a\x13\x90a\x13f``\x8C\x01\x8CaS;V[\x86\x81\x81\x10a\x13vWa\x13vaR\xE2V[\x85Q`\x01`\x01`@\x1B\x03\x16\x92` \x90\x91\x02\x015\x90Pa9\x92V[\x83Q\x90\x91Pa\x13\xAB`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x83\x16a8\xD3V[\x86\x86\x81Q\x81\x10a\x13\xBDWa\x13\xBDaR\xE2V[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\x13\xDA\x91\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\x13\xF7\x90\x83\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\x14\x17\x90\x83\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x152W_a\x14za\x14F``\x8E\x01\x8EaS;V[\x88\x81\x81\x10a\x14VWa\x14VaR\xE2V[\x90P` \x02\x015\x85` \x01Qa\x14k\x90aU\x1CV[`\x01`\x01`\x80\x1B\x03\x16\x90a9\x92V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\x14\x97\x91\x90aU@V[`\x0F\x0B\x90RP\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDDa\x14\xCB` \x8E\x01\x8EaH\xCFV[\x8A\x8E\x80`@\x01\x90a\x14\xDC\x91\x90aS;V[\x8A\x81\x81\x10a\x14\xECWa\x14\xECaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15\x01\x91\x90aH\xCFV[a\x15\x12\x88_\x01Q\x89` \x01Qa8\xBFV[\x88`@\x01Q`@Qa\x15(\x95\x94\x93\x92\x91\x90aUmV[`@Q\x80\x91\x03\x90\xA1P[a\x15\x84a\x15B` \x8D\x01\x8DaH\xCFV[a\x15K\x8Aa5\xACV[a\x15X`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x15hWa\x15haR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15}\x91\x90aH\xCFV[\x87\x87a9\xA8V[\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDDa\x15\xB2` \x8D\x01\x8DaH\xCFV[\x89a\x15\xC0`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x15\xD0Wa\x15\xD0aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15\xE5\x91\x90aH\xCFV[\x86Q`@Qa\x15\xF9\x94\x93\x92\x91\x90C\x90aUmV[`@Q\x80\x91\x03\x90\xA1a\x16Ja\x16\x11` \x8D\x01\x8DaH\xCFV[a\x16\x1E`@\x8E\x01\x8EaS;V[\x88\x81\x81\x10a\x16.Wa\x16.aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x16C\x91\x90aH\xCFV[\x86Qa;\xE0V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xEEt\x93\x7Fa\x16\x86` \x8E\x01\x8EaH\xCFV[a\x16\x93`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x16\xA3Wa\x16\xA3aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x16\xB8\x91\x90aH\xCFV[\x87Q`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\x17W__\xFD[PZ\xF1\x15\x80\x15a\x17)W=__>=_\xFD[PPPPPPPP[`\x01\x01a\x116V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x17i` \x88\x01\x88aH\xCFV[\x84a\x17w`@\x8A\x01\x8AaS;V[\x85a\x17\x85`\x80\x8D\x01\x8DaU\xBEV[`@Qa\x17\x98\x97\x96\x95\x94\x93\x92\x91\x90aV(V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x17\xCE\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xE9Wa\x17\xE9aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18-W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x07W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18IWa\x18IaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x92W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x18gW\x90P[P\x90P_[\x83\x81\x10\x15a\x19\x15W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x18\xC4\x90a\t\x07\x90\x84a7\xC1V[\x90P\x80\x84\x83\x81Q\x81\x10a\x18\xD9Wa\x18\xD9aR\xE2V[` \x02` \x01\x01\x81\x90RPa\x18\xEF\x89\x82\x8Aa\x07BV[\x83\x83\x81Q\x81\x10a\x19\x01Wa\x19\x01aR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x18\x97V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x07v`\x99_a\x196\x86a5\xACV[\x81R` \x01\x90\x81R` \x01_ a<bV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19dWa\x19daGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\taWa\x19\xBE\x85\x82\x81Q\x81\x10a\x19\xB0Wa\x19\xB0aR\xE2V[` \x02` \x01\x01Q\x85a+7V[\x82\x82\x81Q\x81\x10a\x19\xD0Wa\x19\xD0aR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19\x92V[`fT_\x90`\x01\x90\x81\x16\x03a\x1A\x18W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x1A8W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1A\xA1Wa\x1A\x99\x87\x87\x87\x84\x81\x81\x10a\x1AXWa\x1AXaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1Am\x91\x90aH\xCFV[\x86\x86\x85\x81\x81\x10a\x1A\x7FWa\x1A\x7FaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1A\x94\x91\x90aV\xBEV[a<nV[`\x01\x01a\x1A:V[PPPPPPPV[\x83a\x1A\xB4\x81a7\xE3V[a\x1A\xD1W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x16\x81Rc\xFF\xFF\xFF\xFF\x85\x16` \x82\x01R_a\x1A\xFD\x82a5\xACV[\x90Pa\x1B>\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a7\xCC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1B[W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1B\xFEWa\x1Bz\x86\x86\x83\x81\x81\x10a\x0B\xA9Wa\x0B\xA9aR\xE2V[a\x1B\x97W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x84\x81\x81\x10a\x1B\xCBWa\x1B\xCBaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1B\xE0\x91\x90aH\xCFV[`@Qa\x1B\xEE\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x1B]V[PPPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C$Wa\x1C$aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1CMW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\taWa\x1C~\x85\x85\x83\x81Q\x81\x10a\x1CqWa\x1CqaR\xE2V[` \x02` \x01\x01Qa+7V[\x82\x82\x81Q\x81\x10a\x1C\x90Wa\x1C\x90aR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1CRV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D\xABWa\x1C\xE9\x82a7\xE3V[a\x1D\x06W`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DjW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x8E\x91\x90aR\xC3V[a\x1D\xABW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08N\x82\x82a=rV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E;\x91\x90aR\xC3V[a\x1EXW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Eb_\x19a7{V[V[_a\x07v\x83`\x9A_a\x12\xAE\x86a5\xACV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1F\x9FW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1F\x07\x90\x83a?\x13V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a\x1F\x82WPPa\x1F\x9FV[a\x1F\x90\x85\x82` \x01Qa8\xBFV[\x94PPP\x80`\x01\x01\x90Pa\x1E\xCFV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1F\xCF\x90a8\xACV[a\x1F\xD9\x91\x90aT\xFDV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a \x0BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a  a \x1B` \x84\x01\x84aH\xCFV[a7\xE3V[\x80a 9WPa 9a \x1B`@\x84\x01` \x85\x01aH\xCFV[a VW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a e`@\x84\x01\x84aS;V[\x90P\x81\x10\x15a#'W_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a \x8D\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a \xA8`@\x87\x01\x87aS;V[\x85\x81\x81\x10a \xB8Wa \xB8aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a \xCD\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa!\x1A\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a \xFC\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a7\xCCV[a!7W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a!G` \x87\x01\x87aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a!p\x83a5\xACV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a!\xA0W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xDAa!\xAC\x82a5\xACV[`\x9C_a!\xBC` \x89\x01\x89aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a?\x82V[Pa\"\x12a!\xEB` \x86\x01\x86aH\xCFV[`\x9A_a!\xF7\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a?\x8D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\" ` \x85\x01\x85aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa\"X\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a\"\x9B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a\"\xB3` \x88\x01\x88aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\"\xDC\x84a5\xACV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a XV[Pa#;a\x03\x83`@\x84\x01` \x85\x01aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16c0<\xA9Va#V` \x85\x01\x85aH\xCFV[a#f`@\x86\x01` \x87\x01aH\xCFV[a#s`@\x87\x01\x87aS;V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x92\x94\x93\x92\x91\x90aW4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a#\xA9W__\xFD[PZ\xF1\x15\x80\x15a#\xBBW=__>=_\xFD[PPPPPPV[``a\t\x9A`\x9A_a\x196\x85a5\xACV[a#\xDCa?\xA1V[a\x1Eb_a?\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a$\t\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a$$Wa$$aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$hW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a$BW\x90P[P\x90P_[\x82\x81\x10\x15a\taW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a$\x9A\x90a\t\x07\x90\x83a7\xC1V[\x82\x82\x81Q\x81\x10a$\xACWa$\xACaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a$mV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xDBWa$\xDBaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%$W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a$\xF9W\x90P[P\x90P_[\x85Q\x81\x10\x15a%{Wa%V\x86\x82\x81Q\x81\x10a%GWa%GaR\xE2V[` \x02` \x01\x01Q\x86\x86a\x07BV[\x82\x82\x81Q\x81\x10a%hWa%haR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a%)V[P\x94\x93PPPPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xA0Wa%\xA0aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xC9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a%{W`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa&>\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a&\tWa&\taR\xE2V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a@L\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a&PWa&PaR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a%\xCEV[`fT_\x90`\x01\x90\x81\x16\x03a&\x98W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xA1\x83a7\xE3V[a&\xBEW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a&\xCA\x86a0\xF1V[\x91P\x91P\x81a&\xECW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P_\x90P[\x83Q\x81\x10\x15a\x0CtW\x83\x81\x81Q\x81\x10a'\rWa'\raR\xE2V[` \x02` \x01\x01Q`@\x01QQ\x84\x82\x81Q\x81\x10a',Wa',aR\xE2V[` \x02` \x01\x01Q` \x01QQ\x14a'WW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x82\x81Q\x81\x10a'jWa'jaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x80\x82\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x93R`@\x90\x92 \x90\x92Pa'\xAA\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[a'\xC7W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a'\xD2\x87\x83a8\xFBV[\x90P_[\x86\x84\x81Q\x81\x10a'\xE8Wa'\xE8aR\xE2V[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a+,W_\x87\x85\x81Q\x81\x10a(\x0FWa(\x0FaR\xE2V[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a(,Wa(,aR\xE2V[` \x02` \x01\x01Q\x90Pa(C\x89\x82a\xFF\xFFa<nV[__a(R\x8Ba\x07k\x88a5\xACV[\x91P\x91P\x80` \x01Q`\x0F\x0B_\x14a(}W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a(\x8A\x87\x85\x84\x89a@`V[\x90Pa(\xCF\x82_\x01Q\x8C\x8A\x81Q\x81\x10a(\xA5Wa(\xA5aR\xE2V[` \x02` \x01\x01Q`@\x01Q\x87\x81Q\x81\x10a(\xC2Wa(\xC2aR\xE2V[` \x02` \x01\x01Qa@\x96V[`\x0F\x0B` \x83\x01\x81\x90R_\x03a(\xF8W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a*<W\x80\x15a)\xBEWa)ya)\x19\x88a5\xACV[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a)\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[a)\xAE\x90`\x01aV\xDFV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra*\xA9V[a)\xD0\x83` \x01Q\x83` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8AQ\x8B\x90\x89\x90\x81\x10a)\xF2Wa)\xF2aR\xE2V[` \x02` \x01\x01Q`@\x01Q\x85\x81Q\x81\x10a*\x0FWa*\x0FaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x83R_\x90\x83\x01Rc\xFF\xFF\xFF\xFFC\x16`@\x83\x01Ra*\xA9V[_\x82` \x01Q`\x0F\x0B\x13\x15a*\xA9Wa*]\x83` \x01Q\x83` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a*\x93W`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x9D\x89CaV\xDFV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a*\xBE\x8Ca*\xB6\x89a5\xACV[\x86\x86\x86a9\xA8V[\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x8Ca*\xECa\t\x07\x8Aa5\xACV[\x86a*\xFE\x86_\x01Q\x87` \x01Qa8\xBFV[\x86`@\x01Q`@Qa+\x14\x95\x94\x93\x92\x91\x90aUmV[`@Q\x80\x91\x03\x90\xA1PP`\x01\x90\x92\x01\x91Pa'\xD6\x90PV[PPP`\x01\x01a&\xF2V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x07v\x90a8\xACV[\x82a+p\x81a7\xE3V[a+\x8DW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a+\xCFW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x84\x84`@Qa,\n\x92\x91\x90aW`V[`@Q\x80\x91\x03\x90\xA2PPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a,AW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a,K\x81a7\xE3V[a,hW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF0\x91\x90aR\xC3V[a-\rW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a-\x1C` \x85\x01\x85aS;V[\x90P\x81\x10\x15a.\xE4W`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a-?` \x88\x01\x88aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x80` \x01\x90a-]\x91\x90aS;V[\x85\x81\x81\x10a-mWa-maR\xE2V[\x90P` \x02\x01` \x81\x01\x90a-\x82\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa-\xBE\x92\x91\x90\x81\x16\x90a7\xCC\x16V[a-\xDBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\xE5\x86\x82a8\xFBV[\x15a.\x03W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.,a.\x0F\x82a5\xACV[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9C` R`@\x90 \x90a8\x8DV[Pa.X\x86`\x9A_a.=\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a8\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa.\x92\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9E` R`@\x81 `\x01\x91a.\xBD\x84a5\xACV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a-\x0FV[Pa.\xF5a\x03\x83` \x85\x01\x85aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16c\xC6?\xD5\x02\x85a/\x11` \x87\x01\x87aH\xCFV[a/\x1E` \x88\x01\x88aS;V[a/+`@\x8A\x01\x8AaU\xBEV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/L\x96\x95\x94\x93\x92\x91\x90aWsV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/cW__\xFD[PZ\xF1\x15\x80\x15a\x1B\xFEW=__>=_\xFD[_a\t\x9A`\x9A_a/\x85\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a7\xB8V[\x83a/\xA1\x81a7\xE3V[a/\xBEW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x88\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a/\xFD\x92\x91a7\xCC\x16V[a0\x1AW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a0$\x82a5\xACV[\x90P_[\x84\x81\x10\x15a\x1B\xFEWa0m\x86\x86\x83\x81\x81\x10a0EWa0EaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a0Z\x91\x90aH\xCFV[_\x84\x81R`\x99` R`@\x90 \x90a?\x8DV[a0\x8AW`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x87\x87\x84\x81\x81\x10a0\xBEWa0\xBEaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a0\xD3\x91\x90aH\xCFV[`@Qa0\xE1\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a0(V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90Re\x01\0\0\0\0\0\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a1rWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a1\x81WPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x81 a\t\x9A\x90a7\xB8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a1\xD4\x90\x82a\x196\x86a5\xACV[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a1\xFAWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a2\x13WP0;\x15\x80\x15a2\x13WP_T`\xFF\x16`\x01\x14[a2{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a2\x9CW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a2\xA5\x82a7{V[a2\xAE\x83a?\xFBV[\x80\x15a2\xF4W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPV[\x81a3\x03\x81a7\xE3V[a3 W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xB5&W\x87`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x16\x90c\xB5&W\x87\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x88\x91\x90aR\xC3V[a3\xA5W`@Qc\x1D\x0B\x13\xC1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x83a3\xFD\x81a\x0FhV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a2\xEBV[a4%a?\xA1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a4\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a2rV[a4\x93\x81a?\xFBV[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x16\x91\x90aW\xBFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5GW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a5nW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a5\xF7\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\t\x9A\x90aW\xDAV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q``\x81\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x80\x85\x01\x83\x90R\x84Q\x80\x86\x01\x86R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x85R`\xA1\x84R\x86\x85 \x90\x88\x16\x85R\x90\x92R\x93\x82 \x92\x93\x92\x81\x90a6o\x90a8\xACV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a7\x11W\x90\x92P\x90Pa7sV[a7\"\x81_\x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a7`Wa7Q\x82` \x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\t\x9A\x82T\x90V[_a\x07v\x83\x83a@\xADV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07vV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9A\x91\x90aR\xC3V[_a\x07v\x83\x83a@\xD3V[_a\x07v\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a@\xD3V[_a\t\x9A\x82g\r\xE0\xB6\xB3\xA7d\0\0aA\x1FV[_a\x07v\x82`\x01`\x01`@\x1B\x03\x85\x16aU@V[_a\x07v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aAcV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aAcV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a9\x1E\x85a5\xACV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a1\xD4WP` \x01Qc\xFF\xFF\xFF\xFF\x16C\x11\x15\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07vV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01aBHV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA2\x84R`@\x80\x82 \x92\x88\x16\x82R\x91\x90\x93R\x90\x91 T`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14a:nW` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x81\x81R`\xA2\x86R`@\x80\x82 \x93\x8A\x16\x80\x83R\x93\x87R\x90\x81\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x95\x86\x16\x17\x90U\x93Q\x84Q\x91\x82R\x94\x81\x01\x91\x90\x91R\x92\x16\x90\x82\x01R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x90``\x01`@Q\x80\x91\x03\x90\xA1[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x86\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x95\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90U`\x0F\x0B\x15a;PW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;(\x90\x84a8\x98V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a;J\x90\x85a8\x8DV[Pa\x0CtV[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\x0CtW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;\x8D\x90\x84a?\x8DV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;\xB9\x90a7\xB8V[_\x03a\x0CtW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a#\xBB\x90\x85a?\x82V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a<\x0F\x90C\x83aB\xA1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90``\x01a2\xEBV[``_a\x07v\x83aB\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a<\xB8WP\x82a\xFF\xFF\x16\x82\x10[\x15a\x0CtW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a<\xEC\x90aC\x0EV[\x90P__a<\xFB\x88\x84\x89a6\x0FV[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a=\x19WPPPa\x0CtV[a=&\x88\x84\x89\x85\x85a9\xA8V[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a=S\x90aC`V[Pa=]\x85aW\xFDV[\x94Pa=h\x84aX\x15V[\x93PPPPa<\xA6V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a=\xEEWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a>\x08W`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra>>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a2\xEBV[__a?5a?!\x84aC\xDDV[\x85Ta?0\x91\x90`\x0F\x0BaX*V[aDJV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a?hW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_a\x07v\x83\x83aD\xB3V[_a\x07v\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aD\xB3V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a2rV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aE\x96V[_a@q\x84`\x99_a\x12\xAE\x89a5\xACV[\x80\x15a@zWP\x81[\x80\x15a\x1F\xD9WPP\x90Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x93\x92PPPV[_a\x07v`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aXQV[_\x82_\x01\x82\x81T\x81\x10a@\xC2Wa@\xC2aR\xE2V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaA\x18WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\t\x9AV[P_a\t\x9AV[\x81T_\x90\x80\x15aA[WaAE\x84aA8`\x01\x84aT\xEAV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a1\xD4V[P\x90\x92\x91PPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aA\x9AW\x83\x82\x81aA\x90WaA\x90aX~V[\x04\x92PPPa\x07vV[\x80\x84\x11aA\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a2rV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[__aBU\x86\x86\x86aAcV[\x90P`\x01\x83`\x02\x81\x11\x15aBkWaBkaX\x92V[\x14\x80\x15aB\x87WP_\x84\x80aB\x82WaB\x82aX~V[\x86\x88\t\x11[\x15a\x1F\xD9WaB\x97`\x01\x82aX\xA6V[\x96\x95PPPPPPV[a2\xF4\x83\x83`\x01`\x01`@\x1B\x03\x84\x16aE\xDEV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aC\x02W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aB\xEEW[PPPPP\x90P\x91\x90PV[_aC(\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aCFW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_aCz\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aC\x98W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a2rV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14aD\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a2rV[\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aE\x8DW_aD\xD5`\x01\x83aT\xEAV[\x85T\x90\x91P_\x90aD\xE8\x90`\x01\x90aT\xEAV[\x90P\x81\x81\x14aEGW_\x86_\x01\x82\x81T\x81\x10aE\x06WaE\x06aR\xE2V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aE&WaE&aR\xE2V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aEXWaEXaX\xB9V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\t\x9AV[_\x91PPa\t\x9AV[\x82T_\x90\x81aE\xA7\x86\x86\x83\x85aF\xE1V[\x90P\x80\x15aE\xD4WaE\xBE\x86aA8`\x01\x84aT\xEAV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x07qV[P\x91\x94\x93PPPPV[\x82T\x80\x15aF\x94W_aE\xF6\x85aA8`\x01\x85aT\xEAV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aFHW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aF\x92W\x82aFi\x86aA8`\x01\x86aT\xEAV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\taW_aF\xF6\x84\x84aG4V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aG W\x80\x92PaG.V[aG+\x81`\x01aX\xA6V[\x93P[PaF\xE3V[_aGB`\x02\x84\x84\x18aX\xCDV[a\x07v\x90\x84\x84\x16aX\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\x93W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\x98WaG\x98aGbV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xC6WaG\xC6aGbV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aD\xAEW__\xFD[_`@\x82\x84\x03\x12\x15aG\xF1W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\x13WaH\x13aGbV[`@R\x90P\x80\x825aH$\x81aGNV[\x81RaH2` \x84\x01aG\xCEV[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aHPW__\xFD[\x835aH[\x81aGNV[\x92PaHj\x85` \x86\x01aG\xE1V[\x91P``\x84\x015aHz\x81aGNV[\x80\x91PP\x92P\x92P\x92V[\x81Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\t\x9AV[_` \x82\x84\x03\x12\x15aH\xC8W__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15aH\xDFW__\xFD[\x815a\x07v\x81aGNV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICWaI-\x86\x83QaH\xEAV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aI\x1AV[P\x93\x94\x93PPPPV[` \x81R_a\x07v` \x83\x01\x84aI\x08V[_`@\x82\x84\x03\x12\x15aIoW__\xFD[a\x07v\x83\x83aG\xE1V[__\x83`\x1F\x84\x01\x12aI\x89W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x9FW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x19\x1DW__\xFD[___`@\x84\x86\x03\x12\x15aI\xCBW__\xFD[\x835aI\xD6\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF0W__\xFD[aI\xFC\x86\x82\x87\x01aIyV[\x94\x97\x90\x96P\x93\x94PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aJ!WaJ!aGbV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aJ:W__\xFD[\x815aJMaJH\x82aJ\tV[aG\x9EV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aJnW__\xFD[` \x85\x01[\x83\x81\x10\x15aJ\x94W\x805aJ\x86\x81aGNV[\x83R` \x92\x83\x01\x92\x01aJsV[P\x95\x94PPPPPV[____`\xA0\x85\x87\x03\x12\x15aJ\xB1W__\xFD[aJ\xBB\x86\x86aG\xE1V[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD5W__\xFD[aJ\xE1\x87\x82\x88\x01aJ+V[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xFCW__\xFD[aK\x08\x87\x82\x88\x01aJ+V[\x92PPaK\x17`\x80\x86\x01aG\xCEV[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aK4V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aK\xA9W`?\x19\x87\x86\x03\x01\x84RaK\x94\x85\x83QaK\"V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aKxV[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aK\xC6W__\xFD[\x825aK\xD1\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xEBW__\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aK\xFCW__\xFD[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aL\x18W__\xFD[\x825aL#\x81aGNV[\x91P` \x83\x015aK\xFC\x81aGNV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICWaL~\x86\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aLEV[`@\x81R_aL\xA6`@\x83\x01\x85aI\x08V[\x82\x81\x03` \x84\x01Ra\x1F\xD9\x81\x85aL3V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aL\xCAV[` \x81R_a\x07v` \x83\x01\x84aL\xB8V[__`@\x83\x85\x03\x12\x15aM\x14W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM)W__\xFD[aM5\x85\x82\x86\x01aJ+V[\x92PP` \x83\x015aK\xFC\x81aGNV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aM\x86W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aM_V[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aM\xA5W__\xFD[\x855aM\xB0\x81aGNV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xCAW__\xFD[aM\xD6\x88\x82\x89\x01aIyV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xF4W__\xFD[aN\0\x88\x82\x89\x01aIyV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15aN$W__\xFD[\x845aN/\x81aGNV[\x93PaN=` \x86\x01aG\xCEV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aNWW__\xFD[aNc\x87\x82\x88\x01aIyV[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aN\x80W__\xFD[\x825aN\x8B\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xA5W__\xFD[aN\xB1\x85\x82\x86\x01aJ+V[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aN\xCCW__\xFD[\x825aN\xD7\x81aGNV[\x91PaN\xE5` \x84\x01aG\xCEV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aN\xFEW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x07vW__\xFD[__``\x83\x85\x03\x12\x15aO\x1FW__\xFD[\x825aO*\x81aGNV[\x91PaN\xE5\x84` \x85\x01aG\xE1V[_``\x82\x84\x03\x12\x15aOIW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aO_W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aOtW__\xFD[a1\xD4\x84\x82\x85\x01aO9V[___`\x80\x84\x86\x03\x12\x15aO\x92W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xA7W__\xFD[aO\xB3\x86\x82\x87\x01aJ+V[\x93PPaHj\x85` \x86\x01aG\xE1V[` \x81R_a\x07v` \x83\x01\x84aL3V[___``\x84\x86\x03\x12\x15aO\xE7W__\xFD[\x835aO\xF2\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x0CW__\xFD[aP\x18\x86\x82\x87\x01aJ+V[\x92PPaP'`@\x85\x01aG\xCEV[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aPAW__\xFD[\x825aPL\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPfW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aPvW__\xFD[\x805aP\x84aJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aP\xA5W__\xFD[` \x84\x01[\x83\x81\x10\x15aQ\xCAW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xC7W__\xFD[\x85\x01`\x80\x81\x8B\x03`\x1F\x19\x01\x12\x15aP\xDCW__\xFD[aP\xE4aGvV[aP\xF1\x8B` \x84\x01aG\xE1V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x0BW__\xFD[aQ\x1A\x8C` \x83\x86\x01\x01aJ+V[` \x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ8W__\xFD[` \x81\x84\x01\x01\x92PP\x8A`\x1F\x83\x01\x12aQOW__\xFD[\x815aQ]aJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15aQ~W__\xFD[` \x85\x01\x94P[\x82\x85\x10\x15aQ\xB4W\x845`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aQ\xA3W__\xFD[\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aQ\x85V[`@\x84\x01RPP\x84RP` \x92\x83\x01\x92\x01aP\xAAV[P\x80\x94PPPPP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aQ\xEBW__\xFD[\x835aQ\xF6\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x10W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aR W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR5W__\xFD[\x86` \x82\x84\x01\x01\x11\x15aRFW__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[__`@\x83\x85\x03\x12\x15aRhW__\xFD[\x825aRs\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x8DW__\xFD[aN\xB1\x85\x82\x86\x01aO9V[__`@\x83\x85\x03\x12\x15aR\xAAW__\xFD[\x825aR\xB5\x81aGNV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aR\xD3W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07vW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aS\nW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aS$W__\xFD[a\x07v\x82aG\xCEV[`@\x81\x01a\t\x9A\x82\x84aH\xEAV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aSPW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aSiW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x19\x1DW__\xFD[``\x81\x01aS\x8E\x82\x85aH\xEAV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[`@\x81R_aS\xB8`@\x83\x01\x85aL\xB8V[\x82\x81\x03` \x84\x01Ra\x1F\xD9\x81\x85aL\xB8V[_` \x82\x84\x03\x12\x15aS\xDAW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xEFW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aS\xFFW__\xFD[\x80QaT\raJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aT.W__\xFD[` \x84\x01[\x83\x81\x10\x15aT\xCBW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTPW__\xFD[\x85\x01`?\x81\x01\x89\x13aT`W__\xFD[` \x81\x01QaTqaJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aT\x94W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aT\xB6W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\x9BV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaT3V[P\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aU8WaU8aT\xD6V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\t\x9AWa\t\x9AaT\xD6V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aU\x8A` \x83\x01\x87aH\xEAV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aU\xD3W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aU\xECW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19\x1DW__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R_`\xC0\x82\x01aVF` \x84\x01\x8AaH\xEAV[`\xC0``\x84\x01R\x86\x90R\x86`\xE0\x83\x01_[\x88\x81\x10\x15aV\x87W\x825aVj\x81aGNV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aVWV[P\x83\x81\x03`\x80\x85\x01RaV\x9A\x81\x88aK\"V[\x91PP\x82\x81\x03`\xA0\x84\x01RaV\xB0\x81\x85\x87aV\0V[\x9A\x99PPPPPPPPPPV[_` \x82\x84\x03\x12\x15aV\xCEW__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x07vW__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aICWc\xFF\xFF\xFF\xFFaW\x1E\x83aG\xCEV[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aW\x08V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90aB\x97\x90\x83\x01\x84\x86aV\xFBV[` \x81R_a1\xD4` \x83\x01\x84\x86aV\0V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R_\x90aW\x9F\x90\x83\x01\x86\x88aV\xFBV[\x82\x81\x03``\x84\x01RaW\xB2\x81\x85\x87aV\0V[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aW\xCFW__\xFD[\x81Qa\x07v\x81aGNV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aOIW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01aX\x0EWaX\x0EaT\xD6V[P`\x01\x01\x90V[_\x81aX#WaX#aT\xD6V[P_\x19\x01\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aXIWaXIaT\xD6V[PP\x92\x91PPV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\t\x9AWa\t\x9AaT\xD6V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\t\x9AWa\t\x9AaT\xD6V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82aX\xE7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC3\xBA1R\x11\xDB\xC6\xCE\x0C\xA7\x97Q2\x9D\xEEMzA\x1E\xD4\xAF\x89 :\xA0\xCC*\x16\xF5n\xF3\x17dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610281575f3560e01c80636e875dba11610156578063a984eb3a116100ca578063c221d8ae11610084578063c221d8ae146106bc578063cd6dc687146106cf578063d3d96ff4146106e2578063df5cf723146106f5578063f2fde38b1461071c578063fabc1cbc1461072f575f5ffd5b8063a984eb3a1461060e578063adc2e3d914610641578063b2447af714610654578063b66bd98914610667578063b9fbaed11461067a578063ba1a84e5146106a9575f5ffd5b80638ce648541161011b5780638ce64854146105915780638da5cb5b146105b157806394d7d00c146105c2578063952899ee146105d5578063a9333ec8146105e8578063a9821821146105fb575f5ffd5b80636e875dba14610515578063715018a61461052857806379ae50cd146105305780637bc1ef6114610543578063886f11951461056a575f5ffd5b80634657e26a116101f8578063595c6a67116101b2578063595c6a67146104875780635ac86ab71461048f5780635c975abb146104b2578063670d3ba2146104c45780636cfb4481146104d75780636e3492b514610502575f5ffd5b80634657e26a146103f45780634a10ffe51461041b5780634b5046ef1461043b57806350feea201461044e578063547afb871461046157806356c483e614610474575f5ffd5b80632981eb77116102495780632981eb77146103195780632bab2c4a14610355578063304c10cd1461037557806336352057146103a057806340120dab146103b35780634177a87c146103d4575f5ffd5b806310e1b9b814610285578063136439dd146102ae57806315fe5028146102c3578063260dc758146102e3578063261f84e014610306575b5f5ffd5b61029861029336600461483e565b610742565b6040516102a59190614885565b60405180910390f35b6102c16102bc3660046148b8565b61077d565b005b6102d66102d13660046148cf565b610852565b6040516102a5919061494d565b6102f66102f136600461495f565b610969565b60405190151581526020016102a5565b6102c16103143660046149b9565b6109a0565b6103407f000000000000000000000000000000000000000000000000000000000000000081565b60405163ffffffff90911681526020016102a5565b610368610363366004614a9e565b610c7b565b6040516102a59190614b52565b6103886103833660046148cf565b610f68565b6040516001600160a01b0390911681526020016102a5565b6102c16103ae366004614bb5565b610f97565b6103c66103c1366004614c07565b6117a9565b6040516102a5929190614c94565b6103e76103e236600461495f565b611924565b6040516102a59190614cf1565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b61042e610429366004614d03565b611948565b6040516102a59190614d46565b6102c1610449366004614d91565b6119f0565b6102c161045c366004614e11565b611aaa565b61042e61046f366004614e6f565b611c08565b6102c1610482366004614ebb565b611cb0565b6102c1611db5565b6102f661049d366004614eee565b606654600160ff9092169190911b9081161490565b6066545b6040519081526020016102a5565b6102f66104d2366004614f0e565b611e64565b6104ea6104e5366004614c07565b611e75565b6040516001600160401b0390911681526020016102a5565b6102c1610510366004614f4f565b611fe2565b6103e761052336600461495f565b6123c3565b6102c16123d4565b6102d661053e3660046148cf565b6123e5565b6103407f000000000000000000000000000000000000000000000000000000000000000081565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b6105a461059f366004614f80565b6124bf565b6040516102a59190614fc3565b6033546001600160a01b0316610388565b61042e6105d0366004614fd5565b612584565b6102c16105e3366004615030565b612670565b6104ea6105f6366004614c07565b612b37565b6102c16106093660046151d9565b612b66565b6104ea61061c366004614c07565b60a260209081525f92835260408084209091529082529020546001600160401b031681565b6102c161064f366004615257565b612c18565b6104b661066236600461495f565b612f75565b6102c1610675366004614e11565b612f97565b61068d6106883660046148cf565b6130f1565b60408051921515835263ffffffff9091166020830152016102a5565b6104b66106b73660046148cf565b61318b565b6103e76106ca366004614f0e565b6131ab565b6102c16106dd366004615299565b6131dc565b6102c16106f0366004614c07565b6132f9565b6103887f000000000000000000000000000000000000000000000000000000000000000081565b6102c161072a3660046148cf565b61341d565b6102c161073d3660046148b8565b613496565b604080516060810182525f80825260208201819052918101829052906107718561076b866135ac565b8561360f565b925050505b9392505050565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa1580156107df573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061080391906152c3565b61082057604051631d77d47760e21b815260040160405180910390fd5b60665481811681146108455760405163c61dca5d60e01b815260040160405180910390fd5b61084e8261377b565b5050565b6001600160a01b0381165f908152609d6020526040812060609190610876906137b8565b90505f816001600160401b0381111561089157610891614762565b6040519080825280602002602001820160405280156108d557816020015b604080518082019091525f80825260208201528152602001906001900390816108af5790505b5090505f5b82811015610961576001600160a01b0385165f908152609d6020526040902061093c9061090790836137c1565b604080518082019091525f80825260208201525060408051808201909152606082901c815263ffffffff909116602082015290565b82828151811061094e5761094e6152e2565b60209081029190910101526001016108da565b509392505050565b60208082015182516001600160a01b03165f90815260989092526040822061099a9163ffffffff908116906137cc16565b92915050565b826109aa816137e3565b6109c75760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff166109ff5760405163caf9756160e01b815260040160405180910390fd5b5f5b82811015610c74575f6040518060400160405280876001600160a01b03168152602001868685818110610a3657610a366152e2565b9050602002810190610a4891906152f6565b610a56906020810190615314565b63ffffffff168152509050610aa0816020015163ffffffff1660985f896001600160a01b03166001600160a01b031681526020019081526020015f2061388d90919063ffffffff16565b610abd57604051631fb1705560e21b815260040160405180910390fd5b7f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040518060400160405280886001600160a01b03168152602001836020015163ffffffff16815250604051610b13919061532d565b60405180910390a15f610b25826135ac565b90505f5b868685818110610b3b57610b3b6152e2565b9050602002810190610b4d91906152f6565b610b5b90602081019061533b565b9050811015610c6957610bd1878786818110610b7957610b796152e2565b9050602002810190610b8b91906152f6565b610b9990602081019061533b565b83818110610ba957610ba96152e2565b9050602002016020810190610bbe91906148cf565b5f84815260996020526040902090613898565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83888887818110610c0657610c066152e2565b9050602002810190610c1891906152f6565b610c2690602081019061533b565b84818110610c3657610c366152e2565b9050602002016020810190610c4b91906148cf565b604051610c59929190615380565b60405180910390a1600101610b29565b505050600101610a01565b5050505050565b606083516001600160401b03811115610c9657610c96614762565b604051908082528060200260200182016040528015610cc957816020015b6060815260200190600190039081610cb45790505b5090505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663f0e0e67686866040518363ffffffff1660e01b8152600401610d1b9291906153a6565b5f60405180830381865afa158015610d35573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610d5c91908101906153ca565b90505f5b8551811015610f5e575f868281518110610d7c57610d7c6152e2565b6020026020010151905085516001600160401b03811115610d9f57610d9f614762565b604051908082528060200260200182016040528015610dc8578160200160208202803683370190505b50848381518110610ddb57610ddb6152e2565b60209081029190910101525f5b8651811015610f54575f878281518110610e0457610e046152e2565b6020908102919091018101516001600160a01b038086165f90815260a1845260408082209284168252919093528220909250610e3f906138ac565b9050806001600160401b03165f03610e58575050610f4c565b5f610e64858d85610742565b90508863ffffffff16816040015163ffffffff1611158015610e8c57505f8160200151600f0b125b15610eae57610ea2815f015182602001516138bf565b6001600160401b031681525b80515f90610ec9906001600160401b039081169085166138d3565b9050610f1081898981518110610ee157610ee16152e2565b60200260200101518781518110610efa57610efa6152e2565b60200260200101516138e790919063ffffffff16565b898881518110610f2257610f226152e2565b60200260200101518681518110610f3b57610f3b6152e2565b602002602001018181525050505050505b600101610de8565b5050600101610d60565b5050949350505050565b6001600160a01b038082165f908152609760205260408120549091168015610f905780610776565b5090919050565b606654600190600290811603610fc05760405163840a48d560e01b815260040160405180910390fd5b82610fca816137e3565b610fe75760405163932d94f760e01b815260040160405180910390fd5b5f6040518060400160405280866001600160a01b031681526020018560200160208101906110159190615314565b63ffffffff16905290505f61103661103060208701876148cf565b836138fb565b9050611045606086018661533b565b9050611054604087018761533b565b905014611074576040516343714afd60e01b815260040160405180910390fd5b60208083015183516001600160a01b03165f908152609890925260409091206110a69163ffffffff908116906137cc16565b6110c357604051631fb1705560e21b815260040160405180910390fd5b806110e15760405163ebbff49760e01b815260040160405180910390fd5b5f6110ef604087018761533b565b90506001600160401b0381111561110857611108614762565b604051908082528060200260200182016040528015611131578160200160208202803683370190505b5090505f5b611143604088018861533b565b905081101561173a578015806111d65750611161604088018861533b565b61116c6001846154ea565b81811061117b5761117b6152e2565b905060200201602081019061119091906148cf565b6001600160a01b03166111a6604089018961533b565b838181106111b6576111b66152e2565b90506020020160208101906111cb91906148cf565b6001600160a01b0316115b6111f357604051639f1c805360e01b815260040160405180910390fd5b611200606088018861533b565b82818110611210576112106152e2565b905060200201355f1080156112505750670de0b6b3a7640000611236606089018961533b565b83818110611246576112466152e2565b9050602002013511155b61126d57604051631353603160e01b815260040160405180910390fd5b6112c961127d604089018961533b565b8381811061128d5761128d6152e2565b90506020020160208101906112a291906148cf565b60995f6112ae886135ac565b81526020019081526020015f2061397190919063ffffffff16565b6112e6576040516331bc342760e11b815260040160405180910390fd5b5f806113386112f860208b018b6148cf565b611301886135ac565b61130e60408d018d61533b565b8781811061131e5761131e6152e2565b905060200201602081019061133391906148cf565b61360f565b805191935091506001600160401b03165f03611355575050611732565b5f61139061136660608c018c61533b565b86818110611376576113766152e2565b85516001600160401b031692602090910201359050613992565b83519091506113ab6001600160401b038084169083166138d3565b8686815181106113bd576113bd6152e2565b60200260200101818152505081835f018181516113da91906154fd565b6001600160401b03169052508351829085906113f79083906154fd565b6001600160401b03169052506020840180518391906114179083906154fd565b6001600160401b031690525060208301515f600f9190910b1215611532575f61147a61144660608e018e61533b565b88818110611456576114566152e2565b90506020020135856020015161146b9061551c565b6001600160801b031690613992565b9050806001600160401b0316846020018181516114979190615540565b600f0b9052507f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd6114cb60208e018e6148cf565b8a8e80604001906114dc919061533b565b8a8181106114ec576114ec6152e2565b905060200201602081019061150191906148cf565b611512885f015189602001516138bf565b886040015160405161152895949392919061556d565b60405180910390a1505b61158461154260208d018d6148cf565b61154b8a6135ac565b61155860408f018f61533b565b89818110611568576115686152e2565b905060200201602081019061157d91906148cf565b87876139a8565b7f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd6115b260208d018d6148cf565b896115c060408f018f61533b565b898181106115d0576115d06152e2565b90506020020160208101906115e591906148cf565b86516040516115f99493929190439061556d565b60405180910390a161164a61161160208d018d6148cf565b61161e60408e018e61533b565b8881811061162e5761162e6152e2565b905060200201602081019061164391906148cf565b8651613be0565b6001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001663ee74937f61168660208e018e6148cf565b61169360408f018f61533b565b898181106116a3576116a36152e2565b90506020020160208101906116b891906148cf565b875160405160e085901b6001600160e01b03191681526001600160a01b0393841660048201529290911660248301526001600160401b0380861660448401521660648201526084015f604051808303815f87803b158015611717575f5ffd5b505af1158015611729573d5f5f3e3d5ffd5b50505050505050505b600101611136565b507f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe561176960208801886148cf565b8461177760408a018a61533b565b8561178560808d018d6155be565b6040516117989796959493929190615628565b60405180910390a150505050505050565b6001600160a01b0382165f908152609d6020526040812060609182916117ce906137b8565b90505f816001600160401b038111156117e9576117e9614762565b60405190808252806020026020018201604052801561182d57816020015b604080518082019091525f80825260208201528152602001906001900390816118075790505b5090505f826001600160401b0381111561184957611849614762565b60405190808252806020026020018201604052801561189257816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816118675790505b5090505f5b83811015611915576001600160a01b0388165f908152609d602052604081206118c49061090790846137c1565b9050808483815181106118d9576118d96152e2565b60200260200101819052506118ef89828a610742565b838381518110611901576119016152e2565b602090810291909101015250600101611897565b509093509150505b9250929050565b60605f61077660995f611936866135ac565b81526020019081526020015f20613c62565b60605f83516001600160401b0381111561196457611964614762565b60405190808252806020026020018201604052801561198d578160200160208202803683370190505b5090505f5b8451811015610961576119be8582815181106119b0576119b06152e2565b602002602001015185612b37565b8282815181106119d0576119d06152e2565b6001600160401b0390921660209283029190910190910152600101611992565b6066545f90600190811603611a185760405163840a48d560e01b815260040160405180910390fd5b838214611a38576040516343714afd60e01b815260040160405180910390fd5b5f5b84811015611aa157611a9987878784818110611a5857611a586152e2565b9050602002016020810190611a6d91906148cf565b868685818110611a7f57611a7f6152e2565b9050602002016020810190611a9491906156be565b613c6e565b600101611a3a565b50505050505050565b83611ab4816137e3565b611ad15760405163932d94f760e01b815260040160405180910390fd5b604080518082019091526001600160a01b038616815263ffffffff851660208201525f611afd826135ac565b9050611b3e826020015163ffffffff1660985f8a6001600160a01b03166001600160a01b031681526020019081526020015f206137cc90919063ffffffff16565b611b5b57604051631fb1705560e21b815260040160405180910390fd5b5f5b84811015611bfe57611b7a868683818110610ba957610ba96152e2565b611b975760405163585cfb2f60e01b815260040160405180910390fd5b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b83878784818110611bcb57611bcb6152e2565b9050602002016020810190611be091906148cf565b604051611bee929190615380565b60405180910390a1600101611b5d565b5050505050505050565b60605f82516001600160401b03811115611c2457611c24614762565b604051908082528060200260200182016040528015611c4d578160200160208202803683370190505b5090505f5b835181101561096157611c7e85858381518110611c7157611c716152e2565b6020026020010151612b37565b828281518110611c9057611c906152e2565b6001600160401b0390921660209283029190910190910152600101611c52565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611dab57611ce9826137e3565b611d06576040516348f5c3ed60e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0383811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015611d6a573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611d8e91906152c3565b611dab5760405163ccea9e6f60e01b815260040160405180910390fd5b61084e8282613d72565b60405163237dfb4760e11b81523360048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316906346fbf68e90602401602060405180830381865afa158015611e17573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e3b91906152c3565b611e5857604051631d77d47760e21b815260040160405180910390fd5b611e625f1961377b565b565b5f61077683609a5f6112ae866135ac565b6001600160a01b038281165f81815260a2602090815260408083209486168084529482528083205493835260a38252808320948352939052918220546001600160401b0390911690600f81810b600160801b909204900b03825b81811015611f9f576001600160a01b038087165f90815260a3602090815260408083209389168352929052908120611f079083613f13565b6001600160a01b038881165f90815260a0602090815260408083208584528252808320938b16835292815290829020825160608101845290546001600160401b0381168252600160401b8104600f0b92820192909252600160c01b90910463ffffffff16918101829052919250431015611f82575050611f9f565b611f908582602001516138bf565b94505050806001019050611ecf565b506001600160a01b038086165f90815260a1602090815260408083209388168352929052208290611fcf906138ac565b611fd991906154fd565b95945050505050565b60665460029060049081160361200b5760405163840a48d560e01b815260040160405180910390fd5b61202061201b60208401846148cf565b6137e3565b80612039575061203961201b60408401602085016148cf565b612056576040516348f5c3ed60e01b815260040160405180910390fd5b5f5b612065604084018461533b565b9050811015612327575f604051806040016040528085602001602081019061208d91906148cf565b6001600160a01b031681526020016120a8604087018761533b565b858181106120b8576120b86152e2565b90506020020160208101906120cd9190615314565b63ffffffff16815250905061211a816020015163ffffffff1660985f8760200160208101906120fc91906148cf565b6001600160a01b0316815260208101919091526040015f20906137cc565b61213757604051631fb1705560e21b815260040160405180910390fd5b609e5f61214760208701876148cf565b6001600160a01b03166001600160a01b031681526020019081526020015f205f612170836135ac565b815260208101919091526040015f205460ff166121a0576040516325131d4f60e01b815260040160405180910390fd5b6121da6121ac826135ac565b609c5f6121bc60208901896148cf565b6001600160a01b0316815260208101919091526040015f2090613f82565b506122126121eb60208601866148cf565b609a5f6121f7856135ac565b81526020019081526020015f20613f8d90919063ffffffff16565b5061222060208501856148cf565b6001600160a01b03167fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe82604051612258919061532d565b60405180910390a2604080518082019091525f81526020810161229b7f0000000000000000000000000000000000000000000000000000000000000000436156df565b63ffffffff169052609e5f6122b360208801886148cf565b6001600160a01b03166001600160a01b031681526020019081526020015f205f6122dc846135ac565b81526020808201929092526040015f2082518154939092015163ffffffff166101000264ffffffff00199215159290921664ffffffffff199093169290921717905550600101612058565b5061233b61038360408401602085016148cf565b6001600160a01b031663303ca95661235660208501856148cf565b61236660408601602087016148cf565b612373604087018761533b565b6040518563ffffffff1660e01b81526004016123929493929190615734565b5f604051808303815f87803b1580156123a9575f5ffd5b505af11580156123bb573d5f5f3e3d5ffd5b505050505050565b606061099a609a5f611936856135ac565b6123dc613fa1565b611e625f613ffb565b6001600160a01b0381165f908152609c6020526040812060609190612409906137b8565b90505f816001600160401b0381111561242457612424614762565b60405190808252806020026020018201604052801561246857816020015b604080518082019091525f80825260208201528152602001906001900390816124425790505b5090505f5b82811015610961576001600160a01b0385165f908152609c6020526040902061249a9061090790836137c1565b8282815181106124ac576124ac6152e2565b602090810291909101015260010161246d565b60605f84516001600160401b038111156124db576124db614762565b60405190808252806020026020018201604052801561252457816020015b604080516060810182525f80825260208083018290529282015282525f199092019101816124f95790505b5090505f5b855181101561257b57612556868281518110612547576125476152e2565b60200260200101518686610742565b828281518110612568576125686152e2565b6020908102919091010152600101612529565b50949350505050565b60605f83516001600160401b038111156125a0576125a0614762565b6040519080825280602002602001820160405280156125c9578160200160208202803683370190505b5090505f5b845181101561257b576001600160a01b0386165f90815260a160205260408120865161263e92879291899086908110612609576126096152e2565b60200260200101516001600160a01b03166001600160a01b031681526020019081526020015f2061404c90919063ffffffff16565b828281518110612650576126506152e2565b6001600160401b03909216602092830291909101909101526001016125ce565b6066545f906001908116036126985760405163840a48d560e01b815260040160405180910390fd5b6126a1836137e3565b6126be576040516348f5c3ed60e01b815260040160405180910390fd5b5f5f5f6126ca866130f1565b91509150816126ec5760405163fa55fc8160e01b815260040160405180910390fd5b91505f90505b8351811015610c745783818151811061270d5761270d6152e2565b6020026020010151604001515184828151811061272c5761272c6152e2565b6020026020010151602001515114612757576040516343714afd60e01b815260040160405180910390fd5b5f84828151811061276a5761276a6152e2565b602090810291909101810151518082015181516001600160a01b03165f908152609890935260409092209092506127aa9163ffffffff908116906137cc16565b6127c757604051631fb1705560e21b815260040160405180910390fd5b5f6127d287836138fb565b90505f5b8684815181106127e8576127e86152e2565b60200260200101516020015151811015612b2c575f87858151811061280f5761280f6152e2565b602002602001015160200151828151811061282c5761282c6152e2565b60200260200101519050612843898261ffff613c6e565b5f5f6128528b61076b886135ac565b915091508060200151600f0b5f1461287d57604051630d8fcbe360e41b815260040160405180910390fd5b5f61288a87858489614060565b90506128cf825f01518c8a815181106128a5576128a56152e2565b60200260200101516040015187815181106128c2576128c26152e2565b6020026020010151614096565b600f0b602083018190525f036128f857604051634606179360e11b815260040160405180910390fd5b5f8260200151600f0b1215612a3c5780156129be57612979612919886135ac565b6001600160a01b03808f165f90815260a360209081526040808320938a16835292905220908154600160801b90819004600f0b5f818152600180860160205260409091209390935583546001600160801b03908116939091011602179055565b6129a37f0000000000000000000000000000000000000000000000000000000000000000436156df565b6129ae9060016156df565b63ffffffff166040830152612aa9565b6129d0836020015183602001516138bf565b6001600160401b031660208401528a518b90899081106129f2576129f26152e2565b6020026020010151604001518581518110612a0f57612a0f6152e2565b6020908102919091018101516001600160401b031683525f9083015263ffffffff43166040830152612aa9565b5f8260200151600f0b1315612aa957612a5d836020015183602001516138bf565b6001600160401b039081166020850181905284519091161015612a9357604051636c9be0bf60e01b815260040160405180910390fd5b612a9d89436156df565b63ffffffff1660408301525b612abe8c612ab6896135ac565b8686866139a8565b7f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd8c612aec6109078a6135ac565b86612afe865f015187602001516138bf565b8660400151604051612b1495949392919061556d565b60405180910390a15050600190920191506127d69050565b5050506001016126f2565b6001600160a01b038083165f90815260a1602090815260408083209385168352929052908120610776906138ac565b82612b70816137e3565b612b8d5760405163932d94f760e01b815260040160405180910390fd5b6001600160a01b0384165f90815260a4602052604090205460ff16612bcf576001600160a01b0384165f90815260a460205260409020805460ff191660011790555b836001600160a01b03167fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c9437138484604051612c0a929190615760565b60405180910390a250505050565b606654600290600490811603612c415760405163840a48d560e01b815260040160405180910390fd5b82612c4b816137e3565b612c685760405163932d94f760e01b815260040160405180910390fd5b6040516336b87bd760e11b81526001600160a01b0385811660048301527f00000000000000000000000000000000000000000000000000000000000000001690636d70f7ae90602401602060405180830381865afa158015612ccc573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612cf091906152c3565b612d0d5760405163ccea9e6f60e01b815260040160405180910390fd5b5f5b612d1c602085018561533b565b9050811015612ee457604080518082019091525f9080612d3f60208801886148cf565b6001600160a01b03168152602001868060200190612d5d919061533b565b85818110612d6d57612d6d6152e2565b9050602002016020810190612d829190615314565b63ffffffff90811690915260208083015183516001600160a01b03165f90815260989092526040909120929350612dbe9291908116906137cc16565b612ddb57604051631fb1705560e21b815260040160405180910390fd5b612de586826138fb565b15612e0357604051636c6c6e2760e11b815260040160405180910390fd5b612e2c612e0f826135ac565b6001600160a01b0388165f908152609c602052604090209061388d565b50612e5886609a5f612e3d856135ac565b81526020019081526020015f2061389890919063ffffffff16565b50856001600160a01b03167f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e82604051612e92919061532d565b60405180910390a26001600160a01b0386165f908152609e60205260408120600191612ebd846135ac565b815260208101919091526040015f20805460ff191691151591909117905550600101612d0f565b50612ef561038360208501856148cf565b6001600160a01b031663c63fd50285612f1160208701876148cf565b612f1e602088018861533b565b612f2b60408a018a6155be565b6040518763ffffffff1660e01b8152600401612f4c96959493929190615773565b5f604051808303815f87803b158015612f63575f5ffd5b505af1158015611bfe573d5f5f3e3d5ffd5b5f61099a609a5f612f85856135ac565b81526020019081526020015f206137b8565b83612fa1816137e3565b612fbe5760405163932d94f760e01b815260040160405180910390fd5b6040805180820182526001600160a01b03871680825263ffffffff80881660208085018290525f93845260989052939091209192612ffd92916137cc16565b61301a57604051631fb1705560e21b815260040160405180910390fd5b5f613024826135ac565b90505f5b84811015611bfe5761306d868683818110613045576130456152e2565b905060200201602081019061305a91906148cf565b5f84815260996020526040902090613f8d565b61308a576040516331bc342760e11b815260040160405180910390fd5b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee838787848181106130be576130be6152e2565b90506020020160208101906130d391906148cf565b6040516130e1929190615380565b60405180910390a1600101613028565b6001600160a01b0381165f908152609b602090815260408083208151608081018352905463ffffffff80821680845260ff600160201b8404161515958401869052650100000000008304821694840194909452600160481b9091041660608201819052849391929190158015906131725750826060015163ffffffff164310155b15613181575050604081015160015b9590945092505050565b6001600160a01b0381165f90815260986020526040812061099a906137b8565b6001600160a01b0382165f908152609f60205260408120606091906131d49082611936866135ac565b949350505050565b5f54610100900460ff16158080156131fa57505f54600160ff909116105b806132135750303b15801561321357505f5460ff166001145b61327b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b5f805460ff19166001179055801561329c575f805461ff0019166101001790555b6132a58261377b565b6132ae83613ffb565b80156132f4575f805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498906020015b60405180910390a15b505050565b81613303816137e3565b6133205760405163932d94f760e01b815260040160405180910390fd5b60405163b526578760e01b81526001600160a01b03848116600483015283169063b526578790602401602060405180830381865afa158015613364573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061338891906152c3565b6133a557604051631d0b13c160e31b815260040160405180910390fd5b6001600160a01b038381165f90815260976020526040902080546001600160a01b0319169184169190911790557f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf85836133fd81610f68565b604080516001600160a01b039384168152929091166020830152016132eb565b613425613fa1565b6001600160a01b03811661348a5760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401613272565b61349381613ffb565b50565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156134f2573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061351691906157bf565b6001600160a01b0316336001600160a01b0316146135475760405163794821ff60e01b815260040160405180910390fd5b6066548019821981161461356e5760405163c61dca5d60e01b815260040160405180910390fd5b606682905560405182815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200160405180910390a25050565b5f815f0151826020015163ffffffff166040516020016135f792919060609290921b6bffffffffffffffffffffffff1916825260a01b6001600160a01b031916601482015260200190565b60405160208183030381529060405261099a906157da565b6040805180820182525f80825260208083018290528351606081018552828152808201839052808501839052845180860186526001600160a01b03898116855260a184528685209088168552909252938220929392819061366f906138ac565b6001600160401b0390811682526001600160a01b038981165f81815260a260209081526040808320948c168084529482528083205486169682019690965291815260a082528481208b8252825284812092815291815290839020835160608101855290549283168152600160401b8304600f0b91810191909152600160c01b90910463ffffffff16918101829052919250431015613711579092509050613773565b613722815f015182602001516138bf565b6001600160401b0316815260208101515f600f9190910b121561376057613751826020015182602001516138bf565b6001600160401b031660208301525b5f60408201819052602082015290925090505b935093915050565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a250565b5f61099a825490565b5f61077683836140ad565b5f8181526001830160205260408120541515610776565b604051631beb2b9760e31b81526001600160a01b0382811660048301523360248301523060448301525f80356001600160e01b0319166064840152917f00000000000000000000000000000000000000000000000000000000000000009091169063df595cb8906084016020604051808303815f875af1158015613869573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061099a91906152c3565b5f61077683836140d3565b5f610776836001600160a01b0384166140d3565b5f61099a82670de0b6b3a764000061411f565b5f610776826001600160401b038516615540565b5f61077683670de0b6b3a764000084614163565b5f6107768383670de0b6b3a7640000614163565b6001600160a01b0382165f908152609e6020526040812081908161391e856135ac565b815260208082019290925260409081015f2081518083019092525460ff8116151580835261010090910463ffffffff16928201929092529150806131d457506020015163ffffffff164311159392505050565b6001600160a01b0381165f9081526001830160205260408120541515610776565b5f6107768383670de0b6b3a76400006001614248565b6020808301516001600160a01b038088165f90815260a284526040808220928816825291909352909120546001600160401b03908116911614613a6e57602082810180516001600160a01b038881165f81815260a286526040808220938a1680835293875290819020805467ffffffffffffffff19166001600160401b0395861617905593518451918252948101919091529216908201527facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559060600160405180910390a15b6001600160a01b038086165f90815260a060209081526040808320888452825280832093871683529281529082902083518154928501519385015163ffffffff16600160c01b0263ffffffff60c01b196001600160801b038616600160401b026001600160c01b03199095166001600160401b03909316929092179390931716919091179055600f0b15613b50576001600160a01b0385165f908152609f602090815260408083208784529091529020613b289084613898565b506001600160a01b0385165f908152609d60205260409020613b4a908561388d565b50610c74565b80516001600160401b03165f03610c74576001600160a01b0385165f908152609f602090815260408083208784529091529020613b8d9084613f8d565b506001600160a01b0385165f908152609f602090815260408083208784529091529020613bb9906137b8565b5f03610c74576001600160a01b0385165f908152609d602052604090206123bb9085613f82565b6001600160a01b038084165f90815260a160209081526040808320938616835292905220613c0f9043836142a1565b604080516001600160a01b038086168252841660208201526001600160401b038316918101919091527f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c906060016132eb565b60605f610776836142b5565b6001600160a01b038381165f90815260a360209081526040808320938616835292905290812054600f81810b600160801b909204900b035b5f81118015613cb857508261ffff1682105b15610c74576001600160a01b038086165f90815260a3602090815260408083209388168352929052908120613cec9061430e565b90505f5f613cfb88848961360f565b91509150806040015163ffffffff16431015613d1957505050610c74565b613d2688848985856139a8565b6001600160a01b038089165f90815260a360209081526040808320938b16835292905220613d5390614360565b50613d5d856157fd565b9450613d6884615815565b9350505050613ca6565b6001600160a01b0382165f908152609b60209081526040918290208251608081018452905463ffffffff808216835260ff600160201b830416151593830193909352650100000000008104831693820193909352600160481b909204166060820181905215801590613dee5750806060015163ffffffff164310155b15613e0857604081015163ffffffff168152600160208201525b63ffffffff82166040820152613e3e7f0000000000000000000000000000000000000000000000000000000000000000436156df565b63ffffffff90811660608381019182526001600160a01b0386165f818152609b602090815260409182902087518154838a0151858b01519851928a1664ffffffffff1990921691909117600160201b91151591909102176cffffffffffffffff0000000000191665010000000000978916979097026cffffffff000000000000000000191696909617600160481b968816968702179055815192835294871694820194909452928301919091527f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db91016132eb565b5f5f613f35613f21846143dd565b8554613f309190600f0b61582a565b61444a565b8454909150600160801b9004600f90810b9082900b12613f6857604051632d0483c560e21b815260040160405180910390fd5b600f0b5f9081526001939093016020525050604090205490565b5f61077683836144b3565b5f610776836001600160a01b0384166144b3565b6033546001600160a01b03163314611e625760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401613272565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0905f90a35050565b5f6107768383670de0b6b3a7640000614596565b5f6140718460995f6112ae896135ac565b801561407a5750815b8015611fd957505090516001600160401b031615159392505050565b5f6107766001600160401b03808516908416615851565b5f825f0182815481106140c2576140c26152e2565b905f5260205f200154905092915050565b5f81815260018301602052604081205461411857508154600181810184555f84815260208082209093018490558454848252828601909352604090209190915561099a565b505f61099a565b81545f90801561415b57614145846141386001846154ea565b5f91825260209091200190565b54600160201b90046001600160e01b03166131d4565b509092915050565b5f80805f19858709858702925082811083820303915050805f0361419a578382816141905761419061587e565b0492505050610776565b8084116141e15760405162461bcd60e51b81526020600482015260156024820152744d6174683a206d756c446976206f766572666c6f7760581b6044820152606401613272565b5f8486880960026001871981018816978890046003810283188082028403028082028403028082028403028082028403028082028403029081029092039091025f889003889004909101858311909403939093029303949094049190911702949350505050565b5f5f614255868686614163565b9050600183600281111561426b5761426b615892565b14801561428757505f84806142825761428261587e565b868809115b15611fd9576142976001826158a6565b9695505050505050565b6132f483836001600160401b0384166145de565b6060815f0180548060200260200160405190810160405280929190818152602001828054801561430257602002820191905f5260205f20905b8154815260200190600101908083116142ee575b50505050509050919050565b5f6143288254600f81810b600160801b909204900b131590565b1561434657604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f9081526001909101602052604090205490565b5f61437a8254600f81810b600160801b909204900b131590565b1561439857604051631ed9509560e11b815260040160405180910390fd5b508054600f0b5f818152600180840160205260408220805492905583546fffffffffffffffffffffffffffffffff191692016001600160801b03169190911790915590565b5f6001600160ff1b038211156144465760405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608401613272565b5090565b80600f81900b81146144ae5760405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608401613272565b919050565b5f818152600183016020526040812054801561458d575f6144d56001836154ea565b85549091505f906144e8906001906154ea565b9050818114614547575f865f018281548110614506576145066152e2565b905f5260205f200154905080875f018481548110614526576145266152e2565b5f918252602080832090910192909255918252600188019052604090208390555b8554869080614558576145586158b9565b600190038181905f5260205f20015f90559055856001015f8681526020019081526020015f205f90556001935050505061099a565b5f91505061099a565b82545f90816145a7868683856146e1565b905080156145d4576145be866141386001846154ea565b54600160201b90046001600160e01b0316610771565b5091949350505050565b82548015614694575f6145f6856141386001856154ea565b60408051808201909152905463ffffffff808216808452600160201b9092046001600160e01b0316602084015291925090851610156146485760405163151b8e3f60e11b815260040160405180910390fd5b805163ffffffff8086169116036146925782614669866141386001866154ea565b80546001600160e01b0392909216600160201b0263ffffffff9092169190911790555050505050565b505b506040805180820190915263ffffffff92831681526001600160e01b03918216602080830191825285546001810187555f968752952091519051909216600160201b029190921617910155565b5f5b81831015610961575f6146f68484614734565b5f8781526020902090915063ffffffff86169082015463ffffffff1611156147205780925061472e565b61472b8160016158a6565b93505b506146e3565b5f61474260028484186158cd565b610776908484166158a6565b6001600160a01b0381168114613493575f5ffd5b634e487b7160e01b5f52604160045260245ffd5b604051606081016001600160401b038111828210171561479857614798614762565b60405290565b604051601f8201601f191681016001600160401b03811182821017156147c6576147c6614762565b604052919050565b803563ffffffff811681146144ae575f5ffd5b5f604082840312156147f1575f5ffd5b604080519081016001600160401b038111828210171561481357614813614762565b60405290508082356148248161474e565b8152614832602084016147ce565b60208201525092915050565b5f5f5f60808486031215614850575f5ffd5b833561485b8161474e565b925061486a85602086016147e1565b9150606084013561487a8161474e565b809150509250925092565b81516001600160401b03168152602080830151600f0b9082015260408083015163ffffffff16908201526060810161099a565b5f602082840312156148c8575f5ffd5b5035919050565b5f602082840312156148df575f5ffd5b81356107768161474e565b80516001600160a01b0316825260209081015163ffffffff16910152565b5f8151808452602084019350602083015f5b828110156149435761492d8683516148ea565b604095909501946020919091019060010161491a565b5093949350505050565b602081525f6107766020830184614908565b5f6040828403121561496f575f5ffd5b61077683836147e1565b5f5f83601f840112614989575f5ffd5b5081356001600160401b0381111561499f575f5ffd5b6020830191508360208260051b850101111561191d575f5ffd5b5f5f5f604084860312156149cb575f5ffd5b83356149d68161474e565b925060208401356001600160401b038111156149f0575f5ffd5b6149fc86828701614979565b9497909650939450505050565b5f6001600160401b03821115614a2157614a21614762565b5060051b60200190565b5f82601f830112614a3a575f5ffd5b8135614a4d614a4882614a09565b61479e565b8082825260208201915060208360051b860101925085831115614a6e575f5ffd5b602085015b83811015614a94578035614a868161474e565b835260209283019201614a73565b5095945050505050565b5f5f5f5f60a08587031215614ab1575f5ffd5b614abb86866147e1565b935060408501356001600160401b03811115614ad5575f5ffd5b614ae187828801614a2b565b93505060608501356001600160401b03811115614afc575f5ffd5b614b0887828801614a2b565b925050614b17608086016147ce565b905092959194509250565b5f8151808452602084019350602083015f5b82811015614943578151865260209586019590910190600101614b34565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015614ba957603f19878603018452614b94858351614b22565b94506020938401939190910190600101614b78565b50929695505050505050565b5f5f60408385031215614bc6575f5ffd5b8235614bd18161474e565b915060208301356001600160401b03811115614beb575f5ffd5b830160a08186031215614bfc575f5ffd5b809150509250929050565b5f5f60408385031215614c18575f5ffd5b8235614c238161474e565b91506020830135614bfc8161474e565b5f8151808452602084019350602083015f5b8281101561494357614c7e86835180516001600160401b03168252602080820151600f0b9083015260409081015163ffffffff16910152565b6060959095019460209190910190600101614c45565b604081525f614ca66040830185614908565b8281036020840152611fd98185614c33565b5f8151808452602084019350602083015f5b828110156149435781516001600160a01b0316865260209586019590910190600101614cca565b602081525f6107766020830184614cb8565b5f5f60408385031215614d14575f5ffd5b82356001600160401b03811115614d29575f5ffd5b614d3585828601614a2b565b9250506020830135614bfc8161474e565b602080825282518282018190525f918401906040840190835b81811015614d865783516001600160401b0316835260209384019390920191600101614d5f565b509095945050505050565b5f5f5f5f5f60608688031215614da5575f5ffd5b8535614db08161474e565b945060208601356001600160401b03811115614dca575f5ffd5b614dd688828901614979565b90955093505060408601356001600160401b03811115614df4575f5ffd5b614e0088828901614979565b969995985093965092949392505050565b5f5f5f5f60608587031215614e24575f5ffd5b8435614e2f8161474e565b9350614e3d602086016147ce565b925060408501356001600160401b03811115614e57575f5ffd5b614e6387828801614979565b95989497509550505050565b5f5f60408385031215614e80575f5ffd5b8235614e8b8161474e565b915060208301356001600160401b03811115614ea5575f5ffd5b614eb185828601614a2b565b9150509250929050565b5f5f60408385031215614ecc575f5ffd5b8235614ed78161474e565b9150614ee5602084016147ce565b90509250929050565b5f60208284031215614efe575f5ffd5b813560ff81168114610776575f5ffd5b5f5f60608385031215614f1f575f5ffd5b8235614f2a8161474e565b9150614ee584602085016147e1565b5f60608284031215614f49575f5ffd5b50919050565b5f60208284031215614f5f575f5ffd5b81356001600160401b03811115614f74575f5ffd5b6131d484828501614f39565b5f5f5f60808486031215614f92575f5ffd5b83356001600160401b03811115614fa7575f5ffd5b614fb386828701614a2b565b93505061486a85602086016147e1565b602081525f6107766020830184614c33565b5f5f5f60608486031215614fe7575f5ffd5b8335614ff28161474e565b925060208401356001600160401b0381111561500c575f5ffd5b61501886828701614a2b565b925050615027604085016147ce565b90509250925092565b5f5f60408385031215615041575f5ffd5b823561504c8161474e565b915060208301356001600160401b03811115615066575f5ffd5b8301601f81018513615076575f5ffd5b8035615084614a4882614a09565b8082825260208201915060208360051b8501019250878311156150a5575f5ffd5b602084015b838110156151ca5780356001600160401b038111156150c7575f5ffd5b85016080818b03601f190112156150dc575f5ffd5b6150e4614776565b6150f18b602084016147e1565b815260608201356001600160401b0381111561510b575f5ffd5b61511a8c602083860101614a2b565b60208301525060808201356001600160401b03811115615138575f5ffd5b6020818401019250508a601f83011261514f575f5ffd5b813561515d614a4882614a09565b8082825260208201915060208360051b86010192508d83111561517e575f5ffd5b6020850194505b828510156151b45784356001600160401b03811681146151a3575f5ffd5b825260209485019490910190615185565b60408401525050845250602092830192016150aa565b50809450505050509250929050565b5f5f5f604084860312156151eb575f5ffd5b83356151f68161474e565b925060208401356001600160401b03811115615210575f5ffd5b8401601f81018613615220575f5ffd5b80356001600160401b03811115615235575f5ffd5b866020828401011115615246575f5ffd5b939660209190910195509293505050565b5f5f60408385031215615268575f5ffd5b82356152738161474e565b915060208301356001600160401b0381111561528d575f5ffd5b614eb185828601614f39565b5f5f604083850312156152aa575f5ffd5b82356152b58161474e565b946020939093013593505050565b5f602082840312156152d3575f5ffd5b81518015158114610776575f5ffd5b634e487b7160e01b5f52603260045260245ffd5b5f8235603e1983360301811261530a575f5ffd5b9190910192915050565b5f60208284031215615324575f5ffd5b610776826147ce565b6040810161099a82846148ea565b5f5f8335601e19843603018112615350575f5ffd5b8301803591506001600160401b03821115615369575f5ffd5b6020019150600581901b360382131561191d575f5ffd5b6060810161538e82856148ea565b6001600160a01b039290921660409190910152919050565b604081525f6153b86040830185614cb8565b8281036020840152611fd98185614cb8565b5f602082840312156153da575f5ffd5b81516001600160401b038111156153ef575f5ffd5b8201601f810184136153ff575f5ffd5b805161540d614a4882614a09565b8082825260208201915060208360051b85010192508683111561542e575f5ffd5b602084015b838110156154cb5780516001600160401b03811115615450575f5ffd5b8501603f81018913615460575f5ffd5b6020810151615471614a4882614a09565b808282526020820191506020808460051b8601010192508b831115615494575f5ffd5b6040840193505b828410156154b657835182526020938401939091019061549b565b86525050602093840193919091019050615433565b509695505050505050565b634e487b7160e01b5f52601160045260245ffd5b8181038181111561099a5761099a6154d6565b6001600160401b03828116828216039081111561099a5761099a6154d6565b5f81600f0b60016001607f1b03198103615538576155386154d6565b5f0392915050565b600f81810b9083900b0160016001607f1b03811360016001607f1b03198212171561099a5761099a6154d6565b6001600160a01b038616815260c0810161558a60208301876148ea565b6001600160a01b039490941660608201526001600160401b0392909216608083015263ffffffff1660a09091015292915050565b5f5f8335601e198436030181126155d3575f5ffd5b8301803591506001600160401b038211156155ec575f5ffd5b60200191503681900382131561191d575f5ffd5b81835281816020850137505f828201602090810191909152601f909101601f19169091010190565b6001600160a01b03881681525f60c08201615646602084018a6148ea565b60c060608401528690528660e083015f5b8881101561568757823561566a8161474e565b6001600160a01b0316825260209283019290910190600101615657565b50838103608085015261569a8188614b22565b91505082810360a08401526156b0818587615600565b9a9950505050505050505050565b5f602082840312156156ce575f5ffd5b813561ffff81168114610776575f5ffd5b63ffffffff818116838216019081111561099a5761099a6154d6565b8183526020830192505f815f5b848110156149435763ffffffff61571e836147ce565b1686526020958601959190910190600101615708565b6001600160a01b038581168252841660208201526060604082018190525f9061429790830184866156fb565b602081525f6131d4602083018486615600565b6001600160a01b038781168252861660208201526080604082018190525f9061579f90830186886156fb565b82810360608401526157b2818587615600565b9998505050505050505050565b5f602082840312156157cf575f5ffd5b81516107768161474e565b80516020808301519190811015614f49575f1960209190910360031b1b16919050565b5f6001820161580e5761580e6154d6565b5060010190565b5f81615823576158236154d6565b505f190190565b8082018281125f831280158216821582161715615849576158496154d6565b505092915050565b600f82810b9082900b0360016001607f1b0319811260016001607f1b038213171561099a5761099a6154d6565b634e487b7160e01b5f52601260045260245ffd5b634e487b7160e01b5f52602160045260245ffd5b8082018082111561099a5761099a6154d6565b634e487b7160e01b5f52603160045260245ffd5b5f826158e757634e487b7160e01b5f52601260045260245ffd5b50049056fea2646970667358221220c3ba315211dbc6ce0ca79751329dee4d7a411ed4af89203aa0cc2a16f56ef31764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\x81W_5`\xE0\x1C\x80cn\x87]\xBA\x11a\x01VW\x80c\xA9\x84\xEB:\x11a\0\xCAW\x80c\xC2!\xD8\xAE\x11a\0\x84W\x80c\xC2!\xD8\xAE\x14a\x06\xBCW\x80c\xCDm\xC6\x87\x14a\x06\xCFW\x80c\xD3\xD9o\xF4\x14a\x06\xE2W\x80c\xDF\\\xF7#\x14a\x06\xF5W\x80c\xF2\xFD\xE3\x8B\x14a\x07\x1CW\x80c\xFA\xBC\x1C\xBC\x14a\x07/W__\xFD[\x80c\xA9\x84\xEB:\x14a\x06\x0EW\x80c\xAD\xC2\xE3\xD9\x14a\x06AW\x80c\xB2Dz\xF7\x14a\x06TW\x80c\xB6k\xD9\x89\x14a\x06gW\x80c\xB9\xFB\xAE\xD1\x14a\x06zW\x80c\xBA\x1A\x84\xE5\x14a\x06\xA9W__\xFD[\x80c\x8C\xE6HT\x11a\x01\x1BW\x80c\x8C\xE6HT\x14a\x05\x91W\x80c\x8D\xA5\xCB[\x14a\x05\xB1W\x80c\x94\xD7\xD0\x0C\x14a\x05\xC2W\x80c\x95(\x99\xEE\x14a\x05\xD5W\x80c\xA93>\xC8\x14a\x05\xE8W\x80c\xA9\x82\x18!\x14a\x05\xFBW__\xFD[\x80cn\x87]\xBA\x14a\x05\x15W\x80cqP\x18\xA6\x14a\x05(W\x80cy\xAEP\xCD\x14a\x050W\x80c{\xC1\xEFa\x14a\x05CW\x80c\x88o\x11\x95\x14a\x05jW__\xFD[\x80cFW\xE2j\x11a\x01\xF8W\x80cY\\jg\x11a\x01\xB2W\x80cY\\jg\x14a\x04\x87W\x80cZ\xC8j\xB7\x14a\x04\x8FW\x80c\\\x97Z\xBB\x14a\x04\xB2W\x80cg\r;\xA2\x14a\x04\xC4W\x80cl\xFBD\x81\x14a\x04\xD7W\x80cn4\x92\xB5\x14a\x05\x02W__\xFD[\x80cFW\xE2j\x14a\x03\xF4W\x80cJ\x10\xFF\xE5\x14a\x04\x1BW\x80cKPF\xEF\x14a\x04;W\x80cP\xFE\xEA \x14a\x04NW\x80cTz\xFB\x87\x14a\x04aW\x80cV\xC4\x83\xE6\x14a\x04tW__\xFD[\x80c)\x81\xEBw\x11a\x02IW\x80c)\x81\xEBw\x14a\x03\x19W\x80c+\xAB,J\x14a\x03UW\x80c0L\x10\xCD\x14a\x03uW\x80c65 W\x14a\x03\xA0W\x80c@\x12\r\xAB\x14a\x03\xB3W\x80cAw\xA8|\x14a\x03\xD4W__\xFD[\x80c\x10\xE1\xB9\xB8\x14a\x02\x85W\x80c\x13d9\xDD\x14a\x02\xAEW\x80c\x15\xFEP(\x14a\x02\xC3W\x80c&\r\xC7X\x14a\x02\xE3W\x80c&\x1F\x84\xE0\x14a\x03\x06W[__\xFD[a\x02\x98a\x02\x936`\x04aH>V[a\x07BV[`@Qa\x02\xA5\x91\x90aH\x85V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC1a\x02\xBC6`\x04aH\xB8V[a\x07}V[\0[a\x02\xD6a\x02\xD16`\x04aH\xCFV[a\x08RV[`@Qa\x02\xA5\x91\x90aIMV[a\x02\xF6a\x02\xF16`\x04aI_V[a\tiV[`@Q\x90\x15\x15\x81R` \x01a\x02\xA5V[a\x02\xC1a\x03\x146`\x04aI\xB9V[a\t\xA0V[a\x03@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x03ha\x03c6`\x04aJ\x9EV[a\x0C{V[`@Qa\x02\xA5\x91\x90aKRV[a\x03\x88a\x03\x836`\x04aH\xCFV[a\x0FhV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x02\xC1a\x03\xAE6`\x04aK\xB5V[a\x0F\x97V[a\x03\xC6a\x03\xC16`\x04aL\x07V[a\x17\xA9V[`@Qa\x02\xA5\x92\x91\x90aL\x94V[a\x03\xE7a\x03\xE26`\x04aI_V[a\x19$V[`@Qa\x02\xA5\x91\x90aL\xF1V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x04.a\x04)6`\x04aM\x03V[a\x19HV[`@Qa\x02\xA5\x91\x90aMFV[a\x02\xC1a\x04I6`\x04aM\x91V[a\x19\xF0V[a\x02\xC1a\x04\\6`\x04aN\x11V[a\x1A\xAAV[a\x04.a\x04o6`\x04aNoV[a\x1C\x08V[a\x02\xC1a\x04\x826`\x04aN\xBBV[a\x1C\xB0V[a\x02\xC1a\x1D\xB5V[a\x02\xF6a\x04\x9D6`\x04aN\xEEV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02\xA5V[a\x02\xF6a\x04\xD26`\x04aO\x0EV[a\x1EdV[a\x04\xEAa\x04\xE56`\x04aL\x07V[a\x1EuV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x02\xA5V[a\x02\xC1a\x05\x106`\x04aOOV[a\x1F\xE2V[a\x03\xE7a\x05#6`\x04aI_V[a#\xC3V[a\x02\xC1a#\xD4V[a\x02\xD6a\x05>6`\x04aH\xCFV[a#\xE5V[a\x03@\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x05\xA4a\x05\x9F6`\x04aO\x80V[a$\xBFV[`@Qa\x02\xA5\x91\x90aO\xC3V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\x88V[a\x04.a\x05\xD06`\x04aO\xD5V[a%\x84V[a\x02\xC1a\x05\xE36`\x04aP0V[a&pV[a\x04\xEAa\x05\xF66`\x04aL\x07V[a+7V[a\x02\xC1a\x06\t6`\x04aQ\xD9V[a+fV[a\x04\xEAa\x06\x1C6`\x04aL\x07V[`\xA2` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[a\x02\xC1a\x06O6`\x04aRWV[a,\x18V[a\x04\xB6a\x06b6`\x04aI_V[a/uV[a\x02\xC1a\x06u6`\x04aN\x11V[a/\x97V[a\x06\x8Da\x06\x886`\x04aH\xCFV[a0\xF1V[`@\x80Q\x92\x15\x15\x83Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x83\x01R\x01a\x02\xA5V[a\x04\xB6a\x06\xB76`\x04aH\xCFV[a1\x8BV[a\x03\xE7a\x06\xCA6`\x04aO\x0EV[a1\xABV[a\x02\xC1a\x06\xDD6`\x04aR\x99V[a1\xDCV[a\x02\xC1a\x06\xF06`\x04aL\x07V[a2\xF9V[a\x03\x88\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xC1a\x07*6`\x04aH\xCFV[a4\x1DV[a\x02\xC1a\x07=6`\x04aH\xB8V[a4\x96V[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90a\x07q\x85a\x07k\x86a5\xACV[\x85a6\x0FV[\x92PPP[\x93\x92PPPV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xDFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\x03\x91\x90aR\xC3V[a\x08 W`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x81\x81\x16\x81\x14a\x08EW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08N\x82a7{V[PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x90a\x08v\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\x91Wa\x08\x91aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xD5W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x08\xAFW\x90P[P\x90P_[\x82\x81\x10\x15a\taW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a\t<\x90a\t\x07\x90\x83a7\xC1V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01RP`@\x80Q\x80\x82\x01\x90\x91R``\x82\x90\x1C\x81Rc\xFF\xFF\xFF\xFF\x90\x91\x16` \x82\x01R\x90V[\x82\x82\x81Q\x81\x10a\tNWa\tNaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x08\xDAV[P\x93\x92PPPV[` \x80\x82\x01Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x82 a\t\x9A\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[\x92\x91PPV[\x82a\t\xAA\x81a7\xE3V[a\t\xC7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a\t\xFFW`@Qc\xCA\xF9ua`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x82\x81\x10\x15a\x0CtW_`@Q\x80`@\x01`@R\x80\x87`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x86\x85\x81\x81\x10a\n6Wa\n6aR\xE2V[\x90P` \x02\x81\x01\x90a\nH\x91\x90aR\xF6V[a\nV\x90` \x81\x01\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa\n\xA0\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x89`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a8\x8D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\n\xBDW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@Q\x80`@\x01`@R\x80\x88`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x83` \x01Qc\xFF\xFF\xFF\xFF\x16\x81RP`@Qa\x0B\x13\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA1_a\x0B%\x82a5\xACV[\x90P_[\x86\x86\x85\x81\x81\x10a\x0B;Wa\x0B;aR\xE2V[\x90P` \x02\x81\x01\x90a\x0BM\x91\x90aR\xF6V[a\x0B[\x90` \x81\x01\x90aS;V[\x90P\x81\x10\x15a\x0CiWa\x0B\xD1\x87\x87\x86\x81\x81\x10a\x0ByWa\x0ByaR\xE2V[\x90P` \x02\x81\x01\x90a\x0B\x8B\x91\x90aR\xF6V[a\x0B\x99\x90` \x81\x01\x90aS;V[\x83\x81\x81\x10a\x0B\xA9Wa\x0B\xA9aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x0B\xBE\x91\x90aH\xCFV[_\x84\x81R`\x99` R`@\x90 \x90a8\x98V[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x88\x88\x87\x81\x81\x10a\x0C\x06Wa\x0C\x06aR\xE2V[\x90P` \x02\x81\x01\x90a\x0C\x18\x91\x90aR\xF6V[a\x0C&\x90` \x81\x01\x90aS;V[\x84\x81\x81\x10a\x0C6Wa\x0C6aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x0CK\x91\x90aH\xCFV[`@Qa\x0CY\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x0B)V[PPP`\x01\x01a\n\x01V[PPPPPV[``\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C\x96Wa\x0C\x96aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\xC9W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x0C\xB4W\x90P[P\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xF0\xE0\xE6v\x86\x86`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\r\x1B\x92\x91\x90aS\xA6V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r5W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\\\x91\x90\x81\x01\x90aS\xCAV[\x90P_[\x85Q\x81\x10\x15a\x0F^W_\x86\x82\x81Q\x81\x10a\r|Wa\r|aR\xE2V[` \x02` \x01\x01Q\x90P\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r\x9FWa\r\x9FaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\r\xC8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x84\x83\x81Q\x81\x10a\r\xDBWa\r\xDBaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R_[\x86Q\x81\x10\x15a\x0FTW_\x87\x82\x81Q\x81\x10a\x0E\x04Wa\x0E\x04aR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1\x84R`@\x80\x82 \x92\x84\x16\x82R\x91\x90\x93R\x82 \x90\x92Pa\x0E?\x90a8\xACV[\x90P\x80`\x01`\x01`@\x1B\x03\x16_\x03a\x0EXWPPa\x0FLV[_a\x0Ed\x85\x8D\x85a\x07BV[\x90P\x88c\xFF\xFF\xFF\xFF\x16\x81`@\x01Qc\xFF\xFF\xFF\xFF\x16\x11\x15\x80\x15a\x0E\x8CWP_\x81` \x01Q`\x0F\x0B\x12[\x15a\x0E\xAEWa\x0E\xA2\x81_\x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16\x81R[\x80Q_\x90a\x0E\xC9\x90`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x85\x16a8\xD3V[\x90Pa\x0F\x10\x81\x89\x89\x81Q\x81\x10a\x0E\xE1Wa\x0E\xE1aR\xE2V[` \x02` \x01\x01Q\x87\x81Q\x81\x10a\x0E\xFAWa\x0E\xFAaR\xE2V[` \x02` \x01\x01Qa8\xE7\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89\x88\x81Q\x81\x10a\x0F\"Wa\x0F\"aR\xE2V[` \x02` \x01\x01Q\x86\x81Q\x81\x10a\x0F;Wa\x0F;aR\xE2V[` \x02` \x01\x01\x81\x81RPPPPPP[`\x01\x01a\r\xE8V[PP`\x01\x01a\r`V[PP\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\x97` R`@\x81 T\x90\x91\x16\x80\x15a\x0F\x90W\x80a\x07vV[P\x90\x91\x90PV[`fT`\x01\x90`\x02\x90\x81\x16\x03a\x0F\xC0W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a\x0F\xCA\x81a7\xE3V[a\x0F\xE7W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_`@Q\x80`@\x01`@R\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x85` \x01` \x81\x01\x90a\x10\x15\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x90R\x90P_a\x106a\x100` \x87\x01\x87aH\xCFV[\x83a8\xFBV[\x90Pa\x10E``\x86\x01\x86aS;V[\x90Pa\x10T`@\x87\x01\x87aS;V[\x90P\x14a\x10tW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 a\x10\xA6\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[a\x10\xC3W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80a\x10\xE1W`@Qc\xEB\xBF\xF4\x97`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a\x10\xEF`@\x87\x01\x87aS;V[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x08Wa\x11\x08aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x111W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[a\x11C`@\x88\x01\x88aS;V[\x90P\x81\x10\x15a\x17:W\x80\x15\x80a\x11\xD6WPa\x11a`@\x88\x01\x88aS;V[a\x11l`\x01\x84aT\xEAV[\x81\x81\x10a\x11{Wa\x11{aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x11\x90\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16a\x11\xA6`@\x89\x01\x89aS;V[\x83\x81\x81\x10a\x11\xB6Wa\x11\xB6aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x11\xCB\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x11[a\x11\xF3W`@Qc\x9F\x1C\x80S`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\0``\x88\x01\x88aS;V[\x82\x81\x81\x10a\x12\x10Wa\x12\x10aR\xE2V[\x90P` \x02\x015_\x10\x80\x15a\x12PWPg\r\xE0\xB6\xB3\xA7d\0\0a\x126``\x89\x01\x89aS;V[\x83\x81\x81\x10a\x12FWa\x12FaR\xE2V[\x90P` \x02\x015\x11\x15[a\x12mW`@Qc\x13S`1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x12\xC9a\x12}`@\x89\x01\x89aS;V[\x83\x81\x81\x10a\x12\x8DWa\x12\x8DaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x12\xA2\x91\x90aH\xCFV[`\x99_a\x12\xAE\x88a5\xACV[\x81R` \x01\x90\x81R` \x01_ a9q\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xE6W`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x80a\x138a\x12\xF8` \x8B\x01\x8BaH\xCFV[a\x13\x01\x88a5\xACV[a\x13\x0E`@\x8D\x01\x8DaS;V[\x87\x81\x81\x10a\x13\x1EWa\x13\x1EaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x133\x91\x90aH\xCFV[a6\x0FV[\x80Q\x91\x93P\x91P`\x01`\x01`@\x1B\x03\x16_\x03a\x13UWPPa\x172V[_a\x13\x90a\x13f``\x8C\x01\x8CaS;V[\x86\x81\x81\x10a\x13vWa\x13vaR\xE2V[\x85Q`\x01`\x01`@\x1B\x03\x16\x92` \x90\x91\x02\x015\x90Pa9\x92V[\x83Q\x90\x91Pa\x13\xAB`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x83\x16a8\xD3V[\x86\x86\x81Q\x81\x10a\x13\xBDWa\x13\xBDaR\xE2V[` \x02` \x01\x01\x81\x81RPP\x81\x83_\x01\x81\x81Qa\x13\xDA\x91\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP\x83Q\x82\x90\x85\x90a\x13\xF7\x90\x83\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x84\x01\x80Q\x83\x91\x90a\x14\x17\x90\x83\x90aT\xFDV[`\x01`\x01`@\x1B\x03\x16\x90RP` \x83\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a\x152W_a\x14za\x14F``\x8E\x01\x8EaS;V[\x88\x81\x81\x10a\x14VWa\x14VaR\xE2V[\x90P` \x02\x015\x85` \x01Qa\x14k\x90aU\x1CV[`\x01`\x01`\x80\x1B\x03\x16\x90a9\x92V[\x90P\x80`\x01`\x01`@\x1B\x03\x16\x84` \x01\x81\x81Qa\x14\x97\x91\x90aU@V[`\x0F\x0B\x90RP\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDDa\x14\xCB` \x8E\x01\x8EaH\xCFV[\x8A\x8E\x80`@\x01\x90a\x14\xDC\x91\x90aS;V[\x8A\x81\x81\x10a\x14\xECWa\x14\xECaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15\x01\x91\x90aH\xCFV[a\x15\x12\x88_\x01Q\x89` \x01Qa8\xBFV[\x88`@\x01Q`@Qa\x15(\x95\x94\x93\x92\x91\x90aUmV[`@Q\x80\x91\x03\x90\xA1P[a\x15\x84a\x15B` \x8D\x01\x8DaH\xCFV[a\x15K\x8Aa5\xACV[a\x15X`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x15hWa\x15haR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15}\x91\x90aH\xCFV[\x87\x87a9\xA8V[\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDDa\x15\xB2` \x8D\x01\x8DaH\xCFV[\x89a\x15\xC0`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x15\xD0Wa\x15\xD0aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x15\xE5\x91\x90aH\xCFV[\x86Q`@Qa\x15\xF9\x94\x93\x92\x91\x90C\x90aUmV[`@Q\x80\x91\x03\x90\xA1a\x16Ja\x16\x11` \x8D\x01\x8DaH\xCFV[a\x16\x1E`@\x8E\x01\x8EaS;V[\x88\x81\x81\x10a\x16.Wa\x16.aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x16C\x91\x90aH\xCFV[\x86Qa;\xE0V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16c\xEEt\x93\x7Fa\x16\x86` \x8E\x01\x8EaH\xCFV[a\x16\x93`@\x8F\x01\x8FaS;V[\x89\x81\x81\x10a\x16\xA3Wa\x16\xA3aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x16\xB8\x91\x90aH\xCFV[\x87Q`@Q`\xE0\x85\x90\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x04\x82\x01R\x92\x90\x91\x16`$\x83\x01R`\x01`\x01`@\x1B\x03\x80\x86\x16`D\x84\x01R\x16`d\x82\x01R`\x84\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x17\x17W__\xFD[PZ\xF1\x15\x80\x15a\x17)W=__>=_\xFD[PPPPPPPP[`\x01\x01a\x116V[P\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5a\x17i` \x88\x01\x88aH\xCFV[\x84a\x17w`@\x8A\x01\x8AaS;V[\x85a\x17\x85`\x80\x8D\x01\x8DaU\xBEV[`@Qa\x17\x98\x97\x96\x95\x94\x93\x92\x91\x90aV(V[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9D` R`@\x81 ``\x91\x82\x91a\x17\xCE\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17\xE9Wa\x17\xE9aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18-W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\x07W\x90P[P\x90P_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18IWa\x18IaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\x92W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a\x18gW\x90P[P\x90P_[\x83\x81\x10\x15a\x19\x15W`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9D` R`@\x81 a\x18\xC4\x90a\t\x07\x90\x84a7\xC1V[\x90P\x80\x84\x83\x81Q\x81\x10a\x18\xD9Wa\x18\xD9aR\xE2V[` \x02` \x01\x01\x81\x90RPa\x18\xEF\x89\x82\x8Aa\x07BV[\x83\x83\x81Q\x81\x10a\x19\x01Wa\x19\x01aR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x18\x97V[P\x90\x93P\x91PP[\x92P\x92\x90PV[``_a\x07v`\x99_a\x196\x86a5\xACV[\x81R` \x01\x90\x81R` \x01_ a<bV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19dWa\x19daGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x8DW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a\taWa\x19\xBE\x85\x82\x81Q\x81\x10a\x19\xB0Wa\x19\xB0aR\xE2V[` \x02` \x01\x01Q\x85a+7V[\x82\x82\x81Q\x81\x10a\x19\xD0Wa\x19\xD0aR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x19\x92V[`fT_\x90`\x01\x90\x81\x16\x03a\x1A\x18W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83\x82\x14a\x1A8W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1A\xA1Wa\x1A\x99\x87\x87\x87\x84\x81\x81\x10a\x1AXWa\x1AXaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1Am\x91\x90aH\xCFV[\x86\x86\x85\x81\x81\x10a\x1A\x7FWa\x1A\x7FaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1A\x94\x91\x90aV\xBEV[a<nV[`\x01\x01a\x1A:V[PPPPPPPV[\x83a\x1A\xB4\x81a7\xE3V[a\x1A\xD1W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x90\x91R`\x01`\x01`\xA0\x1B\x03\x86\x16\x81Rc\xFF\xFF\xFF\xFF\x85\x16` \x82\x01R_a\x1A\xFD\x82a5\xACV[\x90Pa\x1B>\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x8A`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a7\xCC\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x1B[W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[\x84\x81\x10\x15a\x1B\xFEWa\x1Bz\x86\x86\x83\x81\x81\x10a\x0B\xA9Wa\x0B\xA9aR\xE2V[a\x1B\x97W`@QcX\\\xFB/`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8B\x83\x87\x87\x84\x81\x81\x10a\x1B\xCBWa\x1B\xCBaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a\x1B\xE0\x91\x90aH\xCFV[`@Qa\x1B\xEE\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a\x1B]V[PPPPPPPPV[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C$Wa\x1C$aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1CMW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\taWa\x1C~\x85\x85\x83\x81Q\x81\x10a\x1CqWa\x1CqaR\xE2V[` \x02` \x01\x01Qa+7V[\x82\x82\x81Q\x81\x10a\x1C\x90Wa\x1C\x90aR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a\x1CRV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D\xABWa\x1C\xE9\x82a7\xE3V[a\x1D\x06W`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1DjW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\x8E\x91\x90aR\xC3V[a\x1D\xABW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x08N\x82\x82a=rV[`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x17W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E;\x91\x90aR\xC3V[a\x1EXW`@Qc\x1Dw\xD4w`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1Eb_\x19a7{V[V[_a\x07v\x83`\x9A_a\x12\xAE\x86a5\xACV[`\x01`\x01`\xA0\x1B\x03\x82\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x86\x16\x80\x84R\x94\x82R\x80\x83 T\x93\x83R`\xA3\x82R\x80\x83 \x94\x83R\x93\x90R\x91\x82 T`\x01`\x01`@\x1B\x03\x90\x91\x16\x90`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03\x82[\x81\x81\x10\x15a\x1F\x9FW`\x01`\x01`\xA0\x1B\x03\x80\x87\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x89\x16\x83R\x92\x90R\x90\x81 a\x1F\x07\x90\x83a?\x13V[`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x85\x84R\x82R\x80\x83 \x93\x8B\x16\x83R\x92\x81R\x90\x82\x90 \x82Q``\x81\x01\x84R\x90T`\x01`\x01`@\x1B\x03\x81\x16\x82R`\x01`@\x1B\x81\x04`\x0F\x0B\x92\x82\x01\x92\x90\x92R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a\x1F\x82WPPa\x1F\x9FV[a\x1F\x90\x85\x82` \x01Qa8\xBFV[\x94PPP\x80`\x01\x01\x90Pa\x1E\xCFV[P`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R \x82\x90a\x1F\xCF\x90a8\xACV[a\x1F\xD9\x91\x90aT\xFDV[\x95\x94PPPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a \x0BW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a  a \x1B` \x84\x01\x84aH\xCFV[a7\xE3V[\x80a 9WPa 9a \x1B`@\x84\x01` \x85\x01aH\xCFV[a VW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a e`@\x84\x01\x84aS;V[\x90P\x81\x10\x15a#'W_`@Q\x80`@\x01`@R\x80\x85` \x01` \x81\x01\x90a \x8D\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a \xA8`@\x87\x01\x87aS;V[\x85\x81\x81\x10a \xB8Wa \xB8aR\xE2V[\x90P` \x02\x01` \x81\x01\x90a \xCD\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x16\x81RP\x90Pa!\x1A\x81` \x01Qc\xFF\xFF\xFF\xFF\x16`\x98_\x87` \x01` \x81\x01\x90a \xFC\x91\x90aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a7\xCCV[a!7W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x9E_a!G` \x87\x01\x87aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a!p\x83a5\xACV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ T`\xFF\x16a!\xA0W`@Qc%\x13\x1DO`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a!\xDAa!\xAC\x82a5\xACV[`\x9C_a!\xBC` \x89\x01\x89aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x90a?\x82V[Pa\"\x12a!\xEB` \x86\x01\x86aH\xCFV[`\x9A_a!\xF7\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a?\x8D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[Pa\" ` \x85\x01\x85aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x82`@Qa\"X\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA2`@\x80Q\x80\x82\x01\x90\x91R_\x81R` \x81\x01a\"\x9B\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[c\xFF\xFF\xFF\xFF\x16\x90R`\x9E_a\"\xB3` \x88\x01\x88aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ _a\"\xDC\x84a5\xACV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x01_ \x82Q\x81T\x93\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16a\x01\0\x02d\xFF\xFF\xFF\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x90UP`\x01\x01a XV[Pa#;a\x03\x83`@\x84\x01` \x85\x01aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16c0<\xA9Va#V` \x85\x01\x85aH\xCFV[a#f`@\x86\x01` \x87\x01aH\xCFV[a#s`@\x87\x01\x87aS;V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\x92\x94\x93\x92\x91\x90aW4V[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a#\xA9W__\xFD[PZ\xF1\x15\x80\x15a#\xBBW=__>=_\xFD[PPPPPPV[``a\t\x9A`\x9A_a\x196\x85a5\xACV[a#\xDCa?\xA1V[a\x1Eb_a?\xFBV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9C` R`@\x81 ``\x91\x90a$\t\x90a7\xB8V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a$$Wa$$aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a$hW\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a$BW\x90P[P\x90P_[\x82\x81\x10\x15a\taW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9C` R`@\x90 a$\x9A\x90a\t\x07\x90\x83a7\xC1V[\x82\x82\x81Q\x81\x10a$\xACWa$\xACaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a$mV[``_\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a$\xDBWa$\xDBaGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%$W\x81` \x01[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x92\x82\x01R\x82R_\x19\x90\x92\x01\x91\x01\x81a$\xF9W\x90P[P\x90P_[\x85Q\x81\x10\x15a%{Wa%V\x86\x82\x81Q\x81\x10a%GWa%GaR\xE2V[` \x02` \x01\x01Q\x86\x86a\x07BV[\x82\x82\x81Q\x81\x10a%hWa%haR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a%)V[P\x94\x93PPPPV[``_\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a%\xA0Wa%\xA0aGbV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a%\xC9W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x84Q\x81\x10\x15a%{W`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\xA1` R`@\x81 \x86Qa&>\x92\x87\x92\x91\x89\x90\x86\x90\x81\x10a&\tWa&\taR\xE2V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90\x81R` \x01_ a@L\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a&PWa&PaR\xE2V[`\x01`\x01`@\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`\x01\x01a%\xCEV[`fT_\x90`\x01\x90\x81\x16\x03a&\x98W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a&\xA1\x83a7\xE3V[a&\xBEW`@QcH\xF5\xC3\xED`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[___a&\xCA\x86a0\xF1V[\x91P\x91P\x81a&\xECW`@Qc\xFAU\xFC\x81`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P_\x90P[\x83Q\x81\x10\x15a\x0CtW\x83\x81\x81Q\x81\x10a'\rWa'\raR\xE2V[` \x02` \x01\x01Q`@\x01QQ\x84\x82\x81Q\x81\x10a',Wa',aR\xE2V[` \x02` \x01\x01Q` \x01QQ\x14a'WW`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x84\x82\x81Q\x81\x10a'jWa'jaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x80\x82\x01Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x93R`@\x90\x92 \x90\x92Pa'\xAA\x91c\xFF\xFF\xFF\xFF\x90\x81\x16\x90a7\xCC\x16V[a'\xC7W`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a'\xD2\x87\x83a8\xFBV[\x90P_[\x86\x84\x81Q\x81\x10a'\xE8Wa'\xE8aR\xE2V[` \x02` \x01\x01Q` \x01QQ\x81\x10\x15a+,W_\x87\x85\x81Q\x81\x10a(\x0FWa(\x0FaR\xE2V[` \x02` \x01\x01Q` \x01Q\x82\x81Q\x81\x10a(,Wa(,aR\xE2V[` \x02` \x01\x01Q\x90Pa(C\x89\x82a\xFF\xFFa<nV[__a(R\x8Ba\x07k\x88a5\xACV[\x91P\x91P\x80` \x01Q`\x0F\x0B_\x14a(}W`@Qc\r\x8F\xCB\xE3`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a(\x8A\x87\x85\x84\x89a@`V[\x90Pa(\xCF\x82_\x01Q\x8C\x8A\x81Q\x81\x10a(\xA5Wa(\xA5aR\xE2V[` \x02` \x01\x01Q`@\x01Q\x87\x81Q\x81\x10a(\xC2Wa(\xC2aR\xE2V[` \x02` \x01\x01Qa@\x96V[`\x0F\x0B` \x83\x01\x81\x90R_\x03a(\xF8W`@QcF\x06\x17\x93`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Q`\x0F\x0B\x12\x15a*<W\x80\x15a)\xBEWa)ya)\x19\x88a5\xACV[`\x01`\x01`\xA0\x1B\x03\x80\x8F\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8A\x16\x83R\x92\x90R \x90\x81T`\x01`\x80\x1B\x90\x81\x90\x04`\x0F\x0B_\x81\x81R`\x01\x80\x86\x01` R`@\x90\x91 \x93\x90\x93U\x83T`\x01`\x01`\x80\x1B\x03\x90\x81\x16\x93\x90\x91\x01\x16\x02\x17\x90UV[a)\xA3\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[a)\xAE\x90`\x01aV\xDFV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01Ra*\xA9V[a)\xD0\x83` \x01Q\x83` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16` \x84\x01R\x8AQ\x8B\x90\x89\x90\x81\x10a)\xF2Wa)\xF2aR\xE2V[` \x02` \x01\x01Q`@\x01Q\x85\x81Q\x81\x10a*\x0FWa*\x0FaR\xE2V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q`\x01`\x01`@\x1B\x03\x16\x83R_\x90\x83\x01Rc\xFF\xFF\xFF\xFFC\x16`@\x83\x01Ra*\xA9V[_\x82` \x01Q`\x0F\x0B\x13\x15a*\xA9Wa*]\x83` \x01Q\x83` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x90\x81\x16` \x85\x01\x81\x90R\x84Q\x90\x91\x16\x10\x15a*\x93W`@Qcl\x9B\xE0\xBF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a*\x9D\x89CaV\xDFV[c\xFF\xFF\xFF\xFF\x16`@\x83\x01R[a*\xBE\x8Ca*\xB6\x89a5\xACV[\x86\x86\x86a9\xA8V[\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x8Ca*\xECa\t\x07\x8Aa5\xACV[\x86a*\xFE\x86_\x01Q\x87` \x01Qa8\xBFV[\x86`@\x01Q`@Qa+\x14\x95\x94\x93\x92\x91\x90aUmV[`@Q\x80\x91\x03\x90\xA1PP`\x01\x90\x92\x01\x91Pa'\xD6\x90PV[PPP`\x01\x01a&\xF2V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R\x90\x81 a\x07v\x90a8\xACV[\x82a+p\x81a7\xE3V[a+\x8DW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 T`\xFF\x16a+\xCFW`\x01`\x01`\xA0\x1B\x03\x84\x16_\x90\x81R`\xA4` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90U[\x83`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x84\x84`@Qa,\n\x92\x91\x90aW`V[`@Q\x80\x91\x03\x90\xA2PPPPV[`fT`\x02\x90`\x04\x90\x81\x16\x03a,AW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82a,K\x81a7\xE3V[a,hW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x85\x81\x16`\x04\x83\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90cmp\xF7\xAE\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xCCW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a,\xF0\x91\x90aR\xC3V[a-\rW`@Qc\xCC\xEA\x9Eo`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_[a-\x1C` \x85\x01\x85aS;V[\x90P\x81\x10\x15a.\xE4W`@\x80Q\x80\x82\x01\x90\x91R_\x90\x80a-?` \x88\x01\x88aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x86\x80` \x01\x90a-]\x91\x90aS;V[\x85\x81\x81\x10a-mWa-maR\xE2V[\x90P` \x02\x01` \x81\x01\x90a-\x82\x91\x90aS\x14V[c\xFF\xFF\xFF\xFF\x90\x81\x16\x90\x91R` \x80\x83\x01Q\x83Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x90\x92R`@\x90\x91 \x92\x93Pa-\xBE\x92\x91\x90\x81\x16\x90a7\xCC\x16V[a-\xDBW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a-\xE5\x86\x82a8\xFBV[\x15a.\x03W`@Qclln'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.,a.\x0F\x82a5\xACV[`\x01`\x01`\xA0\x1B\x03\x88\x16_\x90\x81R`\x9C` R`@\x90 \x90a8\x8DV[Pa.X\x86`\x9A_a.=\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a8\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[P\x85`\x01`\x01`\xA0\x1B\x03\x16\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x82`@Qa.\x92\x91\x90aS-V[`@Q\x80\x91\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9E` R`@\x81 `\x01\x91a.\xBD\x84a5\xACV[\x81R` \x81\x01\x91\x90\x91R`@\x01_ \x80T`\xFF\x19\x16\x91\x15\x15\x91\x90\x91\x17\x90UP`\x01\x01a-\x0FV[Pa.\xF5a\x03\x83` \x85\x01\x85aH\xCFV[`\x01`\x01`\xA0\x1B\x03\x16c\xC6?\xD5\x02\x85a/\x11` \x87\x01\x87aH\xCFV[a/\x1E` \x88\x01\x88aS;V[a/+`@\x8A\x01\x8AaU\xBEV[`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a/L\x96\x95\x94\x93\x92\x91\x90aWsV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a/cW__\xFD[PZ\xF1\x15\x80\x15a\x1B\xFEW=__>=_\xFD[_a\t\x9A`\x9A_a/\x85\x85a5\xACV[\x81R` \x01\x90\x81R` \x01_ a7\xB8V[\x83a/\xA1\x81a7\xE3V[a/\xBEW`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x82Rc\xFF\xFF\xFF\xFF\x80\x88\x16` \x80\x85\x01\x82\x90R_\x93\x84R`\x98\x90R\x93\x90\x91 \x91\x92a/\xFD\x92\x91a7\xCC\x16V[a0\x1AW`@Qc\x1F\xB1pU`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[_a0$\x82a5\xACV[\x90P_[\x84\x81\x10\x15a\x1B\xFEWa0m\x86\x86\x83\x81\x81\x10a0EWa0EaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a0Z\x91\x90aH\xCFV[_\x84\x81R`\x99` R`@\x90 \x90a?\x8DV[a0\x8AW`@Qc1\xBC4'`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEE\x83\x87\x87\x84\x81\x81\x10a0\xBEWa0\xBEaR\xE2V[\x90P` \x02\x01` \x81\x01\x90a0\xD3\x91\x90aH\xCFV[`@Qa0\xE1\x92\x91\x90aS\x80V[`@Q\x80\x91\x03\x90\xA1`\x01\x01a0(V[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x9B` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\xFF`\x01` \x1B\x84\x04\x16\x15\x15\x95\x84\x01\x86\x90Re\x01\0\0\0\0\0\x83\x04\x82\x16\x94\x84\x01\x94\x90\x94R`\x01`H\x1B\x90\x91\x04\x16``\x82\x01\x81\x90R\x84\x93\x91\x92\x91\x90\x15\x80\x15\x90a1rWP\x82``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a1\x81WPP`@\x81\x01Q`\x01[\x95\x90\x94P\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x98` R`@\x81 a\t\x9A\x90a7\xB8V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x81 ``\x91\x90a1\xD4\x90\x82a\x196\x86a5\xACV[\x94\x93PPPPV[_Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a1\xFAWP_T`\x01`\xFF\x90\x91\x16\x10[\x80a2\x13WP0;\x15\x80\x15a2\x13WP_T`\xFF\x16`\x01\x14[a2{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a2\x9CW_\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a2\xA5\x82a7{V[a2\xAE\x83a?\xFBV[\x80\x15a2\xF4W_\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01[`@Q\x80\x91\x03\x90\xA1[PPPV[\x81a3\x03\x81a7\xE3V[a3 W`@Qc\x93-\x94\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Qc\xB5&W\x87`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R\x83\x16\x90c\xB5&W\x87\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3dW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x88\x91\x90aR\xC3V[a3\xA5W`@Qc\x1D\x0B\x13\xC1`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\x97` R`@\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x84\x16\x91\x90\x91\x17\x90U\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85\x83a3\xFD\x81a\x0FhV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01a2\xEBV[a4%a?\xA1V[`\x01`\x01`\xA0\x1B\x03\x81\x16a4\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a2rV[a4\x93\x81a?\xFBV[PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a4\xF2W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\x16\x91\x90aW\xBFV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a5GW`@QcyH!\xFF`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`fT\x80\x19\x82\x19\x81\x16\x14a5nW`@Qc\xC6\x1D\xCA]`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`f\x82\x90U`@Q\x82\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01`@Q\x80\x91\x03\x90\xA2PPV[_\x81_\x01Q\x82` \x01Qc\xFF\xFF\xFF\xFF\x16`@Q` \x01a5\xF7\x92\x91\x90``\x92\x90\x92\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x82R`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x14\x82\x01R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\t\x9A\x90aW\xDAV[`@\x80Q\x80\x82\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x83Q``\x81\x01\x85R\x82\x81R\x80\x82\x01\x83\x90R\x80\x85\x01\x83\x90R\x84Q\x80\x86\x01\x86R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16\x85R`\xA1\x84R\x86\x85 \x90\x88\x16\x85R\x90\x92R\x93\x82 \x92\x93\x92\x81\x90a6o\x90a8\xACV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`\x01`\x01`\xA0\x1B\x03\x89\x81\x16_\x81\x81R`\xA2` \x90\x81R`@\x80\x83 \x94\x8C\x16\x80\x84R\x94\x82R\x80\x83 T\x86\x16\x96\x82\x01\x96\x90\x96R\x91\x81R`\xA0\x82R\x84\x81 \x8B\x82R\x82R\x84\x81 \x92\x81R\x91\x81R\x90\x83\x90 \x83Q``\x81\x01\x85R\x90T\x92\x83\x16\x81R`\x01`@\x1B\x83\x04`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x01`\xC0\x1B\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R\x91\x92PC\x10\x15a7\x11W\x90\x92P\x90Pa7sV[a7\"\x81_\x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16\x81R` \x81\x01Q_`\x0F\x91\x90\x91\x0B\x12\x15a7`Wa7Q\x82` \x01Q\x82` \x01Qa8\xBFV[`\x01`\x01`@\x1B\x03\x16` \x83\x01R[_`@\x82\x01\x81\x90R` \x82\x01R\x90\x92P\x90P[\x93P\x93\x91PPV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2PV[_a\t\x9A\x82T\x90V[_a\x07v\x83\x83a@\xADV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07vV[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x81\x16`\x04\x83\x01R3`$\x83\x01R0`D\x83\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x84\x01R\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xDFY\\\xB8\x90`\x84\x01` `@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a8iW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9A\x91\x90aR\xC3V[_a\x07v\x83\x83a@\xD3V[_a\x07v\x83`\x01`\x01`\xA0\x1B\x03\x84\x16a@\xD3V[_a\t\x9A\x82g\r\xE0\xB6\xB3\xA7d\0\0aA\x1FV[_a\x07v\x82`\x01`\x01`@\x1B\x03\x85\x16aU@V[_a\x07v\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84aAcV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aAcV[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9E` R`@\x81 \x81\x90\x81a9\x1E\x85a5\xACV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01_ \x81Q\x80\x83\x01\x90\x92RT`\xFF\x81\x16\x15\x15\x80\x83Ra\x01\0\x90\x91\x04c\xFF\xFF\xFF\xFF\x16\x92\x82\x01\x92\x90\x92R\x91P\x80a1\xD4WP` \x01Qc\xFF\xFF\xFF\xFF\x16C\x11\x15\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16_\x90\x81R`\x01\x83\x01` R`@\x81 T\x15\x15a\x07vV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0`\x01aBHV[` \x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x80\x88\x16_\x90\x81R`\xA2\x84R`@\x80\x82 \x92\x88\x16\x82R\x91\x90\x93R\x90\x91 T`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14a:nW` \x82\x81\x01\x80Q`\x01`\x01`\xA0\x1B\x03\x88\x81\x16_\x81\x81R`\xA2\x86R`@\x80\x82 \x93\x8A\x16\x80\x83R\x93\x87R\x90\x81\x90 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x95\x86\x16\x17\x90U\x93Q\x84Q\x91\x82R\x94\x81\x01\x91\x90\x91R\x92\x16\x90\x82\x01R\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x90``\x01`@Q\x80\x91\x03\x90\xA1[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA0` \x90\x81R`@\x80\x83 \x88\x84R\x82R\x80\x83 \x93\x87\x16\x83R\x92\x81R\x90\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Qc\xFF\xFF\xFF\xFF\x16`\x01`\xC0\x1B\x02c\xFF\xFF\xFF\xFF`\xC0\x1B\x19`\x01`\x01`\x80\x1B\x03\x86\x16`\x01`@\x1B\x02`\x01`\x01`\xC0\x1B\x03\x19\x90\x95\x16`\x01`\x01`@\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90U`\x0F\x0B\x15a;PW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;(\x90\x84a8\x98V[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a;J\x90\x85a8\x8DV[Pa\x0CtV[\x80Q`\x01`\x01`@\x1B\x03\x16_\x03a\x0CtW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;\x8D\x90\x84a?\x8DV[P`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9F` \x90\x81R`@\x80\x83 \x87\x84R\x90\x91R\x90 a;\xB9\x90a7\xB8V[_\x03a\x0CtW`\x01`\x01`\xA0\x1B\x03\x85\x16_\x90\x81R`\x9D` R`@\x90 a#\xBB\x90\x85a?\x82V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R a<\x0F\x90C\x83aB\xA1V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x86\x16\x82R\x84\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x83\x16\x91\x81\x01\x91\x90\x91R\x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x90``\x01a2\xEBV[``_a\x07v\x83aB\xB5V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x03[_\x81\x11\x80\x15a<\xB8WP\x82a\xFF\xFF\x16\x82\x10[\x15a\x0CtW`\x01`\x01`\xA0\x1B\x03\x80\x86\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x88\x16\x83R\x92\x90R\x90\x81 a<\xEC\x90aC\x0EV[\x90P__a<\xFB\x88\x84\x89a6\x0FV[\x91P\x91P\x80`@\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15a=\x19WPPPa\x0CtV[a=&\x88\x84\x89\x85\x85a9\xA8V[`\x01`\x01`\xA0\x1B\x03\x80\x89\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x8B\x16\x83R\x92\x90R a=S\x90aC`V[Pa=]\x85aW\xFDV[\x94Pa=h\x84aX\x15V[\x93PPPPa<\xA6V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\xFF`\x01` \x1B\x83\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93Re\x01\0\0\0\0\0\x81\x04\x83\x16\x93\x82\x01\x93\x90\x93R`\x01`H\x1B\x90\x92\x04\x16``\x82\x01\x81\x90R\x15\x80\x15\x90a=\xEEWP\x80``\x01Qc\xFF\xFF\xFF\xFF\x16C\x10\x15[\x15a>\x08W`@\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81R`\x01` \x82\x01R[c\xFF\xFF\xFF\xFF\x82\x16`@\x82\x01Ra>>\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0CaV\xDFV[c\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x86\x16_\x81\x81R`\x9B` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x83\x8A\x01Q\x85\x8B\x01Q\x98Q\x92\x8A\x16d\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16\x91\x90\x91\x17`\x01` \x1B\x91\x15\x15\x91\x90\x91\x02\x17l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\x19\x16e\x01\0\0\0\0\0\x97\x89\x16\x97\x90\x97\x02l\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\x19\x16\x96\x90\x96\x17`\x01`H\x1B\x96\x88\x16\x96\x87\x02\x17\x90U\x81Q\x92\x83R\x94\x87\x16\x94\x82\x01\x94\x90\x94R\x92\x83\x01\x91\x90\x91R\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91\x01a2\xEBV[__a?5a?!\x84aC\xDDV[\x85Ta?0\x91\x90`\x0F\x0BaX*V[aDJV[\x84T\x90\x91P`\x01`\x80\x1B\x90\x04`\x0F\x90\x81\x0B\x90\x82\x90\x0B\x12a?hW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x0F\x0B_\x90\x81R`\x01\x93\x90\x93\x01` RPP`@\x90 T\x90V[_a\x07v\x83\x83aD\xB3V[_a\x07v\x83`\x01`\x01`\xA0\x1B\x03\x84\x16aD\xB3V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EbW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a2rV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90_\x90\xA3PPV[_a\x07v\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0aE\x96V[_a@q\x84`\x99_a\x12\xAE\x89a5\xACV[\x80\x15a@zWP\x81[\x80\x15a\x1F\xD9WPP\x90Q`\x01`\x01`@\x1B\x03\x16\x15\x15\x93\x92PPPV[_a\x07v`\x01`\x01`@\x1B\x03\x80\x85\x16\x90\x84\x16aXQV[_\x82_\x01\x82\x81T\x81\x10a@\xC2Wa@\xC2aR\xE2V[\x90_R` _ \x01T\x90P\x92\x91PPV[_\x81\x81R`\x01\x83\x01` R`@\x81 TaA\x18WP\x81T`\x01\x81\x81\x01\x84U_\x84\x81R` \x80\x82 \x90\x93\x01\x84\x90U\x84T\x84\x82R\x82\x86\x01\x90\x93R`@\x90 \x91\x90\x91Ua\t\x9AV[P_a\t\x9AV[\x81T_\x90\x80\x15aA[WaAE\x84aA8`\x01\x84aT\xEAV[_\x91\x82R` \x90\x91 \x01\x90V[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a1\xD4V[P\x90\x92\x91PPV[_\x80\x80_\x19\x85\x87\t\x85\x87\x02\x92P\x82\x81\x10\x83\x82\x03\x03\x91PP\x80_\x03aA\x9AW\x83\x82\x81aA\x90WaA\x90aX~V[\x04\x92PPPa\x07vV[\x80\x84\x11aA\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtMath: mulDiv overflow`X\x1B`D\x82\x01R`d\x01a2rV[_\x84\x86\x88\t`\x02`\x01\x87\x19\x81\x01\x88\x16\x97\x88\x90\x04`\x03\x81\x02\x83\x18\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x80\x82\x02\x84\x03\x02\x90\x81\x02\x90\x92\x03\x90\x91\x02_\x88\x90\x03\x88\x90\x04\x90\x91\x01\x85\x83\x11\x90\x94\x03\x93\x90\x93\x02\x93\x03\x94\x90\x94\x04\x91\x90\x91\x17\x02\x94\x93PPPPV[__aBU\x86\x86\x86aAcV[\x90P`\x01\x83`\x02\x81\x11\x15aBkWaBkaX\x92V[\x14\x80\x15aB\x87WP_\x84\x80aB\x82WaB\x82aX~V[\x86\x88\t\x11[\x15a\x1F\xD9WaB\x97`\x01\x82aX\xA6V[\x96\x95PPPPPPV[a2\xF4\x83\x83`\x01`\x01`@\x1B\x03\x84\x16aE\xDEV[``\x81_\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15aC\x02W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11aB\xEEW[PPPPP\x90P\x91\x90PV[_aC(\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aCFW`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[_aCz\x82T`\x0F\x81\x81\x0B`\x01`\x80\x1B\x90\x92\x04\x90\x0B\x13\x15\x90V[\x15aC\x98W`@Qc\x1E\xD9P\x95`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P\x80T`\x0F\x0B_\x81\x81R`\x01\x80\x84\x01` R`@\x82 \x80T\x92\x90U\x83To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x92\x01`\x01`\x01`\x80\x1B\x03\x16\x91\x90\x91\x17\x90\x91U\x90V[_`\x01`\x01`\xFF\x1B\x03\x82\x11\x15aDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x01a2rV[P\x90V[\x80`\x0F\x81\x90\x0B\x81\x14aD\xAEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x01a2rV[\x91\x90PV[_\x81\x81R`\x01\x83\x01` R`@\x81 T\x80\x15aE\x8DW_aD\xD5`\x01\x83aT\xEAV[\x85T\x90\x91P_\x90aD\xE8\x90`\x01\x90aT\xEAV[\x90P\x81\x81\x14aEGW_\x86_\x01\x82\x81T\x81\x10aE\x06WaE\x06aR\xE2V[\x90_R` _ \x01T\x90P\x80\x87_\x01\x84\x81T\x81\x10aE&WaE&aR\xE2V[_\x91\x82R` \x80\x83 \x90\x91\x01\x92\x90\x92U\x91\x82R`\x01\x88\x01\x90R`@\x90 \x83\x90U[\x85T\x86\x90\x80aEXWaEXaX\xB9V[`\x01\x90\x03\x81\x81\x90_R` _ \x01_\x90U\x90U\x85`\x01\x01_\x86\x81R` \x01\x90\x81R` \x01_ _\x90U`\x01\x93PPPPa\t\x9AV[_\x91PPa\t\x9AV[\x82T_\x90\x81aE\xA7\x86\x86\x83\x85aF\xE1V[\x90P\x80\x15aE\xD4WaE\xBE\x86aA8`\x01\x84aT\xEAV[T`\x01` \x1B\x90\x04`\x01`\x01`\xE0\x1B\x03\x16a\x07qV[P\x91\x94\x93PPPPV[\x82T\x80\x15aF\x94W_aE\xF6\x85aA8`\x01\x85aT\xEAV[`@\x80Q\x80\x82\x01\x90\x91R\x90Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x80\x84R`\x01` \x1B\x90\x92\x04`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01R\x91\x92P\x90\x85\x16\x10\x15aFHW`@Qc\x15\x1B\x8E?`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Qc\xFF\xFF\xFF\xFF\x80\x86\x16\x91\x16\x03aF\x92W\x82aFi\x86aA8`\x01\x86aT\xEAV[\x80T`\x01`\x01`\xE0\x1B\x03\x92\x90\x92\x16`\x01` \x1B\x02c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UPPPPPV[P[P`@\x80Q\x80\x82\x01\x90\x91Rc\xFF\xFF\xFF\xFF\x92\x83\x16\x81R`\x01`\x01`\xE0\x1B\x03\x91\x82\x16` \x80\x83\x01\x91\x82R\x85T`\x01\x81\x01\x87U_\x96\x87R\x95 \x91Q\x90Q\x90\x92\x16`\x01` \x1B\x02\x91\x90\x92\x16\x17\x91\x01UV[_[\x81\x83\x10\x15a\taW_aF\xF6\x84\x84aG4V[_\x87\x81R` \x90 \x90\x91Pc\xFF\xFF\xFF\xFF\x86\x16\x90\x82\x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15aG W\x80\x92PaG.V[aG+\x81`\x01aX\xA6V[\x93P[PaF\xE3V[_aGB`\x02\x84\x84\x18aX\xCDV[a\x07v\x90\x84\x84\x16aX\xA6V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a4\x93W__\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\x98WaG\x98aGbV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aG\xC6WaG\xC6aGbV[`@R\x91\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14aD\xAEW__\xFD[_`@\x82\x84\x03\x12\x15aG\xF1W__\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aH\x13WaH\x13aGbV[`@R\x90P\x80\x825aH$\x81aGNV[\x81RaH2` \x84\x01aG\xCEV[` \x82\x01RP\x92\x91PPV[___`\x80\x84\x86\x03\x12\x15aHPW__\xFD[\x835aH[\x81aGNV[\x92PaHj\x85` \x86\x01aG\xE1V[\x91P``\x84\x015aHz\x81aGNV[\x80\x91PP\x92P\x92P\x92V[\x81Q`\x01`\x01`@\x1B\x03\x16\x81R` \x80\x83\x01Q`\x0F\x0B\x90\x82\x01R`@\x80\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x82\x01R``\x81\x01a\t\x9AV[_` \x82\x84\x03\x12\x15aH\xC8W__\xFD[P5\x91\x90PV[_` \x82\x84\x03\x12\x15aH\xDFW__\xFD[\x815a\x07v\x81aGNV[\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICWaI-\x86\x83QaH\xEAV[`@\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aI\x1AV[P\x93\x94\x93PPPPV[` \x81R_a\x07v` \x83\x01\x84aI\x08V[_`@\x82\x84\x03\x12\x15aIoW__\xFD[a\x07v\x83\x83aG\xE1V[__\x83`\x1F\x84\x01\x12aI\x89W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aI\x9FW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x19\x1DW__\xFD[___`@\x84\x86\x03\x12\x15aI\xCBW__\xFD[\x835aI\xD6\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aI\xF0W__\xFD[aI\xFC\x86\x82\x87\x01aIyV[\x94\x97\x90\x96P\x93\x94PPPPV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aJ!WaJ!aGbV[P`\x05\x1B` \x01\x90V[_\x82`\x1F\x83\x01\x12aJ:W__\xFD[\x815aJMaJH\x82aJ\tV[aG\x9EV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15aJnW__\xFD[` \x85\x01[\x83\x81\x10\x15aJ\x94W\x805aJ\x86\x81aGNV[\x83R` \x92\x83\x01\x92\x01aJsV[P\x95\x94PPPPPV[____`\xA0\x85\x87\x03\x12\x15aJ\xB1W__\xFD[aJ\xBB\x86\x86aG\xE1V[\x93P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xD5W__\xFD[aJ\xE1\x87\x82\x88\x01aJ+V[\x93PP``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xFCW__\xFD[aK\x08\x87\x82\x88\x01aJ+V[\x92PPaK\x17`\x80\x86\x01aG\xCEV[\x90P\x92\x95\x91\x94P\x92PV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICW\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aK4V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aK\xA9W`?\x19\x87\x86\x03\x01\x84RaK\x94\x85\x83QaK\"V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aKxV[P\x92\x96\x95PPPPPPV[__`@\x83\x85\x03\x12\x15aK\xC6W__\xFD[\x825aK\xD1\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aK\xEBW__\xFD[\x83\x01`\xA0\x81\x86\x03\x12\x15aK\xFCW__\xFD[\x80\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aL\x18W__\xFD[\x825aL#\x81aGNV[\x91P` \x83\x015aK\xFC\x81aGNV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICWaL~\x86\x83Q\x80Q`\x01`\x01`@\x1B\x03\x16\x82R` \x80\x82\x01Q`\x0F\x0B\x90\x83\x01R`@\x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[``\x95\x90\x95\x01\x94` \x91\x90\x91\x01\x90`\x01\x01aLEV[`@\x81R_aL\xA6`@\x83\x01\x85aI\x08V[\x82\x81\x03` \x84\x01Ra\x1F\xD9\x81\x85aL3V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aICW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aL\xCAV[` \x81R_a\x07v` \x83\x01\x84aL\xB8V[__`@\x83\x85\x03\x12\x15aM\x14W__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aM)W__\xFD[aM5\x85\x82\x86\x01aJ+V[\x92PP` \x83\x015aK\xFC\x81aGNV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aM\x86W\x83Q`\x01`\x01`@\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aM_V[P\x90\x95\x94PPPPPV[_____``\x86\x88\x03\x12\x15aM\xA5W__\xFD[\x855aM\xB0\x81aGNV[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xCAW__\xFD[aM\xD6\x88\x82\x89\x01aIyV[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aM\xF4W__\xFD[aN\0\x88\x82\x89\x01aIyV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[____``\x85\x87\x03\x12\x15aN$W__\xFD[\x845aN/\x81aGNV[\x93PaN=` \x86\x01aG\xCEV[\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aNWW__\xFD[aNc\x87\x82\x88\x01aIyV[\x95\x98\x94\x97P\x95PPPPV[__`@\x83\x85\x03\x12\x15aN\x80W__\xFD[\x825aN\x8B\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xA5W__\xFD[aN\xB1\x85\x82\x86\x01aJ+V[\x91PP\x92P\x92\x90PV[__`@\x83\x85\x03\x12\x15aN\xCCW__\xFD[\x825aN\xD7\x81aGNV[\x91PaN\xE5` \x84\x01aG\xCEV[\x90P\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15aN\xFEW__\xFD[\x815`\xFF\x81\x16\x81\x14a\x07vW__\xFD[__``\x83\x85\x03\x12\x15aO\x1FW__\xFD[\x825aO*\x81aGNV[\x91PaN\xE5\x84` \x85\x01aG\xE1V[_``\x82\x84\x03\x12\x15aOIW__\xFD[P\x91\x90PV[_` \x82\x84\x03\x12\x15aO_W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aOtW__\xFD[a1\xD4\x84\x82\x85\x01aO9V[___`\x80\x84\x86\x03\x12\x15aO\x92W__\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xA7W__\xFD[aO\xB3\x86\x82\x87\x01aJ+V[\x93PPaHj\x85` \x86\x01aG\xE1V[` \x81R_a\x07v` \x83\x01\x84aL3V[___``\x84\x86\x03\x12\x15aO\xE7W__\xFD[\x835aO\xF2\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aP\x0CW__\xFD[aP\x18\x86\x82\x87\x01aJ+V[\x92PPaP'`@\x85\x01aG\xCEV[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aPAW__\xFD[\x825aPL\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aPfW__\xFD[\x83\x01`\x1F\x81\x01\x85\x13aPvW__\xFD[\x805aP\x84aJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15aP\xA5W__\xFD[` \x84\x01[\x83\x81\x10\x15aQ\xCAW\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aP\xC7W__\xFD[\x85\x01`\x80\x81\x8B\x03`\x1F\x19\x01\x12\x15aP\xDCW__\xFD[aP\xE4aGvV[aP\xF1\x8B` \x84\x01aG\xE1V[\x81R``\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ\x0BW__\xFD[aQ\x1A\x8C` \x83\x86\x01\x01aJ+V[` \x83\x01RP`\x80\x82\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aQ8W__\xFD[` \x81\x84\x01\x01\x92PP\x8A`\x1F\x83\x01\x12aQOW__\xFD[\x815aQ]aJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x8D\x83\x11\x15aQ~W__\xFD[` \x85\x01\x94P[\x82\x85\x10\x15aQ\xB4W\x845`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aQ\xA3W__\xFD[\x82R` \x94\x85\x01\x94\x90\x91\x01\x90aQ\x85V[`@\x84\x01RPP\x84RP` \x92\x83\x01\x92\x01aP\xAAV[P\x80\x94PPPPP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aQ\xEBW__\xFD[\x835aQ\xF6\x81aGNV[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x10W__\xFD[\x84\x01`\x1F\x81\x01\x86\x13aR W__\xFD[\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aR5W__\xFD[\x86` \x82\x84\x01\x01\x11\x15aRFW__\xFD[\x93\x96` \x91\x90\x91\x01\x95P\x92\x93PPPV[__`@\x83\x85\x03\x12\x15aRhW__\xFD[\x825aRs\x81aGNV[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aR\x8DW__\xFD[aN\xB1\x85\x82\x86\x01aO9V[__`@\x83\x85\x03\x12\x15aR\xAAW__\xFD[\x825aR\xB5\x81aGNV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aR\xD3W__\xFD[\x81Q\x80\x15\x15\x81\x14a\x07vW__\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x825`>\x19\x836\x03\x01\x81\x12aS\nW__\xFD[\x91\x90\x91\x01\x92\x91PPV[_` \x82\x84\x03\x12\x15aS$W__\xFD[a\x07v\x82aG\xCEV[`@\x81\x01a\t\x9A\x82\x84aH\xEAV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aSPW__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aSiW__\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x19\x1DW__\xFD[``\x81\x01aS\x8E\x82\x85aH\xEAV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`@\x91\x90\x91\x01R\x91\x90PV[`@\x81R_aS\xB8`@\x83\x01\x85aL\xB8V[\x82\x81\x03` \x84\x01Ra\x1F\xD9\x81\x85aL\xB8V[_` \x82\x84\x03\x12\x15aS\xDAW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aS\xEFW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aS\xFFW__\xFD[\x80QaT\raJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15aT.W__\xFD[` \x84\x01[\x83\x81\x10\x15aT\xCBW\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15aTPW__\xFD[\x85\x01`?\x81\x01\x89\x13aT`W__\xFD[` \x81\x01QaTqaJH\x82aJ\tV[\x80\x82\x82R` \x82\x01\x91P` \x80\x84`\x05\x1B\x86\x01\x01\x01\x92P\x8B\x83\x11\x15aT\x94W__\xFD[`@\x84\x01\x93P[\x82\x84\x10\x15aT\xB6W\x83Q\x82R` \x93\x84\x01\x93\x90\x91\x01\x90aT\x9BV[\x86RPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90PaT3V[P\x96\x95PPPPPPV[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[_\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x03aU8WaU8aT\xD6V[_\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17\x15a\t\x9AWa\t\x9AaT\xD6V[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R`\xC0\x81\x01aU\x8A` \x83\x01\x87aH\xEAV[`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x92\x90\x92\x16`\x80\x83\x01Rc\xFF\xFF\xFF\xFF\x16`\xA0\x90\x91\x01R\x92\x91PPV[__\x835`\x1E\x19\x846\x03\x01\x81\x12aU\xD3W__\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aU\xECW__\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\x19\x1DW__\xFD[\x81\x83R\x81\x81` \x85\x017P_\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x81R_`\xC0\x82\x01aVF` \x84\x01\x8AaH\xEAV[`\xC0``\x84\x01R\x86\x90R\x86`\xE0\x83\x01_[\x88\x81\x10\x15aV\x87W\x825aVj\x81aGNV[`\x01`\x01`\xA0\x1B\x03\x16\x82R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01aVWV[P\x83\x81\x03`\x80\x85\x01RaV\x9A\x81\x88aK\"V[\x91PP\x82\x81\x03`\xA0\x84\x01RaV\xB0\x81\x85\x87aV\0V[\x9A\x99PPPPPPPPPPV[_` \x82\x84\x03\x12\x15aV\xCEW__\xFD[\x815a\xFF\xFF\x81\x16\x81\x14a\x07vW__\xFD[c\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\t\x9AWa\t\x9AaT\xD6V[\x81\x83R` \x83\x01\x92P_\x81_[\x84\x81\x10\x15aICWc\xFF\xFF\xFF\xFFaW\x1E\x83aG\xCEV[\x16\x86R` \x95\x86\x01\x95\x91\x90\x91\x01\x90`\x01\x01aW\x08V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R\x84\x16` \x82\x01R```@\x82\x01\x81\x90R_\x90aB\x97\x90\x83\x01\x84\x86aV\xFBV[` \x81R_a1\xD4` \x83\x01\x84\x86aV\0V[`\x01`\x01`\xA0\x1B\x03\x87\x81\x16\x82R\x86\x16` \x82\x01R`\x80`@\x82\x01\x81\x90R_\x90aW\x9F\x90\x83\x01\x86\x88aV\xFBV[\x82\x81\x03``\x84\x01RaW\xB2\x81\x85\x87aV\0V[\x99\x98PPPPPPPPPV[_` \x82\x84\x03\x12\x15aW\xCFW__\xFD[\x81Qa\x07v\x81aGNV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aOIW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[_`\x01\x82\x01aX\x0EWaX\x0EaT\xD6V[P`\x01\x01\x90V[_\x81aX#WaX#aT\xD6V[P_\x19\x01\x90V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15aXIWaXIaT\xD6V[PP\x92\x91PPV[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03`\x01`\x01`\x7F\x1B\x03\x19\x81\x12`\x01`\x01`\x7F\x1B\x03\x82\x13\x17\x15a\t\x9AWa\t\x9AaT\xD6V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x80\x82\x01\x80\x82\x11\x15a\t\x9AWa\t\x9AaT\xD6V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[_\x82aX\xE7WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC3\xBA1R\x11\xDB\xC6\xCE\x0C\xA7\x97Q2\x9D\xEEMzA\x1E\xD4\xAF\x89 :\xA0\xCC*\x16\xF5n\xF3\x17dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
    struct OperatorSet { address avs; uint32 id; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
    /**Custom error with signature `InvalidAVSRegistrar()` and selector `0xe8589e08`.
    ```solidity
    error InvalidAVSRegistrar();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVSRegistrar {}
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
        impl ::core::convert::From<InvalidAVSRegistrar> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVSRegistrar) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVSRegistrar {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVSRegistrar {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVSRegistrar()";
            const SELECTOR: [u8; 4] = [232u8, 88u8, 158u8, 8u8];
            #[inline]
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
    /**Custom error with signature `InvalidAVSWithNoMetadataRegistered()` and selector `0xcaf97561`.
    ```solidity
    error InvalidAVSWithNoMetadataRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVSWithNoMetadataRegistered {}
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
        impl ::core::convert::From<InvalidAVSWithNoMetadataRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVSWithNoMetadataRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVSWithNoMetadataRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVSWithNoMetadataRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVSWithNoMetadataRegistered()";
            const SELECTOR: [u8; 4] = [202u8, 249u8, 117u8, 97u8];
            #[inline]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Custom error with signature `InvalidSnapshotOrdering()` and selector `0x2a371c7e`.
    ```solidity
    error InvalidSnapshotOrdering();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSnapshotOrdering {}
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
        impl ::core::convert::From<InvalidSnapshotOrdering> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSnapshotOrdering) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSnapshotOrdering {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSnapshotOrdering {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSnapshotOrdering()";
            const SELECTOR: [u8; 4] = [42u8, 55u8, 28u8, 126u8];
            #[inline]
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
    /**Custom error with signature `StrategiesMustBeInAscendingOrder()` and selector `0x9f1c8053`.
    ```solidity
    error StrategiesMustBeInAscendingOrder();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategiesMustBeInAscendingOrder {}
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
        impl ::core::convert::From<StrategiesMustBeInAscendingOrder> for UnderlyingRustTuple<'_> {
            fn from(value: StrategiesMustBeInAscendingOrder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategiesMustBeInAscendingOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategiesMustBeInAscendingOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StrategiesMustBeInAscendingOrder()";
            const SELECTOR: [u8; 4] = [159u8, 28u8, 128u8, 83u8];
            #[inline]
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
    constructor(address _delegation, address _pauserRegistry, address _permissionController, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _delegation: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _permissionController: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _DEALLOCATION_DELAY: u32,
        #[allow(missing_docs)]
        pub _ALLOCATION_CONFIGURATION_DELAY: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                        value._permissionController,
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
                        _permissionController: tuple.2,
                        _DEALLOCATION_DELAY: tuple.3,
                        _ALLOCATION_CONFIGURATION_DELAY: tuple.4,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
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
    /**Function with signature `addStrategiesToOperatorSet(address,uint32,address[])` and selector `0x50feea20`.
    ```solidity
    function addStrategiesToOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`addStrategiesToOperatorSet(address,uint32,address[])`](addStrategiesToOperatorSetCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetId: tuple.1,
                        strategies: tuple.2,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategiesToOperatorSet(address,uint32,address[])";
            const SELECTOR: [u8; 4] = [80u8, 254u8, 234u8, 32u8];
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
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
    /**Function with signature `createOperatorSets(address,(uint32,address[])[])` and selector `0x261f84e0`.
    ```solidity
    function createOperatorSets(address avs, IAllocationManagerTypes.CreateSetParams[] memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`createOperatorSets(address,(uint32,address[])[])`](createOperatorSetsCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.avs, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        params: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::CreateSetParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets(address,(uint32,address[])[])";
            const SELECTOR: [u8; 4] = [38u8, 31u8, 132u8, 224u8];
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
                        IAllocationManagerTypes::CreateSetParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`encumberedMagnitude(address,address)`](encumberedMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct encumberedMagnitudeReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAVSRegistrar(address)`](getAVSRegistrarCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAVSRegistrarReturn {
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatableMagnitude(address,address)`](getAllocatableMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatableMagnitudeReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocatedSets(address)`](getAllocatedSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedSetsReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getAllocatedStrategies(address,(address,uint32))`](getAllocatedStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocatedStrategiesReturn {
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocation(address,(address,uint32),address)`](getAllocationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocationDelay(address)`](getAllocationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationDelayReturn {
        #[allow(missing_docs)]
        pub _0: bool,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getAllocations(address[],(address,uint32),address)`](getAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitude(address,address)`](getMaxMagnitudeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudeReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address[],address)`](getMaxMagnitudes_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_0Return {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudes(address,address[])`](getMaxMagnitudes_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudes_1Return {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getMaxMagnitudesAtBlock(address,address[],uint32)`](getMaxMagnitudesAtBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMemberCount((address,uint32))`](getMemberCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMemberCountReturn {
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
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getMembers((address,uint32))`](getMembersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMembersReturn {
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
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub futureBlock: u32,
    }
    ///Container type for the return parameters of the [`getMinimumSlashableStake((address,uint32),address[],address[],uint32)`](getMinimumSlashableStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeReturn {
        #[allow(missing_docs)]
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
    /**Function with signature `getOperatorSetCount(address)` and selector `0xba1a84e5`.
    ```solidity
    function getOperatorSetCount(address avs) external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCountCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorSetCount(address)`](getOperatorSetCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCountReturn {
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
            impl ::core::convert::From<getOperatorSetCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { avs: tuple.0 }
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
            impl ::core::convert::From<getOperatorSetCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorSetCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetCount(address)";
            const SELECTOR: [u8; 4] = [186u8, 26u8, 132u8, 229u8];
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
    /**Function with signature `getRegisteredSets(address)` and selector `0x79ae50cd`.
    ```solidity
    function getRegisteredSets(address operator) external view returns (OperatorSet[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getRegisteredSets(address)`](getRegisteredSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getRegisteredSetsReturn {
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getStrategiesInOperatorSet((address,uint32))`](getStrategiesInOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategiesInOperatorSetReturn {
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
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getStrategyAllocations(address,address)`](getStrategyAllocationsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStrategyAllocationsReturn {
        #[allow(missing_docs)]
        pub _0:
            alloy::sol_types::private::Vec<<OperatorSet as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
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
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
    /**Function with signature `isMemberOfOperatorSet(address,(address,uint32))` and selector `0x670d3ba2`.
    ```solidity
    function isMemberOfOperatorSet(address operator, OperatorSet memory operatorSet) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isMemberOfOperatorSet(address,(address,uint32))`](isMemberOfOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetReturn {
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
            impl ::core::convert::From<isMemberOfOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isMemberOfOperatorSetCall {
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
            impl ::core::convert::From<isMemberOfOperatorSetReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isMemberOfOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isMemberOfOperatorSetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isMemberOfOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isMemberOfOperatorSet(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [103u8, 13u8, 59u8, 162u8];
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
    /**Function with signature `isOperatorSet((address,uint32))` and selector `0x260dc758`.
    ```solidity
    function isOperatorSet(OperatorSet memory operatorSet) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetCall {
        #[allow(missing_docs)]
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isOperatorSet((address,uint32))`](isOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetReturn {
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
    /**Function with signature `modifyAllocations(address,((address,uint32),address[],uint64[])[])` and selector `0x952899ee`.
    ```solidity
    function modifyAllocations(address operator, IAllocationManagerTypes.AllocateParams[] memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params: alloy::sol_types::private::Vec<
            <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`modifyAllocations(address,((address,uint32),address[],uint64[])[])`](modifyAllocationsCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::AllocateParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyAllocationsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        params: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::AllocateParams>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyAllocationsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "modifyAllocations(address,((address,uint32),address[],uint64[])[])";
            const SELECTOR: [u8; 4] = [149u8, 40u8, 153u8, 238u8];
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
                        IAllocationManagerTypes::AllocateParams,
                    > as alloy_sol_types::SolType>::tokenize(&self.params),
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
            impl ::core::convert::From<permissionControllerCall> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerCall {
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
            impl ::core::convert::From<permissionControllerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: permissionControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for permissionControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for permissionControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = permissionControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `registerForOperatorSets(address,(address,uint32[],bytes))` and selector `0xadc2e3d9`.
    ```solidity
    function registerForOperatorSets(address operator, IAllocationManagerTypes.RegisterParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params:
            <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerForOperatorSets(address,(address,uint32[],bytes))`](registerForOperatorSetsCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::RegisterParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerForOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        params: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::RegisterParams,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerForOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "registerForOperatorSets(address,(address,uint32[],bytes))";
            const SELECTOR: [u8; 4] = [173u8, 194u8, 227u8, 217u8];
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
    /**Function with signature `removeStrategiesFromOperatorSet(address,uint32,address[])` and selector `0xb66bd989`.
    ```solidity
    function removeStrategiesFromOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorSetId: u32,
        #[allow(missing_docs)]
        pub strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(address,uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetId: tuple.1,
                        strategies: tuple.2,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "removeStrategiesFromOperatorSet(address,uint32,address[])";
            const SELECTOR: [u8; 4] = [182u8, 107u8, 217u8, 137u8];
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
    /**Function with signature `setAVSRegistrar(address,address)` and selector `0xd3d96ff4`.
    ```solidity
    function setAVSRegistrar(address avs, address registrar) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub registrar: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setAVSRegistrar(address,address)`](setAVSRegistrarCall) function.
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
            impl ::core::convert::From<setAVSRegistrarCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarCall) -> Self {
                    (value.avs, value.registrar)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAVSRegistrarCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        registrar: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setAVSRegistrar(address,address)";
            const SELECTOR: [u8; 4] = [211u8, 217u8, 111u8, 244u8];
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
    pub struct setAllocationDelayCall {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub delay: u32,
    }
    ///Container type for the return parameters of the [`setAllocationDelay(address,uint32)`](setAllocationDelayCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAllocationDelayReturn {}
    #[allow(
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
            impl ::core::convert::From<setAllocationDelayCall> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayCall) -> Self {
                    (value.operator, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelayCall {
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
            impl ::core::convert::From<setAllocationDelayReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setAllocationDelayCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelayReturn;
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
    /**Function with signature `slashOperator(address,(address,uint32,address[],uint256[],string))` and selector `0x36352057`.
    ```solidity
    function slashOperator(address avs, IAllocationManagerTypes.SlashingParams memory params) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub params:
            <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`slashOperator(address,(address,uint32,address[],uint256[],string))`](slashOperatorCall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::SlashingParams,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (value.avs, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        params: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::SlashingParams,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "slashOperator(address,(address,uint32,address[],uint256[],string))";
            const SELECTOR: [u8; 4] = [54u8, 53u8, 32u8, 87u8];
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
    /**Function with signature `updateAVSMetadataURI(address,string)` and selector `0xa9821821`.
    ```solidity
    function updateAVSMetadataURI(address avs, string memory metadataURI) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(address,string)`](updateAVSMetadataURICall) function.
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<updateAVSMetadataURICall> for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value.avs, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        metadataURI: tuple.1,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::String,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(address,string)";
            const SELECTOR: [u8; 4] = [169u8, 130u8, 24u8, 33u8];
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
        #[allow(missing_docs)]
        ALLOCATION_CONFIGURATION_DELAY(ALLOCATION_CONFIGURATION_DELAYCall),
        #[allow(missing_docs)]
        DEALLOCATION_DELAY(DEALLOCATION_DELAYCall),
        #[allow(missing_docs)]
        addStrategiesToOperatorSet(addStrategiesToOperatorSetCall),
        #[allow(missing_docs)]
        clearDeallocationQueue(clearDeallocationQueueCall),
        #[allow(missing_docs)]
        createOperatorSets(createOperatorSetsCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        deregisterFromOperatorSets(deregisterFromOperatorSetsCall),
        #[allow(missing_docs)]
        encumberedMagnitude(encumberedMagnitudeCall),
        #[allow(missing_docs)]
        getAVSRegistrar(getAVSRegistrarCall),
        #[allow(missing_docs)]
        getAllocatableMagnitude(getAllocatableMagnitudeCall),
        #[allow(missing_docs)]
        getAllocatedSets(getAllocatedSetsCall),
        #[allow(missing_docs)]
        getAllocatedStrategies(getAllocatedStrategiesCall),
        #[allow(missing_docs)]
        getAllocation(getAllocationCall),
        #[allow(missing_docs)]
        getAllocationDelay(getAllocationDelayCall),
        #[allow(missing_docs)]
        getAllocations(getAllocationsCall),
        #[allow(missing_docs)]
        getMaxMagnitude(getMaxMagnitudeCall),
        #[allow(missing_docs)]
        getMaxMagnitudes_0(getMaxMagnitudes_0Call),
        #[allow(missing_docs)]
        getMaxMagnitudes_1(getMaxMagnitudes_1Call),
        #[allow(missing_docs)]
        getMaxMagnitudesAtBlock(getMaxMagnitudesAtBlockCall),
        #[allow(missing_docs)]
        getMemberCount(getMemberCountCall),
        #[allow(missing_docs)]
        getMembers(getMembersCall),
        #[allow(missing_docs)]
        getMinimumSlashableStake(getMinimumSlashableStakeCall),
        #[allow(missing_docs)]
        getOperatorSetCount(getOperatorSetCountCall),
        #[allow(missing_docs)]
        getRegisteredSets(getRegisteredSetsCall),
        #[allow(missing_docs)]
        getStrategiesInOperatorSet(getStrategiesInOperatorSetCall),
        #[allow(missing_docs)]
        getStrategyAllocations(getStrategyAllocationsCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        isMemberOfOperatorSet(isMemberOfOperatorSetCall),
        #[allow(missing_docs)]
        isOperatorSet(isOperatorSetCall),
        #[allow(missing_docs)]
        modifyAllocations(modifyAllocationsCall),
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
        permissionController(permissionControllerCall),
        #[allow(missing_docs)]
        registerForOperatorSets(registerForOperatorSetsCall),
        #[allow(missing_docs)]
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        setAVSRegistrar(setAVSRegistrarCall),
        #[allow(missing_docs)]
        setAllocationDelay(setAllocationDelayCall),
        #[allow(missing_docs)]
        slashOperator(slashOperatorCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
        #[allow(missing_docs)]
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
            [16u8, 225u8, 185u8, 184u8],
            [19u8, 100u8, 57u8, 221u8],
            [21u8, 254u8, 80u8, 40u8],
            [38u8, 13u8, 199u8, 88u8],
            [38u8, 31u8, 132u8, 224u8],
            [41u8, 129u8, 235u8, 119u8],
            [43u8, 171u8, 44u8, 74u8],
            [48u8, 76u8, 16u8, 205u8],
            [54u8, 53u8, 32u8, 87u8],
            [64u8, 18u8, 13u8, 171u8],
            [65u8, 119u8, 168u8, 124u8],
            [70u8, 87u8, 226u8, 106u8],
            [74u8, 16u8, 255u8, 229u8],
            [75u8, 80u8, 70u8, 239u8],
            [80u8, 254u8, 234u8, 32u8],
            [84u8, 122u8, 251u8, 135u8],
            [86u8, 196u8, 131u8, 230u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [103u8, 13u8, 59u8, 162u8],
            [108u8, 251u8, 68u8, 129u8],
            [110u8, 52u8, 146u8, 181u8],
            [110u8, 135u8, 93u8, 186u8],
            [113u8, 80u8, 24u8, 166u8],
            [121u8, 174u8, 80u8, 205u8],
            [123u8, 193u8, 239u8, 97u8],
            [136u8, 111u8, 17u8, 149u8],
            [140u8, 230u8, 72u8, 84u8],
            [141u8, 165u8, 203u8, 91u8],
            [148u8, 215u8, 208u8, 12u8],
            [149u8, 40u8, 153u8, 238u8],
            [169u8, 51u8, 62u8, 200u8],
            [169u8, 130u8, 24u8, 33u8],
            [169u8, 132u8, 235u8, 58u8],
            [173u8, 194u8, 227u8, 217u8],
            [178u8, 68u8, 122u8, 247u8],
            [182u8, 107u8, 217u8, 137u8],
            [185u8, 251u8, 174u8, 209u8],
            [186u8, 26u8, 132u8, 229u8],
            [194u8, 33u8, 216u8, 174u8],
            [205u8, 109u8, 198u8, 135u8],
            [211u8, 217u8, 111u8, 244u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerCalls {
        const NAME: &'static str = "AllocationManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 46usize;
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
                Self::getOperatorSetCount(_) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::isMemberOfOperatorSet(_) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::permissionController(_) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setAllocationDelay(_) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::SELECTOR
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
                -> alloy_sol_types::Result<AllocationManagerCalls>] = &[
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
                    fn permissionController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <permissionControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::permissionController)
                    }
                    permissionController
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
                    fn setAllocationDelay(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::setAllocationDelay)
                    }
                    setAllocationDelay
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
                    fn isMemberOfOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::isMemberOfOperatorSet)
                    }
                    isMemberOfOperatorSet
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
                    fn getOperatorSetCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerCalls::getOperatorSetCount)
                    }
                    getOperatorSetCount
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
            DECODE_SHIMS[idx](data, validate)
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
                Self::getOperatorSetCount(inner) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isMemberOfOperatorSet(inner) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setAllocationDelay(inner) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getOperatorSetCount(inner) => {
                    <getOperatorSetCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isMemberOfOperatorSet(inner) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::permissionController(inner) => {
                    <permissionControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setAllocationDelay(inner) => {
                    <setAllocationDelayCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
        #[allow(missing_docs)]
        AlreadyMemberOfSet(AlreadyMemberOfSet),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        Empty(Empty),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InsufficientMagnitude(InsufficientMagnitude),
        #[allow(missing_docs)]
        InvalidAVSRegistrar(InvalidAVSRegistrar),
        #[allow(missing_docs)]
        InvalidAVSWithNoMetadataRegistered(InvalidAVSWithNoMetadataRegistered),
        #[allow(missing_docs)]
        InvalidCaller(InvalidCaller),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidOperator(InvalidOperator),
        #[allow(missing_docs)]
        InvalidOperatorSet(InvalidOperatorSet),
        #[allow(missing_docs)]
        InvalidPermissions(InvalidPermissions),
        #[allow(missing_docs)]
        InvalidSnapshotOrdering(InvalidSnapshotOrdering),
        #[allow(missing_docs)]
        InvalidWadToSlash(InvalidWadToSlash),
        #[allow(missing_docs)]
        ModificationAlreadyPending(ModificationAlreadyPending),
        #[allow(missing_docs)]
        NotMemberOfSet(NotMemberOfSet),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotSlashable(OperatorNotSlashable),
        #[allow(missing_docs)]
        OutOfBounds(OutOfBounds),
        #[allow(missing_docs)]
        SameMagnitude(SameMagnitude),
        #[allow(missing_docs)]
        StrategiesMustBeInAscendingOrder(StrategiesMustBeInAscendingOrder),
        #[allow(missing_docs)]
        StrategyAlreadyInOperatorSet(StrategyAlreadyInOperatorSet),
        #[allow(missing_docs)]
        StrategyNotInOperatorSet(StrategyNotInOperatorSet),
        #[allow(missing_docs)]
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
            [19u8, 83u8, 96u8, 49u8],
            [37u8, 19u8, 29u8, 79u8],
            [42u8, 55u8, 28u8, 126u8],
            [61u8, 178u8, 161u8, 42u8],
            [67u8, 113u8, 74u8, 253u8],
            [72u8, 245u8, 195u8, 237u8],
            [88u8, 92u8, 251u8, 47u8],
            [99u8, 120u8, 104u8, 78u8],
            [108u8, 155u8, 224u8, 191u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [140u8, 12u8, 47u8, 38u8],
            [147u8, 45u8, 148u8, 247u8],
            [159u8, 28u8, 128u8, 83u8],
            [180u8, 18u8, 15u8, 20u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 249u8, 117u8, 97u8],
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 216u8, 220u8, 78u8],
            [216u8, 252u8, 190u8, 48u8],
            [232u8, 88u8, 158u8, 8u8],
            [235u8, 191u8, 244u8, 151u8],
            [250u8, 85u8, 252u8, 129u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AllocationManagerErrors {
        const NAME: &'static str = "AllocationManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
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
                Self::InvalidAVSRegistrar(_) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAVSWithNoMetadataRegistered(_) => {
                    <InvalidAVSWithNoMetadataRegistered as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidPermissions(_) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidSnapshotOrdering(_) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidWadToSlash(_) => {
                    <InvalidWadToSlash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ModificationAlreadyPending(_) => {
                    <ModificationAlreadyPending as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NotMemberOfSet(_) => <NotMemberOfSet as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotSlashable(_) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => <OutOfBounds as alloy_sol_types::SolError>::SELECTOR,
                Self::SameMagnitude(_) => <SameMagnitude as alloy_sol_types::SolError>::SELECTOR,
                Self::StrategiesMustBeInAscendingOrder(_) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::SELECTOR
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
                -> alloy_sol_types::Result<AllocationManagerErrors>] = &[
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
                    fn InvalidSnapshotOrdering(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InvalidSnapshotOrdering)
                    }
                    InvalidSnapshotOrdering
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
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InvalidPermissions)
                    }
                    InvalidPermissions
                },
                {
                    fn StrategiesMustBeInAscendingOrder(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerErrors::StrategiesMustBeInAscendingOrder,
                            )
                    }
                    StrategiesMustBeInAscendingOrder
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
                    fn InvalidAVSWithNoMetadataRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidAVSWithNoMetadataRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AllocationManagerErrors::InvalidAVSWithNoMetadataRegistered,
                            )
                    }
                    InvalidAVSWithNoMetadataRegistered
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
                    fn InvalidAVSRegistrar(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AllocationManagerErrors::InvalidAVSRegistrar)
                    }
                    InvalidAVSRegistrar
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
            DECODE_SHIMS[idx](data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAVSRegistrar(inner) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidAVSWithNoMetadataRegistered(inner) => {
                    <InvalidAVSWithNoMetadataRegistered as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OutOfBounds(inner) => {
                    <OutOfBounds as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StrategiesMustBeInAscendingOrder(inner) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::AlreadyMemberOfSet(inner) => {
                    <AlreadyMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InsufficientMagnitude(inner) => {
                    <InsufficientMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAVSRegistrar(inner) => {
                    <InvalidAVSRegistrar as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidAVSWithNoMetadataRegistered(inner) => {
                    <InvalidAVSWithNoMetadataRegistered as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidPermissions(inner) => {
                    <InvalidPermissions as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidSnapshotOrdering(inner) => {
                    <InvalidSnapshotOrdering as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::NotMemberOfSet(inner) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorNotSlashable(inner) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::SameMagnitude(inner) => {
                    <SameMagnitude as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategiesMustBeInAscendingOrder(inner) => {
                    <StrategiesMustBeInAscendingOrder as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategyAlreadyInOperatorSet(inner) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::StrategyNotInOperatorSet(inner) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_encode_raw(
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
        #[allow(missing_docs)]
        AVSMetadataURIUpdated(AVSMetadataURIUpdated),
        #[allow(missing_docs)]
        AVSRegistrarSet(AVSRegistrarSet),
        #[allow(missing_docs)]
        AllocationDelaySet(AllocationDelaySet),
        #[allow(missing_docs)]
        AllocationUpdated(AllocationUpdated),
        #[allow(missing_docs)]
        EncumberedMagnitudeUpdated(EncumberedMagnitudeUpdated),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        MaxMagnitudeUpdated(MaxMagnitudeUpdated),
        #[allow(missing_docs)]
        OperatorAddedToOperatorSet(OperatorAddedToOperatorSet),
        #[allow(missing_docs)]
        OperatorRemovedFromOperatorSet(OperatorRemovedFromOperatorSet),
        #[allow(missing_docs)]
        OperatorSetCreated(OperatorSetCreated),
        #[allow(missing_docs)]
        OperatorSlashed(OperatorSlashed),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        StrategyAddedToOperatorSet(StrategyAddedToOperatorSet),
        #[allow(missing_docs)]
        StrategyRemovedFromOperatorSet(StrategyRemovedFromOperatorSet),
        #[allow(missing_docs)]
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
        _permissionController: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<AllocationManagerInstance<T, P, N>>>
    {
        AllocationManagerInstance::<T, P, N>::deploy(
            provider,
            _delegation,
            _pauserRegistry,
            _permissionController,
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
        _permissionController: alloy::sol_types::private::Address,
        _DEALLOCATION_DELAY: u32,
        _ALLOCATION_CONFIGURATION_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AllocationManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _delegation,
            _pauserRegistry,
            _permissionController,
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
            _permissionController: alloy::sol_types::private::Address,
            _DEALLOCATION_DELAY: u32,
            _ALLOCATION_CONFIGURATION_DELAY: u32,
        ) -> alloy_contract::Result<AllocationManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegation,
                _pauserRegistry,
                _permissionController,
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
            _permissionController: alloy::sol_types::private::Address,
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
                        _permissionController,
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
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesToOperatorSetCall, N> {
            self.call_builder(&addStrategiesToOperatorSetCall {
                avs,
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
            avs: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(&createOperatorSetsCall { avs, params })
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
        ///Creates a new call builder for the [`getOperatorSetCount`] function.
        pub fn getOperatorSetCount(
            &self,
            avs: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetCountCall, N> {
            self.call_builder(&getOperatorSetCountCall { avs })
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
        ///Creates a new call builder for the [`isMemberOfOperatorSet`] function.
        pub fn isMemberOfOperatorSet(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isMemberOfOperatorSetCall, N> {
            self.call_builder(&isMemberOfOperatorSetCall {
                operator,
                operatorSet,
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
            operator: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::AllocateParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyAllocationsCall, N> {
            self.call_builder(&modifyAllocationsCall { operator, params })
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
        ///Creates a new call builder for the [`permissionController`] function.
        pub fn permissionController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, permissionControllerCall, N> {
            self.call_builder(&permissionControllerCall {})
        }
        ///Creates a new call builder for the [`registerForOperatorSets`] function.
        pub fn registerForOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerForOperatorSetsCall, N> {
            self.call_builder(&registerForOperatorSetsCall { operator, params })
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeStrategiesFromOperatorSetCall, N> {
            self.call_builder(&removeStrategiesFromOperatorSetCall {
                avs,
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
            avs: alloy::sol_types::private::Address,
            registrar: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAVSRegistrarCall, N> {
            self.call_builder(&setAVSRegistrarCall { avs, registrar })
        }
        ///Creates a new call builder for the [`setAllocationDelay`] function.
        pub fn setAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
            delay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllocationDelayCall, N> {
            self.call_builder(&setAllocationDelayCall { operator, delay })
        }
        ///Creates a new call builder for the [`slashOperator`] function.
        pub fn slashOperator(
            &self,
            avs: alloy::sol_types::private::Address,
            params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashOperatorCall, N> {
            self.call_builder(&slashOperatorCall { avs, params })
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
            avs: alloy::sol_types::private::Address,
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(&updateAVSMetadataURICall { avs, metadataURI })
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
