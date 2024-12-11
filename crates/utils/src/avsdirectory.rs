///Module containing a contract's types and functions.
/**

```solidity
library IAVSDirectoryTypes {
    type OperatorAVSRegistrationStatus is uint8;
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IAVSDirectoryTypes {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAVSRegistrationStatus(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<OperatorAVSRegistrationStatus>
        for u8 {
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
        impl OperatorAVSRegistrationStatus {
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
        impl alloy_sol_types::SolType for OperatorAVSRegistrationStatus {
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
        impl alloy_sol_types::EventTopic for OperatorAVSRegistrationStatus {
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
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IAVSDirectoryTypes`](self) contract instance.

See the [wrapper's documentation](`IAVSDirectoryTypesInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IAVSDirectoryTypesInstance<T, P, N> {
        IAVSDirectoryTypesInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IAVSDirectoryTypes`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IAVSDirectoryTypes`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IAVSDirectoryTypesInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IAVSDirectoryTypesInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IAVSDirectoryTypesInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IAVSDirectoryTypesInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IAVSDirectoryTypes`](self) contract instance.

See the [wrapper's documentation](`IAVSDirectoryTypesInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IAVSDirectoryTypesInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IAVSDirectoryTypesInstance<T, P, N> {
            IAVSDirectoryTypesInstance {
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
    > IAVSDirectoryTypesInstance<T, P, N> {
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
    > IAVSDirectoryTypesInstance<T, P, N> {
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
library ISignatureUtils {
    struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod ISignatureUtils {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct SignatureWithSaltAndExpiry { bytes signature; bytes32 salt; uint256 expiry; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureWithSaltAndExpiry {
        pub signature: alloy::sol_types::private::Bytes,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
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
            alloy::sol_types::sol_data::FixedBytes<32>,
            alloy::sol_types::sol_data::Uint<256>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Bytes,
            alloy::sol_types::private::FixedBytes<32>,
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
        impl ::core::convert::From<SignatureWithSaltAndExpiry>
        for UnderlyingRustTuple<'_> {
            fn from(value: SignatureWithSaltAndExpiry) -> Self {
                (value.signature, value.salt, value.expiry)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for SignatureWithSaltAndExpiry {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    signature: tuple.0,
                    salt: tuple.1,
                    expiry: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SignatureWithSaltAndExpiry {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self>
        for SignatureWithSaltAndExpiry {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.signature,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
        impl alloy_sol_types::SolType for SignatureWithSaltAndExpiry {
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
        impl alloy_sol_types::SolStruct for SignatureWithSaltAndExpiry {
            const NAME: &'static str = "SignatureWithSaltAndExpiry";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SignatureWithSaltAndExpiry(bytes signature,bytes32 salt,uint256 expiry)",
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
                            &self.signature,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.expiry)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SignatureWithSaltAndExpiry {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bytes as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.signature,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.expiry,
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
                    &rust.signature,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.expiry,
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
    /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> ISignatureUtilsInstance<T, P, N> {
        ISignatureUtilsInstance::<T, P, N>::new(address, provider)
    }
    /**A [`ISignatureUtils`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`ISignatureUtils`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct ISignatureUtilsInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for ISignatureUtilsInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("ISignatureUtilsInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > ISignatureUtilsInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`ISignatureUtils`](self) contract instance.

See the [wrapper's documentation](`ISignatureUtilsInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> ISignatureUtilsInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> ISignatureUtilsInstance<T, P, N> {
            ISignatureUtilsInstance {
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
    > ISignatureUtilsInstance<T, P, N> {
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
    > ISignatureUtilsInstance<T, P, N> {
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
library IAVSDirectoryTypes {
    type OperatorAVSRegistrationStatus is uint8;
}

library ISignatureUtils {
    struct SignatureWithSaltAndExpiry {
        bytes signature;
        bytes32 salt;
        uint256 expiry;
    }
}

interface AVSDirectory {
    struct OperatorSet {
        address avs;
        uint32 operatorSetId;
    }

    error CurrentlyPaused();
    error InputAddressZero();
    error InvalidAVS();
    error InvalidNewPausedStatus();
    error InvalidOperator();
    error InvalidOperatorSet();
    error InvalidSignature();
    error OnlyPauser();
    error OnlyUnpauser();
    error OperatorAlreadyRegisteredToAVS();
    error OperatorNotRegisteredToAVS();
    error OperatorNotRegisteredToEigenLayer();
    error SaltSpent();
    error SignatureExpired();
    error StrategyAlreadyInOperatorSet();
    error StrategyNotInOperatorSet();

    event AVSMetadataURIUpdated(address indexed avs, string metadataURI);
    event AVSMigratedToOperatorSets(address indexed avs);
    event Initialized(uint8 version);
    event OperatorAVSRegistrationStatusUpdated(address indexed operator, address indexed avs, IAVSDirectoryTypes.OperatorAVSRegistrationStatus status);
    event OperatorAddedToOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorMigratedToOperatorSets(address indexed operator, address indexed avs, uint32[] operatorSetIds);
    event OperatorRemovedFromOperatorSet(address indexed operator, OperatorSet operatorSet);
    event OperatorSetCreated(OperatorSet operatorSet);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event StrategyAddedToOperatorSet(OperatorSet operatorSet, address strategy);
    event StrategyRemovedFromOperatorSet(OperatorSet operatorSet, address strategy);
    event Unpaused(address indexed account, uint256 newPausedStatus);

    constructor(address _delegation, uint32 _DEALLOCATION_DELAY);

    function DEALLOCATION_DELAY() external view returns (uint32);
    function OPERATOR_AVS_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH() external view returns (bytes32);
    function OPERATOR_SET_REGISTRATION_TYPEHASH() external view returns (bytes32);
    function addStrategiesToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function avsOperatorStatus(address, address) external view returns (IAVSDirectoryTypes.OperatorAVSRegistrationStatus);
    function becomeOperatorSetAVS() external;
    function calculateOperatorAVSRegistrationDigestHash(address operator, address avs, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function calculateOperatorSetForceDeregistrationTypehash(address avs, uint32[] memory operatorSetIds, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function calculateOperatorSetRegistrationDigestHash(address avs, uint32[] memory operatorSetIds, bytes32 salt, uint256 expiry) external view returns (bytes32);
    function cancelSalt(bytes32 salt) external;
    function createOperatorSets(uint32[] memory operatorSetIds) external;
    function delegation() external view returns (address);
    function deregisterOperatorFromAVS(address operator) external;
    function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
    function domainSeparator() external view returns (bytes32);
    function forceDeregisterFromOperatorSets(address operator, address avs, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function getNumOperatorSetsOfOperator(address operator) external view returns (uint256);
    function getNumOperatorsInOperatorSet(OperatorSet memory operatorSet) external view returns (uint256);
    function getOperatorSetsOfOperator(address operator, uint256 start, uint256 length) external view returns (OperatorSet[] memory operatorSets);
    function getOperatorsInOperatorSet(OperatorSet memory operatorSet, uint256 start, uint256 length) external view returns (address[] memory operators);
    function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory strategies);
    function inTotalOperatorSets(address operator) external view returns (uint256);
    function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
    function isMember(address operator, OperatorSet memory operatorSet) external view returns (bool);
    function isOperatorSet(address, uint32) external view returns (bool);
    function isOperatorSetAVS(address) external view returns (bool);
    function isOperatorSetBatch(OperatorSet[] memory operatorSets) external view returns (bool);
    function isOperatorSlashable(address operator, OperatorSet memory operatorSet) external view returns (bool);
    function migrateOperatorsToOperatorSets(address[] memory operators, uint32[][] memory operatorSetIds) external;
    function operatorSaltIsSpent(address, bytes32) external view returns (bool);
    function operatorSetMemberAtIndex(OperatorSet memory operatorSet, uint256 index) external view returns (address);
    function operatorSetStatus(address, address, uint32) external view returns (bool registered, uint32 lastDeregisteredTimestamp);
    function operatorSetsMemberOfAtIndex(address operator, uint256 index) external view returns (OperatorSet memory);
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function registerOperatorToOperatorSets(address operator, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
    function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
    function renounceOwnership() external;
    function setPauserRegistry(address newPauserRegistry) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
    function updateAVSMetadataURI(string memory metadataURI) external;
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
        "name": "_DEALLOCATION_DELAY",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "nonpayable"
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
    "name": "OPERATOR_AVS_REGISTRATION_TYPEHASH",
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
    "name": "OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH",
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
    "name": "OPERATOR_SET_REGISTRATION_TYPEHASH",
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
    "name": "addStrategiesToOperatorSet",
    "inputs": [
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
    "name": "avsOperatorStatus",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "enum IAVSDirectoryTypes.OperatorAVSRegistrationStatus"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "becomeOperatorSetAVS",
    "inputs": [],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "calculateOperatorAVSRegistrationDigestHash",
    "inputs": [
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
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "calculateOperatorSetForceDeregistrationTypehash",
    "inputs": [
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
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "calculateOperatorSetRegistrationDigestHash",
    "inputs": [
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
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "expiry",
        "type": "uint256",
        "internalType": "uint256"
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
    "name": "cancelSalt",
    "inputs": [
      {
        "name": "salt",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
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
    "name": "deregisterOperatorFromAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "deregisterOperatorFromOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "domainSeparator",
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
    "name": "forceDeregisterFromOperatorSets",
    "inputs": [
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
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
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
    "name": "getNumOperatorSetsOfOperator",
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
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getNumOperatorsInOperatorSet",
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
            "name": "operatorSetId",
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
    "name": "getOperatorSetsOfOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "operatorSets",
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
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
    "name": "getOperatorsInOperatorSet",
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
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "start",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "length",
        "type": "uint256",
        "internalType": "uint256"
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
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      }
    ],
    "outputs": [
      {
        "name": "strategies",
        "type": "address[]",
        "internalType": "contract IStrategy[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "inTotalOperatorSets",
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
        "type": "uint256",
        "internalType": "uint256"
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
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
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
    "name": "isMember",
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
            "name": "operatorSetId",
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
        "name": "",
        "type": "address",
        "internalType": "address"
      },
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
    "name": "isOperatorSetAVS",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
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
    "name": "isOperatorSetBatch",
    "inputs": [
      {
        "name": "operatorSets",
        "type": "tuple[]",
        "internalType": "struct OperatorSet[]",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
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
    "name": "isOperatorSlashable",
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
            "name": "operatorSetId",
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
    "name": "migrateOperatorsToOperatorSets",
    "inputs": [
      {
        "name": "operators",
        "type": "address[]",
        "internalType": "address[]"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[][]",
        "internalType": "uint32[][]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "operatorSaltIsSpent",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "operatorSetMemberAtIndex",
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
            "name": "operatorSetId",
            "type": "uint32",
            "internalType": "uint32"
          }
        ]
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
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
    "name": "operatorSetStatus",
    "inputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [
      {
        "name": "registered",
        "type": "bool",
        "internalType": "bool"
      },
      {
        "name": "lastDeregisteredTimestamp",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "operatorSetsMemberOfAtIndex",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "index",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct OperatorSet",
        "components": [
          {
            "name": "avs",
            "type": "address",
            "internalType": "address"
          },
          {
            "name": "operatorSetId",
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
    "name": "registerOperatorToAVS",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
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
    "name": "registerOperatorToOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "internalType": "uint32[]"
      },
      {
        "name": "operatorSignature",
        "type": "tuple",
        "internalType": "struct ISignatureUtils.SignatureWithSaltAndExpiry",
        "components": [
          {
            "name": "signature",
            "type": "bytes",
            "internalType": "bytes"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          },
          {
            "name": "expiry",
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
    "name": "removeStrategiesFromOperatorSet",
    "inputs": [
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
    "name": "setPauserRegistry",
    "inputs": [
      {
        "name": "newPauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
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
    "name": "AVSMigratedToOperatorSets",
    "inputs": [
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
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
    "name": "OperatorAVSRegistrationStatusUpdated",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "status",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum IAVSDirectoryTypes.OperatorAVSRegistrationStatus"
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
            "name": "operatorSetId",
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
    "name": "OperatorMigratedToOperatorSets",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "avs",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operatorSetIds",
        "type": "uint32[]",
        "indexed": false,
        "internalType": "uint32[]"
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
            "name": "operatorSetId",
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
            "name": "operatorSetId",
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
    "name": "PauserRegistrySet",
    "inputs": [
      {
        "name": "pauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "newPauserRegistry",
        "type": "address",
        "indexed": false,
        "internalType": "contract IPauserRegistry"
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
            "name": "operatorSetId",
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
            "name": "operatorSetId",
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
    "name": "InvalidAVS",
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
    "name": "InvalidSignature",
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
    "name": "OperatorAlreadyRegisteredToAVS",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegisteredToAVS",
    "inputs": []
  },
  {
    "type": "error",
    "name": "OperatorNotRegisteredToEigenLayer",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SaltSpent",
    "inputs": []
  },
  {
    "type": "error",
    "name": "SignatureExpired",
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
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod AVSDirectory {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100806040523461016157601f612df738819003918201601f19168301916001600160401b03831184841017610165578084926040948552833981010312610161578051906001600160a01b038216820361016157602001519063ffffffff821682036101615760805260a0524660c0525f54600881901c60ff1661010c5760ff808216106100d2575b604051612c7d908161017a823960805181818161061f01528181610d390152611792015260a0518181816115ed0152612029015260c0518161281e015260e051816128440152f35b60ff90811916175f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160ff8152a15f61008a565b60405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608490fd5b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe60806040526004361015610011575f80fd5b5f3560e01c80631023aa3514611cb357806310d67a2f14611c2e5780631352c3e614611c01578063136439dd14611b5557806316ae76cb14611a085780631794bb3c1461186a5780631e2199e2146116f45780631e68134e1461167057806320c4e236146116115780632981eb77146115d1578063374823b5146115885780633fee332d14611478578063411d415b146114295780634177a87c1461133857806349075da3146112da578063595c6a67146112285780635ac86ab7146111f55780635c975abb146111d8578063715018a61461117d5780637357723b1461105b5780637673e93a1461101e5780637699934214610f3957806384d76f7b14610edc578063886f119514610eb45780638da5cb5b14610e8c578063955e669614610e705780639926ee7d14610c58578063a1060c8814610c24578063a364f4da14610b50578063a98fb35514610aae578063aec205c514610a4c578063afe02ed51461093a578063b2841d481461091e578063b5a768ca146108aa578063c1a8e2c51461085b578063c825fe6814610821578063cbdf0e4214610605578063ce7b5e4b146106f9578063d79aceab146106bf578063da2ff05d14610688578063dce974b91461064e578063df5cf7231461060a578063e88d804914610605578063ec76f442146105cd578063ef2dfa8d146103cb578063f2fde38b1461033c578063f698da251461031a5763fabc1cbc14610229575f80fd5b346103165760203660031901126103165760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa90811561030b575f916102dc575b506001600160a01b031633036102cd576066541981198116036102be57806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63c61dca5d60e01b5f5260045ffd5b63794821ff60e01b5f5260045ffd5b6102fe915060203d602011610304575b6102f68183611d15565b810190611f92565b5f61026f565b503d6102ec565b6040513d5f823e3d90fd5b5f80fd5b34610316575f36600319011261031657602061033461281b565b604051908152f35b3461031657602036600319011261031657610355611d36565b61035d61278a565b6001600160a01b0381161561037757610375906124a3565b005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610316576040366003190112610316576004356001600160401b038111610316576103fb903690600401611dee565b6024356001600160401b0381116103165761041a903690600401611dee565b9160028060665416146105be57335f52609a60205260ff60405f205416156105af575f5b81811061044757005b335f52609860205260405f206104666104618385896121a9565b612114565b60018060a01b03165f5260205260ff60405f205416600281101561059b5760010361058c57806104b261049f610461600194868a6121a9565b6104aa838888612374565b91339061250c565b335f52609860205260405f206104cc61046183868a6121a9565b838060a01b03165f5260205260405f2060ff198154169055818060a01b036104f861046183868a6121a9565b16604051905f82527ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4160203393a36105346104618285896121a9565b6105837f54f33cfdd1ca703d795986b986fd47d742eab1904ecd2a5fdb8d6595e5904a01610563848989612374565b9290604051918291602083523396898060a01b03169560208401916121b9565b0390a30161043e565b63ccea9e6f60e01b5f5260045ffd5b634e487b7160e01b5f52602160045260245ffd5b6366e565df60e01b5f5260045ffd5b63840a48d560e01b5f5260045ffd5b3461031657602036600319011261031657335f52609960205260405f206004355f5260205260405f20600160ff198254161790555f80f35b611f5b565b34610316575f366003190112610316576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610316575f3660031901126103165760206040517f4ee65f64218c67b68da66fd0db16560040a6b973290b9e71912d661ee53fe4958152f35b346103165760603660031901126103165760206106b56106a6611d36565b6106af36611da8565b9061233d565b6040519015158152f35b34610316575f3660031901126103165760206040517fda2c89bafdd34776a2b8bb9c83c82f419e20cc8c67207f70edd58249b92661bd8152f35b346103165761070736611ed0565b604051919261071583611ce6565b33835263ffffffff81166020840152335f52609b60205263ffffffff60405f2091165f5260205260ff60405f2054161561081257610752826123b8565b915f5b82811061075e57005b5f848152609e6020526040902061078c906001600160a01b0361078561046185888b6121a9565b1690612a76565b1561080357807f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee6107fa6107c6610461600195888b6121a9565b6040805187516001600160a01b0390811682526020808a015163ffffffff1690830152909216908201529081906060820190565b0390a101610755565b6331bc342760e11b5f5260045ffd5b631fb1705560e21b5f5260045ffd5b34610316575f3660031901126103165760206040517f809c5ac049c45b7a7f050a20f00c16cf63797efbf8b1eb8d749fdfa39ff8f9298152f35b3461031657604036600319011261031657610874611d36565b6024356001600160401b03811161031657610893903690600401611dee565b9060028060665416146105be57610375923361265b565b34610316576040366003190112610316576108c3611d36565b6108cb6120b7565b5060018060a01b03165f52609c60205260406108fa6108ee602435835f206128d8565b90549060031b1c612479565b815181516001600160a01b0316815260209182015163ffffffff1691810191909152f35b3461031657602061033461093136611f10565b939290926122e3565b34610316576020366003190112610316576004356001600160401b0381116103165761096a903690600401611dee565b905f5b82811061097657005b335f52609b60205260405f2063ffffffff61099a6109958487876121a9565b612128565b165f5260205260ff60405f20541661081257600190335f52609b60205260405f2063ffffffff6109ce6109958488886121a9565b165f5260205260405f208260ff198254161790557f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040610a136109958488886121a9565b63ffffffff825191610a2483611ce6565b338084529116602092830190815283519182525163ffffffff1691810191909152a10161096d565b34610316575f36600319011261031657335f52609a60205260ff60405f2054166105af57335f52609a60205260405f20600160ff19825416179055337f702b0c1f6cb1cf511aaa81f72bc05a215bb3497632d72c690c822b044ab494bf5f80a2005b34610316576020366003190112610316576004356001600160401b0381116103165736602382011215610316578060040135906001600160401b038211610316573660248383010111610316577fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c9437139082602460405192602084528260208501520160408301375f604084830101526040813394601f80199101168101030190a2005b3461031657602036600319011261031657610b69611d36565b60018060665416146105be57335f52609860205260405f2060018060a01b0382165f5260205260ff60405f205416600281101561059b57600103610c1557335f52609a60205260ff60405f2054166105af57335f8181526098602090815260408083206001600160a01b0395909516808452948252808320805460ff19169055519182529192917ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4191a3005b6352df45c960e01b5f5260045ffd5b34610316576080366003190112610316576020610334610c42611d36565b610c4a611d4c565b90606435916044359161227e565b3461031657604036600319011261031657610c71611d36565b6024356001600160401b03811161031657610c90903690600401611e39565b60018060665416146105be576040810180514211610e6157335f52609a60205260ff60405f2054166105af57335f52609860205260405f2060018060a01b0384165f5260205260ff60405f205416600281101561059b57600114610e525760018060a01b03831692835f52609960205260405f2092602081019384515f5260205260ff60405f205416610e43576040516336b87bd760e11b8152600481018690526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561030b575f91610e14575b5015610e0557610d8a610d92938551905190338561227e565b9051916124eb565b815f52609960205260405f2090515f5260205260405f20600160ff19825416179055335f52609860205260405f20815f5260205260405f20600160ff1982541617905560405190600182527ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4160203393a3005b639f88c8af60e01b5f5260045ffd5b610e36915060203d602011610e3c575b610e2e8183611d15565b81019061207b565b86610d71565b503d610e24565b630d4c4c9160e21b5f5260045ffd5b631aa528bb60e11b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b34610316576020610334610e8336611f10565b93929092612200565b34610316575f366003190112610316576033546040516001600160a01b039091168152602090f35b34610316575f366003190112610316576065546040516001600160a01b039091168152602090f35b3461031657604036600319011261031657610ef5611d36565b6024359063ffffffff821682036103165760018060a01b03165f52609b60205263ffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b3461031657610f4736611ed0565b6040519192610f5583611ce6565b33835263ffffffff81166020840152335f52609b60205263ffffffff60405f2091165f5260205260ff60405f2054161561081257610f92826123b8565b915f5b828110610f9e57005b5f848152609e60205260409020610fcc906001600160a01b03610fc561046185888b6121a9565b1690612a08565b1561100f57807f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b6110066107c6610461600195888b6121a9565b0390a101610f95565b63585cfb2f60e01b5f5260045ffd5b34610316576020366003190112610316576001600160a01b0361103f611d36565b165f52609a602052602060ff60405f2054166040519015158152f35b346103165760803660031901126103165761107536611d62565b60443590611085606435916123b8565b90815f52609d60205261109c8360405f2054612093565b808211611175575b506110ae816120a0565b926110bc6040519485611d15565b8184526110c8826120a0565b602085019390601f19013685375f5b83811061112b578486604051918291602083019060208452518091526040830191905f5b818110611109575050500390f35b82516001600160a01b03168452859450602093840193909201916001016110fb565b600190825f52609d60205261114d60405f2061114783876120cf565b906128d8565b838060a01b0391549060031b1c1661116582896120dc565b90838060a01b03169052016110d7565b9050836110a4565b34610316575f3660031901126103165761119561278a565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610316575f366003190112610316576020606654604051908152f35b346103165760203660031901126103165760043560ff81168091036103165760016020911b806066541614604051908152f35b34610316575f3660031901126103165760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa90811561030b575f916112bb575b50156112ac575f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b631d77d47760e21b5f5260045ffd5b6112d4915060203d602011610e3c57610e2e8183611d15565b81611272565b34610316576040366003190112610316576112f3611d36565b6112fb611d4c565b9060018060a01b03165f52609860205260405f209060018060a01b03165f5260205260ff60405f205416604051600282101561059b576020918152f35b346103165760403660031901126103165761135a61135536611d62565b6123b8565b805f52609e60205260405f205490611371826120a0565b9161137f6040519384611d15565b80835261138b816120a0565b602084019290601f19013684375f5b8281106113ee578385604051918291602083019060208452518091526040830191905f5b8181106113cc575050500390f35b82516001600160a01b03168452859450602093840193909201916001016113be565b600190825f52609e6020526114068160405f206128d8565b905460039190911b1c60a083901b8390031661142282886120dc565b520161139a565b346103165760603660031901126103165761144661135536611d62565b5f52609d602052602061145f60405f20604435906128d8565b905460405160039290921b1c6001600160a01b03168152f35b3461031657608036600319011261031657611491611d36565b611499611d4c565b906044356001600160401b038111610316576114b9903690600401611dee565b916064356001600160401b038111610316576114d9903690600401611e39565b60028060665416146105be5780515161150657506001600160a01b038116330361058c576103759361265b565b936040850180514211610e615760018060a01b038316805f52609960205260405f2091602088019283515f5260205260ff60405f205416610e4357610375976115596115629285519051908a8a896122e3565b905190866124eb565b5f52609960205260405f2090515f5260205260405f20600160ff1982541617905561265b565b34610316576040366003190112610316576001600160a01b036115a9611d36565b165f52609960205260405f206024355f52602052602060ff60405f2054166040519015158152f35b34610316575f36600319011261031657602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610316576020366003190112610316576004356001600160401b0381116103165736602382011215610316578060040135906001600160401b038211610316573660248360061b830101116103165760209160246106b59201612139565b3461031657606036600319011261031657611689611d36565b611691611d4c565b6044359163ffffffff831683036103165760018060a01b03165f52609f60205260405f209060018060a01b03165f5260205263ffffffff60405f2091165f526020526040805f205463ffffffff82519160ff81161515835260081c166020820152f35b346103165760603660031901126103165761170d611d36565b6024356001600160401b0381116103165761172c903690600401611dee565b906044356001600160401b0381116103165761174c903690600401611e39565b9260028060665416146105be576040840180514211610e61576040516336b87bd760e11b81526001600160a01b03838116600483018190529190602090829060249082907f0000000000000000000000000000000000000000000000000000000000000000165afa90811561030b575f9161184b575b5015610e0557335f52609a60205260ff60405f205416156105af57805f52609960205260405f2091602087019283515f5260205260ff60405f205416610e43576103759661181a611823928551905190898933612200565b905190856124eb565b5f52609960205260405f2090515f5260205260405f20600160ff19825416179055339061250c565b611864915060203d602011610e3c57610e2e8183611d15565b876117c2565b3461031657606036600319011261031657611883611d36565b61188b611d4c565b604435915f549260ff8460081c1615938480956119fb575b80156119e4575b156119885760ff1981166001175f5584611977575b506065546001600160a01b03161580611965575b156119565761191a9281611915926066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a261241c565b6124a3565b61192057005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b6339b190bb60e11b5f5260045ffd5b506001600160a01b03831615156118d3565b61ffff1916610101175f55846118bf565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156118aa5750600160ff8216146118aa565b50600160ff8216106118a3565b3461031657606036600319011261031657611a21611d36565b6001600160a01b03165f818152609c60205260409020546044359060243590611a4b908290612093565b808311611b4d575b50611a5d826120a0565b92611a6b6040519485611d15565b828452601f19611a7a846120a0565b015f5b818110611b2a5750505f5b838110611aef57846040518091602082016020835281518091526020604084019201905f5b818110611abb575050500390f35b825180516001600160a01b0316855260209081015163ffffffff168186015286955060409094019390920191600101611aad565b600190825f52609c602052611b0e6108ee60405f2061114784886120cf565b611b1882886120dc565b52611b2381876120dc565b5001611a88565b602090604051611b3981611ce6565b5f81525f8382015282828901015201611a7d565b915083611a53565b34610316576020366003190112610316576004356024602060018060a01b03606554166040519283809263237dfb4760e11b82523360048301525afa90811561030b575f91611be2575b50156112ac57606654818116036102be57806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b611bfb915060203d602011610e3c57610e2e8183611d15565b82611b9f565b346103165760603660031901126103165760206106b5611c1f611d36565b611c2836611da8565b90611fb1565b3461031657602036600319011261031657611c47611d36565b60655460405163755b36bd60e11b815290602090829060049082906001600160a01b03165afa90811561030b575f91611c94575b506001600160a01b031633036102cd576103759061241c565b611cad915060203d602011610304576102f68183611d15565b82611c7b565b3461031657604036600319011261031657611cd061135536611d62565b5f52609d602052602060405f2054604051908152f35b604081019081106001600160401b03821117611d0157604052565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b03821117611d0157604052565b600435906001600160a01b038216820361031657565b602435906001600160a01b038216820361031657565b60409060031901126103165760405190611d7b82611ce6565b816004356001600160a01b03811681036103165781526024359063ffffffff821682036103165760200152565b60409060231901126103165760405190611dc182611ce6565b816024356001600160a01b03811681036103165781526044359063ffffffff821682036103165760200152565b9181601f84011215610316578235916001600160401b038311610316576020808501948460051b01011161031657565b6001600160401b038111611d0157601f01601f191660200190565b91906060838203126103165760405190606082018281106001600160401b03821117611d0157604052819380356001600160401b0381116103165781019082601f8301121561031657813592611e8e84611e1e565b90611e9c6040519283611d15565b84825260208585010111610316575f6020856040968288970183860137830101528452602081013560208501520135910152565b9060406003198301126103165760043563ffffffff811681036103165791602435906001600160401b03821161031657611f0c91600401611dee565b9091565b906080600319830112610316576004356001600160a01b03811681036103165791602435906001600160401b03821161031657611f4f91600401611dee565b90916044359060643590565b3461031657602036600319011261031657611f74611d36565b60018060a01b03165f52609c602052602060405f2054604051908152f35b9081602091031261031657516001600160a01b03811681036103165790565b611fbb828261233d565b6120745760018060a01b038251165f52609f60205260405f209060018060a01b03165f5260205263ffffffff80602060405f2093015116165f52602052602060405f2063ffffffff6040519161201083611ce6565b5460ff81161515835260081c16918291015263ffffffff7f0000000000000000000000000000000000000000000000000000000000000000160163ffffffff81116120605763ffffffff16421090565b634e487b7160e01b5f52601160045260245ffd5b5050600190565b90816020910312610316575180151581036103165790565b9190820391821161206057565b6001600160401b038111611d015760051b60200190565b604051906120c482611ce6565b5f6020838281520152565b9190820180921161206057565b80518210156120f05760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908110156120f05760061b0190565b356001600160a01b03811681036103165790565b3563ffffffff811681036103165790565b5f5b82811061214a57505050600190565b6001600160a01b03612160610461838686612104565b165f52609b60205260405f2063ffffffff6121876020612181858888612104565b01612128565b165f5260205260ff60405f205416156121a25760010161213b565b5050505f90565b91908110156120f05760051b0190565b916020908281520191905f905b8082106121d35750505090565b90919283359063ffffffff82168203610316576020809163ffffffff6001941681520194019201906121c6565b9291612273919261227b9561225a60405195869460208601987f809c5ac049c45b7a7f050a20f00c16cf63797efbf8b1eb8d749fdfa39ff8f9298a5260018060a01b0316604087015260a0606087015260c08601916121b9565b91608084015260a083015203601f198101835282611d15565b5190206127e2565b90565b919261227b936040519260208401947fda2c89bafdd34776a2b8bb9c83c82f419e20cc8c67207f70edd58249b92661bd865260018060a01b0316604085015260018060a01b03166060840152608083015260a082015260a0815261227360c082611d15565b9291612273919261227b9561225a60405195869460208601987f4ee65f64218c67b68da66fd0db16560040a6b973290b9e71912d661ee53fe4958a5260018060a01b0316604087015260a0606087015260c08601916121b9565b6001600160a01b03165f908152609c6020526040902061227b91612360906123b8565b906001915f520160205260405f2054151590565b91908110156120f05760051b81013590601e19813603018212156103165701908135916001600160401b038311610316576020018260051b36038113610316579190565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b166034820152602081526123fe604082611d15565b519051906020811061240e575090565b5f199060200360031b1b1690565b6001600160a01b0316801561195657606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b6124816120b7565b5063ffffffff6040519161249483611ce6565b8060601c835216602082015290565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b906124f692916128ed565b156124fd57565b638baa579f60e01b5f5260045ffd5b9092915f5b83811061251f575050505050565b61252d6109958286856121a9565b906040519161253b83611ce6565b63ffffffff60018060a01b03881691828552166020840152805f52609b60205260405f2063ffffffff612572610995858a896121a9565b165f5260205260ff60405f205416156108125761258e836123b8565b9060018060a01b03861691825f52609c6020526125ae8160405f20612a08565b505f52609d6020526125c38260405f20612a08565b505f52609f60205260405f20815f5260205260405f2063ffffffff6125ec610995858a896121a9565b165f5260205260405f20805460ff811661058c576001947f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e928660409360ff19161790556126548251809263ffffffff6020809260018060a01b038151168552015116910152565ba201612511565b92914260081b64ffffffff0016905f5b84811061267a57505050505050565b6126886109958287856121a9565b906040519161269683611ce6565b6001600160a01b03881680845263ffffffff9190911660208401526126ba836123b8565b9060018060a01b03871691825f52609c6020526126da8160405f20612a76565b505f52609d6020526126ef8260405f20612a76565b505f52609f60205260405f20815f5260205260405f2063ffffffff612718610995858b896121a9565b165f5260205260405f20805460ff81161561058c5764ffffffffff1916861790556040805184516001600160a01b0316815260209485015163ffffffff16948101949094526001937fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe9190a20161266b565b6033546001600160a01b0316330361279e57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b6127ea61281b565b9060405190602082019261190160f01b84526022830152604282015260428152612815606282611d15565b51902090565b467f000000000000000000000000000000000000000000000000000000000000000003612866577f000000000000000000000000000000000000000000000000000000000000000090565b600a6020604051612878604082611d15565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261281560a082611d15565b80548210156120f0575f5260205f2001905f90565b9190916128fa8284612b3b565b600581101561059b571590816129f2575b506129ea575f92602061296a6084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f198101835282611d15565b51915afa3d156129e3573d61297e81611e1e565b9061298c6040519283611d15565b81523d5f602083013e5b816129d7575b816129a5575090565b905060208180518101031261031657602001516001600160e01b031981169081900361031657630b135d3f60e11b1490565b8051602014915061299c565b6060612996565b505050600190565b6001600160a01b0383811691161490505f61290b565b5f828152600182016020526040902054612a705780549068010000000000000000821015611d015782612a5b612a458460018096018555846128d8565b819391549060031b91821b915f19901b19161790565b90558054925f520160205260405f2055600190565b50505f90565b906001820191815f528260205260405f20548015155f14612b33575f1981018181116120605782545f1981019190821161206057808203612afe575b50505080548015612aea575f190190612acb82826128d8565b8154905f199060031b1b19169055555f526020525f6040812055600190565b634e487b7160e01b5f52603160045260245ffd5b612b1e612b0e612a4593866128d8565b90549060031b1c928392866128d8565b90555f528360205260405f20555f8080612ab2565b505050505f90565b815160418103612b63575090611f0c91602082015190606060408401519301515f1a90612ba5565b604003612b9c5760406020830151920151918260ff1c91601b830180931161206057611f0c936001600160ff1b03169260ff1690612ba5565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411612c3c5760ff1690601b82141580612c31575b612c26576020935f93608093604051938452868401526040830152606082015282805260015afa1561030b575f516001600160a01b03811615612c1e57905f90565b505f90600190565b505050505f90600490565b50601c821415612bdc565b505050505f9060039056fea2646970667358221220f1121655cd31c94df6b82e50ad93bb64ef5a1c8330c880c290a9040dfd22297264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0\x80`@R4a\x01aW`\x1Fa-\xF78\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\x01eW\x80\x84\x92`@\x94\x85R\x839\x81\x01\x03\x12a\x01aW\x80Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x01aW` \x01Q\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x01aW`\x80R`\xA0RF`\xC0R_T`\x08\x81\x90\x1C`\xFF\x16a\x01\x0CW`\xFF\x80\x82\x16\x10a\0\xD2W[`@Qa,}\x90\x81a\x01z\x829`\x80Q\x81\x81\x81a\x06\x1F\x01R\x81\x81a\r9\x01Ra\x17\x92\x01R`\xA0Q\x81\x81\x81a\x15\xED\x01Ra )\x01R`\xC0Q\x81a(\x1E\x01R`\xE0Q\x81a(D\x01R\xF3[`\xFF\x90\x81\x19\x16\x17_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\xFF\x81R\xA1_a\0\x8AV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x90\xFD[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x10#\xAA5\x14a\x1C\xB3W\x80c\x10\xD6z/\x14a\x1C.W\x80c\x13R\xC3\xE6\x14a\x1C\x01W\x80c\x13d9\xDD\x14a\x1BUW\x80c\x16\xAEv\xCB\x14a\x1A\x08W\x80c\x17\x94\xBB<\x14a\x18jW\x80c\x1E!\x99\xE2\x14a\x16\xF4W\x80c\x1Eh\x13N\x14a\x16pW\x80c \xC4\xE26\x14a\x16\x11W\x80c)\x81\xEBw\x14a\x15\xD1W\x80c7H#\xB5\x14a\x15\x88W\x80c?\xEE3-\x14a\x14xW\x80cA\x1DA[\x14a\x14)W\x80cAw\xA8|\x14a\x138W\x80cI\x07]\xA3\x14a\x12\xDAW\x80cY\\jg\x14a\x12(W\x80cZ\xC8j\xB7\x14a\x11\xF5W\x80c\\\x97Z\xBB\x14a\x11\xD8W\x80cqP\x18\xA6\x14a\x11}W\x80csWr;\x14a\x10[W\x80cvs\xE9:\x14a\x10\x1EW\x80cv\x99\x93B\x14a\x0F9W\x80c\x84\xD7o{\x14a\x0E\xDCW\x80c\x88o\x11\x95\x14a\x0E\xB4W\x80c\x8D\xA5\xCB[\x14a\x0E\x8CW\x80c\x95^f\x96\x14a\x0EpW\x80c\x99&\xEE}\x14a\x0CXW\x80c\xA1\x06\x0C\x88\x14a\x0C$W\x80c\xA3d\xF4\xDA\x14a\x0BPW\x80c\xA9\x8F\xB3U\x14a\n\xAEW\x80c\xAE\xC2\x05\xC5\x14a\nLW\x80c\xAF\xE0.\xD5\x14a\t:W\x80c\xB2\x84\x1DH\x14a\t\x1EW\x80c\xB5\xA7h\xCA\x14a\x08\xAAW\x80c\xC1\xA8\xE2\xC5\x14a\x08[W\x80c\xC8%\xFEh\x14a\x08!W\x80c\xCB\xDF\x0EB\x14a\x06\x05W\x80c\xCE{^K\x14a\x06\xF9W\x80c\xD7\x9A\xCE\xAB\x14a\x06\xBFW\x80c\xDA/\xF0]\x14a\x06\x88W\x80c\xDC\xE9t\xB9\x14a\x06NW\x80c\xDF\\\xF7#\x14a\x06\nW\x80c\xE8\x8D\x80I\x14a\x06\x05W\x80c\xECv\xF4B\x14a\x05\xCDW\x80c\xEF-\xFA\x8D\x14a\x03\xCBW\x80c\xF2\xFD\xE3\x8B\x14a\x03<W\x80c\xF6\x98\xDA%\x14a\x03\x1AWc\xFA\xBC\x1C\xBC\x14a\x02)W_\x80\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x02\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xCDW`fT\x19\x81\x19\x81\x16\x03a\x02\xBEW\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x02\xFE\x91P` =` \x11a\x03\x04W[a\x02\xF6\x81\x83a\x1D\x15V[\x81\x01\x90a\x1F\x92V[_a\x02oV[P=a\x02\xECV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` a\x034a(\x1BV[`@Q\x90\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x03Ua\x1D6V[a\x03]a'\x8AV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03wWa\x03u\x90a$\xA3V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x03\xFB\x906\x90`\x04\x01a\x1D\xEEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x04\x1A\x906\x90`\x04\x01a\x1D\xEEV[\x91`\x02\x80`fT\x16\x14a\x05\xBEW3_R`\x9A` R`\xFF`@_ T\x16\x15a\x05\xAFW_[\x81\x81\x10a\x04GW\0[3_R`\x98` R`@_ a\x04fa\x04a\x83\x85\x89a!\xA9V[a!\x14V[`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x03a\x05\x8CW\x80a\x04\xB2a\x04\x9Fa\x04a`\x01\x94\x86\x8Aa!\xA9V[a\x04\xAA\x83\x88\x88a#tV[\x913\x90a%\x0CV[3_R`\x98` R`@_ a\x04\xCCa\x04a\x83\x86\x8Aa!\xA9V[\x83\x80`\xA0\x1B\x03\x16_R` R`@_ `\xFF\x19\x81T\x16\x90U\x81\x80`\xA0\x1B\x03a\x04\xF8a\x04a\x83\x86\x8Aa!\xA9V[\x16`@Q\x90_\x82R\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A` 3\x93\xA3a\x054a\x04a\x82\x85\x89a!\xA9V[a\x05\x83\x7FT\xF3<\xFD\xD1\xCAp=yY\x86\xB9\x86\xFDG\xD7B\xEA\xB1\x90N\xCD*_\xDB\x8De\x95\xE5\x90J\x01a\x05c\x84\x89\x89a#tV[\x92\x90`@Q\x91\x82\x91` \x83R3\x96\x89\x80`\xA0\x1B\x03\x16\x95` \x84\x01\x91a!\xB9V[\x03\x90\xA3\x01a\x04>V[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cf\xE5e\xDF`\xE0\x1B_R`\x04_\xFD[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W3_R`\x99` R`@_ `\x045_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\xF3[a\x1F[V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x81R\xF3[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16W` a\x06\xB5a\x06\xA6a\x1D6V[a\x06\xAF6a\x1D\xA8V[\x90a#=V[`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x81R\xF3[4a\x03\x16Wa\x07\x076a\x1E\xD0V[`@Q\x91\x92a\x07\x15\x83a\x1C\xE6V[3\x83Rc\xFF\xFF\xFF\xFF\x81\x16` \x84\x01R3_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa\x07R\x82a#\xB8V[\x91_[\x82\x81\x10a\x07^W\0[_\x84\x81R`\x9E` R`@\x90 a\x07\x8C\x90`\x01`\x01`\xA0\x1B\x03a\x07\x85a\x04a\x85\x88\x8Ba!\xA9V[\x16\x90a*vV[\x15a\x08\x03W\x80\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEEa\x07\xFAa\x07\xC6a\x04a`\x01\x95\x88\x8Ba!\xA9V[`@\x80Q\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8A\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x07UV[c1\xBC4'`\xE1\x1B_R`\x04_\xFD[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x81R\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x08ta\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x08\x93\x906\x90`\x04\x01a\x1D\xEEV[\x90`\x02\x80`fT\x16\x14a\x05\xBEWa\x03u\x923a&[V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x08\xC3a\x1D6V[a\x08\xCBa \xB7V[P`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@a\x08\xFAa\x08\xEE`$5\x83_ a(\xD8V[\x90T\x90`\x03\x1B\x1Ca$yV[\x81Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x91\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\xF3[4a\x03\x16W` a\x034a\t16a\x1F\x10V[\x93\x92\x90\x92a\"\xE3V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\tj\x906\x90`\x04\x01a\x1D\xEEV[\x90_[\x82\x81\x10a\tvW\0[3_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa\t\x9Aa\t\x95\x84\x87\x87a!\xA9V[a!(V[\x16_R` R`\xFF`@_ T\x16a\x08\x12W`\x01\x903_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa\t\xCEa\t\x95\x84\x88\x88a!\xA9V[\x16_R` R`@_ \x82`\xFF\x19\x82T\x16\x17\x90U\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@a\n\x13a\t\x95\x84\x88\x88a!\xA9V[c\xFF\xFF\xFF\xFF\x82Q\x91a\n$\x83a\x1C\xE6V[3\x80\x84R\x91\x16` \x92\x83\x01\x90\x81R\x83Q\x91\x82RQc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\xA1\x01a\tmV[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_R`\x9A` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3\x7Fp+\x0C\x1Fl\xB1\xCFQ\x1A\xAA\x81\xF7+\xC0Z![\xB3Iv2\xD7,i\x0C\x82+\x04J\xB4\x94\xBF_\x80\xA2\0[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W6`#\x82\x01\x12\x15a\x03\x16W\x80`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16W6`$\x83\x83\x01\x01\x11a\x03\x16W\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x90\x82`$`@Q\x92` \x84R\x82` \x85\x01R\x01`@\x83\x017_`@\x84\x83\x01\x01R`@\x813\x94`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xA2\0[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x0Bia\x1D6V[`\x01\x80`fT\x16\x14a\x05\xBEW3_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x03a\x0C\x15W3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_\x81\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x82R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x91\x82R\x91\x92\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91\xA3\0[cR\xDFE\xC9`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16W` a\x034a\x0CBa\x1D6V[a\x0CJa\x1DLV[\x90`d5\x91`D5\x91a\"~V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x0Cqa\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x0C\x90\x906\x90`\x04\x01a\x1E9V[`\x01\x80`fT\x16\x14a\x05\xBEW`@\x81\x01\x80QB\x11a\x0EaW3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x84\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x14a\x0ERW`\x01\x80`\xA0\x1B\x03\x83\x16\x92\x83_R`\x99` R`@_ \x92` \x81\x01\x93\x84Q_R` R`\xFF`@_ T\x16a\x0ECW`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x0E\x14W[P\x15a\x0E\x05Wa\r\x8Aa\r\x92\x93\x85Q\x90Q\x903\x85a\"~V[\x90Q\x91a$\xEBV[\x81_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3_R`\x98` R`@_ \x81_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90`\x01\x82R\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A` 3\x93\xA3\0[c\x9F\x88\xC8\xAF`\xE0\x1B_R`\x04_\xFD[a\x0E6\x91P` =` \x11a\x0E<W[a\x0E.\x81\x83a\x1D\x15V[\x81\x01\x90a {V[\x86a\rqV[P=a\x0E$V[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[c\x1A\xA5(\xBB`\xE1\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` a\x034a\x0E\x836a\x1F\x10V[\x93\x92\x90\x92a\"\0V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x0E\xF5a\x1D6V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16Wa\x0FG6a\x1E\xD0V[`@Q\x91\x92a\x0FU\x83a\x1C\xE6V[3\x83Rc\xFF\xFF\xFF\xFF\x81\x16` \x84\x01R3_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa\x0F\x92\x82a#\xB8V[\x91_[\x82\x81\x10a\x0F\x9EW\0[_\x84\x81R`\x9E` R`@\x90 a\x0F\xCC\x90`\x01`\x01`\xA0\x1B\x03a\x0F\xC5a\x04a\x85\x88\x8Ba!\xA9V[\x16\x90a*\x08V[\x15a\x10\x0FW\x80\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\x10\x06a\x07\xC6a\x04a`\x01\x95\x88\x8Ba!\xA9V[\x03\x90\xA1\x01a\x0F\x95V[cX\\\xFB/`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x01`\x01`\xA0\x1B\x03a\x10?a\x1D6V[\x16_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16Wa\x10u6a\x1DbV[`D5\x90a\x10\x85`d5\x91a#\xB8V[\x90\x81_R`\x9D` Ra\x10\x9C\x83`@_ Ta \x93V[\x80\x82\x11a\x11uW[Pa\x10\xAE\x81a \xA0V[\x92a\x10\xBC`@Q\x94\x85a\x1D\x15V[\x81\x84Ra\x10\xC8\x82a \xA0V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x83\x81\x10a\x11+W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x11\tWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xFBV[`\x01\x90\x82_R`\x9D` Ra\x11M`@_ a\x11G\x83\x87a \xCFV[\x90a(\xD8V[\x83\x80`\xA0\x1B\x03\x91T\x90`\x03\x1B\x1C\x16a\x11e\x82\x89a \xDCV[\x90\x83\x80`\xA0\x1B\x03\x16\x90R\x01a\x10\xD7V[\x90P\x83a\x10\xA4V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16Wa\x11\x95a'\x8AV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `fT`@Q\x90\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\xFF\x81\x16\x80\x91\x03a\x03\x16W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x12\xBBW[P\x15a\x12\xACW_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[a\x12\xD4\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x81a\x12rV[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x12\xF3a\x1D6V[a\x12\xFBa\x1DLV[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`@_ T\x16`@Q`\x02\x82\x10\x15a\x05\x9BW` \x91\x81R\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x13Za\x13U6a\x1DbV[a#\xB8V[\x80_R`\x9E` R`@_ T\x90a\x13q\x82a \xA0V[\x91a\x13\x7F`@Q\x93\x84a\x1D\x15V[\x80\x83Ra\x13\x8B\x81a \xA0V[` \x84\x01\x92\x90`\x1F\x19\x016\x847_[\x82\x81\x10a\x13\xEEW\x83\x85`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x13\xCCWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13\xBEV[`\x01\x90\x82_R`\x9E` Ra\x14\x06\x81`@_ a(\xD8V[\x90T`\x03\x91\x90\x91\x1B\x1C`\xA0\x83\x90\x1B\x83\x90\x03\x16a\x14\"\x82\x88a \xDCV[R\x01a\x13\x9AV[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x14Fa\x13U6a\x1DbV[_R`\x9D` R` a\x14_`@_ `D5\x90a(\xD8V[\x90T`@Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R\xF3[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16Wa\x14\x91a\x1D6V[a\x14\x99a\x1DLV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x14\xB9\x906\x90`\x04\x01a\x1D\xEEV[\x91`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x14\xD9\x906\x90`\x04\x01a\x1E9V[`\x02\x80`fT\x16\x14a\x05\xBEW\x80QQa\x15\x06WP`\x01`\x01`\xA0\x1B\x03\x81\x163\x03a\x05\x8CWa\x03u\x93a&[V[\x93`@\x85\x01\x80QB\x11a\x0EaW`\x01\x80`\xA0\x1B\x03\x83\x16\x80_R`\x99` R`@_ \x91` \x88\x01\x92\x83Q_R` R`\xFF`@_ T\x16a\x0ECWa\x03u\x97a\x15Ya\x15b\x92\x85Q\x90Q\x90\x8A\x8A\x89a\"\xE3V[\x90Q\x90\x86a$\xEBV[_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua&[V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16W`\x01`\x01`\xA0\x1B\x03a\x15\xA9a\x1D6V[\x16_R`\x99` R`@_ `$5_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W6`#\x82\x01\x12\x15a\x03\x16W\x80`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16W6`$\x83`\x06\x1B\x83\x01\x01\x11a\x03\x16W` \x91`$a\x06\xB5\x92\x01a!9V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x16\x89a\x1D6V[a\x16\x91a\x1DLV[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x03\x16W`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`@\x80_ Tc\xFF\xFF\xFF\xFF\x82Q\x91`\xFF\x81\x16\x15\x15\x83R`\x08\x1C\x16` \x82\x01R\xF3[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x17\ra\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x17,\x906\x90`\x04\x01a\x1D\xEEV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x17L\x906\x90`\x04\x01a\x1E9V[\x92`\x02\x80`fT\x16\x14a\x05\xBEW`@\x84\x01\x80QB\x11a\x0EaW`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01\x81\x90R\x91\x90` \x90\x82\x90`$\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x18KW[P\x15a\x0E\x05W3_R`\x9A` R`\xFF`@_ T\x16\x15a\x05\xAFW\x80_R`\x99` R`@_ \x91` \x87\x01\x92\x83Q_R` R`\xFF`@_ T\x16a\x0ECWa\x03u\x96a\x18\x1Aa\x18#\x92\x85Q\x90Q\x90\x89\x893a\"\0V[\x90Q\x90\x85a$\xEBV[_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3\x90a%\x0CV[a\x18d\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x87a\x17\xC2V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x18\x83a\x1D6V[a\x18\x8Ba\x1DLV[`D5\x91_T\x92`\xFF\x84`\x08\x1C\x16\x15\x93\x84\x80\x95a\x19\xFBW[\x80\x15a\x19\xE4W[\x15a\x19\x88W`\xFF\x19\x81\x16`\x01\x17_U\x84a\x19wW[P`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x19eW[\x15a\x19VWa\x19\x1A\x92\x81a\x19\x15\x92`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a$\x1CV[a$\xA3V[a\x19 W\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15a\x18\xD3V[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x84a\x18\xBFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x18\xAAWP`\x01`\xFF\x82\x16\x14a\x18\xAAV[P`\x01`\xFF\x82\x16\x10a\x18\xA3V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x1A!a\x1D6V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9C` R`@\x90 T`D5\x90`$5\x90a\x1AK\x90\x82\x90a \x93V[\x80\x83\x11a\x1BMW[Pa\x1A]\x82a \xA0V[\x92a\x1Ak`@Q\x94\x85a\x1D\x15V[\x82\x84R`\x1F\x19a\x1Az\x84a \xA0V[\x01_[\x81\x81\x10a\x1B*WPP_[\x83\x81\x10a\x1A\xEFW\x84`@Q\x80\x91` \x82\x01` \x83R\x81Q\x80\x91R` `@\x84\x01\x92\x01\x90_[\x81\x81\x10a\x1A\xBBWPPP\x03\x90\xF3[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x86\x01R\x86\x95P`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x1A\xADV[`\x01\x90\x82_R`\x9C` Ra\x1B\x0Ea\x08\xEE`@_ a\x11G\x84\x88a \xCFV[a\x1B\x18\x82\x88a \xDCV[Ra\x1B#\x81\x87a \xDCV[P\x01a\x1A\x88V[` \x90`@Qa\x1B9\x81a\x1C\xE6V[_\x81R_\x83\x82\x01R\x82\x82\x89\x01\x01R\x01a\x1A}V[\x91P\x83a\x1ASV[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`$` `\x01\x80`\xA0\x1B\x03`eT\x16`@Q\x92\x83\x80\x92c#}\xFBG`\xE1\x1B\x82R3`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\x0BW_\x91a\x1B\xE2W[P\x15a\x12\xACW`fT\x81\x81\x16\x03a\x02\xBEW\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x1B\xFB\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x82a\x1B\x9FV[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16W` a\x06\xB5a\x1C\x1Fa\x1D6V[a\x1C(6a\x1D\xA8V[\x90a\x1F\xB1V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x1CGa\x1D6V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x1C\x94W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xCDWa\x03u\x90a$\x1CV[a\x1C\xAD\x91P` =` \x11a\x03\x04Wa\x02\xF6\x81\x83a\x1D\x15V[\x82a\x1C{V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x1C\xD0a\x13U6a\x1DbV[_R`\x9D` R` `@_ T`@Q\x90\x81R\xF3[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@RV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x16WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x16WV[`@\x90`\x03\x19\x01\x12a\x03\x16W`@Q\x90a\x1D{\x82a\x1C\xE6V[\x81`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x81R`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x01RV[`@\x90`#\x19\x01\x12a\x03\x16W`@Q\x90a\x1D\xC1\x82a\x1C\xE6V[\x81`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x81R`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\x16W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x16W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\x16WV[`\x01`\x01`@\x1B\x03\x81\x11a\x1D\x01W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90``\x83\x82\x03\x12a\x03\x16W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@R\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W\x81\x01\x90\x82`\x1F\x83\x01\x12\x15a\x03\x16W\x815\x92a\x1E\x8E\x84a\x1E\x1EV[\x90a\x1E\x9C`@Q\x92\x83a\x1D\x15V[\x84\x82R` \x85\x85\x01\x01\x11a\x03\x16W_` \x85`@\x96\x82\x88\x97\x01\x83\x86\x017\x83\x01\x01R\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[\x90`@`\x03\x19\x83\x01\x12a\x03\x16W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x16W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16Wa\x1F\x0C\x91`\x04\x01a\x1D\xEEV[\x90\x91V[\x90`\x80`\x03\x19\x83\x01\x12a\x03\x16W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16Wa\x1FO\x91`\x04\x01a\x1D\xEEV[\x90\x91`D5\x90`d5\x90V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x1Fta\x1D6V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R` `@_ T`@Q\x90\x81R\xF3[\x90\x81` \x91\x03\x12a\x03\x16WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x90V[a\x1F\xBB\x82\x82a#=V[a tW`\x01\x80`\xA0\x1B\x03\x82Q\x16_R`\x9F` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` Rc\xFF\xFF\xFF\xFF\x80` `@_ \x93\x01Q\x16\x16_R` R` `@_ c\xFF\xFF\xFF\xFF`@Q\x91a \x10\x83a\x1C\xE6V[T`\xFF\x81\x16\x15\x15\x83R`\x08\x1C\x16\x91\x82\x91\x01Rc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x01c\xFF\xFF\xFF\xFF\x81\x11a `Wc\xFF\xFF\xFF\xFF\x16B\x10\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PP`\x01\x90V[\x90\x81` \x91\x03\x12a\x03\x16WQ\x80\x15\x15\x81\x03a\x03\x16W\x90V[\x91\x90\x82\x03\x91\x82\x11a `WV[`\x01`\x01`@\x1B\x03\x81\x11a\x1D\x01W`\x05\x1B` \x01\x90V[`@Q\x90a \xC4\x82a\x1C\xE6V[_` \x83\x82\x81R\x01RV[\x91\x90\x82\x01\x80\x92\x11a `WV[\x80Q\x82\x10\x15a \xF0W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a \xF0W`\x06\x1B\x01\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x16W\x90V[_[\x82\x81\x10a!JWPPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03a!`a\x04a\x83\x86\x86a!\x04V[\x16_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa!\x87` a!\x81\x85\x88\x88a!\x04V[\x01a!(V[\x16_R` R`\xFF`@_ T\x16\x15a!\xA2W`\x01\x01a!;V[PPP_\x90V[\x91\x90\x81\x10\x15a \xF0W`\x05\x1B\x01\x90V[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a!\xD3WPPP\x90V[\x90\x91\x92\x835\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x80\x91c\xFF\xFF\xFF\xFF`\x01\x94\x16\x81R\x01\x94\x01\x92\x01\x90a!\xC6V[\x92\x91a\"s\x91\x92a\"{\x95a\"Z`@Q\x95\x86\x94` \x86\x01\x98\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x8AR`\x01\x80`\xA0\x1B\x03\x16`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a!\xB9V[\x91`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x1D\x15V[Q\x90 a'\xE2V[\x90V[\x91\x92a\"{\x93`@Q\x92` \x84\x01\x94\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x86R`\x01\x80`\xA0\x1B\x03\x16`@\x85\x01R`\x01\x80`\xA0\x1B\x03\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\"s`\xC0\x82a\x1D\x15V[\x92\x91a\"s\x91\x92a\"{\x95a\"Z`@Q\x95\x86\x94` \x86\x01\x98\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x8AR`\x01\x80`\xA0\x1B\x03\x16`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a!\xB9V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9C` R`@\x90 a\"{\x91a#`\x90a#\xB8V[\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[\x91\x90\x81\x10\x15a \xF0W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\x16W\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x16W` \x01\x82`\x05\x1B6\x03\x81\x13a\x03\x16W\x91\x90V[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra#\xFE`@\x82a\x1D\x15V[Q\x90Q\x90` \x81\x10a$\x0EWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x19VW`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[a$\x81a \xB7V[Pc\xFF\xFF\xFF\xFF`@Q\x91a$\x94\x83a\x1C\xE6V[\x80``\x1C\x83R\x16` \x82\x01R\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90a$\xF6\x92\x91a(\xEDV[\x15a$\xFDWV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91_[\x83\x81\x10a%\x1FWPPPPPV[a%-a\t\x95\x82\x86\x85a!\xA9V[\x90`@Q\x91a%;\x83a\x1C\xE6V[c\xFF\xFF\xFF\xFF`\x01\x80`\xA0\x1B\x03\x88\x16\x91\x82\x85R\x16` \x84\x01R\x80_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa%ra\t\x95\x85\x8A\x89a!\xA9V[\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa%\x8E\x83a#\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x86\x16\x91\x82_R`\x9C` Ra%\xAE\x81`@_ a*\x08V[P_R`\x9D` Ra%\xC3\x82`@_ a*\x08V[P_R`\x9F` R`@_ \x81_R` R`@_ c\xFF\xFF\xFF\xFFa%\xECa\t\x95\x85\x8A\x89a!\xA9V[\x16_R` R`@_ \x80T`\xFF\x81\x16a\x05\x8CW`\x01\x94\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x92\x86`@\x93`\xFF\x19\x16\x17\x90Ua&T\x82Q\x80\x92c\xFF\xFF\xFF\xFF` \x80\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\xA2\x01a%\x11V[\x92\x91B`\x08\x1Bd\xFF\xFF\xFF\xFF\0\x16\x90_[\x84\x81\x10a&zWPPPPPPV[a&\x88a\t\x95\x82\x87\x85a!\xA9V[\x90`@Q\x91a&\x96\x83a\x1C\xE6V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x80\x84Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x84\x01Ra&\xBA\x83a#\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x87\x16\x91\x82_R`\x9C` Ra&\xDA\x81`@_ a*vV[P_R`\x9D` Ra&\xEF\x82`@_ a*vV[P_R`\x9F` R`@_ \x81_R` R`@_ c\xFF\xFF\xFF\xFFa'\x18a\t\x95\x85\x8B\x89a!\xA9V[\x16_R` R`@_ \x80T`\xFF\x81\x16\x15a\x05\x8CWd\xFF\xFF\xFF\xFF\xFF\x19\x16\x86\x17\x90U`@\x80Q\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x94\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R`\x01\x93\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x91\x90\xA2\x01a&kV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a'\x9EWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a'\xEAa(\x1BV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra(\x15`b\x82a\x1D\x15V[Q\x90 \x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a(fW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@Qa(x`@\x82a\x1D\x15V[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra(\x15`\xA0\x82a\x1D\x15V[\x80T\x82\x10\x15a \xF0W_R` _ \x01\x90_\x90V[\x91\x90\x91a(\xFA\x82\x84a+;V[`\x05\x81\x10\x15a\x05\x9BW\x15\x90\x81a)\xF2W[Pa)\xEAW_\x92` a)j`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x1D\x15V[Q\x91Z\xFA=\x15a)\xE3W=a)~\x81a\x1E\x1EV[\x90a)\x8C`@Q\x92\x83a\x1D\x15V[\x81R=_` \x83\x01>[\x81a)\xD7W[\x81a)\xA5WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\x03\x16W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\x03\x16Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa)\x9CV[``a)\x96V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a)\x0BV[_\x82\x81R`\x01\x82\x01` R`@\x90 Ta*pW\x80T\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1D\x01W\x82a*[a*E\x84`\x01\x80\x96\x01\x85U\x84a(\xD8V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80T\x92_R\x01` R`@_ U`\x01\x90V[PP_\x90V[\x90`\x01\x82\x01\x91\x81_R\x82` R`@_ T\x80\x15\x15_\x14a+3W_\x19\x81\x01\x81\x81\x11a `W\x82T_\x19\x81\x01\x91\x90\x82\x11a `W\x80\x82\x03a*\xFEW[PPP\x80T\x80\x15a*\xEAW_\x19\x01\x90a*\xCB\x82\x82a(\xD8V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UU_R` R_`@\x81 U`\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[a+\x1Ea+\x0Ea*E\x93\x86a(\xD8V[\x90T\x90`\x03\x1B\x1C\x92\x83\x92\x86a(\xD8V[\x90U_R\x83` R`@_ U_\x80\x80a*\xB2V[PPPP_\x90V[\x81Q`A\x81\x03a+cWP\x90a\x1F\x0C\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a+\xA5V[`@\x03a+\x9CW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a `Wa\x1F\x0C\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a+\xA5V[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a,<W`\xFF\x16\x90`\x1B\x82\x14\x15\x80a,1W[a,&W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x03\x0BW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a,\x1EW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a+\xDCV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xF1\x12\x16U\xCD1\xC9M\xF6\xB8.P\xAD\x93\xBBd\xEFZ\x1C\x830\xC8\x80\xC2\x90\xA9\x04\r\xFD\")rdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361015610011575f80fd5b5f3560e01c80631023aa3514611cb357806310d67a2f14611c2e5780631352c3e614611c01578063136439dd14611b5557806316ae76cb14611a085780631794bb3c1461186a5780631e2199e2146116f45780631e68134e1461167057806320c4e236146116115780632981eb77146115d1578063374823b5146115885780633fee332d14611478578063411d415b146114295780634177a87c1461133857806349075da3146112da578063595c6a67146112285780635ac86ab7146111f55780635c975abb146111d8578063715018a61461117d5780637357723b1461105b5780637673e93a1461101e5780637699934214610f3957806384d76f7b14610edc578063886f119514610eb45780638da5cb5b14610e8c578063955e669614610e705780639926ee7d14610c58578063a1060c8814610c24578063a364f4da14610b50578063a98fb35514610aae578063aec205c514610a4c578063afe02ed51461093a578063b2841d481461091e578063b5a768ca146108aa578063c1a8e2c51461085b578063c825fe6814610821578063cbdf0e4214610605578063ce7b5e4b146106f9578063d79aceab146106bf578063da2ff05d14610688578063dce974b91461064e578063df5cf7231461060a578063e88d804914610605578063ec76f442146105cd578063ef2dfa8d146103cb578063f2fde38b1461033c578063f698da251461031a5763fabc1cbc14610229575f80fd5b346103165760203660031901126103165760655460405163755b36bd60e11b81526004803592602091839182906001600160a01b03165afa90811561030b575f916102dc575b506001600160a01b031633036102cd576066541981198116036102be57806066556040519081527f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c60203392a2005b63c61dca5d60e01b5f5260045ffd5b63794821ff60e01b5f5260045ffd5b6102fe915060203d602011610304575b6102f68183611d15565b810190611f92565b5f61026f565b503d6102ec565b6040513d5f823e3d90fd5b5f80fd5b34610316575f36600319011261031657602061033461281b565b604051908152f35b3461031657602036600319011261031657610355611d36565b61035d61278a565b6001600160a01b0381161561037757610375906124a3565b005b60405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608490fd5b34610316576040366003190112610316576004356001600160401b038111610316576103fb903690600401611dee565b6024356001600160401b0381116103165761041a903690600401611dee565b9160028060665416146105be57335f52609a60205260ff60405f205416156105af575f5b81811061044757005b335f52609860205260405f206104666104618385896121a9565b612114565b60018060a01b03165f5260205260ff60405f205416600281101561059b5760010361058c57806104b261049f610461600194868a6121a9565b6104aa838888612374565b91339061250c565b335f52609860205260405f206104cc61046183868a6121a9565b838060a01b03165f5260205260405f2060ff198154169055818060a01b036104f861046183868a6121a9565b16604051905f82527ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4160203393a36105346104618285896121a9565b6105837f54f33cfdd1ca703d795986b986fd47d742eab1904ecd2a5fdb8d6595e5904a01610563848989612374565b9290604051918291602083523396898060a01b03169560208401916121b9565b0390a30161043e565b63ccea9e6f60e01b5f5260045ffd5b634e487b7160e01b5f52602160045260245ffd5b6366e565df60e01b5f5260045ffd5b63840a48d560e01b5f5260045ffd5b3461031657602036600319011261031657335f52609960205260405f206004355f5260205260405f20600160ff198254161790555f80f35b611f5b565b34610316575f366003190112610316576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b34610316575f3660031901126103165760206040517f4ee65f64218c67b68da66fd0db16560040a6b973290b9e71912d661ee53fe4958152f35b346103165760603660031901126103165760206106b56106a6611d36565b6106af36611da8565b9061233d565b6040519015158152f35b34610316575f3660031901126103165760206040517fda2c89bafdd34776a2b8bb9c83c82f419e20cc8c67207f70edd58249b92661bd8152f35b346103165761070736611ed0565b604051919261071583611ce6565b33835263ffffffff81166020840152335f52609b60205263ffffffff60405f2091165f5260205260ff60405f2054161561081257610752826123b8565b915f5b82811061075e57005b5f848152609e6020526040902061078c906001600160a01b0361078561046185888b6121a9565b1690612a76565b1561080357807f7b4b073d80dcac55a11177d8459ad9f664ceeb91f71f27167bb14f8152a7eeee6107fa6107c6610461600195888b6121a9565b6040805187516001600160a01b0390811682526020808a015163ffffffff1690830152909216908201529081906060820190565b0390a101610755565b6331bc342760e11b5f5260045ffd5b631fb1705560e21b5f5260045ffd5b34610316575f3660031901126103165760206040517f809c5ac049c45b7a7f050a20f00c16cf63797efbf8b1eb8d749fdfa39ff8f9298152f35b3461031657604036600319011261031657610874611d36565b6024356001600160401b03811161031657610893903690600401611dee565b9060028060665416146105be57610375923361265b565b34610316576040366003190112610316576108c3611d36565b6108cb6120b7565b5060018060a01b03165f52609c60205260406108fa6108ee602435835f206128d8565b90549060031b1c612479565b815181516001600160a01b0316815260209182015163ffffffff1691810191909152f35b3461031657602061033461093136611f10565b939290926122e3565b34610316576020366003190112610316576004356001600160401b0381116103165761096a903690600401611dee565b905f5b82811061097657005b335f52609b60205260405f2063ffffffff61099a6109958487876121a9565b612128565b165f5260205260ff60405f20541661081257600190335f52609b60205260405f2063ffffffff6109ce6109958488886121a9565b165f5260205260405f208260ff198254161790557f31629285ead2335ae0933f86ed2ae63321f7af77b4e6eaabc42c057880977e6c6040610a136109958488886121a9565b63ffffffff825191610a2483611ce6565b338084529116602092830190815283519182525163ffffffff1691810191909152a10161096d565b34610316575f36600319011261031657335f52609a60205260ff60405f2054166105af57335f52609a60205260405f20600160ff19825416179055337f702b0c1f6cb1cf511aaa81f72bc05a215bb3497632d72c690c822b044ab494bf5f80a2005b34610316576020366003190112610316576004356001600160401b0381116103165736602382011215610316578060040135906001600160401b038211610316573660248383010111610316577fa89c1dc243d8908a96dd84944bcc97d6bc6ac00dd78e20621576be6a3c9437139082602460405192602084528260208501520160408301375f604084830101526040813394601f80199101168101030190a2005b3461031657602036600319011261031657610b69611d36565b60018060665416146105be57335f52609860205260405f2060018060a01b0382165f5260205260ff60405f205416600281101561059b57600103610c1557335f52609a60205260ff60405f2054166105af57335f8181526098602090815260408083206001600160a01b0395909516808452948252808320805460ff19169055519182529192917ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4191a3005b6352df45c960e01b5f5260045ffd5b34610316576080366003190112610316576020610334610c42611d36565b610c4a611d4c565b90606435916044359161227e565b3461031657604036600319011261031657610c71611d36565b6024356001600160401b03811161031657610c90903690600401611e39565b60018060665416146105be576040810180514211610e6157335f52609a60205260ff60405f2054166105af57335f52609860205260405f2060018060a01b0384165f5260205260ff60405f205416600281101561059b57600114610e525760018060a01b03831692835f52609960205260405f2092602081019384515f5260205260ff60405f205416610e43576040516336b87bd760e11b8152600481018690526020816024817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa90811561030b575f91610e14575b5015610e0557610d8a610d92938551905190338561227e565b9051916124eb565b815f52609960205260405f2090515f5260205260405f20600160ff19825416179055335f52609860205260405f20815f5260205260405f20600160ff1982541617905560405190600182527ff0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b4160203393a3005b639f88c8af60e01b5f5260045ffd5b610e36915060203d602011610e3c575b610e2e8183611d15565b81019061207b565b86610d71565b503d610e24565b630d4c4c9160e21b5f5260045ffd5b631aa528bb60e11b5f5260045ffd5b630819bdcd60e01b5f5260045ffd5b34610316576020610334610e8336611f10565b93929092612200565b34610316575f366003190112610316576033546040516001600160a01b039091168152602090f35b34610316575f366003190112610316576065546040516001600160a01b039091168152602090f35b3461031657604036600319011261031657610ef5611d36565b6024359063ffffffff821682036103165760018060a01b03165f52609b60205263ffffffff60405f2091165f52602052602060ff60405f2054166040519015158152f35b3461031657610f4736611ed0565b6040519192610f5583611ce6565b33835263ffffffff81166020840152335f52609b60205263ffffffff60405f2091165f5260205260ff60405f2054161561081257610f92826123b8565b915f5b828110610f9e57005b5f848152609e60205260409020610fcc906001600160a01b03610fc561046185888b6121a9565b1690612a08565b1561100f57807f7ab260fe0af193db5f4986770d831bda4ea46099dc817e8b6716dcae8af8e88b6110066107c6610461600195888b6121a9565b0390a101610f95565b63585cfb2f60e01b5f5260045ffd5b34610316576020366003190112610316576001600160a01b0361103f611d36565b165f52609a602052602060ff60405f2054166040519015158152f35b346103165760803660031901126103165761107536611d62565b60443590611085606435916123b8565b90815f52609d60205261109c8360405f2054612093565b808211611175575b506110ae816120a0565b926110bc6040519485611d15565b8184526110c8826120a0565b602085019390601f19013685375f5b83811061112b578486604051918291602083019060208452518091526040830191905f5b818110611109575050500390f35b82516001600160a01b03168452859450602093840193909201916001016110fb565b600190825f52609d60205261114d60405f2061114783876120cf565b906128d8565b838060a01b0391549060031b1c1661116582896120dc565b90838060a01b03169052016110d7565b9050836110a4565b34610316575f3660031901126103165761119561278a565b603380546001600160a01b031981169091555f906001600160a01b03167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e08280a3005b34610316575f366003190112610316576020606654604051908152f35b346103165760203660031901126103165760043560ff81168091036103165760016020911b806066541614604051908152f35b34610316575f3660031901126103165760655460405163237dfb4760e11b815233600482015290602090829060249082906001600160a01b03165afa90811561030b575f916112bb575b50156112ac575f196066556040515f1981527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b631d77d47760e21b5f5260045ffd5b6112d4915060203d602011610e3c57610e2e8183611d15565b81611272565b34610316576040366003190112610316576112f3611d36565b6112fb611d4c565b9060018060a01b03165f52609860205260405f209060018060a01b03165f5260205260ff60405f205416604051600282101561059b576020918152f35b346103165760403660031901126103165761135a61135536611d62565b6123b8565b805f52609e60205260405f205490611371826120a0565b9161137f6040519384611d15565b80835261138b816120a0565b602084019290601f19013684375f5b8281106113ee578385604051918291602083019060208452518091526040830191905f5b8181106113cc575050500390f35b82516001600160a01b03168452859450602093840193909201916001016113be565b600190825f52609e6020526114068160405f206128d8565b905460039190911b1c60a083901b8390031661142282886120dc565b520161139a565b346103165760603660031901126103165761144661135536611d62565b5f52609d602052602061145f60405f20604435906128d8565b905460405160039290921b1c6001600160a01b03168152f35b3461031657608036600319011261031657611491611d36565b611499611d4c565b906044356001600160401b038111610316576114b9903690600401611dee565b916064356001600160401b038111610316576114d9903690600401611e39565b60028060665416146105be5780515161150657506001600160a01b038116330361058c576103759361265b565b936040850180514211610e615760018060a01b038316805f52609960205260405f2091602088019283515f5260205260ff60405f205416610e4357610375976115596115629285519051908a8a896122e3565b905190866124eb565b5f52609960205260405f2090515f5260205260405f20600160ff1982541617905561265b565b34610316576040366003190112610316576001600160a01b036115a9611d36565b165f52609960205260405f206024355f52602052602060ff60405f2054166040519015158152f35b34610316575f36600319011261031657602060405163ffffffff7f0000000000000000000000000000000000000000000000000000000000000000168152f35b34610316576020366003190112610316576004356001600160401b0381116103165736602382011215610316578060040135906001600160401b038211610316573660248360061b830101116103165760209160246106b59201612139565b3461031657606036600319011261031657611689611d36565b611691611d4c565b6044359163ffffffff831683036103165760018060a01b03165f52609f60205260405f209060018060a01b03165f5260205263ffffffff60405f2091165f526020526040805f205463ffffffff82519160ff81161515835260081c166020820152f35b346103165760603660031901126103165761170d611d36565b6024356001600160401b0381116103165761172c903690600401611dee565b906044356001600160401b0381116103165761174c903690600401611e39565b9260028060665416146105be576040840180514211610e61576040516336b87bd760e11b81526001600160a01b03838116600483018190529190602090829060249082907f0000000000000000000000000000000000000000000000000000000000000000165afa90811561030b575f9161184b575b5015610e0557335f52609a60205260ff60405f205416156105af57805f52609960205260405f2091602087019283515f5260205260ff60405f205416610e43576103759661181a611823928551905190898933612200565b905190856124eb565b5f52609960205260405f2090515f5260205260405f20600160ff19825416179055339061250c565b611864915060203d602011610e3c57610e2e8183611d15565b876117c2565b3461031657606036600319011261031657611883611d36565b61188b611d4c565b604435915f549260ff8460081c1615938480956119fb575b80156119e4575b156119885760ff1981166001175f5584611977575b506065546001600160a01b03161580611965575b156119565761191a9281611915926066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a261241c565b6124a3565b61192057005b61ff00195f54165f557f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498602060405160018152a1005b6339b190bb60e11b5f5260045ffd5b506001600160a01b03831615156118d3565b61ffff1916610101175f55846118bf565b60405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608490fd5b50303b1580156118aa5750600160ff8216146118aa565b50600160ff8216106118a3565b3461031657606036600319011261031657611a21611d36565b6001600160a01b03165f818152609c60205260409020546044359060243590611a4b908290612093565b808311611b4d575b50611a5d826120a0565b92611a6b6040519485611d15565b828452601f19611a7a846120a0565b015f5b818110611b2a5750505f5b838110611aef57846040518091602082016020835281518091526020604084019201905f5b818110611abb575050500390f35b825180516001600160a01b0316855260209081015163ffffffff168186015286955060409094019390920191600101611aad565b600190825f52609c602052611b0e6108ee60405f2061114784886120cf565b611b1882886120dc565b52611b2381876120dc565b5001611a88565b602090604051611b3981611ce6565b5f81525f8382015282828901015201611a7d565b915083611a53565b34610316576020366003190112610316576004356024602060018060a01b03606554166040519283809263237dfb4760e11b82523360048301525afa90811561030b575f91611be2575b50156112ac57606654818116036102be57806066556040519081527fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d60203392a2005b611bfb915060203d602011610e3c57610e2e8183611d15565b82611b9f565b346103165760603660031901126103165760206106b5611c1f611d36565b611c2836611da8565b90611fb1565b3461031657602036600319011261031657611c47611d36565b60655460405163755b36bd60e11b815290602090829060049082906001600160a01b03165afa90811561030b575f91611c94575b506001600160a01b031633036102cd576103759061241c565b611cad915060203d602011610304576102f68183611d15565b82611c7b565b3461031657604036600319011261031657611cd061135536611d62565b5f52609d602052602060405f2054604051908152f35b604081019081106001600160401b03821117611d0157604052565b634e487b7160e01b5f52604160045260245ffd5b90601f801991011681019081106001600160401b03821117611d0157604052565b600435906001600160a01b038216820361031657565b602435906001600160a01b038216820361031657565b60409060031901126103165760405190611d7b82611ce6565b816004356001600160a01b03811681036103165781526024359063ffffffff821682036103165760200152565b60409060231901126103165760405190611dc182611ce6565b816024356001600160a01b03811681036103165781526044359063ffffffff821682036103165760200152565b9181601f84011215610316578235916001600160401b038311610316576020808501948460051b01011161031657565b6001600160401b038111611d0157601f01601f191660200190565b91906060838203126103165760405190606082018281106001600160401b03821117611d0157604052819380356001600160401b0381116103165781019082601f8301121561031657813592611e8e84611e1e565b90611e9c6040519283611d15565b84825260208585010111610316575f6020856040968288970183860137830101528452602081013560208501520135910152565b9060406003198301126103165760043563ffffffff811681036103165791602435906001600160401b03821161031657611f0c91600401611dee565b9091565b906080600319830112610316576004356001600160a01b03811681036103165791602435906001600160401b03821161031657611f4f91600401611dee565b90916044359060643590565b3461031657602036600319011261031657611f74611d36565b60018060a01b03165f52609c602052602060405f2054604051908152f35b9081602091031261031657516001600160a01b03811681036103165790565b611fbb828261233d565b6120745760018060a01b038251165f52609f60205260405f209060018060a01b03165f5260205263ffffffff80602060405f2093015116165f52602052602060405f2063ffffffff6040519161201083611ce6565b5460ff81161515835260081c16918291015263ffffffff7f0000000000000000000000000000000000000000000000000000000000000000160163ffffffff81116120605763ffffffff16421090565b634e487b7160e01b5f52601160045260245ffd5b5050600190565b90816020910312610316575180151581036103165790565b9190820391821161206057565b6001600160401b038111611d015760051b60200190565b604051906120c482611ce6565b5f6020838281520152565b9190820180921161206057565b80518210156120f05760209160051b010190565b634e487b7160e01b5f52603260045260245ffd5b91908110156120f05760061b0190565b356001600160a01b03811681036103165790565b3563ffffffff811681036103165790565b5f5b82811061214a57505050600190565b6001600160a01b03612160610461838686612104565b165f52609b60205260405f2063ffffffff6121876020612181858888612104565b01612128565b165f5260205260ff60405f205416156121a25760010161213b565b5050505f90565b91908110156120f05760051b0190565b916020908281520191905f905b8082106121d35750505090565b90919283359063ffffffff82168203610316576020809163ffffffff6001941681520194019201906121c6565b9291612273919261227b9561225a60405195869460208601987f809c5ac049c45b7a7f050a20f00c16cf63797efbf8b1eb8d749fdfa39ff8f9298a5260018060a01b0316604087015260a0606087015260c08601916121b9565b91608084015260a083015203601f198101835282611d15565b5190206127e2565b90565b919261227b936040519260208401947fda2c89bafdd34776a2b8bb9c83c82f419e20cc8c67207f70edd58249b92661bd865260018060a01b0316604085015260018060a01b03166060840152608083015260a082015260a0815261227360c082611d15565b9291612273919261227b9561225a60405195869460208601987f4ee65f64218c67b68da66fd0db16560040a6b973290b9e71912d661ee53fe4958a5260018060a01b0316604087015260a0606087015260c08601916121b9565b6001600160a01b03165f908152609c6020526040902061227b91612360906123b8565b906001915f520160205260405f2054151590565b91908110156120f05760051b81013590601e19813603018212156103165701908135916001600160401b038311610316576020018260051b36038113610316579190565b602081519101516040519060208201926bffffffffffffffffffffffff199060601b16835263ffffffff60a01b9060a01b166034820152602081526123fe604082611d15565b519051906020811061240e575090565b5f199060200360031b1b1690565b6001600160a01b0316801561195657606554604080516001600160a01b0383168152602081018490527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb69190a16001600160a01b03191617606555565b6124816120b7565b5063ffffffff6040519161249483611ce6565b8060601c835216602082015290565b603380546001600160a01b039283166001600160a01b0319821681179092559091167f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e05f80a3565b906124f692916128ed565b156124fd57565b638baa579f60e01b5f5260045ffd5b9092915f5b83811061251f575050505050565b61252d6109958286856121a9565b906040519161253b83611ce6565b63ffffffff60018060a01b03881691828552166020840152805f52609b60205260405f2063ffffffff612572610995858a896121a9565b165f5260205260ff60405f205416156108125761258e836123b8565b9060018060a01b03861691825f52609c6020526125ae8160405f20612a08565b505f52609d6020526125c38260405f20612a08565b505f52609f60205260405f20815f5260205260405f2063ffffffff6125ec610995858a896121a9565b165f5260205260405f20805460ff811661058c576001947f43232edf9071753d2321e5fa7e018363ee248e5f2142e6c08edd3265bfb4895e928660409360ff19161790556126548251809263ffffffff6020809260018060a01b038151168552015116910152565ba201612511565b92914260081b64ffffffff0016905f5b84811061267a57505050505050565b6126886109958287856121a9565b906040519161269683611ce6565b6001600160a01b03881680845263ffffffff9190911660208401526126ba836123b8565b9060018060a01b03871691825f52609c6020526126da8160405f20612a76565b505f52609d6020526126ef8260405f20612a76565b505f52609f60205260405f20815f5260205260405f2063ffffffff612718610995858b896121a9565b165f5260205260405f20805460ff81161561058c5764ffffffffff1916861790556040805184516001600160a01b0316815260209485015163ffffffff16948101949094526001937fad34c3070be1dffbcaa499d000ba2b8d9848aefcac3059df245dd95c4ece14fe9190a20161266b565b6033546001600160a01b0316330361279e57565b606460405162461bcd60e51b815260206004820152602060248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152fd5b6127ea61281b565b9060405190602082019261190160f01b84526022830152604282015260428152612815606282611d15565b51902090565b467f000000000000000000000000000000000000000000000000000000000000000003612866577f000000000000000000000000000000000000000000000000000000000000000090565b600a6020604051612878604082611d15565b828152016922b4b3b2b72630bcb2b960b11b81522060405160208101917f8cad95687ba82c2ce50e74f7b754645e5117c3a5bec8151c0726d5857980a866835260408201524660608201523060808201526080815261281560a082611d15565b80548210156120f0575f5260205f2001905f90565b9190916128fa8284612b3b565b600581101561059b571590816129f2575b506129ea575f92602061296a6084869560405193849181830196630b135d3f60e11b88526024840152604060448401528051918291826064860152018484015e87838284010152601f801991011681010301601f198101835282611d15565b51915afa3d156129e3573d61297e81611e1e565b9061298c6040519283611d15565b81523d5f602083013e5b816129d7575b816129a5575090565b905060208180518101031261031657602001516001600160e01b031981169081900361031657630b135d3f60e11b1490565b8051602014915061299c565b6060612996565b505050600190565b6001600160a01b0383811691161490505f61290b565b5f828152600182016020526040902054612a705780549068010000000000000000821015611d015782612a5b612a458460018096018555846128d8565b819391549060031b91821b915f19901b19161790565b90558054925f520160205260405f2055600190565b50505f90565b906001820191815f528260205260405f20548015155f14612b33575f1981018181116120605782545f1981019190821161206057808203612afe575b50505080548015612aea575f190190612acb82826128d8565b8154905f199060031b1b19169055555f526020525f6040812055600190565b634e487b7160e01b5f52603160045260245ffd5b612b1e612b0e612a4593866128d8565b90549060031b1c928392866128d8565b90555f528360205260405f20555f8080612ab2565b505050505f90565b815160418103612b63575090611f0c91602082015190606060408401519301515f1a90612ba5565b604003612b9c5760406020830151920151918260ff1c91601b830180931161206057611f0c936001600160ff1b03169260ff1690612ba5565b50505f90600290565b907f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08411612c3c5760ff1690601b82141580612c31575b612c26576020935f93608093604051938452868401526040830152606082015282805260015afa1561030b575f516001600160a01b03811615612c1e57905f90565b505f90600190565b505050505f90600490565b50601c821415612bdc565b505050505f9060039056fea2646970667358221220f1121655cd31c94df6b82e50ad93bb64ef5a1c8330c880c290a9040dfd22297264736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10\x15a\0\x11W_\x80\xFD[_5`\xE0\x1C\x80c\x10#\xAA5\x14a\x1C\xB3W\x80c\x10\xD6z/\x14a\x1C.W\x80c\x13R\xC3\xE6\x14a\x1C\x01W\x80c\x13d9\xDD\x14a\x1BUW\x80c\x16\xAEv\xCB\x14a\x1A\x08W\x80c\x17\x94\xBB<\x14a\x18jW\x80c\x1E!\x99\xE2\x14a\x16\xF4W\x80c\x1Eh\x13N\x14a\x16pW\x80c \xC4\xE26\x14a\x16\x11W\x80c)\x81\xEBw\x14a\x15\xD1W\x80c7H#\xB5\x14a\x15\x88W\x80c?\xEE3-\x14a\x14xW\x80cA\x1DA[\x14a\x14)W\x80cAw\xA8|\x14a\x138W\x80cI\x07]\xA3\x14a\x12\xDAW\x80cY\\jg\x14a\x12(W\x80cZ\xC8j\xB7\x14a\x11\xF5W\x80c\\\x97Z\xBB\x14a\x11\xD8W\x80cqP\x18\xA6\x14a\x11}W\x80csWr;\x14a\x10[W\x80cvs\xE9:\x14a\x10\x1EW\x80cv\x99\x93B\x14a\x0F9W\x80c\x84\xD7o{\x14a\x0E\xDCW\x80c\x88o\x11\x95\x14a\x0E\xB4W\x80c\x8D\xA5\xCB[\x14a\x0E\x8CW\x80c\x95^f\x96\x14a\x0EpW\x80c\x99&\xEE}\x14a\x0CXW\x80c\xA1\x06\x0C\x88\x14a\x0C$W\x80c\xA3d\xF4\xDA\x14a\x0BPW\x80c\xA9\x8F\xB3U\x14a\n\xAEW\x80c\xAE\xC2\x05\xC5\x14a\nLW\x80c\xAF\xE0.\xD5\x14a\t:W\x80c\xB2\x84\x1DH\x14a\t\x1EW\x80c\xB5\xA7h\xCA\x14a\x08\xAAW\x80c\xC1\xA8\xE2\xC5\x14a\x08[W\x80c\xC8%\xFEh\x14a\x08!W\x80c\xCB\xDF\x0EB\x14a\x06\x05W\x80c\xCE{^K\x14a\x06\xF9W\x80c\xD7\x9A\xCE\xAB\x14a\x06\xBFW\x80c\xDA/\xF0]\x14a\x06\x88W\x80c\xDC\xE9t\xB9\x14a\x06NW\x80c\xDF\\\xF7#\x14a\x06\nW\x80c\xE8\x8D\x80I\x14a\x06\x05W\x80c\xECv\xF4B\x14a\x05\xCDW\x80c\xEF-\xFA\x8D\x14a\x03\xCBW\x80c\xF2\xFD\xE3\x8B\x14a\x03<W\x80c\xF6\x98\xDA%\x14a\x03\x1AWc\xFA\xBC\x1C\xBC\x14a\x02)W_\x80\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`eT`@Qcu[6\xBD`\xE1\x1B\x81R`\x04\x805\x92` \x91\x83\x91\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x02\xDCW[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xCDW`fT\x19\x81\x19\x81\x16\x03a\x02\xBEW\x80`fU`@Q\x90\x81R\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C` 3\x92\xA2\0[c\xC6\x1D\xCA]`\xE0\x1B_R`\x04_\xFD[cyH!\xFF`\xE0\x1B_R`\x04_\xFD[a\x02\xFE\x91P` =` \x11a\x03\x04W[a\x02\xF6\x81\x83a\x1D\x15V[\x81\x01\x90a\x1F\x92V[_a\x02oV[P=a\x02\xECV[`@Q=_\x82>=\x90\xFD[_\x80\xFD[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` a\x034a(\x1BV[`@Q\x90\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x03Ua\x1D6V[a\x03]a'\x8AV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a\x03wWa\x03u\x90a$\xA3V[\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x90\xFD[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x03\xFB\x906\x90`\x04\x01a\x1D\xEEV[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x04\x1A\x906\x90`\x04\x01a\x1D\xEEV[\x91`\x02\x80`fT\x16\x14a\x05\xBEW3_R`\x9A` R`\xFF`@_ T\x16\x15a\x05\xAFW_[\x81\x81\x10a\x04GW\0[3_R`\x98` R`@_ a\x04fa\x04a\x83\x85\x89a!\xA9V[a!\x14V[`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x03a\x05\x8CW\x80a\x04\xB2a\x04\x9Fa\x04a`\x01\x94\x86\x8Aa!\xA9V[a\x04\xAA\x83\x88\x88a#tV[\x913\x90a%\x0CV[3_R`\x98` R`@_ a\x04\xCCa\x04a\x83\x86\x8Aa!\xA9V[\x83\x80`\xA0\x1B\x03\x16_R` R`@_ `\xFF\x19\x81T\x16\x90U\x81\x80`\xA0\x1B\x03a\x04\xF8a\x04a\x83\x86\x8Aa!\xA9V[\x16`@Q\x90_\x82R\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A` 3\x93\xA3a\x054a\x04a\x82\x85\x89a!\xA9V[a\x05\x83\x7FT\xF3<\xFD\xD1\xCAp=yY\x86\xB9\x86\xFDG\xD7B\xEA\xB1\x90N\xCD*_\xDB\x8De\x95\xE5\x90J\x01a\x05c\x84\x89\x89a#tV[\x92\x90`@Q\x91\x82\x91` \x83R3\x96\x89\x80`\xA0\x1B\x03\x16\x95` \x84\x01\x91a!\xB9V[\x03\x90\xA3\x01a\x04>V[c\xCC\xEA\x9Eo`\xE0\x1B_R`\x04_\xFD[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[cf\xE5e\xDF`\xE0\x1B_R`\x04_\xFD[c\x84\nH\xD5`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W3_R`\x99` R`@_ `\x045_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U_\x80\xF3[a\x1F[V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x81R\xF3[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16W` a\x06\xB5a\x06\xA6a\x1D6V[a\x06\xAF6a\x1D\xA8V[\x90a#=V[`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x81R\xF3[4a\x03\x16Wa\x07\x076a\x1E\xD0V[`@Q\x91\x92a\x07\x15\x83a\x1C\xE6V[3\x83Rc\xFF\xFF\xFF\xFF\x81\x16` \x84\x01R3_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa\x07R\x82a#\xB8V[\x91_[\x82\x81\x10a\x07^W\0[_\x84\x81R`\x9E` R`@\x90 a\x07\x8C\x90`\x01`\x01`\xA0\x1B\x03a\x07\x85a\x04a\x85\x88\x8Ba!\xA9V[\x16\x90a*vV[\x15a\x08\x03W\x80\x7F{K\x07=\x80\xDC\xACU\xA1\x11w\xD8E\x9A\xD9\xF6d\xCE\xEB\x91\xF7\x1F'\x16{\xB1O\x81R\xA7\xEE\xEEa\x07\xFAa\x07\xC6a\x04a`\x01\x95\x88\x8Ba!\xA9V[`@\x80Q\x87Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x82R` \x80\x8A\x01Qc\xFF\xFF\xFF\xFF\x16\x90\x83\x01R\x90\x92\x16\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[\x03\x90\xA1\x01a\x07UV[c1\xBC4'`\xE1\x1B_R`\x04_\xFD[c\x1F\xB1pU`\xE2\x1B_R`\x04_\xFD[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Q\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x81R\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x08ta\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x08\x93\x906\x90`\x04\x01a\x1D\xEEV[\x90`\x02\x80`fT\x16\x14a\x05\xBEWa\x03u\x923a&[V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x08\xC3a\x1D6V[a\x08\xCBa \xB7V[P`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R`@a\x08\xFAa\x08\xEE`$5\x83_ a(\xD8V[\x90T\x90`\x03\x1B\x1Ca$yV[\x81Q\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x91\x82\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\xF3[4a\x03\x16W` a\x034a\t16a\x1F\x10V[\x93\x92\x90\x92a\"\xE3V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\tj\x906\x90`\x04\x01a\x1D\xEEV[\x90_[\x82\x81\x10a\tvW\0[3_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa\t\x9Aa\t\x95\x84\x87\x87a!\xA9V[a!(V[\x16_R` R`\xFF`@_ T\x16a\x08\x12W`\x01\x903_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa\t\xCEa\t\x95\x84\x88\x88a!\xA9V[\x16_R` R`@_ \x82`\xFF\x19\x82T\x16\x17\x90U\x7F1b\x92\x85\xEA\xD23Z\xE0\x93?\x86\xED*\xE63!\xF7\xAFw\xB4\xE6\xEA\xAB\xC4,\x05x\x80\x97~l`@a\n\x13a\t\x95\x84\x88\x88a!\xA9V[c\xFF\xFF\xFF\xFF\x82Q\x91a\n$\x83a\x1C\xE6V[3\x80\x84R\x91\x16` \x92\x83\x01\x90\x81R\x83Q\x91\x82RQc\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\xA1\x01a\tmV[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_R`\x9A` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3\x7Fp+\x0C\x1Fl\xB1\xCFQ\x1A\xAA\x81\xF7+\xC0Z![\xB3Iv2\xD7,i\x0C\x82+\x04J\xB4\x94\xBF_\x80\xA2\0[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W6`#\x82\x01\x12\x15a\x03\x16W\x80`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16W6`$\x83\x83\x01\x01\x11a\x03\x16W\x7F\xA8\x9C\x1D\xC2C\xD8\x90\x8A\x96\xDD\x84\x94K\xCC\x97\xD6\xBCj\xC0\r\xD7\x8E b\x15v\xBEj<\x947\x13\x90\x82`$`@Q\x92` \x84R\x82` \x85\x01R\x01`@\x83\x017_`@\x84\x83\x01\x01R`@\x813\x94`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01\x90\xA2\0[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x0Bia\x1D6V[`\x01\x80`fT\x16\x14a\x05\xBEW3_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x82\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x03a\x0C\x15W3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_\x81\x81R`\x98` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16\x80\x84R\x94\x82R\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x91\x82R\x91\x92\x91\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A\x91\xA3\0[cR\xDFE\xC9`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16W` a\x034a\x0CBa\x1D6V[a\x0CJa\x1DLV[\x90`d5\x91`D5\x91a\"~V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x0Cqa\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x0C\x90\x906\x90`\x04\x01a\x1E9V[`\x01\x80`fT\x16\x14a\x05\xBEW`@\x81\x01\x80QB\x11a\x0EaW3_R`\x9A` R`\xFF`@_ T\x16a\x05\xAFW3_R`\x98` R`@_ `\x01\x80`\xA0\x1B\x03\x84\x16_R` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x9BW`\x01\x14a\x0ERW`\x01\x80`\xA0\x1B\x03\x83\x16\x92\x83_R`\x99` R`@_ \x92` \x81\x01\x93\x84Q_R` R`\xFF`@_ T\x16a\x0ECW`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R` \x81`$\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x0E\x14W[P\x15a\x0E\x05Wa\r\x8Aa\r\x92\x93\x85Q\x90Q\x903\x85a\"~V[\x90Q\x91a$\xEBV[\x81_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3_R`\x98` R`@_ \x81_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x90`\x01\x82R\x7F\xF0\x95+\x1Ce'\x1D\x81\x9D9\x98=*\xBB\x04K\x9C\xAC\xE5\x9B\xCCMM\xD3\x89\xF5\x86\xEB\xDC\xB1[A` 3\x93\xA3\0[c\x9F\x88\xC8\xAF`\xE0\x1B_R`\x04_\xFD[a\x0E6\x91P` =` \x11a\x0E<W[a\x0E.\x81\x83a\x1D\x15V[\x81\x01\x90a {V[\x86a\rqV[P=a\x0E$V[c\rLL\x91`\xE2\x1B_R`\x04_\xFD[c\x1A\xA5(\xBB`\xE1\x1B_R`\x04_\xFD[c\x08\x19\xBD\xCD`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` a\x034a\x0E\x836a\x1F\x10V[\x93\x92\x90\x92a\"\0V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`3T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`eT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x0E\xF5a\x1D6V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W`\x01\x80`\xA0\x1B\x03\x16_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16Wa\x0FG6a\x1E\xD0V[`@Q\x91\x92a\x0FU\x83a\x1C\xE6V[3\x83Rc\xFF\xFF\xFF\xFF\x81\x16` \x84\x01R3_R`\x9B` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa\x0F\x92\x82a#\xB8V[\x91_[\x82\x81\x10a\x0F\x9EW\0[_\x84\x81R`\x9E` R`@\x90 a\x0F\xCC\x90`\x01`\x01`\xA0\x1B\x03a\x0F\xC5a\x04a\x85\x88\x8Ba!\xA9V[\x16\x90a*\x08V[\x15a\x10\x0FW\x80\x7Fz\xB2`\xFE\n\xF1\x93\xDB_I\x86w\r\x83\x1B\xDAN\xA4`\x99\xDC\x81~\x8Bg\x16\xDC\xAE\x8A\xF8\xE8\x8Ba\x10\x06a\x07\xC6a\x04a`\x01\x95\x88\x8Ba!\xA9V[\x03\x90\xA1\x01a\x0F\x95V[cX\\\xFB/`\xE0\x1B_R`\x04_\xFD[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x01`\x01`\xA0\x1B\x03a\x10?a\x1D6V[\x16_R`\x9A` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16Wa\x10u6a\x1DbV[`D5\x90a\x10\x85`d5\x91a#\xB8V[\x90\x81_R`\x9D` Ra\x10\x9C\x83`@_ Ta \x93V[\x80\x82\x11a\x11uW[Pa\x10\xAE\x81a \xA0V[\x92a\x10\xBC`@Q\x94\x85a\x1D\x15V[\x81\x84Ra\x10\xC8\x82a \xA0V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x83\x81\x10a\x11+W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x11\tWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x10\xFBV[`\x01\x90\x82_R`\x9D` Ra\x11M`@_ a\x11G\x83\x87a \xCFV[\x90a(\xD8V[\x83\x80`\xA0\x1B\x03\x91T\x90`\x03\x1B\x1C\x16a\x11e\x82\x89a \xDCV[\x90\x83\x80`\xA0\x1B\x03\x16\x90R\x01a\x10\xD7V[\x90P\x83a\x10\xA4V[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16Wa\x11\x95a'\x8AV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16\x90\x91U_\x90`\x01`\x01`\xA0\x1B\x03\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x82\x80\xA3\0[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `fT`@Q\x90\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\xFF\x81\x16\x80\x91\x03a\x03\x16W`\x01` \x91\x1B\x80`fT\x16\x14`@Q\x90\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R\x90` \x90\x82\x90`$\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x12\xBBW[P\x15a\x12\xACW_\x19`fU`@Q_\x19\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[c\x1Dw\xD4w`\xE2\x1B_R`\x04_\xFD[a\x12\xD4\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x81a\x12rV[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x12\xF3a\x1D6V[a\x12\xFBa\x1DLV[\x90`\x01\x80`\xA0\x1B\x03\x16_R`\x98` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` R`\xFF`@_ T\x16`@Q`\x02\x82\x10\x15a\x05\x9BW` \x91\x81R\xF3[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x13Za\x13U6a\x1DbV[a#\xB8V[\x80_R`\x9E` R`@_ T\x90a\x13q\x82a \xA0V[\x91a\x13\x7F`@Q\x93\x84a\x1D\x15V[\x80\x83Ra\x13\x8B\x81a \xA0V[` \x84\x01\x92\x90`\x1F\x19\x016\x847_[\x82\x81\x10a\x13\xEEW\x83\x85`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x13\xCCWPPP\x03\x90\xF3[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x13\xBEV[`\x01\x90\x82_R`\x9E` Ra\x14\x06\x81`@_ a(\xD8V[\x90T`\x03\x91\x90\x91\x1B\x1C`\xA0\x83\x90\x1B\x83\x90\x03\x16a\x14\"\x82\x88a \xDCV[R\x01a\x13\x9AV[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x14Fa\x13U6a\x1DbV[_R`\x9D` R` a\x14_`@_ `D5\x90a(\xD8V[\x90T`@Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R\xF3[4a\x03\x16W`\x806`\x03\x19\x01\x12a\x03\x16Wa\x14\x91a\x1D6V[a\x14\x99a\x1DLV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x14\xB9\x906\x90`\x04\x01a\x1D\xEEV[\x91`d5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x14\xD9\x906\x90`\x04\x01a\x1E9V[`\x02\x80`fT\x16\x14a\x05\xBEW\x80QQa\x15\x06WP`\x01`\x01`\xA0\x1B\x03\x81\x163\x03a\x05\x8CWa\x03u\x93a&[V[\x93`@\x85\x01\x80QB\x11a\x0EaW`\x01\x80`\xA0\x1B\x03\x83\x16\x80_R`\x99` R`@_ \x91` \x88\x01\x92\x83Q_R` R`\xFF`@_ T\x16a\x0ECWa\x03u\x97a\x15Ya\x15b\x92\x85Q\x90Q\x90\x8A\x8A\x89a\"\xE3V[\x90Q\x90\x86a$\xEBV[_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90Ua&[V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16W`\x01`\x01`\xA0\x1B\x03a\x15\xA9a\x1D6V[\x16_R`\x99` R`@_ `$5_R` R` `\xFF`@_ T\x16`@Q\x90\x15\x15\x81R\xF3[4a\x03\x16W_6`\x03\x19\x01\x12a\x03\x16W` `@Qc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W6`#\x82\x01\x12\x15a\x03\x16W\x80`\x04\x015\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16W6`$\x83`\x06\x1B\x83\x01\x01\x11a\x03\x16W` \x91`$a\x06\xB5\x92\x01a!9V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x16\x89a\x1D6V[a\x16\x91a\x1DLV[`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x03\x16W`\x01\x80`\xA0\x1B\x03\x16_R`\x9F` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` Rc\xFF\xFF\xFF\xFF`@_ \x91\x16_R` R`@\x80_ Tc\xFF\xFF\xFF\xFF\x82Q\x91`\xFF\x81\x16\x15\x15\x83R`\x08\x1C\x16` \x82\x01R\xF3[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x17\ra\x1D6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x17,\x906\x90`\x04\x01a\x1D\xEEV[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16Wa\x17L\x906\x90`\x04\x01a\x1E9V[\x92`\x02\x80`fT\x16\x14a\x05\xBEW`@\x84\x01\x80QB\x11a\x0EaW`@Qc6\xB8{\xD7`\xE1\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x04\x83\x01\x81\x90R\x91\x90` \x90\x82\x90`$\x90\x82\x90\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x18KW[P\x15a\x0E\x05W3_R`\x9A` R`\xFF`@_ T\x16\x15a\x05\xAFW\x80_R`\x99` R`@_ \x91` \x87\x01\x92\x83Q_R` R`\xFF`@_ T\x16a\x0ECWa\x03u\x96a\x18\x1Aa\x18#\x92\x85Q\x90Q\x90\x89\x893a\"\0V[\x90Q\x90\x85a$\xEBV[_R`\x99` R`@_ \x90Q_R` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U3\x90a%\x0CV[a\x18d\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x87a\x17\xC2V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x18\x83a\x1D6V[a\x18\x8Ba\x1DLV[`D5\x91_T\x92`\xFF\x84`\x08\x1C\x16\x15\x93\x84\x80\x95a\x19\xFBW[\x80\x15a\x19\xE4W[\x15a\x19\x88W`\xFF\x19\x81\x16`\x01\x17_U\x84a\x19wW[P`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80a\x19eW[\x15a\x19VWa\x19\x1A\x92\x81a\x19\x15\x92`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2a$\x1CV[a$\xA3V[a\x19 W\0[a\xFF\0\x19_T\x16_U\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98` `@Q`\x01\x81R\xA1\0[c9\xB1\x90\xBB`\xE1\x1B_R`\x04_\xFD[P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15\x15a\x18\xD3V[a\xFF\xFF\x19\x16a\x01\x01\x17_U\x84a\x18\xBFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x90\xFD[P0;\x15\x80\x15a\x18\xAAWP`\x01`\xFF\x82\x16\x14a\x18\xAAV[P`\x01`\xFF\x82\x16\x10a\x18\xA3V[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16Wa\x1A!a\x1D6V[`\x01`\x01`\xA0\x1B\x03\x16_\x81\x81R`\x9C` R`@\x90 T`D5\x90`$5\x90a\x1AK\x90\x82\x90a \x93V[\x80\x83\x11a\x1BMW[Pa\x1A]\x82a \xA0V[\x92a\x1Ak`@Q\x94\x85a\x1D\x15V[\x82\x84R`\x1F\x19a\x1Az\x84a \xA0V[\x01_[\x81\x81\x10a\x1B*WPP_[\x83\x81\x10a\x1A\xEFW\x84`@Q\x80\x91` \x82\x01` \x83R\x81Q\x80\x91R` `@\x84\x01\x92\x01\x90_[\x81\x81\x10a\x1A\xBBWPPP\x03\x90\xF3[\x82Q\x80Q`\x01`\x01`\xA0\x1B\x03\x16\x85R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x86\x01R\x86\x95P`@\x90\x94\x01\x93\x90\x92\x01\x91`\x01\x01a\x1A\xADV[`\x01\x90\x82_R`\x9C` Ra\x1B\x0Ea\x08\xEE`@_ a\x11G\x84\x88a \xCFV[a\x1B\x18\x82\x88a \xDCV[Ra\x1B#\x81\x87a \xDCV[P\x01a\x1A\x88V[` \x90`@Qa\x1B9\x81a\x1C\xE6V[_\x81R_\x83\x82\x01R\x82\x82\x89\x01\x01R\x01a\x1A}V[\x91P\x83a\x1ASV[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16W`\x045`$` `\x01\x80`\xA0\x1B\x03`eT\x16`@Q\x92\x83\x80\x92c#}\xFBG`\xE1\x1B\x82R3`\x04\x83\x01RZ\xFA\x90\x81\x15a\x03\x0BW_\x91a\x1B\xE2W[P\x15a\x12\xACW`fT\x81\x81\x16\x03a\x02\xBEW\x80`fU`@Q\x90\x81R\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=` 3\x92\xA2\0[a\x1B\xFB\x91P` =` \x11a\x0E<Wa\x0E.\x81\x83a\x1D\x15V[\x82a\x1B\x9FV[4a\x03\x16W``6`\x03\x19\x01\x12a\x03\x16W` a\x06\xB5a\x1C\x1Fa\x1D6V[a\x1C(6a\x1D\xA8V[\x90a\x1F\xB1V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x1CGa\x1D6V[`eT`@Qcu[6\xBD`\xE1\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x03\x0BW_\x91a\x1C\x94W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a\x02\xCDWa\x03u\x90a$\x1CV[a\x1C\xAD\x91P` =` \x11a\x03\x04Wa\x02\xF6\x81\x83a\x1D\x15V[\x82a\x1C{V[4a\x03\x16W`@6`\x03\x19\x01\x12a\x03\x16Wa\x1C\xD0a\x13U6a\x1DbV[_R`\x9D` R` `@_ T`@Q\x90\x81R\xF3[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@RV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x16WV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x03\x16WV[`@\x90`\x03\x19\x01\x12a\x03\x16W`@Q\x90a\x1D{\x82a\x1C\xE6V[\x81`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x81R`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x01RV[`@\x90`#\x19\x01\x12a\x03\x16W`@Q\x90a\x1D\xC1\x82a\x1C\xE6V[\x81`$5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x81R`D5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x01RV[\x91\x81`\x1F\x84\x01\x12\x15a\x03\x16W\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x16W` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x03\x16WV[`\x01`\x01`@\x1B\x03\x81\x11a\x1D\x01W`\x1F\x01`\x1F\x19\x16` \x01\x90V[\x91\x90``\x83\x82\x03\x12a\x03\x16W`@Q\x90``\x82\x01\x82\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x1D\x01W`@R\x81\x93\x805`\x01`\x01`@\x1B\x03\x81\x11a\x03\x16W\x81\x01\x90\x82`\x1F\x83\x01\x12\x15a\x03\x16W\x815\x92a\x1E\x8E\x84a\x1E\x1EV[\x90a\x1E\x9C`@Q\x92\x83a\x1D\x15V[\x84\x82R` \x85\x85\x01\x01\x11a\x03\x16W_` \x85`@\x96\x82\x88\x97\x01\x83\x86\x017\x83\x01\x01R\x84R` \x81\x015` \x85\x01R\x015\x91\x01RV[\x90`@`\x03\x19\x83\x01\x12a\x03\x16W`\x045c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x16W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16Wa\x1F\x0C\x91`\x04\x01a\x1D\xEEV[\x90\x91V[\x90`\x80`\x03\x19\x83\x01\x12a\x03\x16W`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x91`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x03\x16Wa\x1FO\x91`\x04\x01a\x1D\xEEV[\x90\x91`D5\x90`d5\x90V[4a\x03\x16W` 6`\x03\x19\x01\x12a\x03\x16Wa\x1Fta\x1D6V[`\x01\x80`\xA0\x1B\x03\x16_R`\x9C` R` `@_ T`@Q\x90\x81R\xF3[\x90\x81` \x91\x03\x12a\x03\x16WQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x90V[a\x1F\xBB\x82\x82a#=V[a tW`\x01\x80`\xA0\x1B\x03\x82Q\x16_R`\x9F` R`@_ \x90`\x01\x80`\xA0\x1B\x03\x16_R` Rc\xFF\xFF\xFF\xFF\x80` `@_ \x93\x01Q\x16\x16_R` R` `@_ c\xFF\xFF\xFF\xFF`@Q\x91a \x10\x83a\x1C\xE6V[T`\xFF\x81\x16\x15\x15\x83R`\x08\x1C\x16\x91\x82\x91\x01Rc\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x01c\xFF\xFF\xFF\xFF\x81\x11a `Wc\xFF\xFF\xFF\xFF\x16B\x10\x90V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[PP`\x01\x90V[\x90\x81` \x91\x03\x12a\x03\x16WQ\x80\x15\x15\x81\x03a\x03\x16W\x90V[\x91\x90\x82\x03\x91\x82\x11a `WV[`\x01`\x01`@\x1B\x03\x81\x11a\x1D\x01W`\x05\x1B` \x01\x90V[`@Q\x90a \xC4\x82a\x1C\xE6V[_` \x83\x82\x81R\x01RV[\x91\x90\x82\x01\x80\x92\x11a `WV[\x80Q\x82\x10\x15a \xF0W` \x91`\x05\x1B\x01\x01\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[\x91\x90\x81\x10\x15a \xF0W`\x06\x1B\x01\x90V[5`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x03\x16W\x90V[5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x03\x16W\x90V[_[\x82\x81\x10a!JWPPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03a!`a\x04a\x83\x86\x86a!\x04V[\x16_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa!\x87` a!\x81\x85\x88\x88a!\x04V[\x01a!(V[\x16_R` R`\xFF`@_ T\x16\x15a!\xA2W`\x01\x01a!;V[PPP_\x90V[\x91\x90\x81\x10\x15a \xF0W`\x05\x1B\x01\x90V[\x91` \x90\x82\x81R\x01\x91\x90_\x90[\x80\x82\x10a!\xD3WPPP\x90V[\x90\x91\x92\x835\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x03\x16W` \x80\x91c\xFF\xFF\xFF\xFF`\x01\x94\x16\x81R\x01\x94\x01\x92\x01\x90a!\xC6V[\x92\x91a\"s\x91\x92a\"{\x95a\"Z`@Q\x95\x86\x94` \x86\x01\x98\x7F\x80\x9CZ\xC0I\xC4[z\x7F\x05\n \xF0\x0C\x16\xCFcy~\xFB\xF8\xB1\xEB\x8Dt\x9F\xDF\xA3\x9F\xF8\xF9)\x8AR`\x01\x80`\xA0\x1B\x03\x16`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a!\xB9V[\x91`\x80\x84\x01R`\xA0\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x1D\x15V[Q\x90 a'\xE2V[\x90V[\x91\x92a\"{\x93`@Q\x92` \x84\x01\x94\x7F\xDA,\x89\xBA\xFD\xD3Gv\xA2\xB8\xBB\x9C\x83\xC8/A\x9E \xCC\x8Cg \x7Fp\xED\xD5\x82I\xB9&a\xBD\x86R`\x01\x80`\xA0\x1B\x03\x16`@\x85\x01R`\x01\x80`\xA0\x1B\x03\x16``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xA0\x81Ra\"s`\xC0\x82a\x1D\x15V[\x92\x91a\"s\x91\x92a\"{\x95a\"Z`@Q\x95\x86\x94` \x86\x01\x98\x7FN\xE6_d!\x8Cg\xB6\x8D\xA6o\xD0\xDB\x16V\0@\xA6\xB9s)\x0B\x9Eq\x91-f\x1E\xE5?\xE4\x95\x8AR`\x01\x80`\xA0\x1B\x03\x16`@\x87\x01R`\xA0``\x87\x01R`\xC0\x86\x01\x91a!\xB9V[`\x01`\x01`\xA0\x1B\x03\x16_\x90\x81R`\x9C` R`@\x90 a\"{\x91a#`\x90a#\xB8V[\x90`\x01\x91_R\x01` R`@_ T\x15\x15\x90V[\x91\x90\x81\x10\x15a \xF0W`\x05\x1B\x81\x015\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x03\x16W\x01\x90\x815\x91`\x01`\x01`@\x1B\x03\x83\x11a\x03\x16W` \x01\x82`\x05\x1B6\x03\x81\x13a\x03\x16W\x91\x90V[` \x81Q\x91\x01Q`@Q\x90` \x82\x01\x92k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90``\x1B\x16\x83Rc\xFF\xFF\xFF\xFF`\xA0\x1B\x90`\xA0\x1B\x16`4\x82\x01R` \x81Ra#\xFE`@\x82a\x1D\x15V[Q\x90Q\x90` \x81\x10a$\x0EWP\x90V[_\x19\x90` \x03`\x03\x1B\x1B\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x80\x15a\x19VW`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R` \x81\x01\x84\x90R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x90\xA1`\x01`\x01`\xA0\x1B\x03\x19\x16\x17`eUV[a$\x81a \xB7V[Pc\xFF\xFF\xFF\xFF`@Q\x91a$\x94\x83a\x1C\xE6V[\x80``\x1C\x83R\x16` \x82\x01R\x90V[`3\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x82\x16\x81\x17\x90\x92U\x90\x91\x16\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0_\x80\xA3V[\x90a$\xF6\x92\x91a(\xEDV[\x15a$\xFDWV[c\x8B\xAAW\x9F`\xE0\x1B_R`\x04_\xFD[\x90\x92\x91_[\x83\x81\x10a%\x1FWPPPPPV[a%-a\t\x95\x82\x86\x85a!\xA9V[\x90`@Q\x91a%;\x83a\x1C\xE6V[c\xFF\xFF\xFF\xFF`\x01\x80`\xA0\x1B\x03\x88\x16\x91\x82\x85R\x16` \x84\x01R\x80_R`\x9B` R`@_ c\xFF\xFF\xFF\xFFa%ra\t\x95\x85\x8A\x89a!\xA9V[\x16_R` R`\xFF`@_ T\x16\x15a\x08\x12Wa%\x8E\x83a#\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x86\x16\x91\x82_R`\x9C` Ra%\xAE\x81`@_ a*\x08V[P_R`\x9D` Ra%\xC3\x82`@_ a*\x08V[P_R`\x9F` R`@_ \x81_R` R`@_ c\xFF\xFF\xFF\xFFa%\xECa\t\x95\x85\x8A\x89a!\xA9V[\x16_R` R`@_ \x80T`\xFF\x81\x16a\x05\x8CW`\x01\x94\x7FC#.\xDF\x90qu=#!\xE5\xFA~\x01\x83c\xEE$\x8E_!B\xE6\xC0\x8E\xDD2e\xBF\xB4\x89^\x92\x86`@\x93`\xFF\x19\x16\x17\x90Ua&T\x82Q\x80\x92c\xFF\xFF\xFF\xFF` \x80\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x85R\x01Q\x16\x91\x01RV[\xA2\x01a%\x11V[\x92\x91B`\x08\x1Bd\xFF\xFF\xFF\xFF\0\x16\x90_[\x84\x81\x10a&zWPPPPPPV[a&\x88a\t\x95\x82\x87\x85a!\xA9V[\x90`@Q\x91a&\x96\x83a\x1C\xE6V[`\x01`\x01`\xA0\x1B\x03\x88\x16\x80\x84Rc\xFF\xFF\xFF\xFF\x91\x90\x91\x16` \x84\x01Ra&\xBA\x83a#\xB8V[\x90`\x01\x80`\xA0\x1B\x03\x87\x16\x91\x82_R`\x9C` Ra&\xDA\x81`@_ a*vV[P_R`\x9D` Ra&\xEF\x82`@_ a*vV[P_R`\x9F` R`@_ \x81_R` R`@_ c\xFF\xFF\xFF\xFFa'\x18a\t\x95\x85\x8B\x89a!\xA9V[\x16_R` R`@_ \x80T`\xFF\x81\x16\x15a\x05\x8CWd\xFF\xFF\xFF\xFF\xFF\x19\x16\x86\x17\x90U`@\x80Q\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x94\x85\x01Qc\xFF\xFF\xFF\xFF\x16\x94\x81\x01\x94\x90\x94R`\x01\x93\x7F\xAD4\xC3\x07\x0B\xE1\xDF\xFB\xCA\xA4\x99\xD0\0\xBA+\x8D\x98H\xAE\xFC\xAC0Y\xDF$]\xD9\\N\xCE\x14\xFE\x91\x90\xA2\x01a&kV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x03a'\x9EWV[`d`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R` `$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R\xFD[a'\xEAa(\x1BV[\x90`@Q\x90` \x82\x01\x92a\x19\x01`\xF0\x1B\x84R`\"\x83\x01R`B\x82\x01R`B\x81Ra(\x15`b\x82a\x1D\x15V[Q\x90 \x90V[F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a(fW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`\n` `@Qa(x`@\x82a\x1D\x15V[\x82\x81R\x01i\"\xB4\xB3\xB2\xB7&0\xBC\xB2\xB9`\xB1\x1B\x81R `@Q` \x81\x01\x91\x7F\x8C\xAD\x95h{\xA8,,\xE5\x0Et\xF7\xB7Td^Q\x17\xC3\xA5\xBE\xC8\x15\x1C\x07&\xD5\x85y\x80\xA8f\x83R`@\x82\x01RF``\x82\x01R0`\x80\x82\x01R`\x80\x81Ra(\x15`\xA0\x82a\x1D\x15V[\x80T\x82\x10\x15a \xF0W_R` _ \x01\x90_\x90V[\x91\x90\x91a(\xFA\x82\x84a+;V[`\x05\x81\x10\x15a\x05\x9BW\x15\x90\x81a)\xF2W[Pa)\xEAW_\x92` a)j`\x84\x86\x95`@Q\x93\x84\x91\x81\x83\x01\x96c\x0B\x13]?`\xE1\x1B\x88R`$\x84\x01R`@`D\x84\x01R\x80Q\x91\x82\x91\x82`d\x86\x01R\x01\x84\x84\x01^\x87\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x81\x01\x03\x01`\x1F\x19\x81\x01\x83R\x82a\x1D\x15V[Q\x91Z\xFA=\x15a)\xE3W=a)~\x81a\x1E\x1EV[\x90a)\x8C`@Q\x92\x83a\x1D\x15V[\x81R=_` \x83\x01>[\x81a)\xD7W[\x81a)\xA5WP\x90V[\x90P` \x81\x80Q\x81\x01\x03\x12a\x03\x16W` \x01Q`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x90\x81\x90\x03a\x03\x16Wc\x0B\x13]?`\xE1\x1B\x14\x90V[\x80Q` \x14\x91Pa)\x9CV[``a)\x96V[PPP`\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x83\x81\x16\x91\x16\x14\x90P_a)\x0BV[_\x82\x81R`\x01\x82\x01` R`@\x90 Ta*pW\x80T\x90h\x01\0\0\0\0\0\0\0\0\x82\x10\x15a\x1D\x01W\x82a*[a*E\x84`\x01\x80\x96\x01\x85U\x84a(\xD8V[\x81\x93\x91T\x90`\x03\x1B\x91\x82\x1B\x91_\x19\x90\x1B\x19\x16\x17\x90V[\x90U\x80T\x92_R\x01` R`@_ U`\x01\x90V[PP_\x90V[\x90`\x01\x82\x01\x91\x81_R\x82` R`@_ T\x80\x15\x15_\x14a+3W_\x19\x81\x01\x81\x81\x11a `W\x82T_\x19\x81\x01\x91\x90\x82\x11a `W\x80\x82\x03a*\xFEW[PPP\x80T\x80\x15a*\xEAW_\x19\x01\x90a*\xCB\x82\x82a(\xD8V[\x81T\x90_\x19\x90`\x03\x1B\x1B\x19\x16\x90UU_R` R_`@\x81 U`\x01\x90V[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[a+\x1Ea+\x0Ea*E\x93\x86a(\xD8V[\x90T\x90`\x03\x1B\x1C\x92\x83\x92\x86a(\xD8V[\x90U_R\x83` R`@_ U_\x80\x80a*\xB2V[PPPP_\x90V[\x81Q`A\x81\x03a+cWP\x90a\x1F\x0C\x91` \x82\x01Q\x90```@\x84\x01Q\x93\x01Q_\x1A\x90a+\xA5V[`@\x03a+\x9CW`@` \x83\x01Q\x92\x01Q\x91\x82`\xFF\x1C\x91`\x1B\x83\x01\x80\x93\x11a `Wa\x1F\x0C\x93`\x01`\x01`\xFF\x1B\x03\x16\x92`\xFF\x16\x90a+\xA5V[PP_\x90`\x02\x90V[\x90\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x84\x11a,<W`\xFF\x16\x90`\x1B\x82\x14\x15\x80a,1W[a,&W` \x93_\x93`\x80\x93`@Q\x93\x84R\x86\x84\x01R`@\x83\x01R``\x82\x01R\x82\x80R`\x01Z\xFA\x15a\x03\x0BW_Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x15a,\x1EW\x90_\x90V[P_\x90`\x01\x90V[PPPP_\x90`\x04\x90V[P`\x1C\x82\x14\x15a+\xDCV[PPPP_\x90`\x03\x90V\xFE\xA2dipfsX\"\x12 \xF1\x12\x16U\xCD1\xC9M\xF6\xB8.P\xAD\x93\xBBd\xEFZ\x1C\x830\xC8\x80\xC2\x90\xA9\x04\r\xFD\")rdsolcC\0\x08\x1B\x003",
    );
    /**```solidity
struct OperatorSet { address avs; uint32 operatorSetId; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorSet {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetId: u32,
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
                (value.avs, value.operatorSetId)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for OperatorSet {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    avs: tuple.0,
                    operatorSetId: tuple.1,
                }
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetId),
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
                    "OperatorSet(address avs,uint32 operatorSetId)",
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
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.operatorSetId)
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
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.operatorSetId,
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
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.operatorSetId,
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
    /**Custom error with signature `InvalidAVS()` and selector `0x66e565df`.
```solidity
error InvalidAVS();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidAVS {}
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
        impl ::core::convert::From<InvalidAVS> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidAVS) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidAVS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidAVS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidAVS()";
            const SELECTOR: [u8; 4] = [102u8, 229u8, 101u8, 223u8];
            #[inline]
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
    /**Custom error with signature `InvalidSignature()` and selector `0x8baa579f`.
```solidity
error InvalidSignature();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct InvalidSignature {}
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
        impl ::core::convert::From<InvalidSignature> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidSignature) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidSignature {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidSignature {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidSignature()";
            const SELECTOR: [u8; 4] = [139u8, 170u8, 87u8, 159u8];
            #[inline]
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
    /**Custom error with signature `OperatorAlreadyRegisteredToAVS()` and selector `0x354a5176`.
```solidity
error OperatorAlreadyRegisteredToAVS();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorAlreadyRegisteredToAVS {}
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
        impl ::core::convert::From<OperatorAlreadyRegisteredToAVS>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorAlreadyRegisteredToAVS) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorAlreadyRegisteredToAVS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorAlreadyRegisteredToAVS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorAlreadyRegisteredToAVS()";
            const SELECTOR: [u8; 4] = [53u8, 74u8, 81u8, 118u8];
            #[inline]
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
    /**Custom error with signature `OperatorNotRegisteredToAVS()` and selector `0x52df45c9`.
```solidity
error OperatorNotRegisteredToAVS();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegisteredToAVS {}
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
        impl ::core::convert::From<OperatorNotRegisteredToAVS>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegisteredToAVS) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorNotRegisteredToAVS {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegisteredToAVS {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegisteredToAVS()";
            const SELECTOR: [u8; 4] = [82u8, 223u8, 69u8, 201u8];
            #[inline]
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
    /**Custom error with signature `OperatorNotRegisteredToEigenLayer()` and selector `0x9f88c8af`.
```solidity
error OperatorNotRegisteredToEigenLayer();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OperatorNotRegisteredToEigenLayer {}
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
        impl ::core::convert::From<OperatorNotRegisteredToEigenLayer>
        for UnderlyingRustTuple<'_> {
            fn from(value: OperatorNotRegisteredToEigenLayer) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for OperatorNotRegisteredToEigenLayer {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for OperatorNotRegisteredToEigenLayer {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OperatorNotRegisteredToEigenLayer()";
            const SELECTOR: [u8; 4] = [159u8, 136u8, 200u8, 175u8];
            #[inline]
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
    /**Custom error with signature `SaltSpent()` and selector `0x35313244`.
```solidity
error SaltSpent();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SaltSpent {}
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
        impl ::core::convert::From<SaltSpent> for UnderlyingRustTuple<'_> {
            fn from(value: SaltSpent) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SaltSpent {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SaltSpent {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SaltSpent()";
            const SELECTOR: [u8; 4] = [53u8, 49u8, 50u8, 68u8];
            #[inline]
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
    /**Custom error with signature `SignatureExpired()` and selector `0x0819bdcd`.
```solidity
error SignatureExpired();
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct SignatureExpired {}
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
        impl ::core::convert::From<SignatureExpired> for UnderlyingRustTuple<'_> {
            fn from(value: SignatureExpired) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SignatureExpired {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SignatureExpired {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SignatureExpired()";
            const SELECTOR: [u8; 4] = [8u8, 25u8, 189u8, 205u8];
            #[inline]
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
    /**Event with signature `AVSMigratedToOperatorSets(address)` and selector `0x702b0c1f6cb1cf511aaa81f72bc05a215bb3497632d72c690c822b044ab494bf`.
```solidity
event AVSMigratedToOperatorSets(address indexed avs);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct AVSMigratedToOperatorSets {
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for AVSMigratedToOperatorSets {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "AVSMigratedToOperatorSets(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                112u8,
                43u8,
                12u8,
                31u8,
                108u8,
                177u8,
                207u8,
                81u8,
                26u8,
                170u8,
                129u8,
                247u8,
                43u8,
                192u8,
                90u8,
                33u8,
                91u8,
                179u8,
                73u8,
                118u8,
                50u8,
                215u8,
                44u8,
                105u8,
                12u8,
                130u8,
                43u8,
                4u8,
                74u8,
                180u8,
                148u8,
                191u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { avs: topics.1 }
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
        impl alloy_sol_types::private::IntoLogData for AVSMigratedToOperatorSets {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&AVSMigratedToOperatorSets> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &AVSMigratedToOperatorSets,
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
    /**Event with signature `OperatorAVSRegistrationStatusUpdated(address,address,uint8)` and selector `0xf0952b1c65271d819d39983d2abb044b9cace59bcc4d4dd389f586ebdcb15b41`.
```solidity
event OperatorAVSRegistrationStatusUpdated(address indexed operator, address indexed avs, IAVSDirectoryTypes.OperatorAVSRegistrationStatus status);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorAVSRegistrationStatusUpdated {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub status: <IAVSDirectoryTypes::OperatorAVSRegistrationStatus as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for OperatorAVSRegistrationStatusUpdated {
            type DataTuple<'a> = (IAVSDirectoryTypes::OperatorAVSRegistrationStatus,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorAVSRegistrationStatusUpdated(address,address,uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                240u8,
                149u8,
                43u8,
                28u8,
                101u8,
                39u8,
                29u8,
                129u8,
                157u8,
                57u8,
                152u8,
                61u8,
                42u8,
                187u8,
                4u8,
                75u8,
                156u8,
                172u8,
                229u8,
                155u8,
                204u8,
                77u8,
                77u8,
                211u8,
                137u8,
                245u8,
                134u8,
                235u8,
                220u8,
                177u8,
                91u8,
                65u8,
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
                    avs: topics.2,
                    status: data.0,
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
                    <IAVSDirectoryTypes::OperatorAVSRegistrationStatus as alloy_sol_types::SolType>::tokenize(
                        &self.status,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone(), self.avs.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData
        for OperatorAVSRegistrationStatusUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorAVSRegistrationStatusUpdated>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorAVSRegistrationStatusUpdated,
            ) -> alloy_sol_types::private::LogData {
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
    /**Event with signature `OperatorMigratedToOperatorSets(address,address,uint32[])` and selector `0x54f33cfdd1ca703d795986b986fd47d742eab1904ecd2a5fdb8d6595e5904a01`.
```solidity
event OperatorMigratedToOperatorSets(address indexed operator, address indexed avs, uint32[] operatorSetIds);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorMigratedToOperatorSets {
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub avs: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorMigratedToOperatorSets {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorMigratedToOperatorSets(address,address,uint32[])";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                84u8,
                243u8,
                60u8,
                253u8,
                209u8,
                202u8,
                112u8,
                61u8,
                121u8,
                89u8,
                134u8,
                185u8,
                134u8,
                253u8,
                71u8,
                215u8,
                66u8,
                234u8,
                177u8,
                144u8,
                78u8,
                205u8,
                42u8,
                95u8,
                219u8,
                141u8,
                101u8,
                149u8,
                229u8,
                144u8,
                74u8,
                1u8,
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
                    avs: topics.2,
                    operatorSetIds: data.0,
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operator.clone(), self.avs.clone())
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
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.avs,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorMigratedToOperatorSets {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorMigratedToOperatorSets>
        for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &OperatorMigratedToOperatorSets,
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
    /**Event with signature `PauserRegistrySet(address,address)` and selector `0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6`.
```solidity
event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct PauserRegistrySet {
        #[allow(missing_docs)]
        pub pauserRegistry: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newPauserRegistry: alloy::sol_types::private::Address,
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
        impl alloy_sol_types::SolEvent for PauserRegistrySet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "PauserRegistrySet(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    pauserRegistry: data.0,
                    newPauserRegistry: data.1,
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
                        &self.pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newPauserRegistry,
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
        impl alloy_sol_types::private::IntoLogData for PauserRegistrySet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&PauserRegistrySet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &PauserRegistrySet) -> alloy_sol_types::private::LogData {
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
constructor(address _delegation, uint32 _DEALLOCATION_DELAY);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _delegation: alloy::sol_types::private::Address,
        pub _DEALLOCATION_DELAY: u32,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._delegation, value._DEALLOCATION_DELAY)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _delegation: tuple.0,
                        _DEALLOCATION_DELAY: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._DEALLOCATION_DELAY),
                )
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
    /**Function with signature `OPERATOR_AVS_REGISTRATION_TYPEHASH()` and selector `0xd79aceab`.
```solidity
function OPERATOR_AVS_REGISTRATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_AVS_REGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_AVS_REGISTRATION_TYPEHASH()`](OPERATOR_AVS_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_AVS_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<OPERATOR_AVS_REGISTRATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_AVS_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_AVS_REGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<OPERATOR_AVS_REGISTRATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_AVS_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_AVS_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_AVS_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_AVS_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_AVS_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [215u8, 154u8, 206u8, 171u8];
            #[inline]
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
    /**Function with signature `OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()` and selector `0xdce974b9`.
```solidity
function OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()`](OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [220u8, 233u8, 116u8, 185u8];
            #[inline]
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
    /**Function with signature `OPERATOR_SET_REGISTRATION_TYPEHASH()` and selector `0xc825fe68`.
```solidity
function OPERATOR_SET_REGISTRATION_TYPEHASH() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_SET_REGISTRATION_TYPEHASHCall {}
    ///Container type for the return parameters of the [`OPERATOR_SET_REGISTRATION_TYPEHASH()`](OPERATOR_SET_REGISTRATION_TYPEHASHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct OPERATOR_SET_REGISTRATION_TYPEHASHReturn {
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
            impl ::core::convert::From<OPERATOR_SET_REGISTRATION_TYPEHASHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_SET_REGISTRATION_TYPEHASHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_SET_REGISTRATION_TYPEHASHCall {
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
            impl ::core::convert::From<OPERATOR_SET_REGISTRATION_TYPEHASHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: OPERATOR_SET_REGISTRATION_TYPEHASHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for OPERATOR_SET_REGISTRATION_TYPEHASHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for OPERATOR_SET_REGISTRATION_TYPEHASHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = OPERATOR_SET_REGISTRATION_TYPEHASHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "OPERATOR_SET_REGISTRATION_TYPEHASH()";
            const SELECTOR: [u8; 4] = [200u8, 37u8, 254u8, 104u8];
            #[inline]
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
    /**Function with signature `addStrategiesToOperatorSet(uint32,address[])` and selector `0x76999342`.
```solidity
function addStrategiesToOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesToOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`addStrategiesToOperatorSet(uint32,address[])`](addStrategiesToOperatorSetCall) function.
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
            impl ::core::convert::From<addStrategiesToOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesToOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for addStrategiesToOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            const SIGNATURE: &'static str = "addStrategiesToOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [118u8, 153u8, 147u8, 66u8];
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
    /**Function with signature `avsOperatorStatus(address,address)` and selector `0x49075da3`.
```solidity
function avsOperatorStatus(address, address) external view returns (IAVSDirectoryTypes.OperatorAVSRegistrationStatus);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsOperatorStatusCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`avsOperatorStatus(address,address)`](avsOperatorStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsOperatorStatusReturn {
        pub _0: <IAVSDirectoryTypes::OperatorAVSRegistrationStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<avsOperatorStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: avsOperatorStatusCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for avsOperatorStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IAVSDirectoryTypes::OperatorAVSRegistrationStatus,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IAVSDirectoryTypes::OperatorAVSRegistrationStatus as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<avsOperatorStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: avsOperatorStatusReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for avsOperatorStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsOperatorStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsOperatorStatusReturn;
            type ReturnTuple<'a> = (IAVSDirectoryTypes::OperatorAVSRegistrationStatus,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsOperatorStatus(address,address)";
            const SELECTOR: [u8; 4] = [73u8, 7u8, 93u8, 163u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
    /**Function with signature `becomeOperatorSetAVS()` and selector `0xaec205c5`.
```solidity
function becomeOperatorSetAVS() external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct becomeOperatorSetAVSCall {}
    ///Container type for the return parameters of the [`becomeOperatorSetAVS()`](becomeOperatorSetAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct becomeOperatorSetAVSReturn {}
    #[allow(
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
            impl ::core::convert::From<becomeOperatorSetAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: becomeOperatorSetAVSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for becomeOperatorSetAVSCall {
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
            impl ::core::convert::From<becomeOperatorSetAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: becomeOperatorSetAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for becomeOperatorSetAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for becomeOperatorSetAVSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = becomeOperatorSetAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "becomeOperatorSetAVS()";
            const SELECTOR: [u8; 4] = [174u8, 194u8, 5u8, 197u8];
            #[inline]
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
    /**Function with signature `calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)` and selector `0xa1060c88`.
```solidity
function calculateOperatorAVSRegistrationDigestHash(address operator, address avs, bytes32 salt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorAVSRegistrationDigestHashCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)`](calculateOperatorAVSRegistrationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorAVSRegistrationDigestHashReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<calculateOperatorAVSRegistrationDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorAVSRegistrationDigestHashCall) -> Self {
                    (value.operator, value.avs, value.salt, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorAVSRegistrationDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        salt: tuple.2,
                        expiry: tuple.3,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorAVSRegistrationDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: calculateOperatorAVSRegistrationDigestHashReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorAVSRegistrationDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for calculateOperatorAVSRegistrationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorAVSRegistrationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorAVSRegistrationDigestHash(address,address,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [161u8, 6u8, 12u8, 136u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Function with signature `calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)` and selector `0xb2841d48`.
```solidity
function calculateOperatorSetForceDeregistrationTypehash(address avs, uint32[] memory operatorSetIds, bytes32 salt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorSetForceDeregistrationTypehashCall {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)`](calculateOperatorSetForceDeregistrationTypehashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorSetForceDeregistrationTypehashReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<
                calculateOperatorSetForceDeregistrationTypehashCall,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: calculateOperatorSetForceDeregistrationTypehashCall,
                ) -> Self {
                    (value.avs, value.operatorSetIds, value.salt, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorSetForceDeregistrationTypehashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetIds: tuple.1,
                        salt: tuple.2,
                        expiry: tuple.3,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<
                calculateOperatorSetForceDeregistrationTypehashReturn,
            > for UnderlyingRustTuple<'_> {
                fn from(
                    value: calculateOperatorSetForceDeregistrationTypehashReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorSetForceDeregistrationTypehashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for calculateOperatorSetForceDeregistrationTypehashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorSetForceDeregistrationTypehashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorSetForceDeregistrationTypehash(address,uint32[],bytes32,uint256)";
            const SELECTOR: [u8; 4] = [178u8, 132u8, 29u8, 72u8];
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Function with signature `calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)` and selector `0x955e6696`.
```solidity
function calculateOperatorSetRegistrationDigestHash(address avs, uint32[] memory operatorSetIds, bytes32 salt, uint256 expiry) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorSetRegistrationDigestHashCall {
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
        pub expiry: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)`](calculateOperatorSetRegistrationDigestHashCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateOperatorSetRegistrationDigestHashReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<calculateOperatorSetRegistrationDigestHashCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateOperatorSetRegistrationDigestHashCall) -> Self {
                    (value.avs, value.operatorSetIds, value.salt, value.expiry)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorSetRegistrationDigestHashCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        avs: tuple.0,
                        operatorSetIds: tuple.1,
                        salt: tuple.2,
                        expiry: tuple.3,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<calculateOperatorSetRegistrationDigestHashReturn>
            for UnderlyingRustTuple<'_> {
                fn from(
                    value: calculateOperatorSetRegistrationDigestHashReturn,
                ) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateOperatorSetRegistrationDigestHashReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall
        for calculateOperatorSetRegistrationDigestHashCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateOperatorSetRegistrationDigestHashReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateOperatorSetRegistrationDigestHash(address,uint32[],bytes32,uint256)";
            const SELECTOR: [u8; 4] = [149u8, 94u8, 102u8, 150u8];
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.expiry),
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
    /**Function with signature `cancelSalt(bytes32)` and selector `0xec76f442`.
```solidity
function cancelSalt(bytes32 salt) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSaltCall {
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`cancelSalt(bytes32)`](cancelSaltCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct cancelSaltReturn {}
    #[allow(
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
            impl ::core::convert::From<cancelSaltCall> for UnderlyingRustTuple<'_> {
                fn from(value: cancelSaltCall) -> Self {
                    (value.salt,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelSaltCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { salt: tuple.0 }
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
            impl ::core::convert::From<cancelSaltReturn> for UnderlyingRustTuple<'_> {
                fn from(value: cancelSaltReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for cancelSaltReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for cancelSaltCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = cancelSaltReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "cancelSalt(bytes32)";
            const SELECTOR: [u8; 4] = [236u8, 118u8, 244u8, 66u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
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
    /**Function with signature `createOperatorSets(uint32[])` and selector `0xafe02ed5`.
```solidity
function createOperatorSets(uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createOperatorSetsCall {
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`createOperatorSets(uint32[])`](createOperatorSetsCall) function.
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
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
                    (value.operatorSetIds,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSetIds: tuple.0 }
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createOperatorSets(uint32[])";
            const SELECTOR: [u8; 4] = [175u8, 224u8, 46u8, 213u8];
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
    /**Function with signature `deregisterOperatorFromAVS(address)` and selector `0xa364f4da`.
```solidity
function deregisterOperatorFromAVS(address operator) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromAVS(address)`](deregisterOperatorFromAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromAVSReturn {}
    #[allow(
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
            impl ::core::convert::From<deregisterOperatorFromAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<deregisterOperatorFromAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromAVS(address)";
            const SELECTOR: [u8; 4] = [163u8, 100u8, 244u8, 218u8];
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
    /**Function with signature `deregisterOperatorFromOperatorSets(address,uint32[])` and selector `0xc1a8e2c5`.
```solidity
function deregisterOperatorFromOperatorSets(address operator, uint32[] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
    }
    ///Container type for the return parameters of the [`deregisterOperatorFromOperatorSets(address,uint32[])`](deregisterOperatorFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorFromOperatorSetsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
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
            impl ::core::convert::From<deregisterOperatorFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperatorFromOperatorSets(address,uint32[])";
            const SELECTOR: [u8; 4] = [193u8, 168u8, 226u8, 197u8];
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
    /**Function with signature `domainSeparator()` and selector `0xf698da25`.
```solidity
function domainSeparator() external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorCall {}
    ///Container type for the return parameters of the [`domainSeparator()`](domainSeparatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct domainSeparatorReturn {
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
            impl ::core::convert::From<domainSeparatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for domainSeparatorCall {
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
            impl ::core::convert::From<domainSeparatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: domainSeparatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for domainSeparatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for domainSeparatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = domainSeparatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "domainSeparator()";
            const SELECTOR: [u8; 4] = [246u8, 152u8, 218u8, 37u8];
            #[inline]
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
    /**Function with signature `forceDeregisterFromOperatorSets(address,address,uint32[],(bytes,bytes32,uint256))` and selector `0x3fee332d`.
```solidity
function forceDeregisterFromOperatorSets(address operator, address avs, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceDeregisterFromOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub avs: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`forceDeregisterFromOperatorSets(address,address,uint32[],(bytes,bytes32,uint256))`](forceDeregisterFromOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct forceDeregisterFromOperatorSetsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<forceDeregisterFromOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceDeregisterFromOperatorSetsCall) -> Self {
                    (
                        value.operator,
                        value.avs,
                        value.operatorSetIds,
                        value.operatorSignature,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceDeregisterFromOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        avs: tuple.1,
                        operatorSetIds: tuple.2,
                        operatorSignature: tuple.3,
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
            impl ::core::convert::From<forceDeregisterFromOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: forceDeregisterFromOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for forceDeregisterFromOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for forceDeregisterFromOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = forceDeregisterFromOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "forceDeregisterFromOperatorSets(address,address,uint32[],(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [63u8, 238u8, 51u8, 45u8];
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
                        &self.avs,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    /**Function with signature `getNumOperatorSetsOfOperator(address)` and selector `0xe88d8049`.
```solidity
function getNumOperatorSetsOfOperator(address operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumOperatorSetsOfOperatorCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getNumOperatorSetsOfOperator(address)`](getNumOperatorSetsOfOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumOperatorSetsOfOperatorReturn {
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
            impl ::core::convert::From<getNumOperatorSetsOfOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumOperatorSetsOfOperatorCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumOperatorSetsOfOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<getNumOperatorSetsOfOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumOperatorSetsOfOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumOperatorSetsOfOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNumOperatorSetsOfOperatorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getNumOperatorSetsOfOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNumOperatorSetsOfOperator(address)";
            const SELECTOR: [u8; 4] = [232u8, 141u8, 128u8, 73u8];
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
    /**Function with signature `getNumOperatorsInOperatorSet((address,uint32))` and selector `0x1023aa35`.
```solidity
function getNumOperatorsInOperatorSet(OperatorSet memory operatorSet) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumOperatorsInOperatorSetCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`getNumOperatorsInOperatorSet((address,uint32))`](getNumOperatorsInOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getNumOperatorsInOperatorSetReturn {
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
            impl ::core::convert::From<getNumOperatorsInOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumOperatorsInOperatorSetCall) -> Self {
                    (value.operatorSet,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumOperatorsInOperatorSetCall {
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
            impl ::core::convert::From<getNumOperatorsInOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getNumOperatorsInOperatorSetReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getNumOperatorsInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getNumOperatorsInOperatorSetCall {
            type Parameters<'a> = (OperatorSet,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getNumOperatorsInOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getNumOperatorsInOperatorSet((address,uint32))";
            const SELECTOR: [u8; 4] = [16u8, 35u8, 170u8, 53u8];
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
    /**Function with signature `getOperatorSetsOfOperator(address,uint256,uint256)` and selector `0x16ae76cb`.
```solidity
function getOperatorSetsOfOperator(address operator, uint256 start, uint256 length) external view returns (OperatorSet[] memory operatorSets);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetsOfOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorSetsOfOperator(address,uint256,uint256)`](getOperatorSetsOfOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorSetsOfOperatorReturn {
        pub operatorSets: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getOperatorSetsOfOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetsOfOperatorCall) -> Self {
                    (value.operator, value.start, value.length)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetsOfOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        start: tuple.1,
                        length: tuple.2,
                    }
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
            impl ::core::convert::From<getOperatorSetsOfOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorSetsOfOperatorReturn) -> Self {
                    (value.operatorSets,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorSetsOfOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSets: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorSetsOfOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorSetsOfOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorSetsOfOperator(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [22u8, 174u8, 118u8, 203u8];
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
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
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
    /**Function with signature `getOperatorsInOperatorSet((address,uint32),uint256,uint256)` and selector `0x7357723b`.
```solidity
function getOperatorsInOperatorSet(OperatorSet memory operatorSet, uint256 start, uint256 length) external view returns (address[] memory operators);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsInOperatorSetCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub start: alloy::sol_types::private::primitives::aliases::U256,
        pub length: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getOperatorsInOperatorSet((address,uint32),uint256,uint256)`](getOperatorsInOperatorSetCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getOperatorsInOperatorSetReturn {
        pub operators: alloy::sol_types::private::Vec<
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<getOperatorsInOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsInOperatorSetCall) -> Self {
                    (value.operatorSet, value.start, value.length)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsInOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        start: tuple.1,
                        length: tuple.2,
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
            impl ::core::convert::From<getOperatorsInOperatorSetReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getOperatorsInOperatorSetReturn) -> Self {
                    (value.operators,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getOperatorsInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operators: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getOperatorsInOperatorSetCall {
            type Parameters<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getOperatorsInOperatorSetReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getOperatorsInOperatorSet((address,uint32),uint256,uint256)";
            const SELECTOR: [u8; 4] = [115u8, 87u8, 114u8, 59u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.start),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.length),
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
function getStrategiesInOperatorSet(OperatorSet memory operatorSet) external view returns (address[] memory strategies);
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
                    (value.strategies,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStrategiesInOperatorSetReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { strategies: tuple.0 }
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
    /**Function with signature `inTotalOperatorSets(address)` and selector `0xcbdf0e42`.
```solidity
function inTotalOperatorSets(address operator) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inTotalOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`inTotalOperatorSets(address)`](inTotalOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct inTotalOperatorSetsReturn {
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
            impl ::core::convert::From<inTotalOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: inTotalOperatorSetsCall) -> Self {
                    (value.operator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for inTotalOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operator: tuple.0 }
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
            impl ::core::convert::From<inTotalOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: inTotalOperatorSetsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for inTotalOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for inTotalOperatorSetsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = inTotalOperatorSetsReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "inTotalOperatorSets(address)";
            const SELECTOR: [u8; 4] = [203u8, 223u8, 14u8, 66u8];
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
    /**Function with signature `initialize(address,address,uint256)` and selector `0x1794bb3c`.
```solidity
function initialize(address initialOwner, address _pauserRegistry, uint256 initialPausedStatus) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initialOwner: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint256)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
                    (
                        value.initialOwner,
                        value._pauserRegistry,
                        value.initialPausedStatus,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initialOwner: tuple.0,
                        _pauserRegistry: tuple.1,
                        initialPausedStatus: tuple.2,
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
            const SIGNATURE: &'static str = "initialize(address,address,uint256)";
            const SELECTOR: [u8; 4] = [23u8, 148u8, 187u8, 60u8];
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
                        &self._pauserRegistry,
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
    /**Function with signature `isMember(address,(address,uint32))` and selector `0xda2ff05d`.
```solidity
function isMember(address operator, OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isMember(address,(address,uint32))`](isMemberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isMemberReturn {
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
            impl ::core::convert::From<isMemberCall> for UnderlyingRustTuple<'_> {
                fn from(value: isMemberCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isMemberCall {
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
            impl ::core::convert::From<isMemberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isMemberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isMemberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isMemberCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isMemberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isMember(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [218u8, 47u8, 240u8, 93u8];
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
    /**Function with signature `isOperatorSet(address,uint32)` and selector `0x84d76f7b`.
```solidity
function isOperatorSet(address, uint32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: u32,
    }
    ///Container type for the return parameters of the [`isOperatorSet(address,uint32)`](isOperatorSetCall) function.
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
            impl ::core::convert::From<isOperatorSetCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetCall {
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
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSet(address,uint32)";
            const SELECTOR: [u8; 4] = [132u8, 215u8, 111u8, 123u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
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
    /**Function with signature `isOperatorSetAVS(address)` and selector `0x7673e93a`.
```solidity
function isOperatorSetAVS(address) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetAVSCall {
        pub _0: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperatorSetAVS(address)`](isOperatorSetAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetAVSReturn {
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
            impl ::core::convert::From<isOperatorSetAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetAVSCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSetAVSCall {
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorSetAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetAVSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSetAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetAVSCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetAVSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSetAVS(address)";
            const SELECTOR: [u8; 4] = [118u8, 115u8, 233u8, 58u8];
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
                        &self._0,
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
    /**Function with signature `isOperatorSetBatch((address,uint32)[])` and selector `0x20c4e236`.
```solidity
function isOperatorSetBatch(OperatorSet[] memory operatorSets) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetBatchCall {
        pub operatorSets: alloy::sol_types::private::Vec<
            <OperatorSet as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`isOperatorSetBatch((address,uint32)[])`](isOperatorSetBatchCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetBatchReturn {
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
            impl ::core::convert::From<isOperatorSetBatchCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetBatchCall) -> Self {
                    (value.operatorSets,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSetBatchCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { operatorSets: tuple.0 }
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
            impl ::core::convert::From<isOperatorSetBatchReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetBatchReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSetBatchReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetBatchCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Array<OperatorSet>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetBatchReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSetBatch((address,uint32)[])";
            const SELECTOR: [u8; 4] = [32u8, 196u8, 226u8, 54u8];
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
                        OperatorSet,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSets),
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
    /**Function with signature `isOperatorSlashable(address,(address,uint32))` and selector `0x1352c3e6`.
```solidity
function isOperatorSlashable(address operator, OperatorSet memory operatorSet) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSlashableCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`isOperatorSlashable(address,(address,uint32))`](isOperatorSlashableCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSlashableReturn {
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
            impl ::core::convert::From<isOperatorSlashableCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSlashableCall) -> Self {
                    (value.operator, value.operatorSet)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSlashableCall {
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
            impl ::core::convert::From<isOperatorSlashableReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSlashableReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for isOperatorSlashableReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSlashableCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address, OperatorSet);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSlashableReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSlashable(address,(address,uint32))";
            const SELECTOR: [u8; 4] = [19u8, 82u8, 195u8, 230u8];
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
    /**Function with signature `migrateOperatorsToOperatorSets(address[],uint32[][])` and selector `0xef2dfa8d`.
```solidity
function migrateOperatorsToOperatorSets(address[] memory operators, uint32[][] memory operatorSetIds) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateOperatorsToOperatorSetsCall {
        pub operators: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
        pub operatorSetIds: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Vec<u32>,
        >,
    }
    ///Container type for the return parameters of the [`migrateOperatorsToOperatorSets(address[],uint32[][])`](migrateOperatorsToOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct migrateOperatorsToOperatorSetsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<alloy::sol_types::private::Address>,
                alloy::sol_types::private::Vec<alloy::sol_types::private::Vec<u32>>,
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
            impl ::core::convert::From<migrateOperatorsToOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateOperatorsToOperatorSetsCall) -> Self {
                    (value.operators, value.operatorSetIds)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateOperatorsToOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operators: tuple.0,
                        operatorSetIds: tuple.1,
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
            impl ::core::convert::From<migrateOperatorsToOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: migrateOperatorsToOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for migrateOperatorsToOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for migrateOperatorsToOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Address>,
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<32>,
                    >,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = migrateOperatorsToOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "migrateOperatorsToOperatorSets(address[],uint32[][])";
            const SELECTOR: [u8; 4] = [239u8, 45u8, 250u8, 141u8];
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
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Array<
                            alloy::sol_types::sol_data::Uint<32>,
                        >,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
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
    /**Function with signature `operatorSaltIsSpent(address,bytes32)` and selector `0x374823b5`.
```solidity
function operatorSaltIsSpent(address, bytes32) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSaltIsSpentCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`operatorSaltIsSpent(address,bytes32)`](operatorSaltIsSpentCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSaltIsSpentReturn {
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
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<operatorSaltIsSpentCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSaltIsSpentCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSaltIsSpentCall {
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
            impl ::core::convert::From<operatorSaltIsSpentReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSaltIsSpentReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSaltIsSpentReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSaltIsSpentCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSaltIsSpentReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSaltIsSpent(address,bytes32)";
            const SELECTOR: [u8; 4] = [55u8, 72u8, 35u8, 181u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
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
    /**Function with signature `operatorSetMemberAtIndex((address,uint32),uint256)` and selector `0x411d415b`.
```solidity
function operatorSetMemberAtIndex(OperatorSet memory operatorSet, uint256 index) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetMemberAtIndexCall {
        pub operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`operatorSetMemberAtIndex((address,uint32),uint256)`](operatorSetMemberAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetMemberAtIndexReturn {
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
            type UnderlyingSolTuple<'a> = (
                OperatorSet,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<operatorSetMemberAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetMemberAtIndexCall) -> Self {
                    (value.operatorSet, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetMemberAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSet: tuple.0,
                        index: tuple.1,
                    }
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
            impl ::core::convert::From<operatorSetMemberAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetMemberAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetMemberAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetMemberAtIndexCall {
            type Parameters<'a> = (OperatorSet, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetMemberAtIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetMemberAtIndex((address,uint32),uint256)";
            const SELECTOR: [u8; 4] = [65u8, 29u8, 65u8, 91u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Function with signature `operatorSetStatus(address,address,uint32)` and selector `0x1e68134e`.
```solidity
function operatorSetStatus(address, address, uint32) external view returns (bool registered, uint32 lastDeregisteredTimestamp);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetStatusCall {
        pub _0: alloy::sol_types::private::Address,
        pub _1: alloy::sol_types::private::Address,
        pub _2: u32,
    }
    ///Container type for the return parameters of the [`operatorSetStatus(address,address,uint32)`](operatorSetStatusCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetStatusReturn {
        pub registered: bool,
        pub lastDeregisteredTimestamp: u32,
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<operatorSetStatusCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetStatusCall) -> Self {
                    (value._0, value._1, value._2)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetStatusCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                        _2: tuple.2,
                    }
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
            impl ::core::convert::From<operatorSetStatusReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetStatusReturn) -> Self {
                    (value.registered, value.lastDeregisteredTimestamp)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetStatusReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        registered: tuple.0,
                        lastDeregisteredTimestamp: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetStatusCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetStatusReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Bool,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetStatus(address,address,uint32)";
            const SELECTOR: [u8; 4] = [30u8, 104u8, 19u8, 78u8];
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
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._1,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self._2),
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
    /**Function with signature `operatorSetsMemberOfAtIndex(address,uint256)` and selector `0xb5a768ca`.
```solidity
function operatorSetsMemberOfAtIndex(address operator, uint256 index) external view returns (OperatorSet memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsMemberOfAtIndexCall {
        pub operator: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`operatorSetsMemberOfAtIndex(address,uint256)`](operatorSetsMemberOfAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct operatorSetsMemberOfAtIndexReturn {
        pub _0: <OperatorSet as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<operatorSetsMemberOfAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsMemberOfAtIndexCall) -> Self {
                    (value.operator, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetsMemberOfAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<operatorSetsMemberOfAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: operatorSetsMemberOfAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for operatorSetsMemberOfAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for operatorSetsMemberOfAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = operatorSetsMemberOfAtIndexReturn;
            type ReturnTuple<'a> = (OperatorSet,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "operatorSetsMemberOfAtIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [181u8, 167u8, 104u8, 202u8];
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
                        256,
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
    /**Function with signature `registerOperatorToAVS(address,(bytes,bytes32,uint256))` and selector `0x9926ee7d`.
```solidity
function registerOperatorToAVS(address operator, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToAVS(address,(bytes,bytes32,uint256))`](registerOperatorToAVSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToAVSReturn {}
    #[allow(
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
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToAVSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSCall) -> Self {
                    (value.operator, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSignature: tuple.1,
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
            impl ::core::convert::From<registerOperatorToAVSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToAVSReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToAVSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToAVSCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToAVSReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToAVS(address,(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [153u8, 38u8, 238u8, 125u8];
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
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    /**Function with signature `registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))` and selector `0x1e2199e2`.
```solidity
function registerOperatorToOperatorSets(address operator, uint32[] memory operatorSetIds, ISignatureUtils.SignatureWithSaltAndExpiry memory operatorSignature) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToOperatorSetsCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorSetIds: alloy::sol_types::private::Vec<u32>,
        pub operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))`](registerOperatorToOperatorSetsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorToOperatorSetsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Vec<u32>,
                <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<registerOperatorToOperatorSetsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToOperatorSetsCall) -> Self {
                    (value.operator, value.operatorSetIds, value.operatorSignature)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToOperatorSetsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorSetIds: tuple.1,
                        operatorSignature: tuple.2,
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
            impl ::core::convert::From<registerOperatorToOperatorSetsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorToOperatorSetsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorToOperatorSetsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorToOperatorSetsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
                ISignatureUtils::SignatureWithSaltAndExpiry,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorToOperatorSetsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperatorToOperatorSets(address,uint32[],(bytes,bytes32,uint256))";
            const SELECTOR: [u8; 4] = [30u8, 33u8, 153u8, 226u8];
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
                        alloy::sol_types::sol_data::Uint<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorSetIds),
                    <ISignatureUtils::SignatureWithSaltAndExpiry as alloy_sol_types::SolType>::tokenize(
                        &self.operatorSignature,
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
    /**Function with signature `removeStrategiesFromOperatorSet(uint32,address[])` and selector `0xce7b5e4b`.
```solidity
function removeStrategiesFromOperatorSet(uint32 operatorSetId, address[] memory strategies) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesFromOperatorSetCall {
        pub operatorSetId: u32,
        pub strategies: alloy::sol_types::private::Vec<
            alloy::sol_types::private::Address,
        >,
    }
    ///Container type for the return parameters of the [`removeStrategiesFromOperatorSet(uint32,address[])`](removeStrategiesFromOperatorSetCall) function.
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
            impl ::core::convert::From<removeStrategiesFromOperatorSetCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesFromOperatorSetCall) -> Self {
                    (value.operatorSetId, value.strategies)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesFromOperatorSetCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorSetId: tuple.0,
                        strategies: tuple.1,
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
            const SIGNATURE: &'static str = "removeStrategiesFromOperatorSet(uint32,address[])";
            const SELECTOR: [u8; 4] = [206u8, 123u8, 94u8, 75u8];
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
    /**Function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`.
```solidity
function setPauserRegistry(address newPauserRegistry) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryCall {
        pub newPauserRegistry: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setPauserRegistry(address)`](setPauserRegistryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setPauserRegistryReturn {}
    #[allow(
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
            impl ::core::convert::From<setPauserRegistryCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryCall) -> Self {
                    (value.newPauserRegistry,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newPauserRegistry: tuple.0 }
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
            impl ::core::convert::From<setPauserRegistryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setPauserRegistryReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setPauserRegistryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setPauserRegistryCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setPauserRegistryReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setPauserRegistry(address)";
            const SELECTOR: [u8; 4] = [16u8, 214u8, 122u8, 47u8];
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
                        &self.newPauserRegistry,
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
    /**Function with signature `updateAVSMetadataURI(string)` and selector `0xa98fb355`.
```solidity
function updateAVSMetadataURI(string memory metadataURI) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateAVSMetadataURICall {
        pub metadataURI: alloy::sol_types::private::String,
    }
    ///Container type for the return parameters of the [`updateAVSMetadataURI(string)`](updateAVSMetadataURICall) function.
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
            impl ::core::convert::From<updateAVSMetadataURICall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateAVSMetadataURICall) -> Self {
                    (value.metadataURI,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateAVSMetadataURICall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { metadataURI: tuple.0 }
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
            type Parameters<'a> = (alloy::sol_types::sol_data::String,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateAVSMetadataURIReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateAVSMetadataURI(string)";
            const SELECTOR: [u8; 4] = [169u8, 143u8, 179u8, 85u8];
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
    ///Container for all the [`AVSDirectory`](self) function calls.
    pub enum AVSDirectoryCalls {
        DEALLOCATION_DELAY(DEALLOCATION_DELAYCall),
        OPERATOR_AVS_REGISTRATION_TYPEHASH(OPERATOR_AVS_REGISTRATION_TYPEHASHCall),
        OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(
            OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall,
        ),
        OPERATOR_SET_REGISTRATION_TYPEHASH(OPERATOR_SET_REGISTRATION_TYPEHASHCall),
        addStrategiesToOperatorSet(addStrategiesToOperatorSetCall),
        avsOperatorStatus(avsOperatorStatusCall),
        becomeOperatorSetAVS(becomeOperatorSetAVSCall),
        calculateOperatorAVSRegistrationDigestHash(
            calculateOperatorAVSRegistrationDigestHashCall,
        ),
        calculateOperatorSetForceDeregistrationTypehash(
            calculateOperatorSetForceDeregistrationTypehashCall,
        ),
        calculateOperatorSetRegistrationDigestHash(
            calculateOperatorSetRegistrationDigestHashCall,
        ),
        cancelSalt(cancelSaltCall),
        createOperatorSets(createOperatorSetsCall),
        delegation(delegationCall),
        deregisterOperatorFromAVS(deregisterOperatorFromAVSCall),
        deregisterOperatorFromOperatorSets(deregisterOperatorFromOperatorSetsCall),
        domainSeparator(domainSeparatorCall),
        forceDeregisterFromOperatorSets(forceDeregisterFromOperatorSetsCall),
        getNumOperatorSetsOfOperator(getNumOperatorSetsOfOperatorCall),
        getNumOperatorsInOperatorSet(getNumOperatorsInOperatorSetCall),
        getOperatorSetsOfOperator(getOperatorSetsOfOperatorCall),
        getOperatorsInOperatorSet(getOperatorsInOperatorSetCall),
        getStrategiesInOperatorSet(getStrategiesInOperatorSetCall),
        inTotalOperatorSets(inTotalOperatorSetsCall),
        initialize(initializeCall),
        isMember(isMemberCall),
        isOperatorSet(isOperatorSetCall),
        isOperatorSetAVS(isOperatorSetAVSCall),
        isOperatorSetBatch(isOperatorSetBatchCall),
        isOperatorSlashable(isOperatorSlashableCall),
        migrateOperatorsToOperatorSets(migrateOperatorsToOperatorSetsCall),
        operatorSaltIsSpent(operatorSaltIsSpentCall),
        operatorSetMemberAtIndex(operatorSetMemberAtIndexCall),
        operatorSetStatus(operatorSetStatusCall),
        operatorSetsMemberOfAtIndex(operatorSetsMemberOfAtIndexCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        registerOperatorToAVS(registerOperatorToAVSCall),
        registerOperatorToOperatorSets(registerOperatorToOperatorSetsCall),
        removeStrategiesFromOperatorSet(removeStrategiesFromOperatorSetCall),
        renounceOwnership(renounceOwnershipCall),
        setPauserRegistry(setPauserRegistryCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        updateAVSMetadataURI(updateAVSMetadataURICall),
    }
    #[automatically_derived]
    impl AVSDirectoryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [16u8, 35u8, 170u8, 53u8],
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 82u8, 195u8, 230u8],
            [19u8, 100u8, 57u8, 221u8],
            [22u8, 174u8, 118u8, 203u8],
            [23u8, 148u8, 187u8, 60u8],
            [30u8, 33u8, 153u8, 226u8],
            [30u8, 104u8, 19u8, 78u8],
            [32u8, 196u8, 226u8, 54u8],
            [41u8, 129u8, 235u8, 119u8],
            [55u8, 72u8, 35u8, 181u8],
            [63u8, 238u8, 51u8, 45u8],
            [65u8, 29u8, 65u8, 91u8],
            [65u8, 119u8, 168u8, 124u8],
            [73u8, 7u8, 93u8, 163u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [113u8, 80u8, 24u8, 166u8],
            [115u8, 87u8, 114u8, 59u8],
            [118u8, 115u8, 233u8, 58u8],
            [118u8, 153u8, 147u8, 66u8],
            [132u8, 215u8, 111u8, 123u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [149u8, 94u8, 102u8, 150u8],
            [153u8, 38u8, 238u8, 125u8],
            [161u8, 6u8, 12u8, 136u8],
            [163u8, 100u8, 244u8, 218u8],
            [169u8, 143u8, 179u8, 85u8],
            [174u8, 194u8, 5u8, 197u8],
            [175u8, 224u8, 46u8, 213u8],
            [178u8, 132u8, 29u8, 72u8],
            [181u8, 167u8, 104u8, 202u8],
            [193u8, 168u8, 226u8, 197u8],
            [200u8, 37u8, 254u8, 104u8],
            [203u8, 223u8, 14u8, 66u8],
            [206u8, 123u8, 94u8, 75u8],
            [215u8, 154u8, 206u8, 171u8],
            [218u8, 47u8, 240u8, 93u8],
            [220u8, 233u8, 116u8, 185u8],
            [223u8, 92u8, 247u8, 35u8],
            [232u8, 141u8, 128u8, 73u8],
            [236u8, 118u8, 244u8, 66u8],
            [239u8, 45u8, 250u8, 141u8],
            [242u8, 253u8, 227u8, 139u8],
            [246u8, 152u8, 218u8, 37u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AVSDirectoryCalls {
        const NAME: &'static str = "AVSDirectoryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 48usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::DEALLOCATION_DELAY(_) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPERATOR_AVS_REGISTRATION_TYPEHASH(_) => {
                    <OPERATOR_AVS_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(_) => {
                    <OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::OPERATOR_SET_REGISTRATION_TYPEHASH(_) => {
                    <OPERATOR_SET_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategiesToOperatorSet(_) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsOperatorStatus(_) => {
                    <avsOperatorStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::becomeOperatorSetAVS(_) => {
                    <becomeOperatorSetAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorAVSRegistrationDigestHash(_) => {
                    <calculateOperatorAVSRegistrationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorSetForceDeregistrationTypehash(_) => {
                    <calculateOperatorSetForceDeregistrationTypehashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateOperatorSetRegistrationDigestHash(_) => {
                    <calculateOperatorSetRegistrationDigestHashCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::cancelSalt(_) => {
                    <cancelSaltCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createOperatorSets(_) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromAVS(_) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperatorFromOperatorSets(_) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::domainSeparator(_) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::forceDeregisterFromOperatorSets(_) => {
                    <forceDeregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNumOperatorSetsOfOperator(_) => {
                    <getNumOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getNumOperatorsInOperatorSet(_) => {
                    <getNumOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorSetsOfOperator(_) => {
                    <getOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getOperatorsInOperatorSet(_) => {
                    <getOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStrategiesInOperatorSet(_) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::inTotalOperatorSets(_) => {
                    <inTotalOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isMember(_) => <isMemberCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::isOperatorSet(_) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSetAVS(_) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSetBatch(_) => {
                    <isOperatorSetBatchCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperatorSlashable(_) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::migrateOperatorsToOperatorSets(_) => {
                    <migrateOperatorsToOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSaltIsSpent(_) => {
                    <operatorSaltIsSpentCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetMemberAtIndex(_) => {
                    <operatorSetMemberAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetStatus(_) => {
                    <operatorSetStatusCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::operatorSetsMemberOfAtIndex(_) => {
                    <operatorSetsMemberOfAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToAVS(_) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperatorToOperatorSets(_) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategiesFromOperatorSet(_) => {
                    <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
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
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<AVSDirectoryCalls>] = &[
                {
                    fn getNumOperatorsInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <getNumOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::getNumOperatorsInOperatorSet)
                    }
                    getNumOperatorsInOperatorSet
                },
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn isOperatorSlashable(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::isOperatorSlashable)
                    }
                    isOperatorSlashable
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::pause)
                    }
                    pause
                },
                {
                    fn getOperatorSetsOfOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <getOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::getOperatorSetsOfOperator)
                    }
                    getOperatorSetsOfOperator
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::initialize)
                    }
                    initialize
                },
                {
                    fn registerOperatorToOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::registerOperatorToOperatorSets)
                    }
                    registerOperatorToOperatorSets
                },
                {
                    fn operatorSetStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <operatorSetStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::operatorSetStatus)
                    }
                    operatorSetStatus
                },
                {
                    fn isOperatorSetBatch(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <isOperatorSetBatchCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::isOperatorSetBatch)
                    }
                    isOperatorSetBatch
                },
                {
                    fn DEALLOCATION_DELAY(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::DEALLOCATION_DELAY)
                    }
                    DEALLOCATION_DELAY
                },
                {
                    fn operatorSaltIsSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <operatorSaltIsSpentCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::operatorSaltIsSpent)
                    }
                    operatorSaltIsSpent
                },
                {
                    fn forceDeregisterFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <forceDeregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::forceDeregisterFromOperatorSets)
                    }
                    forceDeregisterFromOperatorSets
                },
                {
                    fn operatorSetMemberAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <operatorSetMemberAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::operatorSetMemberAtIndex)
                    }
                    operatorSetMemberAtIndex
                },
                {
                    fn getStrategiesInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::getStrategiesInOperatorSet)
                    }
                    getStrategiesInOperatorSet
                },
                {
                    fn avsOperatorStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <avsOperatorStatusCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::avsOperatorStatus)
                    }
                    avsOperatorStatus
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn getOperatorsInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <getOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::getOperatorsInOperatorSet)
                    }
                    getOperatorsInOperatorSet
                },
                {
                    fn isOperatorSetAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::isOperatorSetAVS)
                    }
                    isOperatorSetAVS
                },
                {
                    fn addStrategiesToOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::addStrategiesToOperatorSet)
                    }
                    addStrategiesToOperatorSet
                },
                {
                    fn isOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <isOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::isOperatorSet)
                    }
                    isOperatorSet
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::owner)
                    }
                    owner
                },
                {
                    fn calculateOperatorSetRegistrationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <calculateOperatorSetRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AVSDirectoryCalls::calculateOperatorSetRegistrationDigestHash,
                            )
                    }
                    calculateOperatorSetRegistrationDigestHash
                },
                {
                    fn registerOperatorToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::registerOperatorToAVS)
                    }
                    registerOperatorToAVS
                },
                {
                    fn calculateOperatorAVSRegistrationDigestHash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <calculateOperatorAVSRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AVSDirectoryCalls::calculateOperatorAVSRegistrationDigestHash,
                            )
                    }
                    calculateOperatorAVSRegistrationDigestHash
                },
                {
                    fn deregisterOperatorFromAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::deregisterOperatorFromAVS)
                    }
                    deregisterOperatorFromAVS
                },
                {
                    fn updateAVSMetadataURI(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <updateAVSMetadataURICall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::updateAVSMetadataURI)
                    }
                    updateAVSMetadataURI
                },
                {
                    fn becomeOperatorSetAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <becomeOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::becomeOperatorSetAVS)
                    }
                    becomeOperatorSetAVS
                },
                {
                    fn createOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::createOperatorSets)
                    }
                    createOperatorSets
                },
                {
                    fn calculateOperatorSetForceDeregistrationTypehash(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <calculateOperatorSetForceDeregistrationTypehashCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AVSDirectoryCalls::calculateOperatorSetForceDeregistrationTypehash,
                            )
                    }
                    calculateOperatorSetForceDeregistrationTypehash
                },
                {
                    fn operatorSetsMemberOfAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <operatorSetsMemberOfAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::operatorSetsMemberOfAtIndex)
                    }
                    operatorSetsMemberOfAtIndex
                },
                {
                    fn deregisterOperatorFromOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::deregisterOperatorFromOperatorSets)
                    }
                    deregisterOperatorFromOperatorSets
                },
                {
                    fn OPERATOR_SET_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <OPERATOR_SET_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::OPERATOR_SET_REGISTRATION_TYPEHASH)
                    }
                    OPERATOR_SET_REGISTRATION_TYPEHASH
                },
                {
                    fn inTotalOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <inTotalOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::inTotalOperatorSets)
                    }
                    inTotalOperatorSets
                },
                {
                    fn removeStrategiesFromOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <removeStrategiesFromOperatorSetCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::removeStrategiesFromOperatorSet)
                    }
                    removeStrategiesFromOperatorSet
                },
                {
                    fn OPERATOR_AVS_REGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <OPERATOR_AVS_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::OPERATOR_AVS_REGISTRATION_TYPEHASH)
                    }
                    OPERATOR_AVS_REGISTRATION_TYPEHASH
                },
                {
                    fn isMember(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <isMemberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::isMember)
                    }
                    isMember
                },
                {
                    fn OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                AVSDirectoryCalls::OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH,
                            )
                    }
                    OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::delegation)
                    }
                    delegation
                },
                {
                    fn getNumOperatorSetsOfOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <getNumOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::getNumOperatorSetsOfOperator)
                    }
                    getNumOperatorSetsOfOperator
                },
                {
                    fn cancelSalt(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <cancelSaltCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::cancelSalt)
                    }
                    cancelSalt
                },
                {
                    fn migrateOperatorsToOperatorSets(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <migrateOperatorsToOperatorSetsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::migrateOperatorsToOperatorSets)
                    }
                    migrateOperatorsToOperatorSets
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn domainSeparator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <domainSeparatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::domainSeparator)
                    }
                    domainSeparator
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryCalls::unpause)
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
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPERATOR_AVS_REGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_AVS_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OPERATOR_SET_REGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_SET_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategiesToOperatorSet(inner) => {
                    <addStrategiesToOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::avsOperatorStatus(inner) => {
                    <avsOperatorStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::becomeOperatorSetAVS(inner) => {
                    <becomeOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorAVSRegistrationDigestHash(inner) => {
                    <calculateOperatorAVSRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorSetForceDeregistrationTypehash(inner) => {
                    <calculateOperatorSetForceDeregistrationTypehashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateOperatorSetRegistrationDigestHash(inner) => {
                    <calculateOperatorSetRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::cancelSalt(inner) => {
                    <cancelSaltCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::createOperatorSets(inner) => {
                    <createOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::forceDeregisterFromOperatorSets(inner) => {
                    <forceDeregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNumOperatorSetsOfOperator(inner) => {
                    <getNumOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getNumOperatorsInOperatorSet(inner) => {
                    <getNumOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorSetsOfOperator(inner) => {
                    <getOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getOperatorsInOperatorSet(inner) => {
                    <getOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStrategiesInOperatorSet(inner) => {
                    <getStrategiesInOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::inTotalOperatorSets(inner) => {
                    <inTotalOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isMember(inner) => {
                    <isMemberCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOperatorSet(inner) => {
                    <isOperatorSetCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSetAVS(inner) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSetBatch(inner) => {
                    <isOperatorSetBatchCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::isOperatorSlashable(inner) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::migrateOperatorsToOperatorSets(inner) => {
                    <migrateOperatorsToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSaltIsSpent(inner) => {
                    <operatorSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetMemberAtIndex(inner) => {
                    <operatorSetMemberAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetStatus(inner) => {
                    <operatorSetStatusCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::operatorSetsMemberOfAtIndex(inner) => {
                    <operatorSetsMemberOfAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperatorToOperatorSets(inner) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::DEALLOCATION_DELAY(inner) => {
                    <DEALLOCATION_DELAYCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPERATOR_AVS_REGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_AVS_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OPERATOR_SET_REGISTRATION_TYPEHASH(inner) => {
                    <OPERATOR_SET_REGISTRATION_TYPEHASHCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::avsOperatorStatus(inner) => {
                    <avsOperatorStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::becomeOperatorSetAVS(inner) => {
                    <becomeOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorAVSRegistrationDigestHash(inner) => {
                    <calculateOperatorAVSRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorSetForceDeregistrationTypehash(inner) => {
                    <calculateOperatorSetForceDeregistrationTypehashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateOperatorSetRegistrationDigestHash(inner) => {
                    <calculateOperatorSetRegistrationDigestHashCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::cancelSalt(inner) => {
                    <cancelSaltCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deregisterOperatorFromAVS(inner) => {
                    <deregisterOperatorFromAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::deregisterOperatorFromOperatorSets(inner) => {
                    <deregisterOperatorFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::domainSeparator(inner) => {
                    <domainSeparatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::forceDeregisterFromOperatorSets(inner) => {
                    <forceDeregisterFromOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNumOperatorSetsOfOperator(inner) => {
                    <getNumOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getNumOperatorsInOperatorSet(inner) => {
                    <getNumOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorSetsOfOperator(inner) => {
                    <getOperatorSetsOfOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getOperatorsInOperatorSet(inner) => {
                    <getOperatorsInOperatorSetCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::inTotalOperatorSets(inner) => {
                    <inTotalOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isMember(inner) => {
                    <isMemberCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::isOperatorSetAVS(inner) => {
                    <isOperatorSetAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorSetBatch(inner) => {
                    <isOperatorSetBatchCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperatorSlashable(inner) => {
                    <isOperatorSlashableCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::migrateOperatorsToOperatorSets(inner) => {
                    <migrateOperatorsToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSaltIsSpent(inner) => {
                    <operatorSaltIsSpentCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetMemberAtIndex(inner) => {
                    <operatorSetMemberAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetStatus(inner) => {
                    <operatorSetStatusCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::operatorSetsMemberOfAtIndex(inner) => {
                    <operatorSetsMemberOfAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::registerOperatorToAVS(inner) => {
                    <registerOperatorToAVSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperatorToOperatorSets(inner) => {
                    <registerOperatorToOperatorSetsCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setPauserRegistry(inner) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`AVSDirectory`](self) custom errors.
    pub enum AVSDirectoryErrors {
        CurrentlyPaused(CurrentlyPaused),
        InputAddressZero(InputAddressZero),
        InvalidAVS(InvalidAVS),
        InvalidNewPausedStatus(InvalidNewPausedStatus),
        InvalidOperator(InvalidOperator),
        InvalidOperatorSet(InvalidOperatorSet),
        InvalidSignature(InvalidSignature),
        OnlyPauser(OnlyPauser),
        OnlyUnpauser(OnlyUnpauser),
        OperatorAlreadyRegisteredToAVS(OperatorAlreadyRegisteredToAVS),
        OperatorNotRegisteredToAVS(OperatorNotRegisteredToAVS),
        OperatorNotRegisteredToEigenLayer(OperatorNotRegisteredToEigenLayer),
        SaltSpent(SaltSpent),
        SignatureExpired(SignatureExpired),
        StrategyAlreadyInOperatorSet(StrategyAlreadyInOperatorSet),
        StrategyNotInOperatorSet(StrategyNotInOperatorSet),
    }
    #[automatically_derived]
    impl AVSDirectoryErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [8u8, 25u8, 189u8, 205u8],
            [53u8, 49u8, 50u8, 68u8],
            [53u8, 74u8, 81u8, 118u8],
            [82u8, 223u8, 69u8, 201u8],
            [88u8, 92u8, 251u8, 47u8],
            [99u8, 120u8, 104u8, 78u8],
            [102u8, 229u8, 101u8, 223u8],
            [115u8, 99u8, 33u8, 118u8],
            [117u8, 223u8, 81u8, 220u8],
            [121u8, 72u8, 33u8, 255u8],
            [126u8, 197u8, 193u8, 84u8],
            [132u8, 10u8, 72u8, 213u8],
            [139u8, 170u8, 87u8, 159u8],
            [159u8, 136u8, 200u8, 175u8],
            [198u8, 29u8, 202u8, 93u8],
            [204u8, 234u8, 158u8, 111u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for AVSDirectoryErrors {
        const NAME: &'static str = "AVSDirectoryErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 16usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::CurrentlyPaused(_) => {
                    <CurrentlyPaused as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InputAddressZero(_) => {
                    <InputAddressZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidAVS(_) => {
                    <InvalidAVS as alloy_sol_types::SolError>::SELECTOR
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
                Self::InvalidSignature(_) => {
                    <InvalidSignature as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyPauser(_) => {
                    <OnlyPauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OnlyUnpauser(_) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorAlreadyRegisteredToAVS(_) => {
                    <OperatorAlreadyRegisteredToAVS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegisteredToAVS(_) => {
                    <OperatorNotRegisteredToAVS as alloy_sol_types::SolError>::SELECTOR
                }
                Self::OperatorNotRegisteredToEigenLayer(_) => {
                    <OperatorNotRegisteredToEigenLayer as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SaltSpent(_) => <SaltSpent as alloy_sol_types::SolError>::SELECTOR,
                Self::SignatureExpired(_) => {
                    <SignatureExpired as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyAlreadyInOperatorSet(_) => {
                    <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::SELECTOR
                }
                Self::StrategyNotInOperatorSet(_) => {
                    <StrategyNotInOperatorSet as alloy_sol_types::SolError>::SELECTOR
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
            ) -> alloy_sol_types::Result<AVSDirectoryErrors>] = &[
                {
                    fn SignatureExpired(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <SignatureExpired as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::SignatureExpired)
                    }
                    SignatureExpired
                },
                {
                    fn SaltSpent(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <SaltSpent as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::SaltSpent)
                    }
                    SaltSpent
                },
                {
                    fn OperatorAlreadyRegisteredToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <OperatorAlreadyRegisteredToAVS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::OperatorAlreadyRegisteredToAVS)
                    }
                    OperatorAlreadyRegisteredToAVS
                },
                {
                    fn OperatorNotRegisteredToAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <OperatorNotRegisteredToAVS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::OperatorNotRegisteredToAVS)
                    }
                    OperatorNotRegisteredToAVS
                },
                {
                    fn StrategyAlreadyInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <StrategyAlreadyInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::StrategyAlreadyInOperatorSet)
                    }
                    StrategyAlreadyInOperatorSet
                },
                {
                    fn StrategyNotInOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <StrategyNotInOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::StrategyNotInOperatorSet)
                    }
                    StrategyNotInOperatorSet
                },
                {
                    fn InvalidAVS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InvalidAVS as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InvalidAVS)
                    }
                    InvalidAVS
                },
                {
                    fn InputAddressZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InputAddressZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InputAddressZero)
                    }
                    InputAddressZero
                },
                {
                    fn OnlyPauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <OnlyPauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::OnlyPauser)
                    }
                    OnlyPauser
                },
                {
                    fn OnlyUnpauser(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <OnlyUnpauser as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::OnlyUnpauser)
                    }
                    OnlyUnpauser
                },
                {
                    fn InvalidOperatorSet(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InvalidOperatorSet as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InvalidOperatorSet)
                    }
                    InvalidOperatorSet
                },
                {
                    fn CurrentlyPaused(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <CurrentlyPaused as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::CurrentlyPaused)
                    }
                    CurrentlyPaused
                },
                {
                    fn InvalidSignature(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InvalidSignature as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InvalidSignature)
                    }
                    InvalidSignature
                },
                {
                    fn OperatorNotRegisteredToEigenLayer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <OperatorNotRegisteredToEigenLayer as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::OperatorNotRegisteredToEigenLayer)
                    }
                    OperatorNotRegisteredToEigenLayer
                },
                {
                    fn InvalidNewPausedStatus(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InvalidNewPausedStatus as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InvalidNewPausedStatus)
                    }
                    InvalidNewPausedStatus
                },
                {
                    fn InvalidOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<AVSDirectoryErrors> {
                        <InvalidOperator as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(AVSDirectoryErrors::InvalidOperator)
                    }
                    InvalidOperator
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
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encoded_size(inner)
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OnlyPauser(inner) => {
                    <OnlyPauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OnlyUnpauser(inner) => {
                    <OnlyUnpauser as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::OperatorAlreadyRegisteredToAVS(inner) => {
                    <OperatorAlreadyRegisteredToAVS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegisteredToAVS(inner) => {
                    <OperatorNotRegisteredToAVS as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::OperatorNotRegisteredToEigenLayer(inner) => {
                    <OperatorNotRegisteredToEigenLayer as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encoded_size(
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
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
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
                Self::InvalidAVS(inner) => {
                    <InvalidAVS as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
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
                Self::InvalidSignature(inner) => {
                    <InvalidSignature as alloy_sol_types::SolError>::abi_encode_raw(
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
                Self::OperatorAlreadyRegisteredToAVS(inner) => {
                    <OperatorAlreadyRegisteredToAVS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegisteredToAVS(inner) => {
                    <OperatorNotRegisteredToAVS as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::OperatorNotRegisteredToEigenLayer(inner) => {
                    <OperatorNotRegisteredToEigenLayer as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SaltSpent(inner) => {
                    <SaltSpent as alloy_sol_types::SolError>::abi_encode_raw(inner, out)
                }
                Self::SignatureExpired(inner) => {
                    <SignatureExpired as alloy_sol_types::SolError>::abi_encode_raw(
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
            }
        }
    }
    ///Container for all the [`AVSDirectory`](self) events.
    pub enum AVSDirectoryEvents {
        AVSMetadataURIUpdated(AVSMetadataURIUpdated),
        AVSMigratedToOperatorSets(AVSMigratedToOperatorSets),
        Initialized(Initialized),
        OperatorAVSRegistrationStatusUpdated(OperatorAVSRegistrationStatusUpdated),
        OperatorAddedToOperatorSet(OperatorAddedToOperatorSet),
        OperatorMigratedToOperatorSets(OperatorMigratedToOperatorSets),
        OperatorRemovedFromOperatorSet(OperatorRemovedFromOperatorSet),
        OperatorSetCreated(OperatorSetCreated),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        StrategyAddedToOperatorSet(StrategyAddedToOperatorSet),
        StrategyRemovedFromOperatorSet(StrategyRemovedFromOperatorSet),
        Unpaused(Unpaused),
    }
    #[automatically_derived]
    impl AVSDirectoryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
                84u8,
                243u8,
                60u8,
                253u8,
                209u8,
                202u8,
                112u8,
                61u8,
                121u8,
                89u8,
                134u8,
                185u8,
                134u8,
                253u8,
                71u8,
                215u8,
                66u8,
                234u8,
                177u8,
                144u8,
                78u8,
                205u8,
                42u8,
                95u8,
                219u8,
                141u8,
                101u8,
                149u8,
                229u8,
                144u8,
                74u8,
                1u8,
            ],
            [
                110u8,
                159u8,
                205u8,
                83u8,
                152u8,
                150u8,
                252u8,
                166u8,
                14u8,
                139u8,
                15u8,
                1u8,
                221u8,
                88u8,
                2u8,
                51u8,
                228u8,
                138u8,
                107u8,
                15u8,
                125u8,
                240u8,
                19u8,
                184u8,
                155u8,
                167u8,
                245u8,
                101u8,
                134u8,
                154u8,
                205u8,
                182u8,
            ],
            [
                112u8,
                43u8,
                12u8,
                31u8,
                108u8,
                177u8,
                207u8,
                81u8,
                26u8,
                170u8,
                129u8,
                247u8,
                43u8,
                192u8,
                90u8,
                33u8,
                91u8,
                179u8,
                73u8,
                118u8,
                50u8,
                215u8,
                44u8,
                105u8,
                12u8,
                130u8,
                43u8,
                4u8,
                74u8,
                180u8,
                148u8,
                191u8,
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
            [
                240u8,
                149u8,
                43u8,
                28u8,
                101u8,
                39u8,
                29u8,
                129u8,
                157u8,
                57u8,
                152u8,
                61u8,
                42u8,
                187u8,
                4u8,
                75u8,
                156u8,
                172u8,
                229u8,
                155u8,
                204u8,
                77u8,
                77u8,
                211u8,
                137u8,
                245u8,
                134u8,
                235u8,
                220u8,
                177u8,
                91u8,
                65u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for AVSDirectoryEvents {
        const NAME: &'static str = "AVSDirectoryEvents";
        const COUNT: usize = 14usize;
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
                Some(
                    <AVSMigratedToOperatorSets as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <AVSMigratedToOperatorSets as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::AVSMigratedToOperatorSets)
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
                    <OperatorAVSRegistrationStatusUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorAVSRegistrationStatusUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorAVSRegistrationStatusUpdated)
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
                    <OperatorMigratedToOperatorSets as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorMigratedToOperatorSets as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorMigratedToOperatorSets)
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
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <PauserRegistrySet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::PauserRegistrySet)
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
    impl alloy_sol_types::private::IntoLogData for AVSDirectoryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::AVSMetadataURIUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::AVSMigratedToOperatorSets(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorAVSRegistrationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorMigratedToOperatorSets(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
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
                Self::AVSMigratedToOperatorSets(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialized(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorAVSRegistrationStatusUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorAddedToOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorMigratedToOperatorSets(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorRemovedFromOperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSetCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Paused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::PauserRegistrySet(inner) => {
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
    /**Creates a new wrapper around an on-chain [`AVSDirectory`](self) contract instance.

See the [wrapper's documentation](`AVSDirectoryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> AVSDirectoryInstance<T, P, N> {
        AVSDirectoryInstance::<T, P, N>::new(address, provider)
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
        _DEALLOCATION_DELAY: u32,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<AVSDirectoryInstance<T, P, N>>,
    > {
        AVSDirectoryInstance::<
            T,
            P,
            N,
        >::deploy(provider, _delegation, _DEALLOCATION_DELAY)
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
        _DEALLOCATION_DELAY: u32,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        AVSDirectoryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _delegation, _DEALLOCATION_DELAY)
    }
    /**A [`AVSDirectory`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`AVSDirectory`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct AVSDirectoryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for AVSDirectoryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("AVSDirectoryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > AVSDirectoryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`AVSDirectory`](self) contract instance.

See the [wrapper's documentation](`AVSDirectoryInstance`) for more details.*/
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
            _DEALLOCATION_DELAY: u32,
        ) -> alloy_contract::Result<AVSDirectoryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _delegation,
                _DEALLOCATION_DELAY,
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
            _DEALLOCATION_DELAY: u32,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _delegation,
                            _DEALLOCATION_DELAY,
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
    impl<T, P: ::core::clone::Clone, N> AVSDirectoryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> AVSDirectoryInstance<T, P, N> {
            AVSDirectoryInstance {
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
    > AVSDirectoryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`DEALLOCATION_DELAY`] function.
        pub fn DEALLOCATION_DELAY(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, DEALLOCATION_DELAYCall, N> {
            self.call_builder(&DEALLOCATION_DELAYCall {})
        }
        ///Creates a new call builder for the [`OPERATOR_AVS_REGISTRATION_TYPEHASH`] function.
        pub fn OPERATOR_AVS_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            OPERATOR_AVS_REGISTRATION_TYPEHASHCall,
            N,
        > {
            self.call_builder(
                &OPERATOR_AVS_REGISTRATION_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH`] function.
        pub fn OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall,
            N,
        > {
            self.call_builder(
                &OPERATOR_SET_FORCE_DEREGISTRATION_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`OPERATOR_SET_REGISTRATION_TYPEHASH`] function.
        pub fn OPERATOR_SET_REGISTRATION_TYPEHASH(
            &self,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            OPERATOR_SET_REGISTRATION_TYPEHASHCall,
            N,
        > {
            self.call_builder(
                &OPERATOR_SET_REGISTRATION_TYPEHASHCall {
                },
            )
        }
        ///Creates a new call builder for the [`addStrategiesToOperatorSet`] function.
        pub fn addStrategiesToOperatorSet(
            &self,
            operatorSetId: u32,
            strategies: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesToOperatorSetCall, N> {
            self.call_builder(
                &addStrategiesToOperatorSetCall {
                    operatorSetId,
                    strategies,
                },
            )
        }
        ///Creates a new call builder for the [`avsOperatorStatus`] function.
        pub fn avsOperatorStatus(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsOperatorStatusCall, N> {
            self.call_builder(&avsOperatorStatusCall { _0, _1 })
        }
        ///Creates a new call builder for the [`becomeOperatorSetAVS`] function.
        pub fn becomeOperatorSetAVS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, becomeOperatorSetAVSCall, N> {
            self.call_builder(&becomeOperatorSetAVSCall {})
        }
        ///Creates a new call builder for the [`calculateOperatorAVSRegistrationDigestHash`] function.
        pub fn calculateOperatorAVSRegistrationDigestHash(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateOperatorAVSRegistrationDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateOperatorAVSRegistrationDigestHashCall {
                    operator,
                    avs,
                    salt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`calculateOperatorSetForceDeregistrationTypehash`] function.
        pub fn calculateOperatorSetForceDeregistrationTypehash(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateOperatorSetForceDeregistrationTypehashCall,
            N,
        > {
            self.call_builder(
                &calculateOperatorSetForceDeregistrationTypehashCall {
                    avs,
                    operatorSetIds,
                    salt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`calculateOperatorSetRegistrationDigestHash`] function.
        pub fn calculateOperatorSetRegistrationDigestHash(
            &self,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            salt: alloy::sol_types::private::FixedBytes<32>,
            expiry: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            calculateOperatorSetRegistrationDigestHashCall,
            N,
        > {
            self.call_builder(
                &calculateOperatorSetRegistrationDigestHashCall {
                    avs,
                    operatorSetIds,
                    salt,
                    expiry,
                },
            )
        }
        ///Creates a new call builder for the [`cancelSalt`] function.
        pub fn cancelSalt(
            &self,
            salt: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, cancelSaltCall, N> {
            self.call_builder(&cancelSaltCall { salt })
        }
        ///Creates a new call builder for the [`createOperatorSets`] function.
        pub fn createOperatorSets(
            &self,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, createOperatorSetsCall, N> {
            self.call_builder(
                &createOperatorSetsCall {
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperatorFromAVS`] function.
        pub fn deregisterOperatorFromAVS(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorFromAVSCall, N> {
            self.call_builder(
                &deregisterOperatorFromAVSCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`deregisterOperatorFromOperatorSets`] function.
        pub fn deregisterOperatorFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            deregisterOperatorFromOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &deregisterOperatorFromOperatorSetsCall {
                    operator,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`domainSeparator`] function.
        pub fn domainSeparator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, domainSeparatorCall, N> {
            self.call_builder(&domainSeparatorCall {})
        }
        ///Creates a new call builder for the [`forceDeregisterFromOperatorSets`] function.
        pub fn forceDeregisterFromOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            avs: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            forceDeregisterFromOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &forceDeregisterFromOperatorSetsCall {
                    operator,
                    avs,
                    operatorSetIds,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`getNumOperatorSetsOfOperator`] function.
        pub fn getNumOperatorSetsOfOperator(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getNumOperatorSetsOfOperatorCall, N> {
            self.call_builder(
                &getNumOperatorSetsOfOperatorCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`getNumOperatorsInOperatorSet`] function.
        pub fn getNumOperatorsInOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, getNumOperatorsInOperatorSetCall, N> {
            self.call_builder(
                &getNumOperatorsInOperatorSetCall {
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorSetsOfOperator`] function.
        pub fn getOperatorSetsOfOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            start: alloy::sol_types::private::primitives::aliases::U256,
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorSetsOfOperatorCall, N> {
            self.call_builder(
                &getOperatorSetsOfOperatorCall {
                    operator,
                    start,
                    length,
                },
            )
        }
        ///Creates a new call builder for the [`getOperatorsInOperatorSet`] function.
        pub fn getOperatorsInOperatorSet(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            start: alloy::sol_types::private::primitives::aliases::U256,
            length: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getOperatorsInOperatorSetCall, N> {
            self.call_builder(
                &getOperatorsInOperatorSetCall {
                    operatorSet,
                    start,
                    length,
                },
            )
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
        ///Creates a new call builder for the [`inTotalOperatorSets`] function.
        pub fn inTotalOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, inTotalOperatorSetsCall, N> {
            self.call_builder(
                &inTotalOperatorSetsCall {
                    operator,
                },
            )
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initialOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initialPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initialOwner,
                    _pauserRegistry,
                    initialPausedStatus,
                },
            )
        }
        ///Creates a new call builder for the [`isMember`] function.
        pub fn isMember(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isMemberCall, N> {
            self.call_builder(
                &isMemberCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`isOperatorSet`] function.
        pub fn isOperatorSet(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetCall, N> {
            self.call_builder(&isOperatorSetCall { _0, _1 })
        }
        ///Creates a new call builder for the [`isOperatorSetAVS`] function.
        pub fn isOperatorSetAVS(
            &self,
            _0: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetAVSCall, N> {
            self.call_builder(&isOperatorSetAVSCall { _0 })
        }
        ///Creates a new call builder for the [`isOperatorSetBatch`] function.
        pub fn isOperatorSetBatch(
            &self,
            operatorSets: alloy::sol_types::private::Vec<
                <OperatorSet as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetBatchCall, N> {
            self.call_builder(
                &isOperatorSetBatchCall {
                    operatorSets,
                },
            )
        }
        ///Creates a new call builder for the [`isOperatorSlashable`] function.
        pub fn isOperatorSlashable(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSlashableCall, N> {
            self.call_builder(
                &isOperatorSlashableCall {
                    operator,
                    operatorSet,
                },
            )
        }
        ///Creates a new call builder for the [`migrateOperatorsToOperatorSets`] function.
        pub fn migrateOperatorsToOperatorSets(
            &self,
            operators: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Address,
            >,
            operatorSetIds: alloy::sol_types::private::Vec<
                alloy::sol_types::private::Vec<u32>,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            migrateOperatorsToOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &migrateOperatorsToOperatorSetsCall {
                    operators,
                    operatorSetIds,
                },
            )
        }
        ///Creates a new call builder for the [`operatorSaltIsSpent`] function.
        pub fn operatorSaltIsSpent(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSaltIsSpentCall, N> {
            self.call_builder(&operatorSaltIsSpentCall { _0, _1 })
        }
        ///Creates a new call builder for the [`operatorSetMemberAtIndex`] function.
        pub fn operatorSetMemberAtIndex(
            &self,
            operatorSet: <OperatorSet as alloy::sol_types::SolType>::RustType,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetMemberAtIndexCall, N> {
            self.call_builder(
                &operatorSetMemberAtIndexCall {
                    operatorSet,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`operatorSetStatus`] function.
        pub fn operatorSetStatus(
            &self,
            _0: alloy::sol_types::private::Address,
            _1: alloy::sol_types::private::Address,
            _2: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetStatusCall, N> {
            self.call_builder(
                &operatorSetStatusCall {
                    _0,
                    _1,
                    _2,
                },
            )
        }
        ///Creates a new call builder for the [`operatorSetsMemberOfAtIndex`] function.
        pub fn operatorSetsMemberOfAtIndex(
            &self,
            operator: alloy::sol_types::private::Address,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, operatorSetsMemberOfAtIndexCall, N> {
            self.call_builder(
                &operatorSetsMemberOfAtIndexCall {
                    operator,
                    index,
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
        ///Creates a new call builder for the [`registerOperatorToAVS`] function.
        pub fn registerOperatorToAVS(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorToAVSCall, N> {
            self.call_builder(
                &registerOperatorToAVSCall {
                    operator,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperatorToOperatorSets`] function.
        pub fn registerOperatorToOperatorSets(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorSetIds: alloy::sol_types::private::Vec<u32>,
            operatorSignature: <ISignatureUtils::SignatureWithSaltAndExpiry as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            registerOperatorToOperatorSetsCall,
            N,
        > {
            self.call_builder(
                &registerOperatorToOperatorSetsCall {
                    operator,
                    operatorSetIds,
                    operatorSignature,
                },
            )
        }
        ///Creates a new call builder for the [`removeStrategiesFromOperatorSet`] function.
        pub fn removeStrategiesFromOperatorSet(
            &self,
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
        ///Creates a new call builder for the [`setPauserRegistry`] function.
        pub fn setPauserRegistry(
            &self,
            newPauserRegistry: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setPauserRegistryCall, N> {
            self.call_builder(
                &setPauserRegistryCall {
                    newPauserRegistry,
                },
            )
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
            metadataURI: alloy::sol_types::private::String,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateAVSMetadataURICall, N> {
            self.call_builder(
                &updateAVSMetadataURICall {
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
    > AVSDirectoryInstance<T, P, N> {
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
        ///Creates a new event filter for the [`AVSMigratedToOperatorSets`] event.
        pub fn AVSMigratedToOperatorSets_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, AVSMigratedToOperatorSets, N> {
            self.event_filter::<AVSMigratedToOperatorSets>()
        }
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
        }
        ///Creates a new event filter for the [`OperatorAVSRegistrationStatusUpdated`] event.
        pub fn OperatorAVSRegistrationStatusUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAVSRegistrationStatusUpdated, N> {
            self.event_filter::<OperatorAVSRegistrationStatusUpdated>()
        }
        ///Creates a new event filter for the [`OperatorAddedToOperatorSet`] event.
        pub fn OperatorAddedToOperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorAddedToOperatorSet, N> {
            self.event_filter::<OperatorAddedToOperatorSet>()
        }
        ///Creates a new event filter for the [`OperatorMigratedToOperatorSets`] event.
        pub fn OperatorMigratedToOperatorSets_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorMigratedToOperatorSets, N> {
            self.event_filter::<OperatorMigratedToOperatorSets>()
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
        ///Creates a new event filter for the [`PauserRegistrySet`] event.
        pub fn PauserRegistrySet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, PauserRegistrySet, N> {
            self.event_filter::<PauserRegistrySet>()
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
