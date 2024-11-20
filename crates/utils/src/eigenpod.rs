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
    ///0x60e060405234801561001057600080fd5b50604051613eb8380380613eb883398101604081905261002f91610136565b6001600160a01b03808416608052821660a0526001600160401b03811660c05261005761005f565b50505061018f565b600054610100900460ff16156100cb5760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff9081161461011c576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b038116811461013357600080fd5b50565b60008060006060848603121561014b57600080fd5b83516101568161011e565b60208501519093506101678161011e565b60408501519092506001600160401b038116811461018457600080fd5b809150509250925092565b60805160a05160c051613cad61020b600039600061062d0152600081816102bd0152818161066801528181610712015281816109dd01528181610c1801528181610f0101528181610faa015281816111e8015281816115510152818161168801526128010152600081816104e601526110130152613cad6000f3fe60806040526004361061016a5760003560e01c80636fcd0e53116100d1578063c49074421161008a578063dda3346c11610064578063dda3346c146105bb578063ee94d67c146105db578063f074ba62146105fb578063f28824611461061b57600080fd5b8063c49074421461055b578063c4d66de81461057b578063d06d55871461059b57600080fd5b80636fcd0e53146104705780637439841f1461049d57806374cdd798146104d457806388676cad146105085780639b4e463414610528578063b522538a1461053b57600080fd5b80634665bcda116101235780634665bcda146102ab57806347d28372146102df57806352396a59146103cd578063587533571461040357806358eaee79146104235780636c0d2d5a1461045057600080fd5b8063039157d2146101a95780630b18ff66146101cb5780632340e8d3146102085780633474aa161461022c5780633f65cf191461026457806342ecff2a1461028457600080fd5b366101a4576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156101b557600080fd5b506101c96101c4366004613149565b61064f565b005b3480156101d757600080fd5b506033546101eb906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561021457600080fd5b5061021e60395481565b6040519081526020016101ff565b34801561023857600080fd5b5060345461024c906001600160401b031681565b6040516001600160401b0390911681526020016101ff565b34801561027057600080fd5b506101c961027f36600461320b565b610984565b34801561029057600080fd5b50603a5461024c90600160401b90046001600160401b031681565b3480156102b757600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102eb57600080fd5b506103716040805160a081018252600080825260208201819052918101829052606081018290526080810191909152506040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b9004909116608082015290565b6040516101ff9190600060a0820190508251825262ffffff60208401511660208301526001600160401b036040840151166040830152606083015160070b60608301526001600160401b03608084015116608083015292915050565b3480156103d957600080fd5b5061024c6103e83660046132e9565b603b602052600090815260409020546001600160401b031681565b34801561040f57600080fd5b50603e546101eb906001600160a01b031681565b34801561042f57600080fd5b5061044361043e366004613345565b610c82565b6040516101ff91906133be565b34801561045c57600080fd5b5061021e61046b3660046132e9565b610ce7565b34801561047c57600080fd5b5061049061048b3660046133cc565b610dfb565b6040516101ff91906133e5565b3480156104a957600080fd5b506104436104b83660046133cc565b600090815260366020526040902054600160c01b900460ff1690565b3480156104e057600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b34801561051457600080fd5b506101c9610523366004613447565b610ea8565b6101c9610536366004613464565b610f9f565b34801561054757600080fd5b50610490610556366004613345565b6110ea565b34801561056757600080fd5b506101c96105763660046134fb565b6111dd565b34801561058757600080fd5b506101c9610596366004613527565b611329565b3480156105a757600080fd5b506101c96105b6366004613527565b611479565b3480156105c757600080fd5b506101c96105d636600461361a565b61150d565b3480156105e757600080fd5b50603a5461024c906001600160401b031681565b34801561060757600080fd5b506101c96106163660046136f3565b61166f565b34801561062757600080fd5b5061024c7f000000000000000000000000000000000000000000000000000000000000000081565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156106b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106db919061375f565b156106f95760405163840a48d560e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610761573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610785919061375f565b156107a35760405163840a48d560e01b815260040160405180910390fd5b60006107e96107b2858061377c565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a7592505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561085857610858613386565b600281111561086957610869613386565b81525050905080604001516001600160401b0316876001600160401b0316116108a5576040516337e07ffd60e01b815260040160405180910390fd5b6001816060015160028111156108bd576108bd613386565b146108db5760405163d49e19a760e01b815260040160405180910390fd5b61091f6108e8868061377c565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a9992505050565b61093c5760405163161ce5ed60e31b815260040160405180910390fd5b61094e61094888610ce7565b87611ac3565b610971863561095d878061377c565b61096a60208a018a6137c5565b8651611b69565b61097b6000611c94565b50505050505050565b6033546001600160a01b03163314806109a75750603e546001600160a01b031633145b6109c45760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a2c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a50919061375f565b15610a6e5760405163840a48d560e01b815260040160405180910390fd5b8584148015610a7c57508382145b610a99576040516343714afd60e01b815260040160405180910390fd5b603a546001600160401b03600160401b9091048116908a1611610acf576040516337e07ffd60e01b815260040160405180910390fd5b610ae1610adb8a610ce7565b89611ac3565b6000805b87811015610b7a57610b668a358a8a84818110610b0457610b0461380b565b9050602002016020810190610b199190613821565b898985818110610b2b57610b2b61380b565b9050602002810190610b3d91906137c5565b898987818110610b4f57610b4f61380b565b9050602002810190610b61919061377c565b611e17565b610b70908361385e565b9150600101610ae5565b50603a54600160401b90046001600160401b031615610be857610ba1633b9aca0082613887565b603d8054601390610bc3908490600160981b90046001600160401b031661389b565b92506101000a8154816001600160401b0302191690836001600160401b031602179055505b603354604051630257884360e21b81526001600160a01b03918216600482015260248101839052600060448201527f00000000000000000000000000000000000000000000000000000000000000009091169063095e210c90606401600060405180830381600087803b158015610c5e57600080fd5b505af1158015610c72573d6000803e3d6000fd5b5050505050505050505050505050565b600080610cc484848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061227492505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b6000610cf6611fff600c6138ba565b610d096001600160401b038416426138d1565b10610d2757604051637944e66d60e11b815260040160405180910390fd5b604080516001600160401b03841660208201526000918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f1981840301815290829052610d6f91613908565b600060405180830381855afa9150503d8060008114610daa576040519150601f19603f3d011682016040523d82523d6000602084013e610daf565b606091505b5091509150818015610dc2575060008151115b610ddf5760405163558ad0a360e01b815260040160405180910390fd5b80806020019051810190610df39190613924565b949350505050565b610e236040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff166002811115610e8e57610e8e613386565b6002811115610e9f57610e9f613386565b90525092915050565b6033546001600160a01b0316331480610ecb5750603e546001600160a01b031633145b610ee85760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610f50573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f74919061375f565b15610f925760405163840a48d560e01b815260040160405180910390fd5b610f9b82611c94565b5050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610fe857604051633213a66160e21b815260040160405180910390fd5b346801bc16d674ec800000146110115760405163049696b360e31b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611054612309565b8888886040518863ffffffff1660e01b815260040161107896959493929190613992565b6000604051808303818588803b15801561109157600080fd5b505af11580156110a5573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e2385856040516110db9291906139e1565b60405180910390a15050505050565b6111126040805160808101825260008082526020820181905291810182905290606082015290565b6036600061115585858080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061227492505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff1660028111156111c2576111c2613386565b60028111156111d3576111d3613386565b9052509392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461122657604051633213a66160e21b815260040160405180910390fd5b611234633b9aca00826139f5565b15611252576040516321ddeb1760e21b815260040160405180910390fd5b6000611262633b9aca0083613887565b6034549091506001600160401b039081169082161115611295576040516302c6f54760e21b815260040160405180910390fd5b603480548291906000906112b39084906001600160401b0316613a09565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e8360405161131291815260200190565b60405180910390a2611324838361234e565b505050565b600054610100900460ff16158080156113495750600054600160ff909116105b806113635750303b158015611363575060005460ff166001145b6113cb5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156113ee576000805461ff0019166101001790555b6001600160a01b038216611415576040516339b190bb60e11b815260040160405180910390fd5b603380546001600160a01b0319166001600160a01b0384161790558015610f9b576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b031633146114a45760405163719f370360e11b815260040160405180910390fd5b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b031633146115385760405163719f370360e11b815260040160405180910390fd5b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156115a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115c4919061375f565b156115e25760405163840a48d560e01b815260040160405180910390fd5b8251845114611604576040516343714afd60e01b815260040160405180910390fd5b60005b845181101561166857611660838583815181106116265761162661380b565b60200260200101518784815181106116405761164061380b565b60200260200101516001600160a01b03166124679092919063ffffffff16565b600101611607565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156116d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116fb919061375f565b156117195760405163840a48d560e01b815260040160405180910390fd5b603a54600160401b90046001600160401b0316600081900361174e57604051631a544f4960e01b815260040160405180910390fd5b6040805160a081018252603c54808252603d5462ffffff811660208401526001600160401b0363010000008204811694840194909452600160581b810460070b6060840152600160981b90049092166080820152906117ad90876124b9565b6000805b85811015611a1b57368787838181106117cc576117cc61380b565b90506020028101906117de9190613a28565b80356000908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561184f5761184f613386565b600281111561186057611860613386565b905250905060018160600151600281111561187d5761187d613386565b14611889575050611a13565b856001600160401b031681604001516001600160401b0316106118ad575050611a13565b600080806118be848a8f358861256b565b60208b01805193965091945092506118d582613a3e565b62ffffff169052506080880180518491906118f190839061389b565b6001600160401b0316905250606088018051839190611911908390613a5d565b60070b905250611921818861389b565b85356000908152603660209081526040918290208751815492890151938901516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060880151939a50879390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156119c6576119c6613386565b021790555050845160405164ffffffffff90911691506001600160401b038b16907fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f90600090a350505050505b6001016117b1565b506001600160401b038084166000908152603b6020526040812080548493919291611a489185911661389b565b92506101000a8154816001600160401b0302191690836001600160401b0316021790555061097b82612691565b600081600081518110611a8a57611a8a61380b565b60200260200101519050919050565b600081600381518110611aae57611aae61380b565b60200260200101516000801b14159050919050565b611acf600360206138ba565b611adc60208301836137c5565b905014611afc576040516313717da960e21b815260040160405180910390fd5b611b4c611b0c60208301836137c5565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003612938565b610f9b576040516309bde33960e01b815260040160405180910390fd5b60088414611b8a5760405163200591bd60e01b815260040160405180910390fd5b6005611b986028600161385e565b611ba2919061385e565b611bad9060206138ba565b8214611bcc576040516313717da960e21b815260040160405180910390fd5b6000611c0a86868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061295092505050565b9050600064ffffffffff8316611c226028600161385e565b600b901b179050611c6d85858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050612938565b611c8a576040516309bde33960e01b815260040160405180910390fd5b5050505050505050565b603a54600160401b90046001600160401b031615611cc45760405162be9bc360e81b815260040160405180910390fd5b603a546001600160401b03428116911603611cf2576040516367db5b8b60e01b815260040160405180910390fd5b6034546000906001600160401b0316611d0f633b9aca0047613887565b611d199190613a09565b9050818015611d2f57506001600160401b038116155b15611d4d576040516332dea95960e21b815260040160405180910390fd5b60006040518060a00160405280611d6342610ce7565b815260395462ffffff1660208201526001600160401b038085166040830152600060608301819052608090920191909152603a805442909216600160401b026fffffffffffffffff0000000000000000199092169190911790559050611dc881612691565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b600080611e56848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a7592505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611ec557611ec5613386565b6002811115611ed657611ed6613386565b9052509050600081606001516002811115611ef357611ef3613386565b14611f11576040516335e09e9d60e01b815260040160405180910390fd5b6001600160401b038016611f57868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612be992505050565b6001600160401b031603611f7e57604051631958236d60e21b815260040160405180910390fd5b6001600160401b038016611fc4868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c0e92505050565b6001600160401b031614611feb57604051632eade63760e01b815260040160405180910390fd5b611ff3612309565b611ffc90613a8c565b612038868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c2692505050565b1461205657604051632230566760e11b815260040160405180910390fd5b6000612094868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c3b92505050565b90506120a48a87878b8b8e611b69565b603980549060006120b483613ab0565b9091555050603a546001600160401b0380821691600160401b900416156120ea5750603a54600160401b90046001600160401b03165b6040805160808101825264ffffffffff8c1681526001600160401b03848116602083015283169181019190915260608101600190526000858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156121bd576121bd613386565b02179055505060405164ffffffffff8c1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a16040805164ffffffffff8c1681526001600160401b03838116602083015284168183015290517f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df9181900360600190a1612265633b9aca006001600160401b0384166138ba565b9b9a5050505050505050505050565b6000815160301461229857604051634f88323960e11b815260040160405180910390fd5b6040516002906122af908490600090602001613ac9565b60408051601f19818403018152908290526122c991613908565b602060405180830381855afa1580156122e6573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190610ce19190613924565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b8047101561239e5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016113c2565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146123eb576040519150601f19603f3d011682016040523d82523d6000602084013e6123f0565b606091505b50509050806113245760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016113c2565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611324908490612c53565b6124c56005600361385e565b6124d09060206138ba565b6124dd60208301836137c5565b9050146124fd576040516313717da960e21b815260040160405180910390fd5b606c61254e61250f60208401846137c5565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084612938565b611324576040516309bde33960e01b815260040160405180910390fd5b8351602085015190600090819081612584878388612d28565b9050846001600160401b0316816001600160401b0316146125fe576125a98186612e09565b6040805164ffffffffff851681526001600160401b038b8116602083015284168183015290519195507f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df919081900360600190a15b6001600160401b0380821660208b0181905290891660408b0152600003612685576039805490600061262f83613af8565b9091555050600260608a015261264484613b0f565b92508164ffffffffff16886001600160401b03167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a60405160405180910390a35b50509450945094915050565b806020015162ffffff166000036128a6576000633b9aca00826060015160070b83604001516001600160401b03166126c99190613b36565b600f0b6126d69190613b75565b60408301516034805492935090916000906126fb9084906001600160401b031661389b565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b81049092166001600160801b0319909216919091179055506000603c819055603d80546001600160d81b0319169055808212156127c9576080830151603454600091633b9aca009161277f91906001600160401b031661389b565b6001600160401b031661279291906138ba565b905080670de0b6b3a76400006127a785613ba5565b6127b1908461385e565b6127bb91906138ba565b6127c59190613887565b9150505b603354604051630257884360e21b81526001600160a01b039182166004820152602481018490526001600160401b03831660448201527f00000000000000000000000000000000000000000000000000000000000000009091169063095e210c90606401600060405180830381600087803b15801561284757600080fd5b505af115801561285b573d6000803e3d6000fd5b5050603a546040518581526001600160401b0390911692507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44915060200160405180910390a2505050565b8051603c556020810151603d805460408401516060850151608086015162ffffff9095166affffffffffffffffffffff199093169290921763010000006001600160401b0392831602176fffffffffffffffffffffffffffffffff60581b1916600160581b9282169290920267ffffffffffffffff60981b191691909117600160981b91909316029190911790555b50565b600083612946868585612e1c565b1495945050505050565b600080600283516129619190613887565b90506000816001600160401b0381111561297d5761297d613544565b6040519080825280602002602001820160405280156129a6578160200160208202803683370190505b50905060005b82811015612aa3576002856129c183836138ba565b815181106129d1576129d161380b565b6020026020010151868360026129e791906138ba565b6129f290600161385e565b81518110612a0257612a0261380b565b6020026020010151604051602001612a24929190918252602082015260400190565b60408051601f1981840301815290829052612a3e91613908565b602060405180830381855afa158015612a5b573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190612a7e9190613924565b828281518110612a9057612a9061380b565b60209081029190910101526001016129ac565b50612aaf600283613887565b91505b8115612bc55760005b82811015612bb257600282612ad083836138ba565b81518110612ae057612ae061380b565b602002602001015183836002612af691906138ba565b612b0190600161385e565b81518110612b1157612b1161380b565b6020026020010151604051602001612b33929190918252602082015260400190565b60408051601f1981840301815290829052612b4d91613908565b602060405180830381855afa158015612b6a573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190612b8d9190613924565b828281518110612b9f57612b9f61380b565b6020908102919091010152600101612abb565b50612bbe600283613887565b9150612ab2565b80600081518110612bd857612bd861380b565b602002602001015192505050919050565b6000610ce182600581518110612c0157612c0161380b565b6020026020010151612ef9565b6000610ce182600681518110612c0157612c0161380b565b600081600181518110611a8a57611a8a61380b565b6000610ce182600281518110612c0157612c0161380b565b6000612ca8826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316612f609092919063ffffffff16565b9050805160001480612cc9575080806020019051810190612cc9919061375f565b6113245760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016113c2565b6000612d366026600161385e565b612d419060206138ba565b612d4e60408401846137c5565b905014612d6e576040516313717da960e21b815260040160405180910390fd5b6000612d7b600485613bc1565b64ffffffffff169050612dd5612d9460408501856137c5565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584612938565b612df2576040516309bde33960e01b815260040160405180910390fd5b612e00836020013585612f6f565b95945050505050565b6000612e158284613beb565b9392505050565b60008351600014158015612e3b575060208451612e3991906139f5565b155b612e58576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111612eef57612e7c6002856139f5565b600003612eb2578151600052808601516020526020826040600060026107d05a03fa612ea757600080fd5b600284049350612edd565b8086015160005281516020526020826040600060026107d05a03fa612ed657600080fd5b6002840493505b612ee860208261385e565b9050612e69565b5051949350505050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6060610df38484600085612f9c565b600080612f7d600484613c1a565b612f88906040613c44565b64ffffffffff169050610df384821b612ef9565b606082471015612ffd5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016113c2565b600080866001600160a01b031685876040516130199190613908565b60006040518083038185875af1925050503d8060008114613056576040519150601f19603f3d011682016040523d82523d6000602084013e61305b565b606091505b509150915061306c87838387613077565b979650505050505050565b606083156130e65782516000036130df576001600160a01b0385163b6130df5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016113c2565b5081610df3565b610df383838151156130fb5781518083602001fd5b8060405162461bcd60e51b81526004016113c29190613c64565b80356001600160401b038116811461312c57600080fd5b919050565b60006040828403121561314357600080fd5b50919050565b60008060006060848603121561315e57600080fd5b61316784613115565b925060208401356001600160401b0381111561318257600080fd5b61318e86828701613131565b92505060408401356001600160401b038111156131aa57600080fd5b6131b686828701613131565b9150509250925092565b60008083601f8401126131d257600080fd5b5081356001600160401b038111156131e957600080fd5b6020830191508360208260051b850101111561320457600080fd5b9250929050565b60008060008060008060008060a0898b03121561322757600080fd5b61323089613115565b975060208901356001600160401b0381111561324b57600080fd5b6132578b828c01613131565b97505060408901356001600160401b0381111561327357600080fd5b61327f8b828c016131c0565b90975095505060608901356001600160401b0381111561329e57600080fd5b6132aa8b828c016131c0565b90955093505060808901356001600160401b038111156132c957600080fd5b6132d58b828c016131c0565b999c989b5096995094979396929594505050565b6000602082840312156132fb57600080fd5b612e1582613115565b60008083601f84011261331657600080fd5b5081356001600160401b0381111561332d57600080fd5b60208301915083602082850101111561320457600080fd5b6000806020838503121561335857600080fd5b82356001600160401b0381111561336e57600080fd5b61337a85828601613304565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b600381106133ba57634e487b7160e01b600052602160045260246000fd5b9052565b60208101610ce1828461339c565b6000602082840312156133de57600080fd5b5035919050565b60006080820190506001600160401b0383511682526001600160401b0360208401511660208301526001600160401b0360408401511660408301526060830151613432606084018261339c565b5092915050565b801515811461293557600080fd5b60006020828403121561345957600080fd5b8135612e1581613439565b60008060008060006060868803121561347c57600080fd5b85356001600160401b0381111561349257600080fd5b61349e88828901613304565b90965094505060208601356001600160401b038111156134bd57600080fd5b6134c988828901613304565b96999598509660400135949350505050565b6001600160a01b038116811461293557600080fd5b803561312c816134db565b6000806040838503121561350e57600080fd5b8235613519816134db565b946020939093013593505050565b60006020828403121561353957600080fd5b8135612e15816134db565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b038111828210171561358257613582613544565b604052919050565b60006001600160401b038211156135a3576135a3613544565b5060051b60200190565b600082601f8301126135be57600080fd5b81356135d16135cc8261358a565b61355a565b8082825260208201915060208360051b8601019250858311156135f357600080fd5b602085015b838110156136105780358352602092830192016135f8565b5095945050505050565b60008060006060848603121561362f57600080fd5b83356001600160401b0381111561364557600080fd5b8401601f8101861361365657600080fd5b80356136646135cc8261358a565b8082825260208201915060208360051b85010192508883111561368657600080fd5b6020840193505b828410156136b15783356136a0816134db565b82526020938401939091019061368d565b955050505060208401356001600160401b038111156136cf57600080fd5b6136db868287016135ad565b9250506136ea604085016134f0565b90509250925092565b60008060006040848603121561370857600080fd5b83356001600160401b0381111561371e57600080fd5b61372a86828701613131565b93505060208401356001600160401b0381111561374657600080fd5b613752868287016131c0565b9497909650939450505050565b60006020828403121561377157600080fd5b8151612e1581613439565b6000808335601e1984360301811261379357600080fd5b8301803591506001600160401b038211156137ad57600080fd5b6020019150600581901b360382131561320457600080fd5b6000808335601e198436030181126137dc57600080fd5b8301803591506001600160401b038211156137f657600080fd5b60200191503681900382131561320457600080fd5b634e487b7160e01b600052603260045260246000fd5b60006020828403121561383357600080fd5b813564ffffffffff81168114612e1557600080fd5b634e487b7160e01b600052601160045260246000fd5b80820180821115610ce157610ce1613848565b634e487b7160e01b600052601260045260246000fd5b60008261389657613896613871565b500490565b6001600160401b038181168382160190811115610ce157610ce1613848565b8082028115828204841417610ce157610ce1613848565b81810381811115610ce157610ce1613848565b60005b838110156138ff5781810151838201526020016138e7565b50506000910152565b6000825161391a8184602087016138e4565b9190910192915050565b60006020828403121561393657600080fd5b5051919050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000815180845261397e8160208601602086016138e4565b601f01601f19169290920160200192915050565b6080815260006139a660808301888a61393d565b82810360208401526139b88188613966565b905082810360408401526139cd81868861393d565b915050826060830152979650505050505050565b602081526000610df360208301848661393d565b600082613a0457613a04613871565b500690565b6001600160401b038281168282160390811115610ce157610ce1613848565b60008235605e1983360301811261391a57600080fd5b600062ffffff821680613a5357613a53613848565b6000190192915050565b600781810b9083900b01677fffffffffffffff8113677fffffffffffffff1982121715610ce157610ce1613848565b805160208083015191908110156131435760001960209190910360031b1b16919050565b600060018201613ac257613ac2613848565b5060010190565b60008351613adb8184602088016138e4565b6001600160801b0319939093169190920190815260100192915050565b600081613b0757613b07613848565b506000190190565b60008160070b677fffffffffffffff198103613b2d57613b2d613848565b60000392915050565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81136f7fffffffffffffffffffffffffffffff1982121715610ce157610ce1613848565b80820260008212600160ff1b84141615613b9157613b91613848565b8181058314821517610ce157610ce1613848565b6000600160ff1b8201613bba57613bba613848565b5060000390565b600064ffffffffff831680613bd857613bd8613871565b8064ffffffffff84160491505092915050565b600782810b9082900b03677fffffffffffffff198112677fffffffffffffff82131715610ce157610ce1613848565b600064ffffffffff831680613c3157613c31613871565b8064ffffffffff84160691505092915050565b64ffffffffff818116838216029081169081811461343257613432613848565b602081526000612e15602083018461396656fea2646970667358221220fe6e12820da20dc3e43fbf9e21d1099a7e642fa46ba78f0a42285dde7df11dfe64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa>\xB88\x03\x80a>\xB8\x839\x81\x01`@\x81\x90Ra\0/\x91a\x016V[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x82\x16`\xA0R`\x01`\x01`@\x1B\x03\x81\x16`\xC0Ra\0Wa\0_V[PPPa\x01\x8FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15a\0\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x14a\x01\x1CW`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x013W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x01KW`\0\x80\xFD[\x83Qa\x01V\x81a\x01\x1EV[` \x85\x01Q\x90\x93Pa\x01g\x81a\x01\x1EV[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x01\x84W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0Qa<\xADa\x02\x0B`\09`\0a\x06-\x01R`\0\x81\x81a\x02\xBD\x01R\x81\x81a\x06h\x01R\x81\x81a\x07\x12\x01R\x81\x81a\t\xDD\x01R\x81\x81a\x0C\x18\x01R\x81\x81a\x0F\x01\x01R\x81\x81a\x0F\xAA\x01R\x81\x81a\x11\xE8\x01R\x81\x81a\x15Q\x01R\x81\x81a\x16\x88\x01Ra(\x01\x01R`\0\x81\x81a\x04\xE6\x01Ra\x10\x13\x01Ra<\xAD`\0\xF3\xFE`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80co\xCD\x0ES\x11a\0\xD1W\x80c\xC4\x90tB\x11a\0\x8AW\x80c\xDD\xA34l\x11a\0dW\x80c\xDD\xA34l\x14a\x05\xBBW\x80c\xEE\x94\xD6|\x14a\x05\xDBW\x80c\xF0t\xBAb\x14a\x05\xFBW\x80c\xF2\x88$a\x14a\x06\x1BW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x05[W\x80c\xC4\xD6m\xE8\x14a\x05{W\x80c\xD0mU\x87\x14a\x05\x9BW`\0\x80\xFD[\x80co\xCD\x0ES\x14a\x04pW\x80ct9\x84\x1F\x14a\x04\x9DW\x80ct\xCD\xD7\x98\x14a\x04\xD4W\x80c\x88gl\xAD\x14a\x05\x08W\x80c\x9BNF4\x14a\x05(W\x80c\xB5\"S\x8A\x14a\x05;W`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01#W\x80cFe\xBC\xDA\x14a\x02\xABW\x80cG\xD2\x83r\x14a\x02\xDFW\x80cR9jY\x14a\x03\xCDW\x80cXu3W\x14a\x04\x03W\x80cX\xEA\xEEy\x14a\x04#W\x80cl\r-Z\x14a\x04PW`\0\x80\xFD[\x80c\x03\x91W\xD2\x14a\x01\xA9W\x80c\x0B\x18\xFFf\x14a\x01\xCBW\x80c#@\xE8\xD3\x14a\x02\x08W\x80c4t\xAA\x16\x14a\x02,W\x80c?e\xCF\x19\x14a\x02dW\x80cB\xEC\xFF*\x14a\x02\x84W`\0\x80\xFD[6a\x01\xA4W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a1IV[a\x06OV[\0[4\x80\x15a\x01\xD7W`\0\x80\xFD[P`3Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x1E`9T\x81V[`@Q\x90\x81R` \x01a\x01\xFFV[4\x80\x15a\x028W`\0\x80\xFD[P`4Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x01\xC9a\x02\x7F6`\x04a2\x0BV[a\t\x84V[4\x80\x15a\x02\x90W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x02\xB7W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x03q`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x01\xFF\x91\x90`\0`\xA0\x82\x01\x90P\x82Q\x82Rb\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R`\x01`\x01`@\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x03\xD9W`\0\x80\xFD[Pa\x02La\x03\xE86`\x04a2\xE9V[`;` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04\x0FW`\0\x80\xFD[P`>Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x04Ca\x04>6`\x04a3EV[a\x0C\x82V[`@Qa\x01\xFF\x91\x90a3\xBEV[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x02\x1Ea\x04k6`\x04a2\xE9V[a\x0C\xE7V[4\x80\x15a\x04|W`\0\x80\xFD[Pa\x04\x90a\x04\x8B6`\x04a3\xCCV[a\r\xFBV[`@Qa\x01\xFF\x91\x90a3\xE5V[4\x80\x15a\x04\xA9W`\0\x80\xFD[Pa\x04Ca\x04\xB86`\x04a3\xCCV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x01\xC9a\x05#6`\x04a4GV[a\x0E\xA8V[a\x01\xC9a\x0566`\x04a4dV[a\x0F\x9FV[4\x80\x15a\x05GW`\0\x80\xFD[Pa\x04\x90a\x05V6`\x04a3EV[a\x10\xEAV[4\x80\x15a\x05gW`\0\x80\xFD[Pa\x01\xC9a\x05v6`\x04a4\xFBV[a\x11\xDDV[4\x80\x15a\x05\x87W`\0\x80\xFD[Pa\x01\xC9a\x05\x966`\x04a5'V[a\x13)V[4\x80\x15a\x05\xA7W`\0\x80\xFD[Pa\x01\xC9a\x05\xB66`\x04a5'V[a\x14yV[4\x80\x15a\x05\xC7W`\0\x80\xFD[Pa\x01\xC9a\x05\xD66`\x04a6\x1AV[a\x15\rV[4\x80\x15a\x05\xE7W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x01\xC9a\x06\x166`\x04a6\xF3V[a\x16oV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x02L\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDB\x91\x90a7_V[\x15a\x06\xF9W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a7_V[\x15a\x07\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xE9a\x07\xB2\x85\x80a7|V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1Au\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x08XWa\x08Xa3\x86V[`\x02\x81\x11\x15a\x08iWa\x08ia3\x86V[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\x08\xA5W`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81``\x01Q`\x02\x81\x11\x15a\x08\xBDWa\x08\xBDa3\x86V[\x14a\x08\xDBW`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x1Fa\x08\xE8\x86\x80a7|V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x99\x92PPPV[a\t<W`@Qc\x16\x1C\xE5\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tNa\tH\x88a\x0C\xE7V[\x87a\x1A\xC3V[a\tq\x865a\t]\x87\x80a7|V[a\tj` \x8A\x01\x8Aa7\xC5V[\x86Qa\x1BiV[a\t{`\0a\x1C\x94V[PPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\t\xA7WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\t\xC4W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nP\x91\x90a7_V[\x15a\nnW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14\x80\x15a\n|WP\x83\x82\x14[a\n\x99W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\n\xCFW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xE1a\n\xDB\x8Aa\x0C\xE7V[\x89a\x1A\xC3V[`\0\x80[\x87\x81\x10\x15a\x0BzWa\x0Bf\x8A5\x8A\x8A\x84\x81\x81\x10a\x0B\x04Wa\x0B\x04a8\x0BV[\x90P` \x02\x01` \x81\x01\x90a\x0B\x19\x91\x90a8!V[\x89\x89\x85\x81\x81\x10a\x0B+Wa\x0B+a8\x0BV[\x90P` \x02\x81\x01\x90a\x0B=\x91\x90a7\xC5V[\x89\x89\x87\x81\x81\x10a\x0BOWa\x0BOa8\x0BV[\x90P` \x02\x81\x01\x90a\x0Ba\x91\x90a7|V[a\x1E\x17V[a\x0Bp\x90\x83a8^V[\x91P`\x01\x01a\n\xE5V[P`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a\x0B\xE8Wa\x0B\xA1c;\x9A\xCA\0\x82a8\x87V[`=\x80T`\x13\x90a\x0B\xC3\x90\x84\x90`\x01`\x98\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a8\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[`3T`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`\0`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^!\x0C\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CrW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[`\0\x80a\x0C\xC4\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"t\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`\0a\x0C\xF6a\x1F\xFF`\x0Ca8\xBAV[a\r\t`\x01`\x01`@\x1B\x03\x84\x16Ba8\xD1V[\x10a\r'W`@QcyD\xE6m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\0\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\ro\x91a9\x08V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xAAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xAFV[``\x91P[P\x91P\x91P\x81\x80\x15a\r\xC2WP`\0\x81Q\x11[a\r\xDFW`@QcU\x8A\xD0\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\r\xF3\x91\x90a9$V[\x94\x93PPPPV[a\x0E#`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\x8EWa\x0E\x8Ea3\x86V[`\x02\x81\x11\x15a\x0E\x9FWa\x0E\x9Fa3\x86V[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0E\xCBWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x0E\xE8W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ft\x91\x90a7_V[\x15a\x0F\x92W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9B\x82a\x1C\x94V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\xE8W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x10\x11W`@Qc\x04\x96\x96\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x10Ta#\tV[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10x\x96\x95\x94\x93\x92\x91\x90a9\x92V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x10\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xA5W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x10\xDB\x92\x91\x90a9\xE1V[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x11\x12`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x11U\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"t\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x11\xC2Wa\x11\xC2a3\x86V[`\x02\x81\x11\x15a\x11\xD3Wa\x11\xD3a3\x86V[\x90RP\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12&W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x124c;\x9A\xCA\0\x82a9\xF5V[\x15a\x12RW`@Qc!\xDD\xEB\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12bc;\x9A\xCA\0\x83a8\x87V[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x12\x95W`@Qc\x02\xC6\xF5G`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4\x80T\x82\x91\x90`\0\x90a\x12\xB3\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a:\tV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x13\x12\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x13$\x83\x83a#NV[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13IWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13cWP0;\x15\x80\x15a\x13cWP`\0T`\xFF\x16`\x01\x14[a\x13\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13\xEEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x15W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F\x9BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xA4W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x158W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC4\x91\x90a7_V[\x15a\x15\xE2W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x84Q\x14a\x16\x04W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84Q\x81\x10\x15a\x16hWa\x16`\x83\x85\x83\x81Q\x81\x10a\x16&Wa\x16&a8\x0BV[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x16@Wa\x16@a8\x0BV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a$g\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\x16\x07V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xFB\x91\x90a7_V[\x15a\x17\x19W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\0\x81\x90\x03a\x17NW`@Qc\x1ATOI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R`<T\x80\x82R`=Tb\xFF\xFF\xFF\x81\x16` \x84\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`X\x1B\x81\x04`\x07\x0B``\x84\x01R`\x01`\x98\x1B\x90\x04\x90\x92\x16`\x80\x82\x01R\x90a\x17\xAD\x90\x87a$\xB9V[`\0\x80[\x85\x81\x10\x15a\x1A\x1BW6\x87\x87\x83\x81\x81\x10a\x17\xCCWa\x17\xCCa8\x0BV[\x90P` \x02\x81\x01\x90a\x17\xDE\x91\x90a:(V[\x805`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18OWa\x18Oa3\x86V[`\x02\x81\x11\x15a\x18`Wa\x18`a3\x86V[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x18}Wa\x18}a3\x86V[\x14a\x18\x89WPPa\x1A\x13V[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x18\xADWPPa\x1A\x13V[`\0\x80\x80a\x18\xBE\x84\x8A\x8F5\x88a%kV[` \x8B\x01\x80Q\x93\x96P\x91\x94P\x92Pa\x18\xD5\x82a:>V[b\xFF\xFF\xFF\x16\x90RP`\x80\x88\x01\x80Q\x84\x91\x90a\x18\xF1\x90\x83\x90a8\x9BV[`\x01`\x01`@\x1B\x03\x16\x90RP``\x88\x01\x80Q\x83\x91\x90a\x19\x11\x90\x83\x90a:]V[`\x07\x0B\x90RPa\x19!\x81\x88a8\x9BV[\x855`\0\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x92\x89\x01Q\x93\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x88\x01Q\x93\x9AP\x87\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\x19\xC6Wa\x19\xC6a3\x86V[\x02\x17\x90UPP\x84Q`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?\x90`\0\x90\xA3PPPPP[`\x01\x01a\x17\xB1V[P`\x01`\x01`@\x1B\x03\x80\x84\x16`\0\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a\x1AH\x91\x85\x91\x16a8\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\t{\x82a&\x91V[`\0\x81`\0\x81Q\x81\x10a\x1A\x8AWa\x1A\x8Aa8\x0BV[` \x02` \x01\x01Q\x90P\x91\x90PV[`\0\x81`\x03\x81Q\x81\x10a\x1A\xAEWa\x1A\xAEa8\x0BV[` \x02` \x01\x01Q`\0\x80\x1B\x14\x15\x90P\x91\x90PV[a\x1A\xCF`\x03` a8\xBAV[a\x1A\xDC` \x83\x01\x83a7\xC5V[\x90P\x14a\x1A\xFCW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1BLa\x1B\x0C` \x83\x01\x83a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a)8V[a\x0F\x9BW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x84\x14a\x1B\x8AW`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05a\x1B\x98`(`\x01a8^V[a\x1B\xA2\x91\x90a8^V[a\x1B\xAD\x90` a8\xBAV[\x82\x14a\x1B\xCCW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\n\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)P\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1C\"`(`\x01a8^V[`\x0B\x90\x1B\x17\x90Pa\x1Cm\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa)8V[a\x1C\x8AW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a\x1C\xC4W`@Qb\xBE\x9B\xC3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03B\x81\x16\x91\x16\x03a\x1C\xF2W`@Qcg\xDB[\x8B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4T`\0\x90`\x01`\x01`@\x1B\x03\x16a\x1D\x0Fc;\x9A\xCA\0Ga8\x87V[a\x1D\x19\x91\x90a:\tV[\x90P\x81\x80\x15a\x1D/WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x1DMW`@Qc2\xDE\xA9Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xA0\x01`@R\x80a\x1DcBa\x0C\xE7V[\x81R`9Tb\xFF\xFF\xFF\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x80\x85\x16`@\x83\x01R`\0``\x83\x01\x81\x90R`\x80\x90\x92\x01\x91\x90\x91R`:\x80TB\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x90Pa\x1D\xC8\x81a&\x91V[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x1EV\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1Au\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xC5Wa\x1E\xC5a3\x86V[`\x02\x81\x11\x15a\x1E\xD6Wa\x1E\xD6a3\x86V[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a\x1E\xF3Wa\x1E\xF3a3\x86V[\x14a\x1F\x11W`@Qc5\xE0\x9E\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a\x1FW\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa+\xE9\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x03a\x1F~W`@Qc\x19X#m`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a\x1F\xC4\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a\x1F\xEBW`@Qc.\xAD\xE67`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xF3a#\tV[a\x1F\xFC\x90a:\x8CV[a 8\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,&\x92PPPV[\x14a VW`@Qc\"0Vg`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \x94\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,;\x92PPPV[\x90Pa \xA4\x8A\x87\x87\x8B\x8B\x8Ea\x1BiV[`9\x80T\x90`\0a \xB4\x83a:\xB0V[\x90\x91UPP`:T`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x15a \xEAWP`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R``\x81\x01`\x01\x90R`\0\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a!\xBDWa!\xBDa3\x86V[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x81\x90\x03``\x01\x90\xA1a\"ec;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16a8\xBAV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a\"\x98W`@QcO\x8829`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x02\x90a\"\xAF\x90\x84\x90`\0\x90` \x01a:\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\xC9\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\"\xE6W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE1\x91\x90a9$V[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80G\x10\x15a#\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x13\xC2V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#\xF0V[``\x91P[PP\x90P\x80a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x13\xC2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x13$\x90\x84\x90a,SV[a$\xC5`\x05`\x03a8^V[a$\xD0\x90` a8\xBAV[a$\xDD` \x83\x01\x83a7\xC5V[\x90P\x14a$\xFDW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`la%Na%\x0F` \x84\x01\x84a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a)8V[a\x13$W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q` \x85\x01Q\x90`\0\x90\x81\x90\x81a%\x84\x87\x83\x88a-(V[\x90P\x84`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a%\xFEWa%\xA9\x81\x86a.\tV[`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`@\x1B\x03\x8B\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x95P\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8B\x01\x81\x90R\x90\x89\x16`@\x8B\x01R`\0\x03a&\x85W`9\x80T\x90`\0a&/\x83a:\xF8V[\x90\x91UPP`\x02``\x8A\x01Ra&D\x84a;\x0FV[\x92P\x81d\xFF\xFF\xFF\xFF\xFF\x16\x88`\x01`\x01`@\x1B\x03\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj`@Q`@Q\x80\x91\x03\x90\xA3[PP\x94P\x94P\x94\x91PPV[\x80` \x01Qb\xFF\xFF\xFF\x16`\0\x03a(\xA6W`\0c;\x9A\xCA\0\x82``\x01Q`\x07\x0B\x83`@\x01Q`\x01`\x01`@\x1B\x03\x16a&\xC9\x91\x90a;6V[`\x0F\x0Ba&\xD6\x91\x90a;uV[`@\x83\x01Q`4\x80T\x92\x93P\x90\x91`\0\x90a&\xFB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a8\x9BV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UP`\0`<\x81\x90U`=\x80T`\x01`\x01`\xD8\x1B\x03\x19\x16\x90U\x80\x82\x12\x15a'\xC9W`\x80\x83\x01Q`4T`\0\x91c;\x9A\xCA\0\x91a'\x7F\x91\x90`\x01`\x01`@\x1B\x03\x16a8\x9BV[`\x01`\x01`@\x1B\x03\x16a'\x92\x91\x90a8\xBAV[\x90P\x80g\r\xE0\xB6\xB3\xA7d\0\0a'\xA7\x85a;\xA5V[a'\xB1\x90\x84a8^V[a'\xBB\x91\x90a8\xBAV[a'\xC5\x91\x90a8\x87V[\x91PP[`3T`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`@\x1B\x03\x83\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^!\x0C\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a(GW`\0\x80\xFD[PZ\xF1\x15\x80\x15a([W=`\0\x80>=`\0\xFD[PP`:T`@Q\x85\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16\x92P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x91P` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Q`\x80\x86\x01Qb\xFF\xFF\xFF\x90\x95\x16j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17c\x01\0\0\0`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19\x16`\x01`X\x1B\x92\x82\x16\x92\x90\x92\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16\x91\x90\x91\x17`\x01`\x98\x1B\x91\x90\x93\x16\x02\x91\x90\x91\x17\x90U[PV[`\0\x83a)F\x86\x85\x85a.\x1CV[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa)a\x91\x90a8\x87V[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a)}Wa)}a5DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a*\xA3W`\x02\x85a)\xC1\x83\x83a8\xBAV[\x81Q\x81\x10a)\xD1Wa)\xD1a8\x0BV[` \x02` \x01\x01Q\x86\x83`\x02a)\xE7\x91\x90a8\xBAV[a)\xF2\x90`\x01a8^V[\x81Q\x81\x10a*\x02Wa*\x02a8\x0BV[` \x02` \x01\x01Q`@Q` \x01a*$\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*>\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a*[W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*~\x91\x90a9$V[\x82\x82\x81Q\x81\x10a*\x90Wa*\x90a8\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a)\xACV[Pa*\xAF`\x02\x83a8\x87V[\x91P[\x81\x15a+\xC5W`\0[\x82\x81\x10\x15a+\xB2W`\x02\x82a*\xD0\x83\x83a8\xBAV[\x81Q\x81\x10a*\xE0Wa*\xE0a8\x0BV[` \x02` \x01\x01Q\x83\x83`\x02a*\xF6\x91\x90a8\xBAV[a+\x01\x90`\x01a8^V[\x81Q\x81\x10a+\x11Wa+\x11a8\x0BV[` \x02` \x01\x01Q`@Q` \x01a+3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra+M\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a+jW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x8D\x91\x90a9$V[\x82\x82\x81Q\x81\x10a+\x9FWa+\x9Fa8\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*\xBBV[Pa+\xBE`\x02\x83a8\x87V[\x91Pa*\xB2V[\x80`\0\x81Q\x81\x10a+\xD8Wa+\xD8a8\x0BV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0a\x0C\xE1\x82`\x05\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[` \x02` \x01\x01Qa.\xF9V[`\0a\x0C\xE1\x82`\x06\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[`\0\x81`\x01\x81Q\x81\x10a\x1A\x8AWa\x1A\x8Aa8\x0BV[`\0a\x0C\xE1\x82`\x02\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[`\0a,\xA8\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a/`\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a,\xC9WP\x80\x80` \x01\x90Q\x81\x01\x90a,\xC9\x91\x90a7_V[a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x13\xC2V[`\0a-6`&`\x01a8^V[a-A\x90` a8\xBAV[a-N`@\x84\x01\x84a7\xC5V[\x90P\x14a-nW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a-{`\x04\x85a;\xC1V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa-\xD5a-\x94`@\x85\x01\x85a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a)8V[a-\xF2W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.\0\x83` \x015\x85a/oV[\x95\x94PPPPPV[`\0a.\x15\x82\x84a;\xEBV[\x93\x92PPPV[`\0\x83Q`\0\x14\x15\x80\x15a.;WP` \x84Qa.9\x91\x90a9\xF5V[\x15[a.XW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a.\xEFWa.|`\x02\x85a9\xF5V[`\0\x03a.\xB2W\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa.\xA7W`\0\x80\xFD[`\x02\x84\x04\x93Pa.\xDDV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa.\xD6W`\0\x80\xFD[`\x02\x84\x04\x93P[a.\xE8` \x82a8^V[\x90Pa.iV[PQ\x94\x93PPPPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\r\xF3\x84\x84`\0\x85a/\x9CV[`\0\x80a/}`\x04\x84a<\x1AV[a/\x88\x90`@a<DV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\r\xF3\x84\x82\x1Ba.\xF9V[``\x82G\x10\x15a/\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x13\xC2V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa0\x19\x91\x90a9\x08V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a0VW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0[V[``\x91P[P\x91P\x91Pa0l\x87\x83\x83\x87a0wV[\x97\x96PPPPPPPV[``\x83\x15a0\xE6W\x82Q`\0\x03a0\xDFW`\x01`\x01`\xA0\x1B\x03\x85\x16;a0\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x13\xC2V[P\x81a\r\xF3V[a\r\xF3\x83\x83\x81Q\x15a0\xFBW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x13\xC2\x91\x90a<dV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a1,W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a1CW`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1^W`\0\x80\xFD[a1g\x84a1\x15V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x82W`\0\x80\xFD[a1\x8E\x86\x82\x87\x01a11V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xAAW`\0\x80\xFD[a1\xB6\x86\x82\x87\x01a11V[\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a1\xD2W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a2'W`\0\x80\xFD[a20\x89a1\x15V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2KW`\0\x80\xFD[a2W\x8B\x82\x8C\x01a11V[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2sW`\0\x80\xFD[a2\x7F\x8B\x82\x8C\x01a1\xC0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x9EW`\0\x80\xFD[a2\xAA\x8B\x82\x8C\x01a1\xC0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC9W`\0\x80\xFD[a2\xD5\x8B\x82\x8C\x01a1\xC0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15a2\xFBW`\0\x80\xFD[a.\x15\x82a1\x15V[`\0\x80\x83`\x1F\x84\x01\x12a3\x16W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3-W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\x04W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a3XW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a3nW`\0\x80\xFD[a3z\x85\x82\x86\x01a3\x04V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a3\xBAWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x0C\xE1\x82\x84a3\x9CV[`\0` \x82\x84\x03\x12\x15a3\xDEW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x83Q\x16\x82R`\x01`\x01`@\x1B\x03` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Qa42``\x84\x01\x82a3\x9CV[P\x92\x91PPV[\x80\x15\x15\x81\x14a)5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4YW`\0\x80\xFD[\x815a.\x15\x81a49V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a4|W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x92W`\0\x80\xFD[a4\x9E\x88\x82\x89\x01a3\x04V[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xBDW`\0\x80\xFD[a4\xC9\x88\x82\x89\x01a3\x04V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)5W`\0\x80\xFD[\x805a1,\x81a4\xDBV[`\0\x80`@\x83\x85\x03\x12\x15a5\x0EW`\0\x80\xFD[\x825a5\x19\x81a4\xDBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a59W`\0\x80\xFD[\x815a.\x15\x81a4\xDBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x82Wa5\x82a5DV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\xA3Wa5\xA3a5DV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\xBEW`\0\x80\xFD[\x815a5\xD1a5\xCC\x82a5\x8AV[a5ZV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a5\xF3W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a6\x10W\x805\x83R` \x92\x83\x01\x92\x01a5\xF8V[P\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a6/W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a6EW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a6VW`\0\x80\xFD[\x805a6da5\xCC\x82a5\x8AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a6\x86W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6\xB1W\x835a6\xA0\x81a4\xDBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a6\x8DV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xCFW`\0\x80\xFD[a6\xDB\x86\x82\x87\x01a5\xADV[\x92PPa6\xEA`@\x85\x01a4\xF0V[\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a7\x08W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x1EW`\0\x80\xFD[a7*\x86\x82\x87\x01a11V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7FW`\0\x80\xFD[a7R\x86\x82\x87\x01a1\xC0V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a7qW`\0\x80\xFD[\x81Qa.\x15\x81a49V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a7\x93W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a7\xADW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a2\x04W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a7\xDCW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a7\xF6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a2\x04W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a83W`\0\x80\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a.\x15W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\x96Wa8\x96a8qV[P\x04\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xE1Wa\x0C\xE1a8HV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0[\x83\x81\x10\x15a8\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01a8\xE7V[PP`\0\x91\x01RV[`\0\x82Qa9\x1A\x81\x84` \x87\x01a8\xE4V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a96W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra9~\x81` \x86\x01` \x86\x01a8\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0a9\xA6`\x80\x83\x01\x88\x8Aa9=V[\x82\x81\x03` \x84\x01Ra9\xB8\x81\x88a9fV[\x90P\x82\x81\x03`@\x84\x01Ra9\xCD\x81\x86\x88a9=V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0a\r\xF3` \x83\x01\x84\x86a9=V[`\0\x82a:\x04Wa:\x04a8qV[P\x06\x90V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0\x825`^\x19\x836\x03\x01\x81\x12a9\x1AW`\0\x80\xFD[`\0b\xFF\xFF\xFF\x82\x16\x80a:SWa:Sa8HV[`\0\x19\x01\x92\x91PPV[`\x07\x81\x81\x0B\x90\x83\x90\x0B\x01g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a1CW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0`\x01\x82\x01a:\xC2Wa:\xC2a8HV[P`\x01\x01\x90V[`\0\x83Qa:\xDB\x81\x84` \x88\x01a8\xE4V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81a;\x07Wa;\x07a8HV[P`\0\x19\x01\x90V[`\0\x81`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a;-Wa;-a8HV[`\0\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a;\x91Wa;\x91a8HV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\xE1Wa\x0C\xE1a8HV[`\0`\x01`\xFF\x1B\x82\x01a;\xBAWa;\xBAa8HV[P`\0\x03\x90V[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a;\xD8Wa;\xD8a8qV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\x07\x82\x81\x0B\x90\x82\x90\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a<1Wa<1a8qV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a42Wa42a8HV[` \x81R`\0a.\x15` \x83\x01\x84a9fV\xFE\xA2dipfsX\"\x12 \xFEn\x12\x82\r\xA2\r\xC3\xE4?\xBF\x9E!\xD1\t\x9A~d/\xA4k\xA7\x8F\nB(]\xDE}\xF1\x1D\xFEdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061016a5760003560e01c80636fcd0e53116100d1578063c49074421161008a578063dda3346c11610064578063dda3346c146105bb578063ee94d67c146105db578063f074ba62146105fb578063f28824611461061b57600080fd5b8063c49074421461055b578063c4d66de81461057b578063d06d55871461059b57600080fd5b80636fcd0e53146104705780637439841f1461049d57806374cdd798146104d457806388676cad146105085780639b4e463414610528578063b522538a1461053b57600080fd5b80634665bcda116101235780634665bcda146102ab57806347d28372146102df57806352396a59146103cd578063587533571461040357806358eaee79146104235780636c0d2d5a1461045057600080fd5b8063039157d2146101a95780630b18ff66146101cb5780632340e8d3146102085780633474aa161461022c5780633f65cf191461026457806342ecff2a1461028457600080fd5b366101a4576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156101b557600080fd5b506101c96101c4366004613149565b61064f565b005b3480156101d757600080fd5b506033546101eb906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561021457600080fd5b5061021e60395481565b6040519081526020016101ff565b34801561023857600080fd5b5060345461024c906001600160401b031681565b6040516001600160401b0390911681526020016101ff565b34801561027057600080fd5b506101c961027f36600461320b565b610984565b34801561029057600080fd5b50603a5461024c90600160401b90046001600160401b031681565b3480156102b757600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102eb57600080fd5b506103716040805160a081018252600080825260208201819052918101829052606081018290526080810191909152506040805160a081018252603c548152603d5462ffffff811660208301526001600160401b0363010000008204811693830193909352600160581b810460070b6060830152600160981b9004909116608082015290565b6040516101ff9190600060a0820190508251825262ffffff60208401511660208301526001600160401b036040840151166040830152606083015160070b60608301526001600160401b03608084015116608083015292915050565b3480156103d957600080fd5b5061024c6103e83660046132e9565b603b602052600090815260409020546001600160401b031681565b34801561040f57600080fd5b50603e546101eb906001600160a01b031681565b34801561042f57600080fd5b5061044361043e366004613345565b610c82565b6040516101ff91906133be565b34801561045c57600080fd5b5061021e61046b3660046132e9565b610ce7565b34801561047c57600080fd5b5061049061048b3660046133cc565b610dfb565b6040516101ff91906133e5565b3480156104a957600080fd5b506104436104b83660046133cc565b600090815260366020526040902054600160c01b900460ff1690565b3480156104e057600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b34801561051457600080fd5b506101c9610523366004613447565b610ea8565b6101c9610536366004613464565b610f9f565b34801561054757600080fd5b50610490610556366004613345565b6110ea565b34801561056757600080fd5b506101c96105763660046134fb565b6111dd565b34801561058757600080fd5b506101c9610596366004613527565b611329565b3480156105a757600080fd5b506101c96105b6366004613527565b611479565b3480156105c757600080fd5b506101c96105d636600461361a565b61150d565b3480156105e757600080fd5b50603a5461024c906001600160401b031681565b34801561060757600080fd5b506101c96106163660046136f3565b61166f565b34801561062757600080fd5b5061024c7f000000000000000000000000000000000000000000000000000000000000000081565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156106b7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106db919061375f565b156106f95760405163840a48d560e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610761573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610785919061375f565b156107a35760405163840a48d560e01b815260040160405180910390fd5b60006107e96107b2858061377c565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a7592505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561085857610858613386565b600281111561086957610869613386565b81525050905080604001516001600160401b0316876001600160401b0316116108a5576040516337e07ffd60e01b815260040160405180910390fd5b6001816060015160028111156108bd576108bd613386565b146108db5760405163d49e19a760e01b815260040160405180910390fd5b61091f6108e8868061377c565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a9992505050565b61093c5760405163161ce5ed60e31b815260040160405180910390fd5b61094e61094888610ce7565b87611ac3565b610971863561095d878061377c565b61096a60208a018a6137c5565b8651611b69565b61097b6000611c94565b50505050505050565b6033546001600160a01b03163314806109a75750603e546001600160a01b031633145b6109c45760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610a2c573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a50919061375f565b15610a6e5760405163840a48d560e01b815260040160405180910390fd5b8584148015610a7c57508382145b610a99576040516343714afd60e01b815260040160405180910390fd5b603a546001600160401b03600160401b9091048116908a1611610acf576040516337e07ffd60e01b815260040160405180910390fd5b610ae1610adb8a610ce7565b89611ac3565b6000805b87811015610b7a57610b668a358a8a84818110610b0457610b0461380b565b9050602002016020810190610b199190613821565b898985818110610b2b57610b2b61380b565b9050602002810190610b3d91906137c5565b898987818110610b4f57610b4f61380b565b9050602002810190610b61919061377c565b611e17565b610b70908361385e565b9150600101610ae5565b50603a54600160401b90046001600160401b031615610be857610ba1633b9aca0082613887565b603d8054601390610bc3908490600160981b90046001600160401b031661389b565b92506101000a8154816001600160401b0302191690836001600160401b031602179055505b603354604051630257884360e21b81526001600160a01b03918216600482015260248101839052600060448201527f00000000000000000000000000000000000000000000000000000000000000009091169063095e210c90606401600060405180830381600087803b158015610c5e57600080fd5b505af1158015610c72573d6000803e3d6000fd5b5050505050505050505050505050565b600080610cc484848080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061227492505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b6000610cf6611fff600c6138ba565b610d096001600160401b038416426138d1565b10610d2757604051637944e66d60e11b815260040160405180910390fd5b604080516001600160401b03841660208201526000918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f1981840301815290829052610d6f91613908565b600060405180830381855afa9150503d8060008114610daa576040519150601f19603f3d011682016040523d82523d6000602084013e610daf565b606091505b5091509150818015610dc2575060008151115b610ddf5760405163558ad0a360e01b815260040160405180910390fd5b80806020019051810190610df39190613924565b949350505050565b610e236040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff166002811115610e8e57610e8e613386565b6002811115610e9f57610e9f613386565b90525092915050565b6033546001600160a01b0316331480610ecb5750603e546001600160a01b031633145b610ee85760405163427a777960e01b815260040160405180910390fd5b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610f50573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610f74919061375f565b15610f925760405163840a48d560e01b815260040160405180910390fd5b610f9b82611c94565b5050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610fe857604051633213a66160e21b815260040160405180910390fd5b346801bc16d674ec800000146110115760405163049696b360e31b815260040160405180910390fd5b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec8000008787611054612309565b8888886040518863ffffffff1660e01b815260040161107896959493929190613992565b6000604051808303818588803b15801561109157600080fd5b505af11580156110a5573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e2385856040516110db9291906139e1565b60405180910390a15050505050565b6111126040805160808101825260008082526020820181905291810182905290606082015290565b6036600061115585858080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061227492505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff1660028111156111c2576111c2613386565b60028111156111d3576111d3613386565b9052509392505050565b336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461122657604051633213a66160e21b815260040160405180910390fd5b611234633b9aca00826139f5565b15611252576040516321ddeb1760e21b815260040160405180910390fd5b6000611262633b9aca0083613887565b6034549091506001600160401b039081169082161115611295576040516302c6f54760e21b815260040160405180910390fd5b603480548291906000906112b39084906001600160401b0316613a09565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e8360405161131291815260200190565b60405180910390a2611324838361234e565b505050565b600054610100900460ff16158080156113495750600054600160ff909116105b806113635750303b158015611363575060005460ff166001145b6113cb5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084015b60405180910390fd5b6000805460ff1916600117905580156113ee576000805461ff0019166101001790555b6001600160a01b038216611415576040516339b190bb60e11b815260040160405180910390fd5b603380546001600160a01b0319166001600160a01b0384161790558015610f9b576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b031633146114a45760405163719f370360e11b815260040160405180910390fd5b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b031633146115385760405163719f370360e11b815260040160405180910390fd5b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156115a0573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906115c4919061375f565b156115e25760405163840a48d560e01b815260040160405180910390fd5b8251845114611604576040516343714afd60e01b815260040160405180910390fd5b60005b845181101561166857611660838583815181106116265761162661380b565b60200260200101518784815181106116405761164061380b565b60200260200101516001600160a01b03166124679092919063ffffffff16565b600101611607565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa1580156116d7573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906116fb919061375f565b156117195760405163840a48d560e01b815260040160405180910390fd5b603a54600160401b90046001600160401b0316600081900361174e57604051631a544f4960e01b815260040160405180910390fd5b6040805160a081018252603c54808252603d5462ffffff811660208401526001600160401b0363010000008204811694840194909452600160581b810460070b6060840152600160981b90049092166080820152906117ad90876124b9565b6000805b85811015611a1b57368787838181106117cc576117cc61380b565b90506020028101906117de9190613a28565b80356000908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561184f5761184f613386565b600281111561186057611860613386565b905250905060018160600151600281111561187d5761187d613386565b14611889575050611a13565b856001600160401b031681604001516001600160401b0316106118ad575050611a13565b600080806118be848a8f358861256b565b60208b01805193965091945092506118d582613a3e565b62ffffff169052506080880180518491906118f190839061389b565b6001600160401b0316905250606088018051839190611911908390613a5d565b60070b905250611921818861389b565b85356000908152603660209081526040918290208751815492890151938901516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060880151939a50879390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156119c6576119c6613386565b021790555050845160405164ffffffffff90911691506001600160401b038b16907fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f90600090a350505050505b6001016117b1565b506001600160401b038084166000908152603b6020526040812080548493919291611a489185911661389b565b92506101000a8154816001600160401b0302191690836001600160401b0316021790555061097b82612691565b600081600081518110611a8a57611a8a61380b565b60200260200101519050919050565b600081600381518110611aae57611aae61380b565b60200260200101516000801b14159050919050565b611acf600360206138ba565b611adc60208301836137c5565b905014611afc576040516313717da960e21b815260040160405180910390fd5b611b4c611b0c60208301836137c5565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003612938565b610f9b576040516309bde33960e01b815260040160405180910390fd5b60088414611b8a5760405163200591bd60e01b815260040160405180910390fd5b6005611b986028600161385e565b611ba2919061385e565b611bad9060206138ba565b8214611bcc576040516313717da960e21b815260040160405180910390fd5b6000611c0a86868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061295092505050565b9050600064ffffffffff8316611c226028600161385e565b600b901b179050611c6d85858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050612938565b611c8a576040516309bde33960e01b815260040160405180910390fd5b5050505050505050565b603a54600160401b90046001600160401b031615611cc45760405162be9bc360e81b815260040160405180910390fd5b603a546001600160401b03428116911603611cf2576040516367db5b8b60e01b815260040160405180910390fd5b6034546000906001600160401b0316611d0f633b9aca0047613887565b611d199190613a09565b9050818015611d2f57506001600160401b038116155b15611d4d576040516332dea95960e21b815260040160405180910390fd5b60006040518060a00160405280611d6342610ce7565b815260395462ffffff1660208201526001600160401b038085166040830152600060608301819052608090920191909152603a805442909216600160401b026fffffffffffffffff0000000000000000199092169190911790559050611dc881612691565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b600080611e56848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611a7592505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611ec557611ec5613386565b6002811115611ed657611ed6613386565b9052509050600081606001516002811115611ef357611ef3613386565b14611f11576040516335e09e9d60e01b815260040160405180910390fd5b6001600160401b038016611f57868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612be992505050565b6001600160401b031603611f7e57604051631958236d60e21b815260040160405180910390fd5b6001600160401b038016611fc4868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c0e92505050565b6001600160401b031614611feb57604051632eade63760e01b815260040160405180910390fd5b611ff3612309565b611ffc90613a8c565b612038868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c2692505050565b1461205657604051632230566760e11b815260040160405180910390fd5b6000612094868680806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250612c3b92505050565b90506120a48a87878b8b8e611b69565b603980549060006120b483613ab0565b9091555050603a546001600160401b0380821691600160401b900416156120ea5750603a54600160401b90046001600160401b03165b6040805160808101825264ffffffffff8c1681526001600160401b03848116602083015283169181019190915260608101600190526000858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b8360028111156121bd576121bd613386565b02179055505060405164ffffffffff8c1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a16040805164ffffffffff8c1681526001600160401b03838116602083015284168183015290517f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df9181900360600190a1612265633b9aca006001600160401b0384166138ba565b9b9a5050505050505050505050565b6000815160301461229857604051634f88323960e11b815260040160405180910390fd5b6040516002906122af908490600090602001613ac9565b60408051601f19818403018152908290526122c991613908565b602060405180830381855afa1580156122e6573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190610ce19190613924565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b8047101561239e5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016113c2565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146123eb576040519150601f19603f3d011682016040523d82523d6000602084013e6123f0565b606091505b50509050806113245760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016113c2565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611324908490612c53565b6124c56005600361385e565b6124d09060206138ba565b6124dd60208301836137c5565b9050146124fd576040516313717da960e21b815260040160405180910390fd5b606c61254e61250f60208401846137c5565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084612938565b611324576040516309bde33960e01b815260040160405180910390fd5b8351602085015190600090819081612584878388612d28565b9050846001600160401b0316816001600160401b0316146125fe576125a98186612e09565b6040805164ffffffffff851681526001600160401b038b8116602083015284168183015290519195507f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df919081900360600190a15b6001600160401b0380821660208b0181905290891660408b0152600003612685576039805490600061262f83613af8565b9091555050600260608a015261264484613b0f565b92508164ffffffffff16886001600160401b03167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a60405160405180910390a35b50509450945094915050565b806020015162ffffff166000036128a6576000633b9aca00826060015160070b83604001516001600160401b03166126c99190613b36565b600f0b6126d69190613b75565b60408301516034805492935090916000906126fb9084906001600160401b031661389b565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b81049092166001600160801b0319909216919091179055506000603c819055603d80546001600160d81b0319169055808212156127c9576080830151603454600091633b9aca009161277f91906001600160401b031661389b565b6001600160401b031661279291906138ba565b905080670de0b6b3a76400006127a785613ba5565b6127b1908461385e565b6127bb91906138ba565b6127c59190613887565b9150505b603354604051630257884360e21b81526001600160a01b039182166004820152602481018490526001600160401b03831660448201527f00000000000000000000000000000000000000000000000000000000000000009091169063095e210c90606401600060405180830381600087803b15801561284757600080fd5b505af115801561285b573d6000803e3d6000fd5b5050603a546040518581526001600160401b0390911692507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44915060200160405180910390a2505050565b8051603c556020810151603d805460408401516060850151608086015162ffffff9095166affffffffffffffffffffff199093169290921763010000006001600160401b0392831602176fffffffffffffffffffffffffffffffff60581b1916600160581b9282169290920267ffffffffffffffff60981b191691909117600160981b91909316029190911790555b50565b600083612946868585612e1c565b1495945050505050565b600080600283516129619190613887565b90506000816001600160401b0381111561297d5761297d613544565b6040519080825280602002602001820160405280156129a6578160200160208202803683370190505b50905060005b82811015612aa3576002856129c183836138ba565b815181106129d1576129d161380b565b6020026020010151868360026129e791906138ba565b6129f290600161385e565b81518110612a0257612a0261380b565b6020026020010151604051602001612a24929190918252602082015260400190565b60408051601f1981840301815290829052612a3e91613908565b602060405180830381855afa158015612a5b573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190612a7e9190613924565b828281518110612a9057612a9061380b565b60209081029190910101526001016129ac565b50612aaf600283613887565b91505b8115612bc55760005b82811015612bb257600282612ad083836138ba565b81518110612ae057612ae061380b565b602002602001015183836002612af691906138ba565b612b0190600161385e565b81518110612b1157612b1161380b565b6020026020010151604051602001612b33929190918252602082015260400190565b60408051601f1981840301815290829052612b4d91613908565b602060405180830381855afa158015612b6a573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190612b8d9190613924565b828281518110612b9f57612b9f61380b565b6020908102919091010152600101612abb565b50612bbe600283613887565b9150612ab2565b80600081518110612bd857612bd861380b565b602002602001015192505050919050565b6000610ce182600581518110612c0157612c0161380b565b6020026020010151612ef9565b6000610ce182600681518110612c0157612c0161380b565b600081600181518110611a8a57611a8a61380b565b6000610ce182600281518110612c0157612c0161380b565b6000612ca8826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b0316612f609092919063ffffffff16565b9050805160001480612cc9575080806020019051810190612cc9919061375f565b6113245760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016113c2565b6000612d366026600161385e565b612d419060206138ba565b612d4e60408401846137c5565b905014612d6e576040516313717da960e21b815260040160405180910390fd5b6000612d7b600485613bc1565b64ffffffffff169050612dd5612d9460408501856137c5565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584612938565b612df2576040516309bde33960e01b815260040160405180910390fd5b612e00836020013585612f6f565b95945050505050565b6000612e158284613beb565b9392505050565b60008351600014158015612e3b575060208451612e3991906139f5565b155b612e58576040516313717da960e21b815260040160405180910390fd5b604080516020808201909252848152905b85518111612eef57612e7c6002856139f5565b600003612eb2578151600052808601516020526020826040600060026107d05a03fa612ea757600080fd5b600284049350612edd565b8086015160005281516020526020826040600060026107d05a03fa612ed657600080fd5b6002840493505b612ee860208261385e565b9050612e69565b5051949350505050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6060610df38484600085612f9c565b600080612f7d600484613c1a565b612f88906040613c44565b64ffffffffff169050610df384821b612ef9565b606082471015612ffd5760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016113c2565b600080866001600160a01b031685876040516130199190613908565b60006040518083038185875af1925050503d8060008114613056576040519150601f19603f3d011682016040523d82523d6000602084013e61305b565b606091505b509150915061306c87838387613077565b979650505050505050565b606083156130e65782516000036130df576001600160a01b0385163b6130df5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016113c2565b5081610df3565b610df383838151156130fb5781518083602001fd5b8060405162461bcd60e51b81526004016113c29190613c64565b80356001600160401b038116811461312c57600080fd5b919050565b60006040828403121561314357600080fd5b50919050565b60008060006060848603121561315e57600080fd5b61316784613115565b925060208401356001600160401b0381111561318257600080fd5b61318e86828701613131565b92505060408401356001600160401b038111156131aa57600080fd5b6131b686828701613131565b9150509250925092565b60008083601f8401126131d257600080fd5b5081356001600160401b038111156131e957600080fd5b6020830191508360208260051b850101111561320457600080fd5b9250929050565b60008060008060008060008060a0898b03121561322757600080fd5b61323089613115565b975060208901356001600160401b0381111561324b57600080fd5b6132578b828c01613131565b97505060408901356001600160401b0381111561327357600080fd5b61327f8b828c016131c0565b90975095505060608901356001600160401b0381111561329e57600080fd5b6132aa8b828c016131c0565b90955093505060808901356001600160401b038111156132c957600080fd5b6132d58b828c016131c0565b999c989b5096995094979396929594505050565b6000602082840312156132fb57600080fd5b612e1582613115565b60008083601f84011261331657600080fd5b5081356001600160401b0381111561332d57600080fd5b60208301915083602082850101111561320457600080fd5b6000806020838503121561335857600080fd5b82356001600160401b0381111561336e57600080fd5b61337a85828601613304565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b600381106133ba57634e487b7160e01b600052602160045260246000fd5b9052565b60208101610ce1828461339c565b6000602082840312156133de57600080fd5b5035919050565b60006080820190506001600160401b0383511682526001600160401b0360208401511660208301526001600160401b0360408401511660408301526060830151613432606084018261339c565b5092915050565b801515811461293557600080fd5b60006020828403121561345957600080fd5b8135612e1581613439565b60008060008060006060868803121561347c57600080fd5b85356001600160401b0381111561349257600080fd5b61349e88828901613304565b90965094505060208601356001600160401b038111156134bd57600080fd5b6134c988828901613304565b96999598509660400135949350505050565b6001600160a01b038116811461293557600080fd5b803561312c816134db565b6000806040838503121561350e57600080fd5b8235613519816134db565b946020939093013593505050565b60006020828403121561353957600080fd5b8135612e15816134db565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b038111828210171561358257613582613544565b604052919050565b60006001600160401b038211156135a3576135a3613544565b5060051b60200190565b600082601f8301126135be57600080fd5b81356135d16135cc8261358a565b61355a565b8082825260208201915060208360051b8601019250858311156135f357600080fd5b602085015b838110156136105780358352602092830192016135f8565b5095945050505050565b60008060006060848603121561362f57600080fd5b83356001600160401b0381111561364557600080fd5b8401601f8101861361365657600080fd5b80356136646135cc8261358a565b8082825260208201915060208360051b85010192508883111561368657600080fd5b6020840193505b828410156136b15783356136a0816134db565b82526020938401939091019061368d565b955050505060208401356001600160401b038111156136cf57600080fd5b6136db868287016135ad565b9250506136ea604085016134f0565b90509250925092565b60008060006040848603121561370857600080fd5b83356001600160401b0381111561371e57600080fd5b61372a86828701613131565b93505060208401356001600160401b0381111561374657600080fd5b613752868287016131c0565b9497909650939450505050565b60006020828403121561377157600080fd5b8151612e1581613439565b6000808335601e1984360301811261379357600080fd5b8301803591506001600160401b038211156137ad57600080fd5b6020019150600581901b360382131561320457600080fd5b6000808335601e198436030181126137dc57600080fd5b8301803591506001600160401b038211156137f657600080fd5b60200191503681900382131561320457600080fd5b634e487b7160e01b600052603260045260246000fd5b60006020828403121561383357600080fd5b813564ffffffffff81168114612e1557600080fd5b634e487b7160e01b600052601160045260246000fd5b80820180821115610ce157610ce1613848565b634e487b7160e01b600052601260045260246000fd5b60008261389657613896613871565b500490565b6001600160401b038181168382160190811115610ce157610ce1613848565b8082028115828204841417610ce157610ce1613848565b81810381811115610ce157610ce1613848565b60005b838110156138ff5781810151838201526020016138e7565b50506000910152565b6000825161391a8184602087016138e4565b9190910192915050565b60006020828403121561393657600080fd5b5051919050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000815180845261397e8160208601602086016138e4565b601f01601f19169290920160200192915050565b6080815260006139a660808301888a61393d565b82810360208401526139b88188613966565b905082810360408401526139cd81868861393d565b915050826060830152979650505050505050565b602081526000610df360208301848661393d565b600082613a0457613a04613871565b500690565b6001600160401b038281168282160390811115610ce157610ce1613848565b60008235605e1983360301811261391a57600080fd5b600062ffffff821680613a5357613a53613848565b6000190192915050565b600781810b9083900b01677fffffffffffffff8113677fffffffffffffff1982121715610ce157610ce1613848565b805160208083015191908110156131435760001960209190910360031b1b16919050565b600060018201613ac257613ac2613848565b5060010190565b60008351613adb8184602088016138e4565b6001600160801b0319939093169190920190815260100192915050565b600081613b0757613b07613848565b506000190190565b60008160070b677fffffffffffffff198103613b2d57613b2d613848565b60000392915050565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81136f7fffffffffffffffffffffffffffffff1982121715610ce157610ce1613848565b80820260008212600160ff1b84141615613b9157613b91613848565b8181058314821517610ce157610ce1613848565b6000600160ff1b8201613bba57613bba613848565b5060000390565b600064ffffffffff831680613bd857613bd8613871565b8064ffffffffff84160491505092915050565b600782810b9082900b03677fffffffffffffff198112677fffffffffffffff82131715610ce157610ce1613848565b600064ffffffffff831680613c3157613c31613871565b8064ffffffffff84160691505092915050565b64ffffffffff818116838216029081169081811461343257613432613848565b602081526000612e15602083018461396656fea2646970667358221220fe6e12820da20dc3e43fbf9e21d1099a7e642fa46ba78f0a42285dde7df11dfe64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80co\xCD\x0ES\x11a\0\xD1W\x80c\xC4\x90tB\x11a\0\x8AW\x80c\xDD\xA34l\x11a\0dW\x80c\xDD\xA34l\x14a\x05\xBBW\x80c\xEE\x94\xD6|\x14a\x05\xDBW\x80c\xF0t\xBAb\x14a\x05\xFBW\x80c\xF2\x88$a\x14a\x06\x1BW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x05[W\x80c\xC4\xD6m\xE8\x14a\x05{W\x80c\xD0mU\x87\x14a\x05\x9BW`\0\x80\xFD[\x80co\xCD\x0ES\x14a\x04pW\x80ct9\x84\x1F\x14a\x04\x9DW\x80ct\xCD\xD7\x98\x14a\x04\xD4W\x80c\x88gl\xAD\x14a\x05\x08W\x80c\x9BNF4\x14a\x05(W\x80c\xB5\"S\x8A\x14a\x05;W`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01#W\x80cFe\xBC\xDA\x14a\x02\xABW\x80cG\xD2\x83r\x14a\x02\xDFW\x80cR9jY\x14a\x03\xCDW\x80cXu3W\x14a\x04\x03W\x80cX\xEA\xEEy\x14a\x04#W\x80cl\r-Z\x14a\x04PW`\0\x80\xFD[\x80c\x03\x91W\xD2\x14a\x01\xA9W\x80c\x0B\x18\xFFf\x14a\x01\xCBW\x80c#@\xE8\xD3\x14a\x02\x08W\x80c4t\xAA\x16\x14a\x02,W\x80c?e\xCF\x19\x14a\x02dW\x80cB\xEC\xFF*\x14a\x02\x84W`\0\x80\xFD[6a\x01\xA4W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a1IV[a\x06OV[\0[4\x80\x15a\x01\xD7W`\0\x80\xFD[P`3Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x1E`9T\x81V[`@Q\x90\x81R` \x01a\x01\xFFV[4\x80\x15a\x028W`\0\x80\xFD[P`4Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x01\xC9a\x02\x7F6`\x04a2\x0BV[a\t\x84V[4\x80\x15a\x02\x90W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x02\xB7W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x03q`@\x80Q`\xA0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x91\x90\x91RP`@\x80Q`\xA0\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x81\x04`\x07\x0B``\x83\x01R`\x01`\x98\x1B\x90\x04\x90\x91\x16`\x80\x82\x01R\x90V[`@Qa\x01\xFF\x91\x90`\0`\xA0\x82\x01\x90P\x82Q\x82Rb\xFF\xFF\xFF` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Q`\x07\x0B``\x83\x01R`\x01`\x01`@\x1B\x03`\x80\x84\x01Q\x16`\x80\x83\x01R\x92\x91PPV[4\x80\x15a\x03\xD9W`\0\x80\xFD[Pa\x02La\x03\xE86`\x04a2\xE9V[`;` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x04\x0FW`\0\x80\xFD[P`>Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04/W`\0\x80\xFD[Pa\x04Ca\x04>6`\x04a3EV[a\x0C\x82V[`@Qa\x01\xFF\x91\x90a3\xBEV[4\x80\x15a\x04\\W`\0\x80\xFD[Pa\x02\x1Ea\x04k6`\x04a2\xE9V[a\x0C\xE7V[4\x80\x15a\x04|W`\0\x80\xFD[Pa\x04\x90a\x04\x8B6`\x04a3\xCCV[a\r\xFBV[`@Qa\x01\xFF\x91\x90a3\xE5V[4\x80\x15a\x04\xA9W`\0\x80\xFD[Pa\x04Ca\x04\xB86`\x04a3\xCCV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x04\xE0W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x05\x14W`\0\x80\xFD[Pa\x01\xC9a\x05#6`\x04a4GV[a\x0E\xA8V[a\x01\xC9a\x0566`\x04a4dV[a\x0F\x9FV[4\x80\x15a\x05GW`\0\x80\xFD[Pa\x04\x90a\x05V6`\x04a3EV[a\x10\xEAV[4\x80\x15a\x05gW`\0\x80\xFD[Pa\x01\xC9a\x05v6`\x04a4\xFBV[a\x11\xDDV[4\x80\x15a\x05\x87W`\0\x80\xFD[Pa\x01\xC9a\x05\x966`\x04a5'V[a\x13)V[4\x80\x15a\x05\xA7W`\0\x80\xFD[Pa\x01\xC9a\x05\xB66`\x04a5'V[a\x14yV[4\x80\x15a\x05\xC7W`\0\x80\xFD[Pa\x01\xC9a\x05\xD66`\x04a6\x1AV[a\x15\rV[4\x80\x15a\x05\xE7W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x06\x07W`\0\x80\xFD[Pa\x01\xC9a\x06\x166`\x04a6\xF3V[a\x16oV[4\x80\x15a\x06'W`\0\x80\xFD[Pa\x02L\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xB7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xDB\x91\x90a7_V[\x15a\x06\xF9W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07aW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x85\x91\x90a7_V[\x15a\x07\xA3W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\xE9a\x07\xB2\x85\x80a7|V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1Au\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x08XWa\x08Xa3\x86V[`\x02\x81\x11\x15a\x08iWa\x08ia3\x86V[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\x08\xA5W`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81``\x01Q`\x02\x81\x11\x15a\x08\xBDWa\x08\xBDa3\x86V[\x14a\x08\xDBW`@Qc\xD4\x9E\x19\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\t\x1Fa\x08\xE8\x86\x80a7|V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1A\x99\x92PPPV[a\t<W`@Qc\x16\x1C\xE5\xED`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\tNa\tH\x88a\x0C\xE7V[\x87a\x1A\xC3V[a\tq\x865a\t]\x87\x80a7|V[a\tj` \x8A\x01\x8Aa7\xC5V[\x86Qa\x1BiV[a\t{`\0a\x1C\x94V[PPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\t\xA7WP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\t\xC4W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n,W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nP\x91\x90a7_V[\x15a\nnW`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x85\x84\x14\x80\x15a\n|WP\x83\x82\x14[a\n\x99W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\n\xCFW`@Qc7\xE0\x7F\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\n\xE1a\n\xDB\x8Aa\x0C\xE7V[\x89a\x1A\xC3V[`\0\x80[\x87\x81\x10\x15a\x0BzWa\x0Bf\x8A5\x8A\x8A\x84\x81\x81\x10a\x0B\x04Wa\x0B\x04a8\x0BV[\x90P` \x02\x01` \x81\x01\x90a\x0B\x19\x91\x90a8!V[\x89\x89\x85\x81\x81\x10a\x0B+Wa\x0B+a8\x0BV[\x90P` \x02\x81\x01\x90a\x0B=\x91\x90a7\xC5V[\x89\x89\x87\x81\x81\x10a\x0BOWa\x0BOa8\x0BV[\x90P` \x02\x81\x01\x90a\x0Ba\x91\x90a7|V[a\x1E\x17V[a\x0Bp\x90\x83a8^V[\x91P`\x01\x01a\n\xE5V[P`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a\x0B\xE8Wa\x0B\xA1c;\x9A\xCA\0\x82a8\x87V[`=\x80T`\x13\x90a\x0B\xC3\x90\x84\x90`\x01`\x98\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a8\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[`3T`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R`\0`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^!\x0C\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0CrW=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[`\0\x80a\x0C\xC4\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"t\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`\0a\x0C\xF6a\x1F\xFF`\x0Ca8\xBAV[a\r\t`\x01`\x01`@\x1B\x03\x84\x16Ba8\xD1V[\x10a\r'W`@QcyD\xE6m`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\0\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\ro\x91a9\x08V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\r\xAAW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\r\xAFV[``\x91P[P\x91P\x91P\x81\x80\x15a\r\xC2WP`\0\x81Q\x11[a\r\xDFW`@QcU\x8A\xD0\xA3`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x80` \x01\x90Q\x81\x01\x90a\r\xF3\x91\x90a9$V[\x94\x93PPPPV[a\x0E#`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x0E\x8EWa\x0E\x8Ea3\x86V[`\x02\x81\x11\x15a\x0E\x9FWa\x0E\x9Fa3\x86V[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x0E\xCBWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x0E\xE8W`@QcBzwy`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0FPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ft\x91\x90a7_V[\x15a\x0F\x92W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\x9B\x82a\x1C\x94V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x0F\xE8W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x10\x11W`@Qc\x04\x96\x96\xB3`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x10Ta#\tV[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x10x\x96\x95\x94\x93\x92\x91\x90a9\x92V[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x10\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x10\xA5W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x10\xDB\x92\x91\x90a9\xE1V[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x11\x12`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x11U\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\"t\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x11\xC2Wa\x11\xC2a3\x86V[`\x02\x81\x11\x15a\x11\xD3Wa\x11\xD3a3\x86V[\x90RP\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x12&W`@Qc2\x13\xA6a`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x124c;\x9A\xCA\0\x82a9\xF5V[\x15a\x12RW`@Qc!\xDD\xEB\x17`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12bc;\x9A\xCA\0\x83a8\x87V[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x12\x95W`@Qc\x02\xC6\xF5G`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4\x80T\x82\x91\x90`\0\x90a\x12\xB3\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a:\tV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x13\x12\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x13$\x83\x83a#NV[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x13IWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x13cWP0;\x15\x80\x15a\x13cWP`\0T`\xFF\x16`\x01\x14[a\x13\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x13\xEEW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x14\x15W`@Qc9\xB1\x90\xBB`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x0F\x9BW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x14\xA4W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x158W`@Qcq\x9F7\x03`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\xA0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x15\xC4\x91\x90a7_V[\x15a\x15\xE2W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x84Q\x14a\x16\x04W`@QcCqJ\xFD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x84Q\x81\x10\x15a\x16hWa\x16`\x83\x85\x83\x81Q\x81\x10a\x16&Wa\x16&a8\x0BV[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x16@Wa\x16@a8\x0BV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a$g\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\x16\x07V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xFB\x91\x90a7_V[\x15a\x17\x19W`@Qc\x84\nH\xD5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16`\0\x81\x90\x03a\x17NW`@Qc\x1ATOI`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xA0\x81\x01\x82R`<T\x80\x82R`=Tb\xFF\xFF\xFF\x81\x16` \x84\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x81\x16\x94\x84\x01\x94\x90\x94R`\x01`X\x1B\x81\x04`\x07\x0B``\x84\x01R`\x01`\x98\x1B\x90\x04\x90\x92\x16`\x80\x82\x01R\x90a\x17\xAD\x90\x87a$\xB9V[`\0\x80[\x85\x81\x10\x15a\x1A\x1BW6\x87\x87\x83\x81\x81\x10a\x17\xCCWa\x17\xCCa8\x0BV[\x90P` \x02\x81\x01\x90a\x17\xDE\x91\x90a:(V[\x805`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x18OWa\x18Oa3\x86V[`\x02\x81\x11\x15a\x18`Wa\x18`a3\x86V[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x18}Wa\x18}a3\x86V[\x14a\x18\x89WPPa\x1A\x13V[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x18\xADWPPa\x1A\x13V[`\0\x80\x80a\x18\xBE\x84\x8A\x8F5\x88a%kV[` \x8B\x01\x80Q\x93\x96P\x91\x94P\x92Pa\x18\xD5\x82a:>V[b\xFF\xFF\xFF\x16\x90RP`\x80\x88\x01\x80Q\x84\x91\x90a\x18\xF1\x90\x83\x90a8\x9BV[`\x01`\x01`@\x1B\x03\x16\x90RP``\x88\x01\x80Q\x83\x91\x90a\x19\x11\x90\x83\x90a:]V[`\x07\x0B\x90RPa\x19!\x81\x88a8\x9BV[\x855`\0\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x87Q\x81T\x92\x89\x01Q\x93\x89\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x88\x01Q\x93\x9AP\x87\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\x19\xC6Wa\x19\xC6a3\x86V[\x02\x17\x90UPP\x84Q`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P`\x01`\x01`@\x1B\x03\x8B\x16\x90\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?\x90`\0\x90\xA3PPPPP[`\x01\x01a\x17\xB1V[P`\x01`\x01`@\x1B\x03\x80\x84\x16`\0\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a\x1AH\x91\x85\x91\x16a8\x9BV[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\t{\x82a&\x91V[`\0\x81`\0\x81Q\x81\x10a\x1A\x8AWa\x1A\x8Aa8\x0BV[` \x02` \x01\x01Q\x90P\x91\x90PV[`\0\x81`\x03\x81Q\x81\x10a\x1A\xAEWa\x1A\xAEa8\x0BV[` \x02` \x01\x01Q`\0\x80\x1B\x14\x15\x90P\x91\x90PV[a\x1A\xCF`\x03` a8\xBAV[a\x1A\xDC` \x83\x01\x83a7\xC5V[\x90P\x14a\x1A\xFCW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1BLa\x1B\x0C` \x83\x01\x83a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a)8V[a\x0F\x9BW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x84\x14a\x1B\x8AW`@Qc \x05\x91\xBD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x05a\x1B\x98`(`\x01a8^V[a\x1B\xA2\x91\x90a8^V[a\x1B\xAD\x90` a8\xBAV[\x82\x14a\x1B\xCCW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1C\n\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa)P\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a\x1C\"`(`\x01a8^V[`\x0B\x90\x1B\x17\x90Pa\x1Cm\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa)8V[a\x1C\x8AW`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a\x1C\xC4W`@Qb\xBE\x9B\xC3`\xE8\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`:T`\x01`\x01`@\x1B\x03B\x81\x16\x91\x16\x03a\x1C\xF2W`@Qcg\xDB[\x8B`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`4T`\0\x90`\x01`\x01`@\x1B\x03\x16a\x1D\x0Fc;\x9A\xCA\0Ga8\x87V[a\x1D\x19\x91\x90a:\tV[\x90P\x81\x80\x15a\x1D/WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a\x1DMW`@Qc2\xDE\xA9Y`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0`@Q\x80`\xA0\x01`@R\x80a\x1DcBa\x0C\xE7V[\x81R`9Tb\xFF\xFF\xFF\x16` \x82\x01R`\x01`\x01`@\x1B\x03\x80\x85\x16`@\x83\x01R`\0``\x83\x01\x81\x90R`\x80\x90\x92\x01\x91\x90\x91R`:\x80TB\x90\x92\x16`\x01`@\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x90Pa\x1D\xC8\x81a&\x91V[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a\x1EV\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1Au\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1E\xC5Wa\x1E\xC5a3\x86V[`\x02\x81\x11\x15a\x1E\xD6Wa\x1E\xD6a3\x86V[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a\x1E\xF3Wa\x1E\xF3a3\x86V[\x14a\x1F\x11W`@Qc5\xE0\x9E\x9D`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a\x1FW\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa+\xE9\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x03a\x1F~W`@Qc\x19X#m`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`@\x1B\x03\x80\x16a\x1F\xC4\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a\x1F\xEBW`@Qc.\xAD\xE67`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x1F\xF3a#\tV[a\x1F\xFC\x90a:\x8CV[a 8\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,&\x92PPPV[\x14a VW`@Qc\"0Vg`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a \x94\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa,;\x92PPPV[\x90Pa \xA4\x8A\x87\x87\x8B\x8B\x8Ea\x1BiV[`9\x80T\x90`\0a \xB4\x83a:\xB0V[\x90\x91UPP`:T`\x01`\x01`@\x1B\x03\x80\x82\x16\x91`\x01`@\x1B\x90\x04\x16\x15a \xEAWP`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x84\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R``\x81\x01`\x01\x90R`\0\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a!\xBDWa!\xBDa3\x86V[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x81\x90\x03``\x01\x90\xA1a\"ec;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16a8\xBAV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a\"\x98W`@QcO\x8829`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x02\x90a\"\xAF\x90\x84\x90`\0\x90` \x01a:\xC9V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\"\xC9\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\"\xE6W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\xE1\x91\x90a9$V[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80G\x10\x15a#\x9EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x13\xC2V[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a#\xEBW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a#\xF0V[``\x91P[PP\x90P\x80a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x13\xC2V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x13$\x90\x84\x90a,SV[a$\xC5`\x05`\x03a8^V[a$\xD0\x90` a8\xBAV[a$\xDD` \x83\x01\x83a7\xC5V[\x90P\x14a$\xFDW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`la%Na%\x0F` \x84\x01\x84a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a)8V[a\x13$W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q` \x85\x01Q\x90`\0\x90\x81\x90\x81a%\x84\x87\x83\x88a-(V[\x90P\x84`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a%\xFEWa%\xA9\x81\x86a.\tV[`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x85\x16\x81R`\x01`\x01`@\x1B\x03\x8B\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x95P\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8B\x01\x81\x90R\x90\x89\x16`@\x8B\x01R`\0\x03a&\x85W`9\x80T\x90`\0a&/\x83a:\xF8V[\x90\x91UPP`\x02``\x8A\x01Ra&D\x84a;\x0FV[\x92P\x81d\xFF\xFF\xFF\xFF\xFF\x16\x88`\x01`\x01`@\x1B\x03\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj`@Q`@Q\x80\x91\x03\x90\xA3[PP\x94P\x94P\x94\x91PPV[\x80` \x01Qb\xFF\xFF\xFF\x16`\0\x03a(\xA6W`\0c;\x9A\xCA\0\x82``\x01Q`\x07\x0B\x83`@\x01Q`\x01`\x01`@\x1B\x03\x16a&\xC9\x91\x90a;6V[`\x0F\x0Ba&\xD6\x91\x90a;uV[`@\x83\x01Q`4\x80T\x92\x93P\x90\x91`\0\x90a&\xFB\x90\x84\x90`\x01`\x01`@\x1B\x03\x16a8\x9BV[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UP`\0`<\x81\x90U`=\x80T`\x01`\x01`\xD8\x1B\x03\x19\x16\x90U\x80\x82\x12\x15a'\xC9W`\x80\x83\x01Q`4T`\0\x91c;\x9A\xCA\0\x91a'\x7F\x91\x90`\x01`\x01`@\x1B\x03\x16a8\x9BV[`\x01`\x01`@\x1B\x03\x16a'\x92\x91\x90a8\xBAV[\x90P\x80g\r\xE0\xB6\xB3\xA7d\0\0a'\xA7\x85a;\xA5V[a'\xB1\x90\x84a8^V[a'\xBB\x91\x90a8\xBAV[a'\xC5\x91\x90a8\x87V[\x91PP[`3T`@Qc\x02W\x88C`\xE2\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R`\x01`\x01`@\x1B\x03\x83\x16`D\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\t^!\x0C\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a(GW`\0\x80\xFD[PZ\xF1\x15\x80\x15a([W=`\0\x80>=`\0\xFD[PP`:T`@Q\x85\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16\x92P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x91P` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Q`\x80\x86\x01Qb\xFF\xFF\xFF\x90\x95\x16j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17c\x01\0\0\0`\x01`\x01`@\x1B\x03\x92\x83\x16\x02\x17o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19\x16`\x01`X\x1B\x92\x82\x16\x92\x90\x92\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x98\x1B\x19\x16\x91\x90\x91\x17`\x01`\x98\x1B\x91\x90\x93\x16\x02\x91\x90\x91\x17\x90U[PV[`\0\x83a)F\x86\x85\x85a.\x1CV[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa)a\x91\x90a8\x87V[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a)}Wa)}a5DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xA6W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a*\xA3W`\x02\x85a)\xC1\x83\x83a8\xBAV[\x81Q\x81\x10a)\xD1Wa)\xD1a8\x0BV[` \x02` \x01\x01Q\x86\x83`\x02a)\xE7\x91\x90a8\xBAV[a)\xF2\x90`\x01a8^V[\x81Q\x81\x10a*\x02Wa*\x02a8\x0BV[` \x02` \x01\x01Q`@Q` \x01a*$\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*>\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a*[W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*~\x91\x90a9$V[\x82\x82\x81Q\x81\x10a*\x90Wa*\x90a8\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a)\xACV[Pa*\xAF`\x02\x83a8\x87V[\x91P[\x81\x15a+\xC5W`\0[\x82\x81\x10\x15a+\xB2W`\x02\x82a*\xD0\x83\x83a8\xBAV[\x81Q\x81\x10a*\xE0Wa*\xE0a8\x0BV[` \x02` \x01\x01Q\x83\x83`\x02a*\xF6\x91\x90a8\xBAV[a+\x01\x90`\x01a8^V[\x81Q\x81\x10a+\x11Wa+\x11a8\x0BV[` \x02` \x01\x01Q`@Q` \x01a+3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra+M\x91a9\x08V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a+jW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+\x8D\x91\x90a9$V[\x82\x82\x81Q\x81\x10a+\x9FWa+\x9Fa8\x0BV[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a*\xBBV[Pa+\xBE`\x02\x83a8\x87V[\x91Pa*\xB2V[\x80`\0\x81Q\x81\x10a+\xD8Wa+\xD8a8\x0BV[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0a\x0C\xE1\x82`\x05\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[` \x02` \x01\x01Qa.\xF9V[`\0a\x0C\xE1\x82`\x06\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[`\0\x81`\x01\x81Q\x81\x10a\x1A\x8AWa\x1A\x8Aa8\x0BV[`\0a\x0C\xE1\x82`\x02\x81Q\x81\x10a,\x01Wa,\x01a8\x0BV[`\0a,\xA8\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a/`\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80Q`\0\x14\x80a,\xC9WP\x80\x80` \x01\x90Q\x81\x01\x90a,\xC9\x91\x90a7_V[a\x13$W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x13\xC2V[`\0a-6`&`\x01a8^V[a-A\x90` a8\xBAV[a-N`@\x84\x01\x84a7\xC5V[\x90P\x14a-nW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a-{`\x04\x85a;\xC1V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa-\xD5a-\x94`@\x85\x01\x85a7\xC5V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a)8V[a-\xF2W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a.\0\x83` \x015\x85a/oV[\x95\x94PPPPPV[`\0a.\x15\x82\x84a;\xEBV[\x93\x92PPPV[`\0\x83Q`\0\x14\x15\x80\x15a.;WP` \x84Qa.9\x91\x90a9\xF5V[\x15[a.XW`@Qc\x13q}\xA9`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a.\xEFWa.|`\x02\x85a9\xF5V[`\0\x03a.\xB2W\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa.\xA7W`\0\x80\xFD[`\x02\x84\x04\x93Pa.\xDDV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa.\xD6W`\0\x80\xFD[`\x02\x84\x04\x93P[a.\xE8` \x82a8^V[\x90Pa.iV[PQ\x94\x93PPPPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\r\xF3\x84\x84`\0\x85a/\x9CV[`\0\x80a/}`\x04\x84a<\x1AV[a/\x88\x90`@a<DV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\r\xF3\x84\x82\x1Ba.\xF9V[``\x82G\x10\x15a/\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x13\xC2V[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa0\x19\x91\x90a9\x08V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a0VW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a0[V[``\x91P[P\x91P\x91Pa0l\x87\x83\x83\x87a0wV[\x97\x96PPPPPPPV[``\x83\x15a0\xE6W\x82Q`\0\x03a0\xDFW`\x01`\x01`\xA0\x1B\x03\x85\x16;a0\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x13\xC2V[P\x81a\r\xF3V[a\r\xF3\x83\x83\x81Q\x15a0\xFBW\x81Q\x80\x83` \x01\xFD[\x80`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x13\xC2\x91\x90a<dV[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a1,W`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a1CW`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a1^W`\0\x80\xFD[a1g\x84a1\x15V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\x82W`\0\x80\xFD[a1\x8E\x86\x82\x87\x01a11V[\x92PP`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xAAW`\0\x80\xFD[a1\xB6\x86\x82\x87\x01a11V[\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a1\xD2W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE9W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a2\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a2'W`\0\x80\xFD[a20\x89a1\x15V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2KW`\0\x80\xFD[a2W\x8B\x82\x8C\x01a11V[\x97PP`@\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2sW`\0\x80\xFD[a2\x7F\x8B\x82\x8C\x01a1\xC0V[\x90\x97P\x95PP``\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x9EW`\0\x80\xFD[a2\xAA\x8B\x82\x8C\x01a1\xC0V[\x90\x95P\x93PP`\x80\x89\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xC9W`\0\x80\xFD[a2\xD5\x8B\x82\x8C\x01a1\xC0V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15a2\xFBW`\0\x80\xFD[a.\x15\x82a1\x15V[`\0\x80\x83`\x1F\x84\x01\x12a3\x16W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a3-W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a2\x04W`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a3XW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a3nW`\0\x80\xFD[a3z\x85\x82\x86\x01a3\x04V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a3\xBAWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x0C\xE1\x82\x84a3\x9CV[`\0` \x82\x84\x03\x12\x15a3\xDEW`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x83Q\x16\x82R`\x01`\x01`@\x1B\x03` \x84\x01Q\x16` \x83\x01R`\x01`\x01`@\x1B\x03`@\x84\x01Q\x16`@\x83\x01R``\x83\x01Qa42``\x84\x01\x82a3\x9CV[P\x92\x91PPV[\x80\x15\x15\x81\x14a)5W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4YW`\0\x80\xFD[\x815a.\x15\x81a49V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a4|W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a4\x92W`\0\x80\xFD[a4\x9E\x88\x82\x89\x01a3\x04V[\x90\x96P\x94PP` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xBDW`\0\x80\xFD[a4\xC9\x88\x82\x89\x01a3\x04V[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a)5W`\0\x80\xFD[\x805a1,\x81a4\xDBV[`\0\x80`@\x83\x85\x03\x12\x15a5\x0EW`\0\x80\xFD[\x825a5\x19\x81a4\xDBV[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a59W`\0\x80\xFD[\x815a.\x15\x81a4\xDBV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a5\x82Wa5\x82a5DV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a5\xA3Wa5\xA3a5DV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a5\xBEW`\0\x80\xFD[\x815a5\xD1a5\xCC\x82a5\x8AV[a5ZV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a5\xF3W`\0\x80\xFD[` \x85\x01[\x83\x81\x10\x15a6\x10W\x805\x83R` \x92\x83\x01\x92\x01a5\xF8V[P\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a6/W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a6EW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a6VW`\0\x80\xFD[\x805a6da5\xCC\x82a5\x8AV[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x88\x83\x11\x15a6\x86W`\0\x80\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a6\xB1W\x835a6\xA0\x81a4\xDBV[\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a6\x8DV[\x95PPPP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\xCFW`\0\x80\xFD[a6\xDB\x86\x82\x87\x01a5\xADV[\x92PPa6\xEA`@\x85\x01a4\xF0V[\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a7\x08W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x1EW`\0\x80\xFD[a7*\x86\x82\x87\x01a11V[\x93PP` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a7FW`\0\x80\xFD[a7R\x86\x82\x87\x01a1\xC0V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a7qW`\0\x80\xFD[\x81Qa.\x15\x81a49V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a7\x93W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a7\xADW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a2\x04W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a7\xDCW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a7\xF6W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a2\x04W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a83W`\0\x80\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a.\x15W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a8\x96Wa8\x96a8qV[P\x04\x90V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C\xE1Wa\x0C\xE1a8HV[\x81\x81\x03\x81\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0[\x83\x81\x10\x15a8\xFFW\x81\x81\x01Q\x83\x82\x01R` \x01a8\xE7V[PP`\0\x91\x01RV[`\0\x82Qa9\x1A\x81\x84` \x87\x01a8\xE4V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a96W`\0\x80\xFD[PQ\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84Ra9~\x81` \x86\x01` \x86\x01a8\xE4V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0a9\xA6`\x80\x83\x01\x88\x8Aa9=V[\x82\x81\x03` \x84\x01Ra9\xB8\x81\x88a9fV[\x90P\x82\x81\x03`@\x84\x01Ra9\xCD\x81\x86\x88a9=V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0a\r\xF3` \x83\x01\x84\x86a9=V[`\0\x82a:\x04Wa:\x04a8qV[P\x06\x90V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0\x825`^\x19\x836\x03\x01\x81\x12a9\x1AW`\0\x80\xFD[`\0b\xFF\xFF\xFF\x82\x16\x80a:SWa:Sa8HV[`\0\x19\x01\x92\x91PPV[`\x07\x81\x81\x0B\x90\x83\x90\x0B\x01g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a1CW`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0`\x01\x82\x01a:\xC2Wa:\xC2a8HV[P`\x01\x01\x90V[`\0\x83Qa:\xDB\x81\x84` \x88\x01a8\xE4V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81a;\x07Wa;\x07a8HV[P`\0\x19\x01\x90V[`\0\x81`\x07\x0Bg\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x03a;-Wa;-a8HV[`\0\x03\x92\x91PPV[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x82\x12\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a;\x91Wa;\x91a8HV[\x81\x81\x05\x83\x14\x82\x15\x17a\x0C\xE1Wa\x0C\xE1a8HV[`\0`\x01`\xFF\x1B\x82\x01a;\xBAWa;\xBAa8HV[P`\0\x03\x90V[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a;\xD8Wa;\xD8a8qV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[`\x07\x82\x81\x0B\x90\x82\x90\x0B\x03g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x12g\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x0C\xE1Wa\x0C\xE1a8HV[`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80a<1Wa<1a8qV[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a42Wa42a8HV[` \x81R`\0a.\x15` \x83\x01\x84a9fV\xFE\xA2dipfsX\"\x12 \xFEn\x12\x82\r\xA2\r\xC3\xE4?\xBF\x9E!\xD1\t\x9A~d/\xA4k\xA7\x8F\nB(]\xDE}\xF1\x1D\xFEdsolcC\0\x08\x1B\x003",
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
