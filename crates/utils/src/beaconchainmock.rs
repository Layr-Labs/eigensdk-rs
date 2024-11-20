///Module containing a contract's types and functions.
/**

```solidity
library BeaconChainProofs {
    struct BalanceContainerProof { bytes32 balanceContainerRoot; bytes proof; }
    struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
    struct ValidatorProof { bytes32[] validatorFields; bytes proof; }
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
    /**```solidity
struct ValidatorProof { bytes32[] validatorFields; bytes proof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorProof {
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
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
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::FixedBytes<32>,
            >,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
        impl ::core::convert::From<ValidatorProof> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorProof) -> Self {
                (value.validatorFields, value.proof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorProof {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    validatorFields: tuple.0,
                    proof: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ValidatorProof {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ValidatorProof {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorFields),
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
        impl alloy_sol_types::SolType for ValidatorProof {
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
        impl alloy_sol_types::SolStruct for ValidatorProof {
            const NAME: &'static str = "ValidatorProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorProof(bytes32[] validatorFields,bytes proof)",
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorFields,
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
        impl alloy_sol_types::EventTopic for ValidatorProof {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorFields,
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
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorFields,
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
    struct ValidatorProof {
        bytes32[] validatorFields;
        bytes proof;
    }
}

library StdInvariant {
    struct FuzzSelector {
        address addr;
        bytes4[] selectors;
    }
}

interface BeaconChainMock {
    struct CheckpointProofs {
        BeaconChainProofs.BalanceContainerProof balanceContainerProof;
        BeaconChainProofs.BalanceProof[] balanceProofs;
    }
    struct CredentialProofs {
        uint64 beaconTimestamp;
        BeaconChainProofs.StateRootProof stateRootProof;
        bytes[] validatorFieldsProofs;
        bytes32[][] validatorFields;
    }
    struct StaleBalanceProofs {
        uint64 beaconTimestamp;
        BeaconChainProofs.StateRootProof stateRootProof;
        BeaconChainProofs.ValidatorProof validatorProof;
    }

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

    constructor(address _eigenPodManager, uint64 _genesisTime);

    function CONSENSUS_REWARD_AMOUNT_GWEI() external view returns (uint64);
    function IS_TEST() external view returns (bool);
    function NAME() external pure returns (string memory);
    function SLASH_AMOUNT_GWEI() external view returns (uint64);
    function _advanceEpoch() external;
    function advanceEpoch() external;
    function advanceEpoch_NoRewards() external;
    function advanceEpoch_NoWithdraw() external;
    function balanceOfGwei(uint40 validatorIndex) external view returns (uint64);
    function currentBalance(uint40 validatorIndex) external view returns (uint64);
    function currentEpoch() external view returns (uint64);
    function effectiveBalance(uint40 validatorIndex) external view returns (uint64);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function exitEpoch(uint40 validatorIndex) external view returns (uint64);
    function exitValidator(uint40 validatorIndex) external returns (uint64 exitedBalanceGwei);
    function failed() external returns (bool);
    function getBalanceRoot(uint40 validatorIndex) external view returns (bytes32);
    function getCheckpointProofs(uint40[] memory _validators, uint64 timestamp) external view returns (CheckpointProofs memory);
    function getCredentialProofs(uint40[] memory _validators) external view returns (CredentialProofs memory);
    function getPubkeyHashes(uint40[] memory _validators) external view returns (bytes32[] memory);
    function getStaleBalanceProofs(uint40 validatorIndex) external view returns (StaleBalanceProofs memory);
    function isActive(uint40 validatorIndex) external view returns (bool);
    function newValidator(bytes memory withdrawalCreds) external payable returns (uint40);
    function nextTimestamp() external view returns (uint64);
    function pubkey(uint40 validatorIndex) external pure returns (bytes memory);
    function pubkeyHash(uint40 validatorIndex) external view returns (bytes32);
    function slashValidators(uint40[] memory _validators) external returns (uint64 slashedBalanceGwei);
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
    function totalEffectiveBalanceWei(uint40[] memory validatorIndices) external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract EigenPodManager"
      },
      {
        "name": "_genesisTime",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "CONSENSUS_REWARD_AMOUNT_GWEI",
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
    "name": "NAME",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "string",
        "internalType": "string"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "SLASH_AMOUNT_GWEI",
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
    "name": "_advanceEpoch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "advanceEpoch",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "advanceEpoch_NoRewards",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "advanceEpoch_NoWithdraw",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOfGwei",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
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
    "name": "currentBalance",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
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
    "name": "currentEpoch",
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
    "name": "effectiveBalance",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
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
    "name": "exitEpoch",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
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
    "name": "exitValidator",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [
      {
        "name": "exitedBalanceGwei",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "getBalanceRoot",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCheckpointProofs",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      },
      {
        "name": "timestamp",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct CheckpointProofs",
        "components": [
          {
            "name": "balanceContainerProof",
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
          },
          {
            "name": "balanceProofs",
            "type": "tuple[]",
            "internalType": "struct BeaconChainProofs.BalanceProof[]",
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
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCredentialProofs",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct CredentialProofs",
        "components": [
          {
            "name": "beaconTimestamp",
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
            "name": "validatorFieldsProofs",
            "type": "bytes[]",
            "internalType": "bytes[]"
          },
          {
            "name": "validatorFields",
            "type": "bytes32[][]",
            "internalType": "bytes32[][]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getPubkeyHashes",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStaleBalanceProofs",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct StaleBalanceProofs",
        "components": [
          {
            "name": "beaconTimestamp",
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
            "name": "validatorProof",
            "type": "tuple",
            "internalType": "struct BeaconChainProofs.ValidatorProof",
            "components": [
              {
                "name": "validatorFields",
                "type": "bytes32[]",
                "internalType": "bytes32[]"
              },
              {
                "name": "proof",
                "type": "bytes",
                "internalType": "bytes"
              }
            ]
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "isActive",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
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
    "name": "newValidator",
    "inputs": [
      {
        "name": "withdrawalCreds",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "nextTimestamp",
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
    "name": "pubkey",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "pubkeyHash",
    "inputs": [
      {
        "name": "validatorIndex",
        "type": "uint40",
        "internalType": "uint40"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "slashValidators",
    "inputs": [
      {
        "name": "_validators",
        "type": "uint40[]",
        "internalType": "uint40[]"
      }
    ],
    "outputs": [
      {
        "name": "slashedBalanceGwei",
        "type": "uint64",
        "internalType": "uint64"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "totalEffectiveBalanceWei",
    "inputs": [
      {
        "name": "validatorIndices",
        "type": "uint40[]",
        "internalType": "uint40[]"
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
pub mod BeaconChainMock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61010060405260008054600160ff199182168117909255600480549091169091179055601b80546001600160a01b031916737109709ecfa91a80626ff3989d68f67f5b1dd12d17905561005460036020610637565b608052600561006560286001610654565b61006f9190610654565b61007a906020610637565b60a05261008960056003610654565b610094906020610637565b60c0526100a360266001610654565b6100ae906020610637565b60e0523480156100bd57600080fd5b506040516159d33803806159d38339810160408190526100dc91610667565b601b80546001600160401b038316600160a01b02600160a01b600160e01b031982168117909255601c80546001600160a01b038087166801000000000000000002600160401b600160e01b0319909216919091179091556040519281169116179063b4d6c78290720f3df6d732807ef1319fb7b8bb8522d0beac0290610164602082016105b4565b6020820181038252601f19601f820116604052506040518363ffffffff1660e01b81526004016101959291906106dd565b600060405180830381600087803b1580156101af57600080fd5b505af11580156101c3573d6000803e3d6000fd5b50506040805160088082526101208201909252600093506101f492509060208201610100803683370190505061031b565b604080516064808252610ca0820190925291925060208201610c8080368337505081516102289260269250602001906105c1565b5080602660008154811061023e5761023e610735565b60009182526020909120015560015b60265481101561031257604080516020810184905290810183905260029060600160408051601f19818403018152908290526102889161074b565b602060405180830381855afa1580156102a5573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906102c89190610767565b602682815481106102db576102db610735565b9060005260206000200181905550602681815481106102fc576102fc610735565b600091825260209091200154915060010161024d565b505050506107a2565b6000806002835161032c9190610780565b90506000816001600160401b038111156103485761034861071f565b604051908082528060200260200182016040528015610371578160200160208202803683370190505b50905060005b8281101561046e5760028561038c8383610637565b8151811061039c5761039c610735565b6020026020010151868360026103b29190610637565b6103bd906001610654565b815181106103cd576103cd610735565b60200260200101516040516020016103ef929190918252602082015260400190565b60408051601f19818403018152908290526104099161074b565b602060405180830381855afa158015610426573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906104499190610767565b82828151811061045b5761045b610735565b6020908102919091010152600101610377565b5061047a600283610780565b91505b81156105905760005b8281101561057d5760028261049b8383610637565b815181106104ab576104ab610735565b6020026020010151838360026104c19190610637565b6104cc906001610654565b815181106104dc576104dc610735565b60200260200101516040516020016104fe929190918252602082015260400190565b60408051601f19818403018152908290526105189161074b565b602060405180830381855afa158015610535573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906105589190610767565b82828151811061056a5761056a610735565b6020908102919091010152600101610486565b50610589600283610780565b915061047d565b806000815181106105a3576105a3610735565b602002602001015192505050919050565b61029c8061573783390190565b8280548282559060005260206000209081019282156105fc579160200282015b828111156105fc5782518255916020019190600101906105e1565b5061060892915061060c565b5090565b5b80821115610608576000815560010161060d565b634e487b7160e01b600052601160045260246000fd5b808202811582820484141761064e5761064e610621565b92915050565b8082018082111561064e5761064e610621565b6000806040838503121561067a57600080fd5b82516001600160a01b038116811461069157600080fd5b60208401519092506001600160401b03811681146106ae57600080fd5b809150509250929050565b60005b838110156106d45781810151838201526020016106bc565b50506000910152565b60018060a01b0383168152604060208201526000825180604084015261070a8160608501602087016106b9565b601f01601f1916919091016060019392505050565b634e487b7160e01b600052604160045260246000fd5b634e487b7160e01b600052603260045260246000fd5b6000825161075d8184602087016106b9565b9190910192915050565b60006020828403121561077957600080fd5b5051919050565b60008261079d57634e487b7160e01b600052601260045260246000fd5b500490565b60805160a05160c05160e051614f556107e26000396000613108015260008181612d410152612dae01526000612f1401526000612c260152614f556000f3fe6080604052600436106102045760003560e01c806386a6f9e111610118578063c76f25c0116100a0578063f0acd9881161006f578063f0acd988146105c6578063f7213873146105db578063f833eb63146105fb578063f8f98a4e1461061b578063fa7626d41461063b57600080fd5b8063c76f25c014610546578063e20c9f7114610573578063e3cefb4214610588578063ed3c16051461059d57600080fd5b8063a50a3a1a116100e7578063a50a3a1a14610492578063aa47389c146104bf578063b1b6f6a1146104ef578063b5508aa91461051c578063ba414fa61461053157600080fd5b806386a6f9e1146102da578063908820e014610429578063916a17c614610449578063a3f4df7e1461045e57600080fd5b80633cf80e6c1161019b5780635e6cc2fc1161016a5780635e6cc2fc1461038357806366d9a9a0146103b05780636b3abd97146103d257806376671808146103f257806385226c811461040757600080fd5b80633cf80e6c1461032f5780633e5e3c23146103445780633f7286f41461035957806359d095dd1461036e57600080fd5b806329992faa116101d757806329992faa146102c35780632def6009146102da578063330bc27e146102fa578063357e951f1461030f57600080fd5b806314360958146102095780631ed7831c146102465780631f54365c1461026857806323e82c4c14610296575b600080fd5b34801561021557600080fd5b5061022961022436600461433b565b610655565b6040516001600160401b0390911681526020015b60405180910390f35b34801561025257600080fd5b5061025b6107cc565b60405161023d919061436f565b34801561027457600080fd5b506102886102833660046143bb565b61082e565b60405190815260200161023d565b3480156102a257600080fd5b506102b66102b13660046143bb565b610863565b60405161023d9190614488565b3480156102cf57600080fd5b506102d8610a7d565b005b3480156102e657600080fd5b506102296102f53660046143bb565b610f3f565b34801561030657600080fd5b50610229600a81565b34801561031b57600080fd5b50601c54610229906001600160401b031681565b34801561033b57600080fd5b506102d8610f7e565b34801561035057600080fd5b5061025b610fc5565b34801561036557600080fd5b5061025b611025565b34801561037a57600080fd5b506102d8611085565b34801561038f57600080fd5b506103a361039e3660046143bb565b6110bc565b60405161023d91906144f8565b3480156103bc57600080fd5b506103c56110ec565b60405161023d919061450b565b3480156103de57600080fd5b506102886103ed36600461433b565b6111db565b3480156103fe57600080fd5b50610229611264565b34801561041357600080fd5b5061041c61132b565b60405161023d91906145c5565b34801561043557600080fd5b506102886104443660046143bb565b6113fb565b34801561045557600080fd5b506103c5611429565b34801561046a57600080fd5b5060408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b60208201526103a3565b34801561049e57600080fd5b506104b26104ad36600461433b565b61150f565b60405161023d919061467a565b3480156104cb57600080fd5b506104df6104da3660046143bb565b611935565b604051901515815260200161023d565b3480156104fb57600080fd5b5061050f61050a366004614732565b611986565b60405161023d919061478f565b34801561052857600080fd5b5061041c611d48565b34801561053d57600080fd5b506104df611e18565b34801561055257600080fd5b5061056661056136600461433b565b611f43565b60405161023d9190614835565b34801561057f57600080fd5b5061025b612002565b34801561059457600080fd5b50610229600181565b6105b06105ab366004614894565b612062565b60405164ffffffffff909116815260200161023d565b3480156105d257600080fd5b506102d861221f565b3480156105e757600080fd5b506102296105f63660046143bb565b612265565b34801561060757600080fd5b506102296106163660046143bb565b612279565b34801561062757600080fd5b506102296106363660046143bb565b6122bf565b34801561064757600080fd5b506000546104df9060ff1681565b60006106876040518060400160405280600f81526020016e736c61736856616c696461746f727360881b81525061256a565b60005b82518110156107c65760008382815181106106a7576106a7614913565b602002602001015190506000601d8264ffffffffff16815481106106cd576106cd614913565b60009182526020909120600490910201805490915060ff161561070b5760405162461bcd60e51b815260040161070290614929565b60405180910390fd5b8054610100900460ff1661076257805461ff00191661010017815561072e611264565b6107399060016149b8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055505b600061076d836125fa565b90506001600160401b038116600a11156107965761078b81866149b8565b9450600090506107b1565b6107a1600a866149b8565b94506107ae600a826149d7565b90505b6107bb8382612605565b50505060010161068a565b50919050565b6060600d80548060200260200160405190810160405280929190818152602001828054801561082457602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610806575b5050505050905090565b6000601d8264ffffffffff168154811061084a5761084a614913565b9060005260206000209060040201600101549050919050565b61086b614146565b6021546001600160401b0316600090815260246020908152604080832064ffffffffff8616845282528083208151815460609481028201850184529281018381529093919284928491908401828280156108e457602002820191906000526020600020905b8154815260200190600101908083116108d0575b505050505081526020016001820180546108fd906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054610929906149f6565b80156109765780601f1061094b57610100808354040283529160200191610976565b820191906000526020600020905b81548152906001019060200180831161095957829003601f168201915b505050919092525050604080516060810182526021546001600160401b03168082526000908152602260209081529083902083518085019094528054845260018101805496975092958287019550909291840191906109d4906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054610a00906149f6565b8015610a4d5780601f10610a2257610100808354040283529160200191610a4d565b820191906000526020600020905b815481529060010190602001808311610a3057829003601f168201915b50505091909252505050815260408051808201909152835181526020938401518185015292019190915292915050565b60005b601d54811015610b1a576000601d8281548110610a9f57610a9f614913565b60009182526020909120600490910201805490915060ff1615610ac25750610b12565b6000610acd836125fa565b9050640773594000816001600160401b03161115610aed57506407735940005b600391909101805467ffffffffffffffff19166001600160401b039092169190911790555b600101610a80565b50610b596040518060400160405280601c81526020017f2d2075706461746564206566666563746976652062616c616e6365730000000081525061266e565b610b9b6040518060400160405280601081526020016f05a5a40c6eae4e4cadce840cae0dec6d60831b815250610b8d611264565b6001600160401b031661269d565b6000610ba5611264565b601b549091506001600160a01b031663e5d6bf02610bc2836126da565b6040516001600160e01b031960e084901b1681526001600160401b039091166004820152602401600060405180830381600087803b158015610c0357600080fd5b505af1158015610c17573d6000803e3d6000fd5b50506021805467ffffffffffffffff1916426001600160401b0316179055505060408051808201909152601681527505a40d4eadae0cac840e8de40dccaf0e840cae0dec6d60531b6020820152610c7090610b8d611264565b610cae6040518060400160405280601d81526020017f2d206275696c64696e6720626561636f6e20737461746520747265657300000081525061266e565b601d5415610ccd57601d54610cc590600190614a2a565b602055610d87565b60215460405163159a829560e31b81526001600160401b0390911660048201527fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4706024820152720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a890604401600060405180830381600087803b158015610d4b57600080fd5b505af1158015610d5f573d6000803e3d6000fd5b50505050610d84604051806060016040528060288152602001614ed26028913961266e565b50565b6000610dbf610d9461271a565b610da060286001614a3d565b6021546001600160401b031660009081526027602052604090206127a9565b90506000610dfc610dce612a16565b610dda60266001614a3d565b6021546001600160401b031660009081526027602052604090206002016127a9565b90506000610e32610e0d8484612aaf565b6021546001600160401b031660009081526027602052604090206005906004016127a9565b90506000610e67610e4283612b5d565b6021546001600160401b031660009081526027602052604090206003906006016127a9565b9050610e9f604051806040016040528060148152602001730b4b4818995858dbdb88189b1bd8dac81c9bdbdd60621b81525082612be9565b60215460405163159a829560e31b81526001600160401b03909116600482015260248101829052720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a890604401600060405180830381600087803b158015610efe57600080fd5b505af1158015610f12573d6000803e3d6000fd5b50505050610f1f82612c22565b610f2883612d3d565b610f30612eea565b610f386130d4565b5050505050565b6000601d8264ffffffffff1681548110610f5b57610f5b614913565b60009182526020909120600360049092020101546001600160401b031692915050565b610fab6040518060400160405280600c81526020016b0c2c8ecc2dcc6ca8ae0dec6d60a31b81525061256a565b610fb361322e565b610fbb6132f9565b610fc3610a7d565b565b6060600f805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b6060600e805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b610fb360405180604001604052806016815260200175616476616e636545706f63685f4e6f5265776172647360501b81525061256a565b60408051603080825260608281019093526000919060208201818036833750505060308101939093525090919050565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156111d25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156111ba57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161117c5790505b50505050508152505081526020019060010190611110565b50505050905090565b60008060005b835181101561125d57633b9aca00601d85838151811061120357611203614913565b602002602001015164ffffffffff168154811061122257611222614913565b600091825260209091206003600490920201015461124991906001600160401b0316614a50565b6112539083614a3d565b91506001016111e1565b5092915050565b601b54600090600160a01b90046001600160401b03164210156112ef5760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e2e63757272656e7445706f63683a2063757272656e60448201527f742074696d65206973206265666f72652067656e657369732074696d650000006064820152608401610702565b6112fb600c6020614a67565b601b546001600160401b039182169161131c91600160a01b90041642614a2a565b6113269190614a9f565b905090565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156111d257838290600052602060002001805461136e906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461139a906149f6565b80156113e75780601f106113bc576101008083540402835291602001916113e7565b820191906000526020600020905b8154815290600101906020018083116113ca57829003601f168201915b50505050508152602001906001019061134f565b6000601e8161140b600485614ab3565b64ffffffffff16815260208101919091526040016000205492915050565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156111d25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114f757602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114b95790505b5050505050815250508152602001906001019061144d565b6115176141a8565b60005b825181101561160f5760205483828151811061153857611538614913565b602002602001015164ffffffffff1611156116075760405162461bcd60e51b815260206004820152607760248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f2076616c696461746f7220686173206e6f74206265656e20696e636c7564656460648201527f20696e20626561636f6e20636861696e207374617465202844494420594f552060848201527f43414c4c20616476616e636545706f6368205945543f2900000000000000000060a482015260c401610702565b60010161151a565b50604080516080810182526021546001600160401b031680825260009081526022602090815283822084518086019095528054855260018101805493958386019490938401919061165f906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461168b906149f6565b80156116d85780601f106116ad576101008083540402835291602001916116d8565b820191906000526020600020905b8154815290600101906020018083116116bb57829003601f168201915b505050505081525050815260200184516001600160401b038111156116ff576116ff614257565b60405190808252806020026020018201604052801561173257816020015b606081526020019060019003908161171d5790505b50815260200184516001600160401b0381111561175157611751614257565b60405190808252806020026020018201604052801561178457816020015b606081526020019060019003908161176f5790505b509052905060005b835181101561125d576021546001600160401b03166000908152602460205260408120855182908790859081106117c5576117c5614913565b602002602001015164ffffffffff1664ffffffffff1681526020019081526020016000206040518060400160405290816000820180548060200260200160405190810160405280929190818152602001828054801561184357602002820191906000526020600020905b81548152602001906001019080831161182f575b5050505050815260200160018201805461185c906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611888906149f6565b80156118d55780601f106118aa576101008083540402835291602001916118d5565b820191906000526020600020905b8154815290600101906020018083116118b857829003601f168201915b50505050508152505090508060200151836040015183815181106118fb576118fb614913565b602002602001018190525080600001518360600151838151811061192157611921614913565b60209081029190910101525060010161178c565b60006001600160401b038016601d8364ffffffffff168154811061195b5761195b614913565b6000918252602090912060049091020160030154600160801b90046001600160401b03161492915050565b6119b16040805160808101825260009181019182526060808201529081908152602001606081525090565b60005b8351811015611a83576020548482815181106119d2576119d2614913565b602002602001015164ffffffffff161115611a7b5760405162461bcd60e51b815260206004820152605b60248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f206e6f20636865636b706f696e742070726f6f6620666f756e6420286469642060648201527f796f752063616c6c20616476616e636545706f6368207965743f290000000000608482015260a401610702565b6001016119b4565b50604080516001600160401b038416600090815260236020528281206080830184528054938301938452600181018054929484939092916060850191611ac8906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611af4906149f6565b8015611b415780601f10611b1657610100808354040283529160200191611b41565b820191906000526020600020905b815481529060010190602001808311611b2457829003601f168201915b505050505081525050815260200185516001600160401b03811115611b6857611b68614257565b604051908082528060200260200182016040528015611bb557816020015b60408051606080820183526000808352602083015291810191909152815260200190600190039081611b865790505b509052905060005b8451811015611d3e576000858281518110611bda57611bda614913565b602002602001015190506000611bef826134e5565b6001600160401b038716600090815260256020908152604080832064ffffffffff851684528252808320815180830190925280548252600181018054959650939491939092840191611c40906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611c6c906149f6565b8015611cb95780601f10611c8e57610100808354040283529160200191611cb9565b820191906000526020600020905b815481529060010190602001808311611c9c57829003601f168201915b50505050508152505090506040518060600160405280601d8564ffffffffff1681548110611ce957611ce9614913565b906000526020600020906004020160010154815260200182600001518152602001826020015181525085602001518581518110611d2857611d28614913565b6020908102919091010152505050600101611bbd565b5090505b92915050565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156111d2578382906000526020600020018054611d8b906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611db7906149f6565b8015611e045780601f10611dd957610100808354040283529160200191611e04565b820191906000526020600020905b815481529060010190602001808311611de757829003601f168201915b505050505081526020019060010190611d6c565b60008054610100900460ff1615611e385750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611f3e5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611ec6917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001614add565b60408051601f1981840301815290829052611ee091614b0e565b6000604051808303816000865af19150503d8060008114611f1d576040519150601f19603f3d011682016040523d82523d6000602084013e611f22565b606091505b5091505080806020019051810190611f3a9190614b2a565b9150505b919050565b6060600082516001600160401b03811115611f6057611f60614257565b604051908082528060200260200182016040528015611f89578160200160208202803683370190505b50905060005b835181101561125d57601d848281518110611fac57611fac614913565b602002602001015164ffffffffff1681548110611fcb57611fcb614913565b906000526020600020906004020160010154828281518110611fef57611fef614913565b6020908102919091010152600101611f8f565b6060600c805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b60006120916040518060400160405280600c81526020016b3732bbab30b634b230ba37b960a11b81525061256a565b34670de0b6b3a76400008110156121065760405162461bcd60e51b815260206004820152603360248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a206465604482015272706f7369742076616c756520746f6f206c6f7760681b6064820152608401610702565b612114633b9aca0082614b4c565b156121875760405162461bcd60e51b815260206004820152603860248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a20766160448201527f6c7565206e6f74206d756c7469706c65206f66206777656900000000000000006064820152608401610702565b6000612197633b9aca0083614a9f565b90506001600160401b0381111561220d5760405162461bcd60e51b815260206004820152603460248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a2064656044820152730e0dee6d2e840ecc2d8eaca40e8dede40d0d2ced60631b6064820152608401610702565b61221784826134f2565b949350505050565b61225d6040518060400160405280601781526020017f616476616e636545706f63685f4e6f576974686472617700000000000000000081525061256a565b610fbb61322e565b6000611d42612273836113fb565b8361389f565b6000601d8264ffffffffff168154811061229557612295614913565b6000918252602090912060049091020160030154600160801b90046001600160401b031692915050565b60006122ef6040518060400160405280600d81526020016c32bc34ba2b30b634b230ba37b960991b81525061256a565b6000601d8364ffffffffff168154811061230b5761230b614913565b60009182526020909120600490910201805490915060ff16156123405760405162461bcd60e51b815260040161070290614929565b6003810154600160801b90046001600160401b03908116146123b65760405162461bcd60e51b815260206004820152602960248201527f426561636f6e436861696e4d6f636b3a2076616c696461746f7220616c726561604482015268191e48195e1a5d195960ba1b6064820152608401610702565b6123be611264565b6123c99060016149b8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055506123fa836125fa565b9150612407836000612605565b60006124c5601d8564ffffffffff168154811061242657612426614913565b90600052602060002090600402016002018054612442906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461246e906149f6565b80156124bb5780601f10612490576101008083540402835291602001916124bb565b820191906000526020600020905b81548152906001019060200180831161249e57829003601f168201915b505050505061392a565b601b549091506001600160a01b031663c88a5e6d826124f1633b9aca006001600160401b038816614a50565b612505906001600160a01b03861631614a3d565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561254b57600080fd5b505af115801561255f573d6000803e3d6000fd5b505050505050919050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f506125bb6125b660408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b602082015290565b613946565b6125c48361396f565b6040516020016125d5929190614b60565b60408051601f19818403018152908290526125ef916144f8565b60405180910390a150565b6000611d4282612265565b6000601e81612615600486614ab3565b64ffffffffff1664ffffffffff16815260200190815260200160002054905061263f818484613997565b905080601e6000612651600487614ab3565b64ffffffffff168152602081019190915260400160002055505050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50816040516125ef91906144f8565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516126ce929190614b9c565b60405180910390a15050565b60006126e8600c6020614a67565b6126f38360016149b8565b6126fd9190614a67565b601b54611d429190600160a01b90046001600160401b03166149b8565b601d546060906000906001600160401b0381111561273a5761273a614257565b604051908082528060200260200182016040528015612763578160200160208202803683370190505b50905060005b601d548110156107c65761278461277f82613a0b565b613c92565b82828151811061279657612796614913565b6020908102919091010152600101612769565b6000805b838110156129815760006002865160016127c79190614a3d565b6127d19190614a9f565b90506000816001600160401b038111156127ed576127ed614257565b604051908082528060200260200182016040528015612816578160200160208202803683370190505b50905060005b82811015612975576000612831826002614a50565b90506000612840826001614a3d565b905060008a838151811061285657612856614913565b6020026020010151905060008b5183101561288c578b838151811061287d5761287d614913565b60200260200101519050612898565b61289588613f2b565b90505b6000600283836040516020016128b8929190918252602082015260400190565b60408051601f19818403018152908290526128d291614b0e565b602060405180830381855afa1580156128ef573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906129129190614bbe565b90508087878151811061292757612927614913565b60209081029190910181019190915260008481528c825260408082208590558482528082208690559481526001808e01909252848120839055928352929091205592909201915061281c9050565b509550506001016127ad565b5083516001146129f25760405162461bcd60e51b815260206004820152603660248201527f426561636f6e436861696e4d6f636b2e5f6275696c644d65726b6c65547265656044820152753a20696e76616c6964207472656520736f6d65686f7760501b6064820152608401610702565b83600081518110612a0557612a05614913565b602002602001015190509392505050565b60606000612a22613fa2565b6001600160401b03811115612a3957612a39614257565b604051908082528060200260200182016040528015612a62578160200160208202803683370190505b50905060005b81518110156107c65764ffffffffff81166000908152601e60205260409020548251839083908110612a9c57612a9c614913565b6020908102919091010152600101612a68565b6040805160208082526104208201909252606091600091908082016104008036833701905050905060005b8151811015612b1657612aee816001614a3d565b60001b828281518110612b0357612b03614913565b6020908102919091010152600101612ada565b508381600b81518110612b2b57612b2b614913565b6020026020010181815250508281600c81518110612b4b57612b4b614913565b60209081029190910101529392505050565b60408051600580825260c08201909252606091600091906020820160a08036833701905050905060005b8151811015612bc357612b9b816001614a3d565b60001b828281518110612bb057612bb0614913565b6020908102919091010152600101612b87565b508281600381518110612bd857612bd8614913565b602090810291909101015292915050565b7f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358382612c1483613fda565b6040516126ce929190614bd7565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612c5c57612c5c614257565b6040519080825280601f01601f191660200182016040528015612c86576020820181803683370190505b509050816000805b6003811015612cee576021546001600160401b0316600090815260276020908152604080832086845260068101835281842054858402890184018190529684526007019091529020549282612ce281614c05565b93505050600101612c8e565b5060408051808201825285815260208082018681526021546001600160401b0316600090815260229092529290208151815591519091906001820190612d349082614c65565b50505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612d7757612d77614257565b6040519080825280601f01601f191660200182016040528015612da1576020820181803683370190505b509050816000612dd260207f0000000000000000000000000000000000000000000000000000000000000000614a9f565b90506000805b6005811015612e38576021546001600160401b03166000908152602760209081526040808320878452600481018352818420548584028a0184018190529784526005019091529020549382612e2c81614c05565b93505050600101612dd8565b50805b82811015612e9a576021546001600160401b03166000908152602760209081526040808320878452600681018352818420548584028a0184018190529784526007019091529020549382612e8e81614c05565b93505050600101612e3b565b5060408051808201825286815260208082018781526021546001600160401b0316600090815260239092529290208151815591519091906001820190612ee09082614c65565b5050505050505050565b6021546001600160401b03166000908152602460205260408120905b601d548110156130d05760007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612f4a57612f4a614257565b6040519080825280601f01601f191660200182016040528015612f74576020820181803683370190505b5090506000612f8283613a0b565b90506000612f8f82613c92565b90506000805b612fa160286001614a3d565b811015612ffc576021546001600160401b03166000908152602760209081526040808320868452808352818420548584028a0184018190529684526001019091529020549282612ff081614c05565b93505050600101612f95565b50805b600561300d60286001614a3d565b6130179190614a3d565b811015613075576021546001600160401b03166000908152602760209081526040808320868452600481018352818420548584028a018401819052968452600501909152902054928261306981614c05565b93505050600101612fff565b5064ffffffffff8516600090815260208781526040909120845161309b928601906141f7565b5064ffffffffff851660009081526020879052604090206001016130bf8582614c65565b505060019093019250612f06915050565b5050565b6021546001600160401b03166000908152602560205260408120906130f7613fa2565b905060005b818110156132295760007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0381111561313e5761313e614257565b6040519080825280601f01601f191660200182016040528015613168576020820181803683370190505b5064ffffffffff83166000908152601e60205260408120549192508190805b61319360266001614a3d565b8110156131f1576021546001600160401b03166000908152602760209081526040808320868452600281018352818420548584028a01840181905296845260030190915290205492826131e581614c05565b93505050600101613187565b5064ffffffffff851660009081526020889052604090208381556001016132188582614c65565b5050600190930192506130fc915050565b505050565b6000805b601d548110156132d6576000601d828154811061325157613251614913565b60009182526020909120600490910201805490915060ff161561327457506132ce565b600381015467fffffffffffffffe19600160801b9091046001600160401b0316016132cc5760006132a4836125fa565b90506132b16001826149b8565b9050836132bd81614c05565b9450506132ca8382612605565b505b505b600101613232565b50610d84604051806060016040528060268152602001614efa602691398261269d565b6000805b601d5481101561349f576000601d828154811061331c5761331c614913565b60009182526020909120600490910201805490915060ff161561333f5750613497565b6000633b9aca0061334f846125fa565b6001600160401b03166133629190614a50565b90506000613378836002018054612442906149f6565b905060008061338b633b9aca0085614a9f565b6003860154909150600160801b90046001600160401b03908116146133c757836000036133bc575050505050613497565b5082905060006133f6565b6801bc16d674ec8000008411156133f6576133eb6801bc16d674ec80000085614a2a565b915064077359400090505b601b546001600160a01b039081169063c88a5e6d90859061341b908690831631614a3d565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561346157600080fd5b505af1158015613475573d6000803e3d6000fd5b5050505081876134859190614a3d565b96506134918682612605565b50505050505b6001016132fd565b508015610d8457610d846040518060400160405280601981526020017f2d207769746864726577206578636573732062616c616e6365000000000000008152508261269d565b6000611d42600483614ab3565b601d54600090613503600482614d23565b64ffffffffff166000036136ec57601d54600090613528906001600160401b036149d7565b6040805160308082526060820190925291925060009190602082018180368337019050509050828160300152601d6040518060e00160405280600115158152602001600015158152602001600284600060801b60405160200161358c929190614d4d565b60408051601f19818403018152908290526135a691614b0e565b602060405180830381855afa1580156135c3573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906135e69190614bbe565b815260408051602080820183526000808352818501929092526001600160401b0388811684860152606080860182905260809095015285546001808201885596835291819020855160049093020180549186015161ffff1990921692151561ff00191692909217610100911515919091021781559083015193810193909355810151909190600282019061367a9082614c65565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909316919094161717929092161790556136dc8383612605565b826136e681614d7c565b93505050505b60408051603080825260608201909252600091602082018180368337019050509050818160300152601d6040518060e00160405280600015158152602001600015158152602001600284600060801b60405160200161374c929190614d4d565b60408051601f198184030181529082905261376691614b0e565b602060405180830381855afa158015613783573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906137a69190614bbe565b8152602001878152602001866001600160401b031681526020016137c8611264565b6001600160401b039081168252602091820152825460018181018555600094855293829020835160049092020180549284015115156101000261ff00199215159290921661ffff19909316929092171781556040820151928101929092556060810151909190600282019061383d9082614c65565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990931691909416171792909216179055611d3e8285612605565b6000806138ad600484614d23565b6138b8906040614da3565b64ffffffffff16905061221784821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161760ff60381b60889290921c919091161790565b60008061393683614dc3565b6001600160a01b03169392505050565b6060611d42604051806040016040528060058152602001641b5b39366d60d81b8152508361405e565b6060611d42604051806040016040528060048152602001631b5b336d60e01b8152508361405e565b6000806139a5600485614d23565b6139b0906001614de7565b6139bb906040614da3565b6139c790610100614e04565b64ffffffffff1690506001600160401b03811b1985811660006139e9866140a8565b905060006139f88560c0614a2a565b9190911c91909117979650505050505050565b60408051600880825261012082019092526060916000919060208201610100803683370190505090506000601d8464ffffffffff1681548110613a5057613a50614913565b60009182526020918290206040805160e0810182526004909302909101805460ff8082161515855261010090910416151593830193909352600183015490820152600282018054919291606084019190613aa9906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054613ad5906149f6565b8015613b225780601f10613af757610100808354040283529160200191613b22565b820191906000526020600020905b815481529060010190602001808311613b0557829003601f168201915b5050509183525050600391909101546001600160401b038082166020840152600160401b82048116604080850191909152600160801b909204166060909201919091528101518351919250908390600090613b7f57613b7f614913565b6020026020010181815250508060600151613b9990614dc3565b82600181518110613bac57613bac614913565b602002602001018181525050613bc581608001516140a8565b82600281518110613bd857613bd8614913565b6020026020010181815250508060200151604051602001613bfd911515815260200190565b604051602081830303815290604052613c1590614dc3565b82600381518110613c2857613c28614913565b602002602001018181525050613c418160a001516140a8565b82600581518110613c5457613c54614913565b602002602001018181525050613c6d8160c001516140a8565b82600681518110613c8057613c80614913565b60209081029190910101525092915050565b60008060028351613ca39190614a9f565b90506000816001600160401b03811115613cbf57613cbf614257565b604051908082528060200260200182016040528015613ce8578160200160208202803683370190505b50905060005b82811015613de557600285613d038383614a50565b81518110613d1357613d13614913565b602002602001015186836002613d299190614a50565b613d34906001614a3d565b81518110613d4457613d44614913565b6020026020010151604051602001613d66929190918252602082015260400190565b60408051601f1981840301815290829052613d8091614b0e565b602060405180830381855afa158015613d9d573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190613dc09190614bbe565b828281518110613dd257613dd2614913565b6020908102919091010152600101613cee565b50613df1600283614a9f565b91505b8115613f075760005b82811015613ef457600282613e128383614a50565b81518110613e2257613e22614913565b602002602001015183836002613e389190614a50565b613e43906001614a3d565b81518110613e5357613e53614913565b6020026020010151604051602001613e75929190918252602082015260400190565b60408051601f1981840301815290829052613e8f91614b0e565b602060405180830381855afa158015613eac573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190613ecf9190614bbe565b828281518110613ee157613ee1614913565b6020908102919091010152600101613dfd565b50613f00600283614a9f565b9150613df4565b80600081518110613f1a57613f1a614913565b602002602001015192505050919050565b600060648210613f7d5760405162461bcd60e51b815260206004820152601b60248201527f5f6765745a65726f4e6f64653a20696e76616c696420646570746800000000006044820152606401610702565b60268281548110613f9057613f90614913565b90600052602060002001549050919050565b601d5460009015613fd457601d54600490613fbf90600190614a2a565b613fc99190614a9f565b611326906001614a3d565b50600090565b604051631623433d60e31b815260048101829052606090611d4290737109709ecfa91a80626ff3989d68f67f5b1dd12d9063b11a19e890602401600060405180830381865afa158015614031573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526140599190810190614e21565b61411e565b60608282604051806040016040528060048152602001631b5b306d60e01b81525060405160200161409193929190614e8e565b604051602081830303815290604052905092915050565b603881811b60ff60381b16602883811b66ff0000000000001691909117601884811b65ff00000000001691909117600885811b64ff00000000169190911763ff0000009186901c919091161762ff00009185901c919091161761ff009184901c919091161760ff9290911c919091161760c01b90565b6060611d42604051806040016040528060048152602001631b5b326d60e01b8152508361405e565b604051806060016040528060006001600160401b03168152602001614181604051806040016040528060008019168152602001606081525090565b81526020016141a3604051806040016040528060608152602001606081525090565b905290565b604051806080016040528060006001600160401b031681526020016141e3604051806040016040528060008019168152602001606081525090565b815260200160608152602001606081525090565b828054828255906000526020600020908101928215614232579160200282015b82811115614232578251825591602001919060010190614217565b5061423e929150614242565b5090565b5b8082111561423e5760008155600101614243565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b038111828210171561429557614295614257565b604052919050565b803564ffffffffff81168114611f3e57600080fd5b600082601f8301126142c357600080fd5b81356001600160401b038111156142dc576142dc614257565b8060051b6142ec6020820161426d565b9182526020818501810192908101908684111561430857600080fd5b6020860192505b83831015614331576143208361429d565b82526020928301929091019061430f565b9695505050505050565b60006020828403121561434d57600080fd5b81356001600160401b0381111561436357600080fd5b612217848285016142b2565b602080825282518282018190526000918401906040840190835b818110156143b05783516001600160a01b0316835260209384019390920191600101614389565b509095945050505050565b6000602082840312156143cd57600080fd5b6143d68261429d565b9392505050565b60005b838110156143f85781810151838201526020016143e0565b50506000910152565b600081518084526144198160208601602086016143dd565b601f01601f19169290920160200192915050565b8051825260006020820151604060208501526122176040850182614401565b600081518084526020840193506020830160005b8281101561447e578151865260209586019590910190600101614460565b5093949350505050565b602081526001600160401b03825116602082015260006020830151606060408401526144b7608084018261442d565b90506040840151601f198483030160608501528051604083526144dd604084018261444c565b90506020820151915082810360208401526143318183614401565b6020815260006143d66020830184614401565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156145b957868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b808310156145a15783516001600160e01b03191682526020938401936001939093019290910190614575565b50965050506020938401939190910190600101614533565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156145b957603f19878603018452614609858351614401565b945060209384019391909101906001016145ed565b600082825180855260208501945060208160051b8301016020850160005b8381101561466e57601f1985840301885261465883835161444c565b602098890198909350919091019060010161463c565b50909695505050505050565b602081526001600160401b03825116602082015260006020830151608060408401526146a960a084018261442d565b6040850151848203601f19016060860152805180835291925060209081019181840191600582901b85010160005b8281101561470857601f198683030184526146f3828651614401565b602095860195949094019391506001016146d7565b506060880151878203601f190160808901529450614726818661461e565b98975050505050505050565b6000806040838503121561474557600080fd5b82356001600160401b0381111561475b57600080fd5b614767858286016142b2565b92505060208301356001600160401b038116811461478457600080fd5b809150509250929050565b6020815260008251604060208401526147ab606084018261442d565b602085810151858303601f19016040870152805180845292935081019181840191600582901b85010160005b8281101561482957601f198683030184528451805183526020810151602084015260408101519050606060408401526148136060840182614401565b60209687019695909501949250506001016147d7565b50979650505050505050565b602080825282518282018190526000918401906040840190835b818110156143b057835183526020938401939092019160010161484f565b60006001600160401b0382111561488657614886614257565b50601f01601f191660200190565b6000602082840312156148a657600080fd5b81356001600160401b038111156148bc57600080fd5b8201601f810184136148cd57600080fd5b80356148e06148db8261486d565b61426d565b8181528560208385010111156148f557600080fd5b81602084016020830137600091810160200191909152949350505050565b634e487b7160e01b600052603260045260246000fd5b60208082526053908201527f426561636f6e436861696e4d6f636b3a20617474656d7074696e6720746f206560408201527f7869742064756d6d792076616c696461746f722e205765206e6565642074686f6060820152720e6ca40ccdee440e0e4dedecccecadc407c745606b1b608082015260a00190565b634e487b7160e01b600052601160045260246000fd5b6001600160401b038181168382160190811115611d4257611d426149a2565b6001600160401b038281168282160390811115611d4257611d426149a2565b600181811c90821680614a0a57607f821691505b6020821081036107c657634e487b7160e01b600052602260045260246000fd5b81810381811115611d4257611d426149a2565b80820180821115611d4257611d426149a2565b8082028115828204841417611d4257611d426149a2565b6001600160401b03818116838216029081169081811461125d5761125d6149a2565b634e487b7160e01b600052601260045260246000fd5b600082614aae57614aae614a89565b500490565b600064ffffffffff831680614aca57614aca614a89565b8064ffffffffff84160491505092915050565b6001600160e01b0319831681528151600090614b008160048501602087016143dd565b919091016004019392505050565b60008251614b208184602087016143dd565b9190910192915050565b600060208284031215614b3c57600080fd5b815180151581146143d657600080fd5b600082614b5b57614b5b614a89565b500690565b60008351614b728184602088016143dd565b601760f91b9083019081528351614b908160018401602088016143dd565b01600101949350505050565b604081526000614baf6040830185614401565b90508260208301529392505050565b600060208284031215614bd057600080fd5b5051919050565b604081526000614bea6040830185614401565b8281036020840152614bfc8185614401565b95945050505050565b600060018201614c1757614c176149a2565b5060010190565b601f82111561322957806000526020600020601f840160051c81016020851015614c455750805b601f840160051c820191505b81811015610f385760008155600101614c51565b81516001600160401b03811115614c7e57614c7e614257565b614c9281614c8c84546149f6565b84614c1e565b6020601f821160018114614cc65760008315614cae5750848201515b600019600385901b1c1916600184901b178455610f38565b600084815260208120601f198516915b82811015614cf65787850151825560209485019460019092019101614cd6565b5084821015614d145786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b600064ffffffffff831680614d3a57614d3a614a89565b8064ffffffffff84160691505092915050565b60008351614d5f8184602088016143dd565b6001600160801b0319939093169190920190815260100192915050565b600064ffffffffff821664ffffffffff8103614d9a57614d9a6149a2565b60010192915050565b64ffffffffff818116838216029081169081811461125d5761125d6149a2565b805160208083015191908110156107c65760001960209190910360031b1b16919050565b64ffffffffff8181168382160190811115611d4257611d426149a2565b64ffffffffff8281168282160390811115611d4257611d426149a2565b600060208284031215614e3357600080fd5b81516001600160401b03811115614e4957600080fd5b8201601f81018413614e5a57600080fd5b8051614e686148db8261486d565b818152856020838501011115614e7d57600080fd5b614bfc8260208301602086016143dd565b60008451614ea08184602089016143dd565b845190830190614eb48183602089016143dd565b8451910190614ec78183602088016143dd565b019594505050505056fe2d2d206e6f2076616c696461746f72733b20616464656420656d70747920626c6f636b20726f6f742d2067656e657261746564207265776172647320666f72206e756d2076616c696461746f7273a2646970667358221220c6c7bc513a9397bde6c23a29612109d59a2a2c6163ec81ee6e2ce44b6887ce9564736f6c634300081b0033608060405234801561001057600080fd5b50600436106100365760003560e01c8063643599f2146101ab578063acd414a8146101e7575b6020361461009f5760405162461bcd60e51b815260206004820152602b60248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206d616c666f726d60448201526a6564206d73672e6461746160a81b60648201526084015b60405180910390fd5b60006100ab3682610214565b90508060000361010d5760405162461bcd60e51b815260206004820152602760248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a2074696d6573746160448201526606d7020697320360cc1b6064820152608401610096565b600081815260208190526040812054908190036101a15760405162461bcd60e51b815260206004820152604660248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206e6f20626c6f6360448201527f6b20726f6f7420666f756e642e2044494420594f5520555345204348454154536064820152652e574152503f60d01b608482015260a401610096565b8060005260206000f35b6101d56101b9366004610214565b67ffffffffffffffff1660009081526020819052604090205490565b60405190815260200160405180910390f35b6102126101f536600461022d565b67ffffffffffffffff909116600090815260208190526040902055565b005b60006020828403121561022657600080fd5b5035919050565b6000806040838503121561024057600080fd5b823567ffffffffffffffff8116811461025857600080fd5b94602093909301359350505056fea26469706673582212201c07c1831c6dfce4cd66e3a8b35c5cf327a7e2b43aa85ef4e4daafe0f73a96c764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R`\0\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x04\x80T\x90\x91\x16\x90\x91\x17\x90U`\x1B\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x17\x90Ua\0T`\x03` a\x067V[`\x80R`\x05a\0e`(`\x01a\x06TV[a\0o\x91\x90a\x06TV[a\0z\x90` a\x067V[`\xA0Ra\0\x89`\x05`\x03a\x06TV[a\0\x94\x90` a\x067V[`\xC0Ra\0\xA3`&`\x01a\x06TV[a\0\xAE\x90` a\x067V[`\xE0R4\x80\x15a\0\xBDW`\0\x80\xFD[P`@QaY\xD38\x03\x80aY\xD3\x839\x81\x01`@\x81\x90Ra\0\xDC\x91a\x06gV[`\x1B\x80T`\x01`\x01`@\x1B\x03\x83\x16`\x01`\xA0\x1B\x02`\x01`\xA0\x1B`\x01`\xE0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U`\x1C\x80T`\x01`\x01`\xA0\x1B\x03\x80\x87\x16h\x01\0\0\0\0\0\0\0\0\x02`\x01`@\x1B`\x01`\xE0\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90\x91U`@Q\x92\x81\x16\x91\x16\x17\x90c\xB4\xD6\xC7\x82\x90r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90a\x01d` \x82\x01a\x05\xB4V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x95\x92\x91\x90a\x06\xDDV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xAFW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xC3W=`\0\x80>=`\0\xFD[PP`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R`\0\x93Pa\x01\xF4\x92P\x90` \x82\x01a\x01\0\x806\x837\x01\x90PPa\x03\x1BV[`@\x80Q`d\x80\x82Ra\x0C\xA0\x82\x01\x90\x92R\x91\x92P` \x82\x01a\x0C\x80\x806\x837PP\x81Qa\x02(\x92`&\x92P` \x01\x90a\x05\xC1V[P\x80`&`\0\x81T\x81\x10a\x02>Wa\x02>a\x075V[`\0\x91\x82R` \x90\x91 \x01U`\x01[`&T\x81\x10\x15a\x03\x12W`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x83\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02\x88\x91a\x07KV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\xA5W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xC8\x91\x90a\x07gV[`&\x82\x81T\x81\x10a\x02\xDBWa\x02\xDBa\x075V[\x90`\0R` `\0 \x01\x81\x90UP`&\x81\x81T\x81\x10a\x02\xFCWa\x02\xFCa\x075V[`\0\x91\x82R` \x90\x91 \x01T\x91P`\x01\x01a\x02MV[PPPPa\x07\xA2V[`\0\x80`\x02\x83Qa\x03,\x91\x90a\x07\x80V[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03HWa\x03Ha\x07\x1FV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03qW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a\x04nW`\x02\x85a\x03\x8C\x83\x83a\x067V[\x81Q\x81\x10a\x03\x9CWa\x03\x9Ca\x075V[` \x02` \x01\x01Q\x86\x83`\x02a\x03\xB2\x91\x90a\x067V[a\x03\xBD\x90`\x01a\x06TV[\x81Q\x81\x10a\x03\xCDWa\x03\xCDa\x075V[` \x02` \x01\x01Q`@Q` \x01a\x03\xEF\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04\t\x91a\x07KV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04&W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04I\x91\x90a\x07gV[\x82\x82\x81Q\x81\x10a\x04[Wa\x04[a\x075V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03wV[Pa\x04z`\x02\x83a\x07\x80V[\x91P[\x81\x15a\x05\x90W`\0[\x82\x81\x10\x15a\x05}W`\x02\x82a\x04\x9B\x83\x83a\x067V[\x81Q\x81\x10a\x04\xABWa\x04\xABa\x075V[` \x02` \x01\x01Q\x83\x83`\x02a\x04\xC1\x91\x90a\x067V[a\x04\xCC\x90`\x01a\x06TV[\x81Q\x81\x10a\x04\xDCWa\x04\xDCa\x075V[` \x02` \x01\x01Q`@Q` \x01a\x04\xFE\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x05\x18\x91a\x07KV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x055W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05X\x91\x90a\x07gV[\x82\x82\x81Q\x81\x10a\x05jWa\x05ja\x075V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04\x86V[Pa\x05\x89`\x02\x83a\x07\x80V[\x91Pa\x04}V[\x80`\0\x81Q\x81\x10a\x05\xA3Wa\x05\xA3a\x075V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[a\x02\x9C\x80aW7\x839\x01\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a\x05\xFCW\x91` \x02\x82\x01[\x82\x81\x11\x15a\x05\xFCW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x05\xE1V[Pa\x06\x08\x92\x91Pa\x06\x0CV[P\x90V[[\x80\x82\x11\x15a\x06\x08W`\0\x81U`\x01\x01a\x06\rV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06NWa\x06Na\x06!V[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x06NWa\x06Na\x06!V[`\0\x80`@\x83\x85\x03\x12\x15a\x06zW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\x91W`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x06\xAEW`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[`\0[\x83\x81\x10\x15a\x06\xD4W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xBCV[PP`\0\x91\x01RV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0\x82Q\x80`@\x84\x01Ra\x07\n\x81``\x85\x01` \x87\x01a\x06\xB9V[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01``\x01\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82Qa\x07]\x81\x84` \x87\x01a\x06\xB9V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x07yW`\0\x80\xFD[PQ\x91\x90PV[`\0\x82a\x07\x9DWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0QaOUa\x07\xE2`\09`\0a1\x08\x01R`\0\x81\x81a-A\x01Ra-\xAE\x01R`\0a/\x14\x01R`\0a,&\x01RaOU`\0\xF3\xFE`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80c\x86\xA6\xF9\xE1\x11a\x01\x18W\x80c\xC7o%\xC0\x11a\0\xA0W\x80c\xF0\xAC\xD9\x88\x11a\0oW\x80c\xF0\xAC\xD9\x88\x14a\x05\xC6W\x80c\xF7!8s\x14a\x05\xDBW\x80c\xF83\xEBc\x14a\x05\xFBW\x80c\xF8\xF9\x8AN\x14a\x06\x1BW\x80c\xFAv&\xD4\x14a\x06;W`\0\x80\xFD[\x80c\xC7o%\xC0\x14a\x05FW\x80c\xE2\x0C\x9Fq\x14a\x05sW\x80c\xE3\xCE\xFBB\x14a\x05\x88W\x80c\xED<\x16\x05\x14a\x05\x9DW`\0\x80\xFD[\x80c\xA5\n:\x1A\x11a\0\xE7W\x80c\xA5\n:\x1A\x14a\x04\x92W\x80c\xAAG8\x9C\x14a\x04\xBFW\x80c\xB1\xB6\xF6\xA1\x14a\x04\xEFW\x80c\xB5P\x8A\xA9\x14a\x05\x1CW\x80c\xBAAO\xA6\x14a\x051W`\0\x80\xFD[\x80c\x86\xA6\xF9\xE1\x14a\x02\xDAW\x80c\x90\x88 \xE0\x14a\x04)W\x80c\x91j\x17\xC6\x14a\x04IW\x80c\xA3\xF4\xDF~\x14a\x04^W`\0\x80\xFD[\x80c<\xF8\x0El\x11a\x01\x9BW\x80c^l\xC2\xFC\x11a\x01jW\x80c^l\xC2\xFC\x14a\x03\x83W\x80cf\xD9\xA9\xA0\x14a\x03\xB0W\x80ck:\xBD\x97\x14a\x03\xD2W\x80cvg\x18\x08\x14a\x03\xF2W\x80c\x85\"l\x81\x14a\x04\x07W`\0\x80\xFD[\x80c<\xF8\x0El\x14a\x03/W\x80c>^<#\x14a\x03DW\x80c?r\x86\xF4\x14a\x03YW\x80cY\xD0\x95\xDD\x14a\x03nW`\0\x80\xFD[\x80c)\x99/\xAA\x11a\x01\xD7W\x80c)\x99/\xAA\x14a\x02\xC3W\x80c-\xEF`\t\x14a\x02\xDAW\x80c3\x0B\xC2~\x14a\x02\xFAW\x80c5~\x95\x1F\x14a\x03\x0FW`\0\x80\xFD[\x80c\x146\tX\x14a\x02\tW\x80c\x1E\xD7\x83\x1C\x14a\x02FW\x80c\x1FT6\\\x14a\x02hW\x80c#\xE8,L\x14a\x02\x96W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02)a\x02$6`\x04aC;V[a\x06UV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02RW`\0\x80\xFD[Pa\x02[a\x07\xCCV[`@Qa\x02=\x91\x90aCoV[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x02\x88a\x02\x836`\x04aC\xBBV[a\x08.V[`@Q\x90\x81R` \x01a\x02=V[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x02\xB6a\x02\xB16`\x04aC\xBBV[a\x08cV[`@Qa\x02=\x91\x90aD\x88V[4\x80\x15a\x02\xCFW`\0\x80\xFD[Pa\x02\xD8a\n}V[\0[4\x80\x15a\x02\xE6W`\0\x80\xFD[Pa\x02)a\x02\xF56`\x04aC\xBBV[a\x0F?V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x02)`\n\x81V[4\x80\x15a\x03\x1BW`\0\x80\xFD[P`\x1CTa\x02)\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x02\xD8a\x0F~V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02[a\x0F\xC5V[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x02[a\x10%V[4\x80\x15a\x03zW`\0\x80\xFD[Pa\x02\xD8a\x10\x85V[4\x80\x15a\x03\x8FW`\0\x80\xFD[Pa\x03\xA3a\x03\x9E6`\x04aC\xBBV[a\x10\xBCV[`@Qa\x02=\x91\x90aD\xF8V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x03\xC5a\x10\xECV[`@Qa\x02=\x91\x90aE\x0BV[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02\x88a\x03\xED6`\x04aC;V[a\x11\xDBV[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x02)a\x12dV[4\x80\x15a\x04\x13W`\0\x80\xFD[Pa\x04\x1Ca\x13+V[`@Qa\x02=\x91\x90aE\xC5V[4\x80\x15a\x045W`\0\x80\xFD[Pa\x02\x88a\x04D6`\x04aC\xBBV[a\x13\xFBV[4\x80\x15a\x04UW`\0\x80\xFD[Pa\x03\xC5a\x14)V[4\x80\x15a\x04jW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01Ra\x03\xA3V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xB2a\x04\xAD6`\x04aC;V[a\x15\x0FV[`@Qa\x02=\x91\x90aFzV[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x04\xDFa\x04\xDA6`\x04aC\xBBV[a\x195V[`@Q\x90\x15\x15\x81R` \x01a\x02=V[4\x80\x15a\x04\xFBW`\0\x80\xFD[Pa\x05\x0Fa\x05\n6`\x04aG2V[a\x19\x86V[`@Qa\x02=\x91\x90aG\x8FV[4\x80\x15a\x05(W`\0\x80\xFD[Pa\x04\x1Ca\x1DHV[4\x80\x15a\x05=W`\0\x80\xFD[Pa\x04\xDFa\x1E\x18V[4\x80\x15a\x05RW`\0\x80\xFD[Pa\x05fa\x05a6`\x04aC;V[a\x1FCV[`@Qa\x02=\x91\x90aH5V[4\x80\x15a\x05\x7FW`\0\x80\xFD[Pa\x02[a \x02V[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x02)`\x01\x81V[a\x05\xB0a\x05\xAB6`\x04aH\x94V[a bV[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02=V[4\x80\x15a\x05\xD2W`\0\x80\xFD[Pa\x02\xD8a\"\x1FV[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x02)a\x05\xF66`\x04aC\xBBV[a\"eV[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02)a\x06\x166`\x04aC\xBBV[a\"yV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x02)a\x0666`\x04aC\xBBV[a\"\xBFV[4\x80\x15a\x06GW`\0\x80\xFD[P`\0Ta\x04\xDF\x90`\xFF\x16\x81V[`\0a\x06\x87`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nslashValidators`\x88\x1B\x81RPa%jV[`\0[\x82Q\x81\x10\x15a\x07\xC6W`\0\x83\x82\x81Q\x81\x10a\x06\xA7Wa\x06\xA7aI\x13V[` \x02` \x01\x01Q\x90P`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06\xCDWa\x06\xCDaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\x07\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x02\x90aI)V[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\xFF\x16a\x07bW\x80Ta\xFF\0\x19\x16a\x01\0\x17\x81Ua\x07.a\x12dV[a\x079\x90`\x01aI\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[`\0a\x07m\x83a%\xFAV[\x90P`\x01`\x01`@\x1B\x03\x81\x16`\n\x11\x15a\x07\x96Wa\x07\x8B\x81\x86aI\xB8V[\x94P`\0\x90Pa\x07\xB1V[a\x07\xA1`\n\x86aI\xB8V[\x94Pa\x07\xAE`\n\x82aI\xD7V[\x90P[a\x07\xBB\x83\x82a&\x05V[PPP`\x01\x01a\x06\x8AV[P\x91\x90PV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06W[PPPPP\x90P\x90V[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x08JWa\x08JaI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x90P\x91\x90PV[a\x08kaAFV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x81T``\x94\x81\x02\x82\x01\x85\x01\x84R\x92\x81\x01\x83\x81R\x90\x93\x91\x92\x84\x92\x84\x91\x90\x84\x01\x82\x82\x80\x15a\x08\xE4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xD0W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x08\xFD\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t)\x90aI\xF6V[\x80\x15a\tvW\x80`\x1F\x10a\tKWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tvV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tYW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q``\x81\x01\x82R`!T`\x01`\x01`@\x1B\x03\x16\x80\x82R`\0\x90\x81R`\"` \x90\x81R\x90\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x84R`\x01\x81\x01\x80T\x96\x97P\x92\x95\x82\x87\x01\x95P\x90\x92\x91\x84\x01\x91\x90a\t\xD4\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\0\x90aI\xF6V[\x80\x15a\nMW\x80`\x1F\x10a\n\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nMV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81R` \x93\x84\x01Q\x81\x85\x01R\x92\x01\x91\x90\x91R\x92\x91PPV[`\0[`\x1DT\x81\x10\x15a\x0B\x1AW`\0`\x1D\x82\x81T\x81\x10a\n\x9FWa\n\x9FaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\n\xC2WPa\x0B\x12V[`\0a\n\xCD\x83a%\xFAV[\x90Pd\x07sY@\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\n\xEDWPd\x07sY@\0[`\x03\x91\x90\x91\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01\x01a\n\x80V[Pa\x0BY`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F- updated effective balances\0\0\0\0\x81RPa&nV[a\x0B\x9B`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xA5\xA4\x0Cn\xAENL\xAD\xCE\x84\x0C\xAE\r\xECm`\x83\x1B\x81RPa\x0B\x8Da\x12dV[`\x01`\x01`@\x1B\x03\x16a&\x9DV[`\0a\x0B\xA5a\x12dV[`\x1BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xE5\xD6\xBF\x02a\x0B\xC2\x83a&\xDAV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x17W=`\0\x80>=`\0\xFD[PP`!\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16B`\x01`\x01`@\x1B\x03\x16\x17\x90UPP`@\x80Q\x80\x82\x01\x90\x91R`\x16\x81Ru\x05\xA4\rN\xAD\xAE\x0C\xAC\x84\x0E\x8D\xE4\r\xCC\xAF\x0E\x84\x0C\xAE\r\xECm`S\x1B` \x82\x01Ra\x0Cp\x90a\x0B\x8Da\x12dV[a\x0C\xAE`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F- building beacon state trees\0\0\0\x81RPa&nV[`\x1DT\x15a\x0C\xCDW`\x1DTa\x0C\xC5\x90`\x01\x90aJ*V[` Ua\r\x87V[`!T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p`$\x82\x01Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rKW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r_W=`\0\x80>=`\0\xFD[PPPPa\r\x84`@Q\x80``\x01`@R\x80`(\x81R` \x01aN\xD2`(\x919a&nV[PV[`\0a\r\xBFa\r\x94a'\x1AV[a\r\xA0`(`\x01aJ=V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 a'\xA9V[\x90P`\0a\r\xFCa\r\xCEa*\x16V[a\r\xDA`&`\x01aJ=V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x02\x01a'\xA9V[\x90P`\0a\x0E2a\x0E\r\x84\x84a*\xAFV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x05\x90`\x04\x01a'\xA9V[\x90P`\0a\x0Ega\x0EB\x83a+]V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x03\x90`\x06\x01a'\xA9V[\x90Pa\x0E\x9F`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x0BKH\x18\x99XX\xDB\xDB\x88\x18\x9B\x1B\xD8\xDA\xC8\x1C\x9B\xDB\xDD`b\x1B\x81RP\x82a+\xE9V[`!T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x12W=`\0\x80>=`\0\xFD[PPPPa\x0F\x1F\x82a,\"V[a\x0F(\x83a-=V[a\x0F0a.\xEAV[a\x0F8a0\xD4V[PPPPPV[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F[Wa\x0F[aI\x13V[`\0\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[a\x0F\xAB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0C,\x8E\xCC-\xCCl\xA8\xAE\r\xECm`\xA3\x1B\x81RPa%jV[a\x0F\xB3a2.V[a\x0F\xBBa2\xF9V[a\x0F\xC3a\n}V[V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[a\x0F\xB3`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uadvanceEpoch_NoRewards`P\x1B\x81RPa%jV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R`\0\x91\x90` \x82\x01\x81\x806\x837PPP`0\x81\x01\x93\x90\x93RP\x90\x91\x90PV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11\xBAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11|W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x11\x10V[PPPP\x90P\x90V[`\0\x80`\0[\x83Q\x81\x10\x15a\x12]Wc;\x9A\xCA\0`\x1D\x85\x83\x81Q\x81\x10a\x12\x03Wa\x12\x03aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\"Wa\x12\"aI\x13V[`\0\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01Ta\x12I\x91\x90`\x01`\x01`@\x1B\x03\x16aJPV[a\x12S\x90\x83aJ=V[\x91P`\x01\x01a\x11\xE1V[P\x92\x91PPV[`\x1BT`\0\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x12\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChain.currentEpoch: curren`D\x82\x01R\x7Ft time is before genesis time\0\0\0`d\x82\x01R`\x84\x01a\x07\x02V[a\x12\xFB`\x0C` aJgV[`\x1BT`\x01`\x01`@\x1B\x03\x91\x82\x16\x91a\x13\x1C\x91`\x01`\xA0\x1B\x90\x04\x16BaJ*V[a\x13&\x91\x90aJ\x9FV[\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x13n\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x9A\x90aI\xF6V[\x80\x15a\x13\xE7W\x80`\x1F\x10a\x13\xBCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xE7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xCAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x13OV[`\0`\x1E\x81a\x14\x0B`\x04\x85aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x92\x91PPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xF7W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\xB9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14MV[a\x15\x17aA\xA8V[`\0[\x82Q\x81\x10\x15a\x16\x0FW` T\x83\x82\x81Q\x81\x10a\x158Wa\x158aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F validator has not been included`d\x82\x01R\x7F in beacon chain state (DID YOU `\x84\x82\x01R\x7FCALL advanceEpoch YET?)\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x07\x02V[`\x01\x01a\x15\x1AV[P`@\x80Q`\x80\x81\x01\x82R`!T`\x01`\x01`@\x1B\x03\x16\x80\x82R`\0\x90\x81R`\"` \x90\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01\x80T\x93\x95\x83\x86\x01\x94\x90\x93\x84\x01\x91\x90a\x16_\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\x8B\x90aI\xF6V[\x80\x15a\x16\xD8W\x80`\x1F\x10a\x16\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xD8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xFFWa\x16\xFFaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x172W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\x1DW\x90P[P\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17QWa\x17QaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x84W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17oW\x90P[P\x90R\x90P`\0[\x83Q\x81\x10\x15a\x12]W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` R`@\x81 \x85Q\x82\x90\x87\x90\x85\x90\x81\x10a\x17\xC5Wa\x17\xC5aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x18/W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x18\\\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18\x88\x90aI\xF6V[\x80\x15a\x18\xD5W\x80`\x1F\x10a\x18\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q\x83`@\x01Q\x83\x81Q\x81\x10a\x18\xFBWa\x18\xFBaI\x13V[` \x02` \x01\x01\x81\x90RP\x80`\0\x01Q\x83``\x01Q\x83\x81Q\x81\x10a\x19!Wa\x19!aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\x8CV[`\0`\x01`\x01`@\x1B\x03\x80\x16`\x1D\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x19[Wa\x19[aI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x14\x92\x91PPV[a\x19\xB1`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x91\x82R``\x80\x82\x01R\x90\x81\x90\x81R` \x01``\x81RP\x90V[`\0[\x83Q\x81\x10\x15a\x1A\x83W` T\x84\x82\x81Q\x81\x10a\x19\xD2Wa\x19\xD2aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1A{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F no checkpoint proof found (did `d\x82\x01R\x7Fyou call advanceEpoch yet?)\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\x02V[`\x01\x01a\x19\xB4V[P`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`#` R\x82\x81 `\x80\x83\x01\x84R\x80T\x93\x83\x01\x93\x84R`\x01\x81\x01\x80T\x92\x94\x84\x93\x90\x92\x91``\x85\x01\x91a\x1A\xC8\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\xF4\x90aI\xF6V[\x80\x15a\x1BAW\x80`\x1F\x10a\x1B\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B$W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BhWa\x1BhaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xB5W\x81` \x01[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\x86W\x90P[P\x90R\x90P`\0[\x84Q\x81\x10\x15a\x1D>W`\0\x85\x82\x81Q\x81\x10a\x1B\xDAWa\x1B\xDAaI\x13V[` \x02` \x01\x01Q\x90P`\0a\x1B\xEF\x82a4\xE5V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R`%` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01\x80T\x95\x96P\x93\x94\x91\x93\x90\x92\x84\x01\x91a\x1C@\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Cl\x90aI\xF6V[\x80\x15a\x1C\xB9W\x80`\x1F\x10a\x1C\x8EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xB9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x9CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80``\x01`@R\x80`\x1D\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xE9Wa\x1C\xE9aI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x82`\0\x01Q\x81R` \x01\x82` \x01Q\x81RP\x85` \x01Q\x85\x81Q\x81\x10a\x1D(Wa\x1D(aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x1B\xBDV[P\x90P[\x92\x91PPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1D\x8B\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xB7\x90aI\xF6V[\x80\x15a\x1E\x04W\x80`\x1F\x10a\x1D\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1DlV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1E8WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F>W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1E\xC6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01aJ\xDDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xE0\x91aK\x0EV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1F\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\"V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1F:\x91\x90aK*V[\x91PP[\x91\x90PV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F`Wa\x1F`aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x89W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12]W`\x1D\x84\x82\x81Q\x81\x10a\x1F\xACWa\x1F\xACaI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1F\xCBWa\x1F\xCBaI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x82\x82\x81Q\x81\x10a\x1F\xEFWa\x1F\xEFaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F\x8FV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[`\0a \x91`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k72\xBB\xAB0\xB64\xB20\xBA7\xB9`\xA1\x1B\x81RPa%jV[4g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a!\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rrposit value too low`h\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a!\x14c;\x9A\xCA\0\x82aKLV[\x15a!\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FBeaconChainMock.newValidator: va`D\x82\x01R\x7Flue not multiple of gwei\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x02V[`\0a!\x97c;\x9A\xCA\0\x83aJ\x9FV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rs\x0E\r\xEEm.\x84\x0E\xCC-\x8E\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`c\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a\"\x17\x84\x82a4\xF2V[\x94\x93PPPPV[a\"]`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FadvanceEpoch_NoWithdraw\0\0\0\0\0\0\0\0\0\x81RPa%jV[a\x0F\xBBa2.V[`\0a\x1DBa\"s\x83a\x13\xFBV[\x83a8\x9FV[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\x95Wa\"\x95aI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`\0a\"\xEF`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l2\xBC4\xBA+0\xB64\xB20\xBA7\xB9`\x99\x1B\x81RPa%jV[`\0`\x1D\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#\x0BWa#\x0BaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a#@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x02\x90aI)V[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a#\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBeaconChainMock: validator alrea`D\x82\x01Rh\x19\x1EH\x19^\x1A]\x19Y`\xBA\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a#\xBEa\x12dV[a#\xC9\x90`\x01aI\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa#\xFA\x83a%\xFAV[\x91Pa$\x07\x83`\0a&\x05V[`\0a$\xC5`\x1D\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$&Wa$&aI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x02\x01\x80Ta$B\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$n\x90aI\xF6V[\x80\x15a$\xBBW\x80`\x1F\x10a$\x90Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xBBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x9EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa9*V[`\x1BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x8A^m\x82a$\xF1c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x88\x16aJPV[a%\x05\x90`\x01`\x01`\xA0\x1B\x03\x86\x161aJ=V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%KW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%_W=`\0\x80>=`\0\xFD[PPPPPP\x91\x90PV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPa%\xBBa%\xB6`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01R\x90V[a9FV[a%\xC4\x83a9oV[`@Q` \x01a%\xD5\x92\x91\x90aK`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra%\xEF\x91aD\xF8V[`@Q\x80\x91\x03\x90\xA1PV[`\0a\x1DB\x82a\"eV[`\0`\x1E\x81a&\x15`\x04\x86aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90Pa&?\x81\x84\x84a9\x97V[\x90P\x80`\x1E`\0a&Q`\x04\x87aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 UPPPV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP\x81`@Qa%\xEF\x91\x90aD\xF8V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa&\xCE\x92\x91\x90aK\x9CV[`@Q\x80\x91\x03\x90\xA1PPV[`\0a&\xE8`\x0C` aJgV[a&\xF3\x83`\x01aI\xB8V[a&\xFD\x91\x90aJgV[`\x1BTa\x1DB\x91\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI\xB8V[`\x1DT``\x90`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a':Wa':aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'cW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x1DT\x81\x10\x15a\x07\xC6Wa'\x84a'\x7F\x82a:\x0BV[a<\x92V[\x82\x82\x81Q\x81\x10a'\x96Wa'\x96aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'iV[`\0\x80[\x83\x81\x10\x15a)\x81W`\0`\x02\x86Q`\x01a'\xC7\x91\x90aJ=V[a'\xD1\x91\x90aJ\x9FV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xEDWa'\xEDaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x16W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a)uW`\0a(1\x82`\x02aJPV[\x90P`\0a(@\x82`\x01aJ=V[\x90P`\0\x8A\x83\x81Q\x81\x10a(VWa(VaI\x13V[` \x02` \x01\x01Q\x90P`\0\x8BQ\x83\x10\x15a(\x8CW\x8B\x83\x81Q\x81\x10a(}Wa(}aI\x13V[` \x02` \x01\x01Q\x90Pa(\x98V[a(\x95\x88a?+V[\x90P[`\0`\x02\x83\x83`@Q` \x01a(\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra(\xD2\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a(\xEFW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x12\x91\x90aK\xBEV[\x90P\x80\x87\x87\x81Q\x81\x10a)'Wa)'aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\0\x84\x81R\x8C\x82R`@\x80\x82 \x85\x90U\x84\x82R\x80\x82 \x86\x90U\x94\x81R`\x01\x80\x8E\x01\x90\x92R\x84\x81 \x83\x90U\x92\x83R\x92\x90\x91 U\x92\x90\x92\x01\x91Pa(\x1C\x90PV[P\x95PP`\x01\x01a'\xADV[P\x83Q`\x01\x14a)\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBeaconChainMock._buildMerkleTree`D\x82\x01Ru: invalid tree somehow`P\x1B`d\x82\x01R`\x84\x01a\x07\x02V[\x83`\0\x81Q\x81\x10a*\x05Wa*\x05aI\x13V[` \x02` \x01\x01Q\x90P\x93\x92PPPV[```\0a*\"a?\xA2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a*9Wa*9aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*bW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x81Q\x81\x10\x15a\x07\xC6Wd\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x1E` R`@\x90 T\x82Q\x83\x90\x83\x90\x81\x10a*\x9CWa*\x9CaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*hV[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R``\x91`\0\x91\x90\x80\x82\x01a\x04\0\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81\x10\x15a+\x16Wa*\xEE\x81`\x01aJ=V[`\0\x1B\x82\x82\x81Q\x81\x10a+\x03Wa+\x03aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*\xDAV[P\x83\x81`\x0B\x81Q\x81\x10a++Wa++aI\x13V[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0C\x81Q\x81\x10a+KWa+KaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x93\x92PPPV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81\x10\x15a+\xC3Wa+\x9B\x81`\x01aJ=V[`\0\x1B\x82\x82\x81Q\x81\x10a+\xB0Wa+\xB0aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a+\x87V[P\x82\x81`\x03\x81Q\x81\x10a+\xD8Wa+\xD8aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x82a,\x14\x83a?\xDAV[`@Qa&\xCE\x92\x91\x90aK\xD7V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a,\\Wa,\\aBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a,\x86W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81`\0\x80[`\x03\x81\x10\x15a,\xEEW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x89\x01\x84\x01\x81\x90R\x96\x84R`\x07\x01\x90\x91R\x90 T\x92\x82a,\xE2\x81aL\x05V[\x93PPP`\x01\x01a,\x8EV[P`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x81R`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\"\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a-4\x90\x82aLeV[PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a-wWa-waBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a-\xA1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81`\0a-\xD2` \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJ\x9FV[\x90P`\0\x80[`\x05\x81\x10\x15a.8W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x87\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x05\x01\x90\x91R\x90 T\x93\x82a.,\x81aL\x05V[\x93PPP`\x01\x01a-\xD8V[P\x80[\x82\x81\x10\x15a.\x9AW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x87\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x07\x01\x90\x91R\x90 T\x93\x82a.\x8E\x81aL\x05V[\x93PPP`\x01\x01a.;V[P`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`#\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a.\xE0\x90\x82aLeV[PPPPPPPPV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` R`@\x81 \x90[`\x1DT\x81\x10\x15a0\xD0W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a/JWa/JaBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/tW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0a/\x82\x83a:\x0BV[\x90P`\0a/\x8F\x82a<\x92V[\x90P`\0\x80[a/\xA1`(`\x01aJ=V[\x81\x10\x15a/\xFCW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R\x80\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x01\x01\x90\x91R\x90 T\x92\x82a/\xF0\x81aL\x05V[\x93PPP`\x01\x01a/\x95V[P\x80[`\x05a0\r`(`\x01aJ=V[a0\x17\x91\x90aJ=V[\x81\x10\x15a0uW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x05\x01\x90\x91R\x90 T\x92\x82a0i\x81aL\x05V[\x93PPP`\x01\x01a/\xFFV[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x87\x81R`@\x90\x91 \x84Qa0\x9B\x92\x86\x01\x90aA\xF7V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x87\x90R`@\x90 `\x01\x01a0\xBF\x85\x82aLeV[PP`\x01\x90\x93\x01\x92Pa/\x06\x91PPV[PPV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`%` R`@\x81 \x90a0\xF7a?\xA2V[\x90P`\0[\x81\x81\x10\x15a2)W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a1>Wa1>aBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1hW` \x82\x01\x81\x806\x837\x01\x90P[Pd\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x1E` R`@\x81 T\x91\x92P\x81\x90\x80[a1\x93`&`\x01aJ=V[\x81\x10\x15a1\xF1W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x02\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x03\x01\x90\x91R\x90 T\x92\x82a1\xE5\x81aL\x05V[\x93PPP`\x01\x01a1\x87V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x88\x90R`@\x90 \x83\x81U`\x01\x01a2\x18\x85\x82aLeV[PP`\x01\x90\x93\x01\x92Pa0\xFC\x91PPV[PPPV[`\0\x80[`\x1DT\x81\x10\x15a2\xD6W`\0`\x1D\x82\x81T\x81\x10a2QWa2QaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a2tWPa2\xCEV[`\x03\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x01a2\xCCW`\0a2\xA4\x83a%\xFAV[\x90Pa2\xB1`\x01\x82aI\xB8V[\x90P\x83a2\xBD\x81aL\x05V[\x94PPa2\xCA\x83\x82a&\x05V[P[P[`\x01\x01a22V[Pa\r\x84`@Q\x80``\x01`@R\x80`&\x81R` \x01aN\xFA`&\x919\x82a&\x9DV[`\0\x80[`\x1DT\x81\x10\x15a4\x9FW`\0`\x1D\x82\x81T\x81\x10a3\x1CWa3\x1CaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a3?WPa4\x97V[`\0c;\x9A\xCA\0a3O\x84a%\xFAV[`\x01`\x01`@\x1B\x03\x16a3b\x91\x90aJPV[\x90P`\0a3x\x83`\x02\x01\x80Ta$B\x90aI\xF6V[\x90P`\0\x80a3\x8Bc;\x9A\xCA\0\x85aJ\x9FV[`\x03\x86\x01T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a3\xC7W\x83`\0\x03a3\xBCWPPPPPa4\x97V[P\x82\x90P`\0a3\xF6V[h\x01\xBC\x16\xD6t\xEC\x80\0\0\x84\x11\x15a3\xF6Wa3\xEBh\x01\xBC\x16\xD6t\xEC\x80\0\0\x85aJ*V[\x91Pd\x07sY@\0\x90P[`\x1BT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90c\xC8\x8A^m\x90\x85\x90a4\x1B\x90\x86\x90\x83\x161aJ=V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4aW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4uW=`\0\x80>=`\0\xFD[PPPP\x81\x87a4\x85\x91\x90aJ=V[\x96Pa4\x91\x86\x82a&\x05V[PPPPP[`\x01\x01a2\xFDV[P\x80\x15a\r\x84Wa\r\x84`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- withdrew excess balance\0\0\0\0\0\0\0\x81RP\x82a&\x9DV[`\0a\x1DB`\x04\x83aJ\xB3V[`\x1DT`\0\x90a5\x03`\x04\x82aM#V[d\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a6\xECW`\x1DT`\0\x90a5(\x90`\x01`\x01`@\x1B\x03aI\xD7V[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82\x81`0\x01R`\x1D`@Q\x80`\xE0\x01`@R\x80`\x01\x15\x15\x81R` \x01`\0\x15\x15\x81R` \x01`\x02\x84`\0`\x80\x1B`@Q` \x01a5\x8C\x92\x91\x90aMMV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra5\xA6\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a5\xC3W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE6\x91\x90aK\xBEV[\x81R`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x81\x85\x01\x92\x90\x92R`\x01`\x01`@\x1B\x03\x88\x81\x16\x84\x86\x01R``\x80\x86\x01\x82\x90R`\x80\x90\x95\x01R\x85T`\x01\x80\x82\x01\x88U\x96\x83R\x91\x81\x90 \x85Q`\x04\x90\x93\x02\x01\x80T\x91\x86\x01Qa\xFF\xFF\x19\x90\x92\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x81U\x90\x83\x01Q\x93\x81\x01\x93\x90\x93U\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a6z\x90\x82aLeV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua6\xDC\x83\x83a&\x05V[\x82a6\xE6\x81aM|V[\x93PPPP[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R`\0\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81\x81`0\x01R`\x1D`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x15\x15\x81R` \x01`\x02\x84`\0`\x80\x1B`@Q` \x01a7L\x92\x91\x90aMMV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra7f\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a7\x83W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA6\x91\x90aK\xBEV[\x81R` \x01\x87\x81R` \x01\x86`\x01`\x01`@\x1B\x03\x16\x81R` \x01a7\xC8a\x12dV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x91\x82\x01R\x82T`\x01\x81\x81\x01\x85U`\0\x94\x85R\x93\x82\x90 \x83Q`\x04\x90\x92\x02\x01\x80T\x92\x84\x01Q\x15\x15a\x01\0\x02a\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16a\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x81U`@\x82\x01Q\x92\x81\x01\x92\x90\x92U``\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a8=\x90\x82aLeV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua\x1D>\x82\x85a&\x05V[`\0\x80a8\xAD`\x04\x84aM#V[a8\xB8\x90`@aM\xA3V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\"\x17\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17`\xFF`8\x1B`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[`\0\x80a96\x83aM\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[``a\x1DB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a@^V[``a\x1DB`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a@^V[`\0\x80a9\xA5`\x04\x85aM#V[a9\xB0\x90`\x01aM\xE7V[a9\xBB\x90`@aM\xA3V[a9\xC7\x90a\x01\0aN\x04V[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x01`@\x1B\x03\x81\x1B\x19\x85\x81\x16`\0a9\xE9\x86a@\xA8V[\x90P`\0a9\xF8\x85`\xC0aJ*V[\x91\x90\x91\x1C\x91\x90\x91\x17\x97\x96PPPPPPPV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0`\x1D\x84d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a:PWa:PaI\x13V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x04\x90\x93\x02\x90\x91\x01\x80T`\xFF\x80\x82\x16\x15\x15\x85Ra\x01\0\x90\x91\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01\x83\x01T\x90\x82\x01R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a:\xA9\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta:\xD5\x90aI\xF6V[\x80\x15a;\"W\x80`\x1F\x10a:\xF7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a;\"V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a;\x05W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x82\x16` \x84\x01R`\x01`@\x1B\x82\x04\x81\x16`@\x80\x85\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x92\x04\x16``\x90\x92\x01\x91\x90\x91R\x81\x01Q\x83Q\x91\x92P\x90\x83\x90`\0\x90a;\x7FWa;\x7FaI\x13V[` \x02` \x01\x01\x81\x81RPP\x80``\x01Qa;\x99\x90aM\xC3V[\x82`\x01\x81Q\x81\x10a;\xACWa;\xACaI\x13V[` \x02` \x01\x01\x81\x81RPPa;\xC5\x81`\x80\x01Qa@\xA8V[\x82`\x02\x81Q\x81\x10a;\xD8Wa;\xD8aI\x13V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q`@Q` \x01a;\xFD\x91\x15\x15\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra<\x15\x90aM\xC3V[\x82`\x03\x81Q\x81\x10a<(Wa<(aI\x13V[` \x02` \x01\x01\x81\x81RPPa<A\x81`\xA0\x01Qa@\xA8V[\x82`\x05\x81Q\x81\x10a<TWa<TaI\x13V[` \x02` \x01\x01\x81\x81RPPa<m\x81`\xC0\x01Qa@\xA8V[\x82`\x06\x81Q\x81\x10a<\x80Wa<\x80aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x92\x91PPV[`\0\x80`\x02\x83Qa<\xA3\x91\x90aJ\x9FV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xBFWa<\xBFaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xE8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a=\xE5W`\x02\x85a=\x03\x83\x83aJPV[\x81Q\x81\x10a=\x13Wa=\x13aI\x13V[` \x02` \x01\x01Q\x86\x83`\x02a=)\x91\x90aJPV[a=4\x90`\x01aJ=V[\x81Q\x81\x10a=DWa=DaI\x13V[` \x02` \x01\x01Q`@Q` \x01a=f\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra=\x80\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a=\x9DW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xC0\x91\x90aK\xBEV[\x82\x82\x81Q\x81\x10a=\xD2Wa=\xD2aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a<\xEEV[Pa=\xF1`\x02\x83aJ\x9FV[\x91P[\x81\x15a?\x07W`\0[\x82\x81\x10\x15a>\xF4W`\x02\x82a>\x12\x83\x83aJPV[\x81Q\x81\x10a>\"Wa>\"aI\x13V[` \x02` \x01\x01Q\x83\x83`\x02a>8\x91\x90aJPV[a>C\x90`\x01aJ=V[\x81Q\x81\x10a>SWa>SaI\x13V[` \x02` \x01\x01Q`@Q` \x01a>u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra>\x8F\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a>\xACW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xCF\x91\x90aK\xBEV[\x82\x82\x81Q\x81\x10a>\xE1Wa>\xE1aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a=\xFDV[Pa?\0`\x02\x83aJ\x9FV[\x91Pa=\xF4V[\x80`\0\x81Q\x81\x10a?\x1AWa?\x1AaI\x13V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0`d\x82\x10a?}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7F_getZeroNode: invalid depth\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x02V[`&\x82\x81T\x81\x10a?\x90Wa?\x90aI\x13V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[`\x1DT`\0\x90\x15a?\xD4W`\x1DT`\x04\x90a?\xBF\x90`\x01\x90aJ*V[a?\xC9\x91\x90aJ\x9FV[a\x13&\x90`\x01aJ=V[P`\0\x90V[`@Qc\x16#C=`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R``\x90a\x1DB\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra@Y\x91\x90\x81\x01\x90aN!V[aA\x1EV[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a@\x91\x93\x92\x91\x90aN\x8EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`8\x81\x81\x1B`\xFF`8\x1B\x16`(\x83\x81\x1Bf\xFF\0\0\0\0\0\0\x16\x91\x90\x91\x17`\x18\x84\x81\x1Be\xFF\0\0\0\0\0\x16\x91\x90\x91\x17`\x08\x85\x81\x1Bd\xFF\0\0\0\0\x16\x91\x90\x91\x17c\xFF\0\0\0\x91\x86\x90\x1C\x91\x90\x91\x16\x17b\xFF\0\0\x91\x85\x90\x1C\x91\x90\x91\x16\x17a\xFF\0\x91\x84\x90\x1C\x91\x90\x91\x16\x17`\xFF\x92\x90\x91\x1C\x91\x90\x91\x16\x17`\xC0\x1B\x90V[``a\x1DB`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[2m`\xE0\x1B\x81RP\x83a@^V[`@Q\x80``\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01aA\x81`@Q\x80`@\x01`@R\x80`\0\x80\x19\x16\x81R` \x01``\x81RP\x90V[\x81R` \x01aA\xA3`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80`\x80\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01aA\xE3`@Q\x80`@\x01`@R\x80`\0\x80\x19\x16\x81R` \x01``\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15aB2W\x91` \x02\x82\x01[\x82\x81\x11\x15aB2W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\x17V[PaB>\x92\x91PaBBV[P\x90V[[\x80\x82\x11\x15aB>W`\0\x81U`\x01\x01aBCV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\x95WaB\x95aBWV[`@R\x91\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F>W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aB\xC3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xDCWaB\xDCaBWV[\x80`\x05\x1BaB\xEC` \x82\x01aBmV[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15aC\x08W`\0\x80\xFD[` \x86\x01\x92P[\x83\x83\x10\x15aC1WaC \x83aB\x9DV[\x82R` \x92\x83\x01\x92\x90\x91\x01\x90aC\x0FV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aCMW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aCcW`\0\x80\xFD[a\"\x17\x84\x82\x85\x01aB\xB2V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xB0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aC\x89V[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aC\xCDW`\0\x80\xFD[aC\xD6\x82aB\x9DV[\x93\x92PPPV[`\0[\x83\x81\x10\x15aC\xF8W\x81\x81\x01Q\x83\x82\x01R` \x01aC\xE0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaD\x19\x81` \x86\x01` \x86\x01aC\xDDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x80Q\x82R`\0` \x82\x01Q`@` \x85\x01Ra\"\x17`@\x85\x01\x82aD\x01V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aD~W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aD`V[P\x93\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q```@\x84\x01RaD\xB7`\x80\x84\x01\x82aD-V[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01R\x80Q`@\x83RaD\xDD`@\x84\x01\x82aDLV[\x90P` \x82\x01Q\x91P\x82\x81\x03` \x84\x01RaC1\x81\x83aD\x01V[` \x81R`\0aC\xD6` \x83\x01\x84aD\x01V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15aE\xB9W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15aE\xA1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aEuV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE3V[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15aE\xB9W`?\x19\x87\x86\x03\x01\x84RaF\t\x85\x83QaD\x01V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE\xEDV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15aFnW`\x1F\x19\x85\x84\x03\x01\x88RaFX\x83\x83QaDLV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aF<V[P\x90\x96\x95PPPPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`\x80`@\x84\x01RaF\xA9`\xA0\x84\x01\x82aD-V[`@\x85\x01Q\x84\x82\x03`\x1F\x19\x01``\x86\x01R\x80Q\x80\x83R\x91\x92P` \x90\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01`\0[\x82\x81\x10\x15aG\x08W`\x1F\x19\x86\x83\x03\x01\x84RaF\xF3\x82\x86QaD\x01V[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01aF\xD7V[P``\x88\x01Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01R\x94PaG&\x81\x86aF\x1EV[\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aGEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG[W`\0\x80\xFD[aGg\x85\x82\x86\x01aB\xB2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aG\x84W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[` \x81R`\0\x82Q`@` \x84\x01RaG\xAB``\x84\x01\x82aD-V[` \x85\x81\x01Q\x85\x83\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x84R\x92\x93P\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01`\0[\x82\x81\x10\x15aH)W`\x1F\x19\x86\x83\x03\x01\x84R\x84Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q\x90P```@\x84\x01RaH\x13``\x84\x01\x82aD\x01V[` \x96\x87\x01\x96\x95\x90\x95\x01\x94\x92PP`\x01\x01aG\xD7V[P\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xB0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aHOV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x86WaH\x86aBWV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15aH\xA6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xBCW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aH\xCDW`\0\x80\xFD[\x805aH\xE0aH\xDB\x82aHmV[aBmV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aH\xF5W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`S\x90\x82\x01R\x7FBeaconChainMock: attempting to e`@\x82\x01R\x7Fxit dummy validator. We need tho``\x82\x01Rr\x0El\xA4\x0C\xCD\xEED\x0E\x0EM\xED\xEC\xCC\xEC\xAD\xC4\x07\xC7E`k\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\x01\x81\x81\x1C\x90\x82\x16\x80aJ\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\xC6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[\x80\x82\x01\x80\x82\x11\x15a\x1DBWa\x1DBaI\xA2V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1DBWa\x1DBaI\xA2V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x12]Wa\x12]aI\xA2V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aJ\xAEWaJ\xAEaJ\x89V[P\x04\x90V[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aJ\xCAWaJ\xCAaJ\x89V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90aK\0\x81`\x04\x85\x01` \x87\x01aC\xDDV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82QaK \x81\x84` \x87\x01aC\xDDV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aK<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14aC\xD6W`\0\x80\xFD[`\0\x82aK[WaK[aJ\x89V[P\x06\x90V[`\0\x83QaKr\x81\x84` \x88\x01aC\xDDV[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x83QaK\x90\x81`\x01\x84\x01` \x88\x01aC\xDDV[\x01`\x01\x01\x94\x93PPPPV[`@\x81R`\0aK\xAF`@\x83\x01\x85aD\x01V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK\xD0W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0aK\xEA`@\x83\x01\x85aD\x01V[\x82\x81\x03` \x84\x01RaK\xFC\x81\x85aD\x01V[\x95\x94PPPPPV[`\0`\x01\x82\x01aL\x17WaL\x17aI\xA2V[P`\x01\x01\x90V[`\x1F\x82\x11\x15a2)W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aLEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F8W`\0\x81U`\x01\x01aLQV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL~WaL~aBWV[aL\x92\x81aL\x8C\x84TaI\xF6V[\x84aL\x1EV[` `\x1F\x82\x11`\x01\x81\x14aL\xC6W`\0\x83\x15aL\xAEWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0F8V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aL\xF6W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aL\xD6V[P\x84\x82\x10\x15aM\x14W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aM:WaM:aJ\x89V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0\x83QaM_\x81\x84` \x88\x01aC\xDDV[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0d\xFF\xFF\xFF\xFF\xFF\x82\x16d\xFF\xFF\xFF\xFF\xFF\x81\x03aM\x9AWaM\x9AaI\xA2V[`\x01\x01\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x12]Wa\x12]aI\xA2V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x07\xC6W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[d\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\0` \x82\x84\x03\x12\x15aN3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aNIW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aNZW`\0\x80\xFD[\x80QaNhaH\xDB\x82aHmV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aN}W`\0\x80\xFD[aK\xFC\x82` \x83\x01` \x86\x01aC\xDDV[`\0\x84QaN\xA0\x81\x84` \x89\x01aC\xDDV[\x84Q\x90\x83\x01\x90aN\xB4\x81\x83` \x89\x01aC\xDDV[\x84Q\x91\x01\x90aN\xC7\x81\x83` \x88\x01aC\xDDV[\x01\x95\x94PPPPPV\xFE-- no validators; added empty block root- generated rewards for num validators\xA2dipfsX\"\x12 \xC6\xC7\xBCQ:\x93\x97\xBD\xE6\xC2:)a!\t\xD5\x9A*,ac\xEC\x81\xEEn,\xE4Kh\x87\xCE\x95dsolcC\0\x08\x1B\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x006W`\x005`\xE0\x1C\x80cd5\x99\xF2\x14a\x01\xABW\x80c\xAC\xD4\x14\xA8\x14a\x01\xE7W[` 6\x14a\0\x9FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7F4788OracleMock.fallback: malform`D\x82\x01Rjed msg.data`\xA8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\0\xAB6\x82a\x02\x14V[\x90P\x80`\0\x03a\x01\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7F4788OracleMock.fallback: timesta`D\x82\x01Rf\x06\xD7\x02\x06\x972\x03`\xCC\x1B`d\x82\x01R`\x84\x01a\0\x96V[`\0\x81\x81R` \x81\x90R`@\x81 T\x90\x81\x90\x03a\x01\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7F4788OracleMock.fallback: no bloc`D\x82\x01R\x7Fk root found. DID YOU USE CHEATS`d\x82\x01Re.WARP?`\xD0\x1B`\x84\x82\x01R`\xA4\x01a\0\x96V[\x80`\0R` `\0\xF3[a\x01\xD5a\x01\xB96`\x04a\x02\x14V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x02\x12a\x01\xF56`\x04a\x02-V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16`\0\x90\x81R` \x81\x90R`@\x90 UV[\0[`\0` \x82\x84\x03\x12\x15a\x02&W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02@W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02XW`\0\x80\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 \x1C\x07\xC1\x83\x1Cm\xFC\xE4\xCDf\xE3\xA8\xB3\\\\\xF3'\xA7\xE2\xB4:\xA8^\xF4\xE4\xDA\xAF\xE0\xF7:\x96\xC7dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106102045760003560e01c806386a6f9e111610118578063c76f25c0116100a0578063f0acd9881161006f578063f0acd988146105c6578063f7213873146105db578063f833eb63146105fb578063f8f98a4e1461061b578063fa7626d41461063b57600080fd5b8063c76f25c014610546578063e20c9f7114610573578063e3cefb4214610588578063ed3c16051461059d57600080fd5b8063a50a3a1a116100e7578063a50a3a1a14610492578063aa47389c146104bf578063b1b6f6a1146104ef578063b5508aa91461051c578063ba414fa61461053157600080fd5b806386a6f9e1146102da578063908820e014610429578063916a17c614610449578063a3f4df7e1461045e57600080fd5b80633cf80e6c1161019b5780635e6cc2fc1161016a5780635e6cc2fc1461038357806366d9a9a0146103b05780636b3abd97146103d257806376671808146103f257806385226c811461040757600080fd5b80633cf80e6c1461032f5780633e5e3c23146103445780633f7286f41461035957806359d095dd1461036e57600080fd5b806329992faa116101d757806329992faa146102c35780632def6009146102da578063330bc27e146102fa578063357e951f1461030f57600080fd5b806314360958146102095780631ed7831c146102465780631f54365c1461026857806323e82c4c14610296575b600080fd5b34801561021557600080fd5b5061022961022436600461433b565b610655565b6040516001600160401b0390911681526020015b60405180910390f35b34801561025257600080fd5b5061025b6107cc565b60405161023d919061436f565b34801561027457600080fd5b506102886102833660046143bb565b61082e565b60405190815260200161023d565b3480156102a257600080fd5b506102b66102b13660046143bb565b610863565b60405161023d9190614488565b3480156102cf57600080fd5b506102d8610a7d565b005b3480156102e657600080fd5b506102296102f53660046143bb565b610f3f565b34801561030657600080fd5b50610229600a81565b34801561031b57600080fd5b50601c54610229906001600160401b031681565b34801561033b57600080fd5b506102d8610f7e565b34801561035057600080fd5b5061025b610fc5565b34801561036557600080fd5b5061025b611025565b34801561037a57600080fd5b506102d8611085565b34801561038f57600080fd5b506103a361039e3660046143bb565b6110bc565b60405161023d91906144f8565b3480156103bc57600080fd5b506103c56110ec565b60405161023d919061450b565b3480156103de57600080fd5b506102886103ed36600461433b565b6111db565b3480156103fe57600080fd5b50610229611264565b34801561041357600080fd5b5061041c61132b565b60405161023d91906145c5565b34801561043557600080fd5b506102886104443660046143bb565b6113fb565b34801561045557600080fd5b506103c5611429565b34801561046a57600080fd5b5060408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b60208201526103a3565b34801561049e57600080fd5b506104b26104ad36600461433b565b61150f565b60405161023d919061467a565b3480156104cb57600080fd5b506104df6104da3660046143bb565b611935565b604051901515815260200161023d565b3480156104fb57600080fd5b5061050f61050a366004614732565b611986565b60405161023d919061478f565b34801561052857600080fd5b5061041c611d48565b34801561053d57600080fd5b506104df611e18565b34801561055257600080fd5b5061056661056136600461433b565b611f43565b60405161023d9190614835565b34801561057f57600080fd5b5061025b612002565b34801561059457600080fd5b50610229600181565b6105b06105ab366004614894565b612062565b60405164ffffffffff909116815260200161023d565b3480156105d257600080fd5b506102d861221f565b3480156105e757600080fd5b506102296105f63660046143bb565b612265565b34801561060757600080fd5b506102296106163660046143bb565b612279565b34801561062757600080fd5b506102296106363660046143bb565b6122bf565b34801561064757600080fd5b506000546104df9060ff1681565b60006106876040518060400160405280600f81526020016e736c61736856616c696461746f727360881b81525061256a565b60005b82518110156107c65760008382815181106106a7576106a7614913565b602002602001015190506000601d8264ffffffffff16815481106106cd576106cd614913565b60009182526020909120600490910201805490915060ff161561070b5760405162461bcd60e51b815260040161070290614929565b60405180910390fd5b8054610100900460ff1661076257805461ff00191661010017815561072e611264565b6107399060016149b8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055505b600061076d836125fa565b90506001600160401b038116600a11156107965761078b81866149b8565b9450600090506107b1565b6107a1600a866149b8565b94506107ae600a826149d7565b90505b6107bb8382612605565b50505060010161068a565b50919050565b6060600d80548060200260200160405190810160405280929190818152602001828054801561082457602002820191906000526020600020905b81546001600160a01b03168152600190910190602001808311610806575b5050505050905090565b6000601d8264ffffffffff168154811061084a5761084a614913565b9060005260206000209060040201600101549050919050565b61086b614146565b6021546001600160401b0316600090815260246020908152604080832064ffffffffff8616845282528083208151815460609481028201850184529281018381529093919284928491908401828280156108e457602002820191906000526020600020905b8154815260200190600101908083116108d0575b505050505081526020016001820180546108fd906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054610929906149f6565b80156109765780601f1061094b57610100808354040283529160200191610976565b820191906000526020600020905b81548152906001019060200180831161095957829003601f168201915b505050919092525050604080516060810182526021546001600160401b03168082526000908152602260209081529083902083518085019094528054845260018101805496975092958287019550909291840191906109d4906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054610a00906149f6565b8015610a4d5780601f10610a2257610100808354040283529160200191610a4d565b820191906000526020600020905b815481529060010190602001808311610a3057829003601f168201915b50505091909252505050815260408051808201909152835181526020938401518185015292019190915292915050565b60005b601d54811015610b1a576000601d8281548110610a9f57610a9f614913565b60009182526020909120600490910201805490915060ff1615610ac25750610b12565b6000610acd836125fa565b9050640773594000816001600160401b03161115610aed57506407735940005b600391909101805467ffffffffffffffff19166001600160401b039092169190911790555b600101610a80565b50610b596040518060400160405280601c81526020017f2d2075706461746564206566666563746976652062616c616e6365730000000081525061266e565b610b9b6040518060400160405280601081526020016f05a5a40c6eae4e4cadce840cae0dec6d60831b815250610b8d611264565b6001600160401b031661269d565b6000610ba5611264565b601b549091506001600160a01b031663e5d6bf02610bc2836126da565b6040516001600160e01b031960e084901b1681526001600160401b039091166004820152602401600060405180830381600087803b158015610c0357600080fd5b505af1158015610c17573d6000803e3d6000fd5b50506021805467ffffffffffffffff1916426001600160401b0316179055505060408051808201909152601681527505a40d4eadae0cac840e8de40dccaf0e840cae0dec6d60531b6020820152610c7090610b8d611264565b610cae6040518060400160405280601d81526020017f2d206275696c64696e6720626561636f6e20737461746520747265657300000081525061266e565b601d5415610ccd57601d54610cc590600190614a2a565b602055610d87565b60215460405163159a829560e31b81526001600160401b0390911660048201527fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4706024820152720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a890604401600060405180830381600087803b158015610d4b57600080fd5b505af1158015610d5f573d6000803e3d6000fd5b50505050610d84604051806060016040528060288152602001614ed26028913961266e565b50565b6000610dbf610d9461271a565b610da060286001614a3d565b6021546001600160401b031660009081526027602052604090206127a9565b90506000610dfc610dce612a16565b610dda60266001614a3d565b6021546001600160401b031660009081526027602052604090206002016127a9565b90506000610e32610e0d8484612aaf565b6021546001600160401b031660009081526027602052604090206005906004016127a9565b90506000610e67610e4283612b5d565b6021546001600160401b031660009081526027602052604090206003906006016127a9565b9050610e9f604051806040016040528060148152602001730b4b4818995858dbdb88189b1bd8dac81c9bdbdd60621b81525082612be9565b60215460405163159a829560e31b81526001600160401b03909116600482015260248101829052720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a890604401600060405180830381600087803b158015610efe57600080fd5b505af1158015610f12573d6000803e3d6000fd5b50505050610f1f82612c22565b610f2883612d3d565b610f30612eea565b610f386130d4565b5050505050565b6000601d8264ffffffffff1681548110610f5b57610f5b614913565b60009182526020909120600360049092020101546001600160401b031692915050565b610fab6040518060400160405280600c81526020016b0c2c8ecc2dcc6ca8ae0dec6d60a31b81525061256a565b610fb361322e565b610fbb6132f9565b610fc3610a7d565b565b6060600f805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b6060600e805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b610fb360405180604001604052806016815260200175616476616e636545706f63685f4e6f5265776172647360501b81525061256a565b60408051603080825260608281019093526000919060208201818036833750505060308101939093525090919050565b60606012805480602002602001604051908101604052809291908181526020016000905b828210156111d25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156111ba57602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b0319168152602001906004019060208260030104928301926001038202915080841161117c5790505b50505050508152505081526020019060010190611110565b50505050905090565b60008060005b835181101561125d57633b9aca00601d85838151811061120357611203614913565b602002602001015164ffffffffff168154811061122257611222614913565b600091825260209091206003600490920201015461124991906001600160401b0316614a50565b6112539083614a3d565b91506001016111e1565b5092915050565b601b54600090600160a01b90046001600160401b03164210156112ef5760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e2e63757272656e7445706f63683a2063757272656e60448201527f742074696d65206973206265666f72652067656e657369732074696d650000006064820152608401610702565b6112fb600c6020614a67565b601b546001600160401b039182169161131c91600160a01b90041642614a2a565b6113269190614a9f565b905090565b60606011805480602002602001604051908101604052809291908181526020016000905b828210156111d257838290600052602060002001805461136e906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461139a906149f6565b80156113e75780601f106113bc576101008083540402835291602001916113e7565b820191906000526020600020905b8154815290600101906020018083116113ca57829003601f168201915b50505050508152602001906001019061134f565b6000601e8161140b600485614ab3565b64ffffffffff16815260208101919091526040016000205492915050565b60606013805480602002602001604051908101604052809291908181526020016000905b828210156111d25760008481526020908190206040805180820182526002860290920180546001600160a01b031683526001810180548351818702810187019094528084529394919385830193928301828280156114f757602002820191906000526020600020906000905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116114b95790505b5050505050815250508152602001906001019061144d565b6115176141a8565b60005b825181101561160f5760205483828151811061153857611538614913565b602002602001015164ffffffffff1611156116075760405162461bcd60e51b815260206004820152607760248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f2076616c696461746f7220686173206e6f74206265656e20696e636c7564656460648201527f20696e20626561636f6e20636861696e207374617465202844494420594f552060848201527f43414c4c20616476616e636545706f6368205945543f2900000000000000000060a482015260c401610702565b60010161151a565b50604080516080810182526021546001600160401b031680825260009081526022602090815283822084518086019095528054855260018101805493958386019490938401919061165f906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461168b906149f6565b80156116d85780601f106116ad576101008083540402835291602001916116d8565b820191906000526020600020905b8154815290600101906020018083116116bb57829003601f168201915b505050505081525050815260200184516001600160401b038111156116ff576116ff614257565b60405190808252806020026020018201604052801561173257816020015b606081526020019060019003908161171d5790505b50815260200184516001600160401b0381111561175157611751614257565b60405190808252806020026020018201604052801561178457816020015b606081526020019060019003908161176f5790505b509052905060005b835181101561125d576021546001600160401b03166000908152602460205260408120855182908790859081106117c5576117c5614913565b602002602001015164ffffffffff1664ffffffffff1681526020019081526020016000206040518060400160405290816000820180548060200260200160405190810160405280929190818152602001828054801561184357602002820191906000526020600020905b81548152602001906001019080831161182f575b5050505050815260200160018201805461185c906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611888906149f6565b80156118d55780601f106118aa576101008083540402835291602001916118d5565b820191906000526020600020905b8154815290600101906020018083116118b857829003601f168201915b50505050508152505090508060200151836040015183815181106118fb576118fb614913565b602002602001018190525080600001518360600151838151811061192157611921614913565b60209081029190910101525060010161178c565b60006001600160401b038016601d8364ffffffffff168154811061195b5761195b614913565b6000918252602090912060049091020160030154600160801b90046001600160401b03161492915050565b6119b16040805160808101825260009181019182526060808201529081908152602001606081525090565b60005b8351811015611a83576020548482815181106119d2576119d2614913565b602002602001015164ffffffffff161115611a7b5760405162461bcd60e51b815260206004820152605b60248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f206e6f20636865636b706f696e742070726f6f6620666f756e6420286469642060648201527f796f752063616c6c20616476616e636545706f6368207965743f290000000000608482015260a401610702565b6001016119b4565b50604080516001600160401b038416600090815260236020528281206080830184528054938301938452600181018054929484939092916060850191611ac8906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611af4906149f6565b8015611b415780601f10611b1657610100808354040283529160200191611b41565b820191906000526020600020905b815481529060010190602001808311611b2457829003601f168201915b505050505081525050815260200185516001600160401b03811115611b6857611b68614257565b604051908082528060200260200182016040528015611bb557816020015b60408051606080820183526000808352602083015291810191909152815260200190600190039081611b865790505b509052905060005b8451811015611d3e576000858281518110611bda57611bda614913565b602002602001015190506000611bef826134e5565b6001600160401b038716600090815260256020908152604080832064ffffffffff851684528252808320815180830190925280548252600181018054959650939491939092840191611c40906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611c6c906149f6565b8015611cb95780601f10611c8e57610100808354040283529160200191611cb9565b820191906000526020600020905b815481529060010190602001808311611c9c57829003601f168201915b50505050508152505090506040518060600160405280601d8564ffffffffff1681548110611ce957611ce9614913565b906000526020600020906004020160010154815260200182600001518152602001826020015181525085602001518581518110611d2857611d28614913565b6020908102919091010152505050600101611bbd565b5090505b92915050565b60606010805480602002602001604051908101604052809291908181526020016000905b828210156111d2578382906000526020600020018054611d8b906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054611db7906149f6565b8015611e045780601f10611dd957610100808354040283529160200191611e04565b820191906000526020600020905b815481529060010190602001808311611de757829003601f168201915b505050505081526020019060010190611d6c565b60008054610100900460ff1615611e385750600054610100900460ff1690565b6000737109709ecfa91a80626ff3989d68f67f5b1dd12d3b15611f3e5760408051737109709ecfa91a80626ff3989d68f67f5b1dd12d602082018190526519985a5b195960d21b82840152825180830384018152606083019093526000929091611ec6917f667f9d70ca411d70ead50d8d5c22070dafc36ad75f3dcf5e7237b22ade9aecc491608001614add565b60408051601f1981840301815290829052611ee091614b0e565b6000604051808303816000865af19150503d8060008114611f1d576040519150601f19603f3d011682016040523d82523d6000602084013e611f22565b606091505b5091505080806020019051810190611f3a9190614b2a565b9150505b919050565b6060600082516001600160401b03811115611f6057611f60614257565b604051908082528060200260200182016040528015611f89578160200160208202803683370190505b50905060005b835181101561125d57601d848281518110611fac57611fac614913565b602002602001015164ffffffffff1681548110611fcb57611fcb614913565b906000526020600020906004020160010154828281518110611fef57611fef614913565b6020908102919091010152600101611f8f565b6060600c805480602002602001604051908101604052809291908181526020018280548015610824576020028201919060005260206000209081546001600160a01b03168152600190910190602001808311610806575050505050905090565b60006120916040518060400160405280600c81526020016b3732bbab30b634b230ba37b960a11b81525061256a565b34670de0b6b3a76400008110156121065760405162461bcd60e51b815260206004820152603360248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a206465604482015272706f7369742076616c756520746f6f206c6f7760681b6064820152608401610702565b612114633b9aca0082614b4c565b156121875760405162461bcd60e51b815260206004820152603860248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a20766160448201527f6c7565206e6f74206d756c7469706c65206f66206777656900000000000000006064820152608401610702565b6000612197633b9aca0083614a9f565b90506001600160401b0381111561220d5760405162461bcd60e51b815260206004820152603460248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a2064656044820152730e0dee6d2e840ecc2d8eaca40e8dede40d0d2ced60631b6064820152608401610702565b61221784826134f2565b949350505050565b61225d6040518060400160405280601781526020017f616476616e636545706f63685f4e6f576974686472617700000000000000000081525061256a565b610fbb61322e565b6000611d42612273836113fb565b8361389f565b6000601d8264ffffffffff168154811061229557612295614913565b6000918252602090912060049091020160030154600160801b90046001600160401b031692915050565b60006122ef6040518060400160405280600d81526020016c32bc34ba2b30b634b230ba37b960991b81525061256a565b6000601d8364ffffffffff168154811061230b5761230b614913565b60009182526020909120600490910201805490915060ff16156123405760405162461bcd60e51b815260040161070290614929565b6003810154600160801b90046001600160401b03908116146123b65760405162461bcd60e51b815260206004820152602960248201527f426561636f6e436861696e4d6f636b3a2076616c696461746f7220616c726561604482015268191e48195e1a5d195960ba1b6064820152608401610702565b6123be611264565b6123c99060016149b8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055506123fa836125fa565b9150612407836000612605565b60006124c5601d8564ffffffffff168154811061242657612426614913565b90600052602060002090600402016002018054612442906149f6565b80601f016020809104026020016040519081016040528092919081815260200182805461246e906149f6565b80156124bb5780601f10612490576101008083540402835291602001916124bb565b820191906000526020600020905b81548152906001019060200180831161249e57829003601f168201915b505050505061392a565b601b549091506001600160a01b031663c88a5e6d826124f1633b9aca006001600160401b038816614a50565b612505906001600160a01b03861631614a3d565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561254b57600080fd5b505af115801561255f573d6000803e3d6000fd5b505050505050919050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f506125bb6125b660408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b602082015290565b613946565b6125c48361396f565b6040516020016125d5929190614b60565b60408051601f19818403018152908290526125ef916144f8565b60405180910390a150565b6000611d4282612265565b6000601e81612615600486614ab3565b64ffffffffff1664ffffffffff16815260200190815260200160002054905061263f818484613997565b905080601e6000612651600487614ab3565b64ffffffffff168152602081019190915260400160002055505050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50816040516125ef91906144f8565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a882826040516126ce929190614b9c565b60405180910390a15050565b60006126e8600c6020614a67565b6126f38360016149b8565b6126fd9190614a67565b601b54611d429190600160a01b90046001600160401b03166149b8565b601d546060906000906001600160401b0381111561273a5761273a614257565b604051908082528060200260200182016040528015612763578160200160208202803683370190505b50905060005b601d548110156107c65761278461277f82613a0b565b613c92565b82828151811061279657612796614913565b6020908102919091010152600101612769565b6000805b838110156129815760006002865160016127c79190614a3d565b6127d19190614a9f565b90506000816001600160401b038111156127ed576127ed614257565b604051908082528060200260200182016040528015612816578160200160208202803683370190505b50905060005b82811015612975576000612831826002614a50565b90506000612840826001614a3d565b905060008a838151811061285657612856614913565b6020026020010151905060008b5183101561288c578b838151811061287d5761287d614913565b60200260200101519050612898565b61289588613f2b565b90505b6000600283836040516020016128b8929190918252602082015260400190565b60408051601f19818403018152908290526128d291614b0e565b602060405180830381855afa1580156128ef573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906129129190614bbe565b90508087878151811061292757612927614913565b60209081029190910181019190915260008481528c825260408082208590558482528082208690559481526001808e01909252848120839055928352929091205592909201915061281c9050565b509550506001016127ad565b5083516001146129f25760405162461bcd60e51b815260206004820152603660248201527f426561636f6e436861696e4d6f636b2e5f6275696c644d65726b6c65547265656044820152753a20696e76616c6964207472656520736f6d65686f7760501b6064820152608401610702565b83600081518110612a0557612a05614913565b602002602001015190509392505050565b60606000612a22613fa2565b6001600160401b03811115612a3957612a39614257565b604051908082528060200260200182016040528015612a62578160200160208202803683370190505b50905060005b81518110156107c65764ffffffffff81166000908152601e60205260409020548251839083908110612a9c57612a9c614913565b6020908102919091010152600101612a68565b6040805160208082526104208201909252606091600091908082016104008036833701905050905060005b8151811015612b1657612aee816001614a3d565b60001b828281518110612b0357612b03614913565b6020908102919091010152600101612ada565b508381600b81518110612b2b57612b2b614913565b6020026020010181815250508281600c81518110612b4b57612b4b614913565b60209081029190910101529392505050565b60408051600580825260c08201909252606091600091906020820160a08036833701905050905060005b8151811015612bc357612b9b816001614a3d565b60001b828281518110612bb057612bb0614913565b6020908102919091010152600101612b87565b508281600381518110612bd857612bd8614913565b602090810291909101015292915050565b7f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358382612c1483613fda565b6040516126ce929190614bd7565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612c5c57612c5c614257565b6040519080825280601f01601f191660200182016040528015612c86576020820181803683370190505b509050816000805b6003811015612cee576021546001600160401b0316600090815260276020908152604080832086845260068101835281842054858402890184018190529684526007019091529020549282612ce281614c05565b93505050600101612c8e565b5060408051808201825285815260208082018681526021546001600160401b0316600090815260229092529290208151815591519091906001820190612d349082614c65565b50505050505050565b60007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612d7757612d77614257565b6040519080825280601f01601f191660200182016040528015612da1576020820181803683370190505b509050816000612dd260207f0000000000000000000000000000000000000000000000000000000000000000614a9f565b90506000805b6005811015612e38576021546001600160401b03166000908152602760209081526040808320878452600481018352818420548584028a0184018190529784526005019091529020549382612e2c81614c05565b93505050600101612dd8565b50805b82811015612e9a576021546001600160401b03166000908152602760209081526040808320878452600681018352818420548584028a0184018190529784526007019091529020549382612e8e81614c05565b93505050600101612e3b565b5060408051808201825286815260208082018781526021546001600160401b0316600090815260239092529290208151815591519091906001820190612ee09082614c65565b5050505050505050565b6021546001600160401b03166000908152602460205260408120905b601d548110156130d05760007f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612f4a57612f4a614257565b6040519080825280601f01601f191660200182016040528015612f74576020820181803683370190505b5090506000612f8283613a0b565b90506000612f8f82613c92565b90506000805b612fa160286001614a3d565b811015612ffc576021546001600160401b03166000908152602760209081526040808320868452808352818420548584028a0184018190529684526001019091529020549282612ff081614c05565b93505050600101612f95565b50805b600561300d60286001614a3d565b6130179190614a3d565b811015613075576021546001600160401b03166000908152602760209081526040808320868452600481018352818420548584028a018401819052968452600501909152902054928261306981614c05565b93505050600101612fff565b5064ffffffffff8516600090815260208781526040909120845161309b928601906141f7565b5064ffffffffff851660009081526020879052604090206001016130bf8582614c65565b505060019093019250612f06915050565b5050565b6021546001600160401b03166000908152602560205260408120906130f7613fa2565b905060005b818110156132295760007f00000000000000000000000000000000000000000000000000000000000000006001600160401b0381111561313e5761313e614257565b6040519080825280601f01601f191660200182016040528015613168576020820181803683370190505b5064ffffffffff83166000908152601e60205260408120549192508190805b61319360266001614a3d565b8110156131f1576021546001600160401b03166000908152602760209081526040808320868452600281018352818420548584028a01840181905296845260030190915290205492826131e581614c05565b93505050600101613187565b5064ffffffffff851660009081526020889052604090208381556001016132188582614c65565b5050600190930192506130fc915050565b505050565b6000805b601d548110156132d6576000601d828154811061325157613251614913565b60009182526020909120600490910201805490915060ff161561327457506132ce565b600381015467fffffffffffffffe19600160801b9091046001600160401b0316016132cc5760006132a4836125fa565b90506132b16001826149b8565b9050836132bd81614c05565b9450506132ca8382612605565b505b505b600101613232565b50610d84604051806060016040528060268152602001614efa602691398261269d565b6000805b601d5481101561349f576000601d828154811061331c5761331c614913565b60009182526020909120600490910201805490915060ff161561333f5750613497565b6000633b9aca0061334f846125fa565b6001600160401b03166133629190614a50565b90506000613378836002018054612442906149f6565b905060008061338b633b9aca0085614a9f565b6003860154909150600160801b90046001600160401b03908116146133c757836000036133bc575050505050613497565b5082905060006133f6565b6801bc16d674ec8000008411156133f6576133eb6801bc16d674ec80000085614a2a565b915064077359400090505b601b546001600160a01b039081169063c88a5e6d90859061341b908690831631614a3d565b6040516001600160e01b031960e085901b1681526001600160a01b0390921660048301526024820152604401600060405180830381600087803b15801561346157600080fd5b505af1158015613475573d6000803e3d6000fd5b5050505081876134859190614a3d565b96506134918682612605565b50505050505b6001016132fd565b508015610d8457610d846040518060400160405280601981526020017f2d207769746864726577206578636573732062616c616e6365000000000000008152508261269d565b6000611d42600483614ab3565b601d54600090613503600482614d23565b64ffffffffff166000036136ec57601d54600090613528906001600160401b036149d7565b6040805160308082526060820190925291925060009190602082018180368337019050509050828160300152601d6040518060e00160405280600115158152602001600015158152602001600284600060801b60405160200161358c929190614d4d565b60408051601f19818403018152908290526135a691614b0e565b602060405180830381855afa1580156135c3573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906135e69190614bbe565b815260408051602080820183526000808352818501929092526001600160401b0388811684860152606080860182905260809095015285546001808201885596835291819020855160049093020180549186015161ffff1990921692151561ff00191692909217610100911515919091021781559083015193810193909355810151909190600282019061367a9082614c65565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909316919094161717929092161790556136dc8383612605565b826136e681614d7c565b93505050505b60408051603080825260608201909252600091602082018180368337019050509050818160300152601d6040518060e00160405280600015158152602001600015158152602001600284600060801b60405160200161374c929190614d4d565b60408051601f198184030181529082905261376691614b0e565b602060405180830381855afa158015613783573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906137a69190614bbe565b8152602001878152602001866001600160401b031681526020016137c8611264565b6001600160401b039081168252602091820152825460018181018555600094855293829020835160049092020180549284015115156101000261ff00199215159290921661ffff19909316929092171781556040820151928101929092556060810151909190600282019061383d9082614c65565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990931691909416171792909216179055611d3e8285612605565b6000806138ad600484614d23565b6138b8906040614da3565b64ffffffffff16905061221784821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161760ff60381b60889290921c919091161790565b60008061393683614dc3565b6001600160a01b03169392505050565b6060611d42604051806040016040528060058152602001641b5b39366d60d81b8152508361405e565b6060611d42604051806040016040528060048152602001631b5b336d60e01b8152508361405e565b6000806139a5600485614d23565b6139b0906001614de7565b6139bb906040614da3565b6139c790610100614e04565b64ffffffffff1690506001600160401b03811b1985811660006139e9866140a8565b905060006139f88560c0614a2a565b9190911c91909117979650505050505050565b60408051600880825261012082019092526060916000919060208201610100803683370190505090506000601d8464ffffffffff1681548110613a5057613a50614913565b60009182526020918290206040805160e0810182526004909302909101805460ff8082161515855261010090910416151593830193909352600183015490820152600282018054919291606084019190613aa9906149f6565b80601f0160208091040260200160405190810160405280929190818152602001828054613ad5906149f6565b8015613b225780601f10613af757610100808354040283529160200191613b22565b820191906000526020600020905b815481529060010190602001808311613b0557829003601f168201915b5050509183525050600391909101546001600160401b038082166020840152600160401b82048116604080850191909152600160801b909204166060909201919091528101518351919250908390600090613b7f57613b7f614913565b6020026020010181815250508060600151613b9990614dc3565b82600181518110613bac57613bac614913565b602002602001018181525050613bc581608001516140a8565b82600281518110613bd857613bd8614913565b6020026020010181815250508060200151604051602001613bfd911515815260200190565b604051602081830303815290604052613c1590614dc3565b82600381518110613c2857613c28614913565b602002602001018181525050613c418160a001516140a8565b82600581518110613c5457613c54614913565b602002602001018181525050613c6d8160c001516140a8565b82600681518110613c8057613c80614913565b60209081029190910101525092915050565b60008060028351613ca39190614a9f565b90506000816001600160401b03811115613cbf57613cbf614257565b604051908082528060200260200182016040528015613ce8578160200160208202803683370190505b50905060005b82811015613de557600285613d038383614a50565b81518110613d1357613d13614913565b602002602001015186836002613d299190614a50565b613d34906001614a3d565b81518110613d4457613d44614913565b6020026020010151604051602001613d66929190918252602082015260400190565b60408051601f1981840301815290829052613d8091614b0e565b602060405180830381855afa158015613d9d573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190613dc09190614bbe565b828281518110613dd257613dd2614913565b6020908102919091010152600101613cee565b50613df1600283614a9f565b91505b8115613f075760005b82811015613ef457600282613e128383614a50565b81518110613e2257613e22614913565b602002602001015183836002613e389190614a50565b613e43906001614a3d565b81518110613e5357613e53614913565b6020026020010151604051602001613e75929190918252602082015260400190565b60408051601f1981840301815290829052613e8f91614b0e565b602060405180830381855afa158015613eac573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190613ecf9190614bbe565b828281518110613ee157613ee1614913565b6020908102919091010152600101613dfd565b50613f00600283614a9f565b9150613df4565b80600081518110613f1a57613f1a614913565b602002602001015192505050919050565b600060648210613f7d5760405162461bcd60e51b815260206004820152601b60248201527f5f6765745a65726f4e6f64653a20696e76616c696420646570746800000000006044820152606401610702565b60268281548110613f9057613f90614913565b90600052602060002001549050919050565b601d5460009015613fd457601d54600490613fbf90600190614a2a565b613fc99190614a9f565b611326906001614a3d565b50600090565b604051631623433d60e31b815260048101829052606090611d4290737109709ecfa91a80626ff3989d68f67f5b1dd12d9063b11a19e890602401600060405180830381865afa158015614031573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f191682016040526140599190810190614e21565b61411e565b60608282604051806040016040528060048152602001631b5b306d60e01b81525060405160200161409193929190614e8e565b604051602081830303815290604052905092915050565b603881811b60ff60381b16602883811b66ff0000000000001691909117601884811b65ff00000000001691909117600885811b64ff00000000169190911763ff0000009186901c919091161762ff00009185901c919091161761ff009184901c919091161760ff9290911c919091161760c01b90565b6060611d42604051806040016040528060048152602001631b5b326d60e01b8152508361405e565b604051806060016040528060006001600160401b03168152602001614181604051806040016040528060008019168152602001606081525090565b81526020016141a3604051806040016040528060608152602001606081525090565b905290565b604051806080016040528060006001600160401b031681526020016141e3604051806040016040528060008019168152602001606081525090565b815260200160608152602001606081525090565b828054828255906000526020600020908101928215614232579160200282015b82811115614232578251825591602001919060010190614217565b5061423e929150614242565b5090565b5b8082111561423e5760008155600101614243565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b038111828210171561429557614295614257565b604052919050565b803564ffffffffff81168114611f3e57600080fd5b600082601f8301126142c357600080fd5b81356001600160401b038111156142dc576142dc614257565b8060051b6142ec6020820161426d565b9182526020818501810192908101908684111561430857600080fd5b6020860192505b83831015614331576143208361429d565b82526020928301929091019061430f565b9695505050505050565b60006020828403121561434d57600080fd5b81356001600160401b0381111561436357600080fd5b612217848285016142b2565b602080825282518282018190526000918401906040840190835b818110156143b05783516001600160a01b0316835260209384019390920191600101614389565b509095945050505050565b6000602082840312156143cd57600080fd5b6143d68261429d565b9392505050565b60005b838110156143f85781810151838201526020016143e0565b50506000910152565b600081518084526144198160208601602086016143dd565b601f01601f19169290920160200192915050565b8051825260006020820151604060208501526122176040850182614401565b600081518084526020840193506020830160005b8281101561447e578151865260209586019590910190600101614460565b5093949350505050565b602081526001600160401b03825116602082015260006020830151606060408401526144b7608084018261442d565b90506040840151601f198483030160608501528051604083526144dd604084018261444c565b90506020820151915082810360208401526143318183614401565b6020815260006143d66020830184614401565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156145b957868503603f19018452815180516001600160a01b031686526020908101516040828801819052815190880181905291019060009060608801905b808310156145a15783516001600160e01b03191682526020938401936001939093019290910190614575565b50965050506020938401939190910190600101614533565b50929695505050505050565b6000602082016020835280845180835260408501915060408160051b86010192506020860160005b828110156145b957603f19878603018452614609858351614401565b945060209384019391909101906001016145ed565b600082825180855260208501945060208160051b8301016020850160005b8381101561466e57601f1985840301885261465883835161444c565b602098890198909350919091019060010161463c565b50909695505050505050565b602081526001600160401b03825116602082015260006020830151608060408401526146a960a084018261442d565b6040850151848203601f19016060860152805180835291925060209081019181840191600582901b85010160005b8281101561470857601f198683030184526146f3828651614401565b602095860195949094019391506001016146d7565b506060880151878203601f190160808901529450614726818661461e565b98975050505050505050565b6000806040838503121561474557600080fd5b82356001600160401b0381111561475b57600080fd5b614767858286016142b2565b92505060208301356001600160401b038116811461478457600080fd5b809150509250929050565b6020815260008251604060208401526147ab606084018261442d565b602085810151858303601f19016040870152805180845292935081019181840191600582901b85010160005b8281101561482957601f198683030184528451805183526020810151602084015260408101519050606060408401526148136060840182614401565b60209687019695909501949250506001016147d7565b50979650505050505050565b602080825282518282018190526000918401906040840190835b818110156143b057835183526020938401939092019160010161484f565b60006001600160401b0382111561488657614886614257565b50601f01601f191660200190565b6000602082840312156148a657600080fd5b81356001600160401b038111156148bc57600080fd5b8201601f810184136148cd57600080fd5b80356148e06148db8261486d565b61426d565b8181528560208385010111156148f557600080fd5b81602084016020830137600091810160200191909152949350505050565b634e487b7160e01b600052603260045260246000fd5b60208082526053908201527f426561636f6e436861696e4d6f636b3a20617474656d7074696e6720746f206560408201527f7869742064756d6d792076616c696461746f722e205765206e6565642074686f6060820152720e6ca40ccdee440e0e4dedecccecadc407c745606b1b608082015260a00190565b634e487b7160e01b600052601160045260246000fd5b6001600160401b038181168382160190811115611d4257611d426149a2565b6001600160401b038281168282160390811115611d4257611d426149a2565b600181811c90821680614a0a57607f821691505b6020821081036107c657634e487b7160e01b600052602260045260246000fd5b81810381811115611d4257611d426149a2565b80820180821115611d4257611d426149a2565b8082028115828204841417611d4257611d426149a2565b6001600160401b03818116838216029081169081811461125d5761125d6149a2565b634e487b7160e01b600052601260045260246000fd5b600082614aae57614aae614a89565b500490565b600064ffffffffff831680614aca57614aca614a89565b8064ffffffffff84160491505092915050565b6001600160e01b0319831681528151600090614b008160048501602087016143dd565b919091016004019392505050565b60008251614b208184602087016143dd565b9190910192915050565b600060208284031215614b3c57600080fd5b815180151581146143d657600080fd5b600082614b5b57614b5b614a89565b500690565b60008351614b728184602088016143dd565b601760f91b9083019081528351614b908160018401602088016143dd565b01600101949350505050565b604081526000614baf6040830185614401565b90508260208301529392505050565b600060208284031215614bd057600080fd5b5051919050565b604081526000614bea6040830185614401565b8281036020840152614bfc8185614401565b95945050505050565b600060018201614c1757614c176149a2565b5060010190565b601f82111561322957806000526020600020601f840160051c81016020851015614c455750805b601f840160051c820191505b81811015610f385760008155600101614c51565b81516001600160401b03811115614c7e57614c7e614257565b614c9281614c8c84546149f6565b84614c1e565b6020601f821160018114614cc65760008315614cae5750848201515b600019600385901b1c1916600184901b178455610f38565b600084815260208120601f198516915b82811015614cf65787850151825560209485019460019092019101614cd6565b5084821015614d145786840151600019600387901b60f8161c191681555b50505050600190811b01905550565b600064ffffffffff831680614d3a57614d3a614a89565b8064ffffffffff84160691505092915050565b60008351614d5f8184602088016143dd565b6001600160801b0319939093169190920190815260100192915050565b600064ffffffffff821664ffffffffff8103614d9a57614d9a6149a2565b60010192915050565b64ffffffffff818116838216029081169081811461125d5761125d6149a2565b805160208083015191908110156107c65760001960209190910360031b1b16919050565b64ffffffffff8181168382160190811115611d4257611d426149a2565b64ffffffffff8281168282160390811115611d4257611d426149a2565b600060208284031215614e3357600080fd5b81516001600160401b03811115614e4957600080fd5b8201601f81018413614e5a57600080fd5b8051614e686148db8261486d565b818152856020838501011115614e7d57600080fd5b614bfc8260208301602086016143dd565b60008451614ea08184602089016143dd565b845190830190614eb48183602089016143dd565b8451910190614ec78183602088016143dd565b019594505050505056fe2d2d206e6f2076616c696461746f72733b20616464656420656d70747920626c6f636b20726f6f742d2067656e657261746564207265776172647320666f72206e756d2076616c696461746f7273a2646970667358221220c6c7bc513a9397bde6c23a29612109d59a2a2c6163ec81ee6e2ce44b6887ce9564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x04W`\x005`\xE0\x1C\x80c\x86\xA6\xF9\xE1\x11a\x01\x18W\x80c\xC7o%\xC0\x11a\0\xA0W\x80c\xF0\xAC\xD9\x88\x11a\0oW\x80c\xF0\xAC\xD9\x88\x14a\x05\xC6W\x80c\xF7!8s\x14a\x05\xDBW\x80c\xF83\xEBc\x14a\x05\xFBW\x80c\xF8\xF9\x8AN\x14a\x06\x1BW\x80c\xFAv&\xD4\x14a\x06;W`\0\x80\xFD[\x80c\xC7o%\xC0\x14a\x05FW\x80c\xE2\x0C\x9Fq\x14a\x05sW\x80c\xE3\xCE\xFBB\x14a\x05\x88W\x80c\xED<\x16\x05\x14a\x05\x9DW`\0\x80\xFD[\x80c\xA5\n:\x1A\x11a\0\xE7W\x80c\xA5\n:\x1A\x14a\x04\x92W\x80c\xAAG8\x9C\x14a\x04\xBFW\x80c\xB1\xB6\xF6\xA1\x14a\x04\xEFW\x80c\xB5P\x8A\xA9\x14a\x05\x1CW\x80c\xBAAO\xA6\x14a\x051W`\0\x80\xFD[\x80c\x86\xA6\xF9\xE1\x14a\x02\xDAW\x80c\x90\x88 \xE0\x14a\x04)W\x80c\x91j\x17\xC6\x14a\x04IW\x80c\xA3\xF4\xDF~\x14a\x04^W`\0\x80\xFD[\x80c<\xF8\x0El\x11a\x01\x9BW\x80c^l\xC2\xFC\x11a\x01jW\x80c^l\xC2\xFC\x14a\x03\x83W\x80cf\xD9\xA9\xA0\x14a\x03\xB0W\x80ck:\xBD\x97\x14a\x03\xD2W\x80cvg\x18\x08\x14a\x03\xF2W\x80c\x85\"l\x81\x14a\x04\x07W`\0\x80\xFD[\x80c<\xF8\x0El\x14a\x03/W\x80c>^<#\x14a\x03DW\x80c?r\x86\xF4\x14a\x03YW\x80cY\xD0\x95\xDD\x14a\x03nW`\0\x80\xFD[\x80c)\x99/\xAA\x11a\x01\xD7W\x80c)\x99/\xAA\x14a\x02\xC3W\x80c-\xEF`\t\x14a\x02\xDAW\x80c3\x0B\xC2~\x14a\x02\xFAW\x80c5~\x95\x1F\x14a\x03\x0FW`\0\x80\xFD[\x80c\x146\tX\x14a\x02\tW\x80c\x1E\xD7\x83\x1C\x14a\x02FW\x80c\x1FT6\\\x14a\x02hW\x80c#\xE8,L\x14a\x02\x96W[`\0\x80\xFD[4\x80\x15a\x02\x15W`\0\x80\xFD[Pa\x02)a\x02$6`\x04aC;V[a\x06UV[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02RW`\0\x80\xFD[Pa\x02[a\x07\xCCV[`@Qa\x02=\x91\x90aCoV[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x02\x88a\x02\x836`\x04aC\xBBV[a\x08.V[`@Q\x90\x81R` \x01a\x02=V[4\x80\x15a\x02\xA2W`\0\x80\xFD[Pa\x02\xB6a\x02\xB16`\x04aC\xBBV[a\x08cV[`@Qa\x02=\x91\x90aD\x88V[4\x80\x15a\x02\xCFW`\0\x80\xFD[Pa\x02\xD8a\n}V[\0[4\x80\x15a\x02\xE6W`\0\x80\xFD[Pa\x02)a\x02\xF56`\x04aC\xBBV[a\x0F?V[4\x80\x15a\x03\x06W`\0\x80\xFD[Pa\x02)`\n\x81V[4\x80\x15a\x03\x1BW`\0\x80\xFD[P`\x1CTa\x02)\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03;W`\0\x80\xFD[Pa\x02\xD8a\x0F~V[4\x80\x15a\x03PW`\0\x80\xFD[Pa\x02[a\x0F\xC5V[4\x80\x15a\x03eW`\0\x80\xFD[Pa\x02[a\x10%V[4\x80\x15a\x03zW`\0\x80\xFD[Pa\x02\xD8a\x10\x85V[4\x80\x15a\x03\x8FW`\0\x80\xFD[Pa\x03\xA3a\x03\x9E6`\x04aC\xBBV[a\x10\xBCV[`@Qa\x02=\x91\x90aD\xF8V[4\x80\x15a\x03\xBCW`\0\x80\xFD[Pa\x03\xC5a\x10\xECV[`@Qa\x02=\x91\x90aE\x0BV[4\x80\x15a\x03\xDEW`\0\x80\xFD[Pa\x02\x88a\x03\xED6`\x04aC;V[a\x11\xDBV[4\x80\x15a\x03\xFEW`\0\x80\xFD[Pa\x02)a\x12dV[4\x80\x15a\x04\x13W`\0\x80\xFD[Pa\x04\x1Ca\x13+V[`@Qa\x02=\x91\x90aE\xC5V[4\x80\x15a\x045W`\0\x80\xFD[Pa\x02\x88a\x04D6`\x04aC\xBBV[a\x13\xFBV[4\x80\x15a\x04UW`\0\x80\xFD[Pa\x03\xC5a\x14)V[4\x80\x15a\x04jW`\0\x80\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01Ra\x03\xA3V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x04\xB2a\x04\xAD6`\x04aC;V[a\x15\x0FV[`@Qa\x02=\x91\x90aFzV[4\x80\x15a\x04\xCBW`\0\x80\xFD[Pa\x04\xDFa\x04\xDA6`\x04aC\xBBV[a\x195V[`@Q\x90\x15\x15\x81R` \x01a\x02=V[4\x80\x15a\x04\xFBW`\0\x80\xFD[Pa\x05\x0Fa\x05\n6`\x04aG2V[a\x19\x86V[`@Qa\x02=\x91\x90aG\x8FV[4\x80\x15a\x05(W`\0\x80\xFD[Pa\x04\x1Ca\x1DHV[4\x80\x15a\x05=W`\0\x80\xFD[Pa\x04\xDFa\x1E\x18V[4\x80\x15a\x05RW`\0\x80\xFD[Pa\x05fa\x05a6`\x04aC;V[a\x1FCV[`@Qa\x02=\x91\x90aH5V[4\x80\x15a\x05\x7FW`\0\x80\xFD[Pa\x02[a \x02V[4\x80\x15a\x05\x94W`\0\x80\xFD[Pa\x02)`\x01\x81V[a\x05\xB0a\x05\xAB6`\x04aH\x94V[a bV[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02=V[4\x80\x15a\x05\xD2W`\0\x80\xFD[Pa\x02\xD8a\"\x1FV[4\x80\x15a\x05\xE7W`\0\x80\xFD[Pa\x02)a\x05\xF66`\x04aC\xBBV[a\"eV[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x02)a\x06\x166`\x04aC\xBBV[a\"yV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x02)a\x0666`\x04aC\xBBV[a\"\xBFV[4\x80\x15a\x06GW`\0\x80\xFD[P`\0Ta\x04\xDF\x90`\xFF\x16\x81V[`\0a\x06\x87`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nslashValidators`\x88\x1B\x81RPa%jV[`\0[\x82Q\x81\x10\x15a\x07\xC6W`\0\x83\x82\x81Q\x81\x10a\x06\xA7Wa\x06\xA7aI\x13V[` \x02` \x01\x01Q\x90P`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06\xCDWa\x06\xCDaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\x07\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x02\x90aI)V[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\xFF\x16a\x07bW\x80Ta\xFF\0\x19\x16a\x01\0\x17\x81Ua\x07.a\x12dV[a\x079\x90`\x01aI\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[`\0a\x07m\x83a%\xFAV[\x90P`\x01`\x01`@\x1B\x03\x81\x16`\n\x11\x15a\x07\x96Wa\x07\x8B\x81\x86aI\xB8V[\x94P`\0\x90Pa\x07\xB1V[a\x07\xA1`\n\x86aI\xB8V[\x94Pa\x07\xAE`\n\x82aI\xD7V[\x90P[a\x07\xBB\x83\x82a&\x05V[PPP`\x01\x01a\x06\x8AV[P\x91\x90PV[```\r\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06W[PPPPP\x90P\x90V[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x08JWa\x08JaI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x90P\x91\x90PV[a\x08kaAFV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x81T``\x94\x81\x02\x82\x01\x85\x01\x84R\x92\x81\x01\x83\x81R\x90\x93\x91\x92\x84\x92\x84\x91\x90\x84\x01\x82\x82\x80\x15a\x08\xE4W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xD0W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x08\xFD\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t)\x90aI\xF6V[\x80\x15a\tvW\x80`\x1F\x10a\tKWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tvV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tYW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q``\x81\x01\x82R`!T`\x01`\x01`@\x1B\x03\x16\x80\x82R`\0\x90\x81R`\"` \x90\x81R\x90\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x84R`\x01\x81\x01\x80T\x96\x97P\x92\x95\x82\x87\x01\x95P\x90\x92\x91\x84\x01\x91\x90a\t\xD4\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\0\x90aI\xF6V[\x80\x15a\nMW\x80`\x1F\x10a\n\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\nMV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n0W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81R` \x93\x84\x01Q\x81\x85\x01R\x92\x01\x91\x90\x91R\x92\x91PPV[`\0[`\x1DT\x81\x10\x15a\x0B\x1AW`\0`\x1D\x82\x81T\x81\x10a\n\x9FWa\n\x9FaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\n\xC2WPa\x0B\x12V[`\0a\n\xCD\x83a%\xFAV[\x90Pd\x07sY@\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\n\xEDWPd\x07sY@\0[`\x03\x91\x90\x91\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01\x01a\n\x80V[Pa\x0BY`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F- updated effective balances\0\0\0\0\x81RPa&nV[a\x0B\x9B`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xA5\xA4\x0Cn\xAENL\xAD\xCE\x84\x0C\xAE\r\xECm`\x83\x1B\x81RPa\x0B\x8Da\x12dV[`\x01`\x01`@\x1B\x03\x16a&\x9DV[`\0a\x0B\xA5a\x12dV[`\x1BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xE5\xD6\xBF\x02a\x0B\xC2\x83a&\xDAV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x03W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x17W=`\0\x80>=`\0\xFD[PP`!\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16B`\x01`\x01`@\x1B\x03\x16\x17\x90UPP`@\x80Q\x80\x82\x01\x90\x91R`\x16\x81Ru\x05\xA4\rN\xAD\xAE\x0C\xAC\x84\x0E\x8D\xE4\r\xCC\xAF\x0E\x84\x0C\xAE\r\xECm`S\x1B` \x82\x01Ra\x0Cp\x90a\x0B\x8Da\x12dV[a\x0C\xAE`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F- building beacon state trees\0\0\0\x81RPa&nV[`\x1DT\x15a\x0C\xCDW`\x1DTa\x0C\xC5\x90`\x01\x90aJ*V[` Ua\r\x87V[`!T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p`$\x82\x01Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rKW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r_W=`\0\x80>=`\0\xFD[PPPPa\r\x84`@Q\x80``\x01`@R\x80`(\x81R` \x01aN\xD2`(\x919a&nV[PV[`\0a\r\xBFa\r\x94a'\x1AV[a\r\xA0`(`\x01aJ=V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 a'\xA9V[\x90P`\0a\r\xFCa\r\xCEa*\x16V[a\r\xDA`&`\x01aJ=V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x02\x01a'\xA9V[\x90P`\0a\x0E2a\x0E\r\x84\x84a*\xAFV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x05\x90`\x04\x01a'\xA9V[\x90P`\0a\x0Ega\x0EB\x83a+]V[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` R`@\x90 `\x03\x90`\x06\x01a'\xA9V[\x90Pa\x0E\x9F`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x0BKH\x18\x99XX\xDB\xDB\x88\x18\x9B\x1B\xD8\xDA\xC8\x1C\x9B\xDB\xDD`b\x1B\x81RP\x82a+\xE9V[`!T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xFEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F\x12W=`\0\x80>=`\0\xFD[PPPPa\x0F\x1F\x82a,\"V[a\x0F(\x83a-=V[a\x0F0a.\xEAV[a\x0F8a0\xD4V[PPPPPV[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x0F[Wa\x0F[aI\x13V[`\0\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[a\x0F\xAB`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0C,\x8E\xCC-\xCCl\xA8\xAE\r\xECm`\xA3\x1B\x81RPa%jV[a\x0F\xB3a2.V[a\x0F\xBBa2\xF9V[a\x0F\xC3a\n}V[V[```\x0F\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[```\x0E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[a\x0F\xB3`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uadvanceEpoch_NoRewards`P\x1B\x81RPa%jV[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R`\0\x91\x90` \x82\x01\x81\x806\x837PPP`0\x81\x01\x93\x90\x93RP\x90\x91\x90PV[```\x12\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x11\xBAW` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x11|W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x11\x10V[PPPP\x90P\x90V[`\0\x80`\0[\x83Q\x81\x10\x15a\x12]Wc;\x9A\xCA\0`\x1D\x85\x83\x81Q\x81\x10a\x12\x03Wa\x12\x03aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x12\"Wa\x12\"aI\x13V[`\0\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01Ta\x12I\x91\x90`\x01`\x01`@\x1B\x03\x16aJPV[a\x12S\x90\x83aJ=V[\x91P`\x01\x01a\x11\xE1V[P\x92\x91PPV[`\x1BT`\0\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x12\xEFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChain.currentEpoch: curren`D\x82\x01R\x7Ft time is before genesis time\0\0\0`d\x82\x01R`\x84\x01a\x07\x02V[a\x12\xFB`\x0C` aJgV[`\x1BT`\x01`\x01`@\x1B\x03\x91\x82\x16\x91a\x13\x1C\x91`\x01`\xA0\x1B\x90\x04\x16BaJ*V[a\x13&\x91\x90aJ\x9FV[\x90P\x90V[```\x11\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x13n\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x13\x9A\x90aI\xF6V[\x80\x15a\x13\xE7W\x80`\x1F\x10a\x13\xBCWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x13\xE7V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x13\xCAW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x13OV[`\0`\x1E\x81a\x14\x0B`\x04\x85aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 T\x92\x91PPV[```\x13\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x14\xF7W` \x02\x82\x01\x91\x90`\0R` `\0 \x90`\0\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x14\xB9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x14MV[a\x15\x17aA\xA8V[`\0[\x82Q\x81\x10\x15a\x16\x0FW` T\x83\x82\x81Q\x81\x10a\x158Wa\x158aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\x07W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F validator has not been included`d\x82\x01R\x7F in beacon chain state (DID YOU `\x84\x82\x01R\x7FCALL advanceEpoch YET?)\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x07\x02V[`\x01\x01a\x15\x1AV[P`@\x80Q`\x80\x81\x01\x82R`!T`\x01`\x01`@\x1B\x03\x16\x80\x82R`\0\x90\x81R`\"` \x90\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01\x80T\x93\x95\x83\x86\x01\x94\x90\x93\x84\x01\x91\x90a\x16_\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\x8B\x90aI\xF6V[\x80\x15a\x16\xD8W\x80`\x1F\x10a\x16\xADWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xD8V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xBBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\xFFWa\x16\xFFaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x172W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17\x1DW\x90P[P\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17QWa\x17QaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x17\x84W\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x17oW\x90P[P\x90R\x90P`\0[\x83Q\x81\x10\x15a\x12]W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` R`@\x81 \x85Q\x82\x90\x87\x90\x85\x90\x81\x10a\x17\xC5Wa\x17\xC5aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 `@Q\x80`@\x01`@R\x90\x81`\0\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x18CW` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x18/W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x18\\\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18\x88\x90aI\xF6V[\x80\x15a\x18\xD5W\x80`\x1F\x10a\x18\xAAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\xD5V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18\xB8W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q\x83`@\x01Q\x83\x81Q\x81\x10a\x18\xFBWa\x18\xFBaI\x13V[` \x02` \x01\x01\x81\x90RP\x80`\0\x01Q\x83``\x01Q\x83\x81Q\x81\x10a\x19!Wa\x19!aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x17\x8CV[`\0`\x01`\x01`@\x1B\x03\x80\x16`\x1D\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x19[Wa\x19[aI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x14\x92\x91PPV[a\x19\xB1`@\x80Q`\x80\x81\x01\x82R`\0\x91\x81\x01\x91\x82R``\x80\x82\x01R\x90\x81\x90\x81R` \x01``\x81RP\x90V[`\0[\x83Q\x81\x10\x15a\x1A\x83W` T\x84\x82\x81Q\x81\x10a\x19\xD2Wa\x19\xD2aI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1A{W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F no checkpoint proof found (did `d\x82\x01R\x7Fyou call advanceEpoch yet?)\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\x02V[`\x01\x01a\x19\xB4V[P`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16`\0\x90\x81R`#` R\x82\x81 `\x80\x83\x01\x84R\x80T\x93\x83\x01\x93\x84R`\x01\x81\x01\x80T\x92\x94\x84\x93\x90\x92\x91``\x85\x01\x91a\x1A\xC8\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\xF4\x90aI\xF6V[\x80\x15a\x1BAW\x80`\x1F\x10a\x1B\x16Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1BAV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1B$W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1BhWa\x1BhaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1B\xB5W\x81` \x01[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x83\x01R\x91\x81\x01\x91\x90\x91R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1B\x86W\x90P[P\x90R\x90P`\0[\x84Q\x81\x10\x15a\x1D>W`\0\x85\x82\x81Q\x81\x10a\x1B\xDAWa\x1B\xDAaI\x13V[` \x02` \x01\x01Q\x90P`\0a\x1B\xEF\x82a4\xE5V[`\x01`\x01`@\x1B\x03\x87\x16`\0\x90\x81R`%` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01\x80T\x95\x96P\x93\x94\x91\x93\x90\x92\x84\x01\x91a\x1C@\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1Cl\x90aI\xF6V[\x80\x15a\x1C\xB9W\x80`\x1F\x10a\x1C\x8EWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\xB9V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1C\x9CW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80``\x01`@R\x80`\x1D\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xE9Wa\x1C\xE9aI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x82`\0\x01Q\x81R` \x01\x82` \x01Q\x81RP\x85` \x01Q\x85\x81Q\x81\x10a\x1D(Wa\x1D(aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x1B\xBDV[P\x90P[\x92\x91PPV[```\x10\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01`\0\x90[\x82\x82\x10\x15a\x11\xD2W\x83\x82\x90`\0R` `\0 \x01\x80Ta\x1D\x8B\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D\xB7\x90aI\xF6V[\x80\x15a\x1E\x04W\x80`\x1F\x10a\x1D\xD9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1E\x04V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\xE7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1DlV[`\0\x80Ta\x01\0\x90\x04`\xFF\x16\x15a\x1E8WP`\0Ta\x01\0\x90\x04`\xFF\x16\x90V[`\0sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-;\x15a\x1F>W`@\x80Qsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-` \x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B\x82\x84\x01R\x82Q\x80\x83\x03\x84\x01\x81R``\x83\x01\x90\x93R`\0\x92\x90\x91a\x1E\xC6\x91\x7Ff\x7F\x9Dp\xCAA\x1Dp\xEA\xD5\r\x8D\\\"\x07\r\xAF\xC3j\xD7_=\xCF^r7\xB2*\xDE\x9A\xEC\xC4\x91`\x80\x01aJ\xDDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xE0\x91aK\x0EV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x1F\x1DW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1F\"V[``\x91P[P\x91PP\x80\x80` \x01\x90Q\x81\x01\x90a\x1F:\x91\x90aK*V[\x91PP[\x91\x90PV[```\0\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1F`Wa\x1F`aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1F\x89W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83Q\x81\x10\x15a\x12]W`\x1D\x84\x82\x81Q\x81\x10a\x1F\xACWa\x1F\xACaI\x13V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1F\xCBWa\x1F\xCBaI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x01\x01T\x82\x82\x81Q\x81\x10a\x1F\xEFWa\x1F\xEFaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1F\x8FV[```\x0C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08$W` \x02\x82\x01\x91\x90`\0R` `\0 \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08\x06WPPPPP\x90P\x90V[`\0a \x91`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k72\xBB\xAB0\xB64\xB20\xBA7\xB9`\xA1\x1B\x81RPa%jV[4g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a!\x06W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rrposit value too low`h\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a!\x14c;\x9A\xCA\0\x82aKLV[\x15a!\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FBeaconChainMock.newValidator: va`D\x82\x01R\x7Flue not multiple of gwei\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\x02V[`\0a!\x97c;\x9A\xCA\0\x83aJ\x9FV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rs\x0E\r\xEEm.\x84\x0E\xCC-\x8E\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`c\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a\"\x17\x84\x82a4\xF2V[\x94\x93PPPPV[a\"]`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FadvanceEpoch_NoWithdraw\0\0\0\0\0\0\0\0\0\x81RPa%jV[a\x0F\xBBa2.V[`\0a\x1DBa\"s\x83a\x13\xFBV[\x83a8\x9FV[`\0`\x1D\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\"\x95Wa\"\x95aI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[`\0a\"\xEF`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l2\xBC4\xBA+0\xB64\xB20\xBA7\xB9`\x99\x1B\x81RPa%jV[`\0`\x1D\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a#\x0BWa#\x0BaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a#@W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\x02\x90aI)V[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a#\xB6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBeaconChainMock: validator alrea`D\x82\x01Rh\x19\x1EH\x19^\x1A]\x19Y`\xBA\x1B`d\x82\x01R`\x84\x01a\x07\x02V[a#\xBEa\x12dV[a#\xC9\x90`\x01aI\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa#\xFA\x83a%\xFAV[\x91Pa$\x07\x83`\0a&\x05V[`\0a$\xC5`\x1D\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$&Wa$&aI\x13V[\x90`\0R` `\0 \x90`\x04\x02\x01`\x02\x01\x80Ta$B\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta$n\x90aI\xF6V[\x80\x15a$\xBBW\x80`\x1F\x10a$\x90Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a$\xBBV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a$\x9EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa9*V[`\x1BT\x90\x91P`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x8A^m\x82a$\xF1c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x88\x16aJPV[a%\x05\x90`\x01`\x01`\xA0\x1B\x03\x86\x161aJ=V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a%KW`\0\x80\xFD[PZ\xF1\x15\x80\x15a%_W=`\0\x80>=`\0\xFD[PPPPPP\x91\x90PV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPa%\xBBa%\xB6`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01R\x90V[a9FV[a%\xC4\x83a9oV[`@Q` \x01a%\xD5\x92\x91\x90aK`V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra%\xEF\x91aD\xF8V[`@Q\x80\x91\x03\x90\xA1PV[`\0a\x1DB\x82a\"eV[`\0`\x1E\x81a&\x15`\x04\x86aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 T\x90Pa&?\x81\x84\x84a9\x97V[\x90P\x80`\x1E`\0a&Q`\x04\x87aJ\xB3V[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 UPPPV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP\x81`@Qa%\xEF\x91\x90aD\xF8V[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa&\xCE\x92\x91\x90aK\x9CV[`@Q\x80\x91\x03\x90\xA1PPV[`\0a&\xE8`\x0C` aJgV[a&\xF3\x83`\x01aI\xB8V[a&\xFD\x91\x90aJgV[`\x1BTa\x1DB\x91\x90`\x01`\xA0\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aI\xB8V[`\x1DT``\x90`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a':Wa':aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'cW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[`\x1DT\x81\x10\x15a\x07\xC6Wa'\x84a'\x7F\x82a:\x0BV[a<\x92V[\x82\x82\x81Q\x81\x10a'\x96Wa'\x96aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a'iV[`\0\x80[\x83\x81\x10\x15a)\x81W`\0`\x02\x86Q`\x01a'\xC7\x91\x90aJ=V[a'\xD1\x91\x90aJ\x9FV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xEDWa'\xEDaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a(\x16W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a)uW`\0a(1\x82`\x02aJPV[\x90P`\0a(@\x82`\x01aJ=V[\x90P`\0\x8A\x83\x81Q\x81\x10a(VWa(VaI\x13V[` \x02` \x01\x01Q\x90P`\0\x8BQ\x83\x10\x15a(\x8CW\x8B\x83\x81Q\x81\x10a(}Wa(}aI\x13V[` \x02` \x01\x01Q\x90Pa(\x98V[a(\x95\x88a?+V[\x90P[`\0`\x02\x83\x83`@Q` \x01a(\xB8\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra(\xD2\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a(\xEFW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\x12\x91\x90aK\xBEV[\x90P\x80\x87\x87\x81Q\x81\x10a)'Wa)'aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R`\0\x84\x81R\x8C\x82R`@\x80\x82 \x85\x90U\x84\x82R\x80\x82 \x86\x90U\x94\x81R`\x01\x80\x8E\x01\x90\x92R\x84\x81 \x83\x90U\x92\x83R\x92\x90\x91 U\x92\x90\x92\x01\x91Pa(\x1C\x90PV[P\x95PP`\x01\x01a'\xADV[P\x83Q`\x01\x14a)\xF2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBeaconChainMock._buildMerkleTree`D\x82\x01Ru: invalid tree somehow`P\x1B`d\x82\x01R`\x84\x01a\x07\x02V[\x83`\0\x81Q\x81\x10a*\x05Wa*\x05aI\x13V[` \x02` \x01\x01Q\x90P\x93\x92PPPV[```\0a*\"a?\xA2V[`\x01`\x01`@\x1B\x03\x81\x11\x15a*9Wa*9aBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a*bW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x81Q\x81\x10\x15a\x07\xC6Wd\xFF\xFF\xFF\xFF\xFF\x81\x16`\0\x90\x81R`\x1E` R`@\x90 T\x82Q\x83\x90\x83\x90\x81\x10a*\x9CWa*\x9CaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*hV[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R``\x91`\0\x91\x90\x80\x82\x01a\x04\0\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81\x10\x15a+\x16Wa*\xEE\x81`\x01aJ=V[`\0\x1B\x82\x82\x81Q\x81\x10a+\x03Wa+\x03aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*\xDAV[P\x83\x81`\x0B\x81Q\x81\x10a++Wa++aI\x13V[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0C\x81Q\x81\x10a+KWa+KaI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x93\x92PPPV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P`\0[\x81Q\x81\x10\x15a+\xC3Wa+\x9B\x81`\x01aJ=V[`\0\x1B\x82\x82\x81Q\x81\x10a+\xB0Wa+\xB0aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a+\x87V[P\x82\x81`\x03\x81Q\x81\x10a+\xD8Wa+\xD8aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x82a,\x14\x83a?\xDAV[`@Qa&\xCE\x92\x91\x90aK\xD7V[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a,\\Wa,\\aBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a,\x86W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81`\0\x80[`\x03\x81\x10\x15a,\xEEW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x89\x01\x84\x01\x81\x90R\x96\x84R`\x07\x01\x90\x91R\x90 T\x92\x82a,\xE2\x81aL\x05V[\x93PPP`\x01\x01a,\x8EV[P`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x81R`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`\"\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a-4\x90\x82aLeV[PPPPPPPV[`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a-wWa-waBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a-\xA1W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81`\0a-\xD2` \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aJ\x9FV[\x90P`\0\x80[`\x05\x81\x10\x15a.8W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x87\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x05\x01\x90\x91R\x90 T\x93\x82a.,\x81aL\x05V[\x93PPP`\x01\x01a-\xD8V[P\x80[\x82\x81\x10\x15a.\x9AW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x87\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x07\x01\x90\x91R\x90 T\x93\x82a.\x8E\x81aL\x05V[\x93PPP`\x01\x01a.;V[P`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`#\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a.\xE0\x90\x82aLeV[PPPPPPPPV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`$` R`@\x81 \x90[`\x1DT\x81\x10\x15a0\xD0W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a/JWa/JaBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/tW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P`\0a/\x82\x83a:\x0BV[\x90P`\0a/\x8F\x82a<\x92V[\x90P`\0\x80[a/\xA1`(`\x01aJ=V[\x81\x10\x15a/\xFCW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R\x80\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x01\x01\x90\x91R\x90 T\x92\x82a/\xF0\x81aL\x05V[\x93PPP`\x01\x01a/\x95V[P\x80[`\x05a0\r`(`\x01aJ=V[a0\x17\x91\x90aJ=V[\x81\x10\x15a0uW`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x05\x01\x90\x91R\x90 T\x92\x82a0i\x81aL\x05V[\x93PPP`\x01\x01a/\xFFV[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x87\x81R`@\x90\x91 \x84Qa0\x9B\x92\x86\x01\x90aA\xF7V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x87\x90R`@\x90 `\x01\x01a0\xBF\x85\x82aLeV[PP`\x01\x90\x93\x01\x92Pa/\x06\x91PPV[PPV[`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`%` R`@\x81 \x90a0\xF7a?\xA2V[\x90P`\0[\x81\x81\x10\x15a2)W`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a1>Wa1>aBWV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1hW` \x82\x01\x81\x806\x837\x01\x90P[Pd\xFF\xFF\xFF\xFF\xFF\x83\x16`\0\x90\x81R`\x1E` R`@\x81 T\x91\x92P\x81\x90\x80[a1\x93`&`\x01aJ=V[\x81\x10\x15a1\xF1W`!T`\x01`\x01`@\x1B\x03\x16`\0\x90\x81R`'` \x90\x81R`@\x80\x83 \x86\x84R`\x02\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x03\x01\x90\x91R\x90 T\x92\x82a1\xE5\x81aL\x05V[\x93PPP`\x01\x01a1\x87V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16`\0\x90\x81R` \x88\x90R`@\x90 \x83\x81U`\x01\x01a2\x18\x85\x82aLeV[PP`\x01\x90\x93\x01\x92Pa0\xFC\x91PPV[PPPV[`\0\x80[`\x1DT\x81\x10\x15a2\xD6W`\0`\x1D\x82\x81T\x81\x10a2QWa2QaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a2tWPa2\xCEV[`\x03\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x01a2\xCCW`\0a2\xA4\x83a%\xFAV[\x90Pa2\xB1`\x01\x82aI\xB8V[\x90P\x83a2\xBD\x81aL\x05V[\x94PPa2\xCA\x83\x82a&\x05V[P[P[`\x01\x01a22V[Pa\r\x84`@Q\x80``\x01`@R\x80`&\x81R` \x01aN\xFA`&\x919\x82a&\x9DV[`\0\x80[`\x1DT\x81\x10\x15a4\x9FW`\0`\x1D\x82\x81T\x81\x10a3\x1CWa3\x1CaI\x13V[`\0\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a3?WPa4\x97V[`\0c;\x9A\xCA\0a3O\x84a%\xFAV[`\x01`\x01`@\x1B\x03\x16a3b\x91\x90aJPV[\x90P`\0a3x\x83`\x02\x01\x80Ta$B\x90aI\xF6V[\x90P`\0\x80a3\x8Bc;\x9A\xCA\0\x85aJ\x9FV[`\x03\x86\x01T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a3\xC7W\x83`\0\x03a3\xBCWPPPPPa4\x97V[P\x82\x90P`\0a3\xF6V[h\x01\xBC\x16\xD6t\xEC\x80\0\0\x84\x11\x15a3\xF6Wa3\xEBh\x01\xBC\x16\xD6t\xEC\x80\0\0\x85aJ*V[\x91Pd\x07sY@\0\x90P[`\x1BT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90c\xC8\x8A^m\x90\x85\x90a4\x1B\x90\x86\x90\x83\x161aJ=V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a4aW`\0\x80\xFD[PZ\xF1\x15\x80\x15a4uW=`\0\x80>=`\0\xFD[PPPP\x81\x87a4\x85\x91\x90aJ=V[\x96Pa4\x91\x86\x82a&\x05V[PPPPP[`\x01\x01a2\xFDV[P\x80\x15a\r\x84Wa\r\x84`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- withdrew excess balance\0\0\0\0\0\0\0\x81RP\x82a&\x9DV[`\0a\x1DB`\x04\x83aJ\xB3V[`\x1DT`\0\x90a5\x03`\x04\x82aM#V[d\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a6\xECW`\x1DT`\0\x90a5(\x90`\x01`\x01`@\x1B\x03aI\xD7V[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R\x91\x92P`\0\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82\x81`0\x01R`\x1D`@Q\x80`\xE0\x01`@R\x80`\x01\x15\x15\x81R` \x01`\0\x15\x15\x81R` \x01`\x02\x84`\0`\x80\x1B`@Q` \x01a5\x8C\x92\x91\x90aMMV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra5\xA6\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a5\xC3W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a5\xE6\x91\x90aK\xBEV[\x81R`@\x80Q` \x80\x82\x01\x83R`\0\x80\x83R\x81\x85\x01\x92\x90\x92R`\x01`\x01`@\x1B\x03\x88\x81\x16\x84\x86\x01R``\x80\x86\x01\x82\x90R`\x80\x90\x95\x01R\x85T`\x01\x80\x82\x01\x88U\x96\x83R\x91\x81\x90 \x85Q`\x04\x90\x93\x02\x01\x80T\x91\x86\x01Qa\xFF\xFF\x19\x90\x92\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x81U\x90\x83\x01Q\x93\x81\x01\x93\x90\x93U\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a6z\x90\x82aLeV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua6\xDC\x83\x83a&\x05V[\x82a6\xE6\x81aM|V[\x93PPPP[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R`\0\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81\x81`0\x01R`\x1D`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x15\x15\x81R` \x01`\x02\x84`\0`\x80\x1B`@Q` \x01a7L\x92\x91\x90aMMV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra7f\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a7\x83W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xA6\x91\x90aK\xBEV[\x81R` \x01\x87\x81R` \x01\x86`\x01`\x01`@\x1B\x03\x16\x81R` \x01a7\xC8a\x12dV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x91\x82\x01R\x82T`\x01\x81\x81\x01\x85U`\0\x94\x85R\x93\x82\x90 \x83Q`\x04\x90\x92\x02\x01\x80T\x92\x84\x01Q\x15\x15a\x01\0\x02a\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16a\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x81U`@\x82\x01Q\x92\x81\x01\x92\x90\x92U``\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a8=\x90\x82aLeV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua\x1D>\x82\x85a&\x05V[`\0\x80a8\xAD`\x04\x84aM#V[a8\xB8\x90`@aM\xA3V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\"\x17\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17`\xFF`8\x1B`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[`\0\x80a96\x83aM\xC3V[`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[``a\x1DB`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83a@^V[``a\x1DB`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83a@^V[`\0\x80a9\xA5`\x04\x85aM#V[a9\xB0\x90`\x01aM\xE7V[a9\xBB\x90`@aM\xA3V[a9\xC7\x90a\x01\0aN\x04V[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x01`@\x1B\x03\x81\x1B\x19\x85\x81\x16`\0a9\xE9\x86a@\xA8V[\x90P`\0a9\xF8\x85`\xC0aJ*V[\x91\x90\x91\x1C\x91\x90\x91\x17\x97\x96PPPPPPPV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91`\0\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P`\0`\x1D\x84d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a:PWa:PaI\x13V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x04\x90\x93\x02\x90\x91\x01\x80T`\xFF\x80\x82\x16\x15\x15\x85Ra\x01\0\x90\x91\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01\x83\x01T\x90\x82\x01R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a:\xA9\x90aI\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta:\xD5\x90aI\xF6V[\x80\x15a;\"W\x80`\x1F\x10a:\xF7Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a;\"V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a;\x05W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x82\x16` \x84\x01R`\x01`@\x1B\x82\x04\x81\x16`@\x80\x85\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x92\x04\x16``\x90\x92\x01\x91\x90\x91R\x81\x01Q\x83Q\x91\x92P\x90\x83\x90`\0\x90a;\x7FWa;\x7FaI\x13V[` \x02` \x01\x01\x81\x81RPP\x80``\x01Qa;\x99\x90aM\xC3V[\x82`\x01\x81Q\x81\x10a;\xACWa;\xACaI\x13V[` \x02` \x01\x01\x81\x81RPPa;\xC5\x81`\x80\x01Qa@\xA8V[\x82`\x02\x81Q\x81\x10a;\xD8Wa;\xD8aI\x13V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q`@Q` \x01a;\xFD\x91\x15\x15\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra<\x15\x90aM\xC3V[\x82`\x03\x81Q\x81\x10a<(Wa<(aI\x13V[` \x02` \x01\x01\x81\x81RPPa<A\x81`\xA0\x01Qa@\xA8V[\x82`\x05\x81Q\x81\x10a<TWa<TaI\x13V[` \x02` \x01\x01\x81\x81RPPa<m\x81`\xC0\x01Qa@\xA8V[\x82`\x06\x81Q\x81\x10a<\x80Wa<\x80aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x92\x91PPV[`\0\x80`\x02\x83Qa<\xA3\x91\x90aJ\x9FV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a<\xBFWa<\xBFaBWV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a<\xE8W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a=\xE5W`\x02\x85a=\x03\x83\x83aJPV[\x81Q\x81\x10a=\x13Wa=\x13aI\x13V[` \x02` \x01\x01Q\x86\x83`\x02a=)\x91\x90aJPV[a=4\x90`\x01aJ=V[\x81Q\x81\x10a=DWa=DaI\x13V[` \x02` \x01\x01Q`@Q` \x01a=f\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra=\x80\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a=\x9DW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a=\xC0\x91\x90aK\xBEV[\x82\x82\x81Q\x81\x10a=\xD2Wa=\xD2aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a<\xEEV[Pa=\xF1`\x02\x83aJ\x9FV[\x91P[\x81\x15a?\x07W`\0[\x82\x81\x10\x15a>\xF4W`\x02\x82a>\x12\x83\x83aJPV[\x81Q\x81\x10a>\"Wa>\"aI\x13V[` \x02` \x01\x01Q\x83\x83`\x02a>8\x91\x90aJPV[a>C\x90`\x01aJ=V[\x81Q\x81\x10a>SWa>SaI\x13V[` \x02` \x01\x01Q`@Q` \x01a>u\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra>\x8F\x91aK\x0EV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a>\xACW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a>\xCF\x91\x90aK\xBEV[\x82\x82\x81Q\x81\x10a>\xE1Wa>\xE1aI\x13V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a=\xFDV[Pa?\0`\x02\x83aJ\x9FV[\x91Pa=\xF4V[\x80`\0\x81Q\x81\x10a?\x1AWa?\x1AaI\x13V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0`d\x82\x10a?}W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7F_getZeroNode: invalid depth\0\0\0\0\0`D\x82\x01R`d\x01a\x07\x02V[`&\x82\x81T\x81\x10a?\x90Wa?\x90aI\x13V[\x90`\0R` `\0 \x01T\x90P\x91\x90PV[`\x1DT`\0\x90\x15a?\xD4W`\x1DT`\x04\x90a?\xBF\x90`\x01\x90aJ*V[a?\xC9\x91\x90aJ\x9FV[a\x13&\x90`\x01aJ=V[P`\0\x90V[`@Qc\x16#C=`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R``\x90a\x1DB\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a@1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra@Y\x91\x90\x81\x01\x90aN!V[aA\x1EV[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01a@\x91\x93\x92\x91\x90aN\x8EV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`8\x81\x81\x1B`\xFF`8\x1B\x16`(\x83\x81\x1Bf\xFF\0\0\0\0\0\0\x16\x91\x90\x91\x17`\x18\x84\x81\x1Be\xFF\0\0\0\0\0\x16\x91\x90\x91\x17`\x08\x85\x81\x1Bd\xFF\0\0\0\0\x16\x91\x90\x91\x17c\xFF\0\0\0\x91\x86\x90\x1C\x91\x90\x91\x16\x17b\xFF\0\0\x91\x85\x90\x1C\x91\x90\x91\x16\x17a\xFF\0\x91\x84\x90\x1C\x91\x90\x91\x16\x17`\xFF\x92\x90\x91\x1C\x91\x90\x91\x16\x17`\xC0\x1B\x90V[``a\x1DB`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[2m`\xE0\x1B\x81RP\x83a@^V[`@Q\x80``\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01aA\x81`@Q\x80`@\x01`@R\x80`\0\x80\x19\x16\x81R` \x01``\x81RP\x90V[\x81R` \x01aA\xA3`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80`\x80\x01`@R\x80`\0`\x01`\x01`@\x1B\x03\x16\x81R` \x01aA\xE3`@Q\x80`@\x01`@R\x80`\0\x80\x19\x16\x81R` \x01``\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15aB2W\x91` \x02\x82\x01[\x82\x81\x11\x15aB2W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aB\x17V[PaB>\x92\x91PaBBV[P\x90V[[\x80\x82\x11\x15aB>W`\0\x81U`\x01\x01aBCV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aB\x95WaB\x95aBWV[`@R\x91\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x1F>W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12aB\xC3W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB\xDCWaB\xDCaBWV[\x80`\x05\x1BaB\xEC` \x82\x01aBmV[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15aC\x08W`\0\x80\xFD[` \x86\x01\x92P[\x83\x83\x10\x15aC1WaC \x83aB\x9DV[\x82R` \x92\x83\x01\x92\x90\x91\x01\x90aC\x0FV[\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15aCMW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aCcW`\0\x80\xFD[a\"\x17\x84\x82\x85\x01aB\xB2V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xB0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aC\x89V[P\x90\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15aC\xCDW`\0\x80\xFD[aC\xD6\x82aB\x9DV[\x93\x92PPPV[`\0[\x83\x81\x10\x15aC\xF8W\x81\x81\x01Q\x83\x82\x01R` \x01aC\xE0V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84RaD\x19\x81` \x86\x01` \x86\x01aC\xDDV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x80Q\x82R`\0` \x82\x01Q`@` \x85\x01Ra\"\x17`@\x85\x01\x82aD\x01V[`\0\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01`\0[\x82\x81\x10\x15aD~W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aD`V[P\x93\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q```@\x84\x01RaD\xB7`\x80\x84\x01\x82aD-V[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01R\x80Q`@\x83RaD\xDD`@\x84\x01\x82aDLV[\x90P` \x82\x01Q\x91P\x82\x81\x03` \x84\x01RaC1\x81\x83aD\x01V[` \x81R`\0aC\xD6` \x83\x01\x84aD\x01V[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15aE\xB9W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x82\x88\x01\x81\x90R\x81Q\x90\x88\x01\x81\x90R\x91\x01\x90`\0\x90``\x88\x01\x90[\x80\x83\x10\x15aE\xA1W\x83Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x82R` \x93\x84\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x91\x01\x90aEuV[P\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE3V[P\x92\x96\x95PPPPPPV[`\0` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01`\0[\x82\x81\x10\x15aE\xB9W`?\x19\x87\x86\x03\x01\x84RaF\t\x85\x83QaD\x01V[\x94P` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aE\xEDV[`\0\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01`\0[\x83\x81\x10\x15aFnW`\x1F\x19\x85\x84\x03\x01\x88RaFX\x83\x83QaDLV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aF<V[P\x90\x96\x95PPPPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R`\0` \x83\x01Q`\x80`@\x84\x01RaF\xA9`\xA0\x84\x01\x82aD-V[`@\x85\x01Q\x84\x82\x03`\x1F\x19\x01``\x86\x01R\x80Q\x80\x83R\x91\x92P` \x90\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01`\0[\x82\x81\x10\x15aG\x08W`\x1F\x19\x86\x83\x03\x01\x84RaF\xF3\x82\x86QaD\x01V[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01aF\xD7V[P``\x88\x01Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01R\x94PaG&\x81\x86aF\x1EV[\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15aGEW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aG[W`\0\x80\xFD[aGg\x85\x82\x86\x01aB\xB2V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aG\x84W`\0\x80\xFD[\x80\x91PP\x92P\x92\x90PV[` \x81R`\0\x82Q`@` \x84\x01RaG\xAB``\x84\x01\x82aD-V[` \x85\x81\x01Q\x85\x83\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x84R\x92\x93P\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01`\0[\x82\x81\x10\x15aH)W`\x1F\x19\x86\x83\x03\x01\x84R\x84Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q\x90P```@\x84\x01RaH\x13``\x84\x01\x82aD\x01V[` \x96\x87\x01\x96\x95\x90\x95\x01\x94\x92PP`\x01\x01aG\xD7V[P\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aC\xB0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aHOV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aH\x86WaH\x86aBWV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0` \x82\x84\x03\x12\x15aH\xA6W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aH\xBCW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aH\xCDW`\0\x80\xFD[\x805aH\xE0aH\xDB\x82aHmV[aBmV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aH\xF5W`\0\x80\xFD[\x81` \x84\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[` \x80\x82R`S\x90\x82\x01R\x7FBeaconChainMock: attempting to e`@\x82\x01R\x7Fxit dummy validator. We need tho``\x82\x01Rr\x0El\xA4\x0C\xCD\xEED\x0E\x0EM\xED\xEC\xCC\xEC\xAD\xC4\x07\xC7E`k\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\x01\x81\x81\x1C\x90\x82\x16\x80aJ\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\xC6WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[\x80\x82\x01\x80\x82\x11\x15a\x1DBWa\x1DBaI\xA2V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1DBWa\x1DBaI\xA2V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x12]Wa\x12]aI\xA2V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aJ\xAEWaJ\xAEaJ\x89V[P\x04\x90V[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aJ\xCAWaJ\xCAaJ\x89V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\x01`\x01`\xE0\x1B\x03\x19\x83\x16\x81R\x81Q`\0\x90aK\0\x81`\x04\x85\x01` \x87\x01aC\xDDV[\x91\x90\x91\x01`\x04\x01\x93\x92PPPV[`\0\x82QaK \x81\x84` \x87\x01aC\xDDV[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aK<W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14aC\xD6W`\0\x80\xFD[`\0\x82aK[WaK[aJ\x89V[P\x06\x90V[`\0\x83QaKr\x81\x84` \x88\x01aC\xDDV[`\x17`\xF9\x1B\x90\x83\x01\x90\x81R\x83QaK\x90\x81`\x01\x84\x01` \x88\x01aC\xDDV[\x01`\x01\x01\x94\x93PPPPV[`@\x81R`\0aK\xAF`@\x83\x01\x85aD\x01V[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aK\xD0W`\0\x80\xFD[PQ\x91\x90PV[`@\x81R`\0aK\xEA`@\x83\x01\x85aD\x01V[\x82\x81\x03` \x84\x01RaK\xFC\x81\x85aD\x01V[\x95\x94PPPPPV[`\0`\x01\x82\x01aL\x17WaL\x17aI\xA2V[P`\x01\x01\x90V[`\x1F\x82\x11\x15a2)W\x80`\0R` `\0 `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aLEWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F8W`\0\x81U`\x01\x01aLQV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aL~WaL~aBWV[aL\x92\x81aL\x8C\x84TaI\xF6V[\x84aL\x1EV[` `\x1F\x82\x11`\x01\x81\x14aL\xC6W`\0\x83\x15aL\xAEWP\x84\x82\x01Q[`\0\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0F8V[`\0\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aL\xF6W\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aL\xD6V[P\x84\x82\x10\x15aM\x14W\x86\x84\x01Q`\0\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aM:WaM:aJ\x89V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[`\0\x83QaM_\x81\x84` \x88\x01aC\xDDV[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0d\xFF\xFF\xFF\xFF\xFF\x82\x16d\xFF\xFF\xFF\xFF\xFF\x81\x03aM\x9AWaM\x9AaI\xA2V[`\x01\x01\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x12]Wa\x12]aI\xA2V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x07\xC6W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[d\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1DBWa\x1DBaI\xA2V[`\0` \x82\x84\x03\x12\x15aN3W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aNIW`\0\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13aNZW`\0\x80\xFD[\x80QaNhaH\xDB\x82aHmV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aN}W`\0\x80\xFD[aK\xFC\x82` \x83\x01` \x86\x01aC\xDDV[`\0\x84QaN\xA0\x81\x84` \x89\x01aC\xDDV[\x84Q\x90\x83\x01\x90aN\xB4\x81\x83` \x89\x01aC\xDDV[\x84Q\x91\x01\x90aN\xC7\x81\x83` \x88\x01aC\xDDV[\x01\x95\x94PPPPPV\xFE-- no validators; added empty block root- generated rewards for num validators\xA2dipfsX\"\x12 \xC6\xC7\xBCQ:\x93\x97\xBD\xE6\xC2:)a!\t\xD5\x9A*,ac\xEC\x81\xEEn,\xE4Kh\x87\xCE\x95dsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct CheckpointProofs { BeaconChainProofs.BalanceContainerProof balanceContainerProof; BeaconChainProofs.BalanceProof[] balanceProofs; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointProofs {
        pub balanceContainerProof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
        pub balanceProofs: alloy::sol_types::private::Vec<
            <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
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
            BeaconChainProofs::BalanceContainerProof,
            alloy::sol_types::sol_data::Array<BeaconChainProofs::BalanceProof>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<
                <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<CheckpointProofs> for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointProofs) -> Self {
                (value.balanceContainerProof, value.balanceProofs)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckpointProofs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    balanceContainerProof: tuple.0,
                    balanceProofs: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CheckpointProofs {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CheckpointProofs {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolType>::tokenize(
                        &self.balanceContainerProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::BalanceProof,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceProofs),
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
        impl alloy_sol_types::SolType for CheckpointProofs {
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
        impl alloy_sol_types::SolStruct for CheckpointProofs {
            const NAME: &'static str = "CheckpointProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckpointProofs(BeaconChainProofs.BalanceContainerProof balanceContainerProof,BeaconChainProofs.BalanceProof[] balanceProofs)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BeaconChainProofs::BalanceProof as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeaconChainProofs::BalanceProof as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolType>::eip712_data_word(
                            &self.balanceContainerProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::BalanceProof,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.balanceProofs)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CheckpointProofs {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceContainerProof,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::BalanceProof,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceProofs,
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
                <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceContainerProof,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BeaconChainProofs::BalanceProof,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceProofs,
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
struct CredentialProofs { uint64 beaconTimestamp; BeaconChainProofs.StateRootProof stateRootProof; bytes[] validatorFieldsProofs; bytes32[][] validatorFields; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CredentialProofs {
        pub beaconTimestamp: u64,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Bytes,
        >,
        pub validatorFields: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            alloy::sol_types::sol_data::Uint<64>,
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
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
        impl ::core::convert::From<CredentialProofs> for UnderlyingRustTuple<'_> {
            fn from(value: CredentialProofs) -> Self {
                (
                    value.beaconTimestamp,
                    value.stateRootProof,
                    value.validatorFieldsProofs,
                    value.validatorFields,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CredentialProofs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconTimestamp: tuple.0,
                    stateRootProof: tuple.1,
                    validatorFieldsProofs: tuple.2,
                    validatorFields: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CredentialProofs {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CredentialProofs {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconTimestamp),
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
        impl alloy_sol_types::SolType for CredentialProofs {
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
        impl alloy_sol_types::SolStruct for CredentialProofs {
            const NAME: &'static str = "CredentialProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CredentialProofs(uint64 beaconTimestamp,BeaconChainProofs.StateRootProof stateRootProof,bytes[] validatorFieldsProofs,bytes32[][] validatorFields)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <BeaconChainProofs::StateRootProof as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeaconChainProofs::StateRootProof as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconTimestamp,
                        )
                        .0,
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stateRootProof,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorFieldsProofs,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorFields,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CredentialProofs {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconTimestamp,
                    )
                    + <BeaconChainProofs::StateRootProof as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stateRootProof,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Bytes,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorFieldsProofs,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::FixedBytes<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorFields,
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
                    &rust.beaconTimestamp,
                    out,
                );
                <BeaconChainProofs::StateRootProof as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stateRootProof,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Bytes,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorFieldsProofs,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    >,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorFields,
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
struct StaleBalanceProofs { uint64 beaconTimestamp; BeaconChainProofs.StateRootProof stateRootProof; BeaconChainProofs.ValidatorProof validatorProof; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleBalanceProofs {
        pub beaconTimestamp: u64,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorProof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
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
            BeaconChainProofs::StateRootProof,
            BeaconChainProofs::ValidatorProof,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u64,
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<StaleBalanceProofs> for UnderlyingRustTuple<'_> {
            fn from(value: StaleBalanceProofs) -> Self {
                (value.beaconTimestamp, value.stateRootProof, value.validatorProof)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StaleBalanceProofs {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconTimestamp: tuple.0,
                    stateRootProof: tuple.1,
                    validatorProof: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StaleBalanceProofs {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StaleBalanceProofs {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconTimestamp),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolType>::tokenize(
                        &self.validatorProof,
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
        impl alloy_sol_types::SolType for StaleBalanceProofs {
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
        impl alloy_sol_types::SolStruct for StaleBalanceProofs {
            const NAME: &'static str = "StaleBalanceProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StaleBalanceProofs(uint64 beaconTimestamp,BeaconChainProofs.StateRootProof stateRootProof,BeaconChainProofs.ValidatorProof validatorProof)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(2);
                components
                    .push(
                        <BeaconChainProofs::StateRootProof as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeaconChainProofs::StateRootProof as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
                    .push(
                        <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconTimestamp,
                        )
                        .0,
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::eip712_data_word(
                            &self.stateRootProof,
                        )
                        .0,
                    <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolType>::eip712_data_word(
                            &self.validatorProof,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StaleBalanceProofs {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconTimestamp,
                    )
                    + <BeaconChainProofs::StateRootProof as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.stateRootProof,
                    )
                    + <BeaconChainProofs::ValidatorProof as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.validatorProof,
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
                    &rust.beaconTimestamp,
                    out,
                );
                <BeaconChainProofs::StateRootProof as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stateRootProof,
                    out,
                );
                <BeaconChainProofs::ValidatorProof as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorProof,
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
constructor(address _eigenPodManager, uint64 _genesisTime);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _eigenPodManager: alloy::sol_types::private::Address,
        pub _genesisTime: u64,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, u64);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value._eigenPodManager, value._genesisTime)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _eigenPodManager: tuple.0,
                        _genesisTime: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self._genesisTime),
                )
            }
        }
    };
    /**Function with signature `CONSENSUS_REWARD_AMOUNT_GWEI()` and selector `0xe3cefb42`.
```solidity
function CONSENSUS_REWARD_AMOUNT_GWEI() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CONSENSUS_REWARD_AMOUNT_GWEICall {}
    ///Container type for the return parameters of the [`CONSENSUS_REWARD_AMOUNT_GWEI()`](CONSENSUS_REWARD_AMOUNT_GWEICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CONSENSUS_REWARD_AMOUNT_GWEIReturn {
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
            impl ::core::convert::From<CONSENSUS_REWARD_AMOUNT_GWEICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: CONSENSUS_REWARD_AMOUNT_GWEICall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CONSENSUS_REWARD_AMOUNT_GWEICall {
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
            impl ::core::convert::From<CONSENSUS_REWARD_AMOUNT_GWEIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: CONSENSUS_REWARD_AMOUNT_GWEIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for CONSENSUS_REWARD_AMOUNT_GWEIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CONSENSUS_REWARD_AMOUNT_GWEICall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = CONSENSUS_REWARD_AMOUNT_GWEIReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CONSENSUS_REWARD_AMOUNT_GWEI()";
            const SELECTOR: [u8; 4] = [227u8, 206u8, 251u8, 66u8];
            #[inline]
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
    /**Function with signature `NAME()` and selector `0xa3f4df7e`.
```solidity
function NAME() external pure returns (string memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NAMECall {}
    ///Container type for the return parameters of the [`NAME()`](NAMECall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NAMEReturn {
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
            impl ::core::convert::From<NAMECall> for UnderlyingRustTuple<'_> {
                fn from(value: NAMECall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NAMECall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::String,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::String,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<NAMEReturn> for UnderlyingRustTuple<'_> {
                fn from(value: NAMEReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for NAMEReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for NAMECall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = NAMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NAME()";
            const SELECTOR: [u8; 4] = [163u8, 244u8, 223u8, 126u8];
            #[inline]
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
    /**Function with signature `SLASH_AMOUNT_GWEI()` and selector `0x330bc27e`.
```solidity
function SLASH_AMOUNT_GWEI() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASH_AMOUNT_GWEICall {}
    ///Container type for the return parameters of the [`SLASH_AMOUNT_GWEI()`](SLASH_AMOUNT_GWEICall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SLASH_AMOUNT_GWEIReturn {
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
            impl ::core::convert::From<SLASH_AMOUNT_GWEICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_GWEICall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASH_AMOUNT_GWEICall {
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
            impl ::core::convert::From<SLASH_AMOUNT_GWEIReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_GWEIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for SLASH_AMOUNT_GWEIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASH_AMOUNT_GWEICall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASH_AMOUNT_GWEIReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SLASH_AMOUNT_GWEI()";
            const SELECTOR: [u8; 4] = [51u8, 11u8, 194u8, 126u8];
            #[inline]
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
    /**Function with signature `_advanceEpoch()` and selector `0x29992faa`.
```solidity
function _advanceEpoch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _advanceEpochCall {}
    ///Container type for the return parameters of the [`_advanceEpoch()`](_advanceEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct _advanceEpochReturn {}
    #[allow(
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
            impl ::core::convert::From<_advanceEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: _advanceEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _advanceEpochCall {
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
            impl ::core::convert::From<_advanceEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: _advanceEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for _advanceEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for _advanceEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = _advanceEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "_advanceEpoch()";
            const SELECTOR: [u8; 4] = [41u8, 153u8, 47u8, 170u8];
            #[inline]
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
    /**Function with signature `advanceEpoch()` and selector `0x3cf80e6c`.
```solidity
function advanceEpoch() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpochCall {}
    ///Container type for the return parameters of the [`advanceEpoch()`](advanceEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpochReturn {}
    #[allow(
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
            impl ::core::convert::From<advanceEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpochCall {
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
            impl ::core::convert::From<advanceEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpochReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for advanceEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "advanceEpoch()";
            const SELECTOR: [u8; 4] = [60u8, 248u8, 14u8, 108u8];
            #[inline]
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
    /**Function with signature `advanceEpoch_NoRewards()` and selector `0x59d095dd`.
```solidity
function advanceEpoch_NoRewards() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpoch_NoRewardsCall {}
    ///Container type for the return parameters of the [`advanceEpoch_NoRewards()`](advanceEpoch_NoRewardsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpoch_NoRewardsReturn {}
    #[allow(
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
            impl ::core::convert::From<advanceEpoch_NoRewardsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoRewardsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for advanceEpoch_NoRewardsCall {
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
            impl ::core::convert::From<advanceEpoch_NoRewardsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoRewardsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for advanceEpoch_NoRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for advanceEpoch_NoRewardsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpoch_NoRewardsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "advanceEpoch_NoRewards()";
            const SELECTOR: [u8; 4] = [89u8, 208u8, 149u8, 221u8];
            #[inline]
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
    /**Function with signature `advanceEpoch_NoWithdraw()` and selector `0xf0acd988`.
```solidity
function advanceEpoch_NoWithdraw() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpoch_NoWithdrawCall {}
    ///Container type for the return parameters of the [`advanceEpoch_NoWithdraw()`](advanceEpoch_NoWithdrawCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct advanceEpoch_NoWithdrawReturn {}
    #[allow(
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
            impl ::core::convert::From<advanceEpoch_NoWithdrawCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoWithdrawCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for advanceEpoch_NoWithdrawCall {
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
            impl ::core::convert::From<advanceEpoch_NoWithdrawReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoWithdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for advanceEpoch_NoWithdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for advanceEpoch_NoWithdrawCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpoch_NoWithdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "advanceEpoch_NoWithdraw()";
            const SELECTOR: [u8; 4] = [240u8, 172u8, 217u8, 136u8];
            #[inline]
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
    /**Function with signature `balanceOfGwei(uint40)` and selector `0x2def6009`.
```solidity
function balanceOfGwei(uint40 validatorIndex) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfGweiCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`balanceOfGwei(uint40)`](balanceOfGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct balanceOfGweiReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<balanceOfGweiCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfGweiCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<balanceOfGweiReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfGweiCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOfGwei(uint40)";
            const SELECTOR: [u8; 4] = [45u8, 239u8, 96u8, 9u8];
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
    /**Function with signature `currentBalance(uint40)` and selector `0xf7213873`.
```solidity
function currentBalance(uint40 validatorIndex) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentBalanceCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`currentBalance(uint40)`](currentBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentBalanceReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<currentBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentBalanceCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<currentBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentBalanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentBalance(uint40)";
            const SELECTOR: [u8; 4] = [247u8, 33u8, 56u8, 115u8];
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
    /**Function with signature `currentEpoch()` and selector `0x76671808`.
```solidity
function currentEpoch() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochCall {}
    ///Container type for the return parameters of the [`currentEpoch()`](currentEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentEpochReturn {
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
            impl ::core::convert::From<currentEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochCall {
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
            impl ::core::convert::From<currentEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentEpochCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentEpoch()";
            const SELECTOR: [u8; 4] = [118u8, 103u8, 24u8, 8u8];
            #[inline]
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
    /**Function with signature `effectiveBalance(uint40)` and selector `0x86a6f9e1`.
```solidity
function effectiveBalance(uint40 validatorIndex) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct effectiveBalanceCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`effectiveBalance(uint40)`](effectiveBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct effectiveBalanceReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<effectiveBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: effectiveBalanceCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for effectiveBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<effectiveBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: effectiveBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for effectiveBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for effectiveBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = effectiveBalanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "effectiveBalance(uint40)";
            const SELECTOR: [u8; 4] = [134u8, 166u8, 249u8, 225u8];
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
    /**Function with signature `exitEpoch(uint40)` and selector `0xf833eb63`.
```solidity
function exitEpoch(uint40 validatorIndex) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitEpochCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`exitEpoch(uint40)`](exitEpochCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitEpochReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<exitEpochCall> for UnderlyingRustTuple<'_> {
                fn from(value: exitEpochCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitEpochCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<exitEpochReturn> for UnderlyingRustTuple<'_> {
                fn from(value: exitEpochReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitEpochReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitEpochCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exitEpoch(uint40)";
            const SELECTOR: [u8; 4] = [248u8, 51u8, 235u8, 99u8];
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
    /**Function with signature `exitValidator(uint40)` and selector `0xf8f98a4e`.
```solidity
function exitValidator(uint40 validatorIndex) external returns (uint64 exitedBalanceGwei);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitValidatorCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`exitValidator(uint40)`](exitValidatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct exitValidatorReturn {
        pub exitedBalanceGwei: u64,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<exitValidatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<exitValidatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorReturn) -> Self {
                    (value.exitedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { exitedBalanceGwei: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitValidatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitValidatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exitValidator(uint40)";
            const SELECTOR: [u8; 4] = [248u8, 249u8, 138u8, 78u8];
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
    /**Function with signature `getBalanceRoot(uint40)` and selector `0x908820e0`.
```solidity
function getBalanceRoot(uint40 validatorIndex) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceRootCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`getBalanceRoot(uint40)`](getBalanceRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceRootReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getBalanceRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceRootCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<getBalanceRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalanceRoot(uint40)";
            const SELECTOR: [u8; 4] = [144u8, 136u8, 32u8, 224u8];
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
    /**Function with signature `getCheckpointProofs(uint40[],uint64)` and selector `0xb1b6f6a1`.
```solidity
function getCheckpointProofs(uint40[] memory _validators, uint64 timestamp) external view returns (CheckpointProofs memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckpointProofsCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
        pub timestamp: u64,
    }
    ///Container type for the return parameters of the [`getCheckpointProofs(uint40[],uint64)`](getCheckpointProofsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckpointProofsReturn {
        pub _0: <CheckpointProofs as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<getCheckpointProofsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckpointProofsCall) -> Self {
                    (value._validators, value.timestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckpointProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
                        timestamp: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (CheckpointProofs,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <CheckpointProofs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCheckpointProofsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCheckpointProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCheckpointProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCheckpointProofsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckpointProofsReturn;
            type ReturnTuple<'a> = (CheckpointProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCheckpointProofs(uint40[],uint64)";
            const SELECTOR: [u8; 4] = [177u8, 182u8, 246u8, 161u8];
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
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
                    <alloy::sol_types::sol_data::Uint<
                        64,
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
    /**Function with signature `getCredentialProofs(uint40[])` and selector `0xa50a3a1a`.
```solidity
function getCredentialProofs(uint40[] memory _validators) external view returns (CredentialProofs memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCredentialProofsCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`getCredentialProofs(uint40[])`](getCredentialProofsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCredentialProofsReturn {
        pub _0: <CredentialProofs as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<getCredentialProofsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCredentialProofsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCredentialProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _validators: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (CredentialProofs,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <CredentialProofs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCredentialProofsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCredentialProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCredentialProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCredentialProofsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCredentialProofsReturn;
            type ReturnTuple<'a> = (CredentialProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCredentialProofs(uint40[])";
            const SELECTOR: [u8; 4] = [165u8, 10u8, 58u8, 26u8];
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
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
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
    /**Function with signature `getPubkeyHashes(uint40[])` and selector `0xc76f25c0`.
```solidity
function getPubkeyHashes(uint40[] memory _validators) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPubkeyHashesCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`getPubkeyHashes(uint40[])`](getPubkeyHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPubkeyHashesReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<getPubkeyHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPubkeyHashesCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPubkeyHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _validators: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getPubkeyHashesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getPubkeyHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getPubkeyHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPubkeyHashesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPubkeyHashesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getPubkeyHashes(uint40[])";
            const SELECTOR: [u8; 4] = [199u8, 111u8, 37u8, 192u8];
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
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
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
    /**Function with signature `getStaleBalanceProofs(uint40)` and selector `0x23e82c4c`.
```solidity
function getStaleBalanceProofs(uint40 validatorIndex) external view returns (StaleBalanceProofs memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStaleBalanceProofsCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`getStaleBalanceProofs(uint40)`](getStaleBalanceProofsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStaleBalanceProofsReturn {
        pub _0: <StaleBalanceProofs as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<getStaleBalanceProofsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStaleBalanceProofsCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStaleBalanceProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (StaleBalanceProofs,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <StaleBalanceProofs as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStaleBalanceProofsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStaleBalanceProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStaleBalanceProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStaleBalanceProofsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStaleBalanceProofsReturn;
            type ReturnTuple<'a> = (StaleBalanceProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStaleBalanceProofs(uint40)";
            const SELECTOR: [u8; 4] = [35u8, 232u8, 44u8, 76u8];
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
    /**Function with signature `isActive(uint40)` and selector `0xaa47389c`.
```solidity
function isActive(uint40 validatorIndex) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isActiveCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`isActive(uint40)`](isActiveCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isActiveReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<isActiveCall> for UnderlyingRustTuple<'_> {
                fn from(value: isActiveCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isActiveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<isActiveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isActiveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isActiveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isActiveCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isActiveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isActive(uint40)";
            const SELECTOR: [u8; 4] = [170u8, 71u8, 56u8, 156u8];
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
    /**Function with signature `newValidator(bytes)` and selector `0xed3c1605`.
```solidity
function newValidator(bytes memory withdrawalCreds) external payable returns (uint40);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newValidatorCall {
        pub withdrawalCreds: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`newValidator(bytes)`](newValidatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct newValidatorReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<newValidatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: newValidatorCall) -> Self {
                    (value.withdrawalCreds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newValidatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { withdrawalCreds: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<newValidatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: newValidatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for newValidatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for newValidatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = newValidatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "newValidator(bytes)";
            const SELECTOR: [u8; 4] = [237u8, 60u8, 22u8, 5u8];
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
                        &self.withdrawalCreds,
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
    /**Function with signature `nextTimestamp()` and selector `0x357e951f`.
```solidity
function nextTimestamp() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTimestampCall {}
    ///Container type for the return parameters of the [`nextTimestamp()`](nextTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct nextTimestampReturn {
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
            impl ::core::convert::From<nextTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: nextTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTimestampCall {
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
            impl ::core::convert::From<nextTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: nextTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for nextTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for nextTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "nextTimestamp()";
            const SELECTOR: [u8; 4] = [53u8, 126u8, 149u8, 31u8];
            #[inline]
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
    /**Function with signature `pubkey(uint40)` and selector `0x5e6cc2fc`.
```solidity
function pubkey(uint40 validatorIndex) external pure returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`pubkey(uint40)`](pubkeyCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<pubkeyCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
                }
            }
        }
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
            impl ::core::convert::From<pubkeyReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkey(uint40)";
            const SELECTOR: [u8; 4] = [94u8, 108u8, 194u8, 252u8];
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
    /**Function with signature `pubkeyHash(uint40)` and selector `0x1f54365c`.
```solidity
function pubkeyHash(uint40 validatorIndex) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyHashCall {
        pub validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
    }
    ///Container type for the return parameters of the [`pubkeyHash(uint40)`](pubkeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct pubkeyHashReturn {
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<pubkeyHashCall> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndex: tuple.0 }
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
            impl ::core::convert::From<pubkeyHashReturn> for UnderlyingRustTuple<'_> {
                fn from(value: pubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for pubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for pubkeyHashCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "pubkeyHash(uint40)";
            const SELECTOR: [u8; 4] = [31u8, 84u8, 54u8, 92u8];
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
    /**Function with signature `slashValidators(uint40[])` and selector `0x14360958`.
```solidity
function slashValidators(uint40[] memory _validators) external returns (uint64 slashedBalanceGwei);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashValidatorsCall {
        pub _validators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`slashValidators(uint40[])`](slashValidatorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashValidatorsReturn {
        pub slashedBalanceGwei: u64,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<slashValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashValidatorsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _validators: tuple.0 }
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
            impl ::core::convert::From<slashValidatorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashValidatorsReturn) -> Self {
                    (value.slashedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        slashedBalanceGwei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashValidatorsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashValidatorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashValidators(uint40[])";
            const SELECTOR: [u8; 4] = [20u8, 54u8, 9u8, 88u8];
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
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self._validators),
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
    /**Function with signature `totalEffectiveBalanceWei(uint40[])` and selector `0x6b3abd97`.
```solidity
function totalEffectiveBalanceWei(uint40[] memory validatorIndices) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalEffectiveBalanceWeiCall {
        pub validatorIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U40,
        >,
    }
    ///Container type for the return parameters of the [`totalEffectiveBalanceWei(uint40[])`](totalEffectiveBalanceWeiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct totalEffectiveBalanceWeiReturn {
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U40,
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
            impl ::core::convert::From<totalEffectiveBalanceWeiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalEffectiveBalanceWeiCall) -> Self {
                    (value.validatorIndices,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalEffectiveBalanceWeiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { validatorIndices: tuple.0 }
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
            impl ::core::convert::From<totalEffectiveBalanceWeiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: totalEffectiveBalanceWeiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for totalEffectiveBalanceWeiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalEffectiveBalanceWeiCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalEffectiveBalanceWeiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "totalEffectiveBalanceWei(uint40[])";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 189u8, 151u8];
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
                        alloy::sol_types::sol_data::Uint<40>,
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorIndices),
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
    ///Container for all the [`BeaconChainMock`](self) function calls.
    pub enum BeaconChainMockCalls {
        CONSENSUS_REWARD_AMOUNT_GWEI(CONSENSUS_REWARD_AMOUNT_GWEICall),
        IS_TEST(IS_TESTCall),
        NAME(NAMECall),
        SLASH_AMOUNT_GWEI(SLASH_AMOUNT_GWEICall),
        _advanceEpoch(_advanceEpochCall),
        advanceEpoch(advanceEpochCall),
        advanceEpoch_NoRewards(advanceEpoch_NoRewardsCall),
        advanceEpoch_NoWithdraw(advanceEpoch_NoWithdrawCall),
        balanceOfGwei(balanceOfGweiCall),
        currentBalance(currentBalanceCall),
        currentEpoch(currentEpochCall),
        effectiveBalance(effectiveBalanceCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSenders(excludeSendersCall),
        exitEpoch(exitEpochCall),
        exitValidator(exitValidatorCall),
        failed(failedCall),
        getBalanceRoot(getBalanceRootCall),
        getCheckpointProofs(getCheckpointProofsCall),
        getCredentialProofs(getCredentialProofsCall),
        getPubkeyHashes(getPubkeyHashesCall),
        getStaleBalanceProofs(getStaleBalanceProofsCall),
        isActive(isActiveCall),
        newValidator(newValidatorCall),
        nextTimestamp(nextTimestampCall),
        pubkey(pubkeyCall),
        pubkeyHash(pubkeyHashCall),
        slashValidators(slashValidatorsCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
        totalEffectiveBalanceWei(totalEffectiveBalanceWeiCall),
    }
    #[automatically_derived]
    impl BeaconChainMockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [20u8, 54u8, 9u8, 88u8],
            [30u8, 215u8, 131u8, 28u8],
            [31u8, 84u8, 54u8, 92u8],
            [35u8, 232u8, 44u8, 76u8],
            [41u8, 153u8, 47u8, 170u8],
            [45u8, 239u8, 96u8, 9u8],
            [51u8, 11u8, 194u8, 126u8],
            [53u8, 126u8, 149u8, 31u8],
            [60u8, 248u8, 14u8, 108u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [89u8, 208u8, 149u8, 221u8],
            [94u8, 108u8, 194u8, 252u8],
            [102u8, 217u8, 169u8, 160u8],
            [107u8, 58u8, 189u8, 151u8],
            [118u8, 103u8, 24u8, 8u8],
            [133u8, 34u8, 108u8, 129u8],
            [134u8, 166u8, 249u8, 225u8],
            [144u8, 136u8, 32u8, 224u8],
            [145u8, 106u8, 23u8, 198u8],
            [163u8, 244u8, 223u8, 126u8],
            [165u8, 10u8, 58u8, 26u8],
            [170u8, 71u8, 56u8, 156u8],
            [177u8, 182u8, 246u8, 161u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [199u8, 111u8, 37u8, 192u8],
            [226u8, 12u8, 159u8, 113u8],
            [227u8, 206u8, 251u8, 66u8],
            [237u8, 60u8, 22u8, 5u8],
            [240u8, 172u8, 217u8, 136u8],
            [247u8, 33u8, 56u8, 115u8],
            [248u8, 51u8, 235u8, 99u8],
            [248u8, 249u8, 138u8, 78u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for BeaconChainMockCalls {
        const NAME: &'static str = "BeaconChainMockCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CONSENSUS_REWARD_AMOUNT_GWEI(_) => {
                    <CONSENSUS_REWARD_AMOUNT_GWEICall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::NAME(_) => <NAMECall as alloy_sol_types::SolCall>::SELECTOR,
                Self::SLASH_AMOUNT_GWEI(_) => {
                    <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::_advanceEpoch(_) => {
                    <_advanceEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::advanceEpoch(_) => {
                    <advanceEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::advanceEpoch_NoRewards(_) => {
                    <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::advanceEpoch_NoWithdraw(_) => {
                    <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::balanceOfGwei(_) => {
                    <balanceOfGweiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentBalance(_) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentEpoch(_) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::effectiveBalance(_) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exitEpoch(_) => {
                    <exitEpochCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exitValidator(_) => {
                    <exitValidatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBalanceRoot(_) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCheckpointProofs(_) => {
                    <getCheckpointProofsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCredentialProofs(_) => {
                    <getCredentialProofsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getPubkeyHashes(_) => {
                    <getPubkeyHashesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStaleBalanceProofs(_) => {
                    <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isActive(_) => <isActiveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::newValidator(_) => {
                    <newValidatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::nextTimestamp(_) => {
                    <nextTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::pubkey(_) => <pubkeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pubkeyHash(_) => {
                    <pubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashValidators(_) => {
                    <slashValidatorsCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::totalEffectiveBalanceWei(_) => {
                    <totalEffectiveBalanceWeiCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<BeaconChainMockCalls>] = &[
                {
                    fn slashValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <slashValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::slashValidators)
                    }
                    slashValidators
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn pubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <pubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::pubkeyHash)
                    }
                    pubkeyHash
                },
                {
                    fn getStaleBalanceProofs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::getStaleBalanceProofs)
                    }
                    getStaleBalanceProofs
                },
                {
                    fn _advanceEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <_advanceEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::_advanceEpoch)
                    }
                    _advanceEpoch
                },
                {
                    fn balanceOfGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::balanceOfGwei)
                    }
                    balanceOfGwei
                },
                {
                    fn SLASH_AMOUNT_GWEI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::SLASH_AMOUNT_GWEI)
                    }
                    SLASH_AMOUNT_GWEI
                },
                {
                    fn nextTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <nextTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::nextTimestamp)
                    }
                    nextTimestamp
                },
                {
                    fn advanceEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <advanceEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::advanceEpoch)
                    }
                    advanceEpoch
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn advanceEpoch_NoRewards(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::advanceEpoch_NoRewards)
                    }
                    advanceEpoch_NoRewards
                },
                {
                    fn pubkey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <pubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::pubkey)
                    }
                    pubkey
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn totalEffectiveBalanceWei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <totalEffectiveBalanceWeiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::totalEffectiveBalanceWei)
                    }
                    totalEffectiveBalanceWei
                },
                {
                    fn currentEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <currentEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::currentEpoch)
                    }
                    currentEpoch
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn effectiveBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <effectiveBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::effectiveBalance)
                    }
                    effectiveBalance
                },
                {
                    fn getBalanceRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getBalanceRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::getBalanceRoot)
                    }
                    getBalanceRoot
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn NAME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::NAME)
                    }
                    NAME
                },
                {
                    fn getCredentialProofs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getCredentialProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::getCredentialProofs)
                    }
                    getCredentialProofs
                },
                {
                    fn isActive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <isActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::isActive)
                    }
                    isActive
                },
                {
                    fn getCheckpointProofs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::getCheckpointProofs)
                    }
                    getCheckpointProofs
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::failed)
                    }
                    failed
                },
                {
                    fn getPubkeyHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getPubkeyHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::getPubkeyHashes)
                    }
                    getPubkeyHashes
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn CONSENSUS_REWARD_AMOUNT_GWEI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <CONSENSUS_REWARD_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::CONSENSUS_REWARD_AMOUNT_GWEI)
                    }
                    CONSENSUS_REWARD_AMOUNT_GWEI
                },
                {
                    fn newValidator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <newValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::newValidator)
                    }
                    newValidator
                },
                {
                    fn advanceEpoch_NoWithdraw(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::advanceEpoch_NoWithdraw)
                    }
                    advanceEpoch_NoWithdraw
                },
                {
                    fn currentBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <currentBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::currentBalance)
                    }
                    currentBalance
                },
                {
                    fn exitEpoch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <exitEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::exitEpoch)
                    }
                    exitEpoch
                },
                {
                    fn exitValidator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <exitValidatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::exitValidator)
                    }
                    exitValidator
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(BeaconChainMockCalls::IS_TEST)
                    }
                    IS_TEST
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
                Self::CONSENSUS_REWARD_AMOUNT_GWEI(inner) => {
                    <CONSENSUS_REWARD_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::SLASH_AMOUNT_GWEI(inner) => {
                    <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::_advanceEpoch(inner) => {
                    <_advanceEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::advanceEpoch(inner) => {
                    <advanceEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::advanceEpoch_NoRewards(inner) => {
                    <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::advanceEpoch_NoWithdraw(inner) => {
                    <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::balanceOfGwei(inner) => {
                    <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentBalance(inner) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::effectiveBalance(inner) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
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
                Self::exitEpoch(inner) => {
                    <exitEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exitValidator(inner) => {
                    <exitValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCheckpointProofs(inner) => {
                    <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCredentialProofs(inner) => {
                    <getCredentialProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getPubkeyHashes(inner) => {
                    <getPubkeyHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStaleBalanceProofs(inner) => {
                    <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isActive(inner) => {
                    <isActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::newValidator(inner) => {
                    <newValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::nextTimestamp(inner) => {
                    <nextTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::pubkey(inner) => {
                    <pubkeyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pubkeyHash(inner) => {
                    <pubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slashValidators(inner) => {
                    <slashValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::totalEffectiveBalanceWei(inner) => {
                    <totalEffectiveBalanceWeiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::CONSENSUS_REWARD_AMOUNT_GWEI(inner) => {
                    <CONSENSUS_REWARD_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::SLASH_AMOUNT_GWEI(inner) => {
                    <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::_advanceEpoch(inner) => {
                    <_advanceEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::advanceEpoch(inner) => {
                    <advanceEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::advanceEpoch_NoRewards(inner) => {
                    <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::advanceEpoch_NoWithdraw(inner) => {
                    <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::balanceOfGwei(inner) => {
                    <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentBalance(inner) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::effectiveBalance(inner) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
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
                Self::exitEpoch(inner) => {
                    <exitEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::exitValidator(inner) => {
                    <exitValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCheckpointProofs(inner) => {
                    <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCredentialProofs(inner) => {
                    <getCredentialProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getPubkeyHashes(inner) => {
                    <getPubkeyHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStaleBalanceProofs(inner) => {
                    <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isActive(inner) => {
                    <isActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::newValidator(inner) => {
                    <newValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::nextTimestamp(inner) => {
                    <nextTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::pubkey(inner) => {
                    <pubkeyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pubkeyHash(inner) => {
                    <pubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashValidators(inner) => {
                    <slashValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::totalEffectiveBalanceWei(inner) => {
                    <totalEffectiveBalanceWeiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`BeaconChainMock`](self) events.
    pub enum BeaconChainMockEvents {
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
    impl BeaconChainMockEvents {
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
    impl alloy_sol_types::SolEventInterface for BeaconChainMockEvents {
        const NAME: &'static str = "BeaconChainMockEvents";
        const COUNT: usize = 22usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
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
    impl alloy_sol_types::private::IntoLogData for BeaconChainMockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
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
    /**Creates a new wrapper around an on-chain [`BeaconChainMock`](self) contract instance.

See the [wrapper's documentation](`BeaconChainMockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BeaconChainMockInstance<T, P, N> {
        BeaconChainMockInstance::<T, P, N>::new(address, provider)
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
        _eigenPodManager: alloy::sol_types::private::Address,
        _genesisTime: u64,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<BeaconChainMockInstance<T, P, N>>,
    > {
        BeaconChainMockInstance::<
            T,
            P,
            N,
        >::deploy(provider, _eigenPodManager, _genesisTime)
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
        _eigenPodManager: alloy::sol_types::private::Address,
        _genesisTime: u64,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        BeaconChainMockInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _eigenPodManager, _genesisTime)
    }
    /**A [`BeaconChainMock`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`BeaconChainMock`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BeaconChainMockInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BeaconChainMockInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BeaconChainMockInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > BeaconChainMockInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`BeaconChainMock`](self) contract instance.

See the [wrapper's documentation](`BeaconChainMockInstance`) for more details.*/
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
            _eigenPodManager: alloy::sol_types::private::Address,
            _genesisTime: u64,
        ) -> alloy_contract::Result<BeaconChainMockInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _eigenPodManager,
                _genesisTime,
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
            _eigenPodManager: alloy::sol_types::private::Address,
            _genesisTime: u64,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _eigenPodManager,
                            _genesisTime,
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
    impl<T, P: ::core::clone::Clone, N> BeaconChainMockInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BeaconChainMockInstance<T, P, N> {
            BeaconChainMockInstance {
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
    > BeaconChainMockInstance<T, P, N> {
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
        ///Creates a new call builder for the [`CONSENSUS_REWARD_AMOUNT_GWEI`] function.
        pub fn CONSENSUS_REWARD_AMOUNT_GWEI(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CONSENSUS_REWARD_AMOUNT_GWEICall, N> {
            self.call_builder(
                &CONSENSUS_REWARD_AMOUNT_GWEICall {
                },
            )
        }
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
        }
        ///Creates a new call builder for the [`NAME`] function.
        pub fn NAME(&self) -> alloy_contract::SolCallBuilder<T, &P, NAMECall, N> {
            self.call_builder(&NAMECall {})
        }
        ///Creates a new call builder for the [`SLASH_AMOUNT_GWEI`] function.
        pub fn SLASH_AMOUNT_GWEI(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, SLASH_AMOUNT_GWEICall, N> {
            self.call_builder(&SLASH_AMOUNT_GWEICall {})
        }
        ///Creates a new call builder for the [`_advanceEpoch`] function.
        pub fn _advanceEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, _advanceEpochCall, N> {
            self.call_builder(&_advanceEpochCall {})
        }
        ///Creates a new call builder for the [`advanceEpoch`] function.
        pub fn advanceEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, advanceEpochCall, N> {
            self.call_builder(&advanceEpochCall {})
        }
        ///Creates a new call builder for the [`advanceEpoch_NoRewards`] function.
        pub fn advanceEpoch_NoRewards(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, advanceEpoch_NoRewardsCall, N> {
            self.call_builder(&advanceEpoch_NoRewardsCall {})
        }
        ///Creates a new call builder for the [`advanceEpoch_NoWithdraw`] function.
        pub fn advanceEpoch_NoWithdraw(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, advanceEpoch_NoWithdrawCall, N> {
            self.call_builder(&advanceEpoch_NoWithdrawCall {})
        }
        ///Creates a new call builder for the [`balanceOfGwei`] function.
        pub fn balanceOfGwei(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfGweiCall, N> {
            self.call_builder(
                &balanceOfGweiCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`currentBalance`] function.
        pub fn currentBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentBalanceCall, N> {
            self.call_builder(
                &currentBalanceCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`currentEpoch`] function.
        pub fn currentEpoch(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentEpochCall, N> {
            self.call_builder(&currentEpochCall {})
        }
        ///Creates a new call builder for the [`effectiveBalance`] function.
        pub fn effectiveBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, effectiveBalanceCall, N> {
            self.call_builder(
                &effectiveBalanceCall {
                    validatorIndex,
                },
            )
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
        ///Creates a new call builder for the [`exitEpoch`] function.
        pub fn exitEpoch(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, exitEpochCall, N> {
            self.call_builder(&exitEpochCall { validatorIndex })
        }
        ///Creates a new call builder for the [`exitValidator`] function.
        pub fn exitValidator(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, exitValidatorCall, N> {
            self.call_builder(
                &exitValidatorCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getBalanceRoot`] function.
        pub fn getBalanceRoot(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceRootCall, N> {
            self.call_builder(
                &getBalanceRootCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`getCheckpointProofs`] function.
        pub fn getCheckpointProofs(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            timestamp: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckpointProofsCall, N> {
            self.call_builder(
                &getCheckpointProofsCall {
                    _validators,
                    timestamp,
                },
            )
        }
        ///Creates a new call builder for the [`getCredentialProofs`] function.
        pub fn getCredentialProofs(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCredentialProofsCall, N> {
            self.call_builder(
                &getCredentialProofsCall {
                    _validators,
                },
            )
        }
        ///Creates a new call builder for the [`getPubkeyHashes`] function.
        pub fn getPubkeyHashes(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getPubkeyHashesCall, N> {
            self.call_builder(&getPubkeyHashesCall { _validators })
        }
        ///Creates a new call builder for the [`getStaleBalanceProofs`] function.
        pub fn getStaleBalanceProofs(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStaleBalanceProofsCall, N> {
            self.call_builder(
                &getStaleBalanceProofsCall {
                    validatorIndex,
                },
            )
        }
        ///Creates a new call builder for the [`isActive`] function.
        pub fn isActive(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, isActiveCall, N> {
            self.call_builder(&isActiveCall { validatorIndex })
        }
        ///Creates a new call builder for the [`newValidator`] function.
        pub fn newValidator(
            &self,
            withdrawalCreds: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, newValidatorCall, N> {
            self.call_builder(
                &newValidatorCall {
                    withdrawalCreds,
                },
            )
        }
        ///Creates a new call builder for the [`nextTimestamp`] function.
        pub fn nextTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, nextTimestampCall, N> {
            self.call_builder(&nextTimestampCall {})
        }
        ///Creates a new call builder for the [`pubkey`] function.
        pub fn pubkey(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyCall, N> {
            self.call_builder(&pubkeyCall { validatorIndex })
        }
        ///Creates a new call builder for the [`pubkeyHash`] function.
        pub fn pubkeyHash(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, pubkeyHashCall, N> {
            self.call_builder(&pubkeyHashCall { validatorIndex })
        }
        ///Creates a new call builder for the [`slashValidators`] function.
        pub fn slashValidators(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashValidatorsCall, N> {
            self.call_builder(&slashValidatorsCall { _validators })
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
        ///Creates a new call builder for the [`totalEffectiveBalanceWei`] function.
        pub fn totalEffectiveBalanceWei(
            &self,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalEffectiveBalanceWeiCall, N> {
            self.call_builder(
                &totalEffectiveBalanceWeiCall {
                    validatorIndices,
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
    > BeaconChainMockInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
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
