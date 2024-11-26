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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for FuzzArtifactSelector {
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
        impl alloy_sol_types::SolStruct for FuzzArtifactSelector {
            const NAME: &'static str = "FuzzArtifactSelector";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzArtifactSelector(string artifact,bytes4[] selectors)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
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
        impl alloy_sol_types::SolType for FuzzInterface {
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
        impl alloy_sol_types::SolStruct for FuzzInterface {
            const NAME: &'static str = "FuzzInterface";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "FuzzInterface(address addr,string[] artifacts)",
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
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

interface ProofParsing {
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

    function IS_TEST() external view returns (bool);
    function excludeArtifacts() external view returns (string[] memory excludedArtifacts_);
    function excludeContracts() external view returns (address[] memory excludedContracts_);
    function excludeSelectors() external view returns (StdInvariant.FuzzSelector[] memory excludedSelectors_);
    function excludeSenders() external view returns (address[] memory excludedSenders_);
    function failed() external view returns (bool);
    function getBalanceUpdateProof() external returns (bytes memory);
    function getBeaconStateRoot() external view returns (bytes32);
    function getBlockHeaderProof() external returns (bytes32[18] memory);
    function getBlockRoot() external view returns (bytes32);
    function getBlockRootIndex() external view returns (uint256);
    function getExecutionPayloadProof() external returns (bytes32[7] memory);
    function getExecutionPayloadRoot() external view returns (bytes32);
    function getHistoricalSummaryIndex() external view returns (uint256);
    function getHistoricalSummaryProof() external returns (bytes32[44] memory);
    function getLatestBlockRoot() external view returns (bytes32);
    function getSlot() external view returns (uint256);
    function getSlotProof() external returns (bytes32[3] memory);
    function getSlotRoot() external view returns (bytes32);
    function getStateRootProof() external returns (bytes memory);
    function getTimestampProofCapella() external returns (bytes32[4] memory);
    function getTimestampProofDeneb() external returns (bytes32[5] memory);
    function getTimestampRoot() external view returns (bytes32);
    function getValidatorFields() external returns (bytes32[] memory);
    function getValidatorFieldsProof() external returns (bytes32[] memory);
    function getValidatorIndex() external view returns (uint256);
    function getValidatorProof() external returns (bytes32[46] memory);
    function getValidatorPubkeyHash() external view returns (bytes32);
    function getWithdrawalCredentialProof() external returns (bytes memory);
    function getWithdrawalFields() external returns (bytes32[] memory);
    function getWithdrawalIndex() external view returns (uint256);
    function getWithdrawalProofCapella() external returns (bytes32[9] memory);
    function getWithdrawalProofDeneb() external returns (bytes32[10] memory);
    function setJSON(string memory path) external;
    function targetArtifactSelectors() external view returns (StdInvariant.FuzzArtifactSelector[] memory targetedArtifactSelectors_);
    function targetArtifacts() external view returns (string[] memory targetedArtifacts_);
    function targetContracts() external view returns (address[] memory targetedContracts_);
    function targetInterfaces() external view returns (StdInvariant.FuzzInterface[] memory targetedInterfaces_);
    function targetSelectors() external view returns (StdInvariant.FuzzSelector[] memory targetedSelectors_);
    function targetSenders() external view returns (address[] memory targetedSenders_);
}
```

...which was generated by the following JSON ABI:
```json
[
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
    "name": "getBalanceUpdateProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBeaconStateRoot",
    "inputs": [],
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
    "name": "getBlockHeaderProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[18]",
        "internalType": "bytes32[18]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getBlockRoot",
    "inputs": [],
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
    "name": "getBlockRootIndex",
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
    "name": "getExecutionPayloadProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[7]",
        "internalType": "bytes32[7]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getExecutionPayloadRoot",
    "inputs": [],
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
    "name": "getHistoricalSummaryIndex",
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
    "name": "getHistoricalSummaryProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[44]",
        "internalType": "bytes32[44]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getLatestBlockRoot",
    "inputs": [],
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
    "name": "getSlot",
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
    "name": "getSlotProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[3]",
        "internalType": "bytes32[3]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getSlotRoot",
    "inputs": [],
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
    "name": "getStateRootProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampProofCapella",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[4]",
        "internalType": "bytes32[4]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampProofDeneb",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[5]",
        "internalType": "bytes32[5]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getTimestampRoot",
    "inputs": [],
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
    "name": "getValidatorFields",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorFieldsProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorIndex",
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
    "name": "getValidatorProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[46]",
        "internalType": "bytes32[46]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getValidatorPubkeyHash",
    "inputs": [],
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
    "name": "getWithdrawalCredentialProof",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalFields",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalIndex",
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
    "name": "getWithdrawalProofCapella",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[9]",
        "internalType": "bytes32[9]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "getWithdrawalProofDeneb",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[10]",
        "internalType": "bytes32[10]"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setJSON",
    "inputs": [
      {
        "name": "path",
        "type": "string",
        "internalType": "string"
      }
    ],
    "outputs": [],
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
pub mod ProofParsing {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x6080604052600c8054600160ff199182168117909255601f80549091169091179055348015602b575f5ffd5b506132e8806100395f395ff3fe608060405234801561000f575f5ffd5b506004361061023f575f3560e01c8063765aa60611610135578063b38061bf116100b4578063db364a4011610079578063db364a4014610441578063df18c78314610456578063e20c9f711461046b578063f148082c14610473578063fa7626d41461047b575f5ffd5b8063b38061bf14610409578063b5508aa914610411578063ba414fa614610419578063d6461cbb14610431578063d944472f14610439575f5ffd5b80639acfac29116100fa5780639acfac29146103ba5780639de4a9b3146103cf578063a157ef4c146103e4578063a5077429146103ec578063b0464fdc14610401575f5ffd5b8063765aa6061461037857806385226c8114610380578063893893ca14610395578063916a17c61461039d578063950bb682146103b2575f5ffd5b806334e3d90e116101c15780634c38f913116101865780634c38f9131461032957806364c725351461033e57806364f38ed71461035357806366d9a9a01461035b5780636c877c8414610370575f5ffd5b806334e3d90e146102f45780633e5e3c23146102fc5780633f7286f414610304578063428647341461030c5780634c20f51014610314575f5ffd5b80631ed7831c116102075780631ed7831c14610298578063275378b1146102ad5780632872e20c146102b55780632ade3880146102ca5780632fd1793c146102df575f5ffd5b806308a4d71f1461024357806316f0715314610258578063177541fc1461027357806318aadf311461027b578063196cdfbd14610283575b5f5ffd5b610256610251366004612957565b610488565b005b61026061050a565b6040519081526020015b60405180910390f35b6102606105c3565b61026061067e565b61028b61073f565b60405161026a91906129d1565b6102a0610976565b60405161026a9190612a01565b6102606109d5565b6102bd610a90565b60405161026a9190612a4c565b6102d2610bb8565b60405161026a9190612afc565b6102e7610cf4565b60405161026a9190612b7f565b6102e7610e3b565b6102a0610f58565b6102a0610fb6565b610260611014565b61031c6110d4565b60405161026a9190612b91565b6103316111fa565b60405161026a9190612bc8565b610346611322565b60405161026a9190612bf0565b61026061144a565b610363611505565b60405161026a9190612c5c565b610260611669565b610260611719565b6103886117d3565b60405161026a9190612cda565b61031c61189e565b6103a56119be565b60405161026a9190612cec565b61031c611a9f565b6103c2611bbd565b60405161026a9190612d63565b6103d7611ce5565b60405161026a9190612d8b565b6102e7611e0c565b6103f4611e16565b60405161026a9190612db2565b6103a5611f3e565b61026061201f565b6103886120d8565b6104216121a3565b604051901515815260200161026a565b61026061223c565b610260612304565b6104496123c4565b60405161026a9190612dda565b61045e6124eb565b60405161026a9190612e01565b6102a0612612565b610260612670565b601f546104219060ff1681565b6040516360f9bb1160e01b81525f5160206132935f395f51905f52906360f9bb11906104b8908490600401612b7f565b5f60405180830381865afa1580156104d2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104f99190810190612e28565b6020906105069082612f1b565b5050565b5f6105be6020805461051b90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461054790612e9d565b80156105925780601f1061056957610100808354040283529160200191610592565b820191905f5260205f20905b81548152906001019060200180831161057557829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b81525061272a565b905090565b5f6105be602080546105d490612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461060090612e9d565b801561064b5780601f106106225761010080835404028352916020019161064b565b820191905f5260205f20905b81548152906001019060200180831161062e57829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b81525061272a565b5f6105be6020805461068f90612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546106bb90612e9d565b80156107065780601f106106dd57610100808354040283529160200191610706565b820191905f5260205f20905b8154815290600101906020018083116106e957829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b81525061272a565b6107476127d8565b5f5b60058110156109415760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610798573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526107bf9190810190612e28565b6040516020016107cf9190612fed565b60408051601f19818403018152908290526107ec91602001613009565b604051602081830303815290604052602190816108099190612f1b565b506109246020805461081a90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461084690612e9d565b80156108915780601f1061086857610100808354040283529160200191610891565b820191905f5260205f20905b81548152906001019060200180831161087457829003601f168201915b5050505050602180546108a390612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546108cf90612e9d565b801561091a5780601f106108f15761010080835404028352916020019161091a565b820191905f5260205f20905b8154815290600101906020018083116108fd57829003601f168201915b505050505061272a565b60ab82600581106109375761093761302d565b0155600101610749565b506040805160a08101918290529060ab9060059082845b815481526020019060010190808311610958575b5050505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be602080546109e690612e9d565b80601f0160208091040260200160405190810160405280929190818152602001828054610a1290612e9d565b8015610a5d5780601f10610a3457610100808354040283529160200191610a5d565b820191905f5260205f20905b815481529060010190602001808311610a4057829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b81525061272a565b610a986127f6565b5f5b602c811015610b885760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610ae9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b109190810190612e28565b604051602001610b209190612fed565b60408051601f1981840301815290829052610b3d91602001613041565b60405160208183030381529060405260219081610b5a9190612f1b565b50610b6b6020805461081a90612e9d565b607882602c8110610b7e57610b7e61302d565b0155600101610a9a565b50604080516105808101918290526078805482529091602c90607960208501808311610958575050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610cd4578382905f5260205f20018054610c4990612e9d565b80601f0160208091040260200160405190810160405280929190818152602001828054610c7590612e9d565b8015610cc05780601f10610c9757610100808354040283529160200191610cc0565b820191905f5260205f20905b815481529060010190602001808311610ca357829003601f168201915b505050505081526020019060010190610c2c565b505050508152505081526020019060010190610bdb565b50505050905090565b60408051602e8082526105e082019092526060915f9190602082016105c0803683370190505090505f5b602e811015610e145760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610d6d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610d949190810190612e28565b604051602001610da49190612fed565b60408051601f1981840301815290829052610dc191602001613072565b60405160208183030381529060405260219081610dde9190612f1b565b50610def6020805461081a90612e9d565b828281518110610e0157610e0161302d565b6020908102919091010152600101610d1e565b5080604051602001610e2691906130a3565b60405160208183030381529060405291505090565b604080516003808252608082019092526060915f91906020820184803683370190505090505f5b6003811015610e145760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610eb1573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610ed89190810190612e28565b604051602001610ee89190612fed565b60408051601f1981840301815290829052610f05916020016130d8565b60405160208183030381529060405260219081610f229190612f1b565b50610f336020805461081a90612e9d565b828281518110610f4557610f4561302d565b6020908102919091010152600101610e62565b6060601880548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be6020805461102590612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461105190612e9d565b801561109c5780601f106110735761010080835404028352916020019161109c565b820191905f5260205f20905b81548152906001019060200180831161107f57829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b8152506127a4565b60408051600880825261012082019092526060915f919060208201610100803683370190505090505f5b60088110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561114d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526111749190810190612e28565b6040516020016111849190612fed565b60408051601f19818403018152908290526111a19160200161311a565b604051602081830303815290604052602190816111be9190612f1b565b506111cf6020805461081a90612e9d565b8282815181106111e1576111e161302d565b60209081029190910101526001016110fe565b50919050565b611202612815565b5f5b602e8110156112f25760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611253573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261127a9190810190612e28565b60405160200161128a9190612fed565b60408051601f19818403018152908290526112a79160200161313f565b604051602081830303815290604052602190816112c49190612f1b565b506112d56020805461081a90612e9d565b604a82602e81106112e8576112e861302d565b0155600101611204565b50604080516105c0810191829052604a805482529091602e90604b60208501808311610958575050505050905090565b61132a612834565b5f5b600981101561141a5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561137b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113a29190810190612e28565b6040516020016113b29190612fed565b60408051601f19818403018152908290526113cf91602001613163565b604051602081830303815290604052602190816113ec9190612f1b565b506113fd6020805461081a90612e9d565b604182600981106114105761141061302d565b015560010161132c565b50604080516101208101918290526041805482529091600990604260208501808311610958575050505050905090565b5f6105be6020805461145b90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461148790612e9d565b80156114d25780601f106114a9576101008083540402835291602001916114d2565b820191905f5260205f20905b8154815290600101906020018083116114b557829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b8152506127a4565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2090600202016040518060400160405290815f8201805461155890612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461158490612e9d565b80156115cf5780601f106115a6576101008083540402835291602001916115cf565b820191905f5260205f20905b8154815290600101906020018083116115b257829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561165157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116116135790505b50505050508152505081526020019060010190611528565b5f6105be6020805461167a90612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546116a690612e9d565b80156116f15780601f106116c8576101008083540402835291602001916116f1565b820191905f5260205f20905b8154815290600101906020018083116116d457829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b8152506127a4565b5f6105be6020805461172a90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461175690612e9d565b80156117a15780601f10611778576101008083540402835291602001916117a1565b820191905f5260205f20905b81548152906001019060200180831161178457829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b8152506127a4565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2001805461181390612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461183f90612e9d565b801561188a5780601f106118615761010080835404028352916020019161188a565b820191905f5260205f20905b81548152906001019060200180831161186d57829003601f168201915b5050505050815260200190600101906117f6565b60408051602e8082526105e082019092526060915f9190602082016105c0803683370190505090505f5b602e8110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611917573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261193e9190810190612e28565b60405160200161194e9190612fed565b60408051601f198184030181529082905261196b91602001613188565b604051602081830303815290604052602190816119889190612f1b565b506119996020805461081a90612e9d565b8282815181106119ab576119ab61302d565b60209081029190910101526001016118c8565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611a8757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611a495790505b505050505081525050815260200190600101906119e1565b60408051600480825260a082019092526060915f9190602082016080803683370190505090505f5b60048110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611b16573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b3d9190810190612e28565b604051602001611b4d9190612fed565b60408051601f1981840301815290829052611b6a916020016131b2565b60405160208183030381529060405260219081611b879190612f1b565b50611b986020805461081a90612e9d565b828281518110611baa57611baa61302d565b6020908102919091010152600101611ac7565b611bc5612853565b5f5b600a811015611cb55760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611c16573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611c3d9190810190612e28565b604051602001611c4d9190612fed565b60408051601f1981840301815290829052611c6a91602001613163565b60405160208183030381529060405260219081611c879190612f1b565b50611c986020805461081a90612e9d565b603782600a8110611cab57611cab61302d565b0155600101611bc7565b50604080516101408101918290526037805482529091600a90603860208501808311610958575050505050905090565b611ced612872565b5f5b6003811015611ddd5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611d3e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611d659190810190612e28565b604051602001611d759190612fed565b60408051601f1981840301815290829052611d92916020016131d8565b60405160208183030381529060405260219081611daf9190612f1b565b50611dc06020805461081a90612e9d565b60348260038110611dd357611dd361302d565b0155600101611cef565b506040805160608101918290526034805482529091600390603560208501808311610958575050505050905090565b60606105be610cf4565b611e1e612890565b5f5b6012811015611f0e5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611e6f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611e969190810190612e28565b604051602001611ea69190612fed565b60408051601f1981840301815290829052611ec3916020016131f7565b60405160208183030381529060405260219081611ee09190612f1b565b50611ef16020805461081a90612e9d565b60228260128110611f0457611f0461302d565b0155600101611e20565b50604080516102408101918290526022805482529091601290602360208501808311610958575050505050905090565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561200757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611fc95790505b50505050508152505081526020019060010190611f61565b5f6105be6020805461203090612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461205c90612e9d565b80156120a75780601f1061207e576101008083540402835291602001916120a7565b820191905f5260205f20905b81548152906001019060200180831161208a57829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b81525061272a565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2001805461211890612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461214490612e9d565b801561218f5780601f106121665761010080835404028352916020019161218f565b820191905f5260205f20905b81548152906001019060200180831161217257829003601f168201915b5050505050815260200190600101906120fb565b6008545f9060ff16156121ba575060085460ff1690565b604051630667f9d760e41b81525f5160206132935f395f51905f52600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa158015612211573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612235919061321d565b1415905090565b5f6105be6020805461224d90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461227990612e9d565b80156122c45780601f1061229b576101008083540402835291602001916122c4565b820191905f5260205f20905b8154815290600101906020018083116122a757829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e6465780000000000000000008152506127a4565b5f6105be6020805461231590612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461234190612e9d565b801561238c5780601f106123635761010080835404028352916020019161238c565b820191905f5260205f20905b81548152906001019060200180831161236f57829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b81525061272a565b6123cc6128af565b5f5b60078110156124bc5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561241d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124449190810190612e28565b6040516020016124549190612fed565b60408051601f198184030181529082905261247191602001613234565b6040516020818303038152906040526021908161248e9190612f1b565b5061249f6020805461081a90612e9d565b60a482600781106124b2576124b261302d565b01556001016123ce565b506040805160e081019182905260a480548252909160079060a560208501808311610958575050505050905090565b6124f36128cd565b5f5b60048110156125e35760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015612544573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261256b9190810190612e28565b60405160200161257b9190612fed565b60408051601f198184030181529082905261259891602001613009565b604051602081830303815290604052602190816125b59190612f1b565b506125c66020805461081a90612e9d565b60b082600481106125d9576125d961302d565b01556001016124f5565b5060408051608081019182905260b080548252909160049060b160208501808311610958575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be6020805461268190612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546126ad90612e9d565b80156126f85780601f106126cf576101008083540402835291602001916126f8565b820191905f5260205f20905b8154815290600101906020018083116126db57829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b81525f905f5160206132935f395f51905f5290631777e59d9061275e9086908690600401613265565b602060405180830381865afa158015612779573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061279d919061321d565b9392505050565b6040516356eef15b60e11b81525f905f5160206132935f395f51905f529063addde2b69061275e9086908690600401613265565b6040518060a001604052806005906020820280368337509192915050565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b604051806101400160405280600a906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612928576129286128eb565b604052919050565b5f67ffffffffffffffff821115612949576129496128eb565b50601f01601f191660200190565b5f60208284031215612967575f5ffd5b813567ffffffffffffffff81111561297d575f5ffd5b8201601f8101841361298d575f5ffd5b80356129a061299b82612930565b6128ff565b8181528560208385010111156129b4575f5ffd5b816020840160208301375f91810160200191909152949350505050565b60a0810181835f5b60058110156129f85781518352602092830192909101906001016129d9565b50505092915050565b602080825282518282018190525f918401906040840190835b81811015612a415783516001600160a01b0316835260209384019390920191600101612a1a565b509095945050505050565b610580810181835f5b602c8110156129f8578151835260209283019290910190600101612a55565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b83811015612af057601f19858403018852612ada838351612a74565b6020988901989093509190910190600101612abe565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357868503603f19018452815180516001600160a01b03168652602090810151604091870182905290612b5d90870182612aa2565b9550506020938401939190910190600101612b22565b50929695505050505050565b602081525f61279d6020830184612a74565b602080825282518282018190525f918401906040840190835b81811015612a41578351835260209384019390920191600101612baa565b6105c0810181835f5b602e8110156129f8578151835260209283019290910190600101612bd1565b610120810181835f5b60098110156129f8578151835260209283019290910190600101612bf9565b5f8151808452602084019350602083015f5b82811015612c525781516001600160e01b031916865260209586019590910190600101612c2a565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357603f198786030184528151805160408752612ca86040880182612a74565b9050602082015191508681036020880152612cc38183612c18565b965050506020938401939190910190600101612c82565b602081525f61279d6020830184612aa2565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357868503603f19018452815180516001600160a01b03168652602090810151604091870182905290612d4d90870182612c18565b9550506020938401939190910190600101612d12565b610140810181835f5b600a8110156129f8578151835260209283019290910190600101612d6c565b6060810181835f5b60038110156129f8578151835260209283019290910190600101612d93565b610240810181835f5b60128110156129f8578151835260209283019290910190600101612dbb565b60e0810181835f5b60078110156129f8578151835260209283019290910190600101612de2565b6080810181835f5b60048110156129f8578151835260209283019290910190600101612e09565b5f60208284031215612e38575f5ffd5b815167ffffffffffffffff811115612e4e575f5ffd5b8201601f81018413612e5e575f5ffd5b8051612e6c61299b82612930565b818152856020838501011115612e80575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b600181811c90821680612eb157607f821691505b6020821081036111f457634e487b7160e01b5f52602260045260245ffd5b601f821115612f1657805f5260205f20601f840160051c81016020851015612ef45750805b601f840160051c820191505b81811015612f13575f8155600101612f00565b50505b505050565b815167ffffffffffffffff811115612f3557612f356128eb565b612f4981612f438454612e9d565b84612ecf565b6020601f821160018114612f7b575f8315612f645750848201515b5f19600385901b1c1916600184901b178455612f13565b5f84815260208120601f198516915b82811015612faa5787850151825560209485019460019092019101612f8a565b5084821015612fc757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f81518060208401855e5f93019283525090919050565b5f612ff88284612fd6565b605d60f81b81526001019392505050565b6f2e54696d657374616d7050726f6f665b60801b81525f61279d6010830184612fd6565b634e487b7160e01b5f52603260045260245ffd5b7f2e486973746f726963616c53756d6d61727950726f6f665b000000000000000081525f61279d6018830184612fd6565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b000000000081525f61279d601b830184612fd6565b81515f90829060208501835b828110156130cd5781518452602093840193909101906001016130af565b509195945050505050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b60208201525f61279d6028830184612fd6565b702e56616c696461746f724669656c64735b60781b81525f61279d6011830184612fd6565b6f2e56616c696461746f7250726f6f665b60801b81525f61279d6010830184612fd6565b702e5769746864726177616c50726f6f665b60781b81525f61279d6011830184612fd6565b752e56616c696461746f724669656c647350726f6f665b60501b81525f61279d6016830184612fd6565b712e5769746864726177616c4669656c64735b60701b81525f61279d6012830184612fd6565b6a2e536c6f7450726f6f665b60a81b81525f61279d600b830184612fd6565b712e426c6f636b48656164657250726f6f665b60701b81525f61279d6012830184612fd6565b5f6020828403121561322d575f5ffd5b5051919050565b7f2e457865637574696f6e5061796c6f616450726f6f665b00000000000000000081525f61279d6017830184612fd6565b604081525f6132776040830185612a74565b82810360208401526132898185612a74565b9594505050505056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da2646970667358221220d1bc6a448038a2d6c3e53c1422edbf4b32aec68abaae855e05432d2aededd20664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x0C\x80T`\x01`\xFF\x19\x91\x82\x16\x81\x17\x90\x92U`\x1F\x80T\x90\x91\x16\x90\x91\x17\x90U4\x80\x15`+W__\xFD[Pa2\xE8\x80a\09_9_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02?W_5`\xE0\x1C\x80cvZ\xA6\x06\x11a\x015W\x80c\xB3\x80a\xBF\x11a\0\xB4W\x80c\xDB6J@\x11a\0yW\x80c\xDB6J@\x14a\x04AW\x80c\xDF\x18\xC7\x83\x14a\x04VW\x80c\xE2\x0C\x9Fq\x14a\x04kW\x80c\xF1H\x08,\x14a\x04sW\x80c\xFAv&\xD4\x14a\x04{W__\xFD[\x80c\xB3\x80a\xBF\x14a\x04\tW\x80c\xB5P\x8A\xA9\x14a\x04\x11W\x80c\xBAAO\xA6\x14a\x04\x19W\x80c\xD6F\x1C\xBB\x14a\x041W\x80c\xD9DG/\x14a\x049W__\xFD[\x80c\x9A\xCF\xAC)\x11a\0\xFAW\x80c\x9A\xCF\xAC)\x14a\x03\xBAW\x80c\x9D\xE4\xA9\xB3\x14a\x03\xCFW\x80c\xA1W\xEFL\x14a\x03\xE4W\x80c\xA5\x07t)\x14a\x03\xECW\x80c\xB0FO\xDC\x14a\x04\x01W__\xFD[\x80cvZ\xA6\x06\x14a\x03xW\x80c\x85\"l\x81\x14a\x03\x80W\x80c\x898\x93\xCA\x14a\x03\x95W\x80c\x91j\x17\xC6\x14a\x03\x9DW\x80c\x95\x0B\xB6\x82\x14a\x03\xB2W__\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xC1W\x80cL8\xF9\x13\x11a\x01\x86W\x80cL8\xF9\x13\x14a\x03)W\x80cd\xC7%5\x14a\x03>W\x80cd\xF3\x8E\xD7\x14a\x03SW\x80cf\xD9\xA9\xA0\x14a\x03[W\x80cl\x87|\x84\x14a\x03pW__\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xF4W\x80c>^<#\x14a\x02\xFCW\x80c?r\x86\xF4\x14a\x03\x04W\x80cB\x86G4\x14a\x03\x0CW\x80cL \xF5\x10\x14a\x03\x14W__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02\x07W\x80c\x1E\xD7\x83\x1C\x14a\x02\x98W\x80c'Sx\xB1\x14a\x02\xADW\x80c(r\xE2\x0C\x14a\x02\xB5W\x80c*\xDE8\x80\x14a\x02\xCAW\x80c/\xD1y<\x14a\x02\xDFW__\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x02CW\x80c\x16\xF0qS\x14a\x02XW\x80c\x17uA\xFC\x14a\x02sW\x80c\x18\xAA\xDF1\x14a\x02{W\x80c\x19l\xDF\xBD\x14a\x02\x83W[__\xFD[a\x02Va\x02Q6`\x04a)WV[a\x04\x88V[\0[a\x02`a\x05\nV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02`a\x05\xC3V[a\x02`a\x06~V[a\x02\x8Ba\x07?V[`@Qa\x02j\x91\x90a)\xD1V[a\x02\xA0a\tvV[`@Qa\x02j\x91\x90a*\x01V[a\x02`a\t\xD5V[a\x02\xBDa\n\x90V[`@Qa\x02j\x91\x90a*LV[a\x02\xD2a\x0B\xB8V[`@Qa\x02j\x91\x90a*\xFCV[a\x02\xE7a\x0C\xF4V[`@Qa\x02j\x91\x90a+\x7FV[a\x02\xE7a\x0E;V[a\x02\xA0a\x0FXV[a\x02\xA0a\x0F\xB6V[a\x02`a\x10\x14V[a\x03\x1Ca\x10\xD4V[`@Qa\x02j\x91\x90a+\x91V[a\x031a\x11\xFAV[`@Qa\x02j\x91\x90a+\xC8V[a\x03Fa\x13\"V[`@Qa\x02j\x91\x90a+\xF0V[a\x02`a\x14JV[a\x03ca\x15\x05V[`@Qa\x02j\x91\x90a,\\V[a\x02`a\x16iV[a\x02`a\x17\x19V[a\x03\x88a\x17\xD3V[`@Qa\x02j\x91\x90a,\xDAV[a\x03\x1Ca\x18\x9EV[a\x03\xA5a\x19\xBEV[`@Qa\x02j\x91\x90a,\xECV[a\x03\x1Ca\x1A\x9FV[a\x03\xC2a\x1B\xBDV[`@Qa\x02j\x91\x90a-cV[a\x03\xD7a\x1C\xE5V[`@Qa\x02j\x91\x90a-\x8BV[a\x02\xE7a\x1E\x0CV[a\x03\xF4a\x1E\x16V[`@Qa\x02j\x91\x90a-\xB2V[a\x03\xA5a\x1F>V[a\x02`a \x1FV[a\x03\x88a \xD8V[a\x04!a!\xA3V[`@Q\x90\x15\x15\x81R` \x01a\x02jV[a\x02`a\"<V[a\x02`a#\x04V[a\x04Ia#\xC4V[`@Qa\x02j\x91\x90a-\xDAV[a\x04^a$\xEBV[`@Qa\x02j\x91\x90a.\x01V[a\x02\xA0a&\x12V[a\x02`a&pV[`\x1FTa\x04!\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R_Q` a2\x93_9_Q\x90_R\x90c`\xF9\xBB\x11\x90a\x04\xB8\x90\x84\x90`\x04\x01a+\x7FV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xF9\x91\x90\x81\x01\x90a.(V[` \x90a\x05\x06\x90\x82a/\x1BV[PPV[_a\x05\xBE` \x80Ta\x05\x1B\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05G\x90a.\x9DV[\x80\x15a\x05\x92W\x80`\x1F\x10a\x05iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa'*V[\x90P\x90V[_a\x05\xBE` \x80Ta\x05\xD4\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\0\x90a.\x9DV[\x80\x15a\x06KW\x80`\x1F\x10a\x06\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06KV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa'*V[_a\x05\xBE` \x80Ta\x06\x8F\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xBB\x90a.\x9DV[\x80\x15a\x07\x06W\x80`\x1F\x10a\x06\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa'*V[a\x07Ga'\xD8V[_[`\x05\x81\x10\x15a\tAW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x98W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xBF\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x07\xCF\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xEC\x91` \x01a0\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x08\t\x91\x90a/\x1BV[Pa\t$` \x80Ta\x08\x1A\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08F\x90a.\x9DV[\x80\x15a\x08\x91W\x80`\x1F\x10a\x08hWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x91V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`!\x80Ta\x08\xA3\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xCF\x90a.\x9DV[\x80\x15a\t\x1AW\x80`\x1F\x10a\x08\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa'*V[`\xAB\x82`\x05\x81\x10a\t7Wa\t7a0-V[\x01U`\x01\x01a\x07IV[P`@\x80Q`\xA0\x81\x01\x91\x82\x90R\x90`\xAB\x90`\x05\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\tXW[PPPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\t\xE6\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x12\x90a.\x9DV[\x80\x15a\n]W\x80`\x1F\x10a\n4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n]V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa'*V[a\n\x98a'\xF6V[_[`,\x81\x10\x15a\x0B\x88W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x10\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x0B \x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B=\x91` \x01a0AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x0BZ\x91\x90a/\x1BV[Pa\x0Bk` \x80Ta\x08\x1A\x90a.\x9DV[`x\x82`,\x81\x10a\x0B~Wa\x0B~a0-V[\x01U`\x01\x01a\n\x9AV[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R`x\x80T\x82R\x90\x91`,\x90`y` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0C\xD4W\x83\x82\x90_R` _ \x01\x80Ta\x0CI\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Cu\x90a.\x9DV[\x80\x15a\x0C\xC0W\x80`\x1F\x10a\x0C\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xC0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C,V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xDBV[PPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P_[`.\x81\x10\x15a\x0E\x14W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x94\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\r\xA4\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xC1\x91` \x01a0rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\r\xDE\x91\x90a/\x1BV[Pa\r\xEF` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x0E\x01Wa\x0E\x01a0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\x1EV[P\x80`@Q` \x01a\x0E&\x91\x90a0\xA3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P_[`\x03\x81\x10\x15a\x0E\x14W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xD8\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x0E\xE8\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0F\x05\x91` \x01a0\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x0F\"\x91\x90a/\x1BV[Pa\x0F3` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x0FEWa\x0FEa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0EbV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\x10%\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10Q\x90a.\x9DV[\x80\x15a\x10\x9CW\x80`\x1F\x10a\x10sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x9CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa'\xA4V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11MW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11t\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x11\x84\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11\xA1\x91` \x01a1\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x11\xBE\x91\x90a/\x1BV[Pa\x11\xCF` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x11\xE1Wa\x11\xE1a0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x10\xFEV[P\x91\x90PV[a\x12\x02a(\x15V[_[`.\x81\x10\x15a\x12\xF2W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12SW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12z\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x12\x8A\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12\xA7\x91` \x01a1?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x12\xC4\x91\x90a/\x1BV[Pa\x12\xD5` \x80Ta\x08\x1A\x90a.\x9DV[`J\x82`.\x81\x10a\x12\xE8Wa\x12\xE8a0-V[\x01U`\x01\x01a\x12\x04V[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`J\x80T\x82R\x90\x91`.\x90`K` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a\x13*a(4V[_[`\t\x81\x10\x15a\x14\x1AW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13{W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xA2\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x13\xB2\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13\xCF\x91` \x01a1cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x13\xEC\x91\x90a/\x1BV[Pa\x13\xFD` \x80Ta\x08\x1A\x90a.\x9DV[`A\x82`\t\x81\x10a\x14\x10Wa\x14\x10a0-V[\x01U`\x01\x01a\x13,V[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`A\x80T\x82R\x90\x91`\t\x90`B` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\x14[\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x87\x90a.\x9DV[\x80\x15a\x14\xD2W\x80`\x1F\x10a\x14\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa'\xA4V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x15X\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x84\x90a.\x9DV[\x80\x15a\x15\xCFW\x80`\x1F\x10a\x15\xA6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xCFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16QW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15(V[_a\x05\xBE` \x80Ta\x16z\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xA6\x90a.\x9DV[\x80\x15a\x16\xF1W\x80`\x1F\x10a\x16\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa'\xA4V[_a\x05\xBE` \x80Ta\x17*\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17V\x90a.\x9DV[\x80\x15a\x17\xA1W\x80`\x1F\x10a\x17xWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xA1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa'\xA4V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x01\x80Ta\x18\x13\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18?\x90a.\x9DV[\x80\x15a\x18\x8AW\x80`\x1F\x10a\x18aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x17\xF6V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P_[`.\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x17W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19>\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x19N\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19k\x91` \x01a1\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x19\x88\x91\x90a/\x1BV[Pa\x19\x99` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x19\xABWa\x19\xABa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18\xC8V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1A\x87W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1AIW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19\xE1V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P_[`\x04\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x16W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B=\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1BM\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1Bj\x91` \x01a1\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1B\x87\x91\x90a/\x1BV[Pa\x1B\x98` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x1B\xAAWa\x1B\xAAa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xC7V[a\x1B\xC5a(SV[_[`\n\x81\x10\x15a\x1C\xB5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x16W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C=\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1CM\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1Cj\x91` \x01a1cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1C\x87\x91\x90a/\x1BV[Pa\x1C\x98` \x80Ta\x08\x1A\x90a.\x9DV[`7\x82`\n\x81\x10a\x1C\xABWa\x1C\xABa0-V[\x01U`\x01\x01a\x1B\xC7V[P`@\x80Qa\x01@\x81\x01\x91\x82\x90R`7\x80T\x82R\x90\x91`\n\x90`8` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a\x1C\xEDa(rV[_[`\x03\x81\x10\x15a\x1D\xDDW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D>W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1De\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1Du\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D\x92\x91` \x01a1\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1D\xAF\x91\x90a/\x1BV[Pa\x1D\xC0` \x80Ta\x08\x1A\x90a.\x9DV[`4\x82`\x03\x81\x10a\x1D\xD3Wa\x1D\xD3a0-V[\x01U`\x01\x01a\x1C\xEFV[P`@\x80Q``\x81\x01\x91\x82\x90R`4\x80T\x82R\x90\x91`\x03\x90`5` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[``a\x05\xBEa\x0C\xF4V[a\x1E\x1Ea(\x90V[_[`\x12\x81\x10\x15a\x1F\x0EW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\x96\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1E\xA6\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xC3\x91` \x01a1\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1E\xE0\x91\x90a/\x1BV[Pa\x1E\xF1` \x80Ta\x08\x1A\x90a.\x9DV[`\"\x82`\x12\x81\x10a\x1F\x04Wa\x1F\x04a0-V[\x01U`\x01\x01a\x1E V[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\"\x80T\x82R\x90\x91`\x12\x90`#` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a \x07W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1F\xC9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1FaV[_a\x05\xBE` \x80Ta 0\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \\\x90a.\x9DV[\x80\x15a \xA7W\x80`\x1F\x10a ~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xA7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa'*V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x01\x80Ta!\x18\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!D\x90a.\x9DV[\x80\x15a!\x8FW\x80`\x1F\x10a!fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a \xFBV[`\x08T_\x90`\xFF\x16\x15a!\xBAWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_Q` a2\x93_9_Q\x90_R`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"5\x91\x90a2\x1DV[\x14\x15\x90P\x90V[_a\x05\xBE` \x80Ta\"M\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"y\x90a.\x9DV[\x80\x15a\"\xC4W\x80`\x1F\x10a\"\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa'\xA4V[_a\x05\xBE` \x80Ta#\x15\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#A\x90a.\x9DV[\x80\x15a#\x8CW\x80`\x1F\x10a#cWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa'*V[a#\xCCa(\xAFV[_[`\x07\x81\x10\x15a$\xBCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1DW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$D\x91\x90\x81\x01\x90a.(V[`@Q` \x01a$T\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra$q\x91` \x01a24V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a$\x8E\x91\x90a/\x1BV[Pa$\x9F` \x80Ta\x08\x1A\x90a.\x9DV[`\xA4\x82`\x07\x81\x10a$\xB2Wa$\xB2a0-V[\x01U`\x01\x01a#\xCEV[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\xA4\x80T\x82R\x90\x91`\x07\x90`\xA5` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a$\xF3a(\xCDV[_[`\x04\x81\x10\x15a%\xE3W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%DW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%k\x91\x90\x81\x01\x90a.(V[`@Q` \x01a%{\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra%\x98\x91` \x01a0\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a%\xB5\x91\x90a/\x1BV[Pa%\xC6` \x80Ta\x08\x1A\x90a.\x9DV[`\xB0\x82`\x04\x81\x10a%\xD9Wa%\xD9a0-V[\x01U`\x01\x01a$\xF5V[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\xB0\x80T\x82R\x90\x91`\x04\x90`\xB1` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta&\x81\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xAD\x90a.\x9DV[\x80\x15a&\xF8W\x80`\x1F\x10a&\xCFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\xF8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R_\x90_Q` a2\x93_9_Q\x90_R\x90c\x17w\xE5\x9D\x90a'^\x90\x86\x90\x86\x90`\x04\x01a2eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x9D\x91\x90a2\x1DV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R_\x90_Q` a2\x93_9_Q\x90_R\x90c\xAD\xDD\xE2\xB6\x90a'^\x90\x86\x90\x86\x90`\x04\x01a2eV[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01@\x01`@R\x80`\n\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)(Wa)(a(\xEBV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)IWa)Ia(\xEBV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a)gW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)}W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x8DW__\xFD[\x805a)\xA0a)\x9B\x82a)0V[a(\xFFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xB4W__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xA0\x81\x01\x81\x83_[`\x05\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\xD9V[PPP\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a*AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a*\x1AV[P\x90\x95\x94PPPPPV[a\x05\x80\x81\x01\x81\x83_[`,\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*UV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a*\xF0W`\x1F\x19\x85\x84\x03\x01\x88Ra*\xDA\x83\x83Qa*tV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a*\xBEV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a+]\x90\x87\x01\x82a*\xA2V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a+\"V[P\x92\x96\x95PPPPPPV[` \x81R_a'\x9D` \x83\x01\x84a*tV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a*AW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+\xAAV[a\x05\xC0\x81\x01\x81\x83_[`.\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\xD1V[a\x01 \x81\x01\x81\x83_[`\t\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\xF9V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a,RW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a,*V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra,\xA8`@\x88\x01\x82a*tV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra,\xC3\x81\x83a,\x18V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a,\x82V[` \x81R_a'\x9D` \x83\x01\x84a*\xA2V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a-M\x90\x87\x01\x82a,\x18V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a-\x12V[a\x01@\x81\x01\x81\x83_[`\n\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-lV[``\x81\x01\x81\x83_[`\x03\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\x93V[a\x02@\x81\x01\x81\x83_[`\x12\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\xBBV[`\xE0\x81\x01\x81\x83_[`\x07\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\xE2V[`\x80\x81\x01\x81\x83_[`\x04\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\tV[_` \x82\x84\x03\x12\x15a.8W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.NW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a.^W__\xFD[\x80Qa.la)\x9B\x82a)0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a.\x80W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a.\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x11\xF4WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[`\x1F\x82\x11\x15a/\x16W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\xF4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a/\x13W_\x81U`\x01\x01a/\0V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/5Wa/5a(\xEBV[a/I\x81a/C\x84Ta.\x9DV[\x84a.\xCFV[` `\x1F\x82\x11`\x01\x81\x14a/{W_\x83\x15a/dWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua/\x13V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a/\xAAW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a/\x8AV[P\x84\x82\x10\x15a/\xC7W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a/\xF8\x82\x84a/\xD6V[`]`\xF8\x1B\x81R`\x01\x01\x93\x92PPPV[o.TimestampProof[`\x80\x1B\x81R_a'\x9D`\x10\x83\x01\x84a/\xD6V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R_a'\x9D`\x18\x83\x01\x84a/\xD6V[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R_a'\x9D`\x1B\x83\x01\x84a/\xD6V[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a0\xCDW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0\xAFV[P\x91\x95\x94PPPPPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R_a'\x9D`(\x83\x01\x84a/\xD6V[p.ValidatorFields[`x\x1B\x81R_a'\x9D`\x11\x83\x01\x84a/\xD6V[o.ValidatorProof[`\x80\x1B\x81R_a'\x9D`\x10\x83\x01\x84a/\xD6V[p.WithdrawalProof[`x\x1B\x81R_a'\x9D`\x11\x83\x01\x84a/\xD6V[u.ValidatorFieldsProof[`P\x1B\x81R_a'\x9D`\x16\x83\x01\x84a/\xD6V[q.WithdrawalFields[`p\x1B\x81R_a'\x9D`\x12\x83\x01\x84a/\xD6V[j.SlotProof[`\xA8\x1B\x81R_a'\x9D`\x0B\x83\x01\x84a/\xD6V[q.BlockHeaderProof[`p\x1B\x81R_a'\x9D`\x12\x83\x01\x84a/\xD6V[_` \x82\x84\x03\x12\x15a2-W__\xFD[PQ\x91\x90PV[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R_a'\x9D`\x17\x83\x01\x84a/\xD6V[`@\x81R_a2w`@\x83\x01\x85a*tV[\x82\x81\x03` \x84\x01Ra2\x89\x81\x85a*tV[\x95\x94PPPPPV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \xD1\xBCjD\x808\xA2\xD6\xC3\xE5<\x14\"\xED\xBFK2\xAE\xC6\x8A\xBA\xAE\x85^\x05C-*\xED\xED\xD2\x06dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061023f575f3560e01c8063765aa60611610135578063b38061bf116100b4578063db364a4011610079578063db364a4014610441578063df18c78314610456578063e20c9f711461046b578063f148082c14610473578063fa7626d41461047b575f5ffd5b8063b38061bf14610409578063b5508aa914610411578063ba414fa614610419578063d6461cbb14610431578063d944472f14610439575f5ffd5b80639acfac29116100fa5780639acfac29146103ba5780639de4a9b3146103cf578063a157ef4c146103e4578063a5077429146103ec578063b0464fdc14610401575f5ffd5b8063765aa6061461037857806385226c8114610380578063893893ca14610395578063916a17c61461039d578063950bb682146103b2575f5ffd5b806334e3d90e116101c15780634c38f913116101865780634c38f9131461032957806364c725351461033e57806364f38ed71461035357806366d9a9a01461035b5780636c877c8414610370575f5ffd5b806334e3d90e146102f45780633e5e3c23146102fc5780633f7286f414610304578063428647341461030c5780634c20f51014610314575f5ffd5b80631ed7831c116102075780631ed7831c14610298578063275378b1146102ad5780632872e20c146102b55780632ade3880146102ca5780632fd1793c146102df575f5ffd5b806308a4d71f1461024357806316f0715314610258578063177541fc1461027357806318aadf311461027b578063196cdfbd14610283575b5f5ffd5b610256610251366004612957565b610488565b005b61026061050a565b6040519081526020015b60405180910390f35b6102606105c3565b61026061067e565b61028b61073f565b60405161026a91906129d1565b6102a0610976565b60405161026a9190612a01565b6102606109d5565b6102bd610a90565b60405161026a9190612a4c565b6102d2610bb8565b60405161026a9190612afc565b6102e7610cf4565b60405161026a9190612b7f565b6102e7610e3b565b6102a0610f58565b6102a0610fb6565b610260611014565b61031c6110d4565b60405161026a9190612b91565b6103316111fa565b60405161026a9190612bc8565b610346611322565b60405161026a9190612bf0565b61026061144a565b610363611505565b60405161026a9190612c5c565b610260611669565b610260611719565b6103886117d3565b60405161026a9190612cda565b61031c61189e565b6103a56119be565b60405161026a9190612cec565b61031c611a9f565b6103c2611bbd565b60405161026a9190612d63565b6103d7611ce5565b60405161026a9190612d8b565b6102e7611e0c565b6103f4611e16565b60405161026a9190612db2565b6103a5611f3e565b61026061201f565b6103886120d8565b6104216121a3565b604051901515815260200161026a565b61026061223c565b610260612304565b6104496123c4565b60405161026a9190612dda565b61045e6124eb565b60405161026a9190612e01565b6102a0612612565b610260612670565b601f546104219060ff1681565b6040516360f9bb1160e01b81525f5160206132935f395f51905f52906360f9bb11906104b8908490600401612b7f565b5f60405180830381865afa1580156104d2573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526104f99190810190612e28565b6020906105069082612f1b565b5050565b5f6105be6020805461051b90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461054790612e9d565b80156105925780601f1061056957610100808354040283529160200191610592565b820191905f5260205f20905b81548152906001019060200180831161057557829003601f168201915b5050505050604051806040016040528060098152602001680b9cdb1bdd149bdbdd60ba1b81525061272a565b905090565b5f6105be602080546105d490612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461060090612e9d565b801561064b5780601f106106225761010080835404028352916020019161064b565b820191905f5260205f20905b81548152906001019060200180831161062e57829003601f168201915b50505050506040518060400160405280601081526020016f0b98995858dbdb94dd185d19549bdbdd60821b81525061272a565b5f6105be6020805461068f90612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546106bb90612e9d565b80156107065780601f106106dd57610100808354040283529160200191610706565b820191905f5260205f20905b8154815290600101906020018083116106e957829003601f168201915b5050505050604051806040016040528060168152602001750b9b185d195cdd109b1bd8dad2195859195c949bdbdd60521b81525061272a565b6107476127d8565b5f5b60058110156109415760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610798573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526107bf9190810190612e28565b6040516020016107cf9190612fed565b60408051601f19818403018152908290526107ec91602001613009565b604051602081830303815290604052602190816108099190612f1b565b506109246020805461081a90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461084690612e9d565b80156108915780601f1061086857610100808354040283529160200191610891565b820191905f5260205f20905b81548152906001019060200180831161087457829003601f168201915b5050505050602180546108a390612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546108cf90612e9d565b801561091a5780601f106108f15761010080835404028352916020019161091a565b820191905f5260205f20905b8154815290600101906020018083116108fd57829003601f168201915b505050505061272a565b60ab82600581106109375761093761302d565b0155600101610749565b506040805160a08101918290529060ab9060059082845b815481526020019060010190808311610958575b5050505050905090565b6060601680548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f20905b81546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be602080546109e690612e9d565b80601f0160208091040260200160405190810160405280929190818152602001828054610a1290612e9d565b8015610a5d5780601f10610a3457610100808354040283529160200191610a5d565b820191905f5260205f20905b815481529060010190602001808311610a4057829003601f168201915b50505050506040518060400160405280601081526020016f0b989b1bd8dad2195859195c949bdbdd60821b81525061272a565b610a986127f6565b5f5b602c811015610b885760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610ae9573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610b109190810190612e28565b604051602001610b209190612fed565b60408051601f1981840301815290829052610b3d91602001613041565b60405160208183030381529060405260219081610b5a9190612f1b565b50610b6b6020805461081a90612e9d565b607882602c8110610b7e57610b7e61302d565b0155600101610a9a565b50604080516105808101918290526078805482529091602c90607960208501808311610958575050505050905090565b6060601e805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f84815260208082206040805180820182526002870290920180546001600160a01b03168352600181018054835181870281018701909452808452939591948681019491929084015b82821015610cd4578382905f5260205f20018054610c4990612e9d565b80601f0160208091040260200160405190810160405280929190818152602001828054610c7590612e9d565b8015610cc05780601f10610c9757610100808354040283529160200191610cc0565b820191905f5260205f20905b815481529060010190602001808311610ca357829003601f168201915b505050505081526020019060010190610c2c565b505050508152505081526020019060010190610bdb565b50505050905090565b60408051602e8082526105e082019092526060915f9190602082016105c0803683370190505090505f5b602e811015610e145760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610d6d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610d949190810190612e28565b604051602001610da49190612fed565b60408051601f1981840301815290829052610dc191602001613072565b60405160208183030381529060405260219081610dde9190612f1b565b50610def6020805461081a90612e9d565b828281518110610e0157610e0161302d565b6020908102919091010152600101610d1e565b5080604051602001610e2691906130a3565b60405160208183030381529060405291505090565b604080516003808252608082019092526060915f91906020820184803683370190505090505f5b6003811015610e145760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015610eb1573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052610ed89190810190612e28565b604051602001610ee89190612fed565b60408051601f1981840301815290829052610f05916020016130d8565b60405160208183030381529060405260219081610f229190612f1b565b50610f336020805461081a90612e9d565b828281518110610f4557610f4561302d565b6020908102919091010152600101610e62565b6060601880548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b6060601780548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be6020805461102590612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461105190612e9d565b801561109c5780601f106110735761010080835404028352916020019161109c565b820191905f5260205f20905b81548152906001019060200180831161107f57829003601f168201915b50505050506040518060400160405280601581526020017405cc4d8dec6d690cac2c8cae4a4dedee892dcc8caf605b1b8152506127a4565b60408051600880825261012082019092526060915f919060208201610100803683370190505090505f5b60088110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561114d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526111749190810190612e28565b6040516020016111849190612fed565b60408051601f19818403018152908290526111a19160200161311a565b604051602081830303815290604052602190816111be9190612f1b565b506111cf6020805461081a90612e9d565b8282815181106111e1576111e161302d565b60209081029190910101526001016110fe565b50919050565b611202612815565b5f5b602e8110156112f25760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611253573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261127a9190810190612e28565b60405160200161128a9190612fed565b60408051601f19818403018152908290526112a79160200161313f565b604051602081830303815290604052602190816112c49190612f1b565b506112d56020805461081a90612e9d565b604a82602e81106112e8576112e861302d565b0155600101611204565b50604080516105c0810191829052604a805482529091602e90604b60208501808311610958575050505050905090565b61132a612834565b5f5b600981101561141a5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561137b573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526113a29190810190612e28565b6040516020016113b29190612fed565b60408051601f19818403018152908290526113cf91602001613163565b604051602081830303815290604052602190816113ec9190612f1b565b506113fd6020805461081a90612e9d565b604182600981106114105761141061302d565b015560010161132c565b50604080516101208101918290526041805482529091600990604260208501808311610958575050505050905090565b5f6105be6020805461145b90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461148790612e9d565b80156114d25780601f106114a9576101008083540402835291602001916114d2565b820191905f5260205f20905b8154815290600101906020018083116114b557829003601f168201915b50505050506040518060400160405280601081526020016f05ceed2e8d0c8e4c2eec2d892dcc8caf60831b8152506127a4565b6060601b805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2090600202016040518060400160405290815f8201805461155890612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461158490612e9d565b80156115cf5780601f106115a6576101008083540402835291602001916115cf565b820191905f5260205f20905b8154815290600101906020018083116115b257829003601f168201915b505050505081526020016001820180548060200260200160405190810160405280929190818152602001828054801561165157602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b031916815260200190600401906020826003010492830192600103820291508084116116135790505b50505050508152505081526020019060010190611528565b5f6105be6020805461167a90612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546116a690612e9d565b80156116f15780601f106116c8576101008083540402835291602001916116f1565b820191905f5260205f20905b8154815290600101906020018083116116d457829003601f168201915b5050505050604051806040016040528060058152602001640b9cdb1bdd60da1b8152506127a4565b5f6105be6020805461172a90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461175690612e9d565b80156117a15780601f10611778576101008083540402835291602001916117a1565b820191905f5260205f20905b81548152906001019060200180831161178457829003601f168201915b50505050506040518060400160405280600f81526020016e05cecc2d8d2c8c2e8dee492dcc8caf608b1b8152506127a4565b6060601a805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2001805461181390612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461183f90612e9d565b801561188a5780601f106118615761010080835404028352916020019161188a565b820191905f5260205f20905b81548152906001019060200180831161186d57829003601f168201915b5050505050815260200190600101906117f6565b60408051602e8082526105e082019092526060915f9190602082016105c0803683370190505090505f5b602e8110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611917573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261193e9190810190612e28565b60405160200161194e9190612fed565b60408051601f198184030181529082905261196b91602001613188565b604051602081830303815290604052602190816119889190612f1b565b506119996020805461081a90612e9d565b8282815181106119ab576119ab61302d565b60209081029190910101526001016118c8565b6060601d805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f8481526020908190206040805180820182526002860290920180546001600160a01b03168352600181018054835181870281018701909452808452939491938583019392830182828015611a8757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611a495790505b505050505081525050815260200190600101906119e1565b60408051600480825260a082019092526060915f9190602082016080803683370190505090505f5b60048110156111f45760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611b16573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611b3d9190810190612e28565b604051602001611b4d9190612fed565b60408051601f1981840301815290829052611b6a916020016131b2565b60405160208183030381529060405260219081611b879190612f1b565b50611b986020805461081a90612e9d565b828281518110611baa57611baa61302d565b6020908102919091010152600101611ac7565b611bc5612853565b5f5b600a811015611cb55760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611c16573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611c3d9190810190612e28565b604051602001611c4d9190612fed565b60408051601f1981840301815290829052611c6a91602001613163565b60405160208183030381529060405260219081611c879190612f1b565b50611c986020805461081a90612e9d565b603782600a8110611cab57611cab61302d565b0155600101611bc7565b50604080516101408101918290526037805482529091600a90603860208501808311610958575050505050905090565b611ced612872565b5f5b6003811015611ddd5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611d3e573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611d659190810190612e28565b604051602001611d759190612fed565b60408051601f1981840301815290829052611d92916020016131d8565b60405160208183030381529060405260219081611daf9190612f1b565b50611dc06020805461081a90612e9d565b60348260038110611dd357611dd361302d565b0155600101611cef565b506040805160608101918290526034805482529091600390603560208501808311610958575050505050905090565b60606105be610cf4565b611e1e612890565b5f5b6012811015611f0e5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015611e6f573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611e969190810190612e28565b604051602001611ea69190612fed565b60408051601f1981840301815290829052611ec3916020016131f7565b60405160208183030381529060405260219081611ee09190612f1b565b50611ef16020805461081a90612e9d565b60228260128110611f0457611f0461302d565b0155600101611e20565b50604080516102408101918290526022805482529091601290602360208501808311610958575050505050905090565b6060601c805480602002602001604051908101604052809291908181526020015f905b82821015610ceb575f8481526020908190206040805180820182526002860290920180546001600160a01b0316835260018101805483518187028101870190945280845293949193858301939283018282801561200757602002820191905f5260205f20905f905b82829054906101000a900460e01b6001600160e01b03191681526020019060040190602082600301049283019260010382029150808411611fc95790505b50505050508152505081526020019060010190611f61565b5f6105be6020805461203090612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461205c90612e9d565b80156120a75780601f1061207e576101008083540402835291602001916120a7565b820191905f5260205f20905b81548152906001019060200180831161208a57829003601f168201915b50505050506040518060400160405280600e81526020016d0b9d1a5b595cdd185b5c149bdbdd60921b81525061272a565b60606019805480602002602001604051908101604052809291908181526020015f905b82821015610ceb578382905f5260205f2001805461211890612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461214490612e9d565b801561218f5780601f106121665761010080835404028352916020019161218f565b820191905f5260205f20905b81548152906001019060200180831161217257829003601f168201915b5050505050815260200190600101906120fb565b6008545f9060ff16156121ba575060085460ff1690565b604051630667f9d760e41b81525f5160206132935f395f51905f52600482018190526519985a5b195960d21b60248301525f9163667f9d7090604401602060405180830381865afa158015612211573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612235919061321d565b1415905090565b5f6105be6020805461224d90612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461227990612e9d565b80156122c45780601f1061229b576101008083540402835291602001916122c4565b820191905f5260205f20905b8154815290600101906020018083116122a757829003601f168201915b50505050506040518060400160405280601781526020017f2e686973746f726963616c53756d6d617279496e6465780000000000000000008152506127a4565b5f6105be6020805461231590612e9d565b80601f016020809104026020016040519081016040528092919081815260200182805461234190612e9d565b801561238c5780601f106123635761010080835404028352916020019161238c565b820191905f5260205f20905b81548152906001019060200180831161236f57829003601f168201915b5050505050604051806040016040528060158152602001740b995e1958dd5d1a5bdb94185e5b1bd859149bdbdd605a1b81525061272a565b6123cc6128af565b5f5b60078110156124bc5760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa15801561241d573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f191682016040526124449190810190612e28565b6040516020016124549190612fed565b60408051601f198184030181529082905261247191602001613234565b6040516020818303038152906040526021908161248e9190612f1b565b5061249f6020805461081a90612e9d565b60a482600781106124b2576124b261302d565b01556001016123ce565b506040805160e081019182905260a480548252909160079060a560208501808311610958575050505050905090565b6124f36128cd565b5f5b60048110156125e35760405163348051d760e11b8152600481018290525f5160206132935f395f51905f5290636900a3ae906024015f60405180830381865afa158015612544573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f1916820160405261256b9190810190612e28565b60405160200161257b9190612fed565b60408051601f198184030181529082905261259891602001613009565b604051602081830303815290604052602190816125b59190612f1b565b506125c66020805461081a90612e9d565b60b082600481106125d9576125d961302d565b01556001016124f5565b5060408051608081019182905260b080548252909160049060b160208501808311610958575050505050905090565b6060601580548060200260200160405190810160405280929190818152602001828054801561096c57602002820191905f5260205f209081546001600160a01b031681526001909101906020018083116109ae575050505050905090565b5f6105be6020805461268190612e9d565b80601f01602080910402602001604051908101604052809291908181526020018280546126ad90612e9d565b80156126f85780601f106126cf576101008083540402835291602001916126f8565b820191905f5260205f20905b8154815290600101906020018083116126db57829003601f168201915b5050505050604051806040016040528060138152602001722e56616c696461746f724669656c64735b305d60681b8152505b604051631777e59d60e01b81525f905f5160206132935f395f51905f5290631777e59d9061275e9086908690600401613265565b602060405180830381865afa158015612779573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061279d919061321d565b9392505050565b6040516356eef15b60e11b81525f905f5160206132935f395f51905f529063addde2b69061275e9086908690600401613265565b6040518060a001604052806005906020820280368337509192915050565b604051806105800160405280602c906020820280368337509192915050565b604051806105c00160405280602e906020820280368337509192915050565b6040518061012001604052806009906020820280368337509192915050565b604051806101400160405280600a906020820280368337509192915050565b60405180606001604052806003906020820280368337509192915050565b6040518061024001604052806012906020820280368337509192915050565b6040518060e001604052806007906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b634e487b7160e01b5f52604160045260245ffd5b604051601f8201601f1916810167ffffffffffffffff81118282101715612928576129286128eb565b604052919050565b5f67ffffffffffffffff821115612949576129496128eb565b50601f01601f191660200190565b5f60208284031215612967575f5ffd5b813567ffffffffffffffff81111561297d575f5ffd5b8201601f8101841361298d575f5ffd5b80356129a061299b82612930565b6128ff565b8181528560208385010111156129b4575f5ffd5b816020840160208301375f91810160200191909152949350505050565b60a0810181835f5b60058110156129f85781518352602092830192909101906001016129d9565b50505092915050565b602080825282518282018190525f918401906040840190835b81811015612a415783516001600160a01b0316835260209384019390920191600101612a1a565b509095945050505050565b610580810181835f5b602c8110156129f8578151835260209283019290910190600101612a55565b5f81518084528060208401602086015e5f602082860101526020601f19601f83011685010191505092915050565b5f82825180855260208501945060208160051b830101602085015f5b83811015612af057601f19858403018852612ada838351612a74565b6020988901989093509190910190600101612abe565b50909695505050505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357868503603f19018452815180516001600160a01b03168652602090810151604091870182905290612b5d90870182612aa2565b9550506020938401939190910190600101612b22565b50929695505050505050565b602081525f61279d6020830184612a74565b602080825282518282018190525f918401906040840190835b81811015612a41578351835260209384019390920191600101612baa565b6105c0810181835f5b602e8110156129f8578151835260209283019290910190600101612bd1565b610120810181835f5b60098110156129f8578151835260209283019290910190600101612bf9565b5f8151808452602084019350602083015f5b82811015612c525781516001600160e01b031916865260209586019590910190600101612c2a565b5093949350505050565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357603f198786030184528151805160408752612ca86040880182612a74565b9050602082015191508681036020880152612cc38183612c18565b965050506020938401939190910190600101612c82565b602081525f61279d6020830184612aa2565b5f602082016020835280845180835260408501915060408160051b8601019250602086015f5b82811015612b7357868503603f19018452815180516001600160a01b03168652602090810151604091870182905290612d4d90870182612c18565b9550506020938401939190910190600101612d12565b610140810181835f5b600a8110156129f8578151835260209283019290910190600101612d6c565b6060810181835f5b60038110156129f8578151835260209283019290910190600101612d93565b610240810181835f5b60128110156129f8578151835260209283019290910190600101612dbb565b60e0810181835f5b60078110156129f8578151835260209283019290910190600101612de2565b6080810181835f5b60048110156129f8578151835260209283019290910190600101612e09565b5f60208284031215612e38575f5ffd5b815167ffffffffffffffff811115612e4e575f5ffd5b8201601f81018413612e5e575f5ffd5b8051612e6c61299b82612930565b818152856020838501011115612e80575f5ffd5b8160208401602083015e5f91810160200191909152949350505050565b600181811c90821680612eb157607f821691505b6020821081036111f457634e487b7160e01b5f52602260045260245ffd5b601f821115612f1657805f5260205f20601f840160051c81016020851015612ef45750805b601f840160051c820191505b81811015612f13575f8155600101612f00565b50505b505050565b815167ffffffffffffffff811115612f3557612f356128eb565b612f4981612f438454612e9d565b84612ecf565b6020601f821160018114612f7b575f8315612f645750848201515b5f19600385901b1c1916600184901b178455612f13565b5f84815260208120601f198516915b82811015612faa5787850151825560209485019460019092019101612f8a565b5084821015612fc757868401515f19600387901b60f8161c191681555b50505050600190811b01905550565b5f81518060208401855e5f93019283525090919050565b5f612ff88284612fd6565b605d60f81b81526001019392505050565b6f2e54696d657374616d7050726f6f665b60801b81525f61279d6010830184612fd6565b634e487b7160e01b5f52603260045260245ffd5b7f2e486973746f726963616c53756d6d61727950726f6f665b000000000000000081525f61279d6018830184612fd6565b7f2e5769746864726177616c43726564656e7469616c50726f6f665b000000000081525f61279d601b830184612fd6565b81515f90829060208501835b828110156130cd5781518452602093840193909101906001016130af565b509195945050505050565b7f2e5374617465526f6f74416761696e73744c6174657374426c6f636b48656164815267657250726f6f665b60c01b60208201525f61279d6028830184612fd6565b702e56616c696461746f724669656c64735b60781b81525f61279d6011830184612fd6565b6f2e56616c696461746f7250726f6f665b60801b81525f61279d6010830184612fd6565b702e5769746864726177616c50726f6f665b60781b81525f61279d6011830184612fd6565b752e56616c696461746f724669656c647350726f6f665b60501b81525f61279d6016830184612fd6565b712e5769746864726177616c4669656c64735b60701b81525f61279d6012830184612fd6565b6a2e536c6f7450726f6f665b60a81b81525f61279d600b830184612fd6565b712e426c6f636b48656164657250726f6f665b60701b81525f61279d6012830184612fd6565b5f6020828403121561322d575f5ffd5b5051919050565b7f2e457865637574696f6e5061796c6f616450726f6f665b00000000000000000081525f61279d6017830184612fd6565b604081525f6132776040830185612a74565b82810360208401526132898185612a74565b9594505050505056fe0000000000000000000000007109709ecfa91a80626ff3989d68f67f5b1dd12da2646970667358221220d1bc6a448038a2d6c3e53c1422edbf4b32aec68abaae855e05432d2aededd20664736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02?W_5`\xE0\x1C\x80cvZ\xA6\x06\x11a\x015W\x80c\xB3\x80a\xBF\x11a\0\xB4W\x80c\xDB6J@\x11a\0yW\x80c\xDB6J@\x14a\x04AW\x80c\xDF\x18\xC7\x83\x14a\x04VW\x80c\xE2\x0C\x9Fq\x14a\x04kW\x80c\xF1H\x08,\x14a\x04sW\x80c\xFAv&\xD4\x14a\x04{W__\xFD[\x80c\xB3\x80a\xBF\x14a\x04\tW\x80c\xB5P\x8A\xA9\x14a\x04\x11W\x80c\xBAAO\xA6\x14a\x04\x19W\x80c\xD6F\x1C\xBB\x14a\x041W\x80c\xD9DG/\x14a\x049W__\xFD[\x80c\x9A\xCF\xAC)\x11a\0\xFAW\x80c\x9A\xCF\xAC)\x14a\x03\xBAW\x80c\x9D\xE4\xA9\xB3\x14a\x03\xCFW\x80c\xA1W\xEFL\x14a\x03\xE4W\x80c\xA5\x07t)\x14a\x03\xECW\x80c\xB0FO\xDC\x14a\x04\x01W__\xFD[\x80cvZ\xA6\x06\x14a\x03xW\x80c\x85\"l\x81\x14a\x03\x80W\x80c\x898\x93\xCA\x14a\x03\x95W\x80c\x91j\x17\xC6\x14a\x03\x9DW\x80c\x95\x0B\xB6\x82\x14a\x03\xB2W__\xFD[\x80c4\xE3\xD9\x0E\x11a\x01\xC1W\x80cL8\xF9\x13\x11a\x01\x86W\x80cL8\xF9\x13\x14a\x03)W\x80cd\xC7%5\x14a\x03>W\x80cd\xF3\x8E\xD7\x14a\x03SW\x80cf\xD9\xA9\xA0\x14a\x03[W\x80cl\x87|\x84\x14a\x03pW__\xFD[\x80c4\xE3\xD9\x0E\x14a\x02\xF4W\x80c>^<#\x14a\x02\xFCW\x80c?r\x86\xF4\x14a\x03\x04W\x80cB\x86G4\x14a\x03\x0CW\x80cL \xF5\x10\x14a\x03\x14W__\xFD[\x80c\x1E\xD7\x83\x1C\x11a\x02\x07W\x80c\x1E\xD7\x83\x1C\x14a\x02\x98W\x80c'Sx\xB1\x14a\x02\xADW\x80c(r\xE2\x0C\x14a\x02\xB5W\x80c*\xDE8\x80\x14a\x02\xCAW\x80c/\xD1y<\x14a\x02\xDFW__\xFD[\x80c\x08\xA4\xD7\x1F\x14a\x02CW\x80c\x16\xF0qS\x14a\x02XW\x80c\x17uA\xFC\x14a\x02sW\x80c\x18\xAA\xDF1\x14a\x02{W\x80c\x19l\xDF\xBD\x14a\x02\x83W[__\xFD[a\x02Va\x02Q6`\x04a)WV[a\x04\x88V[\0[a\x02`a\x05\nV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02`a\x05\xC3V[a\x02`a\x06~V[a\x02\x8Ba\x07?V[`@Qa\x02j\x91\x90a)\xD1V[a\x02\xA0a\tvV[`@Qa\x02j\x91\x90a*\x01V[a\x02`a\t\xD5V[a\x02\xBDa\n\x90V[`@Qa\x02j\x91\x90a*LV[a\x02\xD2a\x0B\xB8V[`@Qa\x02j\x91\x90a*\xFCV[a\x02\xE7a\x0C\xF4V[`@Qa\x02j\x91\x90a+\x7FV[a\x02\xE7a\x0E;V[a\x02\xA0a\x0FXV[a\x02\xA0a\x0F\xB6V[a\x02`a\x10\x14V[a\x03\x1Ca\x10\xD4V[`@Qa\x02j\x91\x90a+\x91V[a\x031a\x11\xFAV[`@Qa\x02j\x91\x90a+\xC8V[a\x03Fa\x13\"V[`@Qa\x02j\x91\x90a+\xF0V[a\x02`a\x14JV[a\x03ca\x15\x05V[`@Qa\x02j\x91\x90a,\\V[a\x02`a\x16iV[a\x02`a\x17\x19V[a\x03\x88a\x17\xD3V[`@Qa\x02j\x91\x90a,\xDAV[a\x03\x1Ca\x18\x9EV[a\x03\xA5a\x19\xBEV[`@Qa\x02j\x91\x90a,\xECV[a\x03\x1Ca\x1A\x9FV[a\x03\xC2a\x1B\xBDV[`@Qa\x02j\x91\x90a-cV[a\x03\xD7a\x1C\xE5V[`@Qa\x02j\x91\x90a-\x8BV[a\x02\xE7a\x1E\x0CV[a\x03\xF4a\x1E\x16V[`@Qa\x02j\x91\x90a-\xB2V[a\x03\xA5a\x1F>V[a\x02`a \x1FV[a\x03\x88a \xD8V[a\x04!a!\xA3V[`@Q\x90\x15\x15\x81R` \x01a\x02jV[a\x02`a\"<V[a\x02`a#\x04V[a\x04Ia#\xC4V[`@Qa\x02j\x91\x90a-\xDAV[a\x04^a$\xEBV[`@Qa\x02j\x91\x90a.\x01V[a\x02\xA0a&\x12V[a\x02`a&pV[`\x1FTa\x04!\x90`\xFF\x16\x81V[`@Qc`\xF9\xBB\x11`\xE0\x1B\x81R_Q` a2\x93_9_Q\x90_R\x90c`\xF9\xBB\x11\x90a\x04\xB8\x90\x84\x90`\x04\x01a+\x7FV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xD2W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\xF9\x91\x90\x81\x01\x90a.(V[` \x90a\x05\x06\x90\x82a/\x1BV[PPV[_a\x05\xBE` \x80Ta\x05\x1B\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x05G\x90a.\x9DV[\x80\x15a\x05\x92W\x80`\x1F\x10a\x05iWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x05\x92V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x05uW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x0B\x9C\xDB\x1B\xDD\x14\x9B\xDB\xDD`\xBA\x1B\x81RPa'*V[\x90P\x90V[_a\x05\xBE` \x80Ta\x05\xD4\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\0\x90a.\x9DV[\x80\x15a\x06KW\x80`\x1F\x10a\x06\"Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x06KV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x99XX\xDB\xDB\x94\xDD\x18]\x19T\x9B\xDB\xDD`\x82\x1B\x81RPa'*V[_a\x05\xBE` \x80Ta\x06\x8F\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x06\xBB\x90a.\x9DV[\x80\x15a\x07\x06W\x80`\x1F\x10a\x06\xDDWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x07\x06V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x06\xE9W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x16\x81R` \x01u\x0B\x9B\x18]\x19\\\xDD\x10\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`R\x1B\x81RPa'*V[a\x07Ga'\xD8V[_[`\x05\x81\x10\x15a\tAW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x98W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xBF\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x07\xCF\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x07\xEC\x91` \x01a0\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x08\t\x91\x90a/\x1BV[Pa\t$` \x80Ta\x08\x1A\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08F\x90a.\x9DV[\x80\x15a\x08\x91W\x80`\x1F\x10a\x08hWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x08\x91V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08tW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`!\x80Ta\x08\xA3\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x08\xCF\x90a.\x9DV[\x80\x15a\t\x1AW\x80`\x1F\x10a\x08\xF1Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\t\x1AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x08\xFDW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPPa'*V[`\xAB\x82`\x05\x81\x10a\t7Wa\t7a0-V[\x01U`\x01\x01a\x07IV[P`@\x80Q`\xA0\x81\x01\x91\x82\x90R\x90`\xAB\x90`\x05\x90\x82\x84[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\tXW[PPPPP\x90P\x90V[```\x16\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\t\xE6\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\n\x12\x90a.\x9DV[\x80\x15a\n]W\x80`\x1F\x10a\n4Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\n]V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\n@W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x0B\x98\x9B\x1B\xD8\xDA\xD2\x19XY\x19\\\x94\x9B\xDB\xDD`\x82\x1B\x81RPa'*V[a\n\x98a'\xF6V[_[`,\x81\x10\x15a\x0B\x88W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xE9W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B\x10\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x0B \x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0B=\x91` \x01a0AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x0BZ\x91\x90a/\x1BV[Pa\x0Bk` \x80Ta\x08\x1A\x90a.\x9DV[`x\x82`,\x81\x10a\x0B~Wa\x0B~a0-V[\x01U`\x01\x01a\n\x9AV[P`@\x80Qa\x05\x80\x81\x01\x91\x82\x90R`x\x80T\x82R\x90\x91`,\x90`y` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x1E\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x80\x82 `@\x80Q\x80\x82\x01\x82R`\x02\x87\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x95\x91\x94\x86\x81\x01\x94\x91\x92\x90\x84\x01[\x82\x82\x10\x15a\x0C\xD4W\x83\x82\x90_R` _ \x01\x80Ta\x0CI\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x0Cu\x90a.\x9DV[\x80\x15a\x0C\xC0W\x80`\x1F\x10a\x0C\x97Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x0C\xC0V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x0C\xA3W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x0C,V[PPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0B\xDBV[PPPP\x90P\x90V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P_[`.\x81\x10\x15a\x0E\x14W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rmW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\r\x94\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\r\xA4\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\r\xC1\x91` \x01a0rV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\r\xDE\x91\x90a/\x1BV[Pa\r\xEF` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x0E\x01Wa\x0E\x01a0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\r\x1EV[P\x80`@Q` \x01a\x0E&\x91\x90a0\xA3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x90V[`@\x80Q`\x03\x80\x82R`\x80\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01\x84\x806\x837\x01\x90PP\x90P_[`\x03\x81\x10\x15a\x0E\x14W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\xB1W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0E\xD8\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x0E\xE8\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x0F\x05\x91` \x01a0\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x0F\"\x91\x90a/\x1BV[Pa\x0F3` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x0FEWa\x0FEa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x0EbV[```\x18\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[```\x17\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\x10%\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x10Q\x90a.\x9DV[\x80\x15a\x10\x9CW\x80`\x1F\x10a\x10sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x10\x9CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x10\x7FW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x05\xCCM\x8D\xECmi\x0C\xAC,\x8C\xAEJM\xED\xEE\x89-\xCC\x8C\xAF`[\x1B\x81RPa'\xA4V[`@\x80Q`\x08\x80\x82Ra\x01 \x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x01\0\x806\x837\x01\x90PP\x90P_[`\x08\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11MW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x11t\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x11\x84\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x11\xA1\x91` \x01a1\x1AV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x11\xBE\x91\x90a/\x1BV[Pa\x11\xCF` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x11\xE1Wa\x11\xE1a0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x10\xFEV[P\x91\x90PV[a\x12\x02a(\x15V[_[`.\x81\x10\x15a\x12\xF2W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12SW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12z\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x12\x8A\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x12\xA7\x91` \x01a1?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x12\xC4\x91\x90a/\x1BV[Pa\x12\xD5` \x80Ta\x08\x1A\x90a.\x9DV[`J\x82`.\x81\x10a\x12\xE8Wa\x12\xE8a0-V[\x01U`\x01\x01a\x12\x04V[P`@\x80Qa\x05\xC0\x81\x01\x91\x82\x90R`J\x80T\x82R\x90\x91`.\x90`K` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a\x13*a(4V[_[`\t\x81\x10\x15a\x14\x1AW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13{W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xA2\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x13\xB2\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x13\xCF\x91` \x01a1cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x13\xEC\x91\x90a/\x1BV[Pa\x13\xFD` \x80Ta\x08\x1A\x90a.\x9DV[`A\x82`\t\x81\x10a\x14\x10Wa\x14\x10a0-V[\x01U`\x01\x01a\x13,V[P`@\x80Qa\x01 \x81\x01\x91\x82\x90R`A\x80T\x82R\x90\x91`\t\x90`B` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta\x14[\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x14\x87\x90a.\x9DV[\x80\x15a\x14\xD2W\x80`\x1F\x10a\x14\xA9Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x14\xD2V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x14\xB5W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x10\x81R` \x01o\x05\xCE\xED.\x8D\x0C\x8EL.\xEC-\x89-\xCC\x8C\xAF`\x83\x1B\x81RPa'\xA4V[```\x1B\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x90`\x02\x02\x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01\x80Ta\x15X\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15\x84\x90a.\x9DV[\x80\x15a\x15\xCFW\x80`\x1F\x10a\x15\xA6Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\xCFV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\xB2W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01`\x01\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x16QW` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x16\x13W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x15(V[_a\x05\xBE` \x80Ta\x16z\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x16\xA6\x90a.\x9DV[\x80\x15a\x16\xF1W\x80`\x1F\x10a\x16\xC8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x16\xF1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x16\xD4W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x05\x81R` \x01d\x0B\x9C\xDB\x1B\xDD`\xDA\x1B\x81RPa'\xA4V[_a\x05\xBE` \x80Ta\x17*\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x17V\x90a.\x9DV[\x80\x15a\x17\xA1W\x80`\x1F\x10a\x17xWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x17\xA1V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x17\x84W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0F\x81R` \x01n\x05\xCE\xCC-\x8D,\x8C.\x8D\xEEI-\xCC\x8C\xAF`\x8B\x1B\x81RPa'\xA4V[```\x1A\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x01\x80Ta\x18\x13\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x18?\x90a.\x9DV[\x80\x15a\x18\x8AW\x80`\x1F\x10a\x18aWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x18\x8AV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x18mW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a\x17\xF6V[`@\x80Q`.\x80\x82Ra\x05\xE0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01a\x05\xC0\x806\x837\x01\x90PP\x90P_[`.\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x17W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x19>\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x19N\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x19k\x91` \x01a1\x88V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x19\x88\x91\x90a/\x1BV[Pa\x19\x99` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x19\xABWa\x19\xABa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x18\xC8V[```\x1D\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x1A\x87W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1AIW\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x19\xE1V[`@\x80Q`\x04\x80\x82R`\xA0\x82\x01\x90\x92R``\x91_\x91\x90` \x82\x01`\x80\x806\x837\x01\x90PP\x90P_[`\x04\x81\x10\x15a\x11\xF4W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\x16W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1B=\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1BM\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1Bj\x91` \x01a1\xB2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1B\x87\x91\x90a/\x1BV[Pa\x1B\x98` \x80Ta\x08\x1A\x90a.\x9DV[\x82\x82\x81Q\x81\x10a\x1B\xAAWa\x1B\xAAa0-V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\x1A\xC7V[a\x1B\xC5a(SV[_[`\n\x81\x10\x15a\x1C\xB5W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x16W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C=\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1CM\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1Cj\x91` \x01a1cV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1C\x87\x91\x90a/\x1BV[Pa\x1C\x98` \x80Ta\x08\x1A\x90a.\x9DV[`7\x82`\n\x81\x10a\x1C\xABWa\x1C\xABa0-V[\x01U`\x01\x01a\x1B\xC7V[P`@\x80Qa\x01@\x81\x01\x91\x82\x90R`7\x80T\x82R\x90\x91`\n\x90`8` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a\x1C\xEDa(rV[_[`\x03\x81\x10\x15a\x1D\xDDW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D>W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1De\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1Du\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1D\x92\x91` \x01a1\xD8V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1D\xAF\x91\x90a/\x1BV[Pa\x1D\xC0` \x80Ta\x08\x1A\x90a.\x9DV[`4\x82`\x03\x81\x10a\x1D\xD3Wa\x1D\xD3a0-V[\x01U`\x01\x01a\x1C\xEFV[P`@\x80Q``\x81\x01\x91\x82\x90R`4\x80T\x82R\x90\x91`\x03\x90`5` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[``a\x05\xBEa\x0C\xF4V[a\x1E\x1Ea(\x90V[_[`\x12\x81\x10\x15a\x1F\x0EW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1E\x96\x91\x90\x81\x01\x90a.(V[`@Q` \x01a\x1E\xA6\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra\x1E\xC3\x91` \x01a1\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a\x1E\xE0\x91\x90a/\x1BV[Pa\x1E\xF1` \x80Ta\x08\x1A\x90a.\x9DV[`\"\x82`\x12\x81\x10a\x1F\x04Wa\x1F\x04a0-V[\x01U`\x01\x01a\x1E V[P`@\x80Qa\x02@\x81\x01\x91\x82\x90R`\"\x80T\x82R\x90\x91`\x12\x90`#` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x1C\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW_\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x82R`\x02\x86\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a \x07W` \x02\x82\x01\x91\x90_R` _ \x90_\x90[\x82\x82\x90T\x90a\x01\0\n\x90\x04`\xE0\x1B`\x01`\x01`\xE0\x1B\x03\x19\x16\x81R` \x01\x90`\x04\x01\x90` \x82`\x03\x01\x04\x92\x83\x01\x92`\x01\x03\x82\x02\x91P\x80\x84\x11a\x1F\xC9W\x90P[PPPPP\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x1FaV[_a\x05\xBE` \x80Ta 0\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta \\\x90a.\x9DV[\x80\x15a \xA7W\x80`\x1F\x10a ~Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a \xA7V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a \x8AW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x0E\x81R` \x01m\x0B\x9D\x1A[Y\\\xDD\x18[\\\x14\x9B\xDB\xDD`\x92\x1B\x81RPa'*V[```\x19\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0C\xEBW\x83\x82\x90_R` _ \x01\x80Ta!\x18\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta!D\x90a.\x9DV[\x80\x15a!\x8FW\x80`\x1F\x10a!fWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a!\x8FV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a!rW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81R` \x01\x90`\x01\x01\x90a \xFBV[`\x08T_\x90`\xFF\x16\x15a!\xBAWP`\x08T`\xFF\x16\x90V[`@Qc\x06g\xF9\xD7`\xE4\x1B\x81R_Q` a2\x93_9_Q\x90_R`\x04\x82\x01\x81\x90Re\x19\x98Z[\x19Y`\xD2\x1B`$\x83\x01R_\x91cf\x7F\x9Dp\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\"\x11W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\"5\x91\x90a2\x1DV[\x14\x15\x90P\x90V[_a\x05\xBE` \x80Ta\"M\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\"y\x90a.\x9DV[\x80\x15a\"\xC4W\x80`\x1F\x10a\"\x9BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\"\xC4V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\"\xA7W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x17\x81R` \x01\x7F.historicalSummaryIndex\0\0\0\0\0\0\0\0\0\x81RPa'\xA4V[_a\x05\xBE` \x80Ta#\x15\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta#A\x90a.\x9DV[\x80\x15a#\x8CW\x80`\x1F\x10a#cWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a#\x8CV[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a#oW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x15\x81R` \x01t\x0B\x99^\x19X\xDD]\x1A[\xDB\x94\x18^[\x1B\xD8Y\x14\x9B\xDB\xDD`Z\x1B\x81RPa'*V[a#\xCCa(\xAFV[_[`\x07\x81\x10\x15a$\xBCW`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a$\x1DW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra$D\x91\x90\x81\x01\x90a.(V[`@Q` \x01a$T\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra$q\x91` \x01a24V[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a$\x8E\x91\x90a/\x1BV[Pa$\x9F` \x80Ta\x08\x1A\x90a.\x9DV[`\xA4\x82`\x07\x81\x10a$\xB2Wa$\xB2a0-V[\x01U`\x01\x01a#\xCEV[P`@\x80Q`\xE0\x81\x01\x91\x82\x90R`\xA4\x80T\x82R\x90\x91`\x07\x90`\xA5` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[a$\xF3a(\xCDV[_[`\x04\x81\x10\x15a%\xE3W`@Qc4\x80Q\xD7`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R_Q` a2\x93_9_Q\x90_R\x90ci\0\xA3\xAE\x90`$\x01_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a%DW=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra%k\x91\x90\x81\x01\x90a.(V[`@Q` \x01a%{\x91\x90a/\xEDV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90Ra%\x98\x91` \x01a0\tV[`@Q` \x81\x83\x03\x03\x81R\x90`@R`!\x90\x81a%\xB5\x91\x90a/\x1BV[Pa%\xC6` \x80Ta\x08\x1A\x90a.\x9DV[`\xB0\x82`\x04\x81\x10a%\xD9Wa%\xD9a0-V[\x01U`\x01\x01a$\xF5V[P`@\x80Q`\x80\x81\x01\x91\x82\x90R`\xB0\x80T\x82R\x90\x91`\x04\x90`\xB1` \x85\x01\x80\x83\x11a\tXWPPPPP\x90P\x90V[```\x15\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\tlW` \x02\x82\x01\x91\x90_R` _ \x90\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\t\xAEWPPPPP\x90P\x90V[_a\x05\xBE` \x80Ta&\x81\x90a.\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta&\xAD\x90a.\x9DV[\x80\x15a&\xF8W\x80`\x1F\x10a&\xCFWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a&\xF8V[\x82\x01\x91\x90_R` _ \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a&\xDBW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r.ValidatorFields[0]`h\x1B\x81RP[`@Qc\x17w\xE5\x9D`\xE0\x1B\x81R_\x90_Q` a2\x93_9_Q\x90_R\x90c\x17w\xE5\x9D\x90a'^\x90\x86\x90\x86\x90`\x04\x01a2eV[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'yW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a'\x9D\x91\x90a2\x1DV[\x93\x92PPPV[`@QcV\xEE\xF1[`\xE1\x1B\x81R_\x90_Q` a2\x93_9_Q\x90_R\x90c\xAD\xDD\xE2\xB6\x90a'^\x90\x86\x90\x86\x90`\x04\x01a2eV[`@Q\x80`\xA0\x01`@R\x80`\x05\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\x80\x01`@R\x80`,\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x05\xC0\x01`@R\x80`.\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01 \x01`@R\x80`\t\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x01@\x01`@R\x80`\n\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80a\x02@\x01`@R\x80`\x12\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xE0\x01`@R\x80`\x07\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a)(Wa)(a(\xEBV[`@R\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)IWa)Ia(\xEBV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[_` \x82\x84\x03\x12\x15a)gW__\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)}W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a)\x8DW__\xFD[\x805a)\xA0a)\x9B\x82a)0V[a(\xFFV[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a)\xB4W__\xFD[\x81` \x84\x01` \x83\x017_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\xA0\x81\x01\x81\x83_[`\x05\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a)\xD9V[PPP\x92\x91PPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a*AW\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a*\x1AV[P\x90\x95\x94PPPPPV[a\x05\x80\x81\x01\x81\x83_[`,\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a*UV[_\x81Q\x80\x84R\x80` \x84\x01` \x86\x01^_` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[_\x82\x82Q\x80\x85R` \x85\x01\x94P` \x81`\x05\x1B\x83\x01\x01` \x85\x01_[\x83\x81\x10\x15a*\xF0W`\x1F\x19\x85\x84\x03\x01\x88Ra*\xDA\x83\x83Qa*tV[` \x98\x89\x01\x98\x90\x93P\x91\x90\x91\x01\x90`\x01\x01a*\xBEV[P\x90\x96\x95PPPPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a+]\x90\x87\x01\x82a*\xA2V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a+\"V[P\x92\x96\x95PPPPPPV[` \x81R_a'\x9D` \x83\x01\x84a*tV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a*AW\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a+\xAAV[a\x05\xC0\x81\x01\x81\x83_[`.\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\xD1V[a\x01 \x81\x01\x81\x83_[`\t\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a+\xF9V[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a,RW\x81Q`\x01`\x01`\xE0\x1B\x03\x19\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a,*V[P\x93\x94\x93PPPPV[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW`?\x19\x87\x86\x03\x01\x84R\x81Q\x80Q`@\x87Ra,\xA8`@\x88\x01\x82a*tV[\x90P` \x82\x01Q\x91P\x86\x81\x03` \x88\x01Ra,\xC3\x81\x83a,\x18V[\x96PPP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a,\x82V[` \x81R_a'\x9D` \x83\x01\x84a*\xA2V[_` \x82\x01` \x83R\x80\x84Q\x80\x83R`@\x85\x01\x91P`@\x81`\x05\x1B\x86\x01\x01\x92P` \x86\x01_[\x82\x81\x10\x15a+sW\x86\x85\x03`?\x19\x01\x84R\x81Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x81\x01Q`@\x91\x87\x01\x82\x90R\x90a-M\x90\x87\x01\x82a,\x18V[\x95PP` \x93\x84\x01\x93\x91\x90\x91\x01\x90`\x01\x01a-\x12V[a\x01@\x81\x01\x81\x83_[`\n\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-lV[``\x81\x01\x81\x83_[`\x03\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\x93V[a\x02@\x81\x01\x81\x83_[`\x12\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\xBBV[`\xE0\x81\x01\x81\x83_[`\x07\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a-\xE2V[`\x80\x81\x01\x81\x83_[`\x04\x81\x10\x15a)\xF8W\x81Q\x83R` \x92\x83\x01\x92\x90\x91\x01\x90`\x01\x01a.\tV[_` \x82\x84\x03\x12\x15a.8W__\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a.NW__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a.^W__\xFD[\x80Qa.la)\x9B\x82a)0V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15a.\x80W__\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a.\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x11\xF4WcNH{q`\xE0\x1B_R`\"`\x04R`$_\xFD[`\x1F\x82\x11\x15a/\x16W\x80_R` _ `\x1F\x84\x01`\x05\x1C\x81\x01` \x85\x10\x15a.\xF4WP\x80[`\x1F\x84\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a/\x13W_\x81U`\x01\x01a/\0V[PP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/5Wa/5a(\xEBV[a/I\x81a/C\x84Ta.\x9DV[\x84a.\xCFV[` `\x1F\x82\x11`\x01\x81\x14a/{W_\x83\x15a/dWP\x84\x82\x01Q[_\x19`\x03\x85\x90\x1B\x1C\x19\x16`\x01\x84\x90\x1B\x17\x84Ua/\x13V[_\x84\x81R` \x81 `\x1F\x19\x85\x16\x91[\x82\x81\x10\x15a/\xAAW\x87\x85\x01Q\x82U` \x94\x85\x01\x94`\x01\x90\x92\x01\x91\x01a/\x8AV[P\x84\x82\x10\x15a/\xC7W\x86\x84\x01Q_\x19`\x03\x87\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPP`\x01\x90\x81\x1B\x01\x90UPV[_\x81Q\x80` \x84\x01\x85^_\x93\x01\x92\x83RP\x90\x91\x90PV[_a/\xF8\x82\x84a/\xD6V[`]`\xF8\x1B\x81R`\x01\x01\x93\x92PPPV[o.TimestampProof[`\x80\x1B\x81R_a'\x9D`\x10\x83\x01\x84a/\xD6V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x7F.HistoricalSummaryProof[\0\0\0\0\0\0\0\0\x81R_a'\x9D`\x18\x83\x01\x84a/\xD6V[\x7F.WithdrawalCredentialProof[\0\0\0\0\0\x81R_a'\x9D`\x1B\x83\x01\x84a/\xD6V[\x81Q_\x90\x82\x90` \x85\x01\x83[\x82\x81\x10\x15a0\xCDW\x81Q\x84R` \x93\x84\x01\x93\x90\x91\x01\x90`\x01\x01a0\xAFV[P\x91\x95\x94PPPPPV[\x7F.StateRootAgainstLatestBlockHead\x81RgerProof[`\xC0\x1B` \x82\x01R_a'\x9D`(\x83\x01\x84a/\xD6V[p.ValidatorFields[`x\x1B\x81R_a'\x9D`\x11\x83\x01\x84a/\xD6V[o.ValidatorProof[`\x80\x1B\x81R_a'\x9D`\x10\x83\x01\x84a/\xD6V[p.WithdrawalProof[`x\x1B\x81R_a'\x9D`\x11\x83\x01\x84a/\xD6V[u.ValidatorFieldsProof[`P\x1B\x81R_a'\x9D`\x16\x83\x01\x84a/\xD6V[q.WithdrawalFields[`p\x1B\x81R_a'\x9D`\x12\x83\x01\x84a/\xD6V[j.SlotProof[`\xA8\x1B\x81R_a'\x9D`\x0B\x83\x01\x84a/\xD6V[q.BlockHeaderProof[`p\x1B\x81R_a'\x9D`\x12\x83\x01\x84a/\xD6V[_` \x82\x84\x03\x12\x15a2-W__\xFD[PQ\x91\x90PV[\x7F.ExecutionPayloadProof[\0\0\0\0\0\0\0\0\0\x81R_a'\x9D`\x17\x83\x01\x84a/\xD6V[`@\x81R_a2w`@\x83\x01\x85a*tV[\x82\x81\x03` \x84\x01Ra2\x89\x81\x85a*tV[\x95\x94PPPPPV\xFE\0\0\0\0\0\0\0\0\0\0\0\0q\tp\x9E\xCF\xA9\x1A\x80bo\xF3\x98\x9Dh\xF6\x7F[\x1D\xD1-\xA2dipfsX\"\x12 \xD1\xBCjD\x808\xA2\xD6\xC3\xE5<\x14\"\xED\xBFK2\xAE\xC6\x8A\xBA\xAE\x85^\x05C-*\xED\xED\xD2\x06dsolcC\0\x08\x1B\x003",
    );
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<excludeSelectorsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsCall {
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
            impl ::core::convert::From<excludeSelectorsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: excludeSelectorsReturn) -> Self {
                    (value.excludedSelectors_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for excludeSelectorsReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = excludeSelectorsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzSelector>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
    /**Function with signature `getBalanceUpdateProof()` and selector `0xa157ef4c`.
```solidity
function getBalanceUpdateProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateProofCall {}
    ///Container type for the return parameters of the [`getBalanceUpdateProof()`](getBalanceUpdateProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBalanceUpdateProofReturn {
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
            impl ::core::convert::From<getBalanceUpdateProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getBalanceUpdateProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBalanceUpdateProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBalanceUpdateProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBalanceUpdateProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBalanceUpdateProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBalanceUpdateProof()";
            const SELECTOR: [u8; 4] = [161u8, 87u8, 239u8, 76u8];
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
    /**Function with signature `getBeaconStateRoot()` and selector `0x177541fc`.
```solidity
function getBeaconStateRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBeaconStateRootCall {}
    ///Container type for the return parameters of the [`getBeaconStateRoot()`](getBeaconStateRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBeaconStateRootReturn {
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
            impl ::core::convert::From<getBeaconStateRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBeaconStateRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBeaconStateRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getBeaconStateRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBeaconStateRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBeaconStateRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBeaconStateRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBeaconStateRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBeaconStateRoot()";
            const SELECTOR: [u8; 4] = [23u8, 117u8, 65u8, 252u8];
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
    /**Function with signature `getBlockHeaderProof()` and selector `0xa5077429`.
```solidity
function getBlockHeaderProof() external returns (bytes32[18] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeaderProofCall {}
    ///Container type for the return parameters of the [`getBlockHeaderProof()`](getBlockHeaderProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockHeaderProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 18usize],
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
            impl ::core::convert::From<getBlockHeaderProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeaderProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeaderProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    18usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 18usize],
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
            impl ::core::convert::From<getBlockHeaderProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockHeaderProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockHeaderProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockHeaderProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockHeaderProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    18usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockHeaderProof()";
            const SELECTOR: [u8; 4] = [165u8, 7u8, 116u8, 41u8];
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
    /**Function with signature `getBlockRoot()` and selector `0x275378b1`.
```solidity
function getBlockRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootCall {}
    ///Container type for the return parameters of the [`getBlockRoot()`](getBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootReturn {
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
            impl ::core::convert::From<getBlockRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getBlockRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockRoot()";
            const SELECTOR: [u8; 4] = [39u8, 83u8, 120u8, 177u8];
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
    /**Function with signature `getBlockRootIndex()` and selector `0x42864734`.
```solidity
function getBlockRootIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootIndexCall {}
    ///Container type for the return parameters of the [`getBlockRootIndex()`](getBlockRootIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBlockRootIndexReturn {
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
            impl ::core::convert::From<getBlockRootIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockRootIndexCall {
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
            impl ::core::convert::From<getBlockRootIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getBlockRootIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getBlockRootIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBlockRootIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBlockRootIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBlockRootIndex()";
            const SELECTOR: [u8; 4] = [66u8, 134u8, 71u8, 52u8];
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
    /**Function with signature `getExecutionPayloadProof()` and selector `0xdb364a40`.
```solidity
function getExecutionPayloadProof() external returns (bytes32[7] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadProofCall {}
    ///Container type for the return parameters of the [`getExecutionPayloadProof()`](getExecutionPayloadProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 7usize],
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
            impl ::core::convert::From<getExecutionPayloadProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    7usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 7usize],
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
            impl ::core::convert::From<getExecutionPayloadProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getExecutionPayloadProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getExecutionPayloadProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    7usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getExecutionPayloadProof()";
            const SELECTOR: [u8; 4] = [219u8, 54u8, 74u8, 64u8];
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
    /**Function with signature `getExecutionPayloadRoot()` and selector `0xd944472f`.
```solidity
function getExecutionPayloadRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadRootCall {}
    ///Container type for the return parameters of the [`getExecutionPayloadRoot()`](getExecutionPayloadRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getExecutionPayloadRootReturn {
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
            impl ::core::convert::From<getExecutionPayloadRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getExecutionPayloadRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getExecutionPayloadRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getExecutionPayloadRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getExecutionPayloadRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getExecutionPayloadRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getExecutionPayloadRoot()";
            const SELECTOR: [u8; 4] = [217u8, 68u8, 71u8, 47u8];
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
    /**Function with signature `getHistoricalSummaryIndex()` and selector `0xd6461cbb`.
```solidity
function getHistoricalSummaryIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryIndexCall {}
    ///Container type for the return parameters of the [`getHistoricalSummaryIndex()`](getHistoricalSummaryIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryIndexReturn {
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
            impl ::core::convert::From<getHistoricalSummaryIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryIndexCall {
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
            impl ::core::convert::From<getHistoricalSummaryIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHistoricalSummaryIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHistoricalSummaryIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHistoricalSummaryIndex()";
            const SELECTOR: [u8; 4] = [214u8, 70u8, 28u8, 187u8];
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
    /**Function with signature `getHistoricalSummaryProof()` and selector `0x2872e20c`.
```solidity
function getHistoricalSummaryProof() external returns (bytes32[44] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryProofCall {}
    ///Container type for the return parameters of the [`getHistoricalSummaryProof()`](getHistoricalSummaryProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getHistoricalSummaryProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 44usize],
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
            impl ::core::convert::From<getHistoricalSummaryProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    44usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 44usize],
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
            impl ::core::convert::From<getHistoricalSummaryProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getHistoricalSummaryProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getHistoricalSummaryProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getHistoricalSummaryProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getHistoricalSummaryProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    44usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getHistoricalSummaryProof()";
            const SELECTOR: [u8; 4] = [40u8, 114u8, 226u8, 12u8];
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
    /**Function with signature `getLatestBlockRoot()` and selector `0x18aadf31`.
```solidity
function getLatestBlockRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestBlockRootCall {}
    ///Container type for the return parameters of the [`getLatestBlockRoot()`](getLatestBlockRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestBlockRootReturn {
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
            impl ::core::convert::From<getLatestBlockRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestBlockRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestBlockRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getLatestBlockRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestBlockRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestBlockRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestBlockRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestBlockRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestBlockRoot()";
            const SELECTOR: [u8; 4] = [24u8, 170u8, 223u8, 49u8];
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
    /**Function with signature `getSlot()` and selector `0x6c877c84`.
```solidity
function getSlot() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotCall {}
    ///Container type for the return parameters of the [`getSlot()`](getSlotCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotReturn {
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
            impl ::core::convert::From<getSlotCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotCall {
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
            impl ::core::convert::From<getSlotReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlot()";
            const SELECTOR: [u8; 4] = [108u8, 135u8, 124u8, 132u8];
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
    /**Function with signature `getSlotProof()` and selector `0x9de4a9b3`.
```solidity
function getSlotProof() external returns (bytes32[3] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotProofCall {}
    ///Container type for the return parameters of the [`getSlotProof()`](getSlotProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 3usize],
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
            impl ::core::convert::From<getSlotProofCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    3usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 3usize],
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
            impl ::core::convert::From<getSlotProofReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    3usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlotProof()";
            const SELECTOR: [u8; 4] = [157u8, 228u8, 169u8, 179u8];
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
    /**Function with signature `getSlotRoot()` and selector `0x16f07153`.
```solidity
function getSlotRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotRootCall {}
    ///Container type for the return parameters of the [`getSlotRoot()`](getSlotRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getSlotRootReturn {
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
            impl ::core::convert::From<getSlotRootCall> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getSlotRootReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getSlotRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getSlotRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getSlotRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getSlotRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getSlotRoot()";
            const SELECTOR: [u8; 4] = [22u8, 240u8, 113u8, 83u8];
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
    /**Function with signature `getStateRootProof()` and selector `0x34e3d90e`.
```solidity
function getStateRootProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofCall {}
    ///Container type for the return parameters of the [`getStateRootProof()`](getStateRootProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStateRootProofReturn {
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
            impl ::core::convert::From<getStateRootProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStateRootProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStateRootProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getStateRootProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStateRootProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStateRootProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStateRootProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStateRootProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStateRootProof()";
            const SELECTOR: [u8; 4] = [52u8, 227u8, 217u8, 14u8];
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
    /**Function with signature `getTimestampProofCapella()` and selector `0xdf18c783`.
```solidity
function getTimestampProofCapella() external returns (bytes32[4] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofCapellaCall {}
    ///Container type for the return parameters of the [`getTimestampProofCapella()`](getTimestampProofCapellaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofCapellaReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 4usize],
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
            impl ::core::convert::From<getTimestampProofCapellaCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofCapellaCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofCapellaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    4usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 4usize],
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
            impl ::core::convert::From<getTimestampProofCapellaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofCapellaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofCapellaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampProofCapellaCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampProofCapellaReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    4usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampProofCapella()";
            const SELECTOR: [u8; 4] = [223u8, 24u8, 199u8, 131u8];
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
    /**Function with signature `getTimestampProofDeneb()` and selector `0x196cdfbd`.
```solidity
function getTimestampProofDeneb() external returns (bytes32[5] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofDenebCall {}
    ///Container type for the return parameters of the [`getTimestampProofDeneb()`](getTimestampProofDenebCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampProofDenebReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 5usize],
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
            impl ::core::convert::From<getTimestampProofDenebCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofDenebCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofDenebCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    5usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 5usize],
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
            impl ::core::convert::From<getTimestampProofDenebReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampProofDenebReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampProofDenebReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampProofDenebCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampProofDenebReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    5usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampProofDeneb()";
            const SELECTOR: [u8; 4] = [25u8, 108u8, 223u8, 189u8];
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
    /**Function with signature `getTimestampRoot()` and selector `0xb38061bf`.
```solidity
function getTimestampRoot() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampRootCall {}
    ///Container type for the return parameters of the [`getTimestampRoot()`](getTimestampRootCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTimestampRootReturn {
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
            impl ::core::convert::From<getTimestampRootCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampRootCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampRootCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getTimestampRootReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTimestampRootReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTimestampRootReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTimestampRootCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTimestampRootReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTimestampRoot()";
            const SELECTOR: [u8; 4] = [179u8, 128u8, 97u8, 191u8];
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
    /**Function with signature `getValidatorFields()` and selector `0x4c20f510`.
```solidity
function getValidatorFields() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsCall {}
    ///Container type for the return parameters of the [`getValidatorFields()`](getValidatorFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsReturn {
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
            impl ::core::convert::From<getValidatorFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getValidatorFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorFieldsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorFieldsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorFields()";
            const SELECTOR: [u8; 4] = [76u8, 32u8, 245u8, 16u8];
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
    /**Function with signature `getValidatorFieldsProof()` and selector `0x893893ca`.
```solidity
function getValidatorFieldsProof() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsProofCall {}
    ///Container type for the return parameters of the [`getValidatorFieldsProof()`](getValidatorFieldsProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorFieldsProofReturn {
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
            impl ::core::convert::From<getValidatorFieldsProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getValidatorFieldsProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorFieldsProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorFieldsProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorFieldsProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorFieldsProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorFieldsProof()";
            const SELECTOR: [u8; 4] = [137u8, 56u8, 147u8, 202u8];
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
    /**Function with signature `getValidatorIndex()` and selector `0x765aa606`.
```solidity
function getValidatorIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorIndexCall {}
    ///Container type for the return parameters of the [`getValidatorIndex()`](getValidatorIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorIndexReturn {
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
            impl ::core::convert::From<getValidatorIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorIndexCall {
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
            impl ::core::convert::From<getValidatorIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorIndex()";
            const SELECTOR: [u8; 4] = [118u8, 90u8, 166u8, 6u8];
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
    /**Function with signature `getValidatorProof()` and selector `0x4c38f913`.
```solidity
function getValidatorProof() external returns (bytes32[46] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorProofCall {}
    ///Container type for the return parameters of the [`getValidatorProof()`](getValidatorProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorProofReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 46usize],
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
            impl ::core::convert::From<getValidatorProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    46usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 46usize],
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
            impl ::core::convert::From<getValidatorProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorProofReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    46usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorProof()";
            const SELECTOR: [u8; 4] = [76u8, 56u8, 249u8, 19u8];
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
    /**Function with signature `getValidatorPubkeyHash()` and selector `0xf148082c`.
```solidity
function getValidatorPubkeyHash() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorPubkeyHashCall {}
    ///Container type for the return parameters of the [`getValidatorPubkeyHash()`](getValidatorPubkeyHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getValidatorPubkeyHashReturn {
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
            impl ::core::convert::From<getValidatorPubkeyHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorPubkeyHashCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorPubkeyHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getValidatorPubkeyHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getValidatorPubkeyHashReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getValidatorPubkeyHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getValidatorPubkeyHashCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getValidatorPubkeyHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getValidatorPubkeyHash()";
            const SELECTOR: [u8; 4] = [241u8, 72u8, 8u8, 44u8];
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
    /**Function with signature `getWithdrawalCredentialProof()` and selector `0x2fd1793c`.
```solidity
function getWithdrawalCredentialProof() external returns (bytes memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofCall {}
    ///Container type for the return parameters of the [`getWithdrawalCredentialProof()`](getWithdrawalCredentialProofCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalCredentialProofReturn {
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
            impl ::core::convert::From<getWithdrawalCredentialProofCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalCredentialProofCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalCredentialProofCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getWithdrawalCredentialProofReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalCredentialProofReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalCredentialProofReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalCredentialProofCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalCredentialProofReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalCredentialProof()";
            const SELECTOR: [u8; 4] = [47u8, 209u8, 121u8, 60u8];
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
    /**Function with signature `getWithdrawalFields()` and selector `0x950bb682`.
```solidity
function getWithdrawalFields() external returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalFieldsCall {}
    ///Container type for the return parameters of the [`getWithdrawalFields()`](getWithdrawalFieldsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalFieldsReturn {
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
            impl ::core::convert::From<getWithdrawalFieldsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalFieldsCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalFieldsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
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
            impl ::core::convert::From<getWithdrawalFieldsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalFieldsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalFieldsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalFieldsCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalFieldsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalFields()";
            const SELECTOR: [u8; 4] = [149u8, 11u8, 182u8, 130u8];
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
    /**Function with signature `getWithdrawalIndex()` and selector `0x64f38ed7`.
```solidity
function getWithdrawalIndex() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalIndexCall {}
    ///Container type for the return parameters of the [`getWithdrawalIndex()`](getWithdrawalIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalIndexReturn {
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
            impl ::core::convert::From<getWithdrawalIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalIndexCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalIndexCall {
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
            impl ::core::convert::From<getWithdrawalIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalIndexCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalIndex()";
            const SELECTOR: [u8; 4] = [100u8, 243u8, 142u8, 215u8];
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
    /**Function with signature `getWithdrawalProofCapella()` and selector `0x64c72535`.
```solidity
function getWithdrawalProofCapella() external returns (bytes32[9] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofCapellaCall {}
    ///Container type for the return parameters of the [`getWithdrawalProofCapella()`](getWithdrawalProofCapellaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofCapellaReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 9usize],
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
            impl ::core::convert::From<getWithdrawalProofCapellaCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofCapellaCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofCapellaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    9usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 9usize],
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
            impl ::core::convert::From<getWithdrawalProofCapellaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofCapellaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofCapellaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalProofCapellaCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalProofCapellaReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    9usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalProofCapella()";
            const SELECTOR: [u8; 4] = [100u8, 199u8, 37u8, 53u8];
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
    /**Function with signature `getWithdrawalProofDeneb()` and selector `0x9acfac29`.
```solidity
function getWithdrawalProofDeneb() external returns (bytes32[10] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofDenebCall {}
    ///Container type for the return parameters of the [`getWithdrawalProofDeneb()`](getWithdrawalProofDenebCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getWithdrawalProofDenebReturn {
        pub _0: [alloy::sol_types::private::FixedBytes<32>; 10usize],
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
            impl ::core::convert::From<getWithdrawalProofDenebCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofDenebCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofDenebCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    10usize,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                [alloy::sol_types::private::FixedBytes<32>; 10usize],
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
            impl ::core::convert::From<getWithdrawalProofDenebReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getWithdrawalProofDenebReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getWithdrawalProofDenebReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getWithdrawalProofDenebCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getWithdrawalProofDenebReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                    10usize,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getWithdrawalProofDeneb()";
            const SELECTOR: [u8; 4] = [154u8, 207u8, 172u8, 41u8];
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
    /**Function with signature `setJSON(string)` and selector `0x08a4d71f`.
```solidity
function setJSON(string memory path) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setJSONCall {
        pub path: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`setJSON(string)`](setJSONCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setJSONReturn {}
    #[allow(
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setJSONCall> for UnderlyingRustTuple<'_> {
                fn from(value: setJSONCall) -> Self {
                    (value.path,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setJSONCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { path: tuple.0 }
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
            impl ::core::convert::From<setJSONReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setJSONReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setJSONReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setJSONCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setJSONReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setJSON(string)";
            const SELECTOR: [u8; 4] = [8u8, 164u8, 215u8, 31u8];
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
                        &self.path,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzArtifactSelector as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzArtifactSelector>,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<targetInterfacesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <StdInvariant::FuzzInterface as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<targetInterfacesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: targetInterfacesReturn) -> Self {
                    (value.targetedInterfaces_,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for targetInterfacesReturn {
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
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = targetInterfacesReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<StdInvariant::FuzzInterface>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    ///Container for all the [`ProofParsing`](self) function calls.
    pub enum ProofParsingCalls {
        IS_TEST(IS_TESTCall),
        excludeArtifacts(excludeArtifactsCall),
        excludeContracts(excludeContractsCall),
        excludeSelectors(excludeSelectorsCall),
        excludeSenders(excludeSendersCall),
        failed(failedCall),
        getBalanceUpdateProof(getBalanceUpdateProofCall),
        getBeaconStateRoot(getBeaconStateRootCall),
        getBlockHeaderProof(getBlockHeaderProofCall),
        getBlockRoot(getBlockRootCall),
        getBlockRootIndex(getBlockRootIndexCall),
        getExecutionPayloadProof(getExecutionPayloadProofCall),
        getExecutionPayloadRoot(getExecutionPayloadRootCall),
        getHistoricalSummaryIndex(getHistoricalSummaryIndexCall),
        getHistoricalSummaryProof(getHistoricalSummaryProofCall),
        getLatestBlockRoot(getLatestBlockRootCall),
        getSlot(getSlotCall),
        getSlotProof(getSlotProofCall),
        getSlotRoot(getSlotRootCall),
        getStateRootProof(getStateRootProofCall),
        getTimestampProofCapella(getTimestampProofCapellaCall),
        getTimestampProofDeneb(getTimestampProofDenebCall),
        getTimestampRoot(getTimestampRootCall),
        getValidatorFields(getValidatorFieldsCall),
        getValidatorFieldsProof(getValidatorFieldsProofCall),
        getValidatorIndex(getValidatorIndexCall),
        getValidatorProof(getValidatorProofCall),
        getValidatorPubkeyHash(getValidatorPubkeyHashCall),
        getWithdrawalCredentialProof(getWithdrawalCredentialProofCall),
        getWithdrawalFields(getWithdrawalFieldsCall),
        getWithdrawalIndex(getWithdrawalIndexCall),
        getWithdrawalProofCapella(getWithdrawalProofCapellaCall),
        getWithdrawalProofDeneb(getWithdrawalProofDenebCall),
        setJSON(setJSONCall),
        targetArtifactSelectors(targetArtifactSelectorsCall),
        targetArtifacts(targetArtifactsCall),
        targetContracts(targetContractsCall),
        targetInterfaces(targetInterfacesCall),
        targetSelectors(targetSelectorsCall),
        targetSenders(targetSendersCall),
    }
    #[automatically_derived]
    impl ProofParsingCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 164u8, 215u8, 31u8],
            [22u8, 240u8, 113u8, 83u8],
            [23u8, 117u8, 65u8, 252u8],
            [24u8, 170u8, 223u8, 49u8],
            [25u8, 108u8, 223u8, 189u8],
            [30u8, 215u8, 131u8, 28u8],
            [39u8, 83u8, 120u8, 177u8],
            [40u8, 114u8, 226u8, 12u8],
            [42u8, 222u8, 56u8, 128u8],
            [47u8, 209u8, 121u8, 60u8],
            [52u8, 227u8, 217u8, 14u8],
            [62u8, 94u8, 60u8, 35u8],
            [63u8, 114u8, 134u8, 244u8],
            [66u8, 134u8, 71u8, 52u8],
            [76u8, 32u8, 245u8, 16u8],
            [76u8, 56u8, 249u8, 19u8],
            [100u8, 199u8, 37u8, 53u8],
            [100u8, 243u8, 142u8, 215u8],
            [102u8, 217u8, 169u8, 160u8],
            [108u8, 135u8, 124u8, 132u8],
            [118u8, 90u8, 166u8, 6u8],
            [133u8, 34u8, 108u8, 129u8],
            [137u8, 56u8, 147u8, 202u8],
            [145u8, 106u8, 23u8, 198u8],
            [149u8, 11u8, 182u8, 130u8],
            [154u8, 207u8, 172u8, 41u8],
            [157u8, 228u8, 169u8, 179u8],
            [161u8, 87u8, 239u8, 76u8],
            [165u8, 7u8, 116u8, 41u8],
            [176u8, 70u8, 79u8, 220u8],
            [179u8, 128u8, 97u8, 191u8],
            [181u8, 80u8, 138u8, 169u8],
            [186u8, 65u8, 79u8, 166u8],
            [214u8, 70u8, 28u8, 187u8],
            [217u8, 68u8, 71u8, 47u8],
            [219u8, 54u8, 74u8, 64u8],
            [223u8, 24u8, 199u8, 131u8],
            [226u8, 12u8, 159u8, 113u8],
            [241u8, 72u8, 8u8, 44u8],
            [250u8, 118u8, 38u8, 212u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for ProofParsingCalls {
        const NAME: &'static str = "ProofParsingCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 40usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_TEST(_) => <IS_TESTCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::failed(_) => <failedCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBalanceUpdateProof(_) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBeaconStateRoot(_) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockHeaderProof(_) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockRoot(_) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBlockRootIndex(_) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getExecutionPayloadProof(_) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getExecutionPayloadRoot(_) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHistoricalSummaryIndex(_) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getHistoricalSummaryProof(_) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestBlockRoot(_) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlot(_) => <getSlotCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getSlotProof(_) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getSlotRoot(_) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStateRootProof(_) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampProofCapella(_) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampProofDeneb(_) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTimestampRoot(_) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorFields(_) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorFieldsProof(_) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorIndex(_) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorProof(_) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getValidatorPubkeyHash(_) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalCredentialProof(_) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalFields(_) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalIndex(_) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalProofCapella(_) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getWithdrawalProofDeneb(_) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setJSON(_) => <setJSONCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::targetSenders(_) => {
                    <targetSendersCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<ProofParsingCalls>] = &[
                {
                    fn setJSON(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <setJSONCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::setJSON)
                    }
                    setJSON
                },
                {
                    fn getSlotRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlotRoot)
                    }
                    getSlotRoot
                },
                {
                    fn getBeaconStateRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBeaconStateRoot)
                    }
                    getBeaconStateRoot
                },
                {
                    fn getLatestBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getLatestBlockRoot)
                    }
                    getLatestBlockRoot
                },
                {
                    fn getTimestampProofDeneb(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampProofDeneb)
                    }
                    getTimestampProofDeneb
                },
                {
                    fn excludeSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeSenders)
                    }
                    excludeSenders
                },
                {
                    fn getBlockRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockRoot)
                    }
                    getBlockRoot
                },
                {
                    fn getHistoricalSummaryProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getHistoricalSummaryProof)
                    }
                    getHistoricalSummaryProof
                },
                {
                    fn targetInterfaces(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetInterfacesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetInterfaces)
                    }
                    targetInterfaces
                },
                {
                    fn getWithdrawalCredentialProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalCredentialProof)
                    }
                    getWithdrawalCredentialProof
                },
                {
                    fn getStateRootProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getStateRootProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getStateRootProof)
                    }
                    getStateRootProof
                },
                {
                    fn targetSenders(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetSendersCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetSenders)
                    }
                    targetSenders
                },
                {
                    fn targetContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetContracts)
                    }
                    targetContracts
                },
                {
                    fn getBlockRootIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockRootIndex)
                    }
                    getBlockRootIndex
                },
                {
                    fn getValidatorFields(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorFields)
                    }
                    getValidatorFields
                },
                {
                    fn getValidatorProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorProof)
                    }
                    getValidatorProof
                },
                {
                    fn getWithdrawalProofCapella(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalProofCapella)
                    }
                    getWithdrawalProofCapella
                },
                {
                    fn getWithdrawalIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalIndex)
                    }
                    getWithdrawalIndex
                },
                {
                    fn targetArtifactSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetArtifactSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetArtifactSelectors)
                    }
                    targetArtifactSelectors
                },
                {
                    fn getSlot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlot)
                    }
                    getSlot
                },
                {
                    fn getValidatorIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorIndex)
                    }
                    getValidatorIndex
                },
                {
                    fn targetArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetArtifacts)
                    }
                    targetArtifacts
                },
                {
                    fn getValidatorFieldsProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorFieldsProof)
                    }
                    getValidatorFieldsProof
                },
                {
                    fn targetSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <targetSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::targetSelectors)
                    }
                    targetSelectors
                },
                {
                    fn getWithdrawalFields(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalFields)
                    }
                    getWithdrawalFields
                },
                {
                    fn getWithdrawalProofDeneb(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getWithdrawalProofDeneb)
                    }
                    getWithdrawalProofDeneb
                },
                {
                    fn getSlotProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getSlotProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getSlotProof)
                    }
                    getSlotProof
                },
                {
                    fn getBalanceUpdateProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBalanceUpdateProof)
                    }
                    getBalanceUpdateProof
                },
                {
                    fn getBlockHeaderProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getBlockHeaderProof)
                    }
                    getBlockHeaderProof
                },
                {
                    fn excludeSelectors(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeSelectors)
                    }
                    excludeSelectors
                },
                {
                    fn getTimestampRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampRoot)
                    }
                    getTimestampRoot
                },
                {
                    fn excludeArtifacts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeArtifactsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeArtifacts)
                    }
                    excludeArtifacts
                },
                {
                    fn failed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <failedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::failed)
                    }
                    failed
                },
                {
                    fn getHistoricalSummaryIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getHistoricalSummaryIndex)
                    }
                    getHistoricalSummaryIndex
                },
                {
                    fn getExecutionPayloadRoot(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getExecutionPayloadRoot)
                    }
                    getExecutionPayloadRoot
                },
                {
                    fn getExecutionPayloadProof(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getExecutionPayloadProof)
                    }
                    getExecutionPayloadProof
                },
                {
                    fn getTimestampProofCapella(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getTimestampProofCapella)
                    }
                    getTimestampProofCapella
                },
                {
                    fn excludeContracts(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <excludeContractsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::excludeContracts)
                    }
                    excludeContracts
                },
                {
                    fn getValidatorPubkeyHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::getValidatorPubkeyHash)
                    }
                    getValidatorPubkeyHash
                },
                {
                    fn IS_TEST(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<ProofParsingCalls> {
                        <IS_TESTCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(ProofParsingCalls::IS_TEST)
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
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::getBalanceUpdateProof(inner) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBeaconStateRoot(inner) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockHeaderProof(inner) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockRoot(inner) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBlockRootIndex(inner) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getExecutionPayloadProof(inner) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getExecutionPayloadRoot(inner) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHistoricalSummaryIndex(inner) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getHistoricalSummaryProof(inner) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestBlockRoot(inner) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlot(inner) => {
                    <getSlotCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getSlotProof(inner) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getSlotRoot(inner) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStateRootProof(inner) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampProofCapella(inner) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampProofDeneb(inner) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTimestampRoot(inner) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorFields(inner) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorFieldsProof(inner) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorIndex(inner) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorProof(inner) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getValidatorPubkeyHash(inner) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalCredentialProof(inner) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalFields(inner) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalIndex(inner) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalProofCapella(inner) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getWithdrawalProofDeneb(inner) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setJSON(inner) => {
                    <setJSONCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_TEST(inner) => {
                    <IS_TESTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::excludeSelectors(inner) => {
                    <excludeSelectorsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::getBalanceUpdateProof(inner) => {
                    <getBalanceUpdateProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBeaconStateRoot(inner) => {
                    <getBeaconStateRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockHeaderProof(inner) => {
                    <getBlockHeaderProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockRoot(inner) => {
                    <getBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getBlockRootIndex(inner) => {
                    <getBlockRootIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getExecutionPayloadProof(inner) => {
                    <getExecutionPayloadProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getExecutionPayloadRoot(inner) => {
                    <getExecutionPayloadRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHistoricalSummaryIndex(inner) => {
                    <getHistoricalSummaryIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getHistoricalSummaryProof(inner) => {
                    <getHistoricalSummaryProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestBlockRoot(inner) => {
                    <getLatestBlockRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlot(inner) => {
                    <getSlotCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getSlotProof(inner) => {
                    <getSlotProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getSlotRoot(inner) => {
                    <getSlotRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStateRootProof(inner) => {
                    <getStateRootProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampProofCapella(inner) => {
                    <getTimestampProofCapellaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampProofDeneb(inner) => {
                    <getTimestampProofDenebCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTimestampRoot(inner) => {
                    <getTimestampRootCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorFields(inner) => {
                    <getValidatorFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorFieldsProof(inner) => {
                    <getValidatorFieldsProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorIndex(inner) => {
                    <getValidatorIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorProof(inner) => {
                    <getValidatorProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getValidatorPubkeyHash(inner) => {
                    <getValidatorPubkeyHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalCredentialProof(inner) => {
                    <getWithdrawalCredentialProofCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalFields(inner) => {
                    <getWithdrawalFieldsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalIndex(inner) => {
                    <getWithdrawalIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalProofCapella(inner) => {
                    <getWithdrawalProofCapellaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getWithdrawalProofDeneb(inner) => {
                    <getWithdrawalProofDenebCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setJSON(inner) => {
                    <setJSONCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
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
                Self::targetInterfaces(inner) => {
                    <targetInterfacesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`ProofParsing`](self) events.
    pub enum ProofParsingEvents {
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
    impl ProofParsingEvents {
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
    impl alloy_sol_types::SolEventInterface for ProofParsingEvents {
        const NAME: &'static str = "ProofParsingEvents";
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
    impl alloy_sol_types::private::IntoLogData for ProofParsingEvents {
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
    /**Creates a new wrapper around an on-chain [`ProofParsing`](self) contract instance.

See the [wrapper's documentation](`ProofParsingInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ProofParsingInstance<T, P, N> {
        ProofParsingInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<ProofParsingInstance<T, P, N>>,
    > {
        ProofParsingInstance::<T, P, N>::deploy(provider)
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
        ProofParsingInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`ProofParsing`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ProofParsing`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ProofParsingInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ProofParsingInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ProofParsingInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ProofParsingInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ProofParsing`](self) contract instance.

See the [wrapper's documentation](`ProofParsingInstance`) for more details.*/
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
        ) -> alloy_contract::Result<ProofParsingInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> ProofParsingInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ProofParsingInstance<T, P, N> {
            ProofParsingInstance {
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
    > ProofParsingInstance<T, P, N> {
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
        ///Creates a new call builder for the [`IS_TEST`] function.
        pub fn IS_TEST(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_TESTCall, N> {
            self.call_builder(&IS_TESTCall {})
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
        ///Creates a new call builder for the [`failed`] function.
        pub fn failed(&self) -> alloy_contract::SolCallBuilder<T, &P, failedCall, N> {
            self.call_builder(&failedCall {})
        }
        ///Creates a new call builder for the [`getBalanceUpdateProof`] function.
        pub fn getBalanceUpdateProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBalanceUpdateProofCall, N> {
            self.call_builder(&getBalanceUpdateProofCall {})
        }
        ///Creates a new call builder for the [`getBeaconStateRoot`] function.
        pub fn getBeaconStateRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBeaconStateRootCall, N> {
            self.call_builder(&getBeaconStateRootCall {})
        }
        ///Creates a new call builder for the [`getBlockHeaderProof`] function.
        pub fn getBlockHeaderProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockHeaderProofCall, N> {
            self.call_builder(&getBlockHeaderProofCall {})
        }
        ///Creates a new call builder for the [`getBlockRoot`] function.
        pub fn getBlockRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockRootCall, N> {
            self.call_builder(&getBlockRootCall {})
        }
        ///Creates a new call builder for the [`getBlockRootIndex`] function.
        pub fn getBlockRootIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBlockRootIndexCall, N> {
            self.call_builder(&getBlockRootIndexCall {})
        }
        ///Creates a new call builder for the [`getExecutionPayloadProof`] function.
        pub fn getExecutionPayloadProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getExecutionPayloadProofCall, N> {
            self.call_builder(&getExecutionPayloadProofCall {})
        }
        ///Creates a new call builder for the [`getExecutionPayloadRoot`] function.
        pub fn getExecutionPayloadRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getExecutionPayloadRootCall, N> {
            self.call_builder(&getExecutionPayloadRootCall {})
        }
        ///Creates a new call builder for the [`getHistoricalSummaryIndex`] function.
        pub fn getHistoricalSummaryIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHistoricalSummaryIndexCall, N> {
            self.call_builder(&getHistoricalSummaryIndexCall {})
        }
        ///Creates a new call builder for the [`getHistoricalSummaryProof`] function.
        pub fn getHistoricalSummaryProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getHistoricalSummaryProofCall, N> {
            self.call_builder(&getHistoricalSummaryProofCall {})
        }
        ///Creates a new call builder for the [`getLatestBlockRoot`] function.
        pub fn getLatestBlockRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestBlockRootCall, N> {
            self.call_builder(&getLatestBlockRootCall {})
        }
        ///Creates a new call builder for the [`getSlot`] function.
        pub fn getSlot(&self) -> alloy_contract::SolCallBuilder<T, &P, getSlotCall, N> {
            self.call_builder(&getSlotCall {})
        }
        ///Creates a new call builder for the [`getSlotProof`] function.
        pub fn getSlotProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlotProofCall, N> {
            self.call_builder(&getSlotProofCall {})
        }
        ///Creates a new call builder for the [`getSlotRoot`] function.
        pub fn getSlotRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getSlotRootCall, N> {
            self.call_builder(&getSlotRootCall {})
        }
        ///Creates a new call builder for the [`getStateRootProof`] function.
        pub fn getStateRootProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStateRootProofCall, N> {
            self.call_builder(&getStateRootProofCall {})
        }
        ///Creates a new call builder for the [`getTimestampProofCapella`] function.
        pub fn getTimestampProofCapella(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampProofCapellaCall, N> {
            self.call_builder(&getTimestampProofCapellaCall {})
        }
        ///Creates a new call builder for the [`getTimestampProofDeneb`] function.
        pub fn getTimestampProofDeneb(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampProofDenebCall, N> {
            self.call_builder(&getTimestampProofDenebCall {})
        }
        ///Creates a new call builder for the [`getTimestampRoot`] function.
        pub fn getTimestampRoot(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTimestampRootCall, N> {
            self.call_builder(&getTimestampRootCall {})
        }
        ///Creates a new call builder for the [`getValidatorFields`] function.
        pub fn getValidatorFields(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorFieldsCall, N> {
            self.call_builder(&getValidatorFieldsCall {})
        }
        ///Creates a new call builder for the [`getValidatorFieldsProof`] function.
        pub fn getValidatorFieldsProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorFieldsProofCall, N> {
            self.call_builder(&getValidatorFieldsProofCall {})
        }
        ///Creates a new call builder for the [`getValidatorIndex`] function.
        pub fn getValidatorIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorIndexCall, N> {
            self.call_builder(&getValidatorIndexCall {})
        }
        ///Creates a new call builder for the [`getValidatorProof`] function.
        pub fn getValidatorProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorProofCall, N> {
            self.call_builder(&getValidatorProofCall {})
        }
        ///Creates a new call builder for the [`getValidatorPubkeyHash`] function.
        pub fn getValidatorPubkeyHash(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getValidatorPubkeyHashCall, N> {
            self.call_builder(&getValidatorPubkeyHashCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalCredentialProof`] function.
        pub fn getWithdrawalCredentialProof(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalCredentialProofCall, N> {
            self.call_builder(
                &getWithdrawalCredentialProofCall {
                },
            )
        }
        ///Creates a new call builder for the [`getWithdrawalFields`] function.
        pub fn getWithdrawalFields(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalFieldsCall, N> {
            self.call_builder(&getWithdrawalFieldsCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalIndex`] function.
        pub fn getWithdrawalIndex(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalIndexCall, N> {
            self.call_builder(&getWithdrawalIndexCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalProofCapella`] function.
        pub fn getWithdrawalProofCapella(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalProofCapellaCall, N> {
            self.call_builder(&getWithdrawalProofCapellaCall {})
        }
        ///Creates a new call builder for the [`getWithdrawalProofDeneb`] function.
        pub fn getWithdrawalProofDeneb(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getWithdrawalProofDenebCall, N> {
            self.call_builder(&getWithdrawalProofDenebCall {})
        }
        ///Creates a new call builder for the [`setJSON`] function.
        pub fn setJSON(
            &self,
            path: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, setJSONCall, N> {
            self.call_builder(&setJSONCall { path })
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
        pub fn targetSenders(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, targetSendersCall, N> {
            self.call_builder(&targetSendersCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ProofParsingInstance<T, P, N> {
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
