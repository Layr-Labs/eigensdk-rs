/**

Generated by the following Solidity interface...
```solidity
interface MockAvsContractsParser {
    function IS_SCRIPT() external view returns (bool);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "IS_SCRIPT",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  }
]
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod MockAvsContractsParser {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/mockavscontractsparser.rs
    ///0x608080604052346024576201000162ff00ff19600c541617600c55607d908160298239f35b5f80fdfe60808060405260043610156011575f80fd5b5f3560e01c63f8ccbf47146023575f80fd5b346043575f36600319011260435760209060ff600c5460101c1615158152f35b5f80fdfea264697066735822122038217482404d468dbb7e2e68337173e58c1bb037cfb52e019c1828be16d931cd64736f6c634300081b0033
=======
    ///0x6080604052600c805462ff00ff191662010001179055348015601f575f5ffd5b50608680602b5f395ff3fe6080604052348015600e575f5ffd5b50600436106026575f3560e01c8063f8ccbf4714602a575b5f5ffd5b600c54603c9062010000900460ff1681565b604051901515815260200160405180910390f3fea264697066735822122094e042665602c257b81330da648ead9e86e400a3176872962d51e65f0f7129f464736f6c634300081b0033
>>>>>>> dev:crates/utils/src/deploy/mockavscontractsparser.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/mockavscontractsparser.rs
        b"`\x80\x80`@R4`$Wb\x01\0\x01b\xFF\0\xFF\x19`\x0CT\x16\x17`\x0CU`}\x90\x81`)\x829\xF3[_\x80\xFD\xFE`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Cc\xF8\xCC\xBFG\x14`#W_\x80\xFD[4`CW_6`\x03\x19\x01\x12`CW` \x90`\xFF`\x0CT`\x10\x1C\x16\x15\x15\x81R\xF3[_\x80\xFD\xFE\xA2dipfsX\"\x12 8!t\x82@MF\x8D\xBB~.h3qs\xE5\x8C\x1B\xB07\xCF\xB5.\x01\x9C\x18(\xBE\x16\xD91\xCDdsolcC\0\x08\x1B\x003",
=======
        b"`\x80`@R`\x0C\x80Tb\xFF\0\xFF\x19\x16b\x01\0\x01\x17\x90U4\x80\x15`\x1FW__\xFD[P`\x86\x80`+_9_\xF3\xFE`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x046\x10`&W_5`\xE0\x1C\x80c\xF8\xCC\xBFG\x14`*W[__\xFD[`\x0CT`<\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3\xFE\xA2dipfsX\"\x12 \x94\xE0BfV\x02\xC2W\xB8\x130\xDAd\x8E\xAD\x9E\x86\xE4\0\xA3\x17hr\x96-Q\xE6_\x0Fq)\xF4dsolcC\0\x08\x1B\x003",
>>>>>>> dev:crates/utils/src/deploy/mockavscontractsparser.rs
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
<<<<<<< HEAD:crates/utils/src/mockavscontractsparser.rs
    ///0x60808060405260043610156011575f80fd5b5f3560e01c63f8ccbf47146023575f80fd5b346043575f36600319011260435760209060ff600c5460101c1615158152f35b5f80fdfea264697066735822122038217482404d468dbb7e2e68337173e58c1bb037cfb52e019c1828be16d931cd64736f6c634300081b0033
=======
    ///0x6080604052348015600e575f5ffd5b50600436106026575f3560e01c8063f8ccbf4714602a575b5f5ffd5b600c54603c9062010000900460ff1681565b604051901515815260200160405180910390f3fea264697066735822122094e042665602c257b81330da648ead9e86e400a3176872962d51e65f0f7129f464736f6c634300081b0033
>>>>>>> dev:crates/utils/src/deploy/mockavscontractsparser.rs
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
<<<<<<< HEAD:crates/utils/src/mockavscontractsparser.rs
        b"`\x80\x80`@R`\x046\x10\x15`\x11W_\x80\xFD[_5`\xE0\x1Cc\xF8\xCC\xBFG\x14`#W_\x80\xFD[4`CW_6`\x03\x19\x01\x12`CW` \x90`\xFF`\x0CT`\x10\x1C\x16\x15\x15\x81R\xF3[_\x80\xFD\xFE\xA2dipfsX\"\x12 8!t\x82@MF\x8D\xBB~.h3qs\xE5\x8C\x1B\xB07\xCF\xB5.\x01\x9C\x18(\xBE\x16\xD91\xCDdsolcC\0\x08\x1B\x003",
=======
        b"`\x80`@R4\x80\x15`\x0EW__\xFD[P`\x046\x10`&W_5`\xE0\x1C\x80c\xF8\xCC\xBFG\x14`*W[__\xFD[`\x0CT`<\x90b\x01\0\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01`@Q\x80\x91\x03\x90\xF3\xFE\xA2dipfsX\"\x12 \x94\xE0BfV\x02\xC2W\xB8\x130\xDAd\x8E\xAD\x9E\x86\xE4\0\xA3\x17hr\x96-Q\xE6_\x0Fq)\xF4dsolcC\0\x08\x1B\x003",
>>>>>>> dev:crates/utils/src/deploy/mockavscontractsparser.rs
    );
    /**Function with signature `IS_SCRIPT()` and selector `0xf8ccbf47`.
    ```solidity
    function IS_SCRIPT() external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_SCRIPTCall {}
    ///Container type for the return parameters of the [`IS_SCRIPT()`](IS_SCRIPTCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct IS_SCRIPTReturn {
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
            impl ::core::convert::From<IS_SCRIPTCall> for UnderlyingRustTuple<'_> {
                fn from(value: IS_SCRIPTCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_SCRIPTCall {
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
            impl ::core::convert::From<IS_SCRIPTReturn> for UnderlyingRustTuple<'_> {
                fn from(value: IS_SCRIPTReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for IS_SCRIPTReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for IS_SCRIPTCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = IS_SCRIPTReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "IS_SCRIPT()";
            const SELECTOR: [u8; 4] = [248u8, 204u8, 191u8, 71u8];
            #[inline]
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
    ///Container for all the [`MockAvsContractsParser`](self) function calls.
    pub enum MockAvsContractsParserCalls {
        IS_SCRIPT(IS_SCRIPTCall),
    }
    #[automatically_derived]
    impl MockAvsContractsParserCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[[248u8, 204u8, 191u8, 71u8]];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for MockAvsContractsParserCalls {
        const NAME: &'static str = "MockAvsContractsParserCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 1usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::IS_SCRIPT(_) => <IS_SCRIPTCall as alloy_sol_types::SolCall>::SELECTOR,
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
                -> alloy_sol_types::Result<MockAvsContractsParserCalls>] = &[{
                fn IS_SCRIPT(
                    data: &[u8],
                    validate: bool,
                ) -> alloy_sol_types::Result<MockAvsContractsParserCalls> {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
                        .map(MockAvsContractsParserCalls::IS_SCRIPT)
                }
                IS_SCRIPT
            }];
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
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::IS_SCRIPT(inner) => {
                    <IS_SCRIPTCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`MockAvsContractsParser`](self) contract instance.

    See the [wrapper's documentation](`MockAvsContractsParserInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> MockAvsContractsParserInstance<T, P, N> {
        MockAvsContractsParserInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<MockAvsContractsParserInstance<T, P, N>>,
    > {
        MockAvsContractsParserInstance::<T, P, N>::deploy(provider)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        MockAvsContractsParserInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`MockAvsContractsParser`](self) instance.

    Contains type-safe methods for interacting with an on-chain instance of the
    [`MockAvsContractsParser`](self) contract located at a given `address`, using a given
    provider `P`.

    If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
    documentation on how to provide it), the `deploy` and `deploy_builder` methods can
    be used to deploy a new instance of the contract.

    See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct MockAvsContractsParserInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for MockAvsContractsParserInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("MockAvsContractsParserInstance")
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
        > MockAvsContractsParserInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`MockAvsContractsParser`](self) contract instance.

        See the [wrapper's documentation](`MockAvsContractsParserInstance`) for more details.*/
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
        ) -> alloy_contract::Result<MockAvsContractsParserInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> MockAvsContractsParserInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> MockAvsContractsParserInstance<T, P, N> {
            MockAvsContractsParserInstance {
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
        > MockAvsContractsParserInstance<T, P, N>
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
        ///Creates a new call builder for the [`IS_SCRIPT`] function.
        pub fn IS_SCRIPT(&self) -> alloy_contract::SolCallBuilder<T, &P, IS_SCRIPTCall, N> {
            self.call_builder(&IS_SCRIPTCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > MockAvsContractsParserInstance<T, P, N>
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
