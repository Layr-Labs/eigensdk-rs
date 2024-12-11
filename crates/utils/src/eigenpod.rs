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
library IEigenPodTypes {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int64 balanceDeltasGwei; uint64 beaconChainBalanceBeforeGwei; }
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IEigenPodTypes {
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
struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int64 balanceDeltasGwei; uint64 beaconChainBalanceBeforeGwei; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint {
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        pub proofsRemaining: alloy::sol_types::private::primitives::aliases::U24,
        pub podBalanceGwei: u64,
        pub balanceDeltasGwei: i64,
        pub beaconChainBalanceBeforeGwei: u64,
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
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Uint<64>,
            alloy::sol_types::sol_data::Int<64>,
            alloy::sol_types::sol_data::Uint<64>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U24,
            u64,
            i64,
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
        impl ::core::convert::From<Checkpoint> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint) -> Self {
                (
                    value.beaconBlockRoot,
                    value.proofsRemaining,
                    value.podBalanceGwei,
                    value.balanceDeltasGwei,
                    value.beaconChainBalanceBeforeGwei,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Checkpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    beaconBlockRoot: tuple.0,
                    proofsRemaining: tuple.1,
                    podBalanceGwei: tuple.2,
                    balanceDeltasGwei: tuple.3,
                    beaconChainBalanceBeforeGwei: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Checkpoint {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Checkpoint {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconBlockRoot),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofsRemaining),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.podBalanceGwei),
                    <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceDeltasGwei),
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.beaconChainBalanceBeforeGwei,
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
        impl alloy_sol_types::SolType for Checkpoint {
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
        impl alloy_sol_types::SolStruct for Checkpoint {
            const NAME: &'static str = "Checkpoint";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint(bytes32 beaconBlockRoot,uint24 proofsRemaining,uint64 podBalanceGwei,int64 balanceDeltasGwei,uint64 beaconChainBalanceBeforeGwei)",
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
                            &self.beaconBlockRoot,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.proofsRemaining,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.podBalanceGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.balanceDeltasGwei,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.beaconChainBalanceBeforeGwei,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Checkpoint {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconBlockRoot,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.proofsRemaining,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.podBalanceGwei,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceDeltasGwei,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        64,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.beaconChainBalanceBeforeGwei,
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
                    &rust.beaconBlockRoot,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.proofsRemaining,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.podBalanceGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceDeltasGwei,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.beaconChainBalanceBeforeGwei,
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
struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInfo {
        pub validatorIndex: u64,
        pub restakedBalanceGwei: u64,
        pub lastCheckpointedAt: u64,
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
                    value.lastCheckpointedAt,
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
                    lastCheckpointedAt: tuple.2,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.lastCheckpointedAt),
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
                    "ValidatorInfo(uint64 validatorIndex,uint64 restakedBalanceGwei,uint64 lastCheckpointedAt,uint8 status)",
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
                            &self.lastCheckpointedAt,
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
                        &rust.lastCheckpointedAt,
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
                    &rust.lastCheckpointedAt,
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IEigenPodTypes`](self) contract instance.

See the [wrapper's documentation](`IEigenPodTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IEigenPodTypesInstance<T, P, N> {
        IEigenPodTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IEigenPodTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IEigenPodTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IEigenPodTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IEigenPodTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IEigenPodTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IEigenPodTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IEigenPodTypes`](self) contract instance.

See the [wrapper's documentation](`IEigenPodTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IEigenPodTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IEigenPodTypesInstance<T, P, N> {
            IEigenPodTypesInstance {
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
    > IEigenPodTypesInstance<T, P, N> {
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
    > IEigenPodTypesInstance<T, P, N> {
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

library IEigenPodTypes {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint {
        bytes32 beaconBlockRoot;
        uint24 proofsRemaining;
        uint64 podBalanceGwei;
        int64 balanceDeltasGwei;
        uint64 beaconChainBalanceBeforeGwei;
    }
    struct ValidatorInfo {
        uint64 validatorIndex;
        uint64 restakedBalanceGwei;
        uint64 lastCheckpointedAt;
        VALIDATOR_STATUS status;
    }
}

interface EigenPod {
    error AmountMustBeMultipleOfGwei();
    error BeaconTimestampTooFarInPast();
    error CannotCheckpointTwiceInSingleBlock();
    error CheckpointAlreadyActive();
    error CredentialsAlreadyVerified();
    error CurrentlyPaused();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InsufficientWithdrawableBalance();
    error InvalidEIP4788Response();
    error InvalidProof();
    error InvalidProofLength();
    error InvalidPubKeyLength();
    error InvalidValidatorFieldsLength();
    error MsgValueNot32ETH();
    error NoActiveCheckpoint();
    error NoBalanceToCheckpoint();
    error OnlyEigenPodManager();
    error OnlyEigenPodOwner();
    error OnlyEigenPodOwnerOrProofSubmitter();
    error TimestampOutOfRange();
    error ValidatorInactiveOnBeaconChain();
    error ValidatorIsExitingBeaconChain();
    error ValidatorNotActiveInPod();
    error ValidatorNotSlashedOnBeaconChain();
    error WithdrawCredentialsNotForEigenPod();

    event CheckpointCreated(uint64 indexed checkpointTimestamp, bytes32 indexed beaconBlockRoot, uint256 validatorCount);
    event CheckpointFinalized(uint64 indexed checkpointTimestamp, int256 totalShareDeltaWei);
    event EigenPodStaked(bytes pubkey);
    event Initialized(uint8 version);
    event NonBeaconChainETHReceived(uint256 amountReceived);
    event ProofSubmitterUpdated(address prevProofSubmitter, address newProofSubmitter);
    event RestakedBeaconChainETHWithdrawn(address indexed recipient, uint256 amount);
    event ValidatorBalanceUpdated(uint40 validatorIndex, uint64 balanceTimestamp, uint64 newValidatorBalanceGwei);
    event ValidatorCheckpointed(uint64 indexed checkpointTimestamp, uint40 indexed validatorIndex);
    event ValidatorRestaked(uint40 validatorIndex);
    event ValidatorWithdrawn(uint64 indexed checkpointTimestamp, uint40 indexed validatorIndex);

    constructor(address _ethPOS, address _eigenPodManager, uint64 _GENESIS_TIME);

    receive() external payable;

    function GENESIS_TIME() external view returns (uint64);
    function activeValidatorCount() external view returns (uint256);
    function checkpointBalanceExitedGwei(uint64) external view returns (uint64);
    function currentCheckpoint() external view returns (IEigenPodTypes.Checkpoint memory);
    function currentCheckpointTimestamp() external view returns (uint64);
    function eigenPodManager() external view returns (address);
    function ethPOS() external view returns (address);
    function getParentBlockRoot(uint64 timestamp) external view returns (bytes32);
    function initialize(address _podOwner) external;
    function lastCheckpointTimestamp() external view returns (uint64);
    function podOwner() external view returns (address);
    function proofSubmitter() external view returns (address);
    function recoverTokens(address[] memory tokenList, uint256[] memory amountsToWithdraw, address recipient) external;
    function setProofSubmitter(address newProofSubmitter) external;
    function stake(bytes memory pubkey, bytes memory signature, bytes32 depositDataRoot) external payable;
    function startCheckpoint(bool revertIfNoBalance) external;
    function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPodTypes.ValidatorInfo memory);
    function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPodTypes.ValidatorInfo memory);
    function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
    function verifyCheckpointProofs(BeaconChainProofs.BalanceContainerProof memory balanceContainerProof, BeaconChainProofs.BalanceProof[] memory proofs) external;
    function verifyStaleBalance(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.ValidatorProof memory proof) external;
    function verifyWithdrawalCredentials(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
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
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
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
    "name": "activeValidatorCount",
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
    "name": "checkpointBalanceExitedGwei",
    "inputs": [
      {
        "name": "",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "currentCheckpoint",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IEigenPodTypes.Checkpoint",
        "components": [
          {
            "name": "beaconBlockRoot",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "proofsRemaining",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "podBalanceGwei",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "balanceDeltasGwei",
            "type": "int64",
            "internalType": "int64"
          },
          {
            "name": "beaconChainBalanceBeforeGwei",
            "type": "uint64",
            "internalType": "uint64"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "currentCheckpointTimestamp",
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
    "name": "getParentBlockRoot",
    "inputs": [
      {
        "name": "timestamp",
        "type": "uint64",
        "internalType": "uint64"
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
    "name": "lastCheckpointTimestamp",
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
    "name": "proofSubmitter",
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
    "name": "setProofSubmitter",
    "inputs": [
      {
        "name": "newProofSubmitter",
        "type": "address",
        "internalType": "address"
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
    "name": "startCheckpoint",
    "inputs": [
      {
        "name": "revertIfNoBalance",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
        "internalType": "struct IEigenPodTypes.ValidatorInfo",
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
            "name": "lastCheckpointedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
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
        "internalType": "struct IEigenPodTypes.ValidatorInfo",
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
            "name": "lastCheckpointedAt",
            "type": "uint64",
            "internalType": "uint64"
          },
          {
            "name": "status",
            "type": "uint8",
            "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
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
        "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
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
        "internalType": "enum IEigenPodTypes.VALIDATOR_STATUS"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "verifyCheckpointProofs",
    "inputs": [
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
        "name": "proofs",
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyStaleBalance",
    "inputs": [
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
        "name": "proof",
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
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "verifyWithdrawalCredentials",
    "inputs": [
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
    "name": "CheckpointCreated",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "beaconBlockRoot",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "validatorCount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "CheckpointFinalized",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "totalShareDeltaWei",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      }
    ],
    "anonymous": false
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
    "name": "ProofSubmitterUpdated",
    "inputs": [
      {
        "name": "prevProofSubmitter",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "newProofSubmitter",
        "type": "address",
        "indexed": false,
        "internalType": "address"
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
    "name": "ValidatorCheckpointed",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": true,
        "internalType": "uint40"
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
    "name": "ValidatorWithdrawn",
    "inputs": [
      {
        "name": "checkpointTimestamp",
        "type": "uint64",
        "indexed": true,
        "internalType": "uint64"
      },
      {
        "name": "validatorIndex",
        "type": "uint40",
        "indexed": true,
        "internalType": "uint40"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AmountMustBeMultipleOfGwei",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BeaconTimestampTooFarInPast",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CannotCheckpointTwiceInSingleBlock",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CheckpointAlreadyActive",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CredentialsAlreadyVerified",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
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
    "name": "InsufficientWithdrawableBalance",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidEIP4788Response",
    "inputs": []
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
    "name": "InvalidPubKeyLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidValidatorFieldsLength",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MsgValueNot32ETH",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoActiveCheckpoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NoBalanceToCheckpoint",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodManager",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyEigenPodOwnerOrProofSubmitter",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TimestampOutOfRange",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorInactiveOnBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorIsExitingBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorNotActiveInPod",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ValidatorNotSlashedOnBeaconChain",
    "inputs": []
  },
  {
    "type": "error",
    "name": "WithdrawCredentialsNotForEigenPod",
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
pub mod EigenPod {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e0346101a357601f61318a38819003918201601f19168301916001600160401b038311848410176101a7578084926060946040528339810103126101a3578051906001600160a01b03821682036101a3576020810151906001600160a01b03821682036101a35760400151916001600160401b03831683036101a35760805260a05260c0525f5460ff8160081c1661014e5760ff80821610610114575b604051612fce90816101bc82396080518181816107900152610916015260a0518181816102550152818161045d01528181610821015281816108d601528181610a7401528181610f380152818161101f0152818161130a015281816114f401528181611a110152612b3f015260c05181610fa20152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f61009d565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610022575b3615610018575f80fd5b610020611e77565b005b5f3560e01c8063039157d2146101b15780630b18ff66146101ac5780632340e8d3146101a75780633474aa16146101a25780633f65cf191461019d57806342ecff2a146101985780634665bcda1461019357806347d283721461018e57806352396a5914610189578063587533571461018457806358eaee791461017f5780636c0d2d5a1461017a5780636fcd0e53146101755780637439841f1461017057806374cdd7981461016b57806388676cad146101665780639b4e463414610161578063b522538a1461015c578063c490744214610157578063c4d66de814610152578063d06d55871461014d578063dda3346c14610148578063ee94d67c14610143578063f074ba621461013e5763f28824610361000e57610f83565b610ec9565b610ea3565b610dea565b610c55565b610b5e565b610a4e565b6109ec565b610880565b6107c9565b61077b565b610746565b6106bb565b610642565b6105ff565b610551565b610510565b61048c565b610448565b61041f565b61037a565b610324565b610307565b6102df565b6101de565b600435906001600160401b03821682036101cc57565b5f80fd5b908160409103126101cc5790565b346101cc5760603660031901126101cc576101f76101b6565b6024356001600160401b0381116101cc576102169036906004016101d0565b6044356001600160401b0381116101cc576102359036906004016101d0565b604051635ac86ab760e01b815260066004820152929091906020846024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9384156102d0576100209461029c915f916102a1575b5015610fe9565b610fff565b6102c3915060203d6020116102c9575b6102bb8183610d29565b810190610fc6565b5f610295565b503d6102b1565b610fde565b5f9103126101cc57565b346101cc575f3660031901126101cc576033546040516001600160a01b039091168152602090f35b346101cc575f3660031901126101cc576020603954604051908152f35b346101cc575f3660031901126101cc5760206001600160401b0360345416604051908152f35b9181601f840112156101cc578235916001600160401b0383116101cc576020808501948460051b0101116101cc57565b346101cc5760a03660031901126101cc576103936101b6565b6024356001600160401b0381116101cc576103b29036906004016101d0565b6044356001600160401b0381116101cc576103d190369060040161034a565b6064939193356001600160401b0381116101cc576103f390369060040161034a565b91608435956001600160401b0387116101cc5761041761002097369060040161034a565b9690956112c9565b346101cc575f3660031901126101cc5760206001600160401b03603a5460401c16604051908152f35b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101cc575f3660031901126101cc575f60806040516104ab81610cee565b828152826020820152826040820152826060820152015260a06104cc61163a565b6001600160401b036080604051928051845262ffffff6020820151166020850152826040820151166040850152606081015160070b60608501520151166080820152f35b346101cc5760203660031901126101cc576001600160401b036105316101b6565b165f52603b60205260206001600160401b0360405f205416604051908152f35b346101cc575f3660031901126101cc57603e546040516001600160a01b039091168152602090f35b9181601f840112156101cc578235916001600160401b0383116101cc57602083818601950101116101cc57565b60206003198201126101cc57600435906001600160401b0382116101cc576105d091600401610579565b9091565b600311156105de57565b634e487b7160e01b5f52602160045260245ffd5b9060038210156105de5752565b346101cc5761061f61061a610613366105a6565b36916116a2565b6125d8565b5f526036602052602060ff60405f205460c01c1661064060405180926105f2565bf35b346101cc5760203660031901126101cc5760206106656106606101b6565b611784565b604051908152f35b6106b99092919260608060808301956001600160401b0381511684526001600160401b0360208201511660208501526001600160401b03604082015116604085015201519101906105f2565b565b346101cc5760203660031901126101cc576004356106d7611826565b505f52603660205261074260405f2061073660ff604051926106f884610d0e565b546001600160401b03811684526001600160401b038160401c1660208501526001600160401b038160801c16604085015260c01c16606083016111e1565b6040519182918261066d565b0390f35b346101cc5760203660031901126101cc576004355f526036602052602060ff60405f205460c01c1661064060405180926105f2565b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b801515036101cc57565b346101cc5760203660031901126101cc576004356107e6816107bf565b6033546001600160a01b03163314801561086c575b610804906112b3565b604051635ac86ab760e01b815260066004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156102d05761002092610867915f916102a1575015610fe9565b6122b7565b50603e546001600160a01b031633146107fb565b60603660031901126101cc576004356001600160401b0381116101cc576108ab903690600401610579565b6024356001600160401b0381116101cc576108ca903690600401610579565b929060443593610904337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461184a565b6801bc16d674ec80000034036109dd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610946612630565b92813b156101cc576801bc16d674ec8000005f9461097c604051998a96879586946304512a2360e31b86528c8c600488016118a4565b03925af19283156102d0577f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e23936109c3575b506109be604051928392836118e6565b0390a1005b806109d15f6109d793610d29565b806102d5565b5f6109ae565b63049696b360e31b5f5260045ffd5b346101cc57610a1061061a610a00366105a6565b610a08611826565b5036916116a2565b5f52603660205261074260405f2061073660ff604051926106f884610d0e565b6001600160a01b038116036101cc57565b604435906106b982610a30565b346101cc5760403660031901126101cc57600435610a6b81610a30565b602435610aa2337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461184a565b633b9aca008106610b4f5761002091610b0f610af36001600160401b03633b9aca00850416610ad96034546001600160401b031690565b610aee6001600160401b0382168311156118f7565b61190d565b6001600160401b03166001600160401b03196034541617603455565b6040518281526001600160a01b03919091169081907f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e90602090a261265b565b6321ddeb1760e21b5f5260045ffd5b346101cc5760203660031901126101cc57600435610b7b81610a30565b610bc95f5491610baf610b99610b958560ff9060081c1690565b1590565b80948195610c47575b8115610c27575b5061192d565b82610bc0600160ff195f5416175f55565b610c1057611990565b610bcf57005b610bdd61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016109be565b610c2261010061ff00195f5416175f55565b611990565b303b15915081610c39575b505f610ba9565b60ff1660011490505f610c32565b600160ff8216109150610ba2565b346101cc5760203660031901126101cc57600435610c7281610a30565b610c8760018060a01b036033541633146119c8565b603e54604080516001600160a01b03808416825290931660208401819052927ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac9190a16001600160a01b03191617603e55005b634e487b7160e01b5f52604160045260245ffd5b60a081019081106001600160401b03821117610d0957604052565b610cda565b608081019081106001600160401b03821117610d0957604052565b90601f801991011681019081106001600160401b03821117610d0957604052565b604051906106b960a083610d29565b604051906106b9608083610d29565b906106b96040519283610d29565b6001600160401b038111610d095760051b60200190565b9080601f830112156101cc578135610da481610d76565b92610db26040519485610d29565b81845260208085019260051b8201019283116101cc57602001905b828210610dda5750505090565b8135815260209182019101610dcd565b346101cc5760603660031901126101cc576004356001600160401b0381116101cc57366023820112156101cc57806004013590610e2682610d76565b91610e346040519384610d29565b8083526024602084019160051b830101913683116101cc57602401905b828210610e8957602435846001600160401b0382116101cc57610e7b610020923690600401610d8d565b610e83610a41565b916119de565b602080918335610e9881610a30565b815201910190610e51565b346101cc575f3660031901126101cc5760206001600160401b03603a5416604051908152f35b346101cc5760403660031901126101cc576004356001600160401b0381116101cc57610ef99036906004016101d0565b6024356001600160401b0381116101cc57610f1890369060040161034a565b604051635ac86ab760e01b815260076004820152929091906020846024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9384156102d05761002094610f7e915f916102a1575015610fe9565b611c99565b346101cc575f3660031901126101cc5760206040516001600160401b037f0000000000000000000000000000000000000000000000000000000000000000168152f35b908160209103126101cc5751610fdb816107bf565b90565b6040513d5f823e3d90fd5b15610ff057565b63840a48d560e01b5f5260045ffd5b604051635ac86ab760e01b815260086004820152919291906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156102d0576111589461106b61113892611148955f916102a1575015610fe9565b61108761108261107b8780611160565b3691611195565b611ea3565b5f52603660205261111c816111176110a160405f206111ed565b956110d86110c86110bc60408a01516001600160401b031690565b6001600160401b031690565b6001600160401b0383161161123f565b6110fa600160608901516110eb816105d4565b6110f4816105d4565b14611255565b61066061111261110d61107b8c80611160565b611eb1565b61126b565b611ef0565b359361115261112b8280611160565b9390926020810190611281565b959094516001600160401b031690565b64ffffffffff1690565b94611fd5565b6106b961217d565b903590601e19813603018212156101cc57018035906001600160401b0382116101cc57602001918160051b360383136101cc57565b9291906111a181610d76565b936111af6040519586610d29565b602085838152019160051b81019283116101cc57905b8282106111d157505050565b81358152602091820191016111c5565b60038210156105de5752565b906106b96040516111fd81610d0e565b606060ff8295546001600160401b03811684526001600160401b038160401c1660208501526001600160401b038160801c16604085015260c01c1691016111e1565b1561124657565b6337e07ffd60e01b5f5260045ffd5b1561125c57565b63d49e19a760e01b5f5260045ffd5b1561127257565b63161ce5ed60e31b5f5260045ffd5b903590601e19813603018212156101cc57018035906001600160401b0382116101cc576020019181360383136101cc57565b156112ba57565b63427a777960e01b5f5260045ffd5b9695949392919060018060a01b036033541633148015611355575b6112ed906112b3565b604051635ac86ab760e01b815260026004820152976020896024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9889156102d0576106b999611350915f916102a1575015610fe9565b611468565b50603e546001600160a01b031633146112e4565b1561137057565b6343714afd60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b91908110156113a35760051b0190565b61137f565b3564ffffffffff811681036101cc5790565b908210156113a3576105d09160051b810190611281565b908210156113a3576105d09160051b810190611160565b634e487b7160e01b5f52601160045260245ffd5b906020820180921161140a57565b6113e8565b906001820180921161140a57565b9190820180921161140a57565b8115611434570490565b634e487b7160e01b5f52601260045260245ffd5b906001600160401b03809116911601906001600160401b03821161140a57565b816111176114af92999599989496979398848b1480611631575b611493909b9a99989796959b611369565b6106606110c86110bc603a546001600160401b039060401c1690565b5f965f965b8088106115d35750505050505050506001600160401b036114e1603a546001600160401b039060401c1690565b16611573575b6033546001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692911690823b156101cc57604051630257884360e21b81526001600160a01b0392909216600483015260248201525f6044820181905290918290606490829084905af180156102d0576115655750565b806109d15f6106b993610d29565b6115ce6115a1611588633b9aca0084046110bc565b603d5460981c6001600160401b0316611448565b611448565b603d805467ffffffffffffffff60981b191660989290921b67ffffffffffffffff60981b16919091179055565b6114e7565b909192939495969761162360019161161d89896116158e61160b8f8b611605611600858e81953599611393565b6113a8565b966113ba565b9290918d8d6113d1565b9490936123d5565b9061141d565b9801969594939291906114b4565b50848714611482565b6040519061164782610cee565b81603c54815260806001600160401b0380603d5462ffffff81166020860152818160181c1660408601528060581c60070b606086015260981c1616910152565b6001600160401b038111610d0957601f01601f191660200190565b9291926116ae82611687565b916116bc6040519384610d29565b8294818452818301116101cc578281602093845f960137010152565b90633b9aca00820291808304633b9aca00149015171561140a57565b90670de0b6b3a7640000820291808304670de0b6b3a7640000149015171561140a57565b600181901b91906001600160ff1b0381160361140a57565b3d1561175a573d9061174182611687565b9161174f6040519384610d29565b82523d5f602084013e565b606090565b1561176657565b63558ad0a360e01b5f5260045ffd5b908160209103126101cc575190565b6001600160401b038116420342811161140a5762017ff4111561181757604080516001600160401b0390921660208084019182528352610fdb925f92839291906117ce9082610d29565b5190720f3df6d732807ef1319fb7b8bb8522d0beac025afa6117ee611730565b908061180d575b6117fe9061175f565b60208082518301019101611775565b50805115156117f5565b637944e66d60e11b5f5260045ffd5b6040519061183382610d0e565b5f6060838281528260208201528260408201520152565b1561185157565b633213a66160e21b5f5260045ffd5b908060209392818452848401375f828201840152601f01601f1916010190565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b969594906118e1936118c56118d3926060979560808c5260808c0191611860565b9089820360208b0152611880565b918783036040890152611860565b930152565b916020610fdb938181520191611860565b156118fe57565b6302c6f54760e21b5f5260045ffd5b906001600160401b03809116911603906001600160401b03821161140a57565b1561193457565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6001600160a01b031680156119b9576bffffffffffffffffffffffff60a01b6033541617603355565b6339b190bb60e11b5f5260045ffd5b156119cf57565b63719f370360e11b5f5260045ffd5b91926119f560018060a01b036033541633146119c8565b604051635ac86ab760e01b8152600560048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156102d057611a56915f91611b5c575b5093919315610fe9565b611a638151835114611369565b6040936001600160a01b0316905f5b8151811015611b5457600190611b23875f806001600160a01b03611a968689611b88565b5116611aa2868b611b88565b51828551602081019263a9059cbb60e01b84528c6024830152604482015260448152611acf606482610d29565b611adb87519788610d29565b602087527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020880152611b11843b1515612f08565b51925af1611b1d611730565b90612f54565b805180611b33575b505001611a72565b81602080611b4893611b4d9501019101610fc6565b612dba565b5f80611b2b565b505050509050565b611b75915060203d6020116102c9576102bb8183610d29565b5f611a4c565b8051156113a35760200190565b80518210156113a35760209160051b010190565b15611ba357565b631a544f4960e01b5f5260045ffd5b91908110156113a35760051b81013590605e19813603018212156101cc570190565b62ffffff16801561140a575f190190565b9060070b9060070b0190677fffffffffffffff198212677fffffffffffffff83131761140a57565b9060038110156105de57815460ff60c01b191660c09190911b60ff60c01b16179055565b8151815460208401516040808601516001600160401b039094166001600160c01b031990931692909217911b67ffffffffffffffff60401b161760809190911b67ffffffffffffffff60801b1617815560609091015160038110156105de576106b991611c0d565b603a5460401c6001600160401b031693929184611cb7811515611b9c565b611cbf61163a565b93611ccb848651612735565b5f935f6020870190608088019360608901915b818110611d44575050505050505050611d3f90611d25611d136106b995966001600160401b03165f52603b60205260405f2090565b9161159c83546001600160401b031690565b6001600160401b03166001600160401b0319825416179055565b612a32565b611d4f81838a611bb2565b8035998d611d6d611d688d5f52603660205260405f2090565b6111ed565b9260016060850151611d7e816105d4565b611d87816105d4565b03611e6a578a611da46110bc60408701516001600160401b031690565b1015611e6a57908392918935611dba9285612809565b918951611dc99062ffffff1690565b611dd290611bd4565b62ffffff168a528b516001600160401b031690611dee91611448565b6001600160401b03168b52875160070b90611e0891611be5565b60070b8752611e1691611448565b9a611e29905f52603660205260405f2090565b90611e3391611c31565b5164ffffffffff16877fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f5f80a36001905b01611cde565b5050995050600190611e64565b7f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf496020604051348152a1565b8051156113a3576020015190565b8051600310156113a35760800151151590565b15611ecb57565b6313717da960e21b5f5260045ffd5b15611ee157565b6309bde33960e01b5f5260045ffd5b9091611f23611f1960208501611f136060611f0b8389611281565b905014611ec4565b85611281565b94359436916116a2565b92600393611f3c81518015159081611fc9575b50611ec4565b602092611f4884610d68565b92835283955b82518711611fb85760018116611f8e5783515f52868301518552848460405f60026107cf195a01fa156101cc57611f889060011c966113fc565b95611f4e565b868301515f5283518552848460405f60026107cf195a01fa156101cc57611f889060011c966113fc565b509450506106b99291505114611eda565b601f169050155f611f36565b9291909493946008820361214357611ff49161107b6105c08814611ec4565b805160011c61200281612d32565b915f5b8281106120f157505060011c805b61204f57509161204561204a949264ffffffffff6120346106b99896611b7b565b519416600b60291b179436916116a2565b612c92565b611eda565b5f5b818110612062575060011c80612013565b60205f6120d061207a61207485611718565b87611b88565b516120c461209861209261208d88611718565b61140f565b89611b88565b51916120b66040519384928884019091604092825260208201520190565b03601f198101835282610d29565b604051918280926125c6565b039060025afa156102d0576001905f516120ea8286611b88565b5201612051565b60205f61212261210961210385611718565b86611b88565b516120c461209861211c61208d88611718565b88611b88565b039060025afa156102d0576001905f5161213c8287611b88565b5201612005565b63200591bd60e01b5f5260045ffd5b1561215957565b62be9bc360e81b5f5260045ffd5b1561216e57565b6367db5b8b60e01b5f5260045ffd5b6001600160401b036121ba6121ad603a546110bc846121a6836001600160401b039060401c1690565b1615612152565b4283169216821415612167565b6121de6121cc633b9aca0047046110bc565b6034546001600160401b03169061190d565b907f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef10766122b261229c61220f84611784565b61224f61222060395462ffffff1690565b96612229610d4a565b92835261223f6020840198899062ffffff169052565b6001600160401b03166040830152565b5f60608201525f60808201526122888567ffffffffffffffff60401b603a549160401b169067ffffffffffffffff60401b191617603a55565b61229181612a32565b51945162ffffff1690565b60405162ffffff90911681529081906020820190565b0390a3565b6001600160401b036122e06121ad603a546110bc846121a6836001600160401b039060401c1690565b6122f26121cc633b9aca0047046110bc565b918061233c575b61232d577f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef10766122b261229c61220f84611784565b6332dea95960e21b5f5260045ffd5b506001600160401b038216156122f9565b1561235457565b6335e09e9d60e01b5f5260045ffd5b1561236a57565b631958236d60e21b5f5260045ffd5b1561238057565b632eade63760e01b5f5260045ffd5b6020815191015190602081106123a3575090565b5f199060200360031b1b1690565b156123b857565b632230566760e11b5f5260045ffd5b5f19811461140a5760010190565b92906124d3816001600160401b0396937f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df95610fdb9961241961108236838a611195565b9661244e6060612434611d688b5f52603660205260405f2090565b015161243f816105d4565b612448816105d4565b1561234d565b61246e8b80612466612461368787611195565b612d64565b161415612363565b61248e8b6124886110bc612483368787611195565b612d7b565b14612379565b6124ba6124a461249f368585611195565b612d92565b6124b46124af612630565b61238f565b146123b1565b6124cd6124c8368484611195565b612da3565b99611fd5565b6124e66124e16039546123c7565b603955565b603a546001600160401b03808216939160401c168681166125bc575b5061255590612550612512610d59565b64ffffffffff85168152916001600160401b03881660208401526001600160401b0386166040840152600160608401525f52603660205260405f2090565b611c31565b60405164ffffffffff821681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c1044144990602090a16040805164ffffffffff9290921682526001600160401b03928316602083015291841691810191909152606090a1166116d8565b9250612555612502565b805191908290602001825e015f815290565b6030815103612621575f6126116125ff6120c46010602095604051938491888301906125c6565b86815203600f19810184520182610d29565b039060025afa156102d0575f5190565b634f88323960e11b5f5260045ffd5b604051600160f81b60208201525f60218201523060601b602c82015260208152610fdb604082610d29565b8147106126f0575f918291829182916001600160a01b03165af161267d611730565b501561268557565b60405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608490fd5b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606490fd5b9091612751611f1960208501611f13610100611f0b8389611281565b92606c9361276981518015159081611fc95750611ec4565b60209261277584610d68565b92835283955b82518711611fb857600181166127bb5783515f52868301518552848460405f60026107cf195a01fa156101cc576127b59060011c966113fc565b9561277b565b868301515f5283518552848460405f60026107cf195a01fa156101cc576127b59060011c966113fc565b801561140a575f190190565b60070b677fffffffffffffff19811461140a575f0390565b80515f9586959094939092909164ffffffffff169261283e60208301918561283884516001600160401b031690565b97612e19565b6001600160401b038116916001600160401b03871683036128ee575b6001600160401b0390911690525b6001600160401b038316604083015215612885575b505050929190565b6128a49192955060609061289d6124e16039546127e5565b0160029052565b6001600160401b0364ffffffffff6128be6110bc886127f1565b951691167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a5f80a35f808061287d565b9850806128fe8761286893612e6f565b997f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df60405180612955858a8c849160409194936001600160401b03809264ffffffffff606087019816865216602085015216910152565b0390a190915061285a565b6115a160806106b9928051603c5562ffffff602082015116603d54906affffffffffffffff000000604084015160181b16916affffffffffffffffffffff19161717603d55606081015160070b603d549060581b6001600160401b0360581b16906001600160401b0360581b191617603d5501516001600160401b031690565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b0383131761140a57565b90633b9aca00820291808305633b9aca00149015171561140a57565b600160ff1b811461140a575f0390565b62ffffff612a46602083015162ffffff1690565b16612c89576040810190612ad2610af3612ac1612ab3612aab612a8e612a766110bc89516001600160401b031690565b6fffffffffffffffffffffffffffffffff16600f0b90565b612aa5612a9f606089015160070b90565b60070b90565b906129e0565b600f0b612a06565b94516001600160401b031690565b6034546001600160401b0316611448565b603a54612b019060401c6001600160401b03166001600160401b03166001600160401b0319603a541617603a55565b612b1a67ffffffffffffffff60401b19603a5416603a55565b612b275f603c555f603d55565b5f905f8312612c21575b506033546001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692911690823b156101cc57604051630257884360e21b81526001600160a01b03929092166004830152602482018490526001600160401b03166044820152905f908290606490829084905af180156102d057612c0d575b507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44612c086001600160401b03612bf6603a546001600160401b031690565b60405194855216929081906020820190565b0390a2565b806109d15f612c1b93610d29565b5f612bb7565b612c839150612c64612c5f6110bc6110bc93612c596080612c4a6034546001600160401b031690565b9201516001600160401b031690565b90611448565b6116d8565b612c7e612c79612c7386612a22565b8361141d565b6116f4565b61142a565b5f612b31565b6106b990612960565b9391909293612cab81518015159081611fc95750611ec4565b602092612cb784610d68565b92835283955b82518711612d275760018116612cfd5783515f52868301518552848460405f60026107cf195a01fa156101cc57612cf79060011c966113fc565b95612cbd565b868301515f5283518552848460405f60026107cf195a01fa156101cc57612cf79060011c966113fc565b509450509050511490565b90612d3c82610d76565b612d496040519182610d29565b8281528092612d5a601f1991610d76565b0190602036910137565b8051600510156113a35760c0610fdb910151612ea3565b8051600610156113a35760e0610fdb910151612ea3565b8051600110156113a3576040015190565b8051600210156113a3576060610fdb910151612ea3565b15612dc157565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b610fdb9291612e6561204a60c09360206040870191612e3e6104e0611f0b858b611281565b612045612e56643fffffffff8860021c16948a611281565b939099013598899336916116a2565b60061b161b612ea3565b906001600160401b0380911660070b911660070b03677fffffffffffffff8113677fffffffffffffff1982121761140a5790565b609881901c66ff0000000000001660a882901c65ff00000000001660e883901c61ff001660f884901c1760d884901c62ff0000161760c884901c63ff000000161764ff0000000060b885901c161717179067ff0000000000000060889190911c161790565b15612f0f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b90919015612f60575090565b815115612f705750805190602001fd5b60405162461bcd60e51b815260206004820152908190612f94906024830190611880565b0390fdfea2646970667358221220002ab6e4f00e53d7ef91c791f8e7fe4aef32fa674c512cf7fa25f0f2ad77428a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE04a\x01\xA3W`\x1Fa1\x8A8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01\xA7W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\x01\xA3W\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xA3W` \x81\x01Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01\xA3W`@\x01Q\x91`\x01`\x01`@\x1B\x03\x83\x16\x83\x03a\x01\xA3W`\x80R`\xA0R`\xC0R_T`\xFF\x81`\x08\x1C\x16a\x01NW`\xFF\x80\x82\x16\x10a\x01\x14W[`@Qa/\xCE\x90\x81a\x01\xBC\x829`\x80Q\x81\x81\x81a\x07\x90\x01Ra\t\x16\x01R`\xA0Q\x81\x81\x81a\x02U\x01R\x81\x81a\x04]\x01R\x81\x81a\x08!\x01R\x81\x81a\x08\xD6\x01R\x81\x81a\nt\x01R\x81\x81a\x0F8\x01R\x81\x81a\x10\x1F\x01R\x81\x81a\x13\n\x01R\x81\x81a\x14\xF4\x01R\x81\x81a\x1A\x11\x01Ra+?\x01R`\xC0Q\x81a\x0F\xA2\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\x9DV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\"W[6\x15a\0\x18W_\x80\xFD[a\0 a\x1EwV[\0[_5`\xE0\x1C\x80c\x03\x91W\xD2\x14a\x01\xB1W\x80c\x0B\x18\xFFf\x14a\x01\xACW\x80c#@\xE8\xD3\x14a\x01\xA7W\x80c4t\xAA\x16\x14a\x01\xA2W\x80c?e\xCF\x19\x14a\x01\x9DW\x80cB\xEC\xFF*\x14a\x01\x98W\x80cFe\xBC\xDA\x14a\x01\x93W\x80cG\xD2\x83r\x14a\x01\x8EW\x80cR9jY\x14a\x01\x89W\x80cXu3W\x14a\x01\x84W\x80cX\xEA\xEEy\x14a\x01\x7FW\x80cl\r-Z\x14a\x01zW\x80co\xCD\x0ES\x14a\x01uW\x80ct9\x84\x1F\x14a\x01pW\x80ct\xCD\xD7\x98\x14a\x01kW\x80c\x88gl\xAD\x14a\x01fW\x80c\x9BNF4\x14a\x01aW\x80c\xB5\"S\x8A\x14a\x01\\W\x80c\xC4\x90tB\x14a\x01WW\x80c\xC4\xD6m\xE8\x14a\x01RW\x80c\xD0mU\x87\x14a\x01MW\x80c\xDD\xA34l\x14a\x01HW\x80c\xEE\x94\xD6|\x14a\x01CW\x80c\xF0t\xBAb\x14a\x01>Wc\xF2\x88$a\x03a\0\x0EWa\x0F\x83V[a\x0E\xC9V[a\x0E\xA3V[a\r\xEAV[a\x0CUV[a\x0B^V[a\nNV[a\t\xECV[a\x08\x80V[a\x07\xC9V[a\x07{V[a\x07FV[a\x06\xBBV[a\x06BV[a\x05\xFFV[a\x05QV[a\x05\x10V[a\x04\x8CV[a\x04HV[a\x04\x1FV[a\x03zV[a\x03$V[a\x03\x07V[a\x02\xDFV[a\x01\xDEV[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[_\x80\xFD[\x90\x81`@\x91\x03\x12a\x01\xCCW\x90V[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x01\xF7a\x01\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x02\x16\x906\x90`\x04\x01a\x01\xD0V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x025\x906\x90`\x04\x01a\x01\xD0V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01R\x92\x90\x91\x90` \x84`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x02\xD0Wa\0 \x94a\x02\x9C\x91_\x91a\x02\xA1W[P\x15a\x0F\xE9V[a\x0F\xFFV[a\x02\xC3\x91P` =` \x11a\x02\xC9W[a\x02\xBB\x81\x83a\r)V[\x81\x01\x90a\x0F\xC6V[_a\x02\x95V[P=a\x02\xB1V[a\x0F\xDEV[_\x91\x03\x12a\x01\xCCWV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `9T`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`4T\x16`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xCCWV[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\x03\x93a\x01\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xB2\x906\x90`\x04\x01a\x01\xD0V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xD1\x906\x90`\x04\x01a\x03JV[`d\x93\x91\x935`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xF3\x906\x90`\x04\x01a\x03JV[\x91`\x845\x95`\x01`\x01`@\x1B\x03\x87\x11a\x01\xCCWa\x04\x17a\0 \x976\x90`\x04\x01a\x03JV[\x96\x90\x95a\x12\xC9V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`:T`@\x1C\x16`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW_`\x80`@Qa\x04\xAB\x81a\x0C\xEEV[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R`\xA0a\x04\xCCa\x16:V[`\x01`\x01`@\x1B\x03`\x80`@Q\x92\x80Q\x84Rb\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q`\x07\x0B``\x85\x01R\x01Q\x16`\x80\x82\x01R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x051a\x01\xB6V[\x16_R`;` R` `\x01`\x01`@\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`>T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xCCWV[` `\x03\x19\x82\x01\x12a\x01\xCCW`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x05\xD0\x91`\x04\x01a\x05yV[\x90\x91V[`\x03\x11\x15a\x05\xDEWV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90`\x03\x82\x10\x15a\x05\xDEWRV[4a\x01\xCCWa\x06\x1Fa\x06\x1Aa\x06\x136a\x05\xA6V[6\x91a\x16\xA2V[a%\xD8V[_R`6` R` `\xFF`@_ T`\xC0\x1C\x16a\x06@`@Q\x80\x92a\x05\xF2V[\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW` a\x06ea\x06`a\x01\xB6V[a\x17\x84V[`@Q\x90\x81R\xF3[a\x06\xB9\x90\x92\x91\x92``\x80`\x80\x83\x01\x95`\x01`\x01`@\x1B\x03\x81Q\x16\x84R`\x01`\x01`@\x1B\x03` \x82\x01Q\x16` \x85\x01R`\x01`\x01`@\x1B\x03`@\x82\x01Q\x16`@\x85\x01R\x01Q\x91\x01\x90a\x05\xF2V[V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x06\xD7a\x18&V[P_R`6` Ra\x07B`@_ a\x076`\xFF`@Q\x92a\x06\xF8\x84a\r\x0EV[T`\x01`\x01`@\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03\x81`@\x1C\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16`@\x85\x01R`\xC0\x1C\x16``\x83\x01a\x11\xE1V[`@Q\x91\x82\x91\x82a\x06mV[\x03\x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045_R`6` R` `\xFF`@_ T`\xC0\x1C\x16a\x06@`@Q\x80\x92a\x05\xF2V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x01\xCCWV[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x07\xE6\x81a\x07\xBFV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15a\x08lW[a\x08\x04\x90a\x12\xB3V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x02\xD0Wa\0 \x92a\x08g\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\"\xB7V[P`>T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xFBV[``6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x08\xAB\x906\x90`\x04\x01a\x05yV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x08\xCA\x906\x90`\x04\x01a\x05yV[\x92\x90`D5\x93a\t\x043\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18JV[h\x01\xBC\x16\xD6t\xEC\x80\0\x004\x03a\t\xDDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\tFa&0V[\x92\x81;\x15a\x01\xCCWh\x01\xBC\x16\xD6t\xEC\x80\0\0_\x94a\t|`@Q\x99\x8A\x96\x87\x95\x86\x94c\x04Q*#`\xE3\x1B\x86R\x8C\x8C`\x04\x88\x01a\x18\xA4V[\x03\x92Z\xF1\x92\x83\x15a\x02\xD0W\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x93a\t\xC3W[Pa\t\xBE`@Q\x92\x83\x92\x83a\x18\xE6V[\x03\x90\xA1\0[\x80a\t\xD1_a\t\xD7\x93a\r)V[\x80a\x02\xD5V[_a\t\xAEV[c\x04\x96\x96\xB3`\xE3\x1B_R`\x04_\xFD[4a\x01\xCCWa\n\x10a\x06\x1Aa\n\x006a\x05\xA6V[a\n\x08a\x18&V[P6\x91a\x16\xA2V[_R`6` Ra\x07B`@_ a\x076`\xFF`@Q\x92a\x06\xF8\x84a\r\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xCCWV[`D5\x90a\x06\xB9\x82a\n0V[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW`\x045a\nk\x81a\n0V[`$5a\n\xA23\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18JV[c;\x9A\xCA\0\x81\x06a\x0BOWa\0 \x91a\x0B\x0Fa\n\xF3`\x01`\x01`@\x1B\x03c;\x9A\xCA\0\x85\x04\x16a\n\xD9`4T`\x01`\x01`@\x1B\x03\x16\x90V[a\n\xEE`\x01`\x01`@\x1B\x03\x82\x16\x83\x11\x15a\x18\xF7V[a\x19\rV[`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19`4T\x16\x17`4UV[`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x81\x90\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x90` \x90\xA2a&[V[c!\xDD\xEB\x17`\xE2\x1B_R`\x04_\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x0B{\x81a\n0V[a\x0B\xC9_T\x91a\x0B\xAFa\x0B\x99a\x0B\x95\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x0CGW[\x81\x15a\x0C'W[Pa\x19-V[\x82a\x0B\xC0`\x01`\xFF\x19_T\x16\x17_UV[a\x0C\x10Wa\x19\x90V[a\x0B\xCFW\0[a\x0B\xDDa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\t\xBEV[a\x0C\"a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x19\x90V[0;\x15\x91P\x81a\x0C9W[P_a\x0B\xA9V[`\xFF\x16`\x01\x14\x90P_a\x0C2V[`\x01`\xFF\x82\x16\x10\x91Pa\x0B\xA2V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x0Cr\x81a\n0V[a\x0C\x87`\x01\x80`\xA0\x1B\x03`3T\x163\x14a\x19\xC8V[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`>U\0[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[a\x0C\xDAV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[`@Q\x90a\x06\xB9`\xA0\x83a\r)V[`@Q\x90a\x06\xB9`\x80\x83a\r)V[\x90a\x06\xB9`@Q\x92\x83a\r)V[`\x01`\x01`@\x1B\x03\x81\x11a\r\tW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xCCW\x815a\r\xA4\x81a\rvV[\x92a\r\xB2`@Q\x94\x85a\r)V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xCCW` \x01\x90[\x82\x82\x10a\r\xDAWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\r\xCDV[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW6`#\x82\x01\x12\x15a\x01\xCCW\x80`\x04\x015\x90a\x0E&\x82a\rvV[\x91a\x0E4`@Q\x93\x84a\r)V[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\xCCW`$\x01\x90[\x82\x82\x10a\x0E\x89W`$5\x84`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x0E{a\0 \x926\x90`\x04\x01a\r\x8DV[a\x0E\x83a\nAV[\x91a\x19\xDEV[` \x80\x91\x835a\x0E\x98\x81a\n0V[\x81R\x01\x91\x01\x90a\x0EQV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`:T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x0E\xF9\x906\x90`\x04\x01a\x01\xD0V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x0F\x18\x906\x90`\x04\x01a\x03JV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01R\x92\x90\x91\x90` \x84`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x02\xD0Wa\0 \x94a\x0F~\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x1C\x99V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x81` \x91\x03\x12a\x01\xCCWQa\x0F\xDB\x81a\x07\xBFV[\x90V[`@Q=_\x82>=\x90\xFD[\x15a\x0F\xF0WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01R\x91\x92\x91\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xD0Wa\x11X\x94a\x10ka\x118\x92a\x11H\x95_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x10\x87a\x10\x82a\x10{\x87\x80a\x11`V[6\x91a\x11\x95V[a\x1E\xA3V[_R`6` Ra\x11\x1C\x81a\x11\x17a\x10\xA1`@_ a\x11\xEDV[\x95a\x10\xD8a\x10\xC8a\x10\xBC`@\x8A\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x83\x16\x11a\x12?V[a\x10\xFA`\x01``\x89\x01Qa\x10\xEB\x81a\x05\xD4V[a\x10\xF4\x81a\x05\xD4V[\x14a\x12UV[a\x06`a\x11\x12a\x11\ra\x10{\x8C\x80a\x11`V[a\x1E\xB1V[a\x12kV[a\x1E\xF0V[5\x93a\x11Ra\x11+\x82\x80a\x11`V[\x93\x90\x92` \x81\x01\x90a\x12\x81V[\x95\x90\x94Q`\x01`\x01`@\x1B\x03\x16\x90V[d\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x94a\x1F\xD5V[a\x06\xB9a!}V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xCCWV[\x92\x91\x90a\x11\xA1\x81a\rvV[\x93a\x11\xAF`@Q\x95\x86a\r)V[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x01\xCCW\x90[\x82\x82\x10a\x11\xD1WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\x11\xC5V[`\x03\x82\x10\x15a\x05\xDEWRV[\x90a\x06\xB9`@Qa\x11\xFD\x81a\r\x0EV[```\xFF\x82\x95T`\x01`\x01`@\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03\x81`@\x1C\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16`@\x85\x01R`\xC0\x1C\x16\x91\x01a\x11\xE1V[\x15a\x12FWV[c7\xE0\x7F\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x12\\WV[c\xD4\x9E\x19\xA7`\xE0\x1B_R`\x04_\xFD[\x15a\x12rWV[c\x16\x1C\xE5\xED`\xE3\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW` \x01\x91\x816\x03\x83\x13a\x01\xCCWV[\x15a\x12\xBAWV[cBzwy`\xE0\x1B_R`\x04_\xFD[\x96\x95\x94\x93\x92\x91\x90`\x01\x80`\xA0\x1B\x03`3T\x163\x14\x80\x15a\x13UW[a\x12\xED\x90a\x12\xB3V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01R\x97` \x89`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x98\x89\x15a\x02\xD0Wa\x06\xB9\x99a\x13P\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x14hV[P`>T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xE4V[\x15a\x13pWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x13\xA3W`\x05\x1B\x01\x90V[a\x13\x7FV[5d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01\xCCW\x90V[\x90\x82\x10\x15a\x13\xA3Wa\x05\xD0\x91`\x05\x1B\x81\x01\x90a\x12\x81V[\x90\x82\x10\x15a\x13\xA3Wa\x05\xD0\x91`\x05\x1B\x81\x01\x90a\x11`V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90` \x82\x01\x80\x92\x11a\x14\nWV[a\x13\xE8V[\x90`\x01\x82\x01\x80\x92\x11a\x14\nWV[\x91\x90\x82\x01\x80\x92\x11a\x14\nWV[\x81\x15a\x144W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x01\x90`\x01`\x01`@\x1B\x03\x82\x11a\x14\nWV[\x81a\x11\x17a\x14\xAF\x92\x99\x95\x99\x98\x94\x96\x97\x93\x98\x84\x8B\x14\x80a\x161W[a\x14\x93\x90\x9B\x9A\x99\x98\x97\x96\x95\x9Ba\x13iV[a\x06`a\x10\xC8a\x10\xBC`:T`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[_\x96_\x96[\x80\x88\x10a\x15\xD3WPPPPPPPP`\x01`\x01`@\x1B\x03a\x14\xE1`:T`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[\x16a\x15sW[`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91\x16\x90\x82;\x15a\x01\xCCW`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R_`D\x82\x01\x81\x90R\x90\x91\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02\xD0Wa\x15eWPV[\x80a\t\xD1_a\x06\xB9\x93a\r)V[a\x15\xCEa\x15\xA1a\x15\x88c;\x9A\xCA\0\x84\x04a\x10\xBCV[`=T`\x98\x1C`\x01`\x01`@\x1B\x03\x16a\x14HV[a\x14HV[`=\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16`\x98\x92\x90\x92\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x16\x91\x90\x91\x17\x90UV[a\x14\xE7V[\x90\x91\x92\x93\x94\x95\x96\x97a\x16#`\x01\x91a\x16\x1D\x89\x89a\x16\x15\x8Ea\x16\x0B\x8F\x8Ba\x16\x05a\x16\0\x85\x8E\x81\x955\x99a\x13\x93V[a\x13\xA8V[\x96a\x13\xBAV[\x92\x90\x91\x8D\x8Da\x13\xD1V[\x94\x90\x93a#\xD5V[\x90a\x14\x1DV[\x98\x01\x96\x95\x94\x93\x92\x91\x90a\x14\xB4V[P\x84\x87\x14a\x14\x82V[`@Q\x90a\x16G\x82a\x0C\xEEV[\x81`<T\x81R`\x80`\x01`\x01`@\x1B\x03\x80`=Tb\xFF\xFF\xFF\x81\x16` \x86\x01R\x81\x81`\x18\x1C\x16`@\x86\x01R\x80`X\x1C`\x07\x0B``\x86\x01R`\x98\x1C\x16\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\r\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x16\xAE\x82a\x16\x87V[\x91a\x16\xBC`@Q\x93\x84a\r)V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xCCW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90c;\x9A\xCA\0\x82\x02\x91\x80\x83\x04c;\x9A\xCA\0\x14\x90\x15\x17\x15a\x14\nWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x80\x83\x04g\r\xE0\xB6\xB3\xA7d\0\0\x14\x90\x15\x17\x15a\x14\nWV[`\x01\x81\x90\x1B\x91\x90`\x01`\x01`\xFF\x1B\x03\x81\x16\x03a\x14\nWV[=\x15a\x17ZW=\x90a\x17A\x82a\x16\x87V[\x91a\x17O`@Q\x93\x84a\r)V[\x82R=_` \x84\x01>V[``\x90V[\x15a\x17fWV[cU\x8A\xD0\xA3`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x01\xCCWQ\x90V[`\x01`\x01`@\x1B\x03\x81\x16B\x03B\x81\x11a\x14\nWb\x01\x7F\xF4\x11\x15a\x18\x17W`@\x80Q`\x01`\x01`@\x1B\x03\x90\x92\x16` \x80\x84\x01\x91\x82R\x83Ra\x0F\xDB\x92_\x92\x83\x92\x91\x90a\x17\xCE\x90\x82a\r)V[Q\x90r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02Z\xFAa\x17\xEEa\x170V[\x90\x80a\x18\rW[a\x17\xFE\x90a\x17_V[` \x80\x82Q\x83\x01\x01\x91\x01a\x17uV[P\x80Q\x15\x15a\x17\xF5V[cyD\xE6m`\xE1\x1B_R`\x04_\xFD[`@Q\x90a\x183\x82a\r\x0EV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x15a\x18QWV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x96\x95\x94\x90a\x18\xE1\x93a\x18\xC5a\x18\xD3\x92``\x97\x95`\x80\x8CR`\x80\x8C\x01\x91a\x18`V[\x90\x89\x82\x03` \x8B\x01Ra\x18\x80V[\x91\x87\x83\x03`@\x89\x01Ra\x18`V[\x93\x01RV[\x91` a\x0F\xDB\x93\x81\x81R\x01\x91a\x18`V[\x15a\x18\xFEWV[c\x02\xC6\xF5G`\xE2\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x14\nWV[\x15a\x194WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x19\xB9Wk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`3T\x16\x17`3UV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a\x19\xCFWV[cq\x9F7\x03`\xE1\x1B_R`\x04_\xFD[\x91\x92a\x19\xF5`\x01\x80`\xA0\x1B\x03`3T\x163\x14a\x19\xC8V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xD0Wa\x1AV\x91_\x91a\x1B\\W[P\x93\x91\x93\x15a\x0F\xE9V[a\x1Ac\x81Q\x83Q\x14a\x13iV[`@\x93`\x01`\x01`\xA0\x1B\x03\x16\x90_[\x81Q\x81\x10\x15a\x1BTW`\x01\x90a\x1B#\x87_\x80`\x01`\x01`\xA0\x1B\x03a\x1A\x96\x86\x89a\x1B\x88V[Q\x16a\x1A\xA2\x86\x8Ba\x1B\x88V[Q\x82\x85Q` \x81\x01\x92c\xA9\x05\x9C\xBB`\xE0\x1B\x84R\x8C`$\x83\x01R`D\x82\x01R`D\x81Ra\x1A\xCF`d\x82a\r)V[a\x1A\xDB\x87Q\x97\x88a\r)V[` \x87R\x7FSafeERC20: low-level call failed` \x88\x01Ra\x1B\x11\x84;\x15\x15a/\x08V[Q\x92Z\xF1a\x1B\x1Da\x170V[\x90a/TV[\x80Q\x80a\x1B3W[PP\x01a\x1ArV[\x81` \x80a\x1BH\x93a\x1BM\x95\x01\x01\x91\x01a\x0F\xC6V[a-\xBAV[_\x80a\x1B+V[PPPP\x90PV[a\x1Bu\x91P` =` \x11a\x02\xC9Wa\x02\xBB\x81\x83a\r)V[_a\x1ALV[\x80Q\x15a\x13\xA3W` \x01\x90V[\x80Q\x82\x10\x15a\x13\xA3W` \x91`\x05\x1B\x01\x01\x90V[\x15a\x1B\xA3WV[c\x1ATOI`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x13\xA3W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x90V[b\xFF\xFF\xFF\x16\x80\x15a\x14\nW_\x19\x01\x90V[\x90`\x07\x0B\x90`\x07\x0B\x01\x90g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x13\x17a\x14\nWV[\x90`\x03\x81\x10\x15a\x05\xDEW\x81T`\xFF`\xC0\x1B\x19\x16`\xC0\x91\x90\x91\x1B`\xFF`\xC0\x1B\x16\x17\x90UV[\x81Q\x81T` \x84\x01Q`@\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x94\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x16\x17`\x80\x91\x90\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x81U``\x90\x91\x01Q`\x03\x81\x10\x15a\x05\xDEWa\x06\xB9\x91a\x1C\rV[`:T`@\x1C`\x01`\x01`@\x1B\x03\x16\x93\x92\x91\x84a\x1C\xB7\x81\x15\x15a\x1B\x9CV[a\x1C\xBFa\x16:V[\x93a\x1C\xCB\x84\x86Qa'5V[_\x93_` \x87\x01\x90`\x80\x88\x01\x93``\x89\x01\x91[\x81\x81\x10a\x1DDWPPPPPPPPa\x1D?\x90a\x1D%a\x1D\x13a\x06\xB9\x95\x96`\x01`\x01`@\x1B\x03\x16_R`;` R`@_ \x90V[\x91a\x15\x9C\x83T`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[a*2V[a\x1DO\x81\x83\x8Aa\x1B\xB2V[\x805\x99\x8Da\x1Dma\x1Dh\x8D_R`6` R`@_ \x90V[a\x11\xEDV[\x92`\x01``\x85\x01Qa\x1D~\x81a\x05\xD4V[a\x1D\x87\x81a\x05\xD4V[\x03a\x1EjW\x8Aa\x1D\xA4a\x10\xBC`@\x87\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x10\x15a\x1EjW\x90\x83\x92\x91\x895a\x1D\xBA\x92\x85a(\tV[\x91\x89Qa\x1D\xC9\x90b\xFF\xFF\xFF\x16\x90V[a\x1D\xD2\x90a\x1B\xD4V[b\xFF\xFF\xFF\x16\x8AR\x8BQ`\x01`\x01`@\x1B\x03\x16\x90a\x1D\xEE\x91a\x14HV[`\x01`\x01`@\x1B\x03\x16\x8BR\x87Q`\x07\x0B\x90a\x1E\x08\x91a\x1B\xE5V[`\x07\x0B\x87Ra\x1E\x16\x91a\x14HV[\x9Aa\x1E)\x90_R`6` R`@_ \x90V[\x90a\x1E3\x91a\x1C1V[Qd\xFF\xFF\xFF\xFF\xFF\x16\x87\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?_\x80\xA3`\x01\x90[\x01a\x1C\xDEV[PP\x99PP`\x01\x90a\x1EdV[\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI` `@Q4\x81R\xA1V[\x80Q\x15a\x13\xA3W` \x01Q\x90V[\x80Q`\x03\x10\x15a\x13\xA3W`\x80\x01Q\x15\x15\x90V[\x15a\x1E\xCBWV[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[\x15a\x1E\xE1WV[c\t\xBD\xE39`\xE0\x1B_R`\x04_\xFD[\x90\x91a\x1F#a\x1F\x19` \x85\x01a\x1F\x13``a\x1F\x0B\x83\x89a\x12\x81V[\x90P\x14a\x1E\xC4V[\x85a\x12\x81V[\x945\x946\x91a\x16\xA2V[\x92`\x03\x93a\x1F<\x81Q\x80\x15\x15\x90\x81a\x1F\xC9W[Pa\x1E\xC4V[` \x92a\x1FH\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a\x1F\xB8W`\x01\x81\x16a\x1F\x8EW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa\x1F\x88\x90`\x01\x1C\x96a\x13\xFCV[\x95a\x1FNV[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa\x1F\x88\x90`\x01\x1C\x96a\x13\xFCV[P\x94PPa\x06\xB9\x92\x91PQ\x14a\x1E\xDAV[`\x1F\x16\x90P\x15_a\x1F6V[\x92\x91\x90\x94\x93\x94`\x08\x82\x03a!CWa\x1F\xF4\x91a\x10{a\x05\xC0\x88\x14a\x1E\xC4V[\x80Q`\x01\x1Ca \x02\x81a-2V[\x91_[\x82\x81\x10a \xF1WPP`\x01\x1C\x80[a OWP\x91a Ea J\x94\x92d\xFF\xFF\xFF\xFF\xFFa 4a\x06\xB9\x98\x96a\x1B{V[Q\x94\x16`\x0B`)\x1B\x17\x946\x91a\x16\xA2V[a,\x92V[a\x1E\xDAV[_[\x81\x81\x10a bWP`\x01\x1C\x80a \x13V[` _a \xD0a za t\x85a\x17\x18V[\x87a\x1B\x88V[Qa \xC4a \x98a \x92a \x8D\x88a\x17\x18V[a\x14\x0FV[\x89a\x1B\x88V[Q\x91a \xB6`@Q\x93\x84\x92\x88\x84\x01\x90\x91`@\x92\x82R` \x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\r)V[`@Q\x91\x82\x80\x92a%\xC6V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W`\x01\x90_Qa \xEA\x82\x86a\x1B\x88V[R\x01a QV[` _a!\"a!\ta!\x03\x85a\x17\x18V[\x86a\x1B\x88V[Qa \xC4a \x98a!\x1Ca \x8D\x88a\x17\x18V[\x88a\x1B\x88V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W`\x01\x90_Qa!<\x82\x87a\x1B\x88V[R\x01a \x05V[c \x05\x91\xBD`\xE0\x1B_R`\x04_\xFD[\x15a!YWV[b\xBE\x9B\xC3`\xE8\x1B_R`\x04_\xFD[\x15a!nWV[cg\xDB[\x8B`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`@\x1B\x03a!\xBAa!\xAD`:Ta\x10\xBC\x84a!\xA6\x83`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[\x16\x15a!RV[B\x83\x16\x92\x16\x82\x14\x15a!gV[a!\xDEa!\xCCc;\x9A\xCA\0G\x04a\x10\xBCV[`4T`\x01`\x01`@\x1B\x03\x16\x90a\x19\rV[\x90\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10va\"\xB2a\"\x9Ca\"\x0F\x84a\x17\x84V[a\"Oa\" `9Tb\xFF\xFF\xFF\x16\x90V[\x96a\")a\rJV[\x92\x83Ra\"?` \x84\x01\x98\x89\x90b\xFF\xFF\xFF\x16\x90RV[`\x01`\x01`@\x1B\x03\x16`@\x83\x01RV[_``\x82\x01R_`\x80\x82\x01Ra\"\x88\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B`:T\x91`@\x1B\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16\x17`:UV[a\"\x91\x81a*2V[Q\x94Qb\xFF\xFF\xFF\x16\x90V[`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA3V[`\x01`\x01`@\x1B\x03a\"\xE0a!\xAD`:Ta\x10\xBC\x84a!\xA6\x83`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[a\"\xF2a!\xCCc;\x9A\xCA\0G\x04a\x10\xBCV[\x91\x80a#<W[a#-W\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10va\"\xB2a\"\x9Ca\"\x0F\x84a\x17\x84V[c2\xDE\xA9Y`\xE2\x1B_R`\x04_\xFD[P`\x01`\x01`@\x1B\x03\x82\x16\x15a\"\xF9V[\x15a#TWV[c5\xE0\x9E\x9D`\xE0\x1B_R`\x04_\xFD[\x15a#jWV[c\x19X#m`\xE2\x1B_R`\x04_\xFD[\x15a#\x80WV[c.\xAD\xE67`\xE0\x1B_R`\x04_\xFD[` \x81Q\x91\x01Q\x90` \x81\x10a#\xA3WP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x15a#\xB8WV[c\"0Vg`\xE1\x1B_R`\x04_\xFD[_\x19\x81\x14a\x14\nW`\x01\x01\x90V[\x92\x90a$\xD3\x81`\x01`\x01`@\x1B\x03\x96\x93\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x95a\x0F\xDB\x99a$\x19a\x10\x826\x83\x8Aa\x11\x95V[\x96a$N``a$4a\x1Dh\x8B_R`6` R`@_ \x90V[\x01Qa$?\x81a\x05\xD4V[a$H\x81a\x05\xD4V[\x15a#MV[a$n\x8B\x80a$fa$a6\x87\x87a\x11\x95V[a-dV[\x16\x14\x15a#cV[a$\x8E\x8Ba$\x88a\x10\xBCa$\x836\x87\x87a\x11\x95V[a-{V[\x14a#yV[a$\xBAa$\xA4a$\x9F6\x85\x85a\x11\x95V[a-\x92V[a$\xB4a$\xAFa&0V[a#\x8FV[\x14a#\xB1V[a$\xCDa$\xC86\x84\x84a\x11\x95V[a-\xA3V[\x99a\x1F\xD5V[a$\xE6a$\xE1`9Ta#\xC7V[`9UV[`:T`\x01`\x01`@\x1B\x03\x80\x82\x16\x93\x91`@\x1C\x16\x86\x81\x16a%\xBCW[Pa%U\x90a%Pa%\x12a\rYV[d\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x91`\x01`\x01`@\x1B\x03\x88\x16` \x84\x01R`\x01`\x01`@\x1B\x03\x86\x16`@\x84\x01R`\x01``\x84\x01R_R`6` R`@_ \x90V[a\x1C1V[`@Qd\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x90` \x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x82R`\x01`\x01`@\x1B\x03\x92\x83\x16` \x83\x01R\x91\x84\x16\x91\x81\x01\x91\x90\x91R``\x90\xA1\x16a\x16\xD8V[\x92Pa%Ua%\x02V[\x80Q\x91\x90\x82\x90` \x01\x82^\x01_\x81R\x90V[`0\x81Q\x03a&!W_a&\x11a%\xFFa \xC4`\x10` \x95`@Q\x93\x84\x91\x88\x83\x01\x90a%\xC6V[\x86\x81R\x03`\x0F\x19\x81\x01\x84R\x01\x82a\r)V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W_Q\x90V[cO\x8829`\xE1\x1B_R`\x04_\xFD[`@Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01R0``\x1B`,\x82\x01R` \x81Ra\x0F\xDB`@\x82a\r)V[\x81G\x10a&\xF0W_\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xF1a&}a\x170V[P\x15a&\x85WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x91a'Qa\x1F\x19` \x85\x01a\x1F\x13a\x01\0a\x1F\x0B\x83\x89a\x12\x81V[\x92`l\x93a'i\x81Q\x80\x15\x15\x90\x81a\x1F\xC9WPa\x1E\xC4V[` \x92a'u\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a\x1F\xB8W`\x01\x81\x16a'\xBBW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa'\xB5\x90`\x01\x1C\x96a\x13\xFCV[\x95a'{V[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa'\xB5\x90`\x01\x1C\x96a\x13\xFCV[\x80\x15a\x14\nW_\x19\x01\x90V[`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x14a\x14\nW_\x03\x90V[\x80Q_\x95\x86\x95\x90\x94\x93\x90\x92\x90\x91d\xFF\xFF\xFF\xFF\xFF\x16\x92a(>` \x83\x01\x91\x85a(8\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x97a.\x19V[`\x01`\x01`@\x1B\x03\x81\x16\x91`\x01`\x01`@\x1B\x03\x87\x16\x83\x03a(\xEEW[`\x01`\x01`@\x1B\x03\x90\x91\x16\x90R[`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x15a(\x85W[PPP\x92\x91\x90V[a(\xA4\x91\x92\x95P``\x90a(\x9Da$\xE1`9Ta'\xE5V[\x01`\x02\x90RV[`\x01`\x01`@\x1B\x03d\xFF\xFF\xFF\xFF\xFFa(\xBEa\x10\xBC\x88a'\xF1V[\x95\x16\x91\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj_\x80\xA3_\x80\x80a(}V[\x98P\x80a(\xFE\x87a(h\x93a.oV[\x99\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF`@Q\x80a)U\x85\x8A\x8C\x84\x91`@\x91\x94\x93`\x01`\x01`@\x1B\x03\x80\x92d\xFF\xFF\xFF\xFF\xFF``\x87\x01\x98\x16\x86R\x16` \x85\x01R\x16\x91\x01RV[\x03\x90\xA1\x90\x91Pa(ZV[a\x15\xA1`\x80a\x06\xB9\x92\x80Q`<Ub\xFF\xFF\xFF` \x82\x01Q\x16`=T\x90j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0`@\x84\x01Q`\x18\x1B\x16\x91j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17`=U``\x81\x01Q`\x07\x0B`=T\x90`X\x1B`\x01`\x01`@\x1B\x03`X\x1B\x16\x90`\x01`\x01`@\x1B\x03`X\x1B\x19\x16\x17`=U\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a\x14\nWV[\x90c;\x9A\xCA\0\x82\x02\x91\x80\x83\x05c;\x9A\xCA\0\x14\x90\x15\x17\x15a\x14\nWV[`\x01`\xFF\x1B\x81\x14a\x14\nW_\x03\x90V[b\xFF\xFF\xFFa*F` \x83\x01Qb\xFF\xFF\xFF\x16\x90V[\x16a,\x89W`@\x81\x01\x90a*\xD2a\n\xF3a*\xC1a*\xB3a*\xABa*\x8Ea*va\x10\xBC\x89Q`\x01`\x01`@\x1B\x03\x16\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x0B\x90V[a*\xA5a*\x9F``\x89\x01Q`\x07\x0B\x90V[`\x07\x0B\x90V[\x90a)\xE0V[`\x0F\x0Ba*\x06V[\x94Q`\x01`\x01`@\x1B\x03\x16\x90V[`4T`\x01`\x01`@\x1B\x03\x16a\x14HV[`:Ta+\x01\x90`@\x1C`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19`:T\x16\x17`:UV[a+\x1Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19`:T\x16`:UV[a+'_`<U_`=UV[_\x90_\x83\x12a,!W[P`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91\x16\x90\x82;\x15a\x01\xCCW`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x01`\x01`@\x1B\x03\x16`D\x82\x01R\x90_\x90\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02\xD0Wa,\rW[P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8EDa,\x08`\x01`\x01`@\x1B\x03a+\xF6`:T`\x01`\x01`@\x1B\x03\x16\x90V[`@Q\x94\x85R\x16\x92\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2V[\x80a\t\xD1_a,\x1B\x93a\r)V[_a+\xB7V[a,\x83\x91Pa,da,_a\x10\xBCa\x10\xBC\x93a,Y`\x80a,J`4T`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x14HV[a\x16\xD8V[a,~a,ya,s\x86a*\"V[\x83a\x14\x1DV[a\x16\xF4V[a\x14*V[_a+1V[a\x06\xB9\x90a)`V[\x93\x91\x90\x92\x93a,\xAB\x81Q\x80\x15\x15\x90\x81a\x1F\xC9WPa\x1E\xC4V[` \x92a,\xB7\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a-'W`\x01\x81\x16a,\xFDW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa,\xF7\x90`\x01\x1C\x96a\x13\xFCV[\x95a,\xBDV[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa,\xF7\x90`\x01\x1C\x96a\x13\xFCV[P\x94PP\x90PQ\x14\x90V[\x90a-<\x82a\rvV[a-I`@Q\x91\x82a\r)V[\x82\x81R\x80\x92a-Z`\x1F\x19\x91a\rvV[\x01\x90` 6\x91\x017V[\x80Q`\x05\x10\x15a\x13\xA3W`\xC0a\x0F\xDB\x91\x01Qa.\xA3V[\x80Q`\x06\x10\x15a\x13\xA3W`\xE0a\x0F\xDB\x91\x01Qa.\xA3V[\x80Q`\x01\x10\x15a\x13\xA3W`@\x01Q\x90V[\x80Q`\x02\x10\x15a\x13\xA3W``a\x0F\xDB\x91\x01Qa.\xA3V[\x15a-\xC1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[a\x0F\xDB\x92\x91a.ea J`\xC0\x93` `@\x87\x01\x91a.>a\x04\xE0a\x1F\x0B\x85\x8Ba\x12\x81V[a Ea.Vd?\xFF\xFF\xFF\xFF\x88`\x02\x1C\x16\x94\x8Aa\x12\x81V[\x93\x90\x99\x015\x98\x89\x936\x91a\x16\xA2V[`\x06\x1B\x16\x1Ba.\xA3V[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16`\x07\x0B\x91\x16`\x07\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17a\x14\nW\x90V[`\x98\x81\x90\x1Cf\xFF\0\0\0\0\0\0\x16`\xA8\x82\x90\x1Ce\xFF\0\0\0\0\0\x16`\xE8\x83\x90\x1Ca\xFF\0\x16`\xF8\x84\x90\x1C\x17`\xD8\x84\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x84\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x85\x90\x1C\x16\x17\x17\x17\x90g\xFF\0\0\0\0\0\0\0`\x88\x91\x90\x91\x1C\x16\x17\x90V[\x15a/\x0FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x91\x90\x15a/`WP\x90V[\x81Q\x15a/pWP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a/\x94\x90`$\x83\x01\x90a\x18\x80V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 \0*\xB6\xE4\xF0\x0ES\xD7\xEF\x91\xC7\x91\xF8\xE7\xFEJ\xEF2\xFAgLQ,\xF7\xFA%\xF0\xF2\xADwB\x8AdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610022575b3615610018575f80fd5b610020611e77565b005b5f3560e01c8063039157d2146101b15780630b18ff66146101ac5780632340e8d3146101a75780633474aa16146101a25780633f65cf191461019d57806342ecff2a146101985780634665bcda1461019357806347d283721461018e57806352396a5914610189578063587533571461018457806358eaee791461017f5780636c0d2d5a1461017a5780636fcd0e53146101755780637439841f1461017057806374cdd7981461016b57806388676cad146101665780639b4e463414610161578063b522538a1461015c578063c490744214610157578063c4d66de814610152578063d06d55871461014d578063dda3346c14610148578063ee94d67c14610143578063f074ba621461013e5763f28824610361000e57610f83565b610ec9565b610ea3565b610dea565b610c55565b610b5e565b610a4e565b6109ec565b610880565b6107c9565b61077b565b610746565b6106bb565b610642565b6105ff565b610551565b610510565b61048c565b610448565b61041f565b61037a565b610324565b610307565b6102df565b6101de565b600435906001600160401b03821682036101cc57565b5f80fd5b908160409103126101cc5790565b346101cc5760603660031901126101cc576101f76101b6565b6024356001600160401b0381116101cc576102169036906004016101d0565b6044356001600160401b0381116101cc576102359036906004016101d0565b604051635ac86ab760e01b815260066004820152929091906020846024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9384156102d0576100209461029c915f916102a1575b5015610fe9565b610fff565b6102c3915060203d6020116102c9575b6102bb8183610d29565b810190610fc6565b5f610295565b503d6102b1565b610fde565b5f9103126101cc57565b346101cc575f3660031901126101cc576033546040516001600160a01b039091168152602090f35b346101cc575f3660031901126101cc576020603954604051908152f35b346101cc575f3660031901126101cc5760206001600160401b0360345416604051908152f35b9181601f840112156101cc578235916001600160401b0383116101cc576020808501948460051b0101116101cc57565b346101cc5760a03660031901126101cc576103936101b6565b6024356001600160401b0381116101cc576103b29036906004016101d0565b6044356001600160401b0381116101cc576103d190369060040161034a565b6064939193356001600160401b0381116101cc576103f390369060040161034a565b91608435956001600160401b0387116101cc5761041761002097369060040161034a565b9690956112c9565b346101cc575f3660031901126101cc5760206001600160401b03603a5460401c16604051908152f35b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b346101cc575f3660031901126101cc575f60806040516104ab81610cee565b828152826020820152826040820152826060820152015260a06104cc61163a565b6001600160401b036080604051928051845262ffffff6020820151166020850152826040820151166040850152606081015160070b60608501520151166080820152f35b346101cc5760203660031901126101cc576001600160401b036105316101b6565b165f52603b60205260206001600160401b0360405f205416604051908152f35b346101cc575f3660031901126101cc57603e546040516001600160a01b039091168152602090f35b9181601f840112156101cc578235916001600160401b0383116101cc57602083818601950101116101cc57565b60206003198201126101cc57600435906001600160401b0382116101cc576105d091600401610579565b9091565b600311156105de57565b634e487b7160e01b5f52602160045260245ffd5b9060038210156105de5752565b346101cc5761061f61061a610613366105a6565b36916116a2565b6125d8565b5f526036602052602060ff60405f205460c01c1661064060405180926105f2565bf35b346101cc5760203660031901126101cc5760206106656106606101b6565b611784565b604051908152f35b6106b99092919260608060808301956001600160401b0381511684526001600160401b0360208201511660208501526001600160401b03604082015116604085015201519101906105f2565b565b346101cc5760203660031901126101cc576004356106d7611826565b505f52603660205261074260405f2061073660ff604051926106f884610d0e565b546001600160401b03811684526001600160401b038160401c1660208501526001600160401b038160801c16604085015260c01c16606083016111e1565b6040519182918261066d565b0390f35b346101cc5760203660031901126101cc576004355f526036602052602060ff60405f205460c01c1661064060405180926105f2565b346101cc575f3660031901126101cc576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b801515036101cc57565b346101cc5760203660031901126101cc576004356107e6816107bf565b6033546001600160a01b03163314801561086c575b610804906112b3565b604051635ac86ab760e01b815260066004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156102d05761002092610867915f916102a1575015610fe9565b6122b7565b50603e546001600160a01b031633146107fb565b60603660031901126101cc576004356001600160401b0381116101cc576108ab903690600401610579565b6024356001600160401b0381116101cc576108ca903690600401610579565b929060443593610904337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461184a565b6801bc16d674ec80000034036109dd577f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316610946612630565b92813b156101cc576801bc16d674ec8000005f9461097c604051998a96879586946304512a2360e31b86528c8c600488016118a4565b03925af19283156102d0577f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e23936109c3575b506109be604051928392836118e6565b0390a1005b806109d15f6109d793610d29565b806102d5565b5f6109ae565b63049696b360e31b5f5260045ffd5b346101cc57610a1061061a610a00366105a6565b610a08611826565b5036916116a2565b5f52603660205261074260405f2061073660ff604051926106f884610d0e565b6001600160a01b038116036101cc57565b604435906106b982610a30565b346101cc5760403660031901126101cc57600435610a6b81610a30565b602435610aa2337f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03161461184a565b633b9aca008106610b4f5761002091610b0f610af36001600160401b03633b9aca00850416610ad96034546001600160401b031690565b610aee6001600160401b0382168311156118f7565b61190d565b6001600160401b03166001600160401b03196034541617603455565b6040518281526001600160a01b03919091169081907f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e90602090a261265b565b6321ddeb1760e21b5f5260045ffd5b346101cc5760203660031901126101cc57600435610b7b81610a30565b610bc95f5491610baf610b99610b958560ff9060081c1690565b1590565b80948195610c47575b8115610c27575b5061192d565b82610bc0600160ff195f5416175f55565b610c1057611990565b610bcf57005b610bdd61ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989080602081016109be565b610c2261010061ff00195f5416175f55565b611990565b303b15915081610c39575b505f610ba9565b60ff1660011490505f610c32565b600160ff8216109150610ba2565b346101cc5760203660031901126101cc57600435610c7281610a30565b610c8760018060a01b036033541633146119c8565b603e54604080516001600160a01b03808416825290931660208401819052927ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac9190a16001600160a01b03191617603e55005b634e487b7160e01b5f52604160045260245ffd5b60a081019081106001600160401b03821117610d0957604052565b610cda565b608081019081106001600160401b03821117610d0957604052565b90601f801991011681019081106001600160401b03821117610d0957604052565b604051906106b960a083610d29565b604051906106b9608083610d29565b906106b96040519283610d29565b6001600160401b038111610d095760051b60200190565b9080601f830112156101cc578135610da481610d76565b92610db26040519485610d29565b81845260208085019260051b8201019283116101cc57602001905b828210610dda5750505090565b8135815260209182019101610dcd565b346101cc5760603660031901126101cc576004356001600160401b0381116101cc57366023820112156101cc57806004013590610e2682610d76565b91610e346040519384610d29565b8083526024602084019160051b830101913683116101cc57602401905b828210610e8957602435846001600160401b0382116101cc57610e7b610020923690600401610d8d565b610e83610a41565b916119de565b602080918335610e9881610a30565b815201910190610e51565b346101cc575f3660031901126101cc5760206001600160401b03603a5416604051908152f35b346101cc5760403660031901126101cc576004356001600160401b0381116101cc57610ef99036906004016101d0565b6024356001600160401b0381116101cc57610f1890369060040161034a565b604051635ac86ab760e01b815260076004820152929091906020846024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9384156102d05761002094610f7e915f916102a1575015610fe9565b611c99565b346101cc575f3660031901126101cc5760206040516001600160401b037f0000000000000000000000000000000000000000000000000000000000000000168152f35b908160209103126101cc5751610fdb816107bf565b90565b6040513d5f823e3d90fd5b15610ff057565b63840a48d560e01b5f5260045ffd5b604051635ac86ab760e01b815260086004820152919291906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156102d0576111589461106b61113892611148955f916102a1575015610fe9565b61108761108261107b8780611160565b3691611195565b611ea3565b5f52603660205261111c816111176110a160405f206111ed565b956110d86110c86110bc60408a01516001600160401b031690565b6001600160401b031690565b6001600160401b0383161161123f565b6110fa600160608901516110eb816105d4565b6110f4816105d4565b14611255565b61066061111261110d61107b8c80611160565b611eb1565b61126b565b611ef0565b359361115261112b8280611160565b9390926020810190611281565b959094516001600160401b031690565b64ffffffffff1690565b94611fd5565b6106b961217d565b903590601e19813603018212156101cc57018035906001600160401b0382116101cc57602001918160051b360383136101cc57565b9291906111a181610d76565b936111af6040519586610d29565b602085838152019160051b81019283116101cc57905b8282106111d157505050565b81358152602091820191016111c5565b60038210156105de5752565b906106b96040516111fd81610d0e565b606060ff8295546001600160401b03811684526001600160401b038160401c1660208501526001600160401b038160801c16604085015260c01c1691016111e1565b1561124657565b6337e07ffd60e01b5f5260045ffd5b1561125c57565b63d49e19a760e01b5f5260045ffd5b1561127257565b63161ce5ed60e31b5f5260045ffd5b903590601e19813603018212156101cc57018035906001600160401b0382116101cc576020019181360383136101cc57565b156112ba57565b63427a777960e01b5f5260045ffd5b9695949392919060018060a01b036033541633148015611355575b6112ed906112b3565b604051635ac86ab760e01b815260026004820152976020896024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9889156102d0576106b999611350915f916102a1575015610fe9565b611468565b50603e546001600160a01b031633146112e4565b1561137057565b6343714afd60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b91908110156113a35760051b0190565b61137f565b3564ffffffffff811681036101cc5790565b908210156113a3576105d09160051b810190611281565b908210156113a3576105d09160051b810190611160565b634e487b7160e01b5f52601160045260245ffd5b906020820180921161140a57565b6113e8565b906001820180921161140a57565b9190820180921161140a57565b8115611434570490565b634e487b7160e01b5f52601260045260245ffd5b906001600160401b03809116911601906001600160401b03821161140a57565b816111176114af92999599989496979398848b1480611631575b611493909b9a99989796959b611369565b6106606110c86110bc603a546001600160401b039060401c1690565b5f965f965b8088106115d35750505050505050506001600160401b036114e1603a546001600160401b039060401c1690565b16611573575b6033546001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692911690823b156101cc57604051630257884360e21b81526001600160a01b0392909216600483015260248201525f6044820181905290918290606490829084905af180156102d0576115655750565b806109d15f6106b993610d29565b6115ce6115a1611588633b9aca0084046110bc565b603d5460981c6001600160401b0316611448565b611448565b603d805467ffffffffffffffff60981b191660989290921b67ffffffffffffffff60981b16919091179055565b6114e7565b909192939495969761162360019161161d89896116158e61160b8f8b611605611600858e81953599611393565b6113a8565b966113ba565b9290918d8d6113d1565b9490936123d5565b9061141d565b9801969594939291906114b4565b50848714611482565b6040519061164782610cee565b81603c54815260806001600160401b0380603d5462ffffff81166020860152818160181c1660408601528060581c60070b606086015260981c1616910152565b6001600160401b038111610d0957601f01601f191660200190565b9291926116ae82611687565b916116bc6040519384610d29565b8294818452818301116101cc578281602093845f960137010152565b90633b9aca00820291808304633b9aca00149015171561140a57565b90670de0b6b3a7640000820291808304670de0b6b3a7640000149015171561140a57565b600181901b91906001600160ff1b0381160361140a57565b3d1561175a573d9061174182611687565b9161174f6040519384610d29565b82523d5f602084013e565b606090565b1561176657565b63558ad0a360e01b5f5260045ffd5b908160209103126101cc575190565b6001600160401b038116420342811161140a5762017ff4111561181757604080516001600160401b0390921660208084019182528352610fdb925f92839291906117ce9082610d29565b5190720f3df6d732807ef1319fb7b8bb8522d0beac025afa6117ee611730565b908061180d575b6117fe9061175f565b60208082518301019101611775565b50805115156117f5565b637944e66d60e11b5f5260045ffd5b6040519061183382610d0e565b5f6060838281528260208201528260408201520152565b1561185157565b633213a66160e21b5f5260045ffd5b908060209392818452848401375f828201840152601f01601f1916010190565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b969594906118e1936118c56118d3926060979560808c5260808c0191611860565b9089820360208b0152611880565b918783036040890152611860565b930152565b916020610fdb938181520191611860565b156118fe57565b6302c6f54760e21b5f5260045ffd5b906001600160401b03809116911603906001600160401b03821161140a57565b1561193457565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6001600160a01b031680156119b9576bffffffffffffffffffffffff60a01b6033541617603355565b6339b190bb60e11b5f5260045ffd5b156119cf57565b63719f370360e11b5f5260045ffd5b91926119f560018060a01b036033541633146119c8565b604051635ac86ab760e01b8152600560048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa80156102d057611a56915f91611b5c575b5093919315610fe9565b611a638151835114611369565b6040936001600160a01b0316905f5b8151811015611b5457600190611b23875f806001600160a01b03611a968689611b88565b5116611aa2868b611b88565b51828551602081019263a9059cbb60e01b84528c6024830152604482015260448152611acf606482610d29565b611adb87519788610d29565b602087527f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c65646020880152611b11843b1515612f08565b51925af1611b1d611730565b90612f54565b805180611b33575b505001611a72565b81602080611b4893611b4d9501019101610fc6565b612dba565b5f80611b2b565b505050509050565b611b75915060203d6020116102c9576102bb8183610d29565b5f611a4c565b8051156113a35760200190565b80518210156113a35760209160051b010190565b15611ba357565b631a544f4960e01b5f5260045ffd5b91908110156113a35760051b81013590605e19813603018212156101cc570190565b62ffffff16801561140a575f190190565b9060070b9060070b0190677fffffffffffffff198212677fffffffffffffff83131761140a57565b9060038110156105de57815460ff60c01b191660c09190911b60ff60c01b16179055565b8151815460208401516040808601516001600160401b039094166001600160c01b031990931692909217911b67ffffffffffffffff60401b161760809190911b67ffffffffffffffff60801b1617815560609091015160038110156105de576106b991611c0d565b603a5460401c6001600160401b031693929184611cb7811515611b9c565b611cbf61163a565b93611ccb848651612735565b5f935f6020870190608088019360608901915b818110611d44575050505050505050611d3f90611d25611d136106b995966001600160401b03165f52603b60205260405f2090565b9161159c83546001600160401b031690565b6001600160401b03166001600160401b0319825416179055565b612a32565b611d4f81838a611bb2565b8035998d611d6d611d688d5f52603660205260405f2090565b6111ed565b9260016060850151611d7e816105d4565b611d87816105d4565b03611e6a578a611da46110bc60408701516001600160401b031690565b1015611e6a57908392918935611dba9285612809565b918951611dc99062ffffff1690565b611dd290611bd4565b62ffffff168a528b516001600160401b031690611dee91611448565b6001600160401b03168b52875160070b90611e0891611be5565b60070b8752611e1691611448565b9a611e29905f52603660205260405f2090565b90611e3391611c31565b5164ffffffffff16877fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f5f80a36001905b01611cde565b5050995050600190611e64565b7f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf496020604051348152a1565b8051156113a3576020015190565b8051600310156113a35760800151151590565b15611ecb57565b6313717da960e21b5f5260045ffd5b15611ee157565b6309bde33960e01b5f5260045ffd5b9091611f23611f1960208501611f136060611f0b8389611281565b905014611ec4565b85611281565b94359436916116a2565b92600393611f3c81518015159081611fc9575b50611ec4565b602092611f4884610d68565b92835283955b82518711611fb85760018116611f8e5783515f52868301518552848460405f60026107cf195a01fa156101cc57611f889060011c966113fc565b95611f4e565b868301515f5283518552848460405f60026107cf195a01fa156101cc57611f889060011c966113fc565b509450506106b99291505114611eda565b601f169050155f611f36565b9291909493946008820361214357611ff49161107b6105c08814611ec4565b805160011c61200281612d32565b915f5b8281106120f157505060011c805b61204f57509161204561204a949264ffffffffff6120346106b99896611b7b565b519416600b60291b179436916116a2565b612c92565b611eda565b5f5b818110612062575060011c80612013565b60205f6120d061207a61207485611718565b87611b88565b516120c461209861209261208d88611718565b61140f565b89611b88565b51916120b66040519384928884019091604092825260208201520190565b03601f198101835282610d29565b604051918280926125c6565b039060025afa156102d0576001905f516120ea8286611b88565b5201612051565b60205f61212261210961210385611718565b86611b88565b516120c461209861211c61208d88611718565b88611b88565b039060025afa156102d0576001905f5161213c8287611b88565b5201612005565b63200591bd60e01b5f5260045ffd5b1561215957565b62be9bc360e81b5f5260045ffd5b1561216e57565b6367db5b8b60e01b5f5260045ffd5b6001600160401b036121ba6121ad603a546110bc846121a6836001600160401b039060401c1690565b1615612152565b4283169216821415612167565b6121de6121cc633b9aca0047046110bc565b6034546001600160401b03169061190d565b907f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef10766122b261229c61220f84611784565b61224f61222060395462ffffff1690565b96612229610d4a565b92835261223f6020840198899062ffffff169052565b6001600160401b03166040830152565b5f60608201525f60808201526122888567ffffffffffffffff60401b603a549160401b169067ffffffffffffffff60401b191617603a55565b61229181612a32565b51945162ffffff1690565b60405162ffffff90911681529081906020820190565b0390a3565b6001600160401b036122e06121ad603a546110bc846121a6836001600160401b039060401c1690565b6122f26121cc633b9aca0047046110bc565b918061233c575b61232d577f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef10766122b261229c61220f84611784565b6332dea95960e21b5f5260045ffd5b506001600160401b038216156122f9565b1561235457565b6335e09e9d60e01b5f5260045ffd5b1561236a57565b631958236d60e21b5f5260045ffd5b1561238057565b632eade63760e01b5f5260045ffd5b6020815191015190602081106123a3575090565b5f199060200360031b1b1690565b156123b857565b632230566760e11b5f5260045ffd5b5f19811461140a5760010190565b92906124d3816001600160401b0396937f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df95610fdb9961241961108236838a611195565b9661244e6060612434611d688b5f52603660205260405f2090565b015161243f816105d4565b612448816105d4565b1561234d565b61246e8b80612466612461368787611195565b612d64565b161415612363565b61248e8b6124886110bc612483368787611195565b612d7b565b14612379565b6124ba6124a461249f368585611195565b612d92565b6124b46124af612630565b61238f565b146123b1565b6124cd6124c8368484611195565b612da3565b99611fd5565b6124e66124e16039546123c7565b603955565b603a546001600160401b03808216939160401c168681166125bc575b5061255590612550612512610d59565b64ffffffffff85168152916001600160401b03881660208401526001600160401b0386166040840152600160608401525f52603660205260405f2090565b611c31565b60405164ffffffffff821681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c1044144990602090a16040805164ffffffffff9290921682526001600160401b03928316602083015291841691810191909152606090a1166116d8565b9250612555612502565b805191908290602001825e015f815290565b6030815103612621575f6126116125ff6120c46010602095604051938491888301906125c6565b86815203600f19810184520182610d29565b039060025afa156102d0575f5190565b634f88323960e11b5f5260045ffd5b604051600160f81b60208201525f60218201523060601b602c82015260208152610fdb604082610d29565b8147106126f0575f918291829182916001600160a01b03165af161267d611730565b501561268557565b60405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608490fd5b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606490fd5b9091612751611f1960208501611f13610100611f0b8389611281565b92606c9361276981518015159081611fc95750611ec4565b60209261277584610d68565b92835283955b82518711611fb857600181166127bb5783515f52868301518552848460405f60026107cf195a01fa156101cc576127b59060011c966113fc565b9561277b565b868301515f5283518552848460405f60026107cf195a01fa156101cc576127b59060011c966113fc565b801561140a575f190190565b60070b677fffffffffffffff19811461140a575f0390565b80515f9586959094939092909164ffffffffff169261283e60208301918561283884516001600160401b031690565b97612e19565b6001600160401b038116916001600160401b03871683036128ee575b6001600160401b0390911690525b6001600160401b038316604083015215612885575b505050929190565b6128a49192955060609061289d6124e16039546127e5565b0160029052565b6001600160401b0364ffffffffff6128be6110bc886127f1565b951691167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a5f80a35f808061287d565b9850806128fe8761286893612e6f565b997f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df60405180612955858a8c849160409194936001600160401b03809264ffffffffff606087019816865216602085015216910152565b0390a190915061285a565b6115a160806106b9928051603c5562ffffff602082015116603d54906affffffffffffffff000000604084015160181b16916affffffffffffffffffffff19161717603d55606081015160070b603d549060581b6001600160401b0360581b16906001600160401b0360581b191617603d5501516001600160401b031690565b90600f0b90600f0b019060016001607f1b0319821260016001607f1b0383131761140a57565b90633b9aca00820291808305633b9aca00149015171561140a57565b600160ff1b811461140a575f0390565b62ffffff612a46602083015162ffffff1690565b16612c89576040810190612ad2610af3612ac1612ab3612aab612a8e612a766110bc89516001600160401b031690565b6fffffffffffffffffffffffffffffffff16600f0b90565b612aa5612a9f606089015160070b90565b60070b90565b906129e0565b600f0b612a06565b94516001600160401b031690565b6034546001600160401b0316611448565b603a54612b019060401c6001600160401b03166001600160401b03166001600160401b0319603a541617603a55565b612b1a67ffffffffffffffff60401b19603a5416603a55565b612b275f603c555f603d55565b5f905f8312612c21575b506033546001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000811692911690823b156101cc57604051630257884360e21b81526001600160a01b03929092166004830152602482018490526001600160401b03166044820152905f908290606490829084905af180156102d057612c0d575b507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44612c086001600160401b03612bf6603a546001600160401b031690565b60405194855216929081906020820190565b0390a2565b806109d15f612c1b93610d29565b5f612bb7565b612c839150612c64612c5f6110bc6110bc93612c596080612c4a6034546001600160401b031690565b9201516001600160401b031690565b90611448565b6116d8565b612c7e612c79612c7386612a22565b8361141d565b6116f4565b61142a565b5f612b31565b6106b990612960565b9391909293612cab81518015159081611fc95750611ec4565b602092612cb784610d68565b92835283955b82518711612d275760018116612cfd5783515f52868301518552848460405f60026107cf195a01fa156101cc57612cf79060011c966113fc565b95612cbd565b868301515f5283518552848460405f60026107cf195a01fa156101cc57612cf79060011c966113fc565b509450509050511490565b90612d3c82610d76565b612d496040519182610d29565b8281528092612d5a601f1991610d76565b0190602036910137565b8051600510156113a35760c0610fdb910151612ea3565b8051600610156113a35760e0610fdb910151612ea3565b8051600110156113a3576040015190565b8051600210156113a3576060610fdb910151612ea3565b15612dc157565b60405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b6064820152608490fd5b610fdb9291612e6561204a60c09360206040870191612e3e6104e0611f0b858b611281565b612045612e56643fffffffff8860021c16948a611281565b939099013598899336916116a2565b60061b161b612ea3565b906001600160401b0380911660070b911660070b03677fffffffffffffff8113677fffffffffffffff1982121761140a5790565b609881901c66ff0000000000001660a882901c65ff00000000001660e883901c61ff001660f884901c1760d884901c62ff0000161760c884901c63ff000000161764ff0000000060b885901c161717179067ff0000000000000060889190911c161790565b15612f0f57565b60405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e74726163740000006044820152606490fd5b90919015612f60575090565b815115612f705750805190602001fd5b60405162461bcd60e51b815260206004820152908190612f94906024830190611880565b0390fdfea2646970667358221220002ab6e4f00e53d7ef91c791f8e7fe4aef32fa674c512cf7fa25f0f2ad77428a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\"W[6\x15a\0\x18W_\x80\xFD[a\0 a\x1EwV[\0[_5`\xE0\x1C\x80c\x03\x91W\xD2\x14a\x01\xB1W\x80c\x0B\x18\xFFf\x14a\x01\xACW\x80c#@\xE8\xD3\x14a\x01\xA7W\x80c4t\xAA\x16\x14a\x01\xA2W\x80c?e\xCF\x19\x14a\x01\x9DW\x80cB\xEC\xFF*\x14a\x01\x98W\x80cFe\xBC\xDA\x14a\x01\x93W\x80cG\xD2\x83r\x14a\x01\x8EW\x80cR9jY\x14a\x01\x89W\x80cXu3W\x14a\x01\x84W\x80cX\xEA\xEEy\x14a\x01\x7FW\x80cl\r-Z\x14a\x01zW\x80co\xCD\x0ES\x14a\x01uW\x80ct9\x84\x1F\x14a\x01pW\x80ct\xCD\xD7\x98\x14a\x01kW\x80c\x88gl\xAD\x14a\x01fW\x80c\x9BNF4\x14a\x01aW\x80c\xB5\"S\x8A\x14a\x01\\W\x80c\xC4\x90tB\x14a\x01WW\x80c\xC4\xD6m\xE8\x14a\x01RW\x80c\xD0mU\x87\x14a\x01MW\x80c\xDD\xA34l\x14a\x01HW\x80c\xEE\x94\xD6|\x14a\x01CW\x80c\xF0t\xBAb\x14a\x01>Wc\xF2\x88$a\x03a\0\x0EWa\x0F\x83V[a\x0E\xC9V[a\x0E\xA3V[a\r\xEAV[a\x0CUV[a\x0B^V[a\nNV[a\t\xECV[a\x08\x80V[a\x07\xC9V[a\x07{V[a\x07FV[a\x06\xBBV[a\x06BV[a\x05\xFFV[a\x05QV[a\x05\x10V[a\x04\x8CV[a\x04HV[a\x04\x1FV[a\x03zV[a\x03$V[a\x03\x07V[a\x02\xDFV[a\x01\xDEV[`\x045\x90`\x01`\x01`@\x1B\x03\x82\x16\x82\x03a\x01\xCCWV[_\x80\xFD[\x90\x81`@\x91\x03\x12a\x01\xCCW\x90V[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCWa\x01\xF7a\x01\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x02\x16\x906\x90`\x04\x01a\x01\xD0V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x025\x906\x90`\x04\x01a\x01\xD0V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01R\x92\x90\x91\x90` \x84`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x02\xD0Wa\0 \x94a\x02\x9C\x91_\x91a\x02\xA1W[P\x15a\x0F\xE9V[a\x0F\xFFV[a\x02\xC3\x91P` =` \x11a\x02\xC9W[a\x02\xBB\x81\x83a\r)V[\x81\x01\x90a\x0F\xC6V[_a\x02\x95V[P=a\x02\xB1V[a\x0F\xDEV[_\x91\x03\x12a\x01\xCCWV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `9T`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`4T\x16`@Q\x90\x81R\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x01\xCCWV[4a\x01\xCCW`\xA06`\x03\x19\x01\x12a\x01\xCCWa\x03\x93a\x01\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xB2\x906\x90`\x04\x01a\x01\xD0V[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xD1\x906\x90`\x04\x01a\x03JV[`d\x93\x91\x935`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x03\xF3\x906\x90`\x04\x01a\x03JV[\x91`\x845\x95`\x01`\x01`@\x1B\x03\x87\x11a\x01\xCCWa\x04\x17a\0 \x976\x90`\x04\x01a\x03JV[\x96\x90\x95a\x12\xC9V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`:T`@\x1C\x16`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW_`\x80`@Qa\x04\xAB\x81a\x0C\xEEV[\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R`\xA0a\x04\xCCa\x16:V[`\x01`\x01`@\x1B\x03`\x80`@Q\x92\x80Q\x84Rb\xFF\xFF\xFF` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q`\x07\x0B``\x85\x01R\x01Q\x16`\x80\x82\x01R\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x01`\x01`@\x1B\x03a\x051a\x01\xB6V[\x16_R`;` R` `\x01`\x01`@\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`>T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x91\x81`\x1F\x84\x01\x12\x15a\x01\xCCW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x01\xCCW` \x83\x81\x86\x01\x95\x01\x01\x11a\x01\xCCWV[` `\x03\x19\x82\x01\x12a\x01\xCCW`\x045\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x05\xD0\x91`\x04\x01a\x05yV[\x90\x91V[`\x03\x11\x15a\x05\xDEWV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x90`\x03\x82\x10\x15a\x05\xDEWRV[4a\x01\xCCWa\x06\x1Fa\x06\x1Aa\x06\x136a\x05\xA6V[6\x91a\x16\xA2V[a%\xD8V[_R`6` R` `\xFF`@_ T`\xC0\x1C\x16a\x06@`@Q\x80\x92a\x05\xF2V[\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW` a\x06ea\x06`a\x01\xB6V[a\x17\x84V[`@Q\x90\x81R\xF3[a\x06\xB9\x90\x92\x91\x92``\x80`\x80\x83\x01\x95`\x01`\x01`@\x1B\x03\x81Q\x16\x84R`\x01`\x01`@\x1B\x03` \x82\x01Q\x16` \x85\x01R`\x01`\x01`@\x1B\x03`@\x82\x01Q\x16`@\x85\x01R\x01Q\x91\x01\x90a\x05\xF2V[V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x06\xD7a\x18&V[P_R`6` Ra\x07B`@_ a\x076`\xFF`@Q\x92a\x06\xF8\x84a\r\x0EV[T`\x01`\x01`@\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03\x81`@\x1C\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16`@\x85\x01R`\xC0\x1C\x16``\x83\x01a\x11\xE1V[`@Q\x91\x82\x91\x82a\x06mV[\x03\x90\xF3[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045_R`6` R` `\xFF`@_ T`\xC0\x1C\x16a\x06@`@Q\x80\x92a\x05\xF2V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x01\xCCWV[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x07\xE6\x81a\x07\xBFV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15a\x08lW[a\x08\x04\x90a\x12\xB3V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x02\xD0Wa\0 \x92a\x08g\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\"\xB7V[P`>T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xFBV[``6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x08\xAB\x906\x90`\x04\x01a\x05yV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x08\xCA\x906\x90`\x04\x01a\x05yV[\x92\x90`D5\x93a\t\x043\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18JV[h\x01\xBC\x16\xD6t\xEC\x80\0\x004\x03a\t\xDDW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a\tFa&0V[\x92\x81;\x15a\x01\xCCWh\x01\xBC\x16\xD6t\xEC\x80\0\0_\x94a\t|`@Q\x99\x8A\x96\x87\x95\x86\x94c\x04Q*#`\xE3\x1B\x86R\x8C\x8C`\x04\x88\x01a\x18\xA4V[\x03\x92Z\xF1\x92\x83\x15a\x02\xD0W\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x93a\t\xC3W[Pa\t\xBE`@Q\x92\x83\x92\x83a\x18\xE6V[\x03\x90\xA1\0[\x80a\t\xD1_a\t\xD7\x93a\r)V[\x80a\x02\xD5V[_a\t\xAEV[c\x04\x96\x96\xB3`\xE3\x1B_R`\x04_\xFD[4a\x01\xCCWa\n\x10a\x06\x1Aa\n\x006a\x05\xA6V[a\n\x08a\x18&V[P6\x91a\x16\xA2V[_R`6` Ra\x07B`@_ a\x076`\xFF`@Q\x92a\x06\xF8\x84a\r\x0EV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xCCWV[`D5\x90a\x06\xB9\x82a\n0V[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW`\x045a\nk\x81a\n0V[`$5a\n\xA23\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18JV[c;\x9A\xCA\0\x81\x06a\x0BOWa\0 \x91a\x0B\x0Fa\n\xF3`\x01`\x01`@\x1B\x03c;\x9A\xCA\0\x85\x04\x16a\n\xD9`4T`\x01`\x01`@\x1B\x03\x16\x90V[a\n\xEE`\x01`\x01`@\x1B\x03\x82\x16\x83\x11\x15a\x18\xF7V[a\x19\rV[`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19`4T\x16\x17`4UV[`@Q\x82\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x90\x81\x90\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x90` \x90\xA2a&[V[c!\xDD\xEB\x17`\xE2\x1B_R`\x04_\xFD[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x0B{\x81a\n0V[a\x0B\xC9_T\x91a\x0B\xAFa\x0B\x99a\x0B\x95\x85`\xFF\x90`\x08\x1C\x16\x90V[\x15\x90V[\x80\x94\x81\x95a\x0CGW[\x81\x15a\x0C'W[Pa\x19-V[\x82a\x0B\xC0`\x01`\xFF\x19_T\x16\x17_UV[a\x0C\x10Wa\x19\x90V[a\x0B\xCFW\0[a\x0B\xDDa\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\t\xBEV[a\x0C\"a\x01\0a\xFF\0\x19_T\x16\x17_UV[a\x19\x90V[0;\x15\x91P\x81a\x0C9W[P_a\x0B\xA9V[`\xFF\x16`\x01\x14\x90P_a\x0C2V[`\x01`\xFF\x82\x16\x10\x91Pa\x0B\xA2V[4a\x01\xCCW` 6`\x03\x19\x01\x12a\x01\xCCW`\x045a\x0Cr\x81a\n0V[a\x0C\x87`\x01\x80`\xA0\x1B\x03`3T\x163\x14a\x19\xC8V[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x80\x84\x16\x82R\x90\x93\x16` \x84\x01\x81\x90R\x92\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`>U\0[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`\xA0\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[a\x0C\xDAV[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\r\tW`@RV[`@Q\x90a\x06\xB9`\xA0\x83a\r)V[`@Q\x90a\x06\xB9`\x80\x83a\r)V[\x90a\x06\xB9`@Q\x92\x83a\r)V[`\x01`\x01`@\x1B\x03\x81\x11a\r\tW`\x05\x1B` \x01\x90V[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xCCW\x815a\r\xA4\x81a\rvV[\x92a\r\xB2`@Q\x94\x85a\r)V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x01\xCCW` \x01\x90[\x82\x82\x10a\r\xDAWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\r\xCDV[4a\x01\xCCW``6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCW6`#\x82\x01\x12\x15a\x01\xCCW\x80`\x04\x015\x90a\x0E&\x82a\rvV[\x91a\x0E4`@Q\x93\x84a\r)V[\x80\x83R`$` \x84\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x01\xCCW`$\x01\x90[\x82\x82\x10a\x0E\x89W`$5\x84`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCWa\x0E{a\0 \x926\x90`\x04\x01a\r\x8DV[a\x0E\x83a\nAV[\x91a\x19\xDEV[` \x80\x91\x835a\x0E\x98\x81a\n0V[\x81R\x01\x91\x01\x90a\x0EQV[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `\x01`\x01`@\x1B\x03`:T\x16`@Q\x90\x81R\xF3[4a\x01\xCCW`@6`\x03\x19\x01\x12a\x01\xCCW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x0E\xF9\x906\x90`\x04\x01a\x01\xD0V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x01\xCCWa\x0F\x18\x906\x90`\x04\x01a\x03JV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01R\x92\x90\x91\x90` \x84`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x93\x84\x15a\x02\xD0Wa\0 \x94a\x0F~\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x1C\x99V[4a\x01\xCCW_6`\x03\x19\x01\x12a\x01\xCCW` `@Q`\x01`\x01`@\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[\x90\x81` \x91\x03\x12a\x01\xCCWQa\x0F\xDB\x81a\x07\xBFV[\x90V[`@Q=_\x82>=\x90\xFD[\x15a\x0F\xF0WV[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01R\x91\x92\x91\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xD0Wa\x11X\x94a\x10ka\x118\x92a\x11H\x95_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x10\x87a\x10\x82a\x10{\x87\x80a\x11`V[6\x91a\x11\x95V[a\x1E\xA3V[_R`6` Ra\x11\x1C\x81a\x11\x17a\x10\xA1`@_ a\x11\xEDV[\x95a\x10\xD8a\x10\xC8a\x10\xBC`@\x8A\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x83\x16\x11a\x12?V[a\x10\xFA`\x01``\x89\x01Qa\x10\xEB\x81a\x05\xD4V[a\x10\xF4\x81a\x05\xD4V[\x14a\x12UV[a\x06`a\x11\x12a\x11\ra\x10{\x8C\x80a\x11`V[a\x1E\xB1V[a\x12kV[a\x1E\xF0V[5\x93a\x11Ra\x11+\x82\x80a\x11`V[\x93\x90\x92` \x81\x01\x90a\x12\x81V[\x95\x90\x94Q`\x01`\x01`@\x1B\x03\x16\x90V[d\xFF\xFF\xFF\xFF\xFF\x16\x90V[\x94a\x1F\xD5V[a\x06\xB9a!}V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW` \x01\x91\x81`\x05\x1B6\x03\x83\x13a\x01\xCCWV[\x92\x91\x90a\x11\xA1\x81a\rvV[\x93a\x11\xAF`@Q\x95\x86a\r)V[` \x85\x83\x81R\x01\x91`\x05\x1B\x81\x01\x92\x83\x11a\x01\xCCW\x90[\x82\x82\x10a\x11\xD1WPPPV[\x815\x81R` \x91\x82\x01\x91\x01a\x11\xC5V[`\x03\x82\x10\x15a\x05\xDEWRV[\x90a\x06\xB9`@Qa\x11\xFD\x81a\r\x0EV[```\xFF\x82\x95T`\x01`\x01`@\x1B\x03\x81\x16\x84R`\x01`\x01`@\x1B\x03\x81`@\x1C\x16` \x85\x01R`\x01`\x01`@\x1B\x03\x81`\x80\x1C\x16`@\x85\x01R`\xC0\x1C\x16\x91\x01a\x11\xE1V[\x15a\x12FWV[c7\xE0\x7F\xFD`\xE0\x1B_R`\x04_\xFD[\x15a\x12\\WV[c\xD4\x9E\x19\xA7`\xE0\x1B_R`\x04_\xFD[\x15a\x12rWV[c\x16\x1C\xE5\xED`\xE3\x1B_R`\x04_\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x01\xCCW` \x01\x91\x816\x03\x83\x13a\x01\xCCWV[\x15a\x12\xBAWV[cBzwy`\xE0\x1B_R`\x04_\xFD[\x96\x95\x94\x93\x92\x91\x90`\x01\x80`\xA0\x1B\x03`3T\x163\x14\x80\x15a\x13UW[a\x12\xED\x90a\x12\xB3V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01R\x97` \x89`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x98\x89\x15a\x02\xD0Wa\x06\xB9\x99a\x13P\x91_\x91a\x02\xA1WP\x15a\x0F\xE9V[a\x14hV[P`>T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x12\xE4V[\x15a\x13pWV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a\x13\xA3W`\x05\x1B\x01\x90V[a\x13\x7FV[5d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x01\xCCW\x90V[\x90\x82\x10\x15a\x13\xA3Wa\x05\xD0\x91`\x05\x1B\x81\x01\x90a\x12\x81V[\x90\x82\x10\x15a\x13\xA3Wa\x05\xD0\x91`\x05\x1B\x81\x01\x90a\x11`V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x90` \x82\x01\x80\x92\x11a\x14\nWV[a\x13\xE8V[\x90`\x01\x82\x01\x80\x92\x11a\x14\nWV[\x91\x90\x82\x01\x80\x92\x11a\x14\nWV[\x81\x15a\x144W\x04\x90V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x01\x90`\x01`\x01`@\x1B\x03\x82\x11a\x14\nWV[\x81a\x11\x17a\x14\xAF\x92\x99\x95\x99\x98\x94\x96\x97\x93\x98\x84\x8B\x14\x80a\x161W[a\x14\x93\x90\x9B\x9A\x99\x98\x97\x96\x95\x9Ba\x13iV[a\x06`a\x10\xC8a\x10\xBC`:T`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[_\x96_\x96[\x80\x88\x10a\x15\xD3WPPPPPPPP`\x01`\x01`@\x1B\x03a\x14\xE1`:T`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[\x16a\x15sW[`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91\x16\x90\x82;\x15a\x01\xCCW`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01R_`D\x82\x01\x81\x90R\x90\x91\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02\xD0Wa\x15eWPV[\x80a\t\xD1_a\x06\xB9\x93a\r)V[a\x15\xCEa\x15\xA1a\x15\x88c;\x9A\xCA\0\x84\x04a\x10\xBCV[`=T`\x98\x1C`\x01`\x01`@\x1B\x03\x16a\x14HV[a\x14HV[`=\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16`\x98\x92\x90\x92\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x16\x91\x90\x91\x17\x90UV[a\x14\xE7V[\x90\x91\x92\x93\x94\x95\x96\x97a\x16#`\x01\x91a\x16\x1D\x89\x89a\x16\x15\x8Ea\x16\x0B\x8F\x8Ba\x16\x05a\x16\0\x85\x8E\x81\x955\x99a\x13\x93V[a\x13\xA8V[\x96a\x13\xBAV[\x92\x90\x91\x8D\x8Da\x13\xD1V[\x94\x90\x93a#\xD5V[\x90a\x14\x1DV[\x98\x01\x96\x95\x94\x93\x92\x91\x90a\x14\xB4V[P\x84\x87\x14a\x14\x82V[`@Q\x90a\x16G\x82a\x0C\xEEV[\x81`<T\x81R`\x80`\x01`\x01`@\x1B\x03\x80`=Tb\xFF\xFF\xFF\x81\x16` \x86\x01R\x81\x81`\x18\x1C\x16`@\x86\x01R\x80`X\x1C`\x07\x0B``\x86\x01R`\x98\x1C\x16\x16\x91\x01RV[`\x01`\x01`@\x1B\x03\x81\x11a\r\tW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x16\xAE\x82a\x16\x87V[\x91a\x16\xBC`@Q\x93\x84a\r)V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xCCW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90c;\x9A\xCA\0\x82\x02\x91\x80\x83\x04c;\x9A\xCA\0\x14\x90\x15\x17\x15a\x14\nWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x82\x02\x91\x80\x83\x04g\r\xE0\xB6\xB3\xA7d\0\0\x14\x90\x15\x17\x15a\x14\nWV[`\x01\x81\x90\x1B\x91\x90`\x01`\x01`\xFF\x1B\x03\x81\x16\x03a\x14\nWV[=\x15a\x17ZW=\x90a\x17A\x82a\x16\x87V[\x91a\x17O`@Q\x93\x84a\r)V[\x82R=_` \x84\x01>V[``\x90V[\x15a\x17fWV[cU\x8A\xD0\xA3`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x01\xCCWQ\x90V[`\x01`\x01`@\x1B\x03\x81\x16B\x03B\x81\x11a\x14\nWb\x01\x7F\xF4\x11\x15a\x18\x17W`@\x80Q`\x01`\x01`@\x1B\x03\x90\x92\x16` \x80\x84\x01\x91\x82R\x83Ra\x0F\xDB\x92_\x92\x83\x92\x91\x90a\x17\xCE\x90\x82a\r)V[Q\x90r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02Z\xFAa\x17\xEEa\x170V[\x90\x80a\x18\rW[a\x17\xFE\x90a\x17_V[` \x80\x82Q\x83\x01\x01\x91\x01a\x17uV[P\x80Q\x15\x15a\x17\xF5V[cyD\xE6m`\xE1\x1B_R`\x04_\xFD[`@Q\x90a\x183\x82a\r\x0EV[_``\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x01RV[\x15a\x18QWV[c2\x13\xA6a`\xE2\x1B_R`\x04_\xFD[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x96\x95\x94\x90a\x18\xE1\x93a\x18\xC5a\x18\xD3\x92``\x97\x95`\x80\x8CR`\x80\x8C\x01\x91a\x18`V[\x90\x89\x82\x03` \x8B\x01Ra\x18\x80V[\x91\x87\x83\x03`@\x89\x01Ra\x18`V[\x93\x01RV[\x91` a\x0F\xDB\x93\x81\x81R\x01\x91a\x18`V[\x15a\x18\xFEWV[c\x02\xC6\xF5G`\xE2\x1B_R`\x04_\xFD[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01`@\x1B\x03\x82\x11a\x14\nWV[\x15a\x194WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x19\xB9Wk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B`3T\x16\x17`3UV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[\x15a\x19\xCFWV[cq\x9F7\x03`\xE1\x1B_R`\x04_\xFD[\x91\x92a\x19\xF5`\x01\x80`\xA0\x1B\x03`3T\x163\x14a\x19\xC8V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xD0Wa\x1AV\x91_\x91a\x1B\\W[P\x93\x91\x93\x15a\x0F\xE9V[a\x1Ac\x81Q\x83Q\x14a\x13iV[`@\x93`\x01`\x01`\xA0\x1B\x03\x16\x90_[\x81Q\x81\x10\x15a\x1BTW`\x01\x90a\x1B#\x87_\x80`\x01`\x01`\xA0\x1B\x03a\x1A\x96\x86\x89a\x1B\x88V[Q\x16a\x1A\xA2\x86\x8Ba\x1B\x88V[Q\x82\x85Q` \x81\x01\x92c\xA9\x05\x9C\xBB`\xE0\x1B\x84R\x8C`$\x83\x01R`D\x82\x01R`D\x81Ra\x1A\xCF`d\x82a\r)V[a\x1A\xDB\x87Q\x97\x88a\r)V[` \x87R\x7FSafeERC20: low-level call failed` \x88\x01Ra\x1B\x11\x84;\x15\x15a/\x08V[Q\x92Z\xF1a\x1B\x1Da\x170V[\x90a/TV[\x80Q\x80a\x1B3W[PP\x01a\x1ArV[\x81` \x80a\x1BH\x93a\x1BM\x95\x01\x01\x91\x01a\x0F\xC6V[a-\xBAV[_\x80a\x1B+V[PPPP\x90PV[a\x1Bu\x91P` =` \x11a\x02\xC9Wa\x02\xBB\x81\x83a\r)V[_a\x1ALV[\x80Q\x15a\x13\xA3W` \x01\x90V[\x80Q\x82\x10\x15a\x13\xA3W` \x91`\x05\x1B\x01\x01\x90V[\x15a\x1B\xA3WV[c\x1ATOI`\xE0\x1B_R`\x04_\xFD[\x91\x90\x81\x10\x15a\x13\xA3W`\x05\x1B\x81\x015\x90`^\x19\x816\x03\x01\x82\x12\x15a\x01\xCCW\x01\x90V[b\xFF\xFF\xFF\x16\x80\x15a\x14\nW_\x19\x01\x90V[\x90`\x07\x0B\x90`\x07\x0B\x01\x90g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x13\x17a\x14\nWV[\x90`\x03\x81\x10\x15a\x05\xDEW\x81T`\xFF`\xC0\x1B\x19\x16`\xC0\x91\x90\x91\x1B`\xFF`\xC0\x1B\x16\x17\x90UV[\x81Q\x81T` \x84\x01Q`@\x80\x86\x01Q`\x01`\x01`@\x1B\x03\x90\x94\x16`\x01`\x01`\xC0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x16\x17`\x80\x91\x90\x91\x1Bg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x16\x17\x81U``\x90\x91\x01Q`\x03\x81\x10\x15a\x05\xDEWa\x06\xB9\x91a\x1C\rV[`:T`@\x1C`\x01`\x01`@\x1B\x03\x16\x93\x92\x91\x84a\x1C\xB7\x81\x15\x15a\x1B\x9CV[a\x1C\xBFa\x16:V[\x93a\x1C\xCB\x84\x86Qa'5V[_\x93_` \x87\x01\x90`\x80\x88\x01\x93``\x89\x01\x91[\x81\x81\x10a\x1DDWPPPPPPPPa\x1D?\x90a\x1D%a\x1D\x13a\x06\xB9\x95\x96`\x01`\x01`@\x1B\x03\x16_R`;` R`@_ \x90V[\x91a\x15\x9C\x83T`\x01`\x01`@\x1B\x03\x16\x90V[`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19\x82T\x16\x17\x90UV[a*2V[a\x1DO\x81\x83\x8Aa\x1B\xB2V[\x805\x99\x8Da\x1Dma\x1Dh\x8D_R`6` R`@_ \x90V[a\x11\xEDV[\x92`\x01``\x85\x01Qa\x1D~\x81a\x05\xD4V[a\x1D\x87\x81a\x05\xD4V[\x03a\x1EjW\x8Aa\x1D\xA4a\x10\xBC`@\x87\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x10\x15a\x1EjW\x90\x83\x92\x91\x895a\x1D\xBA\x92\x85a(\tV[\x91\x89Qa\x1D\xC9\x90b\xFF\xFF\xFF\x16\x90V[a\x1D\xD2\x90a\x1B\xD4V[b\xFF\xFF\xFF\x16\x8AR\x8BQ`\x01`\x01`@\x1B\x03\x16\x90a\x1D\xEE\x91a\x14HV[`\x01`\x01`@\x1B\x03\x16\x8BR\x87Q`\x07\x0B\x90a\x1E\x08\x91a\x1B\xE5V[`\x07\x0B\x87Ra\x1E\x16\x91a\x14HV[\x9Aa\x1E)\x90_R`6` R`@_ \x90V[\x90a\x1E3\x91a\x1C1V[Qd\xFF\xFF\xFF\xFF\xFF\x16\x87\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?_\x80\xA3`\x01\x90[\x01a\x1C\xDEV[PP\x99PP`\x01\x90a\x1EdV[\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI` `@Q4\x81R\xA1V[\x80Q\x15a\x13\xA3W` \x01Q\x90V[\x80Q`\x03\x10\x15a\x13\xA3W`\x80\x01Q\x15\x15\x90V[\x15a\x1E\xCBWV[c\x13q}\xA9`\xE2\x1B_R`\x04_\xFD[\x15a\x1E\xE1WV[c\t\xBD\xE39`\xE0\x1B_R`\x04_\xFD[\x90\x91a\x1F#a\x1F\x19` \x85\x01a\x1F\x13``a\x1F\x0B\x83\x89a\x12\x81V[\x90P\x14a\x1E\xC4V[\x85a\x12\x81V[\x945\x946\x91a\x16\xA2V[\x92`\x03\x93a\x1F<\x81Q\x80\x15\x15\x90\x81a\x1F\xC9W[Pa\x1E\xC4V[` \x92a\x1FH\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a\x1F\xB8W`\x01\x81\x16a\x1F\x8EW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa\x1F\x88\x90`\x01\x1C\x96a\x13\xFCV[\x95a\x1FNV[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa\x1F\x88\x90`\x01\x1C\x96a\x13\xFCV[P\x94PPa\x06\xB9\x92\x91PQ\x14a\x1E\xDAV[`\x1F\x16\x90P\x15_a\x1F6V[\x92\x91\x90\x94\x93\x94`\x08\x82\x03a!CWa\x1F\xF4\x91a\x10{a\x05\xC0\x88\x14a\x1E\xC4V[\x80Q`\x01\x1Ca \x02\x81a-2V[\x91_[\x82\x81\x10a \xF1WPP`\x01\x1C\x80[a OWP\x91a Ea J\x94\x92d\xFF\xFF\xFF\xFF\xFFa 4a\x06\xB9\x98\x96a\x1B{V[Q\x94\x16`\x0B`)\x1B\x17\x946\x91a\x16\xA2V[a,\x92V[a\x1E\xDAV[_[\x81\x81\x10a bWP`\x01\x1C\x80a \x13V[` _a \xD0a za t\x85a\x17\x18V[\x87a\x1B\x88V[Qa \xC4a \x98a \x92a \x8D\x88a\x17\x18V[a\x14\x0FV[\x89a\x1B\x88V[Q\x91a \xB6`@Q\x93\x84\x92\x88\x84\x01\x90\x91`@\x92\x82R` \x82\x01R\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\r)V[`@Q\x91\x82\x80\x92a%\xC6V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W`\x01\x90_Qa \xEA\x82\x86a\x1B\x88V[R\x01a QV[` _a!\"a!\ta!\x03\x85a\x17\x18V[\x86a\x1B\x88V[Qa \xC4a \x98a!\x1Ca \x8D\x88a\x17\x18V[\x88a\x1B\x88V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W`\x01\x90_Qa!<\x82\x87a\x1B\x88V[R\x01a \x05V[c \x05\x91\xBD`\xE0\x1B_R`\x04_\xFD[\x15a!YWV[b\xBE\x9B\xC3`\xE8\x1B_R`\x04_\xFD[\x15a!nWV[cg\xDB[\x8B`\xE0\x1B_R`\x04_\xFD[`\x01`\x01`@\x1B\x03a!\xBAa!\xAD`:Ta\x10\xBC\x84a!\xA6\x83`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[\x16\x15a!RV[B\x83\x16\x92\x16\x82\x14\x15a!gV[a!\xDEa!\xCCc;\x9A\xCA\0G\x04a\x10\xBCV[`4T`\x01`\x01`@\x1B\x03\x16\x90a\x19\rV[\x90\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10va\"\xB2a\"\x9Ca\"\x0F\x84a\x17\x84V[a\"Oa\" `9Tb\xFF\xFF\xFF\x16\x90V[\x96a\")a\rJV[\x92\x83Ra\"?` \x84\x01\x98\x89\x90b\xFF\xFF\xFF\x16\x90RV[`\x01`\x01`@\x1B\x03\x16`@\x83\x01RV[_``\x82\x01R_`\x80\x82\x01Ra\"\x88\x85g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B`:T\x91`@\x1B\x16\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16\x17`:UV[a\"\x91\x81a*2V[Q\x94Qb\xFF\xFF\xFF\x16\x90V[`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA3V[`\x01`\x01`@\x1B\x03a\"\xE0a!\xAD`:Ta\x10\xBC\x84a!\xA6\x83`\x01`\x01`@\x1B\x03\x90`@\x1C\x16\x90V[a\"\xF2a!\xCCc;\x9A\xCA\0G\x04a\x10\xBCV[\x91\x80a#<W[a#-W\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10va\"\xB2a\"\x9Ca\"\x0F\x84a\x17\x84V[c2\xDE\xA9Y`\xE2\x1B_R`\x04_\xFD[P`\x01`\x01`@\x1B\x03\x82\x16\x15a\"\xF9V[\x15a#TWV[c5\xE0\x9E\x9D`\xE0\x1B_R`\x04_\xFD[\x15a#jWV[c\x19X#m`\xE2\x1B_R`\x04_\xFD[\x15a#\x80WV[c.\xAD\xE67`\xE0\x1B_R`\x04_\xFD[` \x81Q\x91\x01Q\x90` \x81\x10a#\xA3WP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[\x15a#\xB8WV[c\"0Vg`\xE1\x1B_R`\x04_\xFD[_\x19\x81\x14a\x14\nW`\x01\x01\x90V[\x92\x90a$\xD3\x81`\x01`\x01`@\x1B\x03\x96\x93\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x95a\x0F\xDB\x99a$\x19a\x10\x826\x83\x8Aa\x11\x95V[\x96a$N``a$4a\x1Dh\x8B_R`6` R`@_ \x90V[\x01Qa$?\x81a\x05\xD4V[a$H\x81a\x05\xD4V[\x15a#MV[a$n\x8B\x80a$fa$a6\x87\x87a\x11\x95V[a-dV[\x16\x14\x15a#cV[a$\x8E\x8Ba$\x88a\x10\xBCa$\x836\x87\x87a\x11\x95V[a-{V[\x14a#yV[a$\xBAa$\xA4a$\x9F6\x85\x85a\x11\x95V[a-\x92V[a$\xB4a$\xAFa&0V[a#\x8FV[\x14a#\xB1V[a$\xCDa$\xC86\x84\x84a\x11\x95V[a-\xA3V[\x99a\x1F\xD5V[a$\xE6a$\xE1`9Ta#\xC7V[`9UV[`:T`\x01`\x01`@\x1B\x03\x80\x82\x16\x93\x91`@\x1C\x16\x86\x81\x16a%\xBCW[Pa%U\x90a%Pa%\x12a\rYV[d\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R\x91`\x01`\x01`@\x1B\x03\x88\x16` \x84\x01R`\x01`\x01`@\x1B\x03\x86\x16`@\x84\x01R`\x01``\x84\x01R_R`6` R`@_ \x90V[a\x1C1V[`@Qd\xFF\xFF\xFF\xFF\xFF\x82\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x90` \x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x82R`\x01`\x01`@\x1B\x03\x92\x83\x16` \x83\x01R\x91\x84\x16\x91\x81\x01\x91\x90\x91R``\x90\xA1\x16a\x16\xD8V[\x92Pa%Ua%\x02V[\x80Q\x91\x90\x82\x90` \x01\x82^\x01_\x81R\x90V[`0\x81Q\x03a&!W_a&\x11a%\xFFa \xC4`\x10` \x95`@Q\x93\x84\x91\x88\x83\x01\x90a%\xC6V[\x86\x81R\x03`\x0F\x19\x81\x01\x84R\x01\x82a\r)V[\x03\x90`\x02Z\xFA\x15a\x02\xD0W_Q\x90V[cO\x8829`\xE1\x1B_R`\x04_\xFD[`@Q`\x01`\xF8\x1B` \x82\x01R_`!\x82\x01R0``\x1B`,\x82\x01R` \x81Ra\x0F\xDB`@\x82a\r)V[\x81G\x10a&\xF0W_\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xF1a&}a\x170V[P\x15a&\x85WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x91a'Qa\x1F\x19` \x85\x01a\x1F\x13a\x01\0a\x1F\x0B\x83\x89a\x12\x81V[\x92`l\x93a'i\x81Q\x80\x15\x15\x90\x81a\x1F\xC9WPa\x1E\xC4V[` \x92a'u\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a\x1F\xB8W`\x01\x81\x16a'\xBBW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa'\xB5\x90`\x01\x1C\x96a\x13\xFCV[\x95a'{V[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa'\xB5\x90`\x01\x1C\x96a\x13\xFCV[\x80\x15a\x14\nW_\x19\x01\x90V[`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x14a\x14\nW_\x03\x90V[\x80Q_\x95\x86\x95\x90\x94\x93\x90\x92\x90\x91d\xFF\xFF\xFF\xFF\xFF\x16\x92a(>` \x83\x01\x91\x85a(8\x84Q`\x01`\x01`@\x1B\x03\x16\x90V[\x97a.\x19V[`\x01`\x01`@\x1B\x03\x81\x16\x91`\x01`\x01`@\x1B\x03\x87\x16\x83\x03a(\xEEW[`\x01`\x01`@\x1B\x03\x90\x91\x16\x90R[`\x01`\x01`@\x1B\x03\x83\x16`@\x83\x01R\x15a(\x85W[PPP\x92\x91\x90V[a(\xA4\x91\x92\x95P``\x90a(\x9Da$\xE1`9Ta'\xE5V[\x01`\x02\x90RV[`\x01`\x01`@\x1B\x03d\xFF\xFF\xFF\xFF\xFFa(\xBEa\x10\xBC\x88a'\xF1V[\x95\x16\x91\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj_\x80\xA3_\x80\x80a(}V[\x98P\x80a(\xFE\x87a(h\x93a.oV[\x99\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF`@Q\x80a)U\x85\x8A\x8C\x84\x91`@\x91\x94\x93`\x01`\x01`@\x1B\x03\x80\x92d\xFF\xFF\xFF\xFF\xFF``\x87\x01\x98\x16\x86R\x16` \x85\x01R\x16\x91\x01RV[\x03\x90\xA1\x90\x91Pa(ZV[a\x15\xA1`\x80a\x06\xB9\x92\x80Q`<Ub\xFF\xFF\xFF` \x82\x01Q\x16`=T\x90j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0`@\x84\x01Q`\x18\x1B\x16\x91j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x17\x17`=U``\x81\x01Q`\x07\x0B`=T\x90`X\x1B`\x01`\x01`@\x1B\x03`X\x1B\x16\x90`\x01`\x01`@\x1B\x03`X\x1B\x19\x16\x17`=U\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90`\x0F\x0B\x90`\x0F\x0B\x01\x90`\x01`\x01`\x7F\x1B\x03\x19\x82\x12`\x01`\x01`\x7F\x1B\x03\x83\x13\x17a\x14\nWV[\x90c;\x9A\xCA\0\x82\x02\x91\x80\x83\x05c;\x9A\xCA\0\x14\x90\x15\x17\x15a\x14\nWV[`\x01`\xFF\x1B\x81\x14a\x14\nW_\x03\x90V[b\xFF\xFF\xFFa*F` \x83\x01Qb\xFF\xFF\xFF\x16\x90V[\x16a,\x89W`@\x81\x01\x90a*\xD2a\n\xF3a*\xC1a*\xB3a*\xABa*\x8Ea*va\x10\xBC\x89Q`\x01`\x01`@\x1B\x03\x16\x90V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x0B\x90V[a*\xA5a*\x9F``\x89\x01Q`\x07\x0B\x90V[`\x07\x0B\x90V[\x90a)\xE0V[`\x0F\x0Ba*\x06V[\x94Q`\x01`\x01`@\x1B\x03\x16\x90V[`4T`\x01`\x01`@\x1B\x03\x16a\x14HV[`:Ta+\x01\x90`@\x1C`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x16`\x01`\x01`@\x1B\x03\x19`:T\x16\x17`:UV[a+\x1Ag\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19`:T\x16`:UV[a+'_`<U_`=UV[_\x90_\x83\x12a,!W[P`3T`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x92\x91\x16\x90\x82;\x15a\x01\xCCW`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R`$\x82\x01\x84\x90R`\x01`\x01`@\x1B\x03\x16`D\x82\x01R\x90_\x90\x82\x90`d\x90\x82\x90\x84\x90Z\xF1\x80\x15a\x02\xD0Wa,\rW[P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8EDa,\x08`\x01`\x01`@\x1B\x03a+\xF6`:T`\x01`\x01`@\x1B\x03\x16\x90V[`@Q\x94\x85R\x16\x92\x90\x81\x90` \x82\x01\x90V[\x03\x90\xA2V[\x80a\t\xD1_a,\x1B\x93a\r)V[_a+\xB7V[a,\x83\x91Pa,da,_a\x10\xBCa\x10\xBC\x93a,Y`\x80a,J`4T`\x01`\x01`@\x1B\x03\x16\x90V[\x92\x01Q`\x01`\x01`@\x1B\x03\x16\x90V[\x90a\x14HV[a\x16\xD8V[a,~a,ya,s\x86a*\"V[\x83a\x14\x1DV[a\x16\xF4V[a\x14*V[_a+1V[a\x06\xB9\x90a)`V[\x93\x91\x90\x92\x93a,\xAB\x81Q\x80\x15\x15\x90\x81a\x1F\xC9WPa\x1E\xC4V[` \x92a,\xB7\x84a\rhV[\x92\x83R\x83\x95[\x82Q\x87\x11a-'W`\x01\x81\x16a,\xFDW\x83Q_R\x86\x83\x01Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa,\xF7\x90`\x01\x1C\x96a\x13\xFCV[\x95a,\xBDV[\x86\x83\x01Q_R\x83Q\x85R\x84\x84`@_`\x02a\x07\xCF\x19Z\x01\xFA\x15a\x01\xCCWa,\xF7\x90`\x01\x1C\x96a\x13\xFCV[P\x94PP\x90PQ\x14\x90V[\x90a-<\x82a\rvV[a-I`@Q\x91\x82a\r)V[\x82\x81R\x80\x92a-Z`\x1F\x19\x91a\rvV[\x01\x90` 6\x91\x017V[\x80Q`\x05\x10\x15a\x13\xA3W`\xC0a\x0F\xDB\x91\x01Qa.\xA3V[\x80Q`\x06\x10\x15a\x13\xA3W`\xE0a\x0F\xDB\x91\x01Qa.\xA3V[\x80Q`\x01\x10\x15a\x13\xA3W`@\x01Q\x90V[\x80Q`\x02\x10\x15a\x13\xA3W``a\x0F\xDB\x91\x01Qa.\xA3V[\x15a-\xC1WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x90\xFD[a\x0F\xDB\x92\x91a.ea J`\xC0\x93` `@\x87\x01\x91a.>a\x04\xE0a\x1F\x0B\x85\x8Ba\x12\x81V[a Ea.Vd?\xFF\xFF\xFF\xFF\x88`\x02\x1C\x16\x94\x8Aa\x12\x81V[\x93\x90\x99\x015\x98\x89\x936\x91a\x16\xA2V[`\x06\x1B\x16\x1Ba.\xA3V[\x90`\x01`\x01`@\x1B\x03\x80\x91\x16`\x07\x0B\x91\x16`\x07\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17a\x14\nW\x90V[`\x98\x81\x90\x1Cf\xFF\0\0\0\0\0\0\x16`\xA8\x82\x90\x1Ce\xFF\0\0\0\0\0\x16`\xE8\x83\x90\x1Ca\xFF\0\x16`\xF8\x84\x90\x1C\x17`\xD8\x84\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x84\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x85\x90\x1C\x16\x17\x17\x17\x90g\xFF\0\0\0\0\0\0\0`\x88\x91\x90\x91\x1C\x16\x17\x90V[\x15a/\x0FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x90\xFD[\x90\x91\x90\x15a/`WP\x90V[\x81Q\x15a/pWP\x80Q\x90` \x01\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R\x90\x81\x90a/\x94\x90`$\x83\x01\x90a\x18\x80V[\x03\x90\xFD\xFE\xA2dipfsX\"\x12 \0*\xB6\xE4\xF0\x0ES\xD7\xEF\x91\xC7\x91\xF8\xE7\xFEJ\xEF2\xFAgLQ,\xF7\xFA%\xF0\xF2\xADwB\x8AdsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `AmountMustBeMultipleOfGwei()` and selector `0x8777ac5c`.
```solidity
error AmountMustBeMultipleOfGwei();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct AmountMustBeMultipleOfGwei {}
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
        impl ::core::convert::From<AmountMustBeMultipleOfGwei>
        for UnderlyingRustTuple<'_> {
            fn from(value: AmountMustBeMultipleOfGwei) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for AmountMustBeMultipleOfGwei {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AmountMustBeMultipleOfGwei {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AmountMustBeMultipleOfGwei()";
            const SELECTOR: [u8; 4] = [135u8, 119u8, 172u8, 92u8];
            #[inline]
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
    /**Custom error with signature `BeaconTimestampTooFarInPast()` and selector `0x37e07ffd`.
```solidity
error BeaconTimestampTooFarInPast();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BeaconTimestampTooFarInPast {}
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
        impl ::core::convert::From<BeaconTimestampTooFarInPast>
        for UnderlyingRustTuple<'_> {
            fn from(value: BeaconTimestampTooFarInPast) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for BeaconTimestampTooFarInPast {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BeaconTimestampTooFarInPast {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BeaconTimestampTooFarInPast()";
            const SELECTOR: [u8; 4] = [55u8, 224u8, 127u8, 253u8];
            #[inline]
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
    /**Custom error with signature `CannotCheckpointTwiceInSingleBlock()` and selector `0x67db5b8b`.
```solidity
error CannotCheckpointTwiceInSingleBlock();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CannotCheckpointTwiceInSingleBlock {}
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
        impl ::core::convert::From<CannotCheckpointTwiceInSingleBlock>
        for UnderlyingRustTuple<'_> {
            fn from(value: CannotCheckpointTwiceInSingleBlock) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CannotCheckpointTwiceInSingleBlock {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CannotCheckpointTwiceInSingleBlock {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CannotCheckpointTwiceInSingleBlock()";
            const SELECTOR: [u8; 4] = [103u8, 219u8, 91u8, 139u8];
            #[inline]
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
    /**Custom error with signature `CheckpointAlreadyActive()` and selector `0xbe9bc300`.
```solidity
error CheckpointAlreadyActive();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointAlreadyActive {}
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
        impl ::core::convert::From<CheckpointAlreadyActive> for UnderlyingRustTuple<'_> {
            fn from(value: CheckpointAlreadyActive) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckpointAlreadyActive {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CheckpointAlreadyActive {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CheckpointAlreadyActive()";
            const SELECTOR: [u8; 4] = [190u8, 155u8, 195u8, 0u8];
            #[inline]
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
    /**Custom error with signature `CredentialsAlreadyVerified()` and selector `0x35e09e9d`.
```solidity
error CredentialsAlreadyVerified();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CredentialsAlreadyVerified {}
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
        impl ::core::convert::From<CredentialsAlreadyVerified>
        for UnderlyingRustTuple<'_> {
            fn from(value: CredentialsAlreadyVerified) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CredentialsAlreadyVerified {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CredentialsAlreadyVerified {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CredentialsAlreadyVerified()";
            const SELECTOR: [u8; 4] = [53u8, 224u8, 158u8, 157u8];
            #[inline]
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
    /**Custom error with signature `InsufficientWithdrawableBalance()` and selector `0x0b1bd51c`.
```solidity
error InsufficientWithdrawableBalance();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InsufficientWithdrawableBalance {}
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
        impl ::core::convert::From<InsufficientWithdrawableBalance>
        for UnderlyingRustTuple<'_> {
            fn from(value: InsufficientWithdrawableBalance) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for InsufficientWithdrawableBalance {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InsufficientWithdrawableBalance {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InsufficientWithdrawableBalance()";
            const SELECTOR: [u8; 4] = [11u8, 27u8, 213u8, 28u8];
            #[inline]
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
    /**Custom error with signature `InvalidEIP4788Response()` and selector `0x558ad0a3`.
```solidity
error InvalidEIP4788Response();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidEIP4788Response {}
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
        impl ::core::convert::From<InvalidEIP4788Response> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidEIP4788Response) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidEIP4788Response {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidEIP4788Response {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidEIP4788Response()";
            const SELECTOR: [u8; 4] = [85u8, 138u8, 208u8, 163u8];
            #[inline]
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
    /**Custom error with signature `InvalidPubKeyLength()` and selector `0x9f106472`.
```solidity
error InvalidPubKeyLength();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidPubKeyLength {}
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
        impl ::core::convert::From<InvalidPubKeyLength> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidPubKeyLength) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidPubKeyLength {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidPubKeyLength {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidPubKeyLength()";
            const SELECTOR: [u8; 4] = [159u8, 16u8, 100u8, 114u8];
            #[inline]
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
    /**Custom error with signature `MsgValueNot32ETH()` and selector `0x24b4b598`.
```solidity
error MsgValueNot32ETH();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MsgValueNot32ETH {}
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
        impl ::core::convert::From<MsgValueNot32ETH> for UnderlyingRustTuple<'_> {
            fn from(value: MsgValueNot32ETH) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for MsgValueNot32ETH {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MsgValueNot32ETH {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MsgValueNot32ETH()";
            const SELECTOR: [u8; 4] = [36u8, 180u8, 181u8, 152u8];
            #[inline]
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
    /**Custom error with signature `NoActiveCheckpoint()` and selector `0x1a544f49`.
```solidity
error NoActiveCheckpoint();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoActiveCheckpoint {}
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
        impl ::core::convert::From<NoActiveCheckpoint> for UnderlyingRustTuple<'_> {
            fn from(value: NoActiveCheckpoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoActiveCheckpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoActiveCheckpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoActiveCheckpoint()";
            const SELECTOR: [u8; 4] = [26u8, 84u8, 79u8, 73u8];
            #[inline]
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
    /**Custom error with signature `NoBalanceToCheckpoint()` and selector `0xcb7aa564`.
```solidity
error NoBalanceToCheckpoint();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NoBalanceToCheckpoint {}
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
        impl ::core::convert::From<NoBalanceToCheckpoint> for UnderlyingRustTuple<'_> {
            fn from(value: NoBalanceToCheckpoint) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NoBalanceToCheckpoint {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NoBalanceToCheckpoint {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NoBalanceToCheckpoint()";
            const SELECTOR: [u8; 4] = [203u8, 122u8, 165u8, 100u8];
            #[inline]
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
    /**Custom error with signature `OnlyEigenPodManager()` and selector `0xc84e9984`.
```solidity
error OnlyEigenPodManager();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodManager {}
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
        impl ::core::convert::From<OnlyEigenPodManager> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodManager) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodManager {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodManager {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodManager()";
            const SELECTOR: [u8; 4] = [200u8, 78u8, 153u8, 132u8];
            #[inline]
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
    /**Custom error with signature `OnlyEigenPodOwner()` and selector `0xe33e6e06`.
```solidity
error OnlyEigenPodOwner();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodOwner {}
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
        impl ::core::convert::From<OnlyEigenPodOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyEigenPodOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodOwner()";
            const SELECTOR: [u8; 4] = [227u8, 62u8, 110u8, 6u8];
            #[inline]
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
    /**Custom error with signature `OnlyEigenPodOwnerOrProofSubmitter()` and selector `0x427a7779`.
```solidity
error OnlyEigenPodOwnerOrProofSubmitter();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyEigenPodOwnerOrProofSubmitter {}
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
        impl ::core::convert::From<OnlyEigenPodOwnerOrProofSubmitter>
        for UnderlyingRustTuple<'_> {
            fn from(value: OnlyEigenPodOwnerOrProofSubmitter) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OnlyEigenPodOwnerOrProofSubmitter {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyEigenPodOwnerOrProofSubmitter {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyEigenPodOwnerOrProofSubmitter()";
            const SELECTOR: [u8; 4] = [66u8, 122u8, 119u8, 121u8];
            #[inline]
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
    /**Custom error with signature `TimestampOutOfRange()` and selector `0xf289ccda`.
```solidity
error TimestampOutOfRange();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TimestampOutOfRange {}
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
        impl ::core::convert::From<TimestampOutOfRange> for UnderlyingRustTuple<'_> {
            fn from(value: TimestampOutOfRange) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TimestampOutOfRange {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TimestampOutOfRange {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TimestampOutOfRange()";
            const SELECTOR: [u8; 4] = [242u8, 137u8, 204u8, 218u8];
            #[inline]
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
    /**Custom error with signature `ValidatorInactiveOnBeaconChain()` and selector `0x65608db4`.
```solidity
error ValidatorInactiveOnBeaconChain();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInactiveOnBeaconChain {}
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
        impl ::core::convert::From<ValidatorInactiveOnBeaconChain>
        for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorInactiveOnBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ValidatorInactiveOnBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorInactiveOnBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorInactiveOnBeaconChain()";
            const SELECTOR: [u8; 4] = [101u8, 96u8, 141u8, 180u8];
            #[inline]
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
    /**Custom error with signature `ValidatorIsExitingBeaconChain()` and selector `0x2eade637`.
```solidity
error ValidatorIsExitingBeaconChain();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorIsExitingBeaconChain {}
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
        impl ::core::convert::From<ValidatorIsExitingBeaconChain>
        for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorIsExitingBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ValidatorIsExitingBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorIsExitingBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorIsExitingBeaconChain()";
            const SELECTOR: [u8; 4] = [46u8, 173u8, 230u8, 55u8];
            #[inline]
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
    /**Custom error with signature `ValidatorNotActiveInPod()` and selector `0xd49e19a7`.
```solidity
error ValidatorNotActiveInPod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorNotActiveInPod {}
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
        impl ::core::convert::From<ValidatorNotActiveInPod> for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorNotActiveInPod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ValidatorNotActiveInPod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorNotActiveInPod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorNotActiveInPod()";
            const SELECTOR: [u8; 4] = [212u8, 158u8, 25u8, 167u8];
            #[inline]
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
    /**Custom error with signature `ValidatorNotSlashedOnBeaconChain()` and selector `0xb0e72f68`.
```solidity
error ValidatorNotSlashedOnBeaconChain();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorNotSlashedOnBeaconChain {}
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
        impl ::core::convert::From<ValidatorNotSlashedOnBeaconChain>
        for UnderlyingRustTuple<'_> {
            fn from(value: ValidatorNotSlashedOnBeaconChain) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ValidatorNotSlashedOnBeaconChain {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ValidatorNotSlashedOnBeaconChain {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ValidatorNotSlashedOnBeaconChain()";
            const SELECTOR: [u8; 4] = [176u8, 231u8, 47u8, 104u8];
            #[inline]
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
    /**Custom error with signature `WithdrawCredentialsNotForEigenPod()` and selector `0x4460acce`.
```solidity
error WithdrawCredentialsNotForEigenPod();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WithdrawCredentialsNotForEigenPod {}
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
        impl ::core::convert::From<WithdrawCredentialsNotForEigenPod>
        for UnderlyingRustTuple<'_> {
            fn from(value: WithdrawCredentialsNotForEigenPod) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for WithdrawCredentialsNotForEigenPod {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for WithdrawCredentialsNotForEigenPod {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WithdrawCredentialsNotForEigenPod()";
            const SELECTOR: [u8; 4] = [68u8, 96u8, 172u8, 206u8];
            #[inline]
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
    /**Event with signature `CheckpointCreated(uint64,bytes32,uint256)` and selector `0x575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076`.
```solidity
event CheckpointCreated(uint64 indexed checkpointTimestamp, bytes32 indexed beaconBlockRoot, uint256 validatorCount);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CheckpointCreated {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub validatorCount: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for CheckpointCreated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "CheckpointCreated(uint64,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                87u8,
                87u8,
                150u8,
                19u8,
                59u8,
                190u8,
                211u8,
                55u8,
                229u8,
                179u8,
                154u8,
                164u8,
                154u8,
                48u8,
                220u8,
                37u8,
                86u8,
                169u8,
                30u8,
                12u8,
                108u8,
                42u8,
                244u8,
                183u8,
                184u8,
                134u8,
                174u8,
                119u8,
                235u8,
                239u8,
                16u8,
                118u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    beaconBlockRoot: topics.2,
                    validatorCount: data.0,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.validatorCount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
                    self.beaconBlockRoot.clone(),
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.beaconBlockRoot);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CheckpointCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CheckpointCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CheckpointCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `CheckpointFinalized(uint64,int256)` and selector `0x525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44`.
```solidity
event CheckpointFinalized(uint64 indexed checkpointTimestamp, int256 totalShareDeltaWei);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct CheckpointFinalized {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
        #[allow(missing_docs)]
        pub totalShareDeltaWei: alloy::sol_types::private::primitives::aliases::I256,
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
        impl alloy_sol_types::SolEvent for CheckpointFinalized {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "CheckpointFinalized(uint64,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                82u8,
                84u8,
                8u8,
                194u8,
                1u8,
                188u8,
                21u8,
                118u8,
                235u8,
                68u8,
                17u8,
                111u8,
                100u8,
                120u8,
                241u8,
                194u8,
                165u8,
                71u8,
                117u8,
                177u8,
                154u8,
                4u8,
                59u8,
                207u8,
                220u8,
                112u8,
                131u8,
                100u8,
                247u8,
                79u8,
                142u8,
                68u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    totalShareDeltaWei: data.0,
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalShareDeltaWei),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.checkpointTimestamp.clone())
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for CheckpointFinalized {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&CheckpointFinalized> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &CheckpointFinalized) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Event with signature `ProofSubmitterUpdated(address,address)` and selector `0xfb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac`.
```solidity
event ProofSubmitterUpdated(address prevProofSubmitter, address newProofSubmitter);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ProofSubmitterUpdated {
        #[allow(missing_docs)]
        pub prevProofSubmitter: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newProofSubmitter: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for ProofSubmitterUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProofSubmitterUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                251u8,
                129u8,
                41u8,
                8u8,
                10u8,
                25u8,
                211u8,
                77u8,
                206u8,
                172u8,
                4u8,
                186u8,
                37u8,
                63u8,
                197u8,
                3u8,
                4u8,
                220u8,
                134u8,
                199u8,
                41u8,
                189u8,
                99u8,
                205u8,
                202u8,
                74u8,
                150u8,
                154u8,
                209u8,
                154u8,
                94u8,
                172u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    prevProofSubmitter: data.0,
                    newProofSubmitter: data.1,
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
                        &self.prevProofSubmitter,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newProofSubmitter,
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
        impl alloy_sol_types::private::IntoLogData for ProofSubmitterUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProofSubmitterUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProofSubmitterUpdated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `ValidatorCheckpointed(uint64,uint40)` and selector `0xa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f`.
```solidity
event ValidatorCheckpointed(uint64 indexed checkpointTimestamp, uint40 indexed validatorIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorCheckpointed {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
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
        impl alloy_sol_types::SolEvent for ValidatorCheckpointed {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
            );
            const SIGNATURE: &'static str = "ValidatorCheckpointed(uint64,uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                169u8,
                28u8,
                89u8,
                3u8,
                60u8,
                52u8,
                35u8,
                225u8,
                139u8,
                84u8,
                208u8,
                172u8,
                236u8,
                235u8,
                180u8,
                151u8,
                47u8,
                158u8,
                169u8,
                90u8,
                237u8,
                245u8,
                244u8,
                202u8,
                227u8,
                182u8,
                119u8,
                176u8,
                46u8,
                175u8,
                58u8,
                63u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    validatorIndex: topics.2,
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
                    self.checkpointTimestamp.clone(),
                    self.validatorIndex.clone(),
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    40,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.validatorIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorCheckpointed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorCheckpointed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorCheckpointed) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `ValidatorWithdrawn(uint64,uint40)` and selector `0x2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a`.
```solidity
event ValidatorWithdrawn(uint64 indexed checkpointTimestamp, uint40 indexed validatorIndex);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct ValidatorWithdrawn {
        #[allow(missing_docs)]
        pub checkpointTimestamp: u64,
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
        impl alloy_sol_types::SolEvent for ValidatorWithdrawn {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
            );
            const SIGNATURE: &'static str = "ValidatorWithdrawn(uint64,uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                42u8,
                2u8,
                54u8,
                31u8,
                250u8,
                102u8,
                207u8,
                44u8,
                45u8,
                164u8,
                104u8,
                44u8,
                35u8,
                85u8,
                166u8,
                173u8,
                202u8,
                169u8,
                246u8,
                194u8,
                39u8,
                182u8,
                230u8,
                86u8,
                62u8,
                104u8,
                72u8,
                15u8,
                149u8,
                135u8,
                98u8,
                106u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    checkpointTimestamp: topics.1,
                    validatorIndex: topics.2,
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
                    self.checkpointTimestamp.clone(),
                    self.validatorIndex.clone(),
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
                    64,
                > as alloy_sol_types::EventTopic>::encode_topic(
                    &self.checkpointTimestamp,
                );
                out[2usize] = <alloy::sol_types::sol_data::Uint<
                    40,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.validatorIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ValidatorWithdrawn {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ValidatorWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ValidatorWithdrawn) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _ethPOS, address _eigenPodManager, uint64 _GENESIS_TIME);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _ethPOS: alloy::sol_types::private::Address,
        pub _eigenPodManager: alloy::sol_types::private::Address,
        pub _GENESIS_TIME: u64,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<64>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._ethPOS, value._eigenPodManager, value._GENESIS_TIME)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _ethPOS: tuple.0,
                        _eigenPodManager: tuple.1,
                        _GENESIS_TIME: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                        &self._ethPOS,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
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
    /**Function with signature `activeValidatorCount()` and selector `0x2340e8d3`.
```solidity
function activeValidatorCount() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activeValidatorCountCall {}
    ///Container type for the return parameters of the [`activeValidatorCount()`](activeValidatorCountCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct activeValidatorCountReturn {
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
            impl ::core::convert::From<activeValidatorCountCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for activeValidatorCountCall {
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
            impl ::core::convert::From<activeValidatorCountReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for activeValidatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activeValidatorCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = activeValidatorCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "activeValidatorCount()";
            const SELECTOR: [u8; 4] = [35u8, 64u8, 232u8, 211u8];
            #[inline]
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
    /**Function with signature `checkpointBalanceExitedGwei(uint64)` and selector `0x52396a59`.
```solidity
function checkpointBalanceExitedGwei(uint64) external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBalanceExitedGweiCall {
        pub _0: u64,
    }
    ///Container type for the return parameters of the [`checkpointBalanceExitedGwei(uint64)`](checkpointBalanceExitedGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBalanceExitedGweiReturn {
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
            impl ::core::convert::From<checkpointBalanceExitedGweiCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkpointBalanceExitedGweiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<checkpointBalanceExitedGweiReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for checkpointBalanceExitedGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointBalanceExitedGweiCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkpointBalanceExitedGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkpointBalanceExitedGwei(uint64)";
            const SELECTOR: [u8; 4] = [82u8, 57u8, 106u8, 89u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
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
    /**Function with signature `currentCheckpoint()` and selector `0x47d28372`.
```solidity
function currentCheckpoint() external view returns (IEigenPodTypes.Checkpoint memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointCall {}
    ///Container type for the return parameters of the [`currentCheckpoint()`](currentCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointReturn {
        pub _0: <IEigenPodTypes::Checkpoint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<currentCheckpointCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentCheckpointCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::Checkpoint,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPodTypes::Checkpoint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<currentCheckpointReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentCheckpointReturn;
            type ReturnTuple<'a> = (IEigenPodTypes::Checkpoint,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentCheckpoint()";
            const SELECTOR: [u8; 4] = [71u8, 210u8, 131u8, 114u8];
            #[inline]
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
    /**Function with signature `currentCheckpointTimestamp()` and selector `0x42ecff2a`.
```solidity
function currentCheckpointTimestamp() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointTimestampCall {}
    ///Container type for the return parameters of the [`currentCheckpointTimestamp()`](currentCheckpointTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointTimestampReturn {
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
            impl ::core::convert::From<currentCheckpointTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentCheckpointTimestampCall {
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
            impl ::core::convert::From<currentCheckpointTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for currentCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentCheckpointTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "currentCheckpointTimestamp()";
            const SELECTOR: [u8; 4] = [66u8, 236u8, 255u8, 42u8];
            #[inline]
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
    /**Function with signature `getParentBlockRoot(uint64)` and selector `0x6c0d2d5a`.
```solidity
function getParentBlockRoot(uint64 timestamp) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getParentBlockRootCall {
        pub timestamp: u64,
    }
    ///Container type for the return parameters of the [`getParentBlockRoot(uint64)`](getParentBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getParentBlockRootReturn {
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
            impl ::core::convert::From<getParentBlockRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootCall) -> Self {
                    (value.timestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getParentBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { timestamp: tuple.0 }
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
            impl ::core::convert::From<getParentBlockRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getParentBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getParentBlockRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getParentBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getParentBlockRoot(uint64)";
            const SELECTOR: [u8; 4] = [108u8, 13u8, 45u8, 90u8];
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
    /**Function with signature `lastCheckpointTimestamp()` and selector `0xee94d67c`.
```solidity
function lastCheckpointTimestamp() external view returns (uint64);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastCheckpointTimestampCall {}
    ///Container type for the return parameters of the [`lastCheckpointTimestamp()`](lastCheckpointTimestampCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct lastCheckpointTimestampReturn {
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
            impl ::core::convert::From<lastCheckpointTimestampCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastCheckpointTimestampCall {
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
            impl ::core::convert::From<lastCheckpointTimestampReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for lastCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastCheckpointTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "lastCheckpointTimestamp()";
            const SELECTOR: [u8; 4] = [238u8, 148u8, 214u8, 124u8];
            #[inline]
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
    /**Function with signature `proofSubmitter()` and selector `0x58753357`.
```solidity
function proofSubmitter() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmitterCall {}
    ///Container type for the return parameters of the [`proofSubmitter()`](proofSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct proofSubmitterReturn {
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
            impl ::core::convert::From<proofSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmitterCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proofSubmitterCall {
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
            impl ::core::convert::From<proofSubmitterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for proofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofSubmitterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = proofSubmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "proofSubmitter()";
            const SELECTOR: [u8; 4] = [88u8, 117u8, 51u8, 87u8];
            #[inline]
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
    /**Function with signature `setProofSubmitter(address)` and selector `0xd06d5587`.
```solidity
function setProofSubmitter(address newProofSubmitter) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProofSubmitterCall {
        pub newProofSubmitter: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setProofSubmitter(address)`](setProofSubmitterCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setProofSubmitterReturn {}
    #[allow(
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
            impl ::core::convert::From<setProofSubmitterCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterCall) -> Self {
                    (value.newProofSubmitter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProofSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newProofSubmitter: tuple.0 }
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
            impl ::core::convert::From<setProofSubmitterReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProofSubmitterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProofSubmitterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProofSubmitter(address)";
            const SELECTOR: [u8; 4] = [208u8, 109u8, 85u8, 135u8];
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
                        &self.newProofSubmitter,
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
    /**Function with signature `startCheckpoint(bool)` and selector `0x88676cad`.
```solidity
function startCheckpoint(bool revertIfNoBalance) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointCall {
        pub revertIfNoBalance: bool,
    }
    ///Container type for the return parameters of the [`startCheckpoint(bool)`](startCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct startCheckpointReturn {}
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
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
            impl ::core::convert::From<startCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointCall) -> Self {
                    (value.revertIfNoBalance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { revertIfNoBalance: tuple.0 }
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
            impl ::core::convert::From<startCheckpointReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for startCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startCheckpointCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = startCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "startCheckpoint(bool)";
            const SELECTOR: [u8; 4] = [136u8, 103u8, 108u8, 173u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.revertIfNoBalance,
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
    /**Function with signature `validatorPubkeyHashToInfo(bytes32)` and selector `0x6fcd0e53`.
```solidity
function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPodTypes.ValidatorInfo memory);
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
        pub _0: <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
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
function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPodTypes.ValidatorInfo memory);
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
        pub _0: <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPodTypes::ValidatorInfo as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (IEigenPodTypes::ValidatorInfo,);
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
function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
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
        pub _0: <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
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
function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPodTypes.VALIDATOR_STATUS);
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
        pub _0: <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            type UnderlyingSolTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IEigenPodTypes::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,
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
            type ReturnTuple<'a> = (IEigenPodTypes::VALIDATOR_STATUS,);
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
    /**Function with signature `verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])` and selector `0xf074ba62`.
```solidity
function verifyCheckpointProofs(BeaconChainProofs.BalanceContainerProof memory balanceContainerProof, BeaconChainProofs.BalanceProof[] memory proofs) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCheckpointProofsCall {
        pub balanceContainerProof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
        pub proofs: alloy::sol_types::private::Vec<
            <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])`](verifyCheckpointProofsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyCheckpointProofsReturn {}
    #[allow(
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
            impl ::core::convert::From<verifyCheckpointProofsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsCall) -> Self {
                    (value.balanceContainerProof, value.proofs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyCheckpointProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        balanceContainerProof: tuple.0,
                        proofs: tuple.1,
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
            impl ::core::convert::From<verifyCheckpointProofsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyCheckpointProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyCheckpointProofsCall {
            type Parameters<'a> = (
                BeaconChainProofs::BalanceContainerProof,
                alloy::sol_types::sol_data::Array<BeaconChainProofs::BalanceProof>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCheckpointProofsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])";
            const SELECTOR: [u8; 4] = [240u8, 116u8, 186u8, 98u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <BeaconChainProofs::BalanceContainerProof as alloy_sol_types::SolType>::tokenize(
                        &self.balanceContainerProof,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BeaconChainProofs::BalanceProof,
                    > as alloy_sol_types::SolType>::tokenize(&self.proofs),
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
    /**Function with signature `verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))` and selector `0x039157d2`.
```solidity
function verifyStaleBalance(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, BeaconChainProofs.ValidatorProof memory proof) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceCall {
        pub beaconTimestamp: u64,
        pub stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub proof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))`](verifyStaleBalanceCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyStaleBalanceReturn {}
    #[allow(
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
            impl ::core::convert::From<verifyStaleBalanceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceCall) -> Self {
                    (value.beaconTimestamp, value.stateRootProof, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyStaleBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconTimestamp: tuple.0,
                        stateRootProof: tuple.1,
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
            impl ::core::convert::From<verifyStaleBalanceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyStaleBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyStaleBalanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<64>,
                BeaconChainProofs::StateRootProof,
                BeaconChainProofs::ValidatorProof,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStaleBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))";
            const SELECTOR: [u8; 4] = [3u8, 145u8, 87u8, 210u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconTimestamp),
                    <BeaconChainProofs::StateRootProof as alloy_sol_types::SolType>::tokenize(
                        &self.stateRootProof,
                    ),
                    <BeaconChainProofs::ValidatorProof as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])` and selector `0x3f65cf19`.
```solidity
function verifyWithdrawalCredentials(uint64 beaconTimestamp, BeaconChainProofs.StateRootProof memory stateRootProof, uint40[] memory validatorIndices, bytes[] memory validatorFieldsProofs, bytes32[][] memory validatorFields) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsCall {
        pub beaconTimestamp: u64,
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
    ///Container type for the return parameters of the [`verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])`](verifyWithdrawalCredentialsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct verifyWithdrawalCredentialsReturn {}
    #[allow(
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
            impl ::core::convert::From<verifyWithdrawalCredentialsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsCall) -> Self {
                    (
                        value.beaconTimestamp,
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
            for verifyWithdrawalCredentialsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        beaconTimestamp: tuple.0,
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
            impl ::core::convert::From<verifyWithdrawalCredentialsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for verifyWithdrawalCredentialsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for verifyWithdrawalCredentialsCall {
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
            type Return = verifyWithdrawalCredentialsReturn;
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
                    > as alloy_sol_types::SolType>::tokenize(&self.beaconTimestamp),
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
    ///Container for all the [`EigenPod`](self) function calls.
    pub enum EigenPodCalls {
        GENESIS_TIME(GENESIS_TIMECall),
        activeValidatorCount(activeValidatorCountCall),
        checkpointBalanceExitedGwei(checkpointBalanceExitedGweiCall),
        currentCheckpoint(currentCheckpointCall),
        currentCheckpointTimestamp(currentCheckpointTimestampCall),
        eigenPodManager(eigenPodManagerCall),
        ethPOS(ethPOSCall),
        getParentBlockRoot(getParentBlockRootCall),
        initialize(initializeCall),
        lastCheckpointTimestamp(lastCheckpointTimestampCall),
        podOwner(podOwnerCall),
        proofSubmitter(proofSubmitterCall),
        recoverTokens(recoverTokensCall),
        setProofSubmitter(setProofSubmitterCall),
        stake(stakeCall),
        startCheckpoint(startCheckpointCall),
        validatorPubkeyHashToInfo(validatorPubkeyHashToInfoCall),
        validatorPubkeyToInfo(validatorPubkeyToInfoCall),
        validatorStatus_0(validatorStatus_0Call),
        validatorStatus_1(validatorStatus_1Call),
        verifyCheckpointProofs(verifyCheckpointProofsCall),
        verifyStaleBalance(verifyStaleBalanceCall),
        verifyWithdrawalCredentials(verifyWithdrawalCredentialsCall),
        withdrawRestakedBeaconChainETH(withdrawRestakedBeaconChainETHCall),
        withdrawableRestakedExecutionLayerGwei(
            withdrawableRestakedExecutionLayerGweiCall,
        ),
    }
    #[automatically_derived]
    impl EigenPodCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 145u8, 87u8, 210u8],
            [11u8, 24u8, 255u8, 102u8],
            [35u8, 64u8, 232u8, 211u8],
            [52u8, 116u8, 170u8, 22u8],
            [63u8, 101u8, 207u8, 25u8],
            [66u8, 236u8, 255u8, 42u8],
            [70u8, 101u8, 188u8, 218u8],
            [71u8, 210u8, 131u8, 114u8],
            [82u8, 57u8, 106u8, 89u8],
            [88u8, 117u8, 51u8, 87u8],
            [88u8, 234u8, 238u8, 121u8],
            [108u8, 13u8, 45u8, 90u8],
            [111u8, 205u8, 14u8, 83u8],
            [116u8, 57u8, 132u8, 31u8],
            [116u8, 205u8, 215u8, 152u8],
            [136u8, 103u8, 108u8, 173u8],
            [155u8, 78u8, 70u8, 52u8],
            [181u8, 34u8, 83u8, 138u8],
            [196u8, 144u8, 116u8, 66u8],
            [196u8, 214u8, 109u8, 232u8],
            [208u8, 109u8, 85u8, 135u8],
            [221u8, 163u8, 52u8, 108u8],
            [238u8, 148u8, 214u8, 124u8],
            [240u8, 116u8, 186u8, 98u8],
            [242u8, 136u8, 36u8, 97u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EigenPodCalls {
        const NAME: &'static str = "EigenPodCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 25usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::GENESIS_TIME(_) => {
                    <GENESIS_TIMECall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::activeValidatorCount(_) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkpointBalanceExitedGwei(_) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentCheckpoint(_) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentCheckpointTimestamp(_) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::ethPOS(_) => <ethPOSCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getParentBlockRoot(_) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::lastCheckpointTimestamp(_) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::podOwner(_) => <podOwnerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::proofSubmitter(_) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recoverTokens(_) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProofSubmitter(_) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stake(_) => <stakeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::startCheckpoint(_) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::verifyCheckpointProofs(_) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyStaleBalance(_) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::verifyWithdrawalCredentials(_) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<EigenPodCalls>] = &[
                {
                    fn verifyStaleBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::verifyStaleBalance)
                    }
                    verifyStaleBalance
                },
                {
                    fn podOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <podOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::podOwner)
                    }
                    podOwner
                },
                {
                    fn activeValidatorCount(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::activeValidatorCount)
                    }
                    activeValidatorCount
                },
                {
                    fn withdrawableRestakedExecutionLayerGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <withdrawableRestakedExecutionLayerGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::withdrawableRestakedExecutionLayerGwei)
                    }
                    withdrawableRestakedExecutionLayerGwei
                },
                {
                    fn verifyWithdrawalCredentials(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::verifyWithdrawalCredentials)
                    }
                    verifyWithdrawalCredentials
                },
                {
                    fn currentCheckpointTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::currentCheckpointTimestamp)
                    }
                    currentCheckpointTimestamp
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn currentCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <currentCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::currentCheckpoint)
                    }
                    currentCheckpoint
                },
                {
                    fn checkpointBalanceExitedGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::checkpointBalanceExitedGwei)
                    }
                    checkpointBalanceExitedGwei
                },
                {
                    fn proofSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <proofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::proofSubmitter)
                    }
                    proofSubmitter
                },
                {
                    fn validatorStatus_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorStatus_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::validatorStatus_0)
                    }
                    validatorStatus_0
                },
                {
                    fn getParentBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::getParentBlockRoot)
                    }
                    getParentBlockRoot
                },
                {
                    fn validatorPubkeyHashToInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorPubkeyHashToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::validatorPubkeyHashToInfo)
                    }
                    validatorPubkeyHashToInfo
                },
                {
                    fn validatorStatus_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorStatus_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::validatorStatus_1)
                    }
                    validatorStatus_1
                },
                {
                    fn ethPOS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <ethPOSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::ethPOS)
                    }
                    ethPOS
                },
                {
                    fn startCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <startCheckpointCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::startCheckpoint)
                    }
                    startCheckpoint
                },
                {
                    fn stake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <stakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::stake)
                    }
                    stake
                },
                {
                    fn validatorPubkeyToInfo(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <validatorPubkeyToInfoCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::validatorPubkeyToInfo)
                    }
                    validatorPubkeyToInfo
                },
                {
                    fn withdrawRestakedBeaconChainETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <withdrawRestakedBeaconChainETHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::withdrawRestakedBeaconChainETH)
                    }
                    withdrawRestakedBeaconChainETH
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setProofSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::setProofSubmitter)
                    }
                    setProofSubmitter
                },
                {
                    fn recoverTokens(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <recoverTokensCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::recoverTokens)
                    }
                    recoverTokens
                },
                {
                    fn lastCheckpointTimestamp(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::lastCheckpointTimestamp)
                    }
                    lastCheckpointTimestamp
                },
                {
                    fn verifyCheckpointProofs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::verifyCheckpointProofs)
                    }
                    verifyCheckpointProofs
                },
                {
                    fn GENESIS_TIME(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <GENESIS_TIMECall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodCalls::GENESIS_TIME)
                    }
                    GENESIS_TIME
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
                Self::activeValidatorCount(inner) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkpointBalanceExitedGwei(inner) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentCheckpoint(inner) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::currentCheckpointTimestamp(inner) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getParentBlockRoot(inner) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::lastCheckpointTimestamp(inner) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::podOwner(inner) => {
                    <podOwnerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::proofSubmitter(inner) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recoverTokens(inner) => {
                    <recoverTokensCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProofSubmitter(inner) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::verifyCheckpointProofs(inner) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::activeValidatorCount(inner) => {
                    <activeValidatorCountCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::checkpointBalanceExitedGwei(inner) => {
                    <checkpointBalanceExitedGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentCheckpoint(inner) => {
                    <currentCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::currentCheckpointTimestamp(inner) => {
                    <currentCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getParentBlockRoot(inner) => {
                    <getParentBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::lastCheckpointTimestamp(inner) => {
                    <lastCheckpointTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::proofSubmitter(inner) => {
                    <proofSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setProofSubmitter(inner) => {
                    <setProofSubmitterCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stake(inner) => {
                    <stakeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::startCheckpoint(inner) => {
                    <startCheckpointCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::verifyCheckpointProofs(inner) => {
                    <verifyCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyStaleBalance(inner) => {
                    <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::verifyWithdrawalCredentials(inner) => {
                    <verifyWithdrawalCredentialsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`EigenPod`](self) custom errors.
    pub enum EigenPodErrors {
        AmountMustBeMultipleOfGwei(AmountMustBeMultipleOfGwei),
        BeaconTimestampTooFarInPast(BeaconTimestampTooFarInPast),
        CannotCheckpointTwiceInSingleBlock(CannotCheckpointTwiceInSingleBlock),
        CheckpointAlreadyActive(CheckpointAlreadyActive),
        CredentialsAlreadyVerified(CredentialsAlreadyVerified),
        CurrentlyPaused(CurrentlyPaused),
        InputAddressZero(InputAddressZero),
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        InsufficientWithdrawableBalance(InsufficientWithdrawableBalance),
        InvalidEIP4788Response(InvalidEIP4788Response),
        InvalidProof(InvalidProof),
        InvalidProofLength(InvalidProofLength),
        InvalidPubKeyLength(InvalidPubKeyLength),
        InvalidValidatorFieldsLength(InvalidValidatorFieldsLength),
        MsgValueNot32ETH(MsgValueNot32ETH),
        NoActiveCheckpoint(NoActiveCheckpoint),
        NoBalanceToCheckpoint(NoBalanceToCheckpoint),
        OnlyEigenPodManager(OnlyEigenPodManager),
        OnlyEigenPodOwner(OnlyEigenPodOwner),
        OnlyEigenPodOwnerOrProofSubmitter(OnlyEigenPodOwnerOrProofSubmitter),
        TimestampOutOfRange(TimestampOutOfRange),
        ValidatorInactiveOnBeaconChain(ValidatorInactiveOnBeaconChain),
        ValidatorIsExitingBeaconChain(ValidatorIsExitingBeaconChain),
        ValidatorNotActiveInPod(ValidatorNotActiveInPod),
        ValidatorNotSlashedOnBeaconChain(ValidatorNotSlashedOnBeaconChain),
        WithdrawCredentialsNotForEigenPod(WithdrawCredentialsNotForEigenPod),
    }
    #[automatically_derived]
    impl EigenPodErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [9u8, 189u8, 227u8, 57u8],
            [11u8, 27u8, 213u8, 28u8],
            [26u8, 84u8, 79u8, 73u8],
            [32u8, 5u8, 145u8, 189u8],
            [36u8, 180u8, 181u8, 152u8],
            [46u8, 173u8, 230u8, 55u8],
            [53u8, 224u8, 158u8, 157u8],
            [55u8, 224u8, 127u8, 253u8],
            [66u8, 122u8, 119u8, 121u8],
            [67u8, 113u8, 74u8, 253u8],
            [68u8, 96u8, 172u8, 206u8],
            [77u8, 197u8, 246u8, 164u8],
            [85u8, 138u8, 208u8, 163u8],
            [101u8, 96u8, 141u8, 180u8],
            [103u8, 219u8, 91u8, 139u8],
            [115u8, 99u8, 33u8, 118u8],
            [132u8, 10u8, 72u8, 213u8],
            [135u8, 119u8, 172u8, 92u8],
            [159u8, 16u8, 100u8, 114u8],
            [176u8, 231u8, 47u8, 104u8],
            [190u8, 155u8, 195u8, 0u8],
            [200u8, 78u8, 153u8, 132u8],
            [203u8, 122u8, 165u8, 100u8],
            [212u8, 158u8, 25u8, 167u8],
            [227u8, 62u8, 110u8, 6u8],
            [242u8, 137u8, 204u8, 218u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for EigenPodErrors {
        const NAME: &'static str = "EigenPodErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 26usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AmountMustBeMultipleOfGwei(_) => {
                    <AmountMustBeMultipleOfGwei as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BeaconTimestampTooFarInPast(_) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CannotCheckpointTwiceInSingleBlock(_) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CheckpointAlreadyActive(_) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CredentialsAlreadyVerified(_) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InsufficientWithdrawableBalance(_) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidEIP4788Response(_) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProof(_) => {
                    <InvalidProof as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidProofLength(_) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidPubKeyLength(_) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidValidatorFieldsLength(_) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MsgValueNot32ETH(_) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoActiveCheckpoint(_) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NoBalanceToCheckpoint(_) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodManager(_) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodOwner(_) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(_) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TimestampOutOfRange(_) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorInactiveOnBeaconChain(_) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorIsExitingBeaconChain(_) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorNotActiveInPod(_) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ValidatorNotSlashedOnBeaconChain(_) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::SELECTOR
                }
                Self::WithdrawCredentialsNotForEigenPod(_) => {
                    <WithdrawCredentialsNotForEigenPod as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<EigenPodErrors>] = &[
                {
                    fn InvalidProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProof as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InvalidProof)
                    }
                    InvalidProof
                },
                {
                    fn InsufficientWithdrawableBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InsufficientWithdrawableBalance)
                    }
                    InsufficientWithdrawableBalance
                },
                {
                    fn NoActiveCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::NoActiveCheckpoint)
                    }
                    NoActiveCheckpoint
                },
                {
                    fn InvalidValidatorFieldsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InvalidValidatorFieldsLength)
                    }
                    InvalidValidatorFieldsLength
                },
                {
                    fn MsgValueNot32ETH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::MsgValueNot32ETH)
                    }
                    MsgValueNot32ETH
                },
                {
                    fn ValidatorIsExitingBeaconChain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::ValidatorIsExitingBeaconChain)
                    }
                    ValidatorIsExitingBeaconChain
                },
                {
                    fn CredentialsAlreadyVerified(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::CredentialsAlreadyVerified)
                    }
                    CredentialsAlreadyVerified
                },
                {
                    fn BeaconTimestampTooFarInPast(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::BeaconTimestampTooFarInPast)
                    }
                    BeaconTimestampTooFarInPast
                },
                {
                    fn OnlyEigenPodOwnerOrProofSubmitter(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::OnlyEigenPodOwnerOrProofSubmitter)
                    }
                    OnlyEigenPodOwnerOrProofSubmitter
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn WithdrawCredentialsNotForEigenPod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <WithdrawCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::WithdrawCredentialsNotForEigenPod)
                    }
                    WithdrawCredentialsNotForEigenPod
                },
                {
                    fn InvalidProofLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidProofLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InvalidProofLength)
                    }
                    InvalidProofLength
                },
                {
                    fn InvalidEIP4788Response(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InvalidEIP4788Response)
                    }
                    InvalidEIP4788Response
                },
                {
                    fn ValidatorInactiveOnBeaconChain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::ValidatorInactiveOnBeaconChain)
                    }
                    ValidatorInactiveOnBeaconChain
                },
                {
                    fn CannotCheckpointTwiceInSingleBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::CannotCheckpointTwiceInSingleBlock)
                    }
                    CannotCheckpointTwiceInSingleBlock
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn AmountMustBeMultipleOfGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <AmountMustBeMultipleOfGwei as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::AmountMustBeMultipleOfGwei)
                    }
                    AmountMustBeMultipleOfGwei
                },
                {
                    fn InvalidPubKeyLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::InvalidPubKeyLength)
                    }
                    InvalidPubKeyLength
                },
                {
                    fn ValidatorNotSlashedOnBeaconChain(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::ValidatorNotSlashedOnBeaconChain)
                    }
                    ValidatorNotSlashedOnBeaconChain
                },
                {
                    fn CheckpointAlreadyActive(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::CheckpointAlreadyActive)
                    }
                    CheckpointAlreadyActive
                },
                {
                    fn OnlyEigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::OnlyEigenPodManager)
                    }
                    OnlyEigenPodManager
                },
                {
                    fn NoBalanceToCheckpoint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::NoBalanceToCheckpoint)
                    }
                    NoBalanceToCheckpoint
                },
                {
                    fn ValidatorNotActiveInPod(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::ValidatorNotActiveInPod)
                    }
                    ValidatorNotActiveInPod
                },
                {
                    fn OnlyEigenPodOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::OnlyEigenPodOwner)
                    }
                    OnlyEigenPodOwner
                },
                {
                    fn TimestampOutOfRange(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodErrors> {
                        <TimestampOutOfRange as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(EigenPodErrors::TimestampOutOfRange)
                    }
                    TimestampOutOfRange
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
                Self::AmountMustBeMultipleOfGwei(inner) => {
                    <AmountMustBeMultipleOfGwei as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::BeaconTimestampTooFarInPast(inner) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CannotCheckpointTwiceInSingleBlock(inner) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CheckpointAlreadyActive(inner) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CredentialsAlreadyVerified(inner) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(
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
                Self::InsufficientWithdrawableBalance(inner) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidEIP4788Response(inner) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidProof(inner) => {
                    <InvalidProof as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidProofLength(inner) => {
                    <InvalidProofLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidPubKeyLength(inner) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidValidatorFieldsLength(inner) => {
                    <InvalidValidatorFieldsLength as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::MsgValueNot32ETH(inner) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoActiveCheckpoint(inner) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NoBalanceToCheckpoint(inner) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodOwner(inner) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(inner) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TimestampOutOfRange(inner) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorInactiveOnBeaconChain(inner) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorIsExitingBeaconChain(inner) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorNotActiveInPod(inner) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ValidatorNotSlashedOnBeaconChain(inner) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WithdrawCredentialsNotForEigenPod(inner) => {
                    <WithdrawCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AmountMustBeMultipleOfGwei(inner) => {
                    <AmountMustBeMultipleOfGwei as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::BeaconTimestampTooFarInPast(inner) => {
                    <BeaconTimestampTooFarInPast as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CannotCheckpointTwiceInSingleBlock(inner) => {
                    <CannotCheckpointTwiceInSingleBlock as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CheckpointAlreadyActive(inner) => {
                    <CheckpointAlreadyActive as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CredentialsAlreadyVerified(inner) => {
                    <CredentialsAlreadyVerified as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::InsufficientWithdrawableBalance(inner) => {
                    <InsufficientWithdrawableBalance as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidEIP4788Response(inner) => {
                    <InvalidEIP4788Response as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::InvalidPubKeyLength(inner) => {
                    <InvalidPubKeyLength as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::MsgValueNot32ETH(inner) => {
                    <MsgValueNot32ETH as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoActiveCheckpoint(inner) => {
                    <NoActiveCheckpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NoBalanceToCheckpoint(inner) => {
                    <NoBalanceToCheckpoint as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodManager(inner) => {
                    <OnlyEigenPodManager as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodOwner(inner) => {
                    <OnlyEigenPodOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OnlyEigenPodOwnerOrProofSubmitter(inner) => {
                    <OnlyEigenPodOwnerOrProofSubmitter as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TimestampOutOfRange(inner) => {
                    <TimestampOutOfRange as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorInactiveOnBeaconChain(inner) => {
                    <ValidatorInactiveOnBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorIsExitingBeaconChain(inner) => {
                    <ValidatorIsExitingBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorNotActiveInPod(inner) => {
                    <ValidatorNotActiveInPod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ValidatorNotSlashedOnBeaconChain(inner) => {
                    <ValidatorNotSlashedOnBeaconChain as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WithdrawCredentialsNotForEigenPod(inner) => {
                    <WithdrawCredentialsNotForEigenPod as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`EigenPod`](self) events.
    pub enum EigenPodEvents {
        CheckpointCreated(CheckpointCreated),
        CheckpointFinalized(CheckpointFinalized),
        EigenPodStaked(EigenPodStaked),
        Initialized(Initialized),
        NonBeaconChainETHReceived(NonBeaconChainETHReceived),
        ProofSubmitterUpdated(ProofSubmitterUpdated),
        RestakedBeaconChainETHWithdrawn(RestakedBeaconChainETHWithdrawn),
        ValidatorBalanceUpdated(ValidatorBalanceUpdated),
        ValidatorCheckpointed(ValidatorCheckpointed),
        ValidatorRestaked(ValidatorRestaked),
        ValidatorWithdrawn(ValidatorWithdrawn),
    }
    #[automatically_derived]
    impl EigenPodEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
                42u8,
                2u8,
                54u8,
                31u8,
                250u8,
                102u8,
                207u8,
                44u8,
                45u8,
                164u8,
                104u8,
                44u8,
                35u8,
                85u8,
                166u8,
                173u8,
                202u8,
                169u8,
                246u8,
                194u8,
                39u8,
                182u8,
                230u8,
                86u8,
                62u8,
                104u8,
                72u8,
                15u8,
                149u8,
                135u8,
                98u8,
                106u8,
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
                82u8,
                84u8,
                8u8,
                194u8,
                1u8,
                188u8,
                21u8,
                118u8,
                235u8,
                68u8,
                17u8,
                111u8,
                100u8,
                120u8,
                241u8,
                194u8,
                165u8,
                71u8,
                117u8,
                177u8,
                154u8,
                4u8,
                59u8,
                207u8,
                220u8,
                112u8,
                131u8,
                100u8,
                247u8,
                79u8,
                142u8,
                68u8,
            ],
            [
                87u8,
                87u8,
                150u8,
                19u8,
                59u8,
                190u8,
                211u8,
                55u8,
                229u8,
                179u8,
                154u8,
                164u8,
                154u8,
                48u8,
                220u8,
                37u8,
                86u8,
                169u8,
                30u8,
                12u8,
                108u8,
                42u8,
                244u8,
                183u8,
                184u8,
                134u8,
                174u8,
                119u8,
                235u8,
                239u8,
                16u8,
                118u8,
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
                169u8,
                28u8,
                89u8,
                3u8,
                60u8,
                52u8,
                35u8,
                225u8,
                139u8,
                84u8,
                208u8,
                172u8,
                236u8,
                235u8,
                180u8,
                151u8,
                47u8,
                158u8,
                169u8,
                90u8,
                237u8,
                245u8,
                244u8,
                202u8,
                227u8,
                182u8,
                119u8,
                176u8,
                46u8,
                175u8,
                58u8,
                63u8,
            ],
            [
                251u8,
                129u8,
                41u8,
                8u8,
                10u8,
                25u8,
                211u8,
                77u8,
                206u8,
                172u8,
                4u8,
                186u8,
                37u8,
                63u8,
                197u8,
                3u8,
                4u8,
                220u8,
                134u8,
                199u8,
                41u8,
                189u8,
                99u8,
                205u8,
                202u8,
                74u8,
                150u8,
                154u8,
                209u8,
                154u8,
                94u8,
                172u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for EigenPodEvents {
        const NAME: &'static str = "EigenPodEvents";
        const COUNT: usize = 11usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <CheckpointCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CheckpointCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CheckpointCreated)
                }
                Some(
                    <CheckpointFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <CheckpointFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::CheckpointFinalized)
                }
                Some(<EigenPodStaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EigenPodStaked as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::EigenPodStaked)
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
                    <ProofSubmitterUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProofSubmitterUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProofSubmitterUpdated)
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
                    <ValidatorCheckpointed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidatorCheckpointed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidatorCheckpointed)
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
                Some(
                    <ValidatorWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ValidatorWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ValidatorWithdrawn)
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
    impl alloy_sol_types::private::IntoLogData for EigenPodEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckpointCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::CheckpointFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProofSubmitterUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorCheckpointed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ValidatorWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::CheckpointCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::CheckpointFinalized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::EigenPodStaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NonBeaconChainETHReceived(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProofSubmitterUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::RestakedBeaconChainETHWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorBalanceUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorCheckpointed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorRestaked(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ValidatorWithdrawn(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`EigenPod`](self) contract instance.

See the [wrapper's documentation](`EigenPodInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> EigenPodInstance<T, P, N> {
        EigenPodInstance::<T, P, N>::new(address, provider)
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
        _eigenPodManager: alloy::sol_types::private::Address,
        _GENESIS_TIME: u64,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<EigenPodInstance<T, P, N>>,
    > {
        EigenPodInstance::<
            T,
            P,
            N,
        >::deploy(provider, _ethPOS, _eigenPodManager, _GENESIS_TIME)
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
        _eigenPodManager: alloy::sol_types::private::Address,
        _GENESIS_TIME: u64,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        EigenPodInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _ethPOS, _eigenPodManager, _GENESIS_TIME)
    }
    /**A [`EigenPod`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`EigenPod`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct EigenPodInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for EigenPodInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("EigenPodInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > EigenPodInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`EigenPod`](self) contract instance.

See the [wrapper's documentation](`EigenPodInstance`) for more details.*/
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
            _eigenPodManager: alloy::sol_types::private::Address,
            _GENESIS_TIME: u64,
        ) -> alloy_contract::Result<EigenPodInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _ethPOS,
                _eigenPodManager,
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
            _eigenPodManager: alloy::sol_types::private::Address,
            _GENESIS_TIME: u64,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _ethPOS,
                            _eigenPodManager,
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
    impl<T, P: ::core::clone::Clone, N> EigenPodInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> EigenPodInstance<T, P, N> {
            EigenPodInstance {
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
    > EigenPodInstance<T, P, N> {
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
        ///Creates a new call builder for the [`activeValidatorCount`] function.
        pub fn activeValidatorCount(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, activeValidatorCountCall, N> {
            self.call_builder(&activeValidatorCountCall {})
        }
        ///Creates a new call builder for the [`checkpointBalanceExitedGwei`] function.
        pub fn checkpointBalanceExitedGwei(
            &self,
            _0: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkpointBalanceExitedGweiCall, N> {
            self.call_builder(
                &checkpointBalanceExitedGweiCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`currentCheckpoint`] function.
        pub fn currentCheckpoint(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentCheckpointCall, N> {
            self.call_builder(&currentCheckpointCall {})
        }
        ///Creates a new call builder for the [`currentCheckpointTimestamp`] function.
        pub fn currentCheckpointTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentCheckpointTimestampCall, N> {
            self.call_builder(&currentCheckpointTimestampCall {})
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
        ///Creates a new call builder for the [`getParentBlockRoot`] function.
        pub fn getParentBlockRoot(
            &self,
            timestamp: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getParentBlockRootCall, N> {
            self.call_builder(
                &getParentBlockRootCall {
                    timestamp,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            _podOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall { _podOwner })
        }
        ///Creates a new call builder for the [`lastCheckpointTimestamp`] function.
        pub fn lastCheckpointTimestamp(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, lastCheckpointTimestampCall, N> {
            self.call_builder(&lastCheckpointTimestampCall {})
        }
        ///Creates a new call builder for the [`podOwner`] function.
        pub fn podOwner(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, podOwnerCall, N> {
            self.call_builder(&podOwnerCall {})
        }
        ///Creates a new call builder for the [`proofSubmitter`] function.
        pub fn proofSubmitter(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, proofSubmitterCall, N> {
            self.call_builder(&proofSubmitterCall {})
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
        ///Creates a new call builder for the [`setProofSubmitter`] function.
        pub fn setProofSubmitter(
            &self,
            newProofSubmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProofSubmitterCall, N> {
            self.call_builder(
                &setProofSubmitterCall {
                    newProofSubmitter,
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
        ///Creates a new call builder for the [`startCheckpoint`] function.
        pub fn startCheckpoint(
            &self,
            revertIfNoBalance: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, startCheckpointCall, N> {
            self.call_builder(
                &startCheckpointCall {
                    revertIfNoBalance,
                },
            )
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
        ///Creates a new call builder for the [`verifyCheckpointProofs`] function.
        pub fn verifyCheckpointProofs(
            &self,
            balanceContainerProof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
            proofs: alloy::sol_types::private::Vec<
                <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyCheckpointProofsCall, N> {
            self.call_builder(
                &verifyCheckpointProofsCall {
                    balanceContainerProof,
                    proofs,
                },
            )
        }
        ///Creates a new call builder for the [`verifyStaleBalance`] function.
        pub fn verifyStaleBalance(
            &self,
            beaconTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            proof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyStaleBalanceCall, N> {
            self.call_builder(
                &verifyStaleBalanceCall {
                    beaconTimestamp,
                    stateRootProof,
                    proof,
                },
            )
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials`] function.
        pub fn verifyWithdrawalCredentials(
            &self,
            beaconTimestamp: u64,
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
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyWithdrawalCredentialsCall, N> {
            self.call_builder(
                &verifyWithdrawalCredentialsCall {
                    beaconTimestamp,
                    stateRootProof,
                    validatorIndices,
                    validatorFieldsProofs,
                    validatorFields,
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
    > EigenPodInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`CheckpointCreated`] event.
        pub fn CheckpointCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CheckpointCreated, N> {
            self.event_filter::<CheckpointCreated>()
        }
        ///Creates a new event filter for the [`CheckpointFinalized`] event.
        pub fn CheckpointFinalized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, CheckpointFinalized, N> {
            self.event_filter::<CheckpointFinalized>()
        }
        ///Creates a new event filter for the [`EigenPodStaked`] event.
        pub fn EigenPodStaked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, EigenPodStaked, N> {
            self.event_filter::<EigenPodStaked>()
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
        ///Creates a new event filter for the [`ProofSubmitterUpdated`] event.
        pub fn ProofSubmitterUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProofSubmitterUpdated, N> {
            self.event_filter::<ProofSubmitterUpdated>()
        }
        ///Creates a new event filter for the [`RestakedBeaconChainETHWithdrawn`] event.
        pub fn RestakedBeaconChainETHWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, RestakedBeaconChainETHWithdrawn, N> {
            self.event_filter::<RestakedBeaconChainETHWithdrawn>()
        }
        ///Creates a new event filter for the [`ValidatorBalanceUpdated`] event.
        pub fn ValidatorBalanceUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorBalanceUpdated, N> {
            self.event_filter::<ValidatorBalanceUpdated>()
        }
        ///Creates a new event filter for the [`ValidatorCheckpointed`] event.
        pub fn ValidatorCheckpointed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorCheckpointed, N> {
            self.event_filter::<ValidatorCheckpointed>()
        }
        ///Creates a new event filter for the [`ValidatorRestaked`] event.
        pub fn ValidatorRestaked_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorRestaked, N> {
            self.event_filter::<ValidatorRestaked>()
        }
        ///Creates a new event filter for the [`ValidatorWithdrawn`] event.
        pub fn ValidatorWithdrawn_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ValidatorWithdrawn, N> {
            self.event_filter::<ValidatorWithdrawn>()
        }
    }
}
