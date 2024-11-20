///Module containing a contract's types and functions.
/**

```solidity
library BeaconChainProofs {
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
    struct WithdrawalProof { bytes withdrawalProof; bytes slotProof; bytes executionPayloadProof; bytes timestampProof; bytes historicalSummaryBlockRootProof; uint64 blockRootIndex; uint64 historicalSummaryIndex; uint64 withdrawalIndex; bytes32 blockRoot; bytes32 slotRoot; bytes32 timestampRoot; bytes32 executionPayloadRoot; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod BeaconChainProofs {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StateRootProof {
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: alloy::sol_types::private::Bytes,
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
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<StateRootProof> for UnderlyingRustTuple<'_> {
            fn from(value: StateRootProof) -> Self {
                (value.beaconStateRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StateRootProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconStateRoot: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StateRootProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StateRootProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconStateRoot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
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
        impl alloy_sol_types::SolType for StateRootProof {
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
        impl alloy_sol_types::SolStruct for StateRootProof {
            const NAME: &'static str = "StateRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StateRootProof(bytes32 beaconStateRoot,bytes proof)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconStateRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StateRootProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconStateRoot,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proof,
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
                    &rust.beaconStateRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proof,
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
struct WithdrawalProof { bytes withdrawalProof; bytes slotProof; bytes executionPayloadProof; bytes timestampProof; bytes historicalSummaryBlockRootProof; uint64 blockRootIndex; uint64 historicalSummaryIndex; uint64 withdrawalIndex; bytes32 blockRoot; bytes32 slotRoot; bytes32 timestampRoot; bytes32 executionPayloadRoot; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawalProof {
        pub withdrawalProof: alloy::sol_types::private::Bytes,
        pub slotProof: alloy::sol_types::private::Bytes,
        pub executionPayloadProof: alloy::sol_types::private::Bytes,
        pub timestampProof: alloy::sol_types::private::Bytes,
        pub historicalSummaryBlockRootProof: alloy::sol_types::private::Bytes,
        pub blockRootIndex: u64,
        pub historicalSummaryIndex: u64,
        pub withdrawalIndex: u64,
        pub blockRoot: alloy::sol_types::private::FixedBytes<32>,
        pub slotRoot: alloy::sol_types::private::FixedBytes<32>,
        pub timestampRoot: alloy::sol_types::private::FixedBytes<32>,
        pub executionPayloadRoot: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::Bytes,
            u64,
            u64,
            u64,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<WithdrawalProof> for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawalProof) -> Self {
                (
                    value.withdrawalProof,
                    value.slotProof,
                    value.executionPayloadProof,
                    value.timestampProof,
                    value.historicalSummaryBlockRootProof,
                    value.blockRootIndex,
                    value.historicalSummaryIndex,
                    value.withdrawalIndex,
                    value.blockRoot,
                    value.slotRoot,
                    value.timestampRoot,
                    value.executionPayloadRoot,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for WithdrawalProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    withdrawalProof: tuple.0,
                    slotProof: tuple.1,
                    executionPayloadProof: tuple.2,
                    timestampProof: tuple.3,
                    historicalSummaryBlockRootProof: tuple.4,
                    blockRootIndex: tuple.5,
                    historicalSummaryIndex: tuple.6,
                    withdrawalIndex: tuple.7,
                    blockRoot: tuple.8,
                    slotRoot: tuple.9,
                    timestampRoot: tuple.10,
                    executionPayloadRoot: tuple.11,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for WithdrawalProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for WithdrawalProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.slotProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.executionPayloadProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.timestampProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.historicalSummaryBlockRootProof,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockRootIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.historicalSummaryIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockRoot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slotRoot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.timestampRoot),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.executionPayloadRoot),
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
        impl alloy_sol_types::SolType for WithdrawalProof {
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
        impl alloy_sol_types::SolStruct for WithdrawalProof {
            const NAME: &'static str = "WithdrawalProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "WithdrawalProof(bytes withdrawalProof,bytes slotProof,bytes executionPayloadProof,bytes timestampProof,bytes historicalSummaryBlockRootProof,uint64 blockRootIndex,uint64 historicalSummaryIndex,uint64 withdrawalIndex,bytes32 blockRoot,bytes32 slotRoot,bytes32 timestampRoot,bytes32 executionPayloadRoot)",
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawalProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.slotProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.executionPayloadProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.timestampProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.historicalSummaryBlockRootProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.blockRootIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.historicalSummaryIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.withdrawalIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blockRoot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.slotRoot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.timestampRoot)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.executionPayloadRoot,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for WithdrawalProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawalProof,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slotProof,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.executionPayloadProof,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestampProof,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.historicalSummaryBlockRootProof,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockRootIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.historicalSummaryIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.withdrawalIndex,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.slotRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.timestampRoot,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.executionPayloadRoot,
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
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawalProof,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slotProof,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.executionPayloadProof,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestampProof,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.historicalSummaryBlockRootProof,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockRootIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.historicalSummaryIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.withdrawalIndex,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.slotRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.timestampRoot,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.executionPayloadRoot,
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
    /**Creates a new wrapper around an on-chain [`BeaconChainProofs`](self) contract instance.

See the [wrapper's documentation](`BeaconChainProofsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BeaconChainProofsInstance<T, P, N> {
        BeaconChainProofsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`BeaconChainProofs`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BeaconChainProofs`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BeaconChainProofsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BeaconChainProofsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BeaconChainProofsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BeaconChainProofsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BeaconChainProofs`](self) contract instance.

See the [wrapper's documentation](`BeaconChainProofsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BeaconChainProofsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BeaconChainProofsInstance<T, P, N> {
            BeaconChainProofsInstance {
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
    > BeaconChainProofsInstance<T, P, N> {
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
    > BeaconChainProofsInstance<T, P, N> {
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
library IEigenPod {
    type VALIDATOR_STATUS is uint8;
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 mostRecentBalanceUpdateTimestamp; VALIDATOR_STATUS status; }
    struct VerifiedWithdrawal { uint256 amountToSendGwei; int256 sharesDeltaGwei; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IEigenPod {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VALIDATOR_STATUS(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<VALIDATOR_STATUS> for u8 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl VALIDATOR_STATUS {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: u8) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> u8 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for VALIDATOR_STATUS {
            type RustType = u8;
            type Token<'a> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for VALIDATOR_STATUS {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 mostRecentBalanceUpdateTimestamp; VALIDATOR_STATUS status; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInfo {
        pub validatorIndex: u64,
        pub restakedBalanceGwei: u64,
        pub mostRecentBalanceUpdateTimestamp: u64,
        pub status: <VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Uint<64>,
            VALIDATOR_STATUS,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            u64,
            u64,
            <VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<ValidatorInfo> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorInfo) -> Self {
                (
                    value.validatorIndex,
                    value.restakedBalanceGwei,
                    value.mostRecentBalanceUpdateTimestamp,
                    value.status,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorInfo {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    validatorIndex: tuple.0,
                    restakedBalanceGwei: tuple.1,
                    mostRecentBalanceUpdateTimestamp: tuple.2,
                    status: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidatorInfo {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidatorInfo {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.restakedBalanceGwei),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.mostRecentBalanceUpdateTimestamp,
                    ),
                    <VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(
                        &self.status,
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
        impl alloy_sol_types::SolType for ValidatorInfo {
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
        impl alloy_sol_types::SolStruct for ValidatorInfo {
            const NAME: &'static str = "ValidatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorInfo(uint64 validatorIndex,uint64 restakedBalanceGwei,uint64 mostRecentBalanceUpdateTimestamp,uint8 status)",
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
                            &self.validatorIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.restakedBalanceGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.mostRecentBalanceUpdateTimestamp,
                        )
                        .0,
                    <VALIDATOR_STATUS as alloy_sol_types::SolType>::eip712_data_word(
                            &self.status,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ValidatorInfo {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorIndex,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.restakedBalanceGwei,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.mostRecentBalanceUpdateTimestamp,
                    )
                    + <VALIDATOR_STATUS as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.status,
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
                    &rust.validatorIndex,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.restakedBalanceGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.mostRecentBalanceUpdateTimestamp,
                    out,
                );
                <VALIDATOR_STATUS as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.status,
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
struct VerifiedWithdrawal { uint256 amountToSendGwei; int256 sharesDeltaGwei; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct VerifiedWithdrawal {
        pub amountToSendGwei: alloy::sol_types::private::primitives::aliases::U256,
        pub sharesDeltaGwei: alloy::sol_types::private::primitives::aliases::I256,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Int<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::primitives::aliases::I256,
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
        impl ::core::convert::From<VerifiedWithdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: VerifiedWithdrawal) -> Self {
                (value.amountToSendGwei, value.sharesDeltaGwei)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for VerifiedWithdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amountToSendGwei: tuple.0,
                    sharesDeltaGwei: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for VerifiedWithdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for VerifiedWithdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountToSendGwei),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.sharesDeltaGwei),
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
        impl alloy_sol_types::SolType for VerifiedWithdrawal {
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
        impl alloy_sol_types::SolStruct for VerifiedWithdrawal {
            const NAME: &'static str = "VerifiedWithdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "VerifiedWithdrawal(uint256 amountToSendGwei,int256 sharesDeltaGwei)",
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amountToSendGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sharesDeltaGwei,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for VerifiedWithdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountToSendGwei,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sharesDeltaGwei,
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
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountToSendGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sharesDeltaGwei,
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
    /**Creates a new wrapper around an on-chain [`IEigenPod`](self) contract instance.

See the [wrapper's documentation](`IEigenPodInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEigenPodInstance<T, P, N> {
        IEigenPodInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IEigenPod`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IEigenPod`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEigenPodInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IEigenPodInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEigenPodInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEigenPodInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IEigenPod`](self) contract instance.

See the [wrapper's documentation](`IEigenPodInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IEigenPodInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEigenPodInstance<T, P, N> {
            IEigenPodInstance {
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
    > IEigenPodInstance<T, P, N> {
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
    > IEigenPodInstance<T, P, N> {
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
library StdInvariant {
    struct FuzzSelector { address addr; bytes4[] selectors; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod StdInvariant {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct FuzzSelector { address addr; bytes4[] selectors; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        pub addr: alloy::sol_types::private::Address,
        pub selectors: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<4>,
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
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzSelector) -> Self {
                (value.addr, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::tokenize(&self.selectors),
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.selectors)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<4>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.selectors,
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
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<4>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.selectors,
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
    /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StdInvariantInstance<T, P, N> {
        StdInvariantInstance::<T, P, N>::new(address, provider)
    }
    /**A [`StdInvariant`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StdInvariant`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StdInvariantInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StdInvariantInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StdInvariantInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StdInvariantInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> StdInvariantInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StdInvariantInstance<T, P, N> {
            StdInvariantInstance {
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
    > StdInvariantInstance<T, P, N> {
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
    > StdInvariantInstance<T, P, N> {
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
library BeaconChainProofs {
    struct StateRootProof {
        bytes32 beaconStateRoot;
        bytes proof;
    }
    struct WithdrawalProof {
        bytes withdrawalProof;
        bytes slotProof;
        bytes executionPayloadProof;
        bytes timestampProof;
        bytes historicalSummaryBlockRootProof;
        uint64 blockRootIndex;
        uint64 historicalSummaryIndex;
        uint64 withdrawalIndex;
        bytes32 blockRoot;
        bytes32 slotRoot;
        bytes32 timestampRoot;
        bytes32 executionPayloadRoot;
    }
}

library IEigenPod {
    type VALIDATOR_STATUS is uint8;
    struct ValidatorInfo {
        uint64 validatorIndex;
        uint64 restakedBalanceGwei;
        uint64 mostRecentBalanceUpdateTimestamp;
        VALIDATOR_STATUS status;
    }
    struct VerifiedWithdrawal {
        uint256 amountToSendGwei;
        int256 sharesDeltaGwei;
    }
}

library StdInvariant {
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface EPInternalFunctions {
    event EigenPodStaked(bytes pubkey);
    event FullWithdrawalRedeemed(uint40 validatorIndex, uint64 withdrawalTimestamp, address indexed recipient, uint64 withdrawalAmountGwei);
    event Initialized(uint8 version);
    event NonBeaconChainETHReceived(uint256 amountReceived);
    event NonBeaconChainETHWithdrawn(address indexed recipient, uint256 amountWithdrawn);
    event PartialWithdrawalRedeemed(uint40 validatorIndex, uint64 withdrawalTimestamp, address indexed recipient, uint64 partialWithdrawalAmountGwei);
    event RestakedBeaconChainETHWithdrawn(address indexed recipient, uint256 amount);
    event RestakingActivated(address indexed podOwner);
    event ValidatorBalanceUpdated(uint40 validatorIndex, uint64 balanceTimestamp, uint64 newValidatorBalanceGwei);
    event ValidatorRestaked(uint40 validatorIndex);
    event log(string);
    event log_address(address);
    event log_array(uint256[] val);
    event log_array(int256[] val);
    event log_array(address[] val);
    event log_bytes(bytes);
    event log_bytes32(bytes32);
    event log_int(int256);
    event log_named_address(string key, address val);
    event log_named_array(string key, uint256[] val);
    event log_named_array(string key, int256[] val);
    event log_named_array(string key, address[] val);
    event log_named_bytes(string key, bytes val);
    event log_named_bytes32(string key, bytes32 val);
    event log_named_decimal_int(string key, int256 val, uint256 decimals);
    event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
    event log_named_int(string key, int256 val);
    event log_named_string(string key, string val);
    event log_named_uint(string key, uint256 val);
    event log_string(string);
    event log_uint(uint256);
    event logs(bytes);

    constructor(address _ethPOS, address _delayedWithdrawalRouter, address _eigenPodManager, uint64 _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR, uint64 _GENESIS_TIME);

    receive() external payable;

    function GENESIS_TIME() external view returns (uint64);
    function IS_TEST() external view returns (bool);
    function MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR() external view returns (uint64);
    function activateRestaking() external;
    function delayedWithdrawalRouter() external view returns (address);
    function eigenPodManager() external view returns (address);
    function ethPOS() external view returns (address);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external returns (bool);
    function getActiveValidatorCount() external view returns (uint256);
    function hasRestaked() external view returns (bool);
    function initialize(address _podOwner) external;
    function mostRecentWithdrawalTimestamp() external view returns (uint64);
    function nonBeaconChainETHBalanceWei() external view returns (uint256);
    function podOwner() external view returns (address);
    function processFullWithdrawal(uint40 validatorIndex, bytes32 validatorPubkeyHash, uint64 withdrawalHappenedTimestamp, address recipient, uint64 withdrawalAmountGwei, IEigenPod.ValidatorInfo memory validatorInfo) external returns (IEigenPod.VerifiedWithdrawal memory);
    function processPartialWithdrawal(uint40 validatorIndex, uint64 withdrawalHappenedTimestamp, address recipient, uint64 withdrawalAmountGwei) external returns (IEigenPod.VerifiedWithdrawal memory);
    function provenWithdrawal(bytes32, uint64) external view returns (bool);
    function recoverTokens(address[] memory tokenList, uint256[] memory amountsToWithdraw, address recipient) external;
    function setActiveValidatorCount(uint256 _count) external;
    function setMostRecentWithdrawalTimestamp(uint64 _mostRecentWithdrawalTimestamp) external;
    function setValidatorRestakedBalance(bytes32 pkhash, uint64 restakedBalanceGwei) external;
    function setValidatorStatus(bytes32 pkhash, IEigenPod.VALIDATOR_STATUS status) external;
    function stake(bytes memory pubkey, bytes memory signature, bytes32 depositDataRoot) external payable;
    function sumOfPartialWithdrawalsClaimedGwei() external view returns (uint64);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPod.ValidatorInfo memory);
    function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPod.ValidatorInfo memory);
    function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPod.VALIDATOR_STATUS);
    function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPod.VALIDATOR_STATUS);
    function verifyAndProcessWithdrawal(bytes32 beaconStateRoot, BeaconChainProofs.WithdrawalProof memory withdrawalProof, bytes memory validatorFieldsProof, bytes32[] memory validatorFields, bytes32[] memory withdrawalFields) external returns (IEigenPod.VerifiedWithdrawal memory);
    function verifyAndProcessWithdrawals(uint64 oracleTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.WithdrawalProof[] memory withdrawalProofs, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields, bytes32[][] memory withdrawalFields) external;
    function verifyBalanceUpdate(uint64 oracleTimestamp, uint40 validatorIndex, bytes32 beaconStateRoot, bytes memory validatorFieldsProofs, bytes32[] memory validatorFields, uint64 mostRecentBalanceUpdateTimestamp) external returns (int256);
    function verifyBalanceUpdates(uint64 oracleTimestamp, uint40[] memory validatorIndices, BeaconChainProofs.StateRootProof memory stateRootProof, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
    function verifyWithdrawalCredentials(uint64 oracleTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
    function verifyWithdrawalCredentials(uint64 oracleTimestamp, bytes32 beaconStateRoot, uint40 validatorIndex, bytes memory validatorFieldsProof, bytes32[] memory validatorFields) external returns (uint256);
    function withdrawBeforeRestaking() external;
    function withdrawNonBeaconChainETHBalanceWei(address recipient, uint256 amountToWithdraw) external;
    function withdrawRestakedBeaconChainETH(address recipient, uint256 amountWei) external;
    function withdrawableRestakedExecutionLayerGwei() external view returns (uint64);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_ethPOS",
        "type": "address",
        "internalType": "contract IETHPOSDeposit"
      },
      {
        "name": "_delayedWithdrawalRouter",
        "type": "address",
        "internalType": "contract IDelayedWithdrawalRouter"
      },
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      },
      {
        "name": "_MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "_GENESIS_TIME",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "receive",
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "GENESIS_TIME",
    "inputs": [],
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
    "name": "IS_TEST",
    "inputs": [],
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
    "name": "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR",
    "inputs": [],
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
    "name": "activateRestaking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "delayedWithdrawalRouter",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IDelayedWithdrawalRouter"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "eigenPodManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "ethPOS",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IETHPOSDeposit"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "excludeSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "failed",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getActiveValidatorCount",
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
    "name": "hasRestaked",
    "inputs": [],
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
    "name": "initialize",
    "inputs": [
      {
        "name": "_podOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "mostRecentWithdrawalTimestamp",
    "inputs": [],
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
    "name": "nonBeaconChainETHBalanceWei",
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
    "name": "podOwner",
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
    "name": "processFullWithdrawal",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      },
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "withdrawalHappenedTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "withdrawalAmountGwei",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "validatorInfo",
        "type": "tuple",
        "internalType": "struct IEigenPod.ValidatorInfo",
        "components": [
          {
            "name": "validatorIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "restakedBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "mostRecentBalanceUpdateTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPod.VALIDATOR_STATUS"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPod.VerifiedWithdrawal",
        "components": [
          {
            "name": "amountToSendGwei",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sharesDeltaGwei",
            "type": "int256",
            "internalType": "int256"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "processPartialWithdrawal",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      },
      {
        "name": "withdrawalHappenedTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "withdrawalAmountGwei",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPod.VerifiedWithdrawal",
        "components": [
          {
            "name": "amountToSendGwei",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sharesDeltaGwei",
            "type": "int256",
            "internalType": "int256"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "provenWithdrawal",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "recoverTokens",
    "inputs": [
      {
        "name": "tokenList",
        "type": "address[]",
        "internalType": "contract IERC20[]"
      },
      {
        "name": "amountsToWithdraw",
        "type": "uint256[]",
        "internalType": "uint256[]"
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
    "name": "setActiveValidatorCount",
    "inputs": [
      {
        "name": "_count",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMostRecentWithdrawalTimestamp",
    "inputs": [
      {
        "name": "_mostRecentWithdrawalTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setValidatorRestakedBalance",
    "inputs": [
      {
        "name": "pkhash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "restakedBalanceGwei",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setValidatorStatus",
    "inputs": [
      {
        "name": "pkhash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "status",
        "type": "uint8",
        "internalType": "enum IEigenPod.VALIDATOR_STATUS"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stake",
    "inputs": [
      {
        "name": "pubkey",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "signature",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "depositDataRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "sumOfPartialWithdrawalsClaimedGwei",
    "inputs": [],
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
    "name": "targetArtifactSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifactSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetArtifacts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedArtifacts_",
        "type": "string[]",
        "internalType": "string[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetContracts",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedContracts_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSelectors_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzSelector[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "selectors",
            "type": "bytes4[]",
            "internalType": "bytes4[]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "targetSenders",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedSenders_",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorPubkeyHashToInfo",
    "inputs": [
      {
        "name": "validatorPubkeyHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPod.ValidatorInfo",
        "components": [
          {
            "name": "validatorIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "restakedBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "mostRecentBalanceUpdateTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPod.VALIDATOR_STATUS"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorPubkeyToInfo",
    "inputs": [
      {
        "name": "validatorPubkey",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPod.ValidatorInfo",
        "components": [
          {
            "name": "validatorIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "restakedBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "mostRecentBalanceUpdateTimestamp",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPod.VALIDATOR_STATUS"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorStatus",
    "inputs": [
      {
        "name": "validatorPubkey",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IEigenPod.VALIDATOR_STATUS"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "validatorStatus",
    "inputs": [
      {
        "name": "pubkeyHash",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IEigenPod.VALIDATOR_STATUS"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyAndProcessWithdrawal",
    "inputs": [
      {
        "name": "beaconStateRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "withdrawalProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.WithdrawalProof",
        "components": [
          {
            "name": "withdrawalProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "slotProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "executionPayloadProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "timestampProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "historicalSummaryBlockRootProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "blockRootIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "historicalSummaryIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "withdrawalIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "slotRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "timestampRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "executionPayloadRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "validatorFieldsProof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "withdrawalFields",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPod.VerifiedWithdrawal",
        "components": [
          {
            "name": "amountToSendGwei",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "sharesDeltaGwei",
            "type": "int256",
            "internalType": "int256"
          }
        ]
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyAndProcessWithdrawals",
    "inputs": [
      {
        "name": "oracleTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "stateRootProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.StateRootProof",
        "components": [
          {
            "name": "beaconStateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "withdrawalProofs",
        "type": "tuple[]",
        "internalType": "struct BeaconChainProofs.WithdrawalProof[]",
        "components": [
          {
            "name": "withdrawalProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "slotProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "executionPayloadProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "timestampProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "historicalSummaryBlockRootProof",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "blockRootIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "historicalSummaryIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "withdrawalIndex",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "blockRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "slotRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "timestampRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "executionPayloadRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "validatorFieldsProofs",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      },
      {
        "name": "withdrawalFields",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyBalanceUpdate",
    "inputs": [
      {
        "name": "oracleTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      },
      {
        "name": "beaconStateRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "validatorFieldsProofs",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "mostRecentBalanceUpdateTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyBalanceUpdates",
    "inputs": [
      {
        "name": "oracleTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "validatorIndices",
        "type": "uint40[]",
        "internalType": "uint40[]"
      },
      {
        "name": "stateRootProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.StateRootProof",
        "components": [
          {
            "name": "beaconStateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "validatorFieldsProofs",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyWithdrawalCredentials",
    "inputs": [
      {
        "name": "oracleTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "stateRootProof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.StateRootProof",
        "components": [
          {
            "name": "beaconStateRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      },
      {
        "name": "validatorIndices",
        "type": "uint40[]",
        "internalType": "uint40[]"
      },
      {
        "name": "validatorFieldsProofs",
        "type": "bytes[]",
        "internalType": "bytes[]"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[][]",
        "internalType": "bytes32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyWithdrawalCredentials",
    "inputs": [
      {
        "name": "oracleTimestamp",
        "type": "uint64",
        "internalType": "uint64"
      },
      {
        "name": "beaconStateRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      },
      {
        "name": "validatorFieldsProof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawBeforeRestaking",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawNonBeaconChainETHBalanceWei",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountToWithdraw",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawRestakedBeaconChainETH",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amountWei",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "withdrawableRestakedExecutionLayerGwei",
    "inputs": [],
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
    "type": "event",
    "name": "EigenPodStaked",
    "inputs": [
      {
        "name": "pubkey",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "FullWithdrawalRedeemed",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": false,
        "internalType": "uint40"
      },
      {
        "name": "withdrawalTimestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "withdrawalAmountGwei",
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
    "name": "NonBeaconChainETHReceived",
    "inputs": [
      {
        "name": "amountReceived",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "NonBeaconChainETHWithdrawn",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amountWithdrawn",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "PartialWithdrawalRedeemed",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": false,
        "internalType": "uint40"
      },
      {
        "name": "withdrawalTimestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "partialWithdrawalAmountGwei",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RestakedBeaconChainETHWithdrawn",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "RestakingActivated",
    "inputs": [
      {
        "name": "podOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorBalanceUpdated",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": false,
        "internalType": "uint40"
      },
      {
        "name": "balanceTimestamp",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      },
      {
        "name": "newValidatorBalanceGwei",
        "type": "uint64",
        "indexed": false,
        "internalType": "uint64"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ValidatorRestaked",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": false,
        "internalType": "uint40"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_address",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_array",
    "inputs": [
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_bytes32",
    "inputs": [
      {
        "name": "",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_int",
    "inputs": [
      {
        "name": "",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_address",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256[]",
        "indexed": false,
        "internalType": "uint256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256[]",
        "indexed": false,
        "internalType": "int256[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_array",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "address[]",
        "indexed": false,
        "internalType": "address[]"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_bytes32",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_decimal_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "decimals",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_int",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_string",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_named_uint",
    "inputs": [
      {
        "name": "key",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      },
      {
        "name": "val",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_string",
    "inputs": [
      {
        "name": "",
        "type": "string",
        "indexed": false,
        "internalType": "string"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "log_uint",
    "inputs": [
      {
        "name": "",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "logs",
    "inputs": [
      {
        "name": "",
        "type": "bytes",
        "indexed": false,
        "internalType": "bytes"
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
    clippy::style
)]
pub mod EPInternalFunctions {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61012060405260668054600160ff199182168117909255606a805490911690911790553480156200002f57600080fd5b506040516200700038038062007000833981016040819052620000529162000196565b6001600160a01b0380861660805280851660a052831660c0526001600160401b0380831660e05281166101005284848484846200008e6200009e565b505050505050505050506200020e565b600054610100900460ff16156200010b5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff90811610156200015e576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200017657600080fd5b50565b80516001600160401b03811681146200019157600080fd5b919050565b600080600080600060a08688031215620001af57600080fd5b8551620001bc8162000160565b6020870151909550620001cf8162000160565b6040870151909450620001e28162000160565b9250620001f26060870162000179565b9150620002026080870162000179565b90509295509295909350565b60805160a05160c05160e05161010051616cf76200030960003960008181610951015281816135a20152818161365901526136b10152600081816103ac015281816134060152818161343301528181613b4201528181613b7601528181613f5c0152613f9701526000818161052f015281816109bc01528181610cb401528181610ffc0152818161115101528181611624015281816117df015281816119c001528181611af401528181611ec2015281816123760152818161257e015281816126bd0152818161288a015281816129740152612e7801526000818161037801526141fe0152600081816106b001526116ef0152616cf76000f3fe6080604052600436106102815760003560e01c80637bb3dbf31161014f578063c4907442116100c1578063e20c9f711161007a578063e20c9f71146108ea578063e251ef52146108ff578063e2c834451461091f578063f28824611461093f578063fa7626d414610973578063fe80b0871461098d57600080fd5b8063c490744214610802578063c4d66de814610822578063ca044e8c14610842578063d168eb5114610862578063d79ed726146108aa578063dda3346c146108ca57600080fd5b80639b4e4634116101135780639b4e463414610770578063a50600f414610783578063b522538a146107a3578063b5508aa9146107c3578063ba414fa6146107d8578063baa7145a146107ed57600080fd5b80637bb3dbf3146106d2578063816d53f9146106f257806385226c811461071257806387e0d28914610734578063916a17c61461075b57600080fd5b80633e5e3c23116101f35780635d3f65b6116101ac5780635d3f65b6146105d857806366d9a9a0146105f85780636fcd0e531461061a57806373a97ee8146106475780637439841f1461066757806374cdd7981461069e57600080fd5b80633e5e3c23146104d35780633f65cf19146104e85780633f7286f4146105085780634665bcda1461051d5780635229564a1461055157806358eaee79146105ab57600080fd5b80631ed7831c116102455780631ed7831c146103e65780633106ab53146104085780633474aa161461043957806334bea20a1461045957806337deea70146104945780633b4c57c6146104b357600080fd5b80630b18ff66146102d75780630cd4649e14610314578063115cd5e41461032b5780631a5057be146103665780631d905d5c1461039a57600080fd5b366102d257346037600082825461029891906157ab565b90915550506040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156102e357600080fd5b506033546102f7906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561032057600080fd5b506103296109a3565b005b34801561033757600080fd5b5061034b61034636600461584f565b610b0c565b6040805182518152602092830151928101929092520161030b565b34801561037257600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b3480156103a657600080fd5b506103ce7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160401b03909116815260200161030b565b3480156103f257600080fd5b506103fb610b3d565b60405161030b919061591f565b34801561041457600080fd5b5060345461042990600160401b900460ff1681565b604051901515815260200161030b565b34801561044557600080fd5b506034546103ce906001600160401b031681565b34801561046557600080fd5b5061042961047436600461598c565b603560209081526000928352604080842090915290825290205460ff1681565b3480156104a057600080fd5b506039545b60405190815260200161030b565b3480156104bf57600080fd5b506104a56104ce3660046159d1565b610b9f565b3480156104df57600080fd5b506103fb610c11565b3480156104f457600080fd5b50610329610503366004615a98565b610c71565b34801561051457600080fd5b506103fb6111bc565b34801561052957600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b34801561055d57600080fd5b5061032961056c36600461598c565b60009182526036602052604090912080546001600160401b03909216600160401b026fffffffffffffffff000000000000000019909216919091179055565b3480156105b757600080fd5b506105cb6105c6366004615b09565b61121c565b60405161030b9190615b82565b3480156105e457600080fd5b506038546103ce906001600160401b031681565b34801561060457600080fd5b5061060d611281565b60405161030b9190615b90565b34801561062657600080fd5b5061063a610635366004615c43565b611370565b60405161030b9190615c5c565b34801561065357600080fd5b50610329610662366004615c43565b603955565b34801561067357600080fd5b506105cb610682366004615c43565b600090815260366020526040902054600160c01b900460ff1690565b3480156106aa57600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b3480156106de57600080fd5b5061034b6106ed366004615cc4565b61141d565b3480156106fe57600080fd5b506104a561070d366004615d1e565b611446565b34801561071e57600080fd5b50610727611463565b60405161030b9190615e12565b34801561074057600080fd5b506033546103ce90600160a01b90046001600160401b031681565b34801561076757600080fd5b5061060d611533565b61032961077e366004615e74565b611619565b34801561078f57600080fd5b5061032961079e366004615ee7565b6117c6565b3480156107af57600080fd5b5061063a6107be366004615b09565b611b5c565b3480156107cf57600080fd5b50610727611c4f565b3480156107e457600080fd5b50610429611d1f565b3480156107f957600080fd5b50610329611e4c565b34801561080e57600080fd5b5061032961081d366004615f71565b611eb7565b34801561082e57600080fd5b5061032961083d366004615f9d565b6120f4565b34801561084e57600080fd5b5061034b61085d36600461605a565b6122cc565b34801561086e57600080fd5b5061032961087d366004616128565b603380546001600160401b03909216600160a01b0267ffffffffffffffff60a01b19909216919091179055565b3480156108b657600080fd5b506103296108c5366004616145565b6122f9565b3480156108d657600080fd5b506103296108e53660046161ff565b612333565b3480156108f657600080fd5b506103fb612506565b34801561090b57600080fd5b5061032961091a3660046162d0565b612566565b34801561092b57600080fd5b5061032961093a366004615f71565b612931565b34801561094b57600080fd5b506103ce7f000000000000000000000000000000000000000000000000000000000000000081565b34801561097f57600080fd5b506066546104299060ff1681565b34801561099957600080fd5b506104a560375481565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a0b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a2f91906163cb565b15610a555760405162461bcd60e51b8152600401610a4c906163ed565b60405180910390fd5b6033546001600160a01b03163314610a7f5760405162461bcd60e51b8152600401610a4c9061644a565b603454600160401b900460ff1615610aa95760405162461bcd60e51b8152600401610a4c90616492565b6034805460ff60401b1916600160401b179055603354610ad1906001600160a01b0316612b14565b6033546040516001600160a01b03909116907fca8dfc8c5e0a67a74501c072a3325f685259bebbae7cfd230ab85198a78b70cd90600090a250565b6040805180820190915260008082526020820152610b308989898989898989612b48565b9998505050505050505050565b60606073805480602002602001604051908101604052809291908181526020018280548015610b9557602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610b77575b5050505050905090565b60008084846000818110610bb557610bb56164e1565b60209081029290920135600081815260369093526040909220805467ffffffffffffffff60801b1916600160801b6001600160401b03881602179055509050610c038a8a8a8a8a8a8a6130c0565b9a9950505050505050505050565b60606075805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b6033546001600160a01b03163314610c9b5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610d03573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d2791906163cb565b15610d445760405162461bcd60e51b8152600401610a4c906163ed565b603454600160401b900460ff16610dbc5760405162461bcd60e51b815260206004820152603660248201527f456967656e506f642e686173456e61626c656452657374616b696e673a2072656044820152751cdd185ada5b99c81a5cc81b9bdd08195b98589b195960521b6064820152608401610a4c565b8584148015610dca57508382145b610e5a5760405162461bcd60e51b815260206004820152605560248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2076616c696461746f72496e646963657320616e642070726f6f666064820152740e640daeae6e840c4ca40e6c2daca40d8cadccee8d605b1b608482015260a401610a4c565b603354600160a01b90046001600160401b03161580610eaf5750603354610e9990610e9490600160a01b90046001600160401b031661359e565b613688565b6001600160401b0316896001600160401b031610155b610f3b5760405162461bcd60e51b815260206004820152605160248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2070726f6f66206d75737420626520696e207468652065706f63686064820152701030b33a32b91030b1ba34bb30ba34b7b760791b608482015260a401610a4c565b42610f51613f486001600160401b038c166157ab565b1015610fda5760405162461bcd60e51b815260206004820152604c60248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a207370656369666965642074696d657374616d7020697320746f6f60648201526b0819985c881a5b881c185cdd60a21b608482015260a401610a4c565b60405163d1c64cc960e01b81526001600160401b038a166004820152611083907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa15801561104b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061106f91906164f7565b893561107e60208c018c616510565b6136d5565b6000805b87811015611127576111098b8b358b8b858181106110a7576110a76164e1565b90506020020160208101906110bc9190616556565b8a8a868181106110ce576110ce6164e1565b90506020028101906110e09190616510565b8a8a888181106110f2576110f26164e1565b90506020028101906111049190616571565b613863565b61111390836157ab565b91508061111f816165ba565b915050611087565b5060335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c40906044015b600060405180830381600087803b15801561119857600080fd5b505af11580156111ac573d6000803e3d6000fd5b5050505050505050505050505050565b60606074805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b60008061125e84848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613d1d92505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b60606078805480602002602001604051908101604052809291908181526020016000905b828210156113675760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561134f57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116113115790505b505050505081525050815260200190600101906112a5565b50505050905090565b6113986040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561140357611403615b4a565b600281111561141457611414615b4a565b90525092915050565b604080518082019091526000808252602082015261143d85858585613e17565b95945050505050565b600061145788888888888888613863565b98975050505050505050565b60606077805480602002602001604051908101604052809291908181526020016000905b828210156113675783829060005260206000200180546114a6906165d5565b80601f01602080910402602001604051908101604052809291908181526020018280546114d2906165d5565b801561151f5780601f106114f45761010080835404028352916020019161151f565b820191906000526020600020905b81548152906001019060200180831161150257829003601f168201915b505050505081526020019060010190611487565b60606079805480602002602001604051908101604052809291908181526020016000905b828210156113675760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561160157602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116115c35790505b50505050508152505081526020019060010190611557565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146116615760405162461bcd60e51b8152600401610a4c9061660a565b346801bc16d674ec800000146116ed5760405162461bcd60e51b8152602060048201526044602482018190527f456967656e506f642e7374616b653a206d75737420696e697469616c6c792073908201527f74616b6520666f7220616e792076616c696461746f72207769746820333220656064820152633a3432b960e11b608482015260a401610a4c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611730613ef5565b8888886040518863ffffffff1660e01b815260040161175496959493929190616684565b6000604051808303818588803b15801561176d57600080fd5b505af1158015611781573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e2385856040516117b79291906166d3565b60405180910390a15050505050565b604051635ac86ab760e01b8152600360048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561182e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061185291906163cb565b1561186f5760405162461bcd60e51b8152600401610a4c906163ed565b868414801561187d57508382145b6119065760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e76657269667942616c616e6365557064617465733a207660448201527f616c696461746f72496e646963657320616e642070726f6f6673206d7573742060648201526d0c4ca40e6c2daca40d8cadccee8d60931b608482015260a401610a4c565b4261191c613f486001600160401b038c166157ab565b101561199e5760405162461bcd60e51b815260206004820152604560248201527f456967656e506f642e76657269667942616c616e6365557064617465733a207360448201527f70656369666965642074696d657374616d7020697320746f6f2066617220696e606482015264081c185cdd60da1b608482015260a401610a4c565b60405163d1c64cc960e01b81526001600160401b038a166004820152611a42907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa158015611a0f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a3391906164f7565b873561107e60208a018a616510565b6000805b88811015611ae657611ac88b8b8b84818110611a6457611a646164e1565b9050602002016020810190611a799190616556565b8a358a8a86818110611a8d57611a8d6164e1565b9050602002810190611a9f9190616510565b8a8a88818110611ab157611ab16164e1565b9050602002810190611ac39190616571565b6130c0565b611ad290836166e7565b915080611ade816165ba565b915050611a46565b506033546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169163c2c51c409116611b2b633b9aca0085616728565b6040516001600160e01b031960e085901b1681526001600160a01b039092166004830152602482015260440161117e565b611b846040805160808101825260008082526020820181905291810182905290606082015290565b60366000611bc785858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613d1d92505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff166002811115611c3457611c34615b4a565b6002811115611c4557611c45615b4a565b9052509392505050565b60606076805480602002602001604051908101604052809291908181526020016000905b82821015611367578382906000526020600020018054611c92906165d5565b80601f0160208091040260200160405190810160405280929190818152602001828054611cbe906165d5565b8015611d0b5780601f10611ce057610100808354040283529160200191611d0b565b820191906000526020600020905b815481529060010190602001808311611cee57829003601f168201915b505050505081526020019060010190611c73565b606654600090610100900460ff1615611d415750606654610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611e475760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611dcf917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016167ad565b60408051601f1981840301815290829052611de9916167de565b6000604051808303816000865af19150503d8060008114611e26576040519150601f19603f3d011682016040523d82523d6000602084013e611e2b565b606091505b5091505080806020019051810190611e4391906163cb565b9150505b919050565b6033546001600160a01b03163314611e765760405162461bcd60e51b8152600401610a4c9061644a565b603454600160401b900460ff1615611ea05760405162461bcd60e51b8152600401610a4c90616492565b603354611eb5906001600160a01b0316612b14565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611eff5760405162461bcd60e51b8152600401610a4c9061660a565b611f0d633b9aca0082616810565b15611f975760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74576569206d75737420626520612077686f60648201526d1b194811ddd95a48185b5bdd5b9d60921b608482015260a401610a4c565b6000611fa7633b9aca0083616824565b6034549091506001600160401b0390811690821611156120605760405162461bcd60e51b815260206004820152606260248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74477765692065786365656473207769746860648201527f6472617761626c6552657374616b6564457865637574696f6e4c617965724777608482015261656960f01b60a482015260c401610a4c565b6034805482919060009061207e9084906001600160401b0316616838565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e836040516120dd91815260200190565b60405180910390a26120ef8383613f3a565b505050565b600054610100900460ff16158080156121145750600054600160ff909116105b8061212e5750303b15801561212e575060005460ff166001145b6121915760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610a4c565b6000805460ff1916600117905580156121b4576000805461ff0019166101001790555b6001600160a01b0382166122275760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e696e697469616c697a653a20706f644f776e65722063616044820152736e6e6f74206265207a65726f206164647265737360601b6064820152608401610a4c565b603380546001600160a01b0384166001600160a01b031990911681179091556034805460ff60401b1916600160401b1790556040517fca8dfc8c5e0a67a74501c072a3325f685259bebbae7cfd230ab85198a78b70cd90600090a280156122c8576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050565b60408051808201909152600080825260208201526122ee878787878787613f44565b979650505050505050565b6000828152603660205260409020805482919060ff60c01b1916600160c01b83600281111561232a5761232a615b4a565b02179055505050565b6033546001600160a01b0316331461235d5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156123c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123e991906163cb565b156124065760405162461bcd60e51b8152600401610a4c906163ed565b82518451146124915760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e7265636f766572546f6b656e733a20746f6b656e4c697360448201527f7420616e6420616d6f756e7473546f5769746864726177206d7573742062652060648201526a0e6c2daca40d8cadccee8d60ab1b608482015260a401610a4c565b60005b84518110156124ff576124ed838583815181106124b3576124b36164e1565b60200260200101518784815181106124cd576124cd6164e1565b60200260200101516001600160a01b03166141829092919063ffffffff16565b806124f7816165ba565b915050612494565b5050505050565b60606072805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b604051635ac86ab760e01b81526004808201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156125cd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f191906163cb565b1561260e5760405162461bcd60e51b8152600401610a4c906163ed565b838614801561261c57508588145b801561262757508782145b61269b576040805162461bcd60e51b81526020600482015260248101919091527f456967656e506f642e766572696679416e6450726f636573735769746864726160448201527f77616c733a20696e70757473206d7573742062652073616d65206c656e6774686064820152608401610a4c565b60405163d1c64cc960e01b81526001600160401b038c16600482015261273f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa15801561270c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061273091906164f7565b8b3561107e60208e018e616510565b604080518082019091526000808252602082015260005b8381101561283f5760006127fa8d358d8d85818110612777576127776164e1565b90506020028101906127899190616860565b8c8c8681811061279b5761279b6164e1565b90506020028101906127ad9190616510565b8c8c888181106127bf576127bf6164e1565b90506020028101906127d19190616571565b8c8c8a8181106127e3576127e36164e1565b90506020028101906127f59190616571565b612b48565b8051845191925090849061280f9083906157ab565b90525060208082015190840180516128289083906166e7565b905250819050612837816165ba565b915050612756565b5080511561286e57603354815161286e916001600160a01b03169061286990633b9aca0090616877565b6141d4565b6020810151156129235760335460208201516001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263c2c51c40929116906128c490633b9aca0090616728565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561290a57600080fd5b505af115801561291e573d6000803e3d6000fd5b505050505b505050505050505050505050565b6033546001600160a01b0316331461295b5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156129c3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906129e791906163cb565b15612a045760405162461bcd60e51b8152600401610a4c906163ed565b603754821115612ab55760405162461bcd60e51b815260206004820152606a60248201527f456967656e506f642e77697468647261776e6f6e426561636f6e436861696e4560448201527f544842616c616e63655765693a20616d6f756e74546f5769746864726177206960648201527f732067726561746572207468616e206e6f6e426561636f6e436861696e45544860848201526942616c616e636557656960b01b60a482015260c401610a4c565b8160376000828254612ac79190616896565b90915550506040518281526001600160a01b038416907f30420aacd028abb3c1fd03aba253ae725d6ddd52d16c9ac4cb5742cd43f530969060200160405180910390a26120ef83836141d4565b6033805467ffffffffffffffff60a01b19164263ffffffff16600160a01b021790556000603755612b4581476141d4565b50565b6040805180820190915260008082526020820152612b6d612b688961691c565b614262565b6033546001600160401b03600160a01b90910481169082161015612c2f5760405162461bcd60e51b815260206004820152606760248201527f456967656e506f642e70726f6f664973466f7256616c696454696d657374616d60448201527f703a20626561636f6e20636861696e2070726f6f66206d75737420626520617460648201527f206f72206166746572206d6f7374526563656e745769746864726177616c546960848201526606d657374616d760cc1b60a482015260c401610a4c565b6000612c3d612b688b61691c565b90506000612c7d88888080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b905060008082815260366020526040902054600160c01b900460ff166002811115612caa57612caa615b4a565b1415612d615760405162461bcd60e51b815260206004820152607460248201527f456967656e506f642e5f766572696679416e6450726f6365737357697468647260448201527f6177616c3a2056616c696461746f72206e657665722070726f76656e20746f2060648201527f68617665207769746864726177616c2063726564656e7469616c7320706f696e6084820152731d1959081d1bc81d1a1a5cc818dbdb9d1c9858dd60621b60a482015260c401610a4c565b60008181526035602090815260408083206001600160401b038616845290915290205460ff1615612e205760405162461bcd60e51b815260206004820152605b60248201527f456967656e506f642e5f766572696679416e6450726f6365737357697468647260448201527f6177616c3a207769746864726177616c2068617320616c72656164792062656560648201527f6e2070726f76656e20666f7220746869732074696d657374616d700000000000608482015260a401610a4c565b6001603560008381526020019081526020016000206000846001600160401b03166001600160401b0316815260200190815260200160002060006101000a81548160ff021916908315150217905550612efd8c87878e7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166344e71c806040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ed4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ef89190616a58565b614296565b6000612f3b878780806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614cb792505050565b9050612f4b8d8a8a8e8e86614cdc565b6000612f89888880806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f3392505050565b9050612fc78a8a80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f4b92505050565b6001600160401b0316612fe1612fdc8f61691c565b614f63565b6001600160401b03161061309957603354600084815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b81049093169381019390935261308e93869388938a936001600160a01b03909316928892916060830190600160c01b900460ff16600281111561307557613075615b4a565b600281111561308657613086615b4a565b905250613f44565b9550505050506130b3565b60335461308e90839086906001600160a01b031684613e17565b5098975050505050505050565b6000806130ff848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f7592505050565b9050600061313f85858080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156131ae576131ae615b4a565b60028111156131bf576131bf615b4a565b8152505090508a6001600160401b031681604001516001600160401b0316106132765760405162461bcd60e51b815260206004820152605c60248201527f456967656e506f642e76657269667942616c616e63655570646174653a20566160448201527f6c696461746f72732062616c616e63652068617320616c72656164792062656560648201527f6e207570646174656420666f7220746869732074696d657374616d7000000000608482015260a401610a4c565b60018160600151600281111561328e5761328e615b4a565b146132f65760405162461bcd60e51b815260206004820152603260248201527f456967656e506f642e76657269667942616c616e63655570646174653a2056616044820152716c696461746f72206e6f742061637469766560701b6064820152608401610a4c565b6132ff8b61359e565b6001600160401b0316613344878780806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f4b92505050565b6001600160401b0316116133e7576000836001600160401b0316116133e75760405162461bcd60e51b815260206004820152604d60248201527f456967656e506f642e76657269667942616c616e63655570646174653a20766160448201527f6c696461746f7220697320776974686472617761626c6520627574206861732060648201526c3737ba103bb4ba34323930bbb760991b608482015260a401610a4c565b6133f58987878b8b8f614cdc565b602081015160006001600160401b037f00000000000000000000000000000000000000000000000000000000000000008116908616111561345757507f000000000000000000000000000000000000000000000000000000000000000061345a565b50835b6001600160401b0380821660208086019182528f831660408088019182526000898152603690935290912086518154935192518516600160801b0267ffffffffffffffff60801b19938616600160401b026001600160801b031990951691909516179290921790811683178255606086015186939091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b83600281111561350257613502615b4a565b0217905550905050816001600160401b0316816001600160401b03161461358e577f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df8c8e836040516135799392919064ffffffffff9390931683526001600160401b03918216602084015216604082015260600190565b60405180910390a161358b8183614f8d565b95505b5050505050979650505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0316826001600160401b031610156136485760405162461bcd60e51b815260206004820152603760248201527f456967656e506f642e5f74696d657374616d70546f45706f63683a2074696d6560448201527f7374616d70206973206265666f72652067656e657369730000000000000000006064820152608401610a4c565b613654600c6020616a75565b61367e7f000000000000000000000000000000000000000000000000000000000000000084616838565b61127b9190616aa4565b6000613696600c6020616a75565b6136a1836001616aca565b6136ab9190616a75565b61127b907f0000000000000000000000000000000000000000000000000000000000000000616aca565b6136e160036020616877565b81146137715760405162461bcd60e51b815260206004820152605360248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f74416761696e73744c6174657374426c6f636b526f6f743a2050726f6f6620686064820152720c2e640d2dcc6dee4e4cac6e840d8cadccee8d606b1b608482015260a401610a4c565b6137b682828080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525088925087915060039050614fac565b61385d5760405162461bcd60e51b815260206004820152606660248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f74416761696e73744c6174657374426c6f636b526f6f743a20496e76616c696460648201527f206c617465737420626c6f636b2068656164657220726f6f74206d65726b6c6560848201526510383937b7b360d11b60a482015260c401610a4c565b50505050565b6000806138a284848080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561391157613911615b4a565b600281111561392257613922615b4a565b905250905060008160600151600281111561393f5761393f615b4a565b146139e85760405162461bcd60e51b815260206004820152606760248201527f456967656e506f642e766572696679436f72726563745769746864726177616c60448201527f43726564656e7469616c733a2056616c696461746f72206d757374206265206960648201527f6e61637469766520746f2070726f7665207769746864726177616c2063726564608482015266656e7469616c7360c81b60a482015260c401610a4c565b6139f0613ef5565b6139f990616af5565b613a35868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614fc492505050565b14613abc5760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e766572696679436f72726563745769746864726177616c60448201527f43726564656e7469616c733a2050726f6f66206973206e6f7420666f7220746860648201526a1a5cc8115a59d95b941bd960aa1b608482015260a401610a4c565b6000613afa868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f7592505050565b9050613b0a8a87878b8b8e614cdc565b60398054906000613b1a836165ba565b90915550506001606083015264ffffffffff891682526001600160401b038b811660408401527f000000000000000000000000000000000000000000000000000000000000000081169082161115613ba0576001600160401b037f0000000000000000000000000000000000000000000000000000000000000000166020830152613bb0565b6001600160401b03811660208301525b6000838152603660209081526040918290208451815492860151938601516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060850151859391929091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b836002811115613c4e57613c4e615b4a565b02179055505060405164ffffffffff8b1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a17f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df898c8460200151604051613ce99392919064ffffffffff9390931683526001600160401b03918216602084015216604082015260600190565b60405180910390a1633b9aca0082602001516001600160401b0316613d0e9190616877565b9b9a5050505050505050505050565b60008151603014613da65760405162461bcd60e51b815260206004820152604760248201527f456967656e506f642e5f63616c63756c61746556616c696461746f725075626b60448201527f657948617368206d75737420626520612034382d6279746520424c53207075626064820152666c6963206b657960c81b608482015260a401610a4c565b604051600290613dbd908490600090602001616b19565b60408051601f1981840301815290829052613dd7916167de565b602060405180830381855afa158015613df4573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061127b91906164f7565b60408051808201909152600080825260208201526040805164ffffffffff871681526001600160401b0380871660208301528416918101919091526001600160a01b038416907f8a7335714231dbd551aaba6314f4a97a14c201e53a3e25e1140325cdf67d7a4e9060600160405180910390a260388054839190600090613ea89084906001600160401b0316616aca565b92506101000a8154816001600160401b0302191690836001600160401b031602179055506040518060400160405280836001600160401b0316815260200160008152509050949350505050565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b6122c88282614fd9565b604080518082019091526000808252602082015260007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0316846001600160401b03161115613fbb57507f0000000000000000000000000000000000000000000000000000000000000000613fbe565b50825b6040805180820190915260008082526020820152613fdc8286616838565b6001600160401b039081168252603480548492600091613ffe91859116616aca565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550614030828560200151614f8d565b602082015260028460600151600281111561404d5761404d615b4a565b1461406f576039805490600061406283616b48565b9091555050600260608501525b600060208086018281528a83526036909152604091829020865181549251938801516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516929091169190911792909217928316821781556060870151879391929091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b83600281111561411457614114615b4a565b0217905550506040805164ffffffffff8c1681526001600160401b038a8116602083015288168183015290516001600160a01b03891692507fb76a93bb649ece524688f1a01d184e0bbebcda58eae80c28a898bec3fb5a09639181900360600190a298975050505050505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b1790526120ef9084906150f2565b603354604051633036cd5360e21b81526001600160a01b03918216600482015283821660248201527f00000000000000000000000000000000000000000000000000000000000000009091169063c0db354c9083906044016000604051808303818588803b15801561424557600080fd5b505af1158015614259573d6000803e3d6000fd5b50505050505050565b600061127b8261014001516151c4565b600081600081518110614287576142876164e1565b60200260200101519050919050565b6142a1600280616c43565b83146143155760405162461bcd60e51b81526020600482015260496024820152600080516020616ca283398151915260448201527f616c3a207769746864726177616c4669656c64732068617320696e636f7272656064820152680c6e840d8cadccee8d60bb1b608482015260a401610a4c565b614321600d6002616c43565b61433160c0840160a08501616128565b6001600160401b03161061439b5760405162461bcd60e51b815260206004820152603f6024820152600080516020616ca283398151915260448201527f616c3a20626c6f636b526f6f74496e64657820697320746f6f206c61726765006064820152608401610a4c565b6143a760046002616c43565b6143b8610100840160e08501616128565b6001600160401b031610614424576040805162461bcd60e51b8152602060048201526024810191909152600080516020616ca283398151915260448201527f616c3a207769746864726177616c496e64657820697320746f6f206c617267656064820152608401610a4c565b61443060186002616c43565b61444060e0840160c08501616128565b6001600160401b0316106144ba5760405162461bcd60e51b81526020600482015260476024820152600080516020616ca283398151915260448201527f616c3a20686973746f726963616c53756d6d617279496e64657820697320746f6064820152666f206c6172676560c81b608482015260a401610a4c565b60006001600160401b0382166144d2612b688561691c565b6001600160401b0316106144e75760056144ea565b60045b90506144f76004826157ab565b6145029060016157ab565b61450d906020616877565b6145178480616510565b90501461458b5760405162461bcd60e51b81526020600482015260486024820152600080516020616ca283398151915260448201527f616c3a207769746864726177616c50726f6f662068617320696e636f727265636064820152670e840d8cadccee8d60c31b608482015260a401610a4c565b614597600460036157ab565b6145a2906020616877565b6145af6040850185616510565b9050146146295760405162461bcd60e51b815260206004820152604e6024820152600080516020616ca283398151915260448201527f616c3a20657865637574696f6e5061796c6f616450726f6f662068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a401610a4c565b61463560036020616877565b6146426020850185616510565b9050146146b05760405162461bcd60e51b81526020600482015260426024820152600080516020616ca283398151915260448201527f616c3a20736c6f7450726f6f662068617320696e636f7272656374206c656e676064820152610e8d60f31b608482015260a401610a4c565b6146bb816020616877565b6146c86060850185616510565b90501461473b5760405162461bcd60e51b81526020600482015260476024820152600080516020616ca283398151915260448201527f616c3a2074696d657374616d7050726f6f662068617320696e636f7272656374606482015266040d8cadccee8d60cb1b608482015260a401610a4c565b600d614749601860016157ab565b6147549060056157ab565b61475f9060016157ab565b61476991906157ab565b614774906020616877565b6147816080850185616510565b90501461480a5760405162461bcd60e51b81526020600482015260586024820152600080516020616ca283398151915260448201527f616c3a20686973746f726963616c53756d6d617279426c6f636b526f6f74507260648201527f6f6f662068617320696e636f7272656374206c656e6774680000000000000000608482015260a401610a4c565b600061481c60c0850160a08601616128565b6001600160401b03166000614833600d60016157ab565b61484360e0880160c08901616128565b6001600160401b0316901b600d61485c601860016157ab565b6148679060016157ab565b61487191906157ab565b601b901b17171790506148cc61488a6080860186616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508b9250505061010087013584614fac565b61493f5760405162461bcd60e51b815260206004820152604a6024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420686973746f726963616c73756d6d617279206d656064820152693935b63290383937b7b360b11b608482015260a401610a4c565b61499661494f6020860186616510565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201829052506101008a013593506101208a013592509050614fac565b6149f65760405162461bcd60e51b815260206004820152603d6024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420736c6f74206d65726b6c652070726f6f660000006064820152608401610a4c565b6049614a4e614a086040870187616510565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050505061010087013561016088013584614fac565b614ac05760405162461bcd60e51b81526020600482015260496024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420657865637574696f6e5061796c6f6164206d657260648201526835b63290383937b7b360b91b608482015260a401610a4c565b50614b18614ad16060860186616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050506101608601356101408701356009614fac565b614b835760405162461bcd60e51b81526020600482015260426024820152600080516020616ca283398151915260448201527f616c3a20496e76616c69642074696d657374616d70206d65726b6c652070726f60648201526137b360f11b608482015260a401610a4c565b6000614b96610100860160e08701616128565b6001600160401b0316614bab600460016157ab565b600e901b1790506000614bf088888080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061522b92505050565b9050614c40614bff8780616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050506101608801358385614fac565b614cac5760405162461bcd60e51b81526020600482015260436024820152600080516020616ca283398151915260448201527f616c3a20496e76616c6964207769746864726177616c206d65726b6c6520707260648201526237b7b360e91b608482015260a401610a4c565b505050505050505050565b600061127b82600181518110614ccf57614ccf6164e1565b60200260200101516151c4565b614ce860036002616c43565b8414614d735760405162461bcd60e51b815260206004820152604e60248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a2056616c696461746f72206669656c64732068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a401610a4c565b6005614d81602860016157ab565b614d8b91906157ab565b614d96906020616877565b8214614e165760405162461bcd60e51b815260206004820152604360248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a2050726f6f662068617320696e636f7272656374206c656e6064820152620cee8d60eb1b608482015260a401610a4c565b600064ffffffffff8216614e2c602860016157ab565b600b901b1790506000614e7187878080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061522b92505050565b9050614eb785858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250859150869050614fac565b614f295760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a20496e76616c6964206d65726b6c652070726f6f660000006064820152608401610a4c565b5050505050505050565b600061127b82600381518110614ccf57614ccf6164e1565b600061127b82600781518110614ccf57614ccf6164e1565b6000602061367e8361012001516151c4565b600061127b82600281518110614ccf57614ccf6164e1565b6000614fa56001600160401b03808416908516616c4f565b9392505050565b600083614fba8685856154d8565b1495945050505050565b600081600181518110614287576142876164e1565b804710156150295760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610a4c565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114615076576040519150601f19603f3d011682016040523d82523d6000602084013e61507b565b606091505b50509050806120ef5760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610a4c565b6000615147826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166156249092919063ffffffff16565b8051909150156120ef578080602001905181019061516591906163cb565b6120ef5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610a4c565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6000806002835161523c9190616824565b90506000816001600160401b0381111561525857615258615fba565b604051908082528060200260200182016040528015615281578160200160208202803683370190505b50905060005b828110156153885760028561529c8383616877565b815181106152ac576152ac6164e1565b6020026020010151868360026152c29190616877565b6152cd9060016157ab565b815181106152dd576152dd6164e1565b60200260200101516040516020016152ff929190918252602082015260400190565b60408051601f1981840301815290829052615319916167de565b602060405180830381855afa158015615336573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061535991906164f7565b82828151811061536b5761536b6164e1565b602090810291909101015280615380816165ba565b915050615287565b50615394600283616824565b91505b81156154b45760005b828110156154a1576002826153b58383616877565b815181106153c5576153c56164e1565b6020026020010151838360026153db9190616877565b6153e69060016157ab565b815181106153f6576153f66164e1565b6020026020010151604051602001615418929190918252602082015260400190565b60408051601f1981840301815290829052615432916167de565b602060405180830381855afa15801561544f573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061547291906164f7565b828281518110615484576154846164e1565b602090810291909101015280615499816165ba565b9150506153a0565b506154ad600283616824565b9150615397565b806000815181106154c7576154c76164e1565b602002602001015192505050919050565b600083516000141580156154f75750602084516154f59190616810565b155b6155865760405162461bcd60e51b815260206004820152605460248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f665368613260448201527f35363a2070726f6f66206c656e6774682073686f756c642062652061206e6f6e60648201527316bd32b9379036bab63a34b836329037b310199960611b608482015260a401610a4c565b604080516020808201909252848152905b8551811161561a576155aa600285616810565b6155dd578151600052808601516020526020826040600060026107d05a03fa6155d257600080fd5b600284049350615608565b8086015160005281516020526020826040600060026107d05a03fa61560157600080fd5b6002840493505b6156136020826157ab565b9050615597565b5051949350505050565b6060615633848460008561563b565b949350505050565b60608247101561569c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610a4c565b6001600160a01b0385163b6156f35760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610a4c565b600080866001600160a01b0316858760405161570f91906167de565b60006040518083038185875af1925050503d806000811461574c576040519150601f19603f3d011682016040523d82523d6000602084013e615751565b606091505b50915091506122ee8282866060831561576b575081614fa5565b82511561577b5782518084602001fd5b8160405162461bcd60e51b8152600401610a4c9190616c8e565b634e487b7160e01b600052601160045260246000fd5b600082198211156157be576157be615795565b500190565b60008083601f8401126157d557600080fd5b5081356001600160401b038111156157ec57600080fd5b60208301915083602082850101111561580457600080fd5b9250929050565b60008083601f84011261581d57600080fd5b5081356001600160401b0381111561583457600080fd5b6020830191508360208260051b850101111561580457600080fd5b60008060008060008060008060a0898b03121561586b57600080fd5b8835975060208901356001600160401b038082111561588957600080fd5b908a0190610180828d03121561589e57600080fd5b90975060408a013590808211156158b457600080fd5b6158c08c838d016157c3565b909850965060608b01359150808211156158d957600080fd5b6158e58c838d0161580b565b909650945060808b01359150808211156158fe57600080fd5b5061590b8b828c0161580b565b999c989b5096995094979396929594505050565b6020808252825182820181905260009190848201906040850190845b818110156159605783516001600160a01b03168352928401929184019160010161593b565b50909695505050505050565b6001600160401b0381168114612b4557600080fd5b8035611e478161596c565b6000806040838503121561599f57600080fd5b8235915060208301356159b18161596c565b809150509250929050565b803564ffffffffff81168114611e4757600080fd5b60008060008060008060008060c0898b0312156159ed57600080fd5b88356159f88161596c565b9750615a0660208a016159bc565b96506040890135955060608901356001600160401b0380821115615a2957600080fd5b615a358c838d016157c3565b909750955060808b0135915080821115615a4e57600080fd5b50615a5b8b828c0161580b565b90945092505060a0890135615a6f8161596c565b809150509295985092959890939650565b600060408284031215615a9257600080fd5b50919050565b60008060008060008060008060a0898b031215615ab457600080fd5b8835615abf8161596c565b975060208901356001600160401b0380821115615adb57600080fd5b615ae78c838d01615a80565b985060408b0135915080821115615afd57600080fd5b6158c08c838d0161580b565b60008060208385031215615b1c57600080fd5b82356001600160401b03811115615b3257600080fd5b615b3e858286016157c3565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b60038110615b7e57634e487b7160e01b600052602160045260246000fd5b9052565b6020810161127b8284615b60565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015615c3457898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015615c1f5783516001600160e01b0319168252928b019260019290920191908b0190615bf5565b50978a01979550505091870191600101615bb8565b50919998505050505050505050565b600060208284031215615c5557600080fd5b5035919050565b60006080820190506001600160401b03808451168352806020850151166020840152806040850151166040840152506060830151615c9d6060840182615b60565b5092915050565b6001600160a01b0381168114612b4557600080fd5b8035611e4781615ca4565b60008060008060808587031215615cda57600080fd5b615ce3856159bc565b93506020850135615cf38161596c565b92506040850135615d0381615ca4565b91506060850135615d138161596c565b939692955090935050565b600080600080600080600060a0888a031215615d3957600080fd5b8735615d448161596c565b965060208801359550615d59604089016159bc565b945060608801356001600160401b0380821115615d7557600080fd5b615d818b838c016157c3565b909650945060808a0135915080821115615d9a57600080fd5b50615da78a828b0161580b565b989b979a50959850939692959293505050565b60005b83811015615dd5578181015183820152602001615dbd565b8381111561385d5750506000910152565b60008151808452615dfe816020860160208601615dba565b601f01601f19169290920160200192915050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015615e6757603f19888603018452615e55858351615de6565b94509285019290850190600101615e39565b5092979650505050505050565b600080600080600060608688031215615e8c57600080fd5b85356001600160401b0380821115615ea357600080fd5b615eaf89838a016157c3565b90975095506020880135915080821115615ec857600080fd5b50615ed5888289016157c3565b96999598509660400135949350505050565b60008060008060008060008060a0898b031215615f0357600080fd5b8835615f0e8161596c565b975060208901356001600160401b0380821115615f2a57600080fd5b615f368c838d0161580b565b909950975060408b0135915080821115615f4f57600080fd5b615f5b8c838d01615a80565b965060608b01359150808211156158d957600080fd5b60008060408385031215615f8457600080fd5b8235615f8f81615ca4565b946020939093013593505050565b600060208284031215615faf57600080fd5b8135614fa581615ca4565b634e487b7160e01b600052604160045260246000fd5b604051608081016001600160401b0381118282101715615ff257615ff2615fba565b60405290565b60405161018081016001600160401b0381118282101715615ff257615ff2615fba565b604051601f8201601f191681016001600160401b038111828210171561604357616043615fba565b604052919050565b803560038110611e4757600080fd5b60008060008060008086880361012081121561607557600080fd5b61607e886159bc565b96506020880135955060408801356160958161596c565b945060608801356160a581615ca4565b935060808801356160b58161596c565b92506080609f19820112156160c957600080fd5b506160d2615fd0565b60a08801356160e08161596c565b815260c08801356160f08161596c565b602082015260e08801356161038161596c565b6040820152616115610100890161604b565b6060820152809150509295509295509295565b60006020828403121561613a57600080fd5b8135614fa58161596c565b6000806040838503121561615857600080fd5b823591506161686020840161604b565b90509250929050565b60006001600160401b0382111561618a5761618a615fba565b5060051b60200190565b600082601f8301126161a557600080fd5b813560206161ba6161b583616171565b61601b565b82815260059290921b840181019181810190868411156161d957600080fd5b8286015b848110156161f457803583529183019183016161dd565b509695505050505050565b60008060006060848603121561621457600080fd5b83356001600160401b038082111561622b57600080fd5b818601915086601f83011261623f57600080fd5b8135602061624f6161b583616171565b82815260059290921b8401810191818101908a84111561626e57600080fd5b948201945b8386101561629557853561628681615ca4565b82529482019490820190616273565b975050870135925050808211156162ab57600080fd5b506162b886828701616194565b9250506162c760408501615cb9565b90509250925092565b60008060008060008060008060008060c08b8d0312156162ef57600080fd5b6162f88b615981565b995060208b01356001600160401b038082111561631457600080fd5b6163208e838f01615a80565b9a5060408d013591508082111561633657600080fd5b6163428e838f0161580b565b909a50985060608d013591508082111561635b57600080fd5b6163678e838f0161580b565b909850965060808d013591508082111561638057600080fd5b61638c8e838f0161580b565b909650945060a08d01359150808211156163a557600080fd5b506163b28d828e0161580b565b915080935050809150509295989b9194979a5092959850565b6000602082840312156163dd57600080fd5b81518015158114614fa557600080fd5b6020808252603e908201527f456967656e506f642e6f6e6c795768656e4e6f745061757365643a20696e646560408201527f782069732070617573656420696e20456967656e506f644d616e616765720000606082015260800190565b60208082526028908201527f456967656e506f642e6f6e6c79456967656e506f644f776e65723a206e6f74206040820152673837b227bbb732b960c11b606082015260800190565b6020808252602f908201527f456967656e506f642e6861734e6576657252657374616b65643a20726573746160408201526e1ada5b99c81a5cc8195b98589b1959608a1b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561650957600080fd5b5051919050565b6000808335601e1984360301811261652757600080fd5b8301803591506001600160401b0382111561654157600080fd5b60200191503681900382131561580457600080fd5b60006020828403121561656857600080fd5b614fa5826159bc565b6000808335601e1984360301811261658857600080fd5b8301803591506001600160401b038211156165a257600080fd5b6020019150600581901b360382131561580457600080fd5b60006000198214156165ce576165ce615795565b5060010190565b600181811c908216806165e957607f821691505b60208210811415615a9257634e487b7160e01b600052602260045260246000fd5b60208082526031908201527f456967656e506f642e6f6e6c79456967656e506f644d616e616765723a206e6f6040820152703a1032b4b3b2b72837b226b0b730b3b2b960791b606082015260800190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60808152600061669860808301888a61665b565b82810360208401526166aa8188615de6565b905082810360408401526166bf81868861665b565b915050826060830152979650505050505050565b60208152600061563360208301848661665b565b600080821280156001600160ff1b038490038513161561670957616709615795565b600160ff1b839003841281161561672257616722615795565b50500190565b60006001600160ff1b038184138284138082168684048611161561674e5761674e615795565b600160ff1b600087128281168783058912161561676d5761676d615795565b6000871292508782058712848416161561678957616789615795565b8785058712818416161561679f5761679f615795565b505050929093029392505050565b6001600160e01b03198316815281516000906167d0816004850160208701615dba565b919091016004019392505050565b600082516167f0818460208701615dba565b9190910192915050565b634e487b7160e01b600052601260045260246000fd5b60008261681f5761681f6167fa565b500690565b600082616833576168336167fa565b500490565b60006001600160401b038381169083168181101561685857616858615795565b039392505050565b6000823561017e198336030181126167f057600080fd5b600081600019048311821515161561689157616891615795565b500290565b6000828210156168a8576168a8615795565b500390565b600082601f8301126168be57600080fd5b81356001600160401b038111156168d7576168d7615fba565b6168ea601f8201601f191660200161601b565b8181528460208386010111156168ff57600080fd5b816020850160208301376000918101602001919091529392505050565b6000610180823603121561692f57600080fd5b616937615ff8565b82356001600160401b038082111561694e57600080fd5b61695a368387016168ad565b8352602085013591508082111561697057600080fd5b61697c368387016168ad565b6020840152604085013591508082111561699557600080fd5b6169a1368387016168ad565b604084015260608501359150808211156169ba57600080fd5b6169c6368387016168ad565b606084015260808501359150808211156169df57600080fd5b506169ec368286016168ad565b6080830152506169fe60a08401615981565b60a0820152616a0f60c08401615981565b60c0820152616a2060e08401615981565b60e082015261010083810135908201526101208084013590820152610140808401359082015261016092830135928101929092525090565b600060208284031215616a6a57600080fd5b8151614fa58161596c565b60006001600160401b0380831681851681830481118215151615616a9b57616a9b615795565b02949350505050565b60006001600160401b0380841680616abe57616abe6167fa565b92169190910492915050565b60006001600160401b03808316818516808303821115616aec57616aec615795565b01949350505050565b80516020808301519190811015615a925760001960209190910360031b1b16919050565b60008351616b2b818460208801615dba565b6001600160801b0319939093169190920190815260100192915050565b600081616b5757616b57615795565b506000190190565b600181815b80851115616b9a578160001904821115616b8057616b80615795565b80851615616b8d57918102915b93841c9390800290616b64565b509250929050565b600082616bb15750600161127b565b81616bbe5750600061127b565b8160018114616bd45760028114616bde57616bfa565b600191505061127b565b60ff841115616bef57616bef615795565b50506001821b61127b565b5060208310610133831016604e8410600b8410161715616c1d575081810a61127b565b616c278383616b5f565b8060001904821115616c3b57616c3b615795565b029392505050565b6000614fa58383616ba2565b60008083128015600160ff1b850184121615616c6d57616c6d615795565b6001600160ff1b0384018313811615616c8857616c88615795565b50500390565b602081526000614fa56020830184615de656fe426561636f6e436861696e50726f6f66732e7665726966795769746864726177a2646970667358221220a9c8016b8b8b8d76601725febabd237067aa3aa96c952e1e3ac20cc882f16ea864736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01 `@R`f\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`j\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15b\0\0/W`\0\x80\xFD[P`@Qb\0p\08\x03\x80b\0p\0\x839\x81\x01`@\x81\x90Rb\0\0R\x91b\0\x01\x96V[`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x80R\x80\x85\x16`\xA0R\x83\x16`\xC0R`\x01`\x01`@\x1B\x03\x80\x83\x16`\xE0R\x81\x16a\x01\0R\x84\x84\x84\x84\x84b\0\0\x8Eb\0\0\x9EV[PPPPPPPPPPb\0\x02\x0EV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01^W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01vW`\0\x80\xFD[PV[\x80Q`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\x91W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\x01\xAFW`\0\x80\xFD[\x85Qb\0\x01\xBC\x81b\0\x01`V[` \x87\x01Q\x90\x95Pb\0\x01\xCF\x81b\0\x01`V[`@\x87\x01Q\x90\x94Pb\0\x01\xE2\x81b\0\x01`V[\x92Pb\0\x01\xF2``\x87\x01b\0\x01yV[\x91Pb\0\x02\x02`\x80\x87\x01b\0\x01yV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa\x01\0Qal\xF7b\0\x03\t`\09`\0\x81\x81a\tQ\x01R\x81\x81a5\xA2\x01R\x81\x81a6Y\x01Ra6\xB1\x01R`\0\x81\x81a\x03\xAC\x01R\x81\x81a4\x06\x01R\x81\x81a43\x01R\x81\x81a;B\x01R\x81\x81a;v\x01R\x81\x81a?\\\x01Ra?\x97\x01R`\0\x81\x81a\x05/\x01R\x81\x81a\t\xBC\x01R\x81\x81a\x0C\xB4\x01R\x81\x81a\x0F\xFC\x01R\x81\x81a\x11Q\x01R\x81\x81a\x16$\x01R\x81\x81a\x17\xDF\x01R\x81\x81a\x19\xC0\x01R\x81\x81a\x1A\xF4\x01R\x81\x81a\x1E\xC2\x01R\x81\x81a#v\x01R\x81\x81a%~\x01R\x81\x81a&\xBD\x01R\x81\x81a(\x8A\x01R\x81\x81a)t\x01Ra.x\x01R`\0\x81\x81a\x03x\x01RaA\xFE\x01R`\0\x81\x81a\x06\xB0\x01Ra\x16\xEF\x01Ral\xF7`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x81W`\x005`\xE0\x1C\x80c{\xB3\xDB\xF3\x11a\x01OW\x80c\xC4\x90tB\x11a\0\xC1W\x80c\xE2\x0C\x9Fq\x11a\0zW\x80c\xE2\x0C\x9Fq\x14a\x08\xEAW\x80c\xE2Q\xEFR\x14a\x08\xFFW\x80c\xE2\xC84E\x14a\t\x1FW\x80c\xF2\x88$a\x14a\t?W\x80c\xFAv&\xD4\x14a\tsW\x80c\xFE\x80\xB0\x87\x14a\t\x8DW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x08\x02W\x80c\xC4\xD6m\xE8\x14a\x08\"W\x80c\xCA\x04N\x8C\x14a\x08BW\x80c\xD1h\xEBQ\x14a\x08bW\x80c\xD7\x9E\xD7&\x14a\x08\xAAW\x80c\xDD\xA34l\x14a\x08\xCAW`\0\x80\xFD[\x80c\x9BNF4\x11a\x01\x13W\x80c\x9BNF4\x14a\x07pW\x80c\xA5\x06\0\xF4\x14a\x07\x83W\x80c\xB5\"S\x8A\x14a\x07\xA3W\x80c\xB5P\x8A\xA9\x14a\x07\xC3W\x80c\xBAAO\xA6\x14a\x07\xD8W\x80c\xBA\xA7\x14Z\x14a\x07\xEDW`\0\x80\xFD[\x80c{\xB3\xDB\xF3\x14a\x06\xD2W\x80c\x81mS\xF9\x14a\x06\xF2W\x80c\x85\"l\x81\x14a\x07\x12W\x80c\x87\xE0\xD2\x89\x14a\x074W\x80c\x91j\x17\xC6\x14a\x07[W`\0\x80\xFD[\x80c>^<#\x11a\x01\xF3W\x80c]?e\xB6\x11a\x01\xACW\x80c]?e\xB6\x14a\x05\xD8W\x80cf\xD9\xA9\xA0\x14a\x05\xF8W\x80co\xCD\x0ES\x14a\x06\x1AW\x80cs\xA9~\xE8\x14a\x06GW\x80ct9\x84\x1F\x14a\x06gW\x80ct\xCD\xD7\x98\x14a\x06\x9EW`\0\x80\xFD[\x80c>^<#\x14a\x04\xD3W\x80c?e\xCF\x19\x14a\x04\xE8W\x80c?r\x86\xF4\x14a\x05\x08W\x80cFe\xBC\xDA\x14a\x05\x1DW\x80cR)VJ\x14a\x05QW\x80cX\xEA\xEEy\x14a\x05\xABW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02EW\x80c\x1E\xD7\x83\x1C\x14a\x03\xE6W\x80c1\x06\xABS\x14a\x04\x08W\x80c4t\xAA\x16\x14a\x049W\x80c4\xBE\xA2\n\x14a\x04YW\x80c7\xDE\xEAp\x14a\x04\x94W\x80c;LW\xC6\x14a\x04\xB3W`\0\x80\xFD[\x80c\x0B\x18\xFFf\x14a\x02\xD7W\x80c\x0C\xD4d\x9E\x14a\x03\x14W\x80c\x11\\\xD5\xE4\x14a\x03+W\x80c\x1APW\xBE\x14a\x03fW\x80c\x1D\x90]\\\x14a\x03\x9AW`\0\x80\xFD[6a\x02\xD2W4`7`\0\x82\x82Ta\x02\x98\x91\x90aW\xABV[\x90\x91UPP`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x02\xE3W`\0\x80\xFD[P`3Ta\x02\xF7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x03)a\t\xA3V[\0[4\x80\x15a\x037W`\0\x80\xFD[Pa\x03Ka\x03F6`\x04aXOV[a\x0B\x0CV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x03\x0BV[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xA6W`\0\x80\xFD[Pa\x03\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x0BV[4\x80\x15a\x03\xF2W`\0\x80\xFD[Pa\x03\xFBa\x0B=V[`@Qa\x03\x0B\x91\x90aY\x1FV[4\x80\x15a\x04\x14W`\0\x80\xFD[P`4Ta\x04)\x90`\x01`@\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x0BV[4\x80\x15a\x04EW`\0\x80\xFD[P`4Ta\x03\xCE\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x04)a\x04t6`\x04aY\x8CV[`5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x04\xA0W`\0\x80\xFD[P`9T[`@Q\x90\x81R` \x01a\x03\x0BV[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x04\xA5a\x04\xCE6`\x04aY\xD1V[a\x0B\x9FV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x03\xFBa\x0C\x11V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x03)a\x05\x036`\x04aZ\x98V[a\x0CqV[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x03\xFBa\x11\xBCV[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05]W`\0\x80\xFD[Pa\x03)a\x05l6`\x04aY\x8CV[`\0\x91\x82R`6` R`@\x90\x91 \x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x05\xCBa\x05\xC66`\x04a[\tV[a\x12\x1CV[`@Qa\x03\x0B\x91\x90a[\x82V[4\x80\x15a\x05\xE4W`\0\x80\xFD[P`8Ta\x03\xCE\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x04W`\0\x80\xFD[Pa\x06\ra\x12\x81V[`@Qa\x03\x0B\x91\x90a[\x90V[4\x80\x15a\x06&W`\0\x80\xFD[Pa\x06:a\x0656`\x04a\\CV[a\x13pV[`@Qa\x03\x0B\x91\x90a\\\\V[4\x80\x15a\x06SW`\0\x80\xFD[Pa\x03)a\x06b6`\x04a\\CV[`9UV[4\x80\x15a\x06sW`\0\x80\xFD[Pa\x05\xCBa\x06\x826`\x04a\\CV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x06\xAAW`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xDEW`\0\x80\xFD[Pa\x03Ka\x06\xED6`\x04a\\\xC4V[a\x14\x1DV[4\x80\x15a\x06\xFEW`\0\x80\xFD[Pa\x04\xA5a\x07\r6`\x04a]\x1EV[a\x14FV[4\x80\x15a\x07\x1EW`\0\x80\xFD[Pa\x07'a\x14cV[`@Qa\x03\x0B\x91\x90a^\x12V[4\x80\x15a\x07@W`\0\x80\xFD[P`3Ta\x03\xCE\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x07gW`\0\x80\xFD[Pa\x06\ra\x153V[a\x03)a\x07~6`\x04a^tV[a\x16\x19V[4\x80\x15a\x07\x8FW`\0\x80\xFD[Pa\x03)a\x07\x9E6`\x04a^\xE7V[a\x17\xC6V[4\x80\x15a\x07\xAFW`\0\x80\xFD[Pa\x06:a\x07\xBE6`\x04a[\tV[a\x1B\\V[4\x80\x15a\x07\xCFW`\0\x80\xFD[Pa\x07'a\x1COV[4\x80\x15a\x07\xE4W`\0\x80\xFD[Pa\x04)a\x1D\x1FV[4\x80\x15a\x07\xF9W`\0\x80\xFD[Pa\x03)a\x1ELV[4\x80\x15a\x08\x0EW`\0\x80\xFD[Pa\x03)a\x08\x1D6`\x04a_qV[a\x1E\xB7V[4\x80\x15a\x08.W`\0\x80\xFD[Pa\x03)a\x08=6`\x04a_\x9DV[a \xF4V[4\x80\x15a\x08NW`\0\x80\xFD[Pa\x03Ka\x08]6`\x04a`ZV[a\"\xCCV[4\x80\x15a\x08nW`\0\x80\xFD[Pa\x03)a\x08}6`\x04aa(V[`3\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA0\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x08\xB6W`\0\x80\xFD[Pa\x03)a\x08\xC56`\x04aaEV[a\"\xF9V[4\x80\x15a\x08\xD6W`\0\x80\xFD[Pa\x03)a\x08\xE56`\x04aa\xFFV[a#3V[4\x80\x15a\x08\xF6W`\0\x80\xFD[Pa\x03\xFBa%\x06V[4\x80\x15a\t\x0BW`\0\x80\xFD[Pa\x03)a\t\x1A6`\x04ab\xD0V[a%fV[4\x80\x15a\t+W`\0\x80\xFD[Pa\x03)a\t:6`\x04a_qV[a)1V[4\x80\x15a\tKW`\0\x80\xFD[Pa\x03\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x7FW`\0\x80\xFD[P`fTa\x04)\x90`\xFF\x16\x81V[4\x80\x15a\t\x99W`\0\x80\xFD[Pa\x04\xA5`7T\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n/\x91\x90ac\xCBV[\x15a\nUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`4T`\x01`@\x1B\x90\x04`\xFF\x16\x15a\n\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ad\x92V[`4\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`3Ta\n\xD1\x90`\x01`\x01`\xA0\x1B\x03\x16a+\x14V[`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\xCA\x8D\xFC\x8C^\ng\xA7E\x01\xC0r\xA32_hRY\xBE\xBB\xAE|\xFD#\n\xB8Q\x98\xA7\x8Bp\xCD\x90`\0\x90\xA2PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0B0\x89\x89\x89\x89\x89\x89\x89\x89a+HV[\x99\x98PPPPPPPPPV[```s\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwW[PPPPP\x90P\x90V[`\0\x80\x84\x84`\0\x81\x81\x10a\x0B\xB5Wa\x0B\xB5ad\xE1V[` \x90\x81\x02\x92\x90\x92\x015`\0\x81\x81R`6\x90\x93R`@\x90\x92 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1B`\x01`\x01`@\x1B\x03\x88\x16\x02\x17\x90UP\x90Pa\x0C\x03\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa0\xC0V[\x9A\x99PPPPPPPPPPV[```u\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r'\x91\x90ac\xCBV[\x15a\rDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`4T`\x01`@\x1B\x90\x04`\xFF\x16a\r\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FEigenPod.hasEnabledRestaking: re`D\x82\x01Ru\x1C\xDD\x18Z\xDA[\x99\xC8\x1A\\\xC8\x1B\x9B\xDD\x08\x19[\x98X\x9B\x19Y`R\x1B`d\x82\x01R`\x84\x01a\nLV[\x85\x84\x14\x80\x15a\r\xCAWP\x83\x82\x14[a\x0EZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: validatorIndices and proof`d\x82\x01Rt\x0Ed\r\xAE\xAEn\x84\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`[\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`3T`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15\x80a\x0E\xAFWP`3Ta\x0E\x99\x90a\x0E\x94\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a5\x9EV[a6\x88V[`\x01`\x01`@\x1B\x03\x16\x89`\x01`\x01`@\x1B\x03\x16\x10\x15[a\x0F;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: proof must be in the epoch`d\x82\x01Rp\x100\xB3:2\xB9\x100\xB1\xBA4\xBB0\xBA4\xB7\xB7`y\x1B`\x84\x82\x01R`\xA4\x01a\nLV[Ba\x0FQa?H`\x01`\x01`@\x1B\x03\x8C\x16aW\xABV[\x10\x15a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: specified timestamp is too`d\x82\x01Rk\x08\x19\x98\\\x88\x1A[\x88\x1C\x18\\\xDD`\xA2\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8A\x16`\x04\x82\x01Ra\x10\x83\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10o\x91\x90ad\xF7V[\x895a\x10~` \x8C\x01\x8Cae\x10V[a6\xD5V[`\0\x80[\x87\x81\x10\x15a\x11'Wa\x11\t\x8B\x8B5\x8B\x8B\x85\x81\x81\x10a\x10\xA7Wa\x10\xA7ad\xE1V[\x90P` \x02\x01` \x81\x01\x90a\x10\xBC\x91\x90aeVV[\x8A\x8A\x86\x81\x81\x10a\x10\xCEWa\x10\xCEad\xE1V[\x90P` \x02\x81\x01\x90a\x10\xE0\x91\x90ae\x10V[\x8A\x8A\x88\x81\x81\x10a\x10\xF2Wa\x10\xF2ad\xE1V[\x90P` \x02\x81\x01\x90a\x11\x04\x91\x90aeqV[a8cV[a\x11\x13\x90\x83aW\xABV[\x91P\x80a\x11\x1F\x81ae\xBAV[\x91PPa\x10\x87V[P`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xACW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[```t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`\0\x80a\x12^\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa=\x1D\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[```x\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x13OW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x13\x11W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12\xA5V[PPPP\x90P\x90V[a\x13\x98`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\x03Wa\x14\x03a[JV[`\x02\x81\x11\x15a\x14\x14Wa\x14\x14a[JV[\x90RP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14=\x85\x85\x85\x85a>\x17V[\x95\x94PPPPPV[`\0a\x14W\x88\x88\x88\x88\x88\x88\x88a8cV[\x98\x97PPPPPPPPV[```w\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x14\xA6\x90ae\xD5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD2\x90ae\xD5V[\x80\x15a\x15\x1FW\x80`\x1F\x10a\x14\xF4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x1FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x02W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\x87V[```y\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x01W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15\xC3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15WV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x16aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90af\nV[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FEigenPod.stake: must initially s\x90\x82\x01R\x7Ftake for any validator with 32 e`d\x82\x01Rc:42\xB9`\xE1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x170a>\xF5V[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17T\x96\x95\x94\x93\x92\x91\x90af\x84V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x81W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x17\xB7\x92\x91\x90af\xD3V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x03`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18R\x91\x90ac\xCBV[\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x86\x84\x14\x80\x15a\x18}WP\x83\x82\x14[a\x19\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.verifyBalanceUpdates: v`D\x82\x01R\x7FalidatorIndices and proofs must `d\x82\x01Rm\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[Ba\x19\x1Ca?H`\x01`\x01`@\x1B\x03\x8C\x16aW\xABV[\x10\x15a\x19\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FEigenPod.verifyBalanceUpdates: s`D\x82\x01R\x7Fpecified timestamp is too far in`d\x82\x01Rd\x08\x1C\x18\\\xDD`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8A\x16`\x04\x82\x01Ra\x1AB\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A3\x91\x90ad\xF7V[\x875a\x10~` \x8A\x01\x8Aae\x10V[`\0\x80[\x88\x81\x10\x15a\x1A\xE6Wa\x1A\xC8\x8B\x8B\x8B\x84\x81\x81\x10a\x1AdWa\x1Adad\xE1V[\x90P` \x02\x01` \x81\x01\x90a\x1Ay\x91\x90aeVV[\x8A5\x8A\x8A\x86\x81\x81\x10a\x1A\x8DWa\x1A\x8Dad\xE1V[\x90P` \x02\x81\x01\x90a\x1A\x9F\x91\x90ae\x10V[\x8A\x8A\x88\x81\x81\x10a\x1A\xB1Wa\x1A\xB1ad\xE1V[\x90P` \x02\x81\x01\x90a\x1A\xC3\x91\x90aeqV[a0\xC0V[a\x1A\xD2\x90\x83af\xE7V[\x91P\x80a\x1A\xDE\x81ae\xBAV[\x91PPa\x1AFV[P`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91c\xC2\xC5\x1C@\x91\x16a\x1B+c;\x9A\xCA\0\x85ag(V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x11~V[a\x1B\x84`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x1B\xC7\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa=\x1D\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C4Wa\x1C4a[JV[`\x02\x81\x11\x15a\x1CEWa\x1CEa[JV[\x90RP\x93\x92PPPV[```v\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1C\x92\x90ae\xD5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xBE\x90ae\xD5V[\x80\x15a\x1D\x0BW\x80`\x1F\x10a\x1C\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1CsV[`fT`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x1DAWP`fTa\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1EGW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1D\xCF\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01ag\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D\xE9\x91ag\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1E&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E+V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1EC\x91\x90ac\xCBV[\x91PP[\x91\x90PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`4T`\x01`@\x1B\x90\x04`\xFF\x16\x15a\x1E\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ad\x92V[`3Ta\x1E\xB5\x90`\x01`\x01`\xA0\x1B\x03\x16a+\x14V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90af\nV[a\x1F\rc;\x9A\xCA\0\x82ah\x10V[\x15a\x1F\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountWei must be a who`d\x82\x01Rm\x1B\x19H\x11\xDD\xD9ZH\x18[[\xDD[\x9D`\x92\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0a\x1F\xA7c;\x9A\xCA\0\x83ah$V[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a `W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`b`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountGwei exceeds with`d\x82\x01R\x7FdrawableRestakedExecutionLayerGw`\x84\x82\x01Raei`\xF0\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`4\x80T\x82\x91\x90`\0\x90a ~\x90\x84\x90`\x01`\x01`@\x1B\x03\x16ah8V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa \xDD\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a \xEF\x83\x83a?:V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a!\x14WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a!.WP0;\x15\x80\x15a!.WP`\0T`\xFF\x16`\x01\x14[a!\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\nLV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a!\xB4W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.initialize: podOwner ca`D\x82\x01Rsnnot be zero address``\x1B`d\x82\x01R`\x84\x01a\nLV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x81\x17\x90\x91U`4\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`@Q\x7F\xCA\x8D\xFC\x8C^\ng\xA7E\x01\xC0r\xA32_hRY\xBE\xBB\xAE|\xFD#\n\xB8Q\x98\xA7\x8Bp\xCD\x90`\0\x90\xA2\x80\x15a\"\xC8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\xEE\x87\x87\x87\x87\x87\x87a?DV[\x97\x96PPPPPPPV[`\0\x82\x81R`6` R`@\x90 \x80T\x82\x91\x90`\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a#*Wa#*a[JV[\x02\x17\x90UPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a#]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xE9\x91\x90ac\xCBV[\x15a$\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x82Q\x84Q\x14a$\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.recoverTokens: tokenLis`D\x82\x01R\x7Ft and amountsToWithdraw must be `d\x82\x01Rj\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0[\x84Q\x81\x10\x15a$\xFFWa$\xED\x83\x85\x83\x81Q\x81\x10a$\xB3Wa$\xB3ad\xE1V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a$\xCDWa$\xCDad\xE1V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16aA\x82\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a$\xF7\x81ae\xBAV[\x91PPa$\x94V[PPPPPV[```r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x04\x80\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xF1\x91\x90ac\xCBV[\x15a&\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x83\x86\x14\x80\x15a&\x1CWP\x85\x88\x14[\x80\x15a&'WP\x87\x82\x14[a&\x9BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPod.verifyAndProcessWithdra`D\x82\x01R\x7Fwals: inputs must be same length`d\x82\x01R`\x84\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8C\x16`\x04\x82\x01Ra'?\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'0\x91\x90ad\xF7V[\x8B5a\x10~` \x8E\x01\x8Eae\x10V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83\x81\x10\x15a(?W`\0a'\xFA\x8D5\x8D\x8D\x85\x81\x81\x10a'wWa'wad\xE1V[\x90P` \x02\x81\x01\x90a'\x89\x91\x90ah`V[\x8C\x8C\x86\x81\x81\x10a'\x9BWa'\x9Bad\xE1V[\x90P` \x02\x81\x01\x90a'\xAD\x91\x90ae\x10V[\x8C\x8C\x88\x81\x81\x10a'\xBFWa'\xBFad\xE1V[\x90P` \x02\x81\x01\x90a'\xD1\x91\x90aeqV[\x8C\x8C\x8A\x81\x81\x10a'\xE3Wa'\xE3ad\xE1V[\x90P` \x02\x81\x01\x90a'\xF5\x91\x90aeqV[a+HV[\x80Q\x84Q\x91\x92P\x90\x84\x90a(\x0F\x90\x83\x90aW\xABV[\x90RP` \x80\x82\x01Q\x90\x84\x01\x80Qa((\x90\x83\x90af\xE7V[\x90RP\x81\x90Pa(7\x81ae\xBAV[\x91PPa'VV[P\x80Q\x15a(nW`3T\x81Qa(n\x91`\x01`\x01`\xA0\x1B\x03\x16\x90a(i\x90c;\x9A\xCA\0\x90ahwV[aA\xD4V[` \x81\x01Q\x15a)#W`3T` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c\xC2\xC5\x1C@\x92\x91\x16\x90a(\xC4\x90c;\x9A\xCA\0\x90ag(V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\x1EW=`\0\x80>=`\0\xFD[PPPP[PPPPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a)[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xE7\x91\x90ac\xCBV[\x15a*\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`7T\x82\x11\x15a*\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FEigenPod.withdrawnonBeaconChainE`D\x82\x01R\x7FTHBalanceWei: amountToWithdraw i`d\x82\x01R\x7Fs greater than nonBeaconChainETH`\x84\x82\x01RiBalanceWei`\xB0\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[\x81`7`\0\x82\x82Ta*\xC7\x91\x90ah\x96V[\x90\x91UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F0B\n\xAC\xD0(\xAB\xB3\xC1\xFD\x03\xAB\xA2S\xAEr]m\xDDR\xD1l\x9A\xC4\xCBWB\xCDC\xF50\x96\x90` \x01`@Q\x80\x91\x03\x90\xA2a \xEF\x83\x83aA\xD4V[`3\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16Bc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02\x17\x90U`\0`7Ua+E\x81GaA\xD4V[PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra+ma+h\x89ai\x1CV[aBbV[`3T`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x10\x15a,/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R\x7FEigenPod.proofIsForValidTimestam`D\x82\x01R\x7Fp: beacon chain proof must be at`d\x82\x01R\x7F or after mostRecentWithdrawalTi`\x84\x82\x01Rf\x06\xD6W7F\x16\xD7`\xCC\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`\0a,=a+h\x8Bai\x1CV[\x90P`\0a,}\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[\x90P`\0\x80\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a,\xAAWa,\xAAa[JV[\x14\x15a-aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FEigenPod._verifyAndProcessWithdr`D\x82\x01R\x7Fawal: Validator never proven to `d\x82\x01R\x7Fhave withdrawal credentials poin`\x84\x82\x01Rs\x1D\x19Y\x08\x1D\x1B\xC8\x1D\x1A\x1A\\\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`@\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a. W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FEigenPod._verifyAndProcessWithdr`D\x82\x01R\x7Fawal: withdrawal has already bee`d\x82\x01R\x7Fn proven for this timestamp\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\x01`5`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa.\xFD\x8C\x87\x87\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cD\xE7\x1C\x80`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xF8\x91\x90ajXV[aB\x96V[`\0a/;\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaL\xB7\x92PPPV[\x90Pa/K\x8D\x8A\x8A\x8E\x8E\x86aL\xDCV[`\0a/\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaO3\x92PPPV[\x90Pa/\xC7\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOK\x92PPPV[`\x01`\x01`@\x1B\x03\x16a/\xE1a/\xDC\x8Fai\x1CV[aOcV[`\x01`\x01`@\x1B\x03\x16\x10a0\x99W`3T`\0\x84\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93Ra0\x8E\x93\x86\x93\x88\x93\x8A\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x88\x92\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a0uWa0ua[JV[`\x02\x81\x11\x15a0\x86Wa0\x86a[JV[\x90RPa?DV[\x95PPPPPa0\xB3V[`3Ta0\x8E\x90\x83\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x84a>\x17V[P\x98\x97PPPPPPPPV[`\0\x80a0\xFF\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOu\x92PPPV[\x90P`\0a1?\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a1\xAEWa1\xAEa[JV[`\x02\x81\x11\x15a1\xBFWa1\xBFa[JV[\x81RPP\x90P\x8A`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a2vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: Va`D\x82\x01R\x7Flidators balance has already bee`d\x82\x01R\x7Fn updated for this timestamp\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\x01\x81``\x01Q`\x02\x81\x11\x15a2\x8EWa2\x8Ea[JV[\x14a2\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: Va`D\x82\x01Rqlidator not active`p\x1B`d\x82\x01R`\x84\x01a\nLV[a2\xFF\x8Ba5\x9EV[`\x01`\x01`@\x1B\x03\x16a3D\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOK\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x11a3\xE7W`\0\x83`\x01`\x01`@\x1B\x03\x16\x11a3\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: va`D\x82\x01R\x7Flidator is withdrawable but has `d\x82\x01Rl77\xBA\x10;\xB4\xBA4290\xBB\xB7`\x99\x1B`\x84\x82\x01R`\xA4\x01a\nLV[a3\xF5\x89\x87\x87\x8B\x8B\x8FaL\xDCV[` \x81\x01Q`\0`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x86\x16\x11\x15a4WWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4ZV[P\x83[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x80\x86\x01\x91\x82R\x8F\x83\x16`@\x80\x88\x01\x91\x82R`\0\x89\x81R`6\x90\x93R\x90\x91 \x86Q\x81T\x93Q\x92Q\x85\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x93\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x95\x16\x17\x92\x90\x92\x17\x90\x81\x16\x83\x17\x82U``\x86\x01Q\x86\x93\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a5\x02Wa5\x02a[JV[\x02\x17\x90UP\x90PP\x81`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a5\x8EW\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x8C\x8E\x83`@Qa5y\x93\x92\x91\x90d\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x83R`\x01`\x01`@\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a5\x8B\x81\x83aO\x8DV[\x95P[PPPPP\x97\x96PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a6HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FEigenPod._timestampToEpoch: time`D\x82\x01R\x7Fstamp is before genesis\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nLV[a6T`\x0C` ajuV[a6~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84ah8V[a\x12{\x91\x90aj\xA4V[`\0a6\x96`\x0C` ajuV[a6\xA1\x83`\x01aj\xCAV[a6\xAB\x91\x90ajuV[a\x12{\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aj\xCAV[a6\xE1`\x03` ahwV[\x81\x14a7qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7FtAgainstLatestBlockRoot: Proof h`d\x82\x01Rr\x0C.d\r-\xCCm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`k\x1B`\x84\x82\x01R`\xA4\x01a\nLV[a7\xB6\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92P\x87\x91P`\x03\x90PaO\xACV[a8]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7FtAgainstLatestBlockRoot: Invalid`d\x82\x01R\x7F latest block header root merkle`\x84\x82\x01Re\x10897\xB7\xB3`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[PPPPV[`\0\x80a8\xA2\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a9\x11Wa9\x11a[JV[`\x02\x81\x11\x15a9\"Wa9\"a[JV[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a9?Wa9?a[JV[\x14a9\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R\x7FEigenPod.verifyCorrectWithdrawal`D\x82\x01R\x7FCredentials: Validator must be i`d\x82\x01R\x7Fnactive to prove withdrawal cred`\x84\x82\x01Rfentials`\xC8\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[a9\xF0a>\xF5V[a9\xF9\x90aj\xF5V[a:5\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaO\xC4\x92PPPV[\x14a:\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.verifyCorrectWithdrawal`D\x82\x01R\x7FCredentials: Proof is not for th`d\x82\x01Rj\x1A\\\xC8\x11ZY\xD9[\x94\x1B\xD9`\xAA\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0a:\xFA\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOu\x92PPPV[\x90Pa;\n\x8A\x87\x87\x8B\x8B\x8EaL\xDCV[`9\x80T\x90`\0a;\x1A\x83ae\xBAV[\x90\x91UPP`\x01``\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x89\x16\x82R`\x01`\x01`@\x1B\x03\x8B\x81\x16`@\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x11\x15a;\xA0W`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83\x01Ra;\xB0V[`\x01`\x01`@\x1B\x03\x81\x16` \x83\x01R[`\0\x83\x81R`6` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x92\x86\x01Q\x93\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x85\x01Q\x85\x93\x91\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a<NWa<Na[JV[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8B\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x89\x8C\x84` \x01Q`@Qa<\xE9\x93\x92\x91\x90d\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x83R`\x01`\x01`@\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1c;\x9A\xCA\0\x82` \x01Q`\x01`\x01`@\x1B\x03\x16a=\x0E\x91\x90ahwV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a=\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigenPod._calculateValidatorPubk`D\x82\x01R\x7FeyHash must be a 48-byte BLS pub`d\x82\x01Rflic key`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Q`\x02\x90a=\xBD\x90\x84\x90`\0\x90` \x01ak\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra=\xD7\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a=\xF4W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12{\x91\x90ad\xF7V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x87\x16\x81R`\x01`\x01`@\x1B\x03\x80\x87\x16` \x83\x01R\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\x8As5qB1\xDB\xD5Q\xAA\xBAc\x14\xF4\xA9z\x14\xC2\x01\xE5:>%\xE1\x14\x03%\xCD\xF6}zN\x90``\x01`@Q\x80\x91\x03\x90\xA2`8\x80T\x83\x91\x90`\0\x90a>\xA8\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aj\xCAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@Q\x80`@\x01`@R\x80\x83`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81RP\x90P\x94\x93PPPPV[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\"\xC8\x82\x82aO\xD9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x16\x84`\x01`\x01`@\x1B\x03\x16\x11\x15a?\xBBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a?\xBEV[P\x82[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra?\xDC\x82\x86ah8V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`4\x80T\x84\x92`\0\x91a?\xFE\x91\x85\x91\x16aj\xCAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa@0\x82\x85` \x01QaO\x8DV[` \x82\x01R`\x02\x84``\x01Q`\x02\x81\x11\x15a@MWa@Ma[JV[\x14a@oW`9\x80T\x90`\0a@b\x83akHV[\x90\x91UPP`\x02``\x85\x01R[`\0` \x80\x86\x01\x82\x81R\x8A\x83R`6\x90\x91R`@\x91\x82\x90 \x86Q\x81T\x92Q\x93\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x92\x90\x91\x16\x91\x90\x91\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x87\x01Q\x87\x93\x91\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15aA\x14WaA\x14a[JV[\x02\x17\x90UPP`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x8A\x81\x16` \x83\x01R\x88\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x92P\x7F\xB7j\x93\xBBd\x9E\xCERF\x88\xF1\xA0\x1D\x18N\x0B\xBE\xBC\xDAX\xEA\xE8\x0C(\xA8\x98\xBE\xC3\xFBZ\tc\x91\x81\x90\x03``\x01\x90\xA2\x98\x97PPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra \xEF\x90\x84\x90aP\xF2V[`3T`@Qc06\xCDS`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x83\x82\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC0\xDB5L\x90\x83\x90`D\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15aBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15aBYW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0a\x12{\x82a\x01@\x01QaQ\xC4V[`\0\x81`\0\x81Q\x81\x10aB\x87WaB\x87ad\xE1V[` \x02` \x01\x01Q\x90P\x91\x90PV[aB\xA1`\x02\x80alCV[\x83\x14aC\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalFields has incorre`d\x82\x01Rh\x0Cn\x84\r\x8C\xAD\xCC\xEE\x8D`\xBB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aC!`\r`\x02alCV[aC1`\xC0\x84\x01`\xA0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aC\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: blockRootIndex is too large\0`d\x82\x01R`\x84\x01a\nLV[aC\xA7`\x04`\x02alCV[aC\xB8a\x01\0\x84\x01`\xE0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aD$W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalIndex is too large`d\x82\x01R`\x84\x01a\nLV[aD0`\x18`\x02alCV[aD@`\xE0\x84\x01`\xC0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aD\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: historicalSummaryIndex is to`d\x82\x01Rfo large`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0`\x01`\x01`@\x1B\x03\x82\x16aD\xD2a+h\x85ai\x1CV[`\x01`\x01`@\x1B\x03\x16\x10aD\xE7W`\x05aD\xEAV[`\x04[\x90PaD\xF7`\x04\x82aW\xABV[aE\x02\x90`\x01aW\xABV[aE\r\x90` ahwV[aE\x17\x84\x80ae\x10V[\x90P\x14aE\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalProof has incorrec`d\x82\x01Rg\x0E\x84\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aE\x97`\x04`\x03aW\xABV[aE\xA2\x90` ahwV[aE\xAF`@\x85\x01\x85ae\x10V[\x90P\x14aF)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: executionPayloadProof has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aF5`\x03` ahwV[aFB` \x85\x01\x85ae\x10V[\x90P\x14aF\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: slotProof has incorrect leng`d\x82\x01Ra\x0E\x8D`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aF\xBB\x81` ahwV[aF\xC8``\x85\x01\x85ae\x10V[\x90P\x14aG;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: timestampProof has incorrect`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\raGI`\x18`\x01aW\xABV[aGT\x90`\x05aW\xABV[aG_\x90`\x01aW\xABV[aGi\x91\x90aW\xABV[aGt\x90` ahwV[aG\x81`\x80\x85\x01\x85ae\x10V[\x90P\x14aH\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`X`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: historicalSummaryBlockRootPr`d\x82\x01R\x7Foof has incorrect length\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\0aH\x1C`\xC0\x85\x01`\xA0\x86\x01aa(V[`\x01`\x01`@\x1B\x03\x16`\0aH3`\r`\x01aW\xABV[aHC`\xE0\x88\x01`\xC0\x89\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x90\x1B`\raH\\`\x18`\x01aW\xABV[aHg\x90`\x01aW\xABV[aHq\x91\x90aW\xABV[`\x1B\x90\x1B\x17\x17\x17\x90PaH\xCCaH\x8A`\x80\x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92PPPa\x01\0\x87\x015\x84aO\xACV[aI?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid historicalsummary me`d\x82\x01Ri95\xB62\x90897\xB7\xB3`\xB1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aI\x96aIO` \x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RPa\x01\0\x8A\x015\x93Pa\x01 \x8A\x015\x92P\x90PaO\xACV[aI\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid slot merkle proof\0\0\0`d\x82\x01R`\x84\x01a\nLV[`IaJNaJ\x08`@\x87\x01\x87ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01\0\x87\x015a\x01`\x88\x015\x84aO\xACV[aJ\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid executionPayload mer`d\x82\x01Rh5\xB62\x90897\xB7\xB3`\xB9\x1B`\x84\x82\x01R`\xA4\x01a\nLV[PaK\x18aJ\xD1``\x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01`\x86\x015a\x01@\x87\x015`\taO\xACV[aK\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid timestamp merkle pro`d\x82\x01Ra7\xB3`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0aK\x96a\x01\0\x86\x01`\xE0\x87\x01aa(V[`\x01`\x01`@\x1B\x03\x16aK\xAB`\x04`\x01aW\xABV[`\x0E\x90\x1B\x17\x90P`\0aK\xF0\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaR+\x92PPPV[\x90PaL@aK\xFF\x87\x80ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01`\x88\x015\x83\x85aO\xACV[aL\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid withdrawal merkle pr`d\x82\x01Rb7\xB7\xB3`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\nLV[PPPPPPPPPV[`\0a\x12{\x82`\x01\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[` \x02` \x01\x01QaQ\xC4V[aL\xE8`\x03`\x02alCV[\x84\x14aMsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Validator fields has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\x05aM\x81`(`\x01aW\xABV[aM\x8B\x91\x90aW\xABV[aM\x96\x90` ahwV[\x82\x14aN\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Proof has incorrect len`d\x82\x01Rb\x0C\xEE\x8D`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0d\xFF\xFF\xFF\xFF\xFF\x82\x16aN,`(`\x01aW\xABV[`\x0B\x90\x1B\x17\x90P`\0aNq\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaR+\x92PPPV[\x90PaN\xB7\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x85\x91P\x86\x90PaO\xACV[aO)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Invalid merkle proof\0\0\0`d\x82\x01R`\x84\x01a\nLV[PPPPPPPPV[`\0a\x12{\x82`\x03\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0a\x12{\x82`\x07\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0` a6~\x83a\x01 \x01QaQ\xC4V[`\0a\x12{\x82`\x02\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0aO\xA5`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16alOV[\x93\x92PPPV[`\0\x83aO\xBA\x86\x85\x85aT\xD8V[\x14\x95\x94PPPPPV[`\0\x81`\x01\x81Q\x81\x10aB\x87WaB\x87ad\xE1V[\x80G\x10\x15aP)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\nLV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aPvW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aP{V[``\x91P[PP\x90P\x80a \xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nLV[`\0aQG\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aV$\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a \xEFW\x80\x80` \x01\x90Q\x81\x01\x90aQe\x91\x90ac\xCBV[a \xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\nLV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[`\0\x80`\x02\x83QaR<\x91\x90ah$V[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aRXWaRXa_\xBAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aR\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15aS\x88W`\x02\x85aR\x9C\x83\x83ahwV[\x81Q\x81\x10aR\xACWaR\xACad\xE1V[` \x02` \x01\x01Q\x86\x83`\x02aR\xC2\x91\x90ahwV[aR\xCD\x90`\x01aW\xABV[\x81Q\x81\x10aR\xDDWaR\xDDad\xE1V[` \x02` \x01\x01Q`@Q` \x01aR\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaS\x19\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15aS6W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aSY\x91\x90ad\xF7V[\x82\x82\x81Q\x81\x10aSkWaSkad\xE1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80aS\x80\x81ae\xBAV[\x91PPaR\x87V[PaS\x94`\x02\x83ah$V[\x91P[\x81\x15aT\xB4W`\0[\x82\x81\x10\x15aT\xA1W`\x02\x82aS\xB5\x83\x83ahwV[\x81Q\x81\x10aS\xC5WaS\xC5ad\xE1V[` \x02` \x01\x01Q\x83\x83`\x02aS\xDB\x91\x90ahwV[aS\xE6\x90`\x01aW\xABV[\x81Q\x81\x10aS\xF6WaS\xF6ad\xE1V[` \x02` \x01\x01Q`@Q` \x01aT\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaT2\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15aTOW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aTr\x91\x90ad\xF7V[\x82\x82\x81Q\x81\x10aT\x84WaT\x84ad\xE1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80aT\x99\x81ae\xBAV[\x91PPaS\xA0V[PaT\xAD`\x02\x83ah$V[\x91PaS\x97V[\x80`\0\x81Q\x81\x10aT\xC7WaT\xC7ad\xE1V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0\x83Q`\0\x14\x15\x80\x15aT\xF7WP` \x84QaT\xF5\x91\x90ah\x10V[\x15[aU\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FMerkle.processInclusionProofSha2`D\x82\x01R\x7F56: proof length should be a non`d\x82\x01Rs\x16\xBD2\xB97\x906\xBA\xB6:4\xB862\x907\xB3\x10\x19\x99`a\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11aV\x1AWaU\xAA`\x02\x85ah\x10V[aU\xDDW\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAaU\xD2W`\0\x80\xFD[`\x02\x84\x04\x93PaV\x08V[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAaV\x01W`\0\x80\xFD[`\x02\x84\x04\x93P[aV\x13` \x82aW\xABV[\x90PaU\x97V[PQ\x94\x93PPPPV[``aV3\x84\x84`\0\x85aV;V[\x94\x93PPPPV[``\x82G\x10\x15aV\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\nLV[`\x01`\x01`\xA0\x1B\x03\x85\x16;aV\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nLV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaW\x0F\x91\x90ag\xDEV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aWLW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aWQV[``\x91P[P\x91P\x91Pa\"\xEE\x82\x82\x86``\x83\x15aWkWP\x81aO\xA5V[\x82Q\x15aW{W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x91\x90al\x8EV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aW\xBEWaW\xBEaW\x95V[P\x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12aW\xD5W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aX\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aX\x1DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aX4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aX\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15aXkW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aX\x89W`\0\x80\xFD[\x90\x8A\x01\x90a\x01\x80\x82\x8D\x03\x12\x15aX\x9EW`\0\x80\xFD[\x90\x97P`@\x8A\x015\x90\x80\x82\x11\x15aX\xB4W`\0\x80\xFD[aX\xC0\x8C\x83\x8D\x01aW\xC3V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15aX\xD9W`\0\x80\xFD[aX\xE5\x8C\x83\x8D\x01aX\x0BV[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15aX\xFEW`\0\x80\xFD[PaY\x0B\x8B\x82\x8C\x01aX\x0BV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aY`W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aY;V[P\x90\x96\x95PPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a+EW`\0\x80\xFD[\x805a\x1EG\x81aYlV[`\0\x80`@\x83\x85\x03\x12\x15aY\x9FW`\0\x80\xFD[\x825\x91P` \x83\x015aY\xB1\x81aYlV[\x80\x91PP\x92P\x92\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EGW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aY\xEDW`\0\x80\xFD[\x885aY\xF8\x81aYlV[\x97PaZ\x06` \x8A\x01aY\xBCV[\x96P`@\x89\x015\x95P``\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ)W`\0\x80\xFD[aZ5\x8C\x83\x8D\x01aW\xC3V[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aZNW`\0\x80\xFD[PaZ[\x8B\x82\x8C\x01aX\x0BV[\x90\x94P\x92PP`\xA0\x89\x015aZo\x81aYlV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x84\x03\x12\x15aZ\x92W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15aZ\xB4W`\0\x80\xFD[\x885aZ\xBF\x81aYlV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xDBW`\0\x80\xFD[aZ\xE7\x8C\x83\x8D\x01aZ\x80V[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15aZ\xFDW`\0\x80\xFD[aX\xC0\x8C\x83\x8D\x01aX\x0BV[`\0\x80` \x83\x85\x03\x12\x15a[\x1CW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a[2W`\0\x80\xFD[a[>\x85\x82\x86\x01aW\xC3V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a[~WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x12{\x82\x84a[`V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\\4W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\\\x1FW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a[\xF5V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a[\xB8V[P\x91\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\\UW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Qa\\\x9D``\x84\x01\x82a[`V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a+EW`\0\x80\xFD[\x805a\x1EG\x81a\\\xA4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\\\xDAW`\0\x80\xFD[a\\\xE3\x85aY\xBCV[\x93P` \x85\x015a\\\xF3\x81aYlV[\x92P`@\x85\x015a]\x03\x81a\\\xA4V[\x91P``\x85\x015a]\x13\x81aYlV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a]9W`\0\x80\xFD[\x875a]D\x81aYlV[\x96P` \x88\x015\x95Pa]Y`@\x89\x01aY\xBCV[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a]uW`\0\x80\xFD[a]\x81\x8B\x83\x8C\x01aW\xC3V[\x90\x96P\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15a]\x9AW`\0\x80\xFD[Pa]\xA7\x8A\x82\x8B\x01aX\x0BV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0[\x83\x81\x10\x15a]\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a]\xBDV[\x83\x81\x11\x15a8]WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra]\xFE\x81` \x86\x01` \x86\x01a]\xBAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a^gW`?\x19\x88\x86\x03\x01\x84Ra^U\x85\x83Qa]\xE6V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a^9V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a^\x8CW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a^\xA3W`\0\x80\xFD[a^\xAF\x89\x83\x8A\x01aW\xC3V[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a^\xC8W`\0\x80\xFD[Pa^\xD5\x88\x82\x89\x01aW\xC3V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a_\x03W`\0\x80\xFD[\x885a_\x0E\x81aYlV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a_*W`\0\x80\xFD[a_6\x8C\x83\x8D\x01aX\x0BV[\x90\x99P\x97P`@\x8B\x015\x91P\x80\x82\x11\x15a_OW`\0\x80\xFD[a_[\x8C\x83\x8D\x01aZ\x80V[\x96P``\x8B\x015\x91P\x80\x82\x11\x15aX\xD9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a_\x84W`\0\x80\xFD[\x825a_\x8F\x81a\\\xA4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a_\xAFW`\0\x80\xFD[\x815aO\xA5\x81a\\\xA4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a_\xF2Wa_\xF2a_\xBAV[`@R\x90V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a_\xF2Wa_\xF2a_\xBAV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`CWa`Ca_\xBAV[`@R\x91\x90PV[\x805`\x03\x81\x10a\x1EGW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01 \x81\x12\x15a`uW`\0\x80\xFD[a`~\x88aY\xBCV[\x96P` \x88\x015\x95P`@\x88\x015a`\x95\x81aYlV[\x94P``\x88\x015a`\xA5\x81a\\\xA4V[\x93P`\x80\x88\x015a`\xB5\x81aYlV[\x92P`\x80`\x9F\x19\x82\x01\x12\x15a`\xC9W`\0\x80\xFD[Pa`\xD2a_\xD0V[`\xA0\x88\x015a`\xE0\x81aYlV[\x81R`\xC0\x88\x015a`\xF0\x81aYlV[` \x82\x01R`\xE0\x88\x015aa\x03\x81aYlV[`@\x82\x01Raa\x15a\x01\0\x89\x01a`KV[``\x82\x01R\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15aa:W`\0\x80\xFD[\x815aO\xA5\x81aYlV[`\0\x80`@\x83\x85\x03\x12\x15aaXW`\0\x80\xFD[\x825\x91Paah` \x84\x01a`KV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aa\x8AWaa\x8Aa_\xBAV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aa\xA5W`\0\x80\xFD[\x815` aa\xBAaa\xB5\x83aaqV[a`\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aa\xD9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aa\xF4W\x805\x83R\x91\x83\x01\x91\x83\x01aa\xDDV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15ab\x14W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ab+W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12ab?W`\0\x80\xFD[\x815` abOaa\xB5\x83aaqV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15abnW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15ab\x95W\x855ab\x86\x81a\\\xA4V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90absV[\x97PP\x87\x015\x92PP\x80\x82\x11\x15ab\xABW`\0\x80\xFD[Pab\xB8\x86\x82\x87\x01aa\x94V[\x92PPab\xC7`@\x85\x01a\\\xB9V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15ab\xEFW`\0\x80\xFD[ab\xF8\x8BaY\x81V[\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ac\x14W`\0\x80\xFD[ac \x8E\x83\x8F\x01aZ\x80V[\x9AP`@\x8D\x015\x91P\x80\x82\x11\x15ac6W`\0\x80\xFD[acB\x8E\x83\x8F\x01aX\x0BV[\x90\x9AP\x98P``\x8D\x015\x91P\x80\x82\x11\x15ac[W`\0\x80\xFD[acg\x8E\x83\x8F\x01aX\x0BV[\x90\x98P\x96P`\x80\x8D\x015\x91P\x80\x82\x11\x15ac\x80W`\0\x80\xFD[ac\x8C\x8E\x83\x8F\x01aX\x0BV[\x90\x96P\x94P`\xA0\x8D\x015\x91P\x80\x82\x11\x15ac\xA5W`\0\x80\xFD[Pac\xB2\x8D\x82\x8E\x01aX\x0BV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15ac\xDDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14aO\xA5W`\0\x80\xFD[` \x80\x82R`>\x90\x82\x01R\x7FEigenPod.onlyWhenNotPaused: inde`@\x82\x01R\x7Fx is paused in EigenPodManager\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`(\x90\x82\x01R\x7FEigenPod.onlyEigenPodOwner: not `@\x82\x01Rg87\xB2'\xBB\xB72\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`/\x90\x82\x01R\x7FEigenPod.hasNeverRestaked: resta`@\x82\x01Rn\x1A\xDA[\x99\xC8\x1A\\\xC8\x19[\x98X\x9B\x19Y`\x8A\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15ae\tW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ae'W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aeAW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aX\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aehW`\0\x80\xFD[aO\xA5\x82aY\xBCV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ae\x88W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ae\xA2W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aX\x04W`\0\x80\xFD[`\0`\0\x19\x82\x14\x15ae\xCEWae\xCEaW\x95V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80ae\xE9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aZ\x92WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`1\x90\x82\x01R\x7FEigenPod.onlyEigenPodManager: no`@\x82\x01Rp:\x102\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`y\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0af\x98`\x80\x83\x01\x88\x8Aaf[V[\x82\x81\x03` \x84\x01Raf\xAA\x81\x88a]\xE6V[\x90P\x82\x81\x03`@\x84\x01Raf\xBF\x81\x86\x88af[V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0aV3` \x83\x01\x84\x86af[V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15ag\tWag\taW\x95V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15ag\"Wag\"aW\x95V[PP\x01\x90V[`\0`\x01`\x01`\xFF\x1B\x03\x81\x84\x13\x82\x84\x13\x80\x82\x16\x86\x84\x04\x86\x11\x16\x15agNWagNaW\x95V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15agmWagmaW\x95V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15ag\x89Wag\x89aW\x95V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15ag\x9FWag\x9FaW\x95V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90ag\xD0\x81`\x04\x85\x01` \x87\x01a]\xBAV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qag\xF0\x81\x84` \x87\x01a]\xBAV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82ah\x1FWah\x1Fag\xFAV[P\x06\x90V[`\0\x82ah3Wah3ag\xFAV[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15ahXWahXaW\x95V[\x03\x93\x92PPPV[`\0\x825a\x01~\x19\x836\x03\x01\x81\x12ag\xF0W`\0\x80\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15ah\x91Wah\x91aW\x95V[P\x02\x90V[`\0\x82\x82\x10\x15ah\xA8Wah\xA8aW\x95V[P\x03\x90V[`\0\x82`\x1F\x83\x01\x12ah\xBEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xD7Wah\xD7a_\xBAV[ah\xEA`\x1F\x82\x01`\x1F\x19\x16` \x01a`\x1BV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15ah\xFFW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0a\x01\x80\x826\x03\x12\x15ai/W`\0\x80\xFD[ai7a_\xF8V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aiNW`\0\x80\xFD[aiZ6\x83\x87\x01ah\xADV[\x83R` \x85\x015\x91P\x80\x82\x11\x15aipW`\0\x80\xFD[ai|6\x83\x87\x01ah\xADV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15ai\x95W`\0\x80\xFD[ai\xA16\x83\x87\x01ah\xADV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15ai\xBAW`\0\x80\xFD[ai\xC66\x83\x87\x01ah\xADV[``\x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15ai\xDFW`\0\x80\xFD[Pai\xEC6\x82\x86\x01ah\xADV[`\x80\x83\x01RPai\xFE`\xA0\x84\x01aY\x81V[`\xA0\x82\x01Raj\x0F`\xC0\x84\x01aY\x81V[`\xC0\x82\x01Raj `\xE0\x84\x01aY\x81V[`\xE0\x82\x01Ra\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x80\x84\x015\x90\x82\x01Ra\x01@\x80\x84\x015\x90\x82\x01Ra\x01`\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15ajjW`\0\x80\xFD[\x81QaO\xA5\x81aYlV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aj\x9BWaj\x9BaW\x95V[\x02\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aj\xBEWaj\xBEag\xFAV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aj\xECWaj\xECaW\x95V[\x01\x94\x93PPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aZ\x92W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0\x83Qak+\x81\x84` \x88\x01a]\xBAV[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81akWWakWaW\x95V[P`\0\x19\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15ak\x9AW\x81`\0\x19\x04\x82\x11\x15ak\x80Wak\x80aW\x95V[\x80\x85\x16\x15ak\x8DW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90akdV[P\x92P\x92\x90PV[`\0\x82ak\xB1WP`\x01a\x12{V[\x81ak\xBEWP`\0a\x12{V[\x81`\x01\x81\x14ak\xD4W`\x02\x81\x14ak\xDEWak\xFAV[`\x01\x91PPa\x12{V[`\xFF\x84\x11\x15ak\xEFWak\xEFaW\x95V[PP`\x01\x82\x1Ba\x12{V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15al\x1DWP\x81\x81\na\x12{V[al'\x83\x83ak_V[\x80`\0\x19\x04\x82\x11\x15al;Wal;aW\x95V[\x02\x93\x92PPPV[`\0aO\xA5\x83\x83ak\xA2V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15almWalmaW\x95V[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15al\x88Wal\x88aW\x95V[PP\x03\x90V[` \x81R`\0aO\xA5` \x83\x01\x84a]\xE6V\xFEBeaconChainProofs.verifyWithdraw\xA2dipfsX\"\x12 \xA9\xC8\x01k\x8B\x8B\x8Dv`\x17%\xFE\xBA\xBD#pg\xAA:\xA9l\x95.\x1E:\xC2\x0C\xC8\x82\xF1n\xA8dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102815760003560e01c80637bb3dbf31161014f578063c4907442116100c1578063e20c9f711161007a578063e20c9f71146108ea578063e251ef52146108ff578063e2c834451461091f578063f28824611461093f578063fa7626d414610973578063fe80b0871461098d57600080fd5b8063c490744214610802578063c4d66de814610822578063ca044e8c14610842578063d168eb5114610862578063d79ed726146108aa578063dda3346c146108ca57600080fd5b80639b4e4634116101135780639b4e463414610770578063a50600f414610783578063b522538a146107a3578063b5508aa9146107c3578063ba414fa6146107d8578063baa7145a146107ed57600080fd5b80637bb3dbf3146106d2578063816d53f9146106f257806385226c811461071257806387e0d28914610734578063916a17c61461075b57600080fd5b80633e5e3c23116101f35780635d3f65b6116101ac5780635d3f65b6146105d857806366d9a9a0146105f85780636fcd0e531461061a57806373a97ee8146106475780637439841f1461066757806374cdd7981461069e57600080fd5b80633e5e3c23146104d35780633f65cf19146104e85780633f7286f4146105085780634665bcda1461051d5780635229564a1461055157806358eaee79146105ab57600080fd5b80631ed7831c116102455780631ed7831c146103e65780633106ab53146104085780633474aa161461043957806334bea20a1461045957806337deea70146104945780633b4c57c6146104b357600080fd5b80630b18ff66146102d75780630cd4649e14610314578063115cd5e41461032b5780631a5057be146103665780631d905d5c1461039a57600080fd5b366102d257346037600082825461029891906157ab565b90915550506040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156102e357600080fd5b506033546102f7906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561032057600080fd5b506103296109a3565b005b34801561033757600080fd5b5061034b61034636600461584f565b610b0c565b6040805182518152602092830151928101929092520161030b565b34801561037257600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b3480156103a657600080fd5b506103ce7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160401b03909116815260200161030b565b3480156103f257600080fd5b506103fb610b3d565b60405161030b919061591f565b34801561041457600080fd5b5060345461042990600160401b900460ff1681565b604051901515815260200161030b565b34801561044557600080fd5b506034546103ce906001600160401b031681565b34801561046557600080fd5b5061042961047436600461598c565b603560209081526000928352604080842090915290825290205460ff1681565b3480156104a057600080fd5b506039545b60405190815260200161030b565b3480156104bf57600080fd5b506104a56104ce3660046159d1565b610b9f565b3480156104df57600080fd5b506103fb610c11565b3480156104f457600080fd5b50610329610503366004615a98565b610c71565b34801561051457600080fd5b506103fb6111bc565b34801561052957600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b34801561055d57600080fd5b5061032961056c36600461598c565b60009182526036602052604090912080546001600160401b03909216600160401b026fffffffffffffffff000000000000000019909216919091179055565b3480156105b757600080fd5b506105cb6105c6366004615b09565b61121c565b60405161030b9190615b82565b3480156105e457600080fd5b506038546103ce906001600160401b031681565b34801561060457600080fd5b5061060d611281565b60405161030b9190615b90565b34801561062657600080fd5b5061063a610635366004615c43565b611370565b60405161030b9190615c5c565b34801561065357600080fd5b50610329610662366004615c43565b603955565b34801561067357600080fd5b506105cb610682366004615c43565b600090815260366020526040902054600160c01b900460ff1690565b3480156106aa57600080fd5b506102f77f000000000000000000000000000000000000000000000000000000000000000081565b3480156106de57600080fd5b5061034b6106ed366004615cc4565b61141d565b3480156106fe57600080fd5b506104a561070d366004615d1e565b611446565b34801561071e57600080fd5b50610727611463565b60405161030b9190615e12565b34801561074057600080fd5b506033546103ce90600160a01b90046001600160401b031681565b34801561076757600080fd5b5061060d611533565b61032961077e366004615e74565b611619565b34801561078f57600080fd5b5061032961079e366004615ee7565b6117c6565b3480156107af57600080fd5b5061063a6107be366004615b09565b611b5c565b3480156107cf57600080fd5b50610727611c4f565b3480156107e457600080fd5b50610429611d1f565b3480156107f957600080fd5b50610329611e4c565b34801561080e57600080fd5b5061032961081d366004615f71565b611eb7565b34801561082e57600080fd5b5061032961083d366004615f9d565b6120f4565b34801561084e57600080fd5b5061034b61085d36600461605a565b6122cc565b34801561086e57600080fd5b5061032961087d366004616128565b603380546001600160401b03909216600160a01b0267ffffffffffffffff60a01b19909216919091179055565b3480156108b657600080fd5b506103296108c5366004616145565b6122f9565b3480156108d657600080fd5b506103296108e53660046161ff565b612333565b3480156108f657600080fd5b506103fb612506565b34801561090b57600080fd5b5061032961091a3660046162d0565b612566565b34801561092b57600080fd5b5061032961093a366004615f71565b612931565b34801561094b57600080fd5b506103ce7f000000000000000000000000000000000000000000000000000000000000000081565b34801561097f57600080fd5b506066546104299060ff1681565b34801561099957600080fd5b506104a560375481565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a0b573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a2f91906163cb565b15610a555760405162461bcd60e51b8152600401610a4c906163ed565b60405180910390fd5b6033546001600160a01b03163314610a7f5760405162461bcd60e51b8152600401610a4c9061644a565b603454600160401b900460ff1615610aa95760405162461bcd60e51b8152600401610a4c90616492565b6034805460ff60401b1916600160401b179055603354610ad1906001600160a01b0316612b14565b6033546040516001600160a01b03909116907fca8dfc8c5e0a67a74501c072a3325f685259bebbae7cfd230ab85198a78b70cd90600090a250565b6040805180820190915260008082526020820152610b308989898989898989612b48565b9998505050505050505050565b60606073805480602002602001604051908101604052809291908181526020018280548015610b9557602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610b77575b5050505050905090565b60008084846000818110610bb557610bb56164e1565b60209081029290920135600081815260369093526040909220805467ffffffffffffffff60801b1916600160801b6001600160401b03881602179055509050610c038a8a8a8a8a8a8a6130c0565b9a9950505050505050505050565b60606075805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b6033546001600160a01b03163314610c9b5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610d03573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d2791906163cb565b15610d445760405162461bcd60e51b8152600401610a4c906163ed565b603454600160401b900460ff16610dbc5760405162461bcd60e51b815260206004820152603660248201527f456967656e506f642e686173456e61626c656452657374616b696e673a2072656044820152751cdd185ada5b99c81a5cc81b9bdd08195b98589b195960521b6064820152608401610a4c565b8584148015610dca57508382145b610e5a5760405162461bcd60e51b815260206004820152605560248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2076616c696461746f72496e646963657320616e642070726f6f666064820152740e640daeae6e840c4ca40e6c2daca40d8cadccee8d605b1b608482015260a401610a4c565b603354600160a01b90046001600160401b03161580610eaf5750603354610e9990610e9490600160a01b90046001600160401b031661359e565b613688565b6001600160401b0316896001600160401b031610155b610f3b5760405162461bcd60e51b815260206004820152605160248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2070726f6f66206d75737420626520696e207468652065706f63686064820152701030b33a32b91030b1ba34bb30ba34b7b760791b608482015260a401610a4c565b42610f51613f486001600160401b038c166157ab565b1015610fda5760405162461bcd60e51b815260206004820152604c60248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a207370656369666965642074696d657374616d7020697320746f6f60648201526b0819985c881a5b881c185cdd60a21b608482015260a401610a4c565b60405163d1c64cc960e01b81526001600160401b038a166004820152611083907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa15801561104b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061106f91906164f7565b893561107e60208c018c616510565b6136d5565b6000805b87811015611127576111098b8b358b8b858181106110a7576110a76164e1565b90506020020160208101906110bc9190616556565b8a8a868181106110ce576110ce6164e1565b90506020028101906110e09190616510565b8a8a888181106110f2576110f26164e1565b90506020028101906111049190616571565b613863565b61111390836157ab565b91508061111f816165ba565b915050611087565b5060335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c40906044015b600060405180830381600087803b15801561119857600080fd5b505af11580156111ac573d6000803e3d6000fd5b5050505050505050505050505050565b60606074805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b60008061125e84848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613d1d92505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b60606078805480602002602001604051908101604052809291908181526020016000905b828210156113675760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561134f57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116113115790505b505050505081525050815260200190600101906112a5565b50505050905090565b6113986040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561140357611403615b4a565b600281111561141457611414615b4a565b90525092915050565b604080518082019091526000808252602082015261143d85858585613e17565b95945050505050565b600061145788888888888888613863565b98975050505050505050565b60606077805480602002602001604051908101604052809291908181526020016000905b828210156113675783829060005260206000200180546114a6906165d5565b80601f01602080910402602001604051908101604052809291908181526020018280546114d2906165d5565b801561151f5780601f106114f45761010080835404028352916020019161151f565b820191906000526020600020905b81548152906001019060200180831161150257829003601f168201915b505050505081526020019060010190611487565b60606079805480602002602001604051908101604052809291908181526020016000905b828210156113675760008481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561160157602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116115c35790505b50505050508152505081526020019060010190611557565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146116615760405162461bcd60e51b8152600401610a4c9061660a565b346801bc16d674ec800000146116ed5760405162461bcd60e51b8152602060048201526044602482018190527f456967656e506f642e7374616b653a206d75737420696e697469616c6c792073908201527f74616b6520666f7220616e792076616c696461746f72207769746820333220656064820152633a3432b960e11b608482015260a401610a4c565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611730613ef5565b8888886040518863ffffffff1660e01b815260040161175496959493929190616684565b6000604051808303818588803b15801561176d57600080fd5b505af1158015611781573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e2385856040516117b79291906166d3565b60405180910390a15050505050565b604051635ac86ab760e01b8152600360048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561182e573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061185291906163cb565b1561186f5760405162461bcd60e51b8152600401610a4c906163ed565b868414801561187d57508382145b6119065760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e76657269667942616c616e6365557064617465733a207660448201527f616c696461746f72496e646963657320616e642070726f6f6673206d7573742060648201526d0c4ca40e6c2daca40d8cadccee8d60931b608482015260a401610a4c565b4261191c613f486001600160401b038c166157ab565b101561199e5760405162461bcd60e51b815260206004820152604560248201527f456967656e506f642e76657269667942616c616e6365557064617465733a207360448201527f70656369666965642074696d657374616d7020697320746f6f2066617220696e606482015264081c185cdd60da1b608482015260a401610a4c565b60405163d1c64cc960e01b81526001600160401b038a166004820152611a42907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa158015611a0f573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a3391906164f7565b873561107e60208a018a616510565b6000805b88811015611ae657611ac88b8b8b84818110611a6457611a646164e1565b9050602002016020810190611a799190616556565b8a358a8a86818110611a8d57611a8d6164e1565b9050602002810190611a9f9190616510565b8a8a88818110611ab157611ab16164e1565b9050602002810190611ac39190616571565b6130c0565b611ad290836166e7565b915080611ade816165ba565b915050611a46565b506033546001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169163c2c51c409116611b2b633b9aca0085616728565b6040516001600160e01b031960e085901b1681526001600160a01b039092166004830152602482015260440161117e565b611b846040805160808101825260008082526020820181905291810182905290606082015290565b60366000611bc785858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250613d1d92505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff166002811115611c3457611c34615b4a565b6002811115611c4557611c45615b4a565b9052509392505050565b60606076805480602002602001604051908101604052809291908181526020016000905b82821015611367578382906000526020600020018054611c92906165d5565b80601f0160208091040260200160405190810160405280929190818152602001828054611cbe906165d5565b8015611d0b5780601f10611ce057610100808354040283529160200191611d0b565b820191906000526020600020905b815481529060010190602001808311611cee57829003601f168201915b505050505081526020019060010190611c73565b606654600090610100900460ff1615611d415750606654610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611e475760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611dcf917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc4916080016167ad565b60408051601f1981840301815290829052611de9916167de565b6000604051808303816000865af19150503d8060008114611e26576040519150601f19603f3d011682016040523d82523d6000602084013e611e2b565b606091505b5091505080806020019051810190611e4391906163cb565b9150505b919050565b6033546001600160a01b03163314611e765760405162461bcd60e51b8152600401610a4c9061644a565b603454600160401b900460ff1615611ea05760405162461bcd60e51b8152600401610a4c90616492565b603354611eb5906001600160a01b0316612b14565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611eff5760405162461bcd60e51b8152600401610a4c9061660a565b611f0d633b9aca0082616810565b15611f975760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74576569206d75737420626520612077686f60648201526d1b194811ddd95a48185b5bdd5b9d60921b608482015260a401610a4c565b6000611fa7633b9aca0083616824565b6034549091506001600160401b0390811690821611156120605760405162461bcd60e51b815260206004820152606260248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74477765692065786365656473207769746860648201527f6472617761626c6552657374616b6564457865637574696f6e4c617965724777608482015261656960f01b60a482015260c401610a4c565b6034805482919060009061207e9084906001600160401b0316616838565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e836040516120dd91815260200190565b60405180910390a26120ef8383613f3a565b505050565b600054610100900460ff16158080156121145750600054600160ff909116105b8061212e5750303b15801561212e575060005460ff166001145b6121915760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610a4c565b6000805460ff1916600117905580156121b4576000805461ff0019166101001790555b6001600160a01b0382166122275760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e696e697469616c697a653a20706f644f776e65722063616044820152736e6e6f74206265207a65726f206164647265737360601b6064820152608401610a4c565b603380546001600160a01b0384166001600160a01b031990911681179091556034805460ff60401b1916600160401b1790556040517fca8dfc8c5e0a67a74501c072a3325f685259bebbae7cfd230ab85198a78b70cd90600090a280156122c8576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050565b60408051808201909152600080825260208201526122ee878787878787613f44565b979650505050505050565b6000828152603660205260409020805482919060ff60c01b1916600160c01b83600281111561232a5761232a615b4a565b02179055505050565b6033546001600160a01b0316331461235d5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156123c5573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906123e991906163cb565b156124065760405162461bcd60e51b8152600401610a4c906163ed565b82518451146124915760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e7265636f766572546f6b656e733a20746f6b656e4c697360448201527f7420616e6420616d6f756e7473546f5769746864726177206d7573742062652060648201526a0e6c2daca40d8cadccee8d60ab1b608482015260a401610a4c565b60005b84518110156124ff576124ed838583815181106124b3576124b36164e1565b60200260200101518784815181106124cd576124cd6164e1565b60200260200101516001600160a01b03166141829092919063ffffffff16565b806124f7816165ba565b915050612494565b5050505050565b60606072805480602002602001604051908101604052809291908181526020018280548015610b95576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610b77575050505050905090565b604051635ac86ab760e01b81526004808201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156125cd573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906125f191906163cb565b1561260e5760405162461bcd60e51b8152600401610a4c906163ed565b838614801561261c57508588145b801561262757508782145b61269b576040805162461bcd60e51b81526020600482015260248101919091527f456967656e506f642e766572696679416e6450726f636573735769746864726160448201527f77616c733a20696e70757473206d7573742062652073616d65206c656e6774686064820152608401610a4c565b60405163d1c64cc960e01b81526001600160401b038c16600482015261273f907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169063d1c64cc990602401602060405180830381865afa15801561270c573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061273091906164f7565b8b3561107e60208e018e616510565b604080518082019091526000808252602082015260005b8381101561283f5760006127fa8d358d8d85818110612777576127776164e1565b90506020028101906127899190616860565b8c8c8681811061279b5761279b6164e1565b90506020028101906127ad9190616510565b8c8c888181106127bf576127bf6164e1565b90506020028101906127d19190616571565b8c8c8a8181106127e3576127e36164e1565b90506020028101906127f59190616571565b612b48565b8051845191925090849061280f9083906157ab565b90525060208082015190840180516128289083906166e7565b905250819050612837816165ba565b915050612756565b5080511561286e57603354815161286e916001600160a01b03169061286990633b9aca0090616877565b6141d4565b6020810151156129235760335460208201516001600160a01b037f000000000000000000000000000000000000000000000000000000000000000081169263c2c51c40929116906128c490633b9aca0090616728565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561290a57600080fd5b505af115801561291e573d6000803e3d6000fd5b505050505b505050505050505050505050565b6033546001600160a01b0316331461295b5760405162461bcd60e51b8152600401610a4c9061644a565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156129c3573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906129e791906163cb565b15612a045760405162461bcd60e51b8152600401610a4c906163ed565b603754821115612ab55760405162461bcd60e51b815260206004820152606a60248201527f456967656e506f642e77697468647261776e6f6e426561636f6e436861696e4560448201527f544842616c616e63655765693a20616d6f756e74546f5769746864726177206960648201527f732067726561746572207468616e206e6f6e426561636f6e436861696e45544860848201526942616c616e636557656960b01b60a482015260c401610a4c565b8160376000828254612ac79190616896565b90915550506040518281526001600160a01b038416907f30420aacd028abb3c1fd03aba253ae725d6ddd52d16c9ac4cb5742cd43f530969060200160405180910390a26120ef83836141d4565b6033805467ffffffffffffffff60a01b19164263ffffffff16600160a01b021790556000603755612b4581476141d4565b50565b6040805180820190915260008082526020820152612b6d612b688961691c565b614262565b6033546001600160401b03600160a01b90910481169082161015612c2f5760405162461bcd60e51b815260206004820152606760248201527f456967656e506f642e70726f6f664973466f7256616c696454696d657374616d60448201527f703a20626561636f6e20636861696e2070726f6f66206d75737420626520617460648201527f206f72206166746572206d6f7374526563656e745769746864726177616c546960848201526606d657374616d760cc1b60a482015260c401610a4c565b6000612c3d612b688b61691c565b90506000612c7d88888080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b905060008082815260366020526040902054600160c01b900460ff166002811115612caa57612caa615b4a565b1415612d615760405162461bcd60e51b815260206004820152607460248201527f456967656e506f642e5f766572696679416e6450726f6365737357697468647260448201527f6177616c3a2056616c696461746f72206e657665722070726f76656e20746f2060648201527f68617665207769746864726177616c2063726564656e7469616c7320706f696e6084820152731d1959081d1bc81d1a1a5cc818dbdb9d1c9858dd60621b60a482015260c401610a4c565b60008181526035602090815260408083206001600160401b038616845290915290205460ff1615612e205760405162461bcd60e51b815260206004820152605b60248201527f456967656e506f642e5f766572696679416e6450726f6365737357697468647260448201527f6177616c3a207769746864726177616c2068617320616c72656164792062656560648201527f6e2070726f76656e20666f7220746869732074696d657374616d700000000000608482015260a401610a4c565b6001603560008381526020019081526020016000206000846001600160401b03166001600160401b0316815260200190815260200160002060006101000a81548160ff021916908315150217905550612efd8c87878e7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166344e71c806040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ed4573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612ef89190616a58565b614296565b6000612f3b878780806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614cb792505050565b9050612f4b8d8a8a8e8e86614cdc565b6000612f89888880806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f3392505050565b9050612fc78a8a80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f4b92505050565b6001600160401b0316612fe1612fdc8f61691c565b614f63565b6001600160401b03161061309957603354600084815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b81049093169381019390935261308e93869388938a936001600160a01b03909316928892916060830190600160c01b900460ff16600281111561307557613075615b4a565b600281111561308657613086615b4a565b905250613f44565b9550505050506130b3565b60335461308e90839086906001600160a01b031684613e17565b5098975050505050505050565b6000806130ff848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f7592505050565b9050600061313f85858080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156131ae576131ae615b4a565b60028111156131bf576131bf615b4a565b8152505090508a6001600160401b031681604001516001600160401b0316106132765760405162461bcd60e51b815260206004820152605c60248201527f456967656e506f642e76657269667942616c616e63655570646174653a20566160448201527f6c696461746f72732062616c616e63652068617320616c72656164792062656560648201527f6e207570646174656420666f7220746869732074696d657374616d7000000000608482015260a401610a4c565b60018160600151600281111561328e5761328e615b4a565b146132f65760405162461bcd60e51b815260206004820152603260248201527f456967656e506f642e76657269667942616c616e63655570646174653a2056616044820152716c696461746f72206e6f742061637469766560701b6064820152608401610a4c565b6132ff8b61359e565b6001600160401b0316613344878780806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f4b92505050565b6001600160401b0316116133e7576000836001600160401b0316116133e75760405162461bcd60e51b815260206004820152604d60248201527f456967656e506f642e76657269667942616c616e63655570646174653a20766160448201527f6c696461746f7220697320776974686472617761626c6520627574206861732060648201526c3737ba103bb4ba34323930bbb760991b608482015260a401610a4c565b6133f58987878b8b8f614cdc565b602081015160006001600160401b037f00000000000000000000000000000000000000000000000000000000000000008116908616111561345757507f000000000000000000000000000000000000000000000000000000000000000061345a565b50835b6001600160401b0380821660208086019182528f831660408088019182526000898152603690935290912086518154935192518516600160801b0267ffffffffffffffff60801b19938616600160401b026001600160801b031990951691909516179290921790811683178255606086015186939091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b83600281111561350257613502615b4a565b0217905550905050816001600160401b0316816001600160401b03161461358e577f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df8c8e836040516135799392919064ffffffffff9390931683526001600160401b03918216602084015216604082015260600190565b60405180910390a161358b8183614f8d565b95505b5050505050979650505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0316826001600160401b031610156136485760405162461bcd60e51b815260206004820152603760248201527f456967656e506f642e5f74696d657374616d70546f45706f63683a2074696d6560448201527f7374616d70206973206265666f72652067656e657369730000000000000000006064820152608401610a4c565b613654600c6020616a75565b61367e7f000000000000000000000000000000000000000000000000000000000000000084616838565b61127b9190616aa4565b6000613696600c6020616a75565b6136a1836001616aca565b6136ab9190616a75565b61127b907f0000000000000000000000000000000000000000000000000000000000000000616aca565b6136e160036020616877565b81146137715760405162461bcd60e51b815260206004820152605360248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f74416761696e73744c6174657374426c6f636b526f6f743a2050726f6f6620686064820152720c2e640d2dcc6dee4e4cac6e840d8cadccee8d606b1b608482015260a401610a4c565b6137b682828080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525088925087915060039050614fac565b61385d5760405162461bcd60e51b815260206004820152606660248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f74416761696e73744c6174657374426c6f636b526f6f743a20496e76616c696460648201527f206c617465737420626c6f636b2068656164657220726f6f74206d65726b6c6560848201526510383937b7b360d11b60a482015260c401610a4c565b50505050565b6000806138a284848080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061427292505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561391157613911615b4a565b600281111561392257613922615b4a565b905250905060008160600151600281111561393f5761393f615b4a565b146139e85760405162461bcd60e51b815260206004820152606760248201527f456967656e506f642e766572696679436f72726563745769746864726177616c60448201527f43726564656e7469616c733a2056616c696461746f72206d757374206265206960648201527f6e61637469766520746f2070726f7665207769746864726177616c2063726564608482015266656e7469616c7360c81b60a482015260c401610a4c565b6139f0613ef5565b6139f990616af5565b613a35868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614fc492505050565b14613abc5760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e766572696679436f72726563745769746864726177616c60448201527f43726564656e7469616c733a2050726f6f66206973206e6f7420666f7220746860648201526a1a5cc8115a59d95b941bd960aa1b608482015260a401610a4c565b6000613afa868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250614f7592505050565b9050613b0a8a87878b8b8e614cdc565b60398054906000613b1a836165ba565b90915550506001606083015264ffffffffff891682526001600160401b038b811660408401527f000000000000000000000000000000000000000000000000000000000000000081169082161115613ba0576001600160401b037f0000000000000000000000000000000000000000000000000000000000000000166020830152613bb0565b6001600160401b03811660208301525b6000838152603660209081526040918290208451815492860151938601516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060850151859391929091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b836002811115613c4e57613c4e615b4a565b02179055505060405164ffffffffff8b1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a17f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df898c8460200151604051613ce99392919064ffffffffff9390931683526001600160401b03918216602084015216604082015260600190565b60405180910390a1633b9aca0082602001516001600160401b0316613d0e9190616877565b9b9a5050505050505050505050565b60008151603014613da65760405162461bcd60e51b815260206004820152604760248201527f456967656e506f642e5f63616c63756c61746556616c696461746f725075626b60448201527f657948617368206d75737420626520612034382d6279746520424c53207075626064820152666c6963206b657960c81b608482015260a401610a4c565b604051600290613dbd908490600090602001616b19565b60408051601f1981840301815290829052613dd7916167de565b602060405180830381855afa158015613df4573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061127b91906164f7565b60408051808201909152600080825260208201526040805164ffffffffff871681526001600160401b0380871660208301528416918101919091526001600160a01b038416907f8a7335714231dbd551aaba6314f4a97a14c201e53a3e25e1140325cdf67d7a4e9060600160405180910390a260388054839190600090613ea89084906001600160401b0316616aca565b92506101000a8154816001600160401b0302191690836001600160401b031602179055506040518060400160405280836001600160401b0316815260200160008152509050949350505050565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b6122c88282614fd9565b604080518082019091526000808252602082015260007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0316846001600160401b03161115613fbb57507f0000000000000000000000000000000000000000000000000000000000000000613fbe565b50825b6040805180820190915260008082526020820152613fdc8286616838565b6001600160401b039081168252603480548492600091613ffe91859116616aca565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550614030828560200151614f8d565b602082015260028460600151600281111561404d5761404d615b4a565b1461406f576039805490600061406283616b48565b9091555050600260608501525b600060208086018281528a83526036909152604091829020865181549251938801516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516929091169190911792909217928316821781556060870151879391929091839160ff60c01b191668ffffffffffffffffff60801b1990911617600160c01b83600281111561411457614114615b4a565b0217905550506040805164ffffffffff8c1681526001600160401b038a8116602083015288168183015290516001600160a01b03891692507fb76a93bb649ece524688f1a01d184e0bbebcda58eae80c28a898bec3fb5a09639181900360600190a298975050505050505050565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b1790526120ef9084906150f2565b603354604051633036cd5360e21b81526001600160a01b03918216600482015283821660248201527f00000000000000000000000000000000000000000000000000000000000000009091169063c0db354c9083906044016000604051808303818588803b15801561424557600080fd5b505af1158015614259573d6000803e3d6000fd5b50505050505050565b600061127b8261014001516151c4565b600081600081518110614287576142876164e1565b60200260200101519050919050565b6142a1600280616c43565b83146143155760405162461bcd60e51b81526020600482015260496024820152600080516020616ca283398151915260448201527f616c3a207769746864726177616c4669656c64732068617320696e636f7272656064820152680c6e840d8cadccee8d60bb1b608482015260a401610a4c565b614321600d6002616c43565b61433160c0840160a08501616128565b6001600160401b03161061439b5760405162461bcd60e51b815260206004820152603f6024820152600080516020616ca283398151915260448201527f616c3a20626c6f636b526f6f74496e64657820697320746f6f206c61726765006064820152608401610a4c565b6143a760046002616c43565b6143b8610100840160e08501616128565b6001600160401b031610614424576040805162461bcd60e51b8152602060048201526024810191909152600080516020616ca283398151915260448201527f616c3a207769746864726177616c496e64657820697320746f6f206c617267656064820152608401610a4c565b61443060186002616c43565b61444060e0840160c08501616128565b6001600160401b0316106144ba5760405162461bcd60e51b81526020600482015260476024820152600080516020616ca283398151915260448201527f616c3a20686973746f726963616c53756d6d617279496e64657820697320746f6064820152666f206c6172676560c81b608482015260a401610a4c565b60006001600160401b0382166144d2612b688561691c565b6001600160401b0316106144e75760056144ea565b60045b90506144f76004826157ab565b6145029060016157ab565b61450d906020616877565b6145178480616510565b90501461458b5760405162461bcd60e51b81526020600482015260486024820152600080516020616ca283398151915260448201527f616c3a207769746864726177616c50726f6f662068617320696e636f727265636064820152670e840d8cadccee8d60c31b608482015260a401610a4c565b614597600460036157ab565b6145a2906020616877565b6145af6040850185616510565b9050146146295760405162461bcd60e51b815260206004820152604e6024820152600080516020616ca283398151915260448201527f616c3a20657865637574696f6e5061796c6f616450726f6f662068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a401610a4c565b61463560036020616877565b6146426020850185616510565b9050146146b05760405162461bcd60e51b81526020600482015260426024820152600080516020616ca283398151915260448201527f616c3a20736c6f7450726f6f662068617320696e636f7272656374206c656e676064820152610e8d60f31b608482015260a401610a4c565b6146bb816020616877565b6146c86060850185616510565b90501461473b5760405162461bcd60e51b81526020600482015260476024820152600080516020616ca283398151915260448201527f616c3a2074696d657374616d7050726f6f662068617320696e636f7272656374606482015266040d8cadccee8d60cb1b608482015260a401610a4c565b600d614749601860016157ab565b6147549060056157ab565b61475f9060016157ab565b61476991906157ab565b614774906020616877565b6147816080850185616510565b90501461480a5760405162461bcd60e51b81526020600482015260586024820152600080516020616ca283398151915260448201527f616c3a20686973746f726963616c53756d6d617279426c6f636b526f6f74507260648201527f6f6f662068617320696e636f7272656374206c656e6774680000000000000000608482015260a401610a4c565b600061481c60c0850160a08601616128565b6001600160401b03166000614833600d60016157ab565b61484360e0880160c08901616128565b6001600160401b0316901b600d61485c601860016157ab565b6148679060016157ab565b61487191906157ab565b601b901b17171790506148cc61488a6080860186616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508b9250505061010087013584614fac565b61493f5760405162461bcd60e51b815260206004820152604a6024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420686973746f726963616c73756d6d617279206d656064820152693935b63290383937b7b360b11b608482015260a401610a4c565b61499661494f6020860186616510565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201829052506101008a013593506101208a013592509050614fac565b6149f65760405162461bcd60e51b815260206004820152603d6024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420736c6f74206d65726b6c652070726f6f660000006064820152608401610a4c565b6049614a4e614a086040870187616510565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525050505061010087013561016088013584614fac565b614ac05760405162461bcd60e51b81526020600482015260496024820152600080516020616ca283398151915260448201527f616c3a20496e76616c696420657865637574696f6e5061796c6f6164206d657260648201526835b63290383937b7b360b91b608482015260a401610a4c565b50614b18614ad16060860186616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050506101608601356101408701356009614fac565b614b835760405162461bcd60e51b81526020600482015260426024820152600080516020616ca283398151915260448201527f616c3a20496e76616c69642074696d657374616d70206d65726b6c652070726f60648201526137b360f11b608482015260a401610a4c565b6000614b96610100860160e08701616128565b6001600160401b0316614bab600460016157ab565b600e901b1790506000614bf088888080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061522b92505050565b9050614c40614bff8780616510565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152505050506101608801358385614fac565b614cac5760405162461bcd60e51b81526020600482015260436024820152600080516020616ca283398151915260448201527f616c3a20496e76616c6964207769746864726177616c206d65726b6c6520707260648201526237b7b360e91b608482015260a401610a4c565b505050505050505050565b600061127b82600181518110614ccf57614ccf6164e1565b60200260200101516151c4565b614ce860036002616c43565b8414614d735760405162461bcd60e51b815260206004820152604e60248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a2056616c696461746f72206669656c64732068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a401610a4c565b6005614d81602860016157ab565b614d8b91906157ab565b614d96906020616877565b8214614e165760405162461bcd60e51b815260206004820152604360248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a2050726f6f662068617320696e636f7272656374206c656e6064820152620cee8d60eb1b608482015260a401610a4c565b600064ffffffffff8216614e2c602860016157ab565b600b901b1790506000614e7187878080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061522b92505050565b9050614eb785858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250859150869050614fac565b614f295760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e50726f6f66732e76657269667956616c696461746f60448201527f724669656c64733a20496e76616c6964206d65726b6c652070726f6f660000006064820152608401610a4c565b5050505050505050565b600061127b82600381518110614ccf57614ccf6164e1565b600061127b82600781518110614ccf57614ccf6164e1565b6000602061367e8361012001516151c4565b600061127b82600281518110614ccf57614ccf6164e1565b6000614fa56001600160401b03808416908516616c4f565b9392505050565b600083614fba8685856154d8565b1495945050505050565b600081600181518110614287576142876164e1565b804710156150295760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610a4c565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114615076576040519150601f19603f3d011682016040523d82523d6000602084013e61507b565b606091505b50509050806120ef5760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610a4c565b6000615147826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b03166156249092919063ffffffff16565b8051909150156120ef578080602001905181019061516591906163cb565b6120ef5760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608401610a4c565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6000806002835161523c9190616824565b90506000816001600160401b0381111561525857615258615fba565b604051908082528060200260200182016040528015615281578160200160208202803683370190505b50905060005b828110156153885760028561529c8383616877565b815181106152ac576152ac6164e1565b6020026020010151868360026152c29190616877565b6152cd9060016157ab565b815181106152dd576152dd6164e1565b60200260200101516040516020016152ff929190918252602082015260400190565b60408051601f1981840301815290829052615319916167de565b602060405180830381855afa158015615336573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061535991906164f7565b82828151811061536b5761536b6164e1565b602090810291909101015280615380816165ba565b915050615287565b50615394600283616824565b91505b81156154b45760005b828110156154a1576002826153b58383616877565b815181106153c5576153c56164e1565b6020026020010151838360026153db9190616877565b6153e69060016157ab565b815181106153f6576153f66164e1565b6020026020010151604051602001615418929190918252602082015260400190565b60408051601f1981840301815290829052615432916167de565b602060405180830381855afa15801561544f573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061547291906164f7565b828281518110615484576154846164e1565b602090810291909101015280615499816165ba565b9150506153a0565b506154ad600283616824565b9150615397565b806000815181106154c7576154c76164e1565b602002602001015192505050919050565b600083516000141580156154f75750602084516154f59190616810565b155b6155865760405162461bcd60e51b815260206004820152605460248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f665368613260448201527f35363a2070726f6f66206c656e6774682073686f756c642062652061206e6f6e60648201527316bd32b9379036bab63a34b836329037b310199960611b608482015260a401610a4c565b604080516020808201909252848152905b8551811161561a576155aa600285616810565b6155dd578151600052808601516020526020826040600060026107d05a03fa6155d257600080fd5b600284049350615608565b8086015160005281516020526020826040600060026107d05a03fa61560157600080fd5b6002840493505b6156136020826157ab565b9050615597565b5051949350505050565b6060615633848460008561563b565b949350505050565b60608247101561569c5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b6064820152608401610a4c565b6001600160a01b0385163b6156f35760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606401610a4c565b600080866001600160a01b0316858760405161570f91906167de565b60006040518083038185875af1925050503d806000811461574c576040519150601f19603f3d011682016040523d82523d6000602084013e615751565b606091505b50915091506122ee8282866060831561576b575081614fa5565b82511561577b5782518084602001fd5b8160405162461bcd60e51b8152600401610a4c9190616c8e565b634e487b7160e01b600052601160045260246000fd5b600082198211156157be576157be615795565b500190565b60008083601f8401126157d557600080fd5b5081356001600160401b038111156157ec57600080fd5b60208301915083602082850101111561580457600080fd5b9250929050565b60008083601f84011261581d57600080fd5b5081356001600160401b0381111561583457600080fd5b6020830191508360208260051b850101111561580457600080fd5b60008060008060008060008060a0898b03121561586b57600080fd5b8835975060208901356001600160401b038082111561588957600080fd5b908a0190610180828d03121561589e57600080fd5b90975060408a013590808211156158b457600080fd5b6158c08c838d016157c3565b909850965060608b01359150808211156158d957600080fd5b6158e58c838d0161580b565b909650945060808b01359150808211156158fe57600080fd5b5061590b8b828c0161580b565b999c989b5096995094979396929594505050565b6020808252825182820181905260009190848201906040850190845b818110156159605783516001600160a01b03168352928401929184019160010161593b565b50909695505050505050565b6001600160401b0381168114612b4557600080fd5b8035611e478161596c565b6000806040838503121561599f57600080fd5b8235915060208301356159b18161596c565b809150509250929050565b803564ffffffffff81168114611e4757600080fd5b60008060008060008060008060c0898b0312156159ed57600080fd5b88356159f88161596c565b9750615a0660208a016159bc565b96506040890135955060608901356001600160401b0380821115615a2957600080fd5b615a358c838d016157c3565b909750955060808b0135915080821115615a4e57600080fd5b50615a5b8b828c0161580b565b90945092505060a0890135615a6f8161596c565b809150509295985092959890939650565b600060408284031215615a9257600080fd5b50919050565b60008060008060008060008060a0898b031215615ab457600080fd5b8835615abf8161596c565b975060208901356001600160401b0380821115615adb57600080fd5b615ae78c838d01615a80565b985060408b0135915080821115615afd57600080fd5b6158c08c838d0161580b565b60008060208385031215615b1c57600080fd5b82356001600160401b03811115615b3257600080fd5b615b3e858286016157c3565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b60038110615b7e57634e487b7160e01b600052602160045260246000fd5b9052565b6020810161127b8284615b60565b60006020808301818452808551808352604092508286019150828160051b8701018488016000805b84811015615c3457898403603f19018652825180516001600160a01b03168552880151888501889052805188860181905290890190839060608701905b80831015615c1f5783516001600160e01b0319168252928b019260019290920191908b0190615bf5565b50978a01979550505091870191600101615bb8565b50919998505050505050505050565b600060208284031215615c5557600080fd5b5035919050565b60006080820190506001600160401b03808451168352806020850151166020840152806040850151166040840152506060830151615c9d6060840182615b60565b5092915050565b6001600160a01b0381168114612b4557600080fd5b8035611e4781615ca4565b60008060008060808587031215615cda57600080fd5b615ce3856159bc565b93506020850135615cf38161596c565b92506040850135615d0381615ca4565b91506060850135615d138161596c565b939692955090935050565b600080600080600080600060a0888a031215615d3957600080fd5b8735615d448161596c565b965060208801359550615d59604089016159bc565b945060608801356001600160401b0380821115615d7557600080fd5b615d818b838c016157c3565b909650945060808a0135915080821115615d9a57600080fd5b50615da78a828b0161580b565b989b979a50959850939692959293505050565b60005b83811015615dd5578181015183820152602001615dbd565b8381111561385d5750506000910152565b60008151808452615dfe816020860160208601615dba565b601f01601f19169290920160200192915050565b6000602080830181845280855180835260408601915060408160051b870101925083870160005b82811015615e6757603f19888603018452615e55858351615de6565b94509285019290850190600101615e39565b5092979650505050505050565b600080600080600060608688031215615e8c57600080fd5b85356001600160401b0380821115615ea357600080fd5b615eaf89838a016157c3565b90975095506020880135915080821115615ec857600080fd5b50615ed5888289016157c3565b96999598509660400135949350505050565b60008060008060008060008060a0898b031215615f0357600080fd5b8835615f0e8161596c565b975060208901356001600160401b0380821115615f2a57600080fd5b615f368c838d0161580b565b909950975060408b0135915080821115615f4f57600080fd5b615f5b8c838d01615a80565b965060608b01359150808211156158d957600080fd5b60008060408385031215615f8457600080fd5b8235615f8f81615ca4565b946020939093013593505050565b600060208284031215615faf57600080fd5b8135614fa581615ca4565b634e487b7160e01b600052604160045260246000fd5b604051608081016001600160401b0381118282101715615ff257615ff2615fba565b60405290565b60405161018081016001600160401b0381118282101715615ff257615ff2615fba565b604051601f8201601f191681016001600160401b038111828210171561604357616043615fba565b604052919050565b803560038110611e4757600080fd5b60008060008060008086880361012081121561607557600080fd5b61607e886159bc565b96506020880135955060408801356160958161596c565b945060608801356160a581615ca4565b935060808801356160b58161596c565b92506080609f19820112156160c957600080fd5b506160d2615fd0565b60a08801356160e08161596c565b815260c08801356160f08161596c565b602082015260e08801356161038161596c565b6040820152616115610100890161604b565b6060820152809150509295509295509295565b60006020828403121561613a57600080fd5b8135614fa58161596c565b6000806040838503121561615857600080fd5b823591506161686020840161604b565b90509250929050565b60006001600160401b0382111561618a5761618a615fba565b5060051b60200190565b600082601f8301126161a557600080fd5b813560206161ba6161b583616171565b61601b565b82815260059290921b840181019181810190868411156161d957600080fd5b8286015b848110156161f457803583529183019183016161dd565b509695505050505050565b60008060006060848603121561621457600080fd5b83356001600160401b038082111561622b57600080fd5b818601915086601f83011261623f57600080fd5b8135602061624f6161b583616171565b82815260059290921b8401810191818101908a84111561626e57600080fd5b948201945b8386101561629557853561628681615ca4565b82529482019490820190616273565b975050870135925050808211156162ab57600080fd5b506162b886828701616194565b9250506162c760408501615cb9565b90509250925092565b60008060008060008060008060008060c08b8d0312156162ef57600080fd5b6162f88b615981565b995060208b01356001600160401b038082111561631457600080fd5b6163208e838f01615a80565b9a5060408d013591508082111561633657600080fd5b6163428e838f0161580b565b909a50985060608d013591508082111561635b57600080fd5b6163678e838f0161580b565b909850965060808d013591508082111561638057600080fd5b61638c8e838f0161580b565b909650945060a08d01359150808211156163a557600080fd5b506163b28d828e0161580b565b915080935050809150509295989b9194979a5092959850565b6000602082840312156163dd57600080fd5b81518015158114614fa557600080fd5b6020808252603e908201527f456967656e506f642e6f6e6c795768656e4e6f745061757365643a20696e646560408201527f782069732070617573656420696e20456967656e506f644d616e616765720000606082015260800190565b60208082526028908201527f456967656e506f642e6f6e6c79456967656e506f644f776e65723a206e6f74206040820152673837b227bbb732b960c11b606082015260800190565b6020808252602f908201527f456967656e506f642e6861734e6576657252657374616b65643a20726573746160408201526e1ada5b99c81a5cc8195b98589b1959608a1b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561650957600080fd5b5051919050565b6000808335601e1984360301811261652757600080fd5b8301803591506001600160401b0382111561654157600080fd5b60200191503681900382131561580457600080fd5b60006020828403121561656857600080fd5b614fa5826159bc565b6000808335601e1984360301811261658857600080fd5b8301803591506001600160401b038211156165a257600080fd5b6020019150600581901b360382131561580457600080fd5b60006000198214156165ce576165ce615795565b5060010190565b600181811c908216806165e957607f821691505b60208210811415615a9257634e487b7160e01b600052602260045260246000fd5b60208082526031908201527f456967656e506f642e6f6e6c79456967656e506f644d616e616765723a206e6f6040820152703a1032b4b3b2b72837b226b0b730b3b2b960791b606082015260800190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b60808152600061669860808301888a61665b565b82810360208401526166aa8188615de6565b905082810360408401526166bf81868861665b565b915050826060830152979650505050505050565b60208152600061563360208301848661665b565b600080821280156001600160ff1b038490038513161561670957616709615795565b600160ff1b839003841281161561672257616722615795565b50500190565b60006001600160ff1b038184138284138082168684048611161561674e5761674e615795565b600160ff1b600087128281168783058912161561676d5761676d615795565b6000871292508782058712848416161561678957616789615795565b8785058712818416161561679f5761679f615795565b505050929093029392505050565b6001600160e01b03198316815281516000906167d0816004850160208701615dba565b919091016004019392505050565b600082516167f0818460208701615dba565b9190910192915050565b634e487b7160e01b600052601260045260246000fd5b60008261681f5761681f6167fa565b500690565b600082616833576168336167fa565b500490565b60006001600160401b038381169083168181101561685857616858615795565b039392505050565b6000823561017e198336030181126167f057600080fd5b600081600019048311821515161561689157616891615795565b500290565b6000828210156168a8576168a8615795565b500390565b600082601f8301126168be57600080fd5b81356001600160401b038111156168d7576168d7615fba565b6168ea601f8201601f191660200161601b565b8181528460208386010111156168ff57600080fd5b816020850160208301376000918101602001919091529392505050565b6000610180823603121561692f57600080fd5b616937615ff8565b82356001600160401b038082111561694e57600080fd5b61695a368387016168ad565b8352602085013591508082111561697057600080fd5b61697c368387016168ad565b6020840152604085013591508082111561699557600080fd5b6169a1368387016168ad565b604084015260608501359150808211156169ba57600080fd5b6169c6368387016168ad565b606084015260808501359150808211156169df57600080fd5b506169ec368286016168ad565b6080830152506169fe60a08401615981565b60a0820152616a0f60c08401615981565b60c0820152616a2060e08401615981565b60e082015261010083810135908201526101208084013590820152610140808401359082015261016092830135928101929092525090565b600060208284031215616a6a57600080fd5b8151614fa58161596c565b60006001600160401b0380831681851681830481118215151615616a9b57616a9b615795565b02949350505050565b60006001600160401b0380841680616abe57616abe6167fa565b92169190910492915050565b60006001600160401b03808316818516808303821115616aec57616aec615795565b01949350505050565b80516020808301519190811015615a925760001960209190910360031b1b16919050565b60008351616b2b818460208801615dba565b6001600160801b0319939093169190920190815260100192915050565b600081616b5757616b57615795565b506000190190565b600181815b80851115616b9a578160001904821115616b8057616b80615795565b80851615616b8d57918102915b93841c9390800290616b64565b509250929050565b600082616bb15750600161127b565b81616bbe5750600061127b565b8160018114616bd45760028114616bde57616bfa565b600191505061127b565b60ff841115616bef57616bef615795565b50506001821b61127b565b5060208310610133831016604e8410600b8410161715616c1d575081810a61127b565b616c278383616b5f565b8060001904821115616c3b57616c3b615795565b029392505050565b6000614fa58383616ba2565b60008083128015600160ff1b850184121615616c6d57616c6d615795565b6001600160ff1b0384018313811615616c8857616c88615795565b50500390565b602081526000614fa56020830184615de656fe426561636f6e436861696e50726f6f66732e7665726966795769746864726177a2646970667358221220a9c8016b8b8b8d76601725febabd237067aa3aa96c952e1e3ac20cc882f16ea864736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x81W`\x005`\xE0\x1C\x80c{\xB3\xDB\xF3\x11a\x01OW\x80c\xC4\x90tB\x11a\0\xC1W\x80c\xE2\x0C\x9Fq\x11a\0zW\x80c\xE2\x0C\x9Fq\x14a\x08\xEAW\x80c\xE2Q\xEFR\x14a\x08\xFFW\x80c\xE2\xC84E\x14a\t\x1FW\x80c\xF2\x88$a\x14a\t?W\x80c\xFAv&\xD4\x14a\tsW\x80c\xFE\x80\xB0\x87\x14a\t\x8DW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x08\x02W\x80c\xC4\xD6m\xE8\x14a\x08\"W\x80c\xCA\x04N\x8C\x14a\x08BW\x80c\xD1h\xEBQ\x14a\x08bW\x80c\xD7\x9E\xD7&\x14a\x08\xAAW\x80c\xDD\xA34l\x14a\x08\xCAW`\0\x80\xFD[\x80c\x9BNF4\x11a\x01\x13W\x80c\x9BNF4\x14a\x07pW\x80c\xA5\x06\0\xF4\x14a\x07\x83W\x80c\xB5\"S\x8A\x14a\x07\xA3W\x80c\xB5P\x8A\xA9\x14a\x07\xC3W\x80c\xBAAO\xA6\x14a\x07\xD8W\x80c\xBA\xA7\x14Z\x14a\x07\xEDW`\0\x80\xFD[\x80c{\xB3\xDB\xF3\x14a\x06\xD2W\x80c\x81mS\xF9\x14a\x06\xF2W\x80c\x85\"l\x81\x14a\x07\x12W\x80c\x87\xE0\xD2\x89\x14a\x074W\x80c\x91j\x17\xC6\x14a\x07[W`\0\x80\xFD[\x80c>^<#\x11a\x01\xF3W\x80c]?e\xB6\x11a\x01\xACW\x80c]?e\xB6\x14a\x05\xD8W\x80cf\xD9\xA9\xA0\x14a\x05\xF8W\x80co\xCD\x0ES\x14a\x06\x1AW\x80cs\xA9~\xE8\x14a\x06GW\x80ct9\x84\x1F\x14a\x06gW\x80ct\xCD\xD7\x98\x14a\x06\x9EW`\0\x80\xFD[\x80c>^<#\x14a\x04\xD3W\x80c?e\xCF\x19\x14a\x04\xE8W\x80c?r\x86\xF4\x14a\x05\x08W\x80cFe\xBC\xDA\x14a\x05\x1DW\x80cR)VJ\x14a\x05QW\x80cX\xEA\xEEy\x14a\x05\xABW`\0\x80\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02EW\x80c\x1E\xD7\x83\x1C\x14a\x03\xE6W\x80c1\x06\xABS\x14a\x04\x08W\x80c4t\xAA\x16\x14a\x049W\x80c4\xBE\xA2\n\x14a\x04YW\x80c7\xDE\xEAp\x14a\x04\x94W\x80c;LW\xC6\x14a\x04\xB3W`\0\x80\xFD[\x80c\x0B\x18\xFFf\x14a\x02\xD7W\x80c\x0C\xD4d\x9E\x14a\x03\x14W\x80c\x11\\\xD5\xE4\x14a\x03+W\x80c\x1APW\xBE\x14a\x03fW\x80c\x1D\x90]\\\x14a\x03\x9AW`\0\x80\xFD[6a\x02\xD2W4`7`\0\x82\x82Ta\x02\x98\x91\x90aW\xABV[\x90\x91UPP`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x02\xE3W`\0\x80\xFD[P`3Ta\x02\xF7\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x03 W`\0\x80\xFD[Pa\x03)a\t\xA3V[\0[4\x80\x15a\x037W`\0\x80\xFD[Pa\x03Ka\x03F6`\x04aXOV[a\x0B\x0CV[`@\x80Q\x82Q\x81R` \x92\x83\x01Q\x92\x81\x01\x92\x90\x92R\x01a\x03\x0BV[4\x80\x15a\x03rW`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\xA6W`\0\x80\xFD[Pa\x03\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x03\x0BV[4\x80\x15a\x03\xF2W`\0\x80\xFD[Pa\x03\xFBa\x0B=V[`@Qa\x03\x0B\x91\x90aY\x1FV[4\x80\x15a\x04\x14W`\0\x80\xFD[P`4Ta\x04)\x90`\x01`@\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x03\x0BV[4\x80\x15a\x04EW`\0\x80\xFD[P`4Ta\x03\xCE\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04eW`\0\x80\xFD[Pa\x04)a\x04t6`\x04aY\x8CV[`5` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x04\xA0W`\0\x80\xFD[P`9T[`@Q\x90\x81R` \x01a\x03\x0BV[4\x80\x15a\x04\xBFW`\0\x80\xFD[Pa\x04\xA5a\x04\xCE6`\x04aY\xD1V[a\x0B\x9FV[4\x80\x15a\x04\xDFW`\0\x80\xFD[Pa\x03\xFBa\x0C\x11V[4\x80\x15a\x04\xF4W`\0\x80\xFD[Pa\x03)a\x05\x036`\x04aZ\x98V[a\x0CqV[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x03\xFBa\x11\xBCV[4\x80\x15a\x05)W`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05]W`\0\x80\xFD[Pa\x03)a\x05l6`\x04aY\x8CV[`\0\x91\x82R`6` R`@\x90\x91 \x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x05\xB7W`\0\x80\xFD[Pa\x05\xCBa\x05\xC66`\x04a[\tV[a\x12\x1CV[`@Qa\x03\x0B\x91\x90a[\x82V[4\x80\x15a\x05\xE4W`\0\x80\xFD[P`8Ta\x03\xCE\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x04W`\0\x80\xFD[Pa\x06\ra\x12\x81V[`@Qa\x03\x0B\x91\x90a[\x90V[4\x80\x15a\x06&W`\0\x80\xFD[Pa\x06:a\x0656`\x04a\\CV[a\x13pV[`@Qa\x03\x0B\x91\x90a\\\\V[4\x80\x15a\x06SW`\0\x80\xFD[Pa\x03)a\x06b6`\x04a\\CV[`9UV[4\x80\x15a\x06sW`\0\x80\xFD[Pa\x05\xCBa\x06\x826`\x04a\\CV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x06\xAAW`\0\x80\xFD[Pa\x02\xF7\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x06\xDEW`\0\x80\xFD[Pa\x03Ka\x06\xED6`\x04a\\\xC4V[a\x14\x1DV[4\x80\x15a\x06\xFEW`\0\x80\xFD[Pa\x04\xA5a\x07\r6`\x04a]\x1EV[a\x14FV[4\x80\x15a\x07\x1EW`\0\x80\xFD[Pa\x07'a\x14cV[`@Qa\x03\x0B\x91\x90a^\x12V[4\x80\x15a\x07@W`\0\x80\xFD[P`3Ta\x03\xCE\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x07gW`\0\x80\xFD[Pa\x06\ra\x153V[a\x03)a\x07~6`\x04a^tV[a\x16\x19V[4\x80\x15a\x07\x8FW`\0\x80\xFD[Pa\x03)a\x07\x9E6`\x04a^\xE7V[a\x17\xC6V[4\x80\x15a\x07\xAFW`\0\x80\xFD[Pa\x06:a\x07\xBE6`\x04a[\tV[a\x1B\\V[4\x80\x15a\x07\xCFW`\0\x80\xFD[Pa\x07'a\x1COV[4\x80\x15a\x07\xE4W`\0\x80\xFD[Pa\x04)a\x1D\x1FV[4\x80\x15a\x07\xF9W`\0\x80\xFD[Pa\x03)a\x1ELV[4\x80\x15a\x08\x0EW`\0\x80\xFD[Pa\x03)a\x08\x1D6`\x04a_qV[a\x1E\xB7V[4\x80\x15a\x08.W`\0\x80\xFD[Pa\x03)a\x08=6`\x04a_\x9DV[a \xF4V[4\x80\x15a\x08NW`\0\x80\xFD[Pa\x03Ka\x08]6`\x04a`ZV[a\"\xCCV[4\x80\x15a\x08nW`\0\x80\xFD[Pa\x03)a\x08}6`\x04aa(V[`3\x80T`\x01`\x01`@\x1B\x03\x90\x92\x16`\x01`\xA0\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UV[4\x80\x15a\x08\xB6W`\0\x80\xFD[Pa\x03)a\x08\xC56`\x04aaEV[a\"\xF9V[4\x80\x15a\x08\xD6W`\0\x80\xFD[Pa\x03)a\x08\xE56`\x04aa\xFFV[a#3V[4\x80\x15a\x08\xF6W`\0\x80\xFD[Pa\x03\xFBa%\x06V[4\x80\x15a\t\x0BW`\0\x80\xFD[Pa\x03)a\t\x1A6`\x04ab\xD0V[a%fV[4\x80\x15a\t+W`\0\x80\xFD[Pa\x03)a\t:6`\x04a_qV[a)1V[4\x80\x15a\tKW`\0\x80\xFD[Pa\x03\xCE\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\t\x7FW`\0\x80\xFD[P`fTa\x04)\x90`\xFF\x16\x81V[4\x80\x15a\t\x99W`\0\x80\xFD[Pa\x04\xA5`7T\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n/\x91\x90ac\xCBV[\x15a\nUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`@Q\x80\x91\x03\x90\xFD[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`4T`\x01`@\x1B\x90\x04`\xFF\x16\x15a\n\xA9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ad\x92V[`4\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`3Ta\n\xD1\x90`\x01`\x01`\xA0\x1B\x03\x16a+\x14V[`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x7F\xCA\x8D\xFC\x8C^\ng\xA7E\x01\xC0r\xA32_hRY\xBE\xBB\xAE|\xFD#\n\xB8Q\x98\xA7\x8Bp\xCD\x90`\0\x90\xA2PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x0B0\x89\x89\x89\x89\x89\x89\x89\x89a+HV[\x99\x98PPPPPPPPPV[```s\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwW[PPPPP\x90P\x90V[`\0\x80\x84\x84`\0\x81\x81\x10a\x0B\xB5Wa\x0B\xB5ad\xE1V[` \x90\x81\x02\x92\x90\x92\x015`\0\x81\x81R`6\x90\x93R`@\x90\x92 \x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x16`\x01`\x80\x1B`\x01`\x01`@\x1B\x03\x88\x16\x02\x17\x90UP\x90Pa\x0C\x03\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa0\xC0V[\x9A\x99PPPPPPPPPPV[```u\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x0C\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\x03W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r'\x91\x90ac\xCBV[\x15a\rDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`4T`\x01`@\x1B\x90\x04`\xFF\x16a\r\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FEigenPod.hasEnabledRestaking: re`D\x82\x01Ru\x1C\xDD\x18Z\xDA[\x99\xC8\x1A\\\xC8\x1B\x9B\xDD\x08\x19[\x98X\x9B\x19Y`R\x1B`d\x82\x01R`\x84\x01a\nLV[\x85\x84\x14\x80\x15a\r\xCAWP\x83\x82\x14[a\x0EZW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: validatorIndices and proof`d\x82\x01Rt\x0Ed\r\xAE\xAEn\x84\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`[\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`3T`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15\x80a\x0E\xAFWP`3Ta\x0E\x99\x90a\x0E\x94\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a5\x9EV[a6\x88V[`\x01`\x01`@\x1B\x03\x16\x89`\x01`\x01`@\x1B\x03\x16\x10\x15[a\x0F;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: proof must be in the epoch`d\x82\x01Rp\x100\xB3:2\xB9\x100\xB1\xBA4\xBB0\xBA4\xB7\xB7`y\x1B`\x84\x82\x01R`\xA4\x01a\nLV[Ba\x0FQa?H`\x01`\x01`@\x1B\x03\x8C\x16aW\xABV[\x10\x15a\x0F\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: specified timestamp is too`d\x82\x01Rk\x08\x19\x98\\\x88\x1A[\x88\x1C\x18\\\xDD`\xA2\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8A\x16`\x04\x82\x01Ra\x10\x83\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10KW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x10o\x91\x90ad\xF7V[\x895a\x10~` \x8C\x01\x8Cae\x10V[a6\xD5V[`\0\x80[\x87\x81\x10\x15a\x11'Wa\x11\t\x8B\x8B5\x8B\x8B\x85\x81\x81\x10a\x10\xA7Wa\x10\xA7ad\xE1V[\x90P` \x02\x01` \x81\x01\x90a\x10\xBC\x91\x90aeVV[\x8A\x8A\x86\x81\x81\x10a\x10\xCEWa\x10\xCEad\xE1V[\x90P` \x02\x81\x01\x90a\x10\xE0\x91\x90ae\x10V[\x8A\x8A\x88\x81\x81\x10a\x10\xF2Wa\x10\xF2ad\xE1V[\x90P` \x02\x81\x01\x90a\x11\x04\x91\x90aeqV[a8cV[a\x11\x13\x90\x83aW\xABV[\x91P\x80a\x11\x1F\x81ae\xBAV[\x91PPa\x10\x87V[P`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x11\x98W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x11\xACW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[```t\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`\0\x80a\x12^\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa=\x1D\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[```x\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x13OW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x13\x11W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12\xA5V[PPPP\x90P\x90V[a\x13\x98`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\x03Wa\x14\x03a[JV[`\x02\x81\x11\x15a\x14\x14Wa\x14\x14a[JV[\x90RP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x14=\x85\x85\x85\x85a>\x17V[\x95\x94PPPPPV[`\0a\x14W\x88\x88\x88\x88\x88\x88\x88a8cV[\x98\x97PPPPPPPPV[```w\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x14\xA6\x90ae\xD5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\xD2\x90ae\xD5V[\x80\x15a\x15\x1FW\x80`\x1F\x10a\x14\xF4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x1FV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x02W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\x87V[```y\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x01W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x15\xC3W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15WV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x16aW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90af\nV[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x16\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FEigenPod.stake: must initially s\x90\x82\x01R\x7Ftake for any validator with 32 e`d\x82\x01Rc:42\xB9`\xE1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x170a>\xF5V[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x17T\x96\x95\x94\x93\x92\x91\x90af\x84V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x17mW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x17\x81W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x17\xB7\x92\x91\x90af\xD3V[`@Q\x80\x91\x03\x90\xA1PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x03`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18.W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18R\x91\x90ac\xCBV[\x15a\x18oW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x86\x84\x14\x80\x15a\x18}WP\x83\x82\x14[a\x19\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.verifyBalanceUpdates: v`D\x82\x01R\x7FalidatorIndices and proofs must `d\x82\x01Rm\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[Ba\x19\x1Ca?H`\x01`\x01`@\x1B\x03\x8C\x16aW\xABV[\x10\x15a\x19\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FEigenPod.verifyBalanceUpdates: s`D\x82\x01R\x7Fpecified timestamp is too far in`d\x82\x01Rd\x08\x1C\x18\\\xDD`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8A\x16`\x04\x82\x01Ra\x1AB\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A3\x91\x90ad\xF7V[\x875a\x10~` \x8A\x01\x8Aae\x10V[`\0\x80[\x88\x81\x10\x15a\x1A\xE6Wa\x1A\xC8\x8B\x8B\x8B\x84\x81\x81\x10a\x1AdWa\x1Adad\xE1V[\x90P` \x02\x01` \x81\x01\x90a\x1Ay\x91\x90aeVV[\x8A5\x8A\x8A\x86\x81\x81\x10a\x1A\x8DWa\x1A\x8Dad\xE1V[\x90P` \x02\x81\x01\x90a\x1A\x9F\x91\x90ae\x10V[\x8A\x8A\x88\x81\x81\x10a\x1A\xB1Wa\x1A\xB1ad\xE1V[\x90P` \x02\x81\x01\x90a\x1A\xC3\x91\x90aeqV[a0\xC0V[a\x1A\xD2\x90\x83af\xE7V[\x91P\x80a\x1A\xDE\x81ae\xBAV[\x91PPa\x1AFV[P`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x91c\xC2\xC5\x1C@\x91\x16a\x1B+c;\x9A\xCA\0\x85ag(V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01a\x11~V[a\x1B\x84`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x1B\xC7\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa=\x1D\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C4Wa\x1C4a[JV[`\x02\x81\x11\x15a\x1CEWa\x1CEa[JV[\x90RP\x93\x92PPPV[```v\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x13gW\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1C\x92\x90ae\xD5V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1C\xBE\x90ae\xD5V[\x80\x15a\x1D\x0BW\x80`\x1F\x10a\x1C\xE0Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\x0BV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\xEEW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1CsV[`fT`\0\x90a\x01\0\x90\x04`\xFF\x16\x15a\x1DAWP`fTa\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1EGW`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1D\xCF\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01ag\xADV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D\xE9\x91ag\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1E&W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1E+V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1EC\x91\x90ac\xCBV[\x91PP[\x91\x90PV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1EvW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`4T`\x01`@\x1B\x90\x04`\xFF\x16\x15a\x1E\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ad\x92V[`3Ta\x1E\xB5\x90`\x01`\x01`\xA0\x1B\x03\x16a+\x14V[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1E\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90af\nV[a\x1F\rc;\x9A\xCA\0\x82ah\x10V[\x15a\x1F\x97W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountWei must be a who`d\x82\x01Rm\x1B\x19H\x11\xDD\xD9ZH\x18[[\xDD[\x9D`\x92\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0a\x1F\xA7c;\x9A\xCA\0\x83ah$V[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a `W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`b`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountGwei exceeds with`d\x82\x01R\x7FdrawableRestakedExecutionLayerGw`\x84\x82\x01Raei`\xF0\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`4\x80T\x82\x91\x90`\0\x90a ~\x90\x84\x90`\x01`\x01`@\x1B\x03\x16ah8V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa \xDD\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a \xEF\x83\x83a?:V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a!\x14WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a!.WP0;\x15\x80\x15a!.WP`\0T`\xFF\x16`\x01\x14[a!\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\nLV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a!\xB4W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\"'W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.initialize: podOwner ca`D\x82\x01Rsnnot be zero address``\x1B`d\x82\x01R`\x84\x01a\nLV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x84\x16`\x01`\x01`\xA0\x1B\x03\x19\x90\x91\x16\x81\x17\x90\x91U`4\x80T`\xFF`@\x1B\x19\x16`\x01`@\x1B\x17\x90U`@Q\x7F\xCA\x8D\xFC\x8C^\ng\xA7E\x01\xC0r\xA32_hRY\xBE\xBB\xAE|\xFD#\n\xB8Q\x98\xA7\x8Bp\xCD\x90`\0\x90\xA2\x80\x15a\"\xC8W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\"\xEE\x87\x87\x87\x87\x87\x87a?DV[\x97\x96PPPPPPPV[`\0\x82\x81R`6` R`@\x90 \x80T\x82\x91\x90`\xFF`\xC0\x1B\x19\x16`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a#*Wa#*a[JV[\x02\x17\x90UPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a#]W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xC5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a#\xE9\x91\x90ac\xCBV[\x15a$\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x82Q\x84Q\x14a$\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.recoverTokens: tokenLis`D\x82\x01R\x7Ft and amountsToWithdraw must be `d\x82\x01Rj\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0[\x84Q\x81\x10\x15a$\xFFWa$\xED\x83\x85\x83\x81Q\x81\x10a$\xB3Wa$\xB3ad\xE1V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a$\xCDWa$\xCDad\xE1V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16aA\x82\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a$\xF7\x81ae\xBAV[\x91PPa$\x94V[PPPPPV[```r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x95W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BwWPPPPP\x90P\x90V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x04\x80\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%\xCDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a%\xF1\x91\x90ac\xCBV[\x15a&\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[\x83\x86\x14\x80\x15a&\x1CWP\x85\x88\x14[\x80\x15a&'WP\x87\x82\x14[a&\x9BW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPod.verifyAndProcessWithdra`D\x82\x01R\x7Fwals: inputs must be same length`d\x82\x01R`\x84\x01a\nLV[`@Qc\xD1\xC6L\xC9`\xE0\x1B\x81R`\x01`\x01`@\x1B\x03\x8C\x16`\x04\x82\x01Ra'?\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c\xD1\xC6L\xC9\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'0\x91\x90ad\xF7V[\x8B5a\x10~` \x8E\x01\x8Eae\x10V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0[\x83\x81\x10\x15a(?W`\0a'\xFA\x8D5\x8D\x8D\x85\x81\x81\x10a'wWa'wad\xE1V[\x90P` \x02\x81\x01\x90a'\x89\x91\x90ah`V[\x8C\x8C\x86\x81\x81\x10a'\x9BWa'\x9Bad\xE1V[\x90P` \x02\x81\x01\x90a'\xAD\x91\x90ae\x10V[\x8C\x8C\x88\x81\x81\x10a'\xBFWa'\xBFad\xE1V[\x90P` \x02\x81\x01\x90a'\xD1\x91\x90aeqV[\x8C\x8C\x8A\x81\x81\x10a'\xE3Wa'\xE3ad\xE1V[\x90P` \x02\x81\x01\x90a'\xF5\x91\x90aeqV[a+HV[\x80Q\x84Q\x91\x92P\x90\x84\x90a(\x0F\x90\x83\x90aW\xABV[\x90RP` \x80\x82\x01Q\x90\x84\x01\x80Qa((\x90\x83\x90af\xE7V[\x90RP\x81\x90Pa(7\x81ae\xBAV[\x91PPa'VV[P\x80Q\x15a(nW`3T\x81Qa(n\x91`\x01`\x01`\xA0\x1B\x03\x16\x90a(i\x90c;\x9A\xCA\0\x90ahwV[aA\xD4V[` \x81\x01Q\x15a)#W`3T` \x82\x01Q`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92c\xC2\xC5\x1C@\x92\x91\x16\x90a(\xC4\x90c;\x9A\xCA\0\x90ag(V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a)\nW`\0\x80\xFD[PZ\xF1\x15\x80\x15a)\x1EW=`\0\x80>=`\0\xFD[PPPP[PPPPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a)[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90adJV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xE7\x91\x90ac\xCBV[\x15a*\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x90ac\xEDV[`7T\x82\x11\x15a*\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FEigenPod.withdrawnonBeaconChainE`D\x82\x01R\x7FTHBalanceWei: amountToWithdraw i`d\x82\x01R\x7Fs greater than nonBeaconChainETH`\x84\x82\x01RiBalanceWei`\xB0\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[\x81`7`\0\x82\x82Ta*\xC7\x91\x90ah\x96V[\x90\x91UPP`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F0B\n\xAC\xD0(\xAB\xB3\xC1\xFD\x03\xAB\xA2S\xAEr]m\xDDR\xD1l\x9A\xC4\xCBWB\xCDC\xF50\x96\x90` \x01`@Q\x80\x91\x03\x90\xA2a \xEF\x83\x83aA\xD4V[`3\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x16Bc\xFF\xFF\xFF\xFF\x16`\x01`\xA0\x1B\x02\x17\x90U`\0`7Ua+E\x81GaA\xD4V[PV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra+ma+h\x89ai\x1CV[aBbV[`3T`\x01`\x01`@\x1B\x03`\x01`\xA0\x1B\x90\x91\x04\x81\x16\x90\x82\x16\x10\x15a,/W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R\x7FEigenPod.proofIsForValidTimestam`D\x82\x01R\x7Fp: beacon chain proof must be at`d\x82\x01R\x7F or after mostRecentWithdrawalTi`\x84\x82\x01Rf\x06\xD6W7F\x16\xD7`\xCC\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`\0a,=a+h\x8Bai\x1CV[\x90P`\0a,}\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[\x90P`\0\x80\x82\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a,\xAAWa,\xAAa[JV[\x14\x15a-aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`t`$\x82\x01R\x7FEigenPod._verifyAndProcessWithdr`D\x82\x01R\x7Fawal: Validator never proven to `d\x82\x01R\x7Fhave withdrawal credentials poin`\x84\x82\x01Rs\x1D\x19Y\x08\x1D\x1B\xC8\x1D\x1A\x1A\\\xC8\x18\xDB\xDB\x9D\x1C\x98X\xDD`b\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[`\0\x81\x81R`5` \x90\x81R`@\x80\x83 `\x01`\x01`@\x1B\x03\x86\x16\x84R\x90\x91R\x90 T`\xFF\x16\x15a. W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FEigenPod._verifyAndProcessWithdr`D\x82\x01R\x7Fawal: withdrawal has already bee`d\x82\x01R\x7Fn proven for this timestamp\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\x01`5`\0\x83\x81R` \x01\x90\x81R` \x01`\0 `\0\x84`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83\x15\x15\x02\x17\x90UPa.\xFD\x8C\x87\x87\x8E\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cD\xE7\x1C\x80`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a.\xD4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a.\xF8\x91\x90ajXV[aB\x96V[`\0a/;\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaL\xB7\x92PPPV[\x90Pa/K\x8D\x8A\x8A\x8E\x8E\x86aL\xDCV[`\0a/\x89\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaO3\x92PPPV[\x90Pa/\xC7\x8A\x8A\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOK\x92PPPV[`\x01`\x01`@\x1B\x03\x16a/\xE1a/\xDC\x8Fai\x1CV[aOcV[`\x01`\x01`@\x1B\x03\x16\x10a0\x99W`3T`\0\x84\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93Ra0\x8E\x93\x86\x93\x88\x93\x8A\x93`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x88\x92\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a0uWa0ua[JV[`\x02\x81\x11\x15a0\x86Wa0\x86a[JV[\x90RPa?DV[\x95PPPPPa0\xB3V[`3Ta0\x8E\x90\x83\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x16\x84a>\x17V[P\x98\x97PPPPPPPPV[`\0\x80a0\xFF\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOu\x92PPPV[\x90P`\0a1?\x85\x85\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a1\xAEWa1\xAEa[JV[`\x02\x81\x11\x15a1\xBFWa1\xBFa[JV[\x81RPP\x90P\x8A`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a2vW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\\`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: Va`D\x82\x01R\x7Flidators balance has already bee`d\x82\x01R\x7Fn updated for this timestamp\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\x01\x81``\x01Q`\x02\x81\x11\x15a2\x8EWa2\x8Ea[JV[\x14a2\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`2`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: Va`D\x82\x01Rqlidator not active`p\x1B`d\x82\x01R`\x84\x01a\nLV[a2\xFF\x8Ba5\x9EV[`\x01`\x01`@\x1B\x03\x16a3D\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOK\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x11a3\xE7W`\0\x83`\x01`\x01`@\x1B\x03\x16\x11a3\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FEigenPod.verifyBalanceUpdate: va`D\x82\x01R\x7Flidator is withdrawable but has `d\x82\x01Rl77\xBA\x10;\xB4\xBA4290\xBB\xB7`\x99\x1B`\x84\x82\x01R`\xA4\x01a\nLV[a3\xF5\x89\x87\x87\x8B\x8B\x8FaL\xDCV[` \x81\x01Q`\0`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x86\x16\x11\x15a4WWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a4ZV[P\x83[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x80\x86\x01\x91\x82R\x8F\x83\x16`@\x80\x88\x01\x91\x82R`\0\x89\x81R`6\x90\x93R\x90\x91 \x86Q\x81T\x93Q\x92Q\x85\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x93\x86\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x95\x16\x17\x92\x90\x92\x17\x90\x81\x16\x83\x17\x82U``\x86\x01Q\x86\x93\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a5\x02Wa5\x02a[JV[\x02\x17\x90UP\x90PP\x81`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a5\x8EW\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x8C\x8E\x83`@Qa5y\x93\x92\x91\x90d\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x83R`\x01`\x01`@\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1a5\x8B\x81\x83aO\x8DV[\x95P[PPPPP\x97\x96PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x16\x82`\x01`\x01`@\x1B\x03\x16\x10\x15a6HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FEigenPod._timestampToEpoch: time`D\x82\x01R\x7Fstamp is before genesis\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nLV[a6T`\x0C` ajuV[a6~\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84ah8V[a\x12{\x91\x90aj\xA4V[`\0a6\x96`\x0C` ajuV[a6\xA1\x83`\x01aj\xCAV[a6\xAB\x91\x90ajuV[a\x12{\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aj\xCAV[a6\xE1`\x03` ahwV[\x81\x14a7qW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`S`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7FtAgainstLatestBlockRoot: Proof h`d\x82\x01Rr\x0C.d\r-\xCCm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`k\x1B`\x84\x82\x01R`\xA4\x01a\nLV[a7\xB6\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92P\x87\x91P`\x03\x90PaO\xACV[a8]W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`f`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7FtAgainstLatestBlockRoot: Invalid`d\x82\x01R\x7F latest block header root merkle`\x84\x82\x01Re\x10897\xB7\xB3`\xD1\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[PPPPV[`\0\x80a8\xA2\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaBr\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a9\x11Wa9\x11a[JV[`\x02\x81\x11\x15a9\"Wa9\"a[JV[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a9?Wa9?a[JV[\x14a9\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`g`$\x82\x01R\x7FEigenPod.verifyCorrectWithdrawal`D\x82\x01R\x7FCredentials: Validator must be i`d\x82\x01R\x7Fnactive to prove withdrawal cred`\x84\x82\x01Rfentials`\xC8\x1B`\xA4\x82\x01R`\xC4\x01a\nLV[a9\xF0a>\xF5V[a9\xF9\x90aj\xF5V[a:5\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaO\xC4\x92PPPV[\x14a:\xBCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.verifyCorrectWithdrawal`D\x82\x01R\x7FCredentials: Proof is not for th`d\x82\x01Rj\x1A\\\xC8\x11ZY\xD9[\x94\x1B\xD9`\xAA\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0a:\xFA\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaOu\x92PPPV[\x90Pa;\n\x8A\x87\x87\x8B\x8B\x8EaL\xDCV[`9\x80T\x90`\0a;\x1A\x83ae\xBAV[\x90\x91UPP`\x01``\x83\x01Rd\xFF\xFF\xFF\xFF\xFF\x89\x16\x82R`\x01`\x01`@\x1B\x03\x8B\x81\x16`@\x84\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x90\x82\x16\x11\x15a;\xA0W`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16` \x83\x01Ra;\xB0V[`\x01`\x01`@\x1B\x03\x81\x16` \x83\x01R[`\0\x83\x81R`6` \x90\x81R`@\x91\x82\x90 \x84Q\x81T\x92\x86\x01Q\x93\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x85\x01Q\x85\x93\x91\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a<NWa<Na[JV[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8B\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x89\x8C\x84` \x01Q`@Qa<\xE9\x93\x92\x91\x90d\xFF\xFF\xFF\xFF\xFF\x93\x90\x93\x16\x83R`\x01`\x01`@\x1B\x03\x91\x82\x16` \x84\x01R\x16`@\x82\x01R``\x01\x90V[`@Q\x80\x91\x03\x90\xA1c;\x9A\xCA\0\x82` \x01Q`\x01`\x01`@\x1B\x03\x16a=\x0E\x91\x90ahwV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a=\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigenPod._calculateValidatorPubk`D\x82\x01R\x7FeyHash must be a 48-byte BLS pub`d\x82\x01Rflic key`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@Q`\x02\x90a=\xBD\x90\x84\x90`\0\x90` \x01ak\x19V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra=\xD7\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a=\xF4W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12{\x91\x90ad\xF7V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x87\x16\x81R`\x01`\x01`@\x1B\x03\x80\x87\x16` \x83\x01R\x84\x16\x91\x81\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90\x7F\x8As5qB1\xDB\xD5Q\xAA\xBAc\x14\xF4\xA9z\x14\xC2\x01\xE5:>%\xE1\x14\x03%\xCD\xF6}zN\x90``\x01`@Q\x80\x91\x03\x90\xA2`8\x80T\x83\x91\x90`\0\x90a>\xA8\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aj\xCAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP`@Q\x80`@\x01`@R\x80\x83`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0\x81RP\x90P\x94\x93PPPPV[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[a\"\xC8\x82\x82aO\xD9V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x16\x84`\x01`\x01`@\x1B\x03\x16\x11\x15a?\xBBWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a?\xBEV[P\x82[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra?\xDC\x82\x86ah8V[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R`4\x80T\x84\x92`\0\x91a?\xFE\x91\x85\x91\x16aj\xCAV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa@0\x82\x85` \x01QaO\x8DV[` \x82\x01R`\x02\x84``\x01Q`\x02\x81\x11\x15a@MWa@Ma[JV[\x14a@oW`9\x80T\x90`\0a@b\x83akHV[\x90\x91UPP`\x02``\x85\x01R[`\0` \x80\x86\x01\x82\x81R\x8A\x83R`6\x90\x91R`@\x91\x82\x90 \x86Q\x81T\x92Q\x93\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x92\x90\x91\x16\x91\x90\x91\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x87\x01Q\x87\x93\x91\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15aA\x14WaA\x14a[JV[\x02\x17\x90UPP`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x8A\x81\x16` \x83\x01R\x88\x16\x81\x83\x01R\x90Q`\x01`\x01`\xA0\x1B\x03\x89\x16\x92P\x7F\xB7j\x93\xBBd\x9E\xCERF\x88\xF1\xA0\x1D\x18N\x0B\xBE\xBC\xDAX\xEA\xE8\x0C(\xA8\x98\xBE\xC3\xFBZ\tc\x91\x81\x90\x03``\x01\x90\xA2\x98\x97PPPPPPPPV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra \xEF\x90\x84\x90aP\xF2V[`3T`@Qc06\xCDS`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R\x83\x82\x16`$\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC0\xDB5L\x90\x83\x90`D\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15aBEW`\0\x80\xFD[PZ\xF1\x15\x80\x15aBYW=`\0\x80>=`\0\xFD[PPPPPPPV[`\0a\x12{\x82a\x01@\x01QaQ\xC4V[`\0\x81`\0\x81Q\x81\x10aB\x87WaB\x87ad\xE1V[` \x02` \x01\x01Q\x90P\x91\x90PV[aB\xA1`\x02\x80alCV[\x83\x14aC\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalFields has incorre`d\x82\x01Rh\x0Cn\x84\r\x8C\xAD\xCC\xEE\x8D`\xBB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aC!`\r`\x02alCV[aC1`\xC0\x84\x01`\xA0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aC\x9BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: blockRootIndex is too large\0`d\x82\x01R`\x84\x01a\nLV[aC\xA7`\x04`\x02alCV[aC\xB8a\x01\0\x84\x01`\xE0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aD$W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalIndex is too large`d\x82\x01R`\x84\x01a\nLV[aD0`\x18`\x02alCV[aD@`\xE0\x84\x01`\xC0\x85\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x10aD\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: historicalSummaryIndex is to`d\x82\x01Rfo large`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0`\x01`\x01`@\x1B\x03\x82\x16aD\xD2a+h\x85ai\x1CV[`\x01`\x01`@\x1B\x03\x16\x10aD\xE7W`\x05aD\xEAV[`\x04[\x90PaD\xF7`\x04\x82aW\xABV[aE\x02\x90`\x01aW\xABV[aE\r\x90` ahwV[aE\x17\x84\x80ae\x10V[\x90P\x14aE\x8BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`H`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: withdrawalProof has incorrec`d\x82\x01Rg\x0E\x84\r\x8C\xAD\xCC\xEE\x8D`\xC3\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aE\x97`\x04`\x03aW\xABV[aE\xA2\x90` ahwV[aE\xAF`@\x85\x01\x85ae\x10V[\x90P\x14aF)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: executionPayloadProof has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aF5`\x03` ahwV[aFB` \x85\x01\x85ae\x10V[\x90P\x14aF\xB0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: slotProof has incorrect leng`d\x82\x01Ra\x0E\x8D`\xF3\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aF\xBB\x81` ahwV[aF\xC8``\x85\x01\x85ae\x10V[\x90P\x14aG;W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: timestampProof has incorrect`d\x82\x01Rf\x04\r\x8C\xAD\xCC\xEE\x8D`\xCB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\raGI`\x18`\x01aW\xABV[aGT\x90`\x05aW\xABV[aG_\x90`\x01aW\xABV[aGi\x91\x90aW\xABV[aGt\x90` ahwV[aG\x81`\x80\x85\x01\x85ae\x10V[\x90P\x14aH\nW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`X`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: historicalSummaryBlockRootPr`d\x82\x01R\x7Foof has incorrect length\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\nLV[`\0aH\x1C`\xC0\x85\x01`\xA0\x86\x01aa(V[`\x01`\x01`@\x1B\x03\x16`\0aH3`\r`\x01aW\xABV[aHC`\xE0\x88\x01`\xC0\x89\x01aa(V[`\x01`\x01`@\x1B\x03\x16\x90\x1B`\raH\\`\x18`\x01aW\xABV[aHg\x90`\x01aW\xABV[aHq\x91\x90aW\xABV[`\x1B\x90\x1B\x17\x17\x17\x90PaH\xCCaH\x8A`\x80\x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8B\x92PPPa\x01\0\x87\x015\x84aO\xACV[aI?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`J`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid historicalsummary me`d\x82\x01Ri95\xB62\x90897\xB7\xB3`\xB1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[aI\x96aIO` \x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RPa\x01\0\x8A\x015\x93Pa\x01 \x8A\x015\x92P\x90PaO\xACV[aI\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid slot merkle proof\0\0\0`d\x82\x01R`\x84\x01a\nLV[`IaJNaJ\x08`@\x87\x01\x87ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01\0\x87\x015a\x01`\x88\x015\x84aO\xACV[aJ\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid executionPayload mer`d\x82\x01Rh5\xB62\x90897\xB7\xB3`\xB9\x1B`\x84\x82\x01R`\xA4\x01a\nLV[PaK\x18aJ\xD1``\x86\x01\x86ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01`\x86\x015a\x01@\x87\x015`\taO\xACV[aK\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid timestamp merkle pro`d\x82\x01Ra7\xB3`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0aK\x96a\x01\0\x86\x01`\xE0\x87\x01aa(V[`\x01`\x01`@\x1B\x03\x16aK\xAB`\x04`\x01aW\xABV[`\x0E\x90\x1B\x17\x90P`\0aK\xF0\x88\x88\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaR+\x92PPPV[\x90PaL@aK\xFF\x87\x80ae\x10V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPPPa\x01`\x88\x015\x83\x85aO\xACV[aL\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` al\xA2\x839\x81Q\x91R`D\x82\x01R\x7Fal: Invalid withdrawal merkle pr`d\x82\x01Rb7\xB7\xB3`\xE9\x1B`\x84\x82\x01R`\xA4\x01a\nLV[PPPPPPPPPV[`\0a\x12{\x82`\x01\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[` \x02` \x01\x01QaQ\xC4V[aL\xE8`\x03`\x02alCV[\x84\x14aMsW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Validator fields has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\x05aM\x81`(`\x01aW\xABV[aM\x8B\x91\x90aW\xABV[aM\x96\x90` ahwV[\x82\x14aN\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Proof has incorrect len`d\x82\x01Rb\x0C\xEE\x8D`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`\0d\xFF\xFF\xFF\xFF\xFF\x82\x16aN,`(`\x01aW\xABV[`\x0B\x90\x1B\x17\x90P`\0aNq\x87\x87\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPaR+\x92PPPV[\x90PaN\xB7\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x85\x91P\x86\x90PaO\xACV[aO)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChainProofs.verifyValidato`D\x82\x01R\x7FrFields: Invalid merkle proof\0\0\0`d\x82\x01R`\x84\x01a\nLV[PPPPPPPPV[`\0a\x12{\x82`\x03\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0a\x12{\x82`\x07\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0` a6~\x83a\x01 \x01QaQ\xC4V[`\0a\x12{\x82`\x02\x81Q\x81\x10aL\xCFWaL\xCFad\xE1V[`\0aO\xA5`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16alOV[\x93\x92PPPV[`\0\x83aO\xBA\x86\x85\x85aT\xD8V[\x14\x95\x94PPPPPV[`\0\x81`\x01\x81Q\x81\x10aB\x87WaB\x87ad\xE1V[\x80G\x10\x15aP)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\nLV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aPvW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aP{V[``\x91P[PP\x90P\x80a \xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\nLV[`\0aQG\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16aV$\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a \xEFW\x80\x80` \x01\x90Q\x81\x01\x90aQe\x91\x90ac\xCBV[a \xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\nLV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[`\0\x80`\x02\x83QaR<\x91\x90ah$V[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15aRXWaRXa_\xBAV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15aR\x81W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15aS\x88W`\x02\x85aR\x9C\x83\x83ahwV[\x81Q\x81\x10aR\xACWaR\xACad\xE1V[` \x02` \x01\x01Q\x86\x83`\x02aR\xC2\x91\x90ahwV[aR\xCD\x90`\x01aW\xABV[\x81Q\x81\x10aR\xDDWaR\xDDad\xE1V[` \x02` \x01\x01Q`@Q` \x01aR\xFF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaS\x19\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15aS6W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aSY\x91\x90ad\xF7V[\x82\x82\x81Q\x81\x10aSkWaSkad\xE1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80aS\x80\x81ae\xBAV[\x91PPaR\x87V[PaS\x94`\x02\x83ah$V[\x91P[\x81\x15aT\xB4W`\0[\x82\x81\x10\x15aT\xA1W`\x02\x82aS\xB5\x83\x83ahwV[\x81Q\x81\x10aS\xC5WaS\xC5ad\xE1V[` \x02` \x01\x01Q\x83\x83`\x02aS\xDB\x91\x90ahwV[aS\xE6\x90`\x01aW\xABV[\x81Q\x81\x10aS\xF6WaS\xF6ad\xE1V[` \x02` \x01\x01Q`@Q` \x01aT\x18\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90RaT2\x91ag\xDEV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15aTOW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90aTr\x91\x90ad\xF7V[\x82\x82\x81Q\x81\x10aT\x84WaT\x84ad\xE1V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80aT\x99\x81ae\xBAV[\x91PPaS\xA0V[PaT\xAD`\x02\x83ah$V[\x91PaS\x97V[\x80`\0\x81Q\x81\x10aT\xC7WaT\xC7ad\xE1V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0\x83Q`\0\x14\x15\x80\x15aT\xF7WP` \x84QaT\xF5\x91\x90ah\x10V[\x15[aU\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FMerkle.processInclusionProofSha2`D\x82\x01R\x7F56: proof length should be a non`d\x82\x01Rs\x16\xBD2\xB97\x906\xBA\xB6:4\xB862\x907\xB3\x10\x19\x99`a\x1B`\x84\x82\x01R`\xA4\x01a\nLV[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11aV\x1AWaU\xAA`\x02\x85ah\x10V[aU\xDDW\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAaU\xD2W`\0\x80\xFD[`\x02\x84\x04\x93PaV\x08V[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAaV\x01W`\0\x80\xFD[`\x02\x84\x04\x93P[aV\x13` \x82aW\xABV[\x90PaU\x97V[PQ\x94\x93PPPPV[``aV3\x84\x84`\0\x85aV;V[\x94\x93PPPPV[``\x82G\x10\x15aV\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\nLV[`\x01`\x01`\xA0\x1B\x03\x85\x16;aV\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\nLV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@QaW\x0F\x91\x90ag\xDEV[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14aWLW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>aWQV[``\x91P[P\x91P\x91Pa\"\xEE\x82\x82\x86``\x83\x15aWkWP\x81aO\xA5V[\x82Q\x15aW{W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\nL\x91\x90al\x8EV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aW\xBEWaW\xBEaW\x95V[P\x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12aW\xD5W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aW\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15aX\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12aX\x1DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aX4W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15aX\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15aXkW`\0\x80\xFD[\x885\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aX\x89W`\0\x80\xFD[\x90\x8A\x01\x90a\x01\x80\x82\x8D\x03\x12\x15aX\x9EW`\0\x80\xFD[\x90\x97P`@\x8A\x015\x90\x80\x82\x11\x15aX\xB4W`\0\x80\xFD[aX\xC0\x8C\x83\x8D\x01aW\xC3V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15aX\xD9W`\0\x80\xFD[aX\xE5\x8C\x83\x8D\x01aX\x0BV[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15aX\xFEW`\0\x80\xFD[PaY\x0B\x8B\x82\x8C\x01aX\x0BV[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15aY`W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01aY;V[P\x90\x96\x95PPPPPPV[`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a+EW`\0\x80\xFD[\x805a\x1EG\x81aYlV[`\0\x80`@\x83\x85\x03\x12\x15aY\x9FW`\0\x80\xFD[\x825\x91P` \x83\x015aY\xB1\x81aYlV[\x80\x91PP\x92P\x92\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1EGW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x89\x8B\x03\x12\x15aY\xEDW`\0\x80\xFD[\x885aY\xF8\x81aYlV[\x97PaZ\x06` \x8A\x01aY\xBCV[\x96P`@\x89\x015\x95P``\x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ)W`\0\x80\xFD[aZ5\x8C\x83\x8D\x01aW\xC3V[\x90\x97P\x95P`\x80\x8B\x015\x91P\x80\x82\x11\x15aZNW`\0\x80\xFD[PaZ[\x8B\x82\x8C\x01aX\x0BV[\x90\x94P\x92PP`\xA0\x89\x015aZo\x81aYlV[\x80\x91PP\x92\x95\x98P\x92\x95\x98\x90\x93\x96PV[`\0`@\x82\x84\x03\x12\x15aZ\x92W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15aZ\xB4W`\0\x80\xFD[\x885aZ\xBF\x81aYlV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aZ\xDBW`\0\x80\xFD[aZ\xE7\x8C\x83\x8D\x01aZ\x80V[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15aZ\xFDW`\0\x80\xFD[aX\xC0\x8C\x83\x8D\x01aX\x0BV[`\0\x80` \x83\x85\x03\x12\x15a[\x1CW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a[2W`\0\x80\xFD[a[>\x85\x82\x86\x01aW\xC3V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a[~WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x12{\x82\x84a[`V[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x92P\x82\x86\x01\x91P\x82\x81`\x05\x1B\x87\x01\x01\x84\x88\x01`\0\x80[\x84\x81\x10\x15a\\4W\x89\x84\x03`?\x19\x01\x86R\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x01Q\x88\x85\x01\x88\x90R\x80Q\x88\x86\x01\x81\x90R\x90\x89\x01\x90\x83\x90``\x87\x01\x90[\x80\x83\x10\x15a\\\x1FW\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R\x92\x8B\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x8B\x01\x90a[\xF5V[P\x97\x8A\x01\x97\x95PPP\x91\x87\x01\x91`\x01\x01a[\xB8V[P\x91\x99\x98PPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\\UW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Qa\\\x9D``\x84\x01\x82a[`V[P\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a+EW`\0\x80\xFD[\x805a\x1EG\x81a\\\xA4V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\\\xDAW`\0\x80\xFD[a\\\xE3\x85aY\xBCV[\x93P` \x85\x015a\\\xF3\x81aYlV[\x92P`@\x85\x015a]\x03\x81a\\\xA4V[\x91P``\x85\x015a]\x13\x81aYlV[\x93\x96\x92\x95P\x90\x93PPV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a]9W`\0\x80\xFD[\x875a]D\x81aYlV[\x96P` \x88\x015\x95Pa]Y`@\x89\x01aY\xBCV[\x94P``\x88\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a]uW`\0\x80\xFD[a]\x81\x8B\x83\x8C\x01aW\xC3V[\x90\x96P\x94P`\x80\x8A\x015\x91P\x80\x82\x11\x15a]\x9AW`\0\x80\xFD[Pa]\xA7\x8A\x82\x8B\x01aX\x0BV[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[`\0[\x83\x81\x10\x15a]\xD5W\x81\x81\x01Q\x83\x82\x01R` \x01a]\xBDV[\x83\x81\x11\x15a8]WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra]\xFE\x81` \x86\x01` \x86\x01a]\xBAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\0` \x80\x83\x01\x81\x84R\x80\x85Q\x80\x83R`@\x86\x01\x91P`@\x81`\x05\x1B\x87\x01\x01\x92P\x83\x87\x01`\0[\x82\x81\x10\x15a^gW`?\x19\x88\x86\x03\x01\x84Ra^U\x85\x83Qa]\xE6V[\x94P\x92\x85\x01\x92\x90\x85\x01\x90`\x01\x01a^9V[P\x92\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a^\x8CW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a^\xA3W`\0\x80\xFD[a^\xAF\x89\x83\x8A\x01aW\xC3V[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a^\xC8W`\0\x80\xFD[Pa^\xD5\x88\x82\x89\x01aW\xC3V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a_\x03W`\0\x80\xFD[\x885a_\x0E\x81aYlV[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a_*W`\0\x80\xFD[a_6\x8C\x83\x8D\x01aX\x0BV[\x90\x99P\x97P`@\x8B\x015\x91P\x80\x82\x11\x15a_OW`\0\x80\xFD[a_[\x8C\x83\x8D\x01aZ\x80V[\x96P``\x8B\x015\x91P\x80\x82\x11\x15aX\xD9W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a_\x84W`\0\x80\xFD[\x825a_\x8F\x81a\\\xA4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a_\xAFW`\0\x80\xFD[\x815aO\xA5\x81a\\\xA4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a_\xF2Wa_\xF2a_\xBAV[`@R\x90V[`@Qa\x01\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a_\xF2Wa_\xF2a_\xBAV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a`CWa`Ca_\xBAV[`@R\x91\x90PV[\x805`\x03\x81\x10a\x1EGW`\0\x80\xFD[`\0\x80`\0\x80`\0\x80\x86\x88\x03a\x01 \x81\x12\x15a`uW`\0\x80\xFD[a`~\x88aY\xBCV[\x96P` \x88\x015\x95P`@\x88\x015a`\x95\x81aYlV[\x94P``\x88\x015a`\xA5\x81a\\\xA4V[\x93P`\x80\x88\x015a`\xB5\x81aYlV[\x92P`\x80`\x9F\x19\x82\x01\x12\x15a`\xC9W`\0\x80\xFD[Pa`\xD2a_\xD0V[`\xA0\x88\x015a`\xE0\x81aYlV[\x81R`\xC0\x88\x015a`\xF0\x81aYlV[` \x82\x01R`\xE0\x88\x015aa\x03\x81aYlV[`@\x82\x01Raa\x15a\x01\0\x89\x01a`KV[``\x82\x01R\x80\x91PP\x92\x95P\x92\x95P\x92\x95V[`\0` \x82\x84\x03\x12\x15aa:W`\0\x80\xFD[\x815aO\xA5\x81aYlV[`\0\x80`@\x83\x85\x03\x12\x15aaXW`\0\x80\xFD[\x825\x91Paah` \x84\x01a`KV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aa\x8AWaa\x8Aa_\xBAV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aa\xA5W`\0\x80\xFD[\x815` aa\xBAaa\xB5\x83aaqV[a`\x1BV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aa\xD9W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aa\xF4W\x805\x83R\x91\x83\x01\x91\x83\x01aa\xDDV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15ab\x14W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ab+W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12ab?W`\0\x80\xFD[\x815` abOaa\xB5\x83aaqV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15abnW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15ab\x95W\x855ab\x86\x81a\\\xA4V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90absV[\x97PP\x87\x015\x92PP\x80\x82\x11\x15ab\xABW`\0\x80\xFD[Pab\xB8\x86\x82\x87\x01aa\x94V[\x92PPab\xC7`@\x85\x01a\\\xB9V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15ab\xEFW`\0\x80\xFD[ab\xF8\x8BaY\x81V[\x99P` \x8B\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15ac\x14W`\0\x80\xFD[ac \x8E\x83\x8F\x01aZ\x80V[\x9AP`@\x8D\x015\x91P\x80\x82\x11\x15ac6W`\0\x80\xFD[acB\x8E\x83\x8F\x01aX\x0BV[\x90\x9AP\x98P``\x8D\x015\x91P\x80\x82\x11\x15ac[W`\0\x80\xFD[acg\x8E\x83\x8F\x01aX\x0BV[\x90\x98P\x96P`\x80\x8D\x015\x91P\x80\x82\x11\x15ac\x80W`\0\x80\xFD[ac\x8C\x8E\x83\x8F\x01aX\x0BV[\x90\x96P\x94P`\xA0\x8D\x015\x91P\x80\x82\x11\x15ac\xA5W`\0\x80\xFD[Pac\xB2\x8D\x82\x8E\x01aX\x0BV[\x91P\x80\x93PP\x80\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0` \x82\x84\x03\x12\x15ac\xDDW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14aO\xA5W`\0\x80\xFD[` \x80\x82R`>\x90\x82\x01R\x7FEigenPod.onlyWhenNotPaused: inde`@\x82\x01R\x7Fx is paused in EigenPodManager\0\0``\x82\x01R`\x80\x01\x90V[` \x80\x82R`(\x90\x82\x01R\x7FEigenPod.onlyEigenPodOwner: not `@\x82\x01Rg87\xB2'\xBB\xB72\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[` \x80\x82R`/\x90\x82\x01R\x7FEigenPod.hasNeverRestaked: resta`@\x82\x01Rn\x1A\xDA[\x99\xC8\x1A\\\xC8\x19[\x98X\x9B\x19Y`\x8A\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15ae\tW`\0\x80\xFD[PQ\x91\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ae'W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aeAW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aX\x04W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aehW`\0\x80\xFD[aO\xA5\x82aY\xBCV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12ae\x88W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15ae\xA2W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aX\x04W`\0\x80\xFD[`\0`\0\x19\x82\x14\x15ae\xCEWae\xCEaW\x95V[P`\x01\x01\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80ae\xE9W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aZ\x92WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[` \x80\x82R`1\x90\x82\x01R\x7FEigenPod.onlyEigenPodManager: no`@\x82\x01Rp:\x102\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`y\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x80\x81R`\0af\x98`\x80\x83\x01\x88\x8Aaf[V[\x82\x81\x03` \x84\x01Raf\xAA\x81\x88a]\xE6V[\x90P\x82\x81\x03`@\x84\x01Raf\xBF\x81\x86\x88af[V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0aV3` \x83\x01\x84\x86af[V[`\0\x80\x82\x12\x80\x15`\x01`\x01`\xFF\x1B\x03\x84\x90\x03\x85\x13\x16\x15ag\tWag\taW\x95V[`\x01`\xFF\x1B\x83\x90\x03\x84\x12\x81\x16\x15ag\"Wag\"aW\x95V[PP\x01\x90V[`\0`\x01`\x01`\xFF\x1B\x03\x81\x84\x13\x82\x84\x13\x80\x82\x16\x86\x84\x04\x86\x11\x16\x15agNWagNaW\x95V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15agmWagmaW\x95V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15ag\x89Wag\x89aW\x95V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15ag\x9FWag\x9FaW\x95V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90ag\xD0\x81`\x04\x85\x01` \x87\x01a]\xBAV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82Qag\xF0\x81\x84` \x87\x01a]\xBAV[\x91\x90\x91\x01\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82ah\x1FWah\x1Fag\xFAV[P\x06\x90V[`\0\x82ah3Wah3ag\xFAV[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15ahXWahXaW\x95V[\x03\x93\x92PPPV[`\0\x825a\x01~\x19\x836\x03\x01\x81\x12ag\xF0W`\0\x80\xFD[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15ah\x91Wah\x91aW\x95V[P\x02\x90V[`\0\x82\x82\x10\x15ah\xA8Wah\xA8aW\x95V[P\x03\x90V[`\0\x82`\x1F\x83\x01\x12ah\xBEW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15ah\xD7Wah\xD7a_\xBAV[ah\xEA`\x1F\x82\x01`\x1F\x19\x16` \x01a`\x1BV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15ah\xFFW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0a\x01\x80\x826\x03\x12\x15ai/W`\0\x80\xFD[ai7a_\xF8V[\x825`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aiNW`\0\x80\xFD[aiZ6\x83\x87\x01ah\xADV[\x83R` \x85\x015\x91P\x80\x82\x11\x15aipW`\0\x80\xFD[ai|6\x83\x87\x01ah\xADV[` \x84\x01R`@\x85\x015\x91P\x80\x82\x11\x15ai\x95W`\0\x80\xFD[ai\xA16\x83\x87\x01ah\xADV[`@\x84\x01R``\x85\x015\x91P\x80\x82\x11\x15ai\xBAW`\0\x80\xFD[ai\xC66\x83\x87\x01ah\xADV[``\x84\x01R`\x80\x85\x015\x91P\x80\x82\x11\x15ai\xDFW`\0\x80\xFD[Pai\xEC6\x82\x86\x01ah\xADV[`\x80\x83\x01RPai\xFE`\xA0\x84\x01aY\x81V[`\xA0\x82\x01Raj\x0F`\xC0\x84\x01aY\x81V[`\xC0\x82\x01Raj `\xE0\x84\x01aY\x81V[`\xE0\x82\x01Ra\x01\0\x83\x81\x015\x90\x82\x01Ra\x01 \x80\x84\x015\x90\x82\x01Ra\x01@\x80\x84\x015\x90\x82\x01Ra\x01`\x92\x83\x015\x92\x81\x01\x92\x90\x92RP\x90V[`\0` \x82\x84\x03\x12\x15ajjW`\0\x80\xFD[\x81QaO\xA5\x81aYlV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aj\x9BWaj\x9BaW\x95V[\x02\x94\x93PPPPV[`\0`\x01`\x01`@\x1B\x03\x80\x84\x16\x80aj\xBEWaj\xBEag\xFAV[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aj\xECWaj\xECaW\x95V[\x01\x94\x93PPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15aZ\x92W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0\x83Qak+\x81\x84` \x88\x01a]\xBAV[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81akWWakWaW\x95V[P`\0\x19\x01\x90V[`\x01\x81\x81[\x80\x85\x11\x15ak\x9AW\x81`\0\x19\x04\x82\x11\x15ak\x80Wak\x80aW\x95V[\x80\x85\x16\x15ak\x8DW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90akdV[P\x92P\x92\x90PV[`\0\x82ak\xB1WP`\x01a\x12{V[\x81ak\xBEWP`\0a\x12{V[\x81`\x01\x81\x14ak\xD4W`\x02\x81\x14ak\xDEWak\xFAV[`\x01\x91PPa\x12{V[`\xFF\x84\x11\x15ak\xEFWak\xEFaW\x95V[PP`\x01\x82\x1Ba\x12{V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15al\x1DWP\x81\x81\na\x12{V[al'\x83\x83ak_V[\x80`\0\x19\x04\x82\x11\x15al;Wal;aW\x95V[\x02\x93\x92PPPV[`\0aO\xA5\x83\x83ak\xA2V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15almWalmaW\x95V[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15al\x88Wal\x88aW\x95V[PP\x03\x90V[` \x81R`\0aO\xA5` \x83\x01\x84a]\xE6V\xFEBeaconChainProofs.verifyWithdraw\xA2dipfsX\"\x12 \xA9\xC8\x01k\x8B\x8B\x8Dv`\x17%\xFE\xBA\xBD#pg\xAA:\xA9l\x95.\x1E:\xC2\x0C\xC8\x82\xF1n\xA8dsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `EigenPodStaked(bytes)` and selector `0x606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e23`.
```solidity
event EigenPodStaked(bytes pubkey);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct EigenPodStaked {
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for EigenPodStaked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EigenPodStaked(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                96u8,
                104u8,
                101u8,
                183u8,
                147u8,
                74u8,
                37u8,
                212u8,
                174u8,
                212u8,
                63u8,
                108u8,
                219u8,
                66u8,
                100u8,
                3u8,
                53u8,
                63u8,
                164u8,
                179u8,
                0u8,
                156u8,
                77u8,
                34u8,
                132u8,
                7u8,
                71u8,
                69u8,
                129u8,
                176u8,
                30u8,
                35u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { pubkey: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
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
        impl alloy_sol_types::private::IntoLogData for EigenPodStaked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&EigenPodStaked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &EigenPodStaked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `FullWithdrawalRedeemed(uint40,uint64,address,uint64)` and selector `0xb76a93bb649ece524688f1a01d184e0bbebcda58eae80c28a898bec3fb5a0963`.
```solidity
event FullWithdrawalRedeemed(uint40 validatorIndex, uint64 withdrawalTimestamp, address indexed recipient, uint64 withdrawalAmountGwei);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct FullWithdrawalRedeemed {
        #[allow(missing_docs)]
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        #[allow(missing_docs)]
        pub withdrawalTimestamp: u64,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub withdrawalAmountGwei: u64,
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
        impl alloy_sol_types::SolEvent for FullWithdrawalRedeemed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "FullWithdrawalRedeemed(uint40,uint64,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                183u8,
                106u8,
                147u8,
                187u8,
                100u8,
                158u8,
                206u8,
                82u8,
                70u8,
                136u8,
                241u8,
                160u8,
                29u8,
                24u8,
                78u8,
                11u8,
                190u8,
                188u8,
                218u8,
                88u8,
                234u8,
                232u8,
                12u8,
                40u8,
                168u8,
                152u8,
                190u8,
                195u8,
                251u8,
                90u8,
                9u8,
                99u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    validatorIndex: data.0,
                    withdrawalTimestamp: data.1,
                    recipient: topics.1,
                    withdrawalAmountGwei: data.2,
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalAmountGwei),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
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
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for FullWithdrawalRedeemed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&FullWithdrawalRedeemed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &FullWithdrawalRedeemed) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `NonBeaconChainETHReceived(uint256)` and selector `0x6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf49`.
```solidity
event NonBeaconChainETHReceived(uint256 amountReceived);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NonBeaconChainETHReceived {
        #[allow(missing_docs)]
        pub amountReceived: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NonBeaconChainETHReceived {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NonBeaconChainETHReceived(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                111u8,
                221u8,
                61u8,
                189u8,
                177u8,
                115u8,
                41u8,
                150u8,
                8u8,
                192u8,
                170u8,
                159u8,
                54u8,
                135u8,
                53u8,
                133u8,
                124u8,
                136u8,
                66u8,
                181u8,
                129u8,
                248u8,
                56u8,
                146u8,
                56u8,
                191u8,
                5u8,
                189u8,
                4u8,
                179u8,
                191u8,
                73u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { amountReceived: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountReceived),
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
        impl alloy_sol_types::private::IntoLogData for NonBeaconChainETHReceived {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NonBeaconChainETHReceived> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &NonBeaconChainETHReceived,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `NonBeaconChainETHWithdrawn(address,uint256)` and selector `0x30420aacd028abb3c1fd03aba253ae725d6ddd52d16c9ac4cb5742cd43f53096`.
```solidity
event NonBeaconChainETHWithdrawn(address indexed recipient, uint256 amountWithdrawn);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NonBeaconChainETHWithdrawn {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountWithdrawn: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for NonBeaconChainETHWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "NonBeaconChainETHWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                48u8,
                66u8,
                10u8,
                172u8,
                208u8,
                40u8,
                171u8,
                179u8,
                193u8,
                253u8,
                3u8,
                171u8,
                162u8,
                83u8,
                174u8,
                114u8,
                93u8,
                109u8,
                221u8,
                82u8,
                209u8,
                108u8,
                154u8,
                196u8,
                203u8,
                87u8,
                66u8,
                205u8,
                67u8,
                245u8,
                48u8,
                150u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: topics.1,
                    amountWithdrawn: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amountWithdrawn),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
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
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NonBeaconChainETHWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NonBeaconChainETHWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &NonBeaconChainETHWithdrawn,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `PartialWithdrawalRedeemed(uint40,uint64,address,uint64)` and selector `0x8a7335714231dbd551aaba6314f4a97a14c201e53a3e25e1140325cdf67d7a4e`.
```solidity
event PartialWithdrawalRedeemed(uint40 validatorIndex, uint64 withdrawalTimestamp, address indexed recipient, uint64 partialWithdrawalAmountGwei);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PartialWithdrawalRedeemed {
        #[allow(missing_docs)]
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        #[allow(missing_docs)]
        pub withdrawalTimestamp: u64,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub partialWithdrawalAmountGwei: u64,
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
        impl alloy_sol_types::SolEvent for PartialWithdrawalRedeemed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "PartialWithdrawalRedeemed(uint40,uint64,address,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                138u8,
                115u8,
                53u8,
                113u8,
                66u8,
                49u8,
                219u8,
                213u8,
                81u8,
                170u8,
                186u8,
                99u8,
                20u8,
                244u8,
                169u8,
                122u8,
                20u8,
                194u8,
                1u8,
                229u8,
                58u8,
                62u8,
                37u8,
                225u8,
                20u8,
                3u8,
                37u8,
                205u8,
                246u8,
                125u8,
                122u8,
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
                    validatorIndex: data.0,
                    withdrawalTimestamp: data.1,
                    recipient: topics.1,
                    partialWithdrawalAmountGwei: data.2,
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.partialWithdrawalAmountGwei,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
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
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for PartialWithdrawalRedeemed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PartialWithdrawalRedeemed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &PartialWithdrawalRedeemed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RestakedBeaconChainETHWithdrawn(address,uint256)` and selector `0x8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e`.
```solidity
event RestakedBeaconChainETHWithdrawn(address indexed recipient, uint256 amount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RestakedBeaconChainETHWithdrawn {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for RestakedBeaconChainETHWithdrawn {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RestakedBeaconChainETHWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                71u8,
                253u8,
                44u8,
                224u8,
                126u8,
                249u8,
                204u8,
                48u8,
                44u8,
                78u8,
                143u8,
                4u8,
                97u8,
                1u8,
                86u8,
                21u8,
                217u8,
                28u8,
                232u8,
                81u8,
                86u8,
                72u8,
                57u8,
                233u8,
                28u8,
                200u8,
                4u8,
                194u8,
                244u8,
                157u8,
                142u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: topics.1,
                    amount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.recipient.clone())
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
                    &self.recipient,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RestakedBeaconChainETHWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RestakedBeaconChainETHWithdrawn>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &RestakedBeaconChainETHWithdrawn,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `RestakingActivated(address)` and selector `0xca8dfc8c5e0a67a74501c072a3325f685259bebbae7cfd230ab85198a78b70cd`.
```solidity
event RestakingActivated(address indexed podOwner);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct RestakingActivated {
        #[allow(missing_docs)]
        pub podOwner: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for RestakingActivated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RestakingActivated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                202u8,
                141u8,
                252u8,
                140u8,
                94u8,
                10u8,
                103u8,
                167u8,
                69u8,
                1u8,
                192u8,
                114u8,
                163u8,
                50u8,
                95u8,
                104u8,
                82u8,
                89u8,
                190u8,
                187u8,
                174u8,
                124u8,
                253u8,
                35u8,
                10u8,
                184u8,
                81u8,
                152u8,
                167u8,
                139u8,
                112u8,
                205u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { podOwner: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.podOwner.clone())
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
                    &self.podOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for RestakingActivated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&RestakingActivated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RestakingActivated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ValidatorBalanceUpdated(uint40,uint64,uint64)` and selector `0x0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df`.
```solidity
event ValidatorBalanceUpdated(uint40 validatorIndex, uint64 balanceTimestamp, uint64 newValidatorBalanceGwei);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorBalanceUpdated {
        #[allow(missing_docs)]
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        #[allow(missing_docs)]
        pub balanceTimestamp: u64,
        #[allow(missing_docs)]
        pub newValidatorBalanceGwei: u64,
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
        impl alloy_sol_types::SolEvent for ValidatorBalanceUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorBalanceUpdated(uint40,uint64,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                95u8,
                172u8,
                23u8,
                91u8,
                131u8,
                23u8,
                124u8,
                192u8,
                71u8,
                56u8,
                30u8,
                3u8,
                13u8,
                143u8,
                179u8,
                180u8,
                43u8,
                55u8,
                189u8,
                28u8,
                2u8,
                94u8,
                34u8,
                194u8,
                128u8,
                250u8,
                202u8,
                214u8,
                44u8,
                50u8,
                223u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    validatorIndex: data.0,
                    balanceTimestamp: data.1,
                    newValidatorBalanceGwei: data.2,
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.newValidatorBalanceGwei,
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
        impl alloy_sol_types::private::IntoLogData for ValidatorBalanceUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorBalanceUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ValidatorBalanceUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ValidatorRestaked(uint40)` and selector `0x2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449`.
```solidity
event ValidatorRestaked(uint40 validatorIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorRestaked {
        #[allow(missing_docs)]
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
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
        impl alloy_sol_types::SolEvent for ValidatorRestaked {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorRestaked(uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                45u8,
                8u8,
                0u8,
                187u8,
                195u8,
                119u8,
                234u8,
                84u8,
                160u8,
                140u8,
                93u8,
                182u8,
                168u8,
                122u8,
                175u8,
                255u8,
                94u8,
                62u8,
                156u8,
                143u8,
                234u8,
                208u8,
                237u8,
                161u8,
                16u8,
                228u8,
                14u8,
                12u8,
                16u8,
                68u8,
                20u8,
                73u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { validatorIndex: data.0 }
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
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
        impl alloy_sol_types::private::IntoLogData for ValidatorRestaked {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorRestaked> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorRestaked) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log(string)` and selector `0x41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50`.
```solidity
event log(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_address(address)` and selector `0x7ae74c527414ae135fd97047b12921a5ec3911b804197855d67e25c7b75ee6f3`.
```solidity
event log_address(address);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_address {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_address {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(uint256[])` and selector `0xfb102865d50addddf69da9b5aa1bced66c80cf869a5c8d0471a467e18ce9cab1`.
```solidity
event log_array(uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_0 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(int256[])` and selector `0x890a82679b470f2bd82816ed9b161f97d8b967f37fa3647c21d5bf39749e2dd5`.
```solidity
event log_array(int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_1 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_array(address[])` and selector `0x40e1840f5769073d61bd01372d9b75baa9842d5629a0c99ff103be1178a8e9e2`.
```solidity
event log_array(address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_array_2 {
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { val: data.0 }
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes(bytes)` and selector `0x23b62ad0584d24a75f0bf3560391ef5659ec6db1269c56e11aa241d637f19b20`.
```solidity
event log_bytes(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_bytes {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_bytes32(bytes32)` and selector `0xe81699b85113eea1c73e10588b2b035e55893369632173afd43feb192fac64e3`.
```solidity
event log_bytes32(bytes32);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_bytes32 {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_bytes32 {
            type DataTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_int(int256)` and selector `0x0eb5d52624c8d28ada9fc55a8c502ed5aa3fbe2fb6e91b71b5f376882b1d2fb8`.
```solidity
event log_int(int256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_int {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_int {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_address(string,address)` and selector `0x9c4e8541ca8f0dc1c413f9108f66d82d3cecb1bddbce437a61caa3175c4cc96f`.
```solidity
event log_named_address(string key, address val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_address {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for log_named_address {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_address {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_address> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_address) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,uint256[])` and selector `0x00aaa39c9ffb5f567a4534380c737075702e1f7f14107fc95328e3b56c0325fb`.
```solidity
event log_named_array(string key, uint256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_0 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_0 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_0 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_0> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_0) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,int256[])` and selector `0xa73eda09662f46dde729be4611385ff34fe6c44fbbc6f7e17b042b59a3445b57`.
```solidity
event log_named_array(string key, int256[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_1 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::I256,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_named_array_1 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Int<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_1 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_1> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_1) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_array(string,address[])` and selector `0x3bcfb2ae2e8d132dd1fce7cf278a9a19756a9fceabe470df3bdabb4bc577d1bd`.
```solidity
event log_named_array(string key, address[] val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_array_2 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
        impl alloy_sol_types::SolEvent for log_named_array_2 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_array_2 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_array_2> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_array_2) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes(string,bytes)` and selector `0xd26e16cad4548705e4c9e2d94f98ee91c289085ee425594fd5635fa2964ccf18`.
```solidity
event log_named_bytes(string key, bytes val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for log_named_bytes {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Bytes,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_bytes32(string,bytes32)` and selector `0xafb795c9c61e4fe7468c386f925d7a5429ecad9c0495ddb8d38d690614d32f99`.
```solidity
event log_named_bytes32(string key, bytes32 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_bytes32 {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::FixedBytes<32>,
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
        impl alloy_sol_types::SolEvent for log_named_bytes32 {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_bytes32 {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_bytes32> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_bytes32) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_int(string,int256,uint256)` and selector `0x5da6ce9d51151ba10c09a559ef24d520b9dac5c5b8810ae8434e4d0d86411a95`.
```solidity
event log_named_decimal_int(string key, int256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_decimal_uint(string,uint256,uint256)` and selector `0xeb8ba43ced7537421946bd43e828b8b2b8428927aa8f801c13d934bf11aca57b`.
```solidity
event log_named_decimal_uint(string key, uint256 val, uint256 decimals);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_decimal_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub decimals: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_decimal_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    key: data.0,
                    val: data.1,
                    decimals: data.2,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.decimals),
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
        impl alloy_sol_types::private::IntoLogData for log_named_decimal_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_decimal_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_decimal_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_int(string,int256)` and selector `0x2fe632779174374378442a8e978bccfbdcc1d6b2b0d81f7e8eb776ab2286f168`.
```solidity
event log_named_int(string key, int256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_int {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for log_named_int {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Int<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_int {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_int> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_int) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_string(string,string)` and selector `0x280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf3583`.
```solidity
event log_named_string(string key, string val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_string {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_named_string {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::String,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.val,
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
        impl alloy_sol_types::private::IntoLogData for log_named_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_named_uint(string,uint256)` and selector `0xb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8`.
```solidity
event log_named_uint(string key, uint256 val);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_named_uint {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub val: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for log_named_uint {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::String,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { key: data.0, val: data.1 }
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.val),
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
        impl alloy_sol_types::private::IntoLogData for log_named_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_named_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_named_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_string(string)` and selector `0x0b2e13ff20ac7b474198655583edf70dedd2c1dc980e329c4fbb2fc0748b796b`.
```solidity
event log_string(string);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_string {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::String,
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
        impl alloy_sol_types::SolEvent for log_string {
            type DataTuple<'a> = (alloy::sol_types::sol_data::String,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for log_string {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_string> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_string) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `log_uint(uint256)` and selector `0x2cab9790510fd8bdfbd2115288db33fec66691d476efc5427cfd4c0969301755`.
```solidity
event log_uint(uint256);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct log_uint {
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for log_uint {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
        impl alloy_sol_types::private::IntoLogData for log_uint {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&log_uint> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &log_uint) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `logs(bytes)` and selector `0xe7950ede0394b9f2ce4a5a1bf5a7e1852411f7e6661b4308c913c4bfd11027e4`.
```solidity
event logs(bytes);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct logs {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Bytes,
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
        impl alloy_sol_types::SolEvent for logs {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { _0: data.0 }
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
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
        impl alloy_sol_types::private::IntoLogData for logs {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&logs> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &logs) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _ethPOS, address _delayedWithdrawalRouter, address _eigenPodManager, uint64 _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR, uint64 _GENESIS_TIME);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _ethPOS: alloy::sol_types::private::Address,
        pub _delayedWithdrawalRouter: alloy::sol_types::private::Address,
        pub _eigenPodManager: alloy::sol_types::private::Address,
        pub _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: u64,
        pub _GENESIS_TIME: u64,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                u64,
                u64,
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
                        value._ethPOS,
                        value._delayedWithdrawalRouter,
                        value._eigenPodManager,
                        value._MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
                        value._GENESIS_TIME,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ethPOS: tuple.0,
                        _delayedWithdrawalRouter: tuple.1,
                        _eigenPodManager: tuple.2,
                        _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: tuple.3,
                        _GENESIS_TIME: tuple.4,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<64>,
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
                        &self._ethPOS,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delayedWithdrawalRouter,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._GENESIS_TIME),
                )
            }
        }
    };
    /**Function with signature `GENESIS_TIME()` and selector `0xf2882461`.
```solidity
function GENESIS_TIME() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_TIMECall {}
    ///Container type for the return parameters of the [`GENESIS_TIME()`](GENESIS_TIMECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct GENESIS_TIMEReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<GENESIS_TIMECall> for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_TIMECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENESIS_TIMECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<GENESIS_TIMEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: GENESIS_TIMEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for GENESIS_TIMEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for GENESIS_TIMECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = GENESIS_TIMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "GENESIS_TIME()";
            const SELECTOR: [u8; 4] = [242u8, 136u8, 36u8, 97u8];
            #[inline]
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
    /**Function with signature `IS_TEST()` and selector `0xfa7626d4`.
```solidity
function IS_TEST() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTCall {}
    ///Container type for the return parameters of the [`IS_TEST()`](IS_TESTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_TESTReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<IS_TESTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<IS_TESTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_TESTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_TESTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_TESTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_TEST()";
            const SELECTOR: [u8; 4] = [250u8, 118u8, 38u8, 212u8];
            #[inline]
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
    /**Function with signature `MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()` and selector `0x1d905d5c`.
```solidity
function MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall {}
    ///Container type for the return parameters of the [`MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()`](MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR()";
            const SELECTOR: [u8; 4] = [29u8, 144u8, 93u8, 92u8];
            #[inline]
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
    /**Function with signature `activateRestaking()` and selector `0x0cd4649e`.
```solidity
function activateRestaking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activateRestakingCall {}
    ///Container type for the return parameters of the [`activateRestaking()`](activateRestakingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activateRestakingReturn {}
    #[allow(
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
            impl ::core::convert::From<activateRestakingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: activateRestakingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for activateRestakingCall {
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
            impl ::core::convert::From<activateRestakingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: activateRestakingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for activateRestakingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activateRestakingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = activateRestakingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "activateRestaking()";
            const SELECTOR: [u8; 4] = [12u8, 212u8, 100u8, 158u8];
            #[inline]
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
    /**Function with signature `delayedWithdrawalRouter()` and selector `0x1a5057be`.
```solidity
function delayedWithdrawalRouter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedWithdrawalRouterCall {}
    ///Container type for the return parameters of the [`delayedWithdrawalRouter()`](delayedWithdrawalRouterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct delayedWithdrawalRouterReturn {
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
            impl ::core::convert::From<delayedWithdrawalRouterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedWithdrawalRouterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedWithdrawalRouterCall {
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
            impl ::core::convert::From<delayedWithdrawalRouterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: delayedWithdrawalRouterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for delayedWithdrawalRouterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for delayedWithdrawalRouterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = delayedWithdrawalRouterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "delayedWithdrawalRouter()";
            const SELECTOR: [u8; 4] = [26u8, 80u8, 87u8, 190u8];
            #[inline]
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
    /**Function with signature `eigenPodManager()` and selector `0x4665bcda`.
```solidity
function eigenPodManager() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerCall {}
    ///Container type for the return parameters of the [`eigenPodManager()`](eigenPodManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct eigenPodManagerReturn {
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
            impl ::core::convert::From<eigenPodManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerCall {
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
            impl ::core::convert::From<eigenPodManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "eigenPodManager()";
            const SELECTOR: [u8; 4] = [70u8, 101u8, 188u8, 218u8];
            #[inline]
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
    /**Function with signature `ethPOS()` and selector `0x74cdd798`.
```solidity
function ethPOS() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethPOSCall {}
    ///Container type for the return parameters of the [`ethPOS()`](ethPOSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ethPOSReturn {
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
            impl ::core::convert::From<ethPOSCall> for UnderlyingRustTuple<'_> {
                fn from(value: ethPOSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethPOSCall {
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
            impl ::core::convert::From<ethPOSReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ethPOSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ethPOSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ethPOSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ethPOSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ethPOS()";
            const SELECTOR: [u8; 4] = [116u8, 205u8, 215u8, 152u8];
            #[inline]
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
    /**Function with signature `excludeArtifacts()` and selector `0xb5508aa9`.
```solidity
function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsCall {}
    ///Container type for the return parameters of the [`excludeArtifacts()`](excludeArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeArtifactsReturn {
        pub excludedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeArtifacts()";
            const SELECTOR: [u8; 4] = [181u8, 80u8, 138u8, 169u8];
            #[inline]
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
    /**Function with signature `excludeContracts()` and selector `0xe20c9f71`.
```solidity
function excludeContracts() external view returns (address[] memory excludedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsCall {}
    ///Container type for the return parameters of the [`excludeContracts()`](excludeContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeContractsReturn {
        pub excludedContracts_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<excludeContractsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<excludeContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeContracts()";
            const SELECTOR: [u8; 4] = [226u8, 12u8, 159u8, 113u8];
            #[inline]
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
    /**Function with signature `excludeSenders()` and selector `0x1ed7831c`.
```solidity
function excludeSenders() external view returns (address[] memory excludedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersCall {}
    ///Container type for the return parameters of the [`excludeSenders()`](excludeSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSendersReturn {
        pub excludedSenders_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<excludeSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<excludeSendersReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { excludedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSenders()";
            const SELECTOR: [u8; 4] = [30u8, 215u8, 131u8, 28u8];
            #[inline]
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
```solidity
function failed() external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedCall {}
    ///Container type for the return parameters of the [`failed()`](failedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct failedReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<failedCall> for UnderlyingRustTuple<'_> {
                fn from(value: failedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<failedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: failedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for failedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for failedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "failed()";
            const SELECTOR: [u8; 4] = [186u8, 65u8, 79u8, 166u8];
            #[inline]
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
    /**Function with signature `getActiveValidatorCount()` and selector `0x37deea70`.
```solidity
function getActiveValidatorCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveValidatorCountCall {}
    ///Container type for the return parameters of the [`getActiveValidatorCount()`](getActiveValidatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getActiveValidatorCountReturn {
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
            impl ::core::convert::From<getActiveValidatorCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveValidatorCountCall {
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
            impl ::core::convert::From<getActiveValidatorCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getActiveValidatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getActiveValidatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getActiveValidatorCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getActiveValidatorCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getActiveValidatorCount()";
            const SELECTOR: [u8; 4] = [55u8, 222u8, 234u8, 112u8];
            #[inline]
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
    /**Function with signature `hasRestaked()` and selector `0x3106ab53`.
```solidity
function hasRestaked() external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRestakedCall {}
    ///Container type for the return parameters of the [`hasRestaked()`](hasRestakedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct hasRestakedReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<hasRestakedCall> for UnderlyingRustTuple<'_> {
                fn from(value: hasRestakedCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRestakedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<hasRestakedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: hasRestakedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for hasRestakedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for hasRestakedCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = hasRestakedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "hasRestaked()";
            const SELECTOR: [u8; 4] = [49u8, 6u8, 171u8, 83u8];
            #[inline]
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
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
```solidity
function initialize(address _podOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub _podOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address)`](initializeCall) function.
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value._podOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _podOwner: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize(address)";
            const SELECTOR: [u8; 4] = [196u8, 214u8, 109u8, 232u8];
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
                        &self._podOwner,
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
    /**Function with signature `mostRecentWithdrawalTimestamp()` and selector `0x87e0d289`.
```solidity
function mostRecentWithdrawalTimestamp() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mostRecentWithdrawalTimestampCall {}
    ///Container type for the return parameters of the [`mostRecentWithdrawalTimestamp()`](mostRecentWithdrawalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct mostRecentWithdrawalTimestampReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mostRecentWithdrawalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: mostRecentWithdrawalTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mostRecentWithdrawalTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<mostRecentWithdrawalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: mostRecentWithdrawalTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for mostRecentWithdrawalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mostRecentWithdrawalTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mostRecentWithdrawalTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mostRecentWithdrawalTimestamp()";
            const SELECTOR: [u8; 4] = [135u8, 224u8, 210u8, 137u8];
            #[inline]
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
    /**Function with signature `nonBeaconChainETHBalanceWei()` and selector `0xfe80b087`.
```solidity
function nonBeaconChainETHBalanceWei() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonBeaconChainETHBalanceWeiCall {}
    ///Container type for the return parameters of the [`nonBeaconChainETHBalanceWei()`](nonBeaconChainETHBalanceWeiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nonBeaconChainETHBalanceWeiReturn {
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
            impl ::core::convert::From<nonBeaconChainETHBalanceWeiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: nonBeaconChainETHBalanceWeiCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nonBeaconChainETHBalanceWeiCall {
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
            impl ::core::convert::From<nonBeaconChainETHBalanceWeiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: nonBeaconChainETHBalanceWeiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for nonBeaconChainETHBalanceWeiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nonBeaconChainETHBalanceWeiCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nonBeaconChainETHBalanceWeiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nonBeaconChainETHBalanceWei()";
            const SELECTOR: [u8; 4] = [254u8, 128u8, 176u8, 135u8];
            #[inline]
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
    /**Function with signature `podOwner()` and selector `0x0b18ff66`.
```solidity
function podOwner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podOwnerCall {}
    ///Container type for the return parameters of the [`podOwner()`](podOwnerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct podOwnerReturn {
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
            impl ::core::convert::From<podOwnerCall> for UnderlyingRustTuple<'_> {
                fn from(value: podOwnerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podOwnerCall {
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
            impl ::core::convert::From<podOwnerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: podOwnerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for podOwnerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for podOwnerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = podOwnerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "podOwner()";
            const SELECTOR: [u8; 4] = [11u8, 24u8, 255u8, 102u8];
            #[inline]
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
    /**Function with signature `processFullWithdrawal(uint40,bytes32,uint64,address,uint64,(uint64,uint64,uint64,uint8))` and selector `0xca044e8c`.
```solidity
function processFullWithdrawal(uint40 validatorIndex, bytes32 validatorPubkeyHash, uint64 withdrawalHappenedTimestamp, address recipient, uint64 withdrawalAmountGwei, IEigenPod.ValidatorInfo memory validatorInfo) external returns (IEigenPod.VerifiedWithdrawal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processFullWithdrawalCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        pub withdrawalHappenedTimestamp: u64,
        pub recipient: alloy::sol_types::private::Address,
        pub withdrawalAmountGwei: u64,
        pub validatorInfo: <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`processFullWithdrawal(uint40,bytes32,uint64,address,uint64,(uint64,uint64,uint64,uint8))`](processFullWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processFullWithdrawalReturn {
        pub _0: <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                IEigenPod::ValidatorInfo,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U40,
                alloy::sol_types::private::FixedBytes<32>,
                u64,
                alloy::sol_types::private::Address,
                u64,
                <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<processFullWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processFullWithdrawalCall) -> Self {
                    (
                        value.validatorIndex,
                        value.validatorPubkeyHash,
                        value.withdrawalHappenedTimestamp,
                        value.recipient,
                        value.withdrawalAmountGwei,
                        value.validatorInfo,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processFullWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndex: tuple.0,
                        validatorPubkeyHash: tuple.1,
                        withdrawalHappenedTimestamp: tuple.2,
                        recipient: tuple.3,
                        withdrawalAmountGwei: tuple.4,
                        validatorInfo: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<processFullWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processFullWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processFullWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processFullWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
                IEigenPod::ValidatorInfo,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processFullWithdrawalReturn;
            type ReturnTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processFullWithdrawal(uint40,bytes32,uint64,address,uint64,(uint64,uint64,uint64,uint8))";
            const SELECTOR: [u8; 4] = [202u8, 4u8, 78u8, 140u8];
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorPubkeyHash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalHappenedTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalAmountGwei),
                    <IEigenPod::ValidatorInfo as alloy_sol_types::SolType>::tokenize(
                        &self.validatorInfo,
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
    /**Function with signature `processPartialWithdrawal(uint40,uint64,address,uint64)` and selector `0x7bb3dbf3`.
```solidity
function processPartialWithdrawal(uint40 validatorIndex, uint64 withdrawalHappenedTimestamp, address recipient, uint64 withdrawalAmountGwei) external returns (IEigenPod.VerifiedWithdrawal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processPartialWithdrawalCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        pub withdrawalHappenedTimestamp: u64,
        pub recipient: alloy::sol_types::private::Address,
        pub withdrawalAmountGwei: u64,
    }
    ///Container type for the return parameters of the [`processPartialWithdrawal(uint40,uint64,address,uint64)`](processPartialWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct processPartialWithdrawalReturn {
        pub _0: <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U40,
                u64,
                alloy::sol_types::private::Address,
                u64,
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
            impl ::core::convert::From<processPartialWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: processPartialWithdrawalCall) -> Self {
                    (
                        value.validatorIndex,
                        value.withdrawalHappenedTimestamp,
                        value.recipient,
                        value.withdrawalAmountGwei,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processPartialWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndex: tuple.0,
                        withdrawalHappenedTimestamp: tuple.1,
                        recipient: tuple.2,
                        withdrawalAmountGwei: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<processPartialWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: processPartialWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for processPartialWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for processPartialWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = processPartialWithdrawalReturn;
            type ReturnTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "processPartialWithdrawal(uint40,uint64,address,uint64)";
            const SELECTOR: [u8; 4] = [123u8, 179u8, 219u8, 243u8];
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
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalHappenedTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalAmountGwei),
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
    /**Function with signature `provenWithdrawal(bytes32,uint64)` and selector `0x34bea20a`.
```solidity
function provenWithdrawal(bytes32, uint64) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct provenWithdrawalCall {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
        pub _1: u64,
    }
    ///Container type for the return parameters of the [`provenWithdrawal(bytes32,uint64)`](provenWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct provenWithdrawalReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u64,
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
            impl ::core::convert::From<provenWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: provenWithdrawalCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for provenWithdrawalCall {
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
            impl ::core::convert::From<provenWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: provenWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for provenWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for provenWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = provenWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "provenWithdrawal(bytes32,uint64)";
            const SELECTOR: [u8; 4] = [52u8, 190u8, 162u8, 10u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        64,
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
    /**Function with signature `recoverTokens(address[],uint256[],address)` and selector `0xdda3346c`.
```solidity
function recoverTokens(address[] memory tokenList, uint256[] memory amountsToWithdraw, address recipient) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recoverTokensCall {
        pub tokenList: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub amountsToWithdraw: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`recoverTokens(address[],uint256[],address)`](recoverTokensCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recoverTokensReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<recoverTokensCall> for UnderlyingRustTuple<'_> {
                fn from(value: recoverTokensCall) -> Self {
                    (value.tokenList, value.amountsToWithdraw, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recoverTokensCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        tokenList: tuple.0,
                        amountsToWithdraw: tuple.1,
                        recipient: tuple.2,
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
            impl ::core::convert::From<recoverTokensReturn> for UnderlyingRustTuple<'_> {
                fn from(value: recoverTokensReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for recoverTokensReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recoverTokensCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = recoverTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recoverTokens(address[],uint256[],address)";
            const SELECTOR: [u8; 4] = [221u8, 163u8, 52u8, 108u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.tokenList),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountsToWithdraw),
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
    /**Function with signature `setActiveValidatorCount(uint256)` and selector `0x73a97ee8`.
```solidity
function setActiveValidatorCount(uint256 _count) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setActiveValidatorCountCall {
        pub _count: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setActiveValidatorCount(uint256)`](setActiveValidatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setActiveValidatorCountReturn {}
    #[allow(
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
            impl ::core::convert::From<setActiveValidatorCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setActiveValidatorCountCall) -> Self {
                    (value._count,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setActiveValidatorCountCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _count: tuple.0 }
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
            impl ::core::convert::From<setActiveValidatorCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setActiveValidatorCountReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setActiveValidatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setActiveValidatorCountCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setActiveValidatorCountReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setActiveValidatorCount(uint256)";
            const SELECTOR: [u8; 4] = [115u8, 169u8, 126u8, 232u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._count),
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
    /**Function with signature `setMostRecentWithdrawalTimestamp(uint64)` and selector `0xd168eb51`.
```solidity
function setMostRecentWithdrawalTimestamp(uint64 _mostRecentWithdrawalTimestamp) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMostRecentWithdrawalTimestampCall {
        pub _mostRecentWithdrawalTimestamp: u64,
    }
    ///Container type for the return parameters of the [`setMostRecentWithdrawalTimestamp(uint64)`](setMostRecentWithdrawalTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMostRecentWithdrawalTimestampReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<setMostRecentWithdrawalTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMostRecentWithdrawalTimestampCall) -> Self {
                    (value._mostRecentWithdrawalTimestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMostRecentWithdrawalTimestampCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _mostRecentWithdrawalTimestamp: tuple.0,
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
            impl ::core::convert::From<setMostRecentWithdrawalTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMostRecentWithdrawalTimestampReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMostRecentWithdrawalTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMostRecentWithdrawalTimestampCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMostRecentWithdrawalTimestampReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMostRecentWithdrawalTimestamp(uint64)";
            const SELECTOR: [u8; 4] = [209u8, 104u8, 235u8, 81u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._mostRecentWithdrawalTimestamp,
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
    /**Function with signature `setValidatorRestakedBalance(bytes32,uint64)` and selector `0x5229564a`.
```solidity
function setValidatorRestakedBalance(bytes32 pkhash, uint64 restakedBalanceGwei) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidatorRestakedBalanceCall {
        pub pkhash: alloy::sol_types::private::FixedBytes<32>,
        pub restakedBalanceGwei: u64,
    }
    ///Container type for the return parameters of the [`setValidatorRestakedBalance(bytes32,uint64)`](setValidatorRestakedBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidatorRestakedBalanceReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u64,
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
            impl ::core::convert::From<setValidatorRestakedBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setValidatorRestakedBalanceCall) -> Self {
                    (value.pkhash, value.restakedBalanceGwei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setValidatorRestakedBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pkhash: tuple.0,
                        restakedBalanceGwei: tuple.1,
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
            impl ::core::convert::From<setValidatorRestakedBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setValidatorRestakedBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setValidatorRestakedBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setValidatorRestakedBalanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setValidatorRestakedBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setValidatorRestakedBalance(bytes32,uint64)";
            const SELECTOR: [u8; 4] = [82u8, 41u8, 86u8, 74u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.pkhash),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.restakedBalanceGwei),
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
    /**Function with signature `setValidatorStatus(bytes32,uint8)` and selector `0xd79ed726`.
```solidity
function setValidatorStatus(bytes32 pkhash, IEigenPod.VALIDATOR_STATUS status) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidatorStatusCall {
        pub pkhash: alloy::sol_types::private::FixedBytes<32>,
        pub status: <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setValidatorStatus(bytes32,uint8)`](setValidatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setValidatorStatusReturn {}
    #[allow(
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
                IEigenPod::VALIDATOR_STATUS,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setValidatorStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setValidatorStatusCall) -> Self {
                    (value.pkhash, value.status)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setValidatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pkhash: tuple.0,
                        status: tuple.1,
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
            impl ::core::convert::From<setValidatorStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setValidatorStatusReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setValidatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setValidatorStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                IEigenPod::VALIDATOR_STATUS,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setValidatorStatusReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setValidatorStatus(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [215u8, 158u8, 215u8, 38u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.pkhash),
                    <IEigenPod::VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(
                        &self.status,
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
    /**Function with signature `stake(bytes,bytes,bytes32)` and selector `0x9b4e4634`.
```solidity
function stake(bytes memory pubkey, bytes memory signature, bytes32 depositDataRoot) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeCall {
        pub pubkey: alloy::sol_types::private::Bytes,
        pub signature: alloy::sol_types::private::Bytes,
        pub depositDataRoot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`stake(bytes,bytes,bytes32)`](stakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<stakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeCall) -> Self {
                    (value.pubkey, value.signature, value.depositDataRoot)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pubkey: tuple.0,
                        signature: tuple.1,
                        depositDataRoot: tuple.2,
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
            impl ::core::convert::From<stakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stake(bytes,bytes,bytes32)";
            const SELECTOR: [u8; 4] = [155u8, 78u8, 70u8, 52u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.pubkey,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.depositDataRoot),
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
    /**Function with signature `sumOfPartialWithdrawalsClaimedGwei()` and selector `0x5d3f65b6`.
```solidity
function sumOfPartialWithdrawalsClaimedGwei() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sumOfPartialWithdrawalsClaimedGweiCall {}
    ///Container type for the return parameters of the [`sumOfPartialWithdrawalsClaimedGwei()`](sumOfPartialWithdrawalsClaimedGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct sumOfPartialWithdrawalsClaimedGweiReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<sumOfPartialWithdrawalsClaimedGweiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: sumOfPartialWithdrawalsClaimedGweiCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sumOfPartialWithdrawalsClaimedGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<sumOfPartialWithdrawalsClaimedGweiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: sumOfPartialWithdrawalsClaimedGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for sumOfPartialWithdrawalsClaimedGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for sumOfPartialWithdrawalsClaimedGweiCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = sumOfPartialWithdrawalsClaimedGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sumOfPartialWithdrawalsClaimedGwei()";
            const SELECTOR: [u8; 4] = [93u8, 63u8, 101u8, 182u8];
            #[inline]
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
```solidity
function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifactSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifactSelectors()";
            const SELECTOR: [u8; 4] = [102u8, 217u8, 169u8, 160u8];
            #[inline]
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
    /**Function with signature `targetArtifacts()` and selector `0x85226c81`.
```solidity
function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsCall {}
    ///Container type for the return parameters of the [`targetArtifacts()`](targetArtifactsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactsReturn {
        pub targetedArtifacts_: alloy::sol_types::private::Vec<
            alloy::sol_types::private::String,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<targetArtifactsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetArtifactsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedArtifacts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetArtifactsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetArtifacts()";
            const SELECTOR: [u8; 4] = [133u8, 34u8, 108u8, 129u8];
            #[inline]
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
    /**Function with signature `targetContracts()` and selector `0x3f7286f4`.
```solidity
function targetContracts() external view returns (address[] memory targetedContracts_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsCall {}
    ///Container type for the return parameters of the [`targetContracts()`](targetContractsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetContractsReturn {
        pub targetedContracts_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<targetContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<targetContractsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetContractsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedContracts_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetContractsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetContracts()";
            const SELECTOR: [u8; 4] = [63u8, 114u8, 134u8, 244u8];
            #[inline]
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
    /**Function with signature `targetSelectors()` and selector `0x916a17c6`.
```solidity
function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsCall {}
    ///Container type for the return parameters of the [`targetSelectors()`](targetSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSelectorsReturn {
        pub targetedSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSelectors()";
            const SELECTOR: [u8; 4] = [145u8, 106u8, 23u8, 198u8];
            #[inline]
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
    /**Function with signature `targetSenders()` and selector `0x3e5e3c23`.
```solidity
function targetSenders() external view returns (address[] memory targetedSenders_);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersCall {}
    ///Container type for the return parameters of the [`targetSenders()`](targetSendersCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetSendersReturn {
        pub targetedSenders_: alloy::sol_types::private::Vec<
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
            impl ::core::convert::From<targetSendersCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { targetedSenders_: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetSenders()";
            const SELECTOR: [u8; 4] = [62u8, 94u8, 60u8, 35u8];
            #[inline]
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
    /**Function with signature `validatorPubkeyHashToInfo(bytes32)` and selector `0x6fcd0e53`.
```solidity
function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPod.ValidatorInfo memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyHashToInfoCall {
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validatorPubkeyHashToInfo(bytes32)`](validatorPubkeyHashToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyHashToInfoReturn {
        pub _0: <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorPubkeyHashToInfoCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoCall) -> Self {
                    (value.validatorPubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorPubkeyHashToInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkeyHash: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorPubkeyHashToInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorPubkeyHashToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyHashToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorPubkeyHashToInfoReturn;
            type ReturnTuple<'a> = (IEigenPod::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorPubkeyHashToInfo(bytes32)";
            const SELECTOR: [u8; 4] = [111u8, 205u8, 14u8, 83u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorPubkeyHash),
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
    /**Function with signature `validatorPubkeyToInfo(bytes)` and selector `0xb522538a`.
```solidity
function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPod.ValidatorInfo memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyToInfoCall {
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`validatorPubkeyToInfo(bytes)`](validatorPubkeyToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyToInfoReturn {
        pub _0: <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyToInfoCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoCall) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorPubkeyToInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorPubkey: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorPubkeyToInfoReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorPubkeyToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorPubkeyToInfoReturn;
            type ReturnTuple<'a> = (IEigenPod::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorPubkeyToInfo(bytes)";
            const SELECTOR: [u8; 4] = [181u8, 34u8, 83u8, 138u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorPubkey,
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
    /**Function with signature `validatorStatus(bytes)` and selector `0x58eaee79`.
```solidity
function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPod.VALIDATOR_STATUS);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_0Call {
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`validatorStatus(bytes)`](validatorStatus_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_0Return {
        pub _0: <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Call) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorStatus_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorPubkey: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorStatus_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorStatus_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorStatus_0Return;
            type ReturnTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorStatus(bytes)";
            const SELECTOR: [u8; 4] = [88u8, 234u8, 238u8, 121u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorPubkey,
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
    /**Function with signature `validatorStatus(bytes32)` and selector `0x7439841f`.
```solidity
function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPod.VALIDATOR_STATUS);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_1Call {
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validatorStatus(bytes32)`](validatorStatus_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_1Return {
        pub _0: <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorStatus_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Call) -> Self {
                    (value.pubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorStatus_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { pubkeyHash: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<validatorStatus_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for validatorStatus_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorStatus_1Return;
            type ReturnTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "validatorStatus(bytes32)";
            const SELECTOR: [u8; 4] = [116u8, 57u8, 132u8, 31u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
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
    /**Function with signature `verifyAndProcessWithdrawal(bytes32,(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32),bytes,bytes32[],bytes32[])` and selector `0x115cd5e4`.
```solidity
function verifyAndProcessWithdrawal(bytes32 beaconStateRoot, BeaconChainProofs.WithdrawalProof memory withdrawalProof, bytes memory validatorFieldsProof, bytes32[] memory validatorFields, bytes32[] memory withdrawalFields) external returns (IEigenPod.VerifiedWithdrawal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyAndProcessWithdrawalCall {
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        pub withdrawalProof: <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
        pub validatorFieldsProof: alloy::sol_types::private::Bytes,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        pub withdrawalFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`verifyAndProcessWithdrawal(bytes32,(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32),bytes,bytes32[],bytes32[])`](verifyAndProcessWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyAndProcessWithdrawalReturn {
        pub _0: <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                BeaconChainProofs::WithdrawalProof,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<verifyAndProcessWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyAndProcessWithdrawalCall) -> Self {
                    (
                        value.beaconStateRoot,
                        value.withdrawalProof,
                        value.validatorFieldsProof,
                        value.validatorFields,
                        value.withdrawalFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyAndProcessWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconStateRoot: tuple.0,
                        withdrawalProof: tuple.1,
                        validatorFieldsProof: tuple.2,
                        validatorFields: tuple.3,
                        withdrawalFields: tuple.4,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPod::VerifiedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyAndProcessWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyAndProcessWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyAndProcessWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyAndProcessWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BeaconChainProofs::WithdrawalProof,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyAndProcessWithdrawalReturn;
            type ReturnTuple<'a> = (IEigenPod::VerifiedWithdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyAndProcessWithdrawal(bytes32,(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32),bytes,bytes32[],bytes32[])";
            const SELECTOR: [u8; 4] = [17u8, 92u8, 213u8, 228u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconStateRoot),
                    <BeaconChainProofs::WithdrawalProof as alloy_sol_types::SolType>::tokenize(
                        &self.withdrawalProof,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalFields),
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
    /**Function with signature `verifyAndProcessWithdrawals(uint64,(bytes32,bytes),(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)[],bytes[],bytes32[][],bytes32[][])` and selector `0xe251ef52`.
```solidity
function verifyAndProcessWithdrawals(uint64 oracleTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.WithdrawalProof[] memory withdrawalProofs, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields, bytes32[][] memory withdrawalFields) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyAndProcessWithdrawalsCall {
        pub oracleTimestamp: u64,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub withdrawalProofs: alloy::sol_types::private::Vec<
            <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
        >,
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
        pub withdrawalFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`verifyAndProcessWithdrawals(uint64,(bytes32,bytes),(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)[],bytes[],bytes32[][],bytes32[][])`](verifyAndProcessWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyAndProcessWithdrawalsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<BeaconChainProofs::WithdrawalProof>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::FixedBytes<32>,
                    >,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyAndProcessWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyAndProcessWithdrawalsCall) -> Self {
                    (
                        value.oracleTimestamp,
                        value.stateRootProof,
                        value.withdrawalProofs,
                        value.validatorFieldsProofs,
                        value.validatorFields,
                        value.withdrawalFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyAndProcessWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        oracleTimestamp: tuple.0,
                        stateRootProof: tuple.1,
                        withdrawalProofs: tuple.2,
                        validatorFieldsProofs: tuple.3,
                        validatorFields: tuple.4,
                        withdrawalFields: tuple.5,
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
            impl ::core::convert::From<verifyAndProcessWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyAndProcessWithdrawalsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyAndProcessWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyAndProcessWithdrawalsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<BeaconChainProofs::WithdrawalProof>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyAndProcessWithdrawalsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyAndProcessWithdrawals(uint64,(bytes32,bytes),(bytes,bytes,bytes,bytes,bytes,uint64,uint64,uint64,bytes32,bytes32,bytes32,bytes32)[],bytes[],bytes32[][],bytes32[][])";
            const SELECTOR: [u8; 4] = [226u8, 81u8, 239u8, 82u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleTimestamp),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::WithdrawalProof,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalProofs),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProofs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.withdrawalFields),
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
    /**Function with signature `verifyBalanceUpdate(uint64,uint40,bytes32,bytes,bytes32[],uint64)` and selector `0x3b4c57c6`.
```solidity
function verifyBalanceUpdate(uint64 oracleTimestamp, uint40 validatorIndex, bytes32 beaconStateRoot, bytes memory validatorFieldsProofs, bytes32[] memory validatorFields, uint64 mostRecentBalanceUpdateTimestamp) external returns (int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceUpdateCall {
        pub oracleTimestamp: u64,
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        pub validatorFieldsProofs: alloy::sol_types::private::Bytes,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        pub mostRecentBalanceUpdateTimestamp: u64,
    }
    ///Container type for the return parameters of the [`verifyBalanceUpdate(uint64,uint40,bytes32,bytes,bytes32[],uint64)`](verifyBalanceUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceUpdateReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::I256,
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::primitives::aliases::U40,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                u64,
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
            impl ::core::convert::From<verifyBalanceUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceUpdateCall) -> Self {
                    (
                        value.oracleTimestamp,
                        value.validatorIndex,
                        value.beaconStateRoot,
                        value.validatorFieldsProofs,
                        value.validatorFields,
                        value.mostRecentBalanceUpdateTimestamp,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        oracleTimestamp: tuple.0,
                        validatorIndex: tuple.1,
                        beaconStateRoot: tuple.2,
                        validatorFieldsProofs: tuple.3,
                        validatorFields: tuple.4,
                        mostRecentBalanceUpdateTimestamp: tuple.5,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<verifyBalanceUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyBalanceUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyBalanceUpdateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyBalanceUpdate(uint64,uint40,bytes32,bytes,bytes32[],uint64)";
            const SELECTOR: [u8; 4] = [59u8, 76u8, 87u8, 198u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleTimestamp),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconStateRoot),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProofs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.mostRecentBalanceUpdateTimestamp,
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
    /**Function with signature `verifyBalanceUpdates(uint64,uint40[],(bytes32,bytes),bytes[],bytes32[][])` and selector `0xa50600f4`.
```solidity
function verifyBalanceUpdates(uint64 oracleTimestamp, uint40[] memory validatorIndices, BeaconChainProofs.StateRootProof memory stateRootProof, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceUpdatesCall {
        pub oracleTimestamp: u64,
        pub validatorIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`verifyBalanceUpdates(uint64,uint40[],(bytes32,bytes),bytes[],bytes32[][])`](verifyBalanceUpdatesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceUpdatesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
                >,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyBalanceUpdatesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceUpdatesCall) -> Self {
                    (
                        value.oracleTimestamp,
                        value.validatorIndices,
                        value.stateRootProof,
                        value.validatorFieldsProofs,
                        value.validatorFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceUpdatesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        oracleTimestamp: tuple.0,
                        validatorIndices: tuple.1,
                        stateRootProof: tuple.2,
                        validatorFieldsProofs: tuple.3,
                        validatorFields: tuple.4,
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
            impl ::core::convert::From<verifyBalanceUpdatesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceUpdatesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceUpdatesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyBalanceUpdatesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyBalanceUpdatesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyBalanceUpdates(uint64,uint40[],(bytes32,bytes),bytes[],bytes32[][])";
            const SELECTOR: [u8; 4] = [165u8, 6u8, 0u8, 244u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleTimestamp),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndices),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProofs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
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
    /**Function with signature `verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])` and selector `0x3f65cf19`.
```solidity
function verifyWithdrawalCredentials(uint64 oracleTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentials_0Call {
        pub oracleTimestamp: u64,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        >,
    }
    ///Container type for the return parameters of the [`verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])`](verifyWithdrawalCredentials_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentials_0Return {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
                >,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<verifyWithdrawalCredentials_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentials_0Call) -> Self {
                    (
                        value.oracleTimestamp,
                        value.stateRootProof,
                        value.validatorIndices,
                        value.validatorFieldsProofs,
                        value.validatorFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentials_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        oracleTimestamp: tuple.0,
                        stateRootProof: tuple.1,
                        validatorIndices: tuple.2,
                        validatorFieldsProofs: tuple.3,
                        validatorFields: tuple.4,
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
            impl ::core::convert::From<verifyWithdrawalCredentials_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentials_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentials_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentials_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Bytes>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentials_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])";
            const SELECTOR: [u8; 4] = [63u8, 101u8, 207u8, 25u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleTimestamp),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProofs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
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
    /**Function with signature `verifyWithdrawalCredentials(uint64,bytes32,uint40,bytes,bytes32[])` and selector `0x816d53f9`.
```solidity
function verifyWithdrawalCredentials(uint64 oracleTimestamp, bytes32 beaconStateRoot, uint40 validatorIndex, bytes memory validatorFieldsProof, bytes32[] memory validatorFields) external returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentials_1Call {
        pub oracleTimestamp: u64,
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        pub validatorFieldsProof: alloy::sol_types::private::Bytes,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`verifyWithdrawalCredentials(uint64,bytes32,uint40,bytes,bytes32[])`](verifyWithdrawalCredentials_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentials_1Return {
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
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U40,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<verifyWithdrawalCredentials_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentials_1Call) -> Self {
                    (
                        value.oracleTimestamp,
                        value.beaconStateRoot,
                        value.validatorIndex,
                        value.validatorFieldsProof,
                        value.validatorFields,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentials_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        oracleTimestamp: tuple.0,
                        beaconStateRoot: tuple.1,
                        validatorIndex: tuple.2,
                        validatorFieldsProof: tuple.3,
                        validatorFields: tuple.4,
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
            impl ::core::convert::From<verifyWithdrawalCredentials_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentials_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentials_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentials_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<40>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentials_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyWithdrawalCredentials(uint64,bytes32,uint40,bytes,bytes32[])";
            const SELECTOR: [u8; 4] = [129u8, 109u8, 83u8, 249u8];
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
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.oracleTimestamp),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconStateRoot),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
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
    /**Function with signature `withdrawBeforeRestaking()` and selector `0xbaa7145a`.
```solidity
function withdrawBeforeRestaking() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBeforeRestakingCall {}
    ///Container type for the return parameters of the [`withdrawBeforeRestaking()`](withdrawBeforeRestakingCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawBeforeRestakingReturn {}
    #[allow(
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
            impl ::core::convert::From<withdrawBeforeRestakingCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBeforeRestakingCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBeforeRestakingCall {
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
            impl ::core::convert::From<withdrawBeforeRestakingReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawBeforeRestakingReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawBeforeRestakingReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawBeforeRestakingCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawBeforeRestakingReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawBeforeRestaking()";
            const SELECTOR: [u8; 4] = [186u8, 167u8, 20u8, 90u8];
            #[inline]
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
    /**Function with signature `withdrawNonBeaconChainETHBalanceWei(address,uint256)` and selector `0xe2c83445`.
```solidity
function withdrawNonBeaconChainETHBalanceWei(address recipient, uint256 amountToWithdraw) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawNonBeaconChainETHBalanceWeiCall {
        pub recipient: alloy::sol_types::private::Address,
        pub amountToWithdraw: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawNonBeaconChainETHBalanceWei(address,uint256)`](withdrawNonBeaconChainETHBalanceWeiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawNonBeaconChainETHBalanceWeiReturn {}
    #[allow(
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
            impl ::core::convert::From<withdrawNonBeaconChainETHBalanceWeiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawNonBeaconChainETHBalanceWeiCall) -> Self {
                    (value.recipient, value.amountToWithdraw)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawNonBeaconChainETHBalanceWeiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        amountToWithdraw: tuple.1,
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
            impl ::core::convert::From<withdrawNonBeaconChainETHBalanceWeiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawNonBeaconChainETHBalanceWeiReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawNonBeaconChainETHBalanceWeiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawNonBeaconChainETHBalanceWeiCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawNonBeaconChainETHBalanceWeiReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawNonBeaconChainETHBalanceWei(address,uint256)";
            const SELECTOR: [u8; 4] = [226u8, 200u8, 52u8, 69u8];
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
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountToWithdraw),
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
    /**Function with signature `withdrawRestakedBeaconChainETH(address,uint256)` and selector `0xc4907442`.
```solidity
function withdrawRestakedBeaconChainETH(address recipient, uint256 amountWei) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRestakedBeaconChainETHCall {
        pub recipient: alloy::sol_types::private::Address,
        pub amountWei: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`withdrawRestakedBeaconChainETH(address,uint256)`](withdrawRestakedBeaconChainETHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawRestakedBeaconChainETHReturn {}
    #[allow(
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
            impl ::core::convert::From<withdrawRestakedBeaconChainETHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHCall) -> Self {
                    (value.recipient, value.amountWei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawRestakedBeaconChainETHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        amountWei: tuple.1,
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
            impl ::core::convert::From<withdrawRestakedBeaconChainETHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawRestakedBeaconChainETHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawRestakedBeaconChainETHCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawRestakedBeaconChainETHReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawRestakedBeaconChainETH(address,uint256)";
            const SELECTOR: [u8; 4] = [196u8, 144u8, 116u8, 66u8];
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
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountWei),
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
    /**Function with signature `withdrawableRestakedExecutionLayerGwei()` and selector `0x3474aa16`.
```solidity
function withdrawableRestakedExecutionLayerGwei() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableRestakedExecutionLayerGweiCall {}
    ///Container type for the return parameters of the [`withdrawableRestakedExecutionLayerGwei()`](withdrawableRestakedExecutionLayerGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawableRestakedExecutionLayerGweiReturn {
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
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableRestakedExecutionLayerGweiCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawableRestakedExecutionLayerGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableRestakedExecutionLayerGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawableRestakedExecutionLayerGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawableRestakedExecutionLayerGweiCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawableRestakedExecutionLayerGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawableRestakedExecutionLayerGwei()";
            const SELECTOR: [u8; 4] = [52u8, 116u8, 170u8, 22u8];
            #[inline]
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
    ///Container for all the [`EPInternalFunctions`](self) function calls.
    pub enum EPInternalFunctionsCalls {
        GENESIS_TIME(GENESIS_TIMECall),
        IS_TEST(IS_TESTCall),
        MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(
            MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall,
        ),
        activateRestaking(activateRestakingCall),
        delayedWithdrawalRouter(delayedWithdrawalRouterCall),
        eigenPodManager(eigenPodManagerCall),
        ethPOS(ethPOSCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        getActiveValidatorCount(getActiveValidatorCountCall),
        hasRestaked(hasRestakedCall),
        initialize(initializeCall),
        mostRecentWithdrawalTimestamp(mostRecentWithdrawalTimestampCall),
        nonBeaconChainETHBalanceWei(nonBeaconChainETHBalanceWeiCall),
        podOwner(podOwnerCall),
        processFullWithdrawal(processFullWithdrawalCall),
        processPartialWithdrawal(processPartialWithdrawalCall),
        provenWithdrawal(provenWithdrawalCall),
        recoverTokens(recoverTokensCall),
        setActiveValidatorCount(setActiveValidatorCountCall),
        setMostRecentWithdrawalTimestamp(setMostRecentWithdrawalTimestampCall),
        setValidatorRestakedBalance(setValidatorRestakedBalanceCall),
        setValidatorStatus(setValidatorStatusCall),
        stake(stakeCall),
        sumOfPartialWithdrawalsClaimedGwei(sumOfPartialWithdrawalsClaimedGweiCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        validatorPubkeyHashToInfo(validatorPubkeyHashToInfoCall),
        validatorPubkeyToInfo(validatorPubkeyToInfoCall),
        validatorStatus_0(validatorStatus_0Call),
        validatorStatus_1(validatorStatus_1Call),
        verifyAndProcessWithdrawal(verifyAndProcessWithdrawalCall),
        verifyAndProcessWithdrawals(verifyAndProcessWithdrawalsCall),
        verifyBalanceUpdate(verifyBalanceUpdateCall),
        verifyBalanceUpdates(verifyBalanceUpdatesCall),
        verifyWithdrawalCredentials_0(verifyWithdrawalCredentials_0Call),
        verifyWithdrawalCredentials_1(verifyWithdrawalCredentials_1Call),
        withdrawBeforeRestaking(withdrawBeforeRestakingCall),
        withdrawNonBeaconChainETHBalanceWei(withdrawNonBeaconChainETHBalanceWeiCall),
        withdrawRestakedBeaconChainETH(withdrawRestakedBeaconChainETHCall),
        withdrawableRestakedExecutionLayerGwei(
            withdrawableRestakedExecutionLayerGweiCall,
        ),
    }
    #[automatically_derived]
    impl EPInternalFunctionsCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [11u8, 24u8, 255u8, 102u8],
            [12u8, 212u8, 100u8, 158u8],
            [17u8, 92u8, 213u8, 228u8],
            [26u8, 80u8, 87u8, 190u8],
            [29u8, 144u8, 93u8, 92u8],
            [30u8, 215u8, 131u8, 28u8],
            [49u8, 6u8, 171u8, 83u8],
            [52u8, 116u8, 170u8, 22u8],
            [52u8, 190u8, 162u8, 10u8],
            [55u8, 222u8, 234u8, 112u8],
            [59u8, 76u8, 87u8, 198u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 101u8, 207u8, 25u8],
            [63u8, 114u8, 134u8, 244u8],
            [70u8, 101u8, 188u8, 218u8],
            [82u8, 41u8, 86u8, 74u8],
            [88u8, 234u8, 238u8, 121u8],
            [93u8, 63u8, 101u8, 182u8],
            [102u8, 217u8, 169u8, 160u8],
            [111u8, 205u8, 14u8, 83u8],
            [115u8, 169u8, 126u8, 232u8],
            [116u8, 57u8, 132u8, 31u8],
            [116u8, 205u8, 215u8, 152u8],
            [123u8, 179u8, 219u8, 243u8],
            [129u8, 109u8, 83u8, 249u8],
            [133u8, 34u8, 108u8, 129u8],
            [135u8, 224u8, 210u8, 137u8],
            [145u8, 106u8, 23u8, 198u8],
            [155u8, 78u8, 70u8, 52u8],
            [165u8, 6u8, 0u8, 244u8],
            [181u8, 34u8, 83u8, 138u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [186u8, 167u8, 20u8, 90u8],
            [196u8, 144u8, 116u8, 66u8],
            [196u8, 214u8, 109u8, 232u8],
            [202u8, 4u8, 78u8, 140u8],
            [209u8, 104u8, 235u8, 81u8],
            [215u8, 158u8, 215u8, 38u8],
            [221u8, 163u8, 52u8, 108u8],
            [226u8, 12u8, 159u8, 113u8],
            [226u8, 81u8, 239u8, 82u8],
            [226u8, 200u8, 52u8, 69u8],
            [242u8, 136u8, 36u8, 97u8],
            [250u8, 118u8, 38u8, 212u8],
            [254u8, 128u8, 176u8, 135u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EPInternalFunctionsCalls {
        const NAME: &'static str = "EPInternalFunctionsCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 46usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GENESIS_TIME(_) => {
                    <GENESIS_TIMECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(_) => {
                    <MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::activateRestaking(_) => {
                    <activateRestakingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delayedWithdrawalRouter(_) => {
                    <delayedWithdrawalRouterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ethPOS(_) => <ethPOSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getActiveValidatorCount(_) => {
                    <getActiveValidatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::hasRestaked(_) => {
                    <hasRestakedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mostRecentWithdrawalTimestamp(_) => {
                    <mostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nonBeaconChainETHBalanceWei(_) => {
                    <nonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::podOwner(_) => <podOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::processFullWithdrawal(_) => {
                    <processFullWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::processPartialWithdrawal(_) => {
                    <processPartialWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::provenWithdrawal(_) => {
                    <provenWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recoverTokens(_) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setActiveValidatorCount(_) => {
                    <setActiveValidatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMostRecentWithdrawalTimestamp(_) => {
                    <setMostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setValidatorRestakedBalance(_) => {
                    <setValidatorRestakedBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setValidatorStatus(_) => {
                    <setValidatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stake(_) => <stakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sumOfPartialWithdrawalsClaimedGwei(_) => {
                    <sumOfPartialWithdrawalsClaimedGweiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifactSelectors(_) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetArtifacts(_) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetContracts(_) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorPubkeyHashToInfo(_) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorPubkeyToInfo(_) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorStatus_0(_) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::validatorStatus_1(_) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyAndProcessWithdrawal(_) => {
                    <verifyAndProcessWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyAndProcessWithdrawals(_) => {
                    <verifyAndProcessWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyBalanceUpdate(_) => {
                    <verifyBalanceUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyBalanceUpdates(_) => {
                    <verifyBalanceUpdatesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyWithdrawalCredentials_0(_) => {
                    <verifyWithdrawalCredentials_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyWithdrawalCredentials_1(_) => {
                    <verifyWithdrawalCredentials_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawBeforeRestaking(_) => {
                    <withdrawBeforeRestakingCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawNonBeaconChainETHBalanceWei(_) => {
                    <withdrawNonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawRestakedBeaconChainETH(_) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawableRestakedExecutionLayerGwei(_) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EPInternalFunctionsCalls>] = &[
                {
                    fn podOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <podOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::podOwner)
                    }
                    podOwner
                },
                {
                    fn activateRestaking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <activateRestakingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::activateRestaking)
                    }
                    activateRestaking
                },
                {
                    fn verifyAndProcessWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyAndProcessWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyAndProcessWithdrawal)
                    }
                    verifyAndProcessWithdrawal
                },
                {
                    fn delayedWithdrawalRouter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <delayedWithdrawalRouterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::delayedWithdrawalRouter)
                    }
                    delayedWithdrawalRouter
                },
                {
                    fn MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
                            )
                    }
                    MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn hasRestaked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <hasRestakedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::hasRestaked)
                    }
                    hasRestaked
                },
                {
                    fn withdrawableRestakedExecutionLayerGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::withdrawableRestakedExecutionLayerGwei,
                            )
                    }
                    withdrawableRestakedExecutionLayerGwei
                },
                {
                    fn provenWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <provenWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::provenWithdrawal)
                    }
                    provenWithdrawal
                },
                {
                    fn getActiveValidatorCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <getActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::getActiveValidatorCount)
                    }
                    getActiveValidatorCount
                },
                {
                    fn verifyBalanceUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyBalanceUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyBalanceUpdate)
                    }
                    verifyBalanceUpdate
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn verifyWithdrawalCredentials_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyWithdrawalCredentials_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyWithdrawalCredentials_0)
                    }
                    verifyWithdrawalCredentials_0
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn setValidatorRestakedBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <setValidatorRestakedBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::setValidatorRestakedBalance)
                    }
                    setValidatorRestakedBalance
                },
                {
                    fn validatorStatus_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::validatorStatus_0)
                    }
                    validatorStatus_0
                },
                {
                    fn sumOfPartialWithdrawalsClaimedGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <sumOfPartialWithdrawalsClaimedGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::sumOfPartialWithdrawalsClaimedGwei,
                            )
                    }
                    sumOfPartialWithdrawalsClaimedGwei
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn validatorPubkeyHashToInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::validatorPubkeyHashToInfo)
                    }
                    validatorPubkeyHashToInfo
                },
                {
                    fn setActiveValidatorCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <setActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::setActiveValidatorCount)
                    }
                    setActiveValidatorCount
                },
                {
                    fn validatorStatus_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::validatorStatus_1)
                    }
                    validatorStatus_1
                },
                {
                    fn ethPOS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <ethPOSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::ethPOS)
                    }
                    ethPOS
                },
                {
                    fn processPartialWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <processPartialWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::processPartialWithdrawal)
                    }
                    processPartialWithdrawal
                },
                {
                    fn verifyWithdrawalCredentials_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyWithdrawalCredentials_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyWithdrawalCredentials_1)
                    }
                    verifyWithdrawalCredentials_1
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn mostRecentWithdrawalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <mostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::mostRecentWithdrawalTimestamp)
                    }
                    mostRecentWithdrawalTimestamp
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn stake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <stakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::stake)
                    }
                    stake
                },
                {
                    fn verifyBalanceUpdates(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyBalanceUpdatesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyBalanceUpdates)
                    }
                    verifyBalanceUpdates
                },
                {
                    fn validatorPubkeyToInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::validatorPubkeyToInfo)
                    }
                    validatorPubkeyToInfo
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::failed)
                    }
                    failed
                },
                {
                    fn withdrawBeforeRestaking(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <withdrawBeforeRestakingCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::withdrawBeforeRestaking)
                    }
                    withdrawBeforeRestaking
                },
                {
                    fn withdrawRestakedBeaconChainETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::withdrawRestakedBeaconChainETH,
                            )
                    }
                    withdrawRestakedBeaconChainETH
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::initialize)
                    }
                    initialize
                },
                {
                    fn processFullWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <processFullWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::processFullWithdrawal)
                    }
                    processFullWithdrawal
                },
                {
                    fn setMostRecentWithdrawalTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <setMostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::setMostRecentWithdrawalTimestamp,
                            )
                    }
                    setMostRecentWithdrawalTimestamp
                },
                {
                    fn setValidatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <setValidatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::setValidatorStatus)
                    }
                    setValidatorStatus
                },
                {
                    fn recoverTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <recoverTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::recoverTokens)
                    }
                    recoverTokens
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn verifyAndProcessWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <verifyAndProcessWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::verifyAndProcessWithdrawals)
                    }
                    verifyAndProcessWithdrawals
                },
                {
                    fn withdrawNonBeaconChainETHBalanceWei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <withdrawNonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                EPInternalFunctionsCalls::withdrawNonBeaconChainETHBalanceWei,
                            )
                    }
                    withdrawNonBeaconChainETHBalanceWei
                },
                {
                    fn GENESIS_TIME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <GENESIS_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::GENESIS_TIME)
                    }
                    GENESIS_TIME
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::IS_TEST)
                    }
                    IS_TEST
                },
                {
                    fn nonBeaconChainETHBalanceWei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EPInternalFunctionsCalls> {
                        <nonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EPInternalFunctionsCalls::nonBeaconChainETHBalanceWei)
                    }
                    nonBeaconChainETHBalanceWei
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
                Self::GENESIS_TIME(inner) => {
                    <GENESIS_TIMECall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(inner) => {
                    <MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::activateRestaking(inner) => {
                    <activateRestakingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delayedWithdrawalRouter(inner) => {
                    <delayedWithdrawalRouterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ethPOS(inner) => {
                    <ethPOSCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getActiveValidatorCount(inner) => {
                    <getActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::hasRestaked(inner) => {
                    <hasRestakedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mostRecentWithdrawalTimestamp(inner) => {
                    <mostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nonBeaconChainETHBalanceWei(inner) => {
                    <nonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::podOwner(inner) => {
                    <podOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::processFullWithdrawal(inner) => {
                    <processFullWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::processPartialWithdrawal(inner) => {
                    <processPartialWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::provenWithdrawal(inner) => {
                    <provenWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recoverTokens(inner) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setActiveValidatorCount(inner) => {
                    <setActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMostRecentWithdrawalTimestamp(inner) => {
                    <setMostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setValidatorRestakedBalance(inner) => {
                    <setValidatorRestakedBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setValidatorStatus(inner) => {
                    <setValidatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sumOfPartialWithdrawalsClaimedGwei(inner) => {
                    <sumOfPartialWithdrawalsClaimedGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorPubkeyHashToInfo(inner) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorPubkeyToInfo(inner) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorStatus_0(inner) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::validatorStatus_1(inner) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyAndProcessWithdrawal(inner) => {
                    <verifyAndProcessWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyAndProcessWithdrawals(inner) => {
                    <verifyAndProcessWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyBalanceUpdate(inner) => {
                    <verifyBalanceUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyBalanceUpdates(inner) => {
                    <verifyBalanceUpdatesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyWithdrawalCredentials_0(inner) => {
                    <verifyWithdrawalCredentials_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyWithdrawalCredentials_1(inner) => {
                    <verifyWithdrawalCredentials_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawBeforeRestaking(inner) => {
                    <withdrawBeforeRestakingCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawNonBeaconChainETHBalanceWei(inner) => {
                    <withdrawNonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawRestakedBeaconChainETH(inner) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawableRestakedExecutionLayerGwei(inner) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::GENESIS_TIME(inner) => {
                    <GENESIS_TIMECall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(inner) => {
                    <MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::activateRestaking(inner) => {
                    <activateRestakingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::delayedWithdrawalRouter(inner) => {
                    <delayedWithdrawalRouterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ethPOS(inner) => {
                    <ethPOSCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getActiveValidatorCount(inner) => {
                    <getActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::hasRestaked(inner) => {
                    <hasRestakedCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::mostRecentWithdrawalTimestamp(inner) => {
                    <mostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nonBeaconChainETHBalanceWei(inner) => {
                    <nonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::podOwner(inner) => {
                    <podOwnerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processFullWithdrawal(inner) => {
                    <processFullWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::processPartialWithdrawal(inner) => {
                    <processPartialWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::provenWithdrawal(inner) => {
                    <provenWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::recoverTokens(inner) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setActiveValidatorCount(inner) => {
                    <setActiveValidatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMostRecentWithdrawalTimestamp(inner) => {
                    <setMostRecentWithdrawalTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setValidatorRestakedBalance(inner) => {
                    <setValidatorRestakedBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setValidatorStatus(inner) => {
                    <setValidatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sumOfPartialWithdrawalsClaimedGwei(inner) => {
                    <sumOfPartialWithdrawalsClaimedGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorPubkeyHashToInfo(inner) => {
                    <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorPubkeyToInfo(inner) => {
                    <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorStatus_0(inner) => {
                    <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::validatorStatus_1(inner) => {
                    <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyAndProcessWithdrawal(inner) => {
                    <verifyAndProcessWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyAndProcessWithdrawals(inner) => {
                    <verifyAndProcessWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyBalanceUpdate(inner) => {
                    <verifyBalanceUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyBalanceUpdates(inner) => {
                    <verifyBalanceUpdatesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyWithdrawalCredentials_0(inner) => {
                    <verifyWithdrawalCredentials_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyWithdrawalCredentials_1(inner) => {
                    <verifyWithdrawalCredentials_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawBeforeRestaking(inner) => {
                    <withdrawBeforeRestakingCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawNonBeaconChainETHBalanceWei(inner) => {
                    <withdrawNonBeaconChainETHBalanceWeiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawRestakedBeaconChainETH(inner) => {
                    <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawableRestakedExecutionLayerGwei(inner) => {
                    <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EPInternalFunctions`](self) events.
    pub enum EPInternalFunctionsEvents {
        EigenPodStaked(EigenPodStaked),
        FullWithdrawalRedeemed(FullWithdrawalRedeemed),
        Initialized(Initialized),
        NonBeaconChainETHReceived(NonBeaconChainETHReceived),
        NonBeaconChainETHWithdrawn(NonBeaconChainETHWithdrawn),
        PartialWithdrawalRedeemed(PartialWithdrawalRedeemed),
        RestakedBeaconChainETHWithdrawn(RestakedBeaconChainETHWithdrawn),
        RestakingActivated(RestakingActivated),
        ValidatorBalanceUpdated(ValidatorBalanceUpdated),
        ValidatorRestaked(ValidatorRestaked),
        log(log),
        log_address(log_address),
        log_array_0(log_array_0),
        log_array_1(log_array_1),
        log_array_2(log_array_2),
        log_bytes(log_bytes),
        log_bytes32(log_bytes32),
        log_int(log_int),
        log_named_address(log_named_address),
        log_named_array_0(log_named_array_0),
        log_named_array_1(log_named_array_1),
        log_named_array_2(log_named_array_2),
        log_named_bytes(log_named_bytes),
        log_named_bytes32(log_named_bytes32),
        log_named_decimal_int(log_named_decimal_int),
        log_named_decimal_uint(log_named_decimal_uint),
        log_named_int(log_named_int),
        log_named_string(log_named_string),
        log_named_uint(log_named_uint),
        log_string(log_string),
        log_uint(log_uint),
        logs(logs),
    }
    #[automatically_derived]
    impl EPInternalFunctionsEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                0u8,
                170u8,
                163u8,
                156u8,
                159u8,
                251u8,
                95u8,
                86u8,
                122u8,
                69u8,
                52u8,
                56u8,
                12u8,
                115u8,
                112u8,
                117u8,
                112u8,
                46u8,
                31u8,
                127u8,
                20u8,
                16u8,
                127u8,
                201u8,
                83u8,
                40u8,
                227u8,
                181u8,
                108u8,
                3u8,
                37u8,
                251u8,
            ],
            [
                11u8,
                46u8,
                19u8,
                255u8,
                32u8,
                172u8,
                123u8,
                71u8,
                65u8,
                152u8,
                101u8,
                85u8,
                131u8,
                237u8,
                247u8,
                13u8,
                237u8,
                210u8,
                193u8,
                220u8,
                152u8,
                14u8,
                50u8,
                156u8,
                79u8,
                187u8,
                47u8,
                192u8,
                116u8,
                139u8,
                121u8,
                107u8,
            ],
            [
                14u8,
                95u8,
                172u8,
                23u8,
                91u8,
                131u8,
                23u8,
                124u8,
                192u8,
                71u8,
                56u8,
                30u8,
                3u8,
                13u8,
                143u8,
                179u8,
                180u8,
                43u8,
                55u8,
                189u8,
                28u8,
                2u8,
                94u8,
                34u8,
                194u8,
                128u8,
                250u8,
                202u8,
                214u8,
                44u8,
                50u8,
                223u8,
            ],
            [
                14u8,
                181u8,
                213u8,
                38u8,
                36u8,
                200u8,
                210u8,
                138u8,
                218u8,
                159u8,
                197u8,
                90u8,
                140u8,
                80u8,
                46u8,
                213u8,
                170u8,
                63u8,
                190u8,
                47u8,
                182u8,
                233u8,
                27u8,
                113u8,
                181u8,
                243u8,
                118u8,
                136u8,
                43u8,
                29u8,
                47u8,
                184u8,
            ],
            [
                35u8,
                182u8,
                42u8,
                208u8,
                88u8,
                77u8,
                36u8,
                167u8,
                95u8,
                11u8,
                243u8,
                86u8,
                3u8,
                145u8,
                239u8,
                86u8,
                89u8,
                236u8,
                109u8,
                177u8,
                38u8,
                156u8,
                86u8,
                225u8,
                26u8,
                162u8,
                65u8,
                214u8,
                55u8,
                241u8,
                155u8,
                32u8,
            ],
            [
                40u8,
                15u8,
                68u8,
                70u8,
                178u8,
                138u8,
                19u8,
                114u8,
                65u8,
                125u8,
                218u8,
                101u8,
                141u8,
                48u8,
                185u8,
                91u8,
                41u8,
                146u8,
                177u8,
                42u8,
                201u8,
                199u8,
                243u8,
                120u8,
                83u8,
                95u8,
                41u8,
                169u8,
                122u8,
                207u8,
                53u8,
                131u8,
            ],
            [
                44u8,
                171u8,
                151u8,
                144u8,
                81u8,
                15u8,
                216u8,
                189u8,
                251u8,
                210u8,
                17u8,
                82u8,
                136u8,
                219u8,
                51u8,
                254u8,
                198u8,
                102u8,
                145u8,
                212u8,
                118u8,
                239u8,
                197u8,
                66u8,
                124u8,
                253u8,
                76u8,
                9u8,
                105u8,
                48u8,
                23u8,
                85u8,
            ],
            [
                45u8,
                8u8,
                0u8,
                187u8,
                195u8,
                119u8,
                234u8,
                84u8,
                160u8,
                140u8,
                93u8,
                182u8,
                168u8,
                122u8,
                175u8,
                255u8,
                94u8,
                62u8,
                156u8,
                143u8,
                234u8,
                208u8,
                237u8,
                161u8,
                16u8,
                228u8,
                14u8,
                12u8,
                16u8,
                68u8,
                20u8,
                73u8,
            ],
            [
                47u8,
                230u8,
                50u8,
                119u8,
                145u8,
                116u8,
                55u8,
                67u8,
                120u8,
                68u8,
                42u8,
                142u8,
                151u8,
                139u8,
                204u8,
                251u8,
                220u8,
                193u8,
                214u8,
                178u8,
                176u8,
                216u8,
                31u8,
                126u8,
                142u8,
                183u8,
                118u8,
                171u8,
                34u8,
                134u8,
                241u8,
                104u8,
            ],
            [
                48u8,
                66u8,
                10u8,
                172u8,
                208u8,
                40u8,
                171u8,
                179u8,
                193u8,
                253u8,
                3u8,
                171u8,
                162u8,
                83u8,
                174u8,
                114u8,
                93u8,
                109u8,
                221u8,
                82u8,
                209u8,
                108u8,
                154u8,
                196u8,
                203u8,
                87u8,
                66u8,
                205u8,
                67u8,
                245u8,
                48u8,
                150u8,
            ],
            [
                59u8,
                207u8,
                178u8,
                174u8,
                46u8,
                141u8,
                19u8,
                45u8,
                209u8,
                252u8,
                231u8,
                207u8,
                39u8,
                138u8,
                154u8,
                25u8,
                117u8,
                106u8,
                159u8,
                206u8,
                171u8,
                228u8,
                112u8,
                223u8,
                59u8,
                218u8,
                187u8,
                75u8,
                197u8,
                119u8,
                209u8,
                189u8,
            ],
            [
                64u8,
                225u8,
                132u8,
                15u8,
                87u8,
                105u8,
                7u8,
                61u8,
                97u8,
                189u8,
                1u8,
                55u8,
                45u8,
                155u8,
                117u8,
                186u8,
                169u8,
                132u8,
                45u8,
                86u8,
                41u8,
                160u8,
                201u8,
                159u8,
                241u8,
                3u8,
                190u8,
                17u8,
                120u8,
                168u8,
                233u8,
                226u8,
            ],
            [
                65u8,
                48u8,
                79u8,
                172u8,
                217u8,
                50u8,
                61u8,
                117u8,
                177u8,
                27u8,
                205u8,
                214u8,
                9u8,
                203u8,
                56u8,
                239u8,
                255u8,
                253u8,
                176u8,
                87u8,
                16u8,
                247u8,
                202u8,
                240u8,
                233u8,
                177u8,
                108u8,
                109u8,
                157u8,
                112u8,
                159u8,
                80u8,
            ],
            [
                93u8,
                166u8,
                206u8,
                157u8,
                81u8,
                21u8,
                27u8,
                161u8,
                12u8,
                9u8,
                165u8,
                89u8,
                239u8,
                36u8,
                213u8,
                32u8,
                185u8,
                218u8,
                197u8,
                197u8,
                184u8,
                129u8,
                10u8,
                232u8,
                67u8,
                78u8,
                77u8,
                13u8,
                134u8,
                65u8,
                26u8,
                149u8,
            ],
            [
                96u8,
                104u8,
                101u8,
                183u8,
                147u8,
                74u8,
                37u8,
                212u8,
                174u8,
                212u8,
                63u8,
                108u8,
                219u8,
                66u8,
                100u8,
                3u8,
                53u8,
                63u8,
                164u8,
                179u8,
                0u8,
                156u8,
                77u8,
                34u8,
                132u8,
                7u8,
                71u8,
                69u8,
                129u8,
                176u8,
                30u8,
                35u8,
            ],
            [
                111u8,
                221u8,
                61u8,
                189u8,
                177u8,
                115u8,
                41u8,
                150u8,
                8u8,
                192u8,
                170u8,
                159u8,
                54u8,
                135u8,
                53u8,
                133u8,
                124u8,
                136u8,
                66u8,
                181u8,
                129u8,
                248u8,
                56u8,
                146u8,
                56u8,
                191u8,
                5u8,
                189u8,
                4u8,
                179u8,
                191u8,
                73u8,
            ],
            [
                122u8,
                231u8,
                76u8,
                82u8,
                116u8,
                20u8,
                174u8,
                19u8,
                95u8,
                217u8,
                112u8,
                71u8,
                177u8,
                41u8,
                33u8,
                165u8,
                236u8,
                57u8,
                17u8,
                184u8,
                4u8,
                25u8,
                120u8,
                85u8,
                214u8,
                126u8,
                37u8,
                199u8,
                183u8,
                94u8,
                230u8,
                243u8,
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
                137u8,
                10u8,
                130u8,
                103u8,
                155u8,
                71u8,
                15u8,
                43u8,
                216u8,
                40u8,
                22u8,
                237u8,
                155u8,
                22u8,
                31u8,
                151u8,
                216u8,
                185u8,
                103u8,
                243u8,
                127u8,
                163u8,
                100u8,
                124u8,
                33u8,
                213u8,
                191u8,
                57u8,
                116u8,
                158u8,
                45u8,
                213u8,
            ],
            [
                137u8,
                71u8,
                253u8,
                44u8,
                224u8,
                126u8,
                249u8,
                204u8,
                48u8,
                44u8,
                78u8,
                143u8,
                4u8,
                97u8,
                1u8,
                86u8,
                21u8,
                217u8,
                28u8,
                232u8,
                81u8,
                86u8,
                72u8,
                57u8,
                233u8,
                28u8,
                200u8,
                4u8,
                194u8,
                244u8,
                157u8,
                142u8,
            ],
            [
                138u8,
                115u8,
                53u8,
                113u8,
                66u8,
                49u8,
                219u8,
                213u8,
                81u8,
                170u8,
                186u8,
                99u8,
                20u8,
                244u8,
                169u8,
                122u8,
                20u8,
                194u8,
                1u8,
                229u8,
                58u8,
                62u8,
                37u8,
                225u8,
                20u8,
                3u8,
                37u8,
                205u8,
                246u8,
                125u8,
                122u8,
                78u8,
            ],
            [
                156u8,
                78u8,
                133u8,
                65u8,
                202u8,
                143u8,
                13u8,
                193u8,
                196u8,
                19u8,
                249u8,
                16u8,
                143u8,
                102u8,
                216u8,
                45u8,
                60u8,
                236u8,
                177u8,
                189u8,
                219u8,
                206u8,
                67u8,
                122u8,
                97u8,
                202u8,
                163u8,
                23u8,
                92u8,
                76u8,
                201u8,
                111u8,
            ],
            [
                167u8,
                62u8,
                218u8,
                9u8,
                102u8,
                47u8,
                70u8,
                221u8,
                231u8,
                41u8,
                190u8,
                70u8,
                17u8,
                56u8,
                95u8,
                243u8,
                79u8,
                230u8,
                196u8,
                79u8,
                187u8,
                198u8,
                247u8,
                225u8,
                123u8,
                4u8,
                43u8,
                89u8,
                163u8,
                68u8,
                91u8,
                87u8,
            ],
            [
                175u8,
                183u8,
                149u8,
                201u8,
                198u8,
                30u8,
                79u8,
                231u8,
                70u8,
                140u8,
                56u8,
                111u8,
                146u8,
                93u8,
                122u8,
                84u8,
                41u8,
                236u8,
                173u8,
                156u8,
                4u8,
                149u8,
                221u8,
                184u8,
                211u8,
                141u8,
                105u8,
                6u8,
                20u8,
                211u8,
                47u8,
                153u8,
            ],
            [
                178u8,
                222u8,
                47u8,
                190u8,
                128u8,
                26u8,
                13u8,
                246u8,
                192u8,
                203u8,
                221u8,
                253u8,
                68u8,
                139u8,
                163u8,
                196u8,
                29u8,
                72u8,
                160u8,
                64u8,
                202u8,
                53u8,
                197u8,
                108u8,
                129u8,
                150u8,
                239u8,
                15u8,
                202u8,
                231u8,
                33u8,
                168u8,
            ],
            [
                183u8,
                106u8,
                147u8,
                187u8,
                100u8,
                158u8,
                206u8,
                82u8,
                70u8,
                136u8,
                241u8,
                160u8,
                29u8,
                24u8,
                78u8,
                11u8,
                190u8,
                188u8,
                218u8,
                88u8,
                234u8,
                232u8,
                12u8,
                40u8,
                168u8,
                152u8,
                190u8,
                195u8,
                251u8,
                90u8,
                9u8,
                99u8,
            ],
            [
                202u8,
                141u8,
                252u8,
                140u8,
                94u8,
                10u8,
                103u8,
                167u8,
                69u8,
                1u8,
                192u8,
                114u8,
                163u8,
                50u8,
                95u8,
                104u8,
                82u8,
                89u8,
                190u8,
                187u8,
                174u8,
                124u8,
                253u8,
                35u8,
                10u8,
                184u8,
                81u8,
                152u8,
                167u8,
                139u8,
                112u8,
                205u8,
            ],
            [
                210u8,
                110u8,
                22u8,
                202u8,
                212u8,
                84u8,
                135u8,
                5u8,
                228u8,
                201u8,
                226u8,
                217u8,
                79u8,
                152u8,
                238u8,
                145u8,
                194u8,
                137u8,
                8u8,
                94u8,
                228u8,
                37u8,
                89u8,
                79u8,
                213u8,
                99u8,
                95u8,
                162u8,
                150u8,
                76u8,
                207u8,
                24u8,
            ],
            [
                231u8,
                149u8,
                14u8,
                222u8,
                3u8,
                148u8,
                185u8,
                242u8,
                206u8,
                74u8,
                90u8,
                27u8,
                245u8,
                167u8,
                225u8,
                133u8,
                36u8,
                17u8,
                247u8,
                230u8,
                102u8,
                27u8,
                67u8,
                8u8,
                201u8,
                19u8,
                196u8,
                191u8,
                209u8,
                16u8,
                39u8,
                228u8,
            ],
            [
                232u8,
                22u8,
                153u8,
                184u8,
                81u8,
                19u8,
                238u8,
                161u8,
                199u8,
                62u8,
                16u8,
                88u8,
                139u8,
                43u8,
                3u8,
                94u8,
                85u8,
                137u8,
                51u8,
                105u8,
                99u8,
                33u8,
                115u8,
                175u8,
                212u8,
                63u8,
                235u8,
                25u8,
                47u8,
                172u8,
                100u8,
                227u8,
            ],
            [
                235u8,
                139u8,
                164u8,
                60u8,
                237u8,
                117u8,
                55u8,
                66u8,
                25u8,
                70u8,
                189u8,
                67u8,
                232u8,
                40u8,
                184u8,
                178u8,
                184u8,
                66u8,
                137u8,
                39u8,
                170u8,
                143u8,
                128u8,
                28u8,
                19u8,
                217u8,
                52u8,
                191u8,
                17u8,
                172u8,
                165u8,
                123u8,
            ],
            [
                251u8,
                16u8,
                40u8,
                101u8,
                213u8,
                10u8,
                221u8,
                221u8,
                246u8,
                157u8,
                169u8,
                181u8,
                170u8,
                27u8,
                206u8,
                214u8,
                108u8,
                128u8,
                207u8,
                134u8,
                154u8,
                92u8,
                141u8,
                4u8,
                113u8,
                164u8,
                103u8,
                225u8,
                140u8,
                233u8,
                202u8,
                177u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EPInternalFunctionsEvents {
        const NAME: &'static str = "EPInternalFunctionsEvents";
        const COUNT: usize = 32usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<EigenPodStaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EigenPodStaked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EigenPodStaked)
                }
                Some(
                    <FullWithdrawalRedeemed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <FullWithdrawalRedeemed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::FullWithdrawalRedeemed)
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
                    <NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NonBeaconChainETHReceived)
                }
                Some(
                    <NonBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <NonBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::NonBeaconChainETHWithdrawn)
                }
                Some(
                    <PartialWithdrawalRedeemed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PartialWithdrawalRedeemed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PartialWithdrawalRedeemed)
                }
                Some(
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RestakedBeaconChainETHWithdrawn)
                }
                Some(
                    <RestakingActivated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RestakingActivated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::RestakingActivated)
                }
                Some(
                    <ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidatorBalanceUpdated)
                }
                Some(
                    <ValidatorRestaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidatorRestaked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidatorRestaked)
                }
                Some(<log as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_int)
                }
                Some(
                    <log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_address)
                }
                Some(
                    <log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_0)
                }
                Some(
                    <log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_1)
                }
                Some(
                    <log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes)
                }
                Some(
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_bytes32)
                }
                Some(
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_int)
                }
                Some(
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for EPInternalFunctionsEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::FullWithdrawalRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NonBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PartialWithdrawalRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RestakingActivated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::FullWithdrawalRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NonBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PartialWithdrawalRedeemed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RestakingActivated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_address(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_0(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_1(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_array_2(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_decimal_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_int(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_named_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_string(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::log_uint(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::logs(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EPInternalFunctions`](self) contract instance.

See the [wrapper's documentation](`EPInternalFunctionsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EPInternalFunctionsInstance<T, P, N> {
        EPInternalFunctionsInstance::<T, P, N>::new(address, provider)
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
        _ethPOS: alloy::sol_types::private::Address,
        _delayedWithdrawalRouter: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: u64,
        _GENESIS_TIME: u64,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EPInternalFunctionsInstance<T, P, N>>,
    > {
        EPInternalFunctionsInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _ethPOS,
            _delayedWithdrawalRouter,
            _eigenPodManager,
            _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
            _GENESIS_TIME,
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
        _ethPOS: alloy::sol_types::private::Address,
        _delayedWithdrawalRouter: alloy::sol_types::private::Address,
        _eigenPodManager: alloy::sol_types::private::Address,
        _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: u64,
        _GENESIS_TIME: u64,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EPInternalFunctionsInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _ethPOS,
            _delayedWithdrawalRouter,
            _eigenPodManager,
            _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
            _GENESIS_TIME,
        )
    }
    /**A [`EPInternalFunctions`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EPInternalFunctions`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EPInternalFunctionsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EPInternalFunctionsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EPInternalFunctionsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EPInternalFunctionsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EPInternalFunctions`](self) contract instance.

See the [wrapper's documentation](`EPInternalFunctionsInstance`) for more details.*/
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
            _ethPOS: alloy::sol_types::private::Address,
            _delayedWithdrawalRouter: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: u64,
            _GENESIS_TIME: u64,
        ) -> alloy_contract::Result<EPInternalFunctionsInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _ethPOS,
                _delayedWithdrawalRouter,
                _eigenPodManager,
                _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
                _GENESIS_TIME,
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
            _ethPOS: alloy::sol_types::private::Address,
            _delayedWithdrawalRouter: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR: u64,
            _GENESIS_TIME: u64,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _ethPOS,
                            _delayedWithdrawalRouter,
                            _eigenPodManager,
                            _MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR,
                            _GENESIS_TIME,
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
    impl<T, P: ::core::clone::Clone, N> EPInternalFunctionsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EPInternalFunctionsInstance<T, P, N> {
            EPInternalFunctionsInstance {
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
    > EPInternalFunctionsInstance<T, P, N> {
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
        ///Creates a new call builder for the [`GENESIS_TIME`] function.
        pub fn GENESIS_TIME(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, GENESIS_TIMECall, N> {
            self.call_builder(&GENESIS_TIMECall {})
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR`] function.
        pub fn MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall,
            N,
        > {
            self.call_builder(
                &MAX_RESTAKED_BALANCE_GWEI_PER_VALIDATORCall {
                },
            )
        }
        ///Creates a new call builder for the [`activateRestaking`] function.
        pub fn activateRestaking(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, activateRestakingCall, N> {
            self.call_builder(&activateRestakingCall {})
        }
        ///Creates a new call builder for the [`delayedWithdrawalRouter`] function.
        pub fn delayedWithdrawalRouter(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delayedWithdrawalRouterCall, N> {
            self.call_builder(&delayedWithdrawalRouterCall {})
        }
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
        }
        ///Creates a new call builder for the [`ethPOS`] function.
        pub fn ethPOS(&self) -> alloy_contract::SolCallBuilder<T, &P, ethPOSCall, N> {
            self.call_builder(&ethPOSCall {})
        }
        ///Creates a new call builder for the [`excludeArtifacts`] function.
        pub fn excludeArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeArtifactsCall, N> {
            self.call_builder(&excludeArtifactsCall {})
        }
        ///Creates a new call builder for the [`excludeContracts`] function.
        pub fn excludeContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeContractsCall, N> {
            self.call_builder(&excludeContractsCall {})
        }
        ///Creates a new call builder for the [`excludeSenders`] function.
        pub fn excludeSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSendersCall, N> {
            self.call_builder(&excludeSendersCall {})
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getActiveValidatorCount`] function.
        pub fn getActiveValidatorCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getActiveValidatorCountCall, N> {
            self.call_builder(&getActiveValidatorCountCall {})
        }
        ///Creates a new call builder for the [`hasRestaked`] function.
        pub fn hasRestaked(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, hasRestakedCall, N> {
            self.call_builder(&hasRestakedCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _podOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _podOwner })
        }
        ///Creates a new call builder for the [`mostRecentWithdrawalTimestamp`] function.
        pub fn mostRecentWithdrawalTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            mostRecentWithdrawalTimestampCall,
            N,
        > {
            self.call_builder(
                &mostRecentWithdrawalTimestampCall {
                },
            )
        }
        ///Creates a new call builder for the [`nonBeaconChainETHBalanceWei`] function.
        pub fn nonBeaconChainETHBalanceWei(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, nonBeaconChainETHBalanceWeiCall, N> {
            self.call_builder(&nonBeaconChainETHBalanceWeiCall {})
        }
        ///Creates a new call builder for the [`podOwner`] function.
        pub fn podOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, podOwnerCall, N> {
            self.call_builder(&podOwnerCall {})
        }
        ///Creates a new call builder for the [`processFullWithdrawal`] function.
        pub fn processFullWithdrawal(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
            validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
            withdrawalHappenedTimestamp: u64,
            recipient: alloy::sol_types::private::Address,
            withdrawalAmountGwei: u64,
            validatorInfo: <IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, processFullWithdrawalCall, N> {
            self.call_builder(
                &processFullWithdrawalCall {
                    validatorIndex,
                    validatorPubkeyHash,
                    withdrawalHappenedTimestamp,
                    recipient,
                    withdrawalAmountGwei,
                    validatorInfo,
                },
            )
        }
        ///Creates a new call builder for the [`processPartialWithdrawal`] function.
        pub fn processPartialWithdrawal(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
            withdrawalHappenedTimestamp: u64,
            recipient: alloy::sol_types::private::Address,
            withdrawalAmountGwei: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, processPartialWithdrawalCall, N> {
            self.call_builder(
                &processPartialWithdrawalCall {
                    validatorIndex,
                    withdrawalHappenedTimestamp,
                    recipient,
                    withdrawalAmountGwei,
                },
            )
        }
        ///Creates a new call builder for the [`provenWithdrawal`] function.
        pub fn provenWithdrawal(
            &self,
            _0: alloy::sol_types::private::FixedBytes<32>,
            _1: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, provenWithdrawalCall, N> {
            self.call_builder(&provenWithdrawalCall { _0, _1 })
        }
        ///Creates a new call builder for the [`recoverTokens`] function.
        pub fn recoverTokens(
            &self,
            tokenList: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            amountsToWithdraw: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, recoverTokensCall, N> {
            self.call_builder(
                &recoverTokensCall {
                    tokenList,
                    amountsToWithdraw,
                    recipient,
                },
            )
        }
        ///Creates a new call builder for the [`setActiveValidatorCount`] function.
        pub fn setActiveValidatorCount(
            &self,
            _count: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setActiveValidatorCountCall, N> {
            self.call_builder(
                &setActiveValidatorCountCall {
                    _count,
                },
            )
        }
        ///Creates a new call builder for the [`setMostRecentWithdrawalTimestamp`] function.
        pub fn setMostRecentWithdrawalTimestamp(
            &self,
            _mostRecentWithdrawalTimestamp: u64,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            setMostRecentWithdrawalTimestampCall,
            N,
        > {
            self.call_builder(
                &setMostRecentWithdrawalTimestampCall {
                    _mostRecentWithdrawalTimestamp,
                },
            )
        }
        ///Creates a new call builder for the [`setValidatorRestakedBalance`] function.
        pub fn setValidatorRestakedBalance(
            &self,
            pkhash: alloy::sol_types::private::FixedBytes<32>,
            restakedBalanceGwei: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, setValidatorRestakedBalanceCall, N> {
            self.call_builder(
                &setValidatorRestakedBalanceCall {
                    pkhash,
                    restakedBalanceGwei,
                },
            )
        }
        ///Creates a new call builder for the [`setValidatorStatus`] function.
        pub fn setValidatorStatus(
            &self,
            pkhash: alloy::sol_types::private::FixedBytes<32>,
            status: <IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setValidatorStatusCall, N> {
            self.call_builder(
                &setValidatorStatusCall {
                    pkhash,
                    status,
                },
            )
        }
        ///Creates a new call builder for the [`stake`] function.
        pub fn stake(
            &self,
            pubkey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            depositDataRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeCall, N> {
            self.call_builder(
                &stakeCall {
                    pubkey,
                    signature,
                    depositDataRoot,
                },
            )
        }
        ///Creates a new call builder for the [`sumOfPartialWithdrawalsClaimedGwei`] function.
        pub fn sumOfPartialWithdrawalsClaimedGwei(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            sumOfPartialWithdrawalsClaimedGweiCall,
            N,
        > {
            self.call_builder(
                &sumOfPartialWithdrawalsClaimedGweiCall {
                },
            )
        }
        ///Creates a new call builder for the [`targetArtifactSelectors`] function.
        pub fn targetArtifactSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactSelectorsCall, N> {
            self.call_builder(&targetArtifactSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetArtifacts`] function.
        pub fn targetArtifacts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetArtifactsCall, N> {
            self.call_builder(&targetArtifactsCall {})
        }
        ///Creates a new call builder for the [`targetContracts`] function.
        pub fn targetContracts(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetContractsCall, N> {
            self.call_builder(&targetContractsCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`validatorPubkeyHashToInfo`] function.
        pub fn validatorPubkeyHashToInfo(
            &self,
            validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorPubkeyHashToInfoCall, N> {
            self.call_builder(
                &validatorPubkeyHashToInfoCall {
                    validatorPubkeyHash,
                },
            )
        }
        ///Creates a new call builder for the [`validatorPubkeyToInfo`] function.
        pub fn validatorPubkeyToInfo(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorPubkeyToInfoCall, N> {
            self.call_builder(
                &validatorPubkeyToInfoCall {
                    validatorPubkey,
                },
            )
        }
        ///Creates a new call builder for the [`validatorStatus_0`] function.
        pub fn validatorStatus_0(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorStatus_0Call, N> {
            self.call_builder(
                &validatorStatus_0Call {
                    validatorPubkey,
                },
            )
        }
        ///Creates a new call builder for the [`validatorStatus_1`] function.
        pub fn validatorStatus_1(
            &self,
            pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorStatus_1Call, N> {
            self.call_builder(
                &validatorStatus_1Call {
                    pubkeyHash,
                },
            )
        }
        ///Creates a new call builder for the [`verifyAndProcessWithdrawal`] function.
        pub fn verifyAndProcessWithdrawal(
            &self,
            beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
            withdrawalProof: <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
            validatorFieldsProof: alloy::sol_types::private::Bytes,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            withdrawalFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyAndProcessWithdrawalCall, N> {
            self.call_builder(
                &verifyAndProcessWithdrawalCall {
                    beaconStateRoot,
                    withdrawalProof,
                    validatorFieldsProof,
                    validatorFields,
                    withdrawalFields,
                },
            )
        }
        ///Creates a new call builder for the [`verifyAndProcessWithdrawals`] function.
        pub fn verifyAndProcessWithdrawals(
            &self,
            oracleTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            withdrawalProofs: alloy::sol_types::private::Vec<
                <BeaconChainProofs::WithdrawalProof as alloy::sol_types::SolType>::RustType,
            >,
            validatorFieldsProofs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
            withdrawalFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyAndProcessWithdrawalsCall, N> {
            self.call_builder(
                &verifyAndProcessWithdrawalsCall {
                    oracleTimestamp,
                    stateRootProof,
                    withdrawalProofs,
                    validatorFieldsProofs,
                    validatorFields,
                    withdrawalFields,
                },
            )
        }
        ///Creates a new call builder for the [`verifyBalanceUpdate`] function.
        pub fn verifyBalanceUpdate(
            &self,
            oracleTimestamp: u64,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
            beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
            validatorFieldsProofs: alloy::sol_types::private::Bytes,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            mostRecentBalanceUpdateTimestamp: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyBalanceUpdateCall, N> {
            self.call_builder(
                &verifyBalanceUpdateCall {
                    oracleTimestamp,
                    validatorIndex,
                    beaconStateRoot,
                    validatorFieldsProofs,
                    validatorFields,
                    mostRecentBalanceUpdateTimestamp,
                },
            )
        }
        ///Creates a new call builder for the [`verifyBalanceUpdates`] function.
        pub fn verifyBalanceUpdates(
            &self,
            oracleTimestamp: u64,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            validatorFieldsProofs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyBalanceUpdatesCall, N> {
            self.call_builder(
                &verifyBalanceUpdatesCall {
                    oracleTimestamp,
                    validatorIndices,
                    stateRootProof,
                    validatorFieldsProofs,
                    validatorFields,
                },
            )
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials_0`] function.
        pub fn verifyWithdrawalCredentials_0(
            &self,
            oracleTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            validatorFieldsProofs: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Bytes,
            >,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            verifyWithdrawalCredentials_0Call,
            N,
        > {
            self.call_builder(
                &verifyWithdrawalCredentials_0Call {
                    oracleTimestamp,
                    stateRootProof,
                    validatorIndices,
                    validatorFieldsProofs,
                    validatorFields,
                },
            )
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials_1`] function.
        pub fn verifyWithdrawalCredentials_1(
            &self,
            oracleTimestamp: u64,
            beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
            validatorFieldsProof: alloy::sol_types::private::Bytes,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            verifyWithdrawalCredentials_1Call,
            N,
        > {
            self.call_builder(
                &verifyWithdrawalCredentials_1Call {
                    oracleTimestamp,
                    beaconStateRoot,
                    validatorIndex,
                    validatorFieldsProof,
                    validatorFields,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawBeforeRestaking`] function.
        pub fn withdrawBeforeRestaking(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawBeforeRestakingCall, N> {
            self.call_builder(&withdrawBeforeRestakingCall {})
        }
        ///Creates a new call builder for the [`withdrawNonBeaconChainETHBalanceWei`] function.
        pub fn withdrawNonBeaconChainETHBalanceWei(
            &self,
            recipient: alloy::sol_types::private::Address,
            amountToWithdraw: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            withdrawNonBeaconChainETHBalanceWeiCall,
            N,
        > {
            self.call_builder(
                &withdrawNonBeaconChainETHBalanceWeiCall {
                    recipient,
                    amountToWithdraw,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawRestakedBeaconChainETH`] function.
        pub fn withdrawRestakedBeaconChainETH(
            &self,
            recipient: alloy::sol_types::private::Address,
            amountWei: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            withdrawRestakedBeaconChainETHCall,
            N,
        > {
            self.call_builder(
                &withdrawRestakedBeaconChainETHCall {
                    recipient,
                    amountWei,
                },
            )
        }
        ///Creates a new call builder for the [`withdrawableRestakedExecutionLayerGwei`] function.
        pub fn withdrawableRestakedExecutionLayerGwei(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            withdrawableRestakedExecutionLayerGweiCall,
            N,
        > {
            self.call_builder(
                &withdrawableRestakedExecutionLayerGweiCall {
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
    > EPInternalFunctionsInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`EigenPodStaked`] event.
        pub fn EigenPodStaked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EigenPodStaked, N> {
            self.event_filter::<EigenPodStaked>()
        }
        ///Creates a new event filter for the [`FullWithdrawalRedeemed`] event.
        pub fn FullWithdrawalRedeemed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, FullWithdrawalRedeemed, N> {
            self.event_filter::<FullWithdrawalRedeemed>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NonBeaconChainETHReceived`] event.
        pub fn NonBeaconChainETHReceived_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NonBeaconChainETHReceived, N> {
            self.event_filter::<NonBeaconChainETHReceived>()
        }
        ///Creates a new event filter for the [`NonBeaconChainETHWithdrawn`] event.
        pub fn NonBeaconChainETHWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, NonBeaconChainETHWithdrawn, N> {
            self.event_filter::<NonBeaconChainETHWithdrawn>()
        }
        ///Creates a new event filter for the [`PartialWithdrawalRedeemed`] event.
        pub fn PartialWithdrawalRedeemed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PartialWithdrawalRedeemed, N> {
            self.event_filter::<PartialWithdrawalRedeemed>()
        }
        ///Creates a new event filter for the [`RestakedBeaconChainETHWithdrawn`] event.
        pub fn RestakedBeaconChainETHWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RestakedBeaconChainETHWithdrawn, N> {
            self.event_filter::<RestakedBeaconChainETHWithdrawn>()
        }
        ///Creates a new event filter for the [`RestakingActivated`] event.
        pub fn RestakingActivated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RestakingActivated, N> {
            self.event_filter::<RestakingActivated>()
        }
        ///Creates a new event filter for the [`ValidatorBalanceUpdated`] event.
        pub fn ValidatorBalanceUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorBalanceUpdated, N> {
            self.event_filter::<ValidatorBalanceUpdated>()
        }
        ///Creates a new event filter for the [`ValidatorRestaked`] event.
        pub fn ValidatorRestaked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorRestaked, N> {
            self.event_filter::<ValidatorRestaked>()
        }
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_bytes32, N> {
            self.event_filter::<log_bytes32>()
        }
        ///Creates a new event filter for the [`log_int`] event.
        pub fn log_int_filter(&self) -> alloy_contract::Event<T, &P, log_int, N> {
            self.event_filter::<log_int>()
        }
        ///Creates a new event filter for the [`log_named_address`] event.
        pub fn log_named_address_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_address, N> {
            self.event_filter::<log_named_address>()
        }
        ///Creates a new event filter for the [`log_named_array_0`] event.
        pub fn log_named_array_0_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_0, N> {
            self.event_filter::<log_named_array_0>()
        }
        ///Creates a new event filter for the [`log_named_array_1`] event.
        pub fn log_named_array_1_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_1, N> {
            self.event_filter::<log_named_array_1>()
        }
        ///Creates a new event filter for the [`log_named_array_2`] event.
        pub fn log_named_array_2_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_array_2, N> {
            self.event_filter::<log_named_array_2>()
        }
        ///Creates a new event filter for the [`log_named_bytes`] event.
        pub fn log_named_bytes_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
            self.event_filter::<log_named_bytes>()
        }
        ///Creates a new event filter for the [`log_named_bytes32`] event.
        pub fn log_named_bytes32_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_bytes32, N> {
            self.event_filter::<log_named_bytes32>()
        }
        ///Creates a new event filter for the [`log_named_decimal_int`] event.
        pub fn log_named_decimal_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_int, N> {
            self.event_filter::<log_named_decimal_int>()
        }
        ///Creates a new event filter for the [`log_named_decimal_uint`] event.
        pub fn log_named_decimal_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_decimal_uint, N> {
            self.event_filter::<log_named_decimal_uint>()
        }
        ///Creates a new event filter for the [`log_named_int`] event.
        pub fn log_named_int_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, log_named_uint, N> {
            self.event_filter::<log_named_uint>()
        }
        ///Creates a new event filter for the [`log_string`] event.
        pub fn log_string_filter(&self) -> alloy_contract::Event<T, &P, log_string, N> {
            self.event_filter::<log_string>()
        }
        ///Creates a new event filter for the [`log_uint`] event.
        pub fn log_uint_filter(&self) -> alloy_contract::Event<T, &P, log_uint, N> {
            self.event_filter::<log_uint>()
        }
        ///Creates a new event filter for the [`logs`] event.
        pub fn logs_filter(&self) -> alloy_contract::Event<T, &P, logs, N> {
            self.event_filter::<logs>()
        }
    }
}
