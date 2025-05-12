///Module containing a contract's types and functions.
/**

```solidity
library BN254 {
    struct G1Point { uint256 X; uint256 Y; }
    struct G2Point { uint256[2] X; uint256[2] Y; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod BN254 {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct G1Point { uint256 X; uint256 Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G1Point {
        #[allow(missing_docs)]
        pub X: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub Y: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
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
        impl ::core::convert::From<G1Point> for UnderlyingRustTuple<'_> {
            fn from(value: G1Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G1Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G1Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G1Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.X,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.Y,
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
        impl alloy_sol_types::SolType for G1Point {
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
        impl alloy_sol_types::SolStruct for G1Point {
            const NAME: &'static str = "G1Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G1Point(uint256 X,uint256 Y)")
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
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G1Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.X)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.Y)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.X, out);
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.Y, out);
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
    struct G2Point { uint256[2] X; uint256[2] Y; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct G2Point {
        #[allow(missing_docs)]
        pub X: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
        #[allow(missing_docs)]
        pub Y: [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
            alloy::sol_types::sol_data::FixedArray<alloy::sol_types::sol_data::Uint<256>, 2usize>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
            [alloy::sol_types::private::primitives::aliases::U256; 2usize],
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
        impl ::core::convert::From<G2Point> for UnderlyingRustTuple<'_> {
            fn from(value: G2Point) -> Self {
                (value.X, value.Y)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for G2Point {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    X: tuple.0,
                    Y: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for G2Point {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for G2Point {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.X),
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::tokenize(&self.Y),
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
        impl alloy_sol_types::SolType for G2Point {
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
        impl alloy_sol_types::SolStruct for G2Point {
            const NAME: &'static str = "G2Point";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("G2Point(uint256[2] X,uint256[2] Y)")
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
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.X)
                    .0,
                    <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.Y)
                    .0,
                ]
                .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for G2Point {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.X
                    )
                    + <alloy::sol_types::sol_data::FixedArray<
                        alloy::sol_types::sol_data::Uint<256>,
                        2usize,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.Y
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.X, out
                );
                <alloy::sol_types::sol_data::FixedArray<
                    alloy::sol_types::sol_data::Uint<256>,
                    2usize,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.Y, out
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
    /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

    See the [wrapper's documentation](`BN254Instance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> BN254Instance<T, P, N> {
        BN254Instance::<T, P, N>::new(address, provider)
    }
    /**A [`BN254`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`BN254`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct BN254Instance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for BN254Instance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("BN254Instance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > BN254Instance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`BN254`](self) contract instance.

        See the [wrapper's documentation](`BN254Instance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> BN254Instance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> BN254Instance<T, P, N> {
            BN254Instance {
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
        > BN254Instance<T, P, N>
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
        > BN254Instance<T, P, N>
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
library IAwesomeVaultTaskManager {
    struct Task { TaskInput input; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    struct TaskInput { string key; string value; }
    struct TaskResponse { uint32 referenceTaskIndex; bytes32 result; }
    struct TaskResponseMetadata { uint32 taskRespondedBlock; bytes32 hashOfNonSigners; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IAwesomeVaultTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct Task { TaskInput input; uint32 taskCreatedBlock; bytes quorumNumbers; uint32 quorumThresholdPercentage; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Task {
        #[allow(missing_docs)]
        pub input: <TaskInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskCreatedBlock: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
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
            TaskInput,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Bytes,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <TaskInput as alloy::sol_types::SolType>::RustType,
            u32,
            alloy::sol_types::private::Bytes,
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
        impl ::core::convert::From<Task> for UnderlyingRustTuple<'_> {
            fn from(value: Task) -> Self {
                (
                    value.input,
                    value.taskCreatedBlock,
                    value.quorumNumbers,
                    value.quorumThresholdPercentage,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Task {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    input: tuple.0,
                    taskCreatedBlock: tuple.1,
                    quorumNumbers: tuple.2,
                    quorumThresholdPercentage: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Task {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Task {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <TaskInput as alloy_sol_types::SolType>::tokenize(&self.input),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.taskCreatedBlock,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
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
        impl alloy_sol_types::SolType for Task {
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
        impl alloy_sol_types::SolStruct for Task {
            const NAME: &'static str = "Task";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Task(TaskInput input,uint32 taskCreatedBlock,bytes quorumNumbers,uint32 quorumThresholdPercentage)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components.push(<TaskInput as alloy_sol_types::SolStruct>::eip712_root_type());
                components.extend(<TaskInput as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <TaskInput as alloy_sol_types::SolType>::eip712_data_word(
                            &self.input,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskCreatedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumNumbers,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumThresholdPercentage,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Task {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <TaskInput as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.input,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskCreatedBlock,
                    )
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumNumbers,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumThresholdPercentage,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <TaskInput as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.input, out);
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.taskCreatedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumNumbers,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumThresholdPercentage,
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
    struct TaskInput { string key; string value; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone, Debug)]
    pub struct TaskInput {
        #[allow(missing_docs)]
        pub key: alloy::sol_types::private::String,
        #[allow(missing_docs)]
        pub value: alloy::sol_types::private::String,
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
            alloy::sol_types::sol_data::String,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::String,
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
        impl ::core::convert::From<TaskInput> for UnderlyingRustTuple<'_> {
            fn from(value: TaskInput) -> Self {
                (value.key, value.value)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskInput {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    key: tuple.0,
                    value: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskInput {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskInput {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.key,
                    ),
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
        impl alloy_sol_types::SolType for TaskInput {
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
        impl alloy_sol_types::SolStruct for TaskInput {
            const NAME: &'static str = "TaskInput";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed("TaskInput(string key,string value)")
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
                            &self.key,
                        )
                        .0,
                    <alloy::sol_types::sol_data::String as alloy_sol_types::SolType>::eip712_data_word(
                            &self.value,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskInput {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.key,
                    )
                    + <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.value,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.key,
                    out,
                );
                <alloy::sol_types::sol_data::String as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.value,
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
    struct TaskResponse { uint32 referenceTaskIndex; bytes32 result; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponse {
        #[allow(missing_docs)]
        pub referenceTaskIndex: u32,
        #[allow(missing_docs)]
        pub result: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TaskResponse> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponse) -> Self {
                (value.referenceTaskIndex, value.result)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponse {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    referenceTaskIndex: tuple.0,
                    result: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponse {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponse {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceTaskIndex),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.result),
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
        impl alloy_sol_types::SolType for TaskResponse {
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
        impl alloy_sol_types::SolStruct for TaskResponse {
            const NAME: &'static str = "TaskResponse";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponse(uint32 referenceTaskIndex,bytes32 result)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.referenceTaskIndex,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.result)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponse {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.referenceTaskIndex,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.result,
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
                    &rust.referenceTaskIndex,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.result,
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
    struct TaskResponseMetadata { uint32 taskRespondedBlock; bytes32 hashOfNonSigners; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TaskResponseMetadata {
        #[allow(missing_docs)]
        pub taskRespondedBlock: u32,
        #[allow(missing_docs)]
        pub hashOfNonSigners: alloy::sol_types::private::FixedBytes<32>,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::FixedBytes<32>);
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TaskResponseMetadata> for UnderlyingRustTuple<'_> {
            fn from(value: TaskResponseMetadata) -> Self {
                (value.taskRespondedBlock, value.hashOfNonSigners)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TaskResponseMetadata {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    taskRespondedBlock: tuple.0,
                    hashOfNonSigners: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for TaskResponseMetadata {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for TaskResponseMetadata {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.taskRespondedBlock),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.hashOfNonSigners),
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
        impl alloy_sol_types::SolType for TaskResponseMetadata {
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
        impl alloy_sol_types::SolStruct for TaskResponseMetadata {
            const NAME: &'static str = "TaskResponseMetadata";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "TaskResponseMetadata(uint32 taskRespondedBlock,bytes32 hashOfNonSigners)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.taskRespondedBlock,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hashOfNonSigners,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for TaskResponseMetadata {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.taskRespondedBlock,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hashOfNonSigners,
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
                    &rust.taskRespondedBlock,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hashOfNonSigners,
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
    /**Creates a new wrapper around an on-chain [`IAwesomeVaultTaskManager`](self) contract instance.

    See the [wrapper's documentation](`IAwesomeVaultTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAwesomeVaultTaskManagerInstance<T, P, N> {
        IAwesomeVaultTaskManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAwesomeVaultTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IAwesomeVaultTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAwesomeVaultTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAwesomeVaultTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAwesomeVaultTaskManagerInstance")
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
        > IAwesomeVaultTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IAwesomeVaultTaskManager`](self) contract instance.

        See the [wrapper's documentation](`IAwesomeVaultTaskManagerInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IAwesomeVaultTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAwesomeVaultTaskManagerInstance<T, P, N> {
            IAwesomeVaultTaskManagerInstance {
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
        > IAwesomeVaultTaskManagerInstance<T, P, N>
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
        > IAwesomeVaultTaskManagerInstance<T, P, N>
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
library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod IBLSSignatureCheckerTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct NonSignerStakesAndSignature { uint32[] nonSignerQuorumBitmapIndices; BN254.G1Point[] nonSignerPubkeys; BN254.G1Point[] quorumApks; BN254.G2Point apkG2; BN254.G1Point sigma; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerStakesAndSignature {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerPubkeys:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub quorumApks:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            alloy::sol_types::sol_data::Array<BN254::G1Point>,
            BN254::G2Point,
            BN254::G1Point,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
            <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<NonSignerStakesAndSignature> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerStakesAndSignature) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.nonSignerPubkeys,
                    value.quorumApks,
                    value.apkG2,
                    value.sigma,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerStakesAndSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    nonSignerPubkeys: tuple.1,
                    quorumApks: tuple.2,
                    apkG2: tuple.3,
                    sigma: tuple.4,
                    quorumApkIndices: tuple.5,
                    totalStakeIndices: tuple.6,
                    nonSignerStakeIndices: tuple.7,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for NonSignerStakesAndSignature {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for NonSignerStakesAndSignature {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerPubkeys),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApks),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
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
        impl alloy_sol_types::SolType for NonSignerStakesAndSignature {
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
        impl alloy_sol_types::SolStruct for NonSignerStakesAndSignature {
            const NAME: &'static str = "NonSignerStakesAndSignature";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "NonSignerStakesAndSignature(uint32[] nonSignerQuorumBitmapIndices,BN254.G1Point[] nonSignerPubkeys,BN254.G1Point[] quorumApks,BN254.G2Point apkG2,BN254.G1Point sigma,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
                )
            }
            #[inline]
            fn eip712_components(
            ) -> alloy_sol_types::private::Vec<alloy_sol_types::private::Cow<'static, str>>
            {
                let mut components = alloy_sol_types::private::Vec::with_capacity(4);
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G2Point as alloy_sol_types::SolStruct>::eip712_components());
                components.push(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_root_type());
                components
                    .extend(<BN254::G1Point as alloy_sol_types::SolStruct>::eip712_components());
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerPubkeys,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.quorumApks)
                        .0,
                    <BN254::G2Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.apkG2,
                        )
                        .0,
                    <BN254::G1Point as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sigma,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for NonSignerStakesAndSignature {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerPubkeys,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApks,
                    )
                    + <BN254::G2Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.apkG2,
                    )
                    + <BN254::G1Point as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sigma,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerPubkeys,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    BN254::G1Point,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApks,
                    out,
                );
                <BN254::G2Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.apkG2,
                    out,
                );
                <BN254::G1Point as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sigma,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    struct QuorumStakeTotals { uint96[] signedStakeForQuorum; uint96[] totalStakeForQuorum; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct QuorumStakeTotals {
        #[allow(missing_docs)]
        pub signedStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        #[allow(missing_docs)]
        pub totalStakeForQuorum:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
        impl ::core::convert::From<QuorumStakeTotals> for UnderlyingRustTuple<'_> {
            fn from(value: QuorumStakeTotals) -> Self {
                (value.signedStakeForQuorum, value.totalStakeForQuorum)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for QuorumStakeTotals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signedStakeForQuorum: tuple.0,
                    totalStakeForQuorum: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for QuorumStakeTotals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for QuorumStakeTotals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.signedStakeForQuorum),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeForQuorum),
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
        impl alloy_sol_types::SolType for QuorumStakeTotals {
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
        impl alloy_sol_types::SolStruct for QuorumStakeTotals {
            const NAME: &'static str = "QuorumStakeTotals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "QuorumStakeTotals(uint96[] signedStakeForQuorum,uint96[] totalStakeForQuorum)",
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
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.signedStakeForQuorum,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeForQuorum,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for QuorumStakeTotals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signedStakeForQuorum,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeForQuorum,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.signedStakeForQuorum,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<96>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeForQuorum,
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
    /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

    See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
        IBLSSignatureCheckerTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IBLSSignatureCheckerTypes`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`IBLSSignatureCheckerTypes`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IBLSSignatureCheckerTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IBLSSignatureCheckerTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IBLSSignatureCheckerTypesInstance")
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IBLSSignatureCheckerTypes`](self) contract instance.

        See the [wrapper's documentation](`IBLSSignatureCheckerTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IBLSSignatureCheckerTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IBLSSignatureCheckerTypesInstance<T, P, N> {
            IBLSSignatureCheckerTypesInstance {
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
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
        > IBLSSignatureCheckerTypesInstance<T, P, N>
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
library OperatorStateRetriever {
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod OperatorStateRetriever {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
    struct CheckSignaturesIndices { uint32[] nonSignerQuorumBitmapIndices; uint32[] quorumApkIndices; uint32[] totalStakeIndices; uint32[][] nonSignerStakeIndices; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct CheckSignaturesIndices {
        #[allow(missing_docs)]
        pub nonSignerQuorumBitmapIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub quorumApkIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub totalStakeIndices: alloy::sol_types::private::Vec<u32>,
        #[allow(missing_docs)]
        pub nonSignerStakeIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            alloy::sol_types::sol_data::Array<
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            >,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<u32>,
            alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
        impl ::core::convert::From<CheckSignaturesIndices> for UnderlyingRustTuple<'_> {
            fn from(value: CheckSignaturesIndices) -> Self {
                (
                    value.nonSignerQuorumBitmapIndices,
                    value.quorumApkIndices,
                    value.totalStakeIndices,
                    value.nonSignerStakeIndices,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CheckSignaturesIndices {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    nonSignerQuorumBitmapIndices: tuple.0,
                    quorumApkIndices: tuple.1,
                    totalStakeIndices: tuple.2,
                    nonSignerStakeIndices: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for CheckSignaturesIndices {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for CheckSignaturesIndices {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerQuorumBitmapIndices,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumApkIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.totalStakeIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerStakeIndices),
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
        impl alloy_sol_types::SolType for CheckSignaturesIndices {
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
        impl alloy_sol_types::SolStruct for CheckSignaturesIndices {
            const NAME: &'static str = "CheckSignaturesIndices";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "CheckSignaturesIndices(uint32[] nonSignerQuorumBitmapIndices,uint32[] quorumApkIndices,uint32[] totalStakeIndices,uint32[][] nonSignerStakeIndices)",
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerQuorumBitmapIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.quorumApkIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.totalStakeIndices,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nonSignerStakeIndices,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for CheckSignaturesIndices {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerQuorumBitmapIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.quorumApkIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.totalStakeIndices,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nonSignerStakeIndices,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerQuorumBitmapIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.quorumApkIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Uint<32>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.totalStakeIndices,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nonSignerStakeIndices,
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
    struct Operator { address operator; bytes32 operatorId; uint96 stake; }
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct Operator {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub stake: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::FixedBytes<32>,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<Operator> for UnderlyingRustTuple<'_> {
            fn from(value: Operator) -> Self {
                (value.operator, value.operatorId, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for Operator {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    operator: tuple.0,
                    operatorId: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for Operator {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for Operator {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
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
        impl alloy_sol_types::SolType for Operator {
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
        impl alloy_sol_types::SolStruct for Operator {
            const NAME: &'static str = "Operator";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "Operator(address operator,bytes32 operatorId,uint96 stake)",
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorId)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.stake)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Operator {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operator,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorId,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.stake)
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
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorId,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.stake,
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
    /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

    See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> OperatorStateRetrieverInstance<T, P, N> {
        OperatorStateRetrieverInstance::<T, P, N>::new(address, provider)
    }
    /**A [`OperatorStateRetriever`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`OperatorStateRetriever`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct OperatorStateRetrieverInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for OperatorStateRetrieverInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("OperatorStateRetrieverInstance")
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
        > OperatorStateRetrieverInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`OperatorStateRetriever`](self) contract instance.

        See the [wrapper's documentation](`OperatorStateRetrieverInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> OperatorStateRetrieverInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> OperatorStateRetrieverInstance<T, P, N> {
            OperatorStateRetrieverInstance {
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
        > OperatorStateRetrieverInstance<T, P, N>
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
        > OperatorStateRetrieverInstance<T, P, N>
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
library BN254 {
    struct G1Point {
        uint256 X;
        uint256 Y;
    }
    struct G2Point {
        uint256[2] X;
        uint256[2] Y;
    }
}

library IAwesomeVaultTaskManager {
    struct Task {
        TaskInput input;
        uint32 taskCreatedBlock;
        bytes quorumNumbers;
        uint32 quorumThresholdPercentage;
    }
    struct TaskInput {
        string key;
        string value;
    }
    struct TaskResponse {
        uint32 referenceTaskIndex;
        bytes32 result;
    }
    struct TaskResponseMetadata {
        uint32 taskRespondedBlock;
        bytes32 hashOfNonSigners;
    }
}

library IBLSSignatureCheckerTypes {
    struct NonSignerStakesAndSignature {
        uint32[] nonSignerQuorumBitmapIndices;
        BN254.G1Point[] nonSignerPubkeys;
        BN254.G1Point[] quorumApks;
        BN254.G2Point apkG2;
        BN254.G1Point sigma;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct QuorumStakeTotals {
        uint96[] signedStakeForQuorum;
        uint96[] totalStakeForQuorum;
    }
}

library OperatorStateRetriever {
    struct CheckSignaturesIndices {
        uint32[] nonSignerQuorumBitmapIndices;
        uint32[] quorumApkIndices;
        uint32[] totalStakeIndices;
        uint32[][] nonSignerStakeIndices;
    }
    struct Operator {
        address operator;
        bytes32 operatorId;
        uint96 stake;
    }
}

interface AwesomeVaultTaskManager {
    error BitmapValueTooLarge();
    error BytesArrayLengthTooLong();
    error BytesArrayNotOrdered();
    error CurrentlyPaused();
    error ECAddFailed();
    error ECMulFailed();
    error ExpModFailed();
    error InputAddressZero();
    error InputArrayLengthMismatch();
    error InputEmptyQuorumNumbers();
    error InputNonSignerLengthMismatch();
    error InvalidBLSPairingKey();
    error InvalidBLSSignature();
    error InvalidNewPausedStatus();
    error InvalidQuorumApkHash();
    error InvalidReferenceBlocknumber();
    error NonSignerPubkeysNotSorted();
    error OnlyPauser();
    error OnlyRegistryCoordinatorOwner();
    error OnlyUnpauser();
    error OperatorNotRegistered();
    error ScalarTooLarge();
    error StaleStakesForbidden();

    event Initialized(uint8 version);
    event NewTaskCreated(uint32 indexed taskIndex, IAwesomeVaultTaskManager.Task task);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event StaleStakesForbiddenUpdate(bool value);
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    event TaskCompleted(uint32 indexed taskIndex);
    event TaskResponded(IAwesomeVaultTaskManager.TaskResponse taskResponse, IAwesomeVaultTaskManager.TaskResponseMetadata taskResponseMetadata);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);

    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    function WADS_TO_SLASH() external view returns (uint256);
    function aggregator() external view returns (address);
    function allTaskHashes(uint32) external view returns (bytes32);
    function allTaskResponses(uint32) external view returns (bytes32);
    function allocationManager() external view returns (address);
    function blsApkRegistry() external view returns (address);
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    function createNewTask(IAwesomeVaultTaskManager.TaskInput memory input, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    function delegation() external view returns (address);
    function generator() external view returns (address);
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    function getTaskResponseWindowBlock() external view returns (uint32);
    function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
    function instantSlasher() external view returns (address);
    function latestTaskNum() external view returns (uint32);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function raiseAndResolveChallenge(IAwesomeVaultTaskManager.Task memory task, IAwesomeVaultTaskManager.TaskResponse memory taskResponse, IAwesomeVaultTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    function registryCoordinator() external view returns (address);
    function renounceOwnership() external;
    function respondToTask(IAwesomeVaultTaskManager.Task memory task, IAwesomeVaultTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    function serviceManager() external view returns (address);
    function setStaleStakesForbidden(bool value) external;
    function stakeRegistry() external view returns (address);
    function staleStakesForbidden() external view returns (bool);
    function taskNumber() external view returns (uint32);
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    function transferOwnership(address newOwner) external;
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    function unpause(uint256 newPausedStatus) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "constructor",
    "inputs": [
      {
        "name": "_registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "_taskResponseWindowBlock",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "TASK_CHALLENGE_WINDOW_BLOCK",
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
    "name": "TASK_RESPONSE_WINDOW_BLOCK",
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
    "name": "WADS_TO_SLASH",
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
    "name": "aggregator",
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
    "name": "allTaskHashes",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "allTaskResponses",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "allocationManager",
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
    "name": "blsApkRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IBLSApkRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "checkSignatures",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.QuorumStakeTotals",
        "components": [
          {
            "name": "signedStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          },
          {
            "name": "totalStakeForQuorum",
            "type": "uint96[]",
            "internalType": "uint96[]"
          }
        ]
      },
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
    "name": "createNewTask",
    "inputs": [
      {
        "name": "input",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.TaskInput",
        "components": [
          {
            "name": "key",
            "type": "string",
            "internalType": "string"
          },
          {
            "name": "value",
            "type": "string",
            "internalType": "string"
          }
        ]
      },
      {
        "name": "quorumThresholdPercentage",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
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
    "name": "generator",
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
    "name": "getBatchOperatorFromId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getBatchOperatorId",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      }
    ],
    "outputs": [
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCheckSignaturesIndices",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "referenceBlockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      },
      {
        "name": "nonSignerOperatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorStateRetriever.CheckSignaturesIndices",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
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
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getOperatorState",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "",
        "type": "tuple[][]",
        "internalType": "struct OperatorStateRetriever.Operator[][]",
        "components": [
          {
            "name": "operator",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorId",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "stake",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getQuorumBitmapsAtBlockNumber",
    "inputs": [
      {
        "name": "registryCoordinator",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      },
      {
        "name": "operatorIds",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
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
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTaskResponseWindowBlock",
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
    "name": "initialize",
    "inputs": [
      {
        "name": "initialOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_aggregator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_generator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_allocationManager",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_slasher",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "instantSlasher",
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
    "name": "latestTaskNum",
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
    "name": "raiseAndResolveChallenge",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IAwesomeVaultTaskManager.TaskInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskRespondedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "pubkeysOfNonSigningOperators",
        "type": "tuple[]",
        "internalType": "struct BN254.G1Point[]",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registryCoordinator",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract ISlashingRegistryCoordinator"
      }
    ],
    "stateMutability": "view"
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
    "name": "respondToTask",
    "inputs": [
      {
        "name": "task",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IAwesomeVaultTaskManager.TaskInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "taskResponse",
        "type": "tuple",
        "internalType": "struct IAwesomeVaultTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "nonSignerStakesAndSignature",
        "type": "tuple",
        "internalType": "struct IBLSSignatureCheckerTypes.NonSignerStakesAndSignature",
        "components": [
          {
            "name": "nonSignerQuorumBitmapIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerPubkeys",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApks",
            "type": "tuple[]",
            "internalType": "struct BN254.G1Point[]",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "apkG2",
            "type": "tuple",
            "internalType": "struct BN254.G2Point",
            "components": [
              {
                "name": "X",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              },
              {
                "name": "Y",
                "type": "uint256[2]",
                "internalType": "uint256[2]"
              }
            ]
          },
          {
            "name": "sigma",
            "type": "tuple",
            "internalType": "struct BN254.G1Point",
            "components": [
              {
                "name": "X",
                "type": "uint256",
                "internalType": "uint256"
              },
              {
                "name": "Y",
                "type": "uint256",
                "internalType": "uint256"
              }
            ]
          },
          {
            "name": "quorumApkIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "totalStakeIndices",
            "type": "uint32[]",
            "internalType": "uint32[]"
          },
          {
            "name": "nonSignerStakeIndices",
            "type": "uint32[][]",
            "internalType": "uint32[][]"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "serviceManager",
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
    "name": "setStaleStakesForbidden",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "stakeRegistry",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStakeRegistry"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "staleStakesForbidden",
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
    "name": "taskNumber",
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
    "name": "taskSuccesfullyChallenged",
    "inputs": [
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "trySignatureAndApkVerification",
    "inputs": [
      {
        "name": "msgHash",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "apk",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      },
      {
        "name": "apkG2",
        "type": "tuple",
        "internalType": "struct BN254.G2Point",
        "components": [
          {
            "name": "X",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          },
          {
            "name": "Y",
            "type": "uint256[2]",
            "internalType": "uint256[2]"
          }
        ]
      },
      {
        "name": "sigma",
        "type": "tuple",
        "internalType": "struct BN254.G1Point",
        "components": [
          {
            "name": "X",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "Y",
            "type": "uint256",
            "internalType": "uint256"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "pairingSuccessful",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "siganatureIsValid",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
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
    "name": "NewTaskCreated",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "task",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IAwesomeVaultTaskManager.Task",
        "components": [
          {
            "name": "input",
            "type": "tuple",
            "internalType": "struct IAwesomeVaultTaskManager.TaskInput",
            "components": [
              {
                "name": "key",
                "type": "string",
                "internalType": "string"
              },
              {
                "name": "value",
                "type": "string",
                "internalType": "string"
              }
            ]
          },
          {
            "name": "taskCreatedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "quorumNumbers",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "quorumThresholdPercentage",
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
    "name": "StaleStakesForbiddenUpdate",
    "inputs": [
      {
        "name": "value",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedSuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskChallengedUnsuccessfully",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      },
      {
        "name": "challenger",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskCompleted",
    "inputs": [
      {
        "name": "taskIndex",
        "type": "uint32",
        "indexed": true,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "TaskResponded",
    "inputs": [
      {
        "name": "taskResponse",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IAwesomeVaultTaskManager.TaskResponse",
        "components": [
          {
            "name": "referenceTaskIndex",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "result",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "taskResponseMetadata",
        "type": "tuple",
        "indexed": false,
        "internalType": "struct IAwesomeVaultTaskManager.TaskResponseMetadata",
        "components": [
          {
            "name": "taskRespondedBlock",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "hashOfNonSigners",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
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
    "name": "BitmapValueTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayLengthTooLong",
    "inputs": []
  },
  {
    "type": "error",
    "name": "BytesArrayNotOrdered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrentlyPaused",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECAddFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ECMulFailed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ExpModFailed",
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
    "name": "InputEmptyQuorumNumbers",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InputNonSignerLengthMismatch",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSPairingKey",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBLSSignature",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidNewPausedStatus",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidQuorumApkHash",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidReferenceBlocknumber",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonSignerPubkeysNotSorted",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyPauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyRegistryCoordinatorOwner",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OnlyUnpauser",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegistered",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ScalarTooLarge",
    "inputs": []
  },
  {
    "type": "error",
    "name": "StaleStakesForbidden",
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
pub mod AwesomeVaultTaskManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x61014080604052346101e6576060816151ed80380380916100208285610291565b8339810103126101e65780516001600160a01b038116908181036101e65760208301516001600160a01b038116938482036101e657604001519363ffffffff851685036101e657156102825760805260a052604051636830483560e01b8152602081600481855afa9081156101f2575f9161023f575b5060c052604051632efa2ca360e11b815290602090829060049082905afa9081156101f2575f916101fd575b5060e05260c05160405163df5cf72360e01b815290602090829060049082906001600160a01b03165afa9081156101f2575f916101ac575b506101005261012052604051614f2490816102c982396080518181816102ca01528181610d87015281816115090152612426015260a05181818161097c015281816110d90152818161182e01528181613526015281816135fc0152613b8e015260c051818181611095015281816138fe0152613a49015260e05181818161105101528181611752015261383a01526101005181818161218d015261370f0152610120518181816105ae0152611d2d0152f35b90506020813d6020116101ea575b816101c760209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100fa565b5f80fd5b3d91506101ba565b6040513d5f823e3d90fd5b90506020813d602011610237575b8161021860209383610291565b810103126101e657516001600160a01b03811681036101e6575f6100c2565b3d915061020b565b90506020813d60201161027a575b8161025a60209383610291565b810103126101e657516001600160a01b03811681036101e6576004610096565b3d915061024d565b6339b190bb60e11b5f5260045ffd5b601f909101601f19168101906001600160401b038211908210176102b457604052565b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461029a578063171f1d5b146102955780631ad43189146101e6578063245a7bfc146102905780632cb223d51461028b5780632d89f6fc1461028657806331b36bd9146102815780633563b0d11461027c5780633998fdd314610277578063416c7e5e146102725780634d2b57fe1461026d5780634f739f7414610268578063595c6a67146102635780635a2d7f021461025e5780635ac86ab7146102595780635c155662146102545780635c975abb1461024f5780635decc3f51461024a5780635df459461461024557806368304835146102405780636d14a9871461023b5780636efb463614610236578063715018a61461023157806372d18e8d146102225780637afa1eed1461022c578063886f1195146102275780638b00ce7c146102225780638da5cb5b1461021d57806392a51824146102185780639b290e9814610213578063b70239491461020e578063b98d090814610209578063ca8aa7c714610204578063cc2a9a5b146101ff578063cefdc1d4146101fa578063df5cf723146101f5578063ef029dbc146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b6123fd565b6123e2565b610592565b612351565b6121bc565b612178565b612034565b611ef4565b611ecc565b611eaa565b611c10565b611be8565b61158d565b611538565b6114a9565b6114f4565b6114cc565b61144e565b6113a1565b6110c4565b611080565b61103c565b610ffe565b610fe1565b610e68565b610dfc565b610dcf565b610d5c565b610cb5565b610aaa565b61094a565b610918565b61089e565b6106f4565b61064c565b610613565b6105d2565b610520565b3461035a57602036600319011261035a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103555761032492610310915f91610326575b506124ff565b61031f60665482811614612515565b61467d565b005b610348915060203d60201161034e575b61034081836103ad565b8101906124df565b5f61030a565b503d610336565b6124f4565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761038d57604052565b61035e565b608081019081106001600160401b0382111761038d57604052565b90601f801991011681019081106001600160401b0382111761038d57604052565b604051906103de610100836103ad565b565b604051906103de6040836103ad565b604051906103de6060836103ad565b604051906103de60a0836103ad565b906103de60405192836103ad565b60409060e319011261035a576040519061043482610372565b60e4358252610104356020830152565b919082604091031261035a5760405161045c81610372565b6020808294803584520135910152565b9080601f8301121561035a57604051916104876040846103ad565b82906040810192831161035a57905b8282106104a35750505090565b8135815260209182019101610496565b90608060631983011261035a576040516104cc81610372565b60206104e782946104de81606461046c565b845260a461046c565b910152565b919060808382031261035a5760206104e76040519261050a84610372565b60408496610518838261046c565b86520161046c565b3461035a5761012036600319011261035a57600435604036602319011261035a57610578604091825161055281610372565b60243581526044356020820152610568366104b3565b906105723661041b565b92612569565b8251911515825215156020820152f35b5f91031261035a57565b3461035a575f36600319011261035a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461035a575f36600319011261035a5760cd546040516001600160a01b039091168152602090f35b63ffffffff81160361035a57565b35906103de826105fa565b3461035a57602036600319011261035a5763ffffffff600435610635816105fa565b165f5260cb602052602060405f2054604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561066e816105fa565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361035a57565b6001600160401b03811161038d5760051b60200190565b90602080835192838152019201905f5b8181106106ca5750505090565b82518452602093840193909201916001016106bd565b9060206106f19281815201906106ad565b90565b3461035a57604036600319011261035a5760043561071181610685565b602435906001600160401b03821161035a573660238301121561035a5781600401359161073d83610696565b9261074b60405194856103ad565b8084526024602085019160051b8301019136831161035a57602401905b82821061078c5761078861077c86866126d3565b604051918291826106e0565b0390f35b60208091833561079b81610685565b815201910190610768565b6001600160401b03811161038d57601f01601f191660200190565b9291926107cd826107a6565b916107db60405193846103ad565b82948184528183011161035a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108625750505060208060019297019301930191939290610813565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610843565b3461035a57606036600319011261035a576004356108bb81610685565b6024356001600160401b03811161035a573660238201121561035a57610788916108f26109049236906024816004013591016107c1565b604435916108ff836105fa565b612904565b6040519182916020835260208301906107f7565b3461035a575f36600319011261035a5760d1546040516001600160a01b039091168152602090f35b8015150361035a57565b3461035a57602036600319011261035a5760043561096781610940565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916109dc575b506001600160a01b031633036109cd5761032490614b10565b637070f3b160e11b5f5260045ffd5b6109fe915060203d602011610a04575b6109f681836103ad565b81019061278b565b5f6109b4565b503d6109ec565b9080601f8301121561035a578135610a2281610696565b92610a3060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610a585750505090565b8135815260209182019101610a4b565b60206040818301928281528451809452019201905f5b818110610a8b5750505090565b82516001600160a01b0316845260209384019390920191600101610a7e565b3461035a57604036600319011261035a57600435610ac781610685565b6024356001600160401b03811161035a57610ae6903690600401610a0b565b610af08151612671565b916001600160a01b03165f5b8251811015610b8d57806020610b15610b3593866126b0565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561035557600192610b69915f91610b6f575b50610b5a83886126b0565b6001600160a01b039091169052565b01610afc565b610b87915060203d8111610a04576109f681836103ad565b5f610b4f565b604051806107888682610a68565b9181601f8401121561035a578235916001600160401b03831161035a576020838186019501011161035a57565b90602080835192838152019201905f5b818110610be55750505090565b825163ffffffff16845260209384019390920191600101610bd8565b90602082526060610c4f610c3a610c2484516080602088015260a0870190610bc8565b6020850151868203601f19016040880152610bc8565b6040840151858203601f190184870152610bc8565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610c8857505050505090565b9091929394602080610ca6600193601f198682030187528951610bc8565b97019301930191939290610c79565b3461035a57608036600319011261035a57600435610cd281610685565b60243590610cdf826105fa565b6044356001600160401b03811161035a57610cfe903690600401610b9b565b91606435926001600160401b03841161035a573660238501121561035a578360040135926001600160401b03841161035a573660248560051b8701011161035a57610788956024610d50960193612e03565b60405191829182610c01565b3461035a575f36600319011261035a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557610dc7915f9161032657506124ff565b610324614649565b3461035a575f36600319011261035a57602060405167016345785d8a00008152f35b60ff81160361035a57565b3461035a57602036600319011261035a576020600160ff600435610e1f81610df1565b161b806066541614604051908152f35b60206040818301928281528451809452019201905f5b818110610e525750505090565b8251845260209384019390920191600101610e45565b3461035a57606036600319011261035a57600435610e8581610685565b6024356001600160401b03811161035a57610ea4903690600401610a0b565b60443591610eb1836105fa565b6040516361c8a12f60e11b8152906001600160a01b03165f8280610ed9868860048401613257565b0381845afa918215610355575f92610fbd575b50610ef78351612671565b935f5b8451811015610faf57610f0d81866126b0565b5190602083610f29610f1f84896126b0565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610355576001925f91610f81575b50828060c01b0316610f7a82896126b0565b5201610efa565b610fa2915060203d8111610fa8575b610f9a81836103ad565b810190612d7a565b5f610f68565b503d610f90565b604051806107888882610e2f565b610fda9192503d805f833e610fd281836103ad565b810190612c49565b905f610eec565b3461035a575f36600319011261035a576020606654604051908152f35b3461035a57602036600319011261035a5763ffffffff600435611020816105fa565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f8301121561035a57813561111f81610696565b9261112d60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106111555750505090565b602080918335611164816105fa565b815201910190611148565b81601f8201121561035a57803561118581610696565b9261119360405194856103ad565b81845260208085019260061b8401019281841161035a57602001915b8383106111bd575050505090565b60206040916111cc8486610444565b8152019201916111af565b9080601f8301121561035a5781356111ee81610696565b926111fc60405194856103ad565b81845260208085019260051b8201019183831161035a5760208201905b83821061122857505050505090565b81356001600160401b03811161035a5760209161124a87848094880101611108565b815201910190611219565b9190916101808184031261035a5761126b6103ce565b9281356001600160401b03811161035a5781611288918401611108565b845260208201356001600160401b03811161035a57816112a991840161116f565b602085015260408201356001600160401b03811161035a57816112cd91840161116f565b60408501526112df81606084016104ec565b60608501526112f18160e08401610444565b60808501526101208201356001600160401b03811161035a5781611316918401611108565b60a08501526101408201356001600160401b03811161035a578161133b918401611108565b60c08501526101608201356001600160401b03811161035a5761135e92016111d7565b60e0830152565b90602080835192838152019201905f5b8181106113825750505090565b82516001600160601b0316845260209384019390920191600101611375565b3461035a57608036600319011261035a576004356024356001600160401b03811161035a576113d4903690600401610b9b565b90916044356113e2816105fa565b606435926001600160401b03841161035a576114449461140961140f953690600401611255565b93613449565b60405192839260408452602061143082516040808801526080870190611365565b910151848203603f19016060860152611365565b9060208301520390f35b3461035a575f36600319011261035a57611466614cec565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461035a575f36600319011261035a57602063ffffffff60c95416604051908152f35b3461035a575f36600319011261035a5760ce546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576033546040516001600160a01b039091168152602090f35b9081608091031261035a5790565b604090602319011261035a57602490565b9081604091031261035a5790565b3461035a5760c036600319011261035a576004356001600160401b03811161035a576115bd903690600401611560565b6115c63661156e565b604036606319011261035a5760a4356001600160401b03811161035a576115f190369060040161116f565b6116696115fd83613cd3565b926116216116198563ffffffff165f5260cb60205260405f2090565b541515613cdd565b6116398463ffffffff165f5260cb60205260405f2090565b5490604051611660816116526064602083019586613d51565b03601f1981018352826103ad565b51902014613d6f565b61169461168e6116878463ffffffff165f5260cc60205260405f2090565b5460ff1690565b15613de1565b6116c36116b56116ac6116a76064613cd3565b613e77565b63ffffffff1690565b63ffffffff43161115613ea9565b6116cd8151612671565b925f5b825181101561170d57806116fc6116e9600193866126b0565b5180515f526020015160205260405f2090565b61170682886126b0565b52016116d0565b5092919091602084019161174561172384613cd3565b6040516117398161165287602083019586613409565b51902060843514613f1b565b61174f8451612671565b947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b86518110156117ff578060206117966117b693886126b0565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818a5afa918215610355576001926117db915f916117e1575b50610b5a838c6126b0565b0161177d565b6117f9915060203d8111610a04576109f681836103ad565b5f6117d0565b508691859161185b61182461182c604085019361181c8587613fa6565b939091613cd3565b9236916107c1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612904565b935f905b8551821015611b91575f969596945b61187883896126b0565b5151861015611b81578760208061189c89611896886118be9e6126b0565b516126b0565b510151604051809b81926308f6629d60e31b8352600483019190602083019252565b0381855afa988915610355575f99611b61575b506001975f5b8451811015611b52576119096118fd6118f083886126b0565b516001600160a01b031690565b6001600160a01b031690565b6001600160a01b038c1614611920576001016118d7565b509890975060015f9792975b151514611941575b506001019496959661186e565b97959094969184925f896119ba60ff61199261198c61197e8b611978611a0f9e61197260d15460018060a01b031690565b98613fa6565b90612daf565b356001600160f81b03191690565b60f81c90565b6119ac61199d6103e0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d0546119d1906118fd906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529097889190829081906044820190565b03915afa958615610355575f96611b2e575b50611a2c8651612671565b975f5b8951811015611a55578067016345785d8a0000611a4e6001938d6126b0565b5201611a2f565b50999297909896919395611aa060ff611a7861198c61197e8a6119788d8f613fa6565b611a92611a836103fe565b6001600160a01b039098168852565b1663ffffffff166020860152565b60408401526060830152611ab261405b565b608083015260cf54611ace906118fd906001600160a01b031681565b803b1561035a57604051636a669b4160e01b8152925f918491829084908290611afa90600483016140af565b03925af191821561035557600192611b14575b5090611934565b80611b225f611b28936103ad565b80610588565b89611b0d565b611b4b9196503d805f833e611b4381836103ad565b810190613fd8565b948a611a21565b5098909760019097929761192c565b611b7a91995060203d8111610a04576109f681836103ad565b97896118d1565b959695945060019091019061185f565b86611bba611bad8263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3005b3461035a575f36600319011261035a5760cf546040516001600160a01b039091168152602090f35b3461035a57608036600319011261035a576004356001600160401b03811161035a57611c40903690600401611560565b611c493661156e565b906064356001600160401b03811161035a57611c69903690600401611255565b60cd549092906001600160a01b03163303611e6557611c8c602083949301613cd3565b91611d78611c9d6040860186613fa6565b929094611cef611caf60608901613cd3565b97604051611cc581611652602082019485614184565b519020611ce8611cd488613cd3565b63ffffffff165f5260ca60205260405f2090565b541461423c565b611d19611d12611cfe87613cd3565b63ffffffff165f5260cb60205260405f2090565b54156142ae565b8363ffffffff431696611d5a611d526116ac7f000000000000000000000000000000000000000000000000000000000000000086613e8f565b89111561430f565b6040516020810190611d70816116528b85614371565b519020613449565b919060ff5f9616955b828110611e03577ff2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58868686611dc3611db76103e0565b63ffffffff9094168452565b60208301526040516020810190611ddf81611652868686614441565b519020611dee611cfe83613cd3565b55611dfe60405192839283614441565b0390a1005b80611e5f611e3b611e36611e2a611e1d60019688516126b0565b516001600160601b031690565b6001600160601b031690565b614381565b611e58611e2a8b611e53611e1d8760208b01516126b0565b6143ad565b11156143d0565b01611d81565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b3461035a575f36600319011261035a57602060ff609754166040519015158152f35b3461035a575f36600319011261035a5760d0546040516001600160a01b039091168152602090f35b3461035a5760c036600319011261035a57600435611f1181610685565b611f91602435611f2081610685565b604435611f2c81610685565b606435611f3881610685565b60843591611f4583610685565b60a43593611f5285610685565b5f5496611f7760ff60088a901c16158099819a61200f575b8115611fef575b5061446b565b87611f88600160ff195f5416175f55565b611fd8576144ce565b611f9757005b611fa561ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498908060208101611dfe565b611fea61010061ff00195f5416175f55565b6144ce565b303b15915081612001575b505f611f71565b60ff1660011490505f611ffa565b600160ff8216109150611f6a565b6040906106f19392815281602082015201906107f7565b3461035a57606036600319011261035a5760043561205181610685565b602435604435612060816105fa565b6120a161206b61264f565b9280612076856126a3565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401613257565b0381875afa93841561035557836120cb6116ac610f1f612100986020975f9161215e575b506126a3565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa80156103555761212f925f9161213f575b506001600160c01b03169261212984614d8c565b90612904565b906107886040519283928361201d565b612158915060203d602011610fa857610f9a81836103ad565b5f612115565b61217291503d805f833e610fd281836103ad565b5f6120c5565b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a57606036600319011261035a576004356001600160401b03811161035a576121ec90369060040161157f565b602435906121f9826105fa565b6044356001600160401b03811161035a57612218903690600401610b9b565b60ce5491939092916001600160a01b0316330361230257610324936122ec9361226c61227393612251612249614537565b96369061457d565b86524363ffffffff16602087015263ffffffff166060860152565b36916107c1565b6040820152604051602081019061228e8161165285856145d7565b5190206122a3611cd460c95463ffffffff1690565b5560c95463ffffffff16907fa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd604051806122e463ffffffff861694826145d7565b0390a2613e5f565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461035a57602036600319011261035a5760043561236e81610685565b612376614cec565b6001600160a01b0381161561238e5761032490614d44565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461035a575f36600319011261035a57602060405160648152f35b3461035a57602036600319011261035a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916124c0575b506001600160a01b031633036124b15761247f606654198219811614612515565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6124d9915060203d602011610a04576109f681836103ad565b5f61245e565b9081602091031261035a57516106f181610940565b6040513d5f823e3d90fd5b1561250657565b631d77d47760e21b5f5260045ffd5b1561251c57565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156125505760051b0190565b61252b565b634e487b7160e01b5f52601260045260245ffd5b61264561262261264b9561261c61261585875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526125ec81610120840103601f1981018352826103ad565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966146f3565b90614739565b9261261c61263761263161479b565b94614892565b916126406149ae565b6146f3565b916149e2565b9091565b6040805190919061266083826103ad565b6001815291601f1901366020840137565b9061267b82610696565b61268860405191826103ad565b8281528092612699601f1991610696565b0190602036910137565b8051156125505760200190565b80518210156125505760209160051b010190565b9081602091031261035a575190565b9190916126e08351612671565b925f5b8151811015612786578060206126ff6118f061272894866126b0565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610355576001925f91612758575b5061275182886126b0565b52016126e3565b612779915060203d811161277f575b61277181836103ad565b8101906126c4565b5f612746565b503d612767565b505050565b9081602091031261035a57516106f181610685565b906127aa82610696565b6127b760405191826103ad565b828152602081936127ca601f1991610696565b0191015f5b8281106127db57505050565b6060828201526020016127cf565b908151811015612550570160200190565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161282d81610696565b9261283b60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106128635750505090565b8151815260209182019101612856565b9061287d82610696565b61288a60405191826103ad565b828152809261289b601f1991610696565b015f5b8181106128aa57505050565b6040519060608201918083106001600160401b0384111761038d576020926040525f81525f838201525f60408201528282860101520161289e565b9081602091031261035a57516001600160601b038116810361035a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610355575f95612c04575b50604051634f4c91e160e11b815294602086600481855afa918215610355576004965f93612be2575b5060209060405197888092632efa2ca360e11b82525afa958615610355575f96612bc1575b5061299285939295516127a0565b945f935b8051851015612bb7576129bd61198c6129af87846127e9565b516001600160f81b03191690565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610355575f94612b93575b50612a0d8451612873565b612a17888b6126b0565b52612a22878a6126b0565b505f5b8451811015612b8257806020612a3e612a6093886126b0565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610355575f92612b62575b50612a8681876126b0565b518a60208a612a95858b6126b0565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa93841561035557612b198c8f612b14600198612b2b9789975f92612b32575b50612aff611a836103ef565b60208701526001600160601b03166040860152565b6126b0565b5190612b2583836126b0565b526126b0565b5001612a25565b612b5491925060203d8111612b5b575b612b4c81836103ad565b8101906128e5565b905f612af3565b503d612b42565b612b7b91925060203d8111610a04576109f681836103ad565b905f612a7b565b506001909601959094509150612996565b612bb09194503d805f833e612ba881836103ad565b8101906127fa565b925f612a02565b5050509350505090565b612bdb91965060203d602011610a04576109f681836103ad565b945f612984565b6020919350612bfd90823d8411610a04576109f681836103ad565b929061295f565b612c1e91955060203d602011610a04576109f681836103ad565b935f612936565b60405190612c3282610392565b606080838181528160208201528160408201520152565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a578151612c7c81610696565b92612c8a60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210612cb25750505090565b602080918351612cc1816105fa565b815201910190612ca5565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161035a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f195931681528160208201520191612d03565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114612d655760010190565b612d40565b91908110156125505760051b0190565b9081602091031261035a57516001600160c01b038116810361035a5790565b15612da057565b6325ec6c1f60e01b5f5260045ffd5b90821015612550570190565b9081602091031261035a57516106f1816105fa565b5f198114612d655760010190565b91612dfc60209263ffffffff92969596604086526040860191612d03565b9416910152565b9593949592909192612e13612c25565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610355575f95613236575b50612e50612c25565b946040516361c8a12f60e11b81525f8180612e708d8d8b60048501612ccc565b0381885afa908115610355575f9161321c575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612eb385878b60048501612d23565b0381875afa908115610355575f91613202575b506040870152612ed5816127a0565b9860608701998a525f5b60ff81168381101561314d57885f612f08838f612efb88612671565b905190612b2583836126b0565b505f8a868f5b818410612f85575050505090508c612f2582612671565b915f5b818110612f5257505091612f4791612f4d94935190612b2583836126b0565b50612d54565b612edf565b80612f7f612f6a610f1f6001946118968a89516126b0565b612f7483886126b0565b9063ffffffff169052565b01612f28565b610f1f84612f9a8160209695612fa295612d6a565b3597516126b0565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561035557888f888a918f94613039600161302c81938d809d5f92613121575b5061198c61197e6130169261300f878060c01b0386161515612d99565b8b8d612daf565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14613055575b5050505050600191925001908a918a868f612f0e565b859761307793613070602097999861198c9561197e95612d6a565b3595612daf565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610355578f6130d5906130da9383886001975f936130e9575b5061189690612f749394516126b0565b612dd0565b905082918a888f888a9161303f565b612f749350906131126118969260203d811161311a575b61310a81836103ad565b810190612dbb565b9350906130c5565b503d613100565b61301691925061197e61314461198c9260203d8111610fa857610f9a81836103ad565b93925050612ff2565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa908115610355576131a3945f9485936131e1575b5060405163354952a360e21b81529586948593849360048501612dde565b03916001600160a01b03165afa908115610355575f916131c7575b50602082015290565b6131db91503d805f833e610fd281836103ad565b5f6131be565b6131fb91935060203d602011610a04576109f681836103ad565b915f613185565b61321691503d805f833e610fd281836103ad565b5f612ec6565b61323091503d805f833e610fd281836103ad565b5f612e83565b61325091955060203d602011610a04576109f681836103ad565b935f612e47565b60409063ffffffff6106f1949316815281602082015201906106ad565b6040519061328182610372565b60606020838281520152565b1561329457565b62f8202d60e51b5f5260045ffd5b156132a957565b6343714afd60e01b5f5260045ffd5b156132bf57565b635f832f4160e01b5f5260045ffd5b156132d557565b634b874f4560e01b5f5260045ffd5b9081602091031261035a57516106f181610df1565b5f19810191908211612d6557565b1561330e57565b633fdc650560e21b5f5260045ffd5b9060018201809211612d6557565b9060028201809211612d6557565b9060038201809211612d6557565b9060048201809211612d6557565b9060058201809211612d6557565b91908201809211612d6557565b1561337757565b63affc5edb60e01b5f5260045ffd5b9081602091031261035a575167ffffffffffffffff198116810361035a5790565b156133ae57565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b038211612d6557565b156133e457565b6367988d3360e01b5f5260045ffd5b156133fa57565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106134335750505090565b8251845260209384019390920191600101613426565b949392909193613457613274565b5061346385151561328d565b604084015151851480613cc5575b80613cb7575b80613ca9575b613486906132a2565b613498602085015151855151146132b8565b6134af63ffffffff431663ffffffff8416106132ce565b6134b76103e0565b5f81525f6020820152926134c9613274565b6134d287612671565b60208201526134e087612671565b81526134ea613274565b926134f9602088015151612671565b8452613509602088015151612671565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557613572915f91613c7a575b5061356d368b876107c1565b614b4e565b985f965b602089015180518910156136d1576020886135c6610f1f8c6135be8f96868e6135a36116e98680956126b0565b6135b084848401516126b0565b528261369e575b01516126b0565b5195516126b0565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156103555761261c8a6136738f61366c8f8460208f926136639361365b8460019e6136799e5f91613681575b508f8060c01b031692516126b0565b5201516126b0565b51938d516126b0565b5116614b79565b90614baa565b970196613576565b6136989150863d8111610fa857610f9a81836103ad565b5f61364c565b6136cc6136ae84848401516126b0565b516136c5848401516136bf876132f9565b906126b0565b5110613307565b6135b7565b509095979496506136e6919893929950614c67565b916136f360975460ff1690565b908115613c72576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91613c53575b5091905b5f925b8184106137a45750505050509261378b61378661377f61379e95856116529860806060602099015192015192612569565b91906133dd565b6133f3565b0151604051928391602083019586613409565b51902090565b92989596909399919794878b888c888d613b4d575b610f1f8260a06137f961198c61197e84613801976137f36137e56116e98f9c604060209f9e01516126b0565b67ffffffffffffffff191690565b9b612daf565b9701516126b0565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610355576138c5610f1f8f958f906138bd8f978f96848f6138b760c0966138b0848f60209f906135b761197e9960409361198c9c5f91613b1f575b5067ffffffffffffffff199182169116146133a7565b5190614739565b9c612daf565b9601516126b0565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561035557613952918c8f925f92613afb575b506020613944929301516126b0565b906001600160601b03169052565b6139728c6139448c61396b611e1d8260208601516126b0565b92516126b0565b5f985f5b60208a015151811015613ae2578b8d6139b4896139a761198c61197e868f8961399f91516126b0565b519487612daf565b60ff161c60019081161490565b6139c3575b5050600101613976565b8a8a613a45859f948f96866118968f9360e06139fc610f1f9560206139f461198c61197e839f613a059c8991612daf565b9a01516126b0565b519b01516126b0565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355578f613ab1908f936001959486955f92613abc575b50613aab61394492935193613aa6611e1d84876126b0565b6133bd565b926126b0565b019a90508b8d6139b9565b6139449250613adb613aab9160203d8111612b5b57612b4c81836103ad565b9250613a8e565b5093919796996001919699509a94929a0192919061374e565b6139449250613b18602091823d8111612b5b57612b4c81836103ad565b9250613935565b6020613b4092503d8111613b46575b613b3881836103ad565b810190613386565b5f61389a565b503d613b2e565b613b8a9450613b67925061198c9161197e91602095612daf565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610355576020896138018f938f60a08f9761198c61197e8f8f906137f36137e56116e98f60408b96918f8893610f1f9f613c0e90613c14936137f99f5f92613c2a575b5063ffffffff809116931690613363565b11613370565b50505050505097505050505050929350506137b9565b602063ffffffff9293508291613c4b913d811161277f5761277181836103ad565b929150613bfd565b613c6c915060203d60201161311a5761310a81836103ad565b5f613747565b5f919061374b565b613c9c915060203d602011613ca2575b613c9481836103ad565b8101906132e4565b5f613561565b503d613c8a565b5060e084015151851461347d565b5060c0840151518514613477565b5060a0840151518514613471565b356106f1816105fa565b15613ce457565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613d47816105fa565b1684520135910152565b9160406103de929493613d68816080810197613d33565b0190613d33565b15613d7657565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15613de857565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b63ffffffff60019116019063ffffffff8211612d6557565b63ffffffff60649116019063ffffffff8211612d6557565b9063ffffffff8091169116019063ffffffff8211612d6557565b15613eb057565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b15613f2257565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e198136030182121561035a57018035906001600160401b03821161035a5760200191813603831361035a57565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161400b81610696565b9261401960405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106140415750505090565b60208091835161405081610685565b815201910190614034565b6040519061406a6040836103ad565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b81811061413457505050608061411f6106f194956060850151601f1985830301848601526106ad565b9201519060a0601f198285030191015261408b565b82516001600160a01b03168752602096870196909201916001016140f6565b9035601e198236030181121561035a5701602081359101916001600160401b03821161035a57813603831361035a57565b60208152813590603e198336030182121561035a576080614231606061422a6141f0876106f197018560208801526141de6141d36141c28380614153565b604060a08c015260e08b0191612d03565b916020810190614153565b888303609f190160c08a015290612d03565b61420c6141ff60208a01610608565b63ffffffff166040880152565b6142196040890189614153565b878303601f19018589015290612d03565b9501610608565b63ffffffff16910152565b1561424357565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156142b557565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561431657565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916103de9190613d33565b90606482029180830460641490151715612d6557565b90600682029180830460061490151715612d6557565b906001600160601b03809116911602906001600160601b038216918203612d6557565b156143d757565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091614457846080810197613d33565b63ffffffff81511660408501520151910152565b1561447257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6144d790614d44565b60cd80546001600160a01b03199081166001600160a01b039384161790915560ce805482169383169390931790925560d0805483169382169390931790925560cf805482169383169390931790925560d180549092169216919091179055565b6040519061454482610392565b5f606083614550613274565b81528260208201528160408201520152565b9080601f8301121561035a578160206106f1933591016107c1565b919060408382031261035a576040519061459682610372565b819380356001600160401b03811161035a57826145b4918301614562565b83526020810135916001600160401b03831161035a576020926104e79201614562565b60208152608063ffffffff606061463f61461e8651856020880152602061460a8251604060a08b015260e08a019061408b565b910151878203609f190160c089015261408b565b8360208801511660408701526040870151601f19878303018488015261408b565b9401511691015290565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b604051906146bc82610372565b5f6020838281520152565b604051906101806146d881846103ad565b368337565b604051906146ec6020836103ad565b6020368337565b919060409060606147026146af565b948592602085519261471485856103ad565b8436853780518452015160208301528482015260076107cf195a01fa1561473757565bfe5b60209291608060409261474a6146af565b9586938186519361475b86866103ad565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614737571561478c57565b63d4b68fd760e01b5f5260045ffd5b6040516147a781610372565b60409081516147b683826103ad565b82368237815260208251916147cb84846103ad565b83368437015280516147dd82826103ad565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061483383836103ad565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152614888835193846103ad565b8252602082015290565b5f516020614ecf5f395f51905f52906148a96146af565b505f919006602060c0835b6149a9575f935f516020614ecf5f395f51905f52600381868181800909086040516148df85826103ad565b843682378481856040516148f382826103ad565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614ecf5f395f51905f5260a082015260056107cf195a01fa80156147375761495d90614eb8565b51916149a9575f516020614ecf5f395f51905f528280091461499457505f516020614ecf5f395f51905f5260015f940892936148b4565b929350506149a06103e0565b92835282015290565b612555565b6149b66146af565b506040516149c381610372565b600181526002602082015290565b90600c8110156125505760051b0190565b939290916149f0604061040d565b9485526020850152614a02604061040d565b9182526020820152614a126146c7565b925f5b60028110614a3f57505050602061018092614a2e6146dd565b93849160086201d4c0fa9151151590565b80614a4b600192614397565b614a55828561253f565b5151614a6182896149d1565b526020614a6e838661253f565b510151614a83614a7d8361331d565b896149d1565b52614a8e828661253f565b515151614a9d614a7d8361332b565b52614ab3614aab838761253f565b515160200190565b51614ac0614a7d83613339565b526020614acd838761253f565b51015151614add614a7d83613347565b52614b09614b03614afc6020614af3868a61253f565b51015160200190565b5192613355565b886149d1565b5201614a15565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614b5c60ff93614e40565b928392161b1115614b6a5790565b63ca95733360e01b5f5260045ffd5b805f915b614b85575090565b5f198101818111612d655761ffff9116911661ffff8114612d65576001019080614b7d565b90614bb36146af565b5061ffff811690610200821015614c585760018214614c5357614bd46103e0565b5f81525f602082015292906001905f925b61ffff8316851015614bf957505050505090565b600161ffff831660ff86161c811614614c33575b6001614c29614c1e8360ff94614739565b9460011b61fffe1690565b9401169291614be5565b946001614c29614c1e614c488960ff95614739565b989350505050614c0d565b505090565b637fc4ea7d60e11b5f5260045ffd5b614c6f6146af565b50805190811580614ce0575b15614c9c575050604051614c906040826103ad565b5f81525f602082015290565b60205f516020614ecf5f395f51905f52910151065f516020614ecf5f395f51905f52035f516020614ecf5f395f51905f528111612d65576040519161488883610372565b50602081015115614c7b565b6033546001600160a01b03163303614d0057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614d9882614b79565b16614da2816107a6565b90614db060405192836103ad565b808252614dbf601f19916107a6565b013660208301375f5f5b8251821080614e1f575b15614e18576001811b8416614df1575b614dec90612dd0565b614dc9565b906001614dec9160ff60f81b8460f81b165f1a614e0e82876127e9565b5301919050614de3565b5050905090565b506101008110614dd3565b15614e3157565b631019106960e31b5f5260045ffd5b90610100825111614ea957815115614ea457602082015160019060f81c81901b5b8351821015614e9f57600190614e8a614e8061198c6129af86896127e9565b60ff600191161b90565b90614e96818311614e2a565b17910190614e61565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614ebf57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220251991cf2477ae5902fbdd3d7fdcf158dfc6bc36bf12b1911edaecb9fdf6cfa064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01@\x80`@R4a\x01\xE6W``\x81aQ\xED\x808\x03\x80\x91a\0 \x82\x85a\x02\x91V[\x839\x81\x01\x03\x12a\x01\xE6W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x81\x03a\x01\xE6W` \x83\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x93\x84\x82\x03a\x01\xE6W`@\x01Q\x93c\xFF\xFF\xFF\xFF\x85\x16\x85\x03a\x01\xE6W\x15a\x02\x82W`\x80R`\xA0R`@Qch0H5`\xE0\x1B\x81R` \x81`\x04\x81\x85Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x02?W[P`\xC0R`@Qc.\xFA,\xA3`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xFDW[P`\xE0R`\xC0Q`@Qc\xDF\\\xF7#`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xF2W_\x91a\x01\xACW[Pa\x01\0Ra\x01 R`@QaO$\x90\x81a\x02\xC9\x829`\x80Q\x81\x81\x81a\x02\xCA\x01R\x81\x81a\r\x87\x01R\x81\x81a\x15\t\x01Ra$&\x01R`\xA0Q\x81\x81\x81a\t|\x01R\x81\x81a\x10\xD9\x01R\x81\x81a\x18.\x01R\x81\x81a5&\x01R\x81\x81a5\xFC\x01Ra;\x8E\x01R`\xC0Q\x81\x81\x81a\x10\x95\x01R\x81\x81a8\xFE\x01Ra:I\x01R`\xE0Q\x81\x81\x81a\x10Q\x01R\x81\x81a\x17R\x01Ra8:\x01Ra\x01\0Q\x81\x81\x81a!\x8D\x01Ra7\x0F\x01Ra\x01 Q\x81\x81\x81a\x05\xAE\x01Ra\x1D-\x01R\xF3[\x90P` \x81=` \x11a\x01\xEAW[\x81a\x01\xC7` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xFAV[_\x80\xFD[=\x91Pa\x01\xBAV[`@Q=_\x82>=\x90\xFD[\x90P` \x81=` \x11a\x027W[\x81a\x02\x18` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W_a\0\xC2V[=\x91Pa\x02\x0BV[\x90P` \x81=` \x11a\x02zW[\x81a\x02Z` \x93\x83a\x02\x91V[\x81\x01\x03\x12a\x01\xE6WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x01\xE6W`\x04a\0\x96V[=\x91Pa\x02MV[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[`\x1F\x90\x91\x01`\x1F\x19\x16\x81\x01\x90`\x01`\x01`@\x1B\x03\x82\x11\x90\x82\x10\x17a\x02\xB4W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x9AW\x80c\x17\x1F\x1D[\x14a\x02\x95W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x90W\x80c,\xB2#\xD5\x14a\x02\x8BW\x80c-\x89\xF6\xFC\x14a\x02\x86W\x80c1\xB3k\xD9\x14a\x02\x81W\x80c5c\xB0\xD1\x14a\x02|W\x80c9\x98\xFD\xD3\x14a\x02wW\x80cAl~^\x14a\x02rW\x80cM+W\xFE\x14a\x02mW\x80cOs\x9Ft\x14a\x02hW\x80cY\\jg\x14a\x02cW\x80cZ-\x7F\x02\x14a\x02^W\x80cZ\xC8j\xB7\x14a\x02YW\x80c\\\x15Vb\x14a\x02TW\x80c\\\x97Z\xBB\x14a\x02OW\x80c]\xEC\xC3\xF5\x14a\x02JW\x80c]\xF4YF\x14a\x02EW\x80ch0H5\x14a\x02@W\x80cm\x14\xA9\x87\x14a\x02;W\x80cn\xFBF6\x14a\x026W\x80cqP\x18\xA6\x14a\x021W\x80cr\xD1\x8E\x8D\x14a\x02\"W\x80cz\xFA\x1E\xED\x14a\x02,W\x80c\x88o\x11\x95\x14a\x02'W\x80c\x8B\0\xCE|\x14a\x02\"W\x80c\x8D\xA5\xCB[\x14a\x02\x1DW\x80c\x92\xA5\x18$\x14a\x02\x18W\x80c\x9B)\x0E\x98\x14a\x02\x13W\x80c\xB7\x029I\x14a\x02\x0EW\x80c\xB9\x8D\t\x08\x14a\x02\tW\x80c\xCA\x8A\xA7\xC7\x14a\x02\x04W\x80c\xCC*\x9A[\x14a\x01\xFFW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xFAW\x80c\xDF\\\xF7#\x14a\x01\xF5W\x80c\xEF\x02\x9D\xBC\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a#\xFDV[a#\xE2V[a\x05\x92V[a#QV[a!\xBCV[a!xV[a 4V[a\x1E\xF4V[a\x1E\xCCV[a\x1E\xAAV[a\x1C\x10V[a\x1B\xE8V[a\x15\x8DV[a\x158V[a\x14\xA9V[a\x14\xF4V[a\x14\xCCV[a\x14NV[a\x13\xA1V[a\x10\xC4V[a\x10\x80V[a\x10<V[a\x0F\xFEV[a\x0F\xE1V[a\x0EhV[a\r\xFCV[a\r\xCFV[a\r\\V[a\x0C\xB5V[a\n\xAAV[a\tJV[a\t\x18V[a\x08\x9EV[a\x06\xF4V[a\x06LV[a\x06\x13V[a\x05\xD2V[a\x05 V[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UWa\x03$\x92a\x03\x10\x91_\x91a\x03&W[Pa$\xFFV[a\x03\x1F`fT\x82\x81\x16\x14a%\x15V[aF}V[\0[a\x03H\x91P` =` \x11a\x03NW[a\x03@\x81\x83a\x03\xADV[\x81\x01\x90a$\xDFV[_a\x03\nV[P=a\x036V[a$\xF4V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[a\x03^V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[`@Q\x90a\x03\xDEa\x01\0\x83a\x03\xADV[V[`@Q\x90a\x03\xDE`@\x83a\x03\xADV[`@Q\x90a\x03\xDE``\x83a\x03\xADV[`@Q\x90a\x03\xDE`\xA0\x83a\x03\xADV[\x90a\x03\xDE`@Q\x92\x83a\x03\xADV[`@\x90`\xE3\x19\x01\x12a\x03ZW`@Q\x90a\x044\x82a\x03rV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03ZW`@Qa\x04\\\x81a\x03rV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW`@Q\x91a\x04\x87`@\x84a\x03\xADV[\x82\x90`@\x81\x01\x92\x83\x11a\x03ZW\x90[\x82\x82\x10a\x04\xA3WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x96V[\x90`\x80`c\x19\x83\x01\x12a\x03ZW`@Qa\x04\xCC\x81a\x03rV[` a\x04\xE7\x82\x94a\x04\xDE\x81`da\x04lV[\x84R`\xA4a\x04lV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03ZW` a\x04\xE7`@Q\x92a\x05\n\x84a\x03rV[`@\x84\x96a\x05\x18\x83\x82a\x04lV[\x86R\x01a\x04lV[4a\x03ZWa\x01 6`\x03\x19\x01\x12a\x03ZW`\x045`@6`#\x19\x01\x12a\x03ZWa\x05x`@\x91\x82Qa\x05R\x81a\x03rV[`$5\x81R`D5` \x82\x01Ra\x05h6a\x04\xB3V[\x90a\x05r6a\x04\x1BV[\x92a%iV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03ZWV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03ZWV[5\x90a\x03\xDE\x82a\x05\xFAV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x065\x81a\x05\xFAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x06n\x81a\x05\xFAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03ZWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBDV[\x90` a\x06\xF1\x92\x81\x81R\x01\x90a\x06\xADV[\x90V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\x07\x11\x81a\x06\x85V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW6`#\x83\x01\x12\x15a\x03ZW\x81`\x04\x015\x91a\x07=\x83a\x06\x96V[\x92a\x07K`@Q\x94\x85a\x03\xADV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03ZW`$\x01\x90[\x82\x82\x10a\x07\x8CWa\x07\x88a\x07|\x86\x86a&\xD3V[`@Q\x91\x82\x91\x82a\x06\xE0V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9B\x81a\x06\x85V[\x81R\x01\x91\x01\x90a\x07hV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xCD\x82a\x07\xA6V[\x91a\x07\xDB`@Q\x93\x84a\x03\xADV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03ZW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\"WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08bWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x13V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08CV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x08\xBB\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW6`#\x82\x01\x12\x15a\x03ZWa\x07\x88\x91a\x08\xF2a\t\x04\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC1V[`D5\x91a\x08\xFF\x83a\x05\xFAV[a)\x04V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xF7V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\tg\x81a\t@V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\t\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xCDWa\x03$\x90aK\x10V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\t\xFE\x91P` =` \x11a\n\x04W[a\t\xF6\x81\x83a\x03\xADV[\x81\x01\x90a'\x8BV[_a\t\xB4V[P=a\t\xECV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\n\"\x81a\x06\x96V[\x92a\n0`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\nXWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nKV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n~V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\n\xC7\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\n\xE6\x906\x90`\x04\x01a\n\x0BV[a\n\xF0\x81Qa&qV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x8DW\x80` a\x0B\x15a\x0B5\x93\x86a&\xB0V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03UW`\x01\x92a\x0Bi\x91_\x91a\x0BoW[Pa\x0BZ\x83\x88a&\xB0V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\xFCV[a\x0B\x87\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x0BOV[`@Q\x80a\x07\x88\x86\x82a\nhV[\x91\x81`\x1F\x84\x01\x12\x15a\x03ZW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03ZWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xE5WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xD8V[\x90` \x82R``a\x0COa\x0C:a\x0C$\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0B\xC8V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0B\xC8V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0B\xC8V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xA6`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0B\xC8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0CyV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045a\x0C\xD2\x81a\x06\x85V[`$5\x90a\x0C\xDF\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0C\xFE\x906\x90`\x04\x01a\x0B\x9BV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`#\x85\x01\x12\x15a\x03ZW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03ZWa\x07\x88\x95`$a\rP\x96\x01\x93a.\x03V[`@Q\x91\x82\x91\x82a\x0C\x01V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa\r\xC7\x91_\x91a\x03&WPa$\xFFV[a\x03$aFIV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW` `\x01`\xFF`\x045a\x0E\x1F\x81a\r\xF1V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0ERWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0EEV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x0E\x85\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0E\xA4\x906\x90`\x04\x01a\n\x0BV[`D5\x91a\x0E\xB1\x83a\x05\xFAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x0E\xD9\x86\x88`\x04\x84\x01a2WV[\x03\x81\x84Z\xFA\x91\x82\x15a\x03UW_\x92a\x0F\xBDW[Pa\x0E\xF7\x83Qa&qV[\x93_[\x84Q\x81\x10\x15a\x0F\xAFWa\x0F\r\x81\x86a&\xB0V[Q\x90` \x83a\x0F)a\x0F\x1F\x84\x89a&\xB0V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a\x0F\x81W[P\x82\x80`\xC0\x1B\x03\x16a\x0Fz\x82\x89a&\xB0V[R\x01a\x0E\xFAV[a\x0F\xA2\x91P` =\x81\x11a\x0F\xA8W[a\x0F\x9A\x81\x83a\x03\xADV[\x81\x01\x90a-zV[_a\x0FhV[P=a\x0F\x90V[`@Q\x80a\x07\x88\x88\x82a\x0E/V[a\x0F\xDA\x91\x92P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[\x81\x01\x90a,IV[\x90_a\x0E\xECV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `fT`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x10 \x81a\x05\xFAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x11\x1F\x81a\x06\x96V[\x92a\x11-`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\x11UWPPP\x90V[` \x80\x91\x835a\x11d\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a\x11HV[\x81`\x1F\x82\x01\x12\x15a\x03ZW\x805a\x11\x85\x81a\x06\x96V[\x92a\x11\x93`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03ZW` \x01\x91[\x83\x83\x10a\x11\xBDWPPPP\x90V[` `@\x91a\x11\xCC\x84\x86a\x04DV[\x81R\x01\x92\x01\x91a\x11\xAFV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x11\xEE\x81a\x06\x96V[\x92a\x11\xFC`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03ZW` \x82\x01\x90[\x83\x82\x10a\x12(WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW` \x91a\x12J\x87\x84\x80\x94\x88\x01\x01a\x11\x08V[\x81R\x01\x91\x01\x90a\x12\x19V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03ZWa\x12ka\x03\xCEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\x88\x91\x84\x01a\x11\x08V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\xA9\x91\x84\x01a\x11oV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\xCD\x91\x84\x01a\x11oV[`@\x85\x01Ra\x12\xDF\x81``\x84\x01a\x04\xECV[``\x85\x01Ra\x12\xF1\x81`\xE0\x84\x01a\x04DV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x13\x16\x91\x84\x01a\x11\x08V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x13;\x91\x84\x01a\x11\x08V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13^\x92\x01a\x11\xD7V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x13\x82WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13uV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13\xD4\x906\x90`\x04\x01a\x0B\x9BV[\x90\x91`D5a\x13\xE2\x81a\x05\xFAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZWa\x14D\x94a\x14\ta\x14\x0F\x956\x90`\x04\x01a\x12UV[\x93a4IV[`@Q\x92\x83\x92`@\x84R` a\x140\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x13eV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x13eV[\x90` \x83\x01R\x03\x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZWa\x14faL\xECV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x03ZW\x90V[`@\x90`#\x19\x01\x12a\x03ZW`$\x90V[\x90\x81`@\x91\x03\x12a\x03ZW\x90V[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x15\xBD\x906\x90`\x04\x01a\x15`V[a\x15\xC66a\x15nV[`@6`c\x19\x01\x12a\x03ZW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x15\xF1\x906\x90`\x04\x01a\x11oV[a\x16ia\x15\xFD\x83a<\xD3V[\x92a\x16!a\x16\x19\x85c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a<\xDDV[a\x169\x84c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x90`@Qa\x16`\x81a\x16R`d` \x83\x01\x95\x86a=QV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x14a=oV[a\x16\x94a\x16\x8Ea\x16\x87\x84c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a=\xE1V[a\x16\xC3a\x16\xB5a\x16\xACa\x16\xA7`da<\xD3V[a>wV[c\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a>\xA9V[a\x16\xCD\x81Qa&qV[\x92_[\x82Q\x81\x10\x15a\x17\rW\x80a\x16\xFCa\x16\xE9`\x01\x93\x86a&\xB0V[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x17\x06\x82\x88a&\xB0V[R\x01a\x16\xD0V[P\x92\x91\x90\x91` \x84\x01\x91a\x17Ea\x17#\x84a<\xD3V[`@Qa\x179\x81a\x16R\x87` \x83\x01\x95\x86a4\tV[Q\x90 `\x845\x14a?\x1BV[a\x17O\x84Qa&qV[\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x86Q\x81\x10\x15a\x17\xFFW\x80` a\x17\x96a\x17\xB6\x93\x88a&\xB0V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8AZ\xFA\x91\x82\x15a\x03UW`\x01\x92a\x17\xDB\x91_\x91a\x17\xE1W[Pa\x0BZ\x83\x8Ca&\xB0V[\x01a\x17}V[a\x17\xF9\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x17\xD0V[P\x86\x91\x85\x91a\x18[a\x18$a\x18,`@\x85\x01\x93a\x18\x1C\x85\x87a?\xA6V[\x93\x90\x91a<\xD3V[\x926\x91a\x07\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a)\x04V[\x93_\x90[\x85Q\x82\x10\x15a\x1B\x91W_\x96\x95\x96\x94[a\x18x\x83\x89a&\xB0V[QQ\x86\x10\x15a\x1B\x81W\x87` \x80a\x18\x9C\x89a\x18\x96\x88a\x18\xBE\x9Ea&\xB0V[Qa&\xB0V[Q\x01Q`@Q\x80\x9B\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x98\x89\x15a\x03UW_\x99a\x1BaW[P`\x01\x97_[\x84Q\x81\x10\x15a\x1BRWa\x19\ta\x18\xFDa\x18\xF0\x83\x88a&\xB0V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8C\x16\x14a\x19 W`\x01\x01a\x18\xD7V[P\x98\x90\x97P`\x01_\x97\x92\x97[\x15\x15\x14a\x19AW[P`\x01\x01\x94\x96\x95\x96a\x18nV[\x97\x95\x90\x94\x96\x91\x84\x92_\x89a\x19\xBA`\xFFa\x19\x92a\x19\x8Ca\x19~\x8Ba\x19xa\x1A\x0F\x9Ea\x19r`\xD1T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a?\xA6V[\x90a-\xAFV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[a\x19\xACa\x19\x9Da\x03\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD0Ta\x19\xD1\x90a\x18\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x97\x88\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x95\x86\x15a\x03UW_\x96a\x1B.W[Pa\x1A,\x86Qa&qV[\x97_[\x89Q\x81\x10\x15a\x1AUW\x80g\x01cEx]\x8A\0\0a\x1AN`\x01\x93\x8Da&\xB0V[R\x01a\x1A/V[P\x99\x92\x97\x90\x98\x96\x91\x93\x95a\x1A\xA0`\xFFa\x1Axa\x19\x8Ca\x19~\x8Aa\x19x\x8D\x8Fa?\xA6V[a\x1A\x92a\x1A\x83a\x03\xFEV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra\x1A\xB2a@[V[`\x80\x83\x01R`\xCFTa\x1A\xCE\x90a\x18\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03ZW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a\x1A\xFA\x90`\x04\x83\x01a@\xAFV[\x03\x92Z\xF1\x91\x82\x15a\x03UW`\x01\x92a\x1B\x14W[P\x90a\x194V[\x80a\x1B\"_a\x1B(\x93a\x03\xADV[\x80a\x05\x88V[\x89a\x1B\rV[a\x1BK\x91\x96P=\x80_\x83>a\x1BC\x81\x83a\x03\xADV[\x81\x01\x90a?\xD8V[\x94\x8Aa\x1A!V[P\x98\x90\x97`\x01\x90\x97\x92\x97a\x19,V[a\x1Bz\x91\x99P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x97\x89a\x18\xD1V[\x95\x96\x95\x94P`\x01\x90\x91\x01\x90a\x18_V[\x86a\x1B\xBAa\x1B\xAD\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x1C@\x906\x90`\x04\x01a\x15`V[a\x1CI6a\x15nV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x1Ci\x906\x90`\x04\x01a\x12UV[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1EeWa\x1C\x8C` \x83\x94\x93\x01a<\xD3V[\x91a\x1Dxa\x1C\x9D`@\x86\x01\x86a?\xA6V[\x92\x90\x94a\x1C\xEFa\x1C\xAF``\x89\x01a<\xD3V[\x97`@Qa\x1C\xC5\x81a\x16R` \x82\x01\x94\x85aA\x84V[Q\x90 a\x1C\xE8a\x1C\xD4\x88a<\xD3V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14aB<V[a\x1D\x19a\x1D\x12a\x1C\xFE\x87a<\xD3V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aB\xAEV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x1DZa\x1DRa\x16\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a>\x8FV[\x89\x11\x15aC\x0FV[`@Q` \x81\x01\x90a\x1Dp\x81a\x16R\x8B\x85aCqV[Q\x90 a4IV[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x1E\x03W\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x86\x86\x86a\x1D\xC3a\x1D\xB7a\x03\xE0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x1D\xDF\x81a\x16R\x86\x86\x86aDAV[Q\x90 a\x1D\xEEa\x1C\xFE\x83a<\xD3V[Ua\x1D\xFE`@Q\x92\x83\x92\x83aDAV[\x03\x90\xA1\0[\x80a\x1E_a\x1E;a\x1E6a\x1E*a\x1E\x1D`\x01\x96\x88Qa&\xB0V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[aC\x81V[a\x1EXa\x1E*\x8Ba\x1ESa\x1E\x1D\x87` \x8B\x01Qa&\xB0V[aC\xADV[\x11\x15aC\xD0V[\x01a\x1D\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045a\x1F\x11\x81a\x06\x85V[a\x1F\x91`$5a\x1F \x81a\x06\x85V[`D5a\x1F,\x81a\x06\x85V[`d5a\x1F8\x81a\x06\x85V[`\x845\x91a\x1FE\x83a\x06\x85V[`\xA45\x93a\x1FR\x85a\x06\x85V[_T\x96a\x1Fw`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa \x0FW[\x81\x15a\x1F\xEFW[PaDkV[\x87a\x1F\x88`\x01`\xFF\x19_T\x16\x17_UV[a\x1F\xD8WaD\xCEV[a\x1F\x97W\0[a\x1F\xA5a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x1D\xFEV[a\x1F\xEAa\x01\0a\xFF\0\x19_T\x16\x17_UV[aD\xCEV[0;\x15\x91P\x81a \x01W[P_a\x1FqV[`\xFF\x16`\x01\x14\x90P_a\x1F\xFAV[`\x01`\xFF\x82\x16\x10\x91Pa\x1FjV[`@\x90a\x06\xF1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xF7V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a Q\x81a\x06\x85V[`$5`D5a `\x81a\x05\xFAV[a \xA1a ka&OV[\x92\x80a v\x85a&\xA3V[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a2WV[\x03\x81\x87Z\xFA\x93\x84\x15a\x03UW\x83a \xCBa\x16\xACa\x0F\x1Fa!\0\x98` \x97_\x91a!^W[Pa&\xA3V[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03UWa!/\x92_\x91a!?W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a!)\x84aM\x8CV[\x90a)\x04V[\x90a\x07\x88`@Q\x92\x83\x92\x83a \x1DV[a!X\x91P` =` \x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[_a!\x15V[a!r\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a \xC5V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa!\xEC\x906\x90`\x04\x01a\x15\x7FV[`$5\x90a!\xF9\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\"\x18\x906\x90`\x04\x01a\x0B\x9BV[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a#\x02Wa\x03$\x93a\"\xEC\x93a\"la\"s\x93a\"Qa\"IaE7V[\x966\x90aE}V[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xC1V[`@\x82\x01R`@Q` \x81\x01\x90a\"\x8E\x81a\x16R\x85\x85aE\xD7V[Q\x90 a\"\xA3a\x1C\xD4`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xA6\xB1\x91-\x06o\xD3\xA5A:b\"\xA5xZ\xE4?$$X\xF3\xEE\xF4\\P\xDD\x80\x89\x87\xB8x\xFD`@Q\x80a\"\xE4c\xFF\xFF\xFF\xFF\x86\x16\x94\x82aE\xD7V[\x03\x90\xA2a>_V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a#n\x81a\x06\x85V[a#vaL\xECV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#\x8EWa\x03$\x90aMDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Q`d\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a$\xC0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a$\xB1Wa$\x7F`fT\x19\x82\x19\x81\x16\x14a%\x15V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a$\xD9\x91P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a$^V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\t@V[`@Q=_\x82>=\x90\xFD[\x15a%\x06WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a%\x1CWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a%PW`\x05\x1B\x01\x90V[a%+V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a&Ea&\"a&K\x95a&\x1Ca&\x15\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra%\xEC\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF\xF3V[\x90aG9V[\x92a&\x1Ca&7a&1aG\x9BV[\x94aH\x92V[\x91a&@aI\xAEV[aF\xF3V[\x91aI\xE2V[\x90\x91V[`@\x80Q\x90\x91\x90a&`\x83\x82a\x03\xADV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a&{\x82a\x06\x96V[a&\x88`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a&\x99`\x1F\x19\x91a\x06\x96V[\x01\x90` 6\x91\x017V[\x80Q\x15a%PW` \x01\x90V[\x80Q\x82\x10\x15a%PW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ\x90V[\x91\x90\x91a&\xE0\x83Qa&qV[\x92_[\x81Q\x81\x10\x15a'\x86W\x80` a&\xFFa\x18\xF0a'(\x94\x86a&\xB0V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a'XW[Pa'Q\x82\x88a&\xB0V[R\x01a&\xE3V[a'y\x91P` =\x81\x11a'\x7FW[a'q\x81\x83a\x03\xADV[\x81\x01\x90a&\xC4V[_a'FV[P=a'gV[PPPV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x06\x85V[\x90a'\xAA\x82a\x06\x96V[a'\xB7`@Q\x91\x82a\x03\xADV[\x82\x81R` \x81\x93a'\xCA`\x1F\x19\x91a\x06\x96V[\x01\x91\x01_[\x82\x81\x10a'\xDBWPPPV[``\x82\x82\x01R` \x01a'\xCFV[\x90\x81Q\x81\x10\x15a%PW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa(-\x81a\x06\x96V[\x92a(;`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a(cWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a(VV[\x90a(}\x82a\x06\x96V[a(\x8A`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a(\x9B`\x1F\x19\x91a\x06\x96V[\x01_[\x81\x81\x10a(\xAAWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x8DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a(\x9EV[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03UW_\x95a,\x04W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03UW`\x04\x96_\x93a+\xE2W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03UW_\x96a+\xC1W[Pa)\x92\x85\x93\x92\x95Qa'\xA0V[\x94_\x93[\x80Q\x85\x10\x15a+\xB7Wa)\xBDa\x19\x8Ca)\xAF\x87\x84a'\xE9V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03UW_\x94a+\x93W[Pa*\r\x84Qa(sV[a*\x17\x88\x8Ba&\xB0V[Ra*\"\x87\x8Aa&\xB0V[P_[\x84Q\x81\x10\x15a+\x82W\x80` a*>a*`\x93\x88a&\xB0V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UW_\x92a+bW[Pa*\x86\x81\x87a&\xB0V[Q\x8A` \x8Aa*\x95\x85\x8Ba&\xB0V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03UWa+\x19\x8C\x8Fa+\x14`\x01\x98a++\x97\x89\x97_\x92a+2W[Pa*\xFFa\x1A\x83a\x03\xEFV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a&\xB0V[Q\x90a+%\x83\x83a&\xB0V[Ra&\xB0V[P\x01a*%V[a+T\x91\x92P` =\x81\x11a+[W[a+L\x81\x83a\x03\xADV[\x81\x01\x90a(\xE5V[\x90_a*\xF3V[P=a+BV[a+{\x91\x92P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x90_a*{V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa)\x96V[a+\xB0\x91\x94P=\x80_\x83>a+\xA8\x81\x83a\x03\xADV[\x81\x01\x90a'\xFAV[\x92_a*\x02V[PPP\x93PPP\x90V[a+\xDB\x91\x96P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x94_a)\x84V[` \x91\x93Pa+\xFD\x90\x82=\x84\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x92\x90a)_V[a,\x1E\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a)6V[`@Q\x90a,2\x82a\x03\x92V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa,|\x81a\x06\x96V[\x92a,\x8A`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a,\xB2WPPP\x90V[` \x80\x91\x83Qa,\xC1\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a,\xA5V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03ZW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a-\x03V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a-eW`\x01\x01\x90V[a-@V[\x91\x90\x81\x10\x15a%PW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[\x15a-\xA0WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a%PW\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x05\xFAV[_\x19\x81\x14a-eW`\x01\x01\x90V[\x91a-\xFC` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a-\x03V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a.\x13a,%V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03UW_\x95a26W[Pa.Pa,%V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a.p\x8D\x8D\x8B`\x04\x85\x01a,\xCCV[\x03\x81\x88Z\xFA\x90\x81\x15a\x03UW_\x91a2\x1CW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a.\xB3\x85\x87\x8B`\x04\x85\x01a-#V[\x03\x81\x87Z\xFA\x90\x81\x15a\x03UW_\x91a2\x02W[P`@\x87\x01Ra.\xD5\x81a'\xA0V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a1MW\x88_a/\x08\x83\x8Fa.\xFB\x88a&qV[\x90Q\x90a+%\x83\x83a&\xB0V[P_\x8A\x86\x8F[\x81\x84\x10a/\x85WPPPP\x90P\x8Ca/%\x82a&qV[\x91_[\x81\x81\x10a/RWPP\x91a/G\x91a/M\x94\x93Q\x90a+%\x83\x83a&\xB0V[Pa-TV[a.\xDFV[\x80a/\x7Fa/ja\x0F\x1F`\x01\x94a\x18\x96\x8A\x89Qa&\xB0V[a/t\x83\x88a&\xB0V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a/(V[a\x0F\x1F\x84a/\x9A\x81` \x96\x95a/\xA2\x95a-jV[5\x97Qa&\xB0V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03UW\x88\x8F\x88\x8A\x91\x8F\x94a09`\x01a0,\x81\x93\x8D\x80\x9D_\x92a1!W[Pa\x19\x8Ca\x19~a0\x16\x92a0\x0F\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a-\x99V[\x8B\x8Da-\xAFV[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a0UW[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa/\x0EV[\x85\x97a0w\x93a0p` \x97\x99\x98a\x19\x8C\x95a\x19~\x95a-jV[5\x95a-\xAFV[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03UW\x8Fa0\xD5\x90a0\xDA\x93\x83\x88`\x01\x97_\x93a0\xE9W[Pa\x18\x96\x90a/t\x93\x94Qa&\xB0V[a-\xD0V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a0?V[a/t\x93P\x90a1\x12a\x18\x96\x92` =\x81\x11a1\x1AW[a1\n\x81\x83a\x03\xADV[\x81\x01\x90a-\xBBV[\x93P\x90a0\xC5V[P=a1\0V[a0\x16\x91\x92Pa\x19~a1Da\x19\x8C\x92` =\x81\x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[\x93\x92PPa/\xF2V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03UWa1\xA3\x94_\x94\x85\x93a1\xE1W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a-\xDEV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a1\xC7W[P` \x82\x01R\x90V[a1\xDB\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a1\xBEV[a1\xFB\x91\x93P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x91_a1\x85V[a2\x16\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a.\xC6V[a20\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a.\x83V[a2P\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a.GV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xADV[`@Q\x90a2\x81\x82a\x03rV[``` \x83\x82\x81R\x01RV[\x15a2\x94WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a2\xA9WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a2\xBFWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a2\xD5WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\r\xF1V[_\x19\x81\x01\x91\x90\x82\x11a-eWV[\x15a3\x0EWV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a-eWV[\x90`\x02\x82\x01\x80\x92\x11a-eWV[\x90`\x03\x82\x01\x80\x92\x11a-eWV[\x90`\x04\x82\x01\x80\x92\x11a-eWV[\x90`\x05\x82\x01\x80\x92\x11a-eWV[\x91\x90\x82\x01\x80\x92\x11a-eWV[\x15a3wWV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03ZW\x90V[\x15a3\xAEWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a-eWV[\x15a3\xE4WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a3\xFAWV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a43WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4&V[\x94\x93\x92\x90\x91\x93a4Wa2tV[Pa4c\x85\x15\x15a2\x8DV[`@\x84\x01QQ\x85\x14\x80a<\xC5W[\x80a<\xB7W[\x80a<\xA9W[a4\x86\x90a2\xA2V[a4\x98` \x85\x01QQ\x85QQ\x14a2\xB8V[a4\xAFc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a2\xCEV[a4\xB7a\x03\xE0V[_\x81R_` \x82\x01R\x92a4\xC9a2tV[a4\xD2\x87a&qV[` \x82\x01Ra4\xE0\x87a&qV[\x81Ra4\xEAa2tV[\x92a4\xF9` \x88\x01QQa&qV[\x84Ra5\t` \x88\x01QQa&qV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa5r\x91_\x91a<zW[Pa5m6\x8B\x87a\x07\xC1V[aKNV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a6\xD1W` \x88a5\xC6a\x0F\x1F\x8Ca5\xBE\x8F\x96\x86\x8Ea5\xA3a\x16\xE9\x86\x80\x95a&\xB0V[a5\xB0\x84\x84\x84\x01Qa&\xB0V[R\x82a6\x9EW[\x01Qa&\xB0V[Q\x95Qa&\xB0V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03UWa&\x1C\x8Aa6s\x8Fa6l\x8F\x84` \x8F\x92a6c\x93a6[\x84`\x01\x9Ea6y\x9E_\x91a6\x81W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa&\xB0V[R\x01Qa&\xB0V[Q\x93\x8DQa&\xB0V[Q\x16aKyV[\x90aK\xAAV[\x97\x01\x96a5vV[a6\x98\x91P\x86=\x81\x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[_a6LV[a6\xCCa6\xAE\x84\x84\x84\x01Qa&\xB0V[Qa6\xC5\x84\x84\x01Qa6\xBF\x87a2\xF9V[\x90a&\xB0V[Q\x10a3\x07V[a5\xB7V[P\x90\x95\x97\x94\x96Pa6\xE6\x91\x98\x93\x92\x99PaLgV[\x91a6\xF3`\x97T`\xFF\x16\x90V[\x90\x81\x15a<rW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a<SW[P\x91\x90[_\x92[\x81\x84\x10a7\xA4WPPPPP\x92a7\x8Ba7\x86a7\x7Fa7\x9E\x95\x85a\x16R\x98`\x80``` \x99\x01Q\x92\x01Q\x92a%iV[\x91\x90a3\xDDV[a3\xF3V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a4\tV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da;MW[a\x0F\x1F\x82`\xA0a7\xF9a\x19\x8Ca\x19~\x84a8\x01\x97a7\xF3a7\xE5a\x16\xE9\x8F\x9C`@` \x9F\x9E\x01Qa&\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba-\xAFV[\x97\x01Qa&\xB0V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa8\xC5a\x0F\x1F\x8F\x95\x8F\x90a8\xBD\x8F\x97\x8F\x96\x84\x8Fa8\xB7`\xC0\x96a8\xB0\x84\x8F` \x9F\x90a5\xB7a\x19~\x99`@\x93a\x19\x8C\x9C_\x91a;\x1FW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a3\xA7V[Q\x90aG9V[\x9Ca-\xAFV[\x96\x01Qa&\xB0V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa9R\x91\x8C\x8F\x92_\x92a:\xFBW[P` a9D\x92\x93\x01Qa&\xB0V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a9r\x8Ca9D\x8Ca9ka\x1E\x1D\x82` \x86\x01Qa&\xB0V[\x92Qa&\xB0V[_\x98_[` \x8A\x01QQ\x81\x10\x15a:\xE2W\x8B\x8Da9\xB4\x89a9\xA7a\x19\x8Ca\x19~\x86\x8F\x89a9\x9F\x91Qa&\xB0V[Q\x94\x87a-\xAFV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a9\xC3W[PP`\x01\x01a9vV[\x8A\x8Aa:E\x85\x9F\x94\x8F\x96\x86a\x18\x96\x8F\x93`\xE0a9\xFCa\x0F\x1F\x95` a9\xF4a\x19\x8Ca\x19~\x83\x9Fa:\x05\x9C\x89\x91a-\xAFV[\x9A\x01Qa&\xB0V[Q\x9B\x01Qa&\xB0V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW\x8Fa:\xB1\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a:\xBCW[Pa:\xABa9D\x92\x93Q\x93a:\xA6a\x1E\x1D\x84\x87a&\xB0V[a3\xBDV[\x92a&\xB0V[\x01\x9A\x90P\x8B\x8Da9\xB9V[a9D\x92Pa:\xDBa:\xAB\x91` =\x81\x11a+[Wa+L\x81\x83a\x03\xADV[\x92Pa:\x8EV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a7NV[a9D\x92Pa;\x18` \x91\x82=\x81\x11a+[Wa+L\x81\x83a\x03\xADV[\x92Pa95V[` a;@\x92P=\x81\x11a;FW[a;8\x81\x83a\x03\xADV[\x81\x01\x90a3\x86V[_a8\x9AV[P=a;.V[a;\x8A\x94Pa;g\x92Pa\x19\x8C\x91a\x19~\x91` \x95a-\xAFV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UW` \x89a8\x01\x8F\x93\x8F`\xA0\x8F\x97a\x19\x8Ca\x19~\x8F\x8F\x90a7\xF3a7\xE5a\x16\xE9\x8F`@\x8B\x96\x91\x8F\x88\x93a\x0F\x1F\x9Fa<\x0E\x90a<\x14\x93a7\xF9\x9F_\x92a<*W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a3cV[\x11a3pV[PPPPPP\x97PPPPPP\x92\x93PPa7\xB9V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a<K\x91=\x81\x11a'\x7FWa'q\x81\x83a\x03\xADV[\x92\x91Pa;\xFDV[a<l\x91P` =` \x11a1\x1AWa1\n\x81\x83a\x03\xADV[_a7GV[_\x91\x90a7KV[a<\x9C\x91P` =` \x11a<\xA2W[a<\x94\x81\x83a\x03\xADV[\x81\x01\x90a2\xE4V[_a5aV[P=a<\x8AV[P`\xE0\x84\x01QQ\x85\x14a4}V[P`\xC0\x84\x01QQ\x85\x14a4wV[P`\xA0\x84\x01QQ\x85\x14a4qV[5a\x06\xF1\x81a\x05\xFAV[\x15a<\xE4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a=G\x81a\x05\xFAV[\x16\x84R\x015\x91\x01RV[\x91`@a\x03\xDE\x92\x94\x93a=h\x81`\x80\x81\x01\x97a=3V[\x01\x90a=3V[\x15a=vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a=\xE8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[\x15a>\xB0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a?\"WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03ZW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW` \x01\x91\x816\x03\x83\x13a\x03ZWV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa@\x0B\x81a\x06\x96V[\x92a@\x19`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a@AWPPP\x90V[` \x80\x91\x83Qa@P\x81a\x06\x85V[\x81R\x01\x91\x01\x90a@4V[`@Q\x90a@j`@\x83a\x03\xADV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10aA4WPPP`\x80aA\x1Fa\x06\xF1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xADV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra@\x8BV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a@\xF6V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03ZW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x816\x03\x83\x13a\x03ZWV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x03ZW`\x80aB1``aB*aA\xF0\x87a\x06\xF1\x97\x01\x85` \x88\x01RaA\xDEaA\xD3aA\xC2\x83\x80aASV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a-\x03V[\x91` \x81\x01\x90aASV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a-\x03V[aB\x0CaA\xFF` \x8A\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aB\x19`@\x89\x01\x89aASV[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a-\x03V[\x95\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aBCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aB\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aC\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x03\xDE\x91\x90a=3V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a-eWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a-eWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a-eWV[\x15aC\xD7WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91aDW\x84`\x80\x81\x01\x97a=3V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aDrWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aD\xD7\x90aMDV[`\xCD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCE\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[`@Q\x90aED\x82a\x03\x92V[_``\x83aEPa2tV[\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81` a\x06\xF1\x935\x91\x01a\x07\xC1V[\x91\x90`@\x83\x82\x03\x12a\x03ZW`@Q\x90aE\x96\x82a\x03rV[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x82aE\xB4\x91\x83\x01aEbV[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x92a\x04\xE7\x92\x01aEbV[` \x81R`\x80c\xFF\xFF\xFF\xFF``aF?aF\x1E\x86Q\x85` \x88\x01R` aF\n\x82Q`@`\xA0\x8B\x01R`\xE0\x8A\x01\x90a@\x8BV[\x91\x01Q\x87\x82\x03`\x9F\x19\x01`\xC0\x89\x01Ra@\x8BV[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra@\x8BV[\x94\x01Q\x16\x91\x01R\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\xBC\x82a\x03rV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF\xD8\x81\x84a\x03\xADV[6\x837V[`@Q\x90aF\xEC` \x83a\x03\xADV[` 6\x837V[\x91\x90`@\x90``aG\x02aF\xAFV[\x94\x85\x92` \x85Q\x92aG\x14\x85\x85a\x03\xADV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aG7WV[\xFE[` \x92\x91`\x80`@\x92aGJaF\xAFV[\x95\x86\x93\x81\x86Q\x93aG[\x86\x86a\x03\xADV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aG7W\x15aG\x8CWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaG\xA7\x81a\x03rV[`@\x90\x81QaG\xB6\x83\x82a\x03\xADV[\x826\x827\x81R` \x82Q\x91aG\xCB\x84\x84a\x03\xADV[\x836\x847\x01R\x80QaG\xDD\x82\x82a\x03\xADV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aH3\x83\x83a\x03\xADV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaH\x88\x83Q\x93\x84a\x03\xADV[\x82R` \x82\x01R\x90V[_Q` aN\xCF_9_Q\x90_R\x90aH\xA9aF\xAFV[P_\x91\x90\x06` `\xC0\x83[aI\xA9W_\x93_Q` aN\xCF_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH\xDF\x85\x82a\x03\xADV[\x846\x827\x84\x81\x85`@QaH\xF3\x82\x82a\x03\xADV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aN\xCF_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aG7WaI]\x90aN\xB8V[Q\x91aI\xA9W_Q` aN\xCF_9_Q\x90_R\x82\x80\t\x14aI\x94WP_Q` aN\xCF_9_Q\x90_R`\x01_\x94\x08\x92\x93aH\xB4V[\x92\x93PPaI\xA0a\x03\xE0V[\x92\x83R\x82\x01R\x90V[a%UV[aI\xB6aF\xAFV[P`@QaI\xC3\x81a\x03rV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a%PW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI\xF0`@a\x04\rV[\x94\x85R` \x85\x01RaJ\x02`@a\x04\rV[\x91\x82R` \x82\x01RaJ\x12aF\xC7V[\x92_[`\x02\x81\x10aJ?WPPP` a\x01\x80\x92aJ.aF\xDDV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aJK`\x01\x92aC\x97V[aJU\x82\x85a%?V[QQaJa\x82\x89aI\xD1V[R` aJn\x83\x86a%?V[Q\x01QaJ\x83aJ}\x83a3\x1DV[\x89aI\xD1V[RaJ\x8E\x82\x86a%?V[QQQaJ\x9DaJ}\x83a3+V[RaJ\xB3aJ\xAB\x83\x87a%?V[QQ` \x01\x90V[QaJ\xC0aJ}\x83a39V[R` aJ\xCD\x83\x87a%?V[Q\x01QQaJ\xDDaJ}\x83a3GV[RaK\taK\x03aJ\xFC` aJ\xF3\x86\x8Aa%?V[Q\x01Q` \x01\x90V[Q\x92a3UV[\x88aI\xD1V[R\x01aJ\x15V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aK\\`\xFF\x93aN@V[\x92\x83\x92\x16\x1B\x11\x15aKjW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aK\x85WP\x90V[_\x19\x81\x01\x81\x81\x11a-eWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a-eW`\x01\x01\x90\x80aK}V[\x90aK\xB3aF\xAFV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aLXW`\x01\x82\x14aLSWaK\xD4a\x03\xE0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aK\xF9WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aL3W[`\x01aL)aL\x1E\x83`\xFF\x94aG9V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aK\xE5V[\x94`\x01aL)aL\x1EaLH\x89`\xFF\x95aG9V[\x98\x93PPPPaL\rV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aLoaF\xAFV[P\x80Q\x90\x81\x15\x80aL\xE0W[\x15aL\x9CWPP`@QaL\x90`@\x82a\x03\xADV[_\x81R_` \x82\x01R\x90V[` _Q` aN\xCF_9_Q\x90_R\x91\x01Q\x06_Q` aN\xCF_9_Q\x90_R\x03_Q` aN\xCF_9_Q\x90_R\x81\x11a-eW`@Q\x91aH\x88\x83a\x03rV[P` \x81\x01Q\x15aL{V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM\0WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaM\x98\x82aKyV[\x16aM\xA2\x81a\x07\xA6V[\x90aM\xB0`@Q\x92\x83a\x03\xADV[\x80\x82RaM\xBF`\x1F\x19\x91a\x07\xA6V[\x016` \x83\x017__[\x82Q\x82\x10\x80aN\x1FW[\x15aN\x18W`\x01\x81\x1B\x84\x16aM\xF1W[aM\xEC\x90a-\xD0V[aM\xC9V[\x90`\x01aM\xEC\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaN\x0E\x82\x87a'\xE9V[S\x01\x91\x90PaM\xE3V[PP\x90P\x90V[Pa\x01\0\x81\x10aM\xD3V[\x15aN1WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aN\xA9W\x81Q\x15aN\xA4W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aN\x9FW`\x01\x90aN\x8AaN\x80a\x19\x8Ca)\xAF\x86\x89a'\xE9V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aN\x96\x81\x83\x11aN*V[\x17\x91\x01\x90aNaV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aN\xBFWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 %\x19\x91\xCF$w\xAEY\x02\xFB\xDD=\x7F\xDC\xF1X\xDF\xC6\xBC6\xBF\x12\xB1\x91\x1E\xDA\xEC\xB9\xFD\xF6\xCF\xA0dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c8063136439dd1461029a578063171f1d5b146102955780631ad43189146101e6578063245a7bfc146102905780632cb223d51461028b5780632d89f6fc1461028657806331b36bd9146102815780633563b0d11461027c5780633998fdd314610277578063416c7e5e146102725780634d2b57fe1461026d5780634f739f7414610268578063595c6a67146102635780635a2d7f021461025e5780635ac86ab7146102595780635c155662146102545780635c975abb1461024f5780635decc3f51461024a5780635df459461461024557806368304835146102405780636d14a9871461023b5780636efb463614610236578063715018a61461023157806372d18e8d146102225780637afa1eed1461022c578063886f1195146102275780638b00ce7c146102225780638da5cb5b1461021d57806392a51824146102185780639b290e9814610213578063b70239491461020e578063b98d090814610209578063ca8aa7c714610204578063cc2a9a5b146101ff578063cefdc1d4146101fa578063df5cf723146101f5578063ef029dbc146101f0578063f2fde38b146101eb578063f5c9899d146101e6578063f63c5bab146101e15763fabc1cbc146101dc575f80fd5b6123fd565b6123e2565b610592565b612351565b6121bc565b612178565b612034565b611ef4565b611ecc565b611eaa565b611c10565b611be8565b61158d565b611538565b6114a9565b6114f4565b6114cc565b61144e565b6113a1565b6110c4565b611080565b61103c565b610ffe565b610fe1565b610e68565b610dfc565b610dcf565b610d5c565b610cb5565b610aaa565b61094a565b610918565b61089e565b6106f4565b61064c565b610613565b6105d2565b610520565b3461035a57602036600319011261035a5760043560405163237dfb4760e11b8152336004820152906020826024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa9182156103555761032492610310915f91610326575b506124ff565b61031f60665482811614612515565b61467d565b005b610348915060203d60201161034e575b61034081836103ad565b8101906124df565b5f61030a565b503d610336565b6124f4565b5f80fd5b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b0382111761038d57604052565b61035e565b608081019081106001600160401b0382111761038d57604052565b90601f801991011681019081106001600160401b0382111761038d57604052565b604051906103de610100836103ad565b565b604051906103de6040836103ad565b604051906103de6060836103ad565b604051906103de60a0836103ad565b906103de60405192836103ad565b60409060e319011261035a576040519061043482610372565b60e4358252610104356020830152565b919082604091031261035a5760405161045c81610372565b6020808294803584520135910152565b9080601f8301121561035a57604051916104876040846103ad565b82906040810192831161035a57905b8282106104a35750505090565b8135815260209182019101610496565b90608060631983011261035a576040516104cc81610372565b60206104e782946104de81606461046c565b845260a461046c565b910152565b919060808382031261035a5760206104e76040519261050a84610372565b60408496610518838261046c565b86520161046c565b3461035a5761012036600319011261035a57600435604036602319011261035a57610578604091825161055281610372565b60243581526044356020820152610568366104b3565b906105723661041b565b92612569565b8251911515825215156020820152f35b5f91031261035a57565b3461035a575f36600319011261035a57602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b3461035a575f36600319011261035a5760cd546040516001600160a01b039091168152602090f35b63ffffffff81160361035a57565b35906103de826105fa565b3461035a57602036600319011261035a5763ffffffff600435610635816105fa565b165f5260cb602052602060405f2054604051908152f35b3461035a57602036600319011261035a5763ffffffff60043561066e816105fa565b165f5260ca602052602060405f2054604051908152f35b6001600160a01b0381160361035a57565b6001600160401b03811161038d5760051b60200190565b90602080835192838152019201905f5b8181106106ca5750505090565b82518452602093840193909201916001016106bd565b9060206106f19281815201906106ad565b90565b3461035a57604036600319011261035a5760043561071181610685565b602435906001600160401b03821161035a573660238301121561035a5781600401359161073d83610696565b9261074b60405194856103ad565b8084526024602085019160051b8301019136831161035a57602401905b82821061078c5761078861077c86866126d3565b604051918291826106e0565b0390f35b60208091833561079b81610685565b815201910190610768565b6001600160401b03811161038d57601f01601f191660200190565b9291926107cd826107a6565b916107db60405193846103ad565b82948184528183011161035a578281602093845f960137010152565b9080602083519182815201916020808360051b8301019401925f915b83831061082257505050505090565b9091929394601f19828203018352855190602080835192838152019201905f905b8082106108625750505060208060019297019301930191939290610813565b909192602060606001926001600160601b0360408851868060a01b03815116845285810151868501520151166040820152019401920190610843565b3461035a57606036600319011261035a576004356108bb81610685565b6024356001600160401b03811161035a573660238201121561035a57610788916108f26109049236906024816004013591016107c1565b604435916108ff836105fa565b612904565b6040519182916020835260208301906107f7565b3461035a575f36600319011261035a5760d1546040516001600160a01b039091168152602090f35b8015150361035a57565b3461035a57602036600319011261035a5760043561096781610940565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916109dc575b506001600160a01b031633036109cd5761032490614b10565b637070f3b160e11b5f5260045ffd5b6109fe915060203d602011610a04575b6109f681836103ad565b81019061278b565b5f6109b4565b503d6109ec565b9080601f8301121561035a578135610a2281610696565b92610a3060405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210610a585750505090565b8135815260209182019101610a4b565b60206040818301928281528451809452019201905f5b818110610a8b5750505090565b82516001600160a01b0316845260209384019390920191600101610a7e565b3461035a57604036600319011261035a57600435610ac781610685565b6024356001600160401b03811161035a57610ae6903690600401610a0b565b610af08151612671565b916001600160a01b03165f5b8251811015610b8d57806020610b15610b3593866126b0565b5160405180948192630a5aec1960e21b8352600483019190602083019252565b0381865afa91821561035557600192610b69915f91610b6f575b50610b5a83886126b0565b6001600160a01b039091169052565b01610afc565b610b87915060203d8111610a04576109f681836103ad565b5f610b4f565b604051806107888682610a68565b9181601f8401121561035a578235916001600160401b03831161035a576020838186019501011161035a57565b90602080835192838152019201905f5b818110610be55750505090565b825163ffffffff16845260209384019390920191600101610bd8565b90602082526060610c4f610c3a610c2484516080602088015260a0870190610bc8565b6020850151868203601f19016040880152610bc8565b6040840151858203601f190184870152610bc8565b910151916080601f1982840301910152815180825260208201916020808360051b8301019401925f915b838310610c8857505050505090565b9091929394602080610ca6600193601f198682030187528951610bc8565b97019301930191939290610c79565b3461035a57608036600319011261035a57600435610cd281610685565b60243590610cdf826105fa565b6044356001600160401b03811161035a57610cfe903690600401610b9b565b91606435926001600160401b03841161035a573660238501121561035a578360040135926001600160401b03841161035a573660248560051b8701011161035a57610788956024610d50960193612e03565b60405191829182610c01565b3461035a575f36600319011261035a5760405163237dfb4760e11b81523360048201526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557610dc7915f9161032657506124ff565b610324614649565b3461035a575f36600319011261035a57602060405167016345785d8a00008152f35b60ff81160361035a57565b3461035a57602036600319011261035a576020600160ff600435610e1f81610df1565b161b806066541614604051908152f35b60206040818301928281528451809452019201905f5b818110610e525750505090565b8251845260209384019390920191600101610e45565b3461035a57606036600319011261035a57600435610e8581610685565b6024356001600160401b03811161035a57610ea4903690600401610a0b565b60443591610eb1836105fa565b6040516361c8a12f60e11b8152906001600160a01b03165f8280610ed9868860048401613257565b0381845afa918215610355575f92610fbd575b50610ef78351612671565b935f5b8451811015610faf57610f0d81866126b0565b5190602083610f29610f1f84896126b0565b5163ffffffff1690565b6040516304ec635160e01b8152600481019590955263ffffffff918216602486015216604484015282606481875afa8015610355576001925f91610f81575b50828060c01b0316610f7a82896126b0565b5201610efa565b610fa2915060203d8111610fa8575b610f9a81836103ad565b810190612d7a565b5f610f68565b503d610f90565b604051806107888882610e2f565b610fda9192503d805f833e610fd281836103ad565b810190612c49565b905f610eec565b3461035a575f36600319011261035a576020606654604051908152f35b3461035a57602036600319011261035a5763ffffffff600435611020816105fa565b165f5260cc602052602060ff60405f2054166040519015158152f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b9080601f8301121561035a57813561111f81610696565b9261112d60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106111555750505090565b602080918335611164816105fa565b815201910190611148565b81601f8201121561035a57803561118581610696565b9261119360405194856103ad565b81845260208085019260061b8401019281841161035a57602001915b8383106111bd575050505090565b60206040916111cc8486610444565b8152019201916111af565b9080601f8301121561035a5781356111ee81610696565b926111fc60405194856103ad565b81845260208085019260051b8201019183831161035a5760208201905b83821061122857505050505090565b81356001600160401b03811161035a5760209161124a87848094880101611108565b815201910190611219565b9190916101808184031261035a5761126b6103ce565b9281356001600160401b03811161035a5781611288918401611108565b845260208201356001600160401b03811161035a57816112a991840161116f565b602085015260408201356001600160401b03811161035a57816112cd91840161116f565b60408501526112df81606084016104ec565b60608501526112f18160e08401610444565b60808501526101208201356001600160401b03811161035a5781611316918401611108565b60a08501526101408201356001600160401b03811161035a578161133b918401611108565b60c08501526101608201356001600160401b03811161035a5761135e92016111d7565b60e0830152565b90602080835192838152019201905f5b8181106113825750505090565b82516001600160601b0316845260209384019390920191600101611375565b3461035a57608036600319011261035a576004356024356001600160401b03811161035a576113d4903690600401610b9b565b90916044356113e2816105fa565b606435926001600160401b03841161035a576114449461140961140f953690600401611255565b93613449565b60405192839260408452602061143082516040808801526080870190611365565b910151848203603f19016060860152611365565b9060208301520390f35b3461035a575f36600319011261035a57611466614cec565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b3461035a575f36600319011261035a57602063ffffffff60c95416604051908152f35b3461035a575f36600319011261035a5760ce546040516001600160a01b039091168152602090f35b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a575f36600319011261035a576033546040516001600160a01b039091168152602090f35b9081608091031261035a5790565b604090602319011261035a57602490565b9081604091031261035a5790565b3461035a5760c036600319011261035a576004356001600160401b03811161035a576115bd903690600401611560565b6115c63661156e565b604036606319011261035a5760a4356001600160401b03811161035a576115f190369060040161116f565b6116696115fd83613cd3565b926116216116198563ffffffff165f5260cb60205260405f2090565b541515613cdd565b6116398463ffffffff165f5260cb60205260405f2090565b5490604051611660816116526064602083019586613d51565b03601f1981018352826103ad565b51902014613d6f565b61169461168e6116878463ffffffff165f5260cc60205260405f2090565b5460ff1690565b15613de1565b6116c36116b56116ac6116a76064613cd3565b613e77565b63ffffffff1690565b63ffffffff43161115613ea9565b6116cd8151612671565b925f5b825181101561170d57806116fc6116e9600193866126b0565b5180515f526020015160205260405f2090565b61170682886126b0565b52016116d0565b5092919091602084019161174561172384613cd3565b6040516117398161165287602083019586613409565b51902060843514613f1b565b61174f8451612671565b947f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316935f5b86518110156117ff578060206117966117b693886126b0565b516040518094819263745dcd7360e11b8352600483019190602083019252565b03818a5afa918215610355576001926117db915f916117e1575b50610b5a838c6126b0565b0161177d565b6117f9915060203d8111610a04576109f681836103ad565b5f6117d0565b508691859161185b61182461182c604085019361181c8587613fa6565b939091613cd3565b9236916107c1565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316612904565b935f905b8551821015611b91575f969596945b61187883896126b0565b5151861015611b81578760208061189c89611896886118be9e6126b0565b516126b0565b510151604051809b81926308f6629d60e31b8352600483019190602083019252565b0381855afa988915610355575f99611b61575b506001975f5b8451811015611b52576119096118fd6118f083886126b0565b516001600160a01b031690565b6001600160a01b031690565b6001600160a01b038c1614611920576001016118d7565b509890975060015f9792975b151514611941575b506001019496959661186e565b97959094969184925f896119ba60ff61199261198c61197e8b611978611a0f9e61197260d15460018060a01b031690565b98613fa6565b90612daf565b356001600160f81b03191690565b60f81c90565b6119ac61199d6103e0565b6001600160a01b039095168552565b1663ffffffff166020830152565b60d0546119d1906118fd906001600160a01b031681565b60405163105dea1f60e21b815282516001600160a01b0316600482015260209092015163ffffffff1660248301529097889190829081906044820190565b03915afa958615610355575f96611b2e575b50611a2c8651612671565b975f5b8951811015611a55578067016345785d8a0000611a4e6001938d6126b0565b5201611a2f565b50999297909896919395611aa060ff611a7861198c61197e8a6119788d8f613fa6565b611a92611a836103fe565b6001600160a01b039098168852565b1663ffffffff166020860152565b60408401526060830152611ab261405b565b608083015260cf54611ace906118fd906001600160a01b031681565b803b1561035a57604051636a669b4160e01b8152925f918491829084908290611afa90600483016140af565b03925af191821561035557600192611b14575b5090611934565b80611b225f611b28936103ad565b80610588565b89611b0d565b611b4b9196503d805f833e611b4381836103ad565b810190613fd8565b948a611a21565b5098909760019097929761192c565b611b7a91995060203d8111610a04576109f681836103ad565b97896118d1565b959695945060019091019061185f565b86611bba611bad8263ffffffff165f5260cc60205260405f2090565b805460ff19166001179055565b63ffffffff3391167fc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec5f80a3005b3461035a575f36600319011261035a5760cf546040516001600160a01b039091168152602090f35b3461035a57608036600319011261035a576004356001600160401b03811161035a57611c40903690600401611560565b611c493661156e565b906064356001600160401b03811161035a57611c69903690600401611255565b60cd549092906001600160a01b03163303611e6557611c8c602083949301613cd3565b91611d78611c9d6040860186613fa6565b929094611cef611caf60608901613cd3565b97604051611cc581611652602082019485614184565b519020611ce8611cd488613cd3565b63ffffffff165f5260ca60205260405f2090565b541461423c565b611d19611d12611cfe87613cd3565b63ffffffff165f5260cb60205260405f2090565b54156142ae565b8363ffffffff431696611d5a611d526116ac7f000000000000000000000000000000000000000000000000000000000000000086613e8f565b89111561430f565b6040516020810190611d70816116528b85614371565b519020613449565b919060ff5f9616955b828110611e03577ff2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58868686611dc3611db76103e0565b63ffffffff9094168452565b60208301526040516020810190611ddf81611652868686614441565b519020611dee611cfe83613cd3565b55611dfe60405192839283614441565b0390a1005b80611e5f611e3b611e36611e2a611e1d60019688516126b0565b516001600160601b031690565b6001600160601b031690565b614381565b611e58611e2a8b611e53611e1d8760208b01516126b0565b6143ad565b11156143d0565b01611d81565b60405162461bcd60e51b815260206004820152601d60248201527f41676772656761746f72206d757374206265207468652063616c6c65720000006044820152606490fd5b3461035a575f36600319011261035a57602060ff609754166040519015158152f35b3461035a575f36600319011261035a5760d0546040516001600160a01b039091168152602090f35b3461035a5760c036600319011261035a57600435611f1181610685565b611f91602435611f2081610685565b604435611f2c81610685565b606435611f3881610685565b60843591611f4583610685565b60a43593611f5285610685565b5f5496611f7760ff60088a901c16158099819a61200f575b8115611fef575b5061446b565b87611f88600160ff195f5416175f55565b611fd8576144ce565b611f9757005b611fa561ff00195f54165f55565b604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498908060208101611dfe565b611fea61010061ff00195f5416175f55565b6144ce565b303b15915081612001575b505f611f71565b60ff1660011490505f611ffa565b600160ff8216109150611f6a565b6040906106f19392815281602082015201906107f7565b3461035a57606036600319011261035a5760043561205181610685565b602435604435612060816105fa565b6120a161206b61264f565b9280612076856126a3565b526040516361c8a12f60e11b81526001600160a01b0386169490925f91849182918760048401613257565b0381875afa93841561035557836120cb6116ac610f1f612100986020975f9161215e575b506126a3565b92604051968794859384936304ec635160e01b85526004850163ffffffff604092959493606083019683521660208201520152565b03915afa80156103555761212f925f9161213f575b506001600160c01b03169261212984614d8c565b90612904565b906107886040519283928361201d565b612158915060203d602011610fa857610f9a81836103ad565b5f612115565b61217291503d805f833e610fd281836103ad565b5f6120c5565b3461035a575f36600319011261035a576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461035a57606036600319011261035a576004356001600160401b03811161035a576121ec90369060040161157f565b602435906121f9826105fa565b6044356001600160401b03811161035a57612218903690600401610b9b565b60ce5491939092916001600160a01b0316330361230257610324936122ec9361226c61227393612251612249614537565b96369061457d565b86524363ffffffff16602087015263ffffffff166060860152565b36916107c1565b6040820152604051602081019061228e8161165285856145d7565b5190206122a3611cd460c95463ffffffff1690565b5560c95463ffffffff16907fa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd604051806122e463ffffffff861694826145d7565b0390a2613e5f565b63ffffffff1663ffffffff1960c954161760c955565b60405162461bcd60e51b815260206004820152602160248201527f5461736b2067656e657261746f72206d757374206265207468652063616c6c656044820152603960f91b6064820152608490fd5b3461035a57602036600319011261035a5760043561236e81610685565b612376614cec565b6001600160a01b0381161561238e5761032490614d44565b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b3461035a575f36600319011261035a57602060405160648152f35b3461035a57602036600319011261035a5760043560405163755b36bd60e11b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f916124c0575b506001600160a01b031633036124b15761247f606654198219811614612515565b806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63794821ff60e01b5f5260045ffd5b6124d9915060203d602011610a04576109f681836103ad565b5f61245e565b9081602091031261035a57516106f181610940565b6040513d5f823e3d90fd5b1561250657565b631d77d47760e21b5f5260045ffd5b1561251c57565b63c61dca5d60e01b5f5260045ffd5b634e487b7160e01b5f52603260045260245ffd5b9060028110156125505760051b0190565b61252b565b634e487b7160e01b5f52601260045260245ffd5b61264561262261264b9561261c61261585875160208901518a515160208c51015160208d016020815151915101519189519360208b0151956040519760208901998a5260208a015260408901526060880152608087015260a086015260c085015260e08401526101008301526125ec81610120840103601f1981018352826103ad565b5190207f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001900690565b80966146f3565b90614739565b9261261c61263761263161479b565b94614892565b916126406149ae565b6146f3565b916149e2565b9091565b6040805190919061266083826103ad565b6001815291601f1901366020840137565b9061267b82610696565b61268860405191826103ad565b8281528092612699601f1991610696565b0190602036910137565b8051156125505760200190565b80518210156125505760209160051b010190565b9081602091031261035a575190565b9190916126e08351612671565b925f5b8151811015612786578060206126ff6118f061272894866126b0565b6040516309aa152760e11b81526001600160a01b03909116600482015292839081906024820190565b03816001600160a01b0388165afa8015610355576001925f91612758575b5061275182886126b0565b52016126e3565b612779915060203d811161277f575b61277181836103ad565b8101906126c4565b5f612746565b503d612767565b505050565b9081602091031261035a57516106f181610685565b906127aa82610696565b6127b760405191826103ad565b828152602081936127ca601f1991610696565b0191015f5b8281106127db57505050565b6060828201526020016127cf565b908151811015612550570160200190565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161282d81610696565b9261283b60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106128635750505090565b8151815260209182019101612856565b9061287d82610696565b61288a60405191826103ad565b828152809261289b601f1991610696565b015f5b8181106128aa57505050565b6040519060608201918083106001600160401b0384111761038d576020926040525f81525f838201525f60408201528282860101520161289e565b9081602091031261035a57516001600160601b038116810361035a5790565b604051636830483560e01b815293919291906001600160a01b0316602085600481845afa948515610355575f95612c04575b50604051634f4c91e160e11b815294602086600481855afa918215610355576004965f93612be2575b5060209060405197888092632efa2ca360e11b82525afa958615610355575f96612bc1575b5061299285939295516127a0565b945f935b8051851015612bb7576129bd61198c6129af87846127e9565b516001600160f81b03191690565b604051638902624560e01b815260ff8216600482015263ffffffff88166024820152909490925f846044816001600160a01b0385165afa938415610355575f94612b93575b50612a0d8451612873565b612a17888b6126b0565b52612a22878a6126b0565b505f5b8451811015612b8257806020612a3e612a6093886126b0565b518d60405180809681946308f6629d60e31b8352600483019190602083019252565b03916001600160a01b03165afa918215610355575f92612b62575b50612a8681876126b0565b518a60208a612a95858b6126b0565b5160405163fa28c62760e01b8152600481019190915260ff91909116602482015263ffffffff929092166044830152816064816001600160a01b038d165afa93841561035557612b198c8f612b14600198612b2b9789975f92612b32575b50612aff611a836103ef565b60208701526001600160601b03166040860152565b6126b0565b5190612b2583836126b0565b526126b0565b5001612a25565b612b5491925060203d8111612b5b575b612b4c81836103ad565b8101906128e5565b905f612af3565b503d612b42565b612b7b91925060203d8111610a04576109f681836103ad565b905f612a7b565b506001909601959094509150612996565b612bb09194503d805f833e612ba881836103ad565b8101906127fa565b925f612a02565b5050509350505090565b612bdb91965060203d602011610a04576109f681836103ad565b945f612984565b6020919350612bfd90823d8411610a04576109f681836103ad565b929061295f565b612c1e91955060203d602011610a04576109f681836103ad565b935f612936565b60405190612c3282610392565b606080838181528160208201528160408201520152565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a578151612c7c81610696565b92612c8a60405194856103ad565b81845260208085019260051b82010192831161035a57602001905b828210612cb25750505090565b602080918351612cc1816105fa565b815201910190612ca5565b63ffffffff909116815260406020820181905281018390526001600160fb1b03831161035a5760609260051b809284830137010190565b908060209392818452848401375f828201840152601f01601f1916010190565b60409063ffffffff6106f195931681528160208201520191612d03565b634e487b7160e01b5f52601160045260245ffd5b60ff1660ff8114612d655760010190565b612d40565b91908110156125505760051b0190565b9081602091031261035a57516001600160c01b038116810361035a5790565b15612da057565b6325ec6c1f60e01b5f5260045ffd5b90821015612550570190565b9081602091031261035a57516106f1816105fa565b5f198114612d655760010190565b91612dfc60209263ffffffff92969596604086526040860191612d03565b9416910152565b9593949592909192612e13612c25565b50604051636830483560e01b8152936001600160a01b03919091169190602085600481865afa948515610355575f95613236575b50612e50612c25565b946040516361c8a12f60e11b81525f8180612e708d8d8b60048501612ccc565b0381885afa908115610355575f9161321c575b5086526040516340e03a8160e11b81526001600160a01b039190911692905f8180612eb385878b60048501612d23565b0381875afa908115610355575f91613202575b506040870152612ed5816127a0565b9860608701998a525f5b60ff81168381101561314d57885f612f08838f612efb88612671565b905190612b2583836126b0565b505f8a868f5b818410612f85575050505090508c612f2582612671565b915f5b818110612f5257505091612f4791612f4d94935190612b2583836126b0565b50612d54565b612edf565b80612f7f612f6a610f1f6001946118968a89516126b0565b612f7483886126b0565b9063ffffffff169052565b01612f28565b610f1f84612f9a8160209695612fa295612d6a565b3597516126b0565b6040516304ec635160e01b8152600481019690965263ffffffff9182166024870152166044850152836064818d5afa801561035557888f888a918f94613039600161302c81938d809d5f92613121575b5061198c61197e6130169261300f878060c01b0386161515612d99565b8b8d612daf565b6001600160c01b0391821660ff919091161c1690565b166001600160c01b031690565b14613055575b5050505050600191925001908a918a868f612f0e565b859761307793613070602097999861198c9561197e95612d6a565b3595612daf565b60405163dd9846b960e01b8152600481019290925260ff16602482015263ffffffff939093166044840152826064818c5afa908115610355578f6130d5906130da9383886001975f936130e9575b5061189690612f749394516126b0565b612dd0565b905082918a888f888a9161303f565b612f749350906131126118969260203d811161311a575b61310a81836103ad565b810190612dbb565b9350906130c5565b503d613100565b61301691925061197e61314461198c9260203d8111610fa857610f9a81836103ad565b93925050612ff2565b505050929095975060049496506020915060405194858092632efa2ca360e11b82525afa908115610355576131a3945f9485936131e1575b5060405163354952a360e21b81529586948593849360048501612dde565b03916001600160a01b03165afa908115610355575f916131c7575b50602082015290565b6131db91503d805f833e610fd281836103ad565b5f6131be565b6131fb91935060203d602011610a04576109f681836103ad565b915f613185565b61321691503d805f833e610fd281836103ad565b5f612ec6565b61323091503d805f833e610fd281836103ad565b5f612e83565b61325091955060203d602011610a04576109f681836103ad565b935f612e47565b60409063ffffffff6106f1949316815281602082015201906106ad565b6040519061328182610372565b60606020838281520152565b1561329457565b62f8202d60e51b5f5260045ffd5b156132a957565b6343714afd60e01b5f5260045ffd5b156132bf57565b635f832f4160e01b5f5260045ffd5b156132d557565b634b874f4560e01b5f5260045ffd5b9081602091031261035a57516106f181610df1565b5f19810191908211612d6557565b1561330e57565b633fdc650560e21b5f5260045ffd5b9060018201809211612d6557565b9060028201809211612d6557565b9060038201809211612d6557565b9060048201809211612d6557565b9060058201809211612d6557565b91908201809211612d6557565b1561337757565b63affc5edb60e01b5f5260045ffd5b9081602091031261035a575167ffffffffffffffff198116810361035a5790565b156133ae57565b63e1310aed60e01b5f5260045ffd5b906001600160601b03809116911603906001600160601b038211612d6557565b156133e457565b6367988d3360e01b5f5260045ffd5b156133fa57565b63ab1b236b60e01b5f5260045ffd5b60049163ffffffff60e01b9060e01b1681520160208251919201905f5b8181106134335750505090565b8251845260209384019390920191600101613426565b949392909193613457613274565b5061346385151561328d565b604084015151851480613cc5575b80613cb7575b80613ca9575b613486906132a2565b613498602085015151855151146132b8565b6134af63ffffffff431663ffffffff8416106132ce565b6134b76103e0565b5f81525f6020820152926134c9613274565b6134d287612671565b60208201526134e087612671565b81526134ea613274565b926134f9602088015151612671565b8452613509602088015151612671565b602085810191909152604051639aa1653d60e01b815290816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa801561035557613572915f91613c7a575b5061356d368b876107c1565b614b4e565b985f965b602089015180518910156136d1576020886135c6610f1f8c6135be8f96868e6135a36116e98680956126b0565b6135b084848401516126b0565b528261369e575b01516126b0565b5195516126b0565b6040516304ec635160e01b8152600481019490945263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa9182156103555761261c8a6136738f61366c8f8460208f926136639361365b8460019e6136799e5f91613681575b508f8060c01b031692516126b0565b5201516126b0565b51938d516126b0565b5116614b79565b90614baa565b970196613576565b6136989150863d8111610fa857610f9a81836103ad565b5f61364c565b6136cc6136ae84848401516126b0565b516136c5848401516136bf876132f9565b906126b0565b5110613307565b6135b7565b509095979496506136e6919893929950614c67565b916136f360975460ff1690565b908115613c72576040516318891fd760e31b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355575f91613c53575b5091905b5f925b8184106137a45750505050509261378b61378661377f61379e95856116529860806060602099015192015192612569565b91906133dd565b6133f3565b0151604051928391602083019586613409565b51902090565b92989596909399919794878b888c888d613b4d575b610f1f8260a06137f961198c61197e84613801976137f36137e56116e98f9c604060209f9e01516126b0565b67ffffffffffffffff191690565b9b612daf565b9701516126b0565b604051631a2f32ab60e21b815260ff95909516600486015263ffffffff9182166024860152166044840152826064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa908115610355576138c5610f1f8f958f906138bd8f978f96848f6138b760c0966138b0848f60209f906135b761197e9960409361198c9c5f91613b1f575b5067ffffffffffffffff199182169116146133a7565b5190614739565b9c612daf565b9601516126b0565b604051636414a62b60e11b815260ff94909416600485015263ffffffff9182166024850152166044830152816064816001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000165afa90811561035557613952918c8f925f92613afb575b506020613944929301516126b0565b906001600160601b03169052565b6139728c6139448c61396b611e1d8260208601516126b0565b92516126b0565b5f985f5b60208a015151811015613ae2578b8d6139b4896139a761198c61197e868f8961399f91516126b0565b519487612daf565b60ff161c60019081161490565b6139c3575b5050600101613976565b8a8a613a45859f948f96866118968f9360e06139fc610f1f9560206139f461198c61197e839f613a059c8991612daf565b9a01516126b0565b519b01516126b0565b60405163795f4a5760e11b815260ff909316600484015263ffffffff93841660248401526044830196909652919094166064850152839081906084820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610355578f613ab1908f936001959486955f92613abc575b50613aab61394492935193613aa6611e1d84876126b0565b6133bd565b926126b0565b019a90508b8d6139b9565b6139449250613adb613aab9160203d8111612b5b57612b4c81836103ad565b9250613a8e565b5093919796996001919699509a94929a0192919061374e565b6139449250613b18602091823d8111612b5b57612b4c81836103ad565b9250613935565b6020613b4092503d8111613b46575b613b3881836103ad565b810190613386565b5f61389a565b503d613b2e565b613b8a9450613b67925061198c9161197e91602095612daf565b60405163124d062160e11b815260ff909116600482015291829081906024820190565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa8015610355576020896138018f938f60a08f9761198c61197e8f8f906137f36137e56116e98f60408b96918f8893610f1f9f613c0e90613c14936137f99f5f92613c2a575b5063ffffffff809116931690613363565b11613370565b50505050505097505050505050929350506137b9565b602063ffffffff9293508291613c4b913d811161277f5761277181836103ad565b929150613bfd565b613c6c915060203d60201161311a5761310a81836103ad565b5f613747565b5f919061374b565b613c9c915060203d602011613ca2575b613c9481836103ad565b8101906132e4565b5f613561565b503d613c8a565b5060e084015151851461347d565b5060c0840151518514613477565b5060a0840151518514613471565b356106f1816105fa565b15613ce457565b60405162461bcd60e51b815260206004820152602160248201527f5461736b206861736e2774206265656e20726573706f6e64656420746f2079656044820152601d60fa1b6064820152608490fd5b6020809163ffffffff8135613d47816105fa565b1684520135910152565b9160406103de929493613d68816080810197613d33565b0190613d33565b15613d7657565b60405162461bcd60e51b815260206004820152603d60248201527f5461736b20726573706f6e736520646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b15613de857565b60405162461bcd60e51b815260206004820152604360248201527f54686520726573706f6e736520746f2074686973207461736b2068617320616c60448201527f7265616479206265656e206368616c6c656e676564207375636365737366756c606482015262363c9760e91b608482015260a490fd5b63ffffffff60019116019063ffffffff8211612d6557565b63ffffffff60649116019063ffffffff8211612d6557565b9063ffffffff8091169116019063ffffffff8211612d6557565b15613eb057565b60405162461bcd60e51b815260206004820152603760248201527f546865206368616c6c656e676520706572696f6420666f72207468697320746160448201527f736b2068617320616c726561647920657870697265642e0000000000000000006064820152608490fd5b15613f2257565b60405162461bcd60e51b815260206004820152605060248201527f546865207075626b657973206f66206e6f6e2d7369676e696e67206f7065726160448201527f746f727320737570706c69656420627920746865206368616c6c656e6765722060648201526f30b932903737ba1031b7b93932b1ba1760811b608482015260a490fd5b903590601e198136030182121561035a57018035906001600160401b03821161035a5760200191813603831361035a57565b60208183031261035a578051906001600160401b03821161035a57019080601f8301121561035a57815161400b81610696565b9261401960405194856103ad565b81845260208085019260051b82010192831161035a57602001905b8282106140415750505090565b60208091835161405081610685565b815201910190614034565b6040519061406a6040836103ad565b601282527139b630b9b42fba3432afb7b832b930ba37b960711b6020830152565b805180835260209291819084018484015e5f828201840152601f01601f1916010190565b91906020835260c083019260018060a01b03825116602082015263ffffffff602083015116604082015260408201519360a060608301528451809152602060e083019501905f5b81811061413457505050608061411f6106f194956060850151601f1985830301848601526106ad565b9201519060a0601f198285030191015261408b565b82516001600160a01b03168752602096870196909201916001016140f6565b9035601e198236030181121561035a5701602081359101916001600160401b03821161035a57813603831361035a57565b60208152813590603e198336030182121561035a576080614231606061422a6141f0876106f197018560208801526141de6141d36141c28380614153565b604060a08c015260e08b0191612d03565b916020810190614153565b888303609f190160c08a015290612d03565b61420c6141ff60208a01610608565b63ffffffff166040880152565b6142196040890189614153565b878303601f19018589015290612d03565b9501610608565b63ffffffff16910152565b1561424357565b60405162461bcd60e51b815260206004820152603d60248201527f737570706c696564207461736b20646f6573206e6f74206d617463682074686560448201527f206f6e65207265636f7264656420696e2074686520636f6e74726163740000006064820152608490fd5b156142b557565b60405162461bcd60e51b815260206004820152602c60248201527f41676772656761746f722068617320616c726561647920726573706f6e64656460448201526b20746f20746865207461736b60a01b6064820152608490fd5b1561431657565b60405162461bcd60e51b815260206004820152602d60248201527f41676772656761746f722068617320726573706f6e64656420746f207468652060448201526c7461736b20746f6f206c61746560981b6064820152608490fd5b6040810192916103de9190613d33565b90606482029180830460641490151715612d6557565b90600682029180830460061490151715612d6557565b906001600160601b03809116911602906001600160601b038216918203612d6557565b156143d757565b608460405162461bcd60e51b815260206004820152604060248201527f5369676e61746f7269657320646f206e6f74206f776e206174206c656173742060448201527f7468726573686f6c642070657263656e74616765206f6620612071756f72756d6064820152fd5b9092916020606091614457846080810197613d33565b63ffffffff81511660408501520151910152565b1561447257565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b6144d790614d44565b60cd80546001600160a01b03199081166001600160a01b039384161790915560ce805482169383169390931790925560d0805483169382169390931790925560cf805482169383169390931790925560d180549092169216919091179055565b6040519061454482610392565b5f606083614550613274565b81528260208201528160408201520152565b9080601f8301121561035a578160206106f1933591016107c1565b919060408382031261035a576040519061459682610372565b819380356001600160401b03811161035a57826145b4918301614562565b83526020810135916001600160401b03831161035a576020926104e79201614562565b60208152608063ffffffff606061463f61461e8651856020880152602061460a8251604060a08b015260e08a019061408b565b910151878203609f190160c089015261408b565b8360208801511660408701526040870151601f19878303018488015261408b565b9401511691015290565b5f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2565b604051906146bc82610372565b5f6020838281520152565b604051906101806146d881846103ad565b368337565b604051906146ec6020836103ad565b6020368337565b919060409060606147026146af565b948592602085519261471485856103ad565b8436853780518452015160208301528482015260076107cf195a01fa1561473757565bfe5b60209291608060409261474a6146af565b9586938186519361475b86866103ad565b85368637805185520151828401528051868401520151606082015260066107cf195a01fa8015614737571561478c57565b63d4b68fd760e01b5f5260045ffd5b6040516147a781610372565b60409081516147b683826103ad565b82368237815260208251916147cb84846103ad565b83368437015280516147dd82826103ad565b7f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c281527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed602082015281519061483383836103ad565b7f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d6020830152614888835193846103ad565b8252602082015290565b5f516020614ecf5f395f51905f52906148a96146af565b505f919006602060c0835b6149a9575f935f516020614ecf5f395f51905f52600381868181800909086040516148df85826103ad565b843682378481856040516148f382826103ad565b813682378381528360208201528360408201528560608201527f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f5260808201525f516020614ecf5f395f51905f5260a082015260056107cf195a01fa80156147375761495d90614eb8565b51916149a9575f516020614ecf5f395f51905f528280091461499457505f516020614ecf5f395f51905f5260015f940892936148b4565b929350506149a06103e0565b92835282015290565b612555565b6149b66146af565b506040516149c381610372565b600181526002602082015290565b90600c8110156125505760051b0190565b939290916149f0604061040d565b9485526020850152614a02604061040d565b9182526020820152614a126146c7565b925f5b60028110614a3f57505050602061018092614a2e6146dd565b93849160086201d4c0fa9151151590565b80614a4b600192614397565b614a55828561253f565b5151614a6182896149d1565b526020614a6e838661253f565b510151614a83614a7d8361331d565b896149d1565b52614a8e828661253f565b515151614a9d614a7d8361332b565b52614ab3614aab838761253f565b515160200190565b51614ac0614a7d83613339565b526020614acd838761253f565b51015151614add614a7d83613347565b52614b09614b03614afc6020614af3868a61253f565b51015160200190565b5192613355565b886149d1565b5201614a15565b60207f40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc91151560ff196097541660ff821617609755604051908152a1565b906001614b5c60ff93614e40565b928392161b1115614b6a5790565b63ca95733360e01b5f5260045ffd5b805f915b614b85575090565b5f198101818111612d655761ffff9116911661ffff8114612d65576001019080614b7d565b90614bb36146af565b5061ffff811690610200821015614c585760018214614c5357614bd46103e0565b5f81525f602082015292906001905f925b61ffff8316851015614bf957505050505090565b600161ffff831660ff86161c811614614c33575b6001614c29614c1e8360ff94614739565b9460011b61fffe1690565b9401169291614be5565b946001614c29614c1e614c488960ff95614739565b989350505050614c0d565b505090565b637fc4ea7d60e11b5f5260045ffd5b614c6f6146af565b50805190811580614ce0575b15614c9c575050604051614c906040826103ad565b5f81525f602082015290565b60205f516020614ecf5f395f51905f52910151065f516020614ecf5f395f51905f52035f516020614ecf5f395f51905f528111612d65576040519161488883610372565b50602081015115614c7b565b6033546001600160a01b03163303614d0057565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b61ffff614d9882614b79565b16614da2816107a6565b90614db060405192836103ad565b808252614dbf601f19916107a6565b013660208301375f5f5b8251821080614e1f575b15614e18576001811b8416614df1575b614dec90612dd0565b614dc9565b906001614dec9160ff60f81b8460f81b165f1a614e0e82876127e9565b5301919050614de3565b5050905090565b506101008110614dd3565b15614e3157565b631019106960e31b5f5260045ffd5b90610100825111614ea957815115614ea457602082015160019060f81c81901b5b8351821015614e9f57600190614e8a614e8061198c6129af86896127e9565b60ff600191161b90565b90614e96818311614e2a565b17910190614e61565b925050565b5f9150565b637da54e4760e11b5f5260045ffd5b15614ebf57565b63d51edae360e01b5f5260045ffdfe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a2646970667358221220251991cf2477ae5902fbdd3d7fdcf158dfc6bc36bf12b1911edaecb9fdf6cfa064736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x13d9\xDD\x14a\x02\x9AW\x80c\x17\x1F\x1D[\x14a\x02\x95W\x80c\x1A\xD41\x89\x14a\x01\xE6W\x80c$Z{\xFC\x14a\x02\x90W\x80c,\xB2#\xD5\x14a\x02\x8BW\x80c-\x89\xF6\xFC\x14a\x02\x86W\x80c1\xB3k\xD9\x14a\x02\x81W\x80c5c\xB0\xD1\x14a\x02|W\x80c9\x98\xFD\xD3\x14a\x02wW\x80cAl~^\x14a\x02rW\x80cM+W\xFE\x14a\x02mW\x80cOs\x9Ft\x14a\x02hW\x80cY\\jg\x14a\x02cW\x80cZ-\x7F\x02\x14a\x02^W\x80cZ\xC8j\xB7\x14a\x02YW\x80c\\\x15Vb\x14a\x02TW\x80c\\\x97Z\xBB\x14a\x02OW\x80c]\xEC\xC3\xF5\x14a\x02JW\x80c]\xF4YF\x14a\x02EW\x80ch0H5\x14a\x02@W\x80cm\x14\xA9\x87\x14a\x02;W\x80cn\xFBF6\x14a\x026W\x80cqP\x18\xA6\x14a\x021W\x80cr\xD1\x8E\x8D\x14a\x02\"W\x80cz\xFA\x1E\xED\x14a\x02,W\x80c\x88o\x11\x95\x14a\x02'W\x80c\x8B\0\xCE|\x14a\x02\"W\x80c\x8D\xA5\xCB[\x14a\x02\x1DW\x80c\x92\xA5\x18$\x14a\x02\x18W\x80c\x9B)\x0E\x98\x14a\x02\x13W\x80c\xB7\x029I\x14a\x02\x0EW\x80c\xB9\x8D\t\x08\x14a\x02\tW\x80c\xCA\x8A\xA7\xC7\x14a\x02\x04W\x80c\xCC*\x9A[\x14a\x01\xFFW\x80c\xCE\xFD\xC1\xD4\x14a\x01\xFAW\x80c\xDF\\\xF7#\x14a\x01\xF5W\x80c\xEF\x02\x9D\xBC\x14a\x01\xF0W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xEBW\x80c\xF5\xC9\x89\x9D\x14a\x01\xE6W\x80c\xF6<[\xAB\x14a\x01\xE1Wc\xFA\xBC\x1C\xBC\x14a\x01\xDCW_\x80\xFD[a#\xFDV[a#\xE2V[a\x05\x92V[a#QV[a!\xBCV[a!xV[a 4V[a\x1E\xF4V[a\x1E\xCCV[a\x1E\xAAV[a\x1C\x10V[a\x1B\xE8V[a\x15\x8DV[a\x158V[a\x14\xA9V[a\x14\xF4V[a\x14\xCCV[a\x14NV[a\x13\xA1V[a\x10\xC4V[a\x10\x80V[a\x10<V[a\x0F\xFEV[a\x0F\xE1V[a\x0EhV[a\r\xFCV[a\r\xCFV[a\r\\V[a\x0C\xB5V[a\n\xAAV[a\tJV[a\t\x18V[a\x08\x9EV[a\x06\xF4V[a\x06LV[a\x06\x13V[a\x05\xD2V[a\x05 V[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x82`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UWa\x03$\x92a\x03\x10\x91_\x91a\x03&W[Pa$\xFFV[a\x03\x1F`fT\x82\x81\x16\x14a%\x15V[aF}V[\0[a\x03H\x91P` =` \x11a\x03NW[a\x03@\x81\x83a\x03\xADV[\x81\x01\x90a$\xDFV[_a\x03\nV[P=a\x036V[a$\xF4V[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[a\x03^V[`\x80\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x03\x8DW`@RV[`@Q\x90a\x03\xDEa\x01\0\x83a\x03\xADV[V[`@Q\x90a\x03\xDE`@\x83a\x03\xADV[`@Q\x90a\x03\xDE``\x83a\x03\xADV[`@Q\x90a\x03\xDE`\xA0\x83a\x03\xADV[\x90a\x03\xDE`@Q\x92\x83a\x03\xADV[`@\x90`\xE3\x19\x01\x12a\x03ZW`@Q\x90a\x044\x82a\x03rV[`\xE45\x82Ra\x01\x045` \x83\x01RV[\x91\x90\x82`@\x91\x03\x12a\x03ZW`@Qa\x04\\\x81a\x03rV[` \x80\x82\x94\x805\x84R\x015\x91\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW`@Q\x91a\x04\x87`@\x84a\x03\xADV[\x82\x90`@\x81\x01\x92\x83\x11a\x03ZW\x90[\x82\x82\x10a\x04\xA3WPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\x04\x96V[\x90`\x80`c\x19\x83\x01\x12a\x03ZW`@Qa\x04\xCC\x81a\x03rV[` a\x04\xE7\x82\x94a\x04\xDE\x81`da\x04lV[\x84R`\xA4a\x04lV[\x91\x01RV[\x91\x90`\x80\x83\x82\x03\x12a\x03ZW` a\x04\xE7`@Q\x92a\x05\n\x84a\x03rV[`@\x84\x96a\x05\x18\x83\x82a\x04lV[\x86R\x01a\x04lV[4a\x03ZWa\x01 6`\x03\x19\x01\x12a\x03ZW`\x045`@6`#\x19\x01\x12a\x03ZWa\x05x`@\x91\x82Qa\x05R\x81a\x03rV[`$5\x81R`D5` \x82\x01Ra\x05h6a\x04\xB3V[\x90a\x05r6a\x04\x1BV[\x92a%iV[\x82Q\x91\x15\x15\x82R\x15\x15` \x82\x01R\xF3[_\x91\x03\x12a\x03ZWV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCDT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[c\xFF\xFF\xFF\xFF\x81\x16\x03a\x03ZWV[5\x90a\x03\xDE\x82a\x05\xFAV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x065\x81a\x05\xFAV[\x16_R`\xCB` R` `@_ T`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x06n\x81a\x05\xFAV[\x16_R`\xCA` R` `@_ T`@Q\x90\x81R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03ZWV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x05\x1B` \x01\x90V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x06\xCAWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x06\xBDV[\x90` a\x06\xF1\x92\x81\x81R\x01\x90a\x06\xADV[\x90V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\x07\x11\x81a\x06\x85V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW6`#\x83\x01\x12\x15a\x03ZW\x81`\x04\x015\x91a\x07=\x83a\x06\x96V[\x92a\x07K`@Q\x94\x85a\x03\xADV[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x03ZW`$\x01\x90[\x82\x82\x10a\x07\x8CWa\x07\x88a\x07|\x86\x86a&\xD3V[`@Q\x91\x82\x91\x82a\x06\xE0V[\x03\x90\xF3[` \x80\x91\x835a\x07\x9B\x81a\x06\x85V[\x81R\x01\x91\x01\x90a\x07hV[`\x01`\x01`@\x1B\x03\x81\x11a\x03\x8DW`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x92\x91\x92a\x07\xCD\x82a\x07\xA6V[\x91a\x07\xDB`@Q\x93\x84a\x03\xADV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x03ZW\x82\x81` \x93\x84_\x96\x017\x01\x01RV[\x90\x80` \x83Q\x91\x82\x81R\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x08\"WPPPPP\x90V[\x90\x91\x92\x93\x94`\x1F\x19\x82\x82\x03\x01\x83R\x85Q\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_\x90[\x80\x82\x10a\x08bWPPP` \x80`\x01\x92\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x08\x13V[\x90\x91\x92` ```\x01\x92`\x01`\x01``\x1B\x03`@\x88Q\x86\x80`\xA0\x1B\x03\x81Q\x16\x84R\x85\x81\x01Q\x86\x85\x01R\x01Q\x16`@\x82\x01R\x01\x94\x01\x92\x01\x90a\x08CV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x08\xBB\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW6`#\x82\x01\x12\x15a\x03ZWa\x07\x88\x91a\x08\xF2a\t\x04\x926\x90`$\x81`\x04\x015\x91\x01a\x07\xC1V[`D5\x91a\x08\xFF\x83a\x05\xFAV[a)\x04V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x07\xF7V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD1T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x80\x15\x15\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a\tg\x81a\t@V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a\t\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\t\xCDWa\x03$\x90aK\x10V[cpp\xF3\xB1`\xE1\x1B_R`\x04_\xFD[a\t\xFE\x91P` =` \x11a\n\x04W[a\t\xF6\x81\x83a\x03\xADV[\x81\x01\x90a'\x8BV[_a\t\xB4V[P=a\t\xECV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\n\"\x81a\x06\x96V[\x92a\n0`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\nXWPPP\x90V[\x815\x81R` \x91\x82\x01\x91\x01a\nKV[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\n\x8BWPPP\x90V[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\n~V[4a\x03ZW`@6`\x03\x19\x01\x12a\x03ZW`\x045a\n\xC7\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\n\xE6\x906\x90`\x04\x01a\n\x0BV[a\n\xF0\x81Qa&qV[\x91`\x01`\x01`\xA0\x1B\x03\x16_[\x82Q\x81\x10\x15a\x0B\x8DW\x80` a\x0B\x15a\x0B5\x93\x86a&\xB0V[Q`@Q\x80\x94\x81\x92c\nZ\xEC\x19`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x86Z\xFA\x91\x82\x15a\x03UW`\x01\x92a\x0Bi\x91_\x91a\x0BoW[Pa\x0BZ\x83\x88a&\xB0V[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90RV[\x01a\n\xFCV[a\x0B\x87\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x0BOV[`@Q\x80a\x07\x88\x86\x82a\nhV[\x91\x81`\x1F\x84\x01\x12\x15a\x03ZW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x83\x81\x86\x01\x95\x01\x01\x11a\x03ZWV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x0B\xE5WPPP\x90V[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0B\xD8V[\x90` \x82R``a\x0COa\x0C:a\x0C$\x84Q`\x80` \x88\x01R`\xA0\x87\x01\x90a\x0B\xC8V[` \x85\x01Q\x86\x82\x03`\x1F\x19\x01`@\x88\x01Ra\x0B\xC8V[`@\x84\x01Q\x85\x82\x03`\x1F\x19\x01\x84\x87\x01Ra\x0B\xC8V[\x91\x01Q\x91`\x80`\x1F\x19\x82\x84\x03\x01\x91\x01R\x81Q\x80\x82R` \x82\x01\x91` \x80\x83`\x05\x1B\x83\x01\x01\x94\x01\x92_\x91[\x83\x83\x10a\x0C\x88WPPPPP\x90V[\x90\x91\x92\x93\x94` \x80a\x0C\xA6`\x01\x93`\x1F\x19\x86\x82\x03\x01\x87R\x89Qa\x0B\xC8V[\x97\x01\x93\x01\x93\x01\x91\x93\x92\x90a\x0CyV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045a\x0C\xD2\x81a\x06\x85V[`$5\x90a\x0C\xDF\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0C\xFE\x906\x90`\x04\x01a\x0B\x9BV[\x91`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`#\x85\x01\x12\x15a\x03ZW\x83`\x04\x015\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZW6`$\x85`\x05\x1B\x87\x01\x01\x11a\x03ZWa\x07\x88\x95`$a\rP\x96\x01\x93a.\x03V[`@Q\x91\x82\x91\x82a\x0C\x01V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa\r\xC7\x91_\x91a\x03&WPa$\xFFV[a\x03$aFIV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Qg\x01cEx]\x8A\0\0\x81R\xF3[`\xFF\x81\x16\x03a\x03ZWV[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW` `\x01`\xFF`\x045a\x0E\x1F\x81a\r\xF1V[\x16\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[` `@\x81\x83\x01\x92\x82\x81R\x84Q\x80\x94R\x01\x92\x01\x90_[\x81\x81\x10a\x0ERWPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x0EEV[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a\x0E\x85\x81a\x06\x85V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x0E\xA4\x906\x90`\x04\x01a\n\x0BV[`D5\x91a\x0E\xB1\x83a\x05\xFAV[`@Qca\xC8\xA1/`\xE1\x1B\x81R\x90`\x01`\x01`\xA0\x1B\x03\x16_\x82\x80a\x0E\xD9\x86\x88`\x04\x84\x01a2WV[\x03\x81\x84Z\xFA\x91\x82\x15a\x03UW_\x92a\x0F\xBDW[Pa\x0E\xF7\x83Qa&qV[\x93_[\x84Q\x81\x10\x15a\x0F\xAFWa\x0F\r\x81\x86a&\xB0V[Q\x90` \x83a\x0F)a\x0F\x1F\x84\x89a&\xB0V[Qc\xFF\xFF\xFF\xFF\x16\x90V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x95\x90\x95Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81\x87Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a\x0F\x81W[P\x82\x80`\xC0\x1B\x03\x16a\x0Fz\x82\x89a&\xB0V[R\x01a\x0E\xFAV[a\x0F\xA2\x91P` =\x81\x11a\x0F\xA8W[a\x0F\x9A\x81\x83a\x03\xADV[\x81\x01\x90a-zV[_a\x0FhV[P=a\x0F\x90V[`@Q\x80a\x07\x88\x88\x82a\x0E/V[a\x0F\xDA\x91\x92P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[\x81\x01\x90a,IV[\x90_a\x0E\xECV[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `fT`@Q\x90\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZWc\xFF\xFF\xFF\xFF`\x045a\x10 \x81a\x05\xFAV[\x16_R`\xCC` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x11\x1F\x81a\x06\x96V[\x92a\x11-`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a\x11UWPPP\x90V[` \x80\x91\x835a\x11d\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a\x11HV[\x81`\x1F\x82\x01\x12\x15a\x03ZW\x805a\x11\x85\x81a\x06\x96V[\x92a\x11\x93`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x06\x1B\x84\x01\x01\x92\x81\x84\x11a\x03ZW` \x01\x91[\x83\x83\x10a\x11\xBDWPPPP\x90V[` `@\x91a\x11\xCC\x84\x86a\x04DV[\x81R\x01\x92\x01\x91a\x11\xAFV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x815a\x11\xEE\x81a\x06\x96V[\x92a\x11\xFC`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x03ZW` \x82\x01\x90[\x83\x82\x10a\x12(WPPPPP\x90V[\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW` \x91a\x12J\x87\x84\x80\x94\x88\x01\x01a\x11\x08V[\x81R\x01\x91\x01\x90a\x12\x19V[\x91\x90\x91a\x01\x80\x81\x84\x03\x12a\x03ZWa\x12ka\x03\xCEV[\x92\x815`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\x88\x91\x84\x01a\x11\x08V[\x84R` \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\xA9\x91\x84\x01a\x11oV[` \x85\x01R`@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x12\xCD\x91\x84\x01a\x11oV[`@\x85\x01Ra\x12\xDF\x81``\x84\x01a\x04\xECV[``\x85\x01Ra\x12\xF1\x81`\xE0\x84\x01a\x04DV[`\x80\x85\x01Ra\x01 \x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x13\x16\x91\x84\x01a\x11\x08V[`\xA0\x85\x01Ra\x01@\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x81a\x13;\x91\x84\x01a\x11\x08V[`\xC0\x85\x01Ra\x01`\x82\x015`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13^\x92\x01a\x11\xD7V[`\xE0\x83\x01RV[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x13\x82WPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13uV[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x13\xD4\x906\x90`\x04\x01a\x0B\x9BV[\x90\x91`D5a\x13\xE2\x81a\x05\xFAV[`d5\x92`\x01`\x01`@\x1B\x03\x84\x11a\x03ZWa\x14D\x94a\x14\ta\x14\x0F\x956\x90`\x04\x01a\x12UV[\x93a4IV[`@Q\x92\x83\x92`@\x84R` a\x140\x82Q`@\x80\x88\x01R`\x80\x87\x01\x90a\x13eV[\x91\x01Q\x84\x82\x03`?\x19\x01``\x86\x01Ra\x13eV[\x90` \x83\x01R\x03\x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZWa\x14faL\xECV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` c\xFF\xFF\xFF\xFF`\xC9T\x16`@Q\x90\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCET`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90\x81`\x80\x91\x03\x12a\x03ZW\x90V[`@\x90`#\x19\x01\x12a\x03ZW`$\x90V[\x90\x81`@\x91\x03\x12a\x03ZW\x90V[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x15\xBD\x906\x90`\x04\x01a\x15`V[a\x15\xC66a\x15nV[`@6`c\x19\x01\x12a\x03ZW`\xA45`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x15\xF1\x906\x90`\x04\x01a\x11oV[a\x16ia\x15\xFD\x83a<\xD3V[\x92a\x16!a\x16\x19\x85c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15\x15a<\xDDV[a\x169\x84c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x90`@Qa\x16`\x81a\x16R`d` \x83\x01\x95\x86a=QV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x14a=oV[a\x16\x94a\x16\x8Ea\x16\x87\x84c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[T`\xFF\x16\x90V[\x15a=\xE1V[a\x16\xC3a\x16\xB5a\x16\xACa\x16\xA7`da<\xD3V[a>wV[c\xFF\xFF\xFF\xFF\x16\x90V[c\xFF\xFF\xFF\xFFC\x16\x11\x15a>\xA9V[a\x16\xCD\x81Qa&qV[\x92_[\x82Q\x81\x10\x15a\x17\rW\x80a\x16\xFCa\x16\xE9`\x01\x93\x86a&\xB0V[Q\x80Q_R` \x01Q` R`@_ \x90V[a\x17\x06\x82\x88a&\xB0V[R\x01a\x16\xD0V[P\x92\x91\x90\x91` \x84\x01\x91a\x17Ea\x17#\x84a<\xD3V[`@Qa\x179\x81a\x16R\x87` \x83\x01\x95\x86a4\tV[Q\x90 `\x845\x14a?\x1BV[a\x17O\x84Qa&qV[\x94\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x93_[\x86Q\x81\x10\x15a\x17\xFFW\x80` a\x17\x96a\x17\xB6\x93\x88a&\xB0V[Q`@Q\x80\x94\x81\x92ct]\xCDs`\xE1\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8AZ\xFA\x91\x82\x15a\x03UW`\x01\x92a\x17\xDB\x91_\x91a\x17\xE1W[Pa\x0BZ\x83\x8Ca&\xB0V[\x01a\x17}V[a\x17\xF9\x91P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a\x17\xD0V[P\x86\x91\x85\x91a\x18[a\x18$a\x18,`@\x85\x01\x93a\x18\x1C\x85\x87a?\xA6V[\x93\x90\x91a<\xD3V[\x926\x91a\x07\xC1V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16a)\x04V[\x93_\x90[\x85Q\x82\x10\x15a\x1B\x91W_\x96\x95\x96\x94[a\x18x\x83\x89a&\xB0V[QQ\x86\x10\x15a\x1B\x81W\x87` \x80a\x18\x9C\x89a\x18\x96\x88a\x18\xBE\x9Ea&\xB0V[Qa&\xB0V[Q\x01Q`@Q\x80\x9B\x81\x92c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x85Z\xFA\x98\x89\x15a\x03UW_\x99a\x1BaW[P`\x01\x97_[\x84Q\x81\x10\x15a\x1BRWa\x19\ta\x18\xFDa\x18\xF0\x83\x88a&\xB0V[Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x8C\x16\x14a\x19 W`\x01\x01a\x18\xD7V[P\x98\x90\x97P`\x01_\x97\x92\x97[\x15\x15\x14a\x19AW[P`\x01\x01\x94\x96\x95\x96a\x18nV[\x97\x95\x90\x94\x96\x91\x84\x92_\x89a\x19\xBA`\xFFa\x19\x92a\x19\x8Ca\x19~\x8Ba\x19xa\x1A\x0F\x9Ea\x19r`\xD1T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x98a?\xA6V[\x90a-\xAFV[5`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`\xF8\x1C\x90V[a\x19\xACa\x19\x9Da\x03\xE0V[`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x85RV[\x16c\xFF\xFF\xFF\xFF\x16` \x83\x01RV[`\xD0Ta\x19\xD1\x90a\x18\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Qc\x10]\xEA\x1F`\xE2\x1B\x81R\x82Q`\x01`\x01`\xA0\x1B\x03\x16`\x04\x82\x01R` \x90\x92\x01Qc\xFF\xFF\xFF\xFF\x16`$\x83\x01R\x90\x97\x88\x91\x90\x82\x90\x81\x90`D\x82\x01\x90V[\x03\x91Z\xFA\x95\x86\x15a\x03UW_\x96a\x1B.W[Pa\x1A,\x86Qa&qV[\x97_[\x89Q\x81\x10\x15a\x1AUW\x80g\x01cEx]\x8A\0\0a\x1AN`\x01\x93\x8Da&\xB0V[R\x01a\x1A/V[P\x99\x92\x97\x90\x98\x96\x91\x93\x95a\x1A\xA0`\xFFa\x1Axa\x19\x8Ca\x19~\x8Aa\x19x\x8D\x8Fa?\xA6V[a\x1A\x92a\x1A\x83a\x03\xFEV[`\x01`\x01`\xA0\x1B\x03\x90\x98\x16\x88RV[\x16c\xFF\xFF\xFF\xFF\x16` \x86\x01RV[`@\x84\x01R``\x83\x01Ra\x1A\xB2a@[V[`\x80\x83\x01R`\xCFTa\x1A\xCE\x90a\x18\xFD\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[\x80;\x15a\x03ZW`@Qcjf\x9BA`\xE0\x1B\x81R\x92_\x91\x84\x91\x82\x90\x84\x90\x82\x90a\x1A\xFA\x90`\x04\x83\x01a@\xAFV[\x03\x92Z\xF1\x91\x82\x15a\x03UW`\x01\x92a\x1B\x14W[P\x90a\x194V[\x80a\x1B\"_a\x1B(\x93a\x03\xADV[\x80a\x05\x88V[\x89a\x1B\rV[a\x1BK\x91\x96P=\x80_\x83>a\x1BC\x81\x83a\x03\xADV[\x81\x01\x90a?\xD8V[\x94\x8Aa\x1A!V[P\x98\x90\x97`\x01\x90\x97\x92\x97a\x19,V[a\x1Bz\x91\x99P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x97\x89a\x18\xD1V[\x95\x96\x95\x94P`\x01\x90\x91\x01\x90a\x18_V[\x86a\x1B\xBAa\x1B\xAD\x82c\xFF\xFF\xFF\xFF\x16_R`\xCC` R`@_ \x90V[\x80T`\xFF\x19\x16`\x01\x17\x90UV[c\xFF\xFF\xFF\xFF3\x91\x16\x7F\xC2\r\x1B\xB0\xF1b6\x800k\x83\xD4\xFFK\xB9\x9A+\xEB\x9D\x86\xD9x2\xF3\xCA@\xFD\x13\xA2\x9D\xF1\xEC_\x80\xA3\0[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xCFT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\x806`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x1C@\x906\x90`\x04\x01a\x15`V[a\x1CI6a\x15nV[\x90`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\x1Ci\x906\x90`\x04\x01a\x12UV[`\xCDT\x90\x92\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x1EeWa\x1C\x8C` \x83\x94\x93\x01a<\xD3V[\x91a\x1Dxa\x1C\x9D`@\x86\x01\x86a?\xA6V[\x92\x90\x94a\x1C\xEFa\x1C\xAF``\x89\x01a<\xD3V[\x97`@Qa\x1C\xC5\x81a\x16R` \x82\x01\x94\x85aA\x84V[Q\x90 a\x1C\xE8a\x1C\xD4\x88a<\xD3V[c\xFF\xFF\xFF\xFF\x16_R`\xCA` R`@_ \x90V[T\x14aB<V[a\x1D\x19a\x1D\x12a\x1C\xFE\x87a<\xD3V[c\xFF\xFF\xFF\xFF\x16_R`\xCB` R`@_ \x90V[T\x15aB\xAEV[\x83c\xFF\xFF\xFF\xFFC\x16\x96a\x1DZa\x1DRa\x16\xAC\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86a>\x8FV[\x89\x11\x15aC\x0FV[`@Q` \x81\x01\x90a\x1Dp\x81a\x16R\x8B\x85aCqV[Q\x90 a4IV[\x91\x90`\xFF_\x96\x16\x95[\x82\x81\x10a\x1E\x03W\x7F\xF2\xAF\x11\xFA\xD7=CI\xC9\x9C\xF6/)\x8D3vA\xEA\x0B\xB7\xC0\xF5\xA8\xDB\x92\xA9\x8A'_sOX\x86\x86\x86a\x1D\xC3a\x1D\xB7a\x03\xE0V[c\xFF\xFF\xFF\xFF\x90\x94\x16\x84RV[` \x83\x01R`@Q` \x81\x01\x90a\x1D\xDF\x81a\x16R\x86\x86\x86aDAV[Q\x90 a\x1D\xEEa\x1C\xFE\x83a<\xD3V[Ua\x1D\xFE`@Q\x92\x83\x92\x83aDAV[\x03\x90\xA1\0[\x80a\x1E_a\x1E;a\x1E6a\x1E*a\x1E\x1D`\x01\x96\x88Qa&\xB0V[Q`\x01`\x01``\x1B\x03\x16\x90V[`\x01`\x01``\x1B\x03\x16\x90V[aC\x81V[a\x1EXa\x1E*\x8Ba\x1ESa\x1E\x1D\x87` \x8B\x01Qa&\xB0V[aC\xADV[\x11\x15aC\xD0V[\x01a\x1D\x81V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAggregator must be the caller\0\0\0`D\x82\x01R`d\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `\xFF`\x97T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`\xD0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03ZW`\xC06`\x03\x19\x01\x12a\x03ZW`\x045a\x1F\x11\x81a\x06\x85V[a\x1F\x91`$5a\x1F \x81a\x06\x85V[`D5a\x1F,\x81a\x06\x85V[`d5a\x1F8\x81a\x06\x85V[`\x845\x91a\x1FE\x83a\x06\x85V[`\xA45\x93a\x1FR\x85a\x06\x85V[_T\x96a\x1Fw`\xFF`\x08\x8A\x90\x1C\x16\x15\x80\x99\x81\x9Aa \x0FW[\x81\x15a\x1F\xEFW[PaDkV[\x87a\x1F\x88`\x01`\xFF\x19_T\x16\x17_UV[a\x1F\xD8WaD\xCEV[a\x1F\x97W\0[a\x1F\xA5a\xFF\0\x19_T\x16_UV[`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90\x80` \x81\x01a\x1D\xFEV[a\x1F\xEAa\x01\0a\xFF\0\x19_T\x16\x17_UV[aD\xCEV[0;\x15\x91P\x81a \x01W[P_a\x1FqV[`\xFF\x16`\x01\x14\x90P_a\x1F\xFAV[`\x01`\xFF\x82\x16\x10\x91Pa\x1FjV[`@\x90a\x06\xF1\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x07\xF7V[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045a Q\x81a\x06\x85V[`$5`D5a `\x81a\x05\xFAV[a \xA1a ka&OV[\x92\x80a v\x85a&\xA3V[R`@Qca\xC8\xA1/`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x86\x16\x94\x90\x92_\x91\x84\x91\x82\x91\x87`\x04\x84\x01a2WV[\x03\x81\x87Z\xFA\x93\x84\x15a\x03UW\x83a \xCBa\x16\xACa\x0F\x1Fa!\0\x98` \x97_\x91a!^W[Pa&\xA3V[\x92`@Q\x96\x87\x94\x85\x93\x84\x93c\x04\xECcQ`\xE0\x1B\x85R`\x04\x85\x01c\xFF\xFF\xFF\xFF`@\x92\x95\x94\x93``\x83\x01\x96\x83R\x16` \x82\x01R\x01RV[\x03\x91Z\xFA\x80\x15a\x03UWa!/\x92_\x91a!?W[P`\x01`\x01`\xC0\x1B\x03\x16\x92a!)\x84aM\x8CV[\x90a)\x04V[\x90a\x07\x88`@Q\x92\x83\x92\x83a \x1DV[a!X\x91P` =` \x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[_a!\x15V[a!r\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a \xC5V[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03ZW``6`\x03\x19\x01\x12a\x03ZW`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa!\xEC\x906\x90`\x04\x01a\x15\x7FV[`$5\x90a!\xF9\x82a\x05\xFAV[`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03ZWa\"\x18\x906\x90`\x04\x01a\x0B\x9BV[`\xCET\x91\x93\x90\x92\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a#\x02Wa\x03$\x93a\"\xEC\x93a\"la\"s\x93a\"Qa\"IaE7V[\x966\x90aE}V[\x86RCc\xFF\xFF\xFF\xFF\x16` \x87\x01Rc\xFF\xFF\xFF\xFF\x16``\x86\x01RV[6\x91a\x07\xC1V[`@\x82\x01R`@Q` \x81\x01\x90a\"\x8E\x81a\x16R\x85\x85aE\xD7V[Q\x90 a\"\xA3a\x1C\xD4`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90V[U`\xC9Tc\xFF\xFF\xFF\xFF\x16\x90\x7F\xA6\xB1\x91-\x06o\xD3\xA5A:b\"\xA5xZ\xE4?$$X\xF3\xEE\xF4\\P\xDD\x80\x89\x87\xB8x\xFD`@Q\x80a\"\xE4c\xFF\xFF\xFF\xFF\x86\x16\x94\x82aE\xD7V[\x03\x90\xA2a>_V[c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x19`\xC9T\x16\x17`\xC9UV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask generator must be the calle`D\x82\x01R`9`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045a#n\x81a\x06\x85V[a#vaL\xECV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a#\x8EWa\x03$\x90aMDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03ZW_6`\x03\x19\x01\x12a\x03ZW` `@Q`d\x81R\xF3[4a\x03ZW` 6`\x03\x19\x01\x12a\x03ZW`\x045`@Qcu[6\xBD`\xE1\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a$\xC0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a$\xB1Wa$\x7F`fT\x19\x82\x19\x81\x16\x14a%\x15V[\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a$\xD9\x91P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[_a$^V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\t@V[`@Q=_\x82>=\x90\xFD[\x15a%\x06WV[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[\x15a%\x1CWV[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x90`\x02\x81\x10\x15a%PW`\x05\x1B\x01\x90V[a%+V[cNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[a&Ea&\"a&K\x95a&\x1Ca&\x15\x85\x87Q` \x89\x01Q\x8AQQ` \x8CQ\x01Q` \x8D\x01` \x81QQ\x91Q\x01Q\x91\x89Q\x93` \x8B\x01Q\x95`@Q\x97` \x89\x01\x99\x8AR` \x8A\x01R`@\x89\x01R``\x88\x01R`\x80\x87\x01R`\xA0\x86\x01R`\xC0\x85\x01R`\xE0\x84\x01Ra\x01\0\x83\x01Ra%\xEC\x81a\x01 \x84\x01\x03`\x1F\x19\x81\x01\x83R\x82a\x03\xADV[Q\x90 \x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x90\x06\x90V[\x80\x96aF\xF3V[\x90aG9V[\x92a&\x1Ca&7a&1aG\x9BV[\x94aH\x92V[\x91a&@aI\xAEV[aF\xF3V[\x91aI\xE2V[\x90\x91V[`@\x80Q\x90\x91\x90a&`\x83\x82a\x03\xADV[`\x01\x81R\x91`\x1F\x19\x016` \x84\x017V[\x90a&{\x82a\x06\x96V[a&\x88`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a&\x99`\x1F\x19\x91a\x06\x96V[\x01\x90` 6\x91\x017V[\x80Q\x15a%PW` \x01\x90V[\x80Q\x82\x10\x15a%PW` \x91`\x05\x1B\x01\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ\x90V[\x91\x90\x91a&\xE0\x83Qa&qV[\x92_[\x81Q\x81\x10\x15a'\x86W\x80` a&\xFFa\x18\xF0a'(\x94\x86a&\xB0V[`@Qc\t\xAA\x15'`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x04\x82\x01R\x92\x83\x90\x81\x90`$\x82\x01\x90V[\x03\x81`\x01`\x01`\xA0\x1B\x03\x88\x16Z\xFA\x80\x15a\x03UW`\x01\x92_\x91a'XW[Pa'Q\x82\x88a&\xB0V[R\x01a&\xE3V[a'y\x91P` =\x81\x11a'\x7FW[a'q\x81\x83a\x03\xADV[\x81\x01\x90a&\xC4V[_a'FV[P=a'gV[PPPV[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x06\x85V[\x90a'\xAA\x82a\x06\x96V[a'\xB7`@Q\x91\x82a\x03\xADV[\x82\x81R` \x81\x93a'\xCA`\x1F\x19\x91a\x06\x96V[\x01\x91\x01_[\x82\x81\x10a'\xDBWPPPV[``\x82\x82\x01R` \x01a'\xCFV[\x90\x81Q\x81\x10\x15a%PW\x01` \x01\x90V[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa(-\x81a\x06\x96V[\x92a(;`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a(cWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a(VV[\x90a(}\x82a\x06\x96V[a(\x8A`@Q\x91\x82a\x03\xADV[\x82\x81R\x80\x92a(\x9B`\x1F\x19\x91a\x06\x96V[\x01_[\x81\x81\x10a(\xAAWPPPV[`@Q\x90``\x82\x01\x91\x80\x83\x10`\x01`\x01`@\x1B\x03\x84\x11\x17a\x03\x8DW` \x92`@R_\x81R_\x83\x82\x01R_`@\x82\x01R\x82\x82\x86\x01\x01R\x01a(\x9EV[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[`@Qch0H5`\xE0\x1B\x81R\x93\x91\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x16` \x85`\x04\x81\x84Z\xFA\x94\x85\x15a\x03UW_\x95a,\x04W[P`@QcOL\x91\xE1`\xE1\x1B\x81R\x94` \x86`\x04\x81\x85Z\xFA\x91\x82\x15a\x03UW`\x04\x96_\x93a+\xE2W[P` \x90`@Q\x97\x88\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x95\x86\x15a\x03UW_\x96a+\xC1W[Pa)\x92\x85\x93\x92\x95Qa'\xA0V[\x94_\x93[\x80Q\x85\x10\x15a+\xB7Wa)\xBDa\x19\x8Ca)\xAF\x87\x84a'\xE9V[Q`\x01`\x01`\xF8\x1B\x03\x19\x16\x90V[`@Qc\x89\x02bE`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01Rc\xFF\xFF\xFF\xFF\x88\x16`$\x82\x01R\x90\x94\x90\x92_\x84`D\x81`\x01`\x01`\xA0\x1B\x03\x85\x16Z\xFA\x93\x84\x15a\x03UW_\x94a+\x93W[Pa*\r\x84Qa(sV[a*\x17\x88\x8Ba&\xB0V[Ra*\"\x87\x8Aa&\xB0V[P_[\x84Q\x81\x10\x15a+\x82W\x80` a*>a*`\x93\x88a&\xB0V[Q\x8D`@Q\x80\x80\x96\x81\x94c\x08\xF6b\x9D`\xE3\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x91\x82\x15a\x03UW_\x92a+bW[Pa*\x86\x81\x87a&\xB0V[Q\x8A` \x8Aa*\x95\x85\x8Ba&\xB0V[Q`@Qc\xFA(\xC6'`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\xFF\x91\x90\x91\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x8D\x16Z\xFA\x93\x84\x15a\x03UWa+\x19\x8C\x8Fa+\x14`\x01\x98a++\x97\x89\x97_\x92a+2W[Pa*\xFFa\x1A\x83a\x03\xEFV[` \x87\x01R`\x01`\x01``\x1B\x03\x16`@\x86\x01RV[a&\xB0V[Q\x90a+%\x83\x83a&\xB0V[Ra&\xB0V[P\x01a*%V[a+T\x91\x92P` =\x81\x11a+[W[a+L\x81\x83a\x03\xADV[\x81\x01\x90a(\xE5V[\x90_a*\xF3V[P=a+BV[a+{\x91\x92P` =\x81\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x90_a*{V[P`\x01\x90\x96\x01\x95\x90\x94P\x91Pa)\x96V[a+\xB0\x91\x94P=\x80_\x83>a+\xA8\x81\x83a\x03\xADV[\x81\x01\x90a'\xFAV[\x92_a*\x02V[PPP\x93PPP\x90V[a+\xDB\x91\x96P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x94_a)\x84V[` \x91\x93Pa+\xFD\x90\x82=\x84\x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x92\x90a)_V[a,\x1E\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a)6V[`@Q\x90a,2\x82a\x03\x92V[``\x80\x83\x81\x81R\x81` \x82\x01R\x81`@\x82\x01R\x01RV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa,|\x81a\x06\x96V[\x92a,\x8A`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a,\xB2WPPP\x90V[` \x80\x91\x83Qa,\xC1\x81a\x05\xFAV[\x81R\x01\x91\x01\x90a,\xA5V[c\xFF\xFF\xFF\xFF\x90\x91\x16\x81R`@` \x82\x01\x81\x90R\x81\x01\x83\x90R`\x01`\x01`\xFB\x1B\x03\x83\x11a\x03ZW``\x92`\x05\x1B\x80\x92\x84\x83\x017\x01\x01\x90V[\x90\x80` \x93\x92\x81\x84R\x84\x84\x017_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x95\x93\x16\x81R\x81` \x82\x01R\x01\x91a-\x03V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[`\xFF\x16`\xFF\x81\x14a-eW`\x01\x01\x90V[a-@V[\x91\x90\x81\x10\x15a%PW`\x05\x1B\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQ`\x01`\x01`\xC0\x1B\x03\x81\x16\x81\x03a\x03ZW\x90V[\x15a-\xA0WV[c%\xECl\x1F`\xE0\x1B_R`\x04_\xFD[\x90\x82\x10\x15a%PW\x01\x90V[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\x05\xFAV[_\x19\x81\x14a-eW`\x01\x01\x90V[\x91a-\xFC` \x92c\xFF\xFF\xFF\xFF\x92\x96\x95\x96`@\x86R`@\x86\x01\x91a-\x03V[\x94\x16\x91\x01RV[\x95\x93\x94\x95\x92\x90\x91\x92a.\x13a,%V[P`@Qch0H5`\xE0\x1B\x81R\x93`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x91\x90` \x85`\x04\x81\x86Z\xFA\x94\x85\x15a\x03UW_\x95a26W[Pa.Pa,%V[\x94`@Qca\xC8\xA1/`\xE1\x1B\x81R_\x81\x80a.p\x8D\x8D\x8B`\x04\x85\x01a,\xCCV[\x03\x81\x88Z\xFA\x90\x81\x15a\x03UW_\x91a2\x1CW[P\x86R`@Qc@\xE0:\x81`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16\x92\x90_\x81\x80a.\xB3\x85\x87\x8B`\x04\x85\x01a-#V[\x03\x81\x87Z\xFA\x90\x81\x15a\x03UW_\x91a2\x02W[P`@\x87\x01Ra.\xD5\x81a'\xA0V[\x98``\x87\x01\x99\x8AR_[`\xFF\x81\x16\x83\x81\x10\x15a1MW\x88_a/\x08\x83\x8Fa.\xFB\x88a&qV[\x90Q\x90a+%\x83\x83a&\xB0V[P_\x8A\x86\x8F[\x81\x84\x10a/\x85WPPPP\x90P\x8Ca/%\x82a&qV[\x91_[\x81\x81\x10a/RWPP\x91a/G\x91a/M\x94\x93Q\x90a+%\x83\x83a&\xB0V[Pa-TV[a.\xDFV[\x80a/\x7Fa/ja\x0F\x1F`\x01\x94a\x18\x96\x8A\x89Qa&\xB0V[a/t\x83\x88a&\xB0V[\x90c\xFF\xFF\xFF\xFF\x16\x90RV[\x01a/(V[a\x0F\x1F\x84a/\x9A\x81` \x96\x95a/\xA2\x95a-jV[5\x97Qa&\xB0V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x96\x90\x96Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x87\x01R\x16`D\x85\x01R\x83`d\x81\x8DZ\xFA\x80\x15a\x03UW\x88\x8F\x88\x8A\x91\x8F\x94a09`\x01a0,\x81\x93\x8D\x80\x9D_\x92a1!W[Pa\x19\x8Ca\x19~a0\x16\x92a0\x0F\x87\x80`\xC0\x1B\x03\x86\x16\x15\x15a-\x99V[\x8B\x8Da-\xAFV[`\x01`\x01`\xC0\x1B\x03\x91\x82\x16`\xFF\x91\x90\x91\x16\x1C\x16\x90V[\x16`\x01`\x01`\xC0\x1B\x03\x16\x90V[\x14a0UW[PPPPP`\x01\x91\x92P\x01\x90\x8A\x91\x8A\x86\x8Fa/\x0EV[\x85\x97a0w\x93a0p` \x97\x99\x98a\x19\x8C\x95a\x19~\x95a-jV[5\x95a-\xAFV[`@Qc\xDD\x98F\xB9`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`\xFF\x16`$\x82\x01Rc\xFF\xFF\xFF\xFF\x93\x90\x93\x16`D\x84\x01R\x82`d\x81\x8CZ\xFA\x90\x81\x15a\x03UW\x8Fa0\xD5\x90a0\xDA\x93\x83\x88`\x01\x97_\x93a0\xE9W[Pa\x18\x96\x90a/t\x93\x94Qa&\xB0V[a-\xD0V[\x90P\x82\x91\x8A\x88\x8F\x88\x8A\x91a0?V[a/t\x93P\x90a1\x12a\x18\x96\x92` =\x81\x11a1\x1AW[a1\n\x81\x83a\x03\xADV[\x81\x01\x90a-\xBBV[\x93P\x90a0\xC5V[P=a1\0V[a0\x16\x91\x92Pa\x19~a1Da\x19\x8C\x92` =\x81\x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[\x93\x92PPa/\xF2V[PPP\x92\x90\x95\x97P`\x04\x94\x96P` \x91P`@Q\x94\x85\x80\x92c.\xFA,\xA3`\xE1\x1B\x82RZ\xFA\x90\x81\x15a\x03UWa1\xA3\x94_\x94\x85\x93a1\xE1W[P`@Qc5IR\xA3`\xE2\x1B\x81R\x95\x86\x94\x85\x93\x84\x93`\x04\x85\x01a-\xDEV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a1\xC7W[P` \x82\x01R\x90V[a1\xDB\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a1\xBEV[a1\xFB\x91\x93P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x91_a1\x85V[a2\x16\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a.\xC6V[a20\x91P=\x80_\x83>a\x0F\xD2\x81\x83a\x03\xADV[_a.\x83V[a2P\x91\x95P` =` \x11a\n\x04Wa\t\xF6\x81\x83a\x03\xADV[\x93_a.GV[`@\x90c\xFF\xFF\xFF\xFFa\x06\xF1\x94\x93\x16\x81R\x81` \x82\x01R\x01\x90a\x06\xADV[`@Q\x90a2\x81\x82a\x03rV[``` \x83\x82\x81R\x01RV[\x15a2\x94WV[b\xF8 -`\xE5\x1B_R`\x04_\xFD[\x15a2\xA9WV[cCqJ\xFD`\xE0\x1B_R`\x04_\xFD[\x15a2\xBFWV[c_\x83/A`\xE0\x1B_R`\x04_\xFD[\x15a2\xD5WV[cK\x87OE`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQa\x06\xF1\x81a\r\xF1V[_\x19\x81\x01\x91\x90\x82\x11a-eWV[\x15a3\x0EWV[c?\xDCe\x05`\xE2\x1B_R`\x04_\xFD[\x90`\x01\x82\x01\x80\x92\x11a-eWV[\x90`\x02\x82\x01\x80\x92\x11a-eWV[\x90`\x03\x82\x01\x80\x92\x11a-eWV[\x90`\x04\x82\x01\x80\x92\x11a-eWV[\x90`\x05\x82\x01\x80\x92\x11a-eWV[\x91\x90\x82\x01\x80\x92\x11a-eWV[\x15a3wWV[c\xAF\xFC^\xDB`\xE0\x1B_R`\x04_\xFD[\x90\x81` \x91\x03\x12a\x03ZWQg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x81\x16\x81\x03a\x03ZW\x90V[\x15a3\xAEWV[c\xE11\n\xED`\xE0\x1B_R`\x04_\xFD[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x03\x90`\x01`\x01``\x1B\x03\x82\x11a-eWV[\x15a3\xE4WV[cg\x98\x8D3`\xE0\x1B_R`\x04_\xFD[\x15a3\xFAWV[c\xAB\x1B#k`\xE0\x1B_R`\x04_\xFD[`\x04\x91c\xFF\xFF\xFF\xFF`\xE0\x1B\x90`\xE0\x1B\x16\x81R\x01` \x82Q\x91\x92\x01\x90_[\x81\x81\x10a43WPPP\x90V[\x82Q\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a4&V[\x94\x93\x92\x90\x91\x93a4Wa2tV[Pa4c\x85\x15\x15a2\x8DV[`@\x84\x01QQ\x85\x14\x80a<\xC5W[\x80a<\xB7W[\x80a<\xA9W[a4\x86\x90a2\xA2V[a4\x98` \x85\x01QQ\x85QQ\x14a2\xB8V[a4\xAFc\xFF\xFF\xFF\xFFC\x16c\xFF\xFF\xFF\xFF\x84\x16\x10a2\xCEV[a4\xB7a\x03\xE0V[_\x81R_` \x82\x01R\x92a4\xC9a2tV[a4\xD2\x87a&qV[` \x82\x01Ra4\xE0\x87a&qV[\x81Ra4\xEAa2tV[\x92a4\xF9` \x88\x01QQa&qV[\x84Ra5\t` \x88\x01QQa&qV[` \x85\x81\x01\x91\x90\x91R`@Qc\x9A\xA1e=`\xE0\x1B\x81R\x90\x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UWa5r\x91_\x91a<zW[Pa5m6\x8B\x87a\x07\xC1V[aKNV[\x98_\x96[` \x89\x01Q\x80Q\x89\x10\x15a6\xD1W` \x88a5\xC6a\x0F\x1F\x8Ca5\xBE\x8F\x96\x86\x8Ea5\xA3a\x16\xE9\x86\x80\x95a&\xB0V[a5\xB0\x84\x84\x84\x01Qa&\xB0V[R\x82a6\x9EW[\x01Qa&\xB0V[Q\x95Qa&\xB0V[`@Qc\x04\xECcQ`\xE0\x1B\x81R`\x04\x81\x01\x94\x90\x94Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x91\x82\x15a\x03UWa&\x1C\x8Aa6s\x8Fa6l\x8F\x84` \x8F\x92a6c\x93a6[\x84`\x01\x9Ea6y\x9E_\x91a6\x81W[P\x8F\x80`\xC0\x1B\x03\x16\x92Qa&\xB0V[R\x01Qa&\xB0V[Q\x93\x8DQa&\xB0V[Q\x16aKyV[\x90aK\xAAV[\x97\x01\x96a5vV[a6\x98\x91P\x86=\x81\x11a\x0F\xA8Wa\x0F\x9A\x81\x83a\x03\xADV[_a6LV[a6\xCCa6\xAE\x84\x84\x84\x01Qa&\xB0V[Qa6\xC5\x84\x84\x01Qa6\xBF\x87a2\xF9V[\x90a&\xB0V[Q\x10a3\x07V[a5\xB7V[P\x90\x95\x97\x94\x96Pa6\xE6\x91\x98\x93\x92\x99PaLgV[\x91a6\xF3`\x97T`\xFF\x16\x90V[\x90\x81\x15a<rW`@Qc\x18\x89\x1F\xD7`\xE3\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW_\x91a<SW[P\x91\x90[_\x92[\x81\x84\x10a7\xA4WPPPPP\x92a7\x8Ba7\x86a7\x7Fa7\x9E\x95\x85a\x16R\x98`\x80``` \x99\x01Q\x92\x01Q\x92a%iV[\x91\x90a3\xDDV[a3\xF3V[\x01Q`@Q\x92\x83\x91` \x83\x01\x95\x86a4\tV[Q\x90 \x90V[\x92\x98\x95\x96\x90\x93\x99\x91\x97\x94\x87\x8B\x88\x8C\x88\x8Da;MW[a\x0F\x1F\x82`\xA0a7\xF9a\x19\x8Ca\x19~\x84a8\x01\x97a7\xF3a7\xE5a\x16\xE9\x8F\x9C`@` \x9F\x9E\x01Qa&\xB0V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16\x90V[\x9Ba-\xAFV[\x97\x01Qa&\xB0V[`@Qc\x1A/2\xAB`\xE2\x1B\x81R`\xFF\x95\x90\x95\x16`\x04\x86\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x86\x01R\x16`D\x84\x01R\x82`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa8\xC5a\x0F\x1F\x8F\x95\x8F\x90a8\xBD\x8F\x97\x8F\x96\x84\x8Fa8\xB7`\xC0\x96a8\xB0\x84\x8F` \x9F\x90a5\xB7a\x19~\x99`@\x93a\x19\x8C\x9C_\x91a;\x1FW[Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91\x82\x16\x91\x16\x14a3\xA7V[Q\x90aG9V[\x9Ca-\xAFV[\x96\x01Qa&\xB0V[`@Qcd\x14\xA6+`\xE1\x1B\x81R`\xFF\x94\x90\x94\x16`\x04\x85\x01Rc\xFF\xFF\xFF\xFF\x91\x82\x16`$\x85\x01R\x16`D\x83\x01R\x81`d\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03UWa9R\x91\x8C\x8F\x92_\x92a:\xFBW[P` a9D\x92\x93\x01Qa&\xB0V[\x90`\x01`\x01``\x1B\x03\x16\x90RV[a9r\x8Ca9D\x8Ca9ka\x1E\x1D\x82` \x86\x01Qa&\xB0V[\x92Qa&\xB0V[_\x98_[` \x8A\x01QQ\x81\x10\x15a:\xE2W\x8B\x8Da9\xB4\x89a9\xA7a\x19\x8Ca\x19~\x86\x8F\x89a9\x9F\x91Qa&\xB0V[Q\x94\x87a-\xAFV[`\xFF\x16\x1C`\x01\x90\x81\x16\x14\x90V[a9\xC3W[PP`\x01\x01a9vV[\x8A\x8Aa:E\x85\x9F\x94\x8F\x96\x86a\x18\x96\x8F\x93`\xE0a9\xFCa\x0F\x1F\x95` a9\xF4a\x19\x8Ca\x19~\x83\x9Fa:\x05\x9C\x89\x91a-\xAFV[\x9A\x01Qa&\xB0V[Q\x9B\x01Qa&\xB0V[`@Qcy_JW`\xE1\x1B\x81R`\xFF\x90\x93\x16`\x04\x84\x01Rc\xFF\xFF\xFF\xFF\x93\x84\x16`$\x84\x01R`D\x83\x01\x96\x90\x96R\x91\x90\x94\x16`d\x85\x01R\x83\x90\x81\x90`\x84\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03UW\x8Fa:\xB1\x90\x8F\x93`\x01\x95\x94\x86\x95_\x92a:\xBCW[Pa:\xABa9D\x92\x93Q\x93a:\xA6a\x1E\x1D\x84\x87a&\xB0V[a3\xBDV[\x92a&\xB0V[\x01\x9A\x90P\x8B\x8Da9\xB9V[a9D\x92Pa:\xDBa:\xAB\x91` =\x81\x11a+[Wa+L\x81\x83a\x03\xADV[\x92Pa:\x8EV[P\x93\x91\x97\x96\x99`\x01\x91\x96\x99P\x9A\x94\x92\x9A\x01\x92\x91\x90a7NV[a9D\x92Pa;\x18` \x91\x82=\x81\x11a+[Wa+L\x81\x83a\x03\xADV[\x92Pa95V[` a;@\x92P=\x81\x11a;FW[a;8\x81\x83a\x03\xADV[\x81\x01\x90a3\x86V[_a8\x9AV[P=a;.V[a;\x8A\x94Pa;g\x92Pa\x19\x8C\x91a\x19~\x91` \x95a-\xAFV[`@Qc\x12M\x06!`\xE1\x1B\x81R`\xFF\x90\x91\x16`\x04\x82\x01R\x91\x82\x90\x81\x90`$\x82\x01\x90V[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x03UW` \x89a8\x01\x8F\x93\x8F`\xA0\x8F\x97a\x19\x8Ca\x19~\x8F\x8F\x90a7\xF3a7\xE5a\x16\xE9\x8F`@\x8B\x96\x91\x8F\x88\x93a\x0F\x1F\x9Fa<\x0E\x90a<\x14\x93a7\xF9\x9F_\x92a<*W[Pc\xFF\xFF\xFF\xFF\x80\x91\x16\x93\x16\x90a3cV[\x11a3pV[PPPPPP\x97PPPPPP\x92\x93PPa7\xB9V[` c\xFF\xFF\xFF\xFF\x92\x93P\x82\x91a<K\x91=\x81\x11a'\x7FWa'q\x81\x83a\x03\xADV[\x92\x91Pa;\xFDV[a<l\x91P` =` \x11a1\x1AWa1\n\x81\x83a\x03\xADV[_a7GV[_\x91\x90a7KV[a<\x9C\x91P` =` \x11a<\xA2W[a<\x94\x81\x83a\x03\xADV[\x81\x01\x90a2\xE4V[_a5aV[P=a<\x8AV[P`\xE0\x84\x01QQ\x85\x14a4}V[P`\xC0\x84\x01QQ\x85\x14a4wV[P`\xA0\x84\x01QQ\x85\x14a4qV[5a\x06\xF1\x81a\x05\xFAV[\x15a<\xE4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FTask hasn't been responded to ye`D\x82\x01R`\x1D`\xFA\x1B`d\x82\x01R`\x84\x90\xFD[` \x80\x91c\xFF\xFF\xFF\xFF\x815a=G\x81a\x05\xFAV[\x16\x84R\x015\x91\x01RV[\x91`@a\x03\xDE\x92\x94\x93a=h\x81`\x80\x81\x01\x97a=3V[\x01\x90a=3V[\x15a=vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FTask response does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a=\xE8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FThe response to this task has al`D\x82\x01R\x7Fready been challenged successful`d\x82\x01Rb6<\x97`\xE9\x1B`\x84\x82\x01R`\xA4\x90\xFD[c\xFF\xFF\xFF\xFF`\x01\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[c\xFF\xFF\xFF\xFF`d\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[\x90c\xFF\xFF\xFF\xFF\x80\x91\x16\x91\x16\x01\x90c\xFF\xFF\xFF\xFF\x82\x11a-eWV[\x15a>\xB0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FThe challenge period for this ta`D\x82\x01R\x7Fsk has already expired.\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15a?\"WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`P`$\x82\x01R\x7FThe pubkeys of non-signing opera`D\x82\x01R\x7Ftors supplied by the challenger `d\x82\x01Ro0\xB92\x9077\xBA\x101\xB7\xB992\xB1\xBA\x17`\x81\x1B`\x84\x82\x01R`\xA4\x90\xFD[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03ZW\x01\x805\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW` \x01\x91\x816\x03\x83\x13a\x03ZWV[` \x81\x83\x03\x12a\x03ZW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81Qa@\x0B\x81a\x06\x96V[\x92a@\x19`@Q\x94\x85a\x03\xADV[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x03ZW` \x01\x90[\x82\x82\x10a@AWPPP\x90V[` \x80\x91\x83Qa@P\x81a\x06\x85V[\x81R\x01\x91\x01\x90a@4V[`@Q\x90a@j`@\x83a\x03\xADV[`\x12\x82Rq9\xB60\xB9\xB4/\xBA42\xAF\xB7\xB82\xB90\xBA7\xB9`q\x1B` \x83\x01RV[\x80Q\x80\x83R` \x92\x91\x81\x90\x84\x01\x84\x84\x01^_\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x91\x90` \x83R`\xC0\x83\x01\x92`\x01\x80`\xA0\x1B\x03\x82Q\x16` \x82\x01Rc\xFF\xFF\xFF\xFF` \x83\x01Q\x16`@\x82\x01R`@\x82\x01Q\x93`\xA0``\x83\x01R\x84Q\x80\x91R` `\xE0\x83\x01\x95\x01\x90_[\x81\x81\x10aA4WPPP`\x80aA\x1Fa\x06\xF1\x94\x95``\x85\x01Q`\x1F\x19\x85\x83\x03\x01\x84\x86\x01Ra\x06\xADV[\x92\x01Q\x90`\xA0`\x1F\x19\x82\x85\x03\x01\x91\x01Ra@\x8BV[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x87R` \x96\x87\x01\x96\x90\x92\x01\x91`\x01\x01a@\xF6V[\x905`\x1E\x19\x826\x03\x01\x81\x12\x15a\x03ZW\x01` \x815\x91\x01\x91`\x01`\x01`@\x1B\x03\x82\x11a\x03ZW\x816\x03\x83\x13a\x03ZWV[` \x81R\x815\x90`>\x19\x836\x03\x01\x82\x12\x15a\x03ZW`\x80aB1``aB*aA\xF0\x87a\x06\xF1\x97\x01\x85` \x88\x01RaA\xDEaA\xD3aA\xC2\x83\x80aASV[`@`\xA0\x8C\x01R`\xE0\x8B\x01\x91a-\x03V[\x91` \x81\x01\x90aASV[\x88\x83\x03`\x9F\x19\x01`\xC0\x8A\x01R\x90a-\x03V[aB\x0CaA\xFF` \x8A\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16`@\x88\x01RV[aB\x19`@\x89\x01\x89aASV[\x87\x83\x03`\x1F\x19\x01\x85\x89\x01R\x90a-\x03V[\x95\x01a\x06\x08V[c\xFF\xFF\xFF\xFF\x16\x91\x01RV[\x15aBCWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7Fsupplied task does not match the`D\x82\x01R\x7F one recorded in the contract\0\0\0`d\x82\x01R`\x84\x90\xFD[\x15aB\xB5WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`,`$\x82\x01R\x7FAggregator has already responded`D\x82\x01Rk to the task`\xA0\x1B`d\x82\x01R`\x84\x90\xFD[\x15aC\x16WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`-`$\x82\x01R\x7FAggregator has responded to the `D\x82\x01Rltask too late`\x98\x1B`d\x82\x01R`\x84\x90\xFD[`@\x81\x01\x92\x91a\x03\xDE\x91\x90a=3V[\x90`d\x82\x02\x91\x80\x83\x04`d\x14\x90\x15\x17\x15a-eWV[\x90`\x06\x82\x02\x91\x80\x83\x04`\x06\x14\x90\x15\x17\x15a-eWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x02\x90`\x01`\x01``\x1B\x03\x82\x16\x91\x82\x03a-eWV[\x15aC\xD7WV[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FSignatories do not own at least `D\x82\x01R\x7Fthreshold percentage of a quorum`d\x82\x01R\xFD[\x90\x92\x91` ``\x91aDW\x84`\x80\x81\x01\x97a=3V[c\xFF\xFF\xFF\xFF\x81Q\x16`@\x85\x01R\x01Q\x91\x01RV[\x15aDrWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[aD\xD7\x90aMDV[`\xCD\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x17\x90\x91U`\xCE\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD0\x80T\x83\x16\x93\x82\x16\x93\x90\x93\x17\x90\x92U`\xCF\x80T\x82\x16\x93\x83\x16\x93\x90\x93\x17\x90\x92U`\xD1\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90UV[`@Q\x90aED\x82a\x03\x92V[_``\x83aEPa2tV[\x81R\x82` \x82\x01R\x81`@\x82\x01R\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x03ZW\x81` a\x06\xF1\x935\x91\x01a\x07\xC1V[\x91\x90`@\x83\x82\x03\x12a\x03ZW`@Q\x90aE\x96\x82a\x03rV[\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03ZW\x82aE\xB4\x91\x83\x01aEbV[\x83R` \x81\x015\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03ZW` \x92a\x04\xE7\x92\x01aEbV[` \x81R`\x80c\xFF\xFF\xFF\xFF``aF?aF\x1E\x86Q\x85` \x88\x01R` aF\n\x82Q`@`\xA0\x8B\x01R`\xE0\x8A\x01\x90a@\x8BV[\x91\x01Q\x87\x82\x03`\x9F\x19\x01`\xC0\x89\x01Ra@\x8BV[\x83` \x88\x01Q\x16`@\x87\x01R`@\x87\x01Q`\x1F\x19\x87\x83\x03\x01\x84\x88\x01Ra@\x8BV[\x94\x01Q\x16\x91\x01R\x90V[_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2V[`@Q\x90aF\xBC\x82a\x03rV[_` \x83\x82\x81R\x01RV[`@Q\x90a\x01\x80aF\xD8\x81\x84a\x03\xADV[6\x837V[`@Q\x90aF\xEC` \x83a\x03\xADV[` 6\x837V[\x91\x90`@\x90``aG\x02aF\xAFV[\x94\x85\x92` \x85Q\x92aG\x14\x85\x85a\x03\xADV[\x846\x857\x80Q\x84R\x01Q` \x83\x01R\x84\x82\x01R`\x07a\x07\xCF\x19Z\x01\xFA\x15aG7WV[\xFE[` \x92\x91`\x80`@\x92aGJaF\xAFV[\x95\x86\x93\x81\x86Q\x93aG[\x86\x86a\x03\xADV[\x856\x867\x80Q\x85R\x01Q\x82\x84\x01R\x80Q\x86\x84\x01R\x01Q``\x82\x01R`\x06a\x07\xCF\x19Z\x01\xFA\x80\x15aG7W\x15aG\x8CWV[c\xD4\xB6\x8F\xD7`\xE0\x1B_R`\x04_\xFD[`@QaG\xA7\x81a\x03rV[`@\x90\x81QaG\xB6\x83\x82a\x03\xADV[\x826\x827\x81R` \x82Q\x91aG\xCB\x84\x84a\x03\xADV[\x836\x847\x01R\x80QaG\xDD\x82\x82a\x03\xADV[\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED` \x82\x01R\x81Q\x90aH3\x83\x83a\x03\xADV[\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x01RaH\x88\x83Q\x93\x84a\x03\xADV[\x82R` \x82\x01R\x90V[_Q` aN\xCF_9_Q\x90_R\x90aH\xA9aF\xAFV[P_\x91\x90\x06` `\xC0\x83[aI\xA9W_\x93_Q` aN\xCF_9_Q\x90_R`\x03\x81\x86\x81\x81\x80\t\t\x08`@QaH\xDF\x85\x82a\x03\xADV[\x846\x827\x84\x81\x85`@QaH\xF3\x82\x82a\x03\xADV[\x816\x827\x83\x81R\x83` \x82\x01R\x83`@\x82\x01R\x85``\x82\x01R\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\x80\x82\x01R_Q` aN\xCF_9_Q\x90_R`\xA0\x82\x01R`\x05a\x07\xCF\x19Z\x01\xFA\x80\x15aG7WaI]\x90aN\xB8V[Q\x91aI\xA9W_Q` aN\xCF_9_Q\x90_R\x82\x80\t\x14aI\x94WP_Q` aN\xCF_9_Q\x90_R`\x01_\x94\x08\x92\x93aH\xB4V[\x92\x93PPaI\xA0a\x03\xE0V[\x92\x83R\x82\x01R\x90V[a%UV[aI\xB6aF\xAFV[P`@QaI\xC3\x81a\x03rV[`\x01\x81R`\x02` \x82\x01R\x90V[\x90`\x0C\x81\x10\x15a%PW`\x05\x1B\x01\x90V[\x93\x92\x90\x91aI\xF0`@a\x04\rV[\x94\x85R` \x85\x01RaJ\x02`@a\x04\rV[\x91\x82R` \x82\x01RaJ\x12aF\xC7V[\x92_[`\x02\x81\x10aJ?WPPP` a\x01\x80\x92aJ.aF\xDDV[\x93\x84\x91`\x08b\x01\xD4\xC0\xFA\x91Q\x15\x15\x90V[\x80aJK`\x01\x92aC\x97V[aJU\x82\x85a%?V[QQaJa\x82\x89aI\xD1V[R` aJn\x83\x86a%?V[Q\x01QaJ\x83aJ}\x83a3\x1DV[\x89aI\xD1V[RaJ\x8E\x82\x86a%?V[QQQaJ\x9DaJ}\x83a3+V[RaJ\xB3aJ\xAB\x83\x87a%?V[QQ` \x01\x90V[QaJ\xC0aJ}\x83a39V[R` aJ\xCD\x83\x87a%?V[Q\x01QQaJ\xDDaJ}\x83a3GV[RaK\taK\x03aJ\xFC` aJ\xF3\x86\x8Aa%?V[Q\x01Q` \x01\x90V[Q\x92a3UV[\x88aI\xD1V[R\x01aJ\x15V[` \x7F@\xE4\xED\x88\n)\xE0\xF6\xDD\xCE0tW\xFBu\xCD\xDFO\xEE\xF7\xD3\xEC\xB00\x1B\xFD\xF4\x97j\x0E-\xFC\x91\x15\x15`\xFF\x19`\x97T\x16`\xFF\x82\x16\x17`\x97U`@Q\x90\x81R\xA1V[\x90`\x01aK\\`\xFF\x93aN@V[\x92\x83\x92\x16\x1B\x11\x15aKjW\x90V[c\xCA\x95s3`\xE0\x1B_R`\x04_\xFD[\x80_\x91[aK\x85WP\x90V[_\x19\x81\x01\x81\x81\x11a-eWa\xFF\xFF\x91\x16\x91\x16a\xFF\xFF\x81\x14a-eW`\x01\x01\x90\x80aK}V[\x90aK\xB3aF\xAFV[Pa\xFF\xFF\x81\x16\x90a\x02\0\x82\x10\x15aLXW`\x01\x82\x14aLSWaK\xD4a\x03\xE0V[_\x81R_` \x82\x01R\x92\x90`\x01\x90_\x92[a\xFF\xFF\x83\x16\x85\x10\x15aK\xF9WPPPPP\x90V[`\x01a\xFF\xFF\x83\x16`\xFF\x86\x16\x1C\x81\x16\x14aL3W[`\x01aL)aL\x1E\x83`\xFF\x94aG9V[\x94`\x01\x1Ba\xFF\xFE\x16\x90V[\x94\x01\x16\x92\x91aK\xE5V[\x94`\x01aL)aL\x1EaLH\x89`\xFF\x95aG9V[\x98\x93PPPPaL\rV[PP\x90V[c\x7F\xC4\xEA}`\xE1\x1B_R`\x04_\xFD[aLoaF\xAFV[P\x80Q\x90\x81\x15\x80aL\xE0W[\x15aL\x9CWPP`@QaL\x90`@\x82a\x03\xADV[_\x81R_` \x82\x01R\x90V[` _Q` aN\xCF_9_Q\x90_R\x91\x01Q\x06_Q` aN\xCF_9_Q\x90_R\x03_Q` aN\xCF_9_Q\x90_R\x81\x11a-eW`@Q\x91aH\x88\x83a\x03rV[P` \x81\x01Q\x15aL{V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03aM\0WV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[a\xFF\xFFaM\x98\x82aKyV[\x16aM\xA2\x81a\x07\xA6V[\x90aM\xB0`@Q\x92\x83a\x03\xADV[\x80\x82RaM\xBF`\x1F\x19\x91a\x07\xA6V[\x016` \x83\x017__[\x82Q\x82\x10\x80aN\x1FW[\x15aN\x18W`\x01\x81\x1B\x84\x16aM\xF1W[aM\xEC\x90a-\xD0V[aM\xC9V[\x90`\x01aM\xEC\x91`\xFF`\xF8\x1B\x84`\xF8\x1B\x16_\x1AaN\x0E\x82\x87a'\xE9V[S\x01\x91\x90PaM\xE3V[PP\x90P\x90V[Pa\x01\0\x81\x10aM\xD3V[\x15aN1WV[c\x10\x19\x10i`\xE3\x1B_R`\x04_\xFD[\x90a\x01\0\x82Q\x11aN\xA9W\x81Q\x15aN\xA4W` \x82\x01Q`\x01\x90`\xF8\x1C\x81\x90\x1B[\x83Q\x82\x10\x15aN\x9FW`\x01\x90aN\x8AaN\x80a\x19\x8Ca)\xAF\x86\x89a'\xE9V[`\xFF`\x01\x91\x16\x1B\x90V[\x90aN\x96\x81\x83\x11aN*V[\x17\x91\x01\x90aNaV[\x92PPV[_\x91PV[c}\xA5NG`\xE1\x1B_R`\x04_\xFD[\x15aN\xBFWV[c\xD5\x1E\xDA\xE3`\xE0\x1B_R`\x04_\xFD\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 %\x19\x91\xCF$w\xAEY\x02\xFB\xDD=\x7F\xDC\xF1X\xDF\xC6\xBC6\xBF\x12\xB1\x91\x1E\xDA\xEC\xB9\xFD\xF6\xCF\xA0dsolcC\0\x08\x1B\x003",
    );
    /**Custom error with signature `BitmapValueTooLarge()` and selector `0xca957333`.
    ```solidity
    error BitmapValueTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BitmapValueTooLarge {}
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
        impl ::core::convert::From<BitmapValueTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: BitmapValueTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BitmapValueTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BitmapValueTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BitmapValueTooLarge()";
            const SELECTOR: [u8; 4] = [202u8, 149u8, 115u8, 51u8];
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
    /**Custom error with signature `BytesArrayLengthTooLong()` and selector `0xfb4a9c8e`.
    ```solidity
    error BytesArrayLengthTooLong();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayLengthTooLong {}
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
        impl ::core::convert::From<BytesArrayLengthTooLong> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayLengthTooLong) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayLengthTooLong {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayLengthTooLong {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayLengthTooLong()";
            const SELECTOR: [u8; 4] = [251u8, 74u8, 156u8, 142u8];
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
    /**Custom error with signature `BytesArrayNotOrdered()` and selector `0x80c88348`.
    ```solidity
    error BytesArrayNotOrdered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct BytesArrayNotOrdered {}
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
        impl ::core::convert::From<BytesArrayNotOrdered> for UnderlyingRustTuple<'_> {
            fn from(value: BytesArrayNotOrdered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for BytesArrayNotOrdered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for BytesArrayNotOrdered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "BytesArrayNotOrdered()";
            const SELECTOR: [u8; 4] = [128u8, 200u8, 131u8, 72u8];
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
    /**Custom error with signature `ECAddFailed()` and selector `0xd4b68fd7`.
    ```solidity
    error ECAddFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECAddFailed {}
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
        impl ::core::convert::From<ECAddFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECAddFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECAddFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECAddFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECAddFailed()";
            const SELECTOR: [u8; 4] = [212u8, 182u8, 143u8, 215u8];
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
    /**Custom error with signature `ECMulFailed()` and selector `0x4633be32`.
    ```solidity
    error ECMulFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ECMulFailed {}
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
        impl ::core::convert::From<ECMulFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ECMulFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ECMulFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ECMulFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ECMulFailed()";
            const SELECTOR: [u8; 4] = [70u8, 51u8, 190u8, 50u8];
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
    /**Custom error with signature `ExpModFailed()` and selector `0xd51edae3`.
    ```solidity
    error ExpModFailed();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ExpModFailed {}
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
        impl ::core::convert::From<ExpModFailed> for UnderlyingRustTuple<'_> {
            fn from(value: ExpModFailed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ExpModFailed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ExpModFailed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ExpModFailed()";
            const SELECTOR: [u8; 4] = [213u8, 30u8, 218u8, 227u8];
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
    /**Custom error with signature `InputEmptyQuorumNumbers()` and selector `0x1f0405a0`.
    ```solidity
    error InputEmptyQuorumNumbers();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputEmptyQuorumNumbers {}
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
        impl ::core::convert::From<InputEmptyQuorumNumbers> for UnderlyingRustTuple<'_> {
            fn from(value: InputEmptyQuorumNumbers) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputEmptyQuorumNumbers {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputEmptyQuorumNumbers {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputEmptyQuorumNumbers()";
            const SELECTOR: [u8; 4] = [31u8, 4u8, 5u8, 160u8];
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
    /**Custom error with signature `InputNonSignerLengthMismatch()` and selector `0x5f832f41`.
    ```solidity
    error InputNonSignerLengthMismatch();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InputNonSignerLengthMismatch {}
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
        impl ::core::convert::From<InputNonSignerLengthMismatch> for UnderlyingRustTuple<'_> {
            fn from(value: InputNonSignerLengthMismatch) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InputNonSignerLengthMismatch {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InputNonSignerLengthMismatch {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InputNonSignerLengthMismatch()";
            const SELECTOR: [u8; 4] = [95u8, 131u8, 47u8, 65u8];
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
    /**Custom error with signature `InvalidBLSPairingKey()` and selector `0x67988d33`.
    ```solidity
    error InvalidBLSPairingKey();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSPairingKey {}
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
        impl ::core::convert::From<InvalidBLSPairingKey> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSPairingKey) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSPairingKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSPairingKey {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSPairingKey()";
            const SELECTOR: [u8; 4] = [103u8, 152u8, 141u8, 51u8];
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
    /**Custom error with signature `InvalidBLSSignature()` and selector `0xab1b236b`.
    ```solidity
    error InvalidBLSSignature();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidBLSSignature {}
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
        impl ::core::convert::From<InvalidBLSSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBLSSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBLSSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBLSSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBLSSignature()";
            const SELECTOR: [u8; 4] = [171u8, 27u8, 35u8, 107u8];
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
    /**Custom error with signature `InvalidQuorumApkHash()` and selector `0xe1310aed`.
    ```solidity
    error InvalidQuorumApkHash();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidQuorumApkHash {}
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
        impl ::core::convert::From<InvalidQuorumApkHash> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidQuorumApkHash) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidQuorumApkHash {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidQuorumApkHash {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidQuorumApkHash()";
            const SELECTOR: [u8; 4] = [225u8, 49u8, 10u8, 237u8];
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
    /**Custom error with signature `InvalidReferenceBlocknumber()` and selector `0x4b874f45`.
    ```solidity
    error InvalidReferenceBlocknumber();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidReferenceBlocknumber {}
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
        impl ::core::convert::From<InvalidReferenceBlocknumber> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidReferenceBlocknumber) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidReferenceBlocknumber {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidReferenceBlocknumber {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidReferenceBlocknumber()";
            const SELECTOR: [u8; 4] = [75u8, 135u8, 79u8, 69u8];
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
    /**Custom error with signature `NonSignerPubkeysNotSorted()` and selector `0xff719414`.
    ```solidity
    error NonSignerPubkeysNotSorted();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct NonSignerPubkeysNotSorted {}
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
        impl ::core::convert::From<NonSignerPubkeysNotSorted> for UnderlyingRustTuple<'_> {
            fn from(value: NonSignerPubkeysNotSorted) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonSignerPubkeysNotSorted {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonSignerPubkeysNotSorted {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonSignerPubkeysNotSorted()";
            const SELECTOR: [u8; 4] = [255u8, 113u8, 148u8, 20u8];
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
    /**Custom error with signature `OnlyRegistryCoordinatorOwner()` and selector `0xe0e1e762`.
    ```solidity
    error OnlyRegistryCoordinatorOwner();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OnlyRegistryCoordinatorOwner {}
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
        impl ::core::convert::From<OnlyRegistryCoordinatorOwner> for UnderlyingRustTuple<'_> {
            fn from(value: OnlyRegistryCoordinatorOwner) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OnlyRegistryCoordinatorOwner {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OnlyRegistryCoordinatorOwner {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OnlyRegistryCoordinatorOwner()";
            const SELECTOR: [u8; 4] = [224u8, 225u8, 231u8, 98u8];
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
    /**Custom error with signature `OperatorNotRegistered()` and selector `0x25ec6c1f`.
    ```solidity
    error OperatorNotRegistered();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegistered {}
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
        impl ::core::convert::From<OperatorNotRegistered> for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegistered) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorNotRegistered {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegistered {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegistered()";
            const SELECTOR: [u8; 4] = [37u8, 236u8, 108u8, 31u8];
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
    /**Custom error with signature `ScalarTooLarge()` and selector `0xff89d4fa`.
    ```solidity
    error ScalarTooLarge();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct ScalarTooLarge {}
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
        impl ::core::convert::From<ScalarTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ScalarTooLarge) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ScalarTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ScalarTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ScalarTooLarge()";
            const SELECTOR: [u8; 4] = [255u8, 137u8, 212u8, 250u8];
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
    /**Custom error with signature `StaleStakesForbidden()` and selector `0xaffc5edb`.
    ```solidity
    error StaleStakesForbidden();
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StaleStakesForbidden {}
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
        impl ::core::convert::From<StaleStakesForbidden> for UnderlyingRustTuple<'_> {
            fn from(value: StaleStakesForbidden) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StaleStakesForbidden {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for StaleStakesForbidden {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "StaleStakesForbidden()";
            const SELECTOR: [u8; 4] = [175u8, 252u8, 94u8, 219u8];
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
    /**Event with signature `NewTaskCreated(uint32,((string,string),uint32,bytes,uint32))` and selector `0xa6b1912d066fd3a5413a6222a5785ae43f242458f3eef45c50dd808987b878fd`.
    ```solidity
    event NewTaskCreated(uint32 indexed taskIndex, IAwesomeVaultTaskManager.Task task);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct NewTaskCreated {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub task: <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for NewTaskCreated {
            type DataTuple<'a> = (IAwesomeVaultTaskManager::Task,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str =
                "NewTaskCreated(uint32,((string,string),uint32,bytes,uint32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    166u8, 177u8, 145u8, 45u8, 6u8, 111u8, 211u8, 165u8, 65u8, 58u8, 98u8, 34u8,
                    165u8, 120u8, 90u8, 228u8, 63u8, 36u8, 36u8, 88u8, 243u8, 238u8, 244u8, 92u8,
                    80u8, 221u8, 128u8, 137u8, 135u8, 184u8, 120u8, 253u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    task: data.0,
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
                    <IAwesomeVaultTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for NewTaskCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&NewTaskCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &NewTaskCreated) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `StaleStakesForbiddenUpdate(bool)` and selector `0x40e4ed880a29e0f6ddce307457fb75cddf4feef7d3ecb0301bfdf4976a0e2dfc`.
    ```solidity
    event StaleStakesForbiddenUpdate(bool value);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StaleStakesForbiddenUpdate {
        #[allow(missing_docs)]
        pub value: bool,
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
        impl alloy_sol_types::SolEvent for StaleStakesForbiddenUpdate {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StaleStakesForbiddenUpdate(bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                    87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8,
                    48u8, 27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { value: data.0 }
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
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.value,
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
        impl alloy_sol_types::private::IntoLogData for StaleStakesForbiddenUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StaleStakesForbiddenUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StaleStakesForbiddenUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskChallengedSuccessfully(uint32,address)` and selector `0xc20d1bb0f1623680306b83d4ff4bb99a2beb9d86d97832f3ca40fd13a29df1ec`.
    ```solidity
    event TaskChallengedSuccessfully(uint32 indexed taskIndex, address indexed challenger);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedSuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedSuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedSuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                    255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8,
                    243u8, 202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedSuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedSuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskChallengedSuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskChallengedUnsuccessfully(uint32,address)` and selector `0xfd3e26beeb5967fc5a57a0446914eabc45b4aa474c67a51b4b5160cac60ddb05`.
    ```solidity
    event TaskChallengedUnsuccessfully(uint32 indexed taskIndex, address indexed challenger);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskChallengedUnsuccessfully {
        #[allow(missing_docs)]
        pub taskIndex: u32,
        #[allow(missing_docs)]
        pub challenger: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for TaskChallengedUnsuccessfully {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "TaskChallengedUnsuccessfully(uint32,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                    105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                    75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
                    challenger: topics.2,
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
                    self.taskIndex.clone(),
                    self.challenger.clone(),
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.challenger,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskChallengedUnsuccessfully {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskChallengedUnsuccessfully> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskChallengedUnsuccessfully) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskCompleted(uint32)` and selector `0x9a144f228a931b9d0d1696fbcdaf310b24b5d2d21e799db623fc986a0f547430`.
    ```solidity
    event TaskCompleted(uint32 indexed taskIndex);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskCompleted {
        #[allow(missing_docs)]
        pub taskIndex: u32,
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
        impl alloy_sol_types::SolEvent for TaskCompleted {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            const SIGNATURE: &'static str = "TaskCompleted(uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                    205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                    35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskIndex: topics.1,
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
                (Self::SIGNATURE_HASH.into(), self.taskIndex.clone())
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
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.taskIndex);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for TaskCompleted {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskCompleted> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskCompleted) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `TaskResponded((uint32,bytes32),(uint32,bytes32))` and selector `0xf2af11fad73d4349c99cf62f298d337641ea0bb7c0f5a8db92a98a275f734f58`.
    ```solidity
    event TaskResponded(IAwesomeVaultTaskManager.TaskResponse taskResponse, IAwesomeVaultTaskManager.TaskResponseMetadata taskResponseMetadata);
    ```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct TaskResponded {
        #[allow(missing_docs)]
        pub taskResponse:
            <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata:
            <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for TaskResponded {
            type DataTuple<'a> = (
                IAwesomeVaultTaskManager::TaskResponse,
                IAwesomeVaultTaskManager::TaskResponseMetadata,
            );
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "TaskResponded((uint32,bytes32),(uint32,bytes32))";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    242u8, 175u8, 17u8, 250u8, 215u8, 61u8, 67u8, 73u8, 201u8, 156u8, 246u8, 47u8,
                    41u8, 141u8, 51u8, 118u8, 65u8, 234u8, 11u8, 183u8, 192u8, 245u8, 168u8, 219u8,
                    146u8, 169u8, 138u8, 39u8, 95u8, 115u8, 79u8, 88u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    taskResponse: data.0,
                    taskResponseMetadata: data.1,
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
                    <IAwesomeVaultTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
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
        impl alloy_sol_types::private::IntoLogData for TaskResponded {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&TaskResponded> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &TaskResponded) -> alloy_sol_types::private::LogData {
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
    constructor(address _registryCoordinator, address _pauserRegistry, uint32 _taskResponseWindowBlock);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        #[allow(missing_docs)]
        pub _registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _taskResponseWindowBlock: u32,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
                        value._registryCoordinator,
                        value._pauserRegistry,
                        value._taskResponseWindowBlock,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _pauserRegistry: tuple.1,
                        _taskResponseWindowBlock: tuple.2,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._taskResponseWindowBlock,
                    ),
                )
            }
        }
    };
    /**Function with signature `TASK_CHALLENGE_WINDOW_BLOCK()` and selector `0xf63c5bab`.
    ```solidity
    function TASK_CHALLENGE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_CHALLENGE_WINDOW_BLOCK()`](TASK_CHALLENGE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_CHALLENGE_WINDOW_BLOCKReturn {
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKCall {
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
            impl ::core::convert::From<TASK_CHALLENGE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_CHALLENGE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_CHALLENGE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_CHALLENGE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_CHALLENGE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_CHALLENGE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [246u8, 60u8, 91u8, 171u8];
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
    /**Function with signature `TASK_RESPONSE_WINDOW_BLOCK()` and selector `0x1ad43189`.
    ```solidity
    function TASK_RESPONSE_WINDOW_BLOCK() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKCall {}
    ///Container type for the return parameters of the [`TASK_RESPONSE_WINDOW_BLOCK()`](TASK_RESPONSE_WINDOW_BLOCKCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct TASK_RESPONSE_WINDOW_BLOCKReturn {
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKCall> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKCall {
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
            impl ::core::convert::From<TASK_RESPONSE_WINDOW_BLOCKReturn> for UnderlyingRustTuple<'_> {
                fn from(value: TASK_RESPONSE_WINDOW_BLOCKReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for TASK_RESPONSE_WINDOW_BLOCKReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for TASK_RESPONSE_WINDOW_BLOCKCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = TASK_RESPONSE_WINDOW_BLOCKReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TASK_RESPONSE_WINDOW_BLOCK()";
            const SELECTOR: [u8; 4] = [26u8, 212u8, 49u8, 137u8];
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
    /**Function with signature `WADS_TO_SLASH()` and selector `0x5a2d7f02`.
    ```solidity
    function WADS_TO_SLASH() external view returns (uint256);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHCall {}
    ///Container type for the return parameters of the [`WADS_TO_SLASH()`](WADS_TO_SLASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WADS_TO_SLASHReturn {
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
            impl ::core::convert::From<WADS_TO_SLASHCall> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHCall {
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
            impl ::core::convert::From<WADS_TO_SLASHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: WADS_TO_SLASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WADS_TO_SLASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WADS_TO_SLASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = WADS_TO_SLASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WADS_TO_SLASH()";
            const SELECTOR: [u8; 4] = [90u8, 45u8, 127u8, 2u8];
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
    /**Function with signature `aggregator()` and selector `0x245a7bfc`.
    ```solidity
    function aggregator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorCall {}
    ///Container type for the return parameters of the [`aggregator()`](aggregatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct aggregatorReturn {
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
            impl ::core::convert::From<aggregatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorCall {
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
            impl ::core::convert::From<aggregatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: aggregatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for aggregatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for aggregatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = aggregatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "aggregator()";
            const SELECTOR: [u8; 4] = [36u8, 90u8, 123u8, 252u8];
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
    /**Function with signature `allTaskHashes(uint32)` and selector `0x2d89f6fc`.
    ```solidity
    function allTaskHashes(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskHashes(uint32)`](allTaskHashesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskHashesReturn {
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
            impl ::core::convert::From<allTaskHashesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<allTaskHashesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskHashesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskHashesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskHashesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskHashesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskHashes(uint32)";
            const SELECTOR: [u8; 4] = [45u8, 137u8, 246u8, 252u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `allTaskResponses(uint32)` and selector `0x2cb223d5`.
    ```solidity
    function allTaskResponses(uint32) external view returns (bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`allTaskResponses(uint32)`](allTaskResponsesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allTaskResponsesReturn {
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
            impl ::core::convert::From<allTaskResponsesCall> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<allTaskResponsesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allTaskResponsesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allTaskResponsesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allTaskResponsesCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allTaskResponsesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allTaskResponses(uint32)";
            const SELECTOR: [u8; 4] = [44u8, 178u8, 35u8, 213u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `allocationManager()` and selector `0xca8aa7c7`.
    ```solidity
    function allocationManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerCall {}
    ///Container type for the return parameters of the [`allocationManager()`](allocationManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct allocationManagerReturn {
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
            impl ::core::convert::From<allocationManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerCall {
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
            impl ::core::convert::From<allocationManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allocationManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allocationManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allocationManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = allocationManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allocationManager()";
            const SELECTOR: [u8; 4] = [202u8, 138u8, 167u8, 199u8];
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
    /**Function with signature `blsApkRegistry()` and selector `0x5df45946`.
    ```solidity
    function blsApkRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryCall {}
    ///Container type for the return parameters of the [`blsApkRegistry()`](blsApkRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct blsApkRegistryReturn {
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
            impl ::core::convert::From<blsApkRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryCall {
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
            impl ::core::convert::From<blsApkRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: blsApkRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for blsApkRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for blsApkRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = blsApkRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "blsApkRegistry()";
            const SELECTOR: [u8; 4] = [93u8, 244u8, 89u8, 70u8];
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
    /**Function with signature `checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x6efb4636`.
    ```solidity
    function checkSignatures(bytes32 msgHash, bytes memory quorumNumbers, uint32 referenceBlockNumber, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory params) external view returns (IBLSSignatureCheckerTypes.QuorumStakeTotals memory, bytes32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](checkSignaturesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct checkSignaturesReturn {
        #[allow(missing_docs)]
        pub _0:
            <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::FixedBytes<32>,
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
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::Bytes,
                u32,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkSignaturesCall> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesCall) -> Self {
                    (
                        value.msgHash,
                        value.quorumNumbers,
                        value.referenceBlockNumber,
                        value.params,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        quorumNumbers: tuple.1,
                        referenceBlockNumber: tuple.2,
                        params: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IBLSSignatureCheckerTypes::QuorumStakeTotals as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<checkSignaturesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: checkSignaturesReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for checkSignaturesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for checkSignaturesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = checkSignaturesReturn;
            type ReturnTuple<'a> = (
                IBLSSignatureCheckerTypes::QuorumStakeTotals,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "checkSignatures(bytes32,bytes,uint32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [110u8, 251u8, 70u8, 54u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `createNewTask((string,string),uint32,bytes)` and selector `0xef029dbc`.
    ```solidity
    function createNewTask(IAwesomeVaultTaskManager.TaskInput memory input, uint32 quorumThresholdPercentage, bytes memory quorumNumbers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskCall {
        #[allow(missing_docs)]
        pub input: <IAwesomeVaultTaskManager::TaskInput as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub quorumThresholdPercentage: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`createNewTask((string,string),uint32,bytes)`](createNewTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createNewTaskReturn {}
    #[allow(
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
                IAwesomeVaultTaskManager::TaskInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAwesomeVaultTaskManager::TaskInput as alloy::sol_types::SolType>::RustType,
                u32,
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
            impl ::core::convert::From<createNewTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskCall) -> Self {
                    (
                        value.input,
                        value.quorumThresholdPercentage,
                        value.quorumNumbers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        input: tuple.0,
                        quorumThresholdPercentage: tuple.1,
                        quorumNumbers: tuple.2,
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
            impl ::core::convert::From<createNewTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: createNewTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for createNewTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createNewTaskCall {
            type Parameters<'a> = (
                IAwesomeVaultTaskManager::TaskInput,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = createNewTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createNewTask((string,string),uint32,bytes)";
            const SELECTOR: [u8; 4] = [239u8, 2u8, 157u8, 188u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAwesomeVaultTaskManager::TaskInput as alloy_sol_types::SolType>::tokenize(
                        &self.input,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumThresholdPercentage,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
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
    /**Function with signature `generator()` and selector `0x7afa1eed`.
    ```solidity
    function generator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorCall {}
    ///Container type for the return parameters of the [`generator()`](generatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct generatorReturn {
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
            impl ::core::convert::From<generatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: generatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorCall {
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
            impl ::core::convert::From<generatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: generatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for generatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for generatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = generatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "generator()";
            const SELECTOR: [u8; 4] = [122u8, 250u8, 30u8, 237u8];
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
    /**Function with signature `getBatchOperatorFromId(address,bytes32[])` and selector `0x4d2b57fe`.
    ```solidity
    function getBatchOperatorFromId(address registryCoordinator, bytes32[] memory operatorIds) external view returns (address[] memory operators);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getBatchOperatorFromId(address,bytes32[])`](getBatchOperatorFromIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorFromIdReturn {
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getBatchOperatorFromIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdCall) -> Self {
                    (value.registryCoordinator, value.operatorIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
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
            impl ::core::convert::From<getBatchOperatorFromIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorFromIdReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorFromIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorFromIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorFromIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorFromId(address,bytes32[])";
            const SELECTOR: [u8; 4] = [77u8, 43u8, 87u8, 254u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
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
    /**Function with signature `getBatchOperatorId(address,address[])` and selector `0x31b36bd9`.
    ```solidity
    function getBatchOperatorId(address registryCoordinator, address[] memory operators) external view returns (bytes32[] memory operatorIds);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
    }
    ///Container type for the return parameters of the [`getBatchOperatorId(address,address[])`](getBatchOperatorIdCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getBatchOperatorIdReturn {
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getBatchOperatorIdCall> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdCall) -> Self {
                    (value.registryCoordinator, value.operators)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operators: tuple.1,
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
            impl ::core::convert::From<getBatchOperatorIdReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getBatchOperatorIdReturn) -> Self {
                    (value.operatorIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getBatchOperatorIdReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorIds: tuple.0,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getBatchOperatorIdCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getBatchOperatorIdReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getBatchOperatorId(address,address[])";
            const SELECTOR: [u8; 4] = [49u8, 179u8, 107u8, 217u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Address,
                    > as alloy_sol_types::SolType>::tokenize(&self.operators),
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
    /**Function with signature `getCheckSignaturesIndices(address,uint32,bytes,bytes32[])` and selector `0x4f739f74`.
    ```solidity
    function getCheckSignaturesIndices(address registryCoordinator, uint32 referenceBlockNumber, bytes memory quorumNumbers, bytes32[] memory nonSignerOperatorIds) external view returns (OperatorStateRetriever.CheckSignaturesIndices memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub referenceBlockNumber: u32,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub nonSignerOperatorIds:
            alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
    }
    ///Container type for the return parameters of the [`getCheckSignaturesIndices(address,uint32,bytes,bytes32[])`](getCheckSignaturesIndicesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCheckSignaturesIndicesReturn {
        #[allow(missing_docs)]
        pub _0:
            <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                u32,
                alloy::sol_types::private::Bytes,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getCheckSignaturesIndicesCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.referenceBlockNumber,
                        value.quorumNumbers,
                        value.nonSignerOperatorIds,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        referenceBlockNumber: tuple.1,
                        quorumNumbers: tuple.2,
                        nonSignerOperatorIds: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorStateRetriever::CheckSignaturesIndices as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getCheckSignaturesIndicesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCheckSignaturesIndicesReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCheckSignaturesIndicesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCheckSignaturesIndicesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCheckSignaturesIndicesReturn;
            type ReturnTuple<'a> = (OperatorStateRetriever::CheckSignaturesIndices,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getCheckSignaturesIndices(address,uint32,bytes,bytes32[])";
            const SELECTOR: [u8; 4] = [79u8, 115u8, 159u8, 116u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.referenceBlockNumber),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.nonSignerOperatorIds),
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
    /**Function with signature `getOperatorState(address,bytes,uint32)` and selector `0x3563b0d1`.
    ```solidity
    function getOperatorState(address registryCoordinator, bytes memory quorumNumbers, uint32 blockNumber) external view returns (OperatorStateRetriever.Operator[][] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub quorumNumbers: alloy::sol_types::private::Bytes,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes,uint32)`](getOperatorState_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_0Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Bytes,
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
            impl ::core::convert::From<getOperatorState_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.quorumNumbers,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        quorumNumbers: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorState_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bytes,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes,uint32)";
            const SELECTOR: [u8; 4] = [53u8, 99u8, 176u8, 209u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumbers,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
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
    /**Function with signature `getOperatorState(address,bytes32,uint32)` and selector `0xcefdc1d4`.
    ```solidity
    function getOperatorState(address registryCoordinator, bytes32 operatorId, uint32 blockNumber) external view returns (uint256, OperatorStateRetriever.Operator[][] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Call {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getOperatorState(address,bytes32,uint32)`](getOperatorState_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorState_1Return {
        #[allow(missing_docs)]
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<
                <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<getOperatorState_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Call) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorId,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorId: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::Vec<
                        <OperatorStateRetriever::Operator as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getOperatorState_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorState_1Return) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getOperatorState_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorState_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorState_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<OperatorStateRetriever::Operator>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorState(address,bytes32,uint32)";
            const SELECTOR: [u8; 4] = [206u8, 253u8, 193u8, 212u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)` and selector `0x5c155662`.
    ```solidity
    function getQuorumBitmapsAtBlockNumber(address registryCoordinator, bytes32[] memory operatorIds, uint32 blockNumber) external view returns (uint256[] memory);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberCall {
        #[allow(missing_docs)]
        pub registryCoordinator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        #[allow(missing_docs)]
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)`](getQuorumBitmapsAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getQuorumBitmapsAtBlockNumberReturn {
        #[allow(missing_docs)]
        pub _0:
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberCall) -> Self {
                    (
                        value.registryCoordinator,
                        value.operatorIds,
                        value.blockNumber,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registryCoordinator: tuple.0,
                        operatorIds: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getQuorumBitmapsAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getQuorumBitmapsAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getQuorumBitmapsAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getQuorumBitmapsAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::FixedBytes<32>>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getQuorumBitmapsAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getQuorumBitmapsAtBlockNumber(address,bytes32[],uint32)";
            const SELECTOR: [u8; 4] = [92u8, 21u8, 86u8, 98u8];
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
                        &self.registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorIds),
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
    /**Function with signature `getTaskResponseWindowBlock()` and selector `0xf5c9899d`.
    ```solidity
    function getTaskResponseWindowBlock() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockCall {}
    ///Container type for the return parameters of the [`getTaskResponseWindowBlock()`](getTaskResponseWindowBlockCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTaskResponseWindowBlockReturn {
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
            impl ::core::convert::From<getTaskResponseWindowBlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockCall {
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
            impl ::core::convert::From<getTaskResponseWindowBlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTaskResponseWindowBlockReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTaskResponseWindowBlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTaskResponseWindowBlockCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTaskResponseWindowBlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTaskResponseWindowBlock()";
            const SELECTOR: [u8; 4] = [245u8, 201u8, 137u8, 157u8];
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
    /**Function with signature `initialize(address,address,address,address,address,address)` and selector `0xcc2a9a5b`.
    ```solidity
    function initialize(address initialOwner, address _aggregator, address _generator, address _allocationManager, address _slasher, address _serviceManager) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        #[allow(missing_docs)]
        pub initialOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _aggregator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _generator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _allocationManager: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _slasher: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub _serviceManager: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`initialize(address,address,address,address,address,address)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initialOwner,
                        value._aggregator,
                        value._generator,
                        value._allocationManager,
                        value._slasher,
                        value._serviceManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _aggregator: tuple.1,
                        _generator: tuple.2,
                        _allocationManager: tuple.3,
                        _slasher: tuple.4,
                        _serviceManager: tuple.5,
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
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initialize(address,address,address,address,address,address)";
            const SELECTOR: [u8; 4] = [204u8, 42u8, 154u8, 91u8];
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._aggregator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._generator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._allocationManager,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._slasher,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
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
    /**Function with signature `instantSlasher()` and selector `0x9b290e98`.
    ```solidity
    function instantSlasher() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherCall {}
    ///Container type for the return parameters of the [`instantSlasher()`](instantSlasherCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct instantSlasherReturn {
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
            impl ::core::convert::From<instantSlasherCall> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherCall {
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
            impl ::core::convert::From<instantSlasherReturn> for UnderlyingRustTuple<'_> {
                fn from(value: instantSlasherReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for instantSlasherReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for instantSlasherCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = instantSlasherReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "instantSlasher()";
            const SELECTOR: [u8; 4] = [155u8, 41u8, 14u8, 152u8];
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
    /**Function with signature `latestTaskNum()` and selector `0x8b00ce7c`.
    ```solidity
    function latestTaskNum() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumCall {}
    ///Container type for the return parameters of the [`latestTaskNum()`](latestTaskNumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct latestTaskNumReturn {
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
            impl ::core::convert::From<latestTaskNumCall> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumCall {
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
            impl ::core::convert::From<latestTaskNumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: latestTaskNumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for latestTaskNumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for latestTaskNumCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = latestTaskNumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "latestTaskNum()";
            const SELECTOR: [u8; 4] = [139u8, 0u8, 206u8, 124u8];
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
    /**Function with signature `raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])` and selector `0x92a51824`.
    ```solidity
    function raiseAndResolveChallenge(IAwesomeVaultTaskManager.Task memory task, IAwesomeVaultTaskManager.TaskResponse memory taskResponse, IAwesomeVaultTaskManager.TaskResponseMetadata memory taskResponseMetadata, BN254.G1Point[] memory pubkeysOfNonSigningOperators) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeCall {
        #[allow(missing_docs)]
        pub task: <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse:
            <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponseMetadata:
            <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub pubkeysOfNonSigningOperators:
            alloy::sol_types::private::Vec<<BN254::G1Point as alloy::sol_types::SolType>::RustType>,
    }
    ///Container type for the return parameters of the [`raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])`](raiseAndResolveChallengeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct raiseAndResolveChallengeReturn {}
    #[allow(
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
                IAwesomeVaultTaskManager::Task,
                IAwesomeVaultTaskManager::TaskResponse,
                IAwesomeVaultTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Vec<
                    <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<raiseAndResolveChallengeCall> for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.taskResponseMetadata,
                        value.pubkeysOfNonSigningOperators,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        taskResponseMetadata: tuple.2,
                        pubkeysOfNonSigningOperators: tuple.3,
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
            impl ::core::convert::From<raiseAndResolveChallengeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: raiseAndResolveChallengeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for raiseAndResolveChallengeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for raiseAndResolveChallengeCall {
            type Parameters<'a> = (
                IAwesomeVaultTaskManager::Task,
                IAwesomeVaultTaskManager::TaskResponse,
                IAwesomeVaultTaskManager::TaskResponseMetadata,
                alloy::sol_types::sol_data::Array<BN254::G1Point>,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = raiseAndResolveChallengeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "raiseAndResolveChallenge(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32,bytes32),(uint256,uint256)[])";
            const SELECTOR: [u8; 4] = [146u8, 165u8, 24u8, 36u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAwesomeVaultTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IAwesomeVaultTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponseMetadata,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        BN254::G1Point,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.pubkeysOfNonSigningOperators,
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
    /**Function with signature `registryCoordinator()` and selector `0x6d14a987`.
    ```solidity
    function registryCoordinator() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorCall {}
    ///Container type for the return parameters of the [`registryCoordinator()`](registryCoordinatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registryCoordinatorReturn {
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
            impl ::core::convert::From<registryCoordinatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorCall {
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
            impl ::core::convert::From<registryCoordinatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registryCoordinator()";
            const SELECTOR: [u8; 4] = [109u8, 20u8, 169u8, 135u8];
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
    /**Function with signature `respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xb7023949`.
    ```solidity
    function respondToTask(IAwesomeVaultTaskManager.Task memory task, IAwesomeVaultTaskManager.TaskResponse memory taskResponse, IBLSSignatureCheckerTypes.NonSignerStakesAndSignature memory nonSignerStakesAndSignature) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskCall {
        #[allow(missing_docs)]
        pub task: <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub taskResponse: <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))`](respondToTaskCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct respondToTaskReturn {}
    #[allow(
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
                IAwesomeVaultTaskManager::Task,
                IAwesomeVaultTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
                <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
                <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<respondToTaskCall> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskCall) -> Self {
                    (
                        value.task,
                        value.taskResponse,
                        value.nonSignerStakesAndSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        task: tuple.0,
                        taskResponse: tuple.1,
                        nonSignerStakesAndSignature: tuple.2,
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
            impl ::core::convert::From<respondToTaskReturn> for UnderlyingRustTuple<'_> {
                fn from(value: respondToTaskReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for respondToTaskReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for respondToTaskCall {
            type Parameters<'a> = (
                IAwesomeVaultTaskManager::Task,
                IAwesomeVaultTaskManager::TaskResponse,
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = respondToTaskReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "respondToTask(((string,string),uint32,bytes,uint32),(uint32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))";
            const SELECTOR: [u8; 4] = [183u8, 2u8, 57u8, 73u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <IAwesomeVaultTaskManager::Task as alloy_sol_types::SolType>::tokenize(
                        &self.task,
                    ),
                    <IAwesomeVaultTaskManager::TaskResponse as alloy_sol_types::SolType>::tokenize(
                        &self.taskResponse,
                    ),
                    <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy_sol_types::SolType>::tokenize(
                        &self.nonSignerStakesAndSignature,
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
    /**Function with signature `serviceManager()` and selector `0x3998fdd3`.
    ```solidity
    function serviceManager() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerCall {}
    ///Container type for the return parameters of the [`serviceManager()`](serviceManagerCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct serviceManagerReturn {
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
            impl ::core::convert::From<serviceManagerCall> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerCall {
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
            impl ::core::convert::From<serviceManagerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "serviceManager()";
            const SELECTOR: [u8; 4] = [57u8, 152u8, 253u8, 211u8];
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
    /**Function with signature `setStaleStakesForbidden(bool)` and selector `0x416c7e5e`.
    ```solidity
    function setStaleStakesForbidden(bool value) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenCall {
        #[allow(missing_docs)]
        pub value: bool,
    }
    ///Container type for the return parameters of the [`setStaleStakesForbidden(bool)`](setStaleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStaleStakesForbiddenReturn {}
    #[allow(
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
            impl ::core::convert::From<setStaleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenCall) -> Self {
                    (value.value,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { value: tuple.0 }
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
            impl ::core::convert::From<setStaleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStaleStakesForbiddenReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStaleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStaleStakesForbiddenCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bool,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStaleStakesForbiddenReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStaleStakesForbidden(bool)";
            const SELECTOR: [u8; 4] = [65u8, 108u8, 126u8, 94u8];
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
                        &self.value,
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
    /**Function with signature `stakeRegistry()` and selector `0x68304835`.
    ```solidity
    function stakeRegistry() external view returns (address);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryCall {}
    ///Container type for the return parameters of the [`stakeRegistry()`](stakeRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeRegistryReturn {
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
            impl ::core::convert::From<stakeRegistryCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryCall {
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
            impl ::core::convert::From<stakeRegistryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeRegistryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeRegistryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeRegistryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeRegistry()";
            const SELECTOR: [u8; 4] = [104u8, 48u8, 72u8, 53u8];
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
    /**Function with signature `staleStakesForbidden()` and selector `0xb98d0908`.
    ```solidity
    function staleStakesForbidden() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenCall {}
    ///Container type for the return parameters of the [`staleStakesForbidden()`](staleStakesForbiddenCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct staleStakesForbiddenReturn {
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
            impl ::core::convert::From<staleStakesForbiddenCall> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenCall {
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
            impl ::core::convert::From<staleStakesForbiddenReturn> for UnderlyingRustTuple<'_> {
                fn from(value: staleStakesForbiddenReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for staleStakesForbiddenReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for staleStakesForbiddenCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = staleStakesForbiddenReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "staleStakesForbidden()";
            const SELECTOR: [u8; 4] = [185u8, 141u8, 9u8, 8u8];
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
    /**Function with signature `taskNumber()` and selector `0x72d18e8d`.
    ```solidity
    function taskNumber() external view returns (uint32);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberCall {}
    ///Container type for the return parameters of the [`taskNumber()`](taskNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskNumberReturn {
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
            impl ::core::convert::From<taskNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberCall {
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
            impl ::core::convert::From<taskNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskNumberCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskNumber()";
            const SELECTOR: [u8; 4] = [114u8, 209u8, 142u8, 141u8];
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
    /**Function with signature `taskSuccesfullyChallenged(uint32)` and selector `0x5decc3f5`.
    ```solidity
    function taskSuccesfullyChallenged(uint32) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedCall {
        #[allow(missing_docs)]
        pub _0: u32,
    }
    ///Container type for the return parameters of the [`taskSuccesfullyChallenged(uint32)`](taskSuccesfullyChallengedCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct taskSuccesfullyChallengedReturn {
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
            impl ::core::convert::From<taskSuccesfullyChallengedCall> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<taskSuccesfullyChallengedReturn> for UnderlyingRustTuple<'_> {
                fn from(value: taskSuccesfullyChallengedReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for taskSuccesfullyChallengedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for taskSuccesfullyChallengedCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = taskSuccesfullyChallengedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "taskSuccesfullyChallenged(uint32)";
            const SELECTOR: [u8; 4] = [93u8, 236u8, 195u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`.
    ```solidity
    function trySignatureAndApkVerification(bytes32 msgHash, BN254.G1Point memory apk, BN254.G2Point memory apkG2, BN254.G1Point memory sigma) external view returns (bool pairingSuccessful, bool siganatureIsValid);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationCall {
        #[allow(missing_docs)]
        pub msgHash: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))`](trySignatureAndApkVerificationCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct trySignatureAndApkVerificationReturn {
        #[allow(missing_docs)]
        pub pairingSuccessful: bool,
        #[allow(missing_docs)]
        pub siganatureIsValid: bool,
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
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
                <BN254::G2Point as alloy::sol_types::SolType>::RustType,
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<trySignatureAndApkVerificationCall> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationCall) -> Self {
                    (value.msgHash, value.apk, value.apkG2, value.sigma)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        msgHash: tuple.0,
                        apk: tuple.1,
                        apkG2: tuple.2,
                        sigma: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<trySignatureAndApkVerificationReturn> for UnderlyingRustTuple<'_> {
                fn from(value: trySignatureAndApkVerificationReturn) -> Self {
                    (value.pairingSuccessful, value.siganatureIsValid)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for trySignatureAndApkVerificationReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        pairingSuccessful: tuple.0,
                        siganatureIsValid: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for trySignatureAndApkVerificationCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                BN254::G1Point,
                BN254::G2Point,
                BN254::G1Point,
            );
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = trySignatureAndApkVerificationReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Bool,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))";
            const SELECTOR: [u8; 4] = [23u8, 31u8, 29u8, 91u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.msgHash),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.apk),
                    <BN254::G2Point as alloy_sol_types::SolType>::tokenize(&self.apkG2),
                    <BN254::G1Point as alloy_sol_types::SolType>::tokenize(&self.sigma),
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
    ///Container for all the [`AwesomeVaultTaskManager`](self) function calls.
    pub enum AwesomeVaultTaskManagerCalls {
        #[allow(missing_docs)]
        TASK_CHALLENGE_WINDOW_BLOCK(TASK_CHALLENGE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        TASK_RESPONSE_WINDOW_BLOCK(TASK_RESPONSE_WINDOW_BLOCKCall),
        #[allow(missing_docs)]
        WADS_TO_SLASH(WADS_TO_SLASHCall),
        #[allow(missing_docs)]
        aggregator(aggregatorCall),
        #[allow(missing_docs)]
        allTaskHashes(allTaskHashesCall),
        #[allow(missing_docs)]
        allTaskResponses(allTaskResponsesCall),
        #[allow(missing_docs)]
        allocationManager(allocationManagerCall),
        #[allow(missing_docs)]
        blsApkRegistry(blsApkRegistryCall),
        #[allow(missing_docs)]
        checkSignatures(checkSignaturesCall),
        #[allow(missing_docs)]
        createNewTask(createNewTaskCall),
        #[allow(missing_docs)]
        delegation(delegationCall),
        #[allow(missing_docs)]
        generator(generatorCall),
        #[allow(missing_docs)]
        getBatchOperatorFromId(getBatchOperatorFromIdCall),
        #[allow(missing_docs)]
        getBatchOperatorId(getBatchOperatorIdCall),
        #[allow(missing_docs)]
        getCheckSignaturesIndices(getCheckSignaturesIndicesCall),
        #[allow(missing_docs)]
        getOperatorState_0(getOperatorState_0Call),
        #[allow(missing_docs)]
        getOperatorState_1(getOperatorState_1Call),
        #[allow(missing_docs)]
        getQuorumBitmapsAtBlockNumber(getQuorumBitmapsAtBlockNumberCall),
        #[allow(missing_docs)]
        getTaskResponseWindowBlock(getTaskResponseWindowBlockCall),
        #[allow(missing_docs)]
        initialize(initializeCall),
        #[allow(missing_docs)]
        instantSlasher(instantSlasherCall),
        #[allow(missing_docs)]
        latestTaskNum(latestTaskNumCall),
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
        raiseAndResolveChallenge(raiseAndResolveChallengeCall),
        #[allow(missing_docs)]
        registryCoordinator(registryCoordinatorCall),
        #[allow(missing_docs)]
        renounceOwnership(renounceOwnershipCall),
        #[allow(missing_docs)]
        respondToTask(respondToTaskCall),
        #[allow(missing_docs)]
        serviceManager(serviceManagerCall),
        #[allow(missing_docs)]
        setStaleStakesForbidden(setStaleStakesForbiddenCall),
        #[allow(missing_docs)]
        stakeRegistry(stakeRegistryCall),
        #[allow(missing_docs)]
        staleStakesForbidden(staleStakesForbiddenCall),
        #[allow(missing_docs)]
        taskNumber(taskNumberCall),
        #[allow(missing_docs)]
        taskSuccesfullyChallenged(taskSuccesfullyChallengedCall),
        #[allow(missing_docs)]
        transferOwnership(transferOwnershipCall),
        #[allow(missing_docs)]
        trySignatureAndApkVerification(trySignatureAndApkVerificationCall),
        #[allow(missing_docs)]
        unpause(unpauseCall),
    }
    #[automatically_derived]
    impl AwesomeVaultTaskManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [19u8, 100u8, 57u8, 221u8],
            [23u8, 31u8, 29u8, 91u8],
            [26u8, 212u8, 49u8, 137u8],
            [36u8, 90u8, 123u8, 252u8],
            [44u8, 178u8, 35u8, 213u8],
            [45u8, 137u8, 246u8, 252u8],
            [49u8, 179u8, 107u8, 217u8],
            [53u8, 99u8, 176u8, 209u8],
            [57u8, 152u8, 253u8, 211u8],
            [65u8, 108u8, 126u8, 94u8],
            [77u8, 43u8, 87u8, 254u8],
            [79u8, 115u8, 159u8, 116u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 45u8, 127u8, 2u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 21u8, 86u8, 98u8],
            [92u8, 151u8, 90u8, 187u8],
            [93u8, 236u8, 195u8, 245u8],
            [93u8, 244u8, 89u8, 70u8],
            [104u8, 48u8, 72u8, 53u8],
            [109u8, 20u8, 169u8, 135u8],
            [110u8, 251u8, 70u8, 54u8],
            [113u8, 80u8, 24u8, 166u8],
            [114u8, 209u8, 142u8, 141u8],
            [122u8, 250u8, 30u8, 237u8],
            [136u8, 111u8, 17u8, 149u8],
            [139u8, 0u8, 206u8, 124u8],
            [141u8, 165u8, 203u8, 91u8],
            [146u8, 165u8, 24u8, 36u8],
            [155u8, 41u8, 14u8, 152u8],
            [183u8, 2u8, 57u8, 73u8],
            [185u8, 141u8, 9u8, 8u8],
            [202u8, 138u8, 167u8, 199u8],
            [204u8, 42u8, 154u8, 91u8],
            [206u8, 253u8, 193u8, 212u8],
            [223u8, 92u8, 247u8, 35u8],
            [239u8, 2u8, 157u8, 188u8],
            [242u8, 253u8, 227u8, 139u8],
            [245u8, 201u8, 137u8, 157u8],
            [246u8, 60u8, 91u8, 171u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AwesomeVaultTaskManagerCalls {
        const NAME: &'static str = "AwesomeVaultTaskManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 41usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(_) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(_) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WADS_TO_SLASH(_) => <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::aggregator(_) => <aggregatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allTaskHashes(_) => <allTaskHashesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::allTaskResponses(_) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::allocationManager(_) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::blsApkRegistry(_) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::checkSignatures(_) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createNewTask(_) => <createNewTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::generator(_) => <generatorCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::getBatchOperatorFromId(_) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getBatchOperatorId(_) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCheckSignaturesIndices(_) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_0(_) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorState_1(_) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getQuorumBitmapsAtBlockNumber(_) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTaskResponseWindowBlock(_) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => <initializeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::instantSlasher(_) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::latestTaskNum(_) => <latestTaskNumCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::raiseAndResolveChallenge(_) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::respondToTask(_) => <respondToTaskCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStaleStakesForbidden(_) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeRegistry(_) => <stakeRegistryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::staleStakesForbidden(_) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::taskNumber(_) => <taskNumberCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::taskSuccesfullyChallenged(_) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::trySignatureAndApkVerification(_) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
                -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls>] = &[
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::pause)
                    }
                    pause
                },
                {
                    fn trySignatureAndApkVerification(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultTaskManagerCalls::trySignatureAndApkVerification,
                            )
                    }
                    trySignatureAndApkVerification
                },
                {
                    fn TASK_RESPONSE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultTaskManagerCalls::TASK_RESPONSE_WINDOW_BLOCK,
                            )
                    }
                    TASK_RESPONSE_WINDOW_BLOCK
                },
                {
                    fn aggregator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <aggregatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::aggregator)
                    }
                    aggregator
                },
                {
                    fn allTaskResponses(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::allTaskResponses)
                    }
                    allTaskResponses
                },
                {
                    fn allTaskHashes(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <allTaskHashesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::allTaskHashes)
                    }
                    allTaskHashes
                },
                {
                    fn getBatchOperatorId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::getBatchOperatorId)
                    }
                    getBatchOperatorId
                },
                {
                    fn getOperatorState_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::getOperatorState_0)
                    }
                    getOperatorState_0
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn setStaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::setStaleStakesForbidden)
                    }
                    setStaleStakesForbidden
                },
                {
                    fn getBatchOperatorFromId(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::getBatchOperatorFromId)
                    }
                    getBatchOperatorFromId
                },
                {
                    fn getCheckSignaturesIndices(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::getCheckSignaturesIndices)
                    }
                    getCheckSignaturesIndices
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn WADS_TO_SLASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::WADS_TO_SLASH)
                    }
                    WADS_TO_SLASH
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn getQuorumBitmapsAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultTaskManagerCalls::getQuorumBitmapsAtBlockNumber,
                            )
                    }
                    getQuorumBitmapsAtBlockNumber
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn taskSuccesfullyChallenged(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::taskSuccesfullyChallenged)
                    }
                    taskSuccesfullyChallenged
                },
                {
                    fn blsApkRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::blsApkRegistry)
                    }
                    blsApkRegistry
                },
                {
                    fn stakeRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <stakeRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::stakeRegistry)
                    }
                    stakeRegistry
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn checkSignatures(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <checkSignaturesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::checkSignatures)
                    }
                    checkSignatures
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn taskNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <taskNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::taskNumber)
                    }
                    taskNumber
                },
                {
                    fn generator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <generatorCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::generator)
                    }
                    generator
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn latestTaskNum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <latestTaskNumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::latestTaskNum)
                    }
                    latestTaskNum
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn raiseAndResolveChallenge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::raiseAndResolveChallenge)
                    }
                    raiseAndResolveChallenge
                },
                {
                    fn instantSlasher(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <instantSlasherCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::instantSlasher)
                    }
                    instantSlasher
                },
                {
                    fn respondToTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <respondToTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::respondToTask)
                    }
                    respondToTask
                },
                {
                    fn staleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::staleStakesForbidden)
                    }
                    staleStakesForbidden
                },
                {
                    fn allocationManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <allocationManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::allocationManager)
                    }
                    allocationManager
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn getOperatorState_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::getOperatorState_1)
                    }
                    getOperatorState_1
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::delegation)
                    }
                    delegation
                },
                {
                    fn createNewTask(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <createNewTaskCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::createNewTask)
                    }
                    createNewTask
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn getTaskResponseWindowBlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultTaskManagerCalls::getTaskResponseWindowBlock,
                            )
                    }
                    getTaskResponseWindowBlock
                },
                {
                    fn TASK_CHALLENGE_WINDOW_BLOCK(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AwesomeVaultTaskManagerCalls::TASK_CHALLENGE_WINDOW_BLOCK,
                            )
                    }
                    TASK_CHALLENGE_WINDOW_BLOCK
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerCalls::unpause)
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
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::TASK_CHALLENGE_WINDOW_BLOCK(inner) => {
                    <TASK_CHALLENGE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::TASK_RESPONSE_WINDOW_BLOCK(inner) => {
                    <TASK_RESPONSE_WINDOW_BLOCKCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::WADS_TO_SLASH(inner) => {
                    <WADS_TO_SLASHCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::aggregator(inner) => {
                    <aggregatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskHashes(inner) => {
                    <allTaskHashesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allTaskResponses(inner) => {
                    <allTaskResponsesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::allocationManager(inner) => {
                    <allocationManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::blsApkRegistry(inner) => {
                    <blsApkRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::checkSignatures(inner) => {
                    <checkSignaturesCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::createNewTask(inner) => {
                    <createNewTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::generator(inner) => {
                    <generatorCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getBatchOperatorFromId(inner) => {
                    <getBatchOperatorFromIdCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getBatchOperatorId(inner) => {
                    <getBatchOperatorIdCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getCheckSignaturesIndices(inner) => {
                    <getCheckSignaturesIndicesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getOperatorState_0(inner) => {
                    <getOperatorState_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getOperatorState_1(inner) => {
                    <getOperatorState_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::getQuorumBitmapsAtBlockNumber(inner) => {
                    <getQuorumBitmapsAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::getTaskResponseWindowBlock(inner) => {
                    <getTaskResponseWindowBlockCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::instantSlasher(inner) => {
                    <instantSlasherCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::latestTaskNum(inner) => {
                    <latestTaskNumCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pause(inner) => {
                    <pauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauseAll(inner) => {
                    <pauseAllCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_0(inner) => {
                    <paused_0Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::paused_1(inner) => {
                    <paused_1Call as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::pauserRegistry(inner) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::raiseAndResolveChallenge(inner) => {
                    <raiseAndResolveChallengeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::renounceOwnership(inner) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::respondToTask(inner) => {
                    <respondToTaskCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::setStaleStakesForbidden(inner) => {
                    <setStaleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::stakeRegistry(inner) => {
                    <stakeRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::staleStakesForbidden(inner) => {
                    <staleStakesForbiddenCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::taskNumber(inner) => {
                    <taskNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::taskSuccesfullyChallenged(inner) => {
                    <taskSuccesfullyChallengedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::trySignatureAndApkVerification(inner) => {
                    <trySignatureAndApkVerificationCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::unpause(inner) => {
                    <unpauseCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`AwesomeVaultTaskManager`](self) custom errors.
    pub enum AwesomeVaultTaskManagerErrors {
        #[allow(missing_docs)]
        BitmapValueTooLarge(BitmapValueTooLarge),
        #[allow(missing_docs)]
        BytesArrayLengthTooLong(BytesArrayLengthTooLong),
        #[allow(missing_docs)]
        BytesArrayNotOrdered(BytesArrayNotOrdered),
        #[allow(missing_docs)]
        CurrentlyPaused(CurrentlyPaused),
        #[allow(missing_docs)]
        ECAddFailed(ECAddFailed),
        #[allow(missing_docs)]
        ECMulFailed(ECMulFailed),
        #[allow(missing_docs)]
        ExpModFailed(ExpModFailed),
        #[allow(missing_docs)]
        InputAddressZero(InputAddressZero),
        #[allow(missing_docs)]
        InputArrayLengthMismatch(InputArrayLengthMismatch),
        #[allow(missing_docs)]
        InputEmptyQuorumNumbers(InputEmptyQuorumNumbers),
        #[allow(missing_docs)]
        InputNonSignerLengthMismatch(InputNonSignerLengthMismatch),
        #[allow(missing_docs)]
        InvalidBLSPairingKey(InvalidBLSPairingKey),
        #[allow(missing_docs)]
        InvalidBLSSignature(InvalidBLSSignature),
        #[allow(missing_docs)]
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        #[allow(missing_docs)]
        InvalidQuorumApkHash(InvalidQuorumApkHash),
        #[allow(missing_docs)]
        InvalidReferenceBlocknumber(InvalidReferenceBlocknumber),
        #[allow(missing_docs)]
        NonSignerPubkeysNotSorted(NonSignerPubkeysNotSorted),
        #[allow(missing_docs)]
        OnlyPauser(OnlyPauser),
        #[allow(missing_docs)]
        OnlyRegistryCoordinatorOwner(OnlyRegistryCoordinatorOwner),
        #[allow(missing_docs)]
        OnlyUnpauser(OnlyUnpauser),
        #[allow(missing_docs)]
        OperatorNotRegistered(OperatorNotRegistered),
        #[allow(missing_docs)]
        ScalarTooLarge(ScalarTooLarge),
        #[allow(missing_docs)]
        StaleStakesForbidden(StaleStakesForbidden),
    }
    #[automatically_derived]
    impl AwesomeVaultTaskManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 4u8, 5u8, 160u8],
            [37u8, 236u8, 108u8, 31u8],
            [67u8, 113u8, 74u8, 253u8],
            [70u8, 51u8, 190u8, 50u8],
            [75u8, 135u8, 79u8, 69u8],
            [95u8, 131u8, 47u8, 65u8],
            [103u8, 152u8, 141u8, 51u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [128u8, 200u8, 131u8, 72u8],
            [132u8, 10u8, 72u8, 213u8],
            [171u8, 27u8, 35u8, 107u8],
            [175u8, 252u8, 94u8, 219u8],
            [198u8, 29u8, 202u8, 93u8],
            [202u8, 149u8, 115u8, 51u8],
            [212u8, 182u8, 143u8, 215u8],
            [213u8, 30u8, 218u8, 227u8],
            [224u8, 225u8, 231u8, 98u8],
            [225u8, 49u8, 10u8, 237u8],
            [251u8, 74u8, 156u8, 142u8],
            [255u8, 113u8, 148u8, 20u8],
            [255u8, 137u8, 212u8, 250u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AwesomeVaultTaskManagerErrors {
        const NAME: &'static str = "AwesomeVaultTaskManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 23usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::BitmapValueTooLarge(_) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayLengthTooLong(_) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::SELECTOR
                }
                Self::BytesArrayNotOrdered(_) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ECAddFailed(_) => <ECAddFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ECMulFailed(_) => <ECMulFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::ExpModFailed(_) => <ExpModFailed as alloy_sol_types::SolError>::SELECTOR,
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputArrayLengthMismatch(_) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputEmptyQuorumNumbers(_) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputNonSignerLengthMismatch(_) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSPairingKey(_) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBLSSignature(_) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidNewPausedStatus(_) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidQuorumApkHash(_) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidReferenceBlocknumber(_) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonSignerPubkeysNotSorted(_) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => <OnlyPauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OnlyRegistryCoordinatorOwner(_) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR,
                Self::OperatorNotRegistered(_) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ScalarTooLarge(_) => <ScalarTooLarge as alloy_sol_types::SolError>::SELECTOR,
                Self::StaleStakesForbidden(_) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::SELECTOR
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
                -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>] = &[
                {
                    fn InputEmptyQuorumNumbers(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InputEmptyQuorumNumbers)
                    }
                    InputEmptyQuorumNumbers
                },
                {
                    fn OperatorNotRegistered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <OperatorNotRegistered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::OperatorNotRegistered)
                    }
                    OperatorNotRegistered
                },
                {
                    fn InputArrayLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InputArrayLengthMismatch)
                    }
                    InputArrayLengthMismatch
                },
                {
                    fn ECMulFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <ECMulFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerErrors::ECMulFailed)
                    }
                    ECMulFailed
                },
                {
                    fn InvalidReferenceBlocknumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InvalidReferenceBlocknumber)
                    }
                    InvalidReferenceBlocknumber
                },
                {
                    fn InputNonSignerLengthMismatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InputNonSignerLengthMismatch)
                    }
                    InputNonSignerLengthMismatch
                },
                {
                    fn InvalidBLSPairingKey(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InvalidBLSPairingKey)
                    }
                    InvalidBLSPairingKey
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn BytesArrayNotOrdered(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::BytesArrayNotOrdered)
                    }
                    BytesArrayNotOrdered
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidBLSSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InvalidBLSSignature as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InvalidBLSSignature)
                    }
                    InvalidBLSSignature
                },
                {
                    fn StaleStakesForbidden(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <StaleStakesForbidden as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::StaleStakesForbidden)
                    }
                    StaleStakesForbidden
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn BitmapValueTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::BitmapValueTooLarge)
                    }
                    BitmapValueTooLarge
                },
                {
                    fn ECAddFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <ECAddFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerErrors::ECAddFailed)
                    }
                    ECAddFailed
                },
                {
                    fn ExpModFailed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <ExpModFailed as alloy_sol_types::SolError>::abi_decode_raw(data, validate)
                            .map(AwesomeVaultTaskManagerErrors::ExpModFailed)
                    }
                    ExpModFailed
                },
                {
                    fn OnlyRegistryCoordinatorOwner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::OnlyRegistryCoordinatorOwner)
                    }
                    OnlyRegistryCoordinatorOwner
                },
                {
                    fn InvalidQuorumApkHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::InvalidQuorumApkHash)
                    }
                    InvalidQuorumApkHash
                },
                {
                    fn BytesArrayLengthTooLong(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::BytesArrayLengthTooLong)
                    }
                    BytesArrayLengthTooLong
                },
                {
                    fn NonSignerPubkeysNotSorted(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::NonSignerPubkeysNotSorted)
                    }
                    NonSignerPubkeysNotSorted
                },
                {
                    fn ScalarTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AwesomeVaultTaskManagerErrors>
                    {
                        <ScalarTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                            data, validate,
                        )
                        .map(AwesomeVaultTaskManagerErrors::ScalarTooLarge)
                    }
                    ScalarTooLarge
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
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::BitmapValueTooLarge(inner) => {
                    <BitmapValueTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::BytesArrayLengthTooLong(inner) => {
                    <BytesArrayLengthTooLong as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::BytesArrayNotOrdered(inner) => {
                    <BytesArrayNotOrdered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::CurrentlyPaused(inner) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECAddFailed(inner) => {
                    <ECAddFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ECMulFailed(inner) => {
                    <ECMulFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ExpModFailed(inner) => {
                    <ExpModFailed as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputAddressZero(inner) => {
                    <InputAddressZero as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InputArrayLengthMismatch(inner) => {
                    <InputArrayLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputEmptyQuorumNumbers(inner) => {
                    <InputEmptyQuorumNumbers as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InputNonSignerLengthMismatch(inner) => {
                    <InputNonSignerLengthMismatch as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidBLSPairingKey(inner) => {
                    <InvalidBLSPairingKey as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidBLSSignature(inner) => {
                    <InvalidBLSSignature as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidNewPausedStatus(inner) => {
                    <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::InvalidQuorumApkHash(inner) => {
                    <InvalidQuorumApkHash as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::InvalidReferenceBlocknumber(inner) => {
                    <InvalidReferenceBlocknumber as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::NonSignerPubkeysNotSorted(inner) => {
                    <NonSignerPubkeysNotSorted as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OnlyRegistryCoordinatorOwner(inner) => {
                    <OnlyRegistryCoordinatorOwner as alloy_sol_types::SolError>::abi_encode_raw(
                        inner, out,
                    )
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::OperatorNotRegistered(inner) => {
                    <OperatorNotRegistered as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::ScalarTooLarge(inner) => {
                    <ScalarTooLarge as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::StaleStakesForbidden(inner) => {
                    <StaleStakesForbidden as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    ///Container for all the [`AwesomeVaultTaskManager`](self) events.
    pub enum AwesomeVaultTaskManagerEvents {
        #[allow(missing_docs)]
        Initialized(Initialized),
        #[allow(missing_docs)]
        NewTaskCreated(NewTaskCreated),
        #[allow(missing_docs)]
        OwnershipTransferred(OwnershipTransferred),
        #[allow(missing_docs)]
        Paused(Paused),
        #[allow(missing_docs)]
        StaleStakesForbiddenUpdate(StaleStakesForbiddenUpdate),
        #[allow(missing_docs)]
        TaskChallengedSuccessfully(TaskChallengedSuccessfully),
        #[allow(missing_docs)]
        TaskChallengedUnsuccessfully(TaskChallengedUnsuccessfully),
        #[allow(missing_docs)]
        TaskCompleted(TaskCompleted),
        #[allow(missing_docs)]
        TaskResponded(TaskResponded),
        #[allow(missing_docs)]
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl AwesomeVaultTaskManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                53u8, 130u8, 209u8, 130u8, 142u8, 38u8, 191u8, 86u8, 189u8, 128u8, 21u8, 2u8,
                188u8, 2u8, 26u8, 192u8, 188u8, 138u8, 251u8, 87u8, 200u8, 38u8, 228u8, 152u8,
                107u8, 69u8, 89u8, 60u8, 143u8, 173u8, 56u8, 156u8,
            ],
            [
                64u8, 228u8, 237u8, 136u8, 10u8, 41u8, 224u8, 246u8, 221u8, 206u8, 48u8, 116u8,
                87u8, 251u8, 117u8, 205u8, 223u8, 79u8, 238u8, 247u8, 211u8, 236u8, 176u8, 48u8,
                27u8, 253u8, 244u8, 151u8, 106u8, 14u8, 45u8, 252u8,
            ],
            [
                127u8, 38u8, 184u8, 63u8, 249u8, 110u8, 31u8, 43u8, 106u8, 104u8, 47u8, 19u8, 56u8,
                82u8, 246u8, 121u8, 138u8, 9u8, 196u8, 101u8, 218u8, 149u8, 146u8, 20u8, 96u8,
                206u8, 251u8, 56u8, 71u8, 64u8, 36u8, 152u8,
            ],
            [
                139u8, 224u8, 7u8, 156u8, 83u8, 22u8, 89u8, 20u8, 19u8, 68u8, 205u8, 31u8, 208u8,
                164u8, 242u8, 132u8, 25u8, 73u8, 127u8, 151u8, 34u8, 163u8, 218u8, 175u8, 227u8,
                180u8, 24u8, 111u8, 107u8, 100u8, 87u8, 224u8,
            ],
            [
                154u8, 20u8, 79u8, 34u8, 138u8, 147u8, 27u8, 157u8, 13u8, 22u8, 150u8, 251u8,
                205u8, 175u8, 49u8, 11u8, 36u8, 181u8, 210u8, 210u8, 30u8, 121u8, 157u8, 182u8,
                35u8, 252u8, 152u8, 106u8, 15u8, 84u8, 116u8, 48u8,
            ],
            [
                166u8, 177u8, 145u8, 45u8, 6u8, 111u8, 211u8, 165u8, 65u8, 58u8, 98u8, 34u8, 165u8,
                120u8, 90u8, 228u8, 63u8, 36u8, 36u8, 88u8, 243u8, 238u8, 244u8, 92u8, 80u8, 221u8,
                128u8, 137u8, 135u8, 184u8, 120u8, 253u8,
            ],
            [
                171u8, 64u8, 163u8, 116u8, 188u8, 81u8, 222u8, 55u8, 34u8, 0u8, 168u8, 188u8,
                152u8, 26u8, 248u8, 201u8, 236u8, 220u8, 8u8, 223u8, 218u8, 239u8, 11u8, 182u8,
                224u8, 159u8, 136u8, 243u8, 198u8, 22u8, 239u8, 61u8,
            ],
            [
                194u8, 13u8, 27u8, 176u8, 241u8, 98u8, 54u8, 128u8, 48u8, 107u8, 131u8, 212u8,
                255u8, 75u8, 185u8, 154u8, 43u8, 235u8, 157u8, 134u8, 217u8, 120u8, 50u8, 243u8,
                202u8, 64u8, 253u8, 19u8, 162u8, 157u8, 241u8, 236u8,
            ],
            [
                242u8, 175u8, 17u8, 250u8, 215u8, 61u8, 67u8, 73u8, 201u8, 156u8, 246u8, 47u8,
                41u8, 141u8, 51u8, 118u8, 65u8, 234u8, 11u8, 183u8, 192u8, 245u8, 168u8, 219u8,
                146u8, 169u8, 138u8, 39u8, 95u8, 115u8, 79u8, 88u8,
            ],
            [
                253u8, 62u8, 38u8, 190u8, 235u8, 89u8, 103u8, 252u8, 90u8, 87u8, 160u8, 68u8,
                105u8, 20u8, 234u8, 188u8, 69u8, 180u8, 170u8, 71u8, 76u8, 103u8, 165u8, 27u8,
                75u8, 81u8, 96u8, 202u8, 198u8, 13u8, 219u8, 5u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AwesomeVaultTaskManagerEvents {
        const NAME: &'static str = "AwesomeVaultTaskManagerEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::Initialized)
                }
                Some(<NewTaskCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <NewTaskCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::NewTaskCreated)
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
                Some(<StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StaleStakesForbiddenUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StaleStakesForbiddenUpdate)
                }
                Some(<TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskChallengedSuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskChallengedSuccessfully)
                }
                Some(
                    <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <TaskChallengedUnsuccessfully as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::TaskChallengedUnsuccessfully),
                Some(<TaskCompleted as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskCompleted as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskCompleted)
                }
                Some(<TaskResponded as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <TaskResponded as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::TaskResponded)
                }
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
    impl alloy_sol_types::private::IntoLogData for AwesomeVaultTaskManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Unpaused(inner) => alloy_sol_types::private::IntoLogData::to_log_data(inner),
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::NewTaskCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => alloy_sol_types::private::IntoLogData::into_log_data(inner),
                Self::StaleStakesForbiddenUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedSuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskChallengedUnsuccessfully(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskCompleted(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::TaskResponded(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`AwesomeVaultTaskManager`](self) contract instance.

    See the [wrapper's documentation](`AwesomeVaultTaskManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AwesomeVaultTaskManagerInstance<T, P, N> {
        AwesomeVaultTaskManagerInstance::<T, P, N>::new(address, provider)
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AwesomeVaultTaskManagerInstance<T, P, N>>,
    > {
        AwesomeVaultTaskManagerInstance::<T, P, N>::deploy(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
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
        _registryCoordinator: alloy::sol_types::private::Address,
        _pauserRegistry: alloy::sol_types::private::Address,
        _taskResponseWindowBlock: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AwesomeVaultTaskManagerInstance::<T, P, N>::deploy_builder(
            provider,
            _registryCoordinator,
            _pauserRegistry,
            _taskResponseWindowBlock,
        )
    }
    /**A [`AwesomeVaultTaskManager`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`AwesomeVaultTaskManager`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AwesomeVaultTaskManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AwesomeVaultTaskManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AwesomeVaultTaskManagerInstance")
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
        > AwesomeVaultTaskManagerInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`AwesomeVaultTaskManager`](self) contract instance.

        See the [wrapper's documentation](`AwesomeVaultTaskManagerInstance`) for more details.*/
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::Result<AwesomeVaultTaskManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _pauserRegistry,
                _taskResponseWindowBlock,
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            _taskResponseWindowBlock: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                        _pauserRegistry,
                        _taskResponseWindowBlock,
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
    impl<T, P: ::core::clone::Clone, N> AwesomeVaultTaskManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AwesomeVaultTaskManagerInstance<T, P, N> {
            AwesomeVaultTaskManagerInstance {
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
        > AwesomeVaultTaskManagerInstance<T, P, N>
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
        ///Creates a new call builder for the [`TASK_CHALLENGE_WINDOW_BLOCK`] function.
        pub fn TASK_CHALLENGE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_CHALLENGE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_CHALLENGE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`TASK_RESPONSE_WINDOW_BLOCK`] function.
        pub fn TASK_RESPONSE_WINDOW_BLOCK(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, TASK_RESPONSE_WINDOW_BLOCKCall, N> {
            self.call_builder(&TASK_RESPONSE_WINDOW_BLOCKCall {})
        }
        ///Creates a new call builder for the [`WADS_TO_SLASH`] function.
        pub fn WADS_TO_SLASH(&self) -> alloy_contract::SolCallBuilder<T, &P, WADS_TO_SLASHCall, N> {
            self.call_builder(&WADS_TO_SLASHCall {})
        }
        ///Creates a new call builder for the [`aggregator`] function.
        pub fn aggregator(&self) -> alloy_contract::SolCallBuilder<T, &P, aggregatorCall, N> {
            self.call_builder(&aggregatorCall {})
        }
        ///Creates a new call builder for the [`allTaskHashes`] function.
        pub fn allTaskHashes(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskHashesCall, N> {
            self.call_builder(&allTaskHashesCall { _0 })
        }
        ///Creates a new call builder for the [`allTaskResponses`] function.
        pub fn allTaskResponses(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, allTaskResponsesCall, N> {
            self.call_builder(&allTaskResponsesCall { _0 })
        }
        ///Creates a new call builder for the [`allocationManager`] function.
        pub fn allocationManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, allocationManagerCall, N> {
            self.call_builder(&allocationManagerCall {})
        }
        ///Creates a new call builder for the [`blsApkRegistry`] function.
        pub fn blsApkRegistry(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, blsApkRegistryCall, N> {
            self.call_builder(&blsApkRegistryCall {})
        }
        ///Creates a new call builder for the [`checkSignatures`] function.
        pub fn checkSignatures(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
            referenceBlockNumber: u32,
            params: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, checkSignaturesCall, N> {
            self.call_builder(&checkSignaturesCall {
                msgHash,
                quorumNumbers,
                referenceBlockNumber,
                params,
            })
        }
        ///Creates a new call builder for the [`createNewTask`] function.
        pub fn createNewTask(
            &self,
            input: <IAwesomeVaultTaskManager::TaskInput as alloy::sol_types::SolType>::RustType,
            quorumThresholdPercentage: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, createNewTaskCall, N> {
            self.call_builder(&createNewTaskCall {
                input,
                quorumThresholdPercentage,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`generator`] function.
        pub fn generator(&self) -> alloy_contract::SolCallBuilder<T, &P, generatorCall, N> {
            self.call_builder(&generatorCall {})
        }
        ///Creates a new call builder for the [`getBatchOperatorFromId`] function.
        pub fn getBatchOperatorFromId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorFromIdCall, N> {
            self.call_builder(&getBatchOperatorFromIdCall {
                registryCoordinator,
                operatorIds,
            })
        }
        ///Creates a new call builder for the [`getBatchOperatorId`] function.
        pub fn getBatchOperatorId(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operators: alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
        ) -> alloy_contract::SolCallBuilder<T, &P, getBatchOperatorIdCall, N> {
            self.call_builder(&getBatchOperatorIdCall {
                registryCoordinator,
                operators,
            })
        }
        ///Creates a new call builder for the [`getCheckSignaturesIndices`] function.
        pub fn getCheckSignaturesIndices(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            referenceBlockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
            nonSignerOperatorIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCheckSignaturesIndicesCall, N> {
            self.call_builder(&getCheckSignaturesIndicesCall {
                registryCoordinator,
                referenceBlockNumber,
                quorumNumbers,
                nonSignerOperatorIds,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_0`] function.
        pub fn getOperatorState_0(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            quorumNumbers: alloy::sol_types::private::Bytes,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_0Call, N> {
            self.call_builder(&getOperatorState_0Call {
                registryCoordinator,
                quorumNumbers,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getOperatorState_1`] function.
        pub fn getOperatorState_1(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorState_1Call, N> {
            self.call_builder(&getOperatorState_1Call {
                registryCoordinator,
                operatorId,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getQuorumBitmapsAtBlockNumber`] function.
        pub fn getQuorumBitmapsAtBlockNumber(
            &self,
            registryCoordinator: alloy::sol_types::private::Address,
            operatorIds: alloy::sol_types::private::Vec<alloy::sol_types::private::FixedBytes<32>>,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getQuorumBitmapsAtBlockNumberCall, N> {
            self.call_builder(&getQuorumBitmapsAtBlockNumberCall {
                registryCoordinator,
                operatorIds,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getTaskResponseWindowBlock`] function.
        pub fn getTaskResponseWindowBlock(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTaskResponseWindowBlockCall, N> {
            self.call_builder(&getTaskResponseWindowBlockCall {})
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _aggregator: alloy::sol_types::private::Address,
            _generator: alloy::sol_types::private::Address,
            _allocationManager: alloy::sol_types::private::Address,
            _slasher: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(&initializeCall {
                initialOwner,
                _aggregator,
                _generator,
                _allocationManager,
                _slasher,
                _serviceManager,
            })
        }
        ///Creates a new call builder for the [`instantSlasher`] function.
        pub fn instantSlasher(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, instantSlasherCall, N> {
            self.call_builder(&instantSlasherCall {})
        }
        ///Creates a new call builder for the [`latestTaskNum`] function.
        pub fn latestTaskNum(&self) -> alloy_contract::SolCallBuilder<T, &P, latestTaskNumCall, N> {
            self.call_builder(&latestTaskNumCall {})
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
        ///Creates a new call builder for the [`raiseAndResolveChallenge`] function.
        pub fn raiseAndResolveChallenge(
            &self,
            task: <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            taskResponseMetadata: <IAwesomeVaultTaskManager::TaskResponseMetadata as alloy::sol_types::SolType>::RustType,
            pubkeysOfNonSigningOperators: alloy::sol_types::private::Vec<
                <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, raiseAndResolveChallengeCall, N> {
            self.call_builder(&raiseAndResolveChallengeCall {
                task,
                taskResponse,
                taskResponseMetadata,
                pubkeysOfNonSigningOperators,
            })
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`renounceOwnership`] function.
        pub fn renounceOwnership(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, renounceOwnershipCall, N> {
            self.call_builder(&renounceOwnershipCall {})
        }
        ///Creates a new call builder for the [`respondToTask`] function.
        pub fn respondToTask(
            &self,
            task: <IAwesomeVaultTaskManager::Task as alloy::sol_types::SolType>::RustType,
            taskResponse: <IAwesomeVaultTaskManager::TaskResponse as alloy::sol_types::SolType>::RustType,
            nonSignerStakesAndSignature: <IBLSSignatureCheckerTypes::NonSignerStakesAndSignature as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, respondToTaskCall, N> {
            self.call_builder(&respondToTaskCall {
                task,
                taskResponse,
                nonSignerStakesAndSignature,
            })
        }
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
        }
        ///Creates a new call builder for the [`setStaleStakesForbidden`] function.
        pub fn setStaleStakesForbidden(
            &self,
            value: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStaleStakesForbiddenCall, N> {
            self.call_builder(&setStaleStakesForbiddenCall { value })
        }
        ///Creates a new call builder for the [`stakeRegistry`] function.
        pub fn stakeRegistry(&self) -> alloy_contract::SolCallBuilder<T, &P, stakeRegistryCall, N> {
            self.call_builder(&stakeRegistryCall {})
        }
        ///Creates a new call builder for the [`staleStakesForbidden`] function.
        pub fn staleStakesForbidden(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, staleStakesForbiddenCall, N> {
            self.call_builder(&staleStakesForbiddenCall {})
        }
        ///Creates a new call builder for the [`taskNumber`] function.
        pub fn taskNumber(&self) -> alloy_contract::SolCallBuilder<T, &P, taskNumberCall, N> {
            self.call_builder(&taskNumberCall {})
        }
        ///Creates a new call builder for the [`taskSuccesfullyChallenged`] function.
        pub fn taskSuccesfullyChallenged(
            &self,
            _0: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, taskSuccesfullyChallengedCall, N> {
            self.call_builder(&taskSuccesfullyChallengedCall { _0 })
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`trySignatureAndApkVerification`] function.
        pub fn trySignatureAndApkVerification(
            &self,
            msgHash: alloy::sol_types::private::FixedBytes<32>,
            apk: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
            apkG2: <BN254::G2Point as alloy::sol_types::SolType>::RustType,
            sigma: <BN254::G1Point as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, trySignatureAndApkVerificationCall, N> {
            self.call_builder(&trySignatureAndApkVerificationCall {
                msgHash,
                apk,
                apkG2,
                sigma,
            })
        }
        ///Creates a new call builder for the [`unpause`] function.
        pub fn unpause(
            &self,
            newPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, unpauseCall, N> {
            self.call_builder(&unpauseCall { newPausedStatus })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > AwesomeVaultTaskManagerInstance<T, P, N>
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(&self) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`NewTaskCreated`] event.
        pub fn NewTaskCreated_filter(&self) -> alloy_contract::Event<T, &P, NewTaskCreated, N> {
            self.event_filter::<NewTaskCreated>()
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
        ///Creates a new event filter for the [`StaleStakesForbiddenUpdate`] event.
        pub fn StaleStakesForbiddenUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StaleStakesForbiddenUpdate, N> {
            self.event_filter::<StaleStakesForbiddenUpdate>()
        }
        ///Creates a new event filter for the [`TaskChallengedSuccessfully`] event.
        pub fn TaskChallengedSuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedSuccessfully, N> {
            self.event_filter::<TaskChallengedSuccessfully>()
        }
        ///Creates a new event filter for the [`TaskChallengedUnsuccessfully`] event.
        pub fn TaskChallengedUnsuccessfully_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, TaskChallengedUnsuccessfully, N> {
            self.event_filter::<TaskChallengedUnsuccessfully>()
        }
        ///Creates a new event filter for the [`TaskCompleted`] event.
        pub fn TaskCompleted_filter(&self) -> alloy_contract::Event<T, &P, TaskCompleted, N> {
            self.event_filter::<TaskCompleted>()
        }
        ///Creates a new event filter for the [`TaskResponded`] event.
        pub fn TaskResponded_filter(&self) -> alloy_contract::Event<T, &P, TaskResponded, N> {
            self.event_filter::<TaskResponded>()
        }
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
    }
}
