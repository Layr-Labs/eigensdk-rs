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
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct AllocateParams { OperatorSet operatorSet; address[] strategies; uint64[] newMagnitudes; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AllocateParams {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for AllocateParams {
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
        impl alloy_sol_types::SolStruct for AllocateParams {
            const NAME: &'static str = "AllocateParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "AllocateParams(OperatorSet operatorSet,address[] strategies,uint64[] newMagnitudes)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.currentMagnitude),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.pendingDiff),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
        impl alloy_sol_types::SolType for Allocation {
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
        impl alloy_sol_types::SolStruct for Allocation {
            const NAME: &'static str = "Allocation";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Allocation(uint64 currentMagnitude,int128 pendingDiff,uint32 effectBlock)",
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
                    &rust.effectBlock,
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
struct CreateSetParams { uint32 operatorSetId; address[] strategies; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CreateSetParams {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
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
        impl alloy_sol_types::SolType for CreateSetParams {
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
        impl alloy_sol_types::SolStruct for CreateSetParams {
            const NAME: &'static str = "CreateSetParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CreateSetParams(uint32 operatorSetId,address[] strategies)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for DeregisterParams {
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
        impl alloy_sol_types::SolStruct for DeregisterParams {
            const NAME: &'static str = "DeregisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DeregisterParams(address operator,address avs,uint32[] operatorSetIds)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for RegisterParams {
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
        impl alloy_sol_types::SolStruct for RegisterParams {
            const NAME: &'static str = "RegisterParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "RegisterParams(address avs,uint32[] operatorSetIds,bytes data)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.data,
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
struct SlashingParams { address operator; uint32 operatorSetId; address[] strategies; uint256[] wadsToSlash; string description; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SlashingParams {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub wadsToSlash: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
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
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
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
                    "SlashingParams(address operator,uint32 operatorSetId,address[] strategies,uint256[] wadsToSlash,string description)",
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
    ///0x610120346101ed57601f61520e38819003918201601f19168301916001600160401b038311848410176101f15780849260a0946040528339810103126101ed5780516001600160a01b03811681036101ed576020820151916001600160a01b038316918284036101ed576040820151936001600160a01b03851685036101ed57610097608061009060608601610205565b9401610205565b93156101de5760805260a05260c05260e052610100525f5460ff8160081c166101895760ff8082161061014f575b604051614ff79081610217823960805181818161052301528181611ba6015281816122f10152612f11015260a051818181610b6401528181610f0101528181611ac6015281816128870152612e28015260c0518181816109a501528181611e6b0152613935015260e0518181816122b801526146da0152610100518181816116ce0152613ffa0152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f6100c5565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b6339b190bb60e11b5f5260045ffd5b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b519063ffffffff821682036101ed5756fe60806040526004361015610011575f80fd5b5f3560e01c806310e1b9b8146102f4578063136439dd146102ef57806315fe5028146102ea578063260dc758146102e5578063261f84e0146102e05780632981eb77146102db5780632bab2c4a146102d6578063304c10cd146102d157806336352057146102cc57806340120dab146102c75780634177a87c146102c25780634657e26a146102bd5780634a10ffe5146102b85780634b5046ef146102b357806350feea20146102ae578063547afb87146102a957806356c483e6146102a4578063595c6a671461029f5780635ac86ab71461029a5780635c975abb14610295578063670d3ba2146102905780636cfb44811461028b5780636e3492b5146102865780636e875dba14610281578063715018a61461027c57806379ae50cd146102775780637bc1ef6114610272578063886f11951461026d5780638ce64854146102685780638da5cb5b1461026357806394d7d00c1461025e578063952899ee14610259578063a9333ec814610254578063a98218211461024f578063a984eb3a1461024a578063adc2e3d914610245578063b2447af714610240578063b66bd9891461023b578063b9fbaed114610236578063ba1a84e514610231578063c221d8ae1461022c578063cd6dc68714610227578063d3d96ff414610222578063df5cf7231461021d578063f2fde38b146102185763fabc1cbc14610213575f80fd5b612ee8565b612e57565b612e13565b612d6f565b612c81565b612bf0565b612bb5565b612b78565b612aa0565b612a6d565b612805565b6127b8565b612751565b612702565b612584565b61240e565b6123e6565b612331565b6122dc565b61229c565b612210565b6121b5565b61213c565b611df7565b611c95565b611c3e565b611c21565b611bee565b611b7b565b611a9b565b611a0d565b6118ec565b6117dc565b61173f565b6116b9565b61163b565b611541565b610db1565b610d78565b610ad1565b610989565b610744565b6106b8565b61061c565b6104f3565b61046c565b6001600160a01b0381160361030a57565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761033d57604052565b61030e565b606081019081106001600160401b0382111761033d57604052565b608081019081106001600160401b0382111761033d57604052565b90601f801991011681019081106001600160401b0382111761033d57604052565b604051906103a8604083610378565b565b63ffffffff81160361030a57565b604090602319011261030a57604051906103d182610322565b816024356103de816102f9565b81526020604435916103ef836103aa565b0152565b604090600319011261030a576040519061040c82610322565b81600435610419816102f9565b81526020602435916103ef836103aa565b919082604091031261030a5760405161044281610322565b60208082948035610452816102f9565b84520135916103ef836103aa565b6001600160401b031690565b3461030a57608036600319011261030a5760606104bc60043561048e816102f9565b610497366103b8565b906104b6606435926104a8846102f9565b6104b0612fac565b50613d7e565b90613de2565b90506104f1604051809263ffffffff604080926001600160401b0381511685526020810151600f0b6020860152015116910152565bf35b3461030a57602036600319011261030a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156105ae5761057d92610569915f9161057f575b50613002565b61057860665482811614613018565b613f55565b005b6105a1915060203d6020116105a7575b6105998183610378565b810190612fdf565b5f610563565b503d61058f565b612ff7565b90602080835192838152019201905f5b8181106105d05750505090565b90919260206040826105fd600194885163ffffffff6020809260018060a01b038151168552015116910152565b0194019291016105c3565b9060206106199281815201906105b3565b90565b3461030a57602036600319011261030a57600435610639816102f9565b6001600160a01b03165f818152609d602052604090205461065981613046565b915f5b82811061067557604051806106718682610608565b0390f35b600190825f52609d60205261069c6106908260405f20614a6c565b90549060031b1c613f87565b6106a682876130a9565b526106b181866130a9565b500161065c565b3461030a57604036600319011261030a57602061070a6106d7366103f3565b80516001600160a01b03165f9081526098845260408082209285015163ffffffff16825260019092016020522054151590565b6040519015158152f35b9181601f8401121561030a578235916001600160401b03831161030a576020808501948460051b01011161030a57565b3461030a57604036600319011261030a57600435610761816102f9565b6024356001600160401b03811161030a57610780903690600401610714565b9061079261078d84613fb1565b6130c2565b5f915b80831061079e57005b6107b16107ac8483856130d8565b6130fa565b907f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c61087a6108496107e1610399565b6001600160a01b03891681529463ffffffff16602086019081526001600160a01b0389165f90815260986020526040902061083f9061083a9061083461082b855163ffffffff1690565b63ffffffff1690565b90614a9e565b613104565b5163ffffffff1690565b61086e610854610399565b6001600160a01b038a1681529163ffffffff166020830152565b6040519182918261311a565b0390a161088682613d7e565b925f5b6108a16108978785856130d8565b6020810190613140565b905081101561096f57806108f8876108f26108e66108e16001966108db6108976108d38e5f52609960205260405f2090565b968c8c6130d8565b90613175565b613185565b6001600160a01b031690565b90614046565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b6109666109326108e1846108db6108978d8b8b6130d8565b6040805189516001600160a01b0390811682526020808c015163ffffffff1690830152909216908201529081906060820190565b0390a101610889565b5060019094019392509050610795565b5f91031261030a57565b3461030a575f36600319011261030a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b6001600160401b03811161033d5760051b60200190565b9080601f8301121561030a5781356109f7816109c9565b92610a056040519485610378565b81845260208085019260051b82010192831161030a57602001905b828210610a2d5750505090565b602080918335610a3c816102f9565b815201910190610a20565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310610a7957505050505090565b9091929394603f19828203018352855190602080835192838152019201905f905b808210610ab95750505060208060019297019301930191939290610a6a565b90919260208060019286518152019401920190610a9a565b3461030a5760a036600319011261030a57610aeb366103f3565b6044356001600160401b03811161030a57610b0a9036906004016109e0565b906064356001600160401b03811161030a57610b2a9036906004016109e0565b90608435610b37816103aa565b610b41845161318f565b90604051637870733b60e11b81525f8180610b60888a600484016132c6565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156105ae575f91610d56575b5063ffffffff91909116905f670de0b6b3a7640000945b8751821015610d4857610bd6610bc9838a6130a9565b516001600160a01b031690565b92610be188516132eb565b610beb84886130a9565b52610bf683876130a9565b505f5b888051821015610d3a57908386610c15610bc9846001966130a9565b8b6001600160401b03610c59610c5484610c3f8760018060a01b03165f5260a160205260405f2090565b9060018060a01b03165f5260205260405f2090565b614059565b16918215610d3057610460610c78610caa92610cc997610caf97612fca565b8d63ffffffff610c8f604084015163ffffffff1690565b16111580610d17575b610cdf575b516001600160401b031690565b614c46565b610cc383610cbd89886130a9565b516130a9565b51614bcc565b610cd782610cbd888c6130a9565b525b01610bf9565b610d12610d05610cf683516001600160401b031690565b6020840151600f0b5b90614095565b6001600160401b03168252565b610c9d565b505f610d276020830151600f0b90565b600f0b12610c98565b5050505050610cd9565b505092509060010190610bb3565b604051806106718782610a47565b610d7291503d805f833e610d6a8183610378565b8101906131d8565b5f610b9c565b3461030a57602036600319011261030a576020610d9f600435610d9a816102f9565b61331d565b6040516001600160a01b039091168152f35b3461030a57604036600319011261030a57600435610dce816102f9565b6024356001600160401b03811161030a5760a0600319823603011261030a57610e04610dfe600280606654161490565b15613342565b610e1061078d83613fb1565b610ee5610e1f602483016130fa565b610e39610e2a610399565b6001600160a01b039095168552565b63ffffffff1660208401908152610ee061083a610e6186610e5c87600401613185565b6140b0565b92610e90610e756044880188600401613140565b9050610e876064890189600401613140565b91905014613358565b8651610ecc9061082b90610ec1906001600160a01b03165b6001600160a01b03165f90815260986020526040902090565b925163ffffffff1690565b906001915f520160205260405f2054151590565b61336e565b610eff610ef86044830183600401613140565b90506132eb565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691905f5b610f3d6044840184600401613140565b905081101561142957801580156113c7575b610f58906133a6565b610f6c816108db6064860186600401613140565b3515158061139c575b610f7e906133bc565b610fdc610fd7610f9e610f9088613d7e565b5f52609960205260405f2090565b610fb86108e66108e1866108db60448b018b600401613140565b6001600160a01b03165f90815260019091016020526040902054151590565b6133d2565b611011610feb84600401613185565b610ff487613d7e565b61100b6108e1856108db60448a018a600401613140565b91613de2565b919061102761046084516001600160401b031690565b8015611391576110706104607f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd92611069866108db60648c018c600401613140565b359061411d565b9361112261108584516001600160401b031690565b956110a26001600160401b0388166001600160401b038316614b2c565b6110ac878a6130a9565b526110d76110ca826110c586516001600160401b031690565b6133e8565b6001600160401b03168452565b6110fc6110ef826110c588516001600160401b031690565b6001600160401b03168652565b61111560208601916110c583516001600160401b031690565b6001600160401b03169052565b866111316020830151600f0b90565b855f61113d83600f0b90565b126112b9575b8391508b836111796108e189946108db61116b61116561117f9a600401613185565b96613d7e565b946044810190600401613140565b91614149565b61118b87600401613185565b906111cd6111b76111a96108e1886108db60448e0160048f01613140565b92516001600160401b031690565b926040519384938d63ffffffff43169386613445565b0390a16112076111df86600401613185565b6111f66108e1856108db60448b018b600401613140565b83516001600160401b0316916143b2565b61121385600401613185565b9061123c61122e6108e1856108db60448b018b600401613140565b91516001600160401b031690565b91873b1561030a5760405163601bb36f60e01b81526001600160a01b039182166004820152911660248201526001600160401b039384166044820152921660648301525f8260848183895af19182156105ae5760019261129f575b505b01610f2d565b806112ad5f6112b393610378565b8061097f565b5f611297565b6112fc6104606104606113229561106961130b956108db6112ee6112e26112e26113189b613408565b6001600160801b031690565b936064810190600401613140565b6001600160801b0316600f0b90565b6020840151600f0b61341f565b600f0b6020830152565b81898861138661134c6108e1896108db61133e86600401613185565b956044810190600401613140565b61136d61136087516001600160401b031690565b6020880151600f0b610cff565b604087015163ffffffff165b9160405195869586613445565b0390a1865f85611143565b505060019150611299565b50610f7e670de0b6b3a76400006113bd836108db6064880188600401613140565b3511159050610f75565b50610f586113e86108e66108e66108e1856108db60448a018a600401613140565b6114166108e66108e66108e66108e161140760448b018b600401613140565b6114108a613398565b91613175565b6001600160a01b03909116119050610f4f565b7f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5858361148f6114808761145f81600401613185565b936114706044830183600401613140565b9390926084810190600401613496565b939092604051978897886134e8565b0390a1005b604090600319011261030a576004356114ac816102f9565b90602435610619816102f9565b90602080835192838152019201905f5b8181106114d65750505090565b9091926020606082611511600194885163ffffffff604080926001600160401b0381511685526020810151600f0b6020860152015116910152565b0194019291016114c9565b9091611533610619936040845260408401906105b3565b9160208184039101526114b9565b3461030a5761154f36611494565b6001600160a01b0382165f818152609d602052604090205490929161157382613046565b9261157d836135b3565b945f5b848110611596576040518061067189898361151c565b600190825f52609d6020526115d2856115b56106908460405f20614a6c565b806115c0858c6130a9565b526115cb848b6130a9565b5086612fca565b6115dc828a6130a9565b526115e781896130a9565b5001611580565b90602080835192838152019201905f5b81811061160b5750505090565b82516001600160a01b03168452602093840193909201916001016115fe565b9060206106199281815201906115ee565b3461030a57604036600319011261030a5761165d611658366103f3565b613d7e565b5f52609960205260405f206040519081602082549182815201915f5260205f20905f5b8181106116a3576106718561169781870382610378565b6040519182918261162a565b8254845260209093019260019283019201611680565b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b60206040818301928281528451809452019201905f5b8181106117205750505090565b82516001600160401b0316845260209384019390920191600101611713565b3461030a57604036600319011261030a576004356001600160401b03811161030a5761176f9036906004016109e0565b6024359061177c826102f9565b61178681516132eb565b915f5b82518110156117ce576001906117b2836001600160a01b036117ab84886130a9565b5116613b07565b6001600160401b036117c483886130a9565b9116905201611789565b6040518061067186826116fd565b3461030a57606036600319011261030a576004356117f9816102f9565b6024356001600160401b03811161030a57611818903690600401610714565b90916044356001600160401b03811161030a57611839903690600401610714565b92909361184d610dfe600180606654161490565b611858848314613358565b5f5b82811061186357005b61186e818484613175565b3590611879826102f9565b611884818789613175565b359161ffff8316830361030a5760019261189e9187614562565b0161185a565b606060031982011261030a576004356118bc816102f9565b916024356118c9816103aa565b91604435906001600160401b03821161030a576118e891600401610714565b9091565b3461030a576118fa366118a4565b9161190961078d859395613fb1565b61196261083a6040519261191c84610322565b6001600160a01b038516845263ffffffff1660208401908152610ecc61082b610ec161194787613d7e565b6001600160a01b039098165f90815260986020526040902090565b5f5b83811061196d57005b60019061199e611999611988865f52609960205260405f2090565b6108f26108e66108e1868b8d613175565b613602565b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b611a046119d06108e184898b613175565b6040805187516001600160a01b0390811682526020808a015163ffffffff1690830152909216908201529081906060820190565b0390a101611964565b3461030a57604036600319011261030a57600435611a2a816102f9565b6024356001600160401b03811161030a57611a499036906004016109e0565b611a5381516132eb565b915f5b82518110156117ce57600190611a7f6001600160a01b03611a7783876130a9565b511684613b07565b6001600160401b03611a9183886130a9565b9116905201611a56565b3461030a57604036600319011261030a57600435611ab8816102f9565b602435611ac4816103aa565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169133839003611b02575b61057d925061464c565b6020602493611b18611b1384613fb1565b613618565b6040516336b87bd760e11b81526001600160a01b038416600482015294859182905afa9283156105ae5761057d93611b57915f91611b5c575b5061362e565b611af8565b611b75915060203d6020116105a7576105998183610378565b5f611b51565b3461030a575f36600319011261030a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156105ae57611be6915f9161057f5750613002565b61057d613f21565b3461030a57602036600319011261030a5760043560ff811680910361030a5760016020911b806066541614604051908152f35b3461030a575f36600319011261030a576020606654604051908152f35b3461030a57606036600319011261030a57602061070a600435611c60816102f9565b611c6c611658366103b8565b5f908152609a845260408082206001600160a01b03909316825260019092016020522054151590565b3461030a57611ca336611494565b611cd1611cc482610c3f8560018060a01b03165f5260a260205260405f2090565b546001600160401b031690565b90611d03611cf382610c3f8660018060a01b03165f5260a360205260405f2090565b5480600f0b9060801d600f0b0390565b5f905b808210611d50575b610671611d36856110c5610c5487610c3f8b60018060a01b03165f5260a160205260405f2090565b6040516001600160401b0390911681529081906020820190565b9092611dad611da884610c3f611d8388611d7e84610c3f8d60018060a01b03165f5260a360205260405f2090565b614820565b6001600160a01b038a165f90815260a0602052604090205b905f5260205260405f2090565b613644565b611dc161082b604083015163ffffffff1690565b4310611de25760200151600191611dda91600f0b610cff565b930190611d06565b5092611d0e565b9081606091031261030a5790565b3461030a57602036600319011261030a576004356001600160401b03811161030a57611e27903690600401611de9565b611e38610dfe600480606654161490565b611e49611e4482613185565b613fb1565b80156120d5575b611e5990613618565b6020810190604081014363ffffffff167f00000000000000000000000000000000000000000000000000000000000000005f5b611e968486613140565b905081101561206c578061206686611f0361083a8a610ecc61082b610ec1610ea8611ed46107ac8f9c6108db60019e611ece8a613185565b9c613140565b94611eef611ee0610399565b6001600160a01b03909a168a52565b6108e160208a0196879063ffffffff169052565b611f52611f4d611f46611f31611f188c613185565b6001600160a01b03165f908152609e6020526040902090565b611f3a85613d7e565b5f5260205260405f2090565b5460ff1690565b61367b565b611f89611f7a611f618a613185565b6001600160a01b03165f908152609c6020526040902090565b611f8383613d7e565b90614dec565b50611fb6611fa7611f9983613d7e565b5f52609a60205260405f2090565b611fb08a613185565b906148de565b50611fc36108e689613185565b7fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe60405180611ff2858261311a565b0390a261203561200286886136a9565b9161202061200e610399565b5f815263ffffffff9094166020850152565b611f3a61202f611f188c613185565b91613d7e565b906020908051151560ff80198554169116178355015164ffffffff0082549160081b169064ffffffff001916179055565b01611e8c565b83856120906120806108e6610d9a8b613185565b9261208a83613185565b92613140565b9092803b1561030a576120bd935f809460405196879586948593639d8e0c2360e01b855260048501613703565b03925af16120c757005b806112ad5f61057d93610378565b50611e596120e8611e4460208401613185565b9050611e50565b90602080835192838152019201905f5b81811061210c5750505090565b82516001600160a01b03168452602093840193909201916001016120ff565b9060206106199281815201906120ef565b3461030a57604036600319011261030a57612159611658366103f3565b5f52609a60205260405f206040519081602082549182815201915f5260205f20905f5b81811061219f576106718561219381870382610378565b6040519182918261212b565b825484526020909301926001928301920161217c565b3461030a575f36600319011261030a576121cd6148f1565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461030a57602036600319011261030a5760043561222d816102f9565b6001600160a01b03165f818152609c602052604090205461224d81613046565b915f5b82811061226557604051806106718682610608565b600190825f52609c6020526122806106908260405f20614a6c565b61228a82876130a9565b5261229581866130a9565b5001612250565b3461030a575f36600319011261030a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9060206106199281815201906114b9565b3461030a57608036600319011261030a576004356001600160401b03811161030a576123619036906004016109e0565b61236a366103b8565b90606435612377816102f9565b61238182516135b3565b925f5b83518110156123d8576001906123ba846001600160a01b036123a684896130a9565b51166123b0612fac565b506104b686613d7e565b90506123c682886130a9565b526123d181876130a9565b5001612384565b604051806106718782612320565b3461030a575f36600319011261030a576033546040516001600160a01b039091168152602090f35b3461030a57606036600319011261030a5760043561242b816102f9565b6024356001600160401b03811161030a5761244a9036906004016109e0565b90604435612457816103aa565b61246183516132eb565b925f926001600160a01b03169163ffffffff16905b8051841015612576575f83815260a1602052604090206124b8906001600160a01b036124a287856130a9565b511660018060a01b03165f5260205260405f2090565b938454945f955b80871061252e576001939495965080155f1461250657506124fd9050670de0b6b3a76400005b6124ef83896130a9565b906001600160401b03169052565b01929190612476565b6104606125276124fd9361251c6124e594613398565b905f5260205f200190565b5460201c90565b8087169080881860011c820180921161257157825f528563ffffffff8360205f20015416115f146125625750955b956124bf565b9650600181018091111561255c575b613384565b6040518061067187826116fd565b3461030a57604036600319011261030a576004356125a1816102f9565b6024356001600160401b03811161030a573660238201121561030a578060040135916125cc836109c9565b916125da6040519384610378565b8383526024602084019460051b8201019036821161030a5760248101945b8286106126095761057d8585613725565b85356001600160401b03811161030a5782016080602319823603011261030a576040519061263682610342565b612643366024830161042a565b825260648101356001600160401b03811161030a5761266890602436918401016109e0565b602083015260848101356001600160401b03811161030a57602491010136601f8201121561030a57803561269b816109c9565b916126a96040519384610378565b81835260208084019260051b8201019036821161030a57602001915b8183106126e25750505060408201528152602095860195016125f8565b82356001600160401b038116810361030a578152602092830192016126c5565b3461030a576020612740610c5461271836611494565b6001600160a01b039182165f90815260a1865260408082209290931681526020919091522090565b6001600160401b0360405191168152f35b3461030a57604036600319011261030a5760043561276e816102f9565b602435906001600160401b03821161030a573660238301121561030a578160040135906001600160401b03821161030a57366024838501011161030a57602461057d930190613b36565b3461030a5760206001600160401b036127fb6127d336611494565b6001600160a01b039182165f90815260a2865260408082209290931681526020919091522090565b5416604051908152f35b3461030a57604036600319011261030a57600435612822816102f9565b6024356001600160401b03811161030a57612841903690600401611de9565b90612853610dfe600480606654161490565b61285f61078d82613fb1565b6040516336b87bd760e11b81526001600160a01b0382166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156105ae576128c7915f91611b5c575061362e565b60208201906001600160a01b0381165f5b6128e28486613140565b9050811015612a0357806129fd6129f08761294961083a6129156107ac6001986108db8d61290f88613185565b97613140565b612920610e2a610399565b63ffffffff16602084019081528351610ecc9061082b90610ec1906001600160a01b0316610ea8565b61296261295d612959838a6140b0565b1590565b613b8a565b6001600160a01b0387165f908152609c602052604090206129869061083483613d7e565b5061299c87612997611f9984613d7e565b614046565b50857f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e604051806129cd858261311a565b0390a26001600160a01b0387165f908152609e60205260409020611f3a9061202f565b805460ff19166001179055565b016128d8565b8483612a2f86612a22612a1b6108e6610d9a87613185565b9185613140565b9290946040810190613496565b829591953b1561030a575f94612a5d86926040519889978896879563adcf73f760e01b875260048701613ba0565b03925af180156105ae576120c757005b3461030a57604036600319011261030a57612a8a611658366103f3565b5f52609a602052602060405f2054604051908152f35b3461030a5761083a612afc612ab4366118a4565b9391612ac561078d85979397613fb1565b610ecc61082b610ec160405196612adb88610322565b6001600160a01b038116885263ffffffff9094166020880190815293610ea8565b612b0581613d7e565b905f5b838110612b1157005b600190612b3d610fd7612b2c865f52609960205260405f2090565b611fb06108e66108e1868b8d613175565b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee612b6f6119d06108e184898b613175565b0390a101612b08565b3461030a57602036600319011261030a57604063ffffffff612ba4600435612b9f816102f9565b613c12565b835191151582529091166020820152f35b3461030a57602036600319011261030a57600435612bd2816102f9565b60018060a01b03165f526098602052602060405f2054604051908152f35b3461030a57606036600319011261030a57600435612c0d816102f9565b612c16366103b8565b9060018060a01b03165f52609f602052612c3360405f2091613d7e565b5f5260205260405f206040519081602082549182815201915f5260205f20905f5b818110612c6b576106718561169781870382610378565b8254845260209093019260019283019201612c54565b3461030a57604036600319011261030a57600435612c9e816102f9565b612ce36024355f5492612cc960ff600886901c161580958196612d61575b8115612d41575b50613cc7565b83612cda600160ff195f5416175f55565b612d2a57613d2a565b612ce957005b612cf761ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890806020810161148f565b612d3c61010061ff00195f5416175f55565b613d2a565b303b15915081612d53575b505f612cc3565b60ff1660011490505f612d4c565b600160ff8216109150612cbc565b3461030a57604036600319011261030a577f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf856040600435612daf816102f9565b612dfd60243591612dbf836102f9565b612dcb61078d82613fb1565b6001600160a01b038181165f818152609760205286902080546001600160a01b0319169590921694909417905561331d565b82519182526001600160a01b03166020820152a1005b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461030a57602036600319011261030a57600435612e74816102f9565b612e7c6148f1565b6001600160a01b03811615612e945761057d90614949565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461030a57602036600319011261030a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156105ae575f91612f71575b506001600160a01b03163303612f625761057d90613d3b565b63794821ff60e01b5f5260045ffd5b90506020813d602011612fa4575b81612f8c60209383610378565b8101031261030a5751612f9e816102f9565b5f612f49565b3d9150612f7f565b60405190612fb982610342565b5f6040838281528260208201520152565b6104b6612fda93926104b0612fac565b905090565b9081602091031261030a5751801515810361030a5790565b6040513d5f823e3d90fd5b1561300957565b631d77d47760e21b5f5260045ffd5b1561301f57565b63c61dca5d60e01b5f5260045ffd5b6040519061303b82610322565b5f6020838281520152565b90613050826109c9565b61305d6040519182610378565b828152809261306e601f19916109c9565b01905f5b82811061307e57505050565b60209061308961302e565b82828501015201613072565b634e487b7160e01b5f52603260045260245ffd5b80518210156130bd5760209160051b010190565b613095565b156130c957565b63932d94f760e01b5f5260045ffd5b91908110156130bd5760051b81013590603e198136030182121561030a570190565b35610619816103aa565b1561310b57565b631fb1705560e21b5f5260045ffd5b81516001600160a01b0316815260209182015163ffffffff169181019190915260400190565b903590601e198136030182121561030a57018035906001600160401b03821161030a57602001918160051b3603831361030a57565b91908110156130bd5760051b0190565b35610619816102f9565b90613199826109c9565b6131a66040519182610378565b82815280926131b7601f19916109c9565b01905f5b8281106131c757505050565b8060606020809385010152016131bb565b60208183031261030a578051906001600160401b03821161030a57019080601f8301121561030a5781519061320c826109c9565b9261321a6040519485610378565b82845260208085019360051b8201019082821161030a5760208101935b82851061324657505050505090565b84516001600160401b03811161030a57820184603f8201121561030a57602081015190613272826109c9565b916132806040519384610378565b8083526020808085019260051b840101019187831161030a57604001905b8282106132b657505050815260209485019401613237565b815181526020918201910161329e565b90916132dd610619936040845260408401906120ef565b9160208184039101526115ee565b906132f5826109c9565b6133026040519182610378565b8281528092613313601f19916109c9565b0190602036910137565b6001600160a01b039081165f8181526097602052604090205490911680612fda575090565b1561334957565b63840a48d560e01b5f5260045ffd5b1561335f57565b6343714afd60e01b5f5260045ffd5b1561337557565b63ebbff49760e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b5f1981019190821161257157565b156133ad57565b639f1c805360e01b5f5260045ffd5b156133c357565b631353603160e01b5f5260045ffd5b156133d957565b6331bc342760e11b5f5260045ffd5b906001600160401b03809116911603906001600160401b03821161257157565b600f0b60016001607f1b03198114612571575f0390565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b0383131761257157565b6001600160a01b039182168152825182166020808301919091529092015163ffffffff9081166040840152921660608201526001600160401b0390921660808301529190911660a082015260c00190565b903590601e198136030182121561030a57018035906001600160401b03821161030a5760200191813603831361030a57565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff1660408201529095949293918060c0830160c060608501525260e0820196905f5b8181106135825750505080860360808201526020808551978881520194015f965b80881061356a57505061061994955060a08185039101526134c8565b9094602080600192885181520196019701969061354e565b90919760206135a96001928b35613598816102f9565b6001600160a01b0316815260200190565b990192910161352d565b906135bd826109c9565b6135ca6040519182610378565b82815280926135db601f19916109c9565b01905f5b8281106135eb57505050565b6020906135f6612fac565b828285010152016135df565b1561360957565b63585cfb2f60e01b5f5260045ffd5b1561361f57565b6348f5c3ed60e01b5f5260045ffd5b1561363557565b63ccea9e6f60e01b5f5260045ffd5b9060405161365181610342565b604063ffffffff8294546001600160401b038116845280831c600f0b602085015260c01c16910152565b1561368257565b6325131d4f60e01b5f5260045ffd5b63ffffffff60019116019063ffffffff821161257157565b9063ffffffff8091169116019063ffffffff821161257157565b916020908281520191905f5b8181106136dc5750505090565b90919260208060019263ffffffff87356136f5816103aa565b1681520194019291016136cf565b6001600160a01b039091168152604060208201819052610619939101916136c3565b6137569161373a610dfe600180606654161490565b613746611b1383613fb1565b61374f82613c12565b9390613aaf565b5f925b8151841015613aa95761378b602061377186856130a9565b51015151604061378187866130a9565b5101515114613358565b61379584836130a9565b5151906137c561083a6137b1610ea8855160018060a01b031690565b610ecc61082b602087015163ffffffff1690565b6137cf82856140b0565b5f5b60206137dd88876130a9565b51015151811015613a9b57807f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd81886139bc8b613978896138858d6139728e8d61383a610bc960019f60206138328c896130a9565b5101516130a9565b978894613847868d61446a565b613855868d6104b687613d7e565b9d908e9981998b9661387d61387761387160208b0151600f0b90565b600f0b90565b15613ac5565b878b8a614991565b906138c06138b661389d89516001600160401b031690565b6138b0610c9d8860406138328d8d6130a9565b906149f0565b600f0b6020890152565b6138db6138d461387160208a0151600f0b90565b1515613adb565b6020870151600f0b805f811215613a1c575050505f146139c557505050505061392a61391b84610c3f8c60018060a01b03165f5260a360205260405f2090565b61392483613d7e565b90614a21565b6116586139656139607f000000000000000000000000000000000000000000000000000000000000000063ffffffff43166136a9565b613691565b63ffffffff166040870152565b87614149565b6139896139848b613d7e565b613f87565b9361137960406139b06139a384516001600160401b031690565b6020850151600f0b610cff565b92015163ffffffff1690565b0390a1016137d1565b6138326110ef94610c9d94613a00613a059861111560206040970191610cff60206139f785516001600160401b031690565b920151600f0b90565b6130a9565b5f60208601524363ffffffff166040860152613d7e565b94509550955050505f915013613a34575b5050613d7e565b613a9491613a88613a6361396593610c9d610d0560208c0192613a5e84516001600160401b031690565b614095565b6001600160401b03613a7f6104608b516001600160401b031690565b91161115613af1565b63ffffffff43166136a9565b8e5f613a2d565b505093600191500192613759565b50505050565b15613ab657565b63fa55fc8160e01b5f5260045ffd5b15613acc57565b630d8fcbe360e41b5f5260045ffd5b15613ae257565b634606179360e11b5f5260045ffd5b15613af857565b636c9be0bf60e01b5f5260045ffd5b6001600160a01b039081165f90815260a160209081526040808320939094168252919091522061061990614059565b907fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371391613b6561078d82613fb1565b613b856040519283926020845260018060a01b03169560208401916134c8565b0390a2565b15613b9157565b636c6c6e2760e11b5f5260045ffd5b93916106199593613bc69260018060a01b031686526060602087015260608601916136c3565b9260408185039101526134c8565b90604051613be18161035d565b606063ffffffff829454818116845260ff8160201c1615156020850152818160281c16604085015260481c16910152565b60018060a01b03165f52609b60205260405f2090606060405192613c358461035d565b54613c8d613c83613c7d63ffffffff841680885260ff8560201c1615159788602082015263ffffffff808760281c169687604084015260481c16968791015263ffffffff1690565b95151590565b9263ffffffff1690565b63ffffffff811615159081613cb6575b50613ca757509190565b9192505063ffffffff16600191565b63ffffffff1690504310155f613c9d565b15613cce57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b613d366103a892613f55565b614949565b613d4c606654198219811614613018565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b16603482015260208152613dc4604082610378565b5190519060208110613dd4575090565b5f199060200360031b1b1690565b9291611da8613e9191613df361302e565b50613dfc612fac565b50610c3f613e21610c5483610c3f8a60018060a01b03165f5260a160205260405f2090565b94611d9b613e46611cc485610c3f8c60018060a01b03165f5260a260205260405f2090565b98613e61613e52610399565b6001600160401b039099168952565b613e78602089019a8b906001600160401b03169052565b6001600160a01b03165f90815260a06020526040902090565b926040840190613ea861082b835163ffffffff1690565b4310613f1b575f8092613ec287516001600160401b031690565b92613ee6613ed960208a0195610cff8751600f0b90565b6001600160401b03168952565b8351600f0b90838212613efc575b505052529190565b611115613f1492613a5e83516001600160401b031690565b5f80613ef4565b50509190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b613f8f61302e565b5063ffffffff60405191613fa283610322565b8060601c835216602082015290565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af19081156105ae575f9161402d575090565b610619915060203d6020116105a7576105998183610378565b610619916001600160a01b031690614a9e565b80548061406f5750670de0b6b3a7640000919050565b805f19810111612571576001600160401b03915f525f199060205f2001015460201c1690565b6001600160401b0391826140ac9216600f0b61341f565b1690565b6001600160a01b03165f908152609e60205260409020906140d090613d7e565b5f52602052602060405f20604051906140e882610322565b549063ffffffff60ff831615159283835260081c169283910152151590811561410f575090565b63ffffffff16431115919050565b90670de0b6b3a7640000906141328184614bcc565b920961413b5790565b600181018091116125715790565b939092602061416f611cc485610c3f8960018060a01b03165f5260a260205260405f2090565b910180516001600160401b039081169216829003614312575b5050614210816141b084610c3f87611d9b8a60018060a01b03165f5260a060205260405f2090565b81518154602084015160409485015163ffffffff60c01b60c09190911b1677ffffffffffffffffffffffffffffffff00000000000000009190951b166001600160e01b03199091166001600160401b039092169190911717919091179055565b6020810151600f0b1561427a5750826142586142729261424785611d9b6142779860018060a01b03165f52609f60205260405f2090565b6001600160a01b0390911690614a9e565b506001600160a01b03165f908152609d6020526040902090565b614a9e565b50565b516001600160401b03161561428e57505050565b6142c390611f836108e66142b685611d9b8860018060a01b03165f52609f60205260405f2090565b926001600160a01b031690565b506001600160a01b0382165f908152609f602052604090206142e6908290611d9b565b54156142f0575050565b61430d6142779260018060a01b03165f52609d60205260405f2090565b614dec565b6143776143a891610c9d7facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559461435c88610c3f8c60018060a01b03165f5260a260205260405f2090565b906001600160401b03166001600160401b0319825416179055565b604080516001600160a01b03808a168252871660208201526001600160401b03909216908201529081906060820190565b0390a15f80614188565b6001600160a01b038181165f90815260a1602090815260408083209386168352929052207f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c9361444b91614418906001600160401b0383169063ffffffff431690614efb565b604080516001600160a01b0394851681529490931660208501526001600160401b03169183019190915281906060820190565b0390a1565b5f1981146125715760010190565b8015612571575f190190565b6001600160a01b038082165f90815260a3602090815260408083209386168352929052908120909392919061449e90611cf3565b935b84151580614557575b15614550576144d46144cf84610c3f8560018060a01b03165f5260a360205260405f2090565b614c7d565b6144df848285613de2565b916144f461082b604085015163ffffffff1690565b43106145465761453a9261454094928761450e9388614149565b61453461452f86610c3f8760018060a01b03165f5260a360205260405f2090565b614cbe565b50614450565b9461445e565b936144a0565b5050505050509050565b5050509050565b5061ffff81106144a9565b6001600160a01b038181165f90815260a360209081526040808320938616835292905290812090949061459490611cf3565b945b8515158061463f575b15614637576145c56144cf85610c3f8660018060a01b03165f5260a360205260405f2090565b6145d0858286613de2565b916145e561082b604085015163ffffffff1690565b431061462c57614620926146269492886145ff9389614149565b61453461452f87610c3f8860018060a01b03165f5260a360205260405f2090565b9561445e565b94614596565b505050509350505050565b509350505050565b5061ffff8516811061459f565b907f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9161444b6147996146976146928460018060a01b03165f52609b60205260405f2090565b613bd4565b61083f60608201916146ad835163ffffffff1690565b63ffffffff8116151590816147f4575b506147ca575b63ffffffff8716604082015261470f6147056139607f000000000000000000000000000000000000000000000000000000000000000063ffffffff43166136a9565b63ffffffff168452565b6001600160a01b0386165f908152609b602052604090208151815460208085015160408601516060909601516cffffffff00000000000000000060489190911b1668ffffffff000000000060289790971b9690961664ff0000000091151590921b166cffffffffffffffffffffffffff1990921663ffffffff909316929092171717919091179055565b604080516001600160a01b03909416845263ffffffff94851660208501529316928201929092529081906060820190565b6147e86147de604083015163ffffffff1690565b63ffffffff168252565b600160208201526146c3565b63ffffffff1690504310155f6146bd565b9190915f838201938412911290801582169115161761257157565b805490916001600160ff1b0381116148885761387161484d6148486148549385600f0b614805565b614d2e565b9260801d90565b81600f0b1215614879576001614875920190600f0b5f5260205260405f2090565b5490565b632d0483c560e21b5f5260045ffd5b60405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608490fd5b610619916001600160a01b031690614dec565b6033546001600160a01b0316330361490557565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9061499e6149c692613d7e565b5f52609960205260405f209060018060a01b0316906001915f520160205260405f2054151590565b91826149e8575b50816149d7575090565b6001600160401b0391505116151590565b91505f6149cd565b6001600160401b03809116600f0b9116600f0b0360016001607f1b03811360016001607f1b03198212176125715790565b90815460801d90614a40826001850190600f0b5f5260205260405f2090565b5581546001600160801b0316600190910160801b6fffffffffffffffffffffffffffffffff1916179055565b80548210156130bd575f5260205f2001905f90565b91614a9a9183549060031b91821b915f19901b19161790565b9055565b5f828152600182016020526040902054614b0157805490600160401b82101561033d5782614aec614ad6846001809601855584614a6c565b819391549060031b91821b915f19901b19161790565b90558054925f520160205260405f2055600190565b50505f90565b8115614b11570490565b634e487b7160e01b5f52601260045260245ffd5b1561030a57565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614bc057670de0b6b3a76400008291614b6c868411614b25565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b50906106199250614b07565b5f1982820982820291828083109203918083039214614c355781670de0b6b3a7640000111561030a577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b90915f198383099280830292838086109503948086039514614c7057908291614b6c868411614b25565b5050906106199250614b07565b614c92815480600f0b9060801d600f0b131590565b614caf578054600f0b5f9081526001909101602052604090205490565b631ed9509560e11b5f5260045ffd5b90614cd4825480600f0b9060801d600f0b131590565b614caf578154600f0b9160018101925f614d0f82614cfd818890600f0b5f5260205260405f2090565b549690600f0b5f5260205260405f2090565b5560016001600160801b031983541691016001600160801b0316179055565b60016001607f1b031981121580614da0575b15614d4b57600f0b90565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608490fd5b5060016001607f1b03811315614d40565b80548015614dd8575f190190614dc78282614a6c565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614e87575f1984018481116125715783545f19810194908511612571575f958583611d9b94614e3a9803614e40575b505050614db1565b55600190565b614e70614e6a91614e61614e57614e7e9588614a6c565b90549060031b1c90565b92839187614a6c565b90614a81565b85905f5260205260405f2090565b555f8080614e32565b505050505f90565b15614e9657565b63151b8e3f60e11b5f5260045ffd5b8054600160401b81101561033d57614ec291600182018155614a6c565b614ee857815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b805480614f37575b50614f326103a893614f22614f16610399565b63ffffffff9095168552565b6001600160e01b03166020840152565b614ea5565b805f1981011161257157815f5263ffffffff614f9261082b5f198460205f20010161083f614f8460405192614f6b84610322565b548681169081855260201c602085015263ffffffff1690565b858916958691161115614e8f565b03614f03576103a89392509061251c614faa92613398565b9063ffffffff82549181199060201b16911617905556fea2646970667358221220de0eea5298eb5e93bb9991899e83b61e9d6ebf05e09699f970429f4b4194634964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 4a\x01\xEDW`\x1FaR\x0E8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\xF1W\x80\x84\x92`\xA0\x94`@R\x839\x81\x01\x03\x12a\x01\xEDW\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xEDW` \x82\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x91\x82\x84\x03a\x01\xEDW`@\x82\x01Q\x93`\x01`\x01`\xA0\x1B\x03\x85\x16\x85\x03a\x01\xEDWa\0\x97`\x80a\0\x90``\x86\x01a\x02\x05V[\x94\x01a\x02\x05V[\x93\x15a\x01\xDEW`\x80R`\xA0R`\xC0R`\xE0Ra\x01\0R_T`\xFF\x81`\x08\x1C\x16a\x01\x89W`\xFF\x80\x82\x16\x10a\x01OW[`@QaO\xF7\x90\x81a\x02\x17\x829`\x80Q\x81\x81\x81a\x05#\x01R\x81\x81a\x1B\xA6\x01R\x81\x81a\"\xF1\x01Ra/\x11\x01R`\xA0Q\x81\x81\x81a\x0Bd\x01R\x81\x81a\x0F\x01\x01R\x81\x81a\x1A\xC6\x01R\x81\x81a(\x87\x01Ra.(\x01R`\xC0Q\x81\x81\x81a\t\xA5\x01R\x81\x81a\x1Ek\x01Ra95\x01R`\xE0Q\x81\x81\x81a\"\xB8\x01RaF\xDA\x01Ra\x01\0Q\x81\x81\x81a\x16\xCE\x01Ra?\xFA\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\xC5V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01\xEDWV\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x10\xE1\xB9\xB8\x14a\x02\xF4W\x80c\x13d9\xDD\x14a\x02\xEFW\x80c\x15\xFEP(\x14a\x02\xEAW\x80c&\r\xC7X\x14a\x02\xE5W\x80c&\x1F\x84\xE0\x14a\x02\xE0W\x80c)\x81\xEBw\x14a\x02\xDBW\x80c+\xAB,J\x14a\x02\xD6W\x80c0L\x10\xCD\x14a\x02\xD1W\x80c65 W\x14a\x02\xCCW\x80c@\x12\r\xAB\x14a\x02\xC7W\x80cAw\xA8|\x14a\x02\xC2W\x80cFW\xE2j\x14a\x02\xBDW\x80cJ\x10\xFF\xE5\x14a\x02\xB8W\x80cKPF\xEF\x14a\x02\xB3W\x80cP\xFE\xEA \x14a\x02\xAEW\x80cTz\xFB\x87\x14a\x02\xA9W\x80cV\xC4\x83\xE6\x14a\x02\xA4W\x80cY\\jg\x14a\x02\x9FW\x80cZ\xC8j\xB7\x14a\x02\x9AW\x80c\\\x97Z\xBB\x14a\x02\x95W\x80cg\r;\xA2\x14a\x02\x90W\x80cl\xFBD\x81\x14a\x02\x8BW\x80cn4\x92\xB5\x14a\x02\x86W\x80cn\x87]\xBA\x14a\x02\x81W\x80cqP\x18\xA6\x14a\x02|W\x80cy\xAEP\xCD\x14a\x02wW\x80c{\xC1\xEFa\x14a\x02rW\x80c\x88o\x11\x95\x14a\x02mW\x80c\x8C\xE6HT\x14a\x02hW\x80c\x8D\xA5\xCB[\x14a\x02cW\x80c\x94\xD7\xD0\x0C\x14a\x02^W\x80c\x95(\x99\xEE\x14a\x02YW\x80c\xA93>\xC8\x14a\x02TW\x80c\xA9\x82\x18!\x14a\x02OW\x80c\xA9\x84\xEB:\x14a\x02JW\x80c\xAD\xC2\xE3\xD9\x14a\x02EW\x80c\xB2Dz\xF7\x14a\x02@W\x80c\xB6k\xD9\x89\x14a\x02;W\x80c\xB9\xFB\xAE\xD1\x14a\x026W\x80c\xBA\x1A\x84\xE5\x14a\x021W\x80c\xC2!\xD8\xAE\x14a\x02,W\x80c\xCDm\xC6\x87\x14a\x02'W\x80c\xD3\xD9o\xF4\x14a\x02\"W\x80c\xDF\\\xF7#\x14a\x02\x1DW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x18Wc\xFA\xBC\x1C\xBC\x14a\x02\x13W_\x80\xFD[a.\xE8V[a.WV[a.\x13V[a-oV[a,\x81V[a+\xF0V[a+\xB5V[a+xV[a*\xA0V[a*mV[a(\x05V[a'\xB8V[a'QV[a'\x02V[a%\x84V[a$\x0EV[a#\xE6V[a#1V[a\"\xDCV[a\"\x9CV[a\"\x10V[a!\xB5V[a!<V[a\x1D\xF7V[a\x1C\x95V[a\x1C>V[a\x1C!V[a\x1B\xEEV[a\x1B{V[a\x1A\x9BV[a\x1A\rV[a\x18\xECV[a\x17\xDCV[a\x17?V[a\x16\xB9V[a\x16;V[a\x15AV[a\r\xB1V[a\rxV[a\n\xD1V[a\t\x89V[a\x07DV[a\x06\xB8V[a\x06\x1CV[a\x04\xF3V[a\x04lV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\nWV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[a\x03\x0EV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[`@Q\x90a\x03\xA8`@\x83a\x03xV[V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\nWV[`@\x90`#\x19\x01\x12a\x03\nW`@Q\x90a\x03\xD1\x82a\x03\"V[\x81`$5a\x03\xDE\x81a\x02\xF9V[\x81R` `D5\x91a\x03\xEF\x83a\x03\xAAV[\x01RV[`@\x90`\x03\x19\x01\x12a\x03\nW`@Q\x90a\x04\x0C\x82a\x03\"V[\x81`\x045a\x04\x19\x81a\x02\xF9V[\x81R` `$5\x91a\x03\xEF\x83a\x03\xAAV[\x91\x90\x82`@\x91\x03\x12a\x03\nW`@Qa\x04B\x81a\x03\"V[` \x80\x82\x94\x805a\x04R\x81a\x02\xF9V[\x84R\x015\x91a\x03\xEF\x83a\x03\xAAV[`\x01`\x01`@\x1B\x03\x16\x90V[4a\x03\nW`\x806`\x03\x19\x01\x12a\x03\nW``a\x04\xBC`\x045a\x04\x8E\x81a\x02\xF9V[a\x04\x976a\x03\xB8V[\x90a\x04\xB6`d5\x92a\x04\xA8\x84a\x02\xF9V[a\x04\xB0a/\xACV[Pa=~V[\x90a=\xE2V[\x90Pa\x04\xF1`@Q\x80\x92c\xFF\xFF\xFF\xFF`@\x80\x92`\x01`\x01`@\x1B\x03\x81Q\x16\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R\x01Q\x16\x91\x01RV[\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\xAEWa\x05}\x92a\x05i\x91_\x91a\x05\x7FW[Pa0\x02V[a\x05x`fT\x82\x81\x16\x14a0\x18V[a?UV[\0[a\x05\xA1\x91P` =` \x11a\x05\xA7W[a\x05\x99\x81\x83a\x03xV[\x81\x01\x90a/\xDFV[_a\x05cV[P=a\x05\x8FV[a/\xF7V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x05\xD0WPPP\x90V[\x90\x91\x92` `@\x82a\x05\xFD`\x01\x94\x88Qc\xFF\xFF\xFF\xFF` \x80\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x01\x94\x01\x92\x91\x01a\x05\xC3V[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x05\xB3V[\x90V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a\x069\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9D` R`@\x90 Ta\x06Y\x81a0FV[\x91_[\x82\x81\x10a\x06uW`@Q\x80a\x06q\x86\x82a\x06\x08V[\x03\x90\xF3[`\x01\x90\x82_R`\x9D` Ra\x06\x9Ca\x06\x90\x82`@_ aJlV[\x90T\x90`\x03\x1B\x1Ca?\x87V[a\x06\xA6\x82\x87a0\xA9V[Ra\x06\xB1\x81\x86a0\xA9V[P\x01a\x06\\V[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW` a\x07\na\x06\xD76a\x03\xF3V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x84R`@\x80\x82 \x92\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x90\x92\x01` R T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03\nW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\nW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\nWV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x07a\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x07\x80\x906\x90`\x04\x01a\x07\x14V[\x90a\x07\x92a\x07\x8D\x84a?\xB1V[a0\xC2V[_\x91[\x80\x83\x10a\x07\x9EW\0[a\x07\xB1a\x07\xAC\x84\x83\x85a0\xD8V[a0\xFAV[\x90\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~la\x08za\x08Ia\x07\xE1a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x81R\x94c\xFF\xFF\xFF\xFF\x16` \x86\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x98` R`@\x90 a\x08?\x90a\x08:\x90a\x084a\x08+\x85Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x90aJ\x9EV[a1\x04V[Qc\xFF\xFF\xFF\xFF\x16\x90V[a\x08na\x08Ta\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R\x91c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`@Q\x91\x82\x91\x82a1\x1AV[\x03\x90\xA1a\x08\x86\x82a=~V[\x92_[a\x08\xA1a\x08\x97\x87\x85\x85a0\xD8V[` \x81\x01\x90a1@V[\x90P\x81\x10\x15a\toW\x80a\x08\xF8\x87a\x08\xF2a\x08\xE6a\x08\xE1`\x01\x96a\x08\xDBa\x08\x97a\x08\xD3\x8E_R`\x99` R`@_ \x90V[\x96\x8C\x8Ca0\xD8V[\x90a1uV[a1\x85V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a@FV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\tfa\t2a\x08\xE1\x84a\x08\xDBa\x08\x97\x8D\x8B\x8Ba0\xD8V[`@\x80Q\x89Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8C\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x08\x89V[P`\x01\x90\x94\x01\x93\x92P\x90Pa\x07\x95V[_\x91\x03\x12a\x03\nWV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03=W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\nW\x815a\t\xF7\x81a\t\xC9V[\x92a\n\x05`@Q\x94\x85a\x03xV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\nW` \x01\x90[\x82\x82\x10a\n-WPPP\x90V[` \x80\x91\x835a\n<\x81a\x02\xF9V[\x81R\x01\x91\x01\x90a\n V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\nyWPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\n\xB9WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\njV[\x90\x91\x92` \x80`\x01\x92\x86Q\x81R\x01\x94\x01\x92\x01\x90a\n\x9AV[4a\x03\nW`\xA06`\x03\x19\x01\x12a\x03\nWa\n\xEB6a\x03\xF3V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x0B\n\x906\x90`\x04\x01a\t\xE0V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x0B*\x906\x90`\x04\x01a\t\xE0V[\x90`\x845a\x0B7\x81a\x03\xAAV[a\x0BA\x84Qa1\x8FV[\x90`@Qcxps;`\xE1\x1B\x81R_\x81\x80a\x0B`\x88\x8A`\x04\x84\x01a2\xC6V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05\xAEW_\x91a\rVW[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90_g\r\xE0\xB6\xB3\xA7d\0\0\x94[\x87Q\x82\x10\x15a\rHWa\x0B\xD6a\x0B\xC9\x83\x8Aa0\xA9V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x0B\xE1\x88Qa2\xEBV[a\x0B\xEB\x84\x88a0\xA9V[Ra\x0B\xF6\x83\x87a0\xA9V[P_[\x88\x80Q\x82\x10\x15a\r:W\x90\x83\x86a\x0C\x15a\x0B\xC9\x84`\x01\x96a0\xA9V[\x8B`\x01`\x01`@\x1B\x03a\x0CYa\x0CT\x84a\x0C?\x87`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a@YV[\x16\x91\x82\x15a\r0Wa\x04`a\x0Cxa\x0C\xAA\x92a\x0C\xC9\x97a\x0C\xAF\x97a/\xCAV[\x8Dc\xFF\xFF\xFF\xFFa\x0C\x8F`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15\x80a\r\x17W[a\x0C\xDFW[Q`\x01`\x01`@\x1B\x03\x16\x90V[aLFV[a\x0C\xC3\x83a\x0C\xBD\x89\x88a0\xA9V[Qa0\xA9V[QaK\xCCV[a\x0C\xD7\x82a\x0C\xBD\x88\x8Ca0\xA9V[R[\x01a\x0B\xF9V[a\r\x12a\r\x05a\x0C\xF6\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x84\x01Q`\x0F\x0B[\x90a@\x95V[`\x01`\x01`@\x1B\x03\x16\x82RV[a\x0C\x9DV[P_a\r'` \x83\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x12a\x0C\x98V[PPPPPa\x0C\xD9V[PP\x92P\x90`\x01\x01\x90a\x0B\xB3V[`@Q\x80a\x06q\x87\x82a\nGV[a\rr\x91P=\x80_\x83>a\rj\x81\x83a\x03xV[\x81\x01\x90a1\xD8V[_a\x0B\x9CV[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW` a\r\x9F`\x045a\r\x9A\x81a\x02\xF9V[a3\x1DV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\r\xCE\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW`\xA0`\x03\x19\x826\x03\x01\x12a\x03\nWa\x0E\x04a\r\xFE`\x02\x80`fT\x16\x14\x90V[\x15a3BV[a\x0E\x10a\x07\x8D\x83a?\xB1V[a\x0E\xE5a\x0E\x1F`$\x83\x01a0\xFAV[a\x0E9a\x0E*a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[c\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81Ra\x0E\xE0a\x08:a\x0Ea\x86a\x0E\\\x87`\x04\x01a1\x85V[a@\xB0V[\x92a\x0E\x90a\x0Eu`D\x88\x01\x88`\x04\x01a1@V[\x90Pa\x0E\x87`d\x89\x01\x89`\x04\x01a1@V[\x91\x90P\x14a3XV[\x86Qa\x0E\xCC\x90a\x08+\x90a\x0E\xC1\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98` R`@\x90 \x90V[\x92Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[a3nV[a\x0E\xFFa\x0E\xF8`D\x83\x01\x83`\x04\x01a1@V[\x90Pa2\xEBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x90_[a\x0F=`D\x84\x01\x84`\x04\x01a1@V[\x90P\x81\x10\x15a\x14)W\x80\x15\x80\x15a\x13\xC7W[a\x0FX\x90a3\xA6V[a\x0Fl\x81a\x08\xDB`d\x86\x01\x86`\x04\x01a1@V[5\x15\x15\x80a\x13\x9CW[a\x0F~\x90a3\xBCV[a\x0F\xDCa\x0F\xD7a\x0F\x9Ea\x0F\x90\x88a=~V[_R`\x99` R`@_ \x90V[a\x0F\xB8a\x08\xE6a\x08\xE1\x86a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x15\x15\x90V[a3\xD2V[a\x10\x11a\x0F\xEB\x84`\x04\x01a1\x85V[a\x0F\xF4\x87a=~V[a\x10\x0Ba\x08\xE1\x85a\x08\xDB`D\x8A\x01\x8A`\x04\x01a1@V[\x91a=\xE2V[\x91\x90a\x10'a\x04`\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x80\x15a\x13\x91Wa\x10pa\x04`\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x92a\x10i\x86a\x08\xDB`d\x8C\x01\x8C`\x04\x01a1@V[5\x90aA\x1DV[\x93a\x11\"a\x10\x85\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x95a\x10\xA2`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x83\x16aK,V[a\x10\xAC\x87\x8Aa0\xA9V[Ra\x10\xD7a\x10\xCA\x82a\x10\xC5\x86Q`\x01`\x01`@\x1B\x03\x16\x90V[a3\xE8V[`\x01`\x01`@\x1B\x03\x16\x84RV[a\x10\xFCa\x10\xEF\x82a\x10\xC5\x88Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x86RV[a\x11\x15` \x86\x01\x91a\x10\xC5\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90RV[\x86a\x111` \x83\x01Q`\x0F\x0B\x90V[\x85_a\x11=\x83`\x0F\x0B\x90V[\x12a\x12\xB9W[\x83\x91P\x8B\x83a\x11ya\x08\xE1\x89\x94a\x08\xDBa\x11ka\x11ea\x11\x7F\x9A`\x04\x01a1\x85V[\x96a=~V[\x94`D\x81\x01\x90`\x04\x01a1@V[\x91aAIV[a\x11\x8B\x87`\x04\x01a1\x85V[\x90a\x11\xCDa\x11\xB7a\x11\xA9a\x08\xE1\x88a\x08\xDB`D\x8E\x01`\x04\x8F\x01a1@V[\x92Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92`@Q\x93\x84\x93\x8Dc\xFF\xFF\xFF\xFFC\x16\x93\x86a4EV[\x03\x90\xA1a\x12\x07a\x11\xDF\x86`\x04\x01a1\x85V[a\x11\xF6a\x08\xE1\x85a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[\x83Q`\x01`\x01`@\x1B\x03\x16\x91aC\xB2V[a\x12\x13\x85`\x04\x01a1\x85V[\x90a\x12<a\x12.a\x08\xE1\x85a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[\x91Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x87;\x15a\x03\nW`@Qc`\x1B\xB3o`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x93\x84\x16`D\x82\x01R\x92\x16`d\x83\x01R_\x82`\x84\x81\x83\x89Z\xF1\x91\x82\x15a\x05\xAEW`\x01\x92a\x12\x9FW[P[\x01a\x0F-V[\x80a\x12\xAD_a\x12\xB3\x93a\x03xV[\x80a\t\x7FV[_a\x12\x97V[a\x12\xFCa\x04`a\x04`a\x13\"\x95a\x10ia\x13\x0B\x95a\x08\xDBa\x12\xEEa\x12\xE2a\x12\xE2a\x13\x18\x9Ba4\x08V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93`d\x81\x01\x90`\x04\x01a1@V[`\x01`\x01`\x80\x1B\x03\x16`\x0F\x0B\x90V[` \x84\x01Q`\x0F\x0Ba4\x1FV[`\x0F\x0B` \x83\x01RV[\x81\x89\x88a\x13\x86a\x13La\x08\xE1\x89a\x08\xDBa\x13>\x86`\x04\x01a1\x85V[\x95`D\x81\x01\x90`\x04\x01a1@V[a\x13ma\x13`\x87Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x88\x01Q`\x0F\x0Ba\x0C\xFFV[`@\x87\x01Qc\xFF\xFF\xFF\xFF\x16[\x91`@Q\x95\x86\x95\x86a4EV[\x03\x90\xA1\x86_\x85a\x11CV[PP`\x01\x91Pa\x12\x99V[Pa\x0F~g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xBD\x83a\x08\xDB`d\x88\x01\x88`\x04\x01a1@V[5\x11\x15\x90Pa\x0FuV[Pa\x0FXa\x13\xE8a\x08\xE6a\x08\xE6a\x08\xE1\x85a\x08\xDB`D\x8A\x01\x8A`\x04\x01a1@V[a\x14\x16a\x08\xE6a\x08\xE6a\x08\xE6a\x08\xE1a\x14\x07`D\x8B\x01\x8B`\x04\x01a1@V[a\x14\x10\x8Aa3\x98V[\x91a1uV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x90Pa\x0FOV[\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5\x85\x83a\x14\x8Fa\x14\x80\x87a\x14_\x81`\x04\x01a1\x85V[\x93a\x14p`D\x83\x01\x83`\x04\x01a1@V[\x93\x90\x92`\x84\x81\x01\x90`\x04\x01a4\x96V[\x93\x90\x92`@Q\x97\x88\x97\x88a4\xE8V[\x03\x90\xA1\0[`@\x90`\x03\x19\x01\x12a\x03\nW`\x045a\x14\xAC\x81a\x02\xF9V[\x90`$5a\x06\x19\x81a\x02\xF9V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x14\xD6WPPP\x90V[\x90\x91\x92` ``\x82a\x15\x11`\x01\x94\x88Qc\xFF\xFF\xFF\xFF`@\x80\x92`\x01`\x01`@\x1B\x03\x81Q\x16\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R\x01Q\x16\x91\x01RV[\x01\x94\x01\x92\x91\x01a\x14\xC9V[\x90\x91a\x153a\x06\x19\x93`@\x84R`@\x84\x01\x90a\x05\xB3V[\x91` \x81\x84\x03\x91\x01Ra\x14\xB9V[4a\x03\nWa\x15O6a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x9D` R`@\x90 T\x90\x92\x91a\x15s\x82a0FV[\x92a\x15}\x83a5\xB3V[\x94_[\x84\x81\x10a\x15\x96W`@Q\x80a\x06q\x89\x89\x83a\x15\x1CV[`\x01\x90\x82_R`\x9D` Ra\x15\xD2\x85a\x15\xB5a\x06\x90\x84`@_ aJlV[\x80a\x15\xC0\x85\x8Ca0\xA9V[Ra\x15\xCB\x84\x8Ba0\xA9V[P\x86a/\xCAV[a\x15\xDC\x82\x8Aa0\xA9V[Ra\x15\xE7\x81\x89a0\xA9V[P\x01a\x15\x80V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x16\x0BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xFEV[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x15\xEEV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa\x16]a\x16X6a\x03\xF3V[a=~V[_R`\x99` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\x16\xA3Wa\x06q\x85a\x16\x97\x81\x87\x03\x82a\x03xV[`@Q\x91\x82\x91\x82a\x16*V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x16\x80V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x17 WPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\x13V[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x17o\x906\x90`\x04\x01a\t\xE0V[`$5\x90a\x17|\x82a\x02\xF9V[a\x17\x86\x81Qa2\xEBV[\x91_[\x82Q\x81\x10\x15a\x17\xCEW`\x01\x90a\x17\xB2\x83`\x01`\x01`\xA0\x1B\x03a\x17\xAB\x84\x88a0\xA9V[Q\x16a;\x07V[`\x01`\x01`@\x1B\x03a\x17\xC4\x83\x88a0\xA9V[\x91\x16\x90R\x01a\x17\x89V[`@Q\x80a\x06q\x86\x82a\x16\xFDV[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a\x17\xF9\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x18\x18\x906\x90`\x04\x01a\x07\x14V[\x90\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x189\x906\x90`\x04\x01a\x07\x14V[\x92\x90\x93a\x18Ma\r\xFE`\x01\x80`fT\x16\x14\x90V[a\x18X\x84\x83\x14a3XV[_[\x82\x81\x10a\x18cW\0[a\x18n\x81\x84\x84a1uV[5\x90a\x18y\x82a\x02\xF9V[a\x18\x84\x81\x87\x89a1uV[5\x91a\xFF\xFF\x83\x16\x83\x03a\x03\nW`\x01\x92a\x18\x9E\x91\x87aEbV[\x01a\x18ZV[```\x03\x19\x82\x01\x12a\x03\nW`\x045a\x18\xBC\x81a\x02\xF9V[\x91`$5a\x18\xC9\x81a\x03\xAAV[\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nWa\x18\xE8\x91`\x04\x01a\x07\x14V[\x90\x91V[4a\x03\nWa\x18\xFA6a\x18\xA4V[\x91a\x19\ta\x07\x8D\x85\x93\x95a?\xB1V[a\x19ba\x08:`@Q\x92a\x19\x1C\x84a\x03\"V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x84Rc\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81Ra\x0E\xCCa\x08+a\x0E\xC1a\x19G\x87a=~V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16_\x90\x81R`\x98` R`@\x90 \x90V[_[\x83\x81\x10a\x19mW\0[`\x01\x90a\x19\x9Ea\x19\x99a\x19\x88\x86_R`\x99` R`@_ \x90V[a\x08\xF2a\x08\xE6a\x08\xE1\x86\x8B\x8Da1uV[a6\x02V[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\x1A\x04a\x19\xD0a\x08\xE1\x84\x89\x8Ba1uV[`@\x80Q\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8A\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x19dV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x1A*\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x1AI\x906\x90`\x04\x01a\t\xE0V[a\x1AS\x81Qa2\xEBV[\x91_[\x82Q\x81\x10\x15a\x17\xCEW`\x01\x90a\x1A\x7F`\x01`\x01`\xA0\x1B\x03a\x1Aw\x83\x87a0\xA9V[Q\x16\x84a;\x07V[`\x01`\x01`@\x1B\x03a\x1A\x91\x83\x88a0\xA9V[\x91\x16\x90R\x01a\x1AVV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x1A\xB8\x81a\x02\xF9V[`$5a\x1A\xC4\x81a\x03\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x1B\x02W[a\x05}\x92PaFLV[` `$\x93a\x1B\x18a\x1B\x13\x84a?\xB1V[a6\x18V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x94\x85\x91\x82\x90Z\xFA\x92\x83\x15a\x05\xAEWa\x05}\x93a\x1BW\x91_\x91a\x1B\\W[Pa6.V[a\x1A\xF8V[a\x1Bu\x91P` =` \x11a\x05\xA7Wa\x05\x99\x81\x83a\x03xV[_a\x1BQV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\xAEWa\x1B\xE6\x91_\x91a\x05\x7FWPa0\x02V[a\x05}a?!V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`\xFF\x81\x16\x80\x91\x03a\x03\nW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `fT`@Q\x90\x81R\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW` a\x07\n`\x045a\x1C`\x81a\x02\xF9V[a\x1Cla\x16X6a\x03\xB8V[_\x90\x81R`\x9A\x84R`@\x80\x82 `\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x82R`\x01\x90\x92\x01` R T\x15\x15\x90V[4a\x03\nWa\x1C\xA36a\x14\x94V[a\x1C\xD1a\x1C\xC4\x82a\x0C?\x85`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[T`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x1D\x03a\x1C\xF3\x82a\x0C?\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x03\x90V[_\x90[\x80\x82\x10a\x1DPW[a\x06qa\x1D6\x85a\x10\xC5a\x0CT\x87a\x0C?\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x90\x92a\x1D\xADa\x1D\xA8\x84a\x0C?a\x1D\x83\x88a\x1D~\x84a\x0C?\x8D`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aH V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\xA0` R`@\x90 [\x90_R` R`@_ \x90V[a6DV[a\x1D\xC1a\x08+`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10a\x1D\xE2W` \x01Q`\x01\x91a\x1D\xDA\x91`\x0F\x0Ba\x0C\xFFV[\x93\x01\x90a\x1D\x06V[P\x92a\x1D\x0EV[\x90\x81``\x91\x03\x12a\x03\nW\x90V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x1E'\x906\x90`\x04\x01a\x1D\xE9V[a\x1E8a\r\xFE`\x04\x80`fT\x16\x14\x90V[a\x1EIa\x1ED\x82a1\x85V[a?\xB1V[\x80\x15a \xD5W[a\x1EY\x90a6\x18V[` \x81\x01\x90`@\x81\x01Cc\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_[a\x1E\x96\x84\x86a1@V[\x90P\x81\x10\x15a lW\x80a f\x86a\x1F\x03a\x08:\x8Aa\x0E\xCCa\x08+a\x0E\xC1a\x0E\xA8a\x1E\xD4a\x07\xAC\x8F\x9Ca\x08\xDB`\x01\x9Ea\x1E\xCE\x8Aa1\x85V[\x9Ca1@V[\x94a\x1E\xEFa\x1E\xE0a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x8ARV[a\x08\xE1` \x8A\x01\x96\x87\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1FRa\x1FMa\x1FFa\x1F1a\x1F\x18\x8Ca1\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9E` R`@\x90 \x90V[a\x1F:\x85a=~V[_R` R`@_ \x90V[T`\xFF\x16\x90V[a6{V[a\x1F\x89a\x1Fza\x1Fa\x8Aa1\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9C` R`@\x90 \x90V[a\x1F\x83\x83a=~V[\x90aM\xECV[Pa\x1F\xB6a\x1F\xA7a\x1F\x99\x83a=~V[_R`\x9A` R`@_ \x90V[a\x1F\xB0\x8Aa1\x85V[\x90aH\xDEV[Pa\x1F\xC3a\x08\xE6\x89a1\x85V[\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE`@Q\x80a\x1F\xF2\x85\x82a1\x1AV[\x03\x90\xA2a 5a \x02\x86\x88a6\xA9V[\x91a  a \x0Ea\x03\x99V[_\x81Rc\xFF\xFF\xFF\xFF\x90\x94\x16` \x85\x01RV[a\x1F:a /a\x1F\x18\x8Ca1\x85V[\x91a=~V[\x90` \x90\x80Q\x15\x15`\xFF\x80\x19\x85T\x16\x91\x16\x17\x83U\x01Qd\xFF\xFF\xFF\xFF\0\x82T\x91`\x08\x1B\x16\x90d\xFF\xFF\xFF\xFF\0\x19\x16\x17\x90UV[\x01a\x1E\x8CV[\x83\x85a \x90a \x80a\x08\xE6a\r\x9A\x8Ba1\x85V[\x92a \x8A\x83a1\x85V[\x92a1@V[\x90\x92\x80;\x15a\x03\nWa \xBD\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x9D\x8E\x0C#`\xE0\x1B\x85R`\x04\x85\x01a7\x03V[\x03\x92Z\xF1a \xC7W\0[\x80a\x12\xAD_a\x05}\x93a\x03xV[Pa\x1EYa \xE8a\x1ED` \x84\x01a1\x85V[\x90Pa\x1EPV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a!\x0CWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a \xFFV[\x90` a\x06\x19\x92\x81\x81R\x01\x90a \xEFV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa!Ya\x16X6a\x03\xF3V[_R`\x9A` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a!\x9FWa\x06q\x85a!\x93\x81\x87\x03\x82a\x03xV[`@Q\x91\x82\x91\x82a!+V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!|V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nWa!\xCDaH\xF1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a\"-\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9C` R`@\x90 Ta\"M\x81a0FV[\x91_[\x82\x81\x10a\"eW`@Q\x80a\x06q\x86\x82a\x06\x08V[`\x01\x90\x82_R`\x9C` Ra\"\x80a\x06\x90\x82`@_ aJlV[a\"\x8A\x82\x87a0\xA9V[Ra\"\x95\x81\x86a0\xA9V[P\x01a\"PV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x14\xB9V[4a\x03\nW`\x806`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa#a\x906\x90`\x04\x01a\t\xE0V[a#j6a\x03\xB8V[\x90`d5a#w\x81a\x02\xF9V[a#\x81\x82Qa5\xB3V[\x92_[\x83Q\x81\x10\x15a#\xD8W`\x01\x90a#\xBA\x84`\x01`\x01`\xA0\x1B\x03a#\xA6\x84\x89a0\xA9V[Q\x16a#\xB0a/\xACV[Pa\x04\xB6\x86a=~V[\x90Pa#\xC6\x82\x88a0\xA9V[Ra#\xD1\x81\x87a0\xA9V[P\x01a#\x84V[`@Q\x80a\x06q\x87\x82a# V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a$+\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa$J\x906\x90`\x04\x01a\t\xE0V[\x90`D5a$W\x81a\x03\xAAV[a$a\x83Qa2\xEBV[\x92_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFF\xFF\xFF\xFF\x16\x90[\x80Q\x84\x10\x15a%vW_\x83\x81R`\xA1` R`@\x90 a$\xB8\x90`\x01`\x01`\xA0\x1B\x03a$\xA2\x87\x85a0\xA9V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84T\x94_\x95[\x80\x87\x10a%.W`\x01\x93\x94\x95\x96P\x80\x15_\x14a%\x06WPa$\xFD\x90Pg\r\xE0\xB6\xB3\xA7d\0\0[a$\xEF\x83\x89a0\xA9V[\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x01\x92\x91\x90a$vV[a\x04`a%'a$\xFD\x93a%\x1Ca$\xE5\x94a3\x98V[\x90_R` _ \x01\x90V[T` \x1C\x90V[\x80\x87\x16\x90\x80\x88\x18`\x01\x1C\x82\x01\x80\x92\x11a%qW\x82_R\x85c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a%bWP\x95[\x95a$\xBFV[\x96P`\x01\x81\x01\x80\x91\x11\x15a%\\W[a3\x84V[`@Q\x80a\x06q\x87\x82a\x16\xFDV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a%\xA1\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW6`#\x82\x01\x12\x15a\x03\nW\x80`\x04\x015\x91a%\xCC\x83a\t\xC9V[\x91a%\xDA`@Q\x93\x84a\x03xV[\x83\x83R`$` \x84\x01\x94`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03\nW`$\x81\x01\x94[\x82\x86\x10a&\tWa\x05}\x85\x85a7%V[\x855`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW\x82\x01`\x80`#\x19\x826\x03\x01\x12a\x03\nW`@Q\x90a&6\x82a\x03BV[a&C6`$\x83\x01a\x04*V[\x82R`d\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa&h\x90`$6\x91\x84\x01\x01a\t\xE0V[` \x83\x01R`\x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW`$\x91\x01\x016`\x1F\x82\x01\x12\x15a\x03\nW\x805a&\x9B\x81a\t\xC9V[\x91a&\xA9`@Q\x93\x84a\x03xV[\x81\x83R` \x80\x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03\nW` \x01\x91[\x81\x83\x10a&\xE2WPPP`@\x82\x01R\x81R` \x95\x86\x01\x95\x01a%\xF8V[\x825`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\nW\x81R` \x92\x83\x01\x92\x01a&\xC5V[4a\x03\nW` a'@a\x0CTa'\x186a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA1\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[`\x01`\x01`@\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a'n\x81a\x02\xF9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW6`#\x83\x01\x12\x15a\x03\nW\x81`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW6`$\x83\x85\x01\x01\x11a\x03\nW`$a\x05}\x93\x01\x90a;6V[4a\x03\nW` `\x01`\x01`@\x1B\x03a'\xFBa'\xD36a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a(\"\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa(A\x906\x90`\x04\x01a\x1D\xE9V[\x90a(Sa\r\xFE`\x04\x80`fT\x16\x14\x90V[a(_a\x07\x8D\x82a?\xB1V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\xAEWa(\xC7\x91_\x91a\x1B\\WPa6.V[` \x82\x01\x90`\x01`\x01`\xA0\x1B\x03\x81\x16_[a(\xE2\x84\x86a1@V[\x90P\x81\x10\x15a*\x03W\x80a)\xFDa)\xF0\x87a)Ia\x08:a)\x15a\x07\xAC`\x01\x98a\x08\xDB\x8Da)\x0F\x88a1\x85V[\x97a1@V[a) a\x0E*a\x03\x99V[c\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81R\x83Qa\x0E\xCC\x90a\x08+\x90a\x0E\xC1\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xA8V[a)ba)]a)Y\x83\x8Aa@\xB0V[\x15\x90V[a;\x8AV[`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x9C` R`@\x90 a)\x86\x90a\x084\x83a=~V[Pa)\x9C\x87a)\x97a\x1F\x99\x84a=~V[a@FV[P\x85\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^`@Q\x80a)\xCD\x85\x82a1\x1AV[\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x9E` R`@\x90 a\x1F:\x90a /V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x01a(\xD8V[\x84\x83a*/\x86a*\"a*\x1Ba\x08\xE6a\r\x9A\x87a1\x85V[\x91\x85a1@V[\x92\x90\x94`@\x81\x01\x90a4\x96V[\x82\x95\x91\x95;\x15a\x03\nW_\x94a*]\x86\x92`@Q\x98\x89\x97\x88\x96\x87\x95c\xAD\xCFs\xF7`\xE0\x1B\x87R`\x04\x87\x01a;\xA0V[\x03\x92Z\xF1\x80\x15a\x05\xAEWa \xC7W\0[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa*\x8Aa\x16X6a\x03\xF3V[_R`\x9A` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\nWa\x08:a*\xFCa*\xB46a\x18\xA4V[\x93\x91a*\xC5a\x07\x8D\x85\x97\x93\x97a?\xB1V[a\x0E\xCCa\x08+a\x0E\xC1`@Q\x96a*\xDB\x88a\x03\"V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x88Rc\xFF\xFF\xFF\xFF\x90\x94\x16` \x88\x01\x90\x81R\x93a\x0E\xA8V[a+\x05\x81a=~V[\x90_[\x83\x81\x10a+\x11W\0[`\x01\x90a+=a\x0F\xD7a+,\x86_R`\x99` R`@_ \x90V[a\x1F\xB0a\x08\xE6a\x08\xE1\x86\x8B\x8Da1uV[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEEa+oa\x19\xD0a\x08\xE1\x84\x89\x8Ba1uV[\x03\x90\xA1\x01a+\x08V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`@c\xFF\xFF\xFF\xFFa+\xA4`\x045a+\x9F\x81a\x02\xF9V[a<\x12V[\x83Q\x91\x15\x15\x82R\x90\x91\x16` \x82\x01R\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a+\xD2\x81a\x02\xF9V[`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a,\r\x81a\x02\xF9V[a,\x166a\x03\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` Ra,3`@_ \x91a=~V[_R` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a,kWa\x06q\x85a\x16\x97\x81\x87\x03\x82a\x03xV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,TV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a,\x9E\x81a\x02\xF9V[a,\xE3`$5_T\x92a,\xC9`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a-aW[\x81\x15a-AW[Pa<\xC7V[\x83a,\xDA`\x01`\xFF\x19_T\x16\x17_UV[a-*Wa=*V[a,\xE9W\0[a,\xF7a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x14\x8FV[a-<a\x01\0a\xFF\0\x19_T\x16\x17_UV[a=*V[0;\x15\x91P\x81a-SW[P_a,\xC3V[`\xFF\x16`\x01\x14\x90P_a-LV[`\x01`\xFF\x82\x16\x10\x91Pa,\xBCV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85`@`\x045a-\xAF\x81a\x02\xF9V[a-\xFD`$5\x91a-\xBF\x83a\x02\xF9V[a-\xCBa\x07\x8D\x82a?\xB1V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x97` R\x86\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x92\x16\x94\x90\x94\x17\x90Ua3\x1DV[\x82Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R\xA1\0[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a.t\x81a\x02\xF9V[a.|aH\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a.\x94Wa\x05}\x90aIIV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05\xAEW_\x91a/qW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a/bWa\x05}\x90a=;V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a/\xA4W[\x81a/\x8C` \x93\x83a\x03xV[\x81\x01\x03\x12a\x03\nWQa/\x9E\x81a\x02\xF9V[_a/IV[=\x91Pa/\x7FV[`@Q\x90a/\xB9\x82a\x03BV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[a\x04\xB6a/\xDA\x93\x92a\x04\xB0a/\xACV[\x90P\x90V[\x90\x81` \x91\x03\x12a\x03\nWQ\x80\x15\x15\x81\x03a\x03\nW\x90V[`@Q=_\x82>=\x90\xFD[\x15a0\tWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a0\x1FWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a0;\x82a\x03\"V[_` \x83\x82\x81R\x01RV[\x90a0P\x82a\t\xC9V[a0]`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a0n`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a0~WPPPV[` \x90a0\x89a0.V[\x82\x82\x85\x01\x01R\x01a0rV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a0\xBDW` \x91`\x05\x1B\x01\x01\x90V[a0\x95V[\x15a0\xC9WV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a0\xBDW`\x05\x1B\x81\x015\x90`>\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x90V[5a\x06\x19\x81a\x03\xAAV[\x15a1\x0BWV[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x91\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R`@\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03\nWV[\x91\x90\x81\x10\x15a0\xBDW`\x05\x1B\x01\x90V[5a\x06\x19\x81a\x02\xF9V[\x90a1\x99\x82a\t\xC9V[a1\xA6`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a1\xB7`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a1\xC7WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a1\xBBV[` \x81\x83\x03\x12a\x03\nW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03\nW\x81Q\x90a2\x0C\x82a\t\xC9V[\x92a2\x1A`@Q\x94\x85a\x03xV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x90\x82\x82\x11a\x03\nW` \x81\x01\x93[\x82\x85\x10a2FWPPPPP\x90V[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW\x82\x01\x84`?\x82\x01\x12\x15a\x03\nW` \x81\x01Q\x90a2r\x82a\t\xC9V[\x91a2\x80`@Q\x93\x84a\x03xV[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x87\x83\x11a\x03\nW`@\x01\x90[\x82\x82\x10a2\xB6WPPP\x81R` \x94\x85\x01\x94\x01a27V[\x81Q\x81R` \x91\x82\x01\x91\x01a2\x9EV[\x90\x91a2\xDDa\x06\x19\x93`@\x84R`@\x84\x01\x90a \xEFV[\x91` \x81\x84\x03\x91\x01Ra\x15\xEEV[\x90a2\xF5\x82a\t\xC9V[a3\x02`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a3\x13`\x1F\x19\x91a\t\xC9V[\x01\x90` 6\x91\x017V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x97` R`@\x90 T\x90\x91\x16\x80a/\xDAWP\x90V[\x15a3IWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a3_WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a3uWV[c\xEB\xBF\xF4\x97`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a%qWV[\x15a3\xADWV[c\x9F\x1C\x80S`\xE0\x1B_R`\x04_\xFD[\x15a3\xC3WV[c\x13S`1`\xE0\x1B_R`\x04_\xFD[\x15a3\xD9WV[c1\xBC4'`\xE1\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a%qWV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a%qW_\x03\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a%qWV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x82\x16` \x80\x83\x01\x91\x90\x91R\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`@\x84\x01R\x92\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x80\x83\x01R\x91\x90\x91\x16`\xA0\x82\x01R`\xC0\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW` \x01\x91\x816\x03\x83\x13a\x03\nWV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x90\x95\x94\x92\x93\x91\x80`\xC0\x83\x01`\xC0``\x85\x01RR`\xE0\x82\x01\x96\x90_[\x81\x81\x10a5\x82WPPP\x80\x86\x03`\x80\x82\x01R` \x80\x85Q\x97\x88\x81R\x01\x94\x01_\x96[\x80\x88\x10a5jWPPa\x06\x19\x94\x95P`\xA0\x81\x85\x03\x91\x01Ra4\xC8V[\x90\x94` \x80`\x01\x92\x88Q\x81R\x01\x96\x01\x97\x01\x96\x90a5NV[\x90\x91\x97` a5\xA9`\x01\x92\x8B5a5\x98\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90V[\x99\x01\x92\x91\x01a5-V[\x90a5\xBD\x82a\t\xC9V[a5\xCA`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a5\xDB`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a5\xEBWPPPV[` \x90a5\xF6a/\xACV[\x82\x82\x85\x01\x01R\x01a5\xDFV[\x15a6\tWV[cX\\\xFB/`\xE0\x1B_R`\x04_\xFD[\x15a6\x1FWV[cH\xF5\xC3\xED`\xE0\x1B_R`\x04_\xFD[\x15a65WV[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[\x90`@Qa6Q\x81a\x03BV[`@c\xFF\xFF\xFF\xFF\x82\x94T`\x01`\x01`@\x1B\x03\x81\x16\x84R\x80\x83\x1C`\x0F\x0B` \x85\x01R`\xC0\x1C\x16\x91\x01RV[\x15a6\x82WV[c%\x13\x1DO`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a%qWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a%qWV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a6\xDCWPPP\x90V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x875a6\xF5\x81a\x03\xAAV[\x16\x81R\x01\x94\x01\x92\x91\x01a6\xCFV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x06\x19\x93\x91\x01\x91a6\xC3V[a7V\x91a7:a\r\xFE`\x01\x80`fT\x16\x14\x90V[a7Fa\x1B\x13\x83a?\xB1V[a7O\x82a<\x12V[\x93\x90a:\xAFV[_\x92[\x81Q\x84\x10\x15a:\xA9Wa7\x8B` a7q\x86\x85a0\xA9V[Q\x01QQ`@a7\x81\x87\x86a0\xA9V[Q\x01QQ\x14a3XV[a7\x95\x84\x83a0\xA9V[QQ\x90a7\xC5a\x08:a7\xB1a\x0E\xA8\x85Q`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x0E\xCCa\x08+` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a7\xCF\x82\x85a@\xB0V[_[` a7\xDD\x88\x87a0\xA9V[Q\x01QQ\x81\x10\x15a:\x9BW\x80\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x81\x88a9\xBC\x8Ba9x\x89a8\x85\x8Da9r\x8E\x8Da8:a\x0B\xC9`\x01\x9F` a82\x8C\x89a0\xA9V[Q\x01Qa0\xA9V[\x97\x88\x94a8G\x86\x8DaDjV[a8U\x86\x8Da\x04\xB6\x87a=~V[\x9D\x90\x8E\x99\x81\x99\x8B\x96a8}a8wa8q` \x8B\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x15a:\xC5V[\x87\x8B\x8AaI\x91V[\x90a8\xC0a8\xB6a8\x9D\x89Q`\x01`\x01`@\x1B\x03\x16\x90V[a8\xB0a\x0C\x9D\x88`@a82\x8D\x8Da0\xA9V[\x90aI\xF0V[`\x0F\x0B` \x89\x01RV[a8\xDBa8\xD4a8q` \x8A\x01Q`\x0F\x0B\x90V[\x15\x15a:\xDBV[` \x87\x01Q`\x0F\x0B\x80_\x81\x12\x15a:\x1CWPPP_\x14a9\xC5WPPPPPa9*a9\x1B\x84a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[a9$\x83a=~V[\x90aJ!V[a\x16Xa9ea9`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a6\xA9V[a6\x91V[c\xFF\xFF\xFF\xFF\x16`@\x87\x01RV[\x87aAIV[a9\x89a9\x84\x8Ba=~V[a?\x87V[\x93a\x13y`@a9\xB0a9\xA3\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x85\x01Q`\x0F\x0Ba\x0C\xFFV[\x92\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x03\x90\xA1\x01a7\xD1V[a82a\x10\xEF\x94a\x0C\x9D\x94a:\0a:\x05\x98a\x11\x15` `@\x97\x01\x91a\x0C\xFF` a9\xF7\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x01Q`\x0F\x0B\x90V[a0\xA9V[_` \x86\x01RCc\xFF\xFF\xFF\xFF\x16`@\x86\x01Ra=~V[\x94P\x95P\x95PPP_\x91P\x13a:4W[PPa=~V[a:\x94\x91a:\x88a:ca9e\x93a\x0C\x9Da\r\x05` \x8C\x01\x92a:^\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[a@\x95V[`\x01`\x01`@\x1B\x03a:\x7Fa\x04`\x8BQ`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x16\x11\x15a:\xF1V[c\xFF\xFF\xFF\xFFC\x16a6\xA9V[\x8E_a:-V[PP\x93`\x01\x91P\x01\x92a7YV[PPPPV[\x15a:\xB6WV[c\xFAU\xFC\x81`\xE0\x1B_R`\x04_\xFD[\x15a:\xCCWV[c\r\x8F\xCB\xE3`\xE4\x1B_R`\x04_\xFD[\x15a:\xE2WV[cF\x06\x17\x93`\xE1\x1B_R`\x04_\xFD[\x15a:\xF8WV[cl\x9B\xE0\xBF`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R a\x06\x19\x90a@YV[\x90\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x91a;ea\x07\x8D\x82a?\xB1V[a;\x85`@Q\x92\x83\x92` \x84R`\x01\x80`\xA0\x1B\x03\x16\x95` \x84\x01\x91a4\xC8V[\x03\x90\xA2V[\x15a;\x91WV[clln'`\xE1\x1B_R`\x04_\xFD[\x93\x91a\x06\x19\x95\x93a;\xC6\x92`\x01\x80`\xA0\x1B\x03\x16\x86R``` \x87\x01R``\x86\x01\x91a6\xC3V[\x92`@\x81\x85\x03\x91\x01Ra4\xC8V[\x90`@Qa;\xE1\x81a\x03]V[``c\xFF\xFF\xFF\xFF\x82\x94T\x81\x81\x16\x84R`\xFF\x81` \x1C\x16\x15\x15` \x85\x01R\x81\x81`(\x1C\x16`@\x85\x01R`H\x1C\x16\x91\x01RV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90```@Q\x92a<5\x84a\x03]V[Ta<\x8Da<\x83a<}c\xFF\xFF\xFF\xFF\x84\x16\x80\x88R`\xFF\x85` \x1C\x16\x15\x15\x97\x88` \x82\x01Rc\xFF\xFF\xFF\xFF\x80\x87`(\x1C\x16\x96\x87`@\x84\x01R`H\x1C\x16\x96\x87\x91\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x95\x15\x15\x90V[\x92c\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x90\x81a<\xB6W[Pa<\xA7WP\x91\x90V[\x91\x92PPc\xFF\xFF\xFF\xFF\x16`\x01\x91V[c\xFF\xFF\xFF\xFF\x16\x90PC\x10\x15_a<\x9DV[\x15a<\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a=6a\x03\xA8\x92a?UV[aIIV[a=L`fT\x19\x82\x19\x81\x16\x14a0\x18V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra=\xC4`@\x82a\x03xV[Q\x90Q\x90` \x81\x10a=\xD4WP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x92\x91a\x1D\xA8a>\x91\x91a=\xF3a0.V[Pa=\xFCa/\xACV[Pa\x0C?a>!a\x0CT\x83a\x0C?\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[\x94a\x1D\x9Ba>Fa\x1C\xC4\x85a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x98a>aa>Ra\x03\x99V[`\x01`\x01`@\x1B\x03\x90\x99\x16\x89RV[a>x` \x89\x01\x9A\x8B\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA0` R`@\x90 \x90V[\x92`@\x84\x01\x90a>\xA8a\x08+\x83Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10a?\x1BW_\x80\x92a>\xC2\x87Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92a>\xE6a>\xD9` \x8A\x01\x95a\x0C\xFF\x87Q`\x0F\x0B\x90V[`\x01`\x01`@\x1B\x03\x16\x89RV[\x83Q`\x0F\x0B\x90\x83\x82\x12a>\xFCW[PPRR\x91\x90V[a\x11\x15a?\x14\x92a:^\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[_\x80a>\xF4V[PP\x91\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[a?\x8Fa0.V[Pc\xFF\xFF\xFF\xFF`@Q\x91a?\xA2\x83a\x03\"V[\x80``\x1C\x83R\x16` \x82\x01R\x90V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x05\xAEW_\x91a@-WP\x90V[a\x06\x19\x91P` =` \x11a\x05\xA7Wa\x05\x99\x81\x83a\x03xV[a\x06\x19\x91`\x01`\x01`\xA0\x1B\x03\x16\x90aJ\x9EV[\x80T\x80a@oWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[\x80_\x19\x81\x01\x11a%qW`\x01`\x01`@\x1B\x03\x91_R_\x19\x90` _ \x01\x01T` \x1C\x16\x90V[`\x01`\x01`@\x1B\x03\x91\x82a@\xAC\x92\x16`\x0F\x0Ba4\x1FV[\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9E` R`@\x90 \x90a@\xD0\x90a=~V[_R` R` `@_ `@Q\x90a@\xE8\x82a\x03\"V[T\x90c\xFF\xFF\xFF\xFF`\xFF\x83\x16\x15\x15\x92\x83\x83R`\x08\x1C\x16\x92\x83\x91\x01R\x15\x15\x90\x81\x15aA\x0FWP\x90V[c\xFF\xFF\xFF\xFF\x16C\x11\x15\x91\x90PV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90aA2\x81\x84aK\xCCV[\x92\taA;W\x90V[`\x01\x81\x01\x80\x91\x11a%qW\x90V[\x93\x90\x92` aAoa\x1C\xC4\x85a\x0C?\x89`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x91\x01\x80Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x92\x16\x82\x90\x03aC\x12W[PPaB\x10\x81aA\xB0\x84a\x0C?\x87a\x1D\x9B\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA0` R`@_ \x90V[\x81Q\x81T` \x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x91\x90\x91\x1B\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x95\x1B\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x91\x90\x91\x17\x90UV[` \x81\x01Q`\x0F\x0B\x15aBzWP\x82aBXaBr\x92aBG\x85a\x1D\x9BaBw\x98`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90aJ\x9EV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9D` R`@\x90 \x90V[aJ\x9EV[PV[Q`\x01`\x01`@\x1B\x03\x16\x15aB\x8EWPPPV[aB\xC3\x90a\x1F\x83a\x08\xE6aB\xB6\x85a\x1D\x9B\x88`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[\x92`\x01`\x01`\xA0\x1B\x03\x16\x90V[P`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x90 aB\xE6\x90\x82\x90a\x1D\x9BV[T\x15aB\xF0WPPV[aC\raBw\x92`\x01\x80`\xA0\x1B\x03\x16_R`\x9D` R`@_ \x90V[aM\xECV[aCwaC\xA8\x91a\x0C\x9D\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x94aC\\\x88a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x90`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x82R\x87\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1_\x80aA\x88V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R \x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x93aDK\x91aD\x18\x90`\x01`\x01`@\x1B\x03\x83\x16\x90c\xFF\xFF\xFF\xFFC\x16\x90aN\xFBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x81\x90``\x82\x01\x90V[\x03\x90\xA1V[_\x19\x81\x14a%qW`\x01\x01\x90V[\x80\x15a%qW_\x19\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x93\x92\x91\x90aD\x9E\x90a\x1C\xF3V[\x93[\x84\x15\x15\x80aEWW[\x15aEPWaD\xD4aD\xCF\x84a\x0C?\x85`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aL}V[aD\xDF\x84\x82\x85a=\xE2V[\x91aD\xF4a\x08+`@\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10aEFWaE:\x92aE@\x94\x92\x87aE\x0E\x93\x88aAIV[aE4aE/\x86a\x0C?\x87`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aL\xBEV[PaDPV[\x94aD^V[\x93aD\xA0V[PPPPPP\x90PV[PPP\x90PV[Pa\xFF\xFF\x81\x10aD\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x94\x90aE\x94\x90a\x1C\xF3V[\x94[\x85\x15\x15\x80aF?W[\x15aF7WaE\xC5aD\xCF\x85a\x0C?\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aE\xD0\x85\x82\x86a=\xE2V[\x91aE\xE5a\x08+`@\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10aF,WaF \x92aF&\x94\x92\x88aE\xFF\x93\x89aAIV[aE4aE/\x87a\x0C?\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[\x95aD^V[\x94aE\x96V[PPPP\x93PPPPV[P\x93PPPPV[Pa\xFF\xFF\x85\x16\x81\x10aE\x9FV[\x90\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91aDKaG\x99aF\x97aF\x92\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90V[a;\xD4V[a\x08?``\x82\x01\x91aF\xAD\x83Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x90\x81aG\xF4W[PaG\xCAW[c\xFF\xFF\xFF\xFF\x87\x16`@\x82\x01RaG\x0FaG\x05a9`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a6\xA9V[c\xFF\xFF\xFF\xFF\x16\x84RV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9B` R`@\x90 \x81Q\x81T` \x80\x85\x01Q`@\x86\x01Q``\x90\x96\x01Ql\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0`H\x91\x90\x91\x1B\x16h\xFF\xFF\xFF\xFF\0\0\0\0\0`(\x97\x90\x97\x1B\x96\x90\x96\x16d\xFF\0\0\0\0\x91\x15\x15\x90\x92\x1B\x16l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x85\x01R\x93\x16\x92\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[aG\xE8aG\xDE`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x82RV[`\x01` \x82\x01RaF\xC3V[c\xFF\xFF\xFF\xFF\x16\x90PC\x10\x15_aF\xBDV[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a%qWV[\x80T\x90\x91`\x01`\x01`\xFF\x1B\x03\x81\x11aH\x88Wa8qaHMaHHaHT\x93\x85`\x0F\x0BaH\x05V[aM.V[\x92`\x80\x1D\x90V[\x81`\x0F\x0B\x12\x15aHyW`\x01aHu\x92\x01\x90`\x0F\x0B_R` R`@_ \x90V[T\x90V[c-\x04\x83\xC5`\xE2\x1B_R`\x04_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x90\xFD[a\x06\x19\x91`\x01`\x01`\xA0\x1B\x03\x16\x90aM\xECV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aI\x05WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90aI\x9EaI\xC6\x92a=~V[_R`\x99` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[\x91\x82aI\xE8W[P\x81aI\xD7WP\x90V[`\x01`\x01`@\x1B\x03\x91PQ\x16\x15\x15\x90V[\x91P_aI\xCDV[`\x01`\x01`@\x1B\x03\x80\x91\x16`\x0F\x0B\x91\x16`\x0F\x0B\x03`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17a%qW\x90V[\x90\x81T`\x80\x1D\x90aJ@\x82`\x01\x85\x01\x90`\x0F\x0B_R` R`@_ \x90V[U\x81T`\x01`\x01`\x80\x1B\x03\x16`\x01\x90\x91\x01`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[\x80T\x82\x10\x15a0\xBDW_R` _ \x01\x90_\x90V[\x91aJ\x9A\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[_\x82\x81R`\x01\x82\x01` R`@\x90 TaK\x01W\x80T\x90`\x01`@\x1B\x82\x10\x15a\x03=W\x82aJ\xECaJ\xD6\x84`\x01\x80\x96\x01\x85U\x84aJlV[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80T\x92_R\x01` R`@_ U`\x01\x90V[PP_\x90V[\x81\x15aK\x11W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\x03\nWV[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aK\xC0Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aKl\x86\x84\x11aK%V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x06\x19\x92PaK\x07V[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL5W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03\nW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aLpW\x90\x82\x91aKl\x86\x84\x11aK%V[PP\x90a\x06\x19\x92PaK\x07V[aL\x92\x81T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[aL\xAFW\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[c\x1E\xD9P\x95`\xE1\x1B_R`\x04_\xFD[\x90aL\xD4\x82T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[aL\xAFW\x81T`\x0F\x0B\x91`\x01\x81\x01\x92_aM\x0F\x82aL\xFD\x81\x88\x90`\x0F\x0B_R` R`@_ \x90V[T\x96\x90`\x0F\x0B_R` R`@_ \x90V[U`\x01`\x01`\x01`\x80\x1B\x03\x19\x83T\x16\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x17\x90UV[`\x01`\x01`\x7F\x1B\x03\x19\x81\x12\x15\x80aM\xA0W[\x15aMKW`\x0F\x0B\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[P`\x01`\x01`\x7F\x1B\x03\x81\x13\x15aM@V[\x80T\x80\x15aM\xD8W_\x19\x01\x90aM\xC7\x82\x82aJlV[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aN\x87W_\x19\x84\x01\x84\x81\x11a%qW\x83T_\x19\x81\x01\x94\x90\x85\x11a%qW_\x95\x85\x83a\x1D\x9B\x94aN:\x98\x03aN@W[PPPaM\xB1V[U`\x01\x90V[aNpaNj\x91aNaaNWaN~\x95\x88aJlV[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91\x87aJlV[\x90aJ\x81V[\x85\x90_R` R`@_ \x90V[U_\x80\x80aN2V[PPPP_\x90V[\x15aN\x96WV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03=WaN\xC2\x91`\x01\x82\x01\x81UaJlV[aN\xE8W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aO7W[PaO2a\x03\xA8\x93aO\"aO\x16a\x03\x99V[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aN\xA5V[\x80_\x19\x81\x01\x11a%qW\x81_Rc\xFF\xFF\xFF\xFFaO\x92a\x08+_\x19\x84` _ \x01\x01a\x08?aO\x84`@Q\x92aOk\x84a\x03\"V[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aN\x8FV[\x03aO\x03Wa\x03\xA8\x93\x92P\x90a%\x1CaO\xAA\x92a3\x98V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV\xFE\xA2dipfsX\"\x12 \xDE\x0E\xEAR\x98\xEB^\x93\xBB\x99\x91\x89\x9E\x83\xB6\x1E\x9Dn\xBF\x05\xE0\x96\x99\xF9pB\x9FKA\x94cIdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c806310e1b9b8146102f4578063136439dd146102ef57806315fe5028146102ea578063260dc758146102e5578063261f84e0146102e05780632981eb77146102db5780632bab2c4a146102d6578063304c10cd146102d157806336352057146102cc57806340120dab146102c75780634177a87c146102c25780634657e26a146102bd5780634a10ffe5146102b85780634b5046ef146102b357806350feea20146102ae578063547afb87146102a957806356c483e6146102a4578063595c6a671461029f5780635ac86ab71461029a5780635c975abb14610295578063670d3ba2146102905780636cfb44811461028b5780636e3492b5146102865780636e875dba14610281578063715018a61461027c57806379ae50cd146102775780637bc1ef6114610272578063886f11951461026d5780638ce64854146102685780638da5cb5b1461026357806394d7d00c1461025e578063952899ee14610259578063a9333ec814610254578063a98218211461024f578063a984eb3a1461024a578063adc2e3d914610245578063b2447af714610240578063b66bd9891461023b578063b9fbaed114610236578063ba1a84e514610231578063c221d8ae1461022c578063cd6dc68714610227578063d3d96ff414610222578063df5cf7231461021d578063f2fde38b146102185763fabc1cbc14610213575f80fd5b612ee8565b612e57565b612e13565b612d6f565b612c81565b612bf0565b612bb5565b612b78565b612aa0565b612a6d565b612805565b6127b8565b612751565b612702565b612584565b61240e565b6123e6565b612331565b6122dc565b61229c565b612210565b6121b5565b61213c565b611df7565b611c95565b611c3e565b611c21565b611bee565b611b7b565b611a9b565b611a0d565b6118ec565b6117dc565b61173f565b6116b9565b61163b565b611541565b610db1565b610d78565b610ad1565b610989565b610744565b6106b8565b61061c565b6104f3565b61046c565b6001600160a01b0381160361030a57565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761033d57604052565b61030e565b606081019081106001600160401b0382111761033d57604052565b608081019081106001600160401b0382111761033d57604052565b90601f801991011681019081106001600160401b0382111761033d57604052565b604051906103a8604083610378565b565b63ffffffff81160361030a57565b604090602319011261030a57604051906103d182610322565b816024356103de816102f9565b81526020604435916103ef836103aa565b0152565b604090600319011261030a576040519061040c82610322565b81600435610419816102f9565b81526020602435916103ef836103aa565b919082604091031261030a5760405161044281610322565b60208082948035610452816102f9565b84520135916103ef836103aa565b6001600160401b031690565b3461030a57608036600319011261030a5760606104bc60043561048e816102f9565b610497366103b8565b906104b6606435926104a8846102f9565b6104b0612fac565b50613d7e565b90613de2565b90506104f1604051809263ffffffff604080926001600160401b0381511685526020810151600f0b6020860152015116910152565bf35b3461030a57602036600319011261030a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156105ae5761057d92610569915f9161057f575b50613002565b61057860665482811614613018565b613f55565b005b6105a1915060203d6020116105a7575b6105998183610378565b810190612fdf565b5f610563565b503d61058f565b612ff7565b90602080835192838152019201905f5b8181106105d05750505090565b90919260206040826105fd600194885163ffffffff6020809260018060a01b038151168552015116910152565b0194019291016105c3565b9060206106199281815201906105b3565b90565b3461030a57602036600319011261030a57600435610639816102f9565b6001600160a01b03165f818152609d602052604090205461065981613046565b915f5b82811061067557604051806106718682610608565b0390f35b600190825f52609d60205261069c6106908260405f20614a6c565b90549060031b1c613f87565b6106a682876130a9565b526106b181866130a9565b500161065c565b3461030a57604036600319011261030a57602061070a6106d7366103f3565b80516001600160a01b03165f9081526098845260408082209285015163ffffffff16825260019092016020522054151590565b6040519015158152f35b9181601f8401121561030a578235916001600160401b03831161030a576020808501948460051b01011161030a57565b3461030a57604036600319011261030a57600435610761816102f9565b6024356001600160401b03811161030a57610780903690600401610714565b9061079261078d84613fb1565b6130c2565b5f915b80831061079e57005b6107b16107ac8483856130d8565b6130fa565b907f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c61087a6108496107e1610399565b6001600160a01b03891681529463ffffffff16602086019081526001600160a01b0389165f90815260986020526040902061083f9061083a9061083461082b855163ffffffff1690565b63ffffffff1690565b90614a9e565b613104565b5163ffffffff1690565b61086e610854610399565b6001600160a01b038a1681529163ffffffff166020830152565b6040519182918261311a565b0390a161088682613d7e565b925f5b6108a16108978785856130d8565b6020810190613140565b905081101561096f57806108f8876108f26108e66108e16001966108db6108976108d38e5f52609960205260405f2090565b968c8c6130d8565b90613175565b613185565b6001600160a01b031690565b90614046565b507f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b6109666109326108e1846108db6108978d8b8b6130d8565b6040805189516001600160a01b0390811682526020808c015163ffffffff1690830152909216908201529081906060820190565b0390a101610889565b5060019094019392509050610795565b5f91031261030a57565b3461030a575f36600319011261030a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b6001600160401b03811161033d5760051b60200190565b9080601f8301121561030a5781356109f7816109c9565b92610a056040519485610378565b81845260208085019260051b82010192831161030a57602001905b828210610a2d5750505090565b602080918335610a3c816102f9565b815201910190610a20565b602081016020825282518091526040820191602060408360051b8301019401925f915b838310610a7957505050505090565b9091929394603f19828203018352855190602080835192838152019201905f905b808210610ab95750505060208060019297019301930191939290610a6a565b90919260208060019286518152019401920190610a9a565b3461030a5760a036600319011261030a57610aeb366103f3565b6044356001600160401b03811161030a57610b0a9036906004016109e0565b906064356001600160401b03811161030a57610b2a9036906004016109e0565b90608435610b37816103aa565b610b41845161318f565b90604051637870733b60e11b81525f8180610b60888a600484016132c6565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156105ae575f91610d56575b5063ffffffff91909116905f670de0b6b3a7640000945b8751821015610d4857610bd6610bc9838a6130a9565b516001600160a01b031690565b92610be188516132eb565b610beb84886130a9565b52610bf683876130a9565b505f5b888051821015610d3a57908386610c15610bc9846001966130a9565b8b6001600160401b03610c59610c5484610c3f8760018060a01b03165f5260a160205260405f2090565b9060018060a01b03165f5260205260405f2090565b614059565b16918215610d3057610460610c78610caa92610cc997610caf97612fca565b8d63ffffffff610c8f604084015163ffffffff1690565b16111580610d17575b610cdf575b516001600160401b031690565b614c46565b610cc383610cbd89886130a9565b516130a9565b51614bcc565b610cd782610cbd888c6130a9565b525b01610bf9565b610d12610d05610cf683516001600160401b031690565b6020840151600f0b5b90614095565b6001600160401b03168252565b610c9d565b505f610d276020830151600f0b90565b600f0b12610c98565b5050505050610cd9565b505092509060010190610bb3565b604051806106718782610a47565b610d7291503d805f833e610d6a8183610378565b8101906131d8565b5f610b9c565b3461030a57602036600319011261030a576020610d9f600435610d9a816102f9565b61331d565b6040516001600160a01b039091168152f35b3461030a57604036600319011261030a57600435610dce816102f9565b6024356001600160401b03811161030a5760a0600319823603011261030a57610e04610dfe600280606654161490565b15613342565b610e1061078d83613fb1565b610ee5610e1f602483016130fa565b610e39610e2a610399565b6001600160a01b039095168552565b63ffffffff1660208401908152610ee061083a610e6186610e5c87600401613185565b6140b0565b92610e90610e756044880188600401613140565b9050610e876064890189600401613140565b91905014613358565b8651610ecc9061082b90610ec1906001600160a01b03165b6001600160a01b03165f90815260986020526040902090565b925163ffffffff1690565b906001915f520160205260405f2054151590565b61336e565b610eff610ef86044830183600401613140565b90506132eb565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031691905f5b610f3d6044840184600401613140565b905081101561142957801580156113c7575b610f58906133a6565b610f6c816108db6064860186600401613140565b3515158061139c575b610f7e906133bc565b610fdc610fd7610f9e610f9088613d7e565b5f52609960205260405f2090565b610fb86108e66108e1866108db60448b018b600401613140565b6001600160a01b03165f90815260019091016020526040902054151590565b6133d2565b611011610feb84600401613185565b610ff487613d7e565b61100b6108e1856108db60448a018a600401613140565b91613de2565b919061102761046084516001600160401b031690565b8015611391576110706104607f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd92611069866108db60648c018c600401613140565b359061411d565b9361112261108584516001600160401b031690565b956110a26001600160401b0388166001600160401b038316614b2c565b6110ac878a6130a9565b526110d76110ca826110c586516001600160401b031690565b6133e8565b6001600160401b03168452565b6110fc6110ef826110c588516001600160401b031690565b6001600160401b03168652565b61111560208601916110c583516001600160401b031690565b6001600160401b03169052565b866111316020830151600f0b90565b855f61113d83600f0b90565b126112b9575b8391508b836111796108e189946108db61116b61116561117f9a600401613185565b96613d7e565b946044810190600401613140565b91614149565b61118b87600401613185565b906111cd6111b76111a96108e1886108db60448e0160048f01613140565b92516001600160401b031690565b926040519384938d63ffffffff43169386613445565b0390a16112076111df86600401613185565b6111f66108e1856108db60448b018b600401613140565b83516001600160401b0316916143b2565b61121385600401613185565b9061123c61122e6108e1856108db60448b018b600401613140565b91516001600160401b031690565b91873b1561030a5760405163601bb36f60e01b81526001600160a01b039182166004820152911660248201526001600160401b039384166044820152921660648301525f8260848183895af19182156105ae5760019261129f575b505b01610f2d565b806112ad5f6112b393610378565b8061097f565b5f611297565b6112fc6104606104606113229561106961130b956108db6112ee6112e26112e26113189b613408565b6001600160801b031690565b936064810190600401613140565b6001600160801b0316600f0b90565b6020840151600f0b61341f565b600f0b6020830152565b81898861138661134c6108e1896108db61133e86600401613185565b956044810190600401613140565b61136d61136087516001600160401b031690565b6020880151600f0b610cff565b604087015163ffffffff165b9160405195869586613445565b0390a1865f85611143565b505060019150611299565b50610f7e670de0b6b3a76400006113bd836108db6064880188600401613140565b3511159050610f75565b50610f586113e86108e66108e66108e1856108db60448a018a600401613140565b6114166108e66108e66108e66108e161140760448b018b600401613140565b6114108a613398565b91613175565b6001600160a01b03909116119050610f4f565b7f80969ad29428d6797ee7aad084f9e4a42a82fc506dcd2ca3b6fb431f85ccebe5858361148f6114808761145f81600401613185565b936114706044830183600401613140565b9390926084810190600401613496565b939092604051978897886134e8565b0390a1005b604090600319011261030a576004356114ac816102f9565b90602435610619816102f9565b90602080835192838152019201905f5b8181106114d65750505090565b9091926020606082611511600194885163ffffffff604080926001600160401b0381511685526020810151600f0b6020860152015116910152565b0194019291016114c9565b9091611533610619936040845260408401906105b3565b9160208184039101526114b9565b3461030a5761154f36611494565b6001600160a01b0382165f818152609d602052604090205490929161157382613046565b9261157d836135b3565b945f5b848110611596576040518061067189898361151c565b600190825f52609d6020526115d2856115b56106908460405f20614a6c565b806115c0858c6130a9565b526115cb848b6130a9565b5086612fca565b6115dc828a6130a9565b526115e781896130a9565b5001611580565b90602080835192838152019201905f5b81811061160b5750505090565b82516001600160a01b03168452602093840193909201916001016115fe565b9060206106199281815201906115ee565b3461030a57604036600319011261030a5761165d611658366103f3565b613d7e565b5f52609960205260405f206040519081602082549182815201915f5260205f20905f5b8181106116a3576106718561169781870382610378565b6040519182918261162a565b8254845260209093019260019283019201611680565b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b60206040818301928281528451809452019201905f5b8181106117205750505090565b82516001600160401b0316845260209384019390920191600101611713565b3461030a57604036600319011261030a576004356001600160401b03811161030a5761176f9036906004016109e0565b6024359061177c826102f9565b61178681516132eb565b915f5b82518110156117ce576001906117b2836001600160a01b036117ab84886130a9565b5116613b07565b6001600160401b036117c483886130a9565b9116905201611789565b6040518061067186826116fd565b3461030a57606036600319011261030a576004356117f9816102f9565b6024356001600160401b03811161030a57611818903690600401610714565b90916044356001600160401b03811161030a57611839903690600401610714565b92909361184d610dfe600180606654161490565b611858848314613358565b5f5b82811061186357005b61186e818484613175565b3590611879826102f9565b611884818789613175565b359161ffff8316830361030a5760019261189e9187614562565b0161185a565b606060031982011261030a576004356118bc816102f9565b916024356118c9816103aa565b91604435906001600160401b03821161030a576118e891600401610714565b9091565b3461030a576118fa366118a4565b9161190961078d859395613fb1565b61196261083a6040519261191c84610322565b6001600160a01b038516845263ffffffff1660208401908152610ecc61082b610ec161194787613d7e565b6001600160a01b039098165f90815260986020526040902090565b5f5b83811061196d57005b60019061199e611999611988865f52609960205260405f2090565b6108f26108e66108e1868b8d613175565b613602565b7f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b611a046119d06108e184898b613175565b6040805187516001600160a01b0390811682526020808a015163ffffffff1690830152909216908201529081906060820190565b0390a101611964565b3461030a57604036600319011261030a57600435611a2a816102f9565b6024356001600160401b03811161030a57611a499036906004016109e0565b611a5381516132eb565b915f5b82518110156117ce57600190611a7f6001600160a01b03611a7783876130a9565b511684613b07565b6001600160401b03611a9183886130a9565b9116905201611a56565b3461030a57604036600319011261030a57600435611ab8816102f9565b602435611ac4816103aa565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169133839003611b02575b61057d925061464c565b6020602493611b18611b1384613fb1565b613618565b6040516336b87bd760e11b81526001600160a01b038416600482015294859182905afa9283156105ae5761057d93611b57915f91611b5c575b5061362e565b611af8565b611b75915060203d6020116105a7576105998183610378565b5f611b51565b3461030a575f36600319011261030a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156105ae57611be6915f9161057f5750613002565b61057d613f21565b3461030a57602036600319011261030a5760043560ff811680910361030a5760016020911b806066541614604051908152f35b3461030a575f36600319011261030a576020606654604051908152f35b3461030a57606036600319011261030a57602061070a600435611c60816102f9565b611c6c611658366103b8565b5f908152609a845260408082206001600160a01b03909316825260019092016020522054151590565b3461030a57611ca336611494565b611cd1611cc482610c3f8560018060a01b03165f5260a260205260405f2090565b546001600160401b031690565b90611d03611cf382610c3f8660018060a01b03165f5260a360205260405f2090565b5480600f0b9060801d600f0b0390565b5f905b808210611d50575b610671611d36856110c5610c5487610c3f8b60018060a01b03165f5260a160205260405f2090565b6040516001600160401b0390911681529081906020820190565b9092611dad611da884610c3f611d8388611d7e84610c3f8d60018060a01b03165f5260a360205260405f2090565b614820565b6001600160a01b038a165f90815260a0602052604090205b905f5260205260405f2090565b613644565b611dc161082b604083015163ffffffff1690565b4310611de25760200151600191611dda91600f0b610cff565b930190611d06565b5092611d0e565b9081606091031261030a5790565b3461030a57602036600319011261030a576004356001600160401b03811161030a57611e27903690600401611de9565b611e38610dfe600480606654161490565b611e49611e4482613185565b613fb1565b80156120d5575b611e5990613618565b6020810190604081014363ffffffff167f00000000000000000000000000000000000000000000000000000000000000005f5b611e968486613140565b905081101561206c578061206686611f0361083a8a610ecc61082b610ec1610ea8611ed46107ac8f9c6108db60019e611ece8a613185565b9c613140565b94611eef611ee0610399565b6001600160a01b03909a168a52565b6108e160208a0196879063ffffffff169052565b611f52611f4d611f46611f31611f188c613185565b6001600160a01b03165f908152609e6020526040902090565b611f3a85613d7e565b5f5260205260405f2090565b5460ff1690565b61367b565b611f89611f7a611f618a613185565b6001600160a01b03165f908152609c6020526040902090565b611f8383613d7e565b90614dec565b50611fb6611fa7611f9983613d7e565b5f52609a60205260405f2090565b611fb08a613185565b906148de565b50611fc36108e689613185565b7fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe60405180611ff2858261311a565b0390a261203561200286886136a9565b9161202061200e610399565b5f815263ffffffff9094166020850152565b611f3a61202f611f188c613185565b91613d7e565b906020908051151560ff80198554169116178355015164ffffffff0082549160081b169064ffffffff001916179055565b01611e8c565b83856120906120806108e6610d9a8b613185565b9261208a83613185565b92613140565b9092803b1561030a576120bd935f809460405196879586948593639d8e0c2360e01b855260048501613703565b03925af16120c757005b806112ad5f61057d93610378565b50611e596120e8611e4460208401613185565b9050611e50565b90602080835192838152019201905f5b81811061210c5750505090565b82516001600160a01b03168452602093840193909201916001016120ff565b9060206106199281815201906120ef565b3461030a57604036600319011261030a57612159611658366103f3565b5f52609a60205260405f206040519081602082549182815201915f5260205f20905f5b81811061219f576106718561219381870382610378565b6040519182918261212b565b825484526020909301926001928301920161217c565b3461030a575f36600319011261030a576121cd6148f1565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461030a57602036600319011261030a5760043561222d816102f9565b6001600160a01b03165f818152609c602052604090205461224d81613046565b915f5b82811061226557604051806106718682610608565b600190825f52609c6020526122806106908260405f20614a6c565b61228a82876130a9565b5261229581866130a9565b5001612250565b3461030a575f36600319011261030a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9060206106199281815201906114b9565b3461030a57608036600319011261030a576004356001600160401b03811161030a576123619036906004016109e0565b61236a366103b8565b90606435612377816102f9565b61238182516135b3565b925f5b83518110156123d8576001906123ba846001600160a01b036123a684896130a9565b51166123b0612fac565b506104b686613d7e565b90506123c682886130a9565b526123d181876130a9565b5001612384565b604051806106718782612320565b3461030a575f36600319011261030a576033546040516001600160a01b039091168152602090f35b3461030a57606036600319011261030a5760043561242b816102f9565b6024356001600160401b03811161030a5761244a9036906004016109e0565b90604435612457816103aa565b61246183516132eb565b925f926001600160a01b03169163ffffffff16905b8051841015612576575f83815260a1602052604090206124b8906001600160a01b036124a287856130a9565b511660018060a01b03165f5260205260405f2090565b938454945f955b80871061252e576001939495965080155f1461250657506124fd9050670de0b6b3a76400005b6124ef83896130a9565b906001600160401b03169052565b01929190612476565b6104606125276124fd9361251c6124e594613398565b905f5260205f200190565b5460201c90565b8087169080881860011c820180921161257157825f528563ffffffff8360205f20015416115f146125625750955b956124bf565b9650600181018091111561255c575b613384565b6040518061067187826116fd565b3461030a57604036600319011261030a576004356125a1816102f9565b6024356001600160401b03811161030a573660238201121561030a578060040135916125cc836109c9565b916125da6040519384610378565b8383526024602084019460051b8201019036821161030a5760248101945b8286106126095761057d8585613725565b85356001600160401b03811161030a5782016080602319823603011261030a576040519061263682610342565b612643366024830161042a565b825260648101356001600160401b03811161030a5761266890602436918401016109e0565b602083015260848101356001600160401b03811161030a57602491010136601f8201121561030a57803561269b816109c9565b916126a96040519384610378565b81835260208084019260051b8201019036821161030a57602001915b8183106126e25750505060408201528152602095860195016125f8565b82356001600160401b038116810361030a578152602092830192016126c5565b3461030a576020612740610c5461271836611494565b6001600160a01b039182165f90815260a1865260408082209290931681526020919091522090565b6001600160401b0360405191168152f35b3461030a57604036600319011261030a5760043561276e816102f9565b602435906001600160401b03821161030a573660238301121561030a578160040135906001600160401b03821161030a57366024838501011161030a57602461057d930190613b36565b3461030a5760206001600160401b036127fb6127d336611494565b6001600160a01b039182165f90815260a2865260408082209290931681526020919091522090565b5416604051908152f35b3461030a57604036600319011261030a57600435612822816102f9565b6024356001600160401b03811161030a57612841903690600401611de9565b90612853610dfe600480606654161490565b61285f61078d82613fb1565b6040516336b87bd760e11b81526001600160a01b0382166004820152602081806024810103817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156105ae576128c7915f91611b5c575061362e565b60208201906001600160a01b0381165f5b6128e28486613140565b9050811015612a0357806129fd6129f08761294961083a6129156107ac6001986108db8d61290f88613185565b97613140565b612920610e2a610399565b63ffffffff16602084019081528351610ecc9061082b90610ec1906001600160a01b0316610ea8565b61296261295d612959838a6140b0565b1590565b613b8a565b6001600160a01b0387165f908152609c602052604090206129869061083483613d7e565b5061299c87612997611f9984613d7e565b614046565b50857f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e604051806129cd858261311a565b0390a26001600160a01b0387165f908152609e60205260409020611f3a9061202f565b805460ff19166001179055565b016128d8565b8483612a2f86612a22612a1b6108e6610d9a87613185565b9185613140565b9290946040810190613496565b829591953b1561030a575f94612a5d86926040519889978896879563adcf73f760e01b875260048701613ba0565b03925af180156105ae576120c757005b3461030a57604036600319011261030a57612a8a611658366103f3565b5f52609a602052602060405f2054604051908152f35b3461030a5761083a612afc612ab4366118a4565b9391612ac561078d85979397613fb1565b610ecc61082b610ec160405196612adb88610322565b6001600160a01b038116885263ffffffff9094166020880190815293610ea8565b612b0581613d7e565b905f5b838110612b1157005b600190612b3d610fd7612b2c865f52609960205260405f2090565b611fb06108e66108e1868b8d613175565b7f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee612b6f6119d06108e184898b613175565b0390a101612b08565b3461030a57602036600319011261030a57604063ffffffff612ba4600435612b9f816102f9565b613c12565b835191151582529091166020820152f35b3461030a57602036600319011261030a57600435612bd2816102f9565b60018060a01b03165f526098602052602060405f2054604051908152f35b3461030a57606036600319011261030a57600435612c0d816102f9565b612c16366103b8565b9060018060a01b03165f52609f602052612c3360405f2091613d7e565b5f5260205260405f206040519081602082549182815201915f5260205f20905f5b818110612c6b576106718561169781870382610378565b8254845260209093019260019283019201612c54565b3461030a57604036600319011261030a57600435612c9e816102f9565b612ce36024355f5492612cc960ff600886901c161580958196612d61575b8115612d41575b50613cc7565b83612cda600160ff195f5416175f55565b612d2a57613d2a565b612ce957005b612cf761ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb384740249890806020810161148f565b612d3c61010061ff00195f5416175f55565b613d2a565b303b15915081612d53575b505f612cc3565b60ff1660011490505f612d4c565b600160ff8216109150612cbc565b3461030a57604036600319011261030a577f2ae945c40c44dc0ec263f95609c3fdc6952e0aefa22d6374e44f2c997acedf856040600435612daf816102f9565b612dfd60243591612dbf836102f9565b612dcb61078d82613fb1565b6001600160a01b038181165f818152609760205286902080546001600160a01b0319169590921694909417905561331d565b82519182526001600160a01b03166020820152a1005b3461030a575f36600319011261030a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461030a57602036600319011261030a57600435612e74816102f9565b612e7c6148f1565b6001600160a01b03811615612e945761057d90614949565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461030a57602036600319011261030a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9081156105ae575f91612f71575b506001600160a01b03163303612f625761057d90613d3b565b63794821ff60e01b5f5260045ffd5b90506020813d602011612fa4575b81612f8c60209383610378565b8101031261030a5751612f9e816102f9565b5f612f49565b3d9150612f7f565b60405190612fb982610342565b5f6040838281528260208201520152565b6104b6612fda93926104b0612fac565b905090565b9081602091031261030a5751801515810361030a5790565b6040513d5f823e3d90fd5b1561300957565b631d77d47760e21b5f5260045ffd5b1561301f57565b63c61dca5d60e01b5f5260045ffd5b6040519061303b82610322565b5f6020838281520152565b90613050826109c9565b61305d6040519182610378565b828152809261306e601f19916109c9565b01905f5b82811061307e57505050565b60209061308961302e565b82828501015201613072565b634e487b7160e01b5f52603260045260245ffd5b80518210156130bd5760209160051b010190565b613095565b156130c957565b63932d94f760e01b5f5260045ffd5b91908110156130bd5760051b81013590603e198136030182121561030a570190565b35610619816103aa565b1561310b57565b631fb1705560e21b5f5260045ffd5b81516001600160a01b0316815260209182015163ffffffff169181019190915260400190565b903590601e198136030182121561030a57018035906001600160401b03821161030a57602001918160051b3603831361030a57565b91908110156130bd5760051b0190565b35610619816102f9565b90613199826109c9565b6131a66040519182610378565b82815280926131b7601f19916109c9565b01905f5b8281106131c757505050565b8060606020809385010152016131bb565b60208183031261030a578051906001600160401b03821161030a57019080601f8301121561030a5781519061320c826109c9565b9261321a6040519485610378565b82845260208085019360051b8201019082821161030a5760208101935b82851061324657505050505090565b84516001600160401b03811161030a57820184603f8201121561030a57602081015190613272826109c9565b916132806040519384610378565b8083526020808085019260051b840101019187831161030a57604001905b8282106132b657505050815260209485019401613237565b815181526020918201910161329e565b90916132dd610619936040845260408401906120ef565b9160208184039101526115ee565b906132f5826109c9565b6133026040519182610378565b8281528092613313601f19916109c9565b0190602036910137565b6001600160a01b039081165f8181526097602052604090205490911680612fda575090565b1561334957565b63840a48d560e01b5f5260045ffd5b1561335f57565b6343714afd60e01b5f5260045ffd5b1561337557565b63ebbff49760e01b5f5260045ffd5b634e487b7160e01b5f52601160045260245ffd5b5f1981019190821161257157565b156133ad57565b639f1c805360e01b5f5260045ffd5b156133c357565b631353603160e01b5f5260045ffd5b156133d957565b6331bc342760e11b5f5260045ffd5b906001600160401b03809116911603906001600160401b03821161257157565b600f0b60016001607f1b03198114612571575f0390565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b0383131761257157565b6001600160a01b039182168152825182166020808301919091529092015163ffffffff9081166040840152921660608201526001600160401b0390921660808301529190911660a082015260c00190565b903590601e198136030182121561030a57018035906001600160401b03821161030a5760200191813603831361030a57565b908060209392818452848401375f828201840152601f01601f1916010190565b6001600160a01b03918216815282519091166020808301919091529091015163ffffffff1660408201529095949293918060c0830160c060608501525260e0820196905f5b8181106135825750505080860360808201526020808551978881520194015f965b80881061356a57505061061994955060a08185039101526134c8565b9094602080600192885181520196019701969061354e565b90919760206135a96001928b35613598816102f9565b6001600160a01b0316815260200190565b990192910161352d565b906135bd826109c9565b6135ca6040519182610378565b82815280926135db601f19916109c9565b01905f5b8281106135eb57505050565b6020906135f6612fac565b828285010152016135df565b1561360957565b63585cfb2f60e01b5f5260045ffd5b1561361f57565b6348f5c3ed60e01b5f5260045ffd5b1561363557565b63ccea9e6f60e01b5f5260045ffd5b9060405161365181610342565b604063ffffffff8294546001600160401b038116845280831c600f0b602085015260c01c16910152565b1561368257565b6325131d4f60e01b5f5260045ffd5b63ffffffff60019116019063ffffffff821161257157565b9063ffffffff8091169116019063ffffffff821161257157565b916020908281520191905f5b8181106136dc5750505090565b90919260208060019263ffffffff87356136f5816103aa565b1681520194019291016136cf565b6001600160a01b039091168152604060208201819052610619939101916136c3565b6137569161373a610dfe600180606654161490565b613746611b1383613fb1565b61374f82613c12565b9390613aaf565b5f925b8151841015613aa95761378b602061377186856130a9565b51015151604061378187866130a9565b5101515114613358565b61379584836130a9565b5151906137c561083a6137b1610ea8855160018060a01b031690565b610ecc61082b602087015163ffffffff1690565b6137cf82856140b0565b5f5b60206137dd88876130a9565b51015151811015613a9b57807f1487af5418c47ee5ea45ef4a93398668120890774a9e13487e61e9dc3baf76dd81886139bc8b613978896138858d6139728e8d61383a610bc960019f60206138328c896130a9565b5101516130a9565b978894613847868d61446a565b613855868d6104b687613d7e565b9d908e9981998b9661387d61387761387160208b0151600f0b90565b600f0b90565b15613ac5565b878b8a614991565b906138c06138b661389d89516001600160401b031690565b6138b0610c9d8860406138328d8d6130a9565b906149f0565b600f0b6020890152565b6138db6138d461387160208a0151600f0b90565b1515613adb565b6020870151600f0b805f811215613a1c575050505f146139c557505050505061392a61391b84610c3f8c60018060a01b03165f5260a360205260405f2090565b61392483613d7e565b90614a21565b6116586139656139607f000000000000000000000000000000000000000000000000000000000000000063ffffffff43166136a9565b613691565b63ffffffff166040870152565b87614149565b6139896139848b613d7e565b613f87565b9361137960406139b06139a384516001600160401b031690565b6020850151600f0b610cff565b92015163ffffffff1690565b0390a1016137d1565b6138326110ef94610c9d94613a00613a059861111560206040970191610cff60206139f785516001600160401b031690565b920151600f0b90565b6130a9565b5f60208601524363ffffffff166040860152613d7e565b94509550955050505f915013613a34575b5050613d7e565b613a9491613a88613a6361396593610c9d610d0560208c0192613a5e84516001600160401b031690565b614095565b6001600160401b03613a7f6104608b516001600160401b031690565b91161115613af1565b63ffffffff43166136a9565b8e5f613a2d565b505093600191500192613759565b50505050565b15613ab657565b63fa55fc8160e01b5f5260045ffd5b15613acc57565b630d8fcbe360e41b5f5260045ffd5b15613ae257565b634606179360e11b5f5260045ffd5b15613af857565b636c9be0bf60e01b5f5260045ffd5b6001600160a01b039081165f90815260a160209081526040808320939094168252919091522061061990614059565b907fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c94371391613b6561078d82613fb1565b613b856040519283926020845260018060a01b03169560208401916134c8565b0390a2565b15613b9157565b636c6c6e2760e11b5f5260045ffd5b93916106199593613bc69260018060a01b031686526060602087015260608601916136c3565b9260408185039101526134c8565b90604051613be18161035d565b606063ffffffff829454818116845260ff8160201c1615156020850152818160281c16604085015260481c16910152565b60018060a01b03165f52609b60205260405f2090606060405192613c358461035d565b54613c8d613c83613c7d63ffffffff841680885260ff8560201c1615159788602082015263ffffffff808760281c169687604084015260481c16968791015263ffffffff1690565b95151590565b9263ffffffff1690565b63ffffffff811615159081613cb6575b50613ca757509190565b9192505063ffffffff16600191565b63ffffffff1690504310155f613c9d565b15613cce57565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b613d366103a892613f55565b614949565b613d4c606654198219811614613018565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b16603482015260208152613dc4604082610378565b5190519060208110613dd4575090565b5f199060200360031b1b1690565b9291611da8613e9191613df361302e565b50613dfc612fac565b50610c3f613e21610c5483610c3f8a60018060a01b03165f5260a160205260405f2090565b94611d9b613e46611cc485610c3f8c60018060a01b03165f5260a260205260405f2090565b98613e61613e52610399565b6001600160401b039099168952565b613e78602089019a8b906001600160401b03169052565b6001600160a01b03165f90815260a06020526040902090565b926040840190613ea861082b835163ffffffff1690565b4310613f1b575f8092613ec287516001600160401b031690565b92613ee6613ed960208a0195610cff8751600f0b90565b6001600160401b03168952565b8351600f0b90838212613efc575b505052529190565b611115613f1492613a5e83516001600160401b031690565b5f80613ef4565b50509190565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b613f8f61302e565b5063ffffffff60405191613fa283610322565b8060601c835216602082015290565b604051631beb2b9760e31b81526001600160a01b0391821660048201523360248201523060448201525f80356001600160e01b03191660648301529091602091839160849183917f0000000000000000000000000000000000000000000000000000000000000000165af19081156105ae575f9161402d575090565b610619915060203d6020116105a7576105998183610378565b610619916001600160a01b031690614a9e565b80548061406f5750670de0b6b3a7640000919050565b805f19810111612571576001600160401b03915f525f199060205f2001015460201c1690565b6001600160401b0391826140ac9216600f0b61341f565b1690565b6001600160a01b03165f908152609e60205260409020906140d090613d7e565b5f52602052602060405f20604051906140e882610322565b549063ffffffff60ff831615159283835260081c169283910152151590811561410f575090565b63ffffffff16431115919050565b90670de0b6b3a7640000906141328184614bcc565b920961413b5790565b600181018091116125715790565b939092602061416f611cc485610c3f8960018060a01b03165f5260a260205260405f2090565b910180516001600160401b039081169216829003614312575b5050614210816141b084610c3f87611d9b8a60018060a01b03165f5260a060205260405f2090565b81518154602084015160409485015163ffffffff60c01b60c09190911b1677ffffffffffffffffffffffffffffffff00000000000000009190951b166001600160e01b03199091166001600160401b039092169190911717919091179055565b6020810151600f0b1561427a5750826142586142729261424785611d9b6142779860018060a01b03165f52609f60205260405f2090565b6001600160a01b0390911690614a9e565b506001600160a01b03165f908152609d6020526040902090565b614a9e565b50565b516001600160401b03161561428e57505050565b6142c390611f836108e66142b685611d9b8860018060a01b03165f52609f60205260405f2090565b926001600160a01b031690565b506001600160a01b0382165f908152609f602052604090206142e6908290611d9b565b54156142f0575050565b61430d6142779260018060a01b03165f52609d60205260405f2090565b614dec565b6143776143a891610c9d7facf9095feb3a370c9cf692421c69ef320d4db5c66e6a7d29c7694eb02364fc559461435c88610c3f8c60018060a01b03165f5260a260205260405f2090565b906001600160401b03166001600160401b0319825416179055565b604080516001600160a01b03808a168252871660208201526001600160401b03909216908201529081906060820190565b0390a15f80614188565b6001600160a01b038181165f90815260a1602090815260408083209386168352929052207f1c6458079a41077d003c11faf9bf097e693bd67979e4e6500bac7b29db779b5c9361444b91614418906001600160401b0383169063ffffffff431690614efb565b604080516001600160a01b0394851681529490931660208501526001600160401b03169183019190915281906060820190565b0390a1565b5f1981146125715760010190565b8015612571575f190190565b6001600160a01b038082165f90815260a3602090815260408083209386168352929052908120909392919061449e90611cf3565b935b84151580614557575b15614550576144d46144cf84610c3f8560018060a01b03165f5260a360205260405f2090565b614c7d565b6144df848285613de2565b916144f461082b604085015163ffffffff1690565b43106145465761453a9261454094928761450e9388614149565b61453461452f86610c3f8760018060a01b03165f5260a360205260405f2090565b614cbe565b50614450565b9461445e565b936144a0565b5050505050509050565b5050509050565b5061ffff81106144a9565b6001600160a01b038181165f90815260a360209081526040808320938616835292905290812090949061459490611cf3565b945b8515158061463f575b15614637576145c56144cf85610c3f8660018060a01b03165f5260a360205260405f2090565b6145d0858286613de2565b916145e561082b604085015163ffffffff1690565b431061462c57614620926146269492886145ff9389614149565b61453461452f87610c3f8860018060a01b03165f5260a360205260405f2090565b9561445e565b94614596565b505050509350505050565b509350505050565b5061ffff8516811061459f565b907f4e85751d6331506c6c62335f207eb31f12a61e570f34f5c17640308785c6d4db9161444b6147996146976146928460018060a01b03165f52609b60205260405f2090565b613bd4565b61083f60608201916146ad835163ffffffff1690565b63ffffffff8116151590816147f4575b506147ca575b63ffffffff8716604082015261470f6147056139607f000000000000000000000000000000000000000000000000000000000000000063ffffffff43166136a9565b63ffffffff168452565b6001600160a01b0386165f908152609b602052604090208151815460208085015160408601516060909601516cffffffff00000000000000000060489190911b1668ffffffff000000000060289790971b9690961664ff0000000091151590921b166cffffffffffffffffffffffffff1990921663ffffffff909316929092171717919091179055565b604080516001600160a01b03909416845263ffffffff94851660208501529316928201929092529081906060820190565b6147e86147de604083015163ffffffff1690565b63ffffffff168252565b600160208201526146c3565b63ffffffff1690504310155f6146bd565b9190915f838201938412911290801582169115161761257157565b805490916001600160ff1b0381116148885761387161484d6148486148549385600f0b614805565b614d2e565b9260801d90565b81600f0b1215614879576001614875920190600f0b5f5260205260405f2090565b5490565b632d0483c560e21b5f5260045ffd5b60405162461bcd60e51b815260206004820152602860248201527f53616665436173743a2076616c756520646f65736e27742066697420696e2061604482015267371034b73a191a9b60c11b6064820152608490fd5b610619916001600160a01b031690614dec565b6033546001600160a01b0316330361490557565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b9061499e6149c692613d7e565b5f52609960205260405f209060018060a01b0316906001915f520160205260405f2054151590565b91826149e8575b50816149d7575090565b6001600160401b0391505116151590565b91505f6149cd565b6001600160401b03809116600f0b9116600f0b0360016001607f1b03811360016001607f1b03198212176125715790565b90815460801d90614a40826001850190600f0b5f5260205260405f2090565b5581546001600160801b0316600190910160801b6fffffffffffffffffffffffffffffffff1916179055565b80548210156130bd575f5260205f2001905f90565b91614a9a9183549060031b91821b915f19901b19161790565b9055565b5f828152600182016020526040902054614b0157805490600160401b82101561033d5782614aec614ad6846001809601855584614a6c565b819391549060031b91821b915f19901b19161790565b90558054925f520160205260405f2055600190565b50505f90565b8115614b11570490565b634e487b7160e01b5f52601260045260245ffd5b1561030a57565b5f19670de0b6b3a7640000820991670de0b6b3a7640000820291828085109403938085039414614bc057670de0b6b3a76400008291614b6c868411614b25565b09600182190182168092046002816003021880820260020302808202600203028082026002030280820260020302808202600203028091026002030293600183805f03040190848311900302920304170290565b50906106199250614b07565b5f1982820982820291828083109203918083039214614c355781670de0b6b3a7640000111561030a577faccb18165bd6fe31ae1cf318dc5b51eee0e1ba569b88cd74c1773b91fac1066993670de0b6b3a7640000910990828211900360ee1b910360121c170290565b50670de0b6b3a76400009250500490565b90915f198383099280830292838086109503948086039514614c7057908291614b6c868411614b25565b5050906106199250614b07565b614c92815480600f0b9060801d600f0b131590565b614caf578054600f0b5f9081526001909101602052604090205490565b631ed9509560e11b5f5260045ffd5b90614cd4825480600f0b9060801d600f0b131590565b614caf578154600f0b9160018101925f614d0f82614cfd818890600f0b5f5260205260405f2090565b549690600f0b5f5260205260405f2090565b5560016001600160801b031983541691016001600160801b0316179055565b60016001607f1b031981121580614da0575b15614d4b57600f0b90565b60405162461bcd60e51b815260206004820152602760248201527f53616665436173743a2076616c756520646f65736e27742066697420696e20316044820152663238206269747360c81b6064820152608490fd5b5060016001607f1b03811315614d40565b80548015614dd8575f190190614dc78282614a6c565b8154905f199060031b1b1916905555565b634e487b7160e01b5f52603160045260245ffd5b6001810191805f528260205260405f2054928315155f14614e87575f1984018481116125715783545f19810194908511612571575f958583611d9b94614e3a9803614e40575b505050614db1565b55600190565b614e70614e6a91614e61614e57614e7e9588614a6c565b90549060031b1c90565b92839187614a6c565b90614a81565b85905f5260205260405f2090565b555f8080614e32565b505050505f90565b15614e9657565b63151b8e3f60e11b5f5260045ffd5b8054600160401b81101561033d57614ec291600182018155614a6c565b614ee857815160209283015190921b63ffffffff191663ffffffff909216919091179055565b634e487b7160e01b5f525f60045260245ffd5b805480614f37575b50614f326103a893614f22614f16610399565b63ffffffff9095168552565b6001600160e01b03166020840152565b614ea5565b805f1981011161257157815f5263ffffffff614f9261082b5f198460205f20010161083f614f8460405192614f6b84610322565b548681169081855260201c602085015263ffffffff1690565b858916958691161115614e8f565b03614f03576103a89392509061251c614faa92613398565b9063ffffffff82549181199060201b16911617905556fea2646970667358221220de0eea5298eb5e93bb9991899e83b61e9d6ebf05e09699f970429f4b4194634964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x10\xE1\xB9\xB8\x14a\x02\xF4W\x80c\x13d9\xDD\x14a\x02\xEFW\x80c\x15\xFEP(\x14a\x02\xEAW\x80c&\r\xC7X\x14a\x02\xE5W\x80c&\x1F\x84\xE0\x14a\x02\xE0W\x80c)\x81\xEBw\x14a\x02\xDBW\x80c+\xAB,J\x14a\x02\xD6W\x80c0L\x10\xCD\x14a\x02\xD1W\x80c65 W\x14a\x02\xCCW\x80c@\x12\r\xAB\x14a\x02\xC7W\x80cAw\xA8|\x14a\x02\xC2W\x80cFW\xE2j\x14a\x02\xBDW\x80cJ\x10\xFF\xE5\x14a\x02\xB8W\x80cKPF\xEF\x14a\x02\xB3W\x80cP\xFE\xEA \x14a\x02\xAEW\x80cTz\xFB\x87\x14a\x02\xA9W\x80cV\xC4\x83\xE6\x14a\x02\xA4W\x80cY\\jg\x14a\x02\x9FW\x80cZ\xC8j\xB7\x14a\x02\x9AW\x80c\\\x97Z\xBB\x14a\x02\x95W\x80cg\r;\xA2\x14a\x02\x90W\x80cl\xFBD\x81\x14a\x02\x8BW\x80cn4\x92\xB5\x14a\x02\x86W\x80cn\x87]\xBA\x14a\x02\x81W\x80cqP\x18\xA6\x14a\x02|W\x80cy\xAEP\xCD\x14a\x02wW\x80c{\xC1\xEFa\x14a\x02rW\x80c\x88o\x11\x95\x14a\x02mW\x80c\x8C\xE6HT\x14a\x02hW\x80c\x8D\xA5\xCB[\x14a\x02cW\x80c\x94\xD7\xD0\x0C\x14a\x02^W\x80c\x95(\x99\xEE\x14a\x02YW\x80c\xA93>\xC8\x14a\x02TW\x80c\xA9\x82\x18!\x14a\x02OW\x80c\xA9\x84\xEB:\x14a\x02JW\x80c\xAD\xC2\xE3\xD9\x14a\x02EW\x80c\xB2Dz\xF7\x14a\x02@W\x80c\xB6k\xD9\x89\x14a\x02;W\x80c\xB9\xFB\xAE\xD1\x14a\x026W\x80c\xBA\x1A\x84\xE5\x14a\x021W\x80c\xC2!\xD8\xAE\x14a\x02,W\x80c\xCDm\xC6\x87\x14a\x02'W\x80c\xD3\xD9o\xF4\x14a\x02\"W\x80c\xDF\\\xF7#\x14a\x02\x1DW\x80c\xF2\xFD\xE3\x8B\x14a\x02\x18Wc\xFA\xBC\x1C\xBC\x14a\x02\x13W_\x80\xFD[a.\xE8V[a.WV[a.\x13V[a-oV[a,\x81V[a+\xF0V[a+\xB5V[a+xV[a*\xA0V[a*mV[a(\x05V[a'\xB8V[a'QV[a'\x02V[a%\x84V[a$\x0EV[a#\xE6V[a#1V[a\"\xDCV[a\"\x9CV[a\"\x10V[a!\xB5V[a!<V[a\x1D\xF7V[a\x1C\x95V[a\x1C>V[a\x1C!V[a\x1B\xEEV[a\x1B{V[a\x1A\x9BV[a\x1A\rV[a\x18\xECV[a\x17\xDCV[a\x17?V[a\x16\xB9V[a\x16;V[a\x15AV[a\r\xB1V[a\rxV[a\n\xD1V[a\t\x89V[a\x07DV[a\x06\xB8V[a\x06\x1CV[a\x04\xF3V[a\x04lV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\nWV[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[a\x03\x0EV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03=W`@RV[`@Q\x90a\x03\xA8`@\x83a\x03xV[V[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03\nWV[`@\x90`#\x19\x01\x12a\x03\nW`@Q\x90a\x03\xD1\x82a\x03\"V[\x81`$5a\x03\xDE\x81a\x02\xF9V[\x81R` `D5\x91a\x03\xEF\x83a\x03\xAAV[\x01RV[`@\x90`\x03\x19\x01\x12a\x03\nW`@Q\x90a\x04\x0C\x82a\x03\"V[\x81`\x045a\x04\x19\x81a\x02\xF9V[\x81R` `$5\x91a\x03\xEF\x83a\x03\xAAV[\x91\x90\x82`@\x91\x03\x12a\x03\nW`@Qa\x04B\x81a\x03\"V[` \x80\x82\x94\x805a\x04R\x81a\x02\xF9V[\x84R\x015\x91a\x03\xEF\x83a\x03\xAAV[`\x01`\x01`@\x1B\x03\x16\x90V[4a\x03\nW`\x806`\x03\x19\x01\x12a\x03\nW``a\x04\xBC`\x045a\x04\x8E\x81a\x02\xF9V[a\x04\x976a\x03\xB8V[\x90a\x04\xB6`d5\x92a\x04\xA8\x84a\x02\xF9V[a\x04\xB0a/\xACV[Pa=~V[\x90a=\xE2V[\x90Pa\x04\xF1`@Q\x80\x92c\xFF\xFF\xFF\xFF`@\x80\x92`\x01`\x01`@\x1B\x03\x81Q\x16\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R\x01Q\x16\x91\x01RV[\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x05\xAEWa\x05}\x92a\x05i\x91_\x91a\x05\x7FW[Pa0\x02V[a\x05x`fT\x82\x81\x16\x14a0\x18V[a?UV[\0[a\x05\xA1\x91P` =` \x11a\x05\xA7W[a\x05\x99\x81\x83a\x03xV[\x81\x01\x90a/\xDFV[_a\x05cV[P=a\x05\x8FV[a/\xF7V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x05\xD0WPPP\x90V[\x90\x91\x92` `@\x82a\x05\xFD`\x01\x94\x88Qc\xFF\xFF\xFF\xFF` \x80\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\x01\x94\x01\x92\x91\x01a\x05\xC3V[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x05\xB3V[\x90V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a\x069\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9D` R`@\x90 Ta\x06Y\x81a0FV[\x91_[\x82\x81\x10a\x06uW`@Q\x80a\x06q\x86\x82a\x06\x08V[\x03\x90\xF3[`\x01\x90\x82_R`\x9D` Ra\x06\x9Ca\x06\x90\x82`@_ aJlV[\x90T\x90`\x03\x1B\x1Ca?\x87V[a\x06\xA6\x82\x87a0\xA9V[Ra\x06\xB1\x81\x86a0\xA9V[P\x01a\x06\\V[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW` a\x07\na\x06\xD76a\x03\xF3V[\x80Q`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98\x84R`@\x80\x82 \x92\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x82R`\x01\x90\x92\x01` R T\x15\x15\x90V[`@Q\x90\x15\x15\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x03\nW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\nW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\nWV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x07a\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x07\x80\x906\x90`\x04\x01a\x07\x14V[\x90a\x07\x92a\x07\x8D\x84a?\xB1V[a0\xC2V[_\x91[\x80\x83\x10a\x07\x9EW\0[a\x07\xB1a\x07\xAC\x84\x83\x85a0\xD8V[a0\xFAV[\x90\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~la\x08za\x08Ia\x07\xE1a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x89\x16\x81R\x94c\xFF\xFF\xFF\xFF\x16` \x86\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x89\x16_\x90\x81R`\x98` R`@\x90 a\x08?\x90a\x08:\x90a\x084a\x08+\x85Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x90V[\x90aJ\x9EV[a1\x04V[Qc\xFF\xFF\xFF\xFF\x16\x90V[a\x08na\x08Ta\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x8A\x16\x81R\x91c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`@Q\x91\x82\x91\x82a1\x1AV[\x03\x90\xA1a\x08\x86\x82a=~V[\x92_[a\x08\xA1a\x08\x97\x87\x85\x85a0\xD8V[` \x81\x01\x90a1@V[\x90P\x81\x10\x15a\toW\x80a\x08\xF8\x87a\x08\xF2a\x08\xE6a\x08\xE1`\x01\x96a\x08\xDBa\x08\x97a\x08\xD3\x8E_R`\x99` R`@_ \x90V[\x96\x8C\x8Ca0\xD8V[\x90a1uV[a1\x85V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90a@FV[P\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\tfa\t2a\x08\xE1\x84a\x08\xDBa\x08\x97\x8D\x8B\x8Ba0\xD8V[`@\x80Q\x89Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8C\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x08\x89V[P`\x01\x90\x94\x01\x93\x92P\x90Pa\x07\x95V[_\x91\x03\x12a\x03\nWV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[`\x01`\x01`@\x1B\x03\x81\x11a\x03=W`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x03\nW\x815a\t\xF7\x81a\t\xC9V[\x92a\n\x05`@Q\x94\x85a\x03xV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03\nW` \x01\x90[\x82\x82\x10a\n-WPPP\x90V[` \x80\x91\x835a\n<\x81a\x02\xF9V[\x81R\x01\x91\x01\x90a\n V[` \x81\x01` \x82R\x82Q\x80\x91R`@\x82\x01\x91` `@\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\nyWPPPPP\x90V[\x90\x91\x92\x93\x94`?\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\n\xB9WPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\njV[\x90\x91\x92` \x80`\x01\x92\x86Q\x81R\x01\x94\x01\x92\x01\x90a\n\x9AV[4a\x03\nW`\xA06`\x03\x19\x01\x12a\x03\nWa\n\xEB6a\x03\xF3V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x0B\n\x906\x90`\x04\x01a\t\xE0V[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x0B*\x906\x90`\x04\x01a\t\xE0V[\x90`\x845a\x0B7\x81a\x03\xAAV[a\x0BA\x84Qa1\x8FV[\x90`@Qcxps;`\xE1\x1B\x81R_\x81\x80a\x0B`\x88\x8A`\x04\x84\x01a2\xC6V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05\xAEW_\x91a\rVW[Pc\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x90_g\r\xE0\xB6\xB3\xA7d\0\0\x94[\x87Q\x82\x10\x15a\rHWa\x0B\xD6a\x0B\xC9\x83\x8Aa0\xA9V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x92a\x0B\xE1\x88Qa2\xEBV[a\x0B\xEB\x84\x88a0\xA9V[Ra\x0B\xF6\x83\x87a0\xA9V[P_[\x88\x80Q\x82\x10\x15a\r:W\x90\x83\x86a\x0C\x15a\x0B\xC9\x84`\x01\x96a0\xA9V[\x8B`\x01`\x01`@\x1B\x03a\x0CYa\x0CT\x84a\x0C?\x87`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[a@YV[\x16\x91\x82\x15a\r0Wa\x04`a\x0Cxa\x0C\xAA\x92a\x0C\xC9\x97a\x0C\xAF\x97a/\xCAV[\x8Dc\xFF\xFF\xFF\xFFa\x0C\x8F`@\x84\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x16\x11\x15\x80a\r\x17W[a\x0C\xDFW[Q`\x01`\x01`@\x1B\x03\x16\x90V[aLFV[a\x0C\xC3\x83a\x0C\xBD\x89\x88a0\xA9V[Qa0\xA9V[QaK\xCCV[a\x0C\xD7\x82a\x0C\xBD\x88\x8Ca0\xA9V[R[\x01a\x0B\xF9V[a\r\x12a\r\x05a\x0C\xF6\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x84\x01Q`\x0F\x0B[\x90a@\x95V[`\x01`\x01`@\x1B\x03\x16\x82RV[a\x0C\x9DV[P_a\r'` \x83\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x12a\x0C\x98V[PPPPPa\x0C\xD9V[PP\x92P\x90`\x01\x01\x90a\x0B\xB3V[`@Q\x80a\x06q\x87\x82a\nGV[a\rr\x91P=\x80_\x83>a\rj\x81\x83a\x03xV[\x81\x01\x90a1\xD8V[_a\x0B\x9CV[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW` a\r\x9F`\x045a\r\x9A\x81a\x02\xF9V[a3\x1DV[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\r\xCE\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW`\xA0`\x03\x19\x826\x03\x01\x12a\x03\nWa\x0E\x04a\r\xFE`\x02\x80`fT\x16\x14\x90V[\x15a3BV[a\x0E\x10a\x07\x8D\x83a?\xB1V[a\x0E\xE5a\x0E\x1F`$\x83\x01a0\xFAV[a\x0E9a\x0E*a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[c\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81Ra\x0E\xE0a\x08:a\x0Ea\x86a\x0E\\\x87`\x04\x01a1\x85V[a@\xB0V[\x92a\x0E\x90a\x0Eu`D\x88\x01\x88`\x04\x01a1@V[\x90Pa\x0E\x87`d\x89\x01\x89`\x04\x01a1@V[\x91\x90P\x14a3XV[\x86Qa\x0E\xCC\x90a\x08+\x90a\x0E\xC1\x90`\x01`\x01`\xA0\x1B\x03\x16[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x98` R`@\x90 \x90V[\x92Qc\xFF\xFF\xFF\xFF\x16\x90V[\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[a3nV[a\x0E\xFFa\x0E\xF8`D\x83\x01\x83`\x04\x01a1@V[\x90Pa2\xEBV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x91\x90_[a\x0F=`D\x84\x01\x84`\x04\x01a1@V[\x90P\x81\x10\x15a\x14)W\x80\x15\x80\x15a\x13\xC7W[a\x0FX\x90a3\xA6V[a\x0Fl\x81a\x08\xDB`d\x86\x01\x86`\x04\x01a1@V[5\x15\x15\x80a\x13\x9CW[a\x0F~\x90a3\xBCV[a\x0F\xDCa\x0F\xD7a\x0F\x9Ea\x0F\x90\x88a=~V[_R`\x99` R`@_ \x90V[a\x0F\xB8a\x08\xE6a\x08\xE1\x86a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x15\x15\x90V[a3\xD2V[a\x10\x11a\x0F\xEB\x84`\x04\x01a1\x85V[a\x0F\xF4\x87a=~V[a\x10\x0Ba\x08\xE1\x85a\x08\xDB`D\x8A\x01\x8A`\x04\x01a1@V[\x91a=\xE2V[\x91\x90a\x10'a\x04`\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x80\x15a\x13\x91Wa\x10pa\x04`\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x92a\x10i\x86a\x08\xDB`d\x8C\x01\x8C`\x04\x01a1@V[5\x90aA\x1DV[\x93a\x11\"a\x10\x85\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x95a\x10\xA2`\x01`\x01`@\x1B\x03\x88\x16`\x01`\x01`@\x1B\x03\x83\x16aK,V[a\x10\xAC\x87\x8Aa0\xA9V[Ra\x10\xD7a\x10\xCA\x82a\x10\xC5\x86Q`\x01`\x01`@\x1B\x03\x16\x90V[a3\xE8V[`\x01`\x01`@\x1B\x03\x16\x84RV[a\x10\xFCa\x10\xEF\x82a\x10\xC5\x88Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x86RV[a\x11\x15` \x86\x01\x91a\x10\xC5\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90RV[\x86a\x111` \x83\x01Q`\x0F\x0B\x90V[\x85_a\x11=\x83`\x0F\x0B\x90V[\x12a\x12\xB9W[\x83\x91P\x8B\x83a\x11ya\x08\xE1\x89\x94a\x08\xDBa\x11ka\x11ea\x11\x7F\x9A`\x04\x01a1\x85V[\x96a=~V[\x94`D\x81\x01\x90`\x04\x01a1@V[\x91aAIV[a\x11\x8B\x87`\x04\x01a1\x85V[\x90a\x11\xCDa\x11\xB7a\x11\xA9a\x08\xE1\x88a\x08\xDB`D\x8E\x01`\x04\x8F\x01a1@V[\x92Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92`@Q\x93\x84\x93\x8Dc\xFF\xFF\xFF\xFFC\x16\x93\x86a4EV[\x03\x90\xA1a\x12\x07a\x11\xDF\x86`\x04\x01a1\x85V[a\x11\xF6a\x08\xE1\x85a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[\x83Q`\x01`\x01`@\x1B\x03\x16\x91aC\xB2V[a\x12\x13\x85`\x04\x01a1\x85V[\x90a\x12<a\x12.a\x08\xE1\x85a\x08\xDB`D\x8B\x01\x8B`\x04\x01a1@V[\x91Q`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x87;\x15a\x03\nW`@Qc`\x1B\xB3o`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x91\x16`$\x82\x01R`\x01`\x01`@\x1B\x03\x93\x84\x16`D\x82\x01R\x92\x16`d\x83\x01R_\x82`\x84\x81\x83\x89Z\xF1\x91\x82\x15a\x05\xAEW`\x01\x92a\x12\x9FW[P[\x01a\x0F-V[\x80a\x12\xAD_a\x12\xB3\x93a\x03xV[\x80a\t\x7FV[_a\x12\x97V[a\x12\xFCa\x04`a\x04`a\x13\"\x95a\x10ia\x13\x0B\x95a\x08\xDBa\x12\xEEa\x12\xE2a\x12\xE2a\x13\x18\x9Ba4\x08V[`\x01`\x01`\x80\x1B\x03\x16\x90V[\x93`d\x81\x01\x90`\x04\x01a1@V[`\x01`\x01`\x80\x1B\x03\x16`\x0F\x0B\x90V[` \x84\x01Q`\x0F\x0Ba4\x1FV[`\x0F\x0B` \x83\x01RV[\x81\x89\x88a\x13\x86a\x13La\x08\xE1\x89a\x08\xDBa\x13>\x86`\x04\x01a1\x85V[\x95`D\x81\x01\x90`\x04\x01a1@V[a\x13ma\x13`\x87Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x88\x01Q`\x0F\x0Ba\x0C\xFFV[`@\x87\x01Qc\xFF\xFF\xFF\xFF\x16[\x91`@Q\x95\x86\x95\x86a4EV[\x03\x90\xA1\x86_\x85a\x11CV[PP`\x01\x91Pa\x12\x99V[Pa\x0F~g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xBD\x83a\x08\xDB`d\x88\x01\x88`\x04\x01a1@V[5\x11\x15\x90Pa\x0FuV[Pa\x0FXa\x13\xE8a\x08\xE6a\x08\xE6a\x08\xE1\x85a\x08\xDB`D\x8A\x01\x8A`\x04\x01a1@V[a\x14\x16a\x08\xE6a\x08\xE6a\x08\xE6a\x08\xE1a\x14\x07`D\x8B\x01\x8B`\x04\x01a1@V[a\x14\x10\x8Aa3\x98V[\x91a1uV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x11\x90Pa\x0FOV[\x7F\x80\x96\x9A\xD2\x94(\xD6y~\xE7\xAA\xD0\x84\xF9\xE4\xA4*\x82\xFCPm\xCD,\xA3\xB6\xFBC\x1F\x85\xCC\xEB\xE5\x85\x83a\x14\x8Fa\x14\x80\x87a\x14_\x81`\x04\x01a1\x85V[\x93a\x14p`D\x83\x01\x83`\x04\x01a1@V[\x93\x90\x92`\x84\x81\x01\x90`\x04\x01a4\x96V[\x93\x90\x92`@Q\x97\x88\x97\x88a4\xE8V[\x03\x90\xA1\0[`@\x90`\x03\x19\x01\x12a\x03\nW`\x045a\x14\xAC\x81a\x02\xF9V[\x90`$5a\x06\x19\x81a\x02\xF9V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x14\xD6WPPP\x90V[\x90\x91\x92` ``\x82a\x15\x11`\x01\x94\x88Qc\xFF\xFF\xFF\xFF`@\x80\x92`\x01`\x01`@\x1B\x03\x81Q\x16\x85R` \x81\x01Q`\x0F\x0B` \x86\x01R\x01Q\x16\x91\x01RV[\x01\x94\x01\x92\x91\x01a\x14\xC9V[\x90\x91a\x153a\x06\x19\x93`@\x84R`@\x84\x01\x90a\x05\xB3V[\x91` \x81\x84\x03\x91\x01Ra\x14\xB9V[4a\x03\nWa\x15O6a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x82\x16_\x81\x81R`\x9D` R`@\x90 T\x90\x92\x91a\x15s\x82a0FV[\x92a\x15}\x83a5\xB3V[\x94_[\x84\x81\x10a\x15\x96W`@Q\x80a\x06q\x89\x89\x83a\x15\x1CV[`\x01\x90\x82_R`\x9D` Ra\x15\xD2\x85a\x15\xB5a\x06\x90\x84`@_ aJlV[\x80a\x15\xC0\x85\x8Ca0\xA9V[Ra\x15\xCB\x84\x8Ba0\xA9V[P\x86a/\xCAV[a\x15\xDC\x82\x8Aa0\xA9V[Ra\x15\xE7\x81\x89a0\xA9V[P\x01a\x15\x80V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x16\x0BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x15\xFEV[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x15\xEEV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa\x16]a\x16X6a\x03\xF3V[a=~V[_R`\x99` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\x16\xA3Wa\x06q\x85a\x16\x97\x81\x87\x03\x82a\x03xV[`@Q\x91\x82\x91\x82a\x16*V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x16\x80V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x17 WPPP\x90V[\x82Q`\x01`\x01`@\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x17\x13V[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x17o\x906\x90`\x04\x01a\t\xE0V[`$5\x90a\x17|\x82a\x02\xF9V[a\x17\x86\x81Qa2\xEBV[\x91_[\x82Q\x81\x10\x15a\x17\xCEW`\x01\x90a\x17\xB2\x83`\x01`\x01`\xA0\x1B\x03a\x17\xAB\x84\x88a0\xA9V[Q\x16a;\x07V[`\x01`\x01`@\x1B\x03a\x17\xC4\x83\x88a0\xA9V[\x91\x16\x90R\x01a\x17\x89V[`@Q\x80a\x06q\x86\x82a\x16\xFDV[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a\x17\xF9\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x18\x18\x906\x90`\x04\x01a\x07\x14V[\x90\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x189\x906\x90`\x04\x01a\x07\x14V[\x92\x90\x93a\x18Ma\r\xFE`\x01\x80`fT\x16\x14\x90V[a\x18X\x84\x83\x14a3XV[_[\x82\x81\x10a\x18cW\0[a\x18n\x81\x84\x84a1uV[5\x90a\x18y\x82a\x02\xF9V[a\x18\x84\x81\x87\x89a1uV[5\x91a\xFF\xFF\x83\x16\x83\x03a\x03\nW`\x01\x92a\x18\x9E\x91\x87aEbV[\x01a\x18ZV[```\x03\x19\x82\x01\x12a\x03\nW`\x045a\x18\xBC\x81a\x02\xF9V[\x91`$5a\x18\xC9\x81a\x03\xAAV[\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nWa\x18\xE8\x91`\x04\x01a\x07\x14V[\x90\x91V[4a\x03\nWa\x18\xFA6a\x18\xA4V[\x91a\x19\ta\x07\x8D\x85\x93\x95a?\xB1V[a\x19ba\x08:`@Q\x92a\x19\x1C\x84a\x03\"V[`\x01`\x01`\xA0\x1B\x03\x85\x16\x84Rc\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81Ra\x0E\xCCa\x08+a\x0E\xC1a\x19G\x87a=~V[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16_\x90\x81R`\x98` R`@\x90 \x90V[_[\x83\x81\x10a\x19mW\0[`\x01\x90a\x19\x9Ea\x19\x99a\x19\x88\x86_R`\x99` R`@_ \x90V[a\x08\xF2a\x08\xE6a\x08\xE1\x86\x8B\x8Da1uV[a6\x02V[\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\x1A\x04a\x19\xD0a\x08\xE1\x84\x89\x8Ba1uV[`@\x80Q\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8A\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x19dV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x1A*\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x1AI\x906\x90`\x04\x01a\t\xE0V[a\x1AS\x81Qa2\xEBV[\x91_[\x82Q\x81\x10\x15a\x17\xCEW`\x01\x90a\x1A\x7F`\x01`\x01`\xA0\x1B\x03a\x1Aw\x83\x87a0\xA9V[Q\x16\x84a;\x07V[`\x01`\x01`@\x1B\x03a\x1A\x91\x83\x88a0\xA9V[\x91\x16\x90R\x01a\x1AVV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a\x1A\xB8\x81a\x02\xF9V[`$5a\x1A\xC4\x81a\x03\xAAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x913\x83\x90\x03a\x1B\x02W[a\x05}\x92PaFLV[` `$\x93a\x1B\x18a\x1B\x13\x84a?\xB1V[a6\x18V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16`\x04\x82\x01R\x94\x85\x91\x82\x90Z\xFA\x92\x83\x15a\x05\xAEWa\x05}\x93a\x1BW\x91_\x91a\x1B\\W[Pa6.V[a\x1A\xF8V[a\x1Bu\x91P` =` \x11a\x05\xA7Wa\x05\x99\x81\x83a\x03xV[_a\x1BQV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\xAEWa\x1B\xE6\x91_\x91a\x05\x7FWPa0\x02V[a\x05}a?!V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`\xFF\x81\x16\x80\x91\x03a\x03\nW`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `fT`@Q\x90\x81R\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW` a\x07\n`\x045a\x1C`\x81a\x02\xF9V[a\x1Cla\x16X6a\x03\xB8V[_\x90\x81R`\x9A\x84R`@\x80\x82 `\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x82R`\x01\x90\x92\x01` R T\x15\x15\x90V[4a\x03\nWa\x1C\xA36a\x14\x94V[a\x1C\xD1a\x1C\xC4\x82a\x0C?\x85`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[T`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x1D\x03a\x1C\xF3\x82a\x0C?\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x03\x90V[_\x90[\x80\x82\x10a\x1DPW[a\x06qa\x1D6\x85a\x10\xC5a\x0CT\x87a\x0C?\x8B`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x90\x92a\x1D\xADa\x1D\xA8\x84a\x0C?a\x1D\x83\x88a\x1D~\x84a\x0C?\x8D`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aH V[`\x01`\x01`\xA0\x1B\x03\x8A\x16_\x90\x81R`\xA0` R`@\x90 [\x90_R` R`@_ \x90V[a6DV[a\x1D\xC1a\x08+`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10a\x1D\xE2W` \x01Q`\x01\x91a\x1D\xDA\x91`\x0F\x0Ba\x0C\xFFV[\x93\x01\x90a\x1D\x06V[P\x92a\x1D\x0EV[\x90\x81``\x91\x03\x12a\x03\nW\x90V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa\x1E'\x906\x90`\x04\x01a\x1D\xE9V[a\x1E8a\r\xFE`\x04\x80`fT\x16\x14\x90V[a\x1EIa\x1ED\x82a1\x85V[a?\xB1V[\x80\x15a \xD5W[a\x1EY\x90a6\x18V[` \x81\x01\x90`@\x81\x01Cc\xFF\xFF\xFF\xFF\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_[a\x1E\x96\x84\x86a1@V[\x90P\x81\x10\x15a lW\x80a f\x86a\x1F\x03a\x08:\x8Aa\x0E\xCCa\x08+a\x0E\xC1a\x0E\xA8a\x1E\xD4a\x07\xAC\x8F\x9Ca\x08\xDB`\x01\x9Ea\x1E\xCE\x8Aa1\x85V[\x9Ca1@V[\x94a\x1E\xEFa\x1E\xE0a\x03\x99V[`\x01`\x01`\xA0\x1B\x03\x90\x9A\x16\x8ARV[a\x08\xE1` \x8A\x01\x96\x87\x90c\xFF\xFF\xFF\xFF\x16\x90RV[a\x1FRa\x1FMa\x1FFa\x1F1a\x1F\x18\x8Ca1\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9E` R`@\x90 \x90V[a\x1F:\x85a=~V[_R` R`@_ \x90V[T`\xFF\x16\x90V[a6{V[a\x1F\x89a\x1Fza\x1Fa\x8Aa1\x85V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9C` R`@\x90 \x90V[a\x1F\x83\x83a=~V[\x90aM\xECV[Pa\x1F\xB6a\x1F\xA7a\x1F\x99\x83a=~V[_R`\x9A` R`@_ \x90V[a\x1F\xB0\x8Aa1\x85V[\x90aH\xDEV[Pa\x1F\xC3a\x08\xE6\x89a1\x85V[\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE`@Q\x80a\x1F\xF2\x85\x82a1\x1AV[\x03\x90\xA2a 5a \x02\x86\x88a6\xA9V[\x91a  a \x0Ea\x03\x99V[_\x81Rc\xFF\xFF\xFF\xFF\x90\x94\x16` \x85\x01RV[a\x1F:a /a\x1F\x18\x8Ca1\x85V[\x91a=~V[\x90` \x90\x80Q\x15\x15`\xFF\x80\x19\x85T\x16\x91\x16\x17\x83U\x01Qd\xFF\xFF\xFF\xFF\0\x82T\x91`\x08\x1B\x16\x90d\xFF\xFF\xFF\xFF\0\x19\x16\x17\x90UV[\x01a\x1E\x8CV[\x83\x85a \x90a \x80a\x08\xE6a\r\x9A\x8Ba1\x85V[\x92a \x8A\x83a1\x85V[\x92a1@V[\x90\x92\x80;\x15a\x03\nWa \xBD\x93_\x80\x94`@Q\x96\x87\x95\x86\x94\x85\x93c\x9D\x8E\x0C#`\xE0\x1B\x85R`\x04\x85\x01a7\x03V[\x03\x92Z\xF1a \xC7W\0[\x80a\x12\xAD_a\x05}\x93a\x03xV[Pa\x1EYa \xE8a\x1ED` \x84\x01a1\x85V[\x90Pa\x1EPV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a!\x0CWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a \xFFV[\x90` a\x06\x19\x92\x81\x81R\x01\x90a \xEFV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa!Ya\x16X6a\x03\xF3V[_R`\x9A` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a!\x9FWa\x06q\x85a!\x93\x81\x87\x03\x82a\x03xV[`@Q\x91\x82\x91\x82a!+V[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a!|V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nWa!\xCDaH\xF1V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a\"-\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9C` R`@\x90 Ta\"M\x81a0FV[\x91_[\x82\x81\x10a\"eW`@Q\x80a\x06q\x86\x82a\x06\x08V[`\x01\x90\x82_R`\x9C` Ra\"\x80a\x06\x90\x82`@_ aJlV[a\"\x8A\x82\x87a0\xA9V[Ra\"\x95\x81\x86a0\xA9V[P\x01a\"PV[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90` a\x06\x19\x92\x81\x81R\x01\x90a\x14\xB9V[4a\x03\nW`\x806`\x03\x19\x01\x12a\x03\nW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa#a\x906\x90`\x04\x01a\t\xE0V[a#j6a\x03\xB8V[\x90`d5a#w\x81a\x02\xF9V[a#\x81\x82Qa5\xB3V[\x92_[\x83Q\x81\x10\x15a#\xD8W`\x01\x90a#\xBA\x84`\x01`\x01`\xA0\x1B\x03a#\xA6\x84\x89a0\xA9V[Q\x16a#\xB0a/\xACV[Pa\x04\xB6\x86a=~V[\x90Pa#\xC6\x82\x88a0\xA9V[Ra#\xD1\x81\x87a0\xA9V[P\x01a#\x84V[`@Q\x80a\x06q\x87\x82a# V[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a$+\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa$J\x906\x90`\x04\x01a\t\xE0V[\x90`D5a$W\x81a\x03\xAAV[a$a\x83Qa2\xEBV[\x92_\x92`\x01`\x01`\xA0\x1B\x03\x16\x91c\xFF\xFF\xFF\xFF\x16\x90[\x80Q\x84\x10\x15a%vW_\x83\x81R`\xA1` R`@\x90 a$\xB8\x90`\x01`\x01`\xA0\x1B\x03a$\xA2\x87\x85a0\xA9V[Q\x16`\x01\x80`\xA0\x1B\x03\x16_R` R`@_ \x90V[\x93\x84T\x94_\x95[\x80\x87\x10a%.W`\x01\x93\x94\x95\x96P\x80\x15_\x14a%\x06WPa$\xFD\x90Pg\r\xE0\xB6\xB3\xA7d\0\0[a$\xEF\x83\x89a0\xA9V[\x90`\x01`\x01`@\x1B\x03\x16\x90RV[\x01\x92\x91\x90a$vV[a\x04`a%'a$\xFD\x93a%\x1Ca$\xE5\x94a3\x98V[\x90_R` _ \x01\x90V[T` \x1C\x90V[\x80\x87\x16\x90\x80\x88\x18`\x01\x1C\x82\x01\x80\x92\x11a%qW\x82_R\x85c\xFF\xFF\xFF\xFF\x83` _ \x01T\x16\x11_\x14a%bWP\x95[\x95a$\xBFV[\x96P`\x01\x81\x01\x80\x91\x11\x15a%\\W[a3\x84V[`@Q\x80a\x06q\x87\x82a\x16\xFDV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a%\xA1\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW6`#\x82\x01\x12\x15a\x03\nW\x80`\x04\x015\x91a%\xCC\x83a\t\xC9V[\x91a%\xDA`@Q\x93\x84a\x03xV[\x83\x83R`$` \x84\x01\x94`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03\nW`$\x81\x01\x94[\x82\x86\x10a&\tWa\x05}\x85\x85a7%V[\x855`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW\x82\x01`\x80`#\x19\x826\x03\x01\x12a\x03\nW`@Q\x90a&6\x82a\x03BV[a&C6`$\x83\x01a\x04*V[\x82R`d\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa&h\x90`$6\x91\x84\x01\x01a\t\xE0V[` \x83\x01R`\x84\x81\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW`$\x91\x01\x016`\x1F\x82\x01\x12\x15a\x03\nW\x805a&\x9B\x81a\t\xC9V[\x91a&\xA9`@Q\x93\x84a\x03xV[\x81\x83R` \x80\x84\x01\x92`\x05\x1B\x82\x01\x01\x906\x82\x11a\x03\nW` \x01\x91[\x81\x83\x10a&\xE2WPPP`@\x82\x01R\x81R` \x95\x86\x01\x95\x01a%\xF8V[\x825`\x01`\x01`@\x1B\x03\x81\x16\x81\x03a\x03\nW\x81R` \x92\x83\x01\x92\x01a&\xC5V[4a\x03\nW` a'@a\x0CTa'\x186a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA1\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[`\x01`\x01`@\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a'n\x81a\x02\xF9V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW6`#\x83\x01\x12\x15a\x03\nW\x81`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW6`$\x83\x85\x01\x01\x11a\x03\nW`$a\x05}\x93\x01\x90a;6V[4a\x03\nW` `\x01`\x01`@\x1B\x03a'\xFBa'\xD36a\x14\x94V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16_\x90\x81R`\xA2\x86R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T\x16`@Q\x90\x81R\xF3[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a(\"\x81a\x02\xF9V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\nWa(A\x906\x90`\x04\x01a\x1D\xE9V[\x90a(Sa\r\xFE`\x04\x80`fT\x16\x14\x90V[a(_a\x07\x8D\x82a?\xB1V[`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x82\x16`\x04\x82\x01R` \x81\x80`$\x81\x01\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x05\xAEWa(\xC7\x91_\x91a\x1B\\WPa6.V[` \x82\x01\x90`\x01`\x01`\xA0\x1B\x03\x81\x16_[a(\xE2\x84\x86a1@V[\x90P\x81\x10\x15a*\x03W\x80a)\xFDa)\xF0\x87a)Ia\x08:a)\x15a\x07\xAC`\x01\x98a\x08\xDB\x8Da)\x0F\x88a1\x85V[\x97a1@V[a) a\x0E*a\x03\x99V[c\xFF\xFF\xFF\xFF\x16` \x84\x01\x90\x81R\x83Qa\x0E\xCC\x90a\x08+\x90a\x0E\xC1\x90`\x01`\x01`\xA0\x1B\x03\x16a\x0E\xA8V[a)ba)]a)Y\x83\x8Aa@\xB0V[\x15\x90V[a;\x8AV[`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x9C` R`@\x90 a)\x86\x90a\x084\x83a=~V[Pa)\x9C\x87a)\x97a\x1F\x99\x84a=~V[a@FV[P\x85\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^`@Q\x80a)\xCD\x85\x82a1\x1AV[\x03\x90\xA2`\x01`\x01`\xA0\x1B\x03\x87\x16_\x90\x81R`\x9E` R`@\x90 a\x1F:\x90a /V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[\x01a(\xD8V[\x84\x83a*/\x86a*\"a*\x1Ba\x08\xE6a\r\x9A\x87a1\x85V[\x91\x85a1@V[\x92\x90\x94`@\x81\x01\x90a4\x96V[\x82\x95\x91\x95;\x15a\x03\nW_\x94a*]\x86\x92`@Q\x98\x89\x97\x88\x96\x87\x95c\xAD\xCFs\xF7`\xE0\x1B\x87R`\x04\x87\x01a;\xA0V[\x03\x92Z\xF1\x80\x15a\x05\xAEWa \xC7W\0[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nWa*\x8Aa\x16X6a\x03\xF3V[_R`\x9A` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\nWa\x08:a*\xFCa*\xB46a\x18\xA4V[\x93\x91a*\xC5a\x07\x8D\x85\x97\x93\x97a?\xB1V[a\x0E\xCCa\x08+a\x0E\xC1`@Q\x96a*\xDB\x88a\x03\"V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x88Rc\xFF\xFF\xFF\xFF\x90\x94\x16` \x88\x01\x90\x81R\x93a\x0E\xA8V[a+\x05\x81a=~V[\x90_[\x83\x81\x10a+\x11W\0[`\x01\x90a+=a\x0F\xD7a+,\x86_R`\x99` R`@_ \x90V[a\x1F\xB0a\x08\xE6a\x08\xE1\x86\x8B\x8Da1uV[\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEEa+oa\x19\xD0a\x08\xE1\x84\x89\x8Ba1uV[\x03\x90\xA1\x01a+\x08V[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`@c\xFF\xFF\xFF\xFFa+\xA4`\x045a+\x9F\x81a\x02\xF9V[a<\x12V[\x83Q\x91\x15\x15\x82R\x90\x91\x16` \x82\x01R\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a+\xD2\x81a\x02\xF9V[`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R` `@_ T`@Q\x90\x81R\xF3[4a\x03\nW``6`\x03\x19\x01\x12a\x03\nW`\x045a,\r\x81a\x02\xF9V[a,\x166a\x03\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` Ra,3`@_ \x91a=~V[_R` R`@_ `@Q\x90\x81` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a,kWa\x06q\x85a\x16\x97\x81\x87\x03\x82a\x03xV[\x82T\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a,TV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW`\x045a,\x9E\x81a\x02\xF9V[a,\xE3`$5_T\x92a,\xC9`\xFF`\x08\x86\x90\x1C\x16\x15\x80\x95\x81\x96a-aW[\x81\x15a-AW[Pa<\xC7V[\x83a,\xDA`\x01`\xFF\x19_T\x16\x17_UV[a-*Wa=*V[a,\xE9W\0[a,\xF7a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x14\x8FV[a-<a\x01\0a\xFF\0\x19_T\x16\x17_UV[a=*V[0;\x15\x91P\x81a-SW[P_a,\xC3V[`\xFF\x16`\x01\x14\x90P_a-LV[`\x01`\xFF\x82\x16\x10\x91Pa,\xBCV[4a\x03\nW`@6`\x03\x19\x01\x12a\x03\nW\x7F*\xE9E\xC4\x0CD\xDC\x0E\xC2c\xF9V\t\xC3\xFD\xC6\x95.\n\xEF\xA2-ct\xE4O,\x99z\xCE\xDF\x85`@`\x045a-\xAF\x81a\x02\xF9V[a-\xFD`$5\x91a-\xBF\x83a\x02\xF9V[a-\xCBa\x07\x8D\x82a?\xB1V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x81\x81R`\x97` R\x86\x90 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x95\x90\x92\x16\x94\x90\x94\x17\x90Ua3\x1DV[\x82Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16` \x82\x01R\xA1\0[4a\x03\nW_6`\x03\x19\x01\x12a\x03\nW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045a.t\x81a\x02\xF9V[a.|aH\xF1V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a.\x94Wa\x05}\x90aIIV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\nW` 6`\x03\x19\x01\x12a\x03\nW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x05\xAEW_\x91a/qW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a/bWa\x05}\x90a=;V[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[\x90P` \x81=` \x11a/\xA4W[\x81a/\x8C` \x93\x83a\x03xV[\x81\x01\x03\x12a\x03\nWQa/\x9E\x81a\x02\xF9V[_a/IV[=\x91Pa/\x7FV[`@Q\x90a/\xB9\x82a\x03BV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[a\x04\xB6a/\xDA\x93\x92a\x04\xB0a/\xACV[\x90P\x90V[\x90\x81` \x91\x03\x12a\x03\nWQ\x80\x15\x15\x81\x03a\x03\nW\x90V[`@Q=_\x82>=\x90\xFD[\x15a0\tWV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a0\x1FWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[`@Q\x90a0;\x82a\x03\"V[_` \x83\x82\x81R\x01RV[\x90a0P\x82a\t\xC9V[a0]`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a0n`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a0~WPPPV[` \x90a0\x89a0.V[\x82\x82\x85\x01\x01R\x01a0rV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x80Q\x82\x10\x15a0\xBDW` \x91`\x05\x1B\x01\x01\x90V[a0\x95V[\x15a0\xC9WV[c\x93-\x94\xF7`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a0\xBDW`\x05\x1B\x81\x015\x90`>\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x90V[5a\x06\x19\x81a\x03\xAAV[\x15a1\x0BWV[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x91\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R`@\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x03\nWV[\x91\x90\x81\x10\x15a0\xBDW`\x05\x1B\x01\x90V[5a\x06\x19\x81a\x02\xF9V[\x90a1\x99\x82a\t\xC9V[a1\xA6`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a1\xB7`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a1\xC7WPPPV[\x80``` \x80\x93\x85\x01\x01R\x01a1\xBBV[` \x81\x83\x03\x12a\x03\nW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03\nW\x81Q\x90a2\x0C\x82a\t\xC9V[\x92a2\x1A`@Q\x94\x85a\x03xV[\x82\x84R` \x80\x85\x01\x93`\x05\x1B\x82\x01\x01\x90\x82\x82\x11a\x03\nW` \x81\x01\x93[\x82\x85\x10a2FWPPPPP\x90V[\x84Q`\x01`\x01`@\x1B\x03\x81\x11a\x03\nW\x82\x01\x84`?\x82\x01\x12\x15a\x03\nW` \x81\x01Q\x90a2r\x82a\t\xC9V[\x91a2\x80`@Q\x93\x84a\x03xV[\x80\x83R` \x80\x80\x85\x01\x92`\x05\x1B\x84\x01\x01\x01\x91\x87\x83\x11a\x03\nW`@\x01\x90[\x82\x82\x10a2\xB6WPPP\x81R` \x94\x85\x01\x94\x01a27V[\x81Q\x81R` \x91\x82\x01\x91\x01a2\x9EV[\x90\x91a2\xDDa\x06\x19\x93`@\x84R`@\x84\x01\x90a \xEFV[\x91` \x81\x84\x03\x91\x01Ra\x15\xEEV[\x90a2\xF5\x82a\t\xC9V[a3\x02`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a3\x13`\x1F\x19\x91a\t\xC9V[\x01\x90` 6\x91\x017V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x81\x81R`\x97` R`@\x90 T\x90\x91\x16\x80a/\xDAWP\x90V[\x15a3IWV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[\x15a3_WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a3uWV[c\xEB\xBF\xF4\x97`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[_\x19\x81\x01\x91\x90\x82\x11a%qWV[\x15a3\xADWV[c\x9F\x1C\x80S`\xE0\x1B_R`\x04_\xFD[\x15a3\xC3WV[c\x13S`1`\xE0\x1B_R`\x04_\xFD[\x15a3\xD9WV[c1\xBC4'`\xE1\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a%qWV[`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14a%qW_\x03\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a%qWV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x82\x16` \x80\x83\x01\x91\x90\x91R\x90\x92\x01Qc\xFF\xFF\xFF\xFF\x90\x81\x16`@\x84\x01R\x92\x16``\x82\x01R`\x01`\x01`@\x1B\x03\x90\x92\x16`\x80\x83\x01R\x91\x90\x91\x16`\xA0\x82\x01R`\xC0\x01\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\nW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\nW` \x01\x91\x816\x03\x83\x13a\x03\nWV[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x81R\x82Q\x90\x91\x16` \x80\x83\x01\x91\x90\x91R\x90\x91\x01Qc\xFF\xFF\xFF\xFF\x16`@\x82\x01R\x90\x95\x94\x92\x93\x91\x80`\xC0\x83\x01`\xC0``\x85\x01RR`\xE0\x82\x01\x96\x90_[\x81\x81\x10a5\x82WPPP\x80\x86\x03`\x80\x82\x01R` \x80\x85Q\x97\x88\x81R\x01\x94\x01_\x96[\x80\x88\x10a5jWPPa\x06\x19\x94\x95P`\xA0\x81\x85\x03\x91\x01Ra4\xC8V[\x90\x94` \x80`\x01\x92\x88Q\x81R\x01\x96\x01\x97\x01\x96\x90a5NV[\x90\x91\x97` a5\xA9`\x01\x92\x8B5a5\x98\x81a\x02\xF9V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x90V[\x99\x01\x92\x91\x01a5-V[\x90a5\xBD\x82a\t\xC9V[a5\xCA`@Q\x91\x82a\x03xV[\x82\x81R\x80\x92a5\xDB`\x1F\x19\x91a\t\xC9V[\x01\x90_[\x82\x81\x10a5\xEBWPPPV[` \x90a5\xF6a/\xACV[\x82\x82\x85\x01\x01R\x01a5\xDFV[\x15a6\tWV[cX\\\xFB/`\xE0\x1B_R`\x04_\xFD[\x15a6\x1FWV[cH\xF5\xC3\xED`\xE0\x1B_R`\x04_\xFD[\x15a65WV[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[\x90`@Qa6Q\x81a\x03BV[`@c\xFF\xFF\xFF\xFF\x82\x94T`\x01`\x01`@\x1B\x03\x81\x16\x84R\x80\x83\x1C`\x0F\x0B` \x85\x01R`\xC0\x1C\x16\x91\x01RV[\x15a6\x82WV[c%\x13\x1DO`\xE0\x1B_R`\x04_\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a%qWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a%qWV[\x91` \x90\x82\x81R\x01\x91\x90_[\x81\x81\x10a6\xDCWPPP\x90V[\x90\x91\x92` \x80`\x01\x92c\xFF\xFF\xFF\xFF\x875a6\xF5\x81a\x03\xAAV[\x16\x81R\x01\x94\x01\x92\x91\x01a6\xCFV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R`@` \x82\x01\x81\x90Ra\x06\x19\x93\x91\x01\x91a6\xC3V[a7V\x91a7:a\r\xFE`\x01\x80`fT\x16\x14\x90V[a7Fa\x1B\x13\x83a?\xB1V[a7O\x82a<\x12V[\x93\x90a:\xAFV[_\x92[\x81Q\x84\x10\x15a:\xA9Wa7\x8B` a7q\x86\x85a0\xA9V[Q\x01QQ`@a7\x81\x87\x86a0\xA9V[Q\x01QQ\x14a3XV[a7\x95\x84\x83a0\xA9V[QQ\x90a7\xC5a\x08:a7\xB1a\x0E\xA8\x85Q`\x01\x80`\xA0\x1B\x03\x16\x90V[a\x0E\xCCa\x08+` \x87\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[a7\xCF\x82\x85a@\xB0V[_[` a7\xDD\x88\x87a0\xA9V[Q\x01QQ\x81\x10\x15a:\x9BW\x80\x7F\x14\x87\xAFT\x18\xC4~\xE5\xEAE\xEFJ\x939\x86h\x12\x08\x90wJ\x9E\x13H~a\xE9\xDC;\xAFv\xDD\x81\x88a9\xBC\x8Ba9x\x89a8\x85\x8Da9r\x8E\x8Da8:a\x0B\xC9`\x01\x9F` a82\x8C\x89a0\xA9V[Q\x01Qa0\xA9V[\x97\x88\x94a8G\x86\x8DaDjV[a8U\x86\x8Da\x04\xB6\x87a=~V[\x9D\x90\x8E\x99\x81\x99\x8B\x96a8}a8wa8q` \x8B\x01Q`\x0F\x0B\x90V[`\x0F\x0B\x90V[\x15a:\xC5V[\x87\x8B\x8AaI\x91V[\x90a8\xC0a8\xB6a8\x9D\x89Q`\x01`\x01`@\x1B\x03\x16\x90V[a8\xB0a\x0C\x9D\x88`@a82\x8D\x8Da0\xA9V[\x90aI\xF0V[`\x0F\x0B` \x89\x01RV[a8\xDBa8\xD4a8q` \x8A\x01Q`\x0F\x0B\x90V[\x15\x15a:\xDBV[` \x87\x01Q`\x0F\x0B\x80_\x81\x12\x15a:\x1CWPPP_\x14a9\xC5WPPPPPa9*a9\x1B\x84a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[a9$\x83a=~V[\x90aJ!V[a\x16Xa9ea9`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a6\xA9V[a6\x91V[c\xFF\xFF\xFF\xFF\x16`@\x87\x01RV[\x87aAIV[a9\x89a9\x84\x8Ba=~V[a?\x87V[\x93a\x13y`@a9\xB0a9\xA3\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[` \x85\x01Q`\x0F\x0Ba\x0C\xFFV[\x92\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[\x03\x90\xA1\x01a7\xD1V[a82a\x10\xEF\x94a\x0C\x9D\x94a:\0a:\x05\x98a\x11\x15` `@\x97\x01\x91a\x0C\xFF` a9\xF7\x85Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x01Q`\x0F\x0B\x90V[a0\xA9V[_` \x86\x01RCc\xFF\xFF\xFF\xFF\x16`@\x86\x01Ra=~V[\x94P\x95P\x95PPP_\x91P\x13a:4W[PPa=~V[a:\x94\x91a:\x88a:ca9e\x93a\x0C\x9Da\r\x05` \x8C\x01\x92a:^\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[a@\x95V[`\x01`\x01`@\x1B\x03a:\x7Fa\x04`\x8BQ`\x01`\x01`@\x1B\x03\x16\x90V[\x91\x16\x11\x15a:\xF1V[c\xFF\xFF\xFF\xFFC\x16a6\xA9V[\x8E_a:-V[PP\x93`\x01\x91P\x01\x92a7YV[PPPPV[\x15a:\xB6WV[c\xFAU\xFC\x81`\xE0\x1B_R`\x04_\xFD[\x15a:\xCCWV[c\r\x8F\xCB\xE3`\xE4\x1B_R`\x04_\xFD[\x15a:\xE2WV[cF\x06\x17\x93`\xE1\x1B_R`\x04_\xFD[\x15a:\xF8WV[cl\x9B\xE0\xBF`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x90\x94\x16\x82R\x91\x90\x91R a\x06\x19\x90a@YV[\x90\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x91a;ea\x07\x8D\x82a?\xB1V[a;\x85`@Q\x92\x83\x92` \x84R`\x01\x80`\xA0\x1B\x03\x16\x95` \x84\x01\x91a4\xC8V[\x03\x90\xA2V[\x15a;\x91WV[clln'`\xE1\x1B_R`\x04_\xFD[\x93\x91a\x06\x19\x95\x93a;\xC6\x92`\x01\x80`\xA0\x1B\x03\x16\x86R``` \x87\x01R``\x86\x01\x91a6\xC3V[\x92`@\x81\x85\x03\x91\x01Ra4\xC8V[\x90`@Qa;\xE1\x81a\x03]V[``c\xFF\xFF\xFF\xFF\x82\x94T\x81\x81\x16\x84R`\xFF\x81` \x1C\x16\x15\x15` \x85\x01R\x81\x81`(\x1C\x16`@\x85\x01R`H\x1C\x16\x91\x01RV[`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90```@Q\x92a<5\x84a\x03]V[Ta<\x8Da<\x83a<}c\xFF\xFF\xFF\xFF\x84\x16\x80\x88R`\xFF\x85` \x1C\x16\x15\x15\x97\x88` \x82\x01Rc\xFF\xFF\xFF\xFF\x80\x87`(\x1C\x16\x96\x87`@\x84\x01R`H\x1C\x16\x96\x87\x91\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x95\x15\x15\x90V[\x92c\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x90\x81a<\xB6W[Pa<\xA7WP\x91\x90V[\x91\x92PPc\xFF\xFF\xFF\xFF\x16`\x01\x91V[c\xFF\xFF\xFF\xFF\x16\x90PC\x10\x15_a<\x9DV[\x15a<\xCEWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[a=6a\x03\xA8\x92a?UV[aIIV[a=L`fT\x19\x82\x19\x81\x16\x14a0\x18V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2V[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra=\xC4`@\x82a\x03xV[Q\x90Q\x90` \x81\x10a=\xD4WP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x92\x91a\x1D\xA8a>\x91\x91a=\xF3a0.V[Pa=\xFCa/\xACV[Pa\x0C?a>!a\x0CT\x83a\x0C?\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA1` R`@_ \x90V[\x94a\x1D\x9Ba>Fa\x1C\xC4\x85a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x98a>aa>Ra\x03\x99V[`\x01`\x01`@\x1B\x03\x90\x99\x16\x89RV[a>x` \x89\x01\x9A\x8B\x90`\x01`\x01`@\x1B\x03\x16\x90RV[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\xA0` R`@\x90 \x90V[\x92`@\x84\x01\x90a>\xA8a\x08+\x83Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10a?\x1BW_\x80\x92a>\xC2\x87Q`\x01`\x01`@\x1B\x03\x16\x90V[\x92a>\xE6a>\xD9` \x8A\x01\x95a\x0C\xFF\x87Q`\x0F\x0B\x90V[`\x01`\x01`@\x1B\x03\x16\x89RV[\x83Q`\x0F\x0B\x90\x83\x82\x12a>\xFCW[PPRR\x91\x90V[a\x11\x15a?\x14\x92a:^\x83Q`\x01`\x01`@\x1B\x03\x16\x90V[_\x80a>\xF4V[PP\x91\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[a?\x8Fa0.V[Pc\xFF\xFF\xFF\xFF`@Q\x91a?\xA2\x83a\x03\"V[\x80``\x1C\x83R\x16` \x82\x01R\x90V[`@Qc\x1B\xEB+\x97`\xE3\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R3`$\x82\x01R0`D\x82\x01R_\x805`\x01`\x01`\xE0\x1B\x03\x19\x16`d\x83\x01R\x90\x91` \x91\x83\x91`\x84\x91\x83\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xF1\x90\x81\x15a\x05\xAEW_\x91a@-WP\x90V[a\x06\x19\x91P` =` \x11a\x05\xA7Wa\x05\x99\x81\x83a\x03xV[a\x06\x19\x91`\x01`\x01`\xA0\x1B\x03\x16\x90aJ\x9EV[\x80T\x80a@oWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[\x80_\x19\x81\x01\x11a%qW`\x01`\x01`@\x1B\x03\x91_R_\x19\x90` _ \x01\x01T` \x1C\x16\x90V[`\x01`\x01`@\x1B\x03\x91\x82a@\xAC\x92\x16`\x0F\x0Ba4\x1FV[\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9E` R`@\x90 \x90a@\xD0\x90a=~V[_R` R` `@_ `@Q\x90a@\xE8\x82a\x03\"V[T\x90c\xFF\xFF\xFF\xFF`\xFF\x83\x16\x15\x15\x92\x83\x83R`\x08\x1C\x16\x92\x83\x91\x01R\x15\x15\x90\x81\x15aA\x0FWP\x90V[c\xFF\xFF\xFF\xFF\x16C\x11\x15\x91\x90PV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90aA2\x81\x84aK\xCCV[\x92\taA;W\x90V[`\x01\x81\x01\x80\x91\x11a%qW\x90V[\x93\x90\x92` aAoa\x1C\xC4\x85a\x0C?\x89`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x91\x01\x80Q`\x01`\x01`@\x1B\x03\x90\x81\x16\x92\x16\x82\x90\x03aC\x12W[PPaB\x10\x81aA\xB0\x84a\x0C?\x87a\x1D\x9B\x8A`\x01\x80`\xA0\x1B\x03\x16_R`\xA0` R`@_ \x90V[\x81Q\x81T` \x84\x01Q`@\x94\x85\x01Qc\xFF\xFF\xFF\xFF`\xC0\x1B`\xC0\x91\x90\x91\x1B\x16w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x91\x90\x95\x1B\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x91\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x17\x91\x90\x91\x17\x90UV[` \x81\x01Q`\x0F\x0B\x15aBzWP\x82aBXaBr\x92aBG\x85a\x1D\x9BaBw\x98`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90aJ\x9EV[P`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9D` R`@\x90 \x90V[aJ\x9EV[PV[Q`\x01`\x01`@\x1B\x03\x16\x15aB\x8EWPPPV[aB\xC3\x90a\x1F\x83a\x08\xE6aB\xB6\x85a\x1D\x9B\x88`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90V[\x92`\x01`\x01`\xA0\x1B\x03\x16\x90V[P`\x01`\x01`\xA0\x1B\x03\x82\x16_\x90\x81R`\x9F` R`@\x90 aB\xE6\x90\x82\x90a\x1D\x9BV[T\x15aB\xF0WPPV[aC\raBw\x92`\x01\x80`\xA0\x1B\x03\x16_R`\x9D` R`@_ \x90V[aM\xECV[aCwaC\xA8\x91a\x0C\x9D\x7F\xAC\xF9\t_\xEB:7\x0C\x9C\xF6\x92B\x1Ci\xEF2\rM\xB5\xC6nj})\xC7iN\xB0#d\xFCU\x94aC\\\x88a\x0C?\x8C`\x01\x80`\xA0\x1B\x03\x16_R`\xA2` R`@_ \x90V[\x90`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x8A\x16\x82R\x87\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1_\x80aA\x88V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xA1` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R \x7F\x1CdX\x07\x9AA\x07}\0<\x11\xFA\xF9\xBF\t~i;\xD6yy\xE4\xE6P\x0B\xAC{)\xDBw\x9B\\\x93aDK\x91aD\x18\x90`\x01`\x01`@\x1B\x03\x83\x16\x90c\xFF\xFF\xFF\xFFC\x16\x90aN\xFBV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x85\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x16\x91\x83\x01\x91\x90\x91R\x81\x90``\x82\x01\x90V[\x03\x90\xA1V[_\x19\x81\x14a%qW`\x01\x01\x90V[\x80\x15a%qW_\x19\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x80\x82\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x93\x92\x91\x90aD\x9E\x90a\x1C\xF3V[\x93[\x84\x15\x15\x80aEWW[\x15aEPWaD\xD4aD\xCF\x84a\x0C?\x85`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aL}V[aD\xDF\x84\x82\x85a=\xE2V[\x91aD\xF4a\x08+`@\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10aEFWaE:\x92aE@\x94\x92\x87aE\x0E\x93\x88aAIV[aE4aE/\x86a\x0C?\x87`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aL\xBEV[PaDPV[\x94aD^V[\x93aD\xA0V[PPPPPP\x90PV[PPP\x90PV[Pa\xFF\xFF\x81\x10aD\xA9V[`\x01`\x01`\xA0\x1B\x03\x81\x81\x16_\x90\x81R`\xA3` \x90\x81R`@\x80\x83 \x93\x86\x16\x83R\x92\x90R\x90\x81 \x90\x94\x90aE\x94\x90a\x1C\xF3V[\x94[\x85\x15\x15\x80aF?W[\x15aF7WaE\xC5aD\xCF\x85a\x0C?\x86`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[aE\xD0\x85\x82\x86a=\xE2V[\x91aE\xE5a\x08+`@\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[C\x10aF,WaF \x92aF&\x94\x92\x88aE\xFF\x93\x89aAIV[aE4aE/\x87a\x0C?\x88`\x01\x80`\xA0\x1B\x03\x16_R`\xA3` R`@_ \x90V[\x95aD^V[\x94aE\x96V[PPPP\x93PPPPV[P\x93PPPPV[Pa\xFF\xFF\x85\x16\x81\x10aE\x9FV[\x90\x7FN\x85u\x1Dc1Pllb3_ ~\xB3\x1F\x12\xA6\x1EW\x0F4\xF5\xC1v@0\x87\x85\xC6\xD4\xDB\x91aDKaG\x99aF\x97aF\x92\x84`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` R`@_ \x90V[a;\xD4V[a\x08?``\x82\x01\x91aF\xAD\x83Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x81\x16\x15\x15\x90\x81aG\xF4W[PaG\xCAW[c\xFF\xFF\xFF\xFF\x87\x16`@\x82\x01RaG\x0FaG\x05a9`\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0c\xFF\xFF\xFF\xFFC\x16a6\xA9V[c\xFF\xFF\xFF\xFF\x16\x84RV[`\x01`\x01`\xA0\x1B\x03\x86\x16_\x90\x81R`\x9B` R`@\x90 \x81Q\x81T` \x80\x85\x01Q`@\x86\x01Q``\x90\x96\x01Ql\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0`H\x91\x90\x91\x1B\x16h\xFF\xFF\xFF\xFF\0\0\0\0\0`(\x97\x90\x97\x1B\x96\x90\x96\x16d\xFF\0\0\0\0\x91\x15\x15\x90\x92\x1B\x16l\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x92\x16c\xFF\xFF\xFF\xFF\x90\x93\x16\x92\x90\x92\x17\x17\x17\x91\x90\x91\x17\x90UV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84Rc\xFF\xFF\xFF\xFF\x94\x85\x16` \x85\x01R\x93\x16\x92\x82\x01\x92\x90\x92R\x90\x81\x90``\x82\x01\x90V[aG\xE8aG\xDE`@\x83\x01Qc\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFF\x16\x82RV[`\x01` \x82\x01RaF\xC3V[c\xFF\xFF\xFF\xFF\x16\x90PC\x10\x15_aF\xBDV[\x91\x90\x91_\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a%qWV[\x80T\x90\x91`\x01`\x01`\xFF\x1B\x03\x81\x11aH\x88Wa8qaHMaHHaHT\x93\x85`\x0F\x0BaH\x05V[aM.V[\x92`\x80\x1D\x90V[\x81`\x0F\x0B\x12\x15aHyW`\x01aHu\x92\x01\x90`\x0F\x0B_R` R`@_ \x90V[T\x90V[c-\x04\x83\xC5`\xE2\x1B_R`\x04_\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`(`$\x82\x01R\x7FSafeCast: value doesn't fit in a`D\x82\x01Rg7\x104\xB7:\x19\x1A\x9B`\xC1\x1B`d\x82\x01R`\x84\x90\xFD[a\x06\x19\x91`\x01`\x01`\xA0\x1B\x03\x16\x90aM\xECV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aI\x05WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90aI\x9EaI\xC6\x92a=~V[_R`\x99` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[\x91\x82aI\xE8W[P\x81aI\xD7WP\x90V[`\x01`\x01`@\x1B\x03\x91PQ\x16\x15\x15\x90V[\x91P_aI\xCDV[`\x01`\x01`@\x1B\x03\x80\x91\x16`\x0F\x0B\x91\x16`\x0F\x0B\x03`\x01`\x01`\x7F\x1B\x03\x81\x13`\x01`\x01`\x7F\x1B\x03\x19\x82\x12\x17a%qW\x90V[\x90\x81T`\x80\x1D\x90aJ@\x82`\x01\x85\x01\x90`\x0F\x0B_R` R`@_ \x90V[U\x81T`\x01`\x01`\x80\x1B\x03\x16`\x01\x90\x91\x01`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x90UV[\x80T\x82\x10\x15a0\xBDW_R` _ \x01\x90_\x90V[\x91aJ\x9A\x91\x83T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90UV[_\x82\x81R`\x01\x82\x01` R`@\x90 TaK\x01W\x80T\x90`\x01`@\x1B\x82\x10\x15a\x03=W\x82aJ\xECaJ\xD6\x84`\x01\x80\x96\x01\x85U\x84aJlV[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80T\x92_R\x01` R`@_ U`\x01\x90V[PP_\x90V[\x81\x15aK\x11W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x15a\x03\nWV[_\x19g\r\xE0\xB6\xB3\xA7d\0\0\x82\t\x91g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x82\x80\x85\x10\x94\x03\x93\x80\x85\x03\x94\x14aK\xC0Wg\r\xE0\xB6\xB3\xA7d\0\0\x82\x91aKl\x86\x84\x11aK%V[\t`\x01\x82\x19\x01\x82\x16\x80\x92\x04`\x02\x81`\x03\x02\x18\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x82\x02`\x02\x03\x02\x80\x91\x02`\x02\x03\x02\x93`\x01\x83\x80_\x03\x04\x01\x90\x84\x83\x11\x90\x03\x02\x92\x03\x04\x17\x02\x90V[P\x90a\x06\x19\x92PaK\x07V[_\x19\x82\x82\t\x82\x82\x02\x91\x82\x80\x83\x10\x92\x03\x91\x80\x83\x03\x92\x14aL5W\x81g\r\xE0\xB6\xB3\xA7d\0\0\x11\x15a\x03\nW\x7F\xAC\xCB\x18\x16[\xD6\xFE1\xAE\x1C\xF3\x18\xDC[Q\xEE\xE0\xE1\xBAV\x9B\x88\xCDt\xC1w;\x91\xFA\xC1\x06i\x93g\r\xE0\xB6\xB3\xA7d\0\0\x91\t\x90\x82\x82\x11\x90\x03`\xEE\x1B\x91\x03`\x12\x1C\x17\x02\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x92PP\x04\x90V[\x90\x91_\x19\x83\x83\t\x92\x80\x83\x02\x92\x83\x80\x86\x10\x95\x03\x94\x80\x86\x03\x95\x14aLpW\x90\x82\x91aKl\x86\x84\x11aK%V[PP\x90a\x06\x19\x92PaK\x07V[aL\x92\x81T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[aL\xAFW\x80T`\x0F\x0B_\x90\x81R`\x01\x90\x91\x01` R`@\x90 T\x90V[c\x1E\xD9P\x95`\xE1\x1B_R`\x04_\xFD[\x90aL\xD4\x82T\x80`\x0F\x0B\x90`\x80\x1D`\x0F\x0B\x13\x15\x90V[aL\xAFW\x81T`\x0F\x0B\x91`\x01\x81\x01\x92_aM\x0F\x82aL\xFD\x81\x88\x90`\x0F\x0B_R` R`@_ \x90V[T\x96\x90`\x0F\x0B_R` R`@_ \x90V[U`\x01`\x01`\x01`\x80\x1B\x03\x19\x83T\x16\x91\x01`\x01`\x01`\x80\x1B\x03\x16\x17\x90UV[`\x01`\x01`\x7F\x1B\x03\x19\x81\x12\x15\x80aM\xA0W[\x15aMKW`\x0F\x0B\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FSafeCast: value doesn't fit in 1`D\x82\x01Rf28 bits`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[P`\x01`\x01`\x7F\x1B\x03\x81\x13\x15aM@V[\x80T\x80\x15aM\xD8W_\x19\x01\x90aM\xC7\x82\x82aJlV[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UUV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[`\x01\x81\x01\x91\x80_R\x82` R`@_ T\x92\x83\x15\x15_\x14aN\x87W_\x19\x84\x01\x84\x81\x11a%qW\x83T_\x19\x81\x01\x94\x90\x85\x11a%qW_\x95\x85\x83a\x1D\x9B\x94aN:\x98\x03aN@W[PPPaM\xB1V[U`\x01\x90V[aNpaNj\x91aNaaNWaN~\x95\x88aJlV[\x90T\x90`\x03\x1B\x1C\x90V[\x92\x83\x91\x87aJlV[\x90aJ\x81V[\x85\x90_R` R`@_ \x90V[U_\x80\x80aN2V[PPPP_\x90V[\x15aN\x96WV[c\x15\x1B\x8E?`\xE1\x1B_R`\x04_\xFD[\x80T`\x01`@\x1B\x81\x10\x15a\x03=WaN\xC2\x91`\x01\x82\x01\x81UaJlV[aN\xE8W\x81Q` \x92\x83\x01Q\x90\x92\x1Bc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90UV[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x80T\x80aO7W[PaO2a\x03\xA8\x93aO\"aO\x16a\x03\x99V[c\xFF\xFF\xFF\xFF\x90\x95\x16\x85RV[`\x01`\x01`\xE0\x1B\x03\x16` \x84\x01RV[aN\xA5V[\x80_\x19\x81\x01\x11a%qW\x81_Rc\xFF\xFF\xFF\xFFaO\x92a\x08+_\x19\x84` _ \x01\x01a\x08?aO\x84`@Q\x92aOk\x84a\x03\"V[T\x86\x81\x16\x90\x81\x85R` \x1C` \x85\x01Rc\xFF\xFF\xFF\xFF\x16\x90V[\x85\x89\x16\x95\x86\x91\x16\x11\x15aN\x8FV[\x03aO\x03Wa\x03\xA8\x93\x92P\x90a%\x1CaO\xAA\x92a3\x98V[\x90c\xFF\xFF\xFF\xFF\x82T\x91\x81\x19\x90` \x1B\x16\x91\x16\x17\x90UV\xFE\xA2dipfsX\"\x12 \xDE\x0E\xEAR\x98\xEB^\x93\xBB\x99\x91\x89\x9E\x83\xB6\x1E\x9Dn\xBF\x05\xE0\x96\x99\xF9pB\x9FKA\x94cIdsolcC\0\x08\x1B\x003",
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                (value.avs, value.id)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { avs: tuple.0, id: tuple.1 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
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
                    "OperatorSet(address avs,uint32 id)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.avs,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.id, out);
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategiesMustBeInAscendingOrder>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategiesMustBeInAscendingOrder) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategiesMustBeInAscendingOrder {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategiesMustBeInAscendingOrder {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyAlreadyInOperatorSet>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategyAlreadyInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategyAlreadyInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyAlreadyInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<StrategyNotInOperatorSet>
        for UnderlyingRustTuple<'_> {
            fn from(value: StrategyNotInOperatorSet) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for StrategyNotInOperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StrategyNotInOperatorSet {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AVSMetadataURIUpdated(address,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                168u8,
                156u8,
                29u8,
                194u8,
                67u8,
                216u8,
                144u8,
                138u8,
                150u8,
                221u8,
                132u8,
                148u8,
                75u8,
                204u8,
                151u8,
                214u8,
                188u8,
                106u8,
                192u8,
                13u8,
                215u8,
                142u8,
                32u8,
                98u8,
                21u8,
                118u8,
                190u8,
                106u8,
                60u8,
                148u8,
                55u8,
                19u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AVSRegistrarSet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8,
                233u8,
                69u8,
                196u8,
                12u8,
                68u8,
                220u8,
                14u8,
                194u8,
                99u8,
                249u8,
                86u8,
                9u8,
                195u8,
                253u8,
                198u8,
                149u8,
                46u8,
                10u8,
                239u8,
                162u8,
                45u8,
                99u8,
                116u8,
                228u8,
                79u8,
                44u8,
                153u8,
                122u8,
                206u8,
                223u8,
                133u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
                    effectBlock: data.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "AllocationUpdated(address,(address,uint32),address,uint64,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                20u8,
                135u8,
                175u8,
                84u8,
                24u8,
                196u8,
                126u8,
                229u8,
                234u8,
                69u8,
                239u8,
                74u8,
                147u8,
                57u8,
                134u8,
                104u8,
                18u8,
                8u8,
                144u8,
                119u8,
                74u8,
                158u8,
                19u8,
                72u8,
                126u8,
                97u8,
                233u8,
                220u8,
                59u8,
                175u8,
                118u8,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.effectBlock),
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorAddedToOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                67u8,
                35u8,
                46u8,
                223u8,
                144u8,
                113u8,
                117u8,
                61u8,
                35u8,
                33u8,
                229u8,
                250u8,
                126u8,
                1u8,
                131u8,
                99u8,
                238u8,
                36u8,
                142u8,
                95u8,
                33u8,
                66u8,
                230u8,
                192u8,
                142u8,
                221u8,
                50u8,
                101u8,
                191u8,
                180u8,
                137u8,
                94u8,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
            fn from(
                this: &OperatorAddedToOperatorSet,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorRemovedFromOperatorSet(address,(address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                173u8,
                52u8,
                195u8,
                7u8,
                11u8,
                225u8,
                223u8,
                251u8,
                202u8,
                164u8,
                153u8,
                208u8,
                0u8,
                186u8,
                43u8,
                141u8,
                152u8,
                72u8,
                174u8,
                252u8,
                172u8,
                48u8,
                89u8,
                223u8,
                36u8,
                93u8,
                217u8,
                92u8,
                78u8,
                206u8,
                20u8,
                254u8,
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
        impl alloy_sol_types::private::IntoLogData for OperatorRemovedFromOperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorRemovedFromOperatorSet>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorRemovedFromOperatorSet,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "OperatorSetCreated((address,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                98u8,
                146u8,
                133u8,
                234u8,
                210u8,
                51u8,
                90u8,
                224u8,
                147u8,
                63u8,
                134u8,
                237u8,
                42u8,
                230u8,
                51u8,
                33u8,
                247u8,
                175u8,
                119u8,
                180u8,
                230u8,
                234u8,
                171u8,
                196u8,
                44u8,
                5u8,
                120u8,
                128u8,
                151u8,
                126u8,
                108u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { operatorSet: data.0 }
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StrategyAddedToOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                178u8,
                96u8,
                254u8,
                10u8,
                241u8,
                147u8,
                219u8,
                95u8,
                73u8,
                134u8,
                119u8,
                13u8,
                131u8,
                27u8,
                218u8,
                78u8,
                164u8,
                96u8,
                153u8,
                220u8,
                129u8,
                126u8,
                139u8,
                103u8,
                22u8,
                220u8,
                174u8,
                138u8,
                248u8,
                232u8,
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
                    operatorSet: data.0,
                    strategy: data.1,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
            fn from(
                this: &StrategyAddedToOperatorSet,
            ) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StrategyRemovedFromOperatorSet((address,uint32),address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                123u8,
                75u8,
                7u8,
                61u8,
                128u8,
                220u8,
                172u8,
                85u8,
                161u8,
                17u8,
                119u8,
                216u8,
                69u8,
                154u8,
                217u8,
                246u8,
                100u8,
                206u8,
                235u8,
                145u8,
                247u8,
                31u8,
                39u8,
                22u8,
                123u8,
                177u8,
                79u8,
                129u8,
                82u8,
                167u8,
                238u8,
                238u8,
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
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
        impl From<&StrategyRemovedFromOperatorSet>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyRemovedFromOperatorSet,
            ) -> alloy_sol_types::private::LogData {
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
constructor(address _delegation, address _pauserRegistry, address _permissionController, uint32 _DEALLOCATION_DELAY, uint32 _ALLOCATION_CONFIGURATION_DELAY);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegation: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub _permissionController: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._permissionController,
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
    /**Function with signature `addStrategiesToOperatorSet(address,uint32,address[])` and selector `0x50feea20`.
```solidity
function addStrategiesToOperatorSet(address avs, uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetCall {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategiesToOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetCall) -> Self {
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategiesToOperatorSetCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<addStrategiesToOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategiesToOperatorSetReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesToOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `createOperatorSets(address,(uint32,address[])[])` and selector `0x261f84e0`.
```solidity
function createOperatorSets(address avs, IAllocationManagerTypes.CreateSetParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub avs: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsCall) -> Self {
                    (value.avs, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<createOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::CreateSetParams,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `deregisterFromOperatorSets((address,address,uint32[]))` and selector `0x6e3492b5`.
```solidity
function deregisterFromOperatorSets(IAllocationManagerTypes.DeregisterParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterFromOperatorSetsCall {
        pub params: <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsCall) -> Self {
                    (value.params,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterFromOperatorSetsCall {
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
            impl ::core::convert::From<deregisterFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterFromOperatorSetsCall {
            type Parameters<'a> = (IAllocationManagerTypes::DeregisterParams,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterFromOperatorSets((address,address,uint32[]))";
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getAVSRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAVSRegistrarReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAVSRegistrarReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAVSRegistrarCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAVSRegistrarReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getAllocatedSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                OperatorSet,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocatedStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStrategiesCall {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getAllocatedStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocatedStrategiesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocatedStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getAllocatedStrategiesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocatedStrategiesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type UnderlyingRustTuple<'a> = (
                <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationReturn;
            type ReturnTuple<'a> = (IAllocationManagerTypes::Allocation,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationDelayReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
    /**Function with signature `getAllocations(address[],(address,uint32),address)` and selector `0x8ce64854`.
```solidity
function getAllocations(address[] memory operators, OperatorSet memory operatorSet, address strategy) external view returns (IAllocationManagerTypes.Allocation[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getAllocationsCall {
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IAllocationManagerTypes::Allocation as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getAllocationsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getAllocationsReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getAllocationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudeReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudes_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Call) -> Self {
                    (value.operators, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_0Call {
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
            impl ::core::convert::From<getMaxMagnitudes_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_0Return {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    pub struct getMaxMagnitudes_1Call {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudes_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Call) -> Self {
                    (value.operator, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_1Call {
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
            impl ::core::convert::From<getMaxMagnitudes_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudes_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudes_1Return {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudes_1Return;
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
    /**Function with signature `getMaxMagnitudesAtBlock(address,address[],uint32)` and selector `0x94d7d00c`.
```solidity
function getMaxMagnitudesAtBlock(address operator, address[] memory strategies, uint32 blockNumber) external view returns (uint64[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMaxMagnitudesAtBlockCall {
        pub operator: alloy::sol_types::private::Address,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getMaxMagnitudesAtBlockCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockCall) -> Self {
                    (value.operator, value.strategies, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtBlockCall {
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
            impl ::core::convert::From<getMaxMagnitudesAtBlockReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMaxMagnitudesAtBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMaxMagnitudesAtBlockReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMaxMagnitudesAtBlockReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<64>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMemberCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMemberCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            impl ::core::convert::From<getMemberCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMemberCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMemberCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getMemberCountCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMemberCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getMembersCall> for UnderlyingRustTuple<'_> {
                fn from(value: getMembersCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getMembersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMembersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `getMinimumSlashableStake((address,uint32),address[],address[],uint32)` and selector `0x2bab2c4a`.
```solidity
function getMinimumSlashableStake(OperatorSet memory operatorSet, address[] memory operators, address[] memory strategies, uint32 futureBlock) external view returns (uint256[][] memory slashableStake);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub futureBlock: u32,
    }
    ///Container type for the return parameters of the [`getMinimumSlashableStake((address,uint32),address[],address[],uint32)`](getMinimumSlashableStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getMinimumSlashableStakeReturn {
        pub slashableStake: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<getMinimumSlashableStakeCall>
            for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumSlashableStakeCall {
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
            impl ::core::convert::From<getMinimumSlashableStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getMinimumSlashableStakeReturn) -> Self {
                    (value.slashableStake,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getMinimumSlashableStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slashableStake: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getMinimumSlashableStakeReturn;
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
            const SIGNATURE: &'static str = "getMinimumSlashableStake((address,uint32),address[],address[],uint32)";
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub avs: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getOperatorSetCount(address)`](getOperatorSetCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetCountReturn {
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
            impl ::core::convert::From<getOperatorSetCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountCall) -> Self {
                    (value.avs,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetCountCall {
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
            impl ::core::convert::From<getOperatorSetCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getRegisteredSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getRegisteredSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getRegisteredSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getRegisteredSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getRegisteredSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getRegisteredSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStrategiesInOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategiesInOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getStrategiesInOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategiesInOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategiesInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategiesInOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategiesInOperatorSetReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
        pub _0: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStrategyAllocationsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsCall) -> Self {
                    (value.operator, value.strategy)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategyAllocationsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStrategyAllocationsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStrategyAllocationsReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategyAllocationsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStrategyAllocationsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStrategyAllocationsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<OperatorSet>,
                alloy::sol_types::sol_data::Array<IAllocationManagerTypes::Allocation>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `isMemberOfOperatorSet(address,(address,uint32))` and selector `0x670d3ba2`.
```solidity
function isMemberOfOperatorSet(address operator, OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isMemberOfOperatorSet(address,(address,uint32))`](isMemberOfOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberOfOperatorSetReturn {
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
                OperatorSet,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isMemberOfOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isMemberOfOperatorSetCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isMemberOfOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isMemberOfOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isMemberOfOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isMemberOfOperatorSetCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isMemberOfOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                    <OperatorSet as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSet,
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
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<isOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSet: tuple.0 }
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                (<OperatorSet as alloy_sol_types::SolType>::tokenize(&self.operatorSet),)
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
    /**Function with signature `modifyAllocations(address,((address,uint32),address[],uint64[])[])` and selector `0x952899ee`.
```solidity
function modifyAllocations(address operator, IAllocationManagerTypes.AllocateParams[] memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyAllocationsCall {
        pub operator: alloy::sol_types::private::Address,
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
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::AllocateParams,
                >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyAllocationsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<
                    IAllocationManagerTypes::AllocateParams,
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
            const SIGNATURE: &'static str = "modifyAllocations(address,((address,uint32),address[],uint64[])[])";
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
    /**Function with signature `registerForOperatorSets(address,(address,uint32[],bytes))` and selector `0xadc2e3d9`.
```solidity
function registerForOperatorSets(address operator, IAllocationManagerTypes.RegisterParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerForOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerForOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsCall) -> Self {
                    (value.operator, value.params)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerForOperatorSetsCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerForOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerForOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerForOperatorSetsReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerForOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerForOperatorSets(address,(address,uint32[],bytes))";
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.avs, value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesFromOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesFromOperatorSetReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(address,uint32,address[])";
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
    /**Function with signature `setAVSRegistrar(address,address)` and selector `0xd3d96ff4`.
```solidity
function setAVSRegistrar(address avs, address registrar) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setAVSRegistrarCall {
        pub avs: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAVSRegistrarReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAVSRegistrarReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAVSRegistrarReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAVSRegistrarReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    pub struct setAllocationDelayCall {
        pub operator: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setAllocationDelayCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayCall) -> Self {
                    (value.operator, value.delay)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelayCall {
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
            impl ::core::convert::From<setAllocationDelayReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setAllocationDelayReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setAllocationDelayReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setAllocationDelayReturn;
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
    /**Function with signature `slashOperator(address,(address,uint32,address[],uint256[],string))` and selector `0x36352057`.
```solidity
function slashOperator(address avs, IAllocationManagerTypes.SlashingParams memory params) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashOperatorCall {
        pub avs: alloy::sol_types::private::Address,
        pub params: <IAllocationManagerTypes::SlashingParams as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                IAllocationManagerTypes::SlashingParams,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashOperator(address,(address,uint32,address[],uint256[],string))";
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
    /**Function with signature `updateAVSMetadataURI(address,string)` and selector `0xa9821821`.
```solidity
function updateAVSMetadataURI(address avs, string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub avs: alloy::sol_types::private::Address,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value.avs, value.metadataURI)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateAVSMetadataURIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURIReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURIReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
        getOperatorSetCount(getOperatorSetCountCall),
        getRegisteredSets(getRegisteredSetsCall),
        getStrategiesInOperatorSet(getStrategiesInOperatorSetCall),
        getStrategyAllocations(getStrategyAllocationsCall),
        initialize(initializeCall),
        isMemberOfOperatorSet(isMemberOfOperatorSetCall),
        isOperatorSet(isOperatorSetCall),
        modifyAllocations(modifyAllocationsCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        permissionController(permissionControllerCall),
        registerForOperatorSets(registerForOperatorSetsCall),
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        renounceOwnership(renounceOwnershipCall),
        setAVSRegistrar(setAVSRegistrarCall),
        setAllocationDelay(setAllocationDelayCall),
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
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::getAllocation(_) => {
                    <getAllocationCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::getMembers(_) => {
                    <getMembersCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isMemberOfOperatorSet(_) => {
                    <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSet(_) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::slashOperator(_) => {
                    <slashOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<AllocationManagerCalls>] = &[
                {
                    fn getAllocation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <getAllocationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                    fn isMemberOfOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerCalls> {
                        <isMemberOfOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <getMembersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
        AlreadyMemberOfSet(AlreadyMemberOfSet),
        CurrentlyPaused(CurrentlyPaused),
        Empty(Empty),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InsufficientMagnitude(InsufficientMagnitude),
        InvalidCaller(InvalidCaller),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidOperator(InvalidOperator),
        InvalidOperatorSet(InvalidOperatorSet),
        InvalidPermissions(InvalidPermissions),
        InvalidSnapshotOrdering(InvalidSnapshotOrdering),
        InvalidWadToSlash(InvalidWadToSlash),
        ModificationAlreadyPending(ModificationAlreadyPending),
        NotMemberOfSet(NotMemberOfSet),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorNotSlashable(OperatorNotSlashable),
        OutOfBounds(OutOfBounds),
        SameMagnitude(SameMagnitude),
        StrategiesMustBeInAscendingOrder(StrategiesMustBeInAscendingOrder),
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
            [204u8, 234u8, 158u8, 111u8],
            [216u8, 216u8, 220u8, 78u8],
            [216u8, 252u8, 190u8, 48u8],
            [235u8, 191u8, 244u8, 151u8],
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
                Self::InvalidCaller(_) => {
                    <InvalidCaller as alloy_sol_types::SolError>::SELECTOR
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
                Self::NotMemberOfSet(_) => {
                    <NotMemberOfSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotSlashable(_) => {
                    <OperatorNotSlashable as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OutOfBounds(_) => {
                    <OutOfBounds as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SameMagnitude(_) => {
                    <SameMagnitude as alloy_sol_types::SolError>::SELECTOR
                }
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
            ) -> alloy_sol_types::Result<AllocationManagerErrors>] = &[
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
                    fn NotMemberOfSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <NotMemberOfSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
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
                                data,
                                validate,
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
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                    fn InvalidPermissions(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AllocationManagerErrors> {
                        <InvalidPermissions as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
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
                        <OutOfBounds as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                                data,
                                validate,
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
                20u8,
                135u8,
                175u8,
                84u8,
                24u8,
                196u8,
                126u8,
                229u8,
                234u8,
                69u8,
                239u8,
                74u8,
                147u8,
                57u8,
                134u8,
                104u8,
                18u8,
                8u8,
                144u8,
                119u8,
                74u8,
                158u8,
                19u8,
                72u8,
                126u8,
                97u8,
                233u8,
                220u8,
                59u8,
                175u8,
                118u8,
                221u8,
            ],
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
                42u8,
                233u8,
                69u8,
                196u8,
                12u8,
                68u8,
                220u8,
                14u8,
                194u8,
                99u8,
                249u8,
                86u8,
                9u8,
                195u8,
                253u8,
                198u8,
                149u8,
                46u8,
                10u8,
                239u8,
                162u8,
                45u8,
                99u8,
                116u8,
                228u8,
                79u8,
                44u8,
                153u8,
                122u8,
                206u8,
                223u8,
                133u8,
            ],
            [
                49u8,
                98u8,
                146u8,
                133u8,
                234u8,
                210u8,
                51u8,
                90u8,
                224u8,
                147u8,
                63u8,
                134u8,
                237u8,
                42u8,
                230u8,
                51u8,
                33u8,
                247u8,
                175u8,
                119u8,
                180u8,
                230u8,
                234u8,
                171u8,
                196u8,
                44u8,
                5u8,
                120u8,
                128u8,
                151u8,
                126u8,
                108u8,
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
                67u8,
                35u8,
                46u8,
                223u8,
                144u8,
                113u8,
                117u8,
                61u8,
                35u8,
                33u8,
                229u8,
                250u8,
                126u8,
                1u8,
                131u8,
                99u8,
                238u8,
                36u8,
                142u8,
                95u8,
                33u8,
                66u8,
                230u8,
                192u8,
                142u8,
                221u8,
                50u8,
                101u8,
                191u8,
                180u8,
                137u8,
                94u8,
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
                122u8,
                178u8,
                96u8,
                254u8,
                10u8,
                241u8,
                147u8,
                219u8,
                95u8,
                73u8,
                134u8,
                119u8,
                13u8,
                131u8,
                27u8,
                218u8,
                78u8,
                164u8,
                96u8,
                153u8,
                220u8,
                129u8,
                126u8,
                139u8,
                103u8,
                22u8,
                220u8,
                174u8,
                138u8,
                248u8,
                232u8,
                139u8,
            ],
            [
                123u8,
                75u8,
                7u8,
                61u8,
                128u8,
                220u8,
                172u8,
                85u8,
                161u8,
                17u8,
                119u8,
                216u8,
                69u8,
                154u8,
                217u8,
                246u8,
                100u8,
                206u8,
                235u8,
                145u8,
                247u8,
                31u8,
                39u8,
                22u8,
                123u8,
                177u8,
                79u8,
                129u8,
                82u8,
                167u8,
                238u8,
                238u8,
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
                168u8,
                156u8,
                29u8,
                194u8,
                67u8,
                216u8,
                144u8,
                138u8,
                150u8,
                221u8,
                132u8,
                148u8,
                75u8,
                204u8,
                151u8,
                214u8,
                188u8,
                106u8,
                192u8,
                13u8,
                215u8,
                142u8,
                32u8,
                98u8,
                21u8,
                118u8,
                190u8,
                106u8,
                60u8,
                148u8,
                55u8,
                19u8,
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
            [
                173u8,
                52u8,
                195u8,
                7u8,
                11u8,
                225u8,
                223u8,
                251u8,
                202u8,
                164u8,
                153u8,
                208u8,
                0u8,
                186u8,
                43u8,
                141u8,
                152u8,
                72u8,
                174u8,
                252u8,
                172u8,
                48u8,
                89u8,
                223u8,
                36u8,
                93u8,
                217u8,
                92u8,
                78u8,
                206u8,
                20u8,
                254u8,
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
                Some(
                    <AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AVSMetadataURIUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSMetadataURIUpdated)
                }
                Some(<AVSRegistrarSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <AVSRegistrarSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSRegistrarSet)
                }
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
                    <AllocationUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AllocationUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AllocationUpdated)
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
                    <OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorAddedToOperatorSet)
                }
                Some(
                    <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorRemovedFromOperatorSet)
                }
                Some(
                    <OperatorSetCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorSetCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSetCreated)
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
                    <StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyAddedToOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyAddedToOperatorSet)
                }
                Some(
                    <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyRemovedFromOperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyRemovedFromOperatorSet)
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
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
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
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
        AllocationManagerInstance::<
            T,
            P,
            N,
        >::deploy_builder(
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
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegation,
                            _pauserRegistry,
                            _permissionController,
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
        ///Creates a new call builder for the [`addStrategiesToOperatorSet`] function.
        pub fn addStrategiesToOperatorSet(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesToOperatorSetCall, N> {
            self.call_builder(
                &addStrategiesToOperatorSetCall {
                    avs,
                    operatorSetId,
                    strategies,
                },
            )
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
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            avs: alloy::sol_types::private::Address,
            params: alloy::sol_types::private::Vec<
                <IAllocationManagerTypes::CreateSetParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(
                &createOperatorSetsCall {
                    avs,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterFromOperatorSets`] function.
        pub fn deregisterFromOperatorSets(
            &self,
            params: <IAllocationManagerTypes::DeregisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterFromOperatorSetsCall, N> {
            self.call_builder(
                &deregisterFromOperatorSetsCall {
                    params,
                },
            )
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
            self.call_builder(
                &getAllocatableMagnitudeCall {
                    operator,
                    strategy,
                },
            )
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
            self.call_builder(
                &getAllocatedStrategiesCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`getAllocation`] function.
        pub fn getAllocation(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationCall, N> {
            self.call_builder(
                &getAllocationCall {
                    operator,
                    operatorSet,
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
        ///Creates a new call builder for the [`getAllocations`] function.
        pub fn getAllocations(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getAllocationsCall, N> {
            self.call_builder(
                &getAllocationsCall {
                    operators,
                    operatorSet,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitude`] function.
        pub fn getMaxMagnitude(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudeCall, N> {
            self.call_builder(
                &getMaxMagnitudeCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_0`] function.
        pub fn getMaxMagnitudes_0(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_0Call, N> {
            self.call_builder(
                &getMaxMagnitudes_0Call {
                    operators,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudes_1`] function.
        pub fn getMaxMagnitudes_1(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudes_1Call, N> {
            self.call_builder(
                &getMaxMagnitudes_1Call {
                    operator,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`getMaxMagnitudesAtBlock`] function.
        pub fn getMaxMagnitudesAtBlock(
            &self,
            operator: alloy::sol_types::private::Address,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMaxMagnitudesAtBlockCall, N> {
            self.call_builder(
                &getMaxMagnitudesAtBlockCall {
                    operator,
                    strategies,
                    blockNumber,
                },
            )
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
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            futureBlock: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getMinimumSlashableStakeCall, N> {
            self.call_builder(
                &getMinimumSlashableStakeCall {
                    operatorSet,
                    operators,
                    strategies,
                    futureBlock,
                },
            )
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
            self.call_builder(
                &getStrategiesInOperatorSetCall {
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`getStrategyAllocations`] function.
        pub fn getStrategyAllocations(
            &self,
            operator: alloy::sol_types::private::Address,
            strategy: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStrategyAllocationsCall, N> {
            self.call_builder(
                &getStrategyAllocationsCall {
                    operator,
                    strategy,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    initialPausedStatus,
                },
            )
        }
        ///Creates a new call builder for the [`isMemberOfOperatorSet`] function.
        pub fn isMemberOfOperatorSet(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isMemberOfOperatorSetCall, N> {
            self.call_builder(
                &isMemberOfOperatorSetCall {
                    operator,
                    operatorSet,
                },
            )
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
            self.call_builder(
                &modifyAllocationsCall {
                    operator,
                    params,
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
        ///Creates a new call builder for the [`registerForOperatorSets`] function.
        pub fn registerForOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            params: <IAllocationManagerTypes::RegisterParams as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerForOperatorSetsCall, N> {
            self.call_builder(
                &registerForOperatorSetsCall {
                    operator,
                    params,
                },
            )
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            removeStrategiesFromOperatorSetCall,
            N,
        > {
            self.call_builder(
                &removeStrategiesFromOperatorSetCall {
                    avs,
                    operatorSetId,
                    strategies,
                },
            )
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
            self.call_builder(
                &setAVSRegistrarCall {
                    avs,
                    registrar,
                },
            )
        }
        ///Creates a new call builder for the [`setAllocationDelay`] function.
        pub fn setAllocationDelay(
            &self,
            operator: alloy::sol_types::private::Address,
            delay: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setAllocationDelayCall, N> {
            self.call_builder(
                &setAllocationDelayCall {
                    operator,
                    delay,
                },
            )
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
            self.call_builder(
                &updateAVSMetadataURICall {
                    avs,
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
        ///Creates a new event filter for the [`AVSMetadataURIUpdated`] event.
        pub fn AVSMetadataURIUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSMetadataURIUpdated, N> {
            self.event_filter::<AVSMetadataURIUpdated>()
        }
        ///Creates a new event filter for the [`AVSRegistrarSet`] event.
        pub fn AVSRegistrarSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSRegistrarSet, N> {
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
