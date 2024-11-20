///Module containing a contract's types and functions.
/**

```solidity
library BeaconChainProofs {
    struct BalanceContainerProof { bytes32 balanceContainerRoot; bytes proof; }
    struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
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
struct BalanceContainerProof { bytes32 balanceContainerRoot; bytes proof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceContainerProof {
        pub balanceContainerRoot: alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<BalanceContainerProof> for UnderlyingRustTuple<'_> {
            fn from(value: BalanceContainerProof) -> Self {
                (value.balanceContainerRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BalanceContainerProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balanceContainerRoot: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BalanceContainerProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BalanceContainerProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceContainerRoot),
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
        impl alloy_sol_types::SolType for BalanceContainerProof {
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
        impl alloy_sol_types::SolStruct for BalanceContainerProof {
            const NAME: &'static str = "BalanceContainerProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceContainerProof(bytes32 balanceContainerRoot,bytes proof)",
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
                            &self.balanceContainerRoot,
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
        impl alloy_sol_types::EventTopic for BalanceContainerProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceContainerRoot,
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
                    &rust.balanceContainerRoot,
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
struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceProof {
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        pub balanceRoot: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<BalanceProof> for UnderlyingRustTuple<'_> {
            fn from(value: BalanceProof) -> Self {
                (value.pubkeyHash, value.balanceRoot, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BalanceProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    pubkeyHash: tuple.0,
                    balanceRoot: tuple.1,
                    proof: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for BalanceProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for BalanceProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.pubkeyHash),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceRoot),
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
        impl alloy_sol_types::SolType for BalanceProof {
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
        impl alloy_sol_types::SolStruct for BalanceProof {
            const NAME: &'static str = "BalanceProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceProof(bytes32 pubkeyHash,bytes32 balanceRoot,bytes proof)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.pubkeyHash)
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.balanceRoot)
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
        impl alloy_sol_types::EventTopic for BalanceProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.pubkeyHash,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceRoot,
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
                    &rust.pubkeyHash,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceRoot,
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
/**

Generated by the following Solidity interface...
```solidity
library BeaconChainProofs {
    struct BalanceContainerProof {
        bytes32 balanceContainerRoot;
        bytes proof;
    }
    struct BalanceProof {
        bytes32 pubkeyHash;
        bytes32 balanceRoot;
        bytes proof;
    }
    struct StateRootProof {
        bytes32 beaconStateRoot;
        bytes proof;
    }
}

interface BeaconChainProofsWrapper {
    error InvalidProof();
    error InvalidProofLength();
    error InvalidValidatorFieldsLength();

    function verifyBalanceContainer(bytes32 beaconBlockRoot, BeaconChainProofs.BalanceContainerProof memory proof) external view;
    function verifyStateRoot(bytes32 beaconBlockRoot, BeaconChainProofs.StateRootProof memory proof) external view;
    function verifyValidatorBalance(bytes32 balanceContainerRoot, uint40 validatorIndex, BeaconChainProofs.BalanceProof memory proof) external view;
    function verifyValidatorFields(bytes32 beaconStateRoot, bytes32[] memory validatorFields, bytes memory validatorFieldsProof, uint40 validatorIndex) external view;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "verifyBalanceContainer",
    "inputs": [
      {
        "name": "beaconBlockRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.BalanceContainerProof",
        "components": [
          {
            "name": "balanceContainerRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyStateRoot",
    "inputs": [
      {
        "name": "beaconBlockRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "proof",
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
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyValidatorBalance",
    "inputs": [
      {
        "name": "balanceContainerRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      },
      {
        "name": "proof",
        "type": "tuple",
        "internalType": "struct BeaconChainProofs.BalanceProof",
        "components": [
          {
            "name": "pubkeyHash",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "balanceRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proof",
            "type": "bytes",
            "internalType": "bytes"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyValidatorFields",
    "inputs": [
      {
        "name": "beaconStateRoot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "validatorFields",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      },
      {
        "name": "validatorFieldsProof",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [],
    "stateMutability": "view"
  },
  {
    "type": "error",
    "name": "InvalidProof",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidProofLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidValidatorFieldsLength",
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
pub mod BeaconChainProofsWrapper {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052348015600f57600080fd5b50610c3c8061001f6000396000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80630d361f3a14610051578063256f222b1461006657806331f60d4c146100795780639030a9bb1461008c575b600080fd5b61006461005f366004610887565b61009f565b005b610064610074366004610931565b6100ad565b6100646100873660046109f4565b6100c3565b61006461009a366004610887565b6100d4565b6100a982826100de565b5050565b6100bb868686868686610195565b505050505050565b6100ce8383836102c0565b50505050565b6100a982826103a1565b6100ea60056003610a68565b6100f5906020610a7b565b6101026020830183610a92565b905014610122576040516313717da960e21b815260040160405180910390fd5b606c6101736101346020840184610a92565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084610447565b610190576040516309bde33960e01b815260040160405180910390fd5b505050565b600884146101b65760405163200591bd60e01b815260040160405180910390fd5b60056101c460286001610a68565b6101ce9190610a68565b6101d9906020610a7b565b82146101f8576040516313717da960e21b815260040160405180910390fd5b600061023686868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061045f92505050565b9050600064ffffffffff831661024e60286001610a68565b600b901b17905061029985858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050610447565b6102b6576040516309bde33960e01b815260040160405180910390fd5b5050505050505050565b60006102ce60266001610a68565b6102d9906020610a7b565b6102e66040840184610a92565b905014610306576040516313717da960e21b815260040160405180910390fd5b6000610313600485610aef565b64ffffffffff16905061036d61032c6040850185610a92565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584610447565b61038a576040516309bde33960e01b815260040160405180910390fd5b6103988360200135856106f9565b95945050505050565b6103ad60036020610a7b565b6103ba6020830183610a92565b9050146103da576040516313717da960e21b815260040160405180910390fd5b61042a6103ea6020830183610a92565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003610447565b6100a9576040516309bde33960e01b815260040160405180910390fd5b600083610455868585610792565b1495945050505050565b600080600283516104709190610b19565b905060008167ffffffffffffffff81111561048d5761048d610b2d565b6040519080825280602002602001820160405280156104b6578160200160208202803683370190505b50905060005b828110156105b3576002856104d18383610a7b565b815181106104e1576104e1610b43565b6020026020010151868360026104f79190610a7b565b610502906001610a68565b8151811061051257610512610b43565b6020026020010151604051602001610534929190918252602082015260400190565b60408051601f198184030181529082905261054e91610b59565b602060405180830381855afa15801561056b573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061058e9190610b88565b8282815181106105a0576105a0610b43565b60209081029190910101526001016104bc565b506105bf600283610b19565b91505b81156106d55760005b828110156106c2576002826105e08383610a7b565b815181106105f0576105f0610b43565b6020026020010151838360026106069190610a7b565b610611906001610a68565b8151811061062157610621610b43565b6020026020010151604051602001610643929190918252602082015260400190565b60408051601f198184030181529082905261065d91610b59565b602060405180830381855afa15801561067a573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061069d9190610b88565b8282815181106106af576106af610b43565b60209081029190910101526001016105cb565b506106ce600283610b19565b91506105c2565b806000815181106106e8576106e8610b43565b602002602001015192505050919050565b600080610707600484610ba1565b610712906040610bcb565b64ffffffffff16905061078884821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b9150505b92915050565b600083516000141580156107b15750602084516107af9190610bf2565b155b6107ce576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111610865576107f2600285610bf2565b600003610828578151600052808601516020526020826040600060026107d05a03fa61081d57600080fd5b600284049350610853565b8086015160005281516020526020826040600060026107d05a03fa61084c57600080fd5b6002840493505b61085e602082610a68565b90506107df565b5051949350505050565b60006040828403121561088157600080fd5b50919050565b6000806040838503121561089a57600080fd5b82359150602083013567ffffffffffffffff8111156108b857600080fd5b6108c48582860161086f565b9150509250929050565b60008083601f8401126108e057600080fd5b50813567ffffffffffffffff8111156108f857600080fd5b60208301915083602082850101111561091057600080fd5b9250929050565b803564ffffffffff8116811461092c57600080fd5b919050565b6000806000806000806080878903121561094a57600080fd5b86359550602087013567ffffffffffffffff81111561096857600080fd5b8701601f8101891361097957600080fd5b803567ffffffffffffffff81111561099057600080fd5b8960208260051b84010111156109a557600080fd5b60209190910195509350604087013567ffffffffffffffff8111156109c957600080fd5b6109d589828a016108ce565b90945092506109e8905060608801610917565b90509295509295509295565b600080600060608486031215610a0957600080fd5b83359250610a1960208501610917565b9150604084013567ffffffffffffffff811115610a3557600080fd5b840160608187031215610a4757600080fd5b809150509250925092565b634e487b7160e01b600052601160045260246000fd5b8082018082111561078c5761078c610a52565b808202811582820484141761078c5761078c610a52565b6000808335601e19843603018112610aa957600080fd5b83018035915067ffffffffffffffff821115610ac457600080fd5b60200191503681900382131561091057600080fd5b634e487b7160e01b600052601260045260246000fd5b600064ffffffffff831680610b0657610b06610ad9565b8064ffffffffff84160491505092915050565b600082610b2857610b28610ad9565b500490565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6000825160005b81811015610b7a5760208186018101518583015201610b60565b506000920191825250919050565b600060208284031215610b9a57600080fd5b5051919050565b600064ffffffffff831680610bb857610bb8610ad9565b8064ffffffffff84160691505092915050565b64ffffffffff8181168382160290811690818114610beb57610beb610a52565b5092915050565b600082610c0157610c01610ad9565b50069056fea2646970667358221220952aa3945c3e075de1f6f20dd858576309d97469c7e9efa3bbb70f9d0705e5a964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[Pa\x0C<\x80a\0\x1F`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\r6\x1F:\x14a\0QW\x80c%o\"+\x14a\0fW\x80c1\xF6\rL\x14a\0yW\x80c\x900\xA9\xBB\x14a\0\x8CW[`\0\x80\xFD[a\0da\0_6`\x04a\x08\x87V[a\0\x9FV[\0[a\0da\0t6`\x04a\t1V[a\0\xADV[a\0da\0\x876`\x04a\t\xF4V[a\0\xC3V[a\0da\0\x9A6`\x04a\x08\x87V[a\0\xD4V[a\0\xA9\x82\x82a\0\xDEV[PPV[a\0\xBB\x86\x86\x86\x86\x86\x86a\x01\x95V[PPPPPPV[a\0\xCE\x83\x83\x83a\x02\xC0V[PPPPV[a\0\xA9\x82\x82a\x03\xA1V[a\0\xEA`\x05`\x03a\nhV[a\0\xF5\x90` a\n{V[a\x01\x02` \x83\x01\x83a\n\x92V[\x90P\x14a\x01\"W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`la\x01sa\x014` \x84\x01\x84a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a\x04GV[a\x01\x90W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\x08\x84\x14a\x01\xB6W`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05a\x01\xC4`(`\x01a\nhV[a\x01\xCE\x91\x90a\nhV[a\x01\xD9\x90` a\n{V[\x82\x14a\x01\xF8W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x026\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04_\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a\x02N`(`\x01a\nhV[`\x0B\x90\x1B\x17\x90Pa\x02\x99\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa\x04GV[a\x02\xB6W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0a\x02\xCE`&`\x01a\nhV[a\x02\xD9\x90` a\n{V[a\x02\xE6`@\x84\x01\x84a\n\x92V[\x90P\x14a\x03\x06W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x13`\x04\x85a\n\xEFV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x03ma\x03,`@\x85\x01\x85a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a\x04GV[a\x03\x8AW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x98\x83` \x015\x85a\x06\xF9V[\x95\x94PPPPPV[a\x03\xAD`\x03` a\n{V[a\x03\xBA` \x83\x01\x83a\n\x92V[\x90P\x14a\x03\xDAW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*a\x03\xEA` \x83\x01\x83a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a\x04GV[a\0\xA9W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83a\x04U\x86\x85\x85a\x07\x92V[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa\x04p\x91\x90a\x0B\x19V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8DWa\x04\x8Da\x0B-V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xB6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xB3W`\x02\x85a\x04\xD1\x83\x83a\n{V[\x81Q\x81\x10a\x04\xE1Wa\x04\xE1a\x0BCV[` \x02` \x01\x01Q\x86\x83`\x02a\x04\xF7\x91\x90a\n{V[a\x05\x02\x90`\x01a\nhV[\x81Q\x81\x10a\x05\x12Wa\x05\x12a\x0BCV[` \x02` \x01\x01Q`@Q` \x01a\x054\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05N\x91a\x0BYV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x05kW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x8E\x91\x90a\x0B\x88V[\x82\x82\x81Q\x81\x10a\x05\xA0Wa\x05\xA0a\x0BCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xBCV[Pa\x05\xBF`\x02\x83a\x0B\x19V[\x91P[\x81\x15a\x06\xD5W`\0[\x82\x81\x10\x15a\x06\xC2W`\x02\x82a\x05\xE0\x83\x83a\n{V[\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\x0BCV[` \x02` \x01\x01Q\x83\x83`\x02a\x06\x06\x91\x90a\n{V[a\x06\x11\x90`\x01a\nhV[\x81Q\x81\x10a\x06!Wa\x06!a\x0BCV[` \x02` \x01\x01Q`@Q` \x01a\x06C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06]\x91a\x0BYV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06zW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9D\x91\x90a\x0B\x88V[\x82\x82\x81Q\x81\x10a\x06\xAFWa\x06\xAFa\x0BCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xCBV[Pa\x06\xCE`\x02\x83a\x0B\x19V[\x91Pa\x05\xC2V[\x80`\0\x81Q\x81\x10a\x06\xE8Wa\x06\xE8a\x0BCV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0\x80a\x07\x07`\x04\x84a\x0B\xA1V[a\x07\x12\x90`@a\x0B\xCBV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x07\x88\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[\x91PP[\x92\x91PPV[`\0\x83Q`\0\x14\x15\x80\x15a\x07\xB1WP` \x84Qa\x07\xAF\x91\x90a\x0B\xF2V[\x15[a\x07\xCEW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a\x08eWa\x07\xF2`\x02\x85a\x0B\xF2V[`\0\x03a\x08(W\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa\x08\x1DW`\0\x80\xFD[`\x02\x84\x04\x93Pa\x08SV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa\x08LW`\0\x80\xFD[`\x02\x84\x04\x93P[a\x08^` \x82a\nhV[\x90Pa\x07\xDFV[PQ\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15a\x08\x81W`\0\x80\xFD[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x9AW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB8W`\0\x80\xFD[a\x08\xC4\x85\x82\x86\x01a\x08oV[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xE0W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\x10W`\0\x80\xFD[\x92P\x92\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t,W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\tJW`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\thW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\tyW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x90W`\0\x80\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\t\xA5W`\0\x80\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC9W`\0\x80\xFD[a\t\xD5\x89\x82\x8A\x01a\x08\xCEV[\x90\x94P\x92Pa\t\xE8\x90P``\x88\x01a\t\x17V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\tW`\0\x80\xFD[\x835\x92Pa\n\x19` \x85\x01a\t\x17V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n5W`\0\x80\xFD[\x84\x01``\x81\x87\x03\x12\x15a\nGW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x8CWa\x07\x8Ca\nRV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\x8CWa\x07\x8Ca\nRV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\n\xA9W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xC4W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\x10W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x0B\x06Wa\x0B\x06a\n\xD9V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\0\x82a\x0B(Wa\x0B(a\n\xD9V[P\x04\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x0BzW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x0B`V[P`\0\x92\x01\x91\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x9AW`\0\x80\xFD[PQ\x91\x90PV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x0B\xB8Wa\x0B\xB8a\n\xD9V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x0B\xEBWa\x0B\xEBa\nRV[P\x92\x91PPV[`\0\x82a\x0C\x01Wa\x0C\x01a\n\xD9V[P\x06\x90V\xFE\xA2dipfsX\"\x12 \x95*\xA3\x94\\>\x07]\xE1\xF6\xF2\r\xD8XWc\t\xD9ti\xC7\xE9\xEF\xA3\xBB\xB7\x0F\x9D\x07\x05\xE5\xA9dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506004361061004c5760003560e01c80630d361f3a14610051578063256f222b1461006657806331f60d4c146100795780639030a9bb1461008c575b600080fd5b61006461005f366004610887565b61009f565b005b610064610074366004610931565b6100ad565b6100646100873660046109f4565b6100c3565b61006461009a366004610887565b6100d4565b6100a982826100de565b5050565b6100bb868686868686610195565b505050505050565b6100ce8383836102c0565b50505050565b6100a982826103a1565b6100ea60056003610a68565b6100f5906020610a7b565b6101026020830183610a92565b905014610122576040516313717da960e21b815260040160405180910390fd5b606c6101736101346020840184610a92565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084610447565b610190576040516309bde33960e01b815260040160405180910390fd5b505050565b600884146101b65760405163200591bd60e01b815260040160405180910390fd5b60056101c460286001610a68565b6101ce9190610a68565b6101d9906020610a7b565b82146101f8576040516313717da960e21b815260040160405180910390fd5b600061023686868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061045f92505050565b9050600064ffffffffff831661024e60286001610a68565b600b901b17905061029985858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050610447565b6102b6576040516309bde33960e01b815260040160405180910390fd5b5050505050505050565b60006102ce60266001610a68565b6102d9906020610a7b565b6102e66040840184610a92565b905014610306576040516313717da960e21b815260040160405180910390fd5b6000610313600485610aef565b64ffffffffff16905061036d61032c6040850185610a92565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584610447565b61038a576040516309bde33960e01b815260040160405180910390fd5b6103988360200135856106f9565b95945050505050565b6103ad60036020610a7b565b6103ba6020830183610a92565b9050146103da576040516313717da960e21b815260040160405180910390fd5b61042a6103ea6020830183610a92565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003610447565b6100a9576040516309bde33960e01b815260040160405180910390fd5b600083610455868585610792565b1495945050505050565b600080600283516104709190610b19565b905060008167ffffffffffffffff81111561048d5761048d610b2d565b6040519080825280602002602001820160405280156104b6578160200160208202803683370190505b50905060005b828110156105b3576002856104d18383610a7b565b815181106104e1576104e1610b43565b6020026020010151868360026104f79190610a7b565b610502906001610a68565b8151811061051257610512610b43565b6020026020010151604051602001610534929190918252602082015260400190565b60408051601f198184030181529082905261054e91610b59565b602060405180830381855afa15801561056b573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061058e9190610b88565b8282815181106105a0576105a0610b43565b60209081029190910101526001016104bc565b506105bf600283610b19565b91505b81156106d55760005b828110156106c2576002826105e08383610a7b565b815181106105f0576105f0610b43565b6020026020010151838360026106069190610a7b565b610611906001610a68565b8151811061062157610621610b43565b6020026020010151604051602001610643929190918252602082015260400190565b60408051601f198184030181529082905261065d91610b59565b602060405180830381855afa15801561067a573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061069d9190610b88565b8282815181106106af576106af610b43565b60209081029190910101526001016105cb565b506106ce600283610b19565b91506105c2565b806000815181106106e8576106e8610b43565b602002602001015192505050919050565b600080610707600484610ba1565b610712906040610bcb565b64ffffffffff16905061078884821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b9150505b92915050565b600083516000141580156107b15750602084516107af9190610bf2565b155b6107ce576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111610865576107f2600285610bf2565b600003610828578151600052808601516020526020826040600060026107d05a03fa61081d57600080fd5b600284049350610853565b8086015160005281516020526020826040600060026107d05a03fa61084c57600080fd5b6002840493505b61085e602082610a68565b90506107df565b5051949350505050565b60006040828403121561088157600080fd5b50919050565b6000806040838503121561089a57600080fd5b82359150602083013567ffffffffffffffff8111156108b857600080fd5b6108c48582860161086f565b9150509250929050565b60008083601f8401126108e057600080fd5b50813567ffffffffffffffff8111156108f857600080fd5b60208301915083602082850101111561091057600080fd5b9250929050565b803564ffffffffff8116811461092c57600080fd5b919050565b6000806000806000806080878903121561094a57600080fd5b86359550602087013567ffffffffffffffff81111561096857600080fd5b8701601f8101891361097957600080fd5b803567ffffffffffffffff81111561099057600080fd5b8960208260051b84010111156109a557600080fd5b60209190910195509350604087013567ffffffffffffffff8111156109c957600080fd5b6109d589828a016108ce565b90945092506109e8905060608801610917565b90509295509295509295565b600080600060608486031215610a0957600080fd5b83359250610a1960208501610917565b9150604084013567ffffffffffffffff811115610a3557600080fd5b840160608187031215610a4757600080fd5b809150509250925092565b634e487b7160e01b600052601160045260246000fd5b8082018082111561078c5761078c610a52565b808202811582820484141761078c5761078c610a52565b6000808335601e19843603018112610aa957600080fd5b83018035915067ffffffffffffffff821115610ac457600080fd5b60200191503681900382131561091057600080fd5b634e487b7160e01b600052601260045260246000fd5b600064ffffffffff831680610b0657610b06610ad9565b8064ffffffffff84160491505092915050565b600082610b2857610b28610ad9565b500490565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6000825160005b81811015610b7a5760208186018101518583015201610b60565b506000920191825250919050565b600060208284031215610b9a57600080fd5b5051919050565b600064ffffffffff831680610bb857610bb8610ad9565b8064ffffffffff84160691505092915050565b64ffffffffff8181168382160290811690818114610beb57610beb610a52565b5092915050565b600082610c0157610c01610ad9565b50069056fea2646970667358221220952aa3945c3e075de1f6f20dd858576309d97469c7e9efa3bbb70f9d0705e5a964736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\r6\x1F:\x14a\0QW\x80c%o\"+\x14a\0fW\x80c1\xF6\rL\x14a\0yW\x80c\x900\xA9\xBB\x14a\0\x8CW[`\0\x80\xFD[a\0da\0_6`\x04a\x08\x87V[a\0\x9FV[\0[a\0da\0t6`\x04a\t1V[a\0\xADV[a\0da\0\x876`\x04a\t\xF4V[a\0\xC3V[a\0da\0\x9A6`\x04a\x08\x87V[a\0\xD4V[a\0\xA9\x82\x82a\0\xDEV[PPV[a\0\xBB\x86\x86\x86\x86\x86\x86a\x01\x95V[PPPPPPV[a\0\xCE\x83\x83\x83a\x02\xC0V[PPPPV[a\0\xA9\x82\x82a\x03\xA1V[a\0\xEA`\x05`\x03a\nhV[a\0\xF5\x90` a\n{V[a\x01\x02` \x83\x01\x83a\n\x92V[\x90P\x14a\x01\"W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`la\x01sa\x014` \x84\x01\x84a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a\x04GV[a\x01\x90W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPV[`\x08\x84\x14a\x01\xB6W`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05a\x01\xC4`(`\x01a\nhV[a\x01\xCE\x91\x90a\nhV[a\x01\xD9\x90` a\n{V[\x82\x14a\x01\xF8W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x026\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x04_\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a\x02N`(`\x01a\nhV[`\x0B\x90\x1B\x17\x90Pa\x02\x99\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa\x04GV[a\x02\xB6W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0a\x02\xCE`&`\x01a\nhV[a\x02\xD9\x90` a\n{V[a\x02\xE6`@\x84\x01\x84a\n\x92V[\x90P\x14a\x03\x06W`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x13`\x04\x85a\n\xEFV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x03ma\x03,`@\x85\x01\x85a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a\x04GV[a\x03\x8AW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x98\x83` \x015\x85a\x06\xF9V[\x95\x94PPPPPV[a\x03\xAD`\x03` a\n{V[a\x03\xBA` \x83\x01\x83a\n\x92V[\x90P\x14a\x03\xDAW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04*a\x03\xEA` \x83\x01\x83a\n\x92V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a\x04GV[a\0\xA9W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x83a\x04U\x86\x85\x85a\x07\x92V[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa\x04p\x91\x90a\x0B\x19V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x04\x8DWa\x04\x8Da\x0B-V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x04\xB6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x05\xB3W`\x02\x85a\x04\xD1\x83\x83a\n{V[\x81Q\x81\x10a\x04\xE1Wa\x04\xE1a\x0BCV[` \x02` \x01\x01Q\x86\x83`\x02a\x04\xF7\x91\x90a\n{V[a\x05\x02\x90`\x01a\nhV[\x81Q\x81\x10a\x05\x12Wa\x05\x12a\x0BCV[` \x02` \x01\x01Q`@Q` \x01a\x054\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05N\x91a\x0BYV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x05kW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x8E\x91\x90a\x0B\x88V[\x82\x82\x81Q\x81\x10a\x05\xA0Wa\x05\xA0a\x0BCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\xBCV[Pa\x05\xBF`\x02\x83a\x0B\x19V[\x91P[\x81\x15a\x06\xD5W`\0[\x82\x81\x10\x15a\x06\xC2W`\x02\x82a\x05\xE0\x83\x83a\n{V[\x81Q\x81\x10a\x05\xF0Wa\x05\xF0a\x0BCV[` \x02` \x01\x01Q\x83\x83`\x02a\x06\x06\x91\x90a\n{V[a\x06\x11\x90`\x01a\nhV[\x81Q\x81\x10a\x06!Wa\x06!a\x0BCV[` \x02` \x01\x01Q`@Q` \x01a\x06C\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x06]\x91a\x0BYV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x06zW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x9D\x91\x90a\x0B\x88V[\x82\x82\x81Q\x81\x10a\x06\xAFWa\x06\xAFa\x0BCV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x05\xCBV[Pa\x06\xCE`\x02\x83a\x0B\x19V[\x91Pa\x05\xC2V[\x80`\0\x81Q\x81\x10a\x06\xE8Wa\x06\xE8a\x0BCV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0\x80a\x07\x07`\x04\x84a\x0B\xA1V[a\x07\x12\x90`@a\x0B\xCBV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x07\x88\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[\x91PP[\x92\x91PPV[`\0\x83Q`\0\x14\x15\x80\x15a\x07\xB1WP` \x84Qa\x07\xAF\x91\x90a\x0B\xF2V[\x15[a\x07\xCEW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a\x08eWa\x07\xF2`\x02\x85a\x0B\xF2V[`\0\x03a\x08(W\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa\x08\x1DW`\0\x80\xFD[`\x02\x84\x04\x93Pa\x08SV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa\x08LW`\0\x80\xFD[`\x02\x84\x04\x93P[a\x08^` \x82a\nhV[\x90Pa\x07\xDFV[PQ\x94\x93PPPPV[`\0`@\x82\x84\x03\x12\x15a\x08\x81W`\0\x80\xFD[P\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x08\x9AW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xB8W`\0\x80\xFD[a\x08\xC4\x85\x82\x86\x01a\x08oV[\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x08\xE0W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF8W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\t\x10W`\0\x80\xFD[\x92P\x92\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\t,W`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a\tJW`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\thW`\0\x80\xFD[\x87\x01`\x1F\x81\x01\x89\x13a\tyW`\0\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\x90W`\0\x80\xFD[\x89` \x82`\x05\x1B\x84\x01\x01\x11\x15a\t\xA5W`\0\x80\xFD[` \x91\x90\x91\x01\x95P\x93P`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\xC9W`\0\x80\xFD[a\t\xD5\x89\x82\x8A\x01a\x08\xCEV[\x90\x94P\x92Pa\t\xE8\x90P``\x88\x01a\t\x17V[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0\x80`\0``\x84\x86\x03\x12\x15a\n\tW`\0\x80\xFD[\x835\x92Pa\n\x19` \x85\x01a\t\x17V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\n5W`\0\x80\xFD[\x84\x01``\x81\x87\x03\x12\x15a\nGW`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x07\x8CWa\x07\x8Ca\nRV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07\x8CWa\x07\x8Ca\nRV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\n\xA9W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\n\xC4W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\t\x10W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x0B\x06Wa\x0B\x06a\n\xD9V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\0\x82a\x0B(Wa\x0B(a\n\xD9V[P\x04\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82Q`\0[\x81\x81\x10\x15a\x0BzW` \x81\x86\x01\x81\x01Q\x85\x83\x01R\x01a\x0B`V[P`\0\x92\x01\x91\x82RP\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x9AW`\0\x80\xFD[PQ\x91\x90PV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a\x0B\xB8Wa\x0B\xB8a\n\xD9V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x0B\xEBWa\x0B\xEBa\nRV[P\x92\x91PPV[`\0\x82a\x0C\x01Wa\x0C\x01a\n\xD9V[P\x06\x90V\xFE\xA2dipfsX\"\x12 \x95*\xA3\x94\\>\x07]\xE1\xF6\xF2\r\xD8XWc\t\xD9ti\xC7\xE9\xEF\xA3\xBB\xB7\x0F\x9D\x07\x05\xE5\xA9dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `InvalidProof()` and selector `0x09bde339`.
```solidity
error InvalidProof();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidProof {}
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
        impl ::core::convert::From<InvalidProof> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidProof) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidProof {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidProof()";
            const SELECTOR: [u8; 4] = [9u8, 189u8, 227u8, 57u8];
            #[inline]
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
    /**Custom error with signature `InvalidValidatorFieldsLength()` and selector `0x200591bd`.
```solidity
error InvalidValidatorFieldsLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidValidatorFieldsLength {}
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
        impl ::core::convert::From<InvalidValidatorFieldsLength>
        for UnderlyingRustTuple<'_> {
            fn from(value: InvalidValidatorFieldsLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InvalidValidatorFieldsLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidValidatorFieldsLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidValidatorFieldsLength()";
            const SELECTOR: [u8; 4] = [32u8, 5u8, 145u8, 189u8];
            #[inline]
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
    /**Function with signature `verifyBalanceContainer(bytes32,(bytes32,bytes))` and selector `0x0d361f3a`.
```solidity
function verifyBalanceContainer(bytes32 beaconBlockRoot, BeaconChainProofs.BalanceContainerProof memory proof) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceContainerCall {
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`verifyBalanceContainer(bytes32,(bytes32,bytes))`](verifyBalanceContainerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyBalanceContainerReturn {}
    #[allow(
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
                BeaconChainProofs::BalanceContainerProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyBalanceContainerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceContainerCall) -> Self {
                    (value.beaconBlockRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceContainerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconBlockRoot: tuple.0,
                        proof: tuple.1,
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
            impl ::core::convert::From<verifyBalanceContainerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyBalanceContainerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyBalanceContainerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyBalanceContainerCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BeaconChainProofs::BalanceContainerProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyBalanceContainerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyBalanceContainer(bytes32,(bytes32,bytes))";
            const SELECTOR: [u8; 4] = [13u8, 54u8, 31u8, 58u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconBlockRoot),
                    <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
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
    /**Function with signature `verifyStateRoot(bytes32,(bytes32,bytes))` and selector `0x9030a9bb`.
```solidity
function verifyStateRoot(bytes32 beaconBlockRoot, BeaconChainProofs.StateRootProof memory proof) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStateRootCall {
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`verifyStateRoot(bytes32,(bytes32,bytes))`](verifyStateRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStateRootReturn {}
    #[allow(
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
                BeaconChainProofs::StateRootProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyStateRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStateRootCall) -> Self {
                    (value.beaconBlockRoot, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStateRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconBlockRoot: tuple.0,
                        proof: tuple.1,
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
            impl ::core::convert::From<verifyStateRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyStateRootReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyStateRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyStateRootCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BeaconChainProofs::StateRootProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStateRootReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyStateRoot(bytes32,(bytes32,bytes))";
            const SELECTOR: [u8; 4] = [144u8, 48u8, 169u8, 187u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconBlockRoot),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
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
    /**Function with signature `verifyValidatorBalance(bytes32,uint40,(bytes32,bytes32,bytes))` and selector `0x31f60d4c`.
```solidity
function verifyValidatorBalance(bytes32 balanceContainerRoot, uint40 validatorIndex, BeaconChainProofs.BalanceProof memory proof) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyValidatorBalanceCall {
        pub balanceContainerRoot: alloy::sol_types::private::FixedBytes<32>,
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        pub proof: <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`verifyValidatorBalance(bytes32,uint40,(bytes32,bytes32,bytes))`](verifyValidatorBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyValidatorBalanceReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<40>,
                BeaconChainProofs::BalanceProof,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U40,
                <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<verifyValidatorBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyValidatorBalanceCall) -> Self {
                    (value.balanceContainerRoot, value.validatorIndex, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyValidatorBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        balanceContainerRoot: tuple.0,
                        validatorIndex: tuple.1,
                        proof: tuple.2,
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
            impl ::core::convert::From<verifyValidatorBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyValidatorBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyValidatorBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyValidatorBalanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<40>,
                BeaconChainProofs::BalanceProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyValidatorBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyValidatorBalance(bytes32,uint40,(bytes32,bytes32,bytes))";
            const SELECTOR: [u8; 4] = [49u8, 246u8, 13u8, 76u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceContainerRoot),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
                    <BeaconChainProofs::BalanceProof as alloy_sol_types::SolType>::tokenize(
                        &self.proof,
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
    /**Function with signature `verifyValidatorFields(bytes32,bytes32[],bytes,uint40)` and selector `0x256f222b`.
```solidity
function verifyValidatorFields(bytes32 beaconStateRoot, bytes32[] memory validatorFields, bytes memory validatorFieldsProof, uint40 validatorIndex) external view;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyValidatorFieldsCall {
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
        pub validatorFieldsProof: alloy::sol_types::private::Bytes,
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`verifyValidatorFields(bytes32,bytes32[],bytes,uint40)`](verifyValidatorFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyValidatorFieldsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<40>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<verifyValidatorFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyValidatorFieldsCall) -> Self {
                    (
                        value.beaconStateRoot,
                        value.validatorFields,
                        value.validatorFieldsProof,
                        value.validatorIndex,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyValidatorFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconStateRoot: tuple.0,
                        validatorFields: tuple.1,
                        validatorFieldsProof: tuple.2,
                        validatorIndex: tuple.3,
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
            impl ::core::convert::From<verifyValidatorFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyValidatorFieldsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyValidatorFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyValidatorFieldsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<40>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyValidatorFieldsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyValidatorFields(bytes32,bytes32[],bytes,uint40)";
            const SELECTOR: [u8; 4] = [37u8, 111u8, 34u8, 43u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.validatorFieldsProof,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        40,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndex),
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
    ///Container for all the [`BeaconChainProofsWrapper`](self) function calls.
    pub enum BeaconChainProofsWrapperCalls {
        verifyBalanceContainer(verifyBalanceContainerCall),
        verifyStateRoot(verifyStateRootCall),
        verifyValidatorBalance(verifyValidatorBalanceCall),
        verifyValidatorFields(verifyValidatorFieldsCall),
    }
    #[automatically_derived]
    impl BeaconChainProofsWrapperCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 54u8, 31u8, 58u8],
            [37u8, 111u8, 34u8, 43u8],
            [49u8, 246u8, 13u8, 76u8],
            [144u8, 48u8, 169u8, 187u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BeaconChainProofsWrapperCalls {
        const NAME: &'static str = "BeaconChainProofsWrapperCalls";
        const MIN_DATA_LENGTH: usize = 128usize;
        const COUNT: usize = 4usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::verifyBalanceContainer(_) => {
                    <verifyBalanceContainerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyStateRoot(_) => {
                    <verifyStateRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyValidatorBalance(_) => {
                    <verifyValidatorBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyValidatorFields(_) => {
                    <verifyValidatorFieldsCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<BeaconChainProofsWrapperCalls>] = &[
                {
                    fn verifyBalanceContainer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperCalls> {
                        <verifyBalanceContainerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperCalls::verifyBalanceContainer)
                    }
                    verifyBalanceContainer
                },
                {
                    fn verifyValidatorFields(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperCalls> {
                        <verifyValidatorFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperCalls::verifyValidatorFields)
                    }
                    verifyValidatorFields
                },
                {
                    fn verifyValidatorBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperCalls> {
                        <verifyValidatorBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperCalls::verifyValidatorBalance)
                    }
                    verifyValidatorBalance
                },
                {
                    fn verifyStateRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperCalls> {
                        <verifyStateRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperCalls::verifyStateRoot)
                    }
                    verifyStateRoot
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
                Self::verifyBalanceContainer(inner) => {
                    <verifyBalanceContainerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyStateRoot(inner) => {
                    <verifyStateRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyValidatorBalance(inner) => {
                    <verifyValidatorBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyValidatorFields(inner) => {
                    <verifyValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::verifyBalanceContainer(inner) => {
                    <verifyBalanceContainerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyStateRoot(inner) => {
                    <verifyStateRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyValidatorBalance(inner) => {
                    <verifyValidatorBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyValidatorFields(inner) => {
                    <verifyValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BeaconChainProofsWrapper`](self) custom errors.
    pub enum BeaconChainProofsWrapperErrors {
        InvalidProof(InvalidProof),
        InvalidProofLength(InvalidProofLength),
        InvalidValidatorFieldsLength(InvalidValidatorFieldsLength),
    }
    #[automatically_derived]
    impl BeaconChainProofsWrapperErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 189u8, 227u8, 57u8],
            [32u8, 5u8, 145u8, 189u8],
            [77u8, 197u8, 246u8, 164u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BeaconChainProofsWrapperErrors {
        const NAME: &'static str = "BeaconChainProofsWrapperErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 3usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::InvalidProof(_) => {
                    <InvalidProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidValidatorFieldsLength(_) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<BeaconChainProofsWrapperErrors>] = &[
                {
                    fn InvalidProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn InvalidValidatorFieldsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperErrors> {
                        <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                BeaconChainProofsWrapperErrors::InvalidValidatorFieldsLength,
                            )
                    }
                    InvalidValidatorFieldsLength
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainProofsWrapperErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainProofsWrapperErrors::InvalidProofLength)
                    }
                    InvalidProofLength
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
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidValidatorFieldsLength(inner) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InvalidValidatorFieldsLength(inner) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`BeaconChainProofsWrapper`](self) contract instance.

See the [wrapper's documentation](`BeaconChainProofsWrapperInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BeaconChainProofsWrapperInstance<T, P, N> {
        BeaconChainProofsWrapperInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BeaconChainProofsWrapperInstance<T, P, N>>,
    > {
        BeaconChainProofsWrapperInstance::<T, P, N>::deploy(provider)
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
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        BeaconChainProofsWrapperInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`BeaconChainProofsWrapper`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BeaconChainProofsWrapper`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BeaconChainProofsWrapperInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BeaconChainProofsWrapperInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BeaconChainProofsWrapperInstance")
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
    > BeaconChainProofsWrapperInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BeaconChainProofsWrapper`](self) contract instance.

See the [wrapper's documentation](`BeaconChainProofsWrapperInstance`) for more details.*/
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
        ) -> alloy_contract::Result<BeaconChainProofsWrapperInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
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
    impl<T, P: ::core::clone::Clone, N> BeaconChainProofsWrapperInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BeaconChainProofsWrapperInstance<T, P, N> {
            BeaconChainProofsWrapperInstance {
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
    > BeaconChainProofsWrapperInstance<T, P, N> {
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
        ///Creates a new call builder for the [`verifyBalanceContainer`] function.
        pub fn verifyBalanceContainer(
            &self,
            beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyBalanceContainerCall, N> {
            self.call_builder(
                &verifyBalanceContainerCall {
                    beaconBlockRoot,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`verifyStateRoot`] function.
        pub fn verifyStateRoot(
            &self,
            beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
            proof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyStateRootCall, N> {
            self.call_builder(
                &verifyStateRootCall {
                    beaconBlockRoot,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`verifyValidatorBalance`] function.
        pub fn verifyValidatorBalance(
            &self,
            balanceContainerRoot: alloy::sol_types::private::FixedBytes<32>,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
            proof: <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyValidatorBalanceCall, N> {
            self.call_builder(
                &verifyValidatorBalanceCall {
                    balanceContainerRoot,
                    validatorIndex,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`verifyValidatorFields`] function.
        pub fn verifyValidatorFields(
            &self,
            beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
            validatorFieldsProof: alloy::sol_types::private::Bytes,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyValidatorFieldsCall, N> {
            self.call_builder(
                &verifyValidatorFieldsCall {
                    beaconStateRoot,
                    validatorFields,
                    validatorFieldsProof,
                    validatorIndex,
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
    > BeaconChainProofsWrapperInstance<T, P, N> {
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
