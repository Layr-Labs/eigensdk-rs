///Module containing a contract's types and functions.
/**

```solidity
library IDelayedWithdrawalRouter {
    struct DelayedWithdrawal { uint224 amount; uint32 blockCreated; }
    struct UserDelayedWithdrawals { uint256 delayedWithdrawalsCompleted; DelayedWithdrawal[] delayedWithdrawals; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IDelayedWithdrawalRouter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct DelayedWithdrawal { uint224 amount; uint32 blockCreated; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct DelayedWithdrawal {
        pub amount: alloy::sol_types::private::primitives::aliases::U224,
        pub blockCreated: u32,
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
            alloy::sol_types::sol_data::Uint<224>,
            alloy::sol_types::sol_data::Uint<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U224,
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
        impl ::core::convert::From<DelayedWithdrawal> for UnderlyingRustTuple<'_> {
            fn from(value: DelayedWithdrawal) -> Self {
                (value.amount, value.blockCreated)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelayedWithdrawal {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    amount: tuple.0,
                    blockCreated: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for DelayedWithdrawal {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for DelayedWithdrawal {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        224,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockCreated),
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
        impl alloy_sol_types::SolType for DelayedWithdrawal {
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
        impl alloy_sol_types::SolStruct for DelayedWithdrawal {
            const NAME: &'static str = "DelayedWithdrawal";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "DelayedWithdrawal(uint224 amount,uint32 blockCreated)",
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
                        224,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.amount)
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.blockCreated)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for DelayedWithdrawal {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        224,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amount,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.blockCreated,
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
                    224,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amount,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.blockCreated,
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
struct UserDelayedWithdrawals { uint256 delayedWithdrawalsCompleted; DelayedWithdrawal[] delayedWithdrawals; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct UserDelayedWithdrawals {
        pub delayedWithdrawalsCompleted: alloy::sol_types::private::primitives::aliases::U256,
        pub delayedWithdrawals: alloy::sol_types::private::Vec<
            <DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            alloy::sol_types::sol_data::Uint<256>,
            alloy::sol_types::sol_data::Array<DelayedWithdrawal>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U256,
            alloy::sol_types::private::Vec<
                <DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
        impl ::core::convert::From<UserDelayedWithdrawals> for UnderlyingRustTuple<'_> {
            fn from(value: UserDelayedWithdrawals) -> Self {
                (value.delayedWithdrawalsCompleted, value.delayedWithdrawals)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for UserDelayedWithdrawals {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    delayedWithdrawalsCompleted: tuple.0,
                    delayedWithdrawals: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for UserDelayedWithdrawals {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for UserDelayedWithdrawals {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWithdrawalsCompleted,
                    ),
                    <alloy::sol_types::sol_data::Array<
                        DelayedWithdrawal,
                    > as alloy_sol_types::SolType>::tokenize(&self.delayedWithdrawals),
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
        impl alloy_sol_types::SolType for UserDelayedWithdrawals {
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
        impl alloy_sol_types::SolStruct for UserDelayedWithdrawals {
            const NAME: &'static str = "UserDelayedWithdrawals";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "UserDelayedWithdrawals(uint256 delayedWithdrawalsCompleted,DelayedWithdrawal[] delayedWithdrawals)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                let mut components = alloy_sol_types::private::Vec::with_capacity(1);
                components
                    .push(
                        <DelayedWithdrawal as alloy_sol_types::SolStruct>::eip712_root_type(),
                    );
                components
                    .extend(
                        <DelayedWithdrawal as alloy_sol_types::SolStruct>::eip712_components(),
                    );
                components
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWithdrawalsCompleted,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Array<
                        DelayedWithdrawal,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.delayedWithdrawals,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for UserDelayedWithdrawals {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWithdrawalsCompleted,
                    )
                    + <alloy::sol_types::sol_data::Array<
                        DelayedWithdrawal,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.delayedWithdrawals,
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
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWithdrawalsCompleted,
                    out,
                );
                <alloy::sol_types::sol_data::Array<
                    DelayedWithdrawal,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.delayedWithdrawals,
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
    /**Creates a new wrapper around an on-chain [`IDelayedWithdrawalRouter`](self) contract instance.

See the [wrapper's documentation](`IDelayedWithdrawalRouterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IDelayedWithdrawalRouterInstance<T, P, N> {
        IDelayedWithdrawalRouterInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IDelayedWithdrawalRouter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IDelayedWithdrawalRouter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IDelayedWithdrawalRouterInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IDelayedWithdrawalRouterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IDelayedWithdrawalRouterInstance")
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
    > IDelayedWithdrawalRouterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IDelayedWithdrawalRouter`](self) contract instance.

See the [wrapper's documentation](`IDelayedWithdrawalRouterInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IDelayedWithdrawalRouterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IDelayedWithdrawalRouterInstance<T, P, N> {
            IDelayedWithdrawalRouterInstance {
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
    > IDelayedWithdrawalRouterInstance<T, P, N> {
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
    > IDelayedWithdrawalRouterInstance<T, P, N> {
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
library IDelayedWithdrawalRouter {
    struct DelayedWithdrawal {
        uint224 amount;
        uint32 blockCreated;
    }
    struct UserDelayedWithdrawals {
        uint256 delayedWithdrawalsCompleted;
        DelayedWithdrawal[] delayedWithdrawals;
    }
}

interface DelayedWithdrawalRouterMock {
    event DelayedWithdrawalCreated(address podOwner, address recipient, uint256 amount, uint256 index);
    event DelayedWithdrawalsClaimed(address recipient, uint256 amountClaimed, uint256 delayedWithdrawalsCompleted);
    event WithdrawalDelayBlocksSet(uint256 previousValue, uint256 newValue);

    function canClaimDelayedWithdrawal(address user, uint256 index) external view returns (bool);
    function claimDelayedWithdrawals(uint256 maxNumberOfWithdrawalsToClaim) external;
    function claimDelayedWithdrawals(address recipient, uint256 maxNumberOfWithdrawalsToClaim) external;
    function createDelayedWithdrawal(address podOwner, address recipient) external payable;
    function getClaimableUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
    function getUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
    function setWithdrawalDelayBlocks(uint256 newValue) external;
    function userDelayedWithdrawalByIndex(address user, uint256 index) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal memory);
    function userWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.UserDelayedWithdrawals memory);
    function userWithdrawalsLength(address user) external view returns (uint256);
    function withdrawalDelayBlocks() external view returns (uint256);
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "canClaimDelayedWithdrawal",
    "inputs": [
      {
        "name": "user",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "claimDelayedWithdrawals",
    "inputs": [
      {
        "name": "maxNumberOfWithdrawalsToClaim",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "claimDelayedWithdrawals",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "maxNumberOfWithdrawalsToClaim",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "createDelayedWithdrawal",
    "inputs": [
      {
        "name": "podOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "getClaimableUserDelayedWithdrawals",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IDelayedWithdrawalRouter.DelayedWithdrawal[]",
        "components": [
          {
            "name": "amount",
            "type": "uint224",
            "internalType": "uint224"
          },
          {
            "name": "blockCreated",
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
    "name": "getUserDelayedWithdrawals",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IDelayedWithdrawalRouter.DelayedWithdrawal[]",
        "components": [
          {
            "name": "amount",
            "type": "uint224",
            "internalType": "uint224"
          },
          {
            "name": "blockCreated",
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
    "name": "setWithdrawalDelayBlocks",
    "inputs": [
      {
        "name": "newValue",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "userDelayedWithdrawalByIndex",
    "inputs": [
      {
        "name": "user",
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
        "internalType": "struct IDelayedWithdrawalRouter.DelayedWithdrawal",
        "components": [
          {
            "name": "amount",
            "type": "uint224",
            "internalType": "uint224"
          },
          {
            "name": "blockCreated",
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
    "name": "userWithdrawals",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IDelayedWithdrawalRouter.UserDelayedWithdrawals",
        "components": [
          {
            "name": "delayedWithdrawalsCompleted",
            "type": "uint256",
            "internalType": "uint256"
          },
          {
            "name": "delayedWithdrawals",
            "type": "tuple[]",
            "internalType": "struct IDelayedWithdrawalRouter.DelayedWithdrawal[]",
            "components": [
              {
                "name": "amount",
                "type": "uint224",
                "internalType": "uint224"
              },
              {
                "name": "blockCreated",
                "type": "uint32",
                "internalType": "uint32"
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
    "name": "userWithdrawalsLength",
    "inputs": [
      {
        "name": "user",
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
    "name": "withdrawalDelayBlocks",
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
    "type": "event",
    "name": "DelayedWithdrawalCreated",
    "inputs": [
      {
        "name": "podOwner",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "recipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "index",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "DelayedWithdrawalsClaimed",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "amountClaimed",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "delayedWithdrawalsCompleted",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "WithdrawalDelayBlocksSet",
    "inputs": [
      {
        "name": "previousValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "newValue",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
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
pub mod DelayedWithdrawalRouterMock {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b506103e5806100206000396000f3fe60806040526004361061009c5760003560e01c806385594e581161006457806385594e5814610149578063c0db354c14610176578063d44e1b76146100d8578063e4f4f88714610188578063e5db06c0146101a9578063ecb7cb1b146101c457600080fd5b80631f39d87f146100a15780633e1de008146100a15780634d50f9a4146100d857806350f73e7c146100f85780637560889614610116575b600080fd5b3480156100ad57600080fd5b506100c26100bc36600461023c565b50606090565b6040516100cf919061027c565b60405180910390f35b3480156100e457600080fd5b506100f66100f33660046102c9565b50565b005b34801561010457600080fd5b5060005b6040519081526020016100cf565b34801561012257600080fd5b506101396101313660046102e2565b600092915050565b60405190151581526020016100cf565b34801561015557600080fd5b506101696101643660046102e2565b610205565b6040516100cf919061030c565b6100f661018436600461031a565b5050565b34801561019457600080fd5b506101086101a336600461023c565b50600090565b3480156101b557600080fd5b506100f66101843660046102e2565b3480156101d057600080fd5b506101f86101df36600461023c565b5060408051808201909152600081526060602082015290565b6040516100cf919061034d565b60408051808201909152600080825260208201525b92915050565b919050565b80356001600160a01b038116811461022057600080fd5b60006020828403121561024e57600080fd5b61025782610225565b9392505050565b80516001600160e01b0316825260209081015163ffffffff16910152565b602080825282518282018190526000919060409081850190868401855b828110156102bc576102ac84835161025e565b9284019290850190600101610299565b5091979650505050505050565b6000602082840312156102db57600080fd5b5035919050565b600080604083850312156102f557600080fd5b6102fe83610225565b946020939093013593505050565b6040810161021a828461025e565b6000806040838503121561032d57600080fd5b61033683610225565b915061034460208401610225565b90509250929050565b602080825282518282015282810151604080840181905281516060850181905260009392830191849160808701905b808410156103a35761038f82865161025e565b93850193600193909301929082019061037c565b5097965050505050505056fea26469706673582212207509d5a7f91af58cfc6fb165fd9b60632ee074156f86c7eaf0424cff97eff19264736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x03\xE5\x80a\0 `\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c\x85YNX\x11a\0dW\x80c\x85YNX\x14a\x01IW\x80c\xC0\xDB5L\x14a\x01vW\x80c\xD4N\x1Bv\x14a\0\xD8W\x80c\xE4\xF4\xF8\x87\x14a\x01\x88W\x80c\xE5\xDB\x06\xC0\x14a\x01\xA9W\x80c\xEC\xB7\xCB\x1B\x14a\x01\xC4W`\0\x80\xFD[\x80c\x1F9\xD8\x7F\x14a\0\xA1W\x80c>\x1D\xE0\x08\x14a\0\xA1W\x80cMP\xF9\xA4\x14a\0\xD8W\x80cP\xF7>|\x14a\0\xF8W\x80cu`\x88\x96\x14a\x01\x16W[`\0\x80\xFD[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0\xC2a\0\xBC6`\x04a\x02<V[P``\x90V[`@Qa\0\xCF\x91\x90a\x02|V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE4W`\0\x80\xFD[Pa\0\xF6a\0\xF36`\x04a\x02\xC9V[PV[\0[4\x80\x15a\x01\x04W`\0\x80\xFD[P`\0[`@Q\x90\x81R` \x01a\0\xCFV[4\x80\x15a\x01\"W`\0\x80\xFD[Pa\x019a\x0116`\x04a\x02\xE2V[`\0\x92\x91PPV[`@Q\x90\x15\x15\x81R` \x01a\0\xCFV[4\x80\x15a\x01UW`\0\x80\xFD[Pa\x01ia\x01d6`\x04a\x02\xE2V[a\x02\x05V[`@Qa\0\xCF\x91\x90a\x03\x0CV[a\0\xF6a\x01\x846`\x04a\x03\x1AV[PPV[4\x80\x15a\x01\x94W`\0\x80\xFD[Pa\x01\x08a\x01\xA36`\x04a\x02<V[P`\0\x90V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\0\xF6a\x01\x846`\x04a\x02\xE2V[4\x80\x15a\x01\xD0W`\0\x80\xFD[Pa\x01\xF8a\x01\xDF6`\x04a\x02<V[P`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x90V[`@Qa\0\xCF\x91\x90a\x03MV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x92\x91PPV[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02 W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x02NW`\0\x80\xFD[a\x02W\x82a\x02%V[\x93\x92PPPV[\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x02\xBCWa\x02\xAC\x84\x83Qa\x02^V[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x02\x99V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\xDBW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xF5W`\0\x80\xFD[a\x02\xFE\x83a\x02%V[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81\x01a\x02\x1A\x82\x84a\x02^V[`\0\x80`@\x83\x85\x03\x12\x15a\x03-W`\0\x80\xFD[a\x036\x83a\x02%V[\x91Pa\x03D` \x84\x01a\x02%V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R`\0\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a\x03\xA3Wa\x03\x8F\x82\x86Qa\x02^V[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x03|V[P\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 u\t\xD5\xA7\xF9\x1A\xF5\x8C\xFCo\xB1e\xFD\x9B`c.\xE0t\x15o\x86\xC7\xEA\xF0BL\xFF\x97\xEF\xF1\x92dsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061009c5760003560e01c806385594e581161006457806385594e5814610149578063c0db354c14610176578063d44e1b76146100d8578063e4f4f88714610188578063e5db06c0146101a9578063ecb7cb1b146101c457600080fd5b80631f39d87f146100a15780633e1de008146100a15780634d50f9a4146100d857806350f73e7c146100f85780637560889614610116575b600080fd5b3480156100ad57600080fd5b506100c26100bc36600461023c565b50606090565b6040516100cf919061027c565b60405180910390f35b3480156100e457600080fd5b506100f66100f33660046102c9565b50565b005b34801561010457600080fd5b5060005b6040519081526020016100cf565b34801561012257600080fd5b506101396101313660046102e2565b600092915050565b60405190151581526020016100cf565b34801561015557600080fd5b506101696101643660046102e2565b610205565b6040516100cf919061030c565b6100f661018436600461031a565b5050565b34801561019457600080fd5b506101086101a336600461023c565b50600090565b3480156101b557600080fd5b506100f66101843660046102e2565b3480156101d057600080fd5b506101f86101df36600461023c565b5060408051808201909152600081526060602082015290565b6040516100cf919061034d565b60408051808201909152600080825260208201525b92915050565b919050565b80356001600160a01b038116811461022057600080fd5b60006020828403121561024e57600080fd5b61025782610225565b9392505050565b80516001600160e01b0316825260209081015163ffffffff16910152565b602080825282518282018190526000919060409081850190868401855b828110156102bc576102ac84835161025e565b9284019290850190600101610299565b5091979650505050505050565b6000602082840312156102db57600080fd5b5035919050565b600080604083850312156102f557600080fd5b6102fe83610225565b946020939093013593505050565b6040810161021a828461025e565b6000806040838503121561032d57600080fd5b61033683610225565b915061034460208401610225565b90509250929050565b602080825282518282015282810151604080840181905281516060850181905260009392830191849160808701905b808410156103a35761038f82865161025e565b93850193600193909301929082019061037c565b5097965050505050505056fea26469706673582212207509d5a7f91af58cfc6fb165fd9b60632ee074156f86c7eaf0424cff97eff19264736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\0\x9CW`\x005`\xE0\x1C\x80c\x85YNX\x11a\0dW\x80c\x85YNX\x14a\x01IW\x80c\xC0\xDB5L\x14a\x01vW\x80c\xD4N\x1Bv\x14a\0\xD8W\x80c\xE4\xF4\xF8\x87\x14a\x01\x88W\x80c\xE5\xDB\x06\xC0\x14a\x01\xA9W\x80c\xEC\xB7\xCB\x1B\x14a\x01\xC4W`\0\x80\xFD[\x80c\x1F9\xD8\x7F\x14a\0\xA1W\x80c>\x1D\xE0\x08\x14a\0\xA1W\x80cMP\xF9\xA4\x14a\0\xD8W\x80cP\xF7>|\x14a\0\xF8W\x80cu`\x88\x96\x14a\x01\x16W[`\0\x80\xFD[4\x80\x15a\0\xADW`\0\x80\xFD[Pa\0\xC2a\0\xBC6`\x04a\x02<V[P``\x90V[`@Qa\0\xCF\x91\x90a\x02|V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\0\xE4W`\0\x80\xFD[Pa\0\xF6a\0\xF36`\x04a\x02\xC9V[PV[\0[4\x80\x15a\x01\x04W`\0\x80\xFD[P`\0[`@Q\x90\x81R` \x01a\0\xCFV[4\x80\x15a\x01\"W`\0\x80\xFD[Pa\x019a\x0116`\x04a\x02\xE2V[`\0\x92\x91PPV[`@Q\x90\x15\x15\x81R` \x01a\0\xCFV[4\x80\x15a\x01UW`\0\x80\xFD[Pa\x01ia\x01d6`\x04a\x02\xE2V[a\x02\x05V[`@Qa\0\xCF\x91\x90a\x03\x0CV[a\0\xF6a\x01\x846`\x04a\x03\x1AV[PPV[4\x80\x15a\x01\x94W`\0\x80\xFD[Pa\x01\x08a\x01\xA36`\x04a\x02<V[P`\0\x90V[4\x80\x15a\x01\xB5W`\0\x80\xFD[Pa\0\xF6a\x01\x846`\x04a\x02\xE2V[4\x80\x15a\x01\xD0W`\0\x80\xFD[Pa\x01\xF8a\x01\xDF6`\x04a\x02<V[P`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R\x90V[`@Qa\0\xCF\x91\x90a\x03MV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R[\x92\x91PPV[\x91\x90PV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x02 W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x02NW`\0\x80\xFD[a\x02W\x82a\x02%V[\x93\x92PPPV[\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x02\xBCWa\x02\xAC\x84\x83Qa\x02^V[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x02\x99V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x02\xDBW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x02\xF5W`\0\x80\xFD[a\x02\xFE\x83a\x02%V[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81\x01a\x02\x1A\x82\x84a\x02^V[`\0\x80`@\x83\x85\x03\x12\x15a\x03-W`\0\x80\xFD[a\x036\x83a\x02%V[\x91Pa\x03D` \x84\x01a\x02%V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R`\0\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a\x03\xA3Wa\x03\x8F\x82\x86Qa\x02^V[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x03|V[P\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 u\t\xD5\xA7\xF9\x1A\xF5\x8C\xFCo\xB1e\xFD\x9B`c.\xE0t\x15o\x86\xC7\xEA\xF0BL\xFF\x97\xEF\xF1\x92dsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `DelayedWithdrawalCreated(address,address,uint256,uint256)` and selector `0xb8f1b14c7caf74150801dcc9bc18d575cbeaf5b421943497e409df92c92e0f59`.
```solidity
event DelayedWithdrawalCreated(address podOwner, address recipient, uint256 amount, uint256 index);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelayedWithdrawalCreated {
        #[allow(missing_docs)]
        pub podOwner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub index: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DelayedWithdrawalCreated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DelayedWithdrawalCreated(address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                184u8,
                241u8,
                177u8,
                76u8,
                124u8,
                175u8,
                116u8,
                21u8,
                8u8,
                1u8,
                220u8,
                201u8,
                188u8,
                24u8,
                213u8,
                117u8,
                203u8,
                234u8,
                245u8,
                180u8,
                33u8,
                148u8,
                52u8,
                151u8,
                228u8,
                9u8,
                223u8,
                146u8,
                201u8,
                46u8,
                15u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    podOwner: data.0,
                    recipient: data.1,
                    amount: data.2,
                    index: data.3,
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
                        &self.podOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.index),
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
        impl alloy_sol_types::private::IntoLogData for DelayedWithdrawalCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelayedWithdrawalCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DelayedWithdrawalCreated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `DelayedWithdrawalsClaimed(address,uint256,uint256)` and selector `0x6b7151500bd0b5cc211bcc47b3029831b769004df4549e8e1c9a69da05bb0943`.
```solidity
event DelayedWithdrawalsClaimed(address recipient, uint256 amountClaimed, uint256 delayedWithdrawalsCompleted);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct DelayedWithdrawalsClaimed {
        #[allow(missing_docs)]
        pub recipient: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amountClaimed: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub delayedWithdrawalsCompleted: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for DelayedWithdrawalsClaimed {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "DelayedWithdrawalsClaimed(address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                107u8,
                113u8,
                81u8,
                80u8,
                11u8,
                208u8,
                181u8,
                204u8,
                33u8,
                27u8,
                204u8,
                71u8,
                179u8,
                2u8,
                152u8,
                49u8,
                183u8,
                105u8,
                0u8,
                77u8,
                244u8,
                84u8,
                158u8,
                142u8,
                28u8,
                154u8,
                105u8,
                218u8,
                5u8,
                187u8,
                9u8,
                67u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    recipient: data.0,
                    amountClaimed: data.1,
                    delayedWithdrawalsCompleted: data.2,
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
                        &self.recipient,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountClaimed),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.delayedWithdrawalsCompleted,
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
        impl alloy_sol_types::private::IntoLogData for DelayedWithdrawalsClaimed {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&DelayedWithdrawalsClaimed> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &DelayedWithdrawalsClaimed,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `WithdrawalDelayBlocksSet(uint256,uint256)` and selector `0x4ffb00400574147429ee377a5633386321e66d45d8b14676014b5fa393e61e9e`.
```solidity
event WithdrawalDelayBlocksSet(uint256 previousValue, uint256 newValue);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct WithdrawalDelayBlocksSet {
        #[allow(missing_docs)]
        pub previousValue: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub newValue: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for WithdrawalDelayBlocksSet {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "WithdrawalDelayBlocksSet(uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                79u8,
                251u8,
                0u8,
                64u8,
                5u8,
                116u8,
                20u8,
                116u8,
                41u8,
                238u8,
                55u8,
                122u8,
                86u8,
                51u8,
                56u8,
                99u8,
                33u8,
                230u8,
                109u8,
                69u8,
                216u8,
                177u8,
                70u8,
                118u8,
                1u8,
                75u8,
                95u8,
                163u8,
                147u8,
                230u8,
                30u8,
                158u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    previousValue: data.0,
                    newValue: data.1,
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
                    > as alloy_sol_types::SolType>::tokenize(&self.previousValue),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.newValue),
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
        impl alloy_sol_types::private::IntoLogData for WithdrawalDelayBlocksSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&WithdrawalDelayBlocksSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &WithdrawalDelayBlocksSet,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `canClaimDelayedWithdrawal(address,uint256)` and selector `0x75608896`.
```solidity
function canClaimDelayedWithdrawal(address user, uint256 index) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canClaimDelayedWithdrawalCall {
        pub user: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`canClaimDelayedWithdrawal(address,uint256)`](canClaimDelayedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct canClaimDelayedWithdrawalReturn {
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
            impl ::core::convert::From<canClaimDelayedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: canClaimDelayedWithdrawalCall) -> Self {
                    (value.user, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for canClaimDelayedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        user: tuple.0,
                        index: tuple.1,
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
            impl ::core::convert::From<canClaimDelayedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: canClaimDelayedWithdrawalReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for canClaimDelayedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for canClaimDelayedWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = canClaimDelayedWithdrawalReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "canClaimDelayedWithdrawal(address,uint256)";
            const SELECTOR: [u8; 4] = [117u8, 96u8, 136u8, 150u8];
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
                        &self.user,
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
    /**Function with signature `claimDelayedWithdrawals(uint256)` and selector `0xd44e1b76`.
```solidity
function claimDelayedWithdrawals(uint256 maxNumberOfWithdrawalsToClaim) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_0Call {
        pub maxNumberOfWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`claimDelayedWithdrawals(uint256)`](claimDelayedWithdrawals_0Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_0Return {}
    #[allow(
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
            impl ::core::convert::From<claimDelayedWithdrawals_0Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimDelayedWithdrawals_0Call) -> Self {
                    (value.maxNumberOfWithdrawalsToClaim,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxNumberOfWithdrawalsToClaim: tuple.0,
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
            impl ::core::convert::From<claimDelayedWithdrawals_0Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimDelayedWithdrawals_0Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDelayedWithdrawals_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimDelayedWithdrawals_0Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimDelayedWithdrawals(uint256)";
            const SELECTOR: [u8; 4] = [212u8, 78u8, 27u8, 118u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.maxNumberOfWithdrawalsToClaim,
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
    /**Function with signature `claimDelayedWithdrawals(address,uint256)` and selector `0xe5db06c0`.
```solidity
function claimDelayedWithdrawals(address recipient, uint256 maxNumberOfWithdrawalsToClaim) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_1Call {
        pub recipient: alloy::sol_types::private::Address,
        pub maxNumberOfWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`claimDelayedWithdrawals(address,uint256)`](claimDelayedWithdrawals_1Call) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_1Return {}
    #[allow(
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
            impl ::core::convert::From<claimDelayedWithdrawals_1Call>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimDelayedWithdrawals_1Call) -> Self {
                    (value.recipient, value.maxNumberOfWithdrawalsToClaim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        maxNumberOfWithdrawalsToClaim: tuple.1,
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
            impl ::core::convert::From<claimDelayedWithdrawals_1Return>
            for UnderlyingRustTuple<'_> {
                fn from(value: claimDelayedWithdrawals_1Return) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for claimDelayedWithdrawals_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = claimDelayedWithdrawals_1Return;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "claimDelayedWithdrawals(address,uint256)";
            const SELECTOR: [u8; 4] = [229u8, 219u8, 6u8, 192u8];
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
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.maxNumberOfWithdrawalsToClaim,
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
    /**Function with signature `createDelayedWithdrawal(address,address)` and selector `0xc0db354c`.
```solidity
function createDelayedWithdrawal(address podOwner, address recipient) external payable;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createDelayedWithdrawalCall {
        pub podOwner: alloy::sol_types::private::Address,
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`createDelayedWithdrawal(address,address)`](createDelayedWithdrawalCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct createDelayedWithdrawalReturn {}
    #[allow(
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
            impl ::core::convert::From<createDelayedWithdrawalCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: createDelayedWithdrawalCall) -> Self {
                    (value.podOwner, value.recipient)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createDelayedWithdrawalCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        podOwner: tuple.0,
                        recipient: tuple.1,
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
            impl ::core::convert::From<createDelayedWithdrawalReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: createDelayedWithdrawalReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for createDelayedWithdrawalReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for createDelayedWithdrawalCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = createDelayedWithdrawalReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "createDelayedWithdrawal(address,address)";
            const SELECTOR: [u8; 4] = [192u8, 219u8, 53u8, 76u8];
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
                        &self.podOwner,
                    ),
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
    /**Function with signature `getClaimableUserDelayedWithdrawals(address)` and selector `0x1f39d87f`.
```solidity
function getClaimableUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getClaimableUserDelayedWithdrawalsCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getClaimableUserDelayedWithdrawals(address)`](getClaimableUserDelayedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getClaimableUserDelayedWithdrawalsReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getClaimableUserDelayedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getClaimableUserDelayedWithdrawalsCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getClaimableUserDelayedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelayedWithdrawalRouter::DelayedWithdrawal,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getClaimableUserDelayedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getClaimableUserDelayedWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getClaimableUserDelayedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getClaimableUserDelayedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getClaimableUserDelayedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelayedWithdrawalRouter::DelayedWithdrawal,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getClaimableUserDelayedWithdrawals(address)";
            const SELECTOR: [u8; 4] = [31u8, 57u8, 216u8, 127u8];
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
                        &self.user,
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
    /**Function with signature `getUserDelayedWithdrawals(address)` and selector `0x3e1de008`.
```solidity
function getUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDelayedWithdrawalsCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`getUserDelayedWithdrawals(address)`](getUserDelayedWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getUserDelayedWithdrawalsReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getUserDelayedWithdrawalsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDelayedWithdrawalsCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDelayedWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelayedWithdrawalRouter::DelayedWithdrawal,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getUserDelayedWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getUserDelayedWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getUserDelayedWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getUserDelayedWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getUserDelayedWithdrawalsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    IDelayedWithdrawalRouter::DelayedWithdrawal,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getUserDelayedWithdrawals(address)";
            const SELECTOR: [u8; 4] = [62u8, 29u8, 224u8, 8u8];
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
                        &self.user,
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
    /**Function with signature `setWithdrawalDelayBlocks(uint256)` and selector `0x4d50f9a4`.
```solidity
function setWithdrawalDelayBlocks(uint256 newValue) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setWithdrawalDelayBlocksCall {
        pub newValue: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`setWithdrawalDelayBlocks(uint256)`](setWithdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setWithdrawalDelayBlocksReturn {}
    #[allow(
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
            impl ::core::convert::From<setWithdrawalDelayBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setWithdrawalDelayBlocksCall) -> Self {
                    (value.newValue,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setWithdrawalDelayBlocksCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newValue: tuple.0 }
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
            impl ::core::convert::From<setWithdrawalDelayBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setWithdrawalDelayBlocksReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setWithdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setWithdrawalDelayBlocksCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setWithdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setWithdrawalDelayBlocks(uint256)";
            const SELECTOR: [u8; 4] = [77u8, 80u8, 249u8, 164u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.newValue),
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
    /**Function with signature `userDelayedWithdrawalByIndex(address,uint256)` and selector `0x85594e58`.
```solidity
function userDelayedWithdrawalByIndex(address user, uint256 index) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDelayedWithdrawalByIndexCall {
        pub user: alloy::sol_types::private::Address,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`userDelayedWithdrawalByIndex(address,uint256)`](userDelayedWithdrawalByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userDelayedWithdrawalByIndexReturn {
        pub _0: <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<userDelayedWithdrawalByIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDelayedWithdrawalByIndexCall) -> Self {
                    (value.user, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDelayedWithdrawalByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        user: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IDelayedWithdrawalRouter::DelayedWithdrawal,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelayedWithdrawalRouter::DelayedWithdrawal as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<userDelayedWithdrawalByIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userDelayedWithdrawalByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userDelayedWithdrawalByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userDelayedWithdrawalByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userDelayedWithdrawalByIndexReturn;
            type ReturnTuple<'a> = (IDelayedWithdrawalRouter::DelayedWithdrawal,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userDelayedWithdrawalByIndex(address,uint256)";
            const SELECTOR: [u8; 4] = [133u8, 89u8, 78u8, 88u8];
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
                        &self.user,
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
    /**Function with signature `userWithdrawals(address)` and selector `0xecb7cb1b`.
```solidity
function userWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.UserDelayedWithdrawals memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userWithdrawalsCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`userWithdrawals(address)`](userWithdrawalsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userWithdrawalsReturn {
        pub _0: <IDelayedWithdrawalRouter::UserDelayedWithdrawals as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<userWithdrawalsCall> for UnderlyingRustTuple<'_> {
                fn from(value: userWithdrawalsCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for userWithdrawalsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                IDelayedWithdrawalRouter::UserDelayedWithdrawals,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IDelayedWithdrawalRouter::UserDelayedWithdrawals as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<userWithdrawalsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userWithdrawalsReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userWithdrawalsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userWithdrawalsCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userWithdrawalsReturn;
            type ReturnTuple<'a> = (IDelayedWithdrawalRouter::UserDelayedWithdrawals,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userWithdrawals(address)";
            const SELECTOR: [u8; 4] = [236u8, 183u8, 203u8, 27u8];
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
                        &self.user,
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
    /**Function with signature `userWithdrawalsLength(address)` and selector `0xe4f4f887`.
```solidity
function userWithdrawalsLength(address user) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userWithdrawalsLengthCall {
        pub user: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`userWithdrawalsLength(address)`](userWithdrawalsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct userWithdrawalsLengthReturn {
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
            impl ::core::convert::From<userWithdrawalsLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: userWithdrawalsLengthCall) -> Self {
                    (value.user,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userWithdrawalsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { user: tuple.0 }
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
            impl ::core::convert::From<userWithdrawalsLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: userWithdrawalsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for userWithdrawalsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for userWithdrawalsLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = userWithdrawalsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "userWithdrawalsLength(address)";
            const SELECTOR: [u8; 4] = [228u8, 244u8, 248u8, 135u8];
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
                        &self.user,
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
    /**Function with signature `withdrawalDelayBlocks()` and selector `0x50f73e7c`.
```solidity
function withdrawalDelayBlocks() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawalDelayBlocksCall {}
    ///Container type for the return parameters of the [`withdrawalDelayBlocks()`](withdrawalDelayBlocksCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct withdrawalDelayBlocksReturn {
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
            impl ::core::convert::From<withdrawalDelayBlocksCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawalDelayBlocksCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawalDelayBlocksCall {
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
            impl ::core::convert::From<withdrawalDelayBlocksReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: withdrawalDelayBlocksReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for withdrawalDelayBlocksReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for withdrawalDelayBlocksCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = withdrawalDelayBlocksReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "withdrawalDelayBlocks()";
            const SELECTOR: [u8; 4] = [80u8, 247u8, 62u8, 124u8];
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
    ///Container for all the [`DelayedWithdrawalRouterMock`](self) function calls.
    pub enum DelayedWithdrawalRouterMockCalls {
        canClaimDelayedWithdrawal(canClaimDelayedWithdrawalCall),
        claimDelayedWithdrawals_0(claimDelayedWithdrawals_0Call),
        claimDelayedWithdrawals_1(claimDelayedWithdrawals_1Call),
        createDelayedWithdrawal(createDelayedWithdrawalCall),
        getClaimableUserDelayedWithdrawals(getClaimableUserDelayedWithdrawalsCall),
        getUserDelayedWithdrawals(getUserDelayedWithdrawalsCall),
        setWithdrawalDelayBlocks(setWithdrawalDelayBlocksCall),
        userDelayedWithdrawalByIndex(userDelayedWithdrawalByIndexCall),
        userWithdrawals(userWithdrawalsCall),
        userWithdrawalsLength(userWithdrawalsLengthCall),
        withdrawalDelayBlocks(withdrawalDelayBlocksCall),
    }
    #[automatically_derived]
    impl DelayedWithdrawalRouterMockCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [31u8, 57u8, 216u8, 127u8],
            [62u8, 29u8, 224u8, 8u8],
            [77u8, 80u8, 249u8, 164u8],
            [80u8, 247u8, 62u8, 124u8],
            [117u8, 96u8, 136u8, 150u8],
            [133u8, 89u8, 78u8, 88u8],
            [192u8, 219u8, 53u8, 76u8],
            [212u8, 78u8, 27u8, 118u8],
            [228u8, 244u8, 248u8, 135u8],
            [229u8, 219u8, 6u8, 192u8],
            [236u8, 183u8, 203u8, 27u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelayedWithdrawalRouterMockCalls {
        const NAME: &'static str = "DelayedWithdrawalRouterMockCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 11usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::canClaimDelayedWithdrawal(_) => {
                    <canClaimDelayedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimDelayedWithdrawals_0(_) => {
                    <claimDelayedWithdrawals_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::claimDelayedWithdrawals_1(_) => {
                    <claimDelayedWithdrawals_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::createDelayedWithdrawal(_) => {
                    <createDelayedWithdrawalCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getClaimableUserDelayedWithdrawals(_) => {
                    <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserDelayedWithdrawals(_) => {
                    <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setWithdrawalDelayBlocks(_) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userDelayedWithdrawalByIndex(_) => {
                    <userDelayedWithdrawalByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userWithdrawals(_) => {
                    <userWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::userWithdrawalsLength(_) => {
                    <userWithdrawalsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::withdrawalDelayBlocks(_) => {
                    <withdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls>] = &[
                {
                    fn getClaimableUserDelayedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::getClaimableUserDelayedWithdrawals,
                            )
                    }
                    getClaimableUserDelayedWithdrawals
                },
                {
                    fn getUserDelayedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::getUserDelayedWithdrawals,
                            )
                    }
                    getUserDelayedWithdrawals
                },
                {
                    fn setWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::setWithdrawalDelayBlocks,
                            )
                    }
                    setWithdrawalDelayBlocks
                },
                {
                    fn withdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <withdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterMockCalls::withdrawalDelayBlocks)
                    }
                    withdrawalDelayBlocks
                },
                {
                    fn canClaimDelayedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <canClaimDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::canClaimDelayedWithdrawal,
                            )
                    }
                    canClaimDelayedWithdrawal
                },
                {
                    fn userDelayedWithdrawalByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <userDelayedWithdrawalByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::userDelayedWithdrawalByIndex,
                            )
                    }
                    userDelayedWithdrawalByIndex
                },
                {
                    fn createDelayedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <createDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::createDelayedWithdrawal,
                            )
                    }
                    createDelayedWithdrawal
                },
                {
                    fn claimDelayedWithdrawals_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <claimDelayedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::claimDelayedWithdrawals_0,
                            )
                    }
                    claimDelayedWithdrawals_0
                },
                {
                    fn userWithdrawalsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <userWithdrawalsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterMockCalls::userWithdrawalsLength)
                    }
                    userWithdrawalsLength
                },
                {
                    fn claimDelayedWithdrawals_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <claimDelayedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterMockCalls::claimDelayedWithdrawals_1,
                            )
                    }
                    claimDelayedWithdrawals_1
                },
                {
                    fn userWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterMockCalls> {
                        <userWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterMockCalls::userWithdrawals)
                    }
                    userWithdrawals
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
                Self::canClaimDelayedWithdrawal(inner) => {
                    <canClaimDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimDelayedWithdrawals_0(inner) => {
                    <claimDelayedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::claimDelayedWithdrawals_1(inner) => {
                    <claimDelayedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::createDelayedWithdrawal(inner) => {
                    <createDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getClaimableUserDelayedWithdrawals(inner) => {
                    <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getUserDelayedWithdrawals(inner) => {
                    <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setWithdrawalDelayBlocks(inner) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userDelayedWithdrawalByIndex(inner) => {
                    <userDelayedWithdrawalByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userWithdrawals(inner) => {
                    <userWithdrawalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::userWithdrawalsLength(inner) => {
                    <userWithdrawalsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::withdrawalDelayBlocks(inner) => {
                    <withdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::canClaimDelayedWithdrawal(inner) => {
                    <canClaimDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimDelayedWithdrawals_0(inner) => {
                    <claimDelayedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::claimDelayedWithdrawals_1(inner) => {
                    <claimDelayedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::createDelayedWithdrawal(inner) => {
                    <createDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getClaimableUserDelayedWithdrawals(inner) => {
                    <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getUserDelayedWithdrawals(inner) => {
                    <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setWithdrawalDelayBlocks(inner) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userDelayedWithdrawalByIndex(inner) => {
                    <userDelayedWithdrawalByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userWithdrawals(inner) => {
                    <userWithdrawalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::userWithdrawalsLength(inner) => {
                    <userWithdrawalsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::withdrawalDelayBlocks(inner) => {
                    <withdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`DelayedWithdrawalRouterMock`](self) events.
    pub enum DelayedWithdrawalRouterMockEvents {
        DelayedWithdrawalCreated(DelayedWithdrawalCreated),
        DelayedWithdrawalsClaimed(DelayedWithdrawalsClaimed),
        WithdrawalDelayBlocksSet(WithdrawalDelayBlocksSet),
    }
    #[automatically_derived]
    impl DelayedWithdrawalRouterMockEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                79u8,
                251u8,
                0u8,
                64u8,
                5u8,
                116u8,
                20u8,
                116u8,
                41u8,
                238u8,
                55u8,
                122u8,
                86u8,
                51u8,
                56u8,
                99u8,
                33u8,
                230u8,
                109u8,
                69u8,
                216u8,
                177u8,
                70u8,
                118u8,
                1u8,
                75u8,
                95u8,
                163u8,
                147u8,
                230u8,
                30u8,
                158u8,
            ],
            [
                107u8,
                113u8,
                81u8,
                80u8,
                11u8,
                208u8,
                181u8,
                204u8,
                33u8,
                27u8,
                204u8,
                71u8,
                179u8,
                2u8,
                152u8,
                49u8,
                183u8,
                105u8,
                0u8,
                77u8,
                244u8,
                84u8,
                158u8,
                142u8,
                28u8,
                154u8,
                105u8,
                218u8,
                5u8,
                187u8,
                9u8,
                67u8,
            ],
            [
                184u8,
                241u8,
                177u8,
                76u8,
                124u8,
                175u8,
                116u8,
                21u8,
                8u8,
                1u8,
                220u8,
                201u8,
                188u8,
                24u8,
                213u8,
                117u8,
                203u8,
                234u8,
                245u8,
                180u8,
                33u8,
                148u8,
                52u8,
                151u8,
                228u8,
                9u8,
                223u8,
                146u8,
                201u8,
                46u8,
                15u8,
                89u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for DelayedWithdrawalRouterMockEvents {
        const NAME: &'static str = "DelayedWithdrawalRouterMockEvents";
        const COUNT: usize = 3usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <DelayedWithdrawalCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelayedWithdrawalCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelayedWithdrawalCreated)
                }
                Some(
                    <DelayedWithdrawalsClaimed as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <DelayedWithdrawalsClaimed as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::DelayedWithdrawalsClaimed)
                }
                Some(
                    <WithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <WithdrawalDelayBlocksSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::WithdrawalDelayBlocksSet)
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
    impl alloy_sol_types::private::IntoLogData for DelayedWithdrawalRouterMockEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelayedWithdrawalCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelayedWithdrawalsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::WithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelayedWithdrawalCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::DelayedWithdrawalsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelayedWithdrawalRouterMock`](self) contract instance.

See the [wrapper's documentation](`DelayedWithdrawalRouterMockInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelayedWithdrawalRouterMockInstance<T, P, N> {
        DelayedWithdrawalRouterMockInstance::<T, P, N>::new(address, provider)
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
        Output = alloy_contract::Result<DelayedWithdrawalRouterMockInstance<T, P, N>>,
    > {
        DelayedWithdrawalRouterMockInstance::<T, P, N>::deploy(provider)
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
        DelayedWithdrawalRouterMockInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`DelayedWithdrawalRouterMock`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DelayedWithdrawalRouterMock`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelayedWithdrawalRouterMockInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelayedWithdrawalRouterMockInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelayedWithdrawalRouterMockInstance")
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
    > DelayedWithdrawalRouterMockInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DelayedWithdrawalRouterMock`](self) contract instance.

See the [wrapper's documentation](`DelayedWithdrawalRouterMockInstance`) for more details.*/
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
        ) -> alloy_contract::Result<DelayedWithdrawalRouterMockInstance<T, P, N>> {
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
    impl<T, P: ::core::clone::Clone, N> DelayedWithdrawalRouterMockInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(
            self,
        ) -> DelayedWithdrawalRouterMockInstance<T, P, N> {
            DelayedWithdrawalRouterMockInstance {
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
    > DelayedWithdrawalRouterMockInstance<T, P, N> {
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
        ///Creates a new call builder for the [`canClaimDelayedWithdrawal`] function.
        pub fn canClaimDelayedWithdrawal(
            &self,
            user: alloy::sol_types::private::Address,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, canClaimDelayedWithdrawalCall, N> {
            self.call_builder(
                &canClaimDelayedWithdrawalCall {
                    user,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`claimDelayedWithdrawals_0`] function.
        pub fn claimDelayedWithdrawals_0(
            &self,
            maxNumberOfWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimDelayedWithdrawals_0Call, N> {
            self.call_builder(
                &claimDelayedWithdrawals_0Call {
                    maxNumberOfWithdrawalsToClaim,
                },
            )
        }
        ///Creates a new call builder for the [`claimDelayedWithdrawals_1`] function.
        pub fn claimDelayedWithdrawals_1(
            &self,
            recipient: alloy::sol_types::private::Address,
            maxNumberOfWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimDelayedWithdrawals_1Call, N> {
            self.call_builder(
                &claimDelayedWithdrawals_1Call {
                    recipient,
                    maxNumberOfWithdrawalsToClaim,
                },
            )
        }
        ///Creates a new call builder for the [`createDelayedWithdrawal`] function.
        pub fn createDelayedWithdrawal(
            &self,
            podOwner: alloy::sol_types::private::Address,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, createDelayedWithdrawalCall, N> {
            self.call_builder(
                &createDelayedWithdrawalCall {
                    podOwner,
                    recipient,
                },
            )
        }
        ///Creates a new call builder for the [`getClaimableUserDelayedWithdrawals`] function.
        pub fn getClaimableUserDelayedWithdrawals(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getClaimableUserDelayedWithdrawalsCall,
            N,
        > {
            self.call_builder(
                &getClaimableUserDelayedWithdrawalsCall {
                    user,
                },
            )
        }
        ///Creates a new call builder for the [`getUserDelayedWithdrawals`] function.
        pub fn getUserDelayedWithdrawals(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, getUserDelayedWithdrawalsCall, N> {
            self.call_builder(
                &getUserDelayedWithdrawalsCall {
                    user,
                },
            )
        }
        ///Creates a new call builder for the [`setWithdrawalDelayBlocks`] function.
        pub fn setWithdrawalDelayBlocks(
            &self,
            newValue: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, setWithdrawalDelayBlocksCall, N> {
            self.call_builder(
                &setWithdrawalDelayBlocksCall {
                    newValue,
                },
            )
        }
        ///Creates a new call builder for the [`userDelayedWithdrawalByIndex`] function.
        pub fn userDelayedWithdrawalByIndex(
            &self,
            user: alloy::sol_types::private::Address,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, userDelayedWithdrawalByIndexCall, N> {
            self.call_builder(
                &userDelayedWithdrawalByIndexCall {
                    user,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`userWithdrawals`] function.
        pub fn userWithdrawals(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, userWithdrawalsCall, N> {
            self.call_builder(&userWithdrawalsCall { user })
        }
        ///Creates a new call builder for the [`userWithdrawalsLength`] function.
        pub fn userWithdrawalsLength(
            &self,
            user: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, userWithdrawalsLengthCall, N> {
            self.call_builder(&userWithdrawalsLengthCall { user })
        }
        ///Creates a new call builder for the [`withdrawalDelayBlocks`] function.
        pub fn withdrawalDelayBlocks(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, withdrawalDelayBlocksCall, N> {
            self.call_builder(&withdrawalDelayBlocksCall {})
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > DelayedWithdrawalRouterMockInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`DelayedWithdrawalCreated`] event.
        pub fn DelayedWithdrawalCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelayedWithdrawalCreated, N> {
            self.event_filter::<DelayedWithdrawalCreated>()
        }
        ///Creates a new event filter for the [`DelayedWithdrawalsClaimed`] event.
        pub fn DelayedWithdrawalsClaimed_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, DelayedWithdrawalsClaimed, N> {
            self.event_filter::<DelayedWithdrawalsClaimed>()
        }
        ///Creates a new event filter for the [`WithdrawalDelayBlocksSet`] event.
        pub fn WithdrawalDelayBlocksSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalDelayBlocksSet, N> {
            self.event_filter::<WithdrawalDelayBlocksSet>()
        }
    }
}
