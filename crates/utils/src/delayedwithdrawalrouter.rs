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

interface DelayedWithdrawalRouter {
    event DelayedWithdrawalCreated(address podOwner, address recipient, uint256 amount, uint256 index);
    event DelayedWithdrawalsClaimed(address recipient, uint256 amountClaimed, uint256 delayedWithdrawalsCompleted);
    event Initialized(uint8 version);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
    event Paused(address indexed account, uint256 newPausedStatus);
    event PauserRegistrySet(address pauserRegistry, address newPauserRegistry);
    event Unpaused(address indexed account, uint256 newPausedStatus);
    event WithdrawalDelayBlocksSet(uint256 previousValue, uint256 newValue);

    constructor(address _eigenPodManager);

    function MAX_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
    function canClaimDelayedWithdrawal(address user, uint256 index) external view returns (bool);
    function claimDelayedWithdrawals(uint256 maxNumberOfDelayedWithdrawalsToClaim) external;
    function claimDelayedWithdrawals(address recipient, uint256 maxNumberOfDelayedWithdrawalsToClaim) external;
    function createDelayedWithdrawal(address podOwner, address recipient) external payable;
    function eigenPodManager() external view returns (address);
    function getClaimableUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
    function getUserDelayedWithdrawals(address user) external view returns (IDelayedWithdrawalRouter.DelayedWithdrawal[] memory);
    function initialize(address initOwner, address _pauserRegistry, uint256 initPausedStatus, uint256 _withdrawalDelayBlocks) external;
    function owner() external view returns (address);
    function pause(uint256 newPausedStatus) external;
    function pauseAll() external;
    function paused(uint8 index) external view returns (bool);
    function paused() external view returns (uint256);
    function pauserRegistry() external view returns (address);
    function renounceOwnership() external;
    function setPauserRegistry(address newPauserRegistry) external;
    function setWithdrawalDelayBlocks(uint256 newValue) external;
    function transferOwnership(address newOwner) external;
    function unpause(uint256 newPausedStatus) external;
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
    "type": "constructor",
    "inputs": [
      {
        "name": "_eigenPodManager",
        "type": "address",
        "internalType": "contract IEigenPodManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "MAX_WITHDRAWAL_DELAY_BLOCKS",
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
        "name": "maxNumberOfDelayedWithdrawalsToClaim",
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
        "name": "maxNumberOfDelayedWithdrawalsToClaim",
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
    "name": "initialize",
    "inputs": [
      {
        "name": "initOwner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "_pauserRegistry",
        "type": "address",
        "internalType": "contract IPauserRegistry"
      },
      {
        "name": "initPausedStatus",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "_withdrawalDelayBlocks",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
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
pub mod DelayedWithdrawalRouter {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a06040523480156200001157600080fd5b5060405162001f0e38038062001f0e8339810160408190526200003491620001a8565b6001600160a01b038116620000cb5760405162461bcd60e51b815260206004820152604c60248201527f44656c617965645769746864726177616c526f757465722e636f6e737472756360448201527f746f723a205f656967656e506f644d616e616765722063616e6e6f742062652060648201526b7a65726f206164647265737360a01b608482015260a4015b60405180910390fd5b6001600160a01b038116608052620000e2620000e9565b50620001da565b600054610100900460ff1615620001535760405162461bcd60e51b815260206004820152602760248201527f496e697469616c697a61626c653a20636f6e747261637420697320696e697469604482015266616c697a696e6760c81b6064820152608401620000c2565b60005460ff9081161015620001a6576000805460ff191660ff9081179091556040519081527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b565b600060208284031215620001bb57600080fd5b81516001600160a01b0381168114620001d357600080fd5b9392505050565b608051611d11620001fd600039600081816101fa0152610c000152611d116000f3fe60806040526004361061014b5760003560e01c806385594e58116100b6578063e4f4f8871161006f578063e4f4f887146103cc578063e5db06c014610405578063eb990c5914610425578063ecb7cb1b14610445578063f2fde38b14610472578063fabc1cbc1461049257600080fd5b806385594e5814610317578063886f1195146103445780638da5cb5b14610364578063c0db354c14610382578063ca661c0414610395578063d44e1b76146103ac57600080fd5b806350f73e7c1161010857806350f73e7c14610254578063595c6a67146102785780635ac86ab71461028d5780635c975abb146102cd578063715018a6146102e257806375608896146102f757600080fd5b806310d67a2f14610150578063136439dd146101725780631f39d87f146101925780633e1de008146101c85780634665bcda146101e85780634d50f9a414610234575b600080fd5b34801561015c57600080fd5b5061017061016b36600461196d565b6104b2565b005b34801561017e57600080fd5b5061017061018d366004611991565b61056e565b34801561019e57600080fd5b506101b26101ad36600461196d565b6106ad565b6040516101bf91906119c8565b60405180910390f35b3480156101d457600080fd5b506101b26101e336600461196d565b6108a8565b3480156101f457600080fd5b5061021c7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101bf565b34801561024057600080fd5b5061017061024f366004611991565b6109ee565b34801561026057600080fd5b5061026a60c95481565b6040519081526020016101bf565b34801561028457600080fd5b506101706109ff565b34801561029957600080fd5b506102bd6102a8366004611a15565b609854600160ff9092169190911b9081161490565b60405190151581526020016101bf565b3480156102d957600080fd5b5060985461026a565b3480156102ee57600080fd5b50610170610ac6565b34801561030357600080fd5b506102bd610312366004611a38565b610ada565b34801561032357600080fd5b50610337610332366004611a38565b610b5d565b6040516101bf9190611a64565b34801561035057600080fd5b5060975461021c906001600160a01b031681565b34801561037057600080fd5b506033546001600160a01b031661021c565b610170610390366004611a72565b610bdd565b3480156103a157600080fd5b5061026a62034bc081565b3480156103b857600080fd5b506101706103c7366004611991565b610e9d565b3480156103d857600080fd5b5061026a6103e736600461196d565b6001600160a01b0316600090815260ca602052604090206001015490565b34801561041157600080fd5b50610170610420366004611a38565b610f31565b34801561043157600080fd5b50610170610440366004611aab565b610fc6565b34801561045157600080fd5b5061046561046036600461196d565b6110ee565b6040516101bf9190611af1565b34801561047e57600080fd5b5061017061048d36600461196d565b6111a8565b34801561049e57600080fd5b506101706104ad366004611991565b61121e565b609760009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610505573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105299190611b47565b6001600160a01b0316336001600160a01b0316146105625760405162461bcd60e51b815260040161055990611b64565b60405180910390fd5b61056b8161137a565b50565b60975460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156105b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105da9190611bae565b6105f65760405162461bcd60e51b815260040161055990611bd0565b6098548181161461066f5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610559565b609881905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6001600160a01b038116600090815260ca6020526040812080546001909101546060926106da8383611c2e565b90508060005b82811015610786576001600160a01b038716600090815260ca6020526040812060010161070d8388611c45565b8154811061071d5761071d611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810182905260c95490925061076391611c45565b4310156107735781925050610786565b508061077e81611c73565b9150506106e0565b508060008167ffffffffffffffff8111156107a3576107a3611c8e565b6040519080825280602002602001820160405280156107e857816020015b60408051808201909152600080825260208201528152602001906001900390816107c15790505b509050811561089d5760005b8281101561089b576001600160a01b038916600090815260ca602052604090206001016108218289611c45565b8154811061083157610831611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810191909152825183908390811061087d5761087d611c5d565b6020026020010181905250808061089390611c73565b9150506107f4565b505b979650505050505050565b6001600160a01b038116600090815260ca6020526040812080546001909101546060926108d58383611c2e565b905060008167ffffffffffffffff8111156108f2576108f2611c8e565b60405190808252806020026020018201604052801561093757816020015b60408051808201909152600080825260208201528152602001906001900390816109105790505b50905060005b828110156109e4576001600160a01b038716600090815260ca6020526040902060010161096a8287611c45565b8154811061097a5761097a611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff169181019190915282518390839081106109c6576109c6611c5d565b602002602001018190525080806109dc90611c73565b91505061093d565b5095945050505050565b6109f6611471565b61056b816114cb565b60975460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610a47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a6b9190611bae565b610a875760405162461bcd60e51b815260040161055990611bd0565b600019609881905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b610ace611471565b610ad86000611593565b565b6001600160a01b038216600090815260ca60205260408120548210801590610b54575060c9546001600160a01b038416600090815260ca60205260409020600101805484908110610b2d57610b2d611c5d565b600091825260209091200154610b509190600160e01b900463ffffffff16611c45565b4310155b90505b92915050565b60408051808201909152600080825260208201526001600160a01b038316600090815260ca60205260409020600101805483908110610b9e57610b9e611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff16918101919091529392505050565b60405163a38406a360e01b81526001600160a01b038084166004830152839133917f0000000000000000000000000000000000000000000000000000000000000000169063a38406a390602401602060405180830381865afa158015610c47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c6b9190611b47565b6001600160a01b031614610ce75760405162461bcd60e51b815260206004820152603d60248201527f44656c617965645769746864726177616c526f757465722e6f6e6c794569676560448201527f6e506f643a206e6f7420706f644f776e6572277320456967656e506f640000006064820152608401610559565b60985460009060019081161415610d105760405162461bcd60e51b815260040161055990611ca4565b6001600160a01b038316610da65760405162461bcd60e51b815260206004820152605160248201527f44656c617965645769746864726177616c526f757465722e637265617465446560448201527f6c617965645769746864726177616c3a20726563697069656e742063616e6e6f60648201527074206265207a65726f206164647265737360781b608482015260a401610559565b346001600160e01b03811615610e96576040805180820182526001600160e01b03808416825263ffffffff43811660208085019182526001600160a01b038a16600081815260ca8352968720600190810180548083018255818a5293892088519551909616600160e01b029490961693909317939091019290925593525490917fb8f1b14c7caf74150801dcc9bc18d575cbeaf5b421943497e409df92c92e0f5991889188918691610e5791611c2e565b604080516001600160a01b0395861681529490931660208501526001600160e01b039091169183019190915260608201526080015b60405180910390a1505b5050505050565b60026065541415610ef05760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610559565b600260655560985460009060019081161415610f1e5760405162461bcd60e51b815260040161055990611ca4565b610f2833836115e5565b50506001606555565b60026065541415610f845760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610559565b600260655560985460009060019081161415610fb25760405162461bcd60e51b815260040161055990611ca4565b610fbc83836115e5565b5050600160655550565b600054610100900460ff1615808015610fe65750600054600160ff909116105b806110005750303b158015611000575060005460ff166001145b6110635760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610559565b6000805460ff191660011790558015611086576000805461ff0019166101001790555b61108f85611593565b6110998484611750565b6110a2826114cb565b8015610e96576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050505050565b6040805180820190915260008152606060208201526001600160a01b038216600090815260ca6020908152604080832081518083018352815481526001820180548451818702810187019095528085529195929486810194939192919084015b8282101561119a57600084815260209081902060408051808201909152908401546001600160e01b0381168252600160e01b900463ffffffff168183015282526001909201910161114e565b505050915250909392505050565b6111b0611471565b6001600160a01b0381166112155760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610559565b61056b81611593565b609760009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611271573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112959190611b47565b6001600160a01b0316336001600160a01b0316146112c55760405162461bcd60e51b815260040161055990611b64565b6098541981196098541916146113435760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610559565b609881905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016106a2565b6001600160a01b0381166114085760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610559565b609754604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1609780546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610559565b62034bc08111156115525760405162461bcd60e51b815260206004820152604560248201527f44656c617965645769746864726177616c526f757465722e5f7365745769746860448201527f64726177616c44656c6179426c6f636b733a206e657756616c756520746f6f206064820152646c6172676560d81b608482015260a401610559565b60c95460408051918252602082018390527f4ffb00400574147429ee377a5633386321e66d45d8b14676014b5fa393e61e9e910160405180910390a160c955565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b038216600090815260ca602052604081208054600190910154825b848110801561161e57508161161c8285611c45565b105b156116cb576001600160a01b038616600090815260ca602052604081206001016116488386611c45565b8154811061165857611658611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810182905260c95490925061169e91611c45565b4310156116ab57506116cb565b80516116c0906001600160e01b031686611c45565b945050600101611607565b6116d58184611c45565b6001600160a01b038716600090815260ca602052604090205583156116fe576116fe868561183a565b7f6b7151500bd0b5cc211bcc47b3029831b769004df4549e8e1c9a69da05bb0943868561172b8487611c45565b604080516001600160a01b039094168452602084019290925290820152606001610e8c565b6097546001600160a01b031615801561177157506001600160a01b03821615155b6117f35760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610559565b609881905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26118368261137a565b5050565b8047101561188a5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610559565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146118d7576040519150601f19603f3d011682016040523d82523d6000602084013e6118dc565b606091505b50509050806119535760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610559565b505050565b6001600160a01b038116811461056b57600080fd5b60006020828403121561197f57600080fd5b813561198a81611958565b9392505050565b6000602082840312156119a357600080fd5b5035919050565b80516001600160e01b0316825260209081015163ffffffff16910152565b602080825282518282018190526000919060409081850190868401855b82811015611a08576119f88483516119aa565b92840192908501906001016119e5565b5091979650505050505050565b600060208284031215611a2757600080fd5b813560ff8116811461198a57600080fd5b60008060408385031215611a4b57600080fd5b8235611a5681611958565b946020939093013593505050565b60408101610b5782846119aa565b60008060408385031215611a8557600080fd5b8235611a9081611958565b91506020830135611aa081611958565b809150509250929050565b60008060008060808587031215611ac157600080fd5b8435611acc81611958565b93506020850135611adc81611958565b93969395505050506040820135916060013590565b602080825282518282015282810151604080840181905281516060850181905260009392830191849160808701905b8084101561089b57611b338286516119aa565b938501936001939093019290820190611b20565b600060208284031215611b5957600080fd5b815161198a81611958565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215611bc057600080fd5b8151801515811461198a57600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600082821015611c4057611c40611c18565b500390565b60008219821115611c5857611c58611c18565b500190565b634e487b7160e01b600052603260045260246000fd5b6000600019821415611c8757611c87611c18565b5060010190565b634e487b7160e01b600052604160045260246000fd5b60208082526019908201527f5061757361626c653a20696e646578206973207061757365640000000000000060408201526060019056fea2646970667358221220f6347910fcf3c3989fc8d25677bddba324234c4e1649d2c65fd5268f5add307a64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x1F\x0E8\x03\x80b\0\x1F\x0E\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x01\xA8V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FDelayedWithdrawalRouter.construc`D\x82\x01R\x7Ftor: _eigenPodManager cannot be `d\x82\x01Rkzero address`\xA0\x1B`\x84\x82\x01R`\xA4\x01[`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16`\x80Rb\0\0\xE2b\0\0\xE9V[Pb\0\x01\xDAV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15b\0\x01SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`'`$\x82\x01R\x7FInitializable: contract is initi`D\x82\x01Rfalizing`\xC8\x1B`d\x82\x01R`\x84\x01b\0\0\xC2V[`\0T`\xFF\x90\x81\x16\x10\x15b\0\x01\xA6W`\0\x80T`\xFF\x19\x16`\xFF\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[V[`\0` \x82\x84\x03\x12\x15b\0\x01\xBBW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01\xD3W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1D\x11b\0\x01\xFD`\09`\0\x81\x81a\x01\xFA\x01Ra\x0C\0\x01Ra\x1D\x11`\0\xF3\xFE`\x80`@R`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x85YNX\x11a\0\xB6W\x80c\xE4\xF4\xF8\x87\x11a\0oW\x80c\xE4\xF4\xF8\x87\x14a\x03\xCCW\x80c\xE5\xDB\x06\xC0\x14a\x04\x05W\x80c\xEB\x99\x0CY\x14a\x04%W\x80c\xEC\xB7\xCB\x1B\x14a\x04EW\x80c\xF2\xFD\xE3\x8B\x14a\x04rW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x92W`\0\x80\xFD[\x80c\x85YNX\x14a\x03\x17W\x80c\x88o\x11\x95\x14a\x03DW\x80c\x8D\xA5\xCB[\x14a\x03dW\x80c\xC0\xDB5L\x14a\x03\x82W\x80c\xCAf\x1C\x04\x14a\x03\x95W\x80c\xD4N\x1Bv\x14a\x03\xACW`\0\x80\xFD[\x80cP\xF7>|\x11a\x01\x08W\x80cP\xF7>|\x14a\x02TW\x80cY\\jg\x14a\x02xW\x80cZ\xC8j\xB7\x14a\x02\x8DW\x80c\\\x97Z\xBB\x14a\x02\xCDW\x80cqP\x18\xA6\x14a\x02\xE2W\x80cu`\x88\x96\x14a\x02\xF7W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x01PW\x80c\x13d9\xDD\x14a\x01rW\x80c\x1F9\xD8\x7F\x14a\x01\x92W\x80c>\x1D\xE0\x08\x14a\x01\xC8W\x80cFe\xBC\xDA\x14a\x01\xE8W\x80cMP\xF9\xA4\x14a\x024W[`\0\x80\xFD[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01pa\x01k6`\x04a\x19mV[a\x04\xB2V[\0[4\x80\x15a\x01~W`\0\x80\xFD[Pa\x01pa\x01\x8D6`\x04a\x19\x91V[a\x05nV[4\x80\x15a\x01\x9EW`\0\x80\xFD[Pa\x01\xB2a\x01\xAD6`\x04a\x19mV[a\x06\xADV[`@Qa\x01\xBF\x91\x90a\x19\xC8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD4W`\0\x80\xFD[Pa\x01\xB2a\x01\xE36`\x04a\x19mV[a\x08\xA8V[4\x80\x15a\x01\xF4W`\0\x80\xFD[Pa\x02\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBFV[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x01pa\x02O6`\x04a\x19\x91V[a\t\xEEV[4\x80\x15a\x02`W`\0\x80\xFD[Pa\x02j`\xC9T\x81V[`@Q\x90\x81R` \x01a\x01\xBFV[4\x80\x15a\x02\x84W`\0\x80\xFD[Pa\x01pa\t\xFFV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xBDa\x02\xA86`\x04a\x1A\x15V[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBFV[4\x80\x15a\x02\xD9W`\0\x80\xFD[P`\x98Ta\x02jV[4\x80\x15a\x02\xEEW`\0\x80\xFD[Pa\x01pa\n\xC6V[4\x80\x15a\x03\x03W`\0\x80\xFD[Pa\x02\xBDa\x03\x126`\x04a\x1A8V[a\n\xDAV[4\x80\x15a\x03#W`\0\x80\xFD[Pa\x037a\x0326`\x04a\x1A8V[a\x0B]V[`@Qa\x01\xBF\x91\x90a\x1AdV[4\x80\x15a\x03PW`\0\x80\xFD[P`\x97Ta\x02\x1C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03pW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x1CV[a\x01pa\x03\x906`\x04a\x1ArV[a\x0B\xDDV[4\x80\x15a\x03\xA1W`\0\x80\xFD[Pa\x02jb\x03K\xC0\x81V[4\x80\x15a\x03\xB8W`\0\x80\xFD[Pa\x01pa\x03\xC76`\x04a\x19\x91V[a\x0E\x9DV[4\x80\x15a\x03\xD8W`\0\x80\xFD[Pa\x02ja\x03\xE76`\x04a\x19mV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x01pa\x04 6`\x04a\x1A8V[a\x0F1V[4\x80\x15a\x041W`\0\x80\xFD[Pa\x01pa\x04@6`\x04a\x1A\xABV[a\x0F\xC6V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x04ea\x04`6`\x04a\x19mV[a\x10\xEEV[`@Qa\x01\xBF\x91\x90a\x1A\xF1V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x01pa\x04\x8D6`\x04a\x19mV[a\x11\xA8V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x01pa\x04\xAD6`\x04a\x19\x91V[a\x12\x1EV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05)\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1BdV[`@Q\x80\x91\x03\x90\xFD[a\x05k\x81a\x13zV[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xDA\x91\x90a\x1B\xAEV[a\x05\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1B\xD0V[`\x98T\x81\x81\x16\x14a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T``\x92a\x06\xDA\x83\x83a\x1C.V[\x90P\x80`\0[\x82\x81\x10\x15a\x07\x86W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x81 `\x01\x01a\x07\r\x83\x88a\x1CEV[\x81T\x81\x10a\x07\x1DWa\x07\x1Da\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R`\xC9T\x90\x92Pa\x07c\x91a\x1CEV[C\x10\x15a\x07sW\x81\x92PPa\x07\x86V[P\x80a\x07~\x81a\x1CsV[\x91PPa\x06\xE0V[P\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA3Wa\x07\xA3a\x1C\x8EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xE8W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xC1W\x90P[P\x90P\x81\x15a\x08\x9DW`\0[\x82\x81\x10\x15a\x08\x9BW`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01a\x08!\x82\x89a\x1CEV[\x81T\x81\x10a\x081Wa\x081a\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90\x83\x90\x81\x10a\x08}Wa\x08}a\x1C]V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x08\x93\x90a\x1CsV[\x91PPa\x07\xF4V[P[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T``\x92a\x08\xD5\x83\x83a\x1C.V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF2Wa\x08\xF2a\x1C\x8EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t7W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\x10W\x90P[P\x90P`\0[\x82\x81\x10\x15a\t\xE4W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01a\tj\x82\x87a\x1CEV[\x81T\x81\x10a\tzWa\tza\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90\x83\x90\x81\x10a\t\xC6Wa\t\xC6a\x1C]V[` \x02` \x01\x01\x81\x90RP\x80\x80a\t\xDC\x90a\x1CsV[\x91PPa\t=V[P\x95\x94PPPPPV[a\t\xF6a\x14qV[a\x05k\x81a\x14\xCBV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nk\x91\x90a\x1B\xAEV[a\n\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1B\xD0V[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n\xCEa\x14qV[a\n\xD8`\0a\x15\x93V[V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` R`@\x81 T\x82\x10\x80\x15\x90a\x0BTWP`\xC9T`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01\x80T\x84\x90\x81\x10a\x0B-Wa\x0B-a\x1C]V[`\0\x91\x82R` \x90\x91 \x01Ta\x0BP\x91\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1CEV[C\x10\x15[\x90P[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01\x80T\x83\x90\x81\x10a\x0B\x9EWa\x0B\x9Ea\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@Qc\xA3\x84\x06\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x83\x913\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\x84\x06\xA3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ck\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelayedWithdrawalRouter.onlyEige`D\x82\x01R\x7FnPod: not podOwner's EigenPod\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\r\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FDelayedWithdrawalRouter.createDe`D\x82\x01R\x7FlayedWithdrawal: recipient canno`d\x82\x01Rpt be zero address`x\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[4`\x01`\x01`\xE0\x1B\x03\x81\x16\x15a\x0E\x96W`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xE0\x1B\x03\x80\x84\x16\x82Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x85\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`\xCA\x83R\x96\x87 `\x01\x90\x81\x01\x80T\x80\x83\x01\x82U\x81\x8AR\x93\x89 \x88Q\x95Q\x90\x96\x16`\x01`\xE0\x1B\x02\x94\x90\x96\x16\x93\x90\x93\x17\x93\x90\x91\x01\x92\x90\x92U\x93RT\x90\x91\x7F\xB8\xF1\xB1L|\xAFt\x15\x08\x01\xDC\xC9\xBC\x18\xD5u\xCB\xEA\xF5\xB4!\x944\x97\xE4\t\xDF\x92\xC9.\x0FY\x91\x88\x91\x88\x91\x86\x91a\x0EW\x91a\x1C.V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xA1P[PPPPPV[`\x02`eT\x14\x15a\x0E\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x05YV[`\x02`eU`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\x0F\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[a\x0F(3\x83a\x15\xE5V[PP`\x01`eUV[`\x02`eT\x14\x15a\x0F\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x05YV[`\x02`eU`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\x0F\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[a\x0F\xBC\x83\x83a\x15\xE5V[PP`\x01`eUPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0F\xE6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x10\0WP0;\x15\x80\x15a\x10\0WP`\0T`\xFF\x16`\x01\x14[a\x10cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05YV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x10\x86W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x10\x8F\x85a\x15\x93V[a\x10\x99\x84\x84a\x17PV[a\x10\xA2\x82a\x14\xCBV[\x80\x15a\x0E\x96W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x95\x92\x94\x86\x81\x01\x94\x93\x91\x92\x91\x90\x84\x01[\x82\x82\x10\x15a\x11\x9AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11NV[PPP\x91RP\x90\x93\x92PPPV[a\x11\xB0a\x14qV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05YV[a\x05k\x81a\x15\x93V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x95\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1BdV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x13CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x06\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05YV[b\x03K\xC0\x81\x11\x15a\x15RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelayedWithdrawalRouter._setWith`D\x82\x01R\x7FdrawalDelayBlocks: newValue too `d\x82\x01Rdlarge`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\xC9T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\xC9UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T\x82[\x84\x81\x10\x80\x15a\x16\x1EWP\x81a\x16\x1C\x82\x85a\x1CEV[\x10[\x15a\x16\xCBW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xCA` R`@\x81 `\x01\x01a\x16H\x83\x86a\x1CEV[\x81T\x81\x10a\x16XWa\x16Xa\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R`\xC9T\x90\x92Pa\x16\x9E\x91a\x1CEV[C\x10\x15a\x16\xABWPa\x16\xCBV[\x80Qa\x16\xC0\x90`\x01`\x01`\xE0\x1B\x03\x16\x86a\x1CEV[\x94PP`\x01\x01a\x16\x07V[a\x16\xD5\x81\x84a\x1CEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x90 U\x83\x15a\x16\xFEWa\x16\xFE\x86\x85a\x18:V[\x7FkqQP\x0B\xD0\xB5\xCC!\x1B\xCCG\xB3\x02\x981\xB7i\0M\xF4T\x9E\x8E\x1C\x9Ai\xDA\x05\xBB\tC\x86\x85a\x17+\x84\x87a\x1CEV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x0E\x8CV[`\x97T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x17qWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x17\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x186\x82a\x13zV[PPV[\x80G\x10\x15a\x18\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x05YV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\xD7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\xDCV[``\x91P[PP\x90P\x80a\x19SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05kW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x7FW`\0\x80\xFD[\x815a\x19\x8A\x81a\x19XV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA3W`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A\x08Wa\x19\xF8\x84\x83Qa\x19\xAAV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x19\xE5V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1A'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x19\x8AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1AKW`\0\x80\xFD[\x825a\x1AV\x81a\x19XV[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81\x01a\x0BW\x82\x84a\x19\xAAV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\x85W`\0\x80\xFD[\x825a\x1A\x90\x81a\x19XV[\x91P` \x83\x015a\x1A\xA0\x81a\x19XV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A\xC1W`\0\x80\xFD[\x845a\x1A\xCC\x81a\x19XV[\x93P` \x85\x015a\x1A\xDC\x81a\x19XV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R`\0\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a\x08\x9BWa\x1B3\x82\x86Qa\x19\xAAV[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x1B V[`\0` \x82\x84\x03\x12\x15a\x1BYW`\0\x80\xFD[\x81Qa\x19\x8A\x81a\x19XV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1B\xC0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x19\x8AW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1C@Wa\x1C@a\x1C\x18V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\x1CXWa\x1CXa\x1C\x18V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x1C\x87Wa\x1C\x87a\x1C\x18V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \xF64y\x10\xFC\xF3\xC3\x98\x9F\xC8\xD2Vw\xBD\xDB\xA3$#LN\x16I\xD2\xC6_\xD5&\x8FZ\xDD0zdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x60806040526004361061014b5760003560e01c806385594e58116100b6578063e4f4f8871161006f578063e4f4f887146103cc578063e5db06c014610405578063eb990c5914610425578063ecb7cb1b14610445578063f2fde38b14610472578063fabc1cbc1461049257600080fd5b806385594e5814610317578063886f1195146103445780638da5cb5b14610364578063c0db354c14610382578063ca661c0414610395578063d44e1b76146103ac57600080fd5b806350f73e7c1161010857806350f73e7c14610254578063595c6a67146102785780635ac86ab71461028d5780635c975abb146102cd578063715018a6146102e257806375608896146102f757600080fd5b806310d67a2f14610150578063136439dd146101725780631f39d87f146101925780633e1de008146101c85780634665bcda146101e85780634d50f9a414610234575b600080fd5b34801561015c57600080fd5b5061017061016b36600461196d565b6104b2565b005b34801561017e57600080fd5b5061017061018d366004611991565b61056e565b34801561019e57600080fd5b506101b26101ad36600461196d565b6106ad565b6040516101bf91906119c8565b60405180910390f35b3480156101d457600080fd5b506101b26101e336600461196d565b6108a8565b3480156101f457600080fd5b5061021c7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020016101bf565b34801561024057600080fd5b5061017061024f366004611991565b6109ee565b34801561026057600080fd5b5061026a60c95481565b6040519081526020016101bf565b34801561028457600080fd5b506101706109ff565b34801561029957600080fd5b506102bd6102a8366004611a15565b609854600160ff9092169190911b9081161490565b60405190151581526020016101bf565b3480156102d957600080fd5b5060985461026a565b3480156102ee57600080fd5b50610170610ac6565b34801561030357600080fd5b506102bd610312366004611a38565b610ada565b34801561032357600080fd5b50610337610332366004611a38565b610b5d565b6040516101bf9190611a64565b34801561035057600080fd5b5060975461021c906001600160a01b031681565b34801561037057600080fd5b506033546001600160a01b031661021c565b610170610390366004611a72565b610bdd565b3480156103a157600080fd5b5061026a62034bc081565b3480156103b857600080fd5b506101706103c7366004611991565b610e9d565b3480156103d857600080fd5b5061026a6103e736600461196d565b6001600160a01b0316600090815260ca602052604090206001015490565b34801561041157600080fd5b50610170610420366004611a38565b610f31565b34801561043157600080fd5b50610170610440366004611aab565b610fc6565b34801561045157600080fd5b5061046561046036600461196d565b6110ee565b6040516101bf9190611af1565b34801561047e57600080fd5b5061017061048d36600461196d565b6111a8565b34801561049e57600080fd5b506101706104ad366004611991565b61121e565b609760009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610505573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105299190611b47565b6001600160a01b0316336001600160a01b0316146105625760405162461bcd60e51b815260040161055990611b64565b60405180910390fd5b61056b8161137a565b50565b60975460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156105b6573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906105da9190611bae565b6105f65760405162461bcd60e51b815260040161055990611bd0565b6098548181161461066f5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c69747900000000000000006064820152608401610559565b609881905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b6001600160a01b038116600090815260ca6020526040812080546001909101546060926106da8383611c2e565b90508060005b82811015610786576001600160a01b038716600090815260ca6020526040812060010161070d8388611c45565b8154811061071d5761071d611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810182905260c95490925061076391611c45565b4310156107735781925050610786565b508061077e81611c73565b9150506106e0565b508060008167ffffffffffffffff8111156107a3576107a3611c8e565b6040519080825280602002602001820160405280156107e857816020015b60408051808201909152600080825260208201528152602001906001900390816107c15790505b509050811561089d5760005b8281101561089b576001600160a01b038916600090815260ca602052604090206001016108218289611c45565b8154811061083157610831611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810191909152825183908390811061087d5761087d611c5d565b6020026020010181905250808061089390611c73565b9150506107f4565b505b979650505050505050565b6001600160a01b038116600090815260ca6020526040812080546001909101546060926108d58383611c2e565b905060008167ffffffffffffffff8111156108f2576108f2611c8e565b60405190808252806020026020018201604052801561093757816020015b60408051808201909152600080825260208201528152602001906001900390816109105790505b50905060005b828110156109e4576001600160a01b038716600090815260ca6020526040902060010161096a8287611c45565b8154811061097a5761097a611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff169181019190915282518390839081106109c6576109c6611c5d565b602002602001018190525080806109dc90611c73565b91505061093d565b5095945050505050565b6109f6611471565b61056b816114cb565b60975460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610a47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a6b9190611bae565b610a875760405162461bcd60e51b815260040161055990611bd0565b600019609881905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b610ace611471565b610ad86000611593565b565b6001600160a01b038216600090815260ca60205260408120548210801590610b54575060c9546001600160a01b038416600090815260ca60205260409020600101805484908110610b2d57610b2d611c5d565b600091825260209091200154610b509190600160e01b900463ffffffff16611c45565b4310155b90505b92915050565b60408051808201909152600080825260208201526001600160a01b038316600090815260ca60205260409020600101805483908110610b9e57610b9e611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff16918101919091529392505050565b60405163a38406a360e01b81526001600160a01b038084166004830152839133917f0000000000000000000000000000000000000000000000000000000000000000169063a38406a390602401602060405180830381865afa158015610c47573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610c6b9190611b47565b6001600160a01b031614610ce75760405162461bcd60e51b815260206004820152603d60248201527f44656c617965645769746864726177616c526f757465722e6f6e6c794569676560448201527f6e506f643a206e6f7420706f644f776e6572277320456967656e506f640000006064820152608401610559565b60985460009060019081161415610d105760405162461bcd60e51b815260040161055990611ca4565b6001600160a01b038316610da65760405162461bcd60e51b815260206004820152605160248201527f44656c617965645769746864726177616c526f757465722e637265617465446560448201527f6c617965645769746864726177616c3a20726563697069656e742063616e6e6f60648201527074206265207a65726f206164647265737360781b608482015260a401610559565b346001600160e01b03811615610e96576040805180820182526001600160e01b03808416825263ffffffff43811660208085019182526001600160a01b038a16600081815260ca8352968720600190810180548083018255818a5293892088519551909616600160e01b029490961693909317939091019290925593525490917fb8f1b14c7caf74150801dcc9bc18d575cbeaf5b421943497e409df92c92e0f5991889188918691610e5791611c2e565b604080516001600160a01b0395861681529490931660208501526001600160e01b039091169183019190915260608201526080015b60405180910390a1505b5050505050565b60026065541415610ef05760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610559565b600260655560985460009060019081161415610f1e5760405162461bcd60e51b815260040161055990611ca4565b610f2833836115e5565b50506001606555565b60026065541415610f845760405162461bcd60e51b815260206004820152601f60248201527f5265656e7472616e637947756172643a207265656e7472616e742063616c6c006044820152606401610559565b600260655560985460009060019081161415610fb25760405162461bcd60e51b815260040161055990611ca4565b610fbc83836115e5565b5050600160655550565b600054610100900460ff1615808015610fe65750600054600160ff909116105b806110005750303b158015611000575060005460ff166001145b6110635760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b6064820152608401610559565b6000805460ff191660011790558015611086576000805461ff0019166101001790555b61108f85611593565b6110998484611750565b6110a2826114cb565b8015610e96576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15050505050565b6040805180820190915260008152606060208201526001600160a01b038216600090815260ca6020908152604080832081518083018352815481526001820180548451818702810187019095528085529195929486810194939192919084015b8282101561119a57600084815260209081902060408051808201909152908401546001600160e01b0381168252600160e01b900463ffffffff168183015282526001909201910161114e565b505050915250909392505050565b6111b0611471565b6001600160a01b0381166112155760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b6064820152608401610559565b61056b81611593565b609760009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611271573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906112959190611b47565b6001600160a01b0316336001600160a01b0316146112c55760405162461bcd60e51b815260040161055990611b64565b6098541981196098541916146113435760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c69747900000000000000006064820152608401610559565b609881905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c906020016106a2565b6001600160a01b0381166114085760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a401610559565b609754604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1609780546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b03163314610ad85760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e65726044820152606401610559565b62034bc08111156115525760405162461bcd60e51b815260206004820152604560248201527f44656c617965645769746864726177616c526f757465722e5f7365745769746860448201527f64726177616c44656c6179426c6f636b733a206e657756616c756520746f6f206064820152646c6172676560d81b608482015260a401610559565b60c95460408051918252602082018390527f4ffb00400574147429ee377a5633386321e66d45d8b14676014b5fa393e61e9e910160405180910390a160c955565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b6001600160a01b038216600090815260ca602052604081208054600190910154825b848110801561161e57508161161c8285611c45565b105b156116cb576001600160a01b038616600090815260ca602052604081206001016116488386611c45565b8154811061165857611658611c5d565b6000918252602091829020604080518082019091529101546001600160e01b0381168252600160e01b900463ffffffff1691810182905260c95490925061169e91611c45565b4310156116ab57506116cb565b80516116c0906001600160e01b031686611c45565b945050600101611607565b6116d58184611c45565b6001600160a01b038716600090815260ca602052604090205583156116fe576116fe868561183a565b7f6b7151500bd0b5cc211bcc47b3029831b769004df4549e8e1c9a69da05bb0943868561172b8487611c45565b604080516001600160a01b039094168452602084019290925290820152606001610e8c565b6097546001600160a01b031615801561177157506001600160a01b03821615155b6117f35760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a401610559565b609881905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26118368261137a565b5050565b8047101561188a5760405162461bcd60e51b815260206004820152601d60248201527f416464726573733a20696e73756666696369656e742062616c616e63650000006044820152606401610559565b6000826001600160a01b03168260405160006040518083038185875af1925050503d80600081146118d7576040519150601f19603f3d011682016040523d82523d6000602084013e6118dc565b606091505b50509050806119535760405162461bcd60e51b815260206004820152603a60248201527f416464726573733a20756e61626c6520746f2073656e642076616c75652c207260448201527f6563697069656e74206d617920686176652072657665727465640000000000006064820152608401610559565b505050565b6001600160a01b038116811461056b57600080fd5b60006020828403121561197f57600080fd5b813561198a81611958565b9392505050565b6000602082840312156119a357600080fd5b5035919050565b80516001600160e01b0316825260209081015163ffffffff16910152565b602080825282518282018190526000919060409081850190868401855b82811015611a08576119f88483516119aa565b92840192908501906001016119e5565b5091979650505050505050565b600060208284031215611a2757600080fd5b813560ff8116811461198a57600080fd5b60008060408385031215611a4b57600080fd5b8235611a5681611958565b946020939093013593505050565b60408101610b5782846119aa565b60008060408385031215611a8557600080fd5b8235611a9081611958565b91506020830135611aa081611958565b809150509250929050565b60008060008060808587031215611ac157600080fd5b8435611acc81611958565b93506020850135611adc81611958565b93969395505050506040820135916060013590565b602080825282518282015282810151604080840181905281516060850181905260009392830191849160808701905b8084101561089b57611b338286516119aa565b938501936001939093019290820190611b20565b600060208284031215611b5957600080fd5b815161198a81611958565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b600060208284031215611bc057600080fd5b8151801515811461198a57600080fd5b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052601160045260246000fd5b600082821015611c4057611c40611c18565b500390565b60008219821115611c5857611c58611c18565b500190565b634e487b7160e01b600052603260045260246000fd5b6000600019821415611c8757611c87611c18565b5060010190565b634e487b7160e01b600052604160045260246000fd5b60208082526019908201527f5061757361626c653a20696e646578206973207061757365640000000000000060408201526060019056fea2646970667358221220f6347910fcf3c3989fc8d25677bddba324234c4e1649d2c65fd5268f5add307a64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01KW`\x005`\xE0\x1C\x80c\x85YNX\x11a\0\xB6W\x80c\xE4\xF4\xF8\x87\x11a\0oW\x80c\xE4\xF4\xF8\x87\x14a\x03\xCCW\x80c\xE5\xDB\x06\xC0\x14a\x04\x05W\x80c\xEB\x99\x0CY\x14a\x04%W\x80c\xEC\xB7\xCB\x1B\x14a\x04EW\x80c\xF2\xFD\xE3\x8B\x14a\x04rW\x80c\xFA\xBC\x1C\xBC\x14a\x04\x92W`\0\x80\xFD[\x80c\x85YNX\x14a\x03\x17W\x80c\x88o\x11\x95\x14a\x03DW\x80c\x8D\xA5\xCB[\x14a\x03dW\x80c\xC0\xDB5L\x14a\x03\x82W\x80c\xCAf\x1C\x04\x14a\x03\x95W\x80c\xD4N\x1Bv\x14a\x03\xACW`\0\x80\xFD[\x80cP\xF7>|\x11a\x01\x08W\x80cP\xF7>|\x14a\x02TW\x80cY\\jg\x14a\x02xW\x80cZ\xC8j\xB7\x14a\x02\x8DW\x80c\\\x97Z\xBB\x14a\x02\xCDW\x80cqP\x18\xA6\x14a\x02\xE2W\x80cu`\x88\x96\x14a\x02\xF7W`\0\x80\xFD[\x80c\x10\xD6z/\x14a\x01PW\x80c\x13d9\xDD\x14a\x01rW\x80c\x1F9\xD8\x7F\x14a\x01\x92W\x80c>\x1D\xE0\x08\x14a\x01\xC8W\x80cFe\xBC\xDA\x14a\x01\xE8W\x80cMP\xF9\xA4\x14a\x024W[`\0\x80\xFD[4\x80\x15a\x01\\W`\0\x80\xFD[Pa\x01pa\x01k6`\x04a\x19mV[a\x04\xB2V[\0[4\x80\x15a\x01~W`\0\x80\xFD[Pa\x01pa\x01\x8D6`\x04a\x19\x91V[a\x05nV[4\x80\x15a\x01\x9EW`\0\x80\xFD[Pa\x01\xB2a\x01\xAD6`\x04a\x19mV[a\x06\xADV[`@Qa\x01\xBF\x91\x90a\x19\xC8V[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xD4W`\0\x80\xFD[Pa\x01\xB2a\x01\xE36`\x04a\x19mV[a\x08\xA8V[4\x80\x15a\x01\xF4W`\0\x80\xFD[Pa\x02\x1C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xBFV[4\x80\x15a\x02@W`\0\x80\xFD[Pa\x01pa\x02O6`\x04a\x19\x91V[a\t\xEEV[4\x80\x15a\x02`W`\0\x80\xFD[Pa\x02j`\xC9T\x81V[`@Q\x90\x81R` \x01a\x01\xBFV[4\x80\x15a\x02\x84W`\0\x80\xFD[Pa\x01pa\t\xFFV[4\x80\x15a\x02\x99W`\0\x80\xFD[Pa\x02\xBDa\x02\xA86`\x04a\x1A\x15V[`\x98T`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\xBFV[4\x80\x15a\x02\xD9W`\0\x80\xFD[P`\x98Ta\x02jV[4\x80\x15a\x02\xEEW`\0\x80\xFD[Pa\x01pa\n\xC6V[4\x80\x15a\x03\x03W`\0\x80\xFD[Pa\x02\xBDa\x03\x126`\x04a\x1A8V[a\n\xDAV[4\x80\x15a\x03#W`\0\x80\xFD[Pa\x037a\x0326`\x04a\x1A8V[a\x0B]V[`@Qa\x01\xBF\x91\x90a\x1AdV[4\x80\x15a\x03PW`\0\x80\xFD[P`\x97Ta\x02\x1C\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[4\x80\x15a\x03pW`\0\x80\xFD[P`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\x1CV[a\x01pa\x03\x906`\x04a\x1ArV[a\x0B\xDDV[4\x80\x15a\x03\xA1W`\0\x80\xFD[Pa\x02jb\x03K\xC0\x81V[4\x80\x15a\x03\xB8W`\0\x80\xFD[Pa\x01pa\x03\xC76`\x04a\x19\x91V[a\x0E\x9DV[4\x80\x15a\x03\xD8W`\0\x80\xFD[Pa\x02ja\x03\xE76`\x04a\x19mV[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01T\x90V[4\x80\x15a\x04\x11W`\0\x80\xFD[Pa\x01pa\x04 6`\x04a\x1A8V[a\x0F1V[4\x80\x15a\x041W`\0\x80\xFD[Pa\x01pa\x04@6`\x04a\x1A\xABV[a\x0F\xC6V[4\x80\x15a\x04QW`\0\x80\xFD[Pa\x04ea\x04`6`\x04a\x19mV[a\x10\xEEV[`@Qa\x01\xBF\x91\x90a\x1A\xF1V[4\x80\x15a\x04~W`\0\x80\xFD[Pa\x01pa\x04\x8D6`\x04a\x19mV[a\x11\xA8V[4\x80\x15a\x04\x9EW`\0\x80\xFD[Pa\x01pa\x04\xAD6`\x04a\x19\x91V[a\x12\x1EV[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x05W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05)\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05bW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1BdV[`@Q\x80\x91\x03\x90\xFD[a\x05k\x81a\x13zV[PV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xDA\x91\x90a\x1B\xAEV[a\x05\xF6W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1B\xD0V[`\x98T\x81\x81\x16\x14a\x06oW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T``\x92a\x06\xDA\x83\x83a\x1C.V[\x90P\x80`\0[\x82\x81\x10\x15a\x07\x86W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x81 `\x01\x01a\x07\r\x83\x88a\x1CEV[\x81T\x81\x10a\x07\x1DWa\x07\x1Da\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R`\xC9T\x90\x92Pa\x07c\x91a\x1CEV[C\x10\x15a\x07sW\x81\x92PPa\x07\x86V[P\x80a\x07~\x81a\x1CsV[\x91PPa\x06\xE0V[P\x80`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA3Wa\x07\xA3a\x1C\x8EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x07\xE8W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\x07\xC1W\x90P[P\x90P\x81\x15a\x08\x9DW`\0[\x82\x81\x10\x15a\x08\x9BW`\x01`\x01`\xA0\x1B\x03\x89\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01a\x08!\x82\x89a\x1CEV[\x81T\x81\x10a\x081Wa\x081a\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90\x83\x90\x81\x10a\x08}Wa\x08}a\x1C]V[` \x02` \x01\x01\x81\x90RP\x80\x80a\x08\x93\x90a\x1CsV[\x91PPa\x07\xF4V[P[\x97\x96PPPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T``\x92a\x08\xD5\x83\x83a\x1C.V[\x90P`\0\x81g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xF2Wa\x08\xF2a\x1C\x8EV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t7W\x81` \x01[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81R` \x01\x90`\x01\x90\x03\x90\x81a\t\x10W\x90P[P\x90P`\0[\x82\x81\x10\x15a\t\xE4W`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01a\tj\x82\x87a\x1CEV[\x81T\x81\x10a\tzWa\tza\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x82Q\x83\x90\x83\x90\x81\x10a\t\xC6Wa\t\xC6a\x1C]V[` \x02` \x01\x01\x81\x90RP\x80\x80a\t\xDC\x90a\x1CsV[\x91PPa\t=V[P\x95\x94PPPPPV[a\t\xF6a\x14qV[a\x05k\x81a\x14\xCBV[`\x97T`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\nk\x91\x90a\x1B\xAEV[a\n\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1B\xD0V[`\0\x19`\x98\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n\xCEa\x14qV[a\n\xD8`\0a\x15\x93V[V[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` R`@\x81 T\x82\x10\x80\x15\x90a\x0BTWP`\xC9T`\x01`\x01`\xA0\x1B\x03\x84\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01\x80T\x84\x90\x81\x10a\x0B-Wa\x0B-a\x1C]V[`\0\x91\x82R` \x90\x91 \x01Ta\x0BP\x91\x90`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a\x1CEV[C\x10\x15[\x90P[\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\xCA` R`@\x90 `\x01\x01\x80T\x83\x90\x81\x10a\x0B\x9EWa\x0B\x9Ea\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@Qc\xA3\x84\x06\xA3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x80\x84\x16`\x04\x83\x01R\x83\x913\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA3\x84\x06\xA3\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CGW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0Ck\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0C\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FDelayedWithdrawalRouter.onlyEige`D\x82\x01R\x7FnPod: not podOwner's EigenPod\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\r\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[`\x01`\x01`\xA0\x1B\x03\x83\x16a\r\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`Q`$\x82\x01R\x7FDelayedWithdrawalRouter.createDe`D\x82\x01R\x7FlayedWithdrawal: recipient canno`d\x82\x01Rpt be zero address`x\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[4`\x01`\x01`\xE0\x1B\x03\x81\x16\x15a\x0E\x96W`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xE0\x1B\x03\x80\x84\x16\x82Rc\xFF\xFF\xFF\xFFC\x81\x16` \x80\x85\x01\x91\x82R`\x01`\x01`\xA0\x1B\x03\x8A\x16`\0\x81\x81R`\xCA\x83R\x96\x87 `\x01\x90\x81\x01\x80T\x80\x83\x01\x82U\x81\x8AR\x93\x89 \x88Q\x95Q\x90\x96\x16`\x01`\xE0\x1B\x02\x94\x90\x96\x16\x93\x90\x93\x17\x93\x90\x91\x01\x92\x90\x92U\x93RT\x90\x91\x7F\xB8\xF1\xB1L|\xAFt\x15\x08\x01\xDC\xC9\xBC\x18\xD5u\xCB\xEA\xF5\xB4!\x944\x97\xE4\t\xDF\x92\xC9.\x0FY\x91\x88\x91\x88\x91\x86\x91a\x0EW\x91a\x1C.V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x94\x90\x93\x16` \x85\x01R`\x01`\x01`\xE0\x1B\x03\x90\x91\x16\x91\x83\x01\x91\x90\x91R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xA1P[PPPPPV[`\x02`eT\x14\x15a\x0E\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x05YV[`\x02`eU`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\x0F\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[a\x0F(3\x83a\x15\xE5V[PP`\x01`eUV[`\x02`eT\x14\x15a\x0F\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FReentrancyGuard: reentrant call\0`D\x82\x01R`d\x01a\x05YV[`\x02`eU`\x98T`\0\x90`\x01\x90\x81\x16\x14\x15a\x0F\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1C\xA4V[a\x0F\xBC\x83\x83a\x15\xE5V[PP`\x01`eUPV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x0F\xE6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x10\0WP0;\x15\x80\x15a\x10\0WP`\0T`\xFF\x16`\x01\x14[a\x10cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05YV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x10\x86W`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x10\x8F\x85a\x15\x93V[a\x10\x99\x84\x84a\x17PV[a\x10\xA2\x82a\x14\xCBV[\x80\x15a\x0E\x96W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x81R``` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` \x90\x81R`@\x80\x83 \x81Q\x80\x83\x01\x83R\x81T\x81R`\x01\x82\x01\x80T\x84Q\x81\x87\x02\x81\x01\x87\x01\x90\x95R\x80\x85R\x91\x95\x92\x94\x86\x81\x01\x94\x93\x91\x92\x91\x90\x84\x01[\x82\x82\x10\x15a\x11\x9AW`\0\x84\x81R` \x90\x81\x90 `@\x80Q\x80\x82\x01\x90\x91R\x90\x84\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81\x83\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x11NV[PPP\x91RP\x90\x93\x92PPPV[a\x11\xB0a\x14qV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x12\x15W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05YV[a\x05k\x81a\x15\x93V[`\x97`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x12qW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x95\x91\x90a\x1BGV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x12\xC5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05Y\x90a\x1BdV[`\x98T\x19\x81\x19`\x98T\x19\x16\x14a\x13CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x06\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x14\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\x97T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\n\xD8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05YV[b\x03K\xC0\x81\x11\x15a\x15RW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R\x7FDelayedWithdrawalRouter._setWith`D\x82\x01R\x7FdrawalDelayBlocks: newValue too `d\x82\x01Rdlarge`\xD8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\xC9T`@\x80Q\x91\x82R` \x82\x01\x83\x90R\x7FO\xFB\0@\x05t\x14t)\xEE7zV38c!\xE6mE\xD8\xB1Fv\x01K_\xA3\x93\xE6\x1E\x9E\x91\x01`@Q\x80\x91\x03\x90\xA1`\xC9UV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\xCA` R`@\x81 \x80T`\x01\x90\x91\x01T\x82[\x84\x81\x10\x80\x15a\x16\x1EWP\x81a\x16\x1C\x82\x85a\x1CEV[\x10[\x15a\x16\xCBW`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\xCA` R`@\x81 `\x01\x01a\x16H\x83\x86a\x1CEV[\x81T\x81\x10a\x16XWa\x16Xa\x1C]V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xE0\x1B\x03\x81\x16\x82R`\x01`\xE0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x91\x81\x01\x82\x90R`\xC9T\x90\x92Pa\x16\x9E\x91a\x1CEV[C\x10\x15a\x16\xABWPa\x16\xCBV[\x80Qa\x16\xC0\x90`\x01`\x01`\xE0\x1B\x03\x16\x86a\x1CEV[\x94PP`\x01\x01a\x16\x07V[a\x16\xD5\x81\x84a\x1CEV[`\x01`\x01`\xA0\x1B\x03\x87\x16`\0\x90\x81R`\xCA` R`@\x90 U\x83\x15a\x16\xFEWa\x16\xFE\x86\x85a\x18:V[\x7FkqQP\x0B\xD0\xB5\xCC!\x1B\xCCG\xB3\x02\x981\xB7i\0M\xF4T\x9E\x8E\x1C\x9Ai\xDA\x05\xBB\tC\x86\x85a\x17+\x84\x87a\x1CEV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x94\x16\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x0E\x8CV[`\x97T`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a\x17qWP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a\x17\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05YV[`\x98\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a\x186\x82a\x13zV[PPV[\x80G\x10\x15a\x18\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FAddress: insufficient balance\0\0\0`D\x82\x01R`d\x01a\x05YV[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16\x82`@Q`\0`@Q\x80\x83\x03\x81\x85\x87Z\xF1\x92PPP=\x80`\0\x81\x14a\x18\xD7W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x18\xDCV[``\x91P[PP\x90P\x80a\x19SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`:`$\x82\x01R\x7FAddress: unable to send value, r`D\x82\x01R\x7Fecipient may have reverted\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05YV[PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05kW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x19\x7FW`\0\x80\xFD[\x815a\x19\x8A\x81a\x19XV[\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x19\xA3W`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x82R` \x90\x81\x01Qc\xFF\xFF\xFF\xFF\x16\x91\x01RV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90`@\x90\x81\x85\x01\x90\x86\x84\x01\x85[\x82\x81\x10\x15a\x1A\x08Wa\x19\xF8\x84\x83Qa\x19\xAAV[\x92\x84\x01\x92\x90\x85\x01\x90`\x01\x01a\x19\xE5V[P\x91\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1A'W`\0\x80\xFD[\x815`\xFF\x81\x16\x81\x14a\x19\x8AW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1AKW`\0\x80\xFD[\x825a\x1AV\x81a\x19XV[\x94` \x93\x90\x93\x015\x93PPPV[`@\x81\x01a\x0BW\x82\x84a\x19\xAAV[`\0\x80`@\x83\x85\x03\x12\x15a\x1A\x85W`\0\x80\xFD[\x825a\x1A\x90\x81a\x19XV[\x91P` \x83\x015a\x1A\xA0\x81a\x19XV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1A\xC1W`\0\x80\xFD[\x845a\x1A\xCC\x81a\x19XV[\x93P` \x85\x015a\x1A\xDC\x81a\x19XV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[` \x80\x82R\x82Q\x82\x82\x01R\x82\x81\x01Q`@\x80\x84\x01\x81\x90R\x81Q``\x85\x01\x81\x90R`\0\x93\x92\x83\x01\x91\x84\x91`\x80\x87\x01\x90[\x80\x84\x10\x15a\x08\x9BWa\x1B3\x82\x86Qa\x19\xAAV[\x93\x85\x01\x93`\x01\x93\x90\x93\x01\x92\x90\x82\x01\x90a\x1B V[`\0` \x82\x84\x03\x12\x15a\x1BYW`\0\x80\xFD[\x81Qa\x19\x8A\x81a\x19XV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a\x1B\xC0W`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x19\x8AW`\0\x80\xFD[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x1C@Wa\x1C@a\x1C\x18V[P\x03\x90V[`\0\x82\x19\x82\x11\x15a\x1CXWa\x1CXa\x1C\x18V[P\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x1C\x87Wa\x1C\x87a\x1C\x18V[P`\x01\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R`\x19\x90\x82\x01R\x7FPausable: index is paused\0\0\0\0\0\0\0`@\x82\x01R``\x01\x90V\xFE\xA2dipfsX\"\x12 \xF64y\x10\xFC\xF3\xC3\x98\x9F\xC8\xD2Vw\xBD\xDB\xA3$#LN\x16I\xD2\xC6_\xD5&\x8FZ\xDD0zdsolcC\0\x08\x0C\x003",
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
    /**Constructor`.
```solidity
constructor(address _eigenPodManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _eigenPodManager: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._eigenPodManager,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _eigenPodManager: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
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
                )
            }
        }
    };
    /**Function with signature `MAX_WITHDRAWAL_DELAY_BLOCKS()` and selector `0xca661c04`.
```solidity
function MAX_WITHDRAWAL_DELAY_BLOCKS() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WITHDRAWAL_DELAY_BLOCKSCall {}
    ///Container type for the return parameters of the [`MAX_WITHDRAWAL_DELAY_BLOCKS()`](MAX_WITHDRAWAL_DELAY_BLOCKSCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WITHDRAWAL_DELAY_BLOCKSReturn {
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
            impl ::core::convert::From<MAX_WITHDRAWAL_DELAY_BLOCKSCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WITHDRAWAL_DELAY_BLOCKSCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WITHDRAWAL_DELAY_BLOCKSCall {
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
            impl ::core::convert::From<MAX_WITHDRAWAL_DELAY_BLOCKSReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WITHDRAWAL_DELAY_BLOCKSReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WITHDRAWAL_DELAY_BLOCKSReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WITHDRAWAL_DELAY_BLOCKSCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_WITHDRAWAL_DELAY_BLOCKSReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_WITHDRAWAL_DELAY_BLOCKS()";
            const SELECTOR: [u8; 4] = [202u8, 102u8, 28u8, 4u8];
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
function claimDelayedWithdrawals(uint256 maxNumberOfDelayedWithdrawalsToClaim) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_0Call {
        pub maxNumberOfDelayedWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.maxNumberOfDelayedWithdrawalsToClaim,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        maxNumberOfDelayedWithdrawalsToClaim: tuple.0,
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
                        &self.maxNumberOfDelayedWithdrawalsToClaim,
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
function claimDelayedWithdrawals(address recipient, uint256 maxNumberOfDelayedWithdrawalsToClaim) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct claimDelayedWithdrawals_1Call {
        pub recipient: alloy::sol_types::private::Address,
        pub maxNumberOfDelayedWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
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
                    (value.recipient, value.maxNumberOfDelayedWithdrawalsToClaim)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for claimDelayedWithdrawals_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        maxNumberOfDelayedWithdrawalsToClaim: tuple.1,
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
                        &self.maxNumberOfDelayedWithdrawalsToClaim,
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
    /**Function with signature `initialize(address,address,uint256,uint256)` and selector `0xeb990c59`.
```solidity
function initialize(address initOwner, address _pauserRegistry, uint256 initPausedStatus, uint256 _withdrawalDelayBlocks) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub initOwner: alloy::sol_types::private::Address,
        pub _pauserRegistry: alloy::sol_types::private::Address,
        pub initPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
        pub _withdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`initialize(address,address,uint256,uint256)`](initializeCall) function.
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
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (
                        value.initOwner,
                        value._pauserRegistry,
                        value.initPausedStatus,
                        value._withdrawalDelayBlocks,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        initOwner: tuple.0,
                        _pauserRegistry: tuple.1,
                        initPausedStatus: tuple.2,
                        _withdrawalDelayBlocks: tuple.3,
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
            const SIGNATURE: &'static str = "initialize(address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [235u8, 153u8, 12u8, 89u8];
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
                        &self.initOwner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._pauserRegistry,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.initPausedStatus),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self._withdrawalDelayBlocks,
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
    ///Container for all the [`DelayedWithdrawalRouter`](self) function calls.
    pub enum DelayedWithdrawalRouterCalls {
        MAX_WITHDRAWAL_DELAY_BLOCKS(MAX_WITHDRAWAL_DELAY_BLOCKSCall),
        canClaimDelayedWithdrawal(canClaimDelayedWithdrawalCall),
        claimDelayedWithdrawals_0(claimDelayedWithdrawals_0Call),
        claimDelayedWithdrawals_1(claimDelayedWithdrawals_1Call),
        createDelayedWithdrawal(createDelayedWithdrawalCall),
        eigenPodManager(eigenPodManagerCall),
        getClaimableUserDelayedWithdrawals(getClaimableUserDelayedWithdrawalsCall),
        getUserDelayedWithdrawals(getUserDelayedWithdrawalsCall),
        initialize(initializeCall),
        owner(ownerCall),
        pause(pauseCall),
        pauseAll(pauseAllCall),
        paused_0(paused_0Call),
        paused_1(paused_1Call),
        pauserRegistry(pauserRegistryCall),
        renounceOwnership(renounceOwnershipCall),
        setPauserRegistry(setPauserRegistryCall),
        setWithdrawalDelayBlocks(setWithdrawalDelayBlocksCall),
        transferOwnership(transferOwnershipCall),
        unpause(unpauseCall),
        userDelayedWithdrawalByIndex(userDelayedWithdrawalByIndexCall),
        userWithdrawals(userWithdrawalsCall),
        userWithdrawalsLength(userWithdrawalsLengthCall),
        withdrawalDelayBlocks(withdrawalDelayBlocksCall),
    }
    #[automatically_derived]
    impl DelayedWithdrawalRouterCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [16u8, 214u8, 122u8, 47u8],
            [19u8, 100u8, 57u8, 221u8],
            [31u8, 57u8, 216u8, 127u8],
            [62u8, 29u8, 224u8, 8u8],
            [70u8, 101u8, 188u8, 218u8],
            [77u8, 80u8, 249u8, 164u8],
            [80u8, 247u8, 62u8, 124u8],
            [89u8, 92u8, 106u8, 103u8],
            [90u8, 200u8, 106u8, 183u8],
            [92u8, 151u8, 90u8, 187u8],
            [113u8, 80u8, 24u8, 166u8],
            [117u8, 96u8, 136u8, 150u8],
            [133u8, 89u8, 78u8, 88u8],
            [136u8, 111u8, 17u8, 149u8],
            [141u8, 165u8, 203u8, 91u8],
            [192u8, 219u8, 53u8, 76u8],
            [202u8, 102u8, 28u8, 4u8],
            [212u8, 78u8, 27u8, 118u8],
            [228u8, 244u8, 248u8, 135u8],
            [229u8, 219u8, 6u8, 192u8],
            [235u8, 153u8, 12u8, 89u8],
            [236u8, 183u8, 203u8, 27u8],
            [242u8, 253u8, 227u8, 139u8],
            [250u8, 188u8, 28u8, 188u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for DelayedWithdrawalRouterCalls {
        const NAME: &'static str = "DelayedWithdrawalRouterCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 24usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(_) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::SELECTOR
                }
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
                Self::eigenPodManager(_) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getClaimableUserDelayedWithdrawals(_) => {
                    <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getUserDelayedWithdrawals(_) => {
                    <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pause(_) => <pauseCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauseAll(_) => <pauseAllCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_0(_) => <paused_0Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::paused_1(_) => <paused_1Call as alloy_sol_types::SolCall>::SELECTOR,
                Self::pauserRegistry(_) => {
                    <pauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::renounceOwnership(_) => {
                    <renounceOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setPauserRegistry(_) => {
                    <setPauserRegistryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setWithdrawalDelayBlocks(_) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unpause(_) => <unpauseCall as alloy_sol_types::SolCall>::SELECTOR,
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
            ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls>] = &[
                {
                    fn setPauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <setPauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::setPauserRegistry)
                    }
                    setPauserRegistry
                },
                {
                    fn pause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <pauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::pause)
                    }
                    pause
                },
                {
                    fn getClaimableUserDelayedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <getClaimableUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterCalls::getClaimableUserDelayedWithdrawals,
                            )
                    }
                    getClaimableUserDelayedWithdrawals
                },
                {
                    fn getUserDelayedWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <getUserDelayedWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::getUserDelayedWithdrawals)
                    }
                    getUserDelayedWithdrawals
                },
                {
                    fn eigenPodManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::eigenPodManager)
                    }
                    eigenPodManager
                },
                {
                    fn setWithdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::setWithdrawalDelayBlocks)
                    }
                    setWithdrawalDelayBlocks
                },
                {
                    fn withdrawalDelayBlocks(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <withdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::withdrawalDelayBlocks)
                    }
                    withdrawalDelayBlocks
                },
                {
                    fn pauseAll(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <pauseAllCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::pauseAll)
                    }
                    pauseAll
                },
                {
                    fn paused_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <paused_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::paused_0)
                    }
                    paused_0
                },
                {
                    fn paused_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <paused_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::paused_1)
                    }
                    paused_1
                },
                {
                    fn renounceOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <renounceOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::renounceOwnership)
                    }
                    renounceOwnership
                },
                {
                    fn canClaimDelayedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <canClaimDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::canClaimDelayedWithdrawal)
                    }
                    canClaimDelayedWithdrawal
                },
                {
                    fn userDelayedWithdrawalByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <userDelayedWithdrawalByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterCalls::userDelayedWithdrawalByIndex,
                            )
                    }
                    userDelayedWithdrawalByIndex
                },
                {
                    fn pauserRegistry(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <pauserRegistryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::pauserRegistry)
                    }
                    pauserRegistry
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::owner)
                    }
                    owner
                },
                {
                    fn createDelayedWithdrawal(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <createDelayedWithdrawalCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::createDelayedWithdrawal)
                    }
                    createDelayedWithdrawal
                },
                {
                    fn MAX_WITHDRAWAL_DELAY_BLOCKS(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                DelayedWithdrawalRouterCalls::MAX_WITHDRAWAL_DELAY_BLOCKS,
                            )
                    }
                    MAX_WITHDRAWAL_DELAY_BLOCKS
                },
                {
                    fn claimDelayedWithdrawals_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <claimDelayedWithdrawals_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::claimDelayedWithdrawals_0)
                    }
                    claimDelayedWithdrawals_0
                },
                {
                    fn userWithdrawalsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <userWithdrawalsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::userWithdrawalsLength)
                    }
                    userWithdrawalsLength
                },
                {
                    fn claimDelayedWithdrawals_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <claimDelayedWithdrawals_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::claimDelayedWithdrawals_1)
                    }
                    claimDelayedWithdrawals_1
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::initialize)
                    }
                    initialize
                },
                {
                    fn userWithdrawals(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <userWithdrawalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::userWithdrawals)
                    }
                    userWithdrawals
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn unpause(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<DelayedWithdrawalRouterCalls> {
                        <unpauseCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(DelayedWithdrawalRouterCalls::unpause)
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
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
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
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::setWithdrawalDelayBlocks(inner) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::MAX_WITHDRAWAL_DELAY_BLOCKS(inner) => {
                    <MAX_WITHDRAWAL_DELAY_BLOCKSCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
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
                Self::eigenPodManager(inner) => {
                    <eigenPodManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setWithdrawalDelayBlocks(inner) => {
                    <setWithdrawalDelayBlocksCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`DelayedWithdrawalRouter`](self) events.
    pub enum DelayedWithdrawalRouterEvents {
        DelayedWithdrawalCreated(DelayedWithdrawalCreated),
        DelayedWithdrawalsClaimed(DelayedWithdrawalsClaimed),
        Initialized(Initialized),
        OwnershipTransferred(OwnershipTransferred),
        Paused(Paused),
        PauserRegistrySet(PauserRegistrySet),
        Unpaused(Unpaused),
        WithdrawalDelayBlocksSet(WithdrawalDelayBlocksSet),
    }
    #[automatically_derived]
    impl DelayedWithdrawalRouterEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
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
    impl alloy_sol_types::SolEventInterface for DelayedWithdrawalRouterEvents {
        const NAME: &'static str = "DelayedWithdrawalRouterEvents";
        const COUNT: usize = 8usize;
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
                Some(<Initialized as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialized as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialized)
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
                Some(<Unpaused as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Unpaused as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Unpaused)
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
    impl alloy_sol_types::private::IntoLogData for DelayedWithdrawalRouterEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::DelayedWithdrawalCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::DelayedWithdrawalsClaimed(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialized(inner) => {
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
                Self::Unpaused(inner) => {
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
                Self::Initialized(inner) => {
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
                Self::Unpaused(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::WithdrawalDelayBlocksSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`DelayedWithdrawalRouter`](self) contract instance.

See the [wrapper's documentation](`DelayedWithdrawalRouterInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> DelayedWithdrawalRouterInstance<T, P, N> {
        DelayedWithdrawalRouterInstance::<T, P, N>::new(address, provider)
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<DelayedWithdrawalRouterInstance<T, P, N>>,
    > {
        DelayedWithdrawalRouterInstance::<T, P, N>::deploy(provider, _eigenPodManager)
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
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        DelayedWithdrawalRouterInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _eigenPodManager)
    }
    /**A [`DelayedWithdrawalRouter`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`DelayedWithdrawalRouter`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct DelayedWithdrawalRouterInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for DelayedWithdrawalRouterInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("DelayedWithdrawalRouterInstance")
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
    > DelayedWithdrawalRouterInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`DelayedWithdrawalRouter`](self) contract instance.

See the [wrapper's documentation](`DelayedWithdrawalRouterInstance`) for more details.*/
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
        ) -> alloy_contract::Result<DelayedWithdrawalRouterInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider, _eigenPodManager);
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
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _eigenPodManager,
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
    impl<T, P: ::core::clone::Clone, N> DelayedWithdrawalRouterInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> DelayedWithdrawalRouterInstance<T, P, N> {
            DelayedWithdrawalRouterInstance {
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
    > DelayedWithdrawalRouterInstance<T, P, N> {
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
        ///Creates a new call builder for the [`MAX_WITHDRAWAL_DELAY_BLOCKS`] function.
        pub fn MAX_WITHDRAWAL_DELAY_BLOCKS(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_WITHDRAWAL_DELAY_BLOCKSCall, N> {
            self.call_builder(&MAX_WITHDRAWAL_DELAY_BLOCKSCall {})
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
            maxNumberOfDelayedWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimDelayedWithdrawals_0Call, N> {
            self.call_builder(
                &claimDelayedWithdrawals_0Call {
                    maxNumberOfDelayedWithdrawalsToClaim,
                },
            )
        }
        ///Creates a new call builder for the [`claimDelayedWithdrawals_1`] function.
        pub fn claimDelayedWithdrawals_1(
            &self,
            recipient: alloy::sol_types::private::Address,
            maxNumberOfDelayedWithdrawalsToClaim: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, claimDelayedWithdrawals_1Call, N> {
            self.call_builder(
                &claimDelayedWithdrawals_1Call {
                    recipient,
                    maxNumberOfDelayedWithdrawalsToClaim,
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
        ///Creates a new call builder for the [`eigenPodManager`] function.
        pub fn eigenPodManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, eigenPodManagerCall, N> {
            self.call_builder(&eigenPodManagerCall {})
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
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            initOwner: alloy::sol_types::private::Address,
            _pauserRegistry: alloy::sol_types::private::Address,
            initPausedStatus: alloy::sol_types::private::primitives::aliases::U256,
            _withdrawalDelayBlocks: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    initOwner,
                    _pauserRegistry,
                    initPausedStatus,
                    _withdrawalDelayBlocks,
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
    > DelayedWithdrawalRouterInstance<T, P, N> {
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
        ///Creates a new event filter for the [`Initialized`] event.
        pub fn Initialized_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, Initialized, N> {
            self.event_filter::<Initialized>()
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
        ///Creates a new event filter for the [`Unpaused`] event.
        pub fn Unpaused_filter(&self) -> alloy_contract::Event<T, &P, Unpaused, N> {
            self.event_filter::<Unpaused>()
        }
        ///Creates a new event filter for the [`WithdrawalDelayBlocksSet`] event.
        pub fn WithdrawalDelayBlocksSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, WithdrawalDelayBlocksSet, N> {
            self.event_filter::<WithdrawalDelayBlocksSet>()
        }
    }
}
