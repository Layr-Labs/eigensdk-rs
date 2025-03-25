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
    clippy::style,
    clippy::empty_structs_with_brackets
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
        #[allow(missing_docs)]
        pub balanceContainerRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for BalanceContainerProof {
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
        impl alloy_sol_types::SolStruct for BalanceContainerProof {
            const NAME: &'static str = "BalanceContainerProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceContainerProof(bytes32 balanceContainerRoot,bytes proof)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct BalanceProof { bytes32 pubkeyHash; bytes32 balanceRoot; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BalanceProof {
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub balanceRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for BalanceProof {
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
        impl alloy_sol_types::SolStruct for BalanceProof {
            const NAME: &'static str = "BalanceProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "BalanceProof(bytes32 pubkeyHash,bytes32 balanceRoot,bytes proof)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct StateRootProof { bytes32 beaconStateRoot; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StateRootProof {
        #[allow(missing_docs)]
        pub beaconStateRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for StateRootProof {
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
        impl alloy_sol_types::SolStruct for StateRootProof {
            const NAME: &'static str = "StateRootProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StateRootProof(bytes32 beaconStateRoot,bytes proof)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct ValidatorProof { bytes32[] validatorFields; bytes proof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorProof {
        #[allow(missing_docs)]
        pub validatorFields:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        #[allow(missing_docs)]
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            alloy::sol_types::sol_data::Bytes,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
        impl alloy_sol_types::SolType for ValidatorProof {
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
        impl alloy_sol_types::SolStruct for ValidatorProof {
            const NAME: &'static str = "ValidatorProof";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorProof(bytes32[] validatorFields,bytes proof)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("BeaconChainProofsInstance")
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
        > BeaconChainProofsInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BeaconChainProofs`](self) contract instance.

        See the [wrapper's documentation](`BeaconChainProofsInstance`) for more details.*/
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
        > BeaconChainProofsInstance<T, P, N>
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
        > BeaconChainProofsInstance<T, P, N>
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
///Module containing a contract's types and functions.
/**

```solidity
library IEigenPod {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int128 balanceDeltasGwei; }
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
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
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
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
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    /**```solidity
    struct Checkpoint { bytes32 beaconBlockRoot; uint24 proofsRemaining; uint64 podBalanceGwei; int128 balanceDeltasGwei; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Checkpoint {
        #[allow(missing_docs)]
        pub beaconBlockRoot: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub proofsRemaining: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub podBalanceGwei: u64,
        #[allow(missing_docs)]
        pub balanceDeltasGwei: i128,
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
            alloy::sol_types::sol_data::Int<128>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U24,
            u64,
            i128,
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
        impl ::core::convert::From<Checkpoint> for UnderlyingRustTuple<'_> {
            fn from(value: Checkpoint) -> Self {
                (
                    value.beaconBlockRoot,
                    value.proofsRemaining,
                    value.podBalanceGwei,
                    value.balanceDeltasGwei,
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
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.balanceDeltasGwei),
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
        impl alloy_sol_types::SolType for Checkpoint {
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
        impl alloy_sol_types::SolStruct for Checkpoint {
            const NAME: &'static str = "Checkpoint";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Checkpoint(bytes32 beaconBlockRoot,uint24 proofsRemaining,uint64 podBalanceGwei,int128 balanceDeltasGwei)",
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
                        128,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.balanceDeltasGwei,
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
                        128,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.balanceDeltasGwei,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                    128,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.balanceDeltasGwei,
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
    struct ValidatorInfo { uint64 validatorIndex; uint64 restakedBalanceGwei; uint64 lastCheckpointedAt; VALIDATOR_STATUS status; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ValidatorInfo {
        #[allow(missing_docs)]
        pub validatorIndex: u64,
        #[allow(missing_docs)]
        pub restakedBalanceGwei: u64,
        #[allow(missing_docs)]
        pub lastCheckpointedAt: u64,
        #[allow(missing_docs)]
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.restakedBalanceGwei,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.lastCheckpointedAt,
                    ),
                    <VALIDATOR_STATUS as alloy_sol_types::SolType>::tokenize(&self.status),
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
        impl alloy_sol_types::SolType for ValidatorInfo {
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
        impl alloy_sol_types::SolStruct for ValidatorInfo {
            const NAME: &'static str = "ValidatorInfo";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ValidatorInfo(uint64 validatorIndex,uint64 restakedBalanceGwei,uint64 lastCheckpointedAt,uint8 status)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("IEigenPodInstance")
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
        > IEigenPodInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IEigenPod`](self) contract instance.

        See the [wrapper's documentation](`IEigenPodInstance`) for more details.*/
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
        > IEigenPodInstance<T, P, N>
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
        > IEigenPodInstance<T, P, N>
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

library IEigenPod {
    type VALIDATOR_STATUS is uint8;
    struct Checkpoint {
        bytes32 beaconBlockRoot;
        uint24 proofsRemaining;
        uint64 podBalanceGwei;
        int128 balanceDeltasGwei;
    }
    struct ValidatorInfo {
        uint64 validatorIndex;
        uint64 restakedBalanceGwei;
        uint64 lastCheckpointedAt;
        VALIDATOR_STATUS status;
    }
}

interface EigenPod {
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
    function currentCheckpoint() external view returns (IEigenPod.Checkpoint memory);
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
    function validatorPubkeyHashToInfo(bytes32 validatorPubkeyHash) external view returns (IEigenPod.ValidatorInfo memory);
    function validatorPubkeyToInfo(bytes memory validatorPubkey) external view returns (IEigenPod.ValidatorInfo memory);
    function validatorStatus(bytes memory validatorPubkey) external view returns (IEigenPod.VALIDATOR_STATUS);
    function validatorStatus(bytes32 pubkeyHash) external view returns (IEigenPod.VALIDATOR_STATUS);
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
        "internalType": "struct IEigenPod.Checkpoint",
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
            "type": "int128",
            "internalType": "int128"
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
            "name": "lastCheckpointedAt",
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
            "name": "lastCheckpointedAt",
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
pub mod EigenPod {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60e06040523480156200001157600080fd5b5060405162004ad038038062004ad0833981016040819052620000349162000142565b6001600160a01b03808416608052821660a0526001600160401b03811660c0526200005e62000067565b505050620001a1565b600054610100900460ff1615620000d45760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b606482015260840160405180910390fd5b60005460ff908116101562000127576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b6001600160a01b03811681146200013f57600080fd5b50565b6000806000606084860312156200015857600080fd5b8351620001658162000129565b6020850151909350620001788162000129565b60408501519092506001600160401b03811681146200019657600080fd5b809150509250925092565b60805160a05160c0516148b26200021e60003960006105ff0152600081816102bd0152818161063a015281816106ec01528181610abf01528181610d6c015281816110f40152818161119c0152818161143c015281816118db01528181611a8401526131250152600081816104b8015261126701526148b26000f3fe60806040526004361061016a5760003560e01c80636fcd0e53116100d1578063c49074421161008a578063dda3346c11610064578063dda3346c1461058d578063ee94d67c146105ad578063f074ba62146105cd578063f2882461146105ed57600080fd5b8063c49074421461052d578063c4d66de81461054d578063d06d55871461056d57600080fd5b80636fcd0e53146104425780637439841f1461046f57806374cdd798146104a657806388676cad146104da5780639b4e4634146104fa578063b522538a1461050d57600080fd5b80634665bcda116101235780634665bcda146102ab57806347d28372146102df57806352396a591461039f57806358753357146103d557806358eaee79146103f55780636c0d2d5a1461042257600080fd5b8063039157d2146101a95780630b18ff66146101cb5780632340e8d3146102085780633474aa161461022c5780633f65cf191461026457806342ecff2a1461028457600080fd5b366101a4576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156101b557600080fd5b506101c96101c4366004613b66565b610621565b005b3480156101d757600080fd5b506033546101eb906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561021457600080fd5b5061021e60395481565b6040519081526020016101ff565b34801561023857600080fd5b5060345461024c906001600160401b031681565b6040516001600160401b0390911681526020016101ff565b34801561027057600080fd5b506101c961027f366004613c24565b610a67565b34801561029057600080fd5b50603a5461024c90600160401b90046001600160401b031681565b3480156102b757600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102eb57600080fd5b5061035b6040805160808101825260008082526020820181905291810182905260608101919091525060408051608081018252603c548152603d5462ffffff811660208301526001600160401b03630100000082041692820192909252600160581b909104600f0b606082015290565b6040516101ff91908151815260208083015162ffffff16908201526040808301516001600160401b031690820152606091820151600f0b9181019190915260800190565b3480156103ab57600080fd5b5061024c6103ba366004613cf2565b603b602052600090815260409020546001600160401b031681565b3480156103e157600080fd5b50603e546101eb906001600160a01b031681565b34801561040157600080fd5b50610415610410366004613d4e565b610dd6565b6040516101ff9190613dc7565b34801561042e57600080fd5b5061021e61043d366004613cf2565b610e3b565b34801561044e57600080fd5b5061046261045d366004613dd5565b610fef565b6040516101ff9190613dee565b34801561047b57600080fd5b5061041561048a366004613dd5565b600090815260366020526040902054600160c01b900460ff1690565b3480156104b257600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156104e657600080fd5b506101c96104f5366004613e44565b61109c565b6101c9610508366004613e61565b611191565b34801561051957600080fd5b50610462610528366004613d4e565b61133e565b34801561053957600080fd5b506101c9610548366004613ef4565b611431565b34801561055957600080fd5b506101c9610568366004613f20565b61166e565b34801561057957600080fd5b506101c9610588366004613f20565b611805565b34801561059957600080fd5b506101c96105a8366004614011565b611898565b3480156105b957600080fd5b50603a5461024c906001600160401b031681565b3480156105d957600080fd5b506101c96105e83660046140e2565b611a6b565b3480156105f957600080fd5b5061024c7f000000000000000000000000000000000000000000000000000000000000000081565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610689573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ad919061414a565b156106d35760405162461bcd60e51b81526004016106ca90614167565b60405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561073b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061075f919061414a565b1561077c5760405162461bcd60e51b81526004016106ca90614167565b60006107c261078b85806141c4565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ebe92505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561083157610831613d8f565b600281111561084257610842613d8f565b81525050905080604001516001600160401b0316876001600160401b0316116108d5576040805162461bcd60e51b81526020600482015260248101919091527f456967656e506f642e7665726966795374616c6542616c616e63653a2070726f60448201527f6f66206973206f6c646572207468616e206c61737420636865636b706f696e7460648201526084016106ca565b6001816060015160028111156108ed576108ed613d8f565b146109575760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e7665726966795374616c6542616c616e63653a2076616c604482015273696461746f72206973206e6f742061637469766560601b60648201526084016106ca565b61099b61096486806141c4565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ee292505050565b610a1f5760405162461bcd60e51b815260206004820152604960248201527f456967656e506f642e7665726966795374616c6542616c616e63653a2076616c60448201527f696461746f72206d75737420626520736c617368656420746f206265206d61726064820152686b6564207374616c6560b81b608482015260a4016106ca565b610a31610a2b88610e3b565b87611f0c565b610a548635610a4087806141c4565b610a4d60208a018a61420d565b8651612067565b610a5e600061227e565b50505050505050565b6033546001600160a01b0316331480610a8a5750603e546001600160a01b031633145b610aa65760405162461bcd60e51b81526004016106ca90614253565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610b0e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b32919061414a565b15610b4f5760405162461bcd60e51b81526004016106ca90614167565b8584148015610b5d57508382145b610bed5760405162461bcd60e51b815260206004820152605560248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2076616c696461746f72496e646963657320616e642070726f6f666064820152740e640daeae6e840c4ca40e6c2daca40d8cadccee8d605b1b608482015260a4016106ca565b603a546001600160401b03600160401b9091048116908a1611610c8d5760405162461bcd60e51b815260206004820152604c60248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a207370656369666965642074696d657374616d7020697320746f6f60648201526b0819985c881a5b881c185cdd60a21b608482015260a4016106ca565b610c9f610c998a610e3b565b89611f0c565b6000805b87811015610d4257610d248a358a8a84818110610cc257610cc26142c7565b9050602002016020810190610cd791906142dd565b898985818110610ce957610ce96142c7565b9050602002810190610cfb919061420d565b898987818110610d0d57610d0d6142c7565b9050602002810190610d1f91906141c4565b612514565b610d2e908361431a565b915080610d3a81614332565b915050610ca3565b5060335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c4090604401600060405180830381600087803b158015610db257600080fd5b505af1158015610dc6573d6000803e3d6000fd5b5050505050505050505050505050565b600080610e1884848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250612afa92505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b6000610e4a611fff600c61434d565b610e5d6001600160401b0384164261436c565b10610ec65760405162461bcd60e51b815260206004820152603360248201527f456967656e506f642e676574506172656e74426c6f636b526f6f743a2074696d604482015272657374616d70206f7574206f662072616e676560681b60648201526084016106ca565b604080516001600160401b03841660208201526000918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f1981840301815290829052610f0e916143b3565b600060405180830381855afa9150503d8060008114610f49576040519150601f19603f3d011682016040523d82523d6000602084013e610f4e565b606091505b5091509150818015610f61575060008151115b610fd35760405162461bcd60e51b815260206004820152603860248201527f456967656e506f642e676574506172656e74426c6f636b526f6f743a20696e7660448201527f616c696420626c6f636b20726f6f742072657475726e6564000000000000000060648201526084016106ca565b80806020019051810190610fe791906143cf565b949350505050565b6110176040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561108257611082613d8f565b600281111561109357611093613d8f565b90525092915050565b6033546001600160a01b03163314806110bf5750603e546001600160a01b031633145b6110db5760405162461bcd60e51b81526004016106ca90614253565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611143573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611167919061414a565b156111845760405162461bcd60e51b81526004016106ca90614167565b61118d8261227e565b5050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111d95760405162461bcd60e51b81526004016106ca906143e8565b346801bc16d674ec800000146112655760405162461bcd60e51b8152602060048201526044602482018190527f456967656e506f642e7374616b653a206d75737420696e697469616c6c792073908201527f74616b6520666f7220616e792076616c696461746f72207769746820333220656064820152633a3432b960e11b608482015260a4016106ca565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec80000087876112a8612bf4565b8888886040518863ffffffff1660e01b81526004016112cc9695949392919061448e565b6000604051808303818588803b1580156112e557600080fd5b505af11580156112f9573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e23858560405161132f9291906144dd565b60405180910390a15050505050565b6113666040805160808101825260008082526020820181905291810182905290606082015290565b603660006113a985858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250612afa92505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff16600281111561141657611416613d8f565b600281111561142757611427613d8f565b9052509392505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146114795760405162461bcd60e51b81526004016106ca906143e8565b611487633b9aca0082614507565b156115115760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74576569206d75737420626520612077686f60648201526d1b194811ddd95a48185b5bdd5b9d60921b608482015260a4016106ca565b6000611521633b9aca008361451b565b6034549091506001600160401b0390811690821611156115da5760405162461bcd60e51b815260206004820152606260248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74477765692065786365656473207769746860648201527f6472617761626c6552657374616b6564457865637574696f6e4c617965724777608482015261656960f01b60a482015260c4016106ca565b603480548291906000906115f89084906001600160401b031661452f565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e8360405161165791815260200190565b60405180910390a26116698383612c39565b505050565b600054610100900460ff161580801561168e5750600054600160ff909116105b806116a85750303b1580156116a8575060005460ff166001145b61170b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016106ca565b6000805460ff19166001179055801561172e576000805461ff0019166101001790555b6001600160a01b0382166117a15760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e696e697469616c697a653a20706f644f776e65722063616044820152736e6e6f74206265207a65726f206164647265737360601b60648201526084016106ca565b603380546001600160a01b0319166001600160a01b038416179055801561118d576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b0316331461182f5760405162461bcd60e51b81526004016106ca90614557565b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b031633146118c25760405162461bcd60e51b81526004016106ca90614557565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561192a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061194e919061414a565b1561196b5760405162461bcd60e51b81526004016106ca90614167565b82518451146119f65760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e7265636f766572546f6b656e733a20746f6b656e4c697360448201527f7420616e6420616d6f756e7473546f5769746864726177206d7573742062652060648201526a0e6c2daca40d8cadccee8d60ab1b608482015260a4016106ca565b60005b8451811015611a6457611a5283858381518110611a1857611a186142c7565b6020026020010151878481518110611a3257611a326142c7565b60200260200101516001600160a01b0316612d529092919063ffffffff16565b80611a5c81614332565b9150506119f9565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611ad3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611af7919061414a565b15611b145760405162461bcd60e51b81526004016106ca90614167565b603a54600160401b90046001600160401b031680611bc05760405162461bcd60e51b815260206004820152605860248201527f456967656e506f642e766572696679436865636b706f696e7450726f6f66733a60448201527f206d75737420686176652061637469766520636865636b706f696e7420746f2060648201527f706572666f726d20636865636b706f696e742070726f6f660000000000000000608482015260a4016106ca565b60408051608081018252603c54808252603d5462ffffff811660208401526001600160401b03630100000082041693830193909352600160581b909204600f0b606082015290611c109087612da4565b6000805b85811015611e645736878783818110611c2f57611c2f6142c7565b9050602002810190611c41919061459f565b80356000908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611cb257611cb2613d8f565b6002811115611cc357611cc3613d8f565b9052509050600181606001516002811115611ce057611ce0613d8f565b14611cec575050611e52565b856001600160401b031681604001516001600160401b031610611d10575050611e52565b600080611d2083898e3587612f20565b602089018051929450909250611d35826145b5565b62ffffff16905250606087018051839190611d519083906145d4565b600f0b905250611d618187614623565b84356000908152603660209081526040918290208651815492880151938801516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060870151939950869390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115611e0657611e06613d8f565b021790555050835160405164ffffffffff90911691506001600160401b038a16907fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f90600090a3505050505b80611e5c81614332565b915050611c14565b506001600160401b038084166000908152603b6020526040812080548493919291611e9191859116614623565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550610a5e82613042565b600081600081518110611ed357611ed36142c7565b60200260200101519050919050565b600081600381518110611ef757611ef76142c7565b60200260200101516000801b14159050919050565b611f186003602061434d565b611f25602083018361420d565b905014611f9a5760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f743a2050726f6f662068617320696e636f7272656374206c656e67746800000060648201526084016106ca565b611fea611faa602083018361420d565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003613249565b61118d5760405162461bcd60e51b815260206004820152604260248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f743a20496e76616c696420737461746520726f6f74206d65726b6c652070726f60648201526137b360f11b608482015260a4016106ca565b600884146120e25760405162461bcd60e51b815260206004820152604e602482015260008051602061485d83398151915260448201527f724669656c64733a2056616c696461746f72206669656c64732068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a4016106ca565b60056120f06028600161431a565b6120fa919061431a565b61210590602061434d565b82146121735760405162461bcd60e51b8152602060048201526043602482015260008051602061485d83398151915260448201527f724669656c64733a2050726f6f662068617320696e636f7272656374206c656e6064820152620cee8d60eb1b608482015260a4016106ca565b60006121b186868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061326192505050565b9050600064ffffffffff83166121c96028600161431a565b600b901b17905061221485858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050613249565b6122745760405162461bcd60e51b815260206004820152603d602482015260008051602061485d83398151915260448201527f724669656c64733a20496e76616c6964206d65726b6c652070726f6f6600000060648201526084016106ca565b5050505050505050565b603a54600160401b90046001600160401b03161561231f5760405162461bcd60e51b815260206004820152605260248201527f456967656e506f642e5f7374617274436865636b706f696e743a206d7573742060448201527f66696e6973682070726576696f757320636865636b706f696e74206265666f72606482015271329039ba30b93a34b7339030b737ba3432b960711b608482015260a4016106ca565b603a54426001600160401b03908116911614156123a45760405162461bcd60e51b815260206004820152603f60248201527f456967656e506f642e5f7374617274436865636b706f696e743a2063616e6e6f60448201527f7420636865636b706f696e7420747769636520696e206f6e6520626c6f636b0060648201526084016106ca565b6034546000906001600160401b03166123c1633b9aca004761451b565b6123cb919061452f565b90508180156123e157506001600160401b038116155b156124545760405162461bcd60e51b815260206004820152603d60248201527f456967656e506f642e5f7374617274436865636b706f696e743a206e6f20626160448201527f6c616e636520617661696c61626c6520746f20636865636b706f696e7400000060648201526084016106ca565b6000604051806080016040528061246a42610e3b565b815260200160395462ffffff168152602001836001600160401b031681526020016000600f0b815250905042603a60086101000a8154816001600160401b0302191690836001600160401b031602179055506124c581613042565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b600080612553848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ebe92505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156125c2576125c2613d8f565b60028111156125d3576125d3613d8f565b90525090506000816060015160028111156125f0576125f0613d8f565b146126815760405162461bcd60e51b8152602060048201526061602482015260008051602061483d83398151915260448201527f7469616c733a2076616c696461746f72206d75737420626520696e616374697660648201527f6520746f2070726f7665207769746864726177616c2063726564656e7469616c6084820152607360f81b60a482015260c4016106ca565b6001600160401b0380166126c786868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061350e92505050565b6001600160401b031614156127505760405162461bcd60e51b8152602060048201526055602482015260008051602061483d83398151915260448201527f7469616c733a2076616c696461746f72206d75737420626520696e207468652060648201527470726f63657373206f662061637469766174696e6760581b608482015260a4016106ca565b6001600160401b03801661279686868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061353392505050565b6001600160401b03161461280e5760405162461bcd60e51b81526020600482015260446024820181905260008051602061483d833981519152908201527f7469616c733a2076616c696461746f72206d757374206e6f742062652065786960648201526374696e6760e01b608482015260a4016106ca565b612816612bf4565b61281f9061464e565b61285b86868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061354b92505050565b146128ca5760405162461bcd60e51b8152602060048201526045602482015260008051602061483d83398151915260448201527f7469616c733a2070726f6f66206973206e6f7420666f72207468697320456967606482015264195b941bd960da1b608482015260a4016106ca565b600061290886868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061356092505050565b90506129188a87878b8b8e612067565b6039805490600061292883614332565b9091555050603a54600090600160401b90046001600160401b03161561296057603a54600160401b90046001600160401b031661296d565b603a546001600160401b03165b6040805160808101825264ffffffffff8d1681526001600160401b03858116602083015283169181019190915290915060608101600190526000858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115612a4357612a43613d8f565b02179055505060405164ffffffffff8c1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a16040805164ffffffffff8c1681526001600160401b03838116602083015284168183015290517f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df9181900360600190a1612aeb633b9aca006001600160401b03841661434d565b9b9a5050505050505050505050565b60008151603014612b835760405162461bcd60e51b815260206004820152604760248201527f456967656e506f642e5f63616c63756c61746556616c696461746f725075626b60448201527f657948617368206d75737420626520612034382d6279746520424c53207075626064820152666c6963206b657960c81b608482015260a4016106ca565b604051600290612b9a908490600090602001614672565b60408051601f1981840301815290829052612bb4916143b3565b602060405180830381855afa158015612bd1573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190610e3591906143cf565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b80471015612c895760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016106ca565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114612cd6576040519150601f19603f3d011682016040523d82523d6000602084013e612cdb565b606091505b50509050806116695760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016106ca565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611669908490613578565b612db06005600361431a565b612dbb90602061434d565b612dc8602083018361420d565b905014612e4b5760405162461bcd60e51b8152602060048201526044602482018190527f426561636f6e436861696e50726f6f66732e76657269667942616c616e636543908201527f6f6e7461696e65723a2050726f6f662068617320696e636f7272656374206c656064820152630dccee8d60e31b608482015260a4016106ca565b606c612e9c612e5d602084018461420d565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084613249565b6116695760405162461bcd60e51b815260206004820152604960248201527f426561636f6e436861696e50726f6f66732e76657269667942616c616e63654360448201527f6f6e7461696e65723a20696e76616c69642062616c616e636520636f6e7461696064820152683732b910383937b7b360b91b608482015260a4016106ca565b83516020850151600091829182612f3887848861364a565b9050816001600160401b0316816001600160401b031614612fb257612f5d81836137c1565b6040805164ffffffffff861681526001600160401b038b8116602083015284168183015290519196507f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df919081900360600190a15b6001600160401b0380821660208b0181905290891660408b01526130365760398054906000612fe0836146a1565b9091555050600260608a0152612ff5856146b8565b93508264ffffffffff16886001600160401b03167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a60405160405180910390a35b50505094509492505050565b602081015162ffffff166131c9576000633b9aca00826060015183604001516001600160401b031661307491906145d4565b600f0b61308191906146df565b60408301516034805492935090916000906130a69084906001600160401b0316614623565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b81049092166001600160801b0319909216919091179055506000603c55603d80546001600160d81b031916905560335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c4090604401600060405180830381600087803b15801561316b57600080fd5b505af115801561317f573d6000803e3d6000fd5b5050603a546040518481526001600160401b0390911692507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44915060200160405180910390a25050565b8051603c556020810151603d8054604084015160608501516fffffffffffffffffffffffffffffffff16600160581b026fffffffffffffffffffffffffffffffff60581b196001600160401b039092166301000000026affffffffffffffffffffff1990931662ffffff9095169490941791909117169190911790555b50565b6000836132578685856137d9565b1495945050505050565b60008060028351613272919061451b565b90506000816001600160401b0381111561328e5761328e613f3d565b6040519080825280602002602001820160405280156132b7578160200160208202803683370190505b50905060005b828110156133be576002856132d2838361434d565b815181106132e2576132e26142c7565b6020026020010151868360026132f8919061434d565b61330390600161431a565b81518110613313576133136142c7565b6020026020010151604051602001613335929190918252602082015260400190565b60408051601f198184030181529082905261334f916143b3565b602060405180830381855afa15801561336c573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061338f91906143cf565b8282815181106133a1576133a16142c7565b6020908102919091010152806133b681614332565b9150506132bd565b506133ca60028361451b565b91505b81156134ea5760005b828110156134d7576002826133eb838361434d565b815181106133fb576133fb6142c7565b602002602001015183836002613411919061434d565b61341c90600161431a565b8151811061342c5761342c6142c7565b602002602001015160405160200161344e929190918252602082015260400190565b60408051601f1981840301815290829052613468916143b3565b602060405180830381855afa158015613485573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906134a891906143cf565b8282815181106134ba576134ba6142c7565b6020908102919091010152806134cf81614332565b9150506133d6565b506134e360028361451b565b91506133cd565b806000815181106134fd576134fd6142c7565b602002602001015192505050919050565b6000610e3582600581518110613526576135266142c7565b6020026020010151613925565b6000610e3582600681518110613526576135266142c7565b600081600181518110611ed357611ed36142c7565b6000610e3582600281518110613526576135266142c7565b60006135cd826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661398c9092919063ffffffff16565b80519091501561166957808060200190518101906135eb919061414a565b6116695760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016106ca565b60006136586026600161431a565b61366390602061434d565b613670604084018461420d565b9050146136e15760405162461bcd60e51b81526020600482015260446024820181905260008051602061485d833981519152908201527f7242616c616e63653a2050726f6f662068617320696e636f7272656374206c656064820152630dccee8d60e31b608482015260a4016106ca565b60006136ee600485614764565b64ffffffffff169050613748613707604085018561420d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584613249565b6137a85760405162461bcd60e51b815260206004820152603e602482015260008051602061485d83398151915260448201527f7242616c616e63653a20496e76616c6964206d65726b6c652070726f6f66000060648201526084016106ca565b6137b683602001358561399b565b9150505b9392505050565b60006137ba6001600160401b03808416908516614788565b600083516000141580156137f85750602084516137f69190614507565b155b6138875760405162461bcd60e51b815260206004820152605460248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f665368613260448201527f35363a2070726f6f66206c656e6774682073686f756c642062652061206e6f6e60648201527316bd32b9379036bab63a34b836329037b310199960611b608482015260a4016106ca565b604080516020808201909252848152905b8551811161391b576138ab600285614507565b6138de578151600052808601516020526020826040600060026107d05a03fa6138d357600080fd5b600284049350613909565b8086015160005281516020526020826040600060026107d05a03fa61390257600080fd5b6002840493505b61391460208261431a565b9050613898565b5051949350505050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6060610fe784846000856139c8565b6000806139a96004846147d8565b6139b49060406147fc565b64ffffffffff169050610fe784821b613925565b606082471015613a295760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016106ca565b6001600160a01b0385163b613a805760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016106ca565b600080866001600160a01b03168587604051613a9c91906143b3565b60006040518083038185875af1925050503d8060008114613ad9576040519150601f19603f3d011682016040523d82523d6000602084013e613ade565b606091505b5091509150613aee828286613af9565b979650505050505050565b60608315613b085750816137ba565b825115613b185782518084602001fd5b8160405162461bcd60e51b81526004016106ca9190614829565b80356001600160401b0381168114613b4957600080fd5b919050565b600060408284031215613b6057600080fd5b50919050565b600080600060608486031215613b7b57600080fd5b613b8484613b32565b925060208401356001600160401b0380821115613ba057600080fd5b613bac87838801613b4e565b93506040860135915080821115613bc257600080fd5b50613bcf86828701613b4e565b9150509250925092565b60008083601f840112613beb57600080fd5b5081356001600160401b03811115613c0257600080fd5b6020830191508360208260051b8501011115613c1d57600080fd5b9250929050565b60008060008060008060008060a0898b031215613c4057600080fd5b613c4989613b32565b975060208901356001600160401b0380821115613c6557600080fd5b613c718c838d01613b4e565b985060408b0135915080821115613c8757600080fd5b613c938c838d01613bd9565b909850965060608b0135915080821115613cac57600080fd5b613cb88c838d01613bd9565b909650945060808b0135915080821115613cd157600080fd5b50613cde8b828c01613bd9565b999c989b5096995094979396929594505050565b600060208284031215613d0457600080fd5b6137ba82613b32565b60008083601f840112613d1f57600080fd5b5081356001600160401b03811115613d3657600080fd5b602083019150836020828501011115613c1d57600080fd5b60008060208385031215613d6157600080fd5b82356001600160401b03811115613d7757600080fd5b613d8385828601613d0d565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b60038110613dc357634e487b7160e01b600052602160045260246000fd5b9052565b60208101610e358284613da5565b600060208284031215613de757600080fd5b5035919050565b60006080820190506001600160401b03808451168352806020850151166020840152806040850151166040840152506060830151613e2f6060840182613da5565b5092915050565b801515811461324657600080fd5b600060208284031215613e5657600080fd5b81356137ba81613e36565b600080600080600060608688031215613e7957600080fd5b85356001600160401b0380821115613e9057600080fd5b613e9c89838a01613d0d565b90975095506020880135915080821115613eb557600080fd5b50613ec288828901613d0d565b96999598509660400135949350505050565b6001600160a01b038116811461324657600080fd5b8035613b4981613ed4565b60008060408385031215613f0757600080fd5b8235613f1281613ed4565b946020939093013593505050565b600060208284031215613f3257600080fd5b81356137ba81613ed4565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b0381118282101715613f7b57613f7b613f3d565b604052919050565b60006001600160401b03821115613f9c57613f9c613f3d565b5060051b60200190565b600082601f830112613fb757600080fd5b81356020613fcc613fc783613f83565b613f53565b82815260059290921b84018101918181019086841115613feb57600080fd5b8286015b848110156140065780358352918301918301613fef565b509695505050505050565b60008060006060848603121561402657600080fd5b83356001600160401b038082111561403d57600080fd5b818601915086601f83011261405157600080fd5b81356020614061613fc783613f83565b82815260059290921b8401810191818101908a84111561408057600080fd5b948201945b838610156140a757853561409881613ed4565b82529482019490820190614085565b975050870135925050808211156140bd57600080fd5b506140ca86828701613fa6565b9250506140d960408501613ee9565b90509250925092565b6000806000604084860312156140f757600080fd5b83356001600160401b038082111561410e57600080fd5b61411a87838801613b4e565b9450602086013591508082111561413057600080fd5b5061413d86828701613bd9565b9497909650939450505050565b60006020828403121561415c57600080fd5b81516137ba81613e36565b6020808252603e908201527f456967656e506f642e6f6e6c795768656e4e6f745061757365643a20696e646560408201527f782069732070617573656420696e20456967656e506f644d616e616765720000606082015260800190565b6000808335601e198436030181126141db57600080fd5b8301803591506001600160401b038211156141f557600080fd5b6020019150600581901b3603821315613c1d57600080fd5b6000808335601e1984360301811261422457600080fd5b8301803591506001600160401b0382111561423e57600080fd5b602001915036819003821315613c1d57600080fd5b6020808252604e908201527f456967656e506f642e6f6e6c794f776e65724f7250726f6f665375626d69747460408201527f65723a2063616c6c6572206973206e6f7420706f64206f776e6572206f72207060608201526d3937b7b31039bab136b4ba3a32b960911b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b6000602082840312156142ef57600080fd5b813564ffffffffff811681146137ba57600080fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561432d5761432d614304565b500190565b600060001982141561434657614346614304565b5060010190565b600081600019048311821515161561436757614367614304565b500290565b60008282101561437e5761437e614304565b500390565b60005b8381101561439e578181015183820152602001614386565b838111156143ad576000848401525b50505050565b600082516143c5818460208701614383565b9190910192915050565b6000602082840312156143e157600080fd5b5051919050565b60208082526031908201527f456967656e506f642e6f6e6c79456967656e506f644d616e616765723a206e6f6040820152703a1032b4b3b2b72837b226b0b730b3b2b960791b606082015260800190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000815180845261447a816020860160208601614383565b601f01601f19169290920160200192915050565b6080815260006144a260808301888a614439565b82810360208401526144b48188614462565b905082810360408401526144c9818688614439565b915050826060830152979650505050505050565b602081526000610fe7602083018486614439565b634e487b7160e01b600052601260045260246000fd5b600082614516576145166144f1565b500690565b60008261452a5761452a6144f1565b500490565b60006001600160401b038381169083168181101561454f5761454f614304565b039392505050565b60208082526028908201527f456967656e506f642e6f6e6c79456967656e506f644f776e65723a206e6f74206040820152673837b227bbb732b960c11b606082015260800190565b60008235605e198336030181126143c557600080fd5b600062ffffff8216806145ca576145ca614304565b6000190192915050565b600081600f0b83600f0b600082128260016001607f1b03038213811516156145fe576145fe614304565b8260016001607f1b031903821281161561461a5761461a614304565b50019392505050565b60006001600160401b0380831681851680830382111561464557614645614304565b01949350505050565b80516020808301519190811015613b605760001960209190910360031b1b16919050565b60008351614684818460208801614383565b6001600160801b0319939093169190920190815260100192915050565b6000816146b0576146b0614304565b506000190190565b600081600f0b60016001607f1b03198114156146d6576146d6614304565b60000392915050565b60006001600160ff1b038184138284138082168684048611161561470557614705614304565b600160ff1b600087128281168783058912161561472457614724614304565b6000871292508782058712848416161561474057614740614304565b8785058712818416161561475657614756614304565b505050929093029392505050565b600064ffffffffff8084168061477c5761477c6144f1565b92169190910492915050565b600081600f0b83600f0b600081128160016001607f1b0319018312811516156147b3576147b3614304565b8160016001607f1b030183138116156147ce576147ce614304565b5090039392505050565b600064ffffffffff808416806147f0576147f06144f1565b92169190910692915050565b600064ffffffffff8083168185168183048111821515161561482057614820614304565b02949350505050565b6020815260006137ba602083018461446256fe456967656e506f642e5f7665726966795769746864726177616c43726564656e426561636f6e436861696e50726f6f66732e76657269667956616c696461746fa2646970667358221220f9f5d60682fdf4bd864fc835508826f6acc08906f8f542cf21a19958bf5c855064736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xE0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0J\xD08\x03\x80b\0J\xD0\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01BV[`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x80R\x82\x16`\xA0R`\x01`\x01`@\x1B\x03\x81\x16`\xC0Rb\0\0^b\0\0gV[PPPb\0\x01\xA1V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\0\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01'W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01?W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x01XW`\0\x80\xFD[\x83Qb\0\x01e\x81b\0\x01)V[` \x85\x01Q\x90\x93Pb\0\x01x\x81b\0\x01)V[`@\x85\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14b\0\x01\x96W`\0\x80\xFD[\x80\x91PP\x92P\x92P\x92V[`\x80Q`\xA0Q`\xC0QaH\xB2b\0\x02\x1E`\09`\0a\x05\xFF\x01R`\0\x81\x81a\x02\xBD\x01R\x81\x81a\x06:\x01R\x81\x81a\x06\xEC\x01R\x81\x81a\n\xBF\x01R\x81\x81a\rl\x01R\x81\x81a\x10\xF4\x01R\x81\x81a\x11\x9C\x01R\x81\x81a\x14<\x01R\x81\x81a\x18\xDB\x01R\x81\x81a\x1A\x84\x01Ra1%\x01R`\0\x81\x81a\x04\xB8\x01Ra\x12g\x01RaH\xB2`\0\xF3\xFE`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80co\xCD\x0ES\x11a\0\xD1W\x80c\xC4\x90tB\x11a\0\x8AW\x80c\xDD\xA34l\x11a\0dW\x80c\xDD\xA34l\x14a\x05\x8DW\x80c\xEE\x94\xD6|\x14a\x05\xADW\x80c\xF0t\xBAb\x14a\x05\xCDW\x80c\xF2\x88$a\x14a\x05\xEDW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x05-W\x80c\xC4\xD6m\xE8\x14a\x05MW\x80c\xD0mU\x87\x14a\x05mW`\0\x80\xFD[\x80co\xCD\x0ES\x14a\x04BW\x80ct9\x84\x1F\x14a\x04oW\x80ct\xCD\xD7\x98\x14a\x04\xA6W\x80c\x88gl\xAD\x14a\x04\xDAW\x80c\x9BNF4\x14a\x04\xFAW\x80c\xB5\"S\x8A\x14a\x05\rW`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01#W\x80cFe\xBC\xDA\x14a\x02\xABW\x80cG\xD2\x83r\x14a\x02\xDFW\x80cR9jY\x14a\x03\x9FW\x80cXu3W\x14a\x03\xD5W\x80cX\xEA\xEEy\x14a\x03\xF5W\x80cl\r-Z\x14a\x04\"W`\0\x80\xFD[\x80c\x03\x91W\xD2\x14a\x01\xA9W\x80c\x0B\x18\xFFf\x14a\x01\xCBW\x80c#@\xE8\xD3\x14a\x02\x08W\x80c4t\xAA\x16\x14a\x02,W\x80c?e\xCF\x19\x14a\x02dW\x80cB\xEC\xFF*\x14a\x02\x84W`\0\x80\xFD[6a\x01\xA4W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a;fV[a\x06!V[\0[4\x80\x15a\x01\xD7W`\0\x80\xFD[P`3Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x1E`9T\x81V[`@Q\x90\x81R` \x01a\x01\xFFV[4\x80\x15a\x028W`\0\x80\xFD[P`4Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x01\xC9a\x02\x7F6`\x04a<$V[a\ngV[4\x80\x15a\x02\x90W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x02\xB7W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x03[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RP`@\x80Q`\x80\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01`X\x1B\x90\x91\x04`\x0F\x0B``\x82\x01R\x90V[`@Qa\x01\xFF\x91\x90\x81Q\x81R` \x80\x83\x01Qb\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x02La\x03\xBA6`\x04a<\xF2V[`;` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03\xE1W`\0\x80\xFD[P`>Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x01W`\0\x80\xFD[Pa\x04\x15a\x04\x106`\x04a=NV[a\r\xD6V[`@Qa\x01\xFF\x91\x90a=\xC7V[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x02\x1Ea\x04=6`\x04a<\xF2V[a\x0E;V[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x04ba\x04]6`\x04a=\xD5V[a\x0F\xEFV[`@Qa\x01\xFF\x91\x90a=\xEEV[4\x80\x15a\x04{W`\0\x80\xFD[Pa\x04\x15a\x04\x8A6`\x04a=\xD5V[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x04\xB2W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\xE6W`\0\x80\xFD[Pa\x01\xC9a\x04\xF56`\x04a>DV[a\x10\x9CV[a\x01\xC9a\x05\x086`\x04a>aV[a\x11\x91V[4\x80\x15a\x05\x19W`\0\x80\xFD[Pa\x04ba\x05(6`\x04a=NV[a\x13>V[4\x80\x15a\x059W`\0\x80\xFD[Pa\x01\xC9a\x05H6`\x04a>\xF4V[a\x141V[4\x80\x15a\x05YW`\0\x80\xFD[Pa\x01\xC9a\x05h6`\x04a? V[a\x16nV[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x01\xC9a\x05\x886`\x04a? V[a\x18\x05V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x01\xC9a\x05\xA86`\x04a@\x11V[a\x18\x98V[4\x80\x15a\x05\xB9W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x05\xD9W`\0\x80\xFD[Pa\x01\xC9a\x05\xE86`\x04a@\xE2V[a\x1AkV[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02L\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAD\x91\x90aAJV[\x15a\x06\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07_\x91\x90aAJV[\x15a\x07|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`\0a\x07\xC2a\x07\x8B\x85\x80aA\xC4V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xBE\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x081Wa\x081a=\x8FV[`\x02\x81\x11\x15a\x08BWa\x08Ba=\x8FV[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\x08\xD5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPod.verifyStaleBalance: pro`D\x82\x01R\x7Fof is older than last checkpoint`d\x82\x01R`\x84\x01a\x06\xCAV[`\x01\x81``\x01Q`\x02\x81\x11\x15a\x08\xEDWa\x08\xEDa=\x8FV[\x14a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.verifyStaleBalance: val`D\x82\x01Rsidator is not active``\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[a\t\x9Ba\td\x86\x80aA\xC4V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xE2\x92PPPV[a\n\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FEigenPod.verifyStaleBalance: val`D\x82\x01R\x7Fidator must be slashed to be mar`d\x82\x01Rhked stale`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a\n1a\n+\x88a\x0E;V[\x87a\x1F\x0CV[a\nT\x865a\n@\x87\x80aA\xC4V[a\nM` \x8A\x01\x8AaB\rV[\x86Qa gV[a\n^`\0a\"~V[PPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\n\x8AWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\n\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aBSV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B2\x91\x90aAJV[\x15a\x0BOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[\x85\x84\x14\x80\x15a\x0B]WP\x83\x82\x14[a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: validatorIndices and proof`d\x82\x01Rt\x0Ed\r\xAE\xAEn\x84\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`[\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\x0C\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: specified timestamp is too`d\x82\x01Rk\x08\x19\x98\\\x88\x1A[\x88\x1C\x18\\\xDD`\xA2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a\x0C\x9Fa\x0C\x99\x8Aa\x0E;V[\x89a\x1F\x0CV[`\0\x80[\x87\x81\x10\x15a\rBWa\r$\x8A5\x8A\x8A\x84\x81\x81\x10a\x0C\xC2Wa\x0C\xC2aB\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD7\x91\x90aB\xDDV[\x89\x89\x85\x81\x81\x10a\x0C\xE9Wa\x0C\xE9aB\xC7V[\x90P` \x02\x81\x01\x90a\x0C\xFB\x91\x90aB\rV[\x89\x89\x87\x81\x81\x10a\r\rWa\r\raB\xC7V[\x90P` \x02\x81\x01\x90a\r\x1F\x91\x90aA\xC4V[a%\x14V[a\r.\x90\x83aC\x1AV[\x91P\x80a\r:\x81aC2V[\x91PPa\x0C\xA3V[P`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xC6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[`\0\x80a\x0E\x18\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xFA\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`\0a\x0EJa\x1F\xFF`\x0CaCMV[a\x0E]`\x01`\x01`@\x1B\x03\x84\x16BaClV[\x10a\x0E\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FEigenPod.getParentBlockRoot: tim`D\x82\x01Rrestamp out of range`h\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\0\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0F\x0E\x91aC\xB3V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0FIW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0FNV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0FaWP`\0\x81Q\x11[a\x0F\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FEigenPod.getParentBlockRoot: inv`D\x82\x01R\x7Falid block root returned\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[\x80\x80` \x01\x90Q\x81\x01\x90a\x0F\xE7\x91\x90aC\xCFV[\x94\x93PPPPV[a\x10\x17`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x10\x82Wa\x10\x82a=\x8FV[`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a=\x8FV[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x10\xBFWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x10\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aBSV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11g\x91\x90aAJV[\x15a\x11\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[a\x11\x8D\x82a\"~V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aC\xE8V[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x12eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FEigenPod.stake: must initially s\x90\x82\x01R\x7Ftake for any validator with 32 e`d\x82\x01Rc:42\xB9`\xE1\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x12\xA8a+\xF4V[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xCC\x96\x95\x94\x93\x92\x91\x90aD\x8EV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xF9W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x13/\x92\x91\x90aD\xDDV[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x13f`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x13\xA9\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xFA\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\x16Wa\x14\x16a=\x8FV[`\x02\x81\x11\x15a\x14'Wa\x14'a=\x8FV[\x90RP\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aC\xE8V[a\x14\x87c;\x9A\xCA\0\x82aE\x07V[\x15a\x15\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountWei must be a who`d\x82\x01Rm\x1B\x19H\x11\xDD\xD9ZH\x18[[\xDD[\x9D`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a\x15!c;\x9A\xCA\0\x83aE\x1BV[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x15\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`b`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountGwei exceeds with`d\x82\x01R\x7FdrawableRestakedExecutionLayerGw`\x84\x82\x01Raei`\xF0\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xCAV[`4\x80T\x82\x91\x90`\0\x90a\x15\xF8\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aE/V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x16W\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x16i\x83\x83a,9V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x16\x8EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x16\xA8WP0;\x15\x80\x15a\x16\xA8WP`\0T`\xFF\x16`\x01\x14[a\x17\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17.W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x17\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.initialize: podOwner ca`D\x82\x01Rsnnot be zero address``\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x11\x8DW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aEWV[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aEWV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19N\x91\x90aAJV[\x15a\x19kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[\x82Q\x84Q\x14a\x19\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.recoverTokens: tokenLis`D\x82\x01R\x7Ft and amountsToWithdraw must be `d\x82\x01Rj\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0[\x84Q\x81\x10\x15a\x1AdWa\x1AR\x83\x85\x83\x81Q\x81\x10a\x1A\x18Wa\x1A\x18aB\xC7V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x1A2Wa\x1A2aB\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a-R\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x1A\\\x81aC2V[\x91PPa\x19\xF9V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF7\x91\x90aAJV[\x15a\x1B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80a\x1B\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`X`$\x82\x01R\x7FEigenPod.verifyCheckpointProofs:`D\x82\x01R\x7F must have active checkpoint to `d\x82\x01R\x7Fperform checkpoint proof\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@\x80Q`\x80\x81\x01\x82R`<T\x80\x82R`=Tb\xFF\xFF\xFF\x81\x16` \x84\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x90\x92\x04`\x0F\x0B``\x82\x01R\x90a\x1C\x10\x90\x87a-\xA4V[`\0\x80[\x85\x81\x10\x15a\x1EdW6\x87\x87\x83\x81\x81\x10a\x1C/Wa\x1C/aB\xC7V[\x90P` \x02\x81\x01\x90a\x1CA\x91\x90aE\x9FV[\x805`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C\xB2Wa\x1C\xB2a=\x8FV[`\x02\x81\x11\x15a\x1C\xC3Wa\x1C\xC3a=\x8FV[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x1C\xE0Wa\x1C\xE0a=\x8FV[\x14a\x1C\xECWPPa\x1ERV[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x1D\x10WPPa\x1ERV[`\0\x80a\x1D \x83\x89\x8E5\x87a/ V[` \x89\x01\x80Q\x92\x94P\x90\x92Pa\x1D5\x82aE\xB5V[b\xFF\xFF\xFF\x16\x90RP``\x87\x01\x80Q\x83\x91\x90a\x1DQ\x90\x83\x90aE\xD4V[`\x0F\x0B\x90RPa\x1Da\x81\x87aF#V[\x845`\0\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x86Q\x81T\x92\x88\x01Q\x93\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x87\x01Q\x93\x99P\x86\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\x1E\x06Wa\x1E\x06a=\x8FV[\x02\x17\x90UPP\x83Q`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P`\x01`\x01`@\x1B\x03\x8A\x16\x90\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?\x90`\0\x90\xA3PPPP[\x80a\x1E\\\x81aC2V[\x91PPa\x1C\x14V[P`\x01`\x01`@\x1B\x03\x80\x84\x16`\0\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a\x1E\x91\x91\x85\x91\x16aF#V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\n^\x82a0BV[`\0\x81`\0\x81Q\x81\x10a\x1E\xD3Wa\x1E\xD3aB\xC7V[` \x02` \x01\x01Q\x90P\x91\x90PV[`\0\x81`\x03\x81Q\x81\x10a\x1E\xF7Wa\x1E\xF7aB\xC7V[` \x02` \x01\x01Q`\0\x80\x1B\x14\x15\x90P\x91\x90PV[a\x1F\x18`\x03` aCMV[a\x1F%` \x83\x01\x83aB\rV[\x90P\x14a\x1F\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7Ft: Proof has incorrect length\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[a\x1F\xEAa\x1F\xAA` \x83\x01\x83aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a2IV[a\x11\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7Ft: Invalid state root merkle pro`d\x82\x01Ra7\xB3`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x08\x84\x14a \xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Validator fields has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x05a \xF0`(`\x01aC\x1AV[a \xFA\x91\x90aC\x1AV[a!\x05\x90` aCMV[\x82\x14a!sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Proof has incorrect len`d\x82\x01Rb\x0C\xEE\x8D`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a!\xB1\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa2a\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a!\xC9`(`\x01aC\x1AV[`\x0B\x90\x1B\x17\x90Pa\"\x14\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa2IV[a\"tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Invalid merkle proof\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[PPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a#\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPod._startCheckpoint: must `D\x82\x01R\x7Ffinish previous checkpoint befor`d\x82\x01Rq2\x909\xBA0\xB9:4\xB73\x900\xB77\xBA42\xB9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`:TB`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14\x15a#\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FEigenPod._startCheckpoint: canno`D\x82\x01R\x7Ft checkpoint twice in one block\0`d\x82\x01R`\x84\x01a\x06\xCAV[`4T`\0\x90`\x01`\x01`@\x1B\x03\x16a#\xC1c;\x9A\xCA\0GaE\x1BV[a#\xCB\x91\x90aE/V[\x90P\x81\x80\x15a#\xE1WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a$TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEigenPod._startCheckpoint: no ba`D\x82\x01R\x7Flance available to checkpoint\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[`\0`@Q\x80`\x80\x01`@R\x80a$jBa\x0E;V[\x81R` \x01`9Tb\xFF\xFF\xFF\x16\x81R` \x01\x83`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x0F\x0B\x81RP\x90PB`:`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa$\xC5\x81a0BV[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a%S\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xBE\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a%\xC2Wa%\xC2a=\x8FV[`\x02\x81\x11\x15a%\xD3Wa%\xD3a=\x8FV[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a%\xF0Wa%\xF0a=\x8FV[\x14a&\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: validator must be inactiv`d\x82\x01R\x7Fe to prove withdrawal credential`\x84\x82\x01R`s`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xCAV[`\x01`\x01`@\x1B\x03\x80\x16a&\xC7\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14\x15a'PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: validator must be in the `d\x82\x01Rtprocess of activating`X\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x01`\x01`@\x1B\x03\x80\x16a'\x96\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa53\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a(\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aH=\x839\x81Q\x91R\x90\x82\x01R\x7Ftials: validator must not be exi`d\x82\x01Rcting`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a(\x16a+\xF4V[a(\x1F\x90aFNV[a([\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5K\x92PPPV[\x14a(\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: proof is not for this Eig`d\x82\x01Rd\x19[\x94\x1B\xD9`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a)\x08\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5`\x92PPPV[\x90Pa)\x18\x8A\x87\x87\x8B\x8B\x8Ea gV[`9\x80T\x90`\0a)(\x83aC2V[\x90\x91UPP`:T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a)`W`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a)mV[`:T`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`@\x1B\x03\x85\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R\x90\x91P``\x81\x01`\x01\x90R`\0\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a*CWa*Ca=\x8FV[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x81\x90\x03``\x01\x90\xA1a*\xEBc;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16aCMV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a+\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigenPod._calculateValidatorPubk`D\x82\x01R\x7FeyHash must be a 48-byte BLS pub`d\x82\x01Rflic key`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@Q`\x02\x90a+\x9A\x90\x84\x90`\0\x90` \x01aFrV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra+\xB4\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a+\xD1W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E5\x91\x90aC\xCFV[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80G\x10\x15a,\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x06\xCAV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a,\xD6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a,\xDBV[``\x91P[PP\x90P\x80a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x16i\x90\x84\x90a5xV[a-\xB0`\x05`\x03aC\x1AV[a-\xBB\x90` aCMV[a-\xC8` \x83\x01\x83aB\rV[\x90P\x14a.KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBeaconChainProofs.verifyBalanceC\x90\x82\x01R\x7Fontainer: Proof has incorrect le`d\x82\x01Rc\r\xCC\xEE\x8D`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`la.\x9Ca.]` \x84\x01\x84aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a2IV[a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FBeaconChainProofs.verifyBalanceC`D\x82\x01R\x7Fontainer: invalid balance contai`d\x82\x01Rh72\xB9\x10897\xB7\xB3`\xB9\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[\x83Q` \x85\x01Q`\0\x91\x82\x91\x82a/8\x87\x84\x88a6JV[\x90P\x81`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a/\xB2Wa/]\x81\x83a7\xC1V[`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x01`\x01`@\x1B\x03\x8B\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x96P\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8B\x01\x81\x90R\x90\x89\x16`@\x8B\x01Ra06W`9\x80T\x90`\0a/\xE0\x83aF\xA1V[\x90\x91UPP`\x02``\x8A\x01Ra/\xF5\x85aF\xB8V[\x93P\x82d\xFF\xFF\xFF\xFF\xFF\x16\x88`\x01`\x01`@\x1B\x03\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj`@Q`@Q\x80\x91\x03\x90\xA3[PPP\x94P\x94\x92PPPV[` \x81\x01Qb\xFF\xFF\xFF\x16a1\xC9W`\0c;\x9A\xCA\0\x82``\x01Q\x83`@\x01Q`\x01`\x01`@\x1B\x03\x16a0t\x91\x90aE\xD4V[`\x0F\x0Ba0\x81\x91\x90aF\xDFV[`@\x83\x01Q`4\x80T\x92\x93P\x90\x91`\0\x90a0\xA6\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aF#V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UP`\0`<U`=\x80T`\x01`\x01`\xD8\x1B\x03\x19\x16\x90U`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\x7FW=`\0\x80>=`\0\xFD[PP`:T`@Q\x84\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16\x92P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x91P` \x01`@Q\x80\x91\x03\x90\xA2PPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`X\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19`\x01`\x01`@\x1B\x03\x90\x92\x16c\x01\0\0\0\x02j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16b\xFF\xFF\xFF\x90\x95\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U[PV[`\0\x83a2W\x86\x85\x85a7\xD9V[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa2r\x91\x90aE\x1BV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x8EWa2\x8Ea?=V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a3\xBEW`\x02\x85a2\xD2\x83\x83aCMV[\x81Q\x81\x10a2\xE2Wa2\xE2aB\xC7V[` \x02` \x01\x01Q\x86\x83`\x02a2\xF8\x91\x90aCMV[a3\x03\x90`\x01aC\x1AV[\x81Q\x81\x10a3\x13Wa3\x13aB\xC7V[` \x02` \x01\x01Q`@Q` \x01a35\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra3O\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a3lW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x8F\x91\x90aC\xCFV[\x82\x82\x81Q\x81\x10a3\xA1Wa3\xA1aB\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a3\xB6\x81aC2V[\x91PPa2\xBDV[Pa3\xCA`\x02\x83aE\x1BV[\x91P[\x81\x15a4\xEAW`\0[\x82\x81\x10\x15a4\xD7W`\x02\x82a3\xEB\x83\x83aCMV[\x81Q\x81\x10a3\xFBWa3\xFBaB\xC7V[` \x02` \x01\x01Q\x83\x83`\x02a4\x11\x91\x90aCMV[a4\x1C\x90`\x01aC\x1AV[\x81Q\x81\x10a4,Wa4,aB\xC7V[` \x02` \x01\x01Q`@Q` \x01a4N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4h\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a4\x85W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xA8\x91\x90aC\xCFV[\x82\x82\x81Q\x81\x10a4\xBAWa4\xBAaB\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a4\xCF\x81aC2V[\x91PPa3\xD6V[Pa4\xE3`\x02\x83aE\x1BV[\x91Pa3\xCDV[\x80`\0\x81Q\x81\x10a4\xFDWa4\xFDaB\xC7V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0a\x0E5\x82`\x05\x81Q\x81\x10a5&Wa5&aB\xC7V[` \x02` \x01\x01Qa9%V[`\0a\x0E5\x82`\x06\x81Q\x81\x10a5&Wa5&aB\xC7V[`\0\x81`\x01\x81Q\x81\x10a\x1E\xD3Wa\x1E\xD3aB\xC7V[`\0a\x0E5\x82`\x02\x81Q\x81\x10a5&Wa5&aB\xC7V[`\0a5\xCD\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a9\x8C\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x16iW\x80\x80` \x01\x90Q\x81\x01\x90a5\xEB\x91\x90aAJV[a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\0a6X`&`\x01aC\x1AV[a6c\x90` aCMV[a6p`@\x84\x01\x84aB\rV[\x90P\x14a6\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aH]\x839\x81Q\x91R\x90\x82\x01R\x7FrBalance: Proof has incorrect le`d\x82\x01Rc\r\xCC\xEE\x8D`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a6\xEE`\x04\x85aGdV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa7Ha7\x07`@\x85\x01\x85aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a2IV[a7\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrBalance: Invalid merkle proof\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[a7\xB6\x83` \x015\x85a9\x9BV[\x91PP[\x93\x92PPPV[`\0a7\xBA`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16aG\x88V[`\0\x83Q`\0\x14\x15\x80\x15a7\xF8WP` \x84Qa7\xF6\x91\x90aE\x07V[\x15[a8\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FMerkle.processInclusionProofSha2`D\x82\x01R\x7F56: proof length should be a non`d\x82\x01Rs\x16\xBD2\xB97\x906\xBA\xB6:4\xB862\x907\xB3\x10\x19\x99`a\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a9\x1BWa8\xAB`\x02\x85aE\x07V[a8\xDEW\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa8\xD3W`\0\x80\xFD[`\x02\x84\x04\x93Pa9\tV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa9\x02W`\0\x80\xFD[`\x02\x84\x04\x93P[a9\x14` \x82aC\x1AV[\x90Pa8\x98V[PQ\x94\x93PPPPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\x0F\xE7\x84\x84`\0\x85a9\xC8V[`\0\x80a9\xA9`\x04\x84aG\xD8V[a9\xB4\x90`@aG\xFCV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x0F\xE7\x84\x82\x1Ba9%V[``\x82G\x10\x15a:)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a:\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06\xCAV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa:\x9C\x91\x90aC\xB3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a:\xD9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a:\xDEV[``\x91P[P\x91P\x91Pa:\xEE\x82\x82\x86a:\xF9V[\x97\x96PPPPPPPV[``\x83\x15a;\x08WP\x81a7\xBAV[\x82Q\x15a;\x18W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x91\x90aH)V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a;IW`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a;`W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a;{W`\0\x80\xFD[a;\x84\x84a;2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xA0W`\0\x80\xFD[a;\xAC\x87\x83\x88\x01a;NV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a;\xC2W`\0\x80\xFD[Pa;\xCF\x86\x82\x87\x01a;NV[\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a;\xEBW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x02W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a<\x1DW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a<@W`\0\x80\xFD[a<I\x89a;2V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<eW`\0\x80\xFD[a<q\x8C\x83\x8D\x01a;NV[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15a<\x87W`\0\x80\xFD[a<\x93\x8C\x83\x8D\x01a;\xD9V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a<\xACW`\0\x80\xFD[a<\xB8\x8C\x83\x8D\x01a;\xD9V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a<\xD1W`\0\x80\xFD[Pa<\xDE\x8B\x82\x8C\x01a;\xD9V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15a=\x04W`\0\x80\xFD[a7\xBA\x82a;2V[`\0\x80\x83`\x1F\x84\x01\x12a=\x1FW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a=6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a<\x1DW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a=aW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a=wW`\0\x80\xFD[a=\x83\x85\x82\x86\x01a=\rV[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a=\xC3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x0E5\x82\x84a=\xA5V[`\0` \x82\x84\x03\x12\x15a=\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Qa>/``\x84\x01\x82a=\xA5V[P\x92\x91PPV[\x80\x15\x15\x81\x14a2FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>VW`\0\x80\xFD[\x815a7\xBA\x81a>6V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a>yW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\x90W`\0\x80\xFD[a>\x9C\x89\x83\x8A\x01a=\rV[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a>\xB5W`\0\x80\xFD[Pa>\xC2\x88\x82\x89\x01a=\rV[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2FW`\0\x80\xFD[\x805a;I\x81a>\xD4V[`\0\x80`@\x83\x85\x03\x12\x15a?\x07W`\0\x80\xFD[\x825a?\x12\x81a>\xD4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a?2W`\0\x80\xFD[\x815a7\xBA\x81a>\xD4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?{Wa?{a?=V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a?\x9CWa?\x9Ca?=V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a?\xB7W`\0\x80\xFD[\x815` a?\xCCa?\xC7\x83a?\x83V[a?SV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a?\xEBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x06W\x805\x83R\x91\x83\x01\x91\x83\x01a?\xEFV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a@&W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@=W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a@QW`\0\x80\xFD[\x815` a@aa?\xC7\x83a?\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a@\x80W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a@\xA7W\x855a@\x98\x81a>\xD4V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a@\x85V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a@\xBDW`\0\x80\xFD[Pa@\xCA\x86\x82\x87\x01a?\xA6V[\x92PPa@\xD9`@\x85\x01a>\xE9V[\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a@\xF7W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\x0EW`\0\x80\xFD[aA\x1A\x87\x83\x88\x01a;NV[\x94P` \x86\x015\x91P\x80\x82\x11\x15aA0W`\0\x80\xFD[PaA=\x86\x82\x87\x01a;\xD9V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aA\\W`\0\x80\xFD[\x81Qa7\xBA\x81a>6V[` \x80\x82R`>\x90\x82\x01R\x7FEigenPod.onlyWhenNotPaused: inde`@\x82\x01R\x7Fx is paused in EigenPodManager\0\0``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\xF5W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a<\x1DW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB$W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aB>W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a<\x1DW`\0\x80\xFD[` \x80\x82R`N\x90\x82\x01R\x7FEigenPod.onlyOwnerOrProofSubmitt`@\x82\x01R\x7Fer: caller is not pod owner or p``\x82\x01Rm97\xB7\xB3\x109\xBA\xB16\xB4\xBA:2\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aB\xEFW`\0\x80\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a7\xBAW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aC-WaC-aC\x04V[P\x01\x90V[`\0`\0\x19\x82\x14\x15aCFWaCFaC\x04V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aCgWaCgaC\x04V[P\x02\x90V[`\0\x82\x82\x10\x15aC~WaC~aC\x04V[P\x03\x90V[`\0[\x83\x81\x10\x15aC\x9EW\x81\x81\x01Q\x83\x82\x01R` \x01aC\x86V[\x83\x81\x11\x15aC\xADW`\0\x84\x84\x01R[PPPPV[`\0\x82QaC\xC5\x81\x84` \x87\x01aC\x83V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aC\xE1W`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`1\x90\x82\x01R\x7FEigenPod.onlyEigenPodManager: no`@\x82\x01Rp:\x102\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`y\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84RaDz\x81` \x86\x01` \x86\x01aC\x83V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0aD\xA2`\x80\x83\x01\x88\x8AaD9V[\x82\x81\x03` \x84\x01RaD\xB4\x81\x88aDbV[\x90P\x82\x81\x03`@\x84\x01RaD\xC9\x81\x86\x88aD9V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0a\x0F\xE7` \x83\x01\x84\x86aD9V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aE\x16WaE\x16aD\xF1V[P\x06\x90V[`\0\x82aE*WaE*aD\xF1V[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aEOWaEOaC\x04V[\x03\x93\x92PPPV[` \x80\x82R`(\x90\x82\x01R\x7FEigenPod.onlyEigenPodOwner: not `@\x82\x01Rg87\xB2'\xBB\xB72\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aC\xC5W`\0\x80\xFD[`\0b\xFF\xFF\xFF\x82\x16\x80aE\xCAWaE\xCAaC\x04V[`\0\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aE\xFEWaE\xFEaC\x04V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aF\x1AWaF\x1AaC\x04V[P\x01\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aFEWaFEaC\x04V[\x01\x94\x93PPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a;`W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0\x83QaF\x84\x81\x84` \x88\x01aC\x83V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81aF\xB0WaF\xB0aC\x04V[P`\0\x19\x01\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14\x15aF\xD6WaF\xD6aC\x04V[`\0\x03\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x03\x81\x84\x13\x82\x84\x13\x80\x82\x16\x86\x84\x04\x86\x11\x16\x15aG\x05WaG\x05aC\x04V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15aG$WaG$aC\x04V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aG@WaG@aC\x04V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aGVWaGVaC\x04V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80aG|WaG|aD\xF1V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aG\xB3WaG\xB3aC\x04V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aG\xCEWaG\xCEaC\x04V[P\x90\x03\x93\x92PPPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80aG\xF0WaG\xF0aD\xF1V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aH WaH aC\x04V[\x02\x94\x93PPPPV[` \x81R`\0a7\xBA` \x83\x01\x84aDbV\xFEEigenPod._verifyWithdrawalCredenBeaconChainProofs.verifyValidato\xA2dipfsX\"\x12 \xF9\xF5\xD6\x06\x82\xFD\xF4\xBD\x86O\xC85P\x88&\xF6\xAC\xC0\x89\x06\xF8\xF5B\xCF!\xA1\x99X\xBF\\\x85PdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061016a5760003560e01c80636fcd0e53116100d1578063c49074421161008a578063dda3346c11610064578063dda3346c1461058d578063ee94d67c146105ad578063f074ba62146105cd578063f2882461146105ed57600080fd5b8063c49074421461052d578063c4d66de81461054d578063d06d55871461056d57600080fd5b80636fcd0e53146104425780637439841f1461046f57806374cdd798146104a657806388676cad146104da5780639b4e4634146104fa578063b522538a1461050d57600080fd5b80634665bcda116101235780634665bcda146102ab57806347d28372146102df57806352396a591461039f57806358753357146103d557806358eaee79146103f55780636c0d2d5a1461042257600080fd5b8063039157d2146101a95780630b18ff66146101cb5780632340e8d3146102085780633474aa161461022c5780633f65cf191461026457806342ecff2a1461028457600080fd5b366101a4576040513481527f6fdd3dbdb173299608c0aa9f368735857c8842b581f8389238bf05bd04b3bf499060200160405180910390a1005b600080fd5b3480156101b557600080fd5b506101c96101c4366004613b66565b610621565b005b3480156101d757600080fd5b506033546101eb906001600160a01b031681565b6040516001600160a01b0390911681526020015b60405180910390f35b34801561021457600080fd5b5061021e60395481565b6040519081526020016101ff565b34801561023857600080fd5b5060345461024c906001600160401b031681565b6040516001600160401b0390911681526020016101ff565b34801561027057600080fd5b506101c961027f366004613c24565b610a67565b34801561029057600080fd5b50603a5461024c90600160401b90046001600160401b031681565b3480156102b757600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156102eb57600080fd5b5061035b6040805160808101825260008082526020820181905291810182905260608101919091525060408051608081018252603c548152603d5462ffffff811660208301526001600160401b03630100000082041692820192909252600160581b909104600f0b606082015290565b6040516101ff91908151815260208083015162ffffff16908201526040808301516001600160401b031690820152606091820151600f0b9181019190915260800190565b3480156103ab57600080fd5b5061024c6103ba366004613cf2565b603b602052600090815260409020546001600160401b031681565b3480156103e157600080fd5b50603e546101eb906001600160a01b031681565b34801561040157600080fd5b50610415610410366004613d4e565b610dd6565b6040516101ff9190613dc7565b34801561042e57600080fd5b5061021e61043d366004613cf2565b610e3b565b34801561044e57600080fd5b5061046261045d366004613dd5565b610fef565b6040516101ff9190613dee565b34801561047b57600080fd5b5061041561048a366004613dd5565b600090815260366020526040902054600160c01b900460ff1690565b3480156104b257600080fd5b506101eb7f000000000000000000000000000000000000000000000000000000000000000081565b3480156104e657600080fd5b506101c96104f5366004613e44565b61109c565b6101c9610508366004613e61565b611191565b34801561051957600080fd5b50610462610528366004613d4e565b61133e565b34801561053957600080fd5b506101c9610548366004613ef4565b611431565b34801561055957600080fd5b506101c9610568366004613f20565b61166e565b34801561057957600080fd5b506101c9610588366004613f20565b611805565b34801561059957600080fd5b506101c96105a8366004614011565b611898565b3480156105b957600080fd5b50603a5461024c906001600160401b031681565b3480156105d957600080fd5b506101c96105e83660046140e2565b611a6b565b3480156105f957600080fd5b5061024c7f000000000000000000000000000000000000000000000000000000000000000081565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610689573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106ad919061414a565b156106d35760405162461bcd60e51b81526004016106ca90614167565b60405180910390fd5b604051635ac86ab760e01b8152600860048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561073b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061075f919061414a565b1561077c5760405162461bcd60e51b81526004016106ca90614167565b60006107c261078b85806141c4565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ebe92505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff16600281111561083157610831613d8f565b600281111561084257610842613d8f565b81525050905080604001516001600160401b0316876001600160401b0316116108d5576040805162461bcd60e51b81526020600482015260248101919091527f456967656e506f642e7665726966795374616c6542616c616e63653a2070726f60448201527f6f66206973206f6c646572207468616e206c61737420636865636b706f696e7460648201526084016106ca565b6001816060015160028111156108ed576108ed613d8f565b146109575760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e7665726966795374616c6542616c616e63653a2076616c604482015273696461746f72206973206e6f742061637469766560601b60648201526084016106ca565b61099b61096486806141c4565b80806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ee292505050565b610a1f5760405162461bcd60e51b815260206004820152604960248201527f456967656e506f642e7665726966795374616c6542616c616e63653a2076616c60448201527f696461746f72206d75737420626520736c617368656420746f206265206d61726064820152686b6564207374616c6560b81b608482015260a4016106ca565b610a31610a2b88610e3b565b87611f0c565b610a548635610a4087806141c4565b610a4d60208a018a61420d565b8651612067565b610a5e600061227e565b50505050505050565b6033546001600160a01b0316331480610a8a5750603e546001600160a01b031633145b610aa65760405162461bcd60e51b81526004016106ca90614253565b604051635ac86ab760e01b8152600260048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015610b0e573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610b32919061414a565b15610b4f5760405162461bcd60e51b81526004016106ca90614167565b8584148015610b5d57508382145b610bed5760405162461bcd60e51b815260206004820152605560248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a2076616c696461746f72496e646963657320616e642070726f6f666064820152740e640daeae6e840c4ca40e6c2daca40d8cadccee8d605b1b608482015260a4016106ca565b603a546001600160401b03600160401b9091048116908a1611610c8d5760405162461bcd60e51b815260206004820152604c60248201527f456967656e506f642e7665726966795769746864726177616c43726564656e7460448201527f69616c733a207370656369666965642074696d657374616d7020697320746f6f60648201526b0819985c881a5b881c185cdd60a21b608482015260a4016106ca565b610c9f610c998a610e3b565b89611f0c565b6000805b87811015610d4257610d248a358a8a84818110610cc257610cc26142c7565b9050602002016020810190610cd791906142dd565b898985818110610ce957610ce96142c7565b9050602002810190610cfb919061420d565b898987818110610d0d57610d0d6142c7565b9050602002810190610d1f91906141c4565b612514565b610d2e908361431a565b915080610d3a81614332565b915050610ca3565b5060335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c4090604401600060405180830381600087803b158015610db257600080fd5b505af1158015610dc6573d6000803e3d6000fd5b5050505050505050505050505050565b600080610e1884848080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250612afa92505050565b600090815260366020526040902054600160c01b900460ff169150505b92915050565b6000610e4a611fff600c61434d565b610e5d6001600160401b0384164261436c565b10610ec65760405162461bcd60e51b815260206004820152603360248201527f456967656e506f642e676574506172656e74426c6f636b526f6f743a2074696d604482015272657374616d70206f7574206f662072616e676560681b60648201526084016106ca565b604080516001600160401b03841660208201526000918291720f3df6d732807ef1319fb7b8bb8522d0beac02910160408051601f1981840301815290829052610f0e916143b3565b600060405180830381855afa9150503d8060008114610f49576040519150601f19603f3d011682016040523d82523d6000602084013e610f4e565b606091505b5091509150818015610f61575060008151115b610fd35760405162461bcd60e51b815260206004820152603860248201527f456967656e506f642e676574506172656e74426c6f636b526f6f743a20696e7660448201527f616c696420626c6f636b20726f6f742072657475726e6564000000000000000060648201526084016106ca565b80806020019051810190610fe791906143cf565b949350505050565b6110176040805160808101825260008082526020820181905291810182905290606082015290565b600082815260366020908152604091829020825160808101845281546001600160401b038082168352600160401b8204811694830194909452600160801b810490931693810193909352906060830190600160c01b900460ff16600281111561108257611082613d8f565b600281111561109357611093613d8f565b90525092915050565b6033546001600160a01b03163314806110bf5750603e546001600160a01b031633145b6110db5760405162461bcd60e51b81526004016106ca90614253565b604051635ac86ab760e01b8152600660048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611143573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611167919061414a565b156111845760405162461bcd60e51b81526004016106ca90614167565b61118d8261227e565b5050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111d95760405162461bcd60e51b81526004016106ca906143e8565b346801bc16d674ec800000146112655760405162461bcd60e51b8152602060048201526044602482018190527f456967656e506f642e7374616b653a206d75737420696e697469616c6c792073908201527f74616b6520666f7220616e792076616c696461746f72207769746820333220656064820152633a3432b960e11b608482015260a4016106ca565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663228951186801bc16d674ec80000087876112a8612bf4565b8888886040518863ffffffff1660e01b81526004016112cc9695949392919061448e565b6000604051808303818588803b1580156112e557600080fd5b505af11580156112f9573d6000803e3d6000fd5b50505050507f606865b7934a25d4aed43f6cdb426403353fa4b3009c4d228407474581b01e23858560405161132f9291906144dd565b60405180910390a15050505050565b6113666040805160808101825260008082526020820181905291810182905290606082015290565b603660006113a985858080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250612afa92505050565b81526020808201929092526040908101600020815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b81049094169281019290925290916060830190600160c01b900460ff16600281111561141657611416613d8f565b600281111561142757611427613d8f565b9052509392505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146114795760405162461bcd60e51b81526004016106ca906143e8565b611487633b9aca0082614507565b156115115760405162461bcd60e51b815260206004820152604e60248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74576569206d75737420626520612077686f60648201526d1b194811ddd95a48185b5bdd5b9d60921b608482015260a4016106ca565b6000611521633b9aca008361451b565b6034549091506001600160401b0390811690821611156115da5760405162461bcd60e51b815260206004820152606260248201527f456967656e506f642e776974686472617752657374616b6564426561636f6e4360448201527f6861696e4554483a20616d6f756e74477765692065786365656473207769746860648201527f6472617761626c6552657374616b6564457865637574696f6e4c617965724777608482015261656960f01b60a482015260c4016106ca565b603480548291906000906115f89084906001600160401b031661452f565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550826001600160a01b03167f8947fd2ce07ef9cc302c4e8f0461015615d91ce851564839e91cc804c2f49d8e8360405161165791815260200190565b60405180910390a26116698383612c39565b505050565b600054610100900460ff161580801561168e5750600054600160ff909116105b806116a85750303b1580156116a8575060005460ff166001145b61170b5760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016106ca565b6000805460ff19166001179055801561172e576000805461ff0019166101001790555b6001600160a01b0382166117a15760405162461bcd60e51b815260206004820152603460248201527f456967656e506f642e696e697469616c697a653a20706f644f776e65722063616044820152736e6e6f74206265207a65726f206164647265737360601b60648201526084016106ca565b603380546001600160a01b0319166001600160a01b038416179055801561118d576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050565b6033546001600160a01b0316331461182f5760405162461bcd60e51b81526004016106ca90614557565b603e54604080516001600160a01b03928316815291831660208301527ffb8129080a19d34dceac04ba253fc50304dc86c729bd63cdca4a969ad19a5eac910160405180910390a1603e80546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b031633146118c25760405162461bcd60e51b81526004016106ca90614557565b604051635ac86ab760e01b8152600560048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa15801561192a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061194e919061414a565b1561196b5760405162461bcd60e51b81526004016106ca90614167565b82518451146119f65760405162461bcd60e51b815260206004820152604b60248201527f456967656e506f642e7265636f766572546f6b656e733a20746f6b656e4c697360448201527f7420616e6420616d6f756e7473546f5769746864726177206d7573742062652060648201526a0e6c2daca40d8cadccee8d60ab1b608482015260a4016106ca565b60005b8451811015611a6457611a5283858381518110611a1857611a186142c7565b6020026020010151878481518110611a3257611a326142c7565b60200260200101516001600160a01b0316612d529092919063ffffffff16565b80611a5c81614332565b9150506119f9565b5050505050565b604051635ac86ab760e01b8152600760048201819052907f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031690635ac86ab790602401602060405180830381865afa158015611ad3573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611af7919061414a565b15611b145760405162461bcd60e51b81526004016106ca90614167565b603a54600160401b90046001600160401b031680611bc05760405162461bcd60e51b815260206004820152605860248201527f456967656e506f642e766572696679436865636b706f696e7450726f6f66733a60448201527f206d75737420686176652061637469766520636865636b706f696e7420746f2060648201527f706572666f726d20636865636b706f696e742070726f6f660000000000000000608482015260a4016106ca565b60408051608081018252603c54808252603d5462ffffff811660208401526001600160401b03630100000082041693830193909352600160581b909204600f0b606082015290611c109087612da4565b6000805b85811015611e645736878783818110611c2f57611c2f6142c7565b9050602002810190611c41919061459f565b80356000908152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff166002811115611cb257611cb2613d8f565b6002811115611cc357611cc3613d8f565b9052509050600181606001516002811115611ce057611ce0613d8f565b14611cec575050611e52565b856001600160401b031681604001516001600160401b031610611d10575050611e52565b600080611d2083898e3587612f20565b602089018051929450909250611d35826145b5565b62ffffff16905250606087018051839190611d519083906145d4565b600f0b905250611d618187614623565b84356000908152603660209081526040918290208651815492880151938801516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b0319909516919092161792909217928316821781556060870151939950869390929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115611e0657611e06613d8f565b021790555050835160405164ffffffffff90911691506001600160401b038a16907fa91c59033c3423e18b54d0acecebb4972f9ea95aedf5f4cae3b677b02eaf3a3f90600090a3505050505b80611e5c81614332565b915050611c14565b506001600160401b038084166000908152603b6020526040812080548493919291611e9191859116614623565b92506101000a8154816001600160401b0302191690836001600160401b03160217905550610a5e82613042565b600081600081518110611ed357611ed36142c7565b60200260200101519050919050565b600081600381518110611ef757611ef76142c7565b60200260200101516000801b14159050919050565b611f186003602061434d565b611f25602083018361420d565b905014611f9a5760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f743a2050726f6f662068617320696e636f7272656374206c656e67746800000060648201526084016106ca565b611fea611faa602083018361420d565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525086925050843590506003613249565b61118d5760405162461bcd60e51b815260206004820152604260248201527f426561636f6e436861696e50726f6f66732e7665726966795374617465526f6f60448201527f743a20496e76616c696420737461746520726f6f74206d65726b6c652070726f60648201526137b360f11b608482015260a4016106ca565b600884146120e25760405162461bcd60e51b815260206004820152604e602482015260008051602061485d83398151915260448201527f724669656c64733a2056616c696461746f72206669656c64732068617320696e60648201526d0c6dee4e4cac6e840d8cadccee8d60931b608482015260a4016106ca565b60056120f06028600161431a565b6120fa919061431a565b61210590602061434d565b82146121735760405162461bcd60e51b8152602060048201526043602482015260008051602061485d83398151915260448201527f724669656c64733a2050726f6f662068617320696e636f7272656374206c656e6064820152620cee8d60eb1b608482015260a4016106ca565b60006121b186868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061326192505050565b9050600064ffffffffff83166121c96028600161431a565b600b901b17905061221485858080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508c9250869150859050613249565b6122745760405162461bcd60e51b815260206004820152603d602482015260008051602061485d83398151915260448201527f724669656c64733a20496e76616c6964206d65726b6c652070726f6f6600000060648201526084016106ca565b5050505050505050565b603a54600160401b90046001600160401b03161561231f5760405162461bcd60e51b815260206004820152605260248201527f456967656e506f642e5f7374617274436865636b706f696e743a206d7573742060448201527f66696e6973682070726576696f757320636865636b706f696e74206265666f72606482015271329039ba30b93a34b7339030b737ba3432b960711b608482015260a4016106ca565b603a54426001600160401b03908116911614156123a45760405162461bcd60e51b815260206004820152603f60248201527f456967656e506f642e5f7374617274436865636b706f696e743a2063616e6e6f60448201527f7420636865636b706f696e7420747769636520696e206f6e6520626c6f636b0060648201526084016106ca565b6034546000906001600160401b03166123c1633b9aca004761451b565b6123cb919061452f565b90508180156123e157506001600160401b038116155b156124545760405162461bcd60e51b815260206004820152603d60248201527f456967656e506f642e5f7374617274436865636b706f696e743a206e6f20626160448201527f6c616e636520617661696c61626c6520746f20636865636b706f696e7400000060648201526084016106ca565b6000604051806080016040528061246a42610e3b565b815260200160395462ffffff168152602001836001600160401b031681526020016000600f0b815250905042603a60086101000a8154816001600160401b0302191690836001600160401b031602179055506124c581613042565b805160208083015160405162ffffff90911681526001600160401b034216917f575796133bbed337e5b39aa49a30dc2556a91e0c6c2af4b7b886ae77ebef1076910160405180910390a3505050565b600080612553848480806020026020016040519081016040528093929190818152602001838360200280828437600092019190915250611ebe92505050565b6000818152603660209081526040808320815160808101835281546001600160401b038082168352600160401b8204811695830195909552600160801b8104909416928101929092529394509192906060830190600160c01b900460ff1660028111156125c2576125c2613d8f565b60028111156125d3576125d3613d8f565b90525090506000816060015160028111156125f0576125f0613d8f565b146126815760405162461bcd60e51b8152602060048201526061602482015260008051602061483d83398151915260448201527f7469616c733a2076616c696461746f72206d75737420626520696e616374697660648201527f6520746f2070726f7665207769746864726177616c2063726564656e7469616c6084820152607360f81b60a482015260c4016106ca565b6001600160401b0380166126c786868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061350e92505050565b6001600160401b031614156127505760405162461bcd60e51b8152602060048201526055602482015260008051602061483d83398151915260448201527f7469616c733a2076616c696461746f72206d75737420626520696e207468652060648201527470726f63657373206f662061637469766174696e6760581b608482015260a4016106ca565b6001600160401b03801661279686868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061353392505050565b6001600160401b03161461280e5760405162461bcd60e51b81526020600482015260446024820181905260008051602061483d833981519152908201527f7469616c733a2076616c696461746f72206d757374206e6f742062652065786960648201526374696e6760e01b608482015260a4016106ca565b612816612bf4565b61281f9061464e565b61285b86868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061354b92505050565b146128ca5760405162461bcd60e51b8152602060048201526045602482015260008051602061483d83398151915260448201527f7469616c733a2070726f6f66206973206e6f7420666f72207468697320456967606482015264195b941bd960da1b608482015260a4016106ca565b600061290886868080602002602001604051908101604052809392919081815260200183836020028082843760009201919091525061356092505050565b90506129188a87878b8b8e612067565b6039805490600061292883614332565b9091555050603a54600090600160401b90046001600160401b03161561296057603a54600160401b90046001600160401b031661296d565b603a546001600160401b03165b6040805160808101825264ffffffffff8d1681526001600160401b03858116602083015283169181019190915290915060608101600190526000858152603660209081526040918290208351815492850151938501516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990951691909216179290921792831682178155606084015190929091839160ff60c01b1990911668ffffffffffffffffff60801b1990911617600160c01b836002811115612a4357612a43613d8f565b02179055505060405164ffffffffff8c1681527f2d0800bbc377ea54a08c5db6a87aafff5e3e9c8fead0eda110e40e0c10441449915060200160405180910390a16040805164ffffffffff8c1681526001600160401b03838116602083015284168183015290517f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df9181900360600190a1612aeb633b9aca006001600160401b03841661434d565b9b9a5050505050505050505050565b60008151603014612b835760405162461bcd60e51b815260206004820152604760248201527f456967656e506f642e5f63616c63756c61746556616c696461746f725075626b60448201527f657948617368206d75737420626520612034382d6279746520424c53207075626064820152666c6963206b657960c81b608482015260a4016106ca565b604051600290612b9a908490600090602001614672565b60408051601f1981840301815290829052612bb4916143b3565b602060405180830381855afa158015612bd1573d6000803e3d6000fd5b5050506040513d601f19601f82011682018060405250810190610e3591906143cf565b60408051600160f81b60208201526000602182015230606090811b6bffffffffffffffffffffffff1916602c8301529101604051602081830303815290604052905090565b80471015612c895760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e636500000060448201526064016106ca565b6000826001600160a01b03168260405160006040518083038185875af1925050503d8060008114612cd6576040519150601f19603f3d011682016040523d82523d6000602084013e612cdb565b606091505b50509050806116695760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d6179206861766520726576657274656400000000000060648201526084016106ca565b604080516001600160a01b038416602482015260448082018490528251808303909101815260649091019091526020810180516001600160e01b031663a9059cbb60e01b179052611669908490613578565b612db06005600361431a565b612dbb90602061434d565b612dc8602083018361420d565b905014612e4b5760405162461bcd60e51b8152602060048201526044602482018190527f426561636f6e436861696e50726f6f66732e76657269667942616c616e636543908201527f6f6e7461696e65723a2050726f6f662068617320696e636f7272656374206c656064820152630dccee8d60e31b608482015260a4016106ca565b606c612e9c612e5d602084018461420d565b8080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250879250508535905084613249565b6116695760405162461bcd60e51b815260206004820152604960248201527f426561636f6e436861696e50726f6f66732e76657269667942616c616e63654360448201527f6f6e7461696e65723a20696e76616c69642062616c616e636520636f6e7461696064820152683732b910383937b7b360b91b608482015260a4016106ca565b83516020850151600091829182612f3887848861364a565b9050816001600160401b0316816001600160401b031614612fb257612f5d81836137c1565b6040805164ffffffffff861681526001600160401b038b8116602083015284168183015290519196507f0e5fac175b83177cc047381e030d8fb3b42b37bd1c025e22c280facad62c32df919081900360600190a15b6001600160401b0380821660208b0181905290891660408b01526130365760398054906000612fe0836146a1565b9091555050600260608a0152612ff5856146b8565b93508264ffffffffff16886001600160401b03167f2a02361ffa66cf2c2da4682c2355a6adcaa9f6c227b6e6563e68480f9587626a60405160405180910390a35b50505094509492505050565b602081015162ffffff166131c9576000633b9aca00826060015183604001516001600160401b031661307491906145d4565b600f0b61308191906146df565b60408301516034805492935090916000906130a69084906001600160401b0316614623565b82546101009290920a6001600160401b03818102199093169183160217909155603a8054600160401b81049092166001600160801b0319909216919091179055506000603c55603d80546001600160d81b031916905560335460405163030b147160e61b81526001600160a01b039182166004820152602481018390527f00000000000000000000000000000000000000000000000000000000000000009091169063c2c51c4090604401600060405180830381600087803b15801561316b57600080fd5b505af115801561317f573d6000803e3d6000fd5b5050603a546040518481526001600160401b0390911692507f525408c201bc1576eb44116f6478f1c2a54775b19a043bcfdc708364f74f8e44915060200160405180910390a25050565b8051603c556020810151603d8054604084015160608501516fffffffffffffffffffffffffffffffff16600160581b026fffffffffffffffffffffffffffffffff60581b196001600160401b039092166301000000026affffffffffffffffffffff1990931662ffffff9095169490941791909117169190911790555b50565b6000836132578685856137d9565b1495945050505050565b60008060028351613272919061451b565b90506000816001600160401b0381111561328e5761328e613f3d565b6040519080825280602002602001820160405280156132b7578160200160208202803683370190505b50905060005b828110156133be576002856132d2838361434d565b815181106132e2576132e26142c7565b6020026020010151868360026132f8919061434d565b61330390600161431a565b81518110613313576133136142c7565b6020026020010151604051602001613335929190918252602082015260400190565b60408051601f198184030181529082905261334f916143b3565b602060405180830381855afa15801561336c573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061338f91906143cf565b8282815181106133a1576133a16142c7565b6020908102919091010152806133b681614332565b9150506132bd565b506133ca60028361451b565b91505b81156134ea5760005b828110156134d7576002826133eb838361434d565b815181106133fb576133fb6142c7565b602002602001015183836002613411919061434d565b61341c90600161431a565b8151811061342c5761342c6142c7565b602002602001015160405160200161344e929190918252602082015260400190565b60408051601f1981840301815290829052613468916143b3565b602060405180830381855afa158015613485573d6000803e3d6000fd5b5050506040513d601f19601f820116820180604052508101906134a891906143cf565b8282815181106134ba576134ba6142c7565b6020908102919091010152806134cf81614332565b9150506133d6565b506134e360028361451b565b91506133cd565b806000815181106134fd576134fd6142c7565b602002602001015192505050919050565b6000610e3582600581518110613526576135266142c7565b6020026020010151613925565b6000610e3582600681518110613526576135266142c7565b600081600181518110611ed357611ed36142c7565b6000610e3582600281518110613526576135266142c7565b60006135cd826040518060400160405280602081526020017f5361666545524332303a206c6f772d6c6576656c2063616c6c206661696c6564815250856001600160a01b031661398c9092919063ffffffff16565b80519091501561166957808060200190518101906135eb919061414a565b6116695760405162461bcd60e51b815260206004820152602a60248201527f5361666545524332303a204552433230206f7065726174696f6e20646964206e6044820152691bdd081cdd58d8d9595960b21b60648201526084016106ca565b60006136586026600161431a565b61366390602061434d565b613670604084018461420d565b9050146136e15760405162461bcd60e51b81526020600482015260446024820181905260008051602061485d833981519152908201527f7242616c616e63653a2050726f6f662068617320696e636f7272656374206c656064820152630dccee8d60e31b608482015260a4016106ca565b60006136ee600485614764565b64ffffffffff169050613748613707604085018561420d565b8080601f0160208091040260200160405190810160405280939291908181526020018383808284376000920191909152508992505050602086013584613249565b6137a85760405162461bcd60e51b815260206004820152603e602482015260008051602061485d83398151915260448201527f7242616c616e63653a20496e76616c6964206d65726b6c652070726f6f66000060648201526084016106ca565b6137b683602001358561399b565b9150505b9392505050565b60006137ba6001600160401b03808416908516614788565b600083516000141580156137f85750602084516137f69190614507565b155b6138875760405162461bcd60e51b815260206004820152605460248201527f4d65726b6c652e70726f63657373496e636c7573696f6e50726f6f665368613260448201527f35363a2070726f6f66206c656e6774682073686f756c642062652061206e6f6e60648201527316bd32b9379036bab63a34b836329037b310199960611b608482015260a4016106ca565b604080516020808201909252848152905b8551811161391b576138ab600285614507565b6138de578151600052808601516020526020826040600060026107d05a03fa6138d357600080fd5b600284049350613909565b8086015160005281516020526020826040600060026107d05a03fa61390257600080fd5b6002840493505b61391460208261431a565b9050613898565b5051949350505050565b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161767ff0000000000000060889290921c919091161790565b6060610fe784846000856139c8565b6000806139a96004846147d8565b6139b49060406147fc565b64ffffffffff169050610fe784821b613925565b606082471015613a295760405162461bcd60e51b815260206004820152602660248201527f416464726573733a20696e73756666696369656e742062616c616e636520666f6044820152651c8818d85b1b60d21b60648201526084016106ca565b6001600160a01b0385163b613a805760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a2063616c6c20746f206e6f6e2d636f6e747261637400000060448201526064016106ca565b600080866001600160a01b03168587604051613a9c91906143b3565b60006040518083038185875af1925050503d8060008114613ad9576040519150601f19603f3d011682016040523d82523d6000602084013e613ade565b606091505b5091509150613aee828286613af9565b979650505050505050565b60608315613b085750816137ba565b825115613b185782518084602001fd5b8160405162461bcd60e51b81526004016106ca9190614829565b80356001600160401b0381168114613b4957600080fd5b919050565b600060408284031215613b6057600080fd5b50919050565b600080600060608486031215613b7b57600080fd5b613b8484613b32565b925060208401356001600160401b0380821115613ba057600080fd5b613bac87838801613b4e565b93506040860135915080821115613bc257600080fd5b50613bcf86828701613b4e565b9150509250925092565b60008083601f840112613beb57600080fd5b5081356001600160401b03811115613c0257600080fd5b6020830191508360208260051b8501011115613c1d57600080fd5b9250929050565b60008060008060008060008060a0898b031215613c4057600080fd5b613c4989613b32565b975060208901356001600160401b0380821115613c6557600080fd5b613c718c838d01613b4e565b985060408b0135915080821115613c8757600080fd5b613c938c838d01613bd9565b909850965060608b0135915080821115613cac57600080fd5b613cb88c838d01613bd9565b909650945060808b0135915080821115613cd157600080fd5b50613cde8b828c01613bd9565b999c989b5096995094979396929594505050565b600060208284031215613d0457600080fd5b6137ba82613b32565b60008083601f840112613d1f57600080fd5b5081356001600160401b03811115613d3657600080fd5b602083019150836020828501011115613c1d57600080fd5b60008060208385031215613d6157600080fd5b82356001600160401b03811115613d7757600080fd5b613d8385828601613d0d565b90969095509350505050565b634e487b7160e01b600052602160045260246000fd5b60038110613dc357634e487b7160e01b600052602160045260246000fd5b9052565b60208101610e358284613da5565b600060208284031215613de757600080fd5b5035919050565b60006080820190506001600160401b03808451168352806020850151166020840152806040850151166040840152506060830151613e2f6060840182613da5565b5092915050565b801515811461324657600080fd5b600060208284031215613e5657600080fd5b81356137ba81613e36565b600080600080600060608688031215613e7957600080fd5b85356001600160401b0380821115613e9057600080fd5b613e9c89838a01613d0d565b90975095506020880135915080821115613eb557600080fd5b50613ec288828901613d0d565b96999598509660400135949350505050565b6001600160a01b038116811461324657600080fd5b8035613b4981613ed4565b60008060408385031215613f0757600080fd5b8235613f1281613ed4565b946020939093013593505050565b600060208284031215613f3257600080fd5b81356137ba81613ed4565b634e487b7160e01b600052604160045260246000fd5b604051601f8201601f191681016001600160401b0381118282101715613f7b57613f7b613f3d565b604052919050565b60006001600160401b03821115613f9c57613f9c613f3d565b5060051b60200190565b600082601f830112613fb757600080fd5b81356020613fcc613fc783613f83565b613f53565b82815260059290921b84018101918181019086841115613feb57600080fd5b8286015b848110156140065780358352918301918301613fef565b509695505050505050565b60008060006060848603121561402657600080fd5b83356001600160401b038082111561403d57600080fd5b818601915086601f83011261405157600080fd5b81356020614061613fc783613f83565b82815260059290921b8401810191818101908a84111561408057600080fd5b948201945b838610156140a757853561409881613ed4565b82529482019490820190614085565b975050870135925050808211156140bd57600080fd5b506140ca86828701613fa6565b9250506140d960408501613ee9565b90509250925092565b6000806000604084860312156140f757600080fd5b83356001600160401b038082111561410e57600080fd5b61411a87838801613b4e565b9450602086013591508082111561413057600080fd5b5061413d86828701613bd9565b9497909650939450505050565b60006020828403121561415c57600080fd5b81516137ba81613e36565b6020808252603e908201527f456967656e506f642e6f6e6c795768656e4e6f745061757365643a20696e646560408201527f782069732070617573656420696e20456967656e506f644d616e616765720000606082015260800190565b6000808335601e198436030181126141db57600080fd5b8301803591506001600160401b038211156141f557600080fd5b6020019150600581901b3603821315613c1d57600080fd5b6000808335601e1984360301811261422457600080fd5b8301803591506001600160401b0382111561423e57600080fd5b602001915036819003821315613c1d57600080fd5b6020808252604e908201527f456967656e506f642e6f6e6c794f776e65724f7250726f6f665375626d69747460408201527f65723a2063616c6c6572206973206e6f7420706f64206f776e6572206f72207060608201526d3937b7b31039bab136b4ba3a32b960911b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b6000602082840312156142ef57600080fd5b813564ffffffffff811681146137ba57600080fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561432d5761432d614304565b500190565b600060001982141561434657614346614304565b5060010190565b600081600019048311821515161561436757614367614304565b500290565b60008282101561437e5761437e614304565b500390565b60005b8381101561439e578181015183820152602001614386565b838111156143ad576000848401525b50505050565b600082516143c5818460208701614383565b9190910192915050565b6000602082840312156143e157600080fd5b5051919050565b60208082526031908201527f456967656e506f642e6f6e6c79456967656e506f644d616e616765723a206e6f6040820152703a1032b4b3b2b72837b226b0b730b3b2b960791b606082015260800190565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6000815180845261447a816020860160208601614383565b601f01601f19169290920160200192915050565b6080815260006144a260808301888a614439565b82810360208401526144b48188614462565b905082810360408401526144c9818688614439565b915050826060830152979650505050505050565b602081526000610fe7602083018486614439565b634e487b7160e01b600052601260045260246000fd5b600082614516576145166144f1565b500690565b60008261452a5761452a6144f1565b500490565b60006001600160401b038381169083168181101561454f5761454f614304565b039392505050565b60208082526028908201527f456967656e506f642e6f6e6c79456967656e506f644f776e65723a206e6f74206040820152673837b227bbb732b960c11b606082015260800190565b60008235605e198336030181126143c557600080fd5b600062ffffff8216806145ca576145ca614304565b6000190192915050565b600081600f0b83600f0b600082128260016001607f1b03038213811516156145fe576145fe614304565b8260016001607f1b031903821281161561461a5761461a614304565b50019392505050565b60006001600160401b0380831681851680830382111561464557614645614304565b01949350505050565b80516020808301519190811015613b605760001960209190910360031b1b16919050565b60008351614684818460208801614383565b6001600160801b0319939093169190920190815260100192915050565b6000816146b0576146b0614304565b506000190190565b600081600f0b60016001607f1b03198114156146d6576146d6614304565b60000392915050565b60006001600160ff1b038184138284138082168684048611161561470557614705614304565b600160ff1b600087128281168783058912161561472457614724614304565b6000871292508782058712848416161561474057614740614304565b8785058712818416161561475657614756614304565b505050929093029392505050565b600064ffffffffff8084168061477c5761477c6144f1565b92169190910492915050565b600081600f0b83600f0b600081128160016001607f1b0319018312811516156147b3576147b3614304565b8160016001607f1b030183138116156147ce576147ce614304565b5090039392505050565b600064ffffffffff808416806147f0576147f06144f1565b92169190910692915050565b600064ffffffffff8083168185168183048111821515161561482057614820614304565b02949350505050565b6020815260006137ba602083018461446256fe456967656e506f642e5f7665726966795769746864726177616c43726564656e426561636f6e436861696e50726f6f66732e76657269667956616c696461746fa2646970667358221220f9f5d60682fdf4bd864fc835508826f6acc08906f8f542cf21a19958bf5c855064736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01jW`\x005`\xE0\x1C\x80co\xCD\x0ES\x11a\0\xD1W\x80c\xC4\x90tB\x11a\0\x8AW\x80c\xDD\xA34l\x11a\0dW\x80c\xDD\xA34l\x14a\x05\x8DW\x80c\xEE\x94\xD6|\x14a\x05\xADW\x80c\xF0t\xBAb\x14a\x05\xCDW\x80c\xF2\x88$a\x14a\x05\xEDW`\0\x80\xFD[\x80c\xC4\x90tB\x14a\x05-W\x80c\xC4\xD6m\xE8\x14a\x05MW\x80c\xD0mU\x87\x14a\x05mW`\0\x80\xFD[\x80co\xCD\x0ES\x14a\x04BW\x80ct9\x84\x1F\x14a\x04oW\x80ct\xCD\xD7\x98\x14a\x04\xA6W\x80c\x88gl\xAD\x14a\x04\xDAW\x80c\x9BNF4\x14a\x04\xFAW\x80c\xB5\"S\x8A\x14a\x05\rW`\0\x80\xFD[\x80cFe\xBC\xDA\x11a\x01#W\x80cFe\xBC\xDA\x14a\x02\xABW\x80cG\xD2\x83r\x14a\x02\xDFW\x80cR9jY\x14a\x03\x9FW\x80cXu3W\x14a\x03\xD5W\x80cX\xEA\xEEy\x14a\x03\xF5W\x80cl\r-Z\x14a\x04\"W`\0\x80\xFD[\x80c\x03\x91W\xD2\x14a\x01\xA9W\x80c\x0B\x18\xFFf\x14a\x01\xCBW\x80c#@\xE8\xD3\x14a\x02\x08W\x80c4t\xAA\x16\x14a\x02,W\x80c?e\xCF\x19\x14a\x02dW\x80cB\xEC\xFF*\x14a\x02\x84W`\0\x80\xFD[6a\x01\xA4W`@Q4\x81R\x7Fo\xDD=\xBD\xB1s)\x96\x08\xC0\xAA\x9F6\x875\x85|\x88B\xB5\x81\xF88\x928\xBF\x05\xBD\x04\xB3\xBFI\x90` \x01`@Q\x80\x91\x03\x90\xA1\0[`\0\x80\xFD[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\x01\xC9a\x01\xC46`\x04a;fV[a\x06!V[\0[4\x80\x15a\x01\xD7W`\0\x80\xFD[P`3Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02\x14W`\0\x80\xFD[Pa\x02\x1E`9T\x81V[`@Q\x90\x81R` \x01a\x01\xFFV[4\x80\x15a\x028W`\0\x80\xFD[P`4Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xFFV[4\x80\x15a\x02pW`\0\x80\xFD[Pa\x01\xC9a\x02\x7F6`\x04a<$V[a\ngV[4\x80\x15a\x02\x90W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x02\xB7W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02\xEBW`\0\x80\xFD[Pa\x03[`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91RP`@\x80Q`\x80\x81\x01\x82R`<T\x81R`=Tb\xFF\xFF\xFF\x81\x16` \x83\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x16\x92\x82\x01\x92\x90\x92R`\x01`X\x1B\x90\x91\x04`\x0F\x0B``\x82\x01R\x90V[`@Qa\x01\xFF\x91\x90\x81Q\x81R` \x80\x83\x01Qb\xFF\xFF\xFF\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`@\x1B\x03\x16\x90\x82\x01R``\x91\x82\x01Q`\x0F\x0B\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[4\x80\x15a\x03\xABW`\0\x80\xFD[Pa\x02La\x03\xBA6`\x04a<\xF2V[`;` R`\0\x90\x81R`@\x90 T`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03\xE1W`\0\x80\xFD[P`>Ta\x01\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x04\x01W`\0\x80\xFD[Pa\x04\x15a\x04\x106`\x04a=NV[a\r\xD6V[`@Qa\x01\xFF\x91\x90a=\xC7V[4\x80\x15a\x04.W`\0\x80\xFD[Pa\x02\x1Ea\x04=6`\x04a<\xF2V[a\x0E;V[4\x80\x15a\x04NW`\0\x80\xFD[Pa\x04ba\x04]6`\x04a=\xD5V[a\x0F\xEFV[`@Qa\x01\xFF\x91\x90a=\xEEV[4\x80\x15a\x04{W`\0\x80\xFD[Pa\x04\x15a\x04\x8A6`\x04a=\xD5V[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x90V[4\x80\x15a\x04\xB2W`\0\x80\xFD[Pa\x01\xEB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x04\xE6W`\0\x80\xFD[Pa\x01\xC9a\x04\xF56`\x04a>DV[a\x10\x9CV[a\x01\xC9a\x05\x086`\x04a>aV[a\x11\x91V[4\x80\x15a\x05\x19W`\0\x80\xFD[Pa\x04ba\x05(6`\x04a=NV[a\x13>V[4\x80\x15a\x059W`\0\x80\xFD[Pa\x01\xC9a\x05H6`\x04a>\xF4V[a\x141V[4\x80\x15a\x05YW`\0\x80\xFD[Pa\x01\xC9a\x05h6`\x04a? V[a\x16nV[4\x80\x15a\x05yW`\0\x80\xFD[Pa\x01\xC9a\x05\x886`\x04a? V[a\x18\x05V[4\x80\x15a\x05\x99W`\0\x80\xFD[Pa\x01\xC9a\x05\xA86`\x04a@\x11V[a\x18\x98V[4\x80\x15a\x05\xB9W`\0\x80\xFD[P`:Ta\x02L\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x05\xD9W`\0\x80\xFD[Pa\x01\xC9a\x05\xE86`\x04a@\xE2V[a\x1AkV[4\x80\x15a\x05\xF9W`\0\x80\xFD[Pa\x02L\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xAD\x91\x90aAJV[\x15a\x06\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`@Q\x80\x91\x03\x90\xFD[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x08`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07;W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07_\x91\x90aAJV[\x15a\x07|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`\0a\x07\xC2a\x07\x8B\x85\x80aA\xC4V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xBE\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x081Wa\x081a=\x8FV[`\x02\x81\x11\x15a\x08BWa\x08Ba=\x8FV[\x81RPP\x90P\x80`@\x01Q`\x01`\x01`@\x1B\x03\x16\x87`\x01`\x01`@\x1B\x03\x16\x11a\x08\xD5W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FEigenPod.verifyStaleBalance: pro`D\x82\x01R\x7Fof is older than last checkpoint`d\x82\x01R`\x84\x01a\x06\xCAV[`\x01\x81``\x01Q`\x02\x81\x11\x15a\x08\xEDWa\x08\xEDa=\x8FV[\x14a\tWW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.verifyStaleBalance: val`D\x82\x01Rsidator is not active``\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[a\t\x9Ba\td\x86\x80aA\xC4V[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xE2\x92PPPV[a\n\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FEigenPod.verifyStaleBalance: val`D\x82\x01R\x7Fidator must be slashed to be mar`d\x82\x01Rhked stale`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a\n1a\n+\x88a\x0E;V[\x87a\x1F\x0CV[a\nT\x865a\n@\x87\x80aA\xC4V[a\nM` \x8A\x01\x8AaB\rV[\x86Qa gV[a\n^`\0a\"~V[PPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\n\x8AWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\n\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aBSV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x02`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B2\x91\x90aAJV[\x15a\x0BOW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[\x85\x84\x14\x80\x15a\x0B]WP\x83\x82\x14[a\x0B\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: validatorIndices and proof`d\x82\x01Rt\x0Ed\r\xAE\xAEn\x84\x0CL\xA4\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`[\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`:T`\x01`\x01`@\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x90\x8A\x16\x11a\x0C\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FEigenPod.verifyWithdrawalCredent`D\x82\x01R\x7Fials: specified timestamp is too`d\x82\x01Rk\x08\x19\x98\\\x88\x1A[\x88\x1C\x18\\\xDD`\xA2\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a\x0C\x9Fa\x0C\x99\x8Aa\x0E;V[\x89a\x1F\x0CV[`\0\x80[\x87\x81\x10\x15a\rBWa\r$\x8A5\x8A\x8A\x84\x81\x81\x10a\x0C\xC2Wa\x0C\xC2aB\xC7V[\x90P` \x02\x01` \x81\x01\x90a\x0C\xD7\x91\x90aB\xDDV[\x89\x89\x85\x81\x81\x10a\x0C\xE9Wa\x0C\xE9aB\xC7V[\x90P` \x02\x81\x01\x90a\x0C\xFB\x91\x90aB\rV[\x89\x89\x87\x81\x81\x10a\r\rWa\r\raB\xC7V[\x90P` \x02\x81\x01\x90a\r\x1F\x91\x90aA\xC4V[a%\x14V[a\r.\x90\x83aC\x1AV[\x91P\x80a\r:\x81aC2V[\x91PPa\x0C\xA3V[P`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xB2W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xC6W=`\0\x80>=`\0\xFD[PPPPPPPPPPPPPPV[`\0\x80a\x0E\x18\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xFA\x92PPPV[`\0\x90\x81R`6` R`@\x90 T`\x01`\xC0\x1B\x90\x04`\xFF\x16\x91PP[\x92\x91PPV[`\0a\x0EJa\x1F\xFF`\x0CaCMV[a\x0E]`\x01`\x01`@\x1B\x03\x84\x16BaClV[\x10a\x0E\xC6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FEigenPod.getParentBlockRoot: tim`D\x82\x01Rrestamp out of range`h\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16` \x82\x01R`\0\x91\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0F\x0E\x91aC\xB3V[`\0`@Q\x80\x83\x03\x81\x85Z\xFA\x91PP=\x80`\0\x81\x14a\x0FIW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0FNV[``\x91P[P\x91P\x91P\x81\x80\x15a\x0FaWP`\0\x81Q\x11[a\x0F\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FEigenPod.getParentBlockRoot: inv`D\x82\x01R\x7Falid block root returned\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[\x80\x80` \x01\x90Q\x81\x01\x90a\x0F\xE7\x91\x90aC\xCFV[\x94\x93PPPPV[a\x10\x17`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`\0\x82\x81R`6` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x94\x83\x01\x94\x90\x94R`\x01`\x80\x1B\x81\x04\x90\x93\x16\x93\x81\x01\x93\x90\x93R\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x10\x82Wa\x10\x82a=\x8FV[`\x02\x81\x11\x15a\x10\x93Wa\x10\x93a=\x8FV[\x90RP\x92\x91PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14\x80a\x10\xBFWP`>T`\x01`\x01`\xA0\x1B\x03\x163\x14[a\x10\xDBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aBSV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x06`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11g\x91\x90aAJV[\x15a\x11\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[a\x11\x8D\x82a\"~V[PPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aC\xE8V[4h\x01\xBC\x16\xD6t\xEC\x80\0\0\x14a\x12eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FEigenPod.stake: must initially s\x90\x82\x01R\x7Ftake for any validator with 32 e`d\x82\x01Rc:42\xB9`\xE1\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\"\x89Q\x18h\x01\xBC\x16\xD6t\xEC\x80\0\0\x87\x87a\x12\xA8a+\xF4V[\x88\x88\x88`@Q\x88c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xCC\x96\x95\x94\x93\x92\x91\x90aD\x8EV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x12\xE5W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xF9W=`\0\x80>=`\0\xFD[PPPPP\x7F`he\xB7\x93J%\xD4\xAE\xD4?l\xDBBd\x035?\xA4\xB3\0\x9CM\"\x84\x07GE\x81\xB0\x1E#\x85\x85`@Qa\x13/\x92\x91\x90aD\xDDV[`@Q\x80\x91\x03\x90\xA1PPPPPV[a\x13f`@\x80Q`\x80\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x90``\x82\x01R\x90V[`6`\0a\x13\xA9\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa*\xFA\x92PPPV[\x81R` \x80\x82\x01\x92\x90\x92R`@\x90\x81\x01`\0 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x90\x91``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x14\x16Wa\x14\x16a=\x8FV[`\x02\x81\x11\x15a\x14'Wa\x14'a=\x8FV[\x90RP\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x14yW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aC\xE8V[a\x14\x87c;\x9A\xCA\0\x82aE\x07V[\x15a\x15\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountWei must be a who`d\x82\x01Rm\x1B\x19H\x11\xDD\xD9ZH\x18[[\xDD[\x9D`\x92\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a\x15!c;\x9A\xCA\0\x83aE\x1BV[`4T\x90\x91P`\x01`\x01`@\x1B\x03\x90\x81\x16\x90\x82\x16\x11\x15a\x15\xDAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`b`$\x82\x01R\x7FEigenPod.withdrawRestakedBeaconC`D\x82\x01R\x7FhainETH: amountGwei exceeds with`d\x82\x01R\x7FdrawableRestakedExecutionLayerGw`\x84\x82\x01Raei`\xF0\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xCAV[`4\x80T\x82\x91\x90`\0\x90a\x15\xF8\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aE/V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP\x82`\x01`\x01`\xA0\x1B\x03\x16\x7F\x89G\xFD,\xE0~\xF9\xCC0,N\x8F\x04a\x01V\x15\xD9\x1C\xE8QVH9\xE9\x1C\xC8\x04\xC2\xF4\x9D\x8E\x83`@Qa\x16W\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA2a\x16i\x83\x83a,9V[PPPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x16\x8EWP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x16\xA8WP0;\x15\x80\x15a\x16\xA8WP`\0T`\xFF\x16`\x01\x14[a\x17\x0BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x17.W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x17\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FEigenPod.initialize: podOwner ca`D\x82\x01Rsnnot be zero address``\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U\x80\x15a\x11\x8DW`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18/W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aEWV[`>T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7F\xFB\x81)\x08\n\x19\xD3M\xCE\xAC\x04\xBA%?\xC5\x03\x04\xDC\x86\xC7)\xBDc\xCD\xCAJ\x96\x9A\xD1\x9A^\xAC\x91\x01`@Q\x80\x91\x03\x90\xA1`>\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\xC2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aEWV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x05`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19N\x91\x90aAJV[\x15a\x19kW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[\x82Q\x84Q\x14a\x19\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`K`$\x82\x01R\x7FEigenPod.recoverTokens: tokenLis`D\x82\x01R\x7Ft and amountsToWithdraw must be `d\x82\x01Rj\x0El-\xAC\xA4\r\x8C\xAD\xCC\xEE\x8D`\xAB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0[\x84Q\x81\x10\x15a\x1AdWa\x1AR\x83\x85\x83\x81Q\x81\x10a\x1A\x18Wa\x1A\x18aB\xC7V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x1A2Wa\x1A2aB\xC7V[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16a-R\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80a\x1A\\\x81aC2V[\x91PPa\x19\xF9V[PPPPPV[`@QcZ\xC8j\xB7`\xE0\x1B\x81R`\x07`\x04\x82\x01\x81\x90R\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90cZ\xC8j\xB7\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xF7\x91\x90aAJV[\x15a\x1B\x14W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x90aAgV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x80a\x1B\xC0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`X`$\x82\x01R\x7FEigenPod.verifyCheckpointProofs:`D\x82\x01R\x7F must have active checkpoint to `d\x82\x01R\x7Fperform checkpoint proof\0\0\0\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@\x80Q`\x80\x81\x01\x82R`<T\x80\x82R`=Tb\xFF\xFF\xFF\x81\x16` \x84\x01R`\x01`\x01`@\x1B\x03c\x01\0\0\0\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`X\x1B\x90\x92\x04`\x0F\x0B``\x82\x01R\x90a\x1C\x10\x90\x87a-\xA4V[`\0\x80[\x85\x81\x10\x15a\x1EdW6\x87\x87\x83\x81\x81\x10a\x1C/Wa\x1C/aB\xC7V[\x90P` \x02\x81\x01\x90a\x1CA\x91\x90aE\x9FV[\x805`\0\x90\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a\x1C\xB2Wa\x1C\xB2a=\x8FV[`\x02\x81\x11\x15a\x1C\xC3Wa\x1C\xC3a=\x8FV[\x90RP\x90P`\x01\x81``\x01Q`\x02\x81\x11\x15a\x1C\xE0Wa\x1C\xE0a=\x8FV[\x14a\x1C\xECWPPa\x1ERV[\x85`\x01`\x01`@\x1B\x03\x16\x81`@\x01Q`\x01`\x01`@\x1B\x03\x16\x10a\x1D\x10WPPa\x1ERV[`\0\x80a\x1D \x83\x89\x8E5\x87a/ V[` \x89\x01\x80Q\x92\x94P\x90\x92Pa\x1D5\x82aE\xB5V[b\xFF\xFF\xFF\x16\x90RP``\x87\x01\x80Q\x83\x91\x90a\x1DQ\x90\x83\x90aE\xD4V[`\x0F\x0B\x90RPa\x1Da\x81\x87aF#V[\x845`\0\x90\x81R`6` \x90\x81R`@\x91\x82\x90 \x86Q\x81T\x92\x88\x01Q\x93\x88\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x87\x01Q\x93\x99P\x86\x93\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a\x1E\x06Wa\x1E\x06a=\x8FV[\x02\x17\x90UPP\x83Q`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x91P`\x01`\x01`@\x1B\x03\x8A\x16\x90\x7F\xA9\x1CY\x03<4#\xE1\x8BT\xD0\xAC\xEC\xEB\xB4\x97/\x9E\xA9Z\xED\xF5\xF4\xCA\xE3\xB6w\xB0.\xAF:?\x90`\0\x90\xA3PPPP[\x80a\x1E\\\x81aC2V[\x91PPa\x1C\x14V[P`\x01`\x01`@\x1B\x03\x80\x84\x16`\0\x90\x81R`;` R`@\x81 \x80T\x84\x93\x91\x92\x91a\x1E\x91\x91\x85\x91\x16aF#V[\x92Pa\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa\n^\x82a0BV[`\0\x81`\0\x81Q\x81\x10a\x1E\xD3Wa\x1E\xD3aB\xC7V[` \x02` \x01\x01Q\x90P\x91\x90PV[`\0\x81`\x03\x81Q\x81\x10a\x1E\xF7Wa\x1E\xF7aB\xC7V[` \x02` \x01\x01Q`\0\x80\x1B\x14\x15\x90P\x91\x90PV[a\x1F\x18`\x03` aCMV[a\x1F%` \x83\x01\x83aB\rV[\x90P\x14a\x1F\x9AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7Ft: Proof has incorrect length\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[a\x1F\xEAa\x1F\xAA` \x83\x01\x83aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92PP\x845\x90P`\x03a2IV[a\x11\x8DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`B`$\x82\x01R\x7FBeaconChainProofs.verifyStateRoo`D\x82\x01R\x7Ft: Invalid state root merkle pro`d\x82\x01Ra7\xB3`\xF1\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x08\x84\x14a \xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`N`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Validator fields has in`d\x82\x01Rm\x0Cm\xEENL\xACn\x84\r\x8C\xAD\xCC\xEE\x8D`\x93\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x05a \xF0`(`\x01aC\x1AV[a \xFA\x91\x90aC\x1AV[a!\x05\x90` aCMV[\x82\x14a!sW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Proof has incorrect len`d\x82\x01Rb\x0C\xEE\x8D`\xEB\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a!\xB1\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa2a\x92PPPV[\x90P`\0d\xFF\xFF\xFF\xFF\xFF\x83\x16a!\xC9`(`\x01aC\x1AV[`\x0B\x90\x1B\x17\x90Pa\"\x14\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92P\x86\x91P\x85\x90Pa2IV[a\"tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrFields: Invalid merkle proof\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[PPPPPPPPV[`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a#\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`R`$\x82\x01R\x7FEigenPod._startCheckpoint: must `D\x82\x01R\x7Ffinish previous checkpoint befor`d\x82\x01Rq2\x909\xBA0\xB9:4\xB73\x900\xB77\xBA42\xB9`q\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`:TB`\x01`\x01`@\x1B\x03\x90\x81\x16\x91\x16\x14\x15a#\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`?`$\x82\x01R\x7FEigenPod._startCheckpoint: canno`D\x82\x01R\x7Ft checkpoint twice in one block\0`d\x82\x01R`\x84\x01a\x06\xCAV[`4T`\0\x90`\x01`\x01`@\x1B\x03\x16a#\xC1c;\x9A\xCA\0GaE\x1BV[a#\xCB\x91\x90aE/V[\x90P\x81\x80\x15a#\xE1WP`\x01`\x01`@\x1B\x03\x81\x16\x15[\x15a$TW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FEigenPod._startCheckpoint: no ba`D\x82\x01R\x7Flance available to checkpoint\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[`\0`@Q\x80`\x80\x01`@R\x80a$jBa\x0E;V[\x81R` \x01`9Tb\xFF\xFF\xFF\x16\x81R` \x01\x83`\x01`\x01`@\x1B\x03\x16\x81R` \x01`\0`\x0F\x0B\x81RP\x90PB`:`\x08a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa$\xC5\x81a0BV[\x80Q` \x80\x83\x01Q`@Qb\xFF\xFF\xFF\x90\x91\x16\x81R`\x01`\x01`@\x1B\x03B\x16\x91\x7FWW\x96\x13;\xBE\xD37\xE5\xB3\x9A\xA4\x9A0\xDC%V\xA9\x1E\x0Cl*\xF4\xB7\xB8\x86\xAEw\xEB\xEF\x10v\x91\x01`@Q\x80\x91\x03\x90\xA3PPPV[`\0\x80a%S\x84\x84\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x1E\xBE\x92PPPV[`\0\x81\x81R`6` \x90\x81R`@\x80\x83 \x81Q`\x80\x81\x01\x83R\x81T`\x01`\x01`@\x1B\x03\x80\x82\x16\x83R`\x01`@\x1B\x82\x04\x81\x16\x95\x83\x01\x95\x90\x95R`\x01`\x80\x1B\x81\x04\x90\x94\x16\x92\x81\x01\x92\x90\x92R\x93\x94P\x91\x92\x90``\x83\x01\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x02\x81\x11\x15a%\xC2Wa%\xC2a=\x8FV[`\x02\x81\x11\x15a%\xD3Wa%\xD3a=\x8FV[\x90RP\x90P`\0\x81``\x01Q`\x02\x81\x11\x15a%\xF0Wa%\xF0a=\x8FV[\x14a&\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`a`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: validator must be inactiv`d\x82\x01R\x7Fe to prove withdrawal credential`\x84\x82\x01R`s`\xF8\x1B`\xA4\x82\x01R`\xC4\x01a\x06\xCAV[`\x01`\x01`@\x1B\x03\x80\x16a&\xC7\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5\x0E\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14\x15a'PW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`U`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: validator must be in the `d\x82\x01Rtprocess of activating`X\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\x01`\x01`@\x1B\x03\x80\x16a'\x96\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa53\x92PPPV[`\x01`\x01`@\x1B\x03\x16\x14a(\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aH=\x839\x81Q\x91R\x90\x82\x01R\x7Ftials: validator must not be exi`d\x82\x01Rcting`\xE0\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[a(\x16a+\xF4V[a(\x1F\x90aFNV[a([\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5K\x92PPPV[\x14a(\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` aH=\x839\x81Q\x91R`D\x82\x01R\x7Ftials: proof is not for this Eig`d\x82\x01Rd\x19[\x94\x1B\xD9`\xDA\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a)\x08\x86\x86\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa5`\x92PPPV[\x90Pa)\x18\x8A\x87\x87\x8B\x8B\x8Ea gV[`9\x80T\x90`\0a)(\x83aC2V[\x90\x91UPP`:T`\0\x90`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x15a)`W`:T`\x01`@\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16a)mV[`:T`\x01`\x01`@\x1B\x03\x16[`@\x80Q`\x80\x81\x01\x82Rd\xFF\xFF\xFF\xFF\xFF\x8D\x16\x81R`\x01`\x01`@\x1B\x03\x85\x81\x16` \x83\x01R\x83\x16\x91\x81\x01\x91\x90\x91R\x90\x91P``\x81\x01`\x01\x90R`\0\x85\x81R`6` \x90\x81R`@\x91\x82\x90 \x83Q\x81T\x92\x85\x01Q\x93\x85\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x95\x16\x91\x90\x92\x16\x17\x92\x90\x92\x17\x92\x83\x16\x82\x17\x81U``\x84\x01Q\x90\x92\x90\x91\x83\x91`\xFF`\xC0\x1B\x19\x90\x91\x16h\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x90\x91\x16\x17`\x01`\xC0\x1B\x83`\x02\x81\x11\x15a*CWa*Ca=\x8FV[\x02\x17\x90UPP`@Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R\x7F-\x08\0\xBB\xC3w\xEAT\xA0\x8C]\xB6\xA8z\xAF\xFF^>\x9C\x8F\xEA\xD0\xED\xA1\x10\xE4\x0E\x0C\x10D\x14I\x91P` \x01`@Q\x80\x91\x03\x90\xA1`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x8C\x16\x81R`\x01`\x01`@\x1B\x03\x83\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x81\x90\x03``\x01\x90\xA1a*\xEBc;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x84\x16aCMV[\x9B\x9APPPPPPPPPPPV[`\0\x81Q`0\x14a+\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FEigenPod._calculateValidatorPubk`D\x82\x01R\x7FeyHash must be a 48-byte BLS pub`d\x82\x01Rflic key`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@Q`\x02\x90a+\x9A\x90\x84\x90`\0\x90` \x01aFrV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra+\xB4\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a+\xD1W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E5\x91\x90aC\xCFV[`@\x80Q`\x01`\xF8\x1B` \x82\x01R`\0`!\x82\x01R0``\x90\x81\x1Bk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`,\x83\x01R\x91\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x90V[\x80G\x10\x15a,\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x06\xCAV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a,\xD6W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a,\xDBV[``\x91P[PP\x90P\x80a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x84\x16`$\x82\x01R`D\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`d\x90\x91\x01\x90\x91R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16c\xA9\x05\x9C\xBB`\xE0\x1B\x17\x90Ra\x16i\x90\x84\x90a5xV[a-\xB0`\x05`\x03aC\x1AV[a-\xBB\x90` aCMV[a-\xC8` \x83\x01\x83aB\rV[\x90P\x14a.KW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R\x7FBeaconChainProofs.verifyBalanceC\x90\x82\x01R\x7Fontainer: Proof has incorrect le`d\x82\x01Rc\r\xCC\xEE\x8D`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`la.\x9Ca.]` \x84\x01\x84aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x87\x92PP\x855\x90P\x84a2IV[a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FBeaconChainProofs.verifyBalanceC`D\x82\x01R\x7Fontainer: invalid balance contai`d\x82\x01Rh72\xB9\x10897\xB7\xB3`\xB9\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[\x83Q` \x85\x01Q`\0\x91\x82\x91\x82a/8\x87\x84\x88a6JV[\x90P\x81`\x01`\x01`@\x1B\x03\x16\x81`\x01`\x01`@\x1B\x03\x16\x14a/\xB2Wa/]\x81\x83a7\xC1V[`@\x80Qd\xFF\xFF\xFF\xFF\xFF\x86\x16\x81R`\x01`\x01`@\x1B\x03\x8B\x81\x16` \x83\x01R\x84\x16\x81\x83\x01R\x90Q\x91\x96P\x7F\x0E_\xAC\x17[\x83\x17|\xC0G8\x1E\x03\r\x8F\xB3\xB4+7\xBD\x1C\x02^\"\xC2\x80\xFA\xCA\xD6,2\xDF\x91\x90\x81\x90\x03``\x01\x90\xA1[`\x01`\x01`@\x1B\x03\x80\x82\x16` \x8B\x01\x81\x90R\x90\x89\x16`@\x8B\x01Ra06W`9\x80T\x90`\0a/\xE0\x83aF\xA1V[\x90\x91UPP`\x02``\x8A\x01Ra/\xF5\x85aF\xB8V[\x93P\x82d\xFF\xFF\xFF\xFF\xFF\x16\x88`\x01`\x01`@\x1B\x03\x16\x7F*\x026\x1F\xFAf\xCF,-\xA4h,#U\xA6\xAD\xCA\xA9\xF6\xC2'\xB6\xE6V>hH\x0F\x95\x87bj`@Q`@Q\x80\x91\x03\x90\xA3[PPP\x94P\x94\x92PPPV[` \x81\x01Qb\xFF\xFF\xFF\x16a1\xC9W`\0c;\x9A\xCA\0\x82``\x01Q\x83`@\x01Q`\x01`\x01`@\x1B\x03\x16a0t\x91\x90aE\xD4V[`\x0F\x0Ba0\x81\x91\x90aF\xDFV[`@\x83\x01Q`4\x80T\x92\x93P\x90\x91`\0\x90a0\xA6\x90\x84\x90`\x01`\x01`@\x1B\x03\x16aF#V[\x82Ta\x01\0\x92\x90\x92\n`\x01`\x01`@\x1B\x03\x81\x81\x02\x19\x90\x93\x16\x91\x83\x16\x02\x17\x90\x91U`:\x80T`\x01`@\x1B\x81\x04\x90\x92\x16`\x01`\x01`\x80\x1B\x03\x19\x90\x92\x16\x91\x90\x91\x17\x90UP`\0`<U`=\x80T`\x01`\x01`\xD8\x1B\x03\x19\x16\x90U`3T`@Qc\x03\x0B\x14q`\xE6\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x83\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x91\x16\x90c\xC2\xC5\x1C@\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a1kW`\0\x80\xFD[PZ\xF1\x15\x80\x15a1\x7FW=`\0\x80>=`\0\xFD[PP`:T`@Q\x84\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16\x92P\x7FRT\x08\xC2\x01\xBC\x15v\xEBD\x11odx\xF1\xC2\xA5Gu\xB1\x9A\x04;\xCF\xDCp\x83d\xF7O\x8ED\x91P` \x01`@Q\x80\x91\x03\x90\xA2PPV[\x80Q`<U` \x81\x01Q`=\x80T`@\x84\x01Q``\x85\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x01`X\x1B\x02o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`X\x1B\x19`\x01`\x01`@\x1B\x03\x90\x92\x16c\x01\0\0\0\x02j\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16b\xFF\xFF\xFF\x90\x95\x16\x94\x90\x94\x17\x91\x90\x91\x17\x16\x91\x90\x91\x17\x90U[PV[`\0\x83a2W\x86\x85\x85a7\xD9V[\x14\x95\x94PPPPPV[`\0\x80`\x02\x83Qa2r\x91\x90aE\x1BV[\x90P`\0\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x8EWa2\x8Ea?=V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a2\xB7W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x82\x81\x10\x15a3\xBEW`\x02\x85a2\xD2\x83\x83aCMV[\x81Q\x81\x10a2\xE2Wa2\xE2aB\xC7V[` \x02` \x01\x01Q\x86\x83`\x02a2\xF8\x91\x90aCMV[a3\x03\x90`\x01aC\x1AV[\x81Q\x81\x10a3\x13Wa3\x13aB\xC7V[` \x02` \x01\x01Q`@Q` \x01a35\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra3O\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a3lW=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x8F\x91\x90aC\xCFV[\x82\x82\x81Q\x81\x10a3\xA1Wa3\xA1aB\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a3\xB6\x81aC2V[\x91PPa2\xBDV[Pa3\xCA`\x02\x83aE\x1BV[\x91P[\x81\x15a4\xEAW`\0[\x82\x81\x10\x15a4\xD7W`\x02\x82a3\xEB\x83\x83aCMV[\x81Q\x81\x10a3\xFBWa3\xFBaB\xC7V[` \x02` \x01\x01Q\x83\x83`\x02a4\x11\x91\x90aCMV[a4\x1C\x90`\x01aC\x1AV[\x81Q\x81\x10a4,Wa4,aB\xC7V[` \x02` \x01\x01Q`@Q` \x01a4N\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra4h\x91aC\xB3V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a4\x85W=`\0\x80>=`\0\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a4\xA8\x91\x90aC\xCFV[\x82\x82\x81Q\x81\x10a4\xBAWa4\xBAaB\xC7V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80a4\xCF\x81aC2V[\x91PPa3\xD6V[Pa4\xE3`\x02\x83aE\x1BV[\x91Pa3\xCDV[\x80`\0\x81Q\x81\x10a4\xFDWa4\xFDaB\xC7V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[`\0a\x0E5\x82`\x05\x81Q\x81\x10a5&Wa5&aB\xC7V[` \x02` \x01\x01Qa9%V[`\0a\x0E5\x82`\x06\x81Q\x81\x10a5&Wa5&aB\xC7V[`\0\x81`\x01\x81Q\x81\x10a\x1E\xD3Wa\x1E\xD3aB\xC7V[`\0a\x0E5\x82`\x02\x81Q\x81\x10a5&Wa5&aB\xC7V[`\0a5\xCD\x82`@Q\x80`@\x01`@R\x80` \x81R` \x01\x7FSafeERC20: low-level call failed\x81RP\x85`\x01`\x01`\xA0\x1B\x03\x16a9\x8C\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x80Q\x90\x91P\x15a\x16iW\x80\x80` \x01\x90Q\x81\x01\x90a5\xEB\x91\x90aAJV[a\x16iW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`*`$\x82\x01R\x7FSafeERC20: ERC20 operation did n`D\x82\x01Ri\x1B\xDD\x08\x1C\xDDX\xD8\xD9YY`\xB2\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\0a6X`&`\x01aC\x1AV[a6c\x90` aCMV[a6p`@\x84\x01\x84aB\rV[\x90P\x14a6\xE1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`D`$\x82\x01\x81\x90R`\0\x80Q` aH]\x839\x81Q\x91R\x90\x82\x01R\x7FrBalance: Proof has incorrect le`d\x82\x01Rc\r\xCC\xEE\x8D`\xE3\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`\0a6\xEE`\x04\x85aGdV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa7Ha7\x07`@\x85\x01\x85aB\rV[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x89\x92PPP` \x86\x015\x84a2IV[a7\xA8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`>`$\x82\x01R`\0\x80Q` aH]\x839\x81Q\x91R`D\x82\x01R\x7FrBalance: Invalid merkle proof\0\0`d\x82\x01R`\x84\x01a\x06\xCAV[a7\xB6\x83` \x015\x85a9\x9BV[\x91PP[\x93\x92PPPV[`\0a7\xBA`\x01`\x01`@\x1B\x03\x80\x84\x16\x90\x85\x16aG\x88V[`\0\x83Q`\0\x14\x15\x80\x15a7\xF8WP` \x84Qa7\xF6\x91\x90aE\x07V[\x15[a8\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`T`$\x82\x01R\x7FMerkle.processInclusionProofSha2`D\x82\x01R\x7F56: proof length should be a non`d\x82\x01Rs\x16\xBD2\xB97\x906\xBA\xB6:4\xB862\x907\xB3\x10\x19\x99`a\x1B`\x84\x82\x01R`\xA4\x01a\x06\xCAV[`@\x80Q` \x80\x82\x01\x90\x92R\x84\x81R\x90[\x85Q\x81\x11a9\x1BWa8\xAB`\x02\x85aE\x07V[a8\xDEW\x81Q`\0R\x80\x86\x01Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa8\xD3W`\0\x80\xFD[`\x02\x84\x04\x93Pa9\tV[\x80\x86\x01Q`\0R\x81Q` R` \x82`@`\0`\x02a\x07\xD0Z\x03\xFAa9\x02W`\0\x80\xFD[`\x02\x84\x04\x93P[a9\x14` \x82aC\x1AV[\x90Pa8\x98V[PQ\x94\x93PPPPV[`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17g\xFF\0\0\0\0\0\0\0`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[``a\x0F\xE7\x84\x84`\0\x85a9\xC8V[`\0\x80a9\xA9`\x04\x84aG\xD8V[a9\xB4\x90`@aG\xFCV[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa\x0F\xE7\x84\x82\x1Ba9%V[``\x82G\x10\x15a:)W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FAddress: insufficient balance fo`D\x82\x01Re\x1C\x88\x18\xD8[\x1B`\xD2\x1B`d\x82\x01R`\x84\x01a\x06\xCAV[`\x01`\x01`\xA0\x1B\x03\x85\x16;a:\x80W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: call to non-contract\0\0\0`D\x82\x01R`d\x01a\x06\xCAV[`\0\x80\x86`\x01`\x01`\xA0\x1B\x03\x16\x85\x87`@Qa:\x9C\x91\x90aC\xB3V[`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a:\xD9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a:\xDEV[``\x91P[P\x91P\x91Pa:\xEE\x82\x82\x86a:\xF9V[\x97\x96PPPPPPPV[``\x83\x15a;\x08WP\x81a7\xBAV[\x82Q\x15a;\x18W\x82Q\x80\x84` \x01\xFD[\x81`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\xCA\x91\x90aH)V[\x805`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a;IW`\0\x80\xFD[\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a;`W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a;{W`\0\x80\xFD[a;\x84\x84a;2V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;\xA0W`\0\x80\xFD[a;\xAC\x87\x83\x88\x01a;NV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a;\xC2W`\0\x80\xFD[Pa;\xCF\x86\x82\x87\x01a;NV[\x91PP\x92P\x92P\x92V[`\0\x80\x83`\x1F\x84\x01\x12a;\xEBW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a<\x02W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a<\x1DW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\xA0\x89\x8B\x03\x12\x15a<@W`\0\x80\xFD[a<I\x89a;2V[\x97P` \x89\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<eW`\0\x80\xFD[a<q\x8C\x83\x8D\x01a;NV[\x98P`@\x8B\x015\x91P\x80\x82\x11\x15a<\x87W`\0\x80\xFD[a<\x93\x8C\x83\x8D\x01a;\xD9V[\x90\x98P\x96P``\x8B\x015\x91P\x80\x82\x11\x15a<\xACW`\0\x80\xFD[a<\xB8\x8C\x83\x8D\x01a;\xD9V[\x90\x96P\x94P`\x80\x8B\x015\x91P\x80\x82\x11\x15a<\xD1W`\0\x80\xFD[Pa<\xDE\x8B\x82\x8C\x01a;\xD9V[\x99\x9C\x98\x9BP\x96\x99P\x94\x97\x93\x96\x92\x95\x94PPPV[`\0` \x82\x84\x03\x12\x15a=\x04W`\0\x80\xFD[a7\xBA\x82a;2V[`\0\x80\x83`\x1F\x84\x01\x12a=\x1FW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a=6W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a<\x1DW`\0\x80\xFD[`\0\x80` \x83\x85\x03\x12\x15a=aW`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a=wW`\0\x80\xFD[a=\x83\x85\x82\x86\x01a=\rV[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x03\x81\x10a=\xC3WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a\x0E5\x82\x84a=\xA5V[`\0` \x82\x84\x03\x12\x15a=\xE7W`\0\x80\xFD[P5\x91\x90PV[`\0`\x80\x82\x01\x90P`\x01`\x01`@\x1B\x03\x80\x84Q\x16\x83R\x80` \x85\x01Q\x16` \x84\x01R\x80`@\x85\x01Q\x16`@\x84\x01RP``\x83\x01Qa>/``\x84\x01\x82a=\xA5V[P\x92\x91PPV[\x80\x15\x15\x81\x14a2FW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>VW`\0\x80\xFD[\x815a7\xBA\x81a>6V[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a>yW`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a>\x90W`\0\x80\xFD[a>\x9C\x89\x83\x8A\x01a=\rV[\x90\x97P\x95P` \x88\x015\x91P\x80\x82\x11\x15a>\xB5W`\0\x80\xFD[Pa>\xC2\x88\x82\x89\x01a=\rV[\x96\x99\x95\x98P\x96`@\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a2FW`\0\x80\xFD[\x805a;I\x81a>\xD4V[`\0\x80`@\x83\x85\x03\x12\x15a?\x07W`\0\x80\xFD[\x825a?\x12\x81a>\xD4V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a?2W`\0\x80\xFD[\x815a7\xBA\x81a>\xD4V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a?{Wa?{a?=V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a?\x9CWa?\x9Ca?=V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a?\xB7W`\0\x80\xFD[\x815` a?\xCCa?\xC7\x83a?\x83V[a?SV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a?\xEBW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a@\x06W\x805\x83R\x91\x83\x01\x91\x83\x01a?\xEFV[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a@&W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a@=W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a@QW`\0\x80\xFD[\x815` a@aa?\xC7\x83a?\x83V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a@\x80W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a@\xA7W\x855a@\x98\x81a>\xD4V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a@\x85V[\x97PP\x87\x015\x92PP\x80\x82\x11\x15a@\xBDW`\0\x80\xFD[Pa@\xCA\x86\x82\x87\x01a?\xA6V[\x92PPa@\xD9`@\x85\x01a>\xE9V[\x90P\x92P\x92P\x92V[`\0\x80`\0`@\x84\x86\x03\x12\x15a@\xF7W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aA\x0EW`\0\x80\xFD[aA\x1A\x87\x83\x88\x01a;NV[\x94P` \x86\x015\x91P\x80\x82\x11\x15aA0W`\0\x80\xFD[PaA=\x86\x82\x87\x01a;\xD9V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15aA\\W`\0\x80\xFD[\x81Qa7\xBA\x81a>6V[` \x80\x82R`>\x90\x82\x01R\x7FEigenPod.onlyWhenNotPaused: inde`@\x82\x01R\x7Fx is paused in EigenPodManager\0\0``\x82\x01R`\x80\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xDBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA\xF5W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a<\x1DW`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB$W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aB>W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a<\x1DW`\0\x80\xFD[` \x80\x82R`N\x90\x82\x01R\x7FEigenPod.onlyOwnerOrProofSubmitt`@\x82\x01R\x7Fer: caller is not pod owner or p``\x82\x01Rm97\xB7\xB3\x109\xBA\xB16\xB4\xBA:2\xB9`\x91\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15aB\xEFW`\0\x80\xFD[\x815d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a7\xBAW`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x19\x82\x11\x15aC-WaC-aC\x04V[P\x01\x90V[`\0`\0\x19\x82\x14\x15aCFWaCFaC\x04V[P`\x01\x01\x90V[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aCgWaCgaC\x04V[P\x02\x90V[`\0\x82\x82\x10\x15aC~WaC~aC\x04V[P\x03\x90V[`\0[\x83\x81\x10\x15aC\x9EW\x81\x81\x01Q\x83\x82\x01R` \x01aC\x86V[\x83\x81\x11\x15aC\xADW`\0\x84\x84\x01R[PPPPV[`\0\x82QaC\xC5\x81\x84` \x87\x01aC\x83V[\x91\x90\x91\x01\x92\x91PPV[`\0` \x82\x84\x03\x12\x15aC\xE1W`\0\x80\xFD[PQ\x91\x90PV[` \x80\x82R`1\x90\x82\x01R\x7FEigenPod.onlyEigenPodManager: no`@\x82\x01Rp:\x102\xB4\xB3\xB2\xB7(7\xB2&\xB0\xB70\xB3\xB2\xB9`y\x1B``\x82\x01R`\x80\x01\x90V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\0\x81Q\x80\x84RaDz\x81` \x86\x01` \x86\x01aC\x83V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`\x80\x81R`\0aD\xA2`\x80\x83\x01\x88\x8AaD9V[\x82\x81\x03` \x84\x01RaD\xB4\x81\x88aDbV[\x90P\x82\x81\x03`@\x84\x01RaD\xC9\x81\x86\x88aD9V[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV[` \x81R`\0a\x0F\xE7` \x83\x01\x84\x86aD9V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82aE\x16WaE\x16aD\xF1V[P\x06\x90V[`\0\x82aE*WaE*aD\xF1V[P\x04\x90V[`\0`\x01`\x01`@\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aEOWaEOaC\x04V[\x03\x93\x92PPPV[` \x80\x82R`(\x90\x82\x01R\x7FEigenPod.onlyEigenPodOwner: not `@\x82\x01Rg87\xB2'\xBB\xB72\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[`\0\x825`^\x19\x836\x03\x01\x81\x12aC\xC5W`\0\x80\xFD[`\0b\xFF\xFF\xFF\x82\x16\x80aE\xCAWaE\xCAaC\x04V[`\0\x19\x01\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x82\x12\x82`\x01`\x01`\x7F\x1B\x03\x03\x82\x13\x81\x15\x16\x15aE\xFEWaE\xFEaC\x04V[\x82`\x01`\x01`\x7F\x1B\x03\x19\x03\x82\x12\x81\x16\x15aF\x1AWaF\x1AaC\x04V[P\x01\x93\x92PPPV[`\0`\x01`\x01`@\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aFEWaFEaC\x04V[\x01\x94\x93PPPPV[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a;`W`\0\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[`\0\x83QaF\x84\x81\x84` \x88\x01aC\x83V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x91\x90\x92\x01\x90\x81R`\x10\x01\x92\x91PPV[`\0\x81aF\xB0WaF\xB0aC\x04V[P`\0\x19\x01\x90V[`\0\x81`\x0F\x0B`\x01`\x01`\x7F\x1B\x03\x19\x81\x14\x15aF\xD6WaF\xD6aC\x04V[`\0\x03\x92\x91PPV[`\0`\x01`\x01`\xFF\x1B\x03\x81\x84\x13\x82\x84\x13\x80\x82\x16\x86\x84\x04\x86\x11\x16\x15aG\x05WaG\x05aC\x04V[`\x01`\xFF\x1B`\0\x87\x12\x82\x81\x16\x87\x83\x05\x89\x12\x16\x15aG$WaG$aC\x04V[`\0\x87\x12\x92P\x87\x82\x05\x87\x12\x84\x84\x16\x16\x15aG@WaG@aC\x04V[\x87\x85\x05\x87\x12\x81\x84\x16\x16\x15aGVWaGVaC\x04V[PPP\x92\x90\x93\x02\x93\x92PPPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80aG|WaG|aD\xF1V[\x92\x16\x91\x90\x91\x04\x92\x91PPV[`\0\x81`\x0F\x0B\x83`\x0F\x0B`\0\x81\x12\x81`\x01`\x01`\x7F\x1B\x03\x19\x01\x83\x12\x81\x15\x16\x15aG\xB3WaG\xB3aC\x04V[\x81`\x01`\x01`\x7F\x1B\x03\x01\x83\x13\x81\x16\x15aG\xCEWaG\xCEaC\x04V[P\x90\x03\x93\x92PPPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x84\x16\x80aG\xF0WaG\xF0aD\xF1V[\x92\x16\x91\x90\x91\x06\x92\x91PPV[`\0d\xFF\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aH WaH aC\x04V[\x02\x94\x93PPPPV[` \x81R`\0a7\xBA` \x83\x01\x84aDbV\xFEEigenPod._verifyWithdrawalCredenBeaconChainProofs.verifyValidato\xA2dipfsX\"\x12 \xF9\xF5\xD6\x06\x82\xFD\xF4\xBD\x86O\xC85P\x88&\xF6\xAC\xC0\x89\x06\xF8\xF5B\xCF!\xA1\x99X\xBF\\\x85PdsolcC\0\x08\x0C\x003",
    );
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "CheckpointCreated(uint64,bytes32,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    87u8, 87u8, 150u8, 19u8, 59u8, 190u8, 211u8, 55u8, 229u8, 179u8, 154u8, 164u8,
                    154u8, 48u8, 220u8, 37u8, 86u8, 169u8, 30u8, 12u8, 108u8, 42u8, 244u8, 183u8,
                    184u8, 134u8, 174u8, 119u8, 235u8, 239u8, 16u8, 118u8,
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
                        &self.validatorCount,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
            );
            const SIGNATURE: &'static str = "CheckpointFinalized(uint64,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    82u8, 84u8, 8u8, 194u8, 1u8, 188u8, 21u8, 118u8, 235u8, 68u8, 17u8, 111u8,
                    100u8, 120u8, 241u8, 194u8, 165u8, 71u8, 117u8, 177u8, 154u8, 4u8, 59u8, 207u8,
                    220u8, 112u8, 131u8, 100u8, 247u8, 79u8, 142u8, 68u8,
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
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
                        &self.totalShareDeltaWei,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.checkpointTimestamp.clone(),
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "EigenPodStaked(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    96u8, 104u8, 101u8, 183u8, 147u8, 74u8, 37u8, 212u8, 174u8, 212u8, 63u8, 108u8,
                    219u8, 66u8, 100u8, 3u8, 53u8, 63u8, 164u8, 179u8, 0u8, 156u8, 77u8, 34u8,
                    132u8, 7u8, 71u8, 69u8, 129u8, 176u8, 30u8, 35u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "NonBeaconChainETHReceived(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    111u8, 221u8, 61u8, 189u8, 177u8, 115u8, 41u8, 150u8, 8u8, 192u8, 170u8, 159u8,
                    54u8, 135u8, 53u8, 133u8, 124u8, 136u8, 66u8, 181u8, 129u8, 248u8, 56u8, 146u8,
                    56u8, 191u8, 5u8, 189u8, 4u8, 179u8, 191u8, 73u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    amountReceived: data.0,
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
                        &self.amountReceived,
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
            fn from(this: &NonBeaconChainETHReceived) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ProofSubmitterUpdated(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    251u8, 129u8, 41u8, 8u8, 10u8, 25u8, 211u8, 77u8, 206u8, 172u8, 4u8, 186u8,
                    37u8, 63u8, 197u8, 3u8, 4u8, 220u8, 134u8, 199u8, 41u8, 189u8, 99u8, 205u8,
                    202u8, 74u8, 150u8, 154u8, 209u8, 154u8, 94u8, 172u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "RestakedBeaconChainETHWithdrawn(address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    137u8, 71u8, 253u8, 44u8, 224u8, 126u8, 249u8, 204u8, 48u8, 44u8, 78u8, 143u8,
                    4u8, 97u8, 1u8, 86u8, 21u8, 217u8, 28u8, 232u8, 81u8, 86u8, 72u8, 57u8, 233u8,
                    28u8, 200u8, 4u8, 194u8, 244u8, 157u8, 142u8,
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
                        &self.amount,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        impl From<&RestakedBeaconChainETHWithdrawn> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &RestakedBeaconChainETHWithdrawn) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorBalanceUpdated(uint40,uint64,uint64)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    14u8, 95u8, 172u8, 23u8, 91u8, 131u8, 23u8, 124u8, 192u8, 71u8, 56u8, 30u8,
                    3u8, 13u8, 143u8, 179u8, 180u8, 43u8, 55u8, 189u8, 28u8, 2u8, 94u8, 34u8,
                    194u8, 128u8, 250u8, 202u8, 214u8, 44u8, 50u8, 223u8,
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.balanceTimestamp,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &ValidatorBalanceUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
            );
            const SIGNATURE: &'static str = "ValidatorCheckpointed(uint64,uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    169u8, 28u8, 89u8, 3u8, 60u8, 52u8, 35u8, 225u8, 139u8, 84u8, 208u8, 172u8,
                    236u8, 235u8, 180u8, 151u8, 47u8, 158u8, 169u8, 90u8, 237u8, 245u8, 244u8,
                    202u8, 227u8, 182u8, 119u8, 176u8, 46u8, 175u8, 58u8, 63u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "ValidatorRestaked(uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    45u8, 8u8, 0u8, 187u8, 195u8, 119u8, 234u8, 84u8, 160u8, 140u8, 93u8, 182u8,
                    168u8, 122u8, 175u8, 255u8, 94u8, 62u8, 156u8, 143u8, 234u8, 208u8, 237u8,
                    161u8, 16u8, 228u8, 14u8, 12u8, 16u8, 68u8, 20u8, 73u8,
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<64>,
                alloy::sol_types::sol_data::Uint<40>,
            );
            const SIGNATURE: &'static str = "ValidatorWithdrawn(uint64,uint40)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    42u8, 2u8, 54u8, 31u8, 250u8, 102u8, 207u8, 44u8, 45u8, 164u8, 104u8, 44u8,
                    35u8, 85u8, 166u8, 173u8, 202u8, 169u8, 246u8, 194u8, 39u8, 182u8, 230u8, 86u8,
                    62u8, 104u8, 72u8, 15u8, 149u8, 135u8, 98u8, 106u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        #[allow(missing_docs)]
        pub _ethPOS: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _eigenPodManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
                        &self._ethPOS,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self._GENESIS_TIME,
                    ),
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = GENESIS_TIMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<activeValidatorCountCall> for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activeValidatorCountCall {
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
            impl ::core::convert::From<activeValidatorCountReturn> for UnderlyingRustTuple<'_> {
                fn from(value: activeValidatorCountReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for activeValidatorCountReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for activeValidatorCountCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = activeValidatorCountReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub _0: u64,
    }
    ///Container type for the return parameters of the [`checkpointBalanceExitedGwei(uint64)`](checkpointBalanceExitedGweiCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkpointBalanceExitedGweiReturn {
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
            impl ::core::convert::From<checkpointBalanceExitedGweiCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointBalanceExitedGweiCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<checkpointBalanceExitedGweiReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkpointBalanceExitedGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkpointBalanceExitedGweiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkpointBalanceExitedGweiCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkpointBalanceExitedGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
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
    /**Function with signature `currentCheckpoint()` and selector `0x47d28372`.
    ```solidity
    function currentCheckpoint() external view returns (IEigenPod.Checkpoint memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointCall {}
    ///Container type for the return parameters of the [`currentCheckpoint()`](currentCheckpointCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct currentCheckpointReturn {
        #[allow(missing_docs)]
        pub _0: <IEigenPod::Checkpoint as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<currentCheckpointCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::Checkpoint,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPod::Checkpoint as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentCheckpointReturn;
            type ReturnTuple<'a> = (IEigenPod::Checkpoint,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<currentCheckpointTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointTimestampCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<currentCheckpointTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentCheckpointTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<eigenPodManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: eigenPodManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for eigenPodManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for eigenPodManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = eigenPodManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = ethPOSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub timestamp: u64,
    }
    ///Container type for the return parameters of the [`getParentBlockRoot(uint64)`](getParentBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getParentBlockRootReturn {
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
            impl ::core::convert::From<getParentBlockRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootCall) -> Self {
                    (value.timestamp,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getParentBlockRootCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getParentBlockRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getParentBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getParentBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getParentBlockRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getParentBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.timestamp,
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
    /**Function with signature `initialize(address)` and selector `0xc4d66de8`.
    ```solidity
    function initialize(address _podOwner) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
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
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            impl ::core::convert::From<lastCheckpointTimestampCall> for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastCheckpointTimestampCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<lastCheckpointTimestampReturn> for UnderlyingRustTuple<'_> {
                fn from(value: lastCheckpointTimestampReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for lastCheckpointTimestampReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for lastCheckpointTimestampCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = lastCheckpointTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = podOwnerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<proofSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: proofSubmitterReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for proofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for proofSubmitterCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = proofSubmitterReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub tokenList: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        #[allow(missing_docs)]
        pub amountsToWithdraw:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = recoverTokensReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProofSubmitterCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterCall) -> Self {
                    (value.newProofSubmitter,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProofSubmitterCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        newProofSubmitter: tuple.0,
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
            impl ::core::convert::From<setProofSubmitterReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setProofSubmitterReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProofSubmitterReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProofSubmitterCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProofSubmitterReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub pubkey: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub signature: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        revertIfNoBalance: tuple.0,
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
            impl ::core::convert::From<startCheckpointReturn> for UnderlyingRustTuple<'_> {
                fn from(value: startCheckpointReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for startCheckpointReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for startCheckpointCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = startCheckpointReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validatorPubkeyHashToInfo(bytes32)`](validatorPubkeyHashToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyHashToInfoReturn {
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyHashToInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoCall) -> Self {
                    (value.validatorPubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyHashToInfoCall {
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
            type UnderlyingRustTuple<'a> =
                (<IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyHashToInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyHashToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyHashToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyHashToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorPubkeyHashToInfoReturn;
            type ReturnTuple<'a> = (IEigenPod::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`validatorPubkeyToInfo(bytes)`](validatorPubkeyToInfoCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorPubkeyToInfoReturn {
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyToInfoCall> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoCall) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyToInfoCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkey: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::ValidatorInfo,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPod::ValidatorInfo as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorPubkeyToInfoReturn> for UnderlyingRustTuple<'_> {
                fn from(value: validatorPubkeyToInfoReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorPubkeyToInfoReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorPubkeyToInfoCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorPubkeyToInfoReturn;
            type ReturnTuple<'a> = (IEigenPod::ValidatorInfo,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub validatorPubkey: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`validatorStatus(bytes)`](validatorStatus_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_0Return {
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Call) -> Self {
                    (value.validatorPubkey,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorPubkey: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorStatus_0Return;
            type ReturnTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`validatorStatus(bytes32)`](validatorStatus_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct validatorStatus_1Return {
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Call) -> Self {
                    (value.pubkeyHash,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pubkeyHash: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<IEigenPod::VALIDATOR_STATUS as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<validatorStatus_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: validatorStatus_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for validatorStatus_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for validatorStatus_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = validatorStatus_1Return;
            type ReturnTuple<'a> = (IEigenPod::VALIDATOR_STATUS,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub balanceContainerProof:
            <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyCheckpointProofsCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsCall) -> Self {
                    (value.balanceContainerProof, value.proofs)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCheckpointProofsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyCheckpointProofsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyCheckpointProofsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyCheckpointProofsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyCheckpointProofsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyCheckpointProofs((bytes32,bytes),(bytes32,bytes32,bytes)[])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub beaconTimestamp: u64,
        #[allow(missing_docs)]
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyStaleBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceCall) -> Self {
                    (value.beaconTimestamp, value.stateRootProof, value.proof)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyStaleBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyStaleBalanceReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyStaleBalanceReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyStaleBalanceReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyStaleBalance(uint64,(bytes32,bytes),(bytes32[],bytes))";
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.beaconTimestamp,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub beaconTimestamp: u64,
        #[allow(missing_docs)]
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub validatorIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
        #[allow(missing_docs)]
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
        #[allow(missing_docs)]
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u64,
                <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<verifyWithdrawalCredentialsCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<verifyWithdrawalCredentialsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: verifyWithdrawalCredentialsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for verifyWithdrawalCredentialsReturn {
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                >,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = verifyWithdrawalCredentialsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "verifyWithdrawalCredentials(uint64,(bytes32,bytes),uint40[],bytes[],bytes32[][])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawRestakedBeaconChainETHCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHCall) -> Self {
                    (value.recipient, value.amountWei)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawRestakedBeaconChainETHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawRestakedBeaconChainETHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawRestakedBeaconChainETHReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawRestakedBeaconChainETHReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawRestakedBeaconChainETHReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.amountWei,
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
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiCall> for UnderlyingRustTuple<'_> {
                fn from(value: withdrawableRestakedExecutionLayerGweiCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for withdrawableRestakedExecutionLayerGweiCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<withdrawableRestakedExecutionLayerGweiReturn>
                for UnderlyingRustTuple<'_>
            {
                fn from(value: withdrawableRestakedExecutionLayerGweiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for withdrawableRestakedExecutionLayerGweiReturn
            {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawableRestakedExecutionLayerGweiCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawableRestakedExecutionLayerGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    ///Container for all the [`EigenPod`](self) function calls.
    pub enum EigenPodCalls {
        #[allow(missing_docs)]
        GENESIS_TIME(GENESIS_TIMECall),
        #[allow(missing_docs)]
        activeValidatorCount(activeValidatorCountCall),
        #[allow(missing_docs)]
        checkpointBalanceExitedGwei(checkpointBalanceExitedGweiCall),
        #[allow(missing_docs)]
        currentCheckpoint(currentCheckpointCall),
        #[allow(missing_docs)]
        currentCheckpointTimestamp(currentCheckpointTimestampCall),
        #[allow(missing_docs)]
        eigenPodManager(eigenPodManagerCall),
        #[allow(missing_docs)]
        ethPOS(ethPOSCall),
        #[allow(missing_docs)]
        getParentBlockRoot(getParentBlockRootCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        lastCheckpointTimestamp(lastCheckpointTimestampCall),
        #[allow(missing_docs)]
        podOwner(podOwnerCall),
        #[allow(missing_docs)]
        proofSubmitter(proofSubmitterCall),
        #[allow(missing_docs)]
        recoverTokens(recoverTokensCall),
        #[allow(missing_docs)]
        setProofSubmitter(setProofSubmitterCall),
        #[allow(missing_docs)]
        stake(stakeCall),
        #[allow(missing_docs)]
        startCheckpoint(startCheckpointCall),
        #[allow(missing_docs)]
        validatorPubkeyHashToInfo(validatorPubkeyHashToInfoCall),
        #[allow(missing_docs)]
        validatorPubkeyToInfo(validatorPubkeyToInfoCall),
        #[allow(missing_docs)]
        validatorStatus_0(validatorStatus_0Call),
        #[allow(missing_docs)]
        validatorStatus_1(validatorStatus_1Call),
        #[allow(missing_docs)]
        verifyCheckpointProofs(verifyCheckpointProofsCall),
        #[allow(missing_docs)]
        verifyStaleBalance(verifyStaleBalanceCall),
        #[allow(missing_docs)]
        verifyWithdrawalCredentials(verifyWithdrawalCredentialsCall),
        #[allow(missing_docs)]
        withdrawRestakedBeaconChainETH(withdrawRestakedBeaconChainETHCall),
        #[allow(missing_docs)]
        withdrawableRestakedExecutionLayerGwei(withdrawableRestakedExecutionLayerGweiCall),
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
        #[allow(non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(&[u8], bool) -> alloy_sol_types::Result<EigenPodCalls>] = &[
                {
                    fn verifyStaleBalance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<EigenPodCalls> {
                        <verifyStaleBalanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                        <podOwnerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <ethPOSCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <stakeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(EigenPodCalls::GENESIS_TIME)
                    }
                    GENESIS_TIME
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
    ///Container for all the [`EigenPod`](self) events.
    pub enum EigenPodEvents {
        #[allow(missing_docs)]
        CheckpointCreated(CheckpointCreated),
        #[allow(missing_docs)]
        CheckpointFinalized(CheckpointFinalized),
        #[allow(missing_docs)]
        EigenPodStaked(EigenPodStaked),
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NonBeaconChainETHReceived(NonBeaconChainETHReceived),
        #[allow(missing_docs)]
        ProofSubmitterUpdated(ProofSubmitterUpdated),
        #[allow(missing_docs)]
        RestakedBeaconChainETHWithdrawn(RestakedBeaconChainETHWithdrawn),
        #[allow(missing_docs)]
        ValidatorBalanceUpdated(ValidatorBalanceUpdated),
        #[allow(missing_docs)]
        ValidatorCheckpointed(ValidatorCheckpointed),
        #[allow(missing_docs)]
        ValidatorRestaked(ValidatorRestaked),
        #[allow(missing_docs)]
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
                14u8, 95u8, 172u8, 23u8, 91u8, 131u8, 23u8, 124u8, 192u8, 71u8, 56u8, 30u8, 3u8,
                13u8, 143u8, 179u8, 180u8, 43u8, 55u8, 189u8, 28u8, 2u8, 94u8, 34u8, 194u8, 128u8,
                250u8, 202u8, 214u8, 44u8, 50u8, 223u8,
            ],
            [
                42u8, 2u8, 54u8, 31u8, 250u8, 102u8, 207u8, 44u8, 45u8, 164u8, 104u8, 44u8, 35u8,
                85u8, 166u8, 173u8, 202u8, 169u8, 246u8, 194u8, 39u8, 182u8, 230u8, 86u8, 62u8,
                104u8, 72u8, 15u8, 149u8, 135u8, 98u8, 106u8,
            ],
            [
                45u8, 8u8, 0u8, 187u8, 195u8, 119u8, 234u8, 84u8, 160u8, 140u8, 93u8, 182u8, 168u8,
                122u8, 175u8, 255u8, 94u8, 62u8, 156u8, 143u8, 234u8, 208u8, 237u8, 161u8, 16u8,
                228u8, 14u8, 12u8, 16u8, 68u8, 20u8, 73u8,
            ],
            [
                82u8, 84u8, 8u8, 194u8, 1u8, 188u8, 21u8, 118u8, 235u8, 68u8, 17u8, 111u8, 100u8,
                120u8, 241u8, 194u8, 165u8, 71u8, 117u8, 177u8, 154u8, 4u8, 59u8, 207u8, 220u8,
                112u8, 131u8, 100u8, 247u8, 79u8, 142u8, 68u8,
            ],
            [
                87u8, 87u8, 150u8, 19u8, 59u8, 190u8, 211u8, 55u8, 229u8, 179u8, 154u8, 164u8,
                154u8, 48u8, 220u8, 37u8, 86u8, 169u8, 30u8, 12u8, 108u8, 42u8, 244u8, 183u8,
                184u8, 134u8, 174u8, 119u8, 235u8, 239u8, 16u8, 118u8,
            ],
            [
                96u8, 104u8, 101u8, 183u8, 147u8, 74u8, 37u8, 212u8, 174u8, 212u8, 63u8, 108u8,
                219u8, 66u8, 100u8, 3u8, 53u8, 63u8, 164u8, 179u8, 0u8, 156u8, 77u8, 34u8, 132u8,
                7u8, 71u8, 69u8, 129u8, 176u8, 30u8, 35u8,
            ],
            [
                111u8, 221u8, 61u8, 189u8, 177u8, 115u8, 41u8, 150u8, 8u8, 192u8, 170u8, 159u8,
                54u8, 135u8, 53u8, 133u8, 124u8, 136u8, 66u8, 181u8, 129u8, 248u8, 56u8, 146u8,
                56u8, 191u8, 5u8, 189u8, 4u8, 179u8, 191u8, 73u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                137u8, 71u8, 253u8, 44u8, 224u8, 126u8, 249u8, 204u8, 48u8, 44u8, 78u8, 143u8, 4u8,
                97u8, 1u8, 86u8, 21u8, 217u8, 28u8, 232u8, 81u8, 86u8, 72u8, 57u8, 233u8, 28u8,
                200u8, 4u8, 194u8, 244u8, 157u8, 142u8,
            ],
            [
                169u8, 28u8, 89u8, 3u8, 60u8, 52u8, 35u8, 225u8, 139u8, 84u8, 208u8, 172u8, 236u8,
                235u8, 180u8, 151u8, 47u8, 158u8, 169u8, 90u8, 237u8, 245u8, 244u8, 202u8, 227u8,
                182u8, 119u8, 176u8, 46u8, 175u8, 58u8, 63u8,
            ],
            [
                251u8, 129u8, 41u8, 8u8, 10u8, 25u8, 211u8, 77u8, 206u8, 172u8, 4u8, 186u8, 37u8,
                63u8, 197u8, 3u8, 4u8, 220u8, 134u8, 199u8, 41u8, 189u8, 99u8, 205u8, 202u8, 74u8,
                150u8, 154u8, 209u8, 154u8, 94u8, 172u8,
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
                Some(<CheckpointCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CheckpointCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::CheckpointCreated)
                }
                Some(<CheckpointFinalized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <CheckpointFinalized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::CheckpointFinalized)
                }
                Some(<EigenPodStaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <EigenPodStaked as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::EigenPodStaked)
                }
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NonBeaconChainETHReceived as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NonBeaconChainETHReceived)
                }
                Some(<ProofSubmitterUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ProofSubmitterUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ProofSubmitterUpdated)
                }
                Some(
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <RestakedBeaconChainETHWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::RestakedBeaconChainETHWithdrawn)
                }
                Some(<ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorBalanceUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidatorBalanceUpdated)
                }
                Some(<ValidatorCheckpointed as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorCheckpointed as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidatorCheckpointed)
                }
                Some(<ValidatorRestaked as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorRestaked as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidatorRestaked)
                }
                Some(<ValidatorWithdrawn as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ValidatorWithdrawn as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::ValidatorWithdrawn)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<EigenPodInstance<T, P, N>>>
    {
        EigenPodInstance::<T, P, N>::deploy(provider, _ethPOS, _eigenPodManager, _GENESIS_TIME)
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
        EigenPodInstance::<T, P, N>::deploy_builder(
            provider,
            _ethPOS,
            _eigenPodManager,
            _GENESIS_TIME,
        )
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
            f.debug_tuple("EigenPodInstance")
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
        > EigenPodInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`EigenPod`](self) contract instance.

        See the [wrapper's documentation](`EigenPodInstance`) for more details.*/
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
            _ethPOS: alloy::sol_types::private::Address,
            _eigenPodManager: alloy::sol_types::private::Address,
            _GENESIS_TIME: u64,
        ) -> alloy_contract::Result<EigenPodInstance<T, P, N>> {
            let call_builder =
                Self::deploy_builder(provider, _ethPOS, _eigenPodManager, _GENESIS_TIME);
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _ethPOS,
                        _eigenPodManager,
                        _GENESIS_TIME,
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
        > EigenPodInstance<T, P, N>
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
        ///Creates a new call builder for the [`GENESIS_TIME`] function.
        pub fn GENESIS_TIME(&self) -> alloy_contract::SolCallBuilder<T, &P, GENESIS_TIMECall, N> {
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
            self.call_builder(&checkpointBalanceExitedGweiCall { _0 })
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
            self.call_builder(&getParentBlockRootCall { timestamp })
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
        pub fn podOwner(&self) -> alloy_contract::SolCallBuilder<T, &P, podOwnerCall, N> {
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
            tokenList: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
            amountsToWithdraw: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, recoverTokensCall, N> {
            self.call_builder(&recoverTokensCall {
                tokenList,
                amountsToWithdraw,
                recipient,
            })
        }
        ///Creates a new call builder for the [`setProofSubmitter`] function.
        pub fn setProofSubmitter(
            &self,
            newProofSubmitter: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProofSubmitterCall, N> {
            self.call_builder(&setProofSubmitterCall { newProofSubmitter })
        }
        ///Creates a new call builder for the [`stake`] function.
        pub fn stake(
            &self,
            pubkey: alloy::sol_types::private::Bytes,
            signature: alloy::sol_types::private::Bytes,
            depositDataRoot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeCall, N> {
            self.call_builder(&stakeCall {
                pubkey,
                signature,
                depositDataRoot,
            })
        }
        ///Creates a new call builder for the [`startCheckpoint`] function.
        pub fn startCheckpoint(
            &self,
            revertIfNoBalance: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, startCheckpointCall, N> {
            self.call_builder(&startCheckpointCall { revertIfNoBalance })
        }
        ///Creates a new call builder for the [`validatorPubkeyHashToInfo`] function.
        pub fn validatorPubkeyHashToInfo(
            &self,
            validatorPubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorPubkeyHashToInfoCall, N> {
            self.call_builder(&validatorPubkeyHashToInfoCall {
                validatorPubkeyHash,
            })
        }
        ///Creates a new call builder for the [`validatorPubkeyToInfo`] function.
        pub fn validatorPubkeyToInfo(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorPubkeyToInfoCall, N> {
            self.call_builder(&validatorPubkeyToInfoCall { validatorPubkey })
        }
        ///Creates a new call builder for the [`validatorStatus_0`] function.
        pub fn validatorStatus_0(
            &self,
            validatorPubkey: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorStatus_0Call, N> {
            self.call_builder(&validatorStatus_0Call { validatorPubkey })
        }
        ///Creates a new call builder for the [`validatorStatus_1`] function.
        pub fn validatorStatus_1(
            &self,
            pubkeyHash: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, validatorStatus_1Call, N> {
            self.call_builder(&validatorStatus_1Call { pubkeyHash })
        }
        ///Creates a new call builder for the [`verifyCheckpointProofs`] function.
        pub fn verifyCheckpointProofs(
            &self,
            balanceContainerProof: <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
            proofs: alloy::sol_types::private::Vec<
                <BeaconChainProofs::BalanceProof as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyCheckpointProofsCall, N> {
            self.call_builder(&verifyCheckpointProofsCall {
                balanceContainerProof,
                proofs,
            })
        }
        ///Creates a new call builder for the [`verifyStaleBalance`] function.
        pub fn verifyStaleBalance(
            &self,
            beaconTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            proof: <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyStaleBalanceCall, N> {
            self.call_builder(&verifyStaleBalanceCall {
                beaconTimestamp,
                stateRootProof,
                proof,
            })
        }
        ///Creates a new call builder for the [`verifyWithdrawalCredentials`] function.
        pub fn verifyWithdrawalCredentials(
            &self,
            beaconTimestamp: u64,
            stateRootProof: <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            validatorFieldsProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
            validatorFields: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, verifyWithdrawalCredentialsCall, N> {
            self.call_builder(&verifyWithdrawalCredentialsCall {
                beaconTimestamp,
                stateRootProof,
                validatorIndices,
                validatorFieldsProofs,
                validatorFields,
            })
        }
        ///Creates a new call builder for the [`withdrawRestakedBeaconChainETH`] function.
        pub fn withdrawRestakedBeaconChainETH(
            &self,
            recipient: alloy::sol_types::private::Address,
            amountWei: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawRestakedBeaconChainETHCall, N> {
            self.call_builder(&withdrawRestakedBeaconChainETHCall {
                recipient,
                amountWei,
            })
        }
        ///Creates a new call builder for the [`withdrawableRestakedExecutionLayerGwei`] function.
        pub fn withdrawableRestakedExecutionLayerGwei(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawableRestakedExecutionLayerGweiCall, N>
        {
            self.call_builder(&withdrawableRestakedExecutionLayerGweiCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > EigenPodInstance<T, P, N>
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
        pub fn EigenPodStaked_filter(&self) -> alloy_contract::Event<T, &P, EigenPodStaked, N> {
            self.event_filter::<EigenPodStaked>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
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
