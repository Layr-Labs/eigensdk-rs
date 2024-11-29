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
        pub validatorFields:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
library StdInvariant {
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    struct FuzzInterface { address addr; string[] artifacts; }
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
    struct FuzzArtifactSelector { string artifact; bytes4[] selectors; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzArtifactSelector {
        pub artifact: alloy::sol_types::private::String,
        pub selectors: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
            alloy::sol_types::sol_data::String,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<4>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        impl ::core::convert::From<FuzzArtifactSelector> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzArtifactSelector) -> Self {
                (value.artifact, value.selectors)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzArtifactSelector {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    artifact: tuple.0,
                    selectors: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzArtifactSelector {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzArtifactSelector {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.artifact,
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.artifact,
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
        impl alloy_sol_types::EventTopic for FuzzArtifactSelector {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifact,
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifact,
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
            }
        }
    };
    /**```solidity
    struct FuzzInterface { address addr; string[] artifacts; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzInterface {
        pub addr: alloy::sol_types::private::Address,
        pub artifacts: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
        impl ::core::convert::From<FuzzInterface> for UnderlyingRustTuple<'_> {
            fn from(value: FuzzInterface) -> Self {
                (value.addr, value.artifacts)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for FuzzInterface {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    addr: tuple.0,
                    artifacts: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for FuzzInterface {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for FuzzInterface {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.addr,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::tokenize(&self.artifacts),
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                            &self.addr,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.artifacts)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for FuzzInterface {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.addr,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::String,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.artifacts,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.addr,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::String,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.artifacts,
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
    struct FuzzSelector { address addr; bytes4[] selectors; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct FuzzSelector {
        pub addr: alloy::sol_types::private::Address,
        pub selectors: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<4>>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for FuzzSelector {
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
        impl alloy_sol_types::SolStruct for FuzzSelector {
            const NAME: &'static str = "FuzzSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzSelector(address addr,bytes4[] selectors)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("StdInvariantInstance")
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
        > StdInvariantInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`StdInvariant`](self) contract instance.

        See the [wrapper's documentation](`StdInvariantInstance`) for more details.*/
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
        > StdInvariantInstance<T, P, N>
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
        > StdInvariantInstance<T, P, N>
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

library StdInvariant {
    struct FuzzArtifactSelector {
        string artifact;
        bytes4[] selectors;
    }
    struct FuzzInterface {
        address addr;
        string[] artifacts;
    }
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
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function exitEpoch(uint40 validatorIndex) external view returns (uint64);
    function exitValidator(uint40 validatorIndex) external returns (uint64 exitedBalanceGwei);
    function failed() external view returns (bool);
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
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
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
    "name": "excludeSelectors",
    "inputs": [],
    "outputs": [
      {
        "name": "excludedSelectors_",
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
    "stateMutability": "view"
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
        "internalType": "struct StdInvariant.FuzzArtifactSelector[]",
        "components": [
          {
            "name": "artifact",
            "type": "string",
            "internalType": "string"
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
    "name": "targetInterfaces",
    "inputs": [],
    "outputs": [
      {
        "name": "targetedInterfaces_",
        "type": "tuple[]",
        "internalType": "struct StdInvariant.FuzzInterface[]",
        "components": [
          {
            "name": "addr",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "artifacts",
            "type": "string[]",
            "internalType": "string[]"
          }
        ]
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
    ///0x610100604052600c805460ff19166001179055601f8054747109709ecfa91a80626ff3989d68f67f5b1dd12d016001600160a81b03199091161790556100476003602061060e565b60805260056100586028600161062b565b610062919061062b565b61006d90602061060e565b60a05261007c6005600361062b565b61008790602061060e565b60c0526100966026600161062b565b6100a190602061060e565b60e0523480156100af575f5ffd5b50604051615ab7380380615ab78339810160408190526100ce9161063e565b601f8054600160a81b600160e81b031916600160a81b6001600160401b03841602179081905560208054600160401b600160e01b031916680100000000000000006001600160a01b0386811691909102919091178255604051610100909304169163b4d6c78291720f3df6d732807ef1319fb7b8bb8522d0beac029190610156908201610590565b6020820181038252601f19601f820116604052506040518363ffffffff1660e01b815260040161018792919061068c565b5f604051808303815f87803b15801561019e575f5ffd5b505af11580156101b0573d5f5f3e3d5ffd5b505060408051600880825261012082019092525f93506101e0925090602082016101008036833701905050610300565b604080516064808252610ca0820190925291925060208201610c80803683375050815161021492602a92506020019061059d565b5080602a5f81548110610229576102296106e4565b5f9182526020909120015560015b602a548110156102f757604080516020810184905290810183905260029060600160408051601f1981840301815290829052610272916106f8565b602060405180830381855afa15801561028d573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906102b0919061070e565b602a82815481106102c3576102c36106e4565b905f5260205f200181905550602a81815481106102e2576102e26106e4565b5f918252602090912001549150600101610237565b50505050610744565b5f5f600283516103109190610725565b90505f816001600160401b0381111561032b5761032b6106d0565b604051908082528060200260200182016040528015610354578160200160208202803683370190505b5090505f5b8281101561044e5760028561036e838361060e565b8151811061037e5761037e6106e4565b602002602001015186836002610394919061060e565b61039f90600161062b565b815181106103af576103af6106e4565b60200260200101516040516020016103d1929190918252602082015260400190565b60408051601f19818403018152908290526103eb916106f8565b602060405180830381855afa158015610406573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610429919061070e565b82828151811061043b5761043b6106e4565b6020908102919091010152600101610359565b5061045a600283610725565b91505b811561056d575f5b8281101561055a5760028261047a838361060e565b8151811061048a5761048a6106e4565b6020026020010151838360026104a0919061060e565b6104ab90600161062b565b815181106104bb576104bb6106e4565b60200260200101516040516020016104dd929190918252602082015260400190565b60408051601f19818403018152908290526104f7916106f8565b602060405180830381855afa158015610512573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190610535919061070e565b828281518110610547576105476106e4565b6020908102919091010152600101610465565b50610566600283610725565b915061045d565b805f8151811061057f5761057f6106e4565b602002602001015192505050919050565b61028e8061582983390190565b828054828255905f5260205f209081019282156105d6579160200282015b828111156105d65782518255916020019190600101906105bb565b506105e29291506105e6565b5090565b5b808211156105e2575f81556001016105e7565b634e487b7160e01b5f52601160045260245ffd5b8082028115828204841417610625576106256105fa565b92915050565b80820180821115610625576106256105fa565b5f5f6040838503121561064f575f5ffd5b82516001600160a01b0381168114610665575f5ffd5b60208401519092506001600160401b0381168114610681575f5ffd5b809150509250929050565b60018060a01b0383168152604060208201525f82518060408401528060208501606085015e5f606082850101526060601f19601f8301168401019150509392505050565b634e487b7160e01b5f52604160045260245ffd5b634e487b7160e01b5f52603260045260245ffd5b5f82518060208501845e5f920191825250919050565b5f6020828403121561071e575f5ffd5b5051919050565b5f8261073f57634e487b7160e01b5f52601260045260245ffd5b500490565b60805160a05160c05160e0516150ab61077e5f395f61329701525f8181612ee10152612f4d01525f6130ad01525f612dca01526150ab5ff3fe608060405260043610610212575f3560e01c806386a6f9e11161011e578063ba414fa6116100a8578063f0acd9881161006d578063f0acd988146105f9578063f72138731461060d578063f833eb631461062c578063f8f98a4e1461064b578063fa7626d41461066a575f5ffd5b8063ba414fa614610568578063c76f25c01461057c578063e20c9f71146105a8578063e3cefb42146105bc578063ed3c1605146105d0575f5ffd5b8063a50a3a1a116100ee578063a50a3a1a146104b9578063aa47389c146104e5578063b0464fdc14610514578063b1b6f6a114610528578063b5508aa914610554575f5ffd5b806386a6f9e114610303578063908820e014610446578063916a17c614610465578063a3f4df7e14610486575f5ffd5b80633cf80e6c1161019f5780635e6cc2fc1161016f5780635e6cc2fc146103a557806366d9a9a0146103d15780636b3abd97146103f2578063766718081461041157806385226c8114610425575f5ffd5b80633cf80e6c146103555780633e5e3c23146103695780633f7286f41461037d57806359d095dd14610391575f5ffd5b806329992faa116101e557806329992faa146102cc5780632ade3880146102e25780632def600914610303578063330bc27e14610322578063357e951f14610336575f5ffd5b806314360958146102165780631ed7831c146102525780631f54365c1461027357806323e82c4c146102a0575b5f5ffd5b348015610221575f5ffd5b5061023561023036600461447f565b610683565b6040516001600160401b0390911681526020015b60405180910390f35b34801561025d575f5ffd5b506102666107f3565b60405161024991906144b0565b34801561027e575f5ffd5b5061029261028d3660046144fb565b610853565b604051908152602001610249565b3480156102ab575f5ffd5b506102bf6102ba3660046144fb565b610885565b60405161024991906145a1565b3480156102d7575f5ffd5b506102e0610a97565b005b3480156102ed575f5ffd5b506102f6610f42565b604051610249919061466a565b34801561030e575f5ffd5b5061023561031d3660046144fb565b61107e565b34801561032d575f5ffd5b50610235600a81565b348015610341575f5ffd5b50602054610235906001600160401b031681565b348015610360575f5ffd5b506102e06110bb565b348015610374575f5ffd5b50610266611102565b348015610388575f5ffd5b50610266611160565b34801561039c575f5ffd5b506102e06111be565b3480156103b0575f5ffd5b506103c46103bf3660046144fb565b6111f5565b60405161024991906146ed565b3480156103dc575f5ffd5b506103e5611224565b6040516102499190614739565b3480156103fd575f5ffd5b5061029261040c36600461447f565b611388565b34801561041c575f5ffd5b5061023561140e565b348015610430575f5ffd5b506104396114d4565b60405161024991906147b7565b348015610451575f5ffd5b506102926104603660046144fb565b61159f565b348015610470575f5ffd5b506104796115cb565b60405161024991906147c9565b348015610491575f5ffd5b5060408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b60208201526103c4565b3480156104c4575f5ffd5b506104d86104d336600461447f565b6116ac565b604051610249919061488e565b3480156104f0575f5ffd5b506105046104ff3660046144fb565b611ac5565b6040519015158152602001610249565b34801561051f575f5ffd5b50610479611b14565b348015610533575f5ffd5b50610547610542366004614944565b611bf5565b604051610249919061499d565b34801561055f575f5ffd5b50610439611faf565b348015610573575f5ffd5b5061050461207a565b348015610587575f5ffd5b5061059b61059636600461447f565b61211a565b6040516102499190614a41565b3480156105b3575f5ffd5b506102666121d5565b3480156105c7575f5ffd5b50610235600181565b6105e36105de366004614a9e565b612233565b60405164ffffffffff9091168152602001610249565b348015610604575f5ffd5b506102e06123ee565b348015610618575f5ffd5b506102356106273660046144fb565b612434565b348015610637575f5ffd5b506102356106463660046144fb565b612447565b348015610656575f5ffd5b506102356106653660046144fb565b61248b565b348015610675575f5ffd5b50601f546105049060ff1681565b5f6106b46040518060400160405280600f81526020016e736c61736856616c696461746f727360881b81525061272d565b5f5b82518110156107ed575f8382815181106106d2576106d2614b17565b602002602001015190505f60218264ffffffffff16815481106106f7576106f7614b17565b5f9182526020909120600490910201805490915060ff16156107345760405162461bcd60e51b815260040161072b90614b2b565b60405180910390fd5b8054610100900460ff1661078b57805461ff00191661010017815561075761140e565b610762906001614bb8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055505b5f610795836127bd565b90506001600160401b038116600a11156107bd576107b38186614bb8565b94505f90506107d8565b6107c8600a86614bb8565b94506107d5600a82614bd7565b90505b6107e283826127c7565b5050506001016106b6565b50919050565b6060601680548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161082b575b5050505050905090565b5f60218264ffffffffff168154811061086e5761086e614b17565b905f5260205f209060040201600101549050919050565b61088d614298565b6025546001600160401b03165f90815260286020908152604080832064ffffffffff86168452825280832081518154606094810282018501845292810183815290939192849284919084018282801561090357602002820191905f5260205f20905b8154815260200190600101908083116108ef575b5050505050815260200160018201805461091c90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461094890614bf6565b80156109935780601f1061096a57610100808354040283529160200191610993565b820191905f5260205f20905b81548152906001019060200180831161097657829003601f168201915b505050919092525050604080516060810182526025546001600160401b03168082525f908152602660209081529083902083518085019094528054845260018101805496975092958287019550909291840191906109f090614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054610a1c90614bf6565b8015610a675780601f10610a3e57610100808354040283529160200191610a67565b820191905f5260205f20905b815481529060010190602001808311610a4a57829003601f168201915b50505091909252505050815260408051808201909152835181526020938401518185015292019190915292915050565b5f5b602154811015610b30575f60218281548110610ab757610ab7614b17565b5f9182526020909120600490910201805490915060ff1615610ad95750610b28565b5f610ae3836127bd565b9050640773594000816001600160401b03161115610b0357506407735940005b600391909101805467ffffffffffffffff19166001600160401b039092169190911790555b600101610a99565b50610b6f6040518060400160405280601c81526020017f2d2075706461746564206566666563746976652062616c616e6365730000000081525061282c565b610bb16040518060400160405280601081526020016f05a5a40c6eae4e4cadce840cae0dec6d60831b815250610ba361140e565b6001600160401b031661285b565b5f610bba61140e565b601f5490915061010090046001600160a01b031663e5d6bf02610bdc83612898565b6040516001600160e01b031960e084901b1681526001600160401b0390911660048201526024015f604051808303815f87803b158015610c1a575f5ffd5b505af1158015610c2c573d5f5f3e3d5ffd5b50506025805467ffffffffffffffff1916426001600160401b0316179055505060408051808201909152601681527505a40d4eadae0cac840e8de40dccaf0e840cae0dec6d60531b6020820152610c8590610ba361140e565b610cc36040518060400160405280601d81526020017f2d206275696c64696e6720626561636f6e20737461746520747265657300000081525061282c565b60215415610ce257602154610cda90600190614c28565b602455610d97565b60255460405163159a829560e31b81526001600160401b0390911660048201527fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4706024820152720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a8906044015f604051808303815f87803b158015610d5d575f5ffd5b505af1158015610d6f573d5f5f3e3d5ffd5b50505050610d946040518060600160405280602881526020016150286028913961282c565b50565b5f610dcd610da36128d7565b610daf60286001614c3b565b6025546001600160401b03165f908152602b60205260409020612964565b90505f610e08610ddb612bc4565b610de760266001614c3b565b6025546001600160401b03165f908152602b60205260409020600201612964565b90505f610e3c610e188484612c5a565b6025546001600160401b03165f908152602b60205260409020600590600401612964565b90505f610e6f610e4b83612d05565b6025546001600160401b03165f908152602b60205260409020600390600601612964565b9050610ea7604051806040016040528060148152602001730b4b4818995858dbdb88189b1bd8dac81c9bdbdd60621b81525082612d8e565b60255460405163159a829560e31b81526001600160401b03909116600482015260248101829052720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a8906044015f604051808303815f87803b158015610f03575f5ffd5b505af1158015610f15573d5f5f3e3d5ffd5b50505050610f2282612dc7565b610f2b83612ede565b610f33613085565b610f3b613266565b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015611075575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561105e578382905f5260205f20018054610fd390614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054610fff90614bf6565b801561104a5780601f106110215761010080835404028352916020019161104a565b820191905f5260205f20905b81548152906001019060200180831161102d57829003601f168201915b505050505081526020019060010190610fb6565b505050508152505081526020019060010190610f65565b50505050905090565b5f60218264ffffffffff168154811061109957611099614b17565b5f9182526020909120600360049092020101546001600160401b031692915050565b6110e86040518060400160405280600c81526020016b0c2c8ecc2dcc6ca8ae0dec6d60a31b81525061272d565b6110f06133ba565b6110f8613481565b611100610a97565b565b6060601880548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b6110f060405180604001604052806016815260200175616476616e636545706f63685f4e6f5265776172647360501b81525061272d565b60408051603080825260608281019093525f919060208201818036833750505060308101939093525090919050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f2090600202016040518060400160405290815f8201805461127790614bf6565b80601f01602080910402602001604051908101604052809291908181526020018280546112a390614bf6565b80156112ee5780601f106112c5576101008083540402835291602001916112ee565b820191905f5260205f20905b8154815290600101906020018083116112d157829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561137057602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116113325790505b50505050508152505081526020019060010190611247565b5f80805b835181101561140757633b9aca0060218583815181106113ae576113ae614b17565b602002602001015164ffffffffff16815481106113cd576113cd614b17565b5f9182526020909120600360049092020101546113f391906001600160401b0316614c4e565b6113fd9083614c3b565b915060010161138c565b5092915050565b601f545f90600160a81b90046001600160401b03164210156114985760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e2e63757272656e7445706f63683a2063757272656e60448201527f742074696d65206973206265666f72652067656e657369732074696d65000000606482015260840161072b565b6114a4600c6020614c65565b601f546001600160401b03918216916114c591600160a81b90041642614c28565b6114cf9190614c9b565b905090565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f2001805461151490614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461154090614bf6565b801561158b5780601f106115625761010080835404028352916020019161158b565b820191905f5260205f20905b81548152906001019060200180831161156e57829003601f168201915b5050505050815260200190600101906114f7565b5f6022816115ae600485614cae565b64ffffffffff16815260208101919091526040015f205492915050565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611075575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561169457602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116116565790505b505050505081525050815260200190600101906115ee565b6116b46142f5565b5f5b82518110156117ab576024548382815181106116d4576116d4614b17565b602002602001015164ffffffffff1611156117a35760405162461bcd60e51b815260206004820152607760248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f2076616c696461746f7220686173206e6f74206265656e20696e636c7564656460648201527f20696e20626561636f6e20636861696e207374617465202844494420594f552060848201527f43414c4c20616476616e636545706f6368205945543f2900000000000000000060a482015260c40161072b565b6001016116b6565b50604080516080810182526025546001600160401b03168082525f908152602660209081528382208451808601909552805485526001810180549395838601949093840191906117fa90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461182690614bf6565b80156118715780601f1061184857610100808354040283529160200191611871565b820191905f5260205f20905b81548152906001019060200180831161185457829003601f168201915b505050505081525050815260200184516001600160401b038111156118985761189861439c565b6040519080825280602002602001820160405280156118cb57816020015b60608152602001906001900390816118b65790505b50815260200184516001600160401b038111156118ea576118ea61439c565b60405190808252806020026020018201604052801561191d57816020015b60608152602001906001900390816119085790505b50905290505f5b8351811015611407576025546001600160401b03165f9081526028602052604081208551829087908590811061195c5761195c614b17565b602002602001015164ffffffffff1664ffffffffff1681526020019081526020015f206040518060400160405290815f82018054806020026020016040519081016040528092919081815260200182805480156119d657602002820191905f5260205f20905b8154815260200190600101908083116119c2575b505050505081526020016001820180546119ef90614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611a1b90614bf6565b8015611a665780601f10611a3d57610100808354040283529160200191611a66565b820191905f5260205f20905b815481529060010190602001808311611a4957829003601f168201915b5050505050815250509050806020015183604001518381518110611a8c57611a8c614b17565b6020026020010181905250805f015183606001518381518110611ab157611ab1614b17565b602090810291909101015250600101611924565b5f6001600160401b03801660218364ffffffffff1681548110611aea57611aea614b17565b5f918252602090912060049091020160030154600160801b90046001600160401b03161492915050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015611075575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611bdd57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611b9f5790505b50505050508152505081526020019060010190611b37565b611c1f604080516080810182525f9181019182526060808201529081908152602001606081525090565b5f5b8351811015611cf057602454848281518110611c3f57611c3f614b17565b602002602001015164ffffffffff161115611ce85760405162461bcd60e51b815260206004820152605b60248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f206e6f20636865636b706f696e742070726f6f6620666f756e6420286469642060648201527f796f752063616c6c20616476616e636545706f6368207965743f290000000000608482015260a40161072b565b600101611c21565b50604080516001600160401b0384165f90815260276020528281206080830184528054938301938452600181018054929484939092916060850191611d3490614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611d6090614bf6565b8015611dab5780601f10611d8257610100808354040283529160200191611dab565b820191905f5260205f20905b815481529060010190602001808311611d8e57829003601f168201915b505050505081525050815260200185516001600160401b03811115611dd257611dd261439c565b604051908082528060200260200182016040528015611e2557816020015b611e1260405180606001604052805f81526020015f8152602001606081525090565b815260200190600190039081611df05790505b50905290505f5b8451811015611fa5575f858281518110611e4857611e48614b17565b602002602001015190505f611e5c82613665565b6001600160401b0387165f90815260296020908152604080832064ffffffffff851684528252808320815180830190925280548252600181018054959650939491939092840191611eac90614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611ed890614bf6565b8015611f235780601f10611efa57610100808354040283529160200191611f23565b820191905f5260205f20905b815481529060010190602001808311611f0657829003601f168201915b5050505050815250509050604051806060016040528060218564ffffffffff1681548110611f5357611f53614b17565b905f5260205f209060040201600101548152602001825f01518152602001826020015181525085602001518581518110611f8f57611f8f614b17565b6020908102919091010152505050600101611e2c565b5090505b92915050565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f20018054611fef90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461201b90614bf6565b80156120665780601f1061203d57610100808354040283529160200191612066565b820191905f5260205f20905b81548152906001019060200180831161204957829003601f168201915b505050505081526020019060010190611fd2565b6008545f9060ff1615612091575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa1580156120ef573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121139190614cd7565b1415905090565b60605f82516001600160401b038111156121365761213661439c565b60405190808252806020026020018201604052801561215f578160200160208202803683370190505b5090505f5b835181101561140757602184828151811061218157612181614b17565b602002602001015164ffffffffff16815481106121a0576121a0614b17565b905f5260205f209060040201600101548282815181106121c2576121c2614b17565b6020908102919091010152600101612164565b6060601580548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b5f6122616040518060400160405280600c81526020016b3732bbab30b634b230ba37b960a11b81525061272d565b34670de0b6b3a76400008110156122d65760405162461bcd60e51b815260206004820152603360248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a206465604482015272706f7369742076616c756520746f6f206c6f7760681b606482015260840161072b565b6122e4633b9aca0082614cee565b156123575760405162461bcd60e51b815260206004820152603860248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a20766160448201527f6c7565206e6f74206d756c7469706c65206f6620677765690000000000000000606482015260840161072b565b5f612366633b9aca0083614c9b565b90506001600160401b038111156123dc5760405162461bcd60e51b815260206004820152603460248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a2064656044820152730e0dee6d2e840ecc2d8eaca40e8dede40d0d2ced60631b606482015260840161072b565b6123e68482613671565b949350505050565b61242c6040518060400160405280601781526020017f616476616e636545706f63685f4e6f576974686472617700000000000000000081525061272d565b6110f86133ba565b5f611fa96124418361159f565b83613a0e565b5f60218264ffffffffff168154811061246257612462614b17565b5f918252602090912060049091020160030154600160801b90046001600160401b031692915050565b5f6124ba6040518060400160405280600d81526020016c32bc34ba2b30b634b230ba37b960991b81525061272d565b5f60218364ffffffffff16815481106124d5576124d5614b17565b5f9182526020909120600490910201805490915060ff16156125095760405162461bcd60e51b815260040161072b90614b2b565b6003810154600160801b90046001600160401b039081161461257f5760405162461bcd60e51b815260206004820152602960248201527f426561636f6e436861696e4d6f636b3a2076616c696461746f7220616c726561604482015268191e48195e1a5d195960ba1b606482015260840161072b565b61258761140e565b612592906001614bb8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055506125c3836127bd565b91506125cf835f6127c7565b5f61268860218564ffffffffff16815481106125ed576125ed614b17565b905f5260205f209060040201600201805461260790614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461263390614bf6565b801561267e5780601f106126555761010080835404028352916020019161267e565b820191905f5260205f20905b81548152906001019060200180831161266157829003601f168201915b5050505050613a98565b601f5490915061010090046001600160a01b031663c88a5e6d826126b9633b9aca006001600160401b038816614c4e565b6126cd906001600160a01b03861631614c3b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044015f604051808303815f87803b158015612710575f5ffd5b505af1158015612722573d5f5f3e3d5ffd5b505050505050919050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f5061277e61277960408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b602082015290565b613ab3565b61278783613adc565b604051602001612798929190614d18565b60408051601f19818403018152908290526127b2916146ed565b60405180910390a150565b5f611fa982612434565b5f6022816127d6600486614cae565b64ffffffffff1664ffffffffff1681526020019081526020015f205490506127ff818484613b04565b90508060225f612810600487614cae565b64ffffffffff16815260208101919091526040015f2055505050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50816040516127b291906146ed565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8828260405161288c929190614d40565b60405180910390a15050565b5f6128a5600c6020614c65565b6128b0836001614bb8565b6128ba9190614c65565b601f54611fa99190600160a81b90046001600160401b0316614bb8565b6021546060905f906001600160401b038111156128f6576128f661439c565b60405190808252806020026020018201604052801561291f578160200160208202803683370190505b5090505f5b6021548110156107ed5761293f61293a82613b75565b613df6565b82828151811061295157612951614b17565b6020908102919091010152600101612924565b5f805b83811015612b30575f6002865160016129809190614c3b565b61298a9190614c9b565b90505f816001600160401b038111156129a5576129a561439c565b6040519080825280602002602001820160405280156129ce578160200160208202803683370190505b5090505f5b82811015612b24575f6129e7826002614c4e565b90505f6129f5826001614c3b565b90505f8a8381518110612a0a57612a0a614b17565b602002602001015190505f8b51831015612a3f578b8381518110612a3057612a30614b17565b60200260200101519050612a4b565b612a4888614086565b90505b5f60028383604051602001612a6a929190918252602082015260400190565b60408051601f1981840301815290829052612a8491614d61565b602060405180830381855afa158015612a9f573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190612ac29190614cd7565b905080878781518110612ad757612ad7614b17565b6020908102919091018101919091525f8481528c825260408082208590558482528082208690559481526001808e0190925284812083905592835292909120559290920191506129d39050565b50955050600101612967565b508351600114612ba15760405162461bcd60e51b815260206004820152603660248201527f426561636f6e436861696e4d6f636b2e5f6275696c644d65726b6c65547265656044820152753a20696e76616c6964207472656520736f6d65686f7760501b606482015260840161072b565b835f81518110612bb357612bb3614b17565b602002602001015190509392505050565b60605f612bcf6140fa565b6001600160401b03811115612be657612be661439c565b604051908082528060200260200182016040528015612c0f578160200160208202803683370190505b5090505f5b81518110156107ed5764ffffffffff81165f908152602260205260409020548251839083908110612c4757612c47614b17565b6020908102919091010152600101612c14565b60408051602080825261042082019092526060915f9190808201610400803683370190505090505f5b8151811015612cbe57612c97816001614c3b565b5f1b828281518110612cab57612cab614b17565b6020908102919091010152600101612c83565b508381600b81518110612cd357612cd3614b17565b6020026020010181815250508281600c81518110612cf357612cf3614b17565b60209081029190910101529392505050565b60408051600580825260c082019092526060915f91906020820160a0803683370190505090505f5b8151811015612d6857612d41816001614c3b565b5f1b828281518110612d5557612d55614b17565b6020908102919091010152600101612d2d565b508281600381518110612d7d57612d7d614b17565b602090810291909101015292915050565b7f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358382612db983614130565b60405161288c929190614d6c565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612e0057612e0061439c565b6040519080825280601f01601f191660200182016040528015612e2a576020820181803683370190505b509050815f805b6003811015612e90576025546001600160401b03165f908152602b6020908152604080832086845260068101835281842054858402890184018190529684526007019091529020549282612e8481614d90565b93505050600101612e31565b5060408051808201825285815260208082018681526025546001600160401b03165f90815260269092529290208151815591519091906001820190612ed59082614dec565b50505050505050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612f1757612f1761439c565b6040519080825280601f01601f191660200182016040528015612f41576020820181803683370190505b509050815f612f7160207f0000000000000000000000000000000000000000000000000000000000000000614c9b565b90505f805b6005811015612fd5576025546001600160401b03165f908152602b60209081526040808320878452600481018352818420548584028a0184018190529784526005019091529020549382612fc981614d90565b93505050600101612f76565b50805b82811015613036576025546001600160401b03165f908152602b60209081526040808320878452600681018352818420548584028a018401819052978452600701909152902054938261302a81614d90565b93505050600101612fd8565b5060408051808201825286815260208082018781526025546001600160401b03165f9081526027909252929020815181559151909190600182019061307b9082614dec565b5050505050505050565b6025546001600160401b03165f908152602860205260408120905b602154811015613262575f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b038111156130e3576130e361439c565b6040519080825280601f01601f19166020018201604052801561310d576020820181803683370190505b5090505f61311a83613b75565b90505f61312682613df6565b90505f805b61313760286001614c3b565b811015613191576025546001600160401b03165f908152602b60209081526040808320868452808352818420548584028a018401819052968452600101909152902054928261318581614d90565b9350505060010161312b565b50805b60056131a260286001614c3b565b6131ac9190614c3b565b811015613209576025546001600160401b03165f908152602b60209081526040808320868452600481018352818420548584028a01840181905296845260050190915290205492826131fd81614d90565b93505050600101613194565b5064ffffffffff85165f90815260208781526040909120845161322e9286019061433f565b5064ffffffffff85165f9081526020879052604090206001016132518582614dec565b5050600190930192506130a0915050565b5050565b6025546001600160401b03165f908152602960205260408120906132886140fa565b90505f5b818110156133b5575f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b038111156132cd576132cd61439c565b6040519080825280601f01601f1916602001820160405280156132f7576020820181803683370190505b5064ffffffffff83165f908152602260205260408120549192508190805b61332160266001614c3b565b81101561337e576025546001600160401b03165f908152602b60209081526040808320868452600281018352818420548584028a018401819052968452600301909152902054928261337281614d90565b93505050600101613315565b5064ffffffffff85165f9081526020889052604090208381556001016133a48582614dec565b50506001909301925061328c915050565b505050565b5f805b60215481101561345e575f602182815481106133db576133db614b17565b5f9182526020909120600490910201805490915060ff16156133fd5750613456565b600381015467fffffffffffffffe19600160801b9091046001600160401b031601613454575f61342c836127bd565b9050613439600182614bb8565b90508361344581614d90565b94505061345283826127c7565b505b505b6001016133bd565b50610d94604051806060016040528060268152602001615050602691398261285b565b5f805b60215481101561361f575f602182815481106134a2576134a2614b17565b5f9182526020909120600490910201805490915060ff16156134c45750613617565b5f633b9aca006134d3846127bd565b6001600160401b03166134e69190614c4e565b90505f6134fb83600201805461260790614bf6565b90505f8061350d633b9aca0085614c9b565b6003860154909150600160801b90046001600160401b039081161461354757835f0361353d575050505050613617565b508290505f613576565b6801bc16d674ec8000008411156135765761356b6801bc16d674ec80000085614c28565b915064077359400090505b601f546001600160a01b0361010090910481169063c88a5e6d9085906135a0908690831631614c3b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044015f604051808303815f87803b1580156135e3575f5ffd5b505af11580156135f5573d5f5f3e3d5ffd5b5050505081876136059190614c3b565b965061361186826127c7565b50505050505b600101613484565b508015610d9457610d946040518060400160405280601981526020017f2d207769746864726577206578636573732062616c616e6365000000000000008152508261285b565b5f611fa9600483614cae565b6021545f90613681600482614ea6565b64ffffffffff165f03613862576021545f906136a4906001600160401b03614bd7565b604080516030808252606082019092529192505f919060208201818036833701905050905082816030015260216040518060e001604052806001151581526020015f151581526020016002845f60801b604051602001613705929190614ecf565b60408051601f198184030181529082905261371f91614d61565b602060405180830381855afa15801561373a573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061375d9190614cd7565b815260408051602080820183525f808352818501929092526001600160401b0388811684860152606080860182905260809095015285546001808201885596835291819020855160049093020180549186015161ffff1990921692151561ff0019169290921761010091151591909102178155908301519381019390935581015190919060028201906137f09082614dec565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b03199093169190941617179290921617905561385283836127c7565b8261385c81614ef3565b93505050505b604080516030808252606082019092525f9160208201818036833701905050905081816030015260216040518060e001604052805f151581526020015f151581526020016002845f60801b6040516020016138be929190614ecf565b60408051601f19818403018152908290526138d891614d61565b602060405180830381855afa1580156138f3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906139169190614cd7565b8152602001878152602001866001600160401b0316815260200161393861140e565b6001600160401b0390811682526020918201528254600181810185555f94855293829020835160049092020180549284015115156101000261ff00199215159290921661ffff1990931692909217178155604082015192810192909255606081015190919060028201906139ac9082614dec565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990931691909416171792909216179055611fa582856127c7565b5f80613a1b600484614ea6565b613a26906040614f19565b64ffffffffff1690506123e684821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161760ff60381b60889290921c919091161790565b5f80613aa383614f39565b6001600160a01b03169392505050565b6060611fa9604051806040016040528060058152602001641b5b39366d60d81b815250836141b0565b6060611fa9604051806040016040528060048152602001631b5b336d60e01b815250836141b0565b5f80613b11600485614ea6565b613b1c906001614f5c565b613b27906040614f19565b613b3390610100614f79565b64ffffffffff1690506001600160401b03811b198581165f613b54866141fa565b90505f613b628560c0614c28565b9190911c91909117979650505050505050565b60408051600880825261012082019092526060915f919060208201610100803683370190505090505f60218464ffffffffff1681548110613bb857613bb8614b17565b5f9182526020918290206040805160e0810182526004909302909101805460ff8082161515855261010090910416151593830193909352600183015490820152600282018054919291606084019190613c1090614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054613c3c90614bf6565b8015613c875780601f10613c5e57610100808354040283529160200191613c87565b820191905f5260205f20905b815481529060010190602001808311613c6a57829003601f168201915b5050509183525050600391909101546001600160401b038082166020840152600160401b82048116604080850191909152600160801b9092041660609092019190915281015183519192509083905f90613ce357613ce3614b17565b6020026020010181815250508060600151613cfd90614f39565b82600181518110613d1057613d10614b17565b602002602001018181525050613d2981608001516141fa565b82600281518110613d3c57613d3c614b17565b6020026020010181815250508060200151604051602001613d61911515815260200190565b604051602081830303815290604052613d7990614f39565b82600381518110613d8c57613d8c614b17565b602002602001018181525050613da58160a001516141fa565b82600581518110613db857613db8614b17565b602002602001018181525050613dd18160c001516141fa565b82600681518110613de457613de4614b17565b60209081029190910101525092915050565b5f5f60028351613e069190614c9b565b90505f816001600160401b03811115613e2157613e2161439c565b604051908082528060200260200182016040528015613e4a578160200160208202803683370190505b5090505f5b82811015613f4457600285613e648383614c4e565b81518110613e7457613e74614b17565b602002602001015186836002613e8a9190614c4e565b613e95906001614c3b565b81518110613ea557613ea5614b17565b6020026020010151604051602001613ec7929190918252602082015260400190565b60408051601f1981840301815290829052613ee191614d61565b602060405180830381855afa158015613efc573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190613f1f9190614cd7565b828281518110613f3157613f31614b17565b6020908102919091010152600101613e4f565b50613f50600283614c9b565b91505b8115614063575f5b8281101561405057600282613f708383614c4e565b81518110613f8057613f80614b17565b602002602001015183836002613f969190614c4e565b613fa1906001614c3b565b81518110613fb157613fb1614b17565b6020026020010151604051602001613fd3929190918252602082015260400190565b60408051601f1981840301815290829052613fed91614d61565b602060405180830381855afa158015614008573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061402b9190614cd7565b82828151811061403d5761403d614b17565b6020908102919091010152600101613f5b565b5061405c600283614c9b565b9150613f53565b805f8151811061407557614075614b17565b602002602001015192505050919050565b5f606482106140d75760405162461bcd60e51b815260206004820152601b60248201527f5f6765745a65726f4e6f64653a20696e76616c69642064657074680000000000604482015260640161072b565b602a82815481106140ea576140ea614b17565b905f5260205f2001549050919050565b6021545f901561412b5760215460049061411690600190614c28565b6141209190614c9b565b6114cf906001614c3b565b505f90565b604051631623433d60e31b815260048101829052606090611fa990737109709ecfa91a80626ff3989d68f67f5b1dd12d9063b11a19e8906024015f60405180830381865afa158015614184573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526141ab9190810190614f96565b614270565b60608282604051806040016040528060048152602001631b5b306d60e01b8152506040516020016141e39392919061500a565b604051602081830303815290604052905092915050565b603881811b60ff60381b16602883811b66ff0000000000001691909117601884811b65ff00000000001691909117600885811b64ff00000000169190911763ff0000009186901c919091161762ff00009185901c919091161761ff009184901c919091161760ff9290911c919091161760c01b90565b6060611fa9604051806040016040528060048152602001631b5b326d60e01b815250836141b0565b60405180606001604052805f6001600160401b031681526020016142ce60405180604001604052805f8152602001606081525090565b81526020016142f0604051806040016040528060608152602001606081525090565b905290565b60405180608001604052805f6001600160401b0316815260200161432b60405180604001604052805f8152602001606081525090565b815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614378579160200282015b8281111561437857825182559160200191906001019061435d565b50614384929150614388565b5090565b5b80821115614384575f8155600101614389565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156143d8576143d861439c565b604052919050565b803564ffffffffff811681146143f4575f5ffd5b919050565b5f82601f830112614408575f5ffd5b81356001600160401b038111156144215761442161439c565b8060051b614431602082016143b0565b9182526020818501810192908101908684111561444c575f5ffd5b6020860192505b8383101561447557614464836143e0565b825260209283019290910190614453565b9695505050505050565b5f6020828403121561448f575f5ffd5b81356001600160401b038111156144a4575f5ffd5b6123e6848285016143f9565b602080825282518282018190525f918401906040840190835b818110156144f05783516001600160a01b03168352602093840193909201916001016144c9565b509095945050505050565b5f6020828403121561450b575f5ffd5b614514826143e0565b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b805182525f6020820151604060208501526123e6604085018261451b565b5f8151808452602084019350602083015f5b82811015614597578151865260209586019590910190600101614579565b5093949350505050565b602081526001600160401b0382511660208201525f6020830151606060408401526145cf6080840182614549565b90506040840151601f198483030160608501528051604083526145f56040840182614567565b9050602082015191508281036020840152614475818361451b565b5f82825180855260208501945060208160051b830101602085015f5b8381101561465e57601f1985840301885261464883835161451b565b602098890198909350919091019060010161462c565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157868503603f19018452815180516001600160a01b031686526020908101516040918701829052906146cb90870182614610565b9550506020938401939190910190600101614690565b50929695505050505050565b602081525f614514602083018461451b565b5f8151808452602084019350602083015f5b828110156145975781516001600160e01b031916865260209586019590910190600101614711565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157603f198786030184528151805160408752614785604088018261451b565b90506020820151915086810360208801526147a081836146ff565b96505050602093840193919091019060010161475f565b602081525f6145146020830184614610565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157868503603f19018452815180516001600160a01b0316865260209081015160409187018290529061482a908701826146ff565b95505060209384019391909101906001016147ef565b5f82825180855260208501945060208160051b830101602085015f5b8381101561465e57601f19858403018852614878838351614567565b602098890198909350919091019060010161485c565b602081526001600160401b0382511660208201525f6020830151608060408401526148bc60a0840182614549565b6040850151848203601f19016060860152805180835291925060209081019181840191600582901b8501015f5b8281101561491a57601f1986830301845261490582865161451b565b602095860195949094019391506001016148e9565b506060880151878203601f1901608089015294506149388186614840565b98975050505050505050565b5f5f60408385031215614955575f5ffd5b82356001600160401b0381111561496a575f5ffd5b614976858286016143f9565b92505060208301356001600160401b0381168114614992575f5ffd5b809150509250929050565b602081525f8251604060208401526149b86060840182614549565b602085810151858303601f19016040870152805180845292935081019181840191600582901b8501015f5b82811015614a3557601f19868303018452845180518352602081015160208401526040810151905060606040840152614a1f606084018261451b565b60209687019695909501949250506001016149e3565b50979650505050505050565b602080825282518282018190525f918401906040840190835b818110156144f0578351835260209384019390920191600101614a5a565b5f6001600160401b03821115614a9057614a9061439c565b50601f01601f191660200190565b5f60208284031215614aae575f5ffd5b81356001600160401b03811115614ac3575f5ffd5b8201601f81018413614ad3575f5ffd5b8035614ae6614ae182614a78565b6143b0565b818152856020838501011115614afa575f5ffd5b816020840160208301375f91810160200191909152949350505050565b634e487b7160e01b5f52603260045260245ffd5b60208082526053908201527f426561636f6e436861696e4d6f636b3a20617474656d7074696e6720746f206560408201527f7869742064756d6d792076616c696461746f722e205765206e6565642074686f6060820152720e6ca40ccdee440e0e4dedecccecadc407c745606b1b608082015260a00190565b634e487b7160e01b5f52601160045260245ffd5b6001600160401b038181168382160190811115611fa957611fa9614ba4565b6001600160401b038281168282160390811115611fa957611fa9614ba4565b600181811c90821680614c0a57607f821691505b6020821081036107ed57634e487b7160e01b5f52602260045260245ffd5b81810381811115611fa957611fa9614ba4565b80820180821115611fa957611fa9614ba4565b8082028115828204841417611fa957611fa9614ba4565b6001600160401b03818116838216029081169081811461140757611407614ba4565b634e487b7160e01b5f52601260045260245ffd5b5f82614ca957614ca9614c87565b500490565b5f64ffffffffff831680614cc457614cc4614c87565b8064ffffffffff84160491505092915050565b5f60208284031215614ce7575f5ffd5b5051919050565b5f82614cfc57614cfc614c87565b500690565b5f81518060208401855e5f93019283525090919050565b5f614d238285614d01565b601760f91b8152614d376001820185614d01565b95945050505050565b604081525f614d52604083018561451b565b90508260208301529392505050565b5f6145148284614d01565b604081525f614d7e604083018561451b565b8281036020840152614d37818561451b565b5f60018201614da157614da1614ba4565b5060010190565b601f8211156133b557805f5260205f20601f840160051c81016020851015614dcd5750805b601f840160051c820191505b81811015610f3b575f8155600101614dd9565b81516001600160401b03811115614e0557614e0561439c565b614e1981614e138454614bf6565b84614da8565b6020601f821160018114614e4b575f8315614e345750848201515b5f19600385901b1c1916600184901b178455610f3b565b5f84815260208120601f198516915b82811015614e7a5787850151825560209485019460019092019101614e5a565b5084821015614e9757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f64ffffffffff831680614ebc57614ebc614c87565b8064ffffffffff84160691505092915050565b5f614eda8285614d01565b6001600160801b03199390931683525050601001919050565b5f64ffffffffff821664ffffffffff8103614f1057614f10614ba4565b60010192915050565b64ffffffffff818116838216029081169081811461140757611407614ba4565b805160208083015191908110156107ed575f1960209190910360031b1b16919050565b64ffffffffff8181168382160190811115611fa957611fa9614ba4565b64ffffffffff8281168282160390811115611fa957611fa9614ba4565b5f60208284031215614fa6575f5ffd5b81516001600160401b03811115614fbb575f5ffd5b8201601f81018413614fcb575f5ffd5b8051614fd9614ae182614a78565b818152856020838501011115614fed575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f614d3761502161501b8488614d01565b86614d01565b84614d0156fe2d2d206e6f2076616c696461746f72733b20616464656420656d70747920626c6f636b20726f6f742d2067656e657261746564207265776172647320666f72206e756d2076616c696461746f7273a2646970667358221220c5f3579a73425634af0d920492c2ddf15e5098d83ab611e71a392fce4e0bdf5464736f6c634300081b0033608060405234801561000f575f5ffd5b5060043610610034575f3560e01c8063643599f2146101a4578063acd414a8146101df575b6020361461009d5760405162461bcd60e51b815260206004820152602b60248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206d616c666f726d60448201526a6564206d73672e6461746160a81b60648201526084015b60405180910390fd5b5f6100a8368261020b565b9050805f036101095760405162461bcd60e51b815260206004820152602760248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a2074696d6573746160448201526606d7020697320360cc1b6064820152608401610094565b5f818152602081905260408120549081900361019c5760405162461bcd60e51b815260206004820152604660248201527f343738384f7261636c654d6f636b2e66616c6c6261636b3a206e6f20626c6f6360448201527f6b20726f6f7420666f756e642e2044494420594f5520555345204348454154536064820152652e574152503f60d01b608482015260a401610094565b805f5260205ff35b6101cd6101b236600461020b565b67ffffffffffffffff165f9081526020819052604090205490565b60405190815260200160405180910390f35b6102096101ed366004610222565b67ffffffffffffffff9091165f90815260208190526040902055565b005b5f6020828403121561021b575f5ffd5b5035919050565b5f5f60408385031215610233575f5ffd5b823567ffffffffffffffff8116811461024a575f5ffd5b94602093909301359350505056fea2646970667358221220e3b4eaf33a45b229cee6df28c29bb7c42a1a424389218afa08d0fbdc8cb5dd5e64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R`\x0C\x80T`\xFF\x19\x16`\x01\x17\x90U`\x1F\x80Ttq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x01`\x01`\x01`\xA8\x1B\x03\x19\x90\x91\x16\x17\x90Ua\0G`\x03` a\x06\x0EV[`\x80R`\x05a\0X`(`\x01a\x06+V[a\0b\x91\x90a\x06+V[a\0m\x90` a\x06\x0EV[`\xA0Ra\0|`\x05`\x03a\x06+V[a\0\x87\x90` a\x06\x0EV[`\xC0Ra\0\x96`&`\x01a\x06+V[a\0\xA1\x90` a\x06\x0EV[`\xE0R4\x80\x15a\0\xAFW__\xFD[P`@QaZ\xB78\x03\x80aZ\xB7\x839\x81\x01`@\x81\x90Ra\0\xCE\x91a\x06>V[`\x1F\x80T`\x01`\xA8\x1B`\x01`\xE8\x1B\x03\x19\x16`\x01`\xA8\x1B`\x01`\x01`@\x1B\x03\x84\x16\x02\x17\x90\x81\x90U` \x80T`\x01`@\x1B`\x01`\xE0\x1B\x03\x19\x16h\x01\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x90\x91\x02\x91\x90\x91\x17\x82U`@Qa\x01\0\x90\x93\x04\x16\x91c\xB4\xD6\xC7\x82\x91r\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x91\x90a\x01V\x90\x82\x01a\x05\x90V[` \x82\x01\x81\x03\x82R`\x1F\x19`\x1F\x82\x01\x16`@RP`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x87\x92\x91\x90a\x06\x8CV[_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x01\x9EW__\xFD[PZ\xF1\x15\x80\x15a\x01\xB0W=__>=_\xFD[PP`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R_\x93Pa\x01\xE0\x92P\x90` \x82\x01a\x01\0\x806\x837\x01\x90PPa\x03\0V[`@\x80Q`d\x80\x82Ra\x0C\xA0\x82\x01\x90\x92R\x91\x92P` \x82\x01a\x0C\x80\x806\x837PP\x81Qa\x02\x14\x92`*\x92P` \x01\x90a\x05\x9DV[P\x80`*_\x81T\x81\x10a\x02)Wa\x02)a\x06\xE4V[_\x91\x82R` \x90\x91 \x01U`\x01[`*T\x81\x10\x15a\x02\xF7W`@\x80Q` \x81\x01\x84\x90R\x90\x81\x01\x83\x90R`\x02\x90``\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x02r\x91a\x06\xF8V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x02\x8DW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xB0\x91\x90a\x07\x0EV[`*\x82\x81T\x81\x10a\x02\xC3Wa\x02\xC3a\x06\xE4V[\x90_R` _ \x01\x81\x90UP`*\x81\x81T\x81\x10a\x02\xE2Wa\x02\xE2a\x06\xE4V[_\x91\x82R` \x90\x91 \x01T\x91P`\x01\x01a\x027V[PPPPa\x07DV[__`\x02\x83Qa\x03\x10\x91\x90a\x07%V[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x03+Wa\x03+a\x06\xD0V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x03TW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a\x04NW`\x02\x85a\x03n\x83\x83a\x06\x0EV[\x81Q\x81\x10a\x03~Wa\x03~a\x06\xE4V[` \x02` \x01\x01Q\x86\x83`\x02a\x03\x94\x91\x90a\x06\x0EV[a\x03\x9F\x90`\x01a\x06+V[\x81Q\x81\x10a\x03\xAFWa\x03\xAFa\x06\xE4V[` \x02` \x01\x01Q`@Q` \x01a\x03\xD1\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x03\xEB\x91a\x06\xF8V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x04\x06W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04)\x91\x90a\x07\x0EV[\x82\x82\x81Q\x81\x10a\x04;Wa\x04;a\x06\xE4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x03YV[Pa\x04Z`\x02\x83a\x07%V[\x91P[\x81\x15a\x05mW_[\x82\x81\x10\x15a\x05ZW`\x02\x82a\x04z\x83\x83a\x06\x0EV[\x81Q\x81\x10a\x04\x8AWa\x04\x8Aa\x06\xE4V[` \x02` \x01\x01Q\x83\x83`\x02a\x04\xA0\x91\x90a\x06\x0EV[a\x04\xAB\x90`\x01a\x06+V[\x81Q\x81\x10a\x04\xBBWa\x04\xBBa\x06\xE4V[` \x02` \x01\x01Q`@Q` \x01a\x04\xDD\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x04\xF7\x91a\x06\xF8V[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a\x05\x12W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x055\x91\x90a\x07\x0EV[\x82\x82\x81Q\x81\x10a\x05GWa\x05Ga\x06\xE4V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x04eV[Pa\x05f`\x02\x83a\x07%V[\x91Pa\x04]V[\x80_\x81Q\x81\x10a\x05\x7FWa\x05\x7Fa\x06\xE4V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[a\x02\x8E\x80aX)\x839\x01\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15a\x05\xD6W\x91` \x02\x82\x01[\x82\x81\x11\x15a\x05\xD6W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a\x05\xBBV[Pa\x05\xE2\x92\x91Pa\x05\xE6V[P\x90V[[\x80\x82\x11\x15a\x05\xE2W_\x81U`\x01\x01a\x05\xE7V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x06%Wa\x06%a\x05\xFAV[\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x06%Wa\x06%a\x05\xFAV[__`@\x83\x85\x03\x12\x15a\x06OW__\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06eW__\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x81\x16\x81\x14a\x06\x81W__\xFD[\x80\x91PP\x92P\x92\x90PV[`\x01\x80`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R_\x82Q\x80`@\x84\x01R\x80` \x85\x01``\x85\x01^_``\x82\x85\x01\x01R```\x1F\x19`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x93\x92PPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_\x82Q\x80` \x85\x01\x84^_\x92\x01\x91\x82RP\x91\x90PV[_` \x82\x84\x03\x12\x15a\x07\x1EW__\xFD[PQ\x91\x90PV[_\x82a\x07?WcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x80Q`\xA0Q`\xC0Q`\xE0QaP\xABa\x07~_9_a2\x97\x01R_\x81\x81a.\xE1\x01Ra/M\x01R_a0\xAD\x01R_a-\xCA\x01RaP\xAB_\xF3\xFE`\x80`@R`\x046\x10a\x02\x12W_5`\xE0\x1C\x80c\x86\xA6\xF9\xE1\x11a\x01\x1EW\x80c\xBAAO\xA6\x11a\0\xA8W\x80c\xF0\xAC\xD9\x88\x11a\0mW\x80c\xF0\xAC\xD9\x88\x14a\x05\xF9W\x80c\xF7!8s\x14a\x06\rW\x80c\xF83\xEBc\x14a\x06,W\x80c\xF8\xF9\x8AN\x14a\x06KW\x80c\xFAv&\xD4\x14a\x06jW__\xFD[\x80c\xBAAO\xA6\x14a\x05hW\x80c\xC7o%\xC0\x14a\x05|W\x80c\xE2\x0C\x9Fq\x14a\x05\xA8W\x80c\xE3\xCE\xFBB\x14a\x05\xBCW\x80c\xED<\x16\x05\x14a\x05\xD0W__\xFD[\x80c\xA5\n:\x1A\x11a\0\xEEW\x80c\xA5\n:\x1A\x14a\x04\xB9W\x80c\xAAG8\x9C\x14a\x04\xE5W\x80c\xB0FO\xDC\x14a\x05\x14W\x80c\xB1\xB6\xF6\xA1\x14a\x05(W\x80c\xB5P\x8A\xA9\x14a\x05TW__\xFD[\x80c\x86\xA6\xF9\xE1\x14a\x03\x03W\x80c\x90\x88 \xE0\x14a\x04FW\x80c\x91j\x17\xC6\x14a\x04eW\x80c\xA3\xF4\xDF~\x14a\x04\x86W__\xFD[\x80c<\xF8\x0El\x11a\x01\x9FW\x80c^l\xC2\xFC\x11a\x01oW\x80c^l\xC2\xFC\x14a\x03\xA5W\x80cf\xD9\xA9\xA0\x14a\x03\xD1W\x80ck:\xBD\x97\x14a\x03\xF2W\x80cvg\x18\x08\x14a\x04\x11W\x80c\x85\"l\x81\x14a\x04%W__\xFD[\x80c<\xF8\x0El\x14a\x03UW\x80c>^<#\x14a\x03iW\x80c?r\x86\xF4\x14a\x03}W\x80cY\xD0\x95\xDD\x14a\x03\x91W__\xFD[\x80c)\x99/\xAA\x11a\x01\xE5W\x80c)\x99/\xAA\x14a\x02\xCCW\x80c*\xDE8\x80\x14a\x02\xE2W\x80c-\xEF`\t\x14a\x03\x03W\x80c3\x0B\xC2~\x14a\x03\"W\x80c5~\x95\x1F\x14a\x036W__\xFD[\x80c\x146\tX\x14a\x02\x16W\x80c\x1E\xD7\x83\x1C\x14a\x02RW\x80c\x1FT6\\\x14a\x02sW\x80c#\xE8,L\x14a\x02\xA0W[__\xFD[4\x80\x15a\x02!W__\xFD[Pa\x025a\x0206`\x04aD\x7FV[a\x06\x83V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02]W__\xFD[Pa\x02fa\x07\xF3V[`@Qa\x02I\x91\x90aD\xB0V[4\x80\x15a\x02~W__\xFD[Pa\x02\x92a\x02\x8D6`\x04aD\xFBV[a\x08SV[`@Q\x90\x81R` \x01a\x02IV[4\x80\x15a\x02\xABW__\xFD[Pa\x02\xBFa\x02\xBA6`\x04aD\xFBV[a\x08\x85V[`@Qa\x02I\x91\x90aE\xA1V[4\x80\x15a\x02\xD7W__\xFD[Pa\x02\xE0a\n\x97V[\0[4\x80\x15a\x02\xEDW__\xFD[Pa\x02\xF6a\x0FBV[`@Qa\x02I\x91\x90aFjV[4\x80\x15a\x03\x0EW__\xFD[Pa\x025a\x03\x1D6`\x04aD\xFBV[a\x10~V[4\x80\x15a\x03-W__\xFD[Pa\x025`\n\x81V[4\x80\x15a\x03AW__\xFD[P` Ta\x025\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03`W__\xFD[Pa\x02\xE0a\x10\xBBV[4\x80\x15a\x03tW__\xFD[Pa\x02fa\x11\x02V[4\x80\x15a\x03\x88W__\xFD[Pa\x02fa\x11`V[4\x80\x15a\x03\x9CW__\xFD[Pa\x02\xE0a\x11\xBEV[4\x80\x15a\x03\xB0W__\xFD[Pa\x03\xC4a\x03\xBF6`\x04aD\xFBV[a\x11\xF5V[`@Qa\x02I\x91\x90aF\xEDV[4\x80\x15a\x03\xDCW__\xFD[Pa\x03\xE5a\x12$V[`@Qa\x02I\x91\x90aG9V[4\x80\x15a\x03\xFDW__\xFD[Pa\x02\x92a\x04\x0C6`\x04aD\x7FV[a\x13\x88V[4\x80\x15a\x04\x1CW__\xFD[Pa\x025a\x14\x0EV[4\x80\x15a\x040W__\xFD[Pa\x049a\x14\xD4V[`@Qa\x02I\x91\x90aG\xB7V[4\x80\x15a\x04QW__\xFD[Pa\x02\x92a\x04`6`\x04aD\xFBV[a\x15\x9FV[4\x80\x15a\x04pW__\xFD[Pa\x04ya\x15\xCBV[`@Qa\x02I\x91\x90aG\xC9V[4\x80\x15a\x04\x91W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01Ra\x03\xC4V[4\x80\x15a\x04\xC4W__\xFD[Pa\x04\xD8a\x04\xD36`\x04aD\x7FV[a\x16\xACV[`@Qa\x02I\x91\x90aH\x8EV[4\x80\x15a\x04\xF0W__\xFD[Pa\x05\x04a\x04\xFF6`\x04aD\xFBV[a\x1A\xC5V[`@Q\x90\x15\x15\x81R` \x01a\x02IV[4\x80\x15a\x05\x1FW__\xFD[Pa\x04ya\x1B\x14V[4\x80\x15a\x053W__\xFD[Pa\x05Ga\x05B6`\x04aIDV[a\x1B\xF5V[`@Qa\x02I\x91\x90aI\x9DV[4\x80\x15a\x05_W__\xFD[Pa\x049a\x1F\xAFV[4\x80\x15a\x05sW__\xFD[Pa\x05\x04a zV[4\x80\x15a\x05\x87W__\xFD[Pa\x05\x9Ba\x05\x966`\x04aD\x7FV[a!\x1AV[`@Qa\x02I\x91\x90aJAV[4\x80\x15a\x05\xB3W__\xFD[Pa\x02fa!\xD5V[4\x80\x15a\x05\xC7W__\xFD[Pa\x025`\x01\x81V[a\x05\xE3a\x05\xDE6`\x04aJ\x9EV[a\"3V[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02IV[4\x80\x15a\x06\x04W__\xFD[Pa\x02\xE0a#\xEEV[4\x80\x15a\x06\x18W__\xFD[Pa\x025a\x06'6`\x04aD\xFBV[a$4V[4\x80\x15a\x067W__\xFD[Pa\x025a\x06F6`\x04aD\xFBV[a$GV[4\x80\x15a\x06VW__\xFD[Pa\x025a\x06e6`\x04aD\xFBV[a$\x8BV[4\x80\x15a\x06uW__\xFD[P`\x1FTa\x05\x04\x90`\xFF\x16\x81V[_a\x06\xB4`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nslashValidators`\x88\x1B\x81RPa'-V[_[\x82Q\x81\x10\x15a\x07\xEDW_\x83\x82\x81Q\x81\x10a\x06\xD2Wa\x06\xD2aK\x17V[` \x02` \x01\x01Q\x90P_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06\xF7Wa\x06\xF7aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\x074W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07+\x90aK+V[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\xFF\x16a\x07\x8BW\x80Ta\xFF\0\x19\x16a\x01\0\x17\x81Ua\x07Wa\x14\x0EV[a\x07b\x90`\x01aK\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[_a\x07\x95\x83a'\xBDV[\x90P`\x01`\x01`@\x1B\x03\x81\x16`\n\x11\x15a\x07\xBDWa\x07\xB3\x81\x86aK\xB8V[\x94P_\x90Pa\x07\xD8V[a\x07\xC8`\n\x86aK\xB8V[\x94Pa\x07\xD5`\n\x82aK\xD7V[\x90P[a\x07\xE2\x83\x82a'\xC7V[PPP`\x01\x01a\x06\xB6V[P\x91\x90PV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+W[PPPPP\x90P\x90V[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x08nWa\x08naK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x90P\x91\x90PV[a\x08\x8DaB\x98V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x81T``\x94\x81\x02\x82\x01\x85\x01\x84R\x92\x81\x01\x83\x81R\x90\x93\x91\x92\x84\x92\x84\x91\x90\x84\x01\x82\x82\x80\x15a\t\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xEFW[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\x1C\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tH\x90aK\xF6V[\x80\x15a\t\x93W\x80`\x1F\x10a\tjWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tvW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q``\x81\x01\x82R`%T`\x01`\x01`@\x1B\x03\x16\x80\x82R_\x90\x81R`&` \x90\x81R\x90\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x84R`\x01\x81\x01\x80T\x96\x97P\x92\x95\x82\x87\x01\x95P\x90\x92\x91\x84\x01\x91\x90a\t\xF0\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x1C\x90aK\xF6V[\x80\x15a\ngW\x80`\x1F\x10a\n>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\ngV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nJW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81R` \x93\x84\x01Q\x81\x85\x01R\x92\x01\x91\x90\x91R\x92\x91PPV[_[`!T\x81\x10\x15a\x0B0W_`!\x82\x81T\x81\x10a\n\xB7Wa\n\xB7aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\n\xD9WPa\x0B(V[_a\n\xE3\x83a'\xBDV[\x90Pd\x07sY@\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0B\x03WPd\x07sY@\0[`\x03\x91\x90\x91\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01\x01a\n\x99V[Pa\x0Bo`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F- updated effective balances\0\0\0\0\x81RPa(,V[a\x0B\xB1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xA5\xA4\x0Cn\xAENL\xAD\xCE\x84\x0C\xAE\r\xECm`\x83\x1B\x81RPa\x0B\xA3a\x14\x0EV[`\x01`\x01`@\x1B\x03\x16a([V[_a\x0B\xBAa\x14\x0EV[`\x1FT\x90\x91Pa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xE5\xD6\xBF\x02a\x0B\xDC\x83a(\x98V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x1AW__\xFD[PZ\xF1\x15\x80\x15a\x0C,W=__>=_\xFD[PP`%\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16B`\x01`\x01`@\x1B\x03\x16\x17\x90UPP`@\x80Q\x80\x82\x01\x90\x91R`\x16\x81Ru\x05\xA4\rN\xAD\xAE\x0C\xAC\x84\x0E\x8D\xE4\r\xCC\xAF\x0E\x84\x0C\xAE\r\xECm`S\x1B` \x82\x01Ra\x0C\x85\x90a\x0B\xA3a\x14\x0EV[a\x0C\xC3`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F- building beacon state trees\0\0\0\x81RPa(,V[`!T\x15a\x0C\xE2W`!Ta\x0C\xDA\x90`\x01\x90aL(V[`$Ua\r\x97V[`%T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p`$\x82\x01Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r]W__\xFD[PZ\xF1\x15\x80\x15a\roW=__>=_\xFD[PPPPa\r\x94`@Q\x80``\x01`@R\x80`(\x81R` \x01aP(`(\x919a(,V[PV[_a\r\xCDa\r\xA3a(\xD7V[a\r\xAF`(`\x01aL;V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 a)dV[\x90P_a\x0E\x08a\r\xDBa+\xC4V[a\r\xE7`&`\x01aL;V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x02\x01a)dV[\x90P_a\x0E<a\x0E\x18\x84\x84a,ZV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x05\x90`\x04\x01a)dV[\x90P_a\x0Eoa\x0EK\x83a-\x05V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x03\x90`\x06\x01a)dV[\x90Pa\x0E\xA7`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x0BKH\x18\x99XX\xDB\xDB\x88\x18\x9B\x1B\xD8\xDA\xC8\x1C\x9B\xDB\xDD`b\x1B\x81RP\x82a-\x8EV[`%T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\x03W__\xFD[PZ\xF1\x15\x80\x15a\x0F\x15W=__>=_\xFD[PPPPa\x0F\"\x82a-\xC7V[a\x0F+\x83a.\xDEV[a\x0F3a0\x85V[a\x0F;a2fV[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x10^W\x83\x82\x90_R` _ \x01\x80Ta\x0F\xD3\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFF\x90aK\xF6V[\x80\x15a\x10JW\x80`\x1F\x10a\x10!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10JV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10-W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xB6V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0FeV[PPPP\x90P\x90V[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x10\x99Wa\x10\x99aK\x17V[_\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[a\x10\xE8`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0C,\x8E\xCC-\xCCl\xA8\xAE\r\xECm`\xA3\x1B\x81RPa'-V[a\x10\xF0a3\xBAV[a\x10\xF8a4\x81V[a\x11\0a\n\x97V[V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[a\x10\xF0`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uadvanceEpoch_NoRewards`P\x1B\x81RPa'-V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R_\x91\x90` \x82\x01\x81\x806\x837PPP`0\x81\x01\x93\x90\x93RP\x90\x91\x90PV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x12w\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xA3\x90aK\xF6V[\x80\x15a\x12\xEEW\x80`\x1F\x10a\x12\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xEEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x13pW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x132W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12GV[_\x80\x80[\x83Q\x81\x10\x15a\x14\x07Wc;\x9A\xCA\0`!\x85\x83\x81Q\x81\x10a\x13\xAEWa\x13\xAEaK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x13\xCDWa\x13\xCDaK\x17V[_\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01Ta\x13\xF3\x91\x90`\x01`\x01`@\x1B\x03\x16aLNV[a\x13\xFD\x90\x83aL;V[\x91P`\x01\x01a\x13\x8CV[P\x92\x91PPV[`\x1FT_\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x14\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChain.currentEpoch: curren`D\x82\x01R\x7Ft time is before genesis time\0\0\0`d\x82\x01R`\x84\x01a\x07+V[a\x14\xA4`\x0C` aLeV[`\x1FT`\x01`\x01`@\x1B\x03\x91\x82\x16\x91a\x14\xC5\x91`\x01`\xA8\x1B\x90\x04\x16BaL(V[a\x14\xCF\x91\x90aL\x9BV[\x90P\x90V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x01\x80Ta\x15\x14\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15@\x90aK\xF6V[\x80\x15a\x15\x8BW\x80`\x1F\x10a\x15bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\xF7V[_`\"\x81a\x15\xAE`\x04\x85aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ T\x92\x91PPV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x94W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16VW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\xEEV[a\x16\xB4aB\xF5V[_[\x82Q\x81\x10\x15a\x17\xABW`$T\x83\x82\x81Q\x81\x10a\x16\xD4Wa\x16\xD4aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F validator has not been included`d\x82\x01R\x7F in beacon chain state (DID YOU `\x84\x82\x01R\x7FCALL advanceEpoch YET?)\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x07+V[`\x01\x01a\x16\xB6V[P`@\x80Q`\x80\x81\x01\x82R`%T`\x01`\x01`@\x1B\x03\x16\x80\x82R_\x90\x81R`&` \x90\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01\x80T\x93\x95\x83\x86\x01\x94\x90\x93\x84\x01\x91\x90a\x17\xFA\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18&\x90aK\xF6V[\x80\x15a\x18qW\x80`\x1F\x10a\x18HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18qV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x98Wa\x18\x98aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xCBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xB6W\x90P[P\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xEAWa\x18\xEAaC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x1DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x08W\x90P[P\x90R\x90P_[\x83Q\x81\x10\x15a\x14\x07W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` R`@\x81 \x85Q\x82\x90\x87\x90\x85\x90\x81\x10a\x19\\Wa\x19\\aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x19\xD6W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x19\xC2W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x19\xEF\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x1B\x90aK\xF6V[\x80\x15a\x1AfW\x80`\x1F\x10a\x1A=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1AfV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AIW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q\x83`@\x01Q\x83\x81Q\x81\x10a\x1A\x8CWa\x1A\x8CaK\x17V[` \x02` \x01\x01\x81\x90RP\x80_\x01Q\x83``\x01Q\x83\x81Q\x81\x10a\x1A\xB1Wa\x1A\xB1aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x19$V[_`\x01`\x01`@\x1B\x03\x80\x16`!\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xEAWa\x1A\xEAaK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x14\x92\x91PPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1B\xDDW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1B\x9FW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B7V[a\x1C\x1F`@\x80Q`\x80\x81\x01\x82R_\x91\x81\x01\x91\x82R``\x80\x82\x01R\x90\x81\x90\x81R` \x01``\x81RP\x90V[_[\x83Q\x81\x10\x15a\x1C\xF0W`$T\x84\x82\x81Q\x81\x10a\x1C?Wa\x1C?aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F no checkpoint proof found (did `d\x82\x01R\x7Fyou call advanceEpoch yet?)\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07+V[`\x01\x01a\x1C!V[P`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16_\x90\x81R`'` R\x82\x81 `\x80\x83\x01\x84R\x80T\x93\x83\x01\x93\x84R`\x01\x81\x01\x80T\x92\x94\x84\x93\x90\x92\x91``\x85\x01\x91a\x1D4\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D`\x90aK\xF6V[\x80\x15a\x1D\xABW\x80`\x1F\x10a\x1D\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xABV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\x8EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xD2Wa\x1D\xD2aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E%W\x81` \x01[a\x1E\x12`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xF0W\x90P[P\x90R\x90P_[\x84Q\x81\x10\x15a\x1F\xA5W_\x85\x82\x81Q\x81\x10a\x1EHWa\x1EHaK\x17V[` \x02` \x01\x01Q\x90P_a\x1E\\\x82a6eV[`\x01`\x01`@\x1B\x03\x87\x16_\x90\x81R`)` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01\x80T\x95\x96P\x93\x94\x91\x93\x90\x92\x84\x01\x91a\x1E\xAC\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xD8\x90aK\xF6V[\x80\x15a\x1F#W\x80`\x1F\x10a\x1E\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F#V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80``\x01`@R\x80`!\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1FSWa\x1FSaK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x82_\x01Q\x81R` \x01\x82` \x01Q\x81RP\x85` \x01Q\x85\x81Q\x81\x10a\x1F\x8FWa\x1F\x8FaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x1E,V[P\x90P[\x92\x91PPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x01\x80Ta\x1F\xEF\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1B\x90aK\xF6V[\x80\x15a fW\x80`\x1F\x10a =Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a fV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a IW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xD2V[`\x08T_\x90`\xFF\x16\x15a \x91WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x13\x91\x90aL\xD7V[\x14\x15\x90P\x90V[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!6Wa!6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!_W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x14\x07W`!\x84\x82\x81Q\x81\x10a!\x81Wa!\x81aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\xA0Wa!\xA0aK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x82\x82\x81Q\x81\x10a!\xC2Wa!\xC2aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!dV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[_a\"a`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k72\xBB\xAB0\xB64\xB20\xBA7\xB9`\xA1\x1B\x81RPa'-V[4g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\"\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rrposit value too low`h\x1B`d\x82\x01R`\x84\x01a\x07+V[a\"\xE4c;\x9A\xCA\0\x82aL\xEEV[\x15a#WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FBeaconChainMock.newValidator: va`D\x82\x01R\x7Flue not multiple of gwei\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07+V[_a#fc;\x9A\xCA\0\x83aL\x9BV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rs\x0E\r\xEEm.\x84\x0E\xCC-\x8E\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`c\x1B`d\x82\x01R`\x84\x01a\x07+V[a#\xE6\x84\x82a6qV[\x94\x93PPPPV[a$,`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FadvanceEpoch_NoWithdraw\0\0\0\0\0\0\0\0\0\x81RPa'-V[a\x10\xF8a3\xBAV[_a\x1F\xA9a$A\x83a\x15\x9FV[\x83a:\x0EV[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$bWa$baK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[_a$\xBA`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l2\xBC4\xBA+0\xB64\xB20\xBA7\xB9`\x99\x1B\x81RPa'-V[_`!\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$\xD5Wa$\xD5aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a%\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07+\x90aK+V[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a%\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBeaconChainMock: validator alrea`D\x82\x01Rh\x19\x1EH\x19^\x1A]\x19Y`\xBA\x1B`d\x82\x01R`\x84\x01a\x07+V[a%\x87a\x14\x0EV[a%\x92\x90`\x01aK\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa%\xC3\x83a'\xBDV[\x91Pa%\xCF\x83_a'\xC7V[_a&\x88`!\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%\xEDWa%\xEDaK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x02\x01\x80Ta&\x07\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&3\x90aK\xF6V[\x80\x15a&~W\x80`\x1F\x10a&UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&~V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&aW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:\x98V[`\x1FT\x90\x91Pa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x8A^m\x82a&\xB9c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x88\x16aLNV[a&\xCD\x90`\x01`\x01`\xA0\x1B\x03\x86\x161aL;V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\x10W__\xFD[PZ\xF1\x15\x80\x15a'\"W=__>=_\xFD[PPPPPP\x91\x90PV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPa'~a'y`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01R\x90V[a:\xB3V[a'\x87\x83a:\xDCV[`@Q` \x01a'\x98\x92\x91\x90aM\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xB2\x91aF\xEDV[`@Q\x80\x91\x03\x90\xA1PV[_a\x1F\xA9\x82a$4V[_`\"\x81a'\xD6`\x04\x86aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90Pa'\xFF\x81\x84\x84a;\x04V[\x90P\x80`\"_a(\x10`\x04\x87aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UPPPV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP\x81`@Qa'\xB2\x91\x90aF\xEDV[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa(\x8C\x92\x91\x90aM@V[`@Q\x80\x91\x03\x90\xA1PPV[_a(\xA5`\x0C` aLeV[a(\xB0\x83`\x01aK\xB8V[a(\xBA\x91\x90aLeV[`\x1FTa\x1F\xA9\x91\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aK\xB8V[`!T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xF6Wa(\xF6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[`!T\x81\x10\x15a\x07\xEDWa)?a):\x82a;uV[a=\xF6V[\x82\x82\x81Q\x81\x10a)QWa)QaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a)$V[_\x80[\x83\x81\x10\x15a+0W_`\x02\x86Q`\x01a)\x80\x91\x90aL;V[a)\x8A\x91\x90aL\x9BV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xA5Wa)\xA5aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a+$W_a)\xE7\x82`\x02aLNV[\x90P_a)\xF5\x82`\x01aL;V[\x90P_\x8A\x83\x81Q\x81\x10a*\nWa*\naK\x17V[` \x02` \x01\x01Q\x90P_\x8BQ\x83\x10\x15a*?W\x8B\x83\x81Q\x81\x10a*0Wa*0aK\x17V[` \x02` \x01\x01Q\x90Pa*KV[a*H\x88a@\x86V[\x90P[_`\x02\x83\x83`@Q` \x01a*j\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*\x84\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a*\x9FW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xC2\x91\x90aL\xD7V[\x90P\x80\x87\x87\x81Q\x81\x10a*\xD7Wa*\xD7aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R_\x84\x81R\x8C\x82R`@\x80\x82 \x85\x90U\x84\x82R\x80\x82 \x86\x90U\x94\x81R`\x01\x80\x8E\x01\x90\x92R\x84\x81 \x83\x90U\x92\x83R\x92\x90\x91 U\x92\x90\x92\x01\x91Pa)\xD3\x90PV[P\x95PP`\x01\x01a)gV[P\x83Q`\x01\x14a+\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBeaconChainMock._buildMerkleTree`D\x82\x01Ru: invalid tree somehow`P\x1B`d\x82\x01R`\x84\x01a\x07+V[\x83_\x81Q\x81\x10a+\xB3Wa+\xB3aK\x17V[` \x02` \x01\x01Q\x90P\x93\x92PPPV[``_a+\xCFa@\xFAV[`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xE6Wa+\xE6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a,\x0FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x07\xEDWd\xFF\xFF\xFF\xFF\xFF\x81\x16_\x90\x81R`\"` R`@\x90 T\x82Q\x83\x90\x83\x90\x81\x10a,GWa,GaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a,\x14V[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R``\x91_\x91\x90\x80\x82\x01a\x04\0\x806\x837\x01\x90PP\x90P_[\x81Q\x81\x10\x15a,\xBEWa,\x97\x81`\x01aL;V[_\x1B\x82\x82\x81Q\x81\x10a,\xABWa,\xABaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a,\x83V[P\x83\x81`\x0B\x81Q\x81\x10a,\xD3Wa,\xD3aK\x17V[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0C\x81Q\x81\x10a,\xF3Wa,\xF3aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x93\x92PPPV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P_[\x81Q\x81\x10\x15a-hWa-A\x81`\x01aL;V[_\x1B\x82\x82\x81Q\x81\x10a-UWa-UaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a--V[P\x82\x81`\x03\x81Q\x81\x10a-}Wa-}aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x82a-\xB9\x83aA0V[`@Qa(\x8C\x92\x91\x90aMlV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a.\0Wa.\0aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.*W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81_\x80[`\x03\x81\x10\x15a.\x90W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x89\x01\x84\x01\x81\x90R\x96\x84R`\x07\x01\x90\x91R\x90 T\x92\x82a.\x84\x81aM\x90V[\x93PPP`\x01\x01a.1V[P`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x81R`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`&\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a.\xD5\x90\x82aM\xECV[PPPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x17Wa/\x17aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/AW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81_a/q` \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aL\x9BV[\x90P_\x80[`\x05\x81\x10\x15a/\xD5W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x87\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x05\x01\x90\x91R\x90 T\x93\x82a/\xC9\x81aM\x90V[\x93PPP`\x01\x01a/vV[P\x80[\x82\x81\x10\x15a06W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x87\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x07\x01\x90\x91R\x90 T\x93\x82a0*\x81aM\x90V[\x93PPP`\x01\x01a/\xD8V[P`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`'\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a0{\x90\x82aM\xECV[PPPPPPPPV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` R`@\x81 \x90[`!T\x81\x10\x15a2bW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xE3Wa0\xE3aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\rW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_a1\x1A\x83a;uV[\x90P_a1&\x82a=\xF6V[\x90P_\x80[a17`(`\x01aL;V[\x81\x10\x15a1\x91W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R\x80\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x01\x01\x90\x91R\x90 T\x92\x82a1\x85\x81aM\x90V[\x93PPP`\x01\x01a1+V[P\x80[`\x05a1\xA2`(`\x01aL;V[a1\xAC\x91\x90aL;V[\x81\x10\x15a2\tW`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x05\x01\x90\x91R\x90 T\x92\x82a1\xFD\x81aM\x90V[\x93PPP`\x01\x01a1\x94V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x87\x81R`@\x90\x91 \x84Qa2.\x92\x86\x01\x90aC?V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x87\x90R`@\x90 `\x01\x01a2Q\x85\x82aM\xECV[PP`\x01\x90\x93\x01\x92Pa0\xA0\x91PPV[PPV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`)` R`@\x81 \x90a2\x88a@\xFAV[\x90P_[\x81\x81\x10\x15a3\xB5W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xCDWa2\xCDaC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a2\xF7W` \x82\x01\x81\x806\x837\x01\x90P[Pd\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\"` R`@\x81 T\x91\x92P\x81\x90\x80[a3!`&`\x01aL;V[\x81\x10\x15a3~W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x02\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x03\x01\x90\x91R\x90 T\x92\x82a3r\x81aM\x90V[\x93PPP`\x01\x01a3\x15V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x88\x90R`@\x90 \x83\x81U`\x01\x01a3\xA4\x85\x82aM\xECV[PP`\x01\x90\x93\x01\x92Pa2\x8C\x91PPV[PPPV[_\x80[`!T\x81\x10\x15a4^W_`!\x82\x81T\x81\x10a3\xDBWa3\xDBaK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a3\xFDWPa4VV[`\x03\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x01a4TW_a4,\x83a'\xBDV[\x90Pa49`\x01\x82aK\xB8V[\x90P\x83a4E\x81aM\x90V[\x94PPa4R\x83\x82a'\xC7V[P[P[`\x01\x01a3\xBDV[Pa\r\x94`@Q\x80``\x01`@R\x80`&\x81R` \x01aPP`&\x919\x82a([V[_\x80[`!T\x81\x10\x15a6\x1FW_`!\x82\x81T\x81\x10a4\xA2Wa4\xA2aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a4\xC4WPa6\x17V[_c;\x9A\xCA\0a4\xD3\x84a'\xBDV[`\x01`\x01`@\x1B\x03\x16a4\xE6\x91\x90aLNV[\x90P_a4\xFB\x83`\x02\x01\x80Ta&\x07\x90aK\xF6V[\x90P_\x80a5\rc;\x9A\xCA\0\x85aL\x9BV[`\x03\x86\x01T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a5GW\x83_\x03a5=WPPPPPa6\x17V[P\x82\x90P_a5vV[h\x01\xBC\x16\xD6t\xEC\x80\0\0\x84\x11\x15a5vWa5kh\x01\xBC\x16\xD6t\xEC\x80\0\0\x85aL(V[\x91Pd\x07sY@\0\x90P[`\x1FT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x90c\xC8\x8A^m\x90\x85\x90a5\xA0\x90\x86\x90\x83\x161aL;V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xE3W__\xFD[PZ\xF1\x15\x80\x15a5\xF5W=__>=_\xFD[PPPP\x81\x87a6\x05\x91\x90aL;V[\x96Pa6\x11\x86\x82a'\xC7V[PPPPP[`\x01\x01a4\x84V[P\x80\x15a\r\x94Wa\r\x94`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- withdrew excess balance\0\0\0\0\0\0\0\x81RP\x82a([V[_a\x1F\xA9`\x04\x83aL\xAEV[`!T_\x90a6\x81`\x04\x82aN\xA6V[d\xFF\xFF\xFF\xFF\xFF\x16_\x03a8bW`!T_\x90a6\xA4\x90`\x01`\x01`@\x1B\x03aK\xD7V[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82\x81`0\x01R`!`@Q\x80`\xE0\x01`@R\x80`\x01\x15\x15\x81R` \x01_\x15\x15\x81R` \x01`\x02\x84_`\x80\x1B`@Q` \x01a7\x05\x92\x91\x90aN\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra7\x1F\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a7:W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7]\x91\x90aL\xD7V[\x81R`@\x80Q` \x80\x82\x01\x83R_\x80\x83R\x81\x85\x01\x92\x90\x92R`\x01`\x01`@\x1B\x03\x88\x81\x16\x84\x86\x01R``\x80\x86\x01\x82\x90R`\x80\x90\x95\x01R\x85T`\x01\x80\x82\x01\x88U\x96\x83R\x91\x81\x90 \x85Q`\x04\x90\x93\x02\x01\x80T\x91\x86\x01Qa\xFF\xFF\x19\x90\x92\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x81U\x90\x83\x01Q\x93\x81\x01\x93\x90\x93U\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a7\xF0\x90\x82aM\xECV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua8R\x83\x83a'\xC7V[\x82a8\\\x81aN\xF3V[\x93PPPP[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81\x81`0\x01R`!`@Q\x80`\xE0\x01`@R\x80_\x15\x15\x81R` \x01_\x15\x15\x81R` \x01`\x02\x84_`\x80\x1B`@Q` \x01a8\xBE\x92\x91\x90aN\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra8\xD8\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a8\xF3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x16\x91\x90aL\xD7V[\x81R` \x01\x87\x81R` \x01\x86`\x01`\x01`@\x1B\x03\x16\x81R` \x01a98a\x14\x0EV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x91\x82\x01R\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x82\x90 \x83Q`\x04\x90\x92\x02\x01\x80T\x92\x84\x01Q\x15\x15a\x01\0\x02a\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16a\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x81U`@\x82\x01Q\x92\x81\x01\x92\x90\x92U``\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a9\xAC\x90\x82aM\xECV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua\x1F\xA5\x82\x85a'\xC7V[_\x80a:\x1B`\x04\x84aN\xA6V[a:&\x90`@aO\x19V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa#\xE6\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17`\xFF`8\x1B`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[_\x80a:\xA3\x83aO9V[`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83aA\xB0V[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83aA\xB0V[_\x80a;\x11`\x04\x85aN\xA6V[a;\x1C\x90`\x01aO\\V[a;'\x90`@aO\x19V[a;3\x90a\x01\0aOyV[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x01`@\x1B\x03\x81\x1B\x19\x85\x81\x16_a;T\x86aA\xFAV[\x90P_a;b\x85`\xC0aL(V[\x91\x90\x91\x1C\x91\x90\x91\x17\x97\x96PPPPPPPV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P_`!\x84d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a;\xB8Wa;\xB8aK\x17V[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x04\x90\x93\x02\x90\x91\x01\x80T`\xFF\x80\x82\x16\x15\x15\x85Ra\x01\0\x90\x91\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01\x83\x01T\x90\x82\x01R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a<\x10\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<<\x90aK\xF6V[\x80\x15a<\x87W\x80`\x1F\x10a<^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\x87V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x82\x16` \x84\x01R`\x01`@\x1B\x82\x04\x81\x16`@\x80\x85\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x92\x04\x16``\x90\x92\x01\x91\x90\x91R\x81\x01Q\x83Q\x91\x92P\x90\x83\x90_\x90a<\xE3Wa<\xE3aK\x17V[` \x02` \x01\x01\x81\x81RPP\x80``\x01Qa<\xFD\x90aO9V[\x82`\x01\x81Q\x81\x10a=\x10Wa=\x10aK\x17V[` \x02` \x01\x01\x81\x81RPPa=)\x81`\x80\x01QaA\xFAV[\x82`\x02\x81Q\x81\x10a=<Wa=<aK\x17V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q`@Q` \x01a=a\x91\x15\x15\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra=y\x90aO9V[\x82`\x03\x81Q\x81\x10a=\x8CWa=\x8CaK\x17V[` \x02` \x01\x01\x81\x81RPPa=\xA5\x81`\xA0\x01QaA\xFAV[\x82`\x05\x81Q\x81\x10a=\xB8Wa=\xB8aK\x17V[` \x02` \x01\x01\x81\x81RPPa=\xD1\x81`\xC0\x01QaA\xFAV[\x82`\x06\x81Q\x81\x10a=\xE4Wa=\xE4aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x92\x91PPV[__`\x02\x83Qa>\x06\x91\x90aL\x9BV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a>!Wa>!aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a>JW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a?DW`\x02\x85a>d\x83\x83aLNV[\x81Q\x81\x10a>tWa>taK\x17V[` \x02` \x01\x01Q\x86\x83`\x02a>\x8A\x91\x90aLNV[a>\x95\x90`\x01aL;V[\x81Q\x81\x10a>\xA5Wa>\xA5aK\x17V[` \x02` \x01\x01Q`@Q` \x01a>\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra>\xE1\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a>\xFCW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x1F\x91\x90aL\xD7V[\x82\x82\x81Q\x81\x10a?1Wa?1aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a>OV[Pa?P`\x02\x83aL\x9BV[\x91P[\x81\x15a@cW_[\x82\x81\x10\x15a@PW`\x02\x82a?p\x83\x83aLNV[\x81Q\x81\x10a?\x80Wa?\x80aK\x17V[` \x02` \x01\x01Q\x83\x83`\x02a?\x96\x91\x90aLNV[a?\xA1\x90`\x01aL;V[\x81Q\x81\x10a?\xB1Wa?\xB1aK\x17V[` \x02` \x01\x01Q`@Q` \x01a?\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra?\xED\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a@\x08W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@+\x91\x90aL\xD7V[\x82\x82\x81Q\x81\x10a@=Wa@=aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a?[V[Pa@\\`\x02\x83aL\x9BV[\x91Pa?SV[\x80_\x81Q\x81\x10a@uWa@uaK\x17V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[_`d\x82\x10a@\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7F_getZeroNode: invalid depth\0\0\0\0\0`D\x82\x01R`d\x01a\x07+V[`*\x82\x81T\x81\x10a@\xEAWa@\xEAaK\x17V[\x90_R` _ \x01T\x90P\x91\x90PV[`!T_\x90\x15aA+W`!T`\x04\x90aA\x16\x90`\x01\x90aL(V[aA \x91\x90aL\x9BV[a\x14\xCF\x90`\x01aL;V[P_\x90V[`@Qc\x16#C=`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R``\x90a\x1F\xA9\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x84W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaA\xAB\x91\x90\x81\x01\x90aO\x96V[aBpV[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01aA\xE3\x93\x92\x91\x90aP\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`8\x81\x81\x1B`\xFF`8\x1B\x16`(\x83\x81\x1Bf\xFF\0\0\0\0\0\0\x16\x91\x90\x91\x17`\x18\x84\x81\x1Be\xFF\0\0\0\0\0\x16\x91\x90\x91\x17`\x08\x85\x81\x1Bd\xFF\0\0\0\0\x16\x91\x90\x91\x17c\xFF\0\0\0\x91\x86\x90\x1C\x91\x90\x91\x16\x17b\xFF\0\0\x91\x85\x90\x1C\x91\x90\x91\x16\x17a\xFF\0\x91\x84\x90\x1C\x91\x90\x91\x16\x17`\xFF\x92\x90\x91\x1C\x91\x90\x91\x16\x17`\xC0\x1B\x90V[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[2m`\xE0\x1B\x81RP\x83aA\xB0V[`@Q\x80``\x01`@R\x80_`\x01`\x01`@\x1B\x03\x16\x81R` \x01aB\xCE`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x81R` \x01aB\xF0`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`@\x1B\x03\x16\x81R` \x01aC+`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aCxW\x91` \x02\x82\x01[\x82\x81\x11\x15aCxW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aC]V[PaC\x84\x92\x91PaC\x88V[P\x90V[[\x80\x82\x11\x15aC\x84W_\x81U`\x01\x01aC\x89V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xD8WaC\xD8aC\x9CV[`@R\x91\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aC\xF4W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12aD\x08W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD!WaD!aC\x9CV[\x80`\x05\x1BaD1` \x82\x01aC\xB0V[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15aDLW__\xFD[` \x86\x01\x92P[\x83\x83\x10\x15aDuWaDd\x83aC\xE0V[\x82R` \x92\x83\x01\x92\x90\x91\x01\x90aDSV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aD\x8FW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xA4W__\xFD[a#\xE6\x84\x82\x85\x01aC\xF9V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aD\xF0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\xC9V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aE\x0BW__\xFD[aE\x14\x82aC\xE0V[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01Ra#\xE6`@\x85\x01\x82aE\x1BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aE\x97W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aEyV[P\x93\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q```@\x84\x01RaE\xCF`\x80\x84\x01\x82aEIV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01R\x80Q`@\x83RaE\xF5`@\x84\x01\x82aEgV[\x90P` \x82\x01Q\x91P\x82\x81\x03` \x84\x01RaDu\x81\x83aE\x1BV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aF^W`\x1F\x19\x85\x84\x03\x01\x88RaFH\x83\x83QaE\x1BV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aF,V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aF\xCB\x90\x87\x01\x82aF\x10V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF\x90V[P\x92\x96\x95PPPPPPV[` \x81R_aE\x14` \x83\x01\x84aE\x1BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aE\x97W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aG\x11V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaG\x85`@\x88\x01\x82aE\x1BV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaG\xA0\x81\x83aF\xFFV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aG_V[` \x81R_aE\x14` \x83\x01\x84aF\x10V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aH*\x90\x87\x01\x82aF\xFFV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aG\xEFV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aF^W`\x1F\x19\x85\x84\x03\x01\x88RaHx\x83\x83QaEgV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aH\\V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q`\x80`@\x84\x01RaH\xBC`\xA0\x84\x01\x82aEIV[`@\x85\x01Q\x84\x82\x03`\x1F\x19\x01``\x86\x01R\x80Q\x80\x83R\x91\x92P` \x90\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01_[\x82\x81\x10\x15aI\x1AW`\x1F\x19\x86\x83\x03\x01\x84RaI\x05\x82\x86QaE\x1BV[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01aH\xE9V[P``\x88\x01Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01R\x94PaI8\x81\x86aH@V[\x98\x97PPPPPPPPV[__`@\x83\x85\x03\x12\x15aIUW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aIjW__\xFD[aIv\x85\x82\x86\x01aC\xF9V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aI\x92W__\xFD[\x80\x91PP\x92P\x92\x90PV[` \x81R_\x82Q`@` \x84\x01RaI\xB8``\x84\x01\x82aEIV[` \x85\x81\x01Q\x85\x83\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x84R\x92\x93P\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01_[\x82\x81\x10\x15aJ5W`\x1F\x19\x86\x83\x03\x01\x84R\x84Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q\x90P```@\x84\x01RaJ\x1F``\x84\x01\x82aE\x1BV[` \x96\x87\x01\x96\x95\x90\x95\x01\x94\x92PP`\x01\x01aI\xE3V[P\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aD\xF0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJZV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aJ\x90WaJ\x90aC\x9CV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15aJ\xAEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aJ\xD3W__\xFD[\x805aJ\xE6aJ\xE1\x82aJxV[aC\xB0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aJ\xFAW__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`S\x90\x82\x01R\x7FBeaconChainMock: attempting to e`@\x82\x01R\x7Fxit dummy validator. We need tho``\x82\x01Rr\x0El\xA4\x0C\xCD\xEED\x0E\x0EM\xED\xEC\xCC\xEC\xAD\xC4\x07\xC7E`k\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01\x81\x81\x1C\x90\x82\x16\x80aL\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\xEDWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[\x80\x82\x01\x80\x82\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x14\x07Wa\x14\x07aK\xA4V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aL\xA9WaL\xA9aL\x87V[P\x04\x90V[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aL\xC4WaL\xC4aL\x87V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aL\xE7W__\xFD[PQ\x91\x90PV[_\x82aL\xFCWaL\xFCaL\x87V[P\x06\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aM#\x82\x85aM\x01V[`\x17`\xF9\x1B\x81RaM7`\x01\x82\x01\x85aM\x01V[\x95\x94PPPPPV[`@\x81R_aMR`@\x83\x01\x85aE\x1BV[\x90P\x82` \x83\x01R\x93\x92PPPV[_aE\x14\x82\x84aM\x01V[`@\x81R_aM~`@\x83\x01\x85aE\x1BV[\x82\x81\x03` \x84\x01RaM7\x81\x85aE\x1BV[_`\x01\x82\x01aM\xA1WaM\xA1aK\xA4V[P`\x01\x01\x90V[`\x1F\x82\x11\x15a3\xB5W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aM\xCDWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F;W_\x81U`\x01\x01aM\xD9V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x05WaN\x05aC\x9CV[aN\x19\x81aN\x13\x84TaK\xF6V[\x84aM\xA8V[` `\x1F\x82\x11`\x01\x81\x14aNKW_\x83\x15aN4WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0F;V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aNzW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aNZV[P\x84\x82\x10\x15aN\x97W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aN\xBCWaN\xBCaL\x87V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[_aN\xDA\x82\x85aM\x01V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[_d\xFF\xFF\xFF\xFF\xFF\x82\x16d\xFF\xFF\xFF\xFF\xFF\x81\x03aO\x10WaO\x10aK\xA4V[`\x01\x01\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x14\x07Wa\x14\x07aK\xA4V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x07\xEDW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[d\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[_` \x82\x84\x03\x12\x15aO\xA6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xBBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\xCBW__\xFD[\x80QaO\xD9aJ\xE1\x82aJxV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aO\xEDW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_aM7aP!aP\x1B\x84\x88aM\x01V[\x86aM\x01V[\x84aM\x01V\xFE-- no validators; added empty block root- generated rewards for num validators\xA2dipfsX\"\x12 \xC5\xF3W\x9AsBV4\xAF\r\x92\x04\x92\xC2\xDD\xF1^P\x98\xD8:\xB6\x11\xE7\x1A9/\xCEN\x0B\xDFTdsolcC\0\x08\x1B\x003`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x004W_5`\xE0\x1C\x80cd5\x99\xF2\x14a\x01\xA4W\x80c\xAC\xD4\x14\xA8\x14a\x01\xDFW[` 6\x14a\0\x9DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`+`$\x82\x01R\x7F4788OracleMock.fallback: malform`D\x82\x01Rjed msg.data`\xA8\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[_a\0\xA86\x82a\x02\x0BV[\x90P\x80_\x03a\x01\tW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7F4788OracleMock.fallback: timesta`D\x82\x01Rf\x06\xD7\x02\x06\x972\x03`\xCC\x1B`d\x82\x01R`\x84\x01a\0\x94V[_\x81\x81R` \x81\x90R`@\x81 T\x90\x81\x90\x03a\x01\x9CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7F4788OracleMock.fallback: no bloc`D\x82\x01R\x7Fk root found. DID YOU USE CHEATS`d\x82\x01Re.WARP?`\xD0\x1B`\x84\x82\x01R`\xA4\x01a\0\x94V[\x80_R` _\xF3[a\x01\xCDa\x01\xB26`\x04a\x02\x0BV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\x02\ta\x01\xED6`\x04a\x02\"V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16_\x90\x81R` \x81\x90R`@\x90 UV[\0[_` \x82\x84\x03\x12\x15a\x02\x1BW__\xFD[P5\x91\x90PV[__`@\x83\x85\x03\x12\x15a\x023W__\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02JW__\xFD[\x94` \x93\x90\x93\x015\x93PPPV\xFE\xA2dipfsX\"\x12 \xE3\xB4\xEA\xF3:E\xB2)\xCE\xE6\xDF(\xC2\x9B\xB7\xC4*\x1ABC\x89!\x8A\xFA\x08\xD0\xFB\xDC\x8C\xB5\xDD^dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405260043610610212575f3560e01c806386a6f9e11161011e578063ba414fa6116100a8578063f0acd9881161006d578063f0acd988146105f9578063f72138731461060d578063f833eb631461062c578063f8f98a4e1461064b578063fa7626d41461066a575f5ffd5b8063ba414fa614610568578063c76f25c01461057c578063e20c9f71146105a8578063e3cefb42146105bc578063ed3c1605146105d0575f5ffd5b8063a50a3a1a116100ee578063a50a3a1a146104b9578063aa47389c146104e5578063b0464fdc14610514578063b1b6f6a114610528578063b5508aa914610554575f5ffd5b806386a6f9e114610303578063908820e014610446578063916a17c614610465578063a3f4df7e14610486575f5ffd5b80633cf80e6c1161019f5780635e6cc2fc1161016f5780635e6cc2fc146103a557806366d9a9a0146103d15780636b3abd97146103f2578063766718081461041157806385226c8114610425575f5ffd5b80633cf80e6c146103555780633e5e3c23146103695780633f7286f41461037d57806359d095dd14610391575f5ffd5b806329992faa116101e557806329992faa146102cc5780632ade3880146102e25780632def600914610303578063330bc27e14610322578063357e951f14610336575f5ffd5b806314360958146102165780631ed7831c146102525780631f54365c1461027357806323e82c4c146102a0575b5f5ffd5b348015610221575f5ffd5b5061023561023036600461447f565b610683565b6040516001600160401b0390911681526020015b60405180910390f35b34801561025d575f5ffd5b506102666107f3565b60405161024991906144b0565b34801561027e575f5ffd5b5061029261028d3660046144fb565b610853565b604051908152602001610249565b3480156102ab575f5ffd5b506102bf6102ba3660046144fb565b610885565b60405161024991906145a1565b3480156102d7575f5ffd5b506102e0610a97565b005b3480156102ed575f5ffd5b506102f6610f42565b604051610249919061466a565b34801561030e575f5ffd5b5061023561031d3660046144fb565b61107e565b34801561032d575f5ffd5b50610235600a81565b348015610341575f5ffd5b50602054610235906001600160401b031681565b348015610360575f5ffd5b506102e06110bb565b348015610374575f5ffd5b50610266611102565b348015610388575f5ffd5b50610266611160565b34801561039c575f5ffd5b506102e06111be565b3480156103b0575f5ffd5b506103c46103bf3660046144fb565b6111f5565b60405161024991906146ed565b3480156103dc575f5ffd5b506103e5611224565b6040516102499190614739565b3480156103fd575f5ffd5b5061029261040c36600461447f565b611388565b34801561041c575f5ffd5b5061023561140e565b348015610430575f5ffd5b506104396114d4565b60405161024991906147b7565b348015610451575f5ffd5b506102926104603660046144fb565b61159f565b348015610470575f5ffd5b506104796115cb565b60405161024991906147c9565b348015610491575f5ffd5b5060408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b60208201526103c4565b3480156104c4575f5ffd5b506104d86104d336600461447f565b6116ac565b604051610249919061488e565b3480156104f0575f5ffd5b506105046104ff3660046144fb565b611ac5565b6040519015158152602001610249565b34801561051f575f5ffd5b50610479611b14565b348015610533575f5ffd5b50610547610542366004614944565b611bf5565b604051610249919061499d565b34801561055f575f5ffd5b50610439611faf565b348015610573575f5ffd5b5061050461207a565b348015610587575f5ffd5b5061059b61059636600461447f565b61211a565b6040516102499190614a41565b3480156105b3575f5ffd5b506102666121d5565b3480156105c7575f5ffd5b50610235600181565b6105e36105de366004614a9e565b612233565b60405164ffffffffff9091168152602001610249565b348015610604575f5ffd5b506102e06123ee565b348015610618575f5ffd5b506102356106273660046144fb565b612434565b348015610637575f5ffd5b506102356106463660046144fb565b612447565b348015610656575f5ffd5b506102356106653660046144fb565b61248b565b348015610675575f5ffd5b50601f546105049060ff1681565b5f6106b46040518060400160405280600f81526020016e736c61736856616c696461746f727360881b81525061272d565b5f5b82518110156107ed575f8382815181106106d2576106d2614b17565b602002602001015190505f60218264ffffffffff16815481106106f7576106f7614b17565b5f9182526020909120600490910201805490915060ff16156107345760405162461bcd60e51b815260040161072b90614b2b565b60405180910390fd5b8054610100900460ff1661078b57805461ff00191661010017815561075761140e565b610762906001614bb8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055505b5f610795836127bd565b90506001600160401b038116600a11156107bd576107b38186614bb8565b94505f90506107d8565b6107c8600a86614bb8565b94506107d5600a82614bd7565b90505b6107e283826127c7565b5050506001016106b6565b50919050565b6060601680548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f20905b81546001600160a01b0316815260019091019060200180831161082b575b5050505050905090565b5f60218264ffffffffff168154811061086e5761086e614b17565b905f5260205f209060040201600101549050919050565b61088d614298565b6025546001600160401b03165f90815260286020908152604080832064ffffffffff86168452825280832081518154606094810282018501845292810183815290939192849284919084018282801561090357602002820191905f5260205f20905b8154815260200190600101908083116108ef575b5050505050815260200160018201805461091c90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461094890614bf6565b80156109935780601f1061096a57610100808354040283529160200191610993565b820191905f5260205f20905b81548152906001019060200180831161097657829003601f168201915b505050919092525050604080516060810182526025546001600160401b03168082525f908152602660209081529083902083518085019094528054845260018101805496975092958287019550909291840191906109f090614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054610a1c90614bf6565b8015610a675780601f10610a3e57610100808354040283529160200191610a67565b820191905f5260205f20905b815481529060010190602001808311610a4a57829003601f168201915b50505091909252505050815260408051808201909152835181526020938401518185015292019190915292915050565b5f5b602154811015610b30575f60218281548110610ab757610ab7614b17565b5f9182526020909120600490910201805490915060ff1615610ad95750610b28565b5f610ae3836127bd565b9050640773594000816001600160401b03161115610b0357506407735940005b600391909101805467ffffffffffffffff19166001600160401b039092169190911790555b600101610a99565b50610b6f6040518060400160405280601c81526020017f2d2075706461746564206566666563746976652062616c616e6365730000000081525061282c565b610bb16040518060400160405280601081526020016f05a5a40c6eae4e4cadce840cae0dec6d60831b815250610ba361140e565b6001600160401b031661285b565b5f610bba61140e565b601f5490915061010090046001600160a01b031663e5d6bf02610bdc83612898565b6040516001600160e01b031960e084901b1681526001600160401b0390911660048201526024015f604051808303815f87803b158015610c1a575f5ffd5b505af1158015610c2c573d5f5f3e3d5ffd5b50506025805467ffffffffffffffff1916426001600160401b0316179055505060408051808201909152601681527505a40d4eadae0cac840e8de40dccaf0e840cae0dec6d60531b6020820152610c8590610ba361140e565b610cc36040518060400160405280601d81526020017f2d206275696c64696e6720626561636f6e20737461746520747265657300000081525061282c565b60215415610ce257602154610cda90600190614c28565b602455610d97565b60255460405163159a829560e31b81526001600160401b0390911660048201527fc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a4706024820152720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a8906044015f604051808303815f87803b158015610d5d575f5ffd5b505af1158015610d6f573d5f5f3e3d5ffd5b50505050610d946040518060600160405280602881526020016150286028913961282c565b50565b5f610dcd610da36128d7565b610daf60286001614c3b565b6025546001600160401b03165f908152602b60205260409020612964565b90505f610e08610ddb612bc4565b610de760266001614c3b565b6025546001600160401b03165f908152602b60205260409020600201612964565b90505f610e3c610e188484612c5a565b6025546001600160401b03165f908152602b60205260409020600590600401612964565b90505f610e6f610e4b83612d05565b6025546001600160401b03165f908152602b60205260409020600390600601612964565b9050610ea7604051806040016040528060148152602001730b4b4818995858dbdb88189b1bd8dac81c9bdbdd60621b81525082612d8e565b60255460405163159a829560e31b81526001600160401b03909116600482015260248101829052720f3df6d732807ef1319fb7b8bb8522d0beac029063acd414a8906044015f604051808303815f87803b158015610f03575f5ffd5b505af1158015610f15573d5f5f3e3d5ffd5b50505050610f2282612dc7565b610f2b83612ede565b610f33613085565b610f3b613266565b5050505050565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015611075575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b8282101561105e578382905f5260205f20018054610fd390614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054610fff90614bf6565b801561104a5780601f106110215761010080835404028352916020019161104a565b820191905f5260205f20905b81548152906001019060200180831161102d57829003601f168201915b505050505081526020019060010190610fb6565b505050508152505081526020019060010190610f65565b50505050905090565b5f60218264ffffffffff168154811061109957611099614b17565b5f9182526020909120600360049092020101546001600160401b031692915050565b6110e86040518060400160405280600c81526020016b0c2c8ecc2dcc6ca8ae0dec6d60a31b81525061272d565b6110f06133ba565b6110f8613481565b611100610a97565b565b6060601880548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b6110f060405180604001604052806016815260200175616476616e636545706f63685f4e6f5265776172647360501b81525061272d565b60408051603080825260608281019093525f919060208201818036833750505060308101939093525090919050565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f2090600202016040518060400160405290815f8201805461127790614bf6565b80601f01602080910402602001604051908101604052809291908181526020018280546112a390614bf6565b80156112ee5780601f106112c5576101008083540402835291602001916112ee565b820191905f5260205f20905b8154815290600101906020018083116112d157829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561137057602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116113325790505b50505050508152505081526020019060010190611247565b5f80805b835181101561140757633b9aca0060218583815181106113ae576113ae614b17565b602002602001015164ffffffffff16815481106113cd576113cd614b17565b5f9182526020909120600360049092020101546113f391906001600160401b0316614c4e565b6113fd9083614c3b565b915060010161138c565b5092915050565b601f545f90600160a81b90046001600160401b03164210156114985760405162461bcd60e51b815260206004820152603d60248201527f426561636f6e436861696e2e63757272656e7445706f63683a2063757272656e60448201527f742074696d65206973206265666f72652067656e657369732074696d65000000606482015260840161072b565b6114a4600c6020614c65565b601f546001600160401b03918216916114c591600160a81b90041642614c28565b6114cf9190614c9b565b905090565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f2001805461151490614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461154090614bf6565b801561158b5780601f106115625761010080835404028352916020019161158b565b820191905f5260205f20905b81548152906001019060200180831161156e57829003601f168201915b5050505050815260200190600101906114f7565b5f6022816115ae600485614cae565b64ffffffffff16815260208101919091526040015f205492915050565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015611075575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561169457602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116116565790505b505050505081525050815260200190600101906115ee565b6116b46142f5565b5f5b82518110156117ab576024548382815181106116d4576116d4614b17565b602002602001015164ffffffffff1611156117a35760405162461bcd60e51b815260206004820152607760248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f2076616c696461746f7220686173206e6f74206265656e20696e636c7564656460648201527f20696e20626561636f6e20636861696e207374617465202844494420594f552060848201527f43414c4c20616476616e636545706f6368205945543f2900000000000000000060a482015260c40161072b565b6001016116b6565b50604080516080810182526025546001600160401b03168082525f908152602660209081528382208451808601909552805485526001810180549395838601949093840191906117fa90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461182690614bf6565b80156118715780601f1061184857610100808354040283529160200191611871565b820191905f5260205f20905b81548152906001019060200180831161185457829003601f168201915b505050505081525050815260200184516001600160401b038111156118985761189861439c565b6040519080825280602002602001820160405280156118cb57816020015b60608152602001906001900390816118b65790505b50815260200184516001600160401b038111156118ea576118ea61439c565b60405190808252806020026020018201604052801561191d57816020015b60608152602001906001900390816119085790505b50905290505f5b8351811015611407576025546001600160401b03165f9081526028602052604081208551829087908590811061195c5761195c614b17565b602002602001015164ffffffffff1664ffffffffff1681526020019081526020015f206040518060400160405290815f82018054806020026020016040519081016040528092919081815260200182805480156119d657602002820191905f5260205f20905b8154815260200190600101908083116119c2575b505050505081526020016001820180546119ef90614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611a1b90614bf6565b8015611a665780601f10611a3d57610100808354040283529160200191611a66565b820191905f5260205f20905b815481529060010190602001808311611a4957829003601f168201915b5050505050815250509050806020015183604001518381518110611a8c57611a8c614b17565b6020026020010181905250805f015183606001518381518110611ab157611ab1614b17565b602090810291909101015250600101611924565b5f6001600160401b03801660218364ffffffffff1681548110611aea57611aea614b17565b5f918252602090912060049091020160030154600160801b90046001600160401b03161492915050565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015611075575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611bdd57602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611b9f5790505b50505050508152505081526020019060010190611b37565b611c1f604080516080810182525f9181019182526060808201529081908152602001606081525090565b5f5b8351811015611cf057602454848281518110611c3f57611c3f614b17565b602002602001015164ffffffffff161115611ce85760405162461bcd60e51b815260206004820152605b60248201527f426561636f6e436861696e2e67657443726564656e7469616c50726f6f66733a60448201527f206e6f20636865636b706f696e742070726f6f6620666f756e6420286469642060648201527f796f752063616c6c20616476616e636545706f6368207965743f290000000000608482015260a40161072b565b600101611c21565b50604080516001600160401b0384165f90815260276020528281206080830184528054938301938452600181018054929484939092916060850191611d3490614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611d6090614bf6565b8015611dab5780601f10611d8257610100808354040283529160200191611dab565b820191905f5260205f20905b815481529060010190602001808311611d8e57829003601f168201915b505050505081525050815260200185516001600160401b03811115611dd257611dd261439c565b604051908082528060200260200182016040528015611e2557816020015b611e1260405180606001604052805f81526020015f8152602001606081525090565b815260200190600190039081611df05790505b50905290505f5b8451811015611fa5575f858281518110611e4857611e48614b17565b602002602001015190505f611e5c82613665565b6001600160401b0387165f90815260296020908152604080832064ffffffffff851684528252808320815180830190925280548252600181018054959650939491939092840191611eac90614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054611ed890614bf6565b8015611f235780601f10611efa57610100808354040283529160200191611f23565b820191905f5260205f20905b815481529060010190602001808311611f0657829003601f168201915b5050505050815250509050604051806060016040528060218564ffffffffff1681548110611f5357611f53614b17565b905f5260205f209060040201600101548152602001825f01518152602001826020015181525085602001518581518110611f8f57611f8f614b17565b6020908102919091010152505050600101611e2c565b5090505b92915050565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015611075578382905f5260205f20018054611fef90614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461201b90614bf6565b80156120665780601f1061203d57610100808354040283529160200191612066565b820191905f5260205f20905b81548152906001019060200180831161204957829003601f168201915b505050505081526020019060010190611fd2565b6008545f9060ff1615612091575060085460ff1690565b604051630667f9d760e41b8152737109709ecfa91a80626ff3989d68f67f5b1dd12d600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa1580156120ef573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906121139190614cd7565b1415905090565b60605f82516001600160401b038111156121365761213661439c565b60405190808252806020026020018201604052801561215f578160200160208202803683370190505b5090505f5b835181101561140757602184828151811061218157612181614b17565b602002602001015164ffffffffff16815481106121a0576121a0614b17565b905f5260205f209060040201600101548282815181106121c2576121c2614b17565b6020908102919091010152600101612164565b6060601580548060200260200160405190810160405280929190818152602001828054801561084957602002820191905f5260205f209081546001600160a01b0316815260019091019060200180831161082b575050505050905090565b5f6122616040518060400160405280600c81526020016b3732bbab30b634b230ba37b960a11b81525061272d565b34670de0b6b3a76400008110156122d65760405162461bcd60e51b815260206004820152603360248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a206465604482015272706f7369742076616c756520746f6f206c6f7760681b606482015260840161072b565b6122e4633b9aca0082614cee565b156123575760405162461bcd60e51b815260206004820152603860248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a20766160448201527f6c7565206e6f74206d756c7469706c65206f6620677765690000000000000000606482015260840161072b565b5f612366633b9aca0083614c9b565b90506001600160401b038111156123dc5760405162461bcd60e51b815260206004820152603460248201527f426561636f6e436861696e4d6f636b2e6e657756616c696461746f723a2064656044820152730e0dee6d2e840ecc2d8eaca40e8dede40d0d2ced60631b606482015260840161072b565b6123e68482613671565b949350505050565b61242c6040518060400160405280601781526020017f616476616e636545706f63685f4e6f576974686472617700000000000000000081525061272d565b6110f86133ba565b5f611fa96124418361159f565b83613a0e565b5f60218264ffffffffff168154811061246257612462614b17565b5f918252602090912060049091020160030154600160801b90046001600160401b031692915050565b5f6124ba6040518060400160405280600d81526020016c32bc34ba2b30b634b230ba37b960991b81525061272d565b5f60218364ffffffffff16815481106124d5576124d5614b17565b5f9182526020909120600490910201805490915060ff16156125095760405162461bcd60e51b815260040161072b90614b2b565b6003810154600160801b90046001600160401b039081161461257f5760405162461bcd60e51b815260206004820152602960248201527f426561636f6e436861696e4d6f636b3a2076616c696461746f7220616c726561604482015268191e48195e1a5d195960ba1b606482015260840161072b565b61258761140e565b612592906001614bb8565b8160030160106101000a8154816001600160401b0302191690836001600160401b031602179055506125c3836127bd565b91506125cf835f6127c7565b5f61268860218564ffffffffff16815481106125ed576125ed614b17565b905f5260205f209060040201600201805461260790614bf6565b80601f016020809104026020016040519081016040528092919081815260200182805461263390614bf6565b801561267e5780601f106126555761010080835404028352916020019161267e565b820191905f5260205f20905b81548152906001019060200180831161266157829003601f168201915b5050505050613a98565b601f5490915061010090046001600160a01b031663c88a5e6d826126b9633b9aca006001600160401b038816614c4e565b6126cd906001600160a01b03861631614c3b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044015f604051808303815f87803b158015612710575f5ffd5b505af1158015612722573d5f5f3e3d5ffd5b505050505050919050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f5061277e61277960408051808201909152600b81526a2132b0b1b7b721b430b4b760a91b602082015290565b613ab3565b61278783613adc565b604051602001612798929190614d18565b60408051601f19818403018152908290526127b2916146ed565b60405180910390a150565b5f611fa982612434565b5f6022816127d6600486614cae565b64ffffffffff1664ffffffffff1681526020019081526020015f205490506127ff818484613b04565b90508060225f612810600487614cae565b64ffffffffff16815260208101919091526040015f2055505050565b7f41304facd9323d75b11bcdd609cb38effffdb05710f7caf0e9b16c6d9d709f50816040516127b291906146ed565b7fb2de2fbe801a0df6c0cbddfd448ba3c41d48a040ca35c56c8196ef0fcae721a8828260405161288c929190614d40565b60405180910390a15050565b5f6128a5600c6020614c65565b6128b0836001614bb8565b6128ba9190614c65565b601f54611fa99190600160a81b90046001600160401b0316614bb8565b6021546060905f906001600160401b038111156128f6576128f661439c565b60405190808252806020026020018201604052801561291f578160200160208202803683370190505b5090505f5b6021548110156107ed5761293f61293a82613b75565b613df6565b82828151811061295157612951614b17565b6020908102919091010152600101612924565b5f805b83811015612b30575f6002865160016129809190614c3b565b61298a9190614c9b565b90505f816001600160401b038111156129a5576129a561439c565b6040519080825280602002602001820160405280156129ce578160200160208202803683370190505b5090505f5b82811015612b24575f6129e7826002614c4e565b90505f6129f5826001614c3b565b90505f8a8381518110612a0a57612a0a614b17565b602002602001015190505f8b51831015612a3f578b8381518110612a3057612a30614b17565b60200260200101519050612a4b565b612a4888614086565b90505b5f60028383604051602001612a6a929190918252602082015260400190565b60408051601f1981840301815290829052612a8491614d61565b602060405180830381855afa158015612a9f573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190612ac29190614cd7565b905080878781518110612ad757612ad7614b17565b6020908102919091018101919091525f8481528c825260408082208590558482528082208690559481526001808e0190925284812083905592835292909120559290920191506129d39050565b50955050600101612967565b508351600114612ba15760405162461bcd60e51b815260206004820152603660248201527f426561636f6e436861696e4d6f636b2e5f6275696c644d65726b6c65547265656044820152753a20696e76616c6964207472656520736f6d65686f7760501b606482015260840161072b565b835f81518110612bb357612bb3614b17565b602002602001015190509392505050565b60605f612bcf6140fa565b6001600160401b03811115612be657612be661439c565b604051908082528060200260200182016040528015612c0f578160200160208202803683370190505b5090505f5b81518110156107ed5764ffffffffff81165f908152602260205260409020548251839083908110612c4757612c47614b17565b6020908102919091010152600101612c14565b60408051602080825261042082019092526060915f9190808201610400803683370190505090505f5b8151811015612cbe57612c97816001614c3b565b5f1b828281518110612cab57612cab614b17565b6020908102919091010152600101612c83565b508381600b81518110612cd357612cd3614b17565b6020026020010181815250508281600c81518110612cf357612cf3614b17565b60209081029190910101529392505050565b60408051600580825260c082019092526060915f91906020820160a0803683370190505090505f5b8151811015612d6857612d41816001614c3b565b5f1b828281518110612d5557612d55614b17565b6020908102919091010152600101612d2d565b508281600381518110612d7d57612d7d614b17565b602090810291909101015292915050565b7f280f4446b28a1372417dda658d30b95b2992b12ac9c7f378535f29a97acf358382612db983614130565b60405161288c929190614d6c565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612e0057612e0061439c565b6040519080825280601f01601f191660200182016040528015612e2a576020820181803683370190505b509050815f805b6003811015612e90576025546001600160401b03165f908152602b6020908152604080832086845260068101835281842054858402890184018190529684526007019091529020549282612e8481614d90565b93505050600101612e31565b5060408051808201825285815260208082018681526025546001600160401b03165f90815260269092529290208151815591519091906001820190612ed59082614dec565b50505050505050565b5f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b03811115612f1757612f1761439c565b6040519080825280601f01601f191660200182016040528015612f41576020820181803683370190505b509050815f612f7160207f0000000000000000000000000000000000000000000000000000000000000000614c9b565b90505f805b6005811015612fd5576025546001600160401b03165f908152602b60209081526040808320878452600481018352818420548584028a0184018190529784526005019091529020549382612fc981614d90565b93505050600101612f76565b50805b82811015613036576025546001600160401b03165f908152602b60209081526040808320878452600681018352818420548584028a018401819052978452600701909152902054938261302a81614d90565b93505050600101612fd8565b5060408051808201825286815260208082018781526025546001600160401b03165f9081526027909252929020815181559151909190600182019061307b9082614dec565b5050505050505050565b6025546001600160401b03165f908152602860205260408120905b602154811015613262575f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b038111156130e3576130e361439c565b6040519080825280601f01601f19166020018201604052801561310d576020820181803683370190505b5090505f61311a83613b75565b90505f61312682613df6565b90505f805b61313760286001614c3b565b811015613191576025546001600160401b03165f908152602b60209081526040808320868452808352818420548584028a018401819052968452600101909152902054928261318581614d90565b9350505060010161312b565b50805b60056131a260286001614c3b565b6131ac9190614c3b565b811015613209576025546001600160401b03165f908152602b60209081526040808320868452600481018352818420548584028a01840181905296845260050190915290205492826131fd81614d90565b93505050600101613194565b5064ffffffffff85165f90815260208781526040909120845161322e9286019061433f565b5064ffffffffff85165f9081526020879052604090206001016132518582614dec565b5050600190930192506130a0915050565b5050565b6025546001600160401b03165f908152602960205260408120906132886140fa565b90505f5b818110156133b5575f7f00000000000000000000000000000000000000000000000000000000000000006001600160401b038111156132cd576132cd61439c565b6040519080825280601f01601f1916602001820160405280156132f7576020820181803683370190505b5064ffffffffff83165f908152602260205260408120549192508190805b61332160266001614c3b565b81101561337e576025546001600160401b03165f908152602b60209081526040808320868452600281018352818420548584028a018401819052968452600301909152902054928261337281614d90565b93505050600101613315565b5064ffffffffff85165f9081526020889052604090208381556001016133a48582614dec565b50506001909301925061328c915050565b505050565b5f805b60215481101561345e575f602182815481106133db576133db614b17565b5f9182526020909120600490910201805490915060ff16156133fd5750613456565b600381015467fffffffffffffffe19600160801b9091046001600160401b031601613454575f61342c836127bd565b9050613439600182614bb8565b90508361344581614d90565b94505061345283826127c7565b505b505b6001016133bd565b50610d94604051806060016040528060268152602001615050602691398261285b565b5f805b60215481101561361f575f602182815481106134a2576134a2614b17565b5f9182526020909120600490910201805490915060ff16156134c45750613617565b5f633b9aca006134d3846127bd565b6001600160401b03166134e69190614c4e565b90505f6134fb83600201805461260790614bf6565b90505f8061350d633b9aca0085614c9b565b6003860154909150600160801b90046001600160401b039081161461354757835f0361353d575050505050613617565b508290505f613576565b6801bc16d674ec8000008411156135765761356b6801bc16d674ec80000085614c28565b915064077359400090505b601f546001600160a01b0361010090910481169063c88a5e6d9085906135a0908690831631614c3b565b6040516001600160e01b031960e085901b1681526001600160a01b03909216600483015260248201526044015f604051808303815f87803b1580156135e3575f5ffd5b505af11580156135f5573d5f5f3e3d5ffd5b5050505081876136059190614c3b565b965061361186826127c7565b50505050505b600101613484565b508015610d9457610d946040518060400160405280601981526020017f2d207769746864726577206578636573732062616c616e6365000000000000008152508261285b565b5f611fa9600483614cae565b6021545f90613681600482614ea6565b64ffffffffff165f03613862576021545f906136a4906001600160401b03614bd7565b604080516030808252606082019092529192505f919060208201818036833701905050905082816030015260216040518060e001604052806001151581526020015f151581526020016002845f60801b604051602001613705929190614ecf565b60408051601f198184030181529082905261371f91614d61565b602060405180830381855afa15801561373a573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061375d9190614cd7565b815260408051602080820183525f808352818501929092526001600160401b0388811684860152606080860182905260809095015285546001808201885596835291819020855160049093020180549186015161ffff1990921692151561ff0019169290921761010091151591909102178155908301519381019390935581015190919060028201906137f09082614dec565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b03199093169190941617179290921617905561385283836127c7565b8261385c81614ef3565b93505050505b604080516030808252606082019092525f9160208201818036833701905050905081816030015260216040518060e001604052805f151581526020015f151581526020016002845f60801b6040516020016138be929190614ecf565b60408051601f19818403018152908290526138d891614d61565b602060405180830381855afa1580156138f3573d5f5f3e3d5ffd5b5050506040513d601f19601f820116820180604052508101906139169190614cd7565b8152602001878152602001866001600160401b0316815260200161393861140e565b6001600160401b0390811682526020918201528254600181810185555f94855293829020835160049092020180549284015115156101000261ff00199215159290921661ffff1990931692909217178155604082015192810192909255606081015190919060028201906139ac9082614dec565b5060808201516003909101805460a084015160c0909401516001600160401b03908116600160801b0267ffffffffffffffff60801b19958216600160401b026001600160801b031990931691909416171792909216179055611fa582856127c7565b5f80613a1b600484614ea6565b613a26906040614f19565b64ffffffffff1690506123e684821b60f881901c60e882901c61ff00161760d882901c62ff0000161760c882901c63ff000000161764ff0000000060b883901c161765ff000000000060a883901c161766ff000000000000609883901c161760ff60381b60889290921c919091161790565b5f80613aa383614f39565b6001600160a01b03169392505050565b6060611fa9604051806040016040528060058152602001641b5b39366d60d81b815250836141b0565b6060611fa9604051806040016040528060048152602001631b5b336d60e01b815250836141b0565b5f80613b11600485614ea6565b613b1c906001614f5c565b613b27906040614f19565b613b3390610100614f79565b64ffffffffff1690506001600160401b03811b198581165f613b54866141fa565b90505f613b628560c0614c28565b9190911c91909117979650505050505050565b60408051600880825261012082019092526060915f919060208201610100803683370190505090505f60218464ffffffffff1681548110613bb857613bb8614b17565b5f9182526020918290206040805160e0810182526004909302909101805460ff8082161515855261010090910416151593830193909352600183015490820152600282018054919291606084019190613c1090614bf6565b80601f0160208091040260200160405190810160405280929190818152602001828054613c3c90614bf6565b8015613c875780601f10613c5e57610100808354040283529160200191613c87565b820191905f5260205f20905b815481529060010190602001808311613c6a57829003601f168201915b5050509183525050600391909101546001600160401b038082166020840152600160401b82048116604080850191909152600160801b9092041660609092019190915281015183519192509083905f90613ce357613ce3614b17565b6020026020010181815250508060600151613cfd90614f39565b82600181518110613d1057613d10614b17565b602002602001018181525050613d2981608001516141fa565b82600281518110613d3c57613d3c614b17565b6020026020010181815250508060200151604051602001613d61911515815260200190565b604051602081830303815290604052613d7990614f39565b82600381518110613d8c57613d8c614b17565b602002602001018181525050613da58160a001516141fa565b82600581518110613db857613db8614b17565b602002602001018181525050613dd18160c001516141fa565b82600681518110613de457613de4614b17565b60209081029190910101525092915050565b5f5f60028351613e069190614c9b565b90505f816001600160401b03811115613e2157613e2161439c565b604051908082528060200260200182016040528015613e4a578160200160208202803683370190505b5090505f5b82811015613f4457600285613e648383614c4e565b81518110613e7457613e74614b17565b602002602001015186836002613e8a9190614c4e565b613e95906001614c3b565b81518110613ea557613ea5614b17565b6020026020010151604051602001613ec7929190918252602082015260400190565b60408051601f1981840301815290829052613ee191614d61565b602060405180830381855afa158015613efc573d5f5f3e3d5ffd5b5050506040513d601f19601f82011682018060405250810190613f1f9190614cd7565b828281518110613f3157613f31614b17565b6020908102919091010152600101613e4f565b50613f50600283614c9b565b91505b8115614063575f5b8281101561405057600282613f708383614c4e565b81518110613f8057613f80614b17565b602002602001015183836002613f969190614c4e565b613fa1906001614c3b565b81518110613fb157613fb1614b17565b6020026020010151604051602001613fd3929190918252602082015260400190565b60408051601f1981840301815290829052613fed91614d61565b602060405180830381855afa158015614008573d5f5f3e3d5ffd5b5050506040513d601f19601f8201168201806040525081019061402b9190614cd7565b82828151811061403d5761403d614b17565b6020908102919091010152600101613f5b565b5061405c600283614c9b565b9150613f53565b805f8151811061407557614075614b17565b602002602001015192505050919050565b5f606482106140d75760405162461bcd60e51b815260206004820152601b60248201527f5f6765745a65726f4e6f64653a20696e76616c69642064657074680000000000604482015260640161072b565b602a82815481106140ea576140ea614b17565b905f5260205f2001549050919050565b6021545f901561412b5760215460049061411690600190614c28565b6141209190614c9b565b6114cf906001614c3b565b505f90565b604051631623433d60e31b815260048101829052606090611fa990737109709ecfa91a80626ff3989d68f67f5b1dd12d9063b11a19e8906024015f60405180830381865afa158015614184573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526141ab9190810190614f96565b614270565b60608282604051806040016040528060048152602001631b5b306d60e01b8152506040516020016141e39392919061500a565b604051602081830303815290604052905092915050565b603881811b60ff60381b16602883811b66ff0000000000001691909117601884811b65ff00000000001691909117600885811b64ff00000000169190911763ff0000009186901c919091161762ff00009185901c919091161761ff009184901c919091161760ff9290911c919091161760c01b90565b6060611fa9604051806040016040528060048152602001631b5b326d60e01b815250836141b0565b60405180606001604052805f6001600160401b031681526020016142ce60405180604001604052805f8152602001606081525090565b81526020016142f0604051806040016040528060608152602001606081525090565b905290565b60405180608001604052805f6001600160401b0316815260200161432b60405180604001604052805f8152602001606081525090565b815260200160608152602001606081525090565b828054828255905f5260205f20908101928215614378579160200282015b8281111561437857825182559160200191906001019061435d565b50614384929150614388565b5090565b5b80821115614384575f8155600101614389565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f191681016001600160401b03811182821017156143d8576143d861439c565b604052919050565b803564ffffffffff811681146143f4575f5ffd5b919050565b5f82601f830112614408575f5ffd5b81356001600160401b038111156144215761442161439c565b8060051b614431602082016143b0565b9182526020818501810192908101908684111561444c575f5ffd5b6020860192505b8383101561447557614464836143e0565b825260209283019290910190614453565b9695505050505050565b5f6020828403121561448f575f5ffd5b81356001600160401b038111156144a4575f5ffd5b6123e6848285016143f9565b602080825282518282018190525f918401906040840190835b818110156144f05783516001600160a01b03168352602093840193909201916001016144c9565b509095945050505050565b5f6020828403121561450b575f5ffd5b614514826143e0565b9392505050565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b805182525f6020820151604060208501526123e6604085018261451b565b5f8151808452602084019350602083015f5b82811015614597578151865260209586019590910190600101614579565b5093949350505050565b602081526001600160401b0382511660208201525f6020830151606060408401526145cf6080840182614549565b90506040840151601f198483030160608501528051604083526145f56040840182614567565b9050602082015191508281036020840152614475818361451b565b5f82825180855260208501945060208160051b830101602085015f5b8381101561465e57601f1985840301885261464883835161451b565b602098890198909350919091019060010161462c565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157868503603f19018452815180516001600160a01b031686526020908101516040918701829052906146cb90870182614610565b9550506020938401939190910190600101614690565b50929695505050505050565b602081525f614514602083018461451b565b5f8151808452602084019350602083015f5b828110156145975781516001600160e01b031916865260209586019590910190600101614711565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157603f198786030184528151805160408752614785604088018261451b565b90506020820151915086810360208801526147a081836146ff565b96505050602093840193919091019060010161475f565b602081525f6145146020830184614610565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b828110156146e157868503603f19018452815180516001600160a01b0316865260209081015160409187018290529061482a908701826146ff565b95505060209384019391909101906001016147ef565b5f82825180855260208501945060208160051b830101602085015f5b8381101561465e57601f19858403018852614878838351614567565b602098890198909350919091019060010161485c565b602081526001600160401b0382511660208201525f6020830151608060408401526148bc60a0840182614549565b6040850151848203601f19016060860152805180835291925060209081019181840191600582901b8501015f5b8281101561491a57601f1986830301845261490582865161451b565b602095860195949094019391506001016148e9565b506060880151878203601f1901608089015294506149388186614840565b98975050505050505050565b5f5f60408385031215614955575f5ffd5b82356001600160401b0381111561496a575f5ffd5b614976858286016143f9565b92505060208301356001600160401b0381168114614992575f5ffd5b809150509250929050565b602081525f8251604060208401526149b86060840182614549565b602085810151858303601f19016040870152805180845292935081019181840191600582901b8501015f5b82811015614a3557601f19868303018452845180518352602081015160208401526040810151905060606040840152614a1f606084018261451b565b60209687019695909501949250506001016149e3565b50979650505050505050565b602080825282518282018190525f918401906040840190835b818110156144f0578351835260209384019390920191600101614a5a565b5f6001600160401b03821115614a9057614a9061439c565b50601f01601f191660200190565b5f60208284031215614aae575f5ffd5b81356001600160401b03811115614ac3575f5ffd5b8201601f81018413614ad3575f5ffd5b8035614ae6614ae182614a78565b6143b0565b818152856020838501011115614afa575f5ffd5b816020840160208301375f91810160200191909152949350505050565b634e487b7160e01b5f52603260045260245ffd5b60208082526053908201527f426561636f6e436861696e4d6f636b3a20617474656d7074696e6720746f206560408201527f7869742064756d6d792076616c696461746f722e205765206e6565642074686f6060820152720e6ca40ccdee440e0e4dedecccecadc407c745606b1b608082015260a00190565b634e487b7160e01b5f52601160045260245ffd5b6001600160401b038181168382160190811115611fa957611fa9614ba4565b6001600160401b038281168282160390811115611fa957611fa9614ba4565b600181811c90821680614c0a57607f821691505b6020821081036107ed57634e487b7160e01b5f52602260045260245ffd5b81810381811115611fa957611fa9614ba4565b80820180821115611fa957611fa9614ba4565b8082028115828204841417611fa957611fa9614ba4565b6001600160401b03818116838216029081169081811461140757611407614ba4565b634e487b7160e01b5f52601260045260245ffd5b5f82614ca957614ca9614c87565b500490565b5f64ffffffffff831680614cc457614cc4614c87565b8064ffffffffff84160491505092915050565b5f60208284031215614ce7575f5ffd5b5051919050565b5f82614cfc57614cfc614c87565b500690565b5f81518060208401855e5f93019283525090919050565b5f614d238285614d01565b601760f91b8152614d376001820185614d01565b95945050505050565b604081525f614d52604083018561451b565b90508260208301529392505050565b5f6145148284614d01565b604081525f614d7e604083018561451b565b8281036020840152614d37818561451b565b5f60018201614da157614da1614ba4565b5060010190565b601f8211156133b557805f5260205f20601f840160051c81016020851015614dcd5750805b601f840160051c820191505b81811015610f3b575f8155600101614dd9565b81516001600160401b03811115614e0557614e0561439c565b614e1981614e138454614bf6565b84614da8565b6020601f821160018114614e4b575f8315614e345750848201515b5f19600385901b1c1916600184901b178455610f3b565b5f84815260208120601f198516915b82811015614e7a5787850151825560209485019460019092019101614e5a565b5084821015614e9757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f64ffffffffff831680614ebc57614ebc614c87565b8064ffffffffff84160691505092915050565b5f614eda8285614d01565b6001600160801b03199390931683525050601001919050565b5f64ffffffffff821664ffffffffff8103614f1057614f10614ba4565b60010192915050565b64ffffffffff818116838216029081169081811461140757611407614ba4565b805160208083015191908110156107ed575f1960209190910360031b1b16919050565b64ffffffffff8181168382160190811115611fa957611fa9614ba4565b64ffffffffff8281168282160390811115611fa957611fa9614ba4565b5f60208284031215614fa6575f5ffd5b81516001600160401b03811115614fbb575f5ffd5b8201601f81018413614fcb575f5ffd5b8051614fd9614ae182614a78565b818152856020838501011115614fed575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b5f614d3761502161501b8488614d01565b86614d01565b84614d0156fe2d2d206e6f2076616c696461746f72733b20616464656420656d70747920626c6f636b20726f6f742d2067656e657261746564207265776172647320666f72206e756d2076616c696461746f7273a2646970667358221220c5f3579a73425634af0d920492c2ddf15e5098d83ab611e71a392fce4e0bdf5464736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x02\x12W_5`\xE0\x1C\x80c\x86\xA6\xF9\xE1\x11a\x01\x1EW\x80c\xBAAO\xA6\x11a\0\xA8W\x80c\xF0\xAC\xD9\x88\x11a\0mW\x80c\xF0\xAC\xD9\x88\x14a\x05\xF9W\x80c\xF7!8s\x14a\x06\rW\x80c\xF83\xEBc\x14a\x06,W\x80c\xF8\xF9\x8AN\x14a\x06KW\x80c\xFAv&\xD4\x14a\x06jW__\xFD[\x80c\xBAAO\xA6\x14a\x05hW\x80c\xC7o%\xC0\x14a\x05|W\x80c\xE2\x0C\x9Fq\x14a\x05\xA8W\x80c\xE3\xCE\xFBB\x14a\x05\xBCW\x80c\xED<\x16\x05\x14a\x05\xD0W__\xFD[\x80c\xA5\n:\x1A\x11a\0\xEEW\x80c\xA5\n:\x1A\x14a\x04\xB9W\x80c\xAAG8\x9C\x14a\x04\xE5W\x80c\xB0FO\xDC\x14a\x05\x14W\x80c\xB1\xB6\xF6\xA1\x14a\x05(W\x80c\xB5P\x8A\xA9\x14a\x05TW__\xFD[\x80c\x86\xA6\xF9\xE1\x14a\x03\x03W\x80c\x90\x88 \xE0\x14a\x04FW\x80c\x91j\x17\xC6\x14a\x04eW\x80c\xA3\xF4\xDF~\x14a\x04\x86W__\xFD[\x80c<\xF8\x0El\x11a\x01\x9FW\x80c^l\xC2\xFC\x11a\x01oW\x80c^l\xC2\xFC\x14a\x03\xA5W\x80cf\xD9\xA9\xA0\x14a\x03\xD1W\x80ck:\xBD\x97\x14a\x03\xF2W\x80cvg\x18\x08\x14a\x04\x11W\x80c\x85\"l\x81\x14a\x04%W__\xFD[\x80c<\xF8\x0El\x14a\x03UW\x80c>^<#\x14a\x03iW\x80c?r\x86\xF4\x14a\x03}W\x80cY\xD0\x95\xDD\x14a\x03\x91W__\xFD[\x80c)\x99/\xAA\x11a\x01\xE5W\x80c)\x99/\xAA\x14a\x02\xCCW\x80c*\xDE8\x80\x14a\x02\xE2W\x80c-\xEF`\t\x14a\x03\x03W\x80c3\x0B\xC2~\x14a\x03\"W\x80c5~\x95\x1F\x14a\x036W__\xFD[\x80c\x146\tX\x14a\x02\x16W\x80c\x1E\xD7\x83\x1C\x14a\x02RW\x80c\x1FT6\\\x14a\x02sW\x80c#\xE8,L\x14a\x02\xA0W[__\xFD[4\x80\x15a\x02!W__\xFD[Pa\x025a\x0206`\x04aD\x7FV[a\x06\x83V[`@Q`\x01`\x01`@\x1B\x03\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02]W__\xFD[Pa\x02fa\x07\xF3V[`@Qa\x02I\x91\x90aD\xB0V[4\x80\x15a\x02~W__\xFD[Pa\x02\x92a\x02\x8D6`\x04aD\xFBV[a\x08SV[`@Q\x90\x81R` \x01a\x02IV[4\x80\x15a\x02\xABW__\xFD[Pa\x02\xBFa\x02\xBA6`\x04aD\xFBV[a\x08\x85V[`@Qa\x02I\x91\x90aE\xA1V[4\x80\x15a\x02\xD7W__\xFD[Pa\x02\xE0a\n\x97V[\0[4\x80\x15a\x02\xEDW__\xFD[Pa\x02\xF6a\x0FBV[`@Qa\x02I\x91\x90aFjV[4\x80\x15a\x03\x0EW__\xFD[Pa\x025a\x03\x1D6`\x04aD\xFBV[a\x10~V[4\x80\x15a\x03-W__\xFD[Pa\x025`\n\x81V[4\x80\x15a\x03AW__\xFD[P` Ta\x025\x90`\x01`\x01`@\x1B\x03\x16\x81V[4\x80\x15a\x03`W__\xFD[Pa\x02\xE0a\x10\xBBV[4\x80\x15a\x03tW__\xFD[Pa\x02fa\x11\x02V[4\x80\x15a\x03\x88W__\xFD[Pa\x02fa\x11`V[4\x80\x15a\x03\x9CW__\xFD[Pa\x02\xE0a\x11\xBEV[4\x80\x15a\x03\xB0W__\xFD[Pa\x03\xC4a\x03\xBF6`\x04aD\xFBV[a\x11\xF5V[`@Qa\x02I\x91\x90aF\xEDV[4\x80\x15a\x03\xDCW__\xFD[Pa\x03\xE5a\x12$V[`@Qa\x02I\x91\x90aG9V[4\x80\x15a\x03\xFDW__\xFD[Pa\x02\x92a\x04\x0C6`\x04aD\x7FV[a\x13\x88V[4\x80\x15a\x04\x1CW__\xFD[Pa\x025a\x14\x0EV[4\x80\x15a\x040W__\xFD[Pa\x049a\x14\xD4V[`@Qa\x02I\x91\x90aG\xB7V[4\x80\x15a\x04QW__\xFD[Pa\x02\x92a\x04`6`\x04aD\xFBV[a\x15\x9FV[4\x80\x15a\x04pW__\xFD[Pa\x04ya\x15\xCBV[`@Qa\x02I\x91\x90aG\xC9V[4\x80\x15a\x04\x91W__\xFD[P`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01Ra\x03\xC4V[4\x80\x15a\x04\xC4W__\xFD[Pa\x04\xD8a\x04\xD36`\x04aD\x7FV[a\x16\xACV[`@Qa\x02I\x91\x90aH\x8EV[4\x80\x15a\x04\xF0W__\xFD[Pa\x05\x04a\x04\xFF6`\x04aD\xFBV[a\x1A\xC5V[`@Q\x90\x15\x15\x81R` \x01a\x02IV[4\x80\x15a\x05\x1FW__\xFD[Pa\x04ya\x1B\x14V[4\x80\x15a\x053W__\xFD[Pa\x05Ga\x05B6`\x04aIDV[a\x1B\xF5V[`@Qa\x02I\x91\x90aI\x9DV[4\x80\x15a\x05_W__\xFD[Pa\x049a\x1F\xAFV[4\x80\x15a\x05sW__\xFD[Pa\x05\x04a zV[4\x80\x15a\x05\x87W__\xFD[Pa\x05\x9Ba\x05\x966`\x04aD\x7FV[a!\x1AV[`@Qa\x02I\x91\x90aJAV[4\x80\x15a\x05\xB3W__\xFD[Pa\x02fa!\xD5V[4\x80\x15a\x05\xC7W__\xFD[Pa\x025`\x01\x81V[a\x05\xE3a\x05\xDE6`\x04aJ\x9EV[a\"3V[`@Qd\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02IV[4\x80\x15a\x06\x04W__\xFD[Pa\x02\xE0a#\xEEV[4\x80\x15a\x06\x18W__\xFD[Pa\x025a\x06'6`\x04aD\xFBV[a$4V[4\x80\x15a\x067W__\xFD[Pa\x025a\x06F6`\x04aD\xFBV[a$GV[4\x80\x15a\x06VW__\xFD[Pa\x025a\x06e6`\x04aD\xFBV[a$\x8BV[4\x80\x15a\x06uW__\xFD[P`\x1FTa\x05\x04\x90`\xFF\x16\x81V[_a\x06\xB4`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01nslashValidators`\x88\x1B\x81RPa'-V[_[\x82Q\x81\x10\x15a\x07\xEDW_\x83\x82\x81Q\x81\x10a\x06\xD2Wa\x06\xD2aK\x17V[` \x02` \x01\x01Q\x90P_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x06\xF7Wa\x06\xF7aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\x074W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07+\x90aK+V[`@Q\x80\x91\x03\x90\xFD[\x80Ta\x01\0\x90\x04`\xFF\x16a\x07\x8BW\x80Ta\xFF\0\x19\x16a\x01\0\x17\x81Ua\x07Wa\x14\x0EV[a\x07b\x90`\x01aK\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UP[_a\x07\x95\x83a'\xBDV[\x90P`\x01`\x01`@\x1B\x03\x81\x16`\n\x11\x15a\x07\xBDWa\x07\xB3\x81\x86aK\xB8V[\x94P_\x90Pa\x07\xD8V[a\x07\xC8`\n\x86aK\xB8V[\x94Pa\x07\xD5`\n\x82aK\xD7V[\x90P[a\x07\xE2\x83\x82a'\xC7V[PPP`\x01\x01a\x06\xB6V[P\x91\x90PV[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+W[PPPPP\x90P\x90V[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x08nWa\x08naK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x90P\x91\x90PV[a\x08\x8DaB\x98V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x86\x16\x84R\x82R\x80\x83 \x81Q\x81T``\x94\x81\x02\x82\x01\x85\x01\x84R\x92\x81\x01\x83\x81R\x90\x93\x91\x92\x84\x92\x84\x91\x90\x84\x01\x82\x82\x80\x15a\t\x03W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x08\xEFW[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\t\x1C\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\tH\x90aK\xF6V[\x80\x15a\t\x93W\x80`\x1F\x10a\tjWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x93V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tvW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPP`@\x80Q``\x81\x01\x82R`%T`\x01`\x01`@\x1B\x03\x16\x80\x82R_\x90\x81R`&` \x90\x81R\x90\x83\x90 \x83Q\x80\x85\x01\x90\x94R\x80T\x84R`\x01\x81\x01\x80T\x96\x97P\x92\x95\x82\x87\x01\x95P\x90\x92\x91\x84\x01\x91\x90a\t\xF0\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x1C\x90aK\xF6V[\x80\x15a\ngW\x80`\x1F\x10a\n>Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\ngV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\nJW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x90\x92RPPP\x81R`@\x80Q\x80\x82\x01\x90\x91R\x83Q\x81R` \x93\x84\x01Q\x81\x85\x01R\x92\x01\x91\x90\x91R\x92\x91PPV[_[`!T\x81\x10\x15a\x0B0W_`!\x82\x81T\x81\x10a\n\xB7Wa\n\xB7aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a\n\xD9WPa\x0B(V[_a\n\xE3\x83a'\xBDV[\x90Pd\x07sY@\0\x81`\x01`\x01`@\x1B\x03\x16\x11\x15a\x0B\x03WPd\x07sY@\0[`\x03\x91\x90\x91\x01\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`@\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U[`\x01\x01a\n\x99V[Pa\x0Bo`@Q\x80`@\x01`@R\x80`\x1C\x81R` \x01\x7F- updated effective balances\0\0\0\0\x81RPa(,V[a\x0B\xB1`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xA5\xA4\x0Cn\xAENL\xAD\xCE\x84\x0C\xAE\r\xECm`\x83\x1B\x81RPa\x0B\xA3a\x14\x0EV[`\x01`\x01`@\x1B\x03\x16a([V[_a\x0B\xBAa\x14\x0EV[`\x1FT\x90\x91Pa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xE5\xD6\xBF\x02a\x0B\xDC\x83a(\x98V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x84\x90\x1B\x16\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0C\x1AW__\xFD[PZ\xF1\x15\x80\x15a\x0C,W=__>=_\xFD[PP`%\x80Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16B`\x01`\x01`@\x1B\x03\x16\x17\x90UPP`@\x80Q\x80\x82\x01\x90\x91R`\x16\x81Ru\x05\xA4\rN\xAD\xAE\x0C\xAC\x84\x0E\x8D\xE4\r\xCC\xAF\x0E\x84\x0C\xAE\r\xECm`S\x1B` \x82\x01Ra\x0C\x85\x90a\x0B\xA3a\x14\x0EV[a\x0C\xC3`@Q\x80`@\x01`@R\x80`\x1D\x81R` \x01\x7F- building beacon state trees\0\0\0\x81RPa(,V[`!T\x15a\x0C\xE2W`!Ta\x0C\xDA\x90`\x01\x90aL(V[`$Ua\r\x97V[`%T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p`$\x82\x01Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\r]W__\xFD[PZ\xF1\x15\x80\x15a\roW=__>=_\xFD[PPPPa\r\x94`@Q\x80``\x01`@R\x80`(\x81R` \x01aP(`(\x919a(,V[PV[_a\r\xCDa\r\xA3a(\xD7V[a\r\xAF`(`\x01aL;V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 a)dV[\x90P_a\x0E\x08a\r\xDBa+\xC4V[a\r\xE7`&`\x01aL;V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x02\x01a)dV[\x90P_a\x0E<a\x0E\x18\x84\x84a,ZV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x05\x90`\x04\x01a)dV[\x90P_a\x0Eoa\x0EK\x83a-\x05V[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` R`@\x90 `\x03\x90`\x06\x01a)dV[\x90Pa\x0E\xA7`@Q\x80`@\x01`@R\x80`\x14\x81R` \x01s\x0BKH\x18\x99XX\xDB\xDB\x88\x18\x9B\x1B\xD8\xDA\xC8\x1C\x9B\xDB\xDD`b\x1B\x81RP\x82a-\x8EV[`%T`@Qc\x15\x9A\x82\x95`\xE3\x1B\x81R`\x01`\x01`@\x1B\x03\x90\x91\x16`\x04\x82\x01R`$\x81\x01\x82\x90Rr\x0F=\xF6\xD72\x80~\xF11\x9F\xB7\xB8\xBB\x85\"\xD0\xBE\xAC\x02\x90c\xAC\xD4\x14\xA8\x90`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a\x0F\x03W__\xFD[PZ\xF1\x15\x80\x15a\x0F\x15W=__>=_\xFD[PPPPa\x0F\"\x82a-\xC7V[a\x0F+\x83a.\xDEV[a\x0F3a0\x85V[a\x0F;a2fV[PPPPPV[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x10^W\x83\x82\x90_R` _ \x01\x80Ta\x0F\xD3\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0F\xFF\x90aK\xF6V[\x80\x15a\x10JW\x80`\x1F\x10a\x10!Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10JV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10-W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0F\xB6V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0FeV[PPPP\x90P\x90V[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x10\x99Wa\x10\x99aK\x17V[_\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01T`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[a\x10\xE8`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k\x0C,\x8E\xCC-\xCCl\xA8\xAE\r\xECm`\xA3\x1B\x81RPa'-V[a\x10\xF0a3\xBAV[a\x10\xF8a4\x81V[a\x11\0a\n\x97V[V[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[a\x10\xF0`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01uadvanceEpoch_NoRewards`P\x1B\x81RPa'-V[`@\x80Q`0\x80\x82R``\x82\x81\x01\x90\x93R_\x91\x90` \x82\x01\x81\x806\x837PPP`0\x81\x01\x93\x90\x93RP\x90\x91\x90PV[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x12w\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x12\xA3\x90aK\xF6V[\x80\x15a\x12\xEEW\x80`\x1F\x10a\x12\xC5Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x12\xEEV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x12\xD1W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x13pW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x132W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x12GV[_\x80\x80[\x83Q\x81\x10\x15a\x14\x07Wc;\x9A\xCA\0`!\x85\x83\x81Q\x81\x10a\x13\xAEWa\x13\xAEaK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x13\xCDWa\x13\xCDaK\x17V[_\x91\x82R` \x90\x91 `\x03`\x04\x90\x92\x02\x01\x01Ta\x13\xF3\x91\x90`\x01`\x01`@\x1B\x03\x16aLNV[a\x13\xFD\x90\x83aL;V[\x91P`\x01\x01a\x13\x8CV[P\x92\x91PPV[`\x1FT_\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16B\x10\x15a\x14\x98W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FBeaconChain.currentEpoch: curren`D\x82\x01R\x7Ft time is before genesis time\0\0\0`d\x82\x01R`\x84\x01a\x07+V[a\x14\xA4`\x0C` aLeV[`\x1FT`\x01`\x01`@\x1B\x03\x91\x82\x16\x91a\x14\xC5\x91`\x01`\xA8\x1B\x90\x04\x16BaL(V[a\x14\xCF\x91\x90aL\x9BV[\x90P\x90V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x01\x80Ta\x15\x14\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15@\x90aK\xF6V[\x80\x15a\x15\x8BW\x80`\x1F\x10a\x15bWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x8BV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15nW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x14\xF7V[_`\"\x81a\x15\xAE`\x04\x85aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ T\x92\x91PPV[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x16\x94W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16VW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15\xEEV[a\x16\xB4aB\xF5V[_[\x82Q\x81\x10\x15a\x17\xABW`$T\x83\x82\x81Q\x81\x10a\x16\xD4Wa\x16\xD4aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xA3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`w`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F validator has not been included`d\x82\x01R\x7F in beacon chain state (DID YOU `\x84\x82\x01R\x7FCALL advanceEpoch YET?)\0\0\0\0\0\0\0\0\0`\xA4\x82\x01R`\xC4\x01a\x07+V[`\x01\x01a\x16\xB6V[P`@\x80Q`\x80\x81\x01\x82R`%T`\x01`\x01`@\x1B\x03\x16\x80\x82R_\x90\x81R`&` \x90\x81R\x83\x82 \x84Q\x80\x86\x01\x90\x95R\x80T\x85R`\x01\x81\x01\x80T\x93\x95\x83\x86\x01\x94\x90\x93\x84\x01\x91\x90a\x17\xFA\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18&\x90aK\xF6V[\x80\x15a\x18qW\x80`\x1F\x10a\x18HWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18qV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18TW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\x98Wa\x18\x98aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x18\xCBW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x18\xB6W\x90P[P\x81R` \x01\x84Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x18\xEAWa\x18\xEAaC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x19\x1DW\x81` \x01[``\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x19\x08W\x90P[P\x90R\x90P_[\x83Q\x81\x10\x15a\x14\x07W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` R`@\x81 \x85Q\x82\x90\x87\x90\x85\x90\x81\x10a\x19\\Wa\x19\\aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x19\xD6W` \x02\x82\x01\x91\x90_R` _ \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x19\xC2W[PPPPP\x81R` \x01`\x01\x82\x01\x80Ta\x19\xEF\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1A\x1B\x90aK\xF6V[\x80\x15a\x1AfW\x80`\x1F\x10a\x1A=Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1AfV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1AIW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P\x80` \x01Q\x83`@\x01Q\x83\x81Q\x81\x10a\x1A\x8CWa\x1A\x8CaK\x17V[` \x02` \x01\x01\x81\x90RP\x80_\x01Q\x83``\x01Q\x83\x81Q\x81\x10a\x1A\xB1Wa\x1A\xB1aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x19$V[_`\x01`\x01`@\x1B\x03\x80\x16`!\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1A\xEAWa\x1A\xEAaK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x14\x92\x91PPV[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1B\xDDW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1B\x9FW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1B7V[a\x1C\x1F`@\x80Q`\x80\x81\x01\x82R_\x91\x81\x01\x91\x82R``\x80\x82\x01R\x90\x81\x90\x81R` \x01``\x81RP\x90V[_[\x83Q\x81\x10\x15a\x1C\xF0W`$T\x84\x82\x81Q\x81\x10a\x1C?Wa\x1C?aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a\x1C\xE8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FBeaconChain.getCredentialProofs:`D\x82\x01R\x7F no checkpoint proof found (did `d\x82\x01R\x7Fyou call advanceEpoch yet?)\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07+V[`\x01\x01a\x1C!V[P`@\x80Q`\x01`\x01`@\x1B\x03\x84\x16_\x90\x81R`'` R\x82\x81 `\x80\x83\x01\x84R\x80T\x93\x83\x01\x93\x84R`\x01\x81\x01\x80T\x92\x94\x84\x93\x90\x92\x91``\x85\x01\x91a\x1D4\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1D`\x90aK\xF6V[\x80\x15a\x1D\xABW\x80`\x1F\x10a\x1D\x82Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1D\xABV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1D\x8EW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x81R` \x01\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1D\xD2Wa\x1D\xD2aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x1E%W\x81` \x01[a\x1E\x12`@Q\x80``\x01`@R\x80_\x81R` \x01_\x81R` \x01``\x81RP\x90V[\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x1D\xF0W\x90P[P\x90R\x90P_[\x84Q\x81\x10\x15a\x1F\xA5W_\x85\x82\x81Q\x81\x10a\x1EHWa\x1EHaK\x17V[` \x02` \x01\x01Q\x90P_a\x1E\\\x82a6eV[`\x01`\x01`@\x1B\x03\x87\x16_\x90\x81R`)` \x90\x81R`@\x80\x83 d\xFF\xFF\xFF\xFF\xFF\x85\x16\x84R\x82R\x80\x83 \x81Q\x80\x83\x01\x90\x92R\x80T\x82R`\x01\x81\x01\x80T\x95\x96P\x93\x94\x91\x93\x90\x92\x84\x01\x91a\x1E\xAC\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1E\xD8\x90aK\xF6V[\x80\x15a\x1F#W\x80`\x1F\x10a\x1E\xFAWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1F#V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1F\x06W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81RPP\x90P`@Q\x80``\x01`@R\x80`!\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1FSWa\x1FSaK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x81R` \x01\x82_\x01Q\x81R` \x01\x82` \x01Q\x81RP\x85` \x01Q\x85\x81Q\x81\x10a\x1F\x8FWa\x1F\x8FaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RPPP`\x01\x01a\x1E,V[P\x90P[\x92\x91PPV[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x10uW\x83\x82\x90_R` _ \x01\x80Ta\x1F\xEF\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \x1B\x90aK\xF6V[\x80\x15a fW\x80`\x1F\x10a =Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a fV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a IW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x1F\xD2V[`\x08T_\x90`\xFF\x16\x15a \x91WP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81Rsq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a \xEFW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a!\x13\x91\x90aL\xD7V[\x14\x15\x90P\x90V[``_\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a!6Wa!6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a!_W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83Q\x81\x10\x15a\x14\x07W`!\x84\x82\x81Q\x81\x10a!\x81Wa!\x81aK\x17V[` \x02` \x01\x01Qd\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\xA0Wa!\xA0aK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x01\x01T\x82\x82\x81Q\x81\x10a!\xC2Wa!\xC2aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a!dV[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x08IW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x08+WPPPPP\x90P\x90V[_a\"a`@Q\x80`@\x01`@R\x80`\x0C\x81R` \x01k72\xBB\xAB0\xB64\xB20\xBA7\xB9`\xA1\x1B\x81RPa'-V[4g\r\xE0\xB6\xB3\xA7d\0\0\x81\x10\x15a\"\xD6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`3`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rrposit value too low`h\x1B`d\x82\x01R`\x84\x01a\x07+V[a\"\xE4c;\x9A\xCA\0\x82aL\xEEV[\x15a#WW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FBeaconChainMock.newValidator: va`D\x82\x01R\x7Flue not multiple of gwei\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07+V[_a#fc;\x9A\xCA\0\x83aL\x9BV[\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15a#\xDCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`4`$\x82\x01R\x7FBeaconChainMock.newValidator: de`D\x82\x01Rs\x0E\r\xEEm.\x84\x0E\xCC-\x8E\xAC\xA4\x0E\x8D\xED\xE4\r\r,\xED`c\x1B`d\x82\x01R`\x84\x01a\x07+V[a#\xE6\x84\x82a6qV[\x94\x93PPPPV[a$,`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7FadvanceEpoch_NoWithdraw\0\0\0\0\0\0\0\0\0\x81RPa'-V[a\x10\xF8a3\xBAV[_a\x1F\xA9a$A\x83a\x15\x9FV[\x83a:\x0EV[_`!\x82d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$bWa$baK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01`\x03\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16\x92\x91PPV[_a$\xBA`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l2\xBC4\xBA+0\xB64\xB20\xBA7\xB9`\x99\x1B\x81RPa'-V[_`!\x83d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a$\xD5Wa$\xD5aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a%\tW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07+\x90aK+V[`\x03\x81\x01T`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a%\x7FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`)`$\x82\x01R\x7FBeaconChainMock: validator alrea`D\x82\x01Rh\x19\x1EH\x19^\x1A]\x19Y`\xBA\x1B`d\x82\x01R`\x84\x01a\x07+V[a%\x87a\x14\x0EV[a%\x92\x90`\x01aK\xB8V[\x81`\x03\x01`\x10a\x01\0\n\x81T\x81`\x01`\x01`@\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`@\x1B\x03\x16\x02\x17\x90UPa%\xC3\x83a'\xBDV[\x91Pa%\xCF\x83_a'\xC7V[_a&\x88`!\x85d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a%\xEDWa%\xEDaK\x17V[\x90_R` _ \x90`\x04\x02\x01`\x02\x01\x80Ta&\x07\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&3\x90aK\xF6V[\x80\x15a&~W\x80`\x1F\x10a&UWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&~V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&aW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa:\x98V[`\x1FT\x90\x91Pa\x01\0\x90\x04`\x01`\x01`\xA0\x1B\x03\x16c\xC8\x8A^m\x82a&\xB9c;\x9A\xCA\0`\x01`\x01`@\x1B\x03\x88\x16aLNV[a&\xCD\x90`\x01`\x01`\xA0\x1B\x03\x86\x161aL;V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a'\x10W__\xFD[PZ\xF1\x15\x80\x15a'\"W=__>=_\xFD[PPPPPP\x91\x90PV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FPa'~a'y`@\x80Q\x80\x82\x01\x90\x91R`\x0B\x81Rj!2\xB0\xB1\xB7\xB7!\xB40\xB4\xB7`\xA9\x1B` \x82\x01R\x90V[a:\xB3V[a'\x87\x83a:\xDCV[`@Q` \x01a'\x98\x92\x91\x90aM\x18V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra'\xB2\x91aF\xEDV[`@Q\x80\x91\x03\x90\xA1PV[_a\x1F\xA9\x82a$4V[_`\"\x81a'\xD6`\x04\x86aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x90\x81R` \x01_ T\x90Pa'\xFF\x81\x84\x84a;\x04V[\x90P\x80`\"_a(\x10`\x04\x87aL\xAEV[d\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01_ UPPPV[\x7FA0O\xAC\xD92=u\xB1\x1B\xCD\xD6\t\xCB8\xEF\xFF\xFD\xB0W\x10\xF7\xCA\xF0\xE9\xB1lm\x9Dp\x9FP\x81`@Qa'\xB2\x91\x90aF\xEDV[\x7F\xB2\xDE/\xBE\x80\x1A\r\xF6\xC0\xCB\xDD\xFDD\x8B\xA3\xC4\x1DH\xA0@\xCA5\xC5l\x81\x96\xEF\x0F\xCA\xE7!\xA8\x82\x82`@Qa(\x8C\x92\x91\x90aM@V[`@Q\x80\x91\x03\x90\xA1PPV[_a(\xA5`\x0C` aLeV[a(\xB0\x83`\x01aK\xB8V[a(\xBA\x91\x90aLeV[`\x1FTa\x1F\xA9\x91\x90`\x01`\xA8\x1B\x90\x04`\x01`\x01`@\x1B\x03\x16aK\xB8V[`!T``\x90_\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a(\xF6Wa(\xF6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\x1FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[`!T\x81\x10\x15a\x07\xEDWa)?a):\x82a;uV[a=\xF6V[\x82\x82\x81Q\x81\x10a)QWa)QaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a)$V[_\x80[\x83\x81\x10\x15a+0W_`\x02\x86Q`\x01a)\x80\x91\x90aL;V[a)\x8A\x91\x90aL\x9BV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a)\xA5Wa)\xA5aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a)\xCEW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a+$W_a)\xE7\x82`\x02aLNV[\x90P_a)\xF5\x82`\x01aL;V[\x90P_\x8A\x83\x81Q\x81\x10a*\nWa*\naK\x17V[` \x02` \x01\x01Q\x90P_\x8BQ\x83\x10\x15a*?W\x8B\x83\x81Q\x81\x10a*0Wa*0aK\x17V[` \x02` \x01\x01Q\x90Pa*KV[a*H\x88a@\x86V[\x90P[_`\x02\x83\x83`@Q` \x01a*j\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra*\x84\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a*\x9FW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a*\xC2\x91\x90aL\xD7V[\x90P\x80\x87\x87\x81Q\x81\x10a*\xD7Wa*\xD7aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01\x91\x90\x91R_\x84\x81R\x8C\x82R`@\x80\x82 \x85\x90U\x84\x82R\x80\x82 \x86\x90U\x94\x81R`\x01\x80\x8E\x01\x90\x92R\x84\x81 \x83\x90U\x92\x83R\x92\x90\x91 U\x92\x90\x92\x01\x91Pa)\xD3\x90PV[P\x95PP`\x01\x01a)gV[P\x83Q`\x01\x14a+\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`6`$\x82\x01R\x7FBeaconChainMock._buildMerkleTree`D\x82\x01Ru: invalid tree somehow`P\x1B`d\x82\x01R`\x84\x01a\x07+V[\x83_\x81Q\x81\x10a+\xB3Wa+\xB3aK\x17V[` \x02` \x01\x01Q\x90P\x93\x92PPPV[``_a+\xCFa@\xFAV[`\x01`\x01`@\x1B\x03\x81\x11\x15a+\xE6Wa+\xE6aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a,\x0FW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x81Q\x81\x10\x15a\x07\xEDWd\xFF\xFF\xFF\xFF\xFF\x81\x16_\x90\x81R`\"` R`@\x90 T\x82Q\x83\x90\x83\x90\x81\x10a,GWa,GaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a,\x14V[`@\x80Q` \x80\x82Ra\x04 \x82\x01\x90\x92R``\x91_\x91\x90\x80\x82\x01a\x04\0\x806\x837\x01\x90PP\x90P_[\x81Q\x81\x10\x15a,\xBEWa,\x97\x81`\x01aL;V[_\x1B\x82\x82\x81Q\x81\x10a,\xABWa,\xABaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a,\x83V[P\x83\x81`\x0B\x81Q\x81\x10a,\xD3Wa,\xD3aK\x17V[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x0C\x81Q\x81\x10a,\xF3Wa,\xF3aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x93\x92PPPV[`@\x80Q`\x05\x80\x82R`\xC0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01`\xA0\x806\x837\x01\x90PP\x90P_[\x81Q\x81\x10\x15a-hWa-A\x81`\x01aL;V[_\x1B\x82\x82\x81Q\x81\x10a-UWa-UaK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a--V[P\x82\x81`\x03\x81Q\x81\x10a-}Wa-}aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x92\x91PPV[\x7F(\x0FDF\xB2\x8A\x13rA}\xDAe\x8D0\xB9[)\x92\xB1*\xC9\xC7\xF3xS_)\xA9z\xCF5\x83\x82a-\xB9\x83aA0V[`@Qa(\x8C\x92\x91\x90aMlV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a.\0Wa.\0aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a.*W` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81_\x80[`\x03\x81\x10\x15a.\x90W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x89\x01\x84\x01\x81\x90R\x96\x84R`\x07\x01\x90\x91R\x90 T\x92\x82a.\x84\x81aM\x90V[\x93PPP`\x01\x01a.1V[P`@\x80Q\x80\x82\x01\x82R\x85\x81R` \x80\x82\x01\x86\x81R`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`&\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a.\xD5\x90\x82aM\xECV[PPPPPPPV[_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x17Wa/\x17aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a/AW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P\x81_a/q` \x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aL\x9BV[\x90P_\x80[`\x05\x81\x10\x15a/\xD5W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x87\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x05\x01\x90\x91R\x90 T\x93\x82a/\xC9\x81aM\x90V[\x93PPP`\x01\x01a/vV[P\x80[\x82\x81\x10\x15a06W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x87\x84R`\x06\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x97\x84R`\x07\x01\x90\x91R\x90 T\x93\x82a0*\x81aM\x90V[\x93PPP`\x01\x01a/\xD8V[P`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x87\x81R`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`'\x90\x92R\x92\x90 \x81Q\x81U\x91Q\x90\x91\x90`\x01\x82\x01\x90a0{\x90\x82aM\xECV[PPPPPPPPV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`(` R`@\x81 \x90[`!T\x81\x10\x15a2bW_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xE3Wa0\xE3aC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a1\rW` \x82\x01\x81\x806\x837\x01\x90P[P\x90P_a1\x1A\x83a;uV[\x90P_a1&\x82a=\xF6V[\x90P_\x80[a17`(`\x01aL;V[\x81\x10\x15a1\x91W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R\x80\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x01\x01\x90\x91R\x90 T\x92\x82a1\x85\x81aM\x90V[\x93PPP`\x01\x01a1+V[P\x80[`\x05a1\xA2`(`\x01aL;V[a1\xAC\x91\x90aL;V[\x81\x10\x15a2\tW`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x04\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x05\x01\x90\x91R\x90 T\x92\x82a1\xFD\x81aM\x90V[\x93PPP`\x01\x01a1\x94V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x87\x81R`@\x90\x91 \x84Qa2.\x92\x86\x01\x90aC?V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x87\x90R`@\x90 `\x01\x01a2Q\x85\x82aM\xECV[PP`\x01\x90\x93\x01\x92Pa0\xA0\x91PPV[PPV[`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`)` R`@\x81 \x90a2\x88a@\xFAV[\x90P_[\x81\x81\x10\x15a3\xB5W_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xCDWa2\xCDaC\x9CV[`@Q\x90\x80\x82R\x80`\x1F\x01`\x1F\x19\x16` \x01\x82\x01`@R\x80\x15a2\xF7W` \x82\x01\x81\x806\x837\x01\x90P[Pd\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\"` R`@\x81 T\x91\x92P\x81\x90\x80[a3!`&`\x01aL;V[\x81\x10\x15a3~W`%T`\x01`\x01`@\x1B\x03\x16_\x90\x81R`+` \x90\x81R`@\x80\x83 \x86\x84R`\x02\x81\x01\x83R\x81\x84 T\x85\x84\x02\x8A\x01\x84\x01\x81\x90R\x96\x84R`\x03\x01\x90\x91R\x90 T\x92\x82a3r\x81aM\x90V[\x93PPP`\x01\x01a3\x15V[Pd\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R` \x88\x90R`@\x90 \x83\x81U`\x01\x01a3\xA4\x85\x82aM\xECV[PP`\x01\x90\x93\x01\x92Pa2\x8C\x91PPV[PPPV[_\x80[`!T\x81\x10\x15a4^W_`!\x82\x81T\x81\x10a3\xDBWa3\xDBaK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a3\xFDWPa4VV[`\x03\x81\x01Tg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\x19`\x01`\x80\x1B\x90\x91\x04`\x01`\x01`@\x1B\x03\x16\x01a4TW_a4,\x83a'\xBDV[\x90Pa49`\x01\x82aK\xB8V[\x90P\x83a4E\x81aM\x90V[\x94PPa4R\x83\x82a'\xC7V[P[P[`\x01\x01a3\xBDV[Pa\r\x94`@Q\x80``\x01`@R\x80`&\x81R` \x01aPP`&\x919\x82a([V[_\x80[`!T\x81\x10\x15a6\x1FW_`!\x82\x81T\x81\x10a4\xA2Wa4\xA2aK\x17V[_\x91\x82R` \x90\x91 `\x04\x90\x91\x02\x01\x80T\x90\x91P`\xFF\x16\x15a4\xC4WPa6\x17V[_c;\x9A\xCA\0a4\xD3\x84a'\xBDV[`\x01`\x01`@\x1B\x03\x16a4\xE6\x91\x90aLNV[\x90P_a4\xFB\x83`\x02\x01\x80Ta&\x07\x90aK\xF6V[\x90P_\x80a5\rc;\x9A\xCA\0\x85aL\x9BV[`\x03\x86\x01T\x90\x91P`\x01`\x80\x1B\x90\x04`\x01`\x01`@\x1B\x03\x90\x81\x16\x14a5GW\x83_\x03a5=WPPPPPa6\x17V[P\x82\x90P_a5vV[h\x01\xBC\x16\xD6t\xEC\x80\0\0\x84\x11\x15a5vWa5kh\x01\xBC\x16\xD6t\xEC\x80\0\0\x85aL(V[\x91Pd\x07sY@\0\x90P[`\x1FT`\x01`\x01`\xA0\x1B\x03a\x01\0\x90\x91\x04\x81\x16\x90c\xC8\x8A^m\x90\x85\x90a5\xA0\x90\x86\x90\x83\x161aL;V[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01_`@Q\x80\x83\x03\x81_\x87\x80;\x15\x80\x15a5\xE3W__\xFD[PZ\xF1\x15\x80\x15a5\xF5W=__>=_\xFD[PPPP\x81\x87a6\x05\x91\x90aL;V[\x96Pa6\x11\x86\x82a'\xC7V[PPPPP[`\x01\x01a4\x84V[P\x80\x15a\r\x94Wa\r\x94`@Q\x80`@\x01`@R\x80`\x19\x81R` \x01\x7F- withdrew excess balance\0\0\0\0\0\0\0\x81RP\x82a([V[_a\x1F\xA9`\x04\x83aL\xAEV[`!T_\x90a6\x81`\x04\x82aN\xA6V[d\xFF\xFF\xFF\xFF\xFF\x16_\x03a8bW`!T_\x90a6\xA4\x90`\x01`\x01`@\x1B\x03aK\xD7V[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R\x91\x92P_\x91\x90` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x82\x81`0\x01R`!`@Q\x80`\xE0\x01`@R\x80`\x01\x15\x15\x81R` \x01_\x15\x15\x81R` \x01`\x02\x84_`\x80\x1B`@Q` \x01a7\x05\x92\x91\x90aN\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra7\x1F\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a7:W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7]\x91\x90aL\xD7V[\x81R`@\x80Q` \x80\x82\x01\x83R_\x80\x83R\x81\x85\x01\x92\x90\x92R`\x01`\x01`@\x1B\x03\x88\x81\x16\x84\x86\x01R``\x80\x86\x01\x82\x90R`\x80\x90\x95\x01R\x85T`\x01\x80\x82\x01\x88U\x96\x83R\x91\x81\x90 \x85Q`\x04\x90\x93\x02\x01\x80T\x91\x86\x01Qa\xFF\xFF\x19\x90\x92\x16\x92\x15\x15a\xFF\0\x19\x16\x92\x90\x92\x17a\x01\0\x91\x15\x15\x91\x90\x91\x02\x17\x81U\x90\x83\x01Q\x93\x81\x01\x93\x90\x93U\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a7\xF0\x90\x82aM\xECV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua8R\x83\x83a'\xC7V[\x82a8\\\x81aN\xF3V[\x93PPPP[`@\x80Q`0\x80\x82R``\x82\x01\x90\x92R_\x91` \x82\x01\x81\x806\x837\x01\x90PP\x90P\x81\x81`0\x01R`!`@Q\x80`\xE0\x01`@R\x80_\x15\x15\x81R` \x01_\x15\x15\x81R` \x01`\x02\x84_`\x80\x1B`@Q` \x01a8\xBE\x92\x91\x90aN\xCFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra8\xD8\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a8\xF3W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9\x16\x91\x90aL\xD7V[\x81R` \x01\x87\x81R` \x01\x86`\x01`\x01`@\x1B\x03\x16\x81R` \x01a98a\x14\x0EV[`\x01`\x01`@\x1B\x03\x90\x81\x16\x82R` \x91\x82\x01R\x82T`\x01\x81\x81\x01\x85U_\x94\x85R\x93\x82\x90 \x83Q`\x04\x90\x92\x02\x01\x80T\x92\x84\x01Q\x15\x15a\x01\0\x02a\xFF\0\x19\x92\x15\x15\x92\x90\x92\x16a\xFF\xFF\x19\x90\x93\x16\x92\x90\x92\x17\x17\x81U`@\x82\x01Q\x92\x81\x01\x92\x90\x92U``\x81\x01Q\x90\x91\x90`\x02\x82\x01\x90a9\xAC\x90\x82aM\xECV[P`\x80\x82\x01Q`\x03\x90\x91\x01\x80T`\xA0\x84\x01Q`\xC0\x90\x94\x01Q`\x01`\x01`@\x1B\x03\x90\x81\x16`\x01`\x80\x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x1B\x19\x95\x82\x16`\x01`@\x1B\x02`\x01`\x01`\x80\x1B\x03\x19\x90\x93\x16\x91\x90\x94\x16\x17\x17\x92\x90\x92\x16\x17\x90Ua\x1F\xA5\x82\x85a'\xC7V[_\x80a:\x1B`\x04\x84aN\xA6V[a:&\x90`@aO\x19V[d\xFF\xFF\xFF\xFF\xFF\x16\x90Pa#\xE6\x84\x82\x1B`\xF8\x81\x90\x1C`\xE8\x82\x90\x1Ca\xFF\0\x16\x17`\xD8\x82\x90\x1Cb\xFF\0\0\x16\x17`\xC8\x82\x90\x1Cc\xFF\0\0\0\x16\x17d\xFF\0\0\0\0`\xB8\x83\x90\x1C\x16\x17e\xFF\0\0\0\0\0`\xA8\x83\x90\x1C\x16\x17f\xFF\0\0\0\0\0\0`\x98\x83\x90\x1C\x16\x17`\xFF`8\x1B`\x88\x92\x90\x92\x1C\x91\x90\x91\x16\x17\x90V[_\x80a:\xA3\x83aO9V[`\x01`\x01`\xA0\x1B\x03\x16\x93\x92PPPV[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x1B[96m`\xD8\x1B\x81RP\x83aA\xB0V[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[3m`\xE0\x1B\x81RP\x83aA\xB0V[_\x80a;\x11`\x04\x85aN\xA6V[a;\x1C\x90`\x01aO\\V[a;'\x90`@aO\x19V[a;3\x90a\x01\0aOyV[d\xFF\xFF\xFF\xFF\xFF\x16\x90P`\x01`\x01`@\x1B\x03\x81\x1B\x19\x85\x81\x16_a;T\x86aA\xFAV[\x90P_a;b\x85`\xC0aL(V[\x91\x90\x91\x1C\x91\x90\x91\x17\x97\x96PPPPPPPV[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P_`!\x84d\xFF\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a;\xB8Wa;\xB8aK\x17V[_\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x04\x90\x93\x02\x90\x91\x01\x80T`\xFF\x80\x82\x16\x15\x15\x85Ra\x01\0\x90\x91\x04\x16\x15\x15\x93\x83\x01\x93\x90\x93R`\x01\x83\x01T\x90\x82\x01R`\x02\x82\x01\x80T\x91\x92\x91``\x84\x01\x91\x90a<\x10\x90aK\xF6V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta<<\x90aK\xF6V[\x80\x15a<\x87W\x80`\x1F\x10a<^Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a<\x87V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a<jW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPP\x91\x83RPP`\x03\x91\x90\x91\x01T`\x01`\x01`@\x1B\x03\x80\x82\x16` \x84\x01R`\x01`@\x1B\x82\x04\x81\x16`@\x80\x85\x01\x91\x90\x91R`\x01`\x80\x1B\x90\x92\x04\x16``\x90\x92\x01\x91\x90\x91R\x81\x01Q\x83Q\x91\x92P\x90\x83\x90_\x90a<\xE3Wa<\xE3aK\x17V[` \x02` \x01\x01\x81\x81RPP\x80``\x01Qa<\xFD\x90aO9V[\x82`\x01\x81Q\x81\x10a=\x10Wa=\x10aK\x17V[` \x02` \x01\x01\x81\x81RPPa=)\x81`\x80\x01QaA\xFAV[\x82`\x02\x81Q\x81\x10a=<Wa=<aK\x17V[` \x02` \x01\x01\x81\x81RPP\x80` \x01Q`@Q` \x01a=a\x91\x15\x15\x81R` \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@Ra=y\x90aO9V[\x82`\x03\x81Q\x81\x10a=\x8CWa=\x8CaK\x17V[` \x02` \x01\x01\x81\x81RPPa=\xA5\x81`\xA0\x01QaA\xFAV[\x82`\x05\x81Q\x81\x10a=\xB8Wa=\xB8aK\x17V[` \x02` \x01\x01\x81\x81RPPa=\xD1\x81`\xC0\x01QaA\xFAV[\x82`\x06\x81Q\x81\x10a=\xE4Wa=\xE4aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01RP\x92\x91PPV[__`\x02\x83Qa>\x06\x91\x90aL\x9BV[\x90P_\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a>!Wa>!aC\x9CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a>JW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x82\x81\x10\x15a?DW`\x02\x85a>d\x83\x83aLNV[\x81Q\x81\x10a>tWa>taK\x17V[` \x02` \x01\x01Q\x86\x83`\x02a>\x8A\x91\x90aLNV[a>\x95\x90`\x01aL;V[\x81Q\x81\x10a>\xA5Wa>\xA5aK\x17V[` \x02` \x01\x01Q`@Q` \x01a>\xC7\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra>\xE1\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a>\xFCW=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a?\x1F\x91\x90aL\xD7V[\x82\x82\x81Q\x81\x10a?1Wa?1aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a>OV[Pa?P`\x02\x83aL\x9BV[\x91P[\x81\x15a@cW_[\x82\x81\x10\x15a@PW`\x02\x82a?p\x83\x83aLNV[\x81Q\x81\x10a?\x80Wa?\x80aK\x17V[` \x02` \x01\x01Q\x83\x83`\x02a?\x96\x91\x90aLNV[a?\xA1\x90`\x01aL;V[\x81Q\x81\x10a?\xB1Wa?\xB1aK\x17V[` \x02` \x01\x01Q`@Q` \x01a?\xD3\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra?\xED\x91aMaV[` `@Q\x80\x83\x03\x81\x85Z\xFA\x15\x80\x15a@\x08W=__>=_\xFD[PPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a@+\x91\x90aL\xD7V[\x82\x82\x81Q\x81\x10a@=Wa@=aK\x17V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a?[V[Pa@\\`\x02\x83aL\x9BV[\x91Pa?SV[\x80_\x81Q\x81\x10a@uWa@uaK\x17V[` \x02` \x01\x01Q\x92PPP\x91\x90PV[_`d\x82\x10a@\xD7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1B`$\x82\x01R\x7F_getZeroNode: invalid depth\0\0\0\0\0`D\x82\x01R`d\x01a\x07+V[`*\x82\x81T\x81\x10a@\xEAWa@\xEAaK\x17V[\x90_R` _ \x01T\x90P\x91\x90PV[`!T_\x90\x15aA+W`!T`\x04\x90aA\x16\x90`\x01\x90aL(V[aA \x91\x90aL\x9BV[a\x14\xCF\x90`\x01aL;V[P_\x90V[`@Qc\x16#C=`\xE3\x1B\x81R`\x04\x81\x01\x82\x90R``\x90a\x1F\xA9\x90sq\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\x90c\xB1\x1A\x19\xE8\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15aA\x84W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@RaA\xAB\x91\x90\x81\x01\x90aO\x96V[aBpV[``\x82\x82`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[0m`\xE0\x1B\x81RP`@Q` \x01aA\xE3\x93\x92\x91\x90aP\nV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`8\x81\x81\x1B`\xFF`8\x1B\x16`(\x83\x81\x1Bf\xFF\0\0\0\0\0\0\x16\x91\x90\x91\x17`\x18\x84\x81\x1Be\xFF\0\0\0\0\0\x16\x91\x90\x91\x17`\x08\x85\x81\x1Bd\xFF\0\0\0\0\x16\x91\x90\x91\x17c\xFF\0\0\0\x91\x86\x90\x1C\x91\x90\x91\x16\x17b\xFF\0\0\x91\x85\x90\x1C\x91\x90\x91\x16\x17a\xFF\0\x91\x84\x90\x1C\x91\x90\x91\x16\x17`\xFF\x92\x90\x91\x1C\x91\x90\x91\x16\x17`\xC0\x1B\x90V[``a\x1F\xA9`@Q\x80`@\x01`@R\x80`\x04\x81R` \x01c\x1B[2m`\xE0\x1B\x81RP\x83aA\xB0V[`@Q\x80``\x01`@R\x80_`\x01`\x01`@\x1B\x03\x16\x81R` \x01aB\xCE`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x81R` \x01aB\xF0`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x90R\x90V[`@Q\x80`\x80\x01`@R\x80_`\x01`\x01`@\x1B\x03\x16\x81R` \x01aC+`@Q\x80`@\x01`@R\x80_\x81R` \x01``\x81RP\x90V[\x81R` \x01``\x81R` \x01``\x81RP\x90V[\x82\x80T\x82\x82U\x90_R` _ \x90\x81\x01\x92\x82\x15aCxW\x91` \x02\x82\x01[\x82\x81\x11\x15aCxW\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90aC]V[PaC\x84\x92\x91PaC\x88V[P\x90V[[\x80\x82\x11\x15aC\x84W_\x81U`\x01\x01aC\x89V[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15aC\xD8WaC\xD8aC\x9CV[`@R\x91\x90PV[\x805d\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14aC\xF4W__\xFD[\x91\x90PV[_\x82`\x1F\x83\x01\x12aD\x08W__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD!WaD!aC\x9CV[\x80`\x05\x1BaD1` \x82\x01aC\xB0V[\x91\x82R` \x81\x85\x01\x81\x01\x92\x90\x81\x01\x90\x86\x84\x11\x15aDLW__\xFD[` \x86\x01\x92P[\x83\x83\x10\x15aDuWaDd\x83aC\xE0V[\x82R` \x92\x83\x01\x92\x90\x91\x01\x90aDSV[\x96\x95PPPPPPV[_` \x82\x84\x03\x12\x15aD\x8FW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xA4W__\xFD[a#\xE6\x84\x82\x85\x01aC\xF9V[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aD\xF0W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aD\xC9V[P\x90\x95\x94PPPPPV[_` \x82\x84\x03\x12\x15aE\x0BW__\xFD[aE\x14\x82aC\xE0V[\x93\x92PPPV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[\x80Q\x82R_` \x82\x01Q`@` \x85\x01Ra#\xE6`@\x85\x01\x82aE\x1BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aE\x97W\x81Q\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aEyV[P\x93\x94\x93PPPPV[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q```@\x84\x01RaE\xCF`\x80\x84\x01\x82aEIV[\x90P`@\x84\x01Q`\x1F\x19\x84\x83\x03\x01``\x85\x01R\x80Q`@\x83RaE\xF5`@\x84\x01\x82aEgV[\x90P` \x82\x01Q\x91P\x82\x81\x03` \x84\x01RaDu\x81\x83aE\x1BV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aF^W`\x1F\x19\x85\x84\x03\x01\x88RaFH\x83\x83QaE\x1BV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aF,V[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aF\xCB\x90\x87\x01\x82aF\x10V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aF\x90V[P\x92\x96\x95PPPPPPV[` \x81R_aE\x14` \x83\x01\x84aE\x1BV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15aE\x97W\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01aG\x11V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87RaG\x85`@\x88\x01\x82aE\x1BV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01RaG\xA0\x81\x83aF\xFFV[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aG_V[` \x81R_aE\x14` \x83\x01\x84aF\x10V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15aF\xE1W\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90aH*\x90\x87\x01\x82aF\xFFV[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01aG\xEFV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15aF^W`\x1F\x19\x85\x84\x03\x01\x88RaHx\x83\x83QaEgV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01aH\\V[` \x81R`\x01`\x01`@\x1B\x03\x82Q\x16` \x82\x01R_` \x83\x01Q`\x80`@\x84\x01RaH\xBC`\xA0\x84\x01\x82aEIV[`@\x85\x01Q\x84\x82\x03`\x1F\x19\x01``\x86\x01R\x80Q\x80\x83R\x91\x92P` \x90\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01_[\x82\x81\x10\x15aI\x1AW`\x1F\x19\x86\x83\x03\x01\x84RaI\x05\x82\x86QaE\x1BV[` \x95\x86\x01\x95\x94\x90\x94\x01\x93\x91P`\x01\x01aH\xE9V[P``\x88\x01Q\x87\x82\x03`\x1F\x19\x01`\x80\x89\x01R\x94PaI8\x81\x86aH@V[\x98\x97PPPPPPPPV[__`@\x83\x85\x03\x12\x15aIUW__\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15aIjW__\xFD[aIv\x85\x82\x86\x01aC\xF9V[\x92PP` \x83\x015`\x01`\x01`@\x1B\x03\x81\x16\x81\x14aI\x92W__\xFD[\x80\x91PP\x92P\x92\x90PV[` \x81R_\x82Q`@` \x84\x01RaI\xB8``\x84\x01\x82aEIV[` \x85\x81\x01Q\x85\x83\x03`\x1F\x19\x01`@\x87\x01R\x80Q\x80\x84R\x92\x93P\x81\x01\x91\x81\x84\x01\x91`\x05\x82\x90\x1B\x85\x01\x01_[\x82\x81\x10\x15aJ5W`\x1F\x19\x86\x83\x03\x01\x84R\x84Q\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q\x90P```@\x84\x01RaJ\x1F``\x84\x01\x82aE\x1BV[` \x96\x87\x01\x96\x95\x90\x95\x01\x94\x92PP`\x01\x01aI\xE3V[P\x97\x96PPPPPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15aD\xF0W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01aJZV[_`\x01`\x01`@\x1B\x03\x82\x11\x15aJ\x90WaJ\x90aC\x9CV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15aJ\xAEW__\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aJ\xC3W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aJ\xD3W__\xFD[\x805aJ\xE6aJ\xE1\x82aJxV[aC\xB0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aJ\xFAW__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[` \x80\x82R`S\x90\x82\x01R\x7FBeaconChainMock: attempting to e`@\x82\x01R\x7Fxit dummy validator. We need tho``\x82\x01Rr\x0El\xA4\x0C\xCD\xEED\x0E\x0EM\xED\xEC\xCC\xEC\xAD\xC4\x07\xC7E`k\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01`\x01`@\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01\x81\x81\x1C\x90\x82\x16\x80aL\nW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x07\xEDWcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[\x80\x82\x01\x80\x82\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x1F\xA9Wa\x1F\xA9aK\xA4V[`\x01`\x01`@\x1B\x03\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x14\x07Wa\x14\x07aK\xA4V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[_\x82aL\xA9WaL\xA9aL\x87V[P\x04\x90V[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aL\xC4WaL\xC4aL\x87V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x04\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aL\xE7W__\xFD[PQ\x91\x90PV[_\x82aL\xFCWaL\xFCaL\x87V[P\x06\x90V[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_aM#\x82\x85aM\x01V[`\x17`\xF9\x1B\x81RaM7`\x01\x82\x01\x85aM\x01V[\x95\x94PPPPPV[`@\x81R_aMR`@\x83\x01\x85aE\x1BV[\x90P\x82` \x83\x01R\x93\x92PPPV[_aE\x14\x82\x84aM\x01V[`@\x81R_aM~`@\x83\x01\x85aE\x1BV[\x82\x81\x03` \x84\x01RaM7\x81\x85aE\x1BV[_`\x01\x82\x01aM\xA1WaM\xA1aK\xA4V[P`\x01\x01\x90V[`\x1F\x82\x11\x15a3\xB5W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15aM\xCDWP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0F;W_\x81U`\x01\x01aM\xD9V[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aN\x05WaN\x05aC\x9CV[aN\x19\x81aN\x13\x84TaK\xF6V[\x84aM\xA8V[` `\x1F\x82\x11`\x01\x81\x14aNKW_\x83\x15aN4WP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua\x0F;V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15aNzW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01aNZV[P\x84\x82\x10\x15aN\x97W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_d\xFF\xFF\xFF\xFF\xFF\x83\x16\x80aN\xBCWaN\xBCaL\x87V[\x80d\xFF\xFF\xFF\xFF\xFF\x84\x16\x06\x91PP\x92\x91PPV[_aN\xDA\x82\x85aM\x01V[`\x01`\x01`\x80\x1B\x03\x19\x93\x90\x93\x16\x83RPP`\x10\x01\x91\x90PV[_d\xFF\xFF\xFF\xFF\xFF\x82\x16d\xFF\xFF\xFF\xFF\xFF\x81\x03aO\x10WaO\x10aK\xA4V[`\x01\x01\x92\x91PPV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x02\x90\x81\x16\x90\x81\x81\x14a\x14\x07Wa\x14\x07aK\xA4V[\x80Q` \x80\x83\x01Q\x91\x90\x81\x10\x15a\x07\xEDW_\x19` \x91\x90\x91\x03`\x03\x1B\x1B\x16\x91\x90PV[d\xFF\xFF\xFF\xFF\xFF\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[d\xFF\xFF\xFF\xFF\xFF\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x1F\xA9Wa\x1F\xA9aK\xA4V[_` \x82\x84\x03\x12\x15aO\xA6W__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xBBW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13aO\xCBW__\xFD[\x80QaO\xD9aJ\xE1\x82aJxV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15aO\xEDW__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[_aM7aP!aP\x1B\x84\x88aM\x01V[\x86aM\x01V[\x84aM\x01V\xFE-- no validators; added empty block root- generated rewards for num validators\xA2dipfsX\"\x12 \xC5\xF3W\x9AsBV4\xAF\r\x92\x04\x92\xC2\xDD\xF1^P\x98\xD8:\xB6\x11\xE7\x1A9/\xCEN\x0B\xDFTdsolcC\0\x08\x1B\x003",
    );
    /**```solidity
    struct CheckpointProofs { BeaconChainProofs.BalanceContainerProof balanceContainerProof; BeaconChainProofs.BalanceProof[] balanceProofs; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckpointProofs {
        pub balanceContainerProof:
            <BeaconChainProofs::BalanceContainerProof as alloy::sol_types::SolType>::RustType,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for CheckpointProofs {
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
        impl alloy_sol_types::SolStruct for CheckpointProofs {
            const NAME: &'static str = "CheckpointProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckpointProofs(BeaconChainProofs.BalanceContainerProof balanceContainerProof,BeaconChainProofs.BalanceProof[] balanceProofs)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorFieldsProofs: alloy::sol_types::private::Vec<alloy::sol_types::private::Bytes>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
        impl alloy_sol_types::SolType for CredentialProofs {
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
        impl alloy_sol_types::SolStruct for CredentialProofs {
            const NAME: &'static str = "CredentialProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CredentialProofs(uint64 beaconTimestamp,BeaconChainProofs.StateRootProof stateRootProof,bytes[] validatorFieldsProofs,bytes32[][] validatorFields)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.validatorFields,
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
    struct StaleBalanceProofs { uint64 beaconTimestamp; BeaconChainProofs.StateRootProof stateRootProof; BeaconChainProofs.ValidatorProof validatorProof; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleBalanceProofs {
        pub beaconTimestamp: u64,
        pub stateRootProof:
            <BeaconChainProofs::StateRootProof as alloy::sol_types::SolType>::RustType,
        pub validatorProof:
            <BeaconChainProofs::ValidatorProof as alloy::sol_types::SolType>::RustType,
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                (
                    value.beaconTimestamp,
                    value.stateRootProof,
                    value.validatorProof,
                )
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
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self.beaconTimestamp,
                    ),
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
        impl alloy_sol_types::SolType for StaleBalanceProofs {
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
        impl alloy_sol_types::SolStruct for StaleBalanceProofs {
            const NAME: &'static str = "StaleBalanceProofs";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StaleBalanceProofs(uint64 beaconTimestamp,BeaconChainProofs.StateRootProof stateRootProof,BeaconChainProofs.ValidatorProof validatorProof)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8, 214u8,
                    9u8, 203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8, 202u8, 240u8,
                    233u8, 177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_address(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8, 71u8,
                    177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8, 120u8, 85u8,
                    214u8, 126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8,
                    181u8, 170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8, 92u8,
                    141u8, 4u8, 113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<256>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::I256>,
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Int<256>>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8, 237u8,
                    155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8, 163u8, 100u8,
                    124u8, 33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Int<256>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
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
            type DataTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_array(address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8, 45u8,
                    155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8, 159u8,
                    241u8, 3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Address,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.val
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8, 86u8,
                    3u8, 145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8, 86u8, 225u8,
                    26u8, 162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_bytes32(bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8, 88u8,
                    139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8, 115u8, 175u8,
                    212u8, 63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_int(int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8, 90u8,
                    140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8, 27u8, 113u8,
                    181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_address(string,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8, 16u8,
                    143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8, 67u8,
                    122u8, 97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,uint256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8, 56u8,
                    12u8, 115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8, 127u8, 201u8,
                    83u8, 40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
        pub val:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::I256>,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,int256[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8, 70u8,
                    17u8, 56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8, 247u8, 225u8,
                    123u8, 4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_array(string,address[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8, 207u8,
                    39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8, 228u8, 112u8,
                    223u8, 59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes(string,bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8, 217u8,
                    79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8, 89u8, 79u8,
                    213u8, 99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_bytes32(string,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8, 111u8,
                    146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8, 221u8, 184u8,
                    211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_int(string,int256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8, 89u8,
                    239u8, 36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8, 10u8,
                    232u8, 67u8, 78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.decimals,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_decimal_uint(string,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8, 67u8,
                    232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8, 128u8, 28u8,
                    19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.val,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.decimals,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_int(string,int256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8, 142u8,
                    151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8, 216u8, 31u8,
                    126u8, 142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Int<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_string(string,string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8, 101u8,
                    141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8, 243u8, 120u8,
                    83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_named_uint(string,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8,
                    253u8, 68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8, 197u8,
                    108u8, 129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
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
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_string(string)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8, 85u8,
                    131u8, 237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8, 50u8,
                    156u8, 79u8, 187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "log_uint(uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8, 82u8,
                    136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8, 239u8, 197u8,
                    66u8, 124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "logs(bytes)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8, 27u8,
                    245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8, 67u8, 8u8,
                    201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
                        &self._eigenPodManager,
                    ),
                    <alloy::sol_types::sol_data::Uint<64> as alloy_sol_types::SolType>::tokenize(
                        &self._genesisTime,
                    ),
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<CONSENSUS_REWARD_AMOUNT_GWEICall> for UnderlyingRustTuple<'_> {
                fn from(value: CONSENSUS_REWARD_AMOUNT_GWEICall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CONSENSUS_REWARD_AMOUNT_GWEICall {
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
            impl ::core::convert::From<CONSENSUS_REWARD_AMOUNT_GWEIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: CONSENSUS_REWARD_AMOUNT_GWEIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for CONSENSUS_REWARD_AMOUNT_GWEIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for CONSENSUS_REWARD_AMOUNT_GWEICall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = CONSENSUS_REWARD_AMOUNT_GWEIReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_TESTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = NAMEReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::String,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<SLASH_AMOUNT_GWEICall> for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_GWEICall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASH_AMOUNT_GWEICall {
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
            impl ::core::convert::From<SLASH_AMOUNT_GWEIReturn> for UnderlyingRustTuple<'_> {
                fn from(value: SLASH_AMOUNT_GWEIReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for SLASH_AMOUNT_GWEIReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for SLASH_AMOUNT_GWEICall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = SLASH_AMOUNT_GWEIReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = _advanceEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpochReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<advanceEpoch_NoRewardsCall> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoRewardsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpoch_NoRewardsCall {
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
            impl ::core::convert::From<advanceEpoch_NoRewardsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoRewardsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpoch_NoRewardsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for advanceEpoch_NoRewardsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpoch_NoRewardsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<advanceEpoch_NoWithdrawCall> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoWithdrawCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpoch_NoWithdrawCall {
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
            impl ::core::convert::From<advanceEpoch_NoWithdrawReturn> for UnderlyingRustTuple<'_> {
                fn from(value: advanceEpoch_NoWithdrawReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for advanceEpoch_NoWithdrawReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for advanceEpoch_NoWithdrawCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = advanceEpoch_NoWithdrawReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfGweiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
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
            impl ::core::convert::From<currentBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: currentBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for currentBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for currentBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentBalanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = currentEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<effectiveBalanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: effectiveBalanceCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for effectiveBalanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndex: tuple.0,
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
            impl ::core::convert::From<effectiveBalanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: effectiveBalanceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for effectiveBalanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for effectiveBalanceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = effectiveBalanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
        pub excludedArtifacts_: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            impl ::core::convert::From<excludeArtifactsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeArtifactsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::String>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeArtifactsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeArtifactsReturn) -> Self {
                    (value.excludedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeArtifactsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeArtifactsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub excludedContracts_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeContractsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeContractsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<excludeContractsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeContractsReturn) -> Self {
                    (value.excludedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeContractsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeContractsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `excludeSelectors()` and selector `0xb0464fdc`.
    ```solidity
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsCall {}
    ///Container type for the return parameters of the [`excludeSelectors()`](excludeSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct excludeSelectorsReturn {
        pub excludedSelectors_: alloy::sol_types::private::Vec<
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<excludeSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSelectorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSelectors_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSelectorsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "excludeSelectors()";
            const SELECTOR: [u8; 4] = [176u8, 70u8, 79u8, 220u8];
            #[inline]
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
        pub excludedSenders_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<excludeSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: excludeSendersReturn) -> Self {
                    (value.excludedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for excludeSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        excludedSenders_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for excludeSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSendersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitEpochReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
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
            impl ::core::convert::From<exitValidatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: exitValidatorReturn) -> Self {
                    (value.exitedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exitValidatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        exitedBalanceGwei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exitValidatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = exitValidatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
    /**Function with signature `failed()` and selector `0xba414fa6`.
    ```solidity
    function failed() external view returns (bool);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = failedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
                    }
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
            impl ::core::convert::From<getBalanceRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBalanceRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceRootCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
    /**Function with signature `getCheckpointProofs(uint40[],uint64)` and selector `0xb1b6f6a1`.
    ```solidity
    function getCheckpointProofs(uint40[] memory _validators, uint64 timestamp) external view returns (CheckpointProofs memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckpointProofsCall {
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<getCheckpointProofsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckpointProofsCall) -> Self {
                    (value._validators, value.timestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckpointProofsCall {
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
            type UnderlyingRustTuple<'a> =
                (<CheckpointProofs as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCheckpointProofsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckpointProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckpointProofsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckpointProofsReturn;
            type ReturnTuple<'a> = (CheckpointProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<getCredentialProofsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCredentialProofsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCredentialProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (CredentialProofs,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<CredentialProofs as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCredentialProofsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCredentialProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCredentialProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCredentialProofsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCredentialProofsReturn;
            type ReturnTuple<'a> = (CredentialProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._validators
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
    /**Function with signature `getPubkeyHashes(uint40[])` and selector `0xc76f25c0`.
    ```solidity
    function getPubkeyHashes(uint40[] memory _validators) external view returns (bytes32[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPubkeyHashesCall {
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
    }
    ///Container type for the return parameters of the [`getPubkeyHashes(uint40[])`](getPubkeyHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getPubkeyHashesReturn {
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<getPubkeyHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getPubkeyHashesCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPubkeyHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getPubkeyHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getPubkeyHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getPubkeyHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getPubkeyHashesCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getPubkeyHashesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._validators
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStaleBalanceProofsCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStaleBalanceProofsCall) -> Self {
                    (value.validatorIndex,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStaleBalanceProofsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndex: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (StaleBalanceProofs,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (<StaleBalanceProofs as alloy::sol_types::SolType>::RustType,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStaleBalanceProofsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStaleBalanceProofsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStaleBalanceProofsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStaleBalanceProofsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStaleBalanceProofsReturn;
            type ReturnTuple<'a> = (StaleBalanceProofs,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isActiveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        withdrawalCreds: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = newValidatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<40>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = nextTimestampReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U40,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    Self {
                        validatorIndex: tuple.0,
                    }
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = pubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<40> as alloy_sol_types::SolType>::tokenize(
                        &self.validatorIndex,
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
    /**Function with signature `slashValidators(uint40[])` and selector `0x14360958`.
    ```solidity
    function slashValidators(uint40[] memory _validators) external returns (uint64 slashedBalanceGwei);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashValidatorsCall {
        pub _validators:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<slashValidatorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashValidatorsCall) -> Self {
                    (value._validators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashValidatorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _validators: tuple.0,
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
            impl ::core::convert::From<slashValidatorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashValidatorsReturn) -> Self {
                    (value.slashedBalanceGwei,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashValidatorsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        slashedBalanceGwei: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashValidatorsCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashValidatorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<64>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self._validators
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
    /**Function with signature `targetArtifactSelectors()` and selector `0x66d9a9a0`.
    ```solidity
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsCall {}
    ///Container type for the return parameters of the [`targetArtifactSelectors()`](targetArtifactSelectorsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetArtifactSelectorsReturn {
        pub targetedArtifactSelectors_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactSelectorsCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactSelectorsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetArtifactSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactSelectorsReturn) -> Self {
                    (value.targetedArtifactSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactSelectorsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedArtifacts_: alloy::sol_types::private::Vec<alloy::sol_types::private::String>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> =
                (alloy::sol_types::private::Vec<alloy::sol_types::private::String>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetArtifactsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetArtifactsReturn) -> Self {
                    (value.targetedArtifacts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetArtifactsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetArtifactsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::String>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedContracts_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetContractsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetContractsReturn) -> Self {
                    (value.targetedContracts_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetContractsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetContractsReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `targetInterfaces()` and selector `0x2ade3880`.
    ```solidity
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesCall {}
    ///Container type for the return parameters of the [`targetInterfaces()`](targetInterfacesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct targetInterfacesReturn {
        pub targetedInterfaces_: alloy::sol_types::private::Vec<
            <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall> for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetInterfacesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedInterfaces_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetInterfacesCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "targetInterfaces()";
            const SELECTOR: [u8; 4] = [42u8, 222u8, 56u8, 128u8];
            #[inline]
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzSelector as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetSelectorsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSelectorsReturn) -> Self {
                    (value.targetedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSelectorsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub targetedSenders_: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
            impl ::core::convert::From<targetSendersReturn> for UnderlyingRustTuple<'_> {
                fn from(value: targetSendersReturn) -> Self {
                    (value.targetedSenders_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for targetSendersReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        targetedSenders_: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for targetSendersCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetSendersReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub validatorIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U40>,
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
            impl ::core::convert::From<totalEffectiveBalanceWeiCall> for UnderlyingRustTuple<'_> {
                fn from(value: totalEffectiveBalanceWeiCall) -> Self {
                    (value.validatorIndices,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalEffectiveBalanceWeiCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        validatorIndices: tuple.0,
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
            impl ::core::convert::From<totalEffectiveBalanceWeiReturn> for UnderlyingRustTuple<'_> {
                fn from(value: totalEffectiveBalanceWeiReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalEffectiveBalanceWeiReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for totalEffectiveBalanceWeiCall {
            type Parameters<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<40>>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = totalEffectiveBalanceWeiReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                (<alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<40>,
                > as alloy_sol_types::SolType>::tokenize(
                    &self.validatorIndices,
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
        excludeSelectors(excludeSelectorsCall),
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
        targetInterfaces(targetInterfacesCall),
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
            [42u8, 222u8, 56u8, 128u8],
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
            [176u8, 70u8, 79u8, 220u8],
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
        const COUNT: usize = 37usize;
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
                Self::_advanceEpoch(_) => <_advanceEpochCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::advanceEpoch(_) => <advanceEpochCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::advanceEpoch_NoRewards(_) => {
                    <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::advanceEpoch_NoWithdraw(_) => {
                    <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::balanceOfGwei(_) => <balanceOfGweiCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::currentBalance(_) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::currentEpoch(_) => <currentEpochCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::effectiveBalance(_) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeArtifacts(_) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeContracts(_) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSelectors(_) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::excludeSenders(_) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exitEpoch(_) => <exitEpochCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::exitValidator(_) => <exitValidatorCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::newValidator(_) => <newValidatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::nextTimestamp(_) => <nextTimestampCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pubkey(_) => <pubkeyCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pubkeyHash(_) => <pubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::targetInterfaces(_) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSelectors(_) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::targetSenders(_) => <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<BeaconChainMockCalls>] = &[
                {
                    fn slashValidators(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <slashValidatorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <pubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(BeaconChainMockCalls::_advanceEpoch)
                    }
                    _advanceEpoch
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BeaconChainMockCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn balanceOfGwei(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <pubkeyCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <NAMECall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <isActiveCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BeaconChainMockCalls::isActive)
                    }
                    isActive
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(BeaconChainMockCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn getCheckpointProofs(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<BeaconChainMockCalls> {
                        <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <exitEpochCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(BeaconChainMockCalls::IS_TEST)
                    }
                    IS_TEST
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
                    <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::_advanceEpoch(inner) => {
                    <_advanceEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::advanceEpoch(inner) => {
                    <advanceEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                    <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::currentBalance(inner) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::effectiveBalance(inner) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exitEpoch(inner) => {
                    <exitEpochCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exitValidator(inner) => {
                    <exitValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getCheckpointProofs(inner) => {
                    <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getCredentialProofs(inner) => {
                    <getCredentialProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getPubkeyHashes(inner) => {
                    <getPubkeyHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getStaleBalanceProofs(inner) => {
                    <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isActive(inner) => {
                    <isActiveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::newValidator(inner) => {
                    <newValidatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::nextTimestamp(inner) => {
                    <nextTimestampCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pubkey(inner) => {
                    <pubkeyCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::pubkeyHash(inner) => {
                    <pubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::slashValidators(inner) => {
                    <slashValidatorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                        inner, out,
                    )
                }
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::NAME(inner) => {
                    <NAMECall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::SLASH_AMOUNT_GWEI(inner) => {
                    <SLASH_AMOUNT_GWEICall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::_advanceEpoch(inner) => {
                    <_advanceEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::advanceEpoch(inner) => {
                    <advanceEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::advanceEpoch_NoRewards(inner) => {
                    <advanceEpoch_NoRewardsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::advanceEpoch_NoWithdraw(inner) => {
                    <advanceEpoch_NoWithdrawCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::balanceOfGwei(inner) => {
                    <balanceOfGweiCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::currentBalance(inner) => {
                    <currentBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::currentEpoch(inner) => {
                    <currentEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::effectiveBalance(inner) => {
                    <effectiveBalanceCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeArtifacts(inner) => {
                    <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeContracts(inner) => {
                    <excludeContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::excludeSenders(inner) => {
                    <excludeSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::exitEpoch(inner) => {
                    <exitEpochCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::exitValidator(inner) => {
                    <exitValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::failed(inner) => {
                    <failedCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBalanceRoot(inner) => {
                    <getBalanceRootCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCheckpointProofs(inner) => {
                    <getCheckpointProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getCredentialProofs(inner) => {
                    <getCredentialProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getPubkeyHashes(inner) => {
                    <getPubkeyHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getStaleBalanceProofs(inner) => {
                    <getStaleBalanceProofsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::isActive(inner) => {
                    <isActiveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::newValidator(inner) => {
                    <newValidatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::nextTimestamp(inner) => {
                    <nextTimestampCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pubkey(inner) => {
                    <pubkeyCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pubkeyHash(inner) => {
                    <pubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::slashValidators(inner) => {
                    <slashValidatorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetArtifactSelectors(inner) => {
                    <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::targetArtifacts(inner) => {
                    <targetArtifactsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetContracts(inner) => {
                    <targetContractsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSelectors(inner) => {
                    <targetSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::targetSenders(inner) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::totalEffectiveBalanceWei(inner) => {
                    <totalEffectiveBalanceWeiCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
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
                0u8, 170u8, 163u8, 156u8, 159u8, 251u8, 95u8, 86u8, 122u8, 69u8, 52u8, 56u8, 12u8,
                115u8, 112u8, 117u8, 112u8, 46u8, 31u8, 127u8, 20u8, 16u8, 127u8, 201u8, 83u8,
                40u8, 227u8, 181u8, 108u8, 3u8, 37u8, 251u8,
            ],
            [
                11u8, 46u8, 19u8, 255u8, 32u8, 172u8, 123u8, 71u8, 65u8, 152u8, 101u8, 85u8, 131u8,
                237u8, 247u8, 13u8, 237u8, 210u8, 193u8, 220u8, 152u8, 14u8, 50u8, 156u8, 79u8,
                187u8, 47u8, 192u8, 116u8, 139u8, 121u8, 107u8,
            ],
            [
                14u8, 181u8, 213u8, 38u8, 36u8, 200u8, 210u8, 138u8, 218u8, 159u8, 197u8, 90u8,
                140u8, 80u8, 46u8, 213u8, 170u8, 63u8, 190u8, 47u8, 182u8, 233u8, 27u8, 113u8,
                181u8, 243u8, 118u8, 136u8, 43u8, 29u8, 47u8, 184u8,
            ],
            [
                35u8, 182u8, 42u8, 208u8, 88u8, 77u8, 36u8, 167u8, 95u8, 11u8, 243u8, 86u8, 3u8,
                145u8, 239u8, 86u8, 89u8, 236u8, 109u8, 177u8, 38u8, 156u8, 86u8, 225u8, 26u8,
                162u8, 65u8, 214u8, 55u8, 241u8, 155u8, 32u8,
            ],
            [
                40u8, 15u8, 68u8, 70u8, 178u8, 138u8, 19u8, 114u8, 65u8, 125u8, 218u8, 101u8,
                141u8, 48u8, 185u8, 91u8, 41u8, 146u8, 177u8, 42u8, 201u8, 199u8, 243u8, 120u8,
                83u8, 95u8, 41u8, 169u8, 122u8, 207u8, 53u8, 131u8,
            ],
            [
                44u8, 171u8, 151u8, 144u8, 81u8, 15u8, 216u8, 189u8, 251u8, 210u8, 17u8, 82u8,
                136u8, 219u8, 51u8, 254u8, 198u8, 102u8, 145u8, 212u8, 118u8, 239u8, 197u8, 66u8,
                124u8, 253u8, 76u8, 9u8, 105u8, 48u8, 23u8, 85u8,
            ],
            [
                47u8, 230u8, 50u8, 119u8, 145u8, 116u8, 55u8, 67u8, 120u8, 68u8, 42u8, 142u8,
                151u8, 139u8, 204u8, 251u8, 220u8, 193u8, 214u8, 178u8, 176u8, 216u8, 31u8, 126u8,
                142u8, 183u8, 118u8, 171u8, 34u8, 134u8, 241u8, 104u8,
            ],
            [
                59u8, 207u8, 178u8, 174u8, 46u8, 141u8, 19u8, 45u8, 209u8, 252u8, 231u8, 207u8,
                39u8, 138u8, 154u8, 25u8, 117u8, 106u8, 159u8, 206u8, 171u8, 228u8, 112u8, 223u8,
                59u8, 218u8, 187u8, 75u8, 197u8, 119u8, 209u8, 189u8,
            ],
            [
                64u8, 225u8, 132u8, 15u8, 87u8, 105u8, 7u8, 61u8, 97u8, 189u8, 1u8, 55u8, 45u8,
                155u8, 117u8, 186u8, 169u8, 132u8, 45u8, 86u8, 41u8, 160u8, 201u8, 159u8, 241u8,
                3u8, 190u8, 17u8, 120u8, 168u8, 233u8, 226u8,
            ],
            [
                65u8, 48u8, 79u8, 172u8, 217u8, 50u8, 61u8, 117u8, 177u8, 27u8, 205u8, 214u8, 9u8,
                203u8, 56u8, 239u8, 255u8, 253u8, 176u8, 87u8, 16u8, 247u8, 202u8, 240u8, 233u8,
                177u8, 108u8, 109u8, 157u8, 112u8, 159u8, 80u8,
            ],
            [
                93u8, 166u8, 206u8, 157u8, 81u8, 21u8, 27u8, 161u8, 12u8, 9u8, 165u8, 89u8, 239u8,
                36u8, 213u8, 32u8, 185u8, 218u8, 197u8, 197u8, 184u8, 129u8, 10u8, 232u8, 67u8,
                78u8, 77u8, 13u8, 134u8, 65u8, 26u8, 149u8,
            ],
            [
                122u8, 231u8, 76u8, 82u8, 116u8, 20u8, 174u8, 19u8, 95u8, 217u8, 112u8, 71u8,
                177u8, 41u8, 33u8, 165u8, 236u8, 57u8, 17u8, 184u8, 4u8, 25u8, 120u8, 85u8, 214u8,
                126u8, 37u8, 199u8, 183u8, 94u8, 230u8, 243u8,
            ],
            [
                137u8, 10u8, 130u8, 103u8, 155u8, 71u8, 15u8, 43u8, 216u8, 40u8, 22u8, 237u8,
                155u8, 22u8, 31u8, 151u8, 216u8, 185u8, 103u8, 243u8, 127u8, 163u8, 100u8, 124u8,
                33u8, 213u8, 191u8, 57u8, 116u8, 158u8, 45u8, 213u8,
            ],
            [
                156u8, 78u8, 133u8, 65u8, 202u8, 143u8, 13u8, 193u8, 196u8, 19u8, 249u8, 16u8,
                143u8, 102u8, 216u8, 45u8, 60u8, 236u8, 177u8, 189u8, 219u8, 206u8, 67u8, 122u8,
                97u8, 202u8, 163u8, 23u8, 92u8, 76u8, 201u8, 111u8,
            ],
            [
                167u8, 62u8, 218u8, 9u8, 102u8, 47u8, 70u8, 221u8, 231u8, 41u8, 190u8, 70u8, 17u8,
                56u8, 95u8, 243u8, 79u8, 230u8, 196u8, 79u8, 187u8, 198u8, 247u8, 225u8, 123u8,
                4u8, 43u8, 89u8, 163u8, 68u8, 91u8, 87u8,
            ],
            [
                175u8, 183u8, 149u8, 201u8, 198u8, 30u8, 79u8, 231u8, 70u8, 140u8, 56u8, 111u8,
                146u8, 93u8, 122u8, 84u8, 41u8, 236u8, 173u8, 156u8, 4u8, 149u8, 221u8, 184u8,
                211u8, 141u8, 105u8, 6u8, 20u8, 211u8, 47u8, 153u8,
            ],
            [
                178u8, 222u8, 47u8, 190u8, 128u8, 26u8, 13u8, 246u8, 192u8, 203u8, 221u8, 253u8,
                68u8, 139u8, 163u8, 196u8, 29u8, 72u8, 160u8, 64u8, 202u8, 53u8, 197u8, 108u8,
                129u8, 150u8, 239u8, 15u8, 202u8, 231u8, 33u8, 168u8,
            ],
            [
                210u8, 110u8, 22u8, 202u8, 212u8, 84u8, 135u8, 5u8, 228u8, 201u8, 226u8, 217u8,
                79u8, 152u8, 238u8, 145u8, 194u8, 137u8, 8u8, 94u8, 228u8, 37u8, 89u8, 79u8, 213u8,
                99u8, 95u8, 162u8, 150u8, 76u8, 207u8, 24u8,
            ],
            [
                231u8, 149u8, 14u8, 222u8, 3u8, 148u8, 185u8, 242u8, 206u8, 74u8, 90u8, 27u8,
                245u8, 167u8, 225u8, 133u8, 36u8, 17u8, 247u8, 230u8, 102u8, 27u8, 67u8, 8u8,
                201u8, 19u8, 196u8, 191u8, 209u8, 16u8, 39u8, 228u8,
            ],
            [
                232u8, 22u8, 153u8, 184u8, 81u8, 19u8, 238u8, 161u8, 199u8, 62u8, 16u8, 88u8,
                139u8, 43u8, 3u8, 94u8, 85u8, 137u8, 51u8, 105u8, 99u8, 33u8, 115u8, 175u8, 212u8,
                63u8, 235u8, 25u8, 47u8, 172u8, 100u8, 227u8,
            ],
            [
                235u8, 139u8, 164u8, 60u8, 237u8, 117u8, 55u8, 66u8, 25u8, 70u8, 189u8, 67u8,
                232u8, 40u8, 184u8, 178u8, 184u8, 66u8, 137u8, 39u8, 170u8, 143u8, 128u8, 28u8,
                19u8, 217u8, 52u8, 191u8, 17u8, 172u8, 165u8, 123u8,
            ],
            [
                251u8, 16u8, 40u8, 101u8, 213u8, 10u8, 221u8, 221u8, 246u8, 157u8, 169u8, 181u8,
                170u8, 27u8, 206u8, 214u8, 108u8, 128u8, 207u8, 134u8, 154u8, 92u8, 141u8, 4u8,
                113u8, 164u8, 103u8, 225u8, 140u8, 233u8, 202u8, 177u8,
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
                    <log as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log)
                }
                Some(<log_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_address as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_address)
                }
                Some(<log_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_0)
                }
                Some(<log_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_1)
                }
                Some(<log_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_array_2)
                }
                Some(<log_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_bytes)
                }
                Some(<log_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_bytes32)
                }
                Some(<log_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_int as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_int)
                }
                Some(<log_named_address as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_address as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_address)
                }
                Some(<log_named_array_0 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_0 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_0)
                }
                Some(<log_named_array_1 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_1 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_1)
                }
                Some(<log_named_array_2 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_array_2 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_array_2)
                }
                Some(<log_named_bytes as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_bytes)
                }
                Some(<log_named_bytes32 as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_bytes32 as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_bytes32)
                }
                Some(<log_named_decimal_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_decimal_int as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_decimal_int)
                }
                Some(<log_named_decimal_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_decimal_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_decimal_uint)
                }
                Some(<log_named_int as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_int as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_int)
                }
                Some(<log_named_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_string as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_string)
                }
                Some(<log_named_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_named_uint as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_named_uint)
                }
                Some(<log_string as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_string as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::log_string)
                }
                Some(<log_uint as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <log_uint as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::log_uint)
                }
                Some(<logs as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <logs as alloy_sol_types::SolEvent>::decode_raw_log(topics, data, validate)
                        .map(Self::logs)
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
    impl alloy_sol_types::private::IntoLogData for BeaconChainMockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::log_bytes(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::log_bytes32(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::log_int(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
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
                Self::log_uint(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::logs(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::log(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
                Self::log_int(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
                Self::logs(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<BeaconChainMockInstance<T, P, N>>>
    {
        BeaconChainMockInstance::<T, P, N>::deploy(provider, _eigenPodManager, _genesisTime)
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
        BeaconChainMockInstance::<T, P, N>::deploy_builder(provider, _eigenPodManager, _genesisTime)
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
            f.debug_tuple("BeaconChainMockInstance")
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
        > BeaconChainMockInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BeaconChainMock`](self) contract instance.

        See the [wrapper's documentation](`BeaconChainMockInstance`) for more details.*/
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
            _eigenPodManager: alloy::sol_types::private::Address,
            _genesisTime: u64,
        ) -> alloy_contract::Result<BeaconChainMockInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _eigenPodManager, _genesisTime);
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _eigenPodManager,
                        _genesisTime,
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
        > BeaconChainMockInstance<T, P, N>
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
        ///Creates a new call builder for the [`CONSENSUS_REWARD_AMOUNT_GWEI`] function.
        pub fn CONSENSUS_REWARD_AMOUNT_GWEI(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, CONSENSUS_REWARD_AMOUNT_GWEICall, N> {
            self.call_builder(&CONSENSUS_REWARD_AMOUNT_GWEICall {})
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
        pub fn _advanceEpoch(&self) -> alloy_contract::SolCallBuilder<T, &P, _advanceEpochCall, N> {
            self.call_builder(&_advanceEpochCall {})
        }
        ///Creates a new call builder for the [`advanceEpoch`] function.
        pub fn advanceEpoch(&self) -> alloy_contract::SolCallBuilder<T, &P, advanceEpochCall, N> {
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
            self.call_builder(&balanceOfGweiCall { validatorIndex })
        }
        ///Creates a new call builder for the [`currentBalance`] function.
        pub fn currentBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, currentBalanceCall, N> {
            self.call_builder(&currentBalanceCall { validatorIndex })
        }
        ///Creates a new call builder for the [`currentEpoch`] function.
        pub fn currentEpoch(&self) -> alloy_contract::SolCallBuilder<T, &P, currentEpochCall, N> {
            self.call_builder(&currentEpochCall {})
        }
        ///Creates a new call builder for the [`effectiveBalance`] function.
        pub fn effectiveBalance(
            &self,
            validatorIndex: alloy::sol_types::private::primitives::aliases::U40,
        ) -> alloy_contract::SolCallBuilder<T, &P, effectiveBalanceCall, N> {
            self.call_builder(&effectiveBalanceCall { validatorIndex })
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
        ///Creates a new call builder for the [`excludeSelectors`] function.
        pub fn excludeSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, excludeSelectorsCall, N> {
            self.call_builder(&excludeSelectorsCall {})
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
            self.call_builder(&exitValidatorCall { validatorIndex })
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
            self.call_builder(&getBalanceRootCall { validatorIndex })
        }
        ///Creates a new call builder for the [`getCheckpointProofs`] function.
        pub fn getCheckpointProofs(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
            timestamp: u64,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckpointProofsCall, N> {
            self.call_builder(&getCheckpointProofsCall {
                _validators,
                timestamp,
            })
        }
        ///Creates a new call builder for the [`getCredentialProofs`] function.
        pub fn getCredentialProofs(
            &self,
            _validators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCredentialProofsCall, N> {
            self.call_builder(&getCredentialProofsCall { _validators })
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
            self.call_builder(&getStaleBalanceProofsCall { validatorIndex })
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
            self.call_builder(&newValidatorCall { withdrawalCreds })
        }
        ///Creates a new call builder for the [`nextTimestamp`] function.
        pub fn nextTimestamp(&self) -> alloy_contract::SolCallBuilder<T, &P, nextTimestampCall, N> {
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
        ///Creates a new call builder for the [`targetInterfaces`] function.
        pub fn targetInterfaces(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetInterfacesCall, N> {
            self.call_builder(&targetInterfacesCall {})
        }
        ///Creates a new call builder for the [`targetSelectors`] function.
        pub fn targetSelectors(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSelectorsCall, N> {
            self.call_builder(&targetSelectorsCall {})
        }
        ///Creates a new call builder for the [`targetSenders`] function.
        pub fn targetSenders(&self) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
        ///Creates a new call builder for the [`totalEffectiveBalanceWei`] function.
        pub fn totalEffectiveBalanceWei(
            &self,
            validatorIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U40,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, totalEffectiveBalanceWeiCall, N> {
            self.call_builder(&totalEffectiveBalanceWeiCall { validatorIndices })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BeaconChainMockInstance<T, P, N>
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
        ///Creates a new event filter for the [`log`] event.
        pub fn log_filter(&self) -> alloy_contract::Event<T, &P, log, N> {
            self.event_filter::<log>()
        }
        ///Creates a new event filter for the [`log_address`] event.
        pub fn log_address_filter(&self) -> alloy_contract::Event<T, &P, log_address, N> {
            self.event_filter::<log_address>()
        }
        ///Creates a new event filter for the [`log_array_0`] event.
        pub fn log_array_0_filter(&self) -> alloy_contract::Event<T, &P, log_array_0, N> {
            self.event_filter::<log_array_0>()
        }
        ///Creates a new event filter for the [`log_array_1`] event.
        pub fn log_array_1_filter(&self) -> alloy_contract::Event<T, &P, log_array_1, N> {
            self.event_filter::<log_array_1>()
        }
        ///Creates a new event filter for the [`log_array_2`] event.
        pub fn log_array_2_filter(&self) -> alloy_contract::Event<T, &P, log_array_2, N> {
            self.event_filter::<log_array_2>()
        }
        ///Creates a new event filter for the [`log_bytes`] event.
        pub fn log_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_bytes, N> {
            self.event_filter::<log_bytes>()
        }
        ///Creates a new event filter for the [`log_bytes32`] event.
        pub fn log_bytes32_filter(&self) -> alloy_contract::Event<T, &P, log_bytes32, N> {
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
        pub fn log_named_bytes_filter(&self) -> alloy_contract::Event<T, &P, log_named_bytes, N> {
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
        pub fn log_named_int_filter(&self) -> alloy_contract::Event<T, &P, log_named_int, N> {
            self.event_filter::<log_named_int>()
        }
        ///Creates a new event filter for the [`log_named_string`] event.
        pub fn log_named_string_filter(&self) -> alloy_contract::Event<T, &P, log_named_string, N> {
            self.event_filter::<log_named_string>()
        }
        ///Creates a new event filter for the [`log_named_uint`] event.
        pub fn log_named_uint_filter(&self) -> alloy_contract::Event<T, &P, log_named_uint, N> {
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
