///Module containing a contract's types and functions.
/**

```solidity
library IStakeRegistry {
    struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
    struct StrategyParams { address strategy; uint96 multiplier; }
}
```*/
#[allow(
    non_camel_case_types,
    non_snake_case,
    clippy::pub_underscore_fields,
    clippy::style
)]
pub mod IStakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct StakeUpdate { uint32 updateBlockNumber; uint32 nextUpdateBlockNumber; uint96 stake; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeUpdate {
        pub updateBlockNumber: u32,
        pub nextUpdateBlockNumber: u32,
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
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<32>,
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            u32,
            u32,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<StakeUpdate> for UnderlyingRustTuple<'_> {
            fn from(value: StakeUpdate) -> Self {
                (value.updateBlockNumber, value.nextUpdateBlockNumber, value.stake)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StakeUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    updateBlockNumber: tuple.0,
                    nextUpdateBlockNumber: tuple.1,
                    stake: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StakeUpdate {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StakeUpdate {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.updateBlockNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
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
        impl alloy_sol_types::SolType for StakeUpdate {
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
        impl alloy_sol_types::SolStruct for StakeUpdate {
            const NAME: &'static str = "StakeUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint96 stake)",
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
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.updateBlockNumber,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.nextUpdateBlockNumber,
                        )
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
        impl alloy_sol_types::EventTopic for StakeUpdate {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.updateBlockNumber,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.nextUpdateBlockNumber,
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
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.updateBlockNumber,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.nextUpdateBlockNumber,
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
struct StrategyParams { address strategy; uint96 multiplier; }
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StrategyParams {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
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
            alloy::sol_types::sol_data::Uint<96>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::primitives::aliases::U96,
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
        impl ::core::convert::From<StrategyParams> for UnderlyingRustTuple<'_> {
            fn from(value: StrategyParams) -> Self {
                (value.strategy, value.multiplier)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for StrategyParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    strategy: tuple.0,
                    multiplier: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for StrategyParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for StrategyParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
                            &self.strategy,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.multiplier)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for StrategyParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.strategy,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.multiplier,
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
                    &rust.strategy,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    96,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.multiplier,
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
    /**Creates a new wrapper around an on-chain [`IStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IStakeRegistryInstance<T, P, N> {
        IStakeRegistryInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IStakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IStakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IStakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IStakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IStakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IStakeRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IStakeRegistry`](self) contract instance.

See the [wrapper's documentation](`IStakeRegistryInstance`) for more details.*/
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
    impl<T, P: ::core::clone::Clone, N> IStakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IStakeRegistryInstance<T, P, N> {
            IStakeRegistryInstance {
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
    > IStakeRegistryInstance<T, P, N> {
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
    > IStakeRegistryInstance<T, P, N> {
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
library IStakeRegistry {
    struct StakeUpdate {
        uint32 updateBlockNumber;
        uint32 nextUpdateBlockNumber;
        uint96 stake;
    }
    struct StrategyParams {
        address strategy;
        uint96 multiplier;
    }
}

interface StakeRegistry {
    event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
    event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
    event QuorumCreated(uint8 indexed quorumNumber);
    event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
    event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
    event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);

    constructor(address _registryCoordinator, address _delegationManager);

    function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
    function WEIGHTING_DIVISOR() external view returns (uint256);
    function addStrategies(uint8 quorumNumber, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function delegation() external view returns (address);
    function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
    function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
    function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
    function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistry.StakeUpdate memory);
    function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
    function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
    function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistry.StakeUpdate[] memory);
    function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
    function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistry.StakeUpdate memory);
    function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
    function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
    function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
    function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
    function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistry.StakeUpdate memory);
    function initializeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
    function registryCoordinator() external view returns (address);
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
    function strategiesPerQuorum(uint8, uint256) external view returns (address);
    function strategyParams(uint8, uint256) external view returns (address strategy, uint96 multiplier);
    function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistry.StrategyParams memory);
    function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
    function updateOperatorStake(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint192);
    function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
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
        "internalType": "contract IRegistryCoordinator"
      },
      {
        "name": "_delegationManager",
        "type": "address",
        "internalType": "contract IDelegationManager"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "MAX_WEIGHING_FUNCTION_LENGTH",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "WEIGHTING_DIVISOR",
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
    "name": "addStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistry.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
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
    "name": "deregisterOperator",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
    "name": "getCurrentStake",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getCurrentTotalStake",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getLatestStakeUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple",
        "internalType": "struct IStakeRegistry.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
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
    "name": "getStakeAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeAtBlockNumberAndIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getStakeHistory",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistry.StakeUpdate[]",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
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
    "name": "getStakeHistoryLength",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "getStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
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
        "internalType": "struct IStakeRegistry.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
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
    "name": "getStakeUpdateIndexAtBlockNumber",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeAtBlockNumberFromIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
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
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeHistoryLength",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "getTotalStakeIndicesAtBlockNumber",
    "inputs": [
      {
        "name": "blockNumber",
        "type": "uint32",
        "internalType": "uint32"
      },
      {
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint32[]",
        "internalType": "uint32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "getTotalStakeUpdateAtIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
        "internalType": "struct IStakeRegistry.StakeUpdate",
        "components": [
          {
            "name": "updateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
          },
          {
            "name": "nextUpdateBlockNumber",
            "type": "uint32",
            "internalType": "uint32"
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
    "name": "initializeQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "_strategyParams",
        "type": "tuple[]",
        "internalType": "struct IStakeRegistry.StrategyParams[]",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
            "type": "uint96",
            "internalType": "uint96"
          }
        ]
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "minimumStakeForQuorum",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "modifyStrategyParams",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "strategyIndices",
        "type": "uint256[]",
        "internalType": "uint256[]"
      },
      {
        "name": "newMultipliers",
        "type": "uint96[]",
        "internalType": "uint96[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "registerOperator",
    "inputs": [
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
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96[]",
        "internalType": "uint96[]"
      },
      {
        "name": "",
        "type": "uint96[]",
        "internalType": "uint96[]"
      }
    ],
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
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "removeStrategies",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "indicesToRemove",
        "type": "uint256[]",
        "internalType": "uint256[]"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setMinimumStakeForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "strategiesPerQuorum",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IStrategy"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParams",
    "inputs": [
      {
        "name": "",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "strategy",
        "type": "address",
        "internalType": "contract IStrategy"
      },
      {
        "name": "multiplier",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "strategyParamsByIndex",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
        "internalType": "struct IStakeRegistry.StrategyParams",
        "components": [
          {
            "name": "strategy",
            "type": "address",
            "internalType": "contract IStrategy"
          },
          {
            "name": "multiplier",
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
    "name": "strategyParamsLength",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
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
    "name": "updateOperatorStake",
    "inputs": [
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
        "name": "quorumNumbers",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint192",
        "internalType": "uint192"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "weightOfOperatorForQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "event",
    "name": "MinimumStakeForQuorumUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "minimumStake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorStakeUpdate",
    "inputs": [
      {
        "name": "operatorId",
        "type": "bytes32",
        "indexed": true,
        "internalType": "bytes32"
      },
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": false,
        "internalType": "uint8"
      },
      {
        "name": "stake",
        "type": "uint96",
        "indexed": false,
        "internalType": "uint96"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "QuorumCreated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyAddedToQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
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
    "name": "StrategyMultiplierUpdated",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
      },
      {
        "name": "multiplier",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "StrategyRemovedFromQuorum",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "indexed": true,
        "internalType": "uint8"
      },
      {
        "name": "strategy",
        "type": "address",
        "indexed": false,
        "internalType": "contract IStrategy"
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
pub mod StakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b506040516200395838038062003958833981016040819052620000349162000065565b6001600160a01b0391821660a05216608052620000a4565b6001600160a01b03811681146200006257600080fd5b50565b600080604083850312156200007957600080fd5b825162000086816200004c565b602084015190925062000099816200004c565b809150509250929050565b60805160a05161384f620001096000396000818161037a015281816106320152818161096f01528181610ce6015281816111110152818161170301528181611805015281816119290152611ce70152600081816105290152611ea8015261384f6000f3fe608060405234801561001057600080fd5b50600436106101e55760003560e01c80639f3ccf651161010f578063c8294c56116100a2578063f2be94ae11610071578063f2be94ae1461054b578063f851e1981461055e578063fa28c62714610571578063ff694a771461058457600080fd5b8063c8294c56146104d6578063d5eccc05146104e9578063dd9846b9146104fc578063df5cf7231461052457600080fd5b8063bc9a40c3116100de578063bc9a40c314610474578063bd29b8cd14610487578063c46778a51461049a578063c601527d146104c357600080fd5b80639f3ccf65146103ee578063ac6bfb0314610401578063adc804da14610421578063b6904b781461046157600080fd5b80634bd26e091161018757806366acfefe1161015657806366acfefe1461034a5780636d14a987146103755780637c172347146103b457806381c07502146103ce57600080fd5b80634bd26e09146102e55780635401ed27146103155780635e5a6775146103285780635f1f2d771461033757600080fd5b806320b66298116101c357806320b662981461026c57806325504777146102815780632cd95940146102a25780633ca5a5f5146102c257600080fd5b80630491b41c146101ea57806308732461146102205780631f9b74e014610241575b600080fd5b61020d6101f8366004612c27565b60ff1660009081526001602052604090205490565b6040519081526020015b60405180910390f35b61023361022e366004612c42565b610597565b604051610217929190612c6c565b61025461024f366004612ca6565b6105e0565b6040516001600160601b039091168152602001610217565b61027f61027a366004612d21565b610630565b005b61029461028f366004612de2565b610961565b604051610217929190612e81565b6102b56102b0366004612ea6565b610c2c565b6040516102179190612ed2565b61020d6102d0366004612c27565b60ff1660009081526003602052604090205490565b61020d6102f3366004612ea6565b600091825260026020908152604080842060ff93909316845291905290205490565b610254610323366004612ea6565b610ccb565b61020d670de0b6b3a764000081565b61027f610345366004612fdb565b610ce4565b61035d610358366004612de2565b611104565b6040516001600160c01b039091168152602001610217565b61039c7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610217565b6103bc602081565b60405160ff9091168152602001610217565b6103e16103dc366004613097565b61125e565b60405161021791906130e9565b61039c6103fc366004612c42565b611528565b61041461040f366004613127565b611560565b604051610217919061315a565b61043461042f366004612c42565b6115f8565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610217565b61041461046f366004612c42565b611672565b61027f6104823660046131a6565b611701565b61027f6104953660046131d0565b6117fa565b6102546104a8366004612c27565b6000602081905290815260409020546001600160601b031681565b61027f6104d136600461329c565b611927565b6102546104e43660046132e9565b611a1b565b6102546104f7366004612c27565b611a99565b61050f61050a366004613325565b611aec565b60405163ffffffff9091168152602001610217565b61039c7f000000000000000000000000000000000000000000000000000000000000000081565b610254610559366004613361565b611b01565b61041461056c366004612ea6565b611b96565b61025461057f366004613325565b611c7b565b61027f6105923660046133a3565b611cdc565b600360205281600052604060002081815481106105b357600080fd5b6000918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b60ff8216600090815260016020526040812054839061061a5760405162461bcd60e51b815260040161061190613400565b60405180910390fd5b60006106268585611e47565b5095945050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561068e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106b29190613451565b6001600160a01b0316336001600160a01b0316146106e25760405162461bcd60e51b81526004016106119061346e565b846106fe8160ff16600090815260016020526040902054151590565b61071a5760405162461bcd60e51b815260040161061190613400565b8380610790576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152608401610611565b8281146108055760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608401610611565b60ff87166000908152600360205260408120905b8281101561095657858582818110610833576108336134ea565b90506020020160208101906108489190613500565b8289898481811061085b5761085b6134ea565b9050602002013581548110610872576108726134ea565b9060005260206000200160000160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a858181106108db576108db6134ea565b90506020020135815481106108f2576108f26134ea565b6000918252602090912001546001600160a01b0316888885818110610919576109196134ea565b905060200201602081019061092e9190613500565b60405161093c929190612c6c565b60405180910390a28061094e81613531565b915050610819565b505050505050505050565b606080336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109ac5760405162461bcd60e51b81526004016106119061354c565b6000836001600160401b038111156109c6576109c6612f4a565b6040519080825280602002602001820160405280156109ef578160200160208202803683370190505b5090506000846001600160401b03811115610a0c57610a0c612f4a565b604051908082528060200260200182016040528015610a35578160200160208202803683370190505b50905060005b85811015610c1e576000878783818110610a5757610a576134ea565b919091013560f81c60008181526001602052604090205490925015159050610adf5760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608401610611565b600080610aec838d611e47565b9150915080610b895760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a401610611565b6000610b968c8585612045565b905082878681518110610bab57610bab6134ea565b60200260200101906001600160601b031690816001600160601b031681525050610bd584826122c5565b868681518110610be757610be76134ea565b60200260200101906001600160601b031690816001600160601b031681525050505050508080610c1690613531565b915050610a3b565b509097909650945050505050565b600082815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610cbe576000848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610c65565b5050505090505b92915050565b600080610cd88484611b96565b60400151949350505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d42573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d669190613451565b6001600160a01b0316336001600160a01b031614610d965760405162461bcd60e51b81526004016106119061346e565b81610db28160ff16600090815260016020526040902054151590565b610dce5760405162461bcd60e51b815260040161061190613400565b815180610e435760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608401610611565b60ff841660009081526003602090815260408083206004909252822090915b838110156110fb578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610ea257610ea26134ea565b602002602001015181548110610eba57610eba6134ea565b600091825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610f1857610f186134ea565b602002602001015181548110610f3057610f306134ea565b600091825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610f70906001906135be565b81548110610f8057610f806134ea565b9060005260206000200183878381518110610f9d57610f9d6134ea565b602002602001015181548110610fb557610fb56134ea565b600091825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558254839080611008576110086135d5565b60008281526020812082016000199081019190915501905581548290611030906001906135be565b81548110611040576110406134ea565b9060005260206000200160009054906101000a90046001600160a01b031682878381518110611071576110716134ea565b602002602001015181548110611089576110896134ea565b9060005260206000200160006101000a8154816001600160a01b0302191690836001600160a01b03160217905550818054806110c7576110c76135d5565b600082815260209020810160001990810180546001600160a01b0319169055019055806110f381613531565b915050610e62565b50505050505050565b6000336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461114e5760405162461bcd60e51b81526004016106119061354c565b6000805b8381101561062657600085858381811061116e5761116e6134ea565b919091013560f81c600081815260016020526040902054909250151590506111fe5760405162461bcd60e51b815260206004820152603860248201527f5374616b6552656769737472792e7570646174654f70657261746f725374616b60448201527f653a2071756f72756d20646f6573206e6f7420657869737400000000000000006064820152608401610611565b60008061120b838b611e47565b915091508061122d5760009150600160ff84161b6001600160c01b0386161794505b600061123a8a8585612045565b905061124684826122c5565b5050505050808061125690613531565b915050611152565b60606000826001600160401b0381111561127a5761127a612f4a565b6040519080825280602002602001820160405280156112a3578160200160208202803683370190505b50905060005b8381101561151d5760008585838181106112c5576112c56134ea565b919091013560f81c600081815260016020526040902054909250151590506113645760405162461bcd60e51b815260206004820152604660248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20646f6573206e6f7460648201526508195e1a5cdd60d21b608482015260a401610611565b60ff81166000908152600160205260408120805463ffffffff8a16929061138d5761138d6134ea565b60009182526020909120015463ffffffff1611156114395760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a401610611565b60ff8116600090815260016020526040812054905b818110156115075760ff8316600090815260016020819052604090912063ffffffff8b169161147d84866135be565b61148791906135be565b81548110611497576114976134ea565b60009182526020909120015463ffffffff16116114f55760016114ba82846135be565b6114c491906135be565b8585815181106114d6576114d66134ea565b602002602001019063ffffffff16908163ffffffff1681525050611507565b806114ff81613531565b91505061144e565b505050808061151590613531565b9150506112a9565b5090505b9392505050565b6004602052816000526040600020818154811061154457600080fd5b6000918252602090912001546001600160a01b03169150829050565b60408051606081018252600080825260208083018290528284018290528582526002815283822060ff881683529052919091208054839081106115a5576115a56134ea565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091526000808252602082015260ff83166000908152600360205260409020805483908110611630576116306134ea565b6000918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260019052919091208054839081106116af576116af6134ea565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561175f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117839190613451565b6001600160a01b0316336001600160a01b0316146117b35760405162461bcd60e51b81526004016106119061346e565b816117cf8160ff16600090815260016020526040902054151590565b6117eb5760405162461bcd60e51b815260040161061190613400565b6117f5838361243f565b505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146118425760405162461bcd60e51b81526004016106119061354c565b60005b81811015611921576000838383818110611861576118616134ea565b919091013560f81c600081815260016020526040902054909250151590506118f15760405162461bcd60e51b815260206004820152603760248201527f5374616b6552656769737472792e646572656769737465724f70657261746f7260448201527f3a2071756f72756d20646f6573206e6f742065786973740000000000000000006064820152608401610611565b60006118ff86836000612045565b905061190b82826122c5565b505050808061191990613531565b915050611845565b50505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611985573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119a99190613451565b6001600160a01b0316336001600160a01b0316146119d95760405162461bcd60e51b81526004016106119061346e565b816119f58160ff16600090815260016020526040902054151590565b611a115760405162461bcd60e51b815260040161061190613400565b6117f583836124a8565b60ff83166000908152600160205260408120805482919084908110611a4257611a426134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610cd881856128eb565b60ff81166000908152600160208190526040822080549091611aba916135be565b81548110611aca57611aca6134ea565b600091825260209091200154600160401b90046001600160601b031692915050565b6000611af9848484612a65565b949350505050565b600082815260026020908152604080832060ff881684529091528120805482919084908110611b3257611b326134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611b8981866128eb565b6040015195945050505050565b6040805160608082018352600080835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382015290919081611bef579150610cc59050565b600085815260026020908152604080832060ff881684529091529020611c166001846135be565b81548110611c2657611c266134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610cc5915050565b600083815260026020908152604080832060ff861684529091528120611ca2858585612a65565b63ffffffff1681548110611cb857611cb86134ea565b600091825260209091200154600160401b90046001600160601b0316949350505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611d245760405162461bcd60e51b81526004016106119061354c565b60ff831660009081526001602052604090205415611da25760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608401610611565b611dac83826124a8565b611db6838361243f565b505060ff166000908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b600080600080611e668660ff1660009081526003602052604090205490565b604080518082019091526000808252602082015290915060ff871660009081526004602081905260408083209051639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611edb928c92016135eb565b600060405180830381865afa158015611ef8573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f20919081019061364a565b905060005b838110156120115760ff89166000908152600360205260409020805482908110611f5157611f516134ea565b60009182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611f9f57611f9f6134ea565b60200260200101511115611fff57670de0b6b3a764000083602001516001600160601b0316838381518110611fd657611fd66134ea565b6020026020010151611fe891906136da565b611ff291906136f9565b611ffc908661371b565b94505b8061200981613531565b915050611f25565b50505060ff8616600090815260208190526040902054919350506001600160601b03908116908316101590505b9250929050565b600083815260026020908152604080832060ff8616845290915281205481908061210957600086815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561226b565b600086815260026020908152604080832060ff8916845290915281206121306001846135be565b81548110612140576121406134ea565b600091825260209091200180546001600160601b03600160401b9091048116945090915085168314156121795760009350505050611521565b80544363ffffffff908116911614156121b3578054600160401b600160a01b031916600160401b6001600160601b03871602178155612269565b805467ffffffff000000001916600160201b4363ffffffff90811682810293909317845560008a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26122bb8285612bcb565b9695505050505050565b60ff8216600090815260016020819052604082208054918391906122e990846135be565b815481106122f9576122f96134ea565b90600052602060002001905083600014156123285754600160401b90046001600160601b03169150610cc59050565b805460009061234790600160401b90046001600160601b031686612be3565b82549091504363ffffffff90811691161415612384578154600160401b600160a01b031916600160401b6001600160601b03831602178255612436565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff8916600090815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b60ff82166000818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b600081511161250d5760405162461bcd60e51b815260206004820152603860248201526000805160206137fa83398151915260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608401610611565b805160ff8316600090815260036020908152604090912054906125308383613746565b11156125a05760405162461bcd60e51b815260206004820152604560248201526000805160206137fa83398151915260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a401610611565b60005b828110156128e45760005b6125b88284613746565b811015612699578482815181106125d1576125d16134ea565b6020026020010151600001516001600160a01b0316600360008860ff1660ff1681526020019081526020016000208281548110612610576126106134ea565b6000918252602090912001546001600160a01b031614156126875760405162461bcd60e51b815260206004820152603d60248201526000805160206137fa83398151915260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608401610611565b8061269181613531565b9150506125ae565b5060008482815181106126ae576126ae6134ea565b6020026020010151602001516001600160601b0316116127335760405162461bcd60e51b815260206004820152604660248201526000805160206137fa83398151915260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a401610611565b60ff851660009081526003602052604090208451859083908110612759576127596134ea565b602090810291909101810151825460018101845560009384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106127be576127be6134ea565b6020908102919091018101515182546001810184556000938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612835576128356134ea565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612892576128926134ea565b6020026020010151600001518684815181106128b0576128b06134ea565b6020026020010151602001516040516128ca929190612c6c565b60405180910390a2806128dc81613531565b9150506125a3565b5050505050565b816000015163ffffffff168163ffffffff1610156129905760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a401610611565b602082015163ffffffff1615806129b65750816020015163ffffffff168163ffffffff16105b612a615760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c401610611565b5050565b600083815260026020908152604080832060ff86168452909152812054805b8015612b0657600086815260026020908152604080832060ff89168452909152902063ffffffff851690612ab96001846135be565b81548110612ac957612ac96134ea565b60009182526020909120015463ffffffff1611612af457612aeb6001826135be565b92505050611521565b80612afe8161375e565b915050612a84565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e401610611565b60006115216001600160601b03808516908416613775565b600080821215612c0757612bf6826137b4565b612c0090846137d1565b9050610cc5565b612c00828461371b565b803560ff81168114612c2257600080fd5b919050565b600060208284031215612c3957600080fd5b61152182612c11565b60008060408385031215612c5557600080fd5b612c5e83612c11565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114612ca357600080fd5b50565b60008060408385031215612cb957600080fd5b612cc283612c11565b91506020830135612cd281612c8e565b809150509250929050565b60008083601f840112612cef57600080fd5b5081356001600160401b03811115612d0657600080fd5b6020830191508360208260051b850101111561203e57600080fd5b600080600080600060608688031215612d3957600080fd5b612d4286612c11565b945060208601356001600160401b0380821115612d5e57600080fd5b612d6a89838a01612cdd565b90965094506040880135915080821115612d8357600080fd5b50612d9088828901612cdd565b969995985093965092949392505050565b60008083601f840112612db357600080fd5b5081356001600160401b03811115612dca57600080fd5b60208301915083602082850101111561203e57600080fd5b60008060008060608587031215612df857600080fd5b8435612e0381612c8e565b93506020850135925060408501356001600160401b03811115612e2557600080fd5b612e3187828801612da1565b95989497509550505050565b600081518084526020808501945080840160005b83811015612e765781516001600160601b031687529582019590820190600101612e51565b509495945050505050565b604081526000612e946040830185612e3d565b82810360208401526124368185612e3d565b60008060408385031215612eb957600080fd5b82359150612ec960208401612c11565b90509250929050565b6020808252825182820181905260009190848201906040850190845b81811015612f3e57612f2b83855163ffffffff808251168352806020830151166020840152506001600160601b0360408201511660408301525050565b9284019260609290920191600101612eee565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715612f8257612f82612f4a565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612fb057612fb0612f4a565b604052919050565b60006001600160401b03821115612fd157612fd1612f4a565b5060051b60200190565b60008060408385031215612fee57600080fd5b612ff783612c11565b91506020808401356001600160401b0381111561301357600080fd5b8401601f8101861361302457600080fd5b803561303761303282612fb8565b612f88565b81815260059190911b8201830190838101908883111561305657600080fd5b928401925b828410156130745783358252928401929084019061305b565b80955050505050509250929050565b803563ffffffff81168114612c2257600080fd5b6000806000604084860312156130ac57600080fd5b6130b584613083565b925060208401356001600160401b038111156130d057600080fd5b6130dc86828701612da1565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015612f3e57835163ffffffff1683529284019291840191600101613105565b60008060006060848603121561313c57600080fd5b61314584612c11565b95602085013595506040909401359392505050565b815163ffffffff9081168252602080840151909116908201526040808301516001600160601b03169082015260608101610cc5565b80356001600160601b0381168114612c2257600080fd5b600080604083850312156131b957600080fd5b6131c283612c11565b9150612ec96020840161318f565b6000806000604084860312156131e557600080fd5b8335925060208401356001600160401b038111156130d057600080fd5b600082601f83011261321357600080fd5b8135602061322361303283612fb8565b82815260069290921b8401810191818101908684111561324257600080fd5b8286015b84811015613291576040818903121561325f5760008081fd5b613267612f60565b813561327281612c8e565b815261327f82860161318f565b81860152835291830191604001613246565b509695505050505050565b600080604083850312156132af57600080fd5b6132b883612c11565b915060208301356001600160401b038111156132d357600080fd5b6132df85828601613202565b9150509250929050565b6000806000606084860312156132fe57600080fd5b61330784612c11565b925061331560208501613083565b9150604084013590509250925092565b60008060006060848603121561333a57600080fd5b8335925061334a60208501612c11565b915061335860408501613083565b90509250925092565b6000806000806080858703121561337757600080fd5b61338085612c11565b935061338e60208601613083565b93969395505050506040820135916060013590565b6000806000606084860312156133b857600080fd5b6133c184612c11565b92506133cf6020850161318f565b915060408401356001600160401b038111156133ea57600080fd5b6133f686828701613202565b9150509250925092565b60208082526031908201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726040820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b606082015260800190565b60006020828403121561346357600080fd5b815161152181612c8e565b60208082526056908201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60408201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746060820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561351257600080fd5b6115218261318f565b634e487b7160e01b600052601160045260246000fd5b60006000198214156135455761354561351b565b5060010190565b6020808252604c908201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960408201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260608201526b3ca1b7b7b93234b730ba37b960a11b608082015260a00190565b6000828210156135d0576135d061351b565b500390565b634e487b7160e01b600052603160045260246000fd5b60006040820160018060a01b03808616845260206040818601528286548085526060870191508760005282600020945060005b8181101561363c57855485168352600195860195928401920161361e565b509098975050505050505050565b6000602080838503121561365d57600080fd5b82516001600160401b0381111561367357600080fd5b8301601f8101851361368457600080fd5b805161369261303282612fb8565b81815260059190911b820183019083810190878311156136b157600080fd5b928401925b828410156136cf578351825292840192908401906136b6565b979650505050505050565b60008160001904831182151516156136f4576136f461351b565b500290565b60008261371657634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160601b0380831681851680830382111561373d5761373d61351b565b01949350505050565b600082198211156137595761375961351b565b500190565b60008161376d5761376d61351b565b506000190190565b60008083128015600160ff1b8501841216156137935761379361351b565b6001600160ff1b03840183138116156137ae576137ae61351b565b50500390565b6000600160ff1b8214156137ca576137ca61351b565b5060000390565b60006001600160601b03838116908316818110156137f1576137f161351b565b03939250505056fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a264697066735822122014c70732d3158fe130cdb5a2d819a732d72da424c0223c808ae97682e4399e0c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\09X8\x03\x80b\09X\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0eV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\x80Rb\0\0\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0bW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0yW`\0\x80\xFD[\x82Qb\0\0\x86\x81b\0\0LV[` \x84\x01Q\x90\x92Pb\0\0\x99\x81b\0\0LV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa8Ob\0\x01\t`\09`\0\x81\x81a\x03z\x01R\x81\x81a\x062\x01R\x81\x81a\to\x01R\x81\x81a\x0C\xE6\x01R\x81\x81a\x11\x11\x01R\x81\x81a\x17\x03\x01R\x81\x81a\x18\x05\x01R\x81\x81a\x19)\x01Ra\x1C\xE7\x01R`\0\x81\x81a\x05)\x01Ra\x1E\xA8\x01Ra8O`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01\x0FW\x80c\xC8)LV\x11a\0\xA2W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x05KW\x80c\xF8Q\xE1\x98\x14a\x05^W\x80c\xFA(\xC6'\x14a\x05qW\x80c\xFFiJw\x14a\x05\x84W`\0\x80\xFD[\x80c\xC8)LV\x14a\x04\xD6W\x80c\xD5\xEC\xCC\x05\x14a\x04\xE9W\x80c\xDD\x98F\xB9\x14a\x04\xFCW\x80c\xDF\\\xF7#\x14a\x05$W`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xDEW\x80c\xBC\x9A@\xC3\x14a\x04tW\x80c\xBD)\xB8\xCD\x14a\x04\x87W\x80c\xC4gx\xA5\x14a\x04\x9AW\x80c\xC6\x01R}\x14a\x04\xC3W`\0\x80\xFD[\x80c\x9F<\xCFe\x14a\x03\xEEW\x80c\xACk\xFB\x03\x14a\x04\x01W\x80c\xAD\xC8\x04\xDA\x14a\x04!W\x80c\xB6\x90Kx\x14a\x04aW`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x87W\x80cf\xAC\xFE\xFE\x11a\x01VW\x80cf\xAC\xFE\xFE\x14a\x03JW\x80cm\x14\xA9\x87\x14a\x03uW\x80c|\x17#G\x14a\x03\xB4W\x80c\x81\xC0u\x02\x14a\x03\xCEW`\0\x80\xFD[\x80cK\xD2n\t\x14a\x02\xE5W\x80cT\x01\xED'\x14a\x03\x15W\x80c^Zgu\x14a\x03(W\x80c_\x1F-w\x14a\x037W`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xC3W\x80c \xB6b\x98\x14a\x02lW\x80c%PGw\x14a\x02\x81W\x80c,\xD9Y@\x14a\x02\xA2W\x80c<\xA5\xA5\xF5\x14a\x02\xC2W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xEAW\x80c\x08s$a\x14a\x02 W\x80c\x1F\x9Bt\xE0\x14a\x02AW[`\0\x80\xFD[a\x02\ra\x01\xF86`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a,BV[a\x05\x97V[`@Qa\x02\x17\x92\x91\x90a,lV[a\x02Ta\x02O6`\x04a,\xA6V[a\x05\xE0V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\x7Fa\x02z6`\x04a-!V[a\x060V[\0[a\x02\x94a\x02\x8F6`\x04a-\xE2V[a\taV[`@Qa\x02\x17\x92\x91\x90a.\x81V[a\x02\xB5a\x02\xB06`\x04a.\xA6V[a\x0C,V[`@Qa\x02\x17\x91\x90a.\xD2V[a\x02\ra\x02\xD06`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\ra\x02\xF36`\x04a.\xA6V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02Ta\x03#6`\x04a.\xA6V[a\x0C\xCBV[a\x02\rg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x7Fa\x03E6`\x04a/\xDBV[a\x0C\xE4V[a\x03]a\x03X6`\x04a-\xE2V[a\x11\x04V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xBC` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xE1a\x03\xDC6`\x04a0\x97V[a\x12^V[`@Qa\x02\x17\x91\x90a0\xE9V[a\x03\x9Ca\x03\xFC6`\x04a,BV[a\x15(V[a\x04\x14a\x04\x0F6`\x04a1'V[a\x15`V[`@Qa\x02\x17\x91\x90a1ZV[a\x044a\x04/6`\x04a,BV[a\x15\xF8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x17V[a\x04\x14a\x04o6`\x04a,BV[a\x16rV[a\x02\x7Fa\x04\x826`\x04a1\xA6V[a\x17\x01V[a\x02\x7Fa\x04\x956`\x04a1\xD0V[a\x17\xFAV[a\x02Ta\x04\xA86`\x04a,'V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x7Fa\x04\xD16`\x04a2\x9CV[a\x19'V[a\x02Ta\x04\xE46`\x04a2\xE9V[a\x1A\x1BV[a\x02Ta\x04\xF76`\x04a,'V[a\x1A\x99V[a\x05\x0Fa\x05\n6`\x04a3%V[a\x1A\xECV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Ta\x05Y6`\x04a3aV[a\x1B\x01V[a\x04\x14a\x05l6`\x04a.\xA6V[a\x1B\x96V[a\x02Ta\x05\x7F6`\x04a3%V[a\x1C{V[a\x02\x7Fa\x05\x926`\x04a3\xA3V[a\x1C\xDCV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\xB3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06&\x85\x85a\x1EGV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x84a\x06\xFE\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x83\x80a\x07\x90W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x11V[\x82\x81\x14a\x08\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\tVW\x85\x85\x82\x81\x81\x10a\x083Wa\x083a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x08H\x91\x90a5\0V[\x82\x89\x89\x84\x81\x81\x10a\x08[Wa\x08[a4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08rWa\x08ra4\xEAV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\xDBWa\x08\xDBa4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF2Wa\x08\xF2a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x19Wa\t\x19a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t.\x91\x90a5\0V[`@Qa\t<\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a\tN\x81a51V[\x91PPa\x08\x19V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xC6Wa\t\xC6a/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x0CWa\n\x0Ca/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x1EW`\0\x87\x87\x83\x81\x81\x10a\nWWa\nWa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\n\xEC\x83\x8Da\x1EGV[\x91P\x91P\x80a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0a\x0B\x96\x8C\x85\x85a EV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B\xABWa\x0B\xABa4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0B\xD5\x84\x82a\"\xC5V[\x86\x86\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x16\x90a51V[\x91PPa\n;V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C\xBEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0CeV[PPPP\x90P[\x92\x91PPV[`\0\x80a\x0C\xD8\x84\x84a\x1B\x96V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rf\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\r\xB2\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x81Q\x80a\x0ECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x10\xFBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0E\xA2Wa\x0E\xA2a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0E\xBAWa\x0E\xBAa4\xEAV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F0Wa\x0F0a4\xEAV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0Fp\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x0F\x80Wa\x0F\x80a4\xEAV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x0F\x9DWa\x0F\x9Da4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB5Wa\x0F\xB5a4\xEAV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x08Wa\x10\x08a5\xD5V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x100\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x10@Wa\x10@a4\xEAV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10qWa\x10qa4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x89Wa\x10\x89a4\xEAV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x10\xC7Wa\x10\xC7a5\xD5V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x10\xF3\x81a51V[\x91PPa\x0EbV[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x80[\x83\x81\x10\x15a\x06&W`\0\x85\x85\x83\x81\x81\x10a\x11nWa\x11na4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\x12\x0B\x83\x8Ba\x1EGV[\x91P\x91P\x80a\x12-W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12:\x8A\x85\x85a EV[\x90Pa\x12F\x84\x82a\"\xC5V[PPPPP\x80\x80a\x12V\x90a51V[\x91PPa\x11RV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12zWa\x12za/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\x1DW`\0\x85\x85\x83\x81\x81\x10a\x12\xC5Wa\x12\xC5a4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x13\x8DWa\x13\x8Da4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x149W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\x07W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x14}\x84\x86a5\xBEV[a\x14\x87\x91\x90a5\xBEV[\x81T\x81\x10a\x14\x97Wa\x14\x97a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\xF5W`\x01a\x14\xBA\x82\x84a5\xBEV[a\x14\xC4\x91\x90a5\xBEV[\x85\x85\x81Q\x81\x10a\x14\xD6Wa\x14\xD6a4\xEAV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\x07V[\x80a\x14\xFF\x81a51V[\x91PPa\x14NV[PPP\x80\x80a\x15\x15\x90a51V[\x91PPa\x12\xA9V[P\x90P[\x93\x92PPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xA5Wa\x15\xA5a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x160Wa\x160a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16\xAFWa\x16\xAFa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x83\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x17\xCF\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x17\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$?V[PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0[\x81\x81\x10\x15a\x19!W`\0\x83\x83\x83\x81\x81\x10a\x18aWa\x18aa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0a\x18\xFF\x86\x83`\0a EV[\x90Pa\x19\x0B\x82\x82a\"\xC5V[PPP\x80\x80a\x19\x19\x90a51V[\x91PPa\x18EV[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA9\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x19\xF5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$\xA8V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1ABWa\x1ABa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0C\xD8\x81\x85a(\xEBV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A\xBA\x91a5\xBEV[\x81T\x81\x10a\x1A\xCAWa\x1A\xCAa4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x1A\xF9\x84\x84\x84a*eV[\x94\x93PPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B2Wa\x1B2a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B\x89\x81\x86a(\xEBV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1B\xEFW\x91Pa\x0C\xC5\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\x16`\x01\x84a5\xBEV[\x81T\x81\x10a\x1C&Wa\x1C&a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C\xC5\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1C\xA2\x85\x85\x85a*eV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1D\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x11V[a\x1D\xAC\x83\x82a$\xA8V[a\x1D\xB6\x83\x83a$?V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\0\x80`\0\x80a\x1Ef\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1E\xDB\x92\x8C\x92\x01a5\xEBV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F \x91\x90\x81\x01\x90a6JV[\x90P`\0[\x83\x81\x10\x15a \x11W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1FQWa\x1FQa4\xEAV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1F\x9FWa\x1F\x9Fa4\xEAV[` \x02` \x01\x01Q\x11\x15a\x1F\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1F\xD6Wa\x1F\xD6a4\xEAV[` \x02` \x01\x01Qa\x1F\xE8\x91\x90a6\xDAV[a\x1F\xF2\x91\x90a6\xF9V[a\x1F\xFC\x90\x86a7\x1BV[\x94P[\x80a \t\x81a51V[\x91PPa\x1F%V[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a!\tW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"kV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!0`\x01\x84a5\xBEV[\x81T\x81\x10a!@Wa!@a4\xEAV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!yW`\0\x93PPPPa\x15!V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xB3W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"iV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xBB\x82\x85a+\xCBV[\x96\x95PPPPPPV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\xE9\x90\x84a5\xBEV[\x81T\x81\x10a\"\xF9Wa\"\xF9a4\xEAV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a#(WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C\xC5\x90PV[\x80T`\0\x90a#G\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xE3V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\x84W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua$6V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a%0\x83\x83a7FV[\x11\x15a%\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0[\x82\x81\x10\x15a(\xE4W`\0[a%\xB8\x82\x84a7FV[\x81\x10\x15a&\x99W\x84\x82\x81Q\x81\x10a%\xD1Wa%\xD1a4\xEAV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\x10Wa&\x10a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a&\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80a&\x91\x81a51V[\x91PPa%\xAEV[P`\0\x84\x82\x81Q\x81\x10a&\xAEWa&\xAEa4\xEAV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'YWa'Ya4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xBEWa'\xBEa4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(5Wa(5a4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x92Wa(\x92a4\xEAV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a4\xEAV[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a(\xDC\x81a51V[\x91PPa%\xA3V[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\xB6WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x11V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\x06W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a*\xB9`\x01\x84a5\xBEV[\x81T\x81\x10a*\xC9Wa*\xC9a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xF4Wa*\xEB`\x01\x82a5\xBEV[\x92PPPa\x15!V[\x80a*\xFE\x81a7^V[\x91PPa*\x84V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x11V[`\0a\x15!`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a7uV[`\0\x80\x82\x12\x15a,\x07Wa+\xF6\x82a7\xB4V[a,\0\x90\x84a7\xD1V[\x90Pa\x0C\xC5V[a,\0\x82\x84a7\x1BV[\x805`\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,9W`\0\x80\xFD[a\x15!\x82a,\x11V[`\0\x80`@\x83\x85\x03\x12\x15a,UW`\0\x80\xFD[a,^\x83a,\x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,\xA3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xB9W`\0\x80\xFD[a,\xC2\x83a,\x11V[\x91P` \x83\x015a,\xD2\x81a,\x8EV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,\xEFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x06W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-9W`\0\x80\xFD[a-B\x86a,\x11V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-^W`\0\x80\xFD[a-j\x89\x83\x8A\x01a,\xDDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\x83W`\0\x80\xFD[Pa-\x90\x88\x82\x89\x01a,\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-\xB3W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\xF8W`\0\x80\xFD[\x845a.\x03\x81a,\x8EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.%W`\0\x80\xFD[a.1\x87\x82\x88\x01a-\xA1V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a.vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.QV[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x94`@\x83\x01\x85a.=V[\x82\x81\x03` \x84\x01Ra$6\x81\x85a.=V[`\0\x80`@\x83\x85\x03\x12\x15a.\xB9W`\0\x80\xFD[\x825\x91Pa.\xC9` \x84\x01a,\x11V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>Wa/+\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xEEV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\x82Wa/\x82a/JV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xB0Wa/\xB0a/JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xD1Wa/\xD1a/JV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/\xEEW`\0\x80\xFD[a/\xF7\x83a,\x11V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x13W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0$W`\0\x80\xFD[\x805a07a02\x82a/\xB8V[a/\x88V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a0tW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a0[V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[a0\xB5\x84a0\x83V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[a0\xDC\x86\x82\x87\x01a-\xA1V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x05V[`\0\x80`\0``\x84\x86\x03\x12\x15a1<W`\0\x80\xFD[a1E\x84a,\x11V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0C\xC5V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xB9W`\0\x80\xFD[a1\xC2\x83a,\x11V[\x91Pa.\xC9` \x84\x01a1\x8FV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xE5W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\x13W`\0\x80\xFD[\x815` a2#a02\x83a/\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2\x91W`@\x81\x89\x03\x12\x15a2_W`\0\x80\x81\xFD[a2ga/`V[\x815a2r\x81a,\x8EV[\x81Ra2\x7F\x82\x86\x01a1\x8FV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xAFW`\0\x80\xFD[a2\xB8\x83a,\x11V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xD3W`\0\x80\xFD[a2\xDF\x85\x82\x86\x01a2\x02V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xFEW`\0\x80\xFD[a3\x07\x84a,\x11V[\x92Pa3\x15` \x85\x01a0\x83V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3:W`\0\x80\xFD[\x835\x92Pa3J` \x85\x01a,\x11V[\x91Pa3X`@\x85\x01a0\x83V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3wW`\0\x80\xFD[a3\x80\x85a,\x11V[\x93Pa3\x8E` \x86\x01a0\x83V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xB8W`\0\x80\xFD[a3\xC1\x84a,\x11V[\x92Pa3\xCF` \x85\x01a1\x8FV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW`\0\x80\xFD[a3\xF6\x86\x82\x87\x01a2\x02V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4cW`\0\x80\xFD[\x81Qa\x15!\x81a,\x8EV[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x12W`\0\x80\xFD[a\x15!\x82a1\x8FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a5EWa5Ea5\x1BV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xD0Wa5\xD0a5\x1BV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a6<W\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a6\x1EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a6]W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6sW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\x84W`\0\x80\xFD[\x80Qa6\x92a02\x82a/\xB8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a6\xB1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\xCFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a6\xB6V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\xF4Wa6\xF4a5\x1BV[P\x02\x90V[`\0\x82a7\x16WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7=Wa7=a5\x1BV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a7YWa7Ya5\x1BV[P\x01\x90V[`\0\x81a7mWa7ma5\x1BV[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a7\x93Wa7\x93a5\x1BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a7\xAEWa7\xAEa5\x1BV[PP\x03\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a7\xCAWa7\xCAa5\x1BV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7\xF1Wa7\xF1a5\x1BV[\x03\x93\x92PPPV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x14\xC7\x072\xD3\x15\x8F\xE10\xCD\xB5\xA2\xD8\x19\xA72\xD7-\xA4$\xC0\"<\x80\x8A\xE9v\x82\xE49\x9E\x0CdsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106101e55760003560e01c80639f3ccf651161010f578063c8294c56116100a2578063f2be94ae11610071578063f2be94ae1461054b578063f851e1981461055e578063fa28c62714610571578063ff694a771461058457600080fd5b8063c8294c56146104d6578063d5eccc05146104e9578063dd9846b9146104fc578063df5cf7231461052457600080fd5b8063bc9a40c3116100de578063bc9a40c314610474578063bd29b8cd14610487578063c46778a51461049a578063c601527d146104c357600080fd5b80639f3ccf65146103ee578063ac6bfb0314610401578063adc804da14610421578063b6904b781461046157600080fd5b80634bd26e091161018757806366acfefe1161015657806366acfefe1461034a5780636d14a987146103755780637c172347146103b457806381c07502146103ce57600080fd5b80634bd26e09146102e55780635401ed27146103155780635e5a6775146103285780635f1f2d771461033757600080fd5b806320b66298116101c357806320b662981461026c57806325504777146102815780632cd95940146102a25780633ca5a5f5146102c257600080fd5b80630491b41c146101ea57806308732461146102205780631f9b74e014610241575b600080fd5b61020d6101f8366004612c27565b60ff1660009081526001602052604090205490565b6040519081526020015b60405180910390f35b61023361022e366004612c42565b610597565b604051610217929190612c6c565b61025461024f366004612ca6565b6105e0565b6040516001600160601b039091168152602001610217565b61027f61027a366004612d21565b610630565b005b61029461028f366004612de2565b610961565b604051610217929190612e81565b6102b56102b0366004612ea6565b610c2c565b6040516102179190612ed2565b61020d6102d0366004612c27565b60ff1660009081526003602052604090205490565b61020d6102f3366004612ea6565b600091825260026020908152604080842060ff93909316845291905290205490565b610254610323366004612ea6565b610ccb565b61020d670de0b6b3a764000081565b61027f610345366004612fdb565b610ce4565b61035d610358366004612de2565b611104565b6040516001600160c01b039091168152602001610217565b61039c7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610217565b6103bc602081565b60405160ff9091168152602001610217565b6103e16103dc366004613097565b61125e565b60405161021791906130e9565b61039c6103fc366004612c42565b611528565b61041461040f366004613127565b611560565b604051610217919061315a565b61043461042f366004612c42565b6115f8565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610217565b61041461046f366004612c42565b611672565b61027f6104823660046131a6565b611701565b61027f6104953660046131d0565b6117fa565b6102546104a8366004612c27565b6000602081905290815260409020546001600160601b031681565b61027f6104d136600461329c565b611927565b6102546104e43660046132e9565b611a1b565b6102546104f7366004612c27565b611a99565b61050f61050a366004613325565b611aec565b60405163ffffffff9091168152602001610217565b61039c7f000000000000000000000000000000000000000000000000000000000000000081565b610254610559366004613361565b611b01565b61041461056c366004612ea6565b611b96565b61025461057f366004613325565b611c7b565b61027f6105923660046133a3565b611cdc565b600360205281600052604060002081815481106105b357600080fd5b6000918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b60ff8216600090815260016020526040812054839061061a5760405162461bcd60e51b815260040161061190613400565b60405180910390fd5b60006106268585611e47565b5095945050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561068e573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106b29190613451565b6001600160a01b0316336001600160a01b0316146106e25760405162461bcd60e51b81526004016106119061346e565b846106fe8160ff16600090815260016020526040902054151590565b61071a5760405162461bcd60e51b815260040161061190613400565b8380610790576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152608401610611565b8281146108055760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608401610611565b60ff87166000908152600360205260408120905b8281101561095657858582818110610833576108336134ea565b90506020020160208101906108489190613500565b8289898481811061085b5761085b6134ea565b9050602002013581548110610872576108726134ea565b9060005260206000200160000160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a858181106108db576108db6134ea565b90506020020135815481106108f2576108f26134ea565b6000918252602090912001546001600160a01b0316888885818110610919576109196134ea565b905060200201602081019061092e9190613500565b60405161093c929190612c6c565b60405180910390a28061094e81613531565b915050610819565b505050505050505050565b606080336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146109ac5760405162461bcd60e51b81526004016106119061354c565b6000836001600160401b038111156109c6576109c6612f4a565b6040519080825280602002602001820160405280156109ef578160200160208202803683370190505b5090506000846001600160401b03811115610a0c57610a0c612f4a565b604051908082528060200260200182016040528015610a35578160200160208202803683370190505b50905060005b85811015610c1e576000878783818110610a5757610a576134ea565b919091013560f81c60008181526001602052604090205490925015159050610adf5760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608401610611565b600080610aec838d611e47565b9150915080610b895760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a401610611565b6000610b968c8585612045565b905082878681518110610bab57610bab6134ea565b60200260200101906001600160601b031690816001600160601b031681525050610bd584826122c5565b868681518110610be757610be76134ea565b60200260200101906001600160601b031690816001600160601b031681525050505050508080610c1690613531565b915050610a3b565b509097909650945050505050565b600082815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610cbe576000848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610c65565b5050505090505b92915050565b600080610cd88484611b96565b60400151949350505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610d42573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610d669190613451565b6001600160a01b0316336001600160a01b031614610d965760405162461bcd60e51b81526004016106119061346e565b81610db28160ff16600090815260016020526040902054151590565b610dce5760405162461bcd60e51b815260040161061190613400565b815180610e435760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608401610611565b60ff841660009081526003602090815260408083206004909252822090915b838110156110fb578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610ea257610ea26134ea565b602002602001015181548110610eba57610eba6134ea565b600091825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610f1857610f186134ea565b602002602001015181548110610f3057610f306134ea565b600091825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610f70906001906135be565b81548110610f8057610f806134ea565b9060005260206000200183878381518110610f9d57610f9d6134ea565b602002602001015181548110610fb557610fb56134ea565b600091825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558254839080611008576110086135d5565b60008281526020812082016000199081019190915501905581548290611030906001906135be565b81548110611040576110406134ea565b9060005260206000200160009054906101000a90046001600160a01b031682878381518110611071576110716134ea565b602002602001015181548110611089576110896134ea565b9060005260206000200160006101000a8154816001600160a01b0302191690836001600160a01b03160217905550818054806110c7576110c76135d5565b600082815260209020810160001990810180546001600160a01b0319169055019055806110f381613531565b915050610e62565b50505050505050565b6000336001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000161461114e5760405162461bcd60e51b81526004016106119061354c565b6000805b8381101561062657600085858381811061116e5761116e6134ea565b919091013560f81c600081815260016020526040902054909250151590506111fe5760405162461bcd60e51b815260206004820152603860248201527f5374616b6552656769737472792e7570646174654f70657261746f725374616b60448201527f653a2071756f72756d20646f6573206e6f7420657869737400000000000000006064820152608401610611565b60008061120b838b611e47565b915091508061122d5760009150600160ff84161b6001600160c01b0386161794505b600061123a8a8585612045565b905061124684826122c5565b5050505050808061125690613531565b915050611152565b60606000826001600160401b0381111561127a5761127a612f4a565b6040519080825280602002602001820160405280156112a3578160200160208202803683370190505b50905060005b8381101561151d5760008585838181106112c5576112c56134ea565b919091013560f81c600081815260016020526040902054909250151590506113645760405162461bcd60e51b815260206004820152604660248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20646f6573206e6f7460648201526508195e1a5cdd60d21b608482015260a401610611565b60ff81166000908152600160205260408120805463ffffffff8a16929061138d5761138d6134ea565b60009182526020909120015463ffffffff1611156114395760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a401610611565b60ff8116600090815260016020526040812054905b818110156115075760ff8316600090815260016020819052604090912063ffffffff8b169161147d84866135be565b61148791906135be565b81548110611497576114976134ea565b60009182526020909120015463ffffffff16116114f55760016114ba82846135be565b6114c491906135be565b8585815181106114d6576114d66134ea565b602002602001019063ffffffff16908163ffffffff1681525050611507565b806114ff81613531565b91505061144e565b505050808061151590613531565b9150506112a9565b5090505b9392505050565b6004602052816000526040600020818154811061154457600080fd5b6000918252602090912001546001600160a01b03169150829050565b60408051606081018252600080825260208083018290528284018290528582526002815283822060ff881683529052919091208054839081106115a5576115a56134ea565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091526000808252602082015260ff83166000908152600360205260409020805483908110611630576116306134ea565b6000918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182526000808252602080830182905282840182905260ff8616825260019052919091208054839081106116af576116af6134ea565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa15801561175f573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906117839190613451565b6001600160a01b0316336001600160a01b0316146117b35760405162461bcd60e51b81526004016106119061346e565b816117cf8160ff16600090815260016020526040902054151590565b6117eb5760405162461bcd60e51b815260040161061190613400565b6117f5838361243f565b505050565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146118425760405162461bcd60e51b81526004016106119061354c565b60005b81811015611921576000838383818110611861576118616134ea565b919091013560f81c600081815260016020526040902054909250151590506118f15760405162461bcd60e51b815260206004820152603760248201527f5374616b6552656769737472792e646572656769737465724f70657261746f7260448201527f3a2071756f72756d20646f6573206e6f742065786973740000000000000000006064820152608401610611565b60006118ff86836000612045565b905061190b82826122c5565b505050808061191990613531565b915050611845565b50505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611985573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906119a99190613451565b6001600160a01b0316336001600160a01b0316146119d95760405162461bcd60e51b81526004016106119061346e565b816119f58160ff16600090815260016020526040902054151590565b611a115760405162461bcd60e51b815260040161061190613400565b6117f583836124a8565b60ff83166000908152600160205260408120805482919084908110611a4257611a426134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610cd881856128eb565b60ff81166000908152600160208190526040822080549091611aba916135be565b81548110611aca57611aca6134ea565b600091825260209091200154600160401b90046001600160601b031692915050565b6000611af9848484612a65565b949350505050565b600082815260026020908152604080832060ff881684529091528120805482919084908110611b3257611b326134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611b8981866128eb565b6040015195945050505050565b6040805160608082018352600080835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382015290919081611bef579150610cc59050565b600085815260026020908152604080832060ff881684529091529020611c166001846135be565b81548110611c2657611c266134ea565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610cc5915050565b600083815260026020908152604080832060ff861684529091528120611ca2858585612a65565b63ffffffff1681548110611cb857611cb86134ea565b600091825260209091200154600160401b90046001600160601b0316949350505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611d245760405162461bcd60e51b81526004016106119061354c565b60ff831660009081526001602052604090205415611da25760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608401610611565b611dac83826124a8565b611db6838361243f565b505060ff166000908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b600080600080611e668660ff1660009081526003602052604090205490565b604080518082019091526000808252602082015290915060ff871660009081526004602081905260408083209051639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611edb928c92016135eb565b600060405180830381865afa158015611ef8573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052611f20919081019061364a565b905060005b838110156120115760ff89166000908152600360205260409020805482908110611f5157611f516134ea565b60009182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611f9f57611f9f6134ea565b60200260200101511115611fff57670de0b6b3a764000083602001516001600160601b0316838381518110611fd657611fd66134ea565b6020026020010151611fe891906136da565b611ff291906136f9565b611ffc908661371b565b94505b8061200981613531565b915050611f25565b50505060ff8616600090815260208190526040902054919350506001600160601b03908116908316101590505b9250929050565b600083815260026020908152604080832060ff8616845290915281205481908061210957600086815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561226b565b600086815260026020908152604080832060ff8916845290915281206121306001846135be565b81548110612140576121406134ea565b600091825260209091200180546001600160601b03600160401b9091048116945090915085168314156121795760009350505050611521565b80544363ffffffff908116911614156121b3578054600160401b600160a01b031916600160401b6001600160601b03871602178155612269565b805467ffffffff000000001916600160201b4363ffffffff90811682810293909317845560008a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26122bb8285612bcb565b9695505050505050565b60ff8216600090815260016020819052604082208054918391906122e990846135be565b815481106122f9576122f96134ea565b90600052602060002001905083600014156123285754600160401b90046001600160601b03169150610cc59050565b805460009061234790600160401b90046001600160601b031686612be3565b82549091504363ffffffff90811691161415612384578154600160401b600160a01b031916600160401b6001600160601b03831602178255612436565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff8916600090815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b60ff82166000818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b600081511161250d5760405162461bcd60e51b815260206004820152603860248201526000805160206137fa83398151915260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608401610611565b805160ff8316600090815260036020908152604090912054906125308383613746565b11156125a05760405162461bcd60e51b815260206004820152604560248201526000805160206137fa83398151915260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a401610611565b60005b828110156128e45760005b6125b88284613746565b811015612699578482815181106125d1576125d16134ea565b6020026020010151600001516001600160a01b0316600360008860ff1660ff1681526020019081526020016000208281548110612610576126106134ea565b6000918252602090912001546001600160a01b031614156126875760405162461bcd60e51b815260206004820152603d60248201526000805160206137fa83398151915260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608401610611565b8061269181613531565b9150506125ae565b5060008482815181106126ae576126ae6134ea565b6020026020010151602001516001600160601b0316116127335760405162461bcd60e51b815260206004820152604660248201526000805160206137fa83398151915260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a401610611565b60ff851660009081526003602052604090208451859083908110612759576127596134ea565b602090810291909101810151825460018101845560009384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106127be576127be6134ea565b6020908102919091018101515182546001810184556000938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612835576128356134ea565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612892576128926134ea565b6020026020010151600001518684815181106128b0576128b06134ea565b6020026020010151602001516040516128ca929190612c6c565b60405180910390a2806128dc81613531565b9150506125a3565b5050505050565b816000015163ffffffff168163ffffffff1610156129905760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a401610611565b602082015163ffffffff1615806129b65750816020015163ffffffff168163ffffffff16105b612a615760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c401610611565b5050565b600083815260026020908152604080832060ff86168452909152812054805b8015612b0657600086815260026020908152604080832060ff89168452909152902063ffffffff851690612ab96001846135be565b81548110612ac957612ac96134ea565b60009182526020909120015463ffffffff1611612af457612aeb6001826135be565b92505050611521565b80612afe8161375e565b915050612a84565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e401610611565b60006115216001600160601b03808516908416613775565b600080821215612c0757612bf6826137b4565b612c0090846137d1565b9050610cc5565b612c00828461371b565b803560ff81168114612c2257600080fd5b919050565b600060208284031215612c3957600080fd5b61152182612c11565b60008060408385031215612c5557600080fd5b612c5e83612c11565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114612ca357600080fd5b50565b60008060408385031215612cb957600080fd5b612cc283612c11565b91506020830135612cd281612c8e565b809150509250929050565b60008083601f840112612cef57600080fd5b5081356001600160401b03811115612d0657600080fd5b6020830191508360208260051b850101111561203e57600080fd5b600080600080600060608688031215612d3957600080fd5b612d4286612c11565b945060208601356001600160401b0380821115612d5e57600080fd5b612d6a89838a01612cdd565b90965094506040880135915080821115612d8357600080fd5b50612d9088828901612cdd565b969995985093965092949392505050565b60008083601f840112612db357600080fd5b5081356001600160401b03811115612dca57600080fd5b60208301915083602082850101111561203e57600080fd5b60008060008060608587031215612df857600080fd5b8435612e0381612c8e565b93506020850135925060408501356001600160401b03811115612e2557600080fd5b612e3187828801612da1565b95989497509550505050565b600081518084526020808501945080840160005b83811015612e765781516001600160601b031687529582019590820190600101612e51565b509495945050505050565b604081526000612e946040830185612e3d565b82810360208401526124368185612e3d565b60008060408385031215612eb957600080fd5b82359150612ec960208401612c11565b90509250929050565b6020808252825182820181905260009190848201906040850190845b81811015612f3e57612f2b83855163ffffffff808251168352806020830151166020840152506001600160601b0360408201511660408301525050565b9284019260609290920191600101612eee565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b0381118282101715612f8257612f82612f4a565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612fb057612fb0612f4a565b604052919050565b60006001600160401b03821115612fd157612fd1612f4a565b5060051b60200190565b60008060408385031215612fee57600080fd5b612ff783612c11565b91506020808401356001600160401b0381111561301357600080fd5b8401601f8101861361302457600080fd5b803561303761303282612fb8565b612f88565b81815260059190911b8201830190838101908883111561305657600080fd5b928401925b828410156130745783358252928401929084019061305b565b80955050505050509250929050565b803563ffffffff81168114612c2257600080fd5b6000806000604084860312156130ac57600080fd5b6130b584613083565b925060208401356001600160401b038111156130d057600080fd5b6130dc86828701612da1565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015612f3e57835163ffffffff1683529284019291840191600101613105565b60008060006060848603121561313c57600080fd5b61314584612c11565b95602085013595506040909401359392505050565b815163ffffffff9081168252602080840151909116908201526040808301516001600160601b03169082015260608101610cc5565b80356001600160601b0381168114612c2257600080fd5b600080604083850312156131b957600080fd5b6131c283612c11565b9150612ec96020840161318f565b6000806000604084860312156131e557600080fd5b8335925060208401356001600160401b038111156130d057600080fd5b600082601f83011261321357600080fd5b8135602061322361303283612fb8565b82815260069290921b8401810191818101908684111561324257600080fd5b8286015b84811015613291576040818903121561325f5760008081fd5b613267612f60565b813561327281612c8e565b815261327f82860161318f565b81860152835291830191604001613246565b509695505050505050565b600080604083850312156132af57600080fd5b6132b883612c11565b915060208301356001600160401b038111156132d357600080fd5b6132df85828601613202565b9150509250929050565b6000806000606084860312156132fe57600080fd5b61330784612c11565b925061331560208501613083565b9150604084013590509250925092565b60008060006060848603121561333a57600080fd5b8335925061334a60208501612c11565b915061335860408501613083565b90509250925092565b6000806000806080858703121561337757600080fd5b61338085612c11565b935061338e60208601613083565b93969395505050506040820135916060013590565b6000806000606084860312156133b857600080fd5b6133c184612c11565b92506133cf6020850161318f565b915060408401356001600160401b038111156133ea57600080fd5b6133f686828701613202565b9150509250925092565b60208082526031908201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726040820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b606082015260800190565b60006020828403121561346357600080fd5b815161152181612c8e565b60208082526056908201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60408201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746060820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561351257600080fd5b6115218261318f565b634e487b7160e01b600052601160045260246000fd5b60006000198214156135455761354561351b565b5060010190565b6020808252604c908201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960408201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260608201526b3ca1b7b7b93234b730ba37b960a11b608082015260a00190565b6000828210156135d0576135d061351b565b500390565b634e487b7160e01b600052603160045260246000fd5b60006040820160018060a01b03808616845260206040818601528286548085526060870191508760005282600020945060005b8181101561363c57855485168352600195860195928401920161361e565b509098975050505050505050565b6000602080838503121561365d57600080fd5b82516001600160401b0381111561367357600080fd5b8301601f8101851361368457600080fd5b805161369261303282612fb8565b81815260059190911b820183019083810190878311156136b157600080fd5b928401925b828410156136cf578351825292840192908401906136b6565b979650505050505050565b60008160001904831182151516156136f4576136f461351b565b500290565b60008261371657634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160601b0380831681851680830382111561373d5761373d61351b565b01949350505050565b600082198211156137595761375961351b565b500190565b60008161376d5761376d61351b565b506000190190565b60008083128015600160ff1b8501841216156137935761379361351b565b6001600160ff1b03840183138116156137ae576137ae61351b565b50500390565b6000600160ff1b8214156137ca576137ca61351b565b5060000390565b60006001600160601b03838116908316818110156137f1576137f161351b565b03939250505056fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a264697066735822122014c70732d3158fe130cdb5a2d819a732d72da424c0223c808ae97682e4399e0c64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xE5W`\x005`\xE0\x1C\x80c\x9F<\xCFe\x11a\x01\x0FW\x80c\xC8)LV\x11a\0\xA2W\x80c\xF2\xBE\x94\xAE\x11a\0qW\x80c\xF2\xBE\x94\xAE\x14a\x05KW\x80c\xF8Q\xE1\x98\x14a\x05^W\x80c\xFA(\xC6'\x14a\x05qW\x80c\xFFiJw\x14a\x05\x84W`\0\x80\xFD[\x80c\xC8)LV\x14a\x04\xD6W\x80c\xD5\xEC\xCC\x05\x14a\x04\xE9W\x80c\xDD\x98F\xB9\x14a\x04\xFCW\x80c\xDF\\\xF7#\x14a\x05$W`\0\x80\xFD[\x80c\xBC\x9A@\xC3\x11a\0\xDEW\x80c\xBC\x9A@\xC3\x14a\x04tW\x80c\xBD)\xB8\xCD\x14a\x04\x87W\x80c\xC4gx\xA5\x14a\x04\x9AW\x80c\xC6\x01R}\x14a\x04\xC3W`\0\x80\xFD[\x80c\x9F<\xCFe\x14a\x03\xEEW\x80c\xACk\xFB\x03\x14a\x04\x01W\x80c\xAD\xC8\x04\xDA\x14a\x04!W\x80c\xB6\x90Kx\x14a\x04aW`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\x87W\x80cf\xAC\xFE\xFE\x11a\x01VW\x80cf\xAC\xFE\xFE\x14a\x03JW\x80cm\x14\xA9\x87\x14a\x03uW\x80c|\x17#G\x14a\x03\xB4W\x80c\x81\xC0u\x02\x14a\x03\xCEW`\0\x80\xFD[\x80cK\xD2n\t\x14a\x02\xE5W\x80cT\x01\xED'\x14a\x03\x15W\x80c^Zgu\x14a\x03(W\x80c_\x1F-w\x14a\x037W`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xC3W\x80c \xB6b\x98\x14a\x02lW\x80c%PGw\x14a\x02\x81W\x80c,\xD9Y@\x14a\x02\xA2W\x80c<\xA5\xA5\xF5\x14a\x02\xC2W`\0\x80\xFD[\x80c\x04\x91\xB4\x1C\x14a\x01\xEAW\x80c\x08s$a\x14a\x02 W\x80c\x1F\x9Bt\xE0\x14a\x02AW[`\0\x80\xFD[a\x02\ra\x01\xF86`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a,BV[a\x05\x97V[`@Qa\x02\x17\x92\x91\x90a,lV[a\x02Ta\x02O6`\x04a,\xA6V[a\x05\xE0V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\x7Fa\x02z6`\x04a-!V[a\x060V[\0[a\x02\x94a\x02\x8F6`\x04a-\xE2V[a\taV[`@Qa\x02\x17\x92\x91\x90a.\x81V[a\x02\xB5a\x02\xB06`\x04a.\xA6V[a\x0C,V[`@Qa\x02\x17\x91\x90a.\xD2V[a\x02\ra\x02\xD06`\x04a,'V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02\ra\x02\xF36`\x04a.\xA6V[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02Ta\x03#6`\x04a.\xA6V[a\x0C\xCBV[a\x02\rg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\x7Fa\x03E6`\x04a/\xDBV[a\x0C\xE4V[a\x03]a\x03X6`\x04a-\xE2V[a\x11\x04V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xBC` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\xE1a\x03\xDC6`\x04a0\x97V[a\x12^V[`@Qa\x02\x17\x91\x90a0\xE9V[a\x03\x9Ca\x03\xFC6`\x04a,BV[a\x15(V[a\x04\x14a\x04\x0F6`\x04a1'V[a\x15`V[`@Qa\x02\x17\x91\x90a1ZV[a\x044a\x04/6`\x04a,BV[a\x15\xF8V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02\x17V[a\x04\x14a\x04o6`\x04a,BV[a\x16rV[a\x02\x7Fa\x04\x826`\x04a1\xA6V[a\x17\x01V[a\x02\x7Fa\x04\x956`\x04a1\xD0V[a\x17\xFAV[a\x02Ta\x04\xA86`\x04a,'V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\x7Fa\x04\xD16`\x04a2\x9CV[a\x19'V[a\x02Ta\x04\xE46`\x04a2\xE9V[a\x1A\x1BV[a\x02Ta\x04\xF76`\x04a,'V[a\x1A\x99V[a\x05\x0Fa\x05\n6`\x04a3%V[a\x1A\xECV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03\x9C\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02Ta\x05Y6`\x04a3aV[a\x1B\x01V[a\x04\x14a\x05l6`\x04a.\xA6V[a\x1B\x96V[a\x02Ta\x05\x7F6`\x04a3%V[a\x1C{V[a\x02\x7Fa\x05\x926`\x04a3\xA3V[a\x1C\xDCV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x05\xB3W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06&\x85\x85a\x1EGV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB2\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x84a\x06\xFE\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\x1AW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x83\x80a\x07\x90W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x11V[\x82\x81\x14a\x08\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\tVW\x85\x85\x82\x81\x81\x10a\x083Wa\x083a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\x08H\x91\x90a5\0V[\x82\x89\x89\x84\x81\x81\x10a\x08[Wa\x08[a4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08rWa\x08ra4\xEAV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x08\xDBWa\x08\xDBa4\xEAV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF2Wa\x08\xF2a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\x19Wa\t\x19a4\xEAV[\x90P` \x02\x01` \x81\x01\x90a\t.\x91\x90a5\0V[`@Qa\t<\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a\tN\x81a51V[\x91PPa\x08\x19V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\xACW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xC6Wa\t\xC6a/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xEFW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x0CWa\n\x0Ca/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\x1EW`\0\x87\x87\x83\x81\x81\x10a\nWWa\nWa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\n\xDFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\n\xEC\x83\x8Da\x1EGV[\x91P\x91P\x80a\x0B\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0a\x0B\x96\x8C\x85\x85a EV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B\xABWa\x0B\xABa4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0B\xD5\x84\x82a\"\xC5V[\x86\x86\x81Q\x81\x10a\x0B\xE7Wa\x0B\xE7a4\xEAV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x16\x90a51V[\x91PPa\n;V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C\xBEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0CeV[PPPP\x90P[\x92\x91PPV[`\0\x80a\x0C\xD8\x84\x84a\x1B\x96V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\rf\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\r\x96W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\r\xB2\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\r\xCEW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[\x81Q\x80a\x0ECW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x10\xFBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0E\xA2Wa\x0E\xA2a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0E\xBAWa\x0E\xBAa4\xEAV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F0Wa\x0F0a4\xEAV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0Fp\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x0F\x80Wa\x0F\x80a4\xEAV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x0F\x9DWa\x0F\x9Da4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB5Wa\x0F\xB5a4\xEAV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x08Wa\x10\x08a5\xD5V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x100\x90`\x01\x90a5\xBEV[\x81T\x81\x10a\x10@Wa\x10@a4\xEAV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10qWa\x10qa4\xEAV[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x89Wa\x10\x89a4\xEAV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x10\xC7Wa\x10\xC7a5\xD5V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x10\xF3\x81a51V[\x91PPa\x0EbV[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0\x80[\x83\x81\x10\x15a\x06&W`\0\x85\x85\x83\x81\x81\x10a\x11nWa\x11na4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x11\xFEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0\x80a\x12\x0B\x83\x8Ba\x1EGV[\x91P\x91P\x80a\x12-W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12:\x8A\x85\x85a EV[\x90Pa\x12F\x84\x82a\"\xC5V[PPPPP\x80\x80a\x12V\x90a51V[\x91PPa\x11RV[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x12zWa\x12za/JV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\x1DW`\0\x85\x85\x83\x81\x81\x10a\x12\xC5Wa\x12\xC5a4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x13dW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x13\x8DWa\x13\x8Da4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x149W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\x07W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x14}\x84\x86a5\xBEV[a\x14\x87\x91\x90a5\xBEV[\x81T\x81\x10a\x14\x97Wa\x14\x97a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x14\xF5W`\x01a\x14\xBA\x82\x84a5\xBEV[a\x14\xC4\x91\x90a5\xBEV[\x85\x85\x81Q\x81\x10a\x14\xD6Wa\x14\xD6a4\xEAV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\x07V[\x80a\x14\xFF\x81a51V[\x91PPa\x14NV[PPP\x80\x80a\x15\x15\x90a51V[\x91PPa\x12\xA9V[P\x90P[\x93\x92PPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15DW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xA5Wa\x15\xA5a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x160Wa\x160a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16\xAFWa\x16\xAFa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17_W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17\x83\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x17\xB3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x17\xCF\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x17\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$?V[PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18BW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\0[\x81\x81\x10\x15a\x19!W`\0\x83\x83\x83\x81\x81\x10a\x18aWa\x18aa4\xEAV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x18\xF1W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[`\0a\x18\xFF\x86\x83`\0a EV[\x90Pa\x19\x0B\x82\x82a\"\xC5V[PPP\x80\x80a\x19\x19\x90a51V[\x91PPa\x18EV[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x19\x85W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\xA9\x91\x90a4QV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x19\xD9W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4nV[\x81a\x19\xF5\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\x11W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a4\0V[a\x17\xF5\x83\x83a$\xA8V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1ABWa\x1ABa4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0C\xD8\x81\x85a(\xEBV[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1A\xBA\x91a5\xBEV[\x81T\x81\x10a\x1A\xCAWa\x1A\xCAa4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x1A\xF9\x84\x84\x84a*eV[\x94\x93PPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B2Wa\x1B2a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1B\x89\x81\x86a(\xEBV[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1B\xEFW\x91Pa\x0C\xC5\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\x16`\x01\x84a5\xBEV[\x81T\x81\x10a\x1C&Wa\x1C&a4\xEAV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C\xC5\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1C\xA2\x85\x85\x85a*eV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1C\xB8Wa\x1C\xB8a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D$W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x11\x90a5LV[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1D\xA2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x11V[a\x1D\xAC\x83\x82a$\xA8V[a\x1D\xB6\x83\x83a$?V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\0\x80`\0\x80a\x1Ef\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1E\xDB\x92\x8C\x92\x01a5\xEBV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\xF8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1F \x91\x90\x81\x01\x90a6JV[\x90P`\0[\x83\x81\x10\x15a \x11W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1FQWa\x1FQa4\xEAV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1F\x9FWa\x1F\x9Fa4\xEAV[` \x02` \x01\x01Q\x11\x15a\x1F\xFFWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1F\xD6Wa\x1F\xD6a4\xEAV[` \x02` \x01\x01Qa\x1F\xE8\x91\x90a6\xDAV[a\x1F\xF2\x91\x90a6\xF9V[a\x1F\xFC\x90\x86a7\x1BV[\x94P[\x80a \t\x81a51V[\x91PPa\x1F%V[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a!\tW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"kV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a!0`\x01\x84a5\xBEV[\x81T\x81\x10a!@Wa!@a4\xEAV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a!yW`\0\x93PPPPa\x15!V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a!\xB3W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"iV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\xBB\x82\x85a+\xCBV[\x96\x95PPPPPPV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\xE9\x90\x84a5\xBEV[\x81T\x81\x10a\"\xF9Wa\"\xF9a4\xEAV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a#(WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C\xC5\x90PV[\x80T`\0\x90a#G\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a+\xE3V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\x84W\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua$6V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a%0\x83\x83a7FV[\x11\x15a%\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\0[\x82\x81\x10\x15a(\xE4W`\0[a%\xB8\x82\x84a7FV[\x81\x10\x15a&\x99W\x84\x82\x81Q\x81\x10a%\xD1Wa%\xD1a4\xEAV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\x10Wa&\x10a4\xEAV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a&\x87W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x11V[\x80a&\x91\x81a51V[\x91PPa%\xAEV[P`\0\x84\x82\x81Q\x81\x10a&\xAEWa&\xAEa4\xEAV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a'3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a7\xFA\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'YWa'Ya4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a'\xBEWa'\xBEa4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a(5Wa(5a4\xEAV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a(\x92Wa(\x92a4\xEAV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a(\xB0Wa(\xB0a4\xEAV[` \x02` \x01\x01Q` \x01Q`@Qa(\xCA\x92\x91\x90a,lV[`@Q\x80\x91\x03\x90\xA2\x80a(\xDC\x81a51V[\x91PPa%\xA3V[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)\x90W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x11V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\xB6WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*aW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x11V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\x06W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a*\xB9`\x01\x84a5\xBEV[\x81T\x81\x10a*\xC9Wa*\xC9a4\xEAV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a*\xF4Wa*\xEB`\x01\x82a5\xBEV[\x92PPPa\x15!V[\x80a*\xFE\x81a7^V[\x91PPa*\x84V[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x11V[`\0a\x15!`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a7uV[`\0\x80\x82\x12\x15a,\x07Wa+\xF6\x82a7\xB4V[a,\0\x90\x84a7\xD1V[\x90Pa\x0C\xC5V[a,\0\x82\x84a7\x1BV[\x805`\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a,9W`\0\x80\xFD[a\x15!\x82a,\x11V[`\0\x80`@\x83\x85\x03\x12\x15a,UW`\0\x80\xFD[a,^\x83a,\x11V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a,\xA3W`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xB9W`\0\x80\xFD[a,\xC2\x83a,\x11V[\x91P` \x83\x015a,\xD2\x81a,\x8EV[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a,\xEFW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\x06W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-9W`\0\x80\xFD[a-B\x86a,\x11V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a-^W`\0\x80\xFD[a-j\x89\x83\x8A\x01a,\xDDV[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a-\x83W`\0\x80\xFD[Pa-\x90\x88\x82\x89\x01a,\xDDV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a-\xB3W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xCAW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a >W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a-\xF8W`\0\x80\xFD[\x845a.\x03\x81a,\x8EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.%W`\0\x80\xFD[a.1\x87\x82\x88\x01a-\xA1V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a.vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.QV[P\x94\x95\x94PPPPPV[`@\x81R`\0a.\x94`@\x83\x01\x85a.=V[\x82\x81\x03` \x84\x01Ra$6\x81\x85a.=V[`\0\x80`@\x83\x85\x03\x12\x15a.\xB9W`\0\x80\xFD[\x825\x91Pa.\xC9` \x84\x01a,\x11V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>Wa/+\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a.\xEEV[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\x82Wa/\x82a/JV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a/\xB0Wa/\xB0a/JV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a/\xD1Wa/\xD1a/JV[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a/\xEEW`\0\x80\xFD[a/\xF7\x83a,\x11V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\x13W`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0$W`\0\x80\xFD[\x805a07a02\x82a/\xB8V[a/\x88V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0VW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a0tW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a0[V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a0\xACW`\0\x80\xFD[a0\xB5\x84a0\x83V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[a0\xDC\x86\x82\x87\x01a-\xA1V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/>W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a1\x05V[`\0\x80`\0``\x84\x86\x03\x12\x15a1<W`\0\x80\xFD[a1E\x84a,\x11V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x0C\xC5V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\"W`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a1\xB9W`\0\x80\xFD[a1\xC2\x83a,\x11V[\x91Pa.\xC9` \x84\x01a1\x8FV[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xE5W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xD0W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a2\x13W`\0\x80\xFD[\x815` a2#a02\x83a/\xB8V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a2BW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a2\x91W`@\x81\x89\x03\x12\x15a2_W`\0\x80\x81\xFD[a2ga/`V[\x815a2r\x81a,\x8EV[\x81Ra2\x7F\x82\x86\x01a1\x8FV[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a2FV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a2\xAFW`\0\x80\xFD[a2\xB8\x83a,\x11V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\xD3W`\0\x80\xFD[a2\xDF\x85\x82\x86\x01a2\x02V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xFEW`\0\x80\xFD[a3\x07\x84a,\x11V[\x92Pa3\x15` \x85\x01a0\x83V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a3:W`\0\x80\xFD[\x835\x92Pa3J` \x85\x01a,\x11V[\x91Pa3X`@\x85\x01a0\x83V[\x90P\x92P\x92P\x92V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a3wW`\0\x80\xFD[a3\x80\x85a,\x11V[\x93Pa3\x8E` \x86\x01a0\x83V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xB8W`\0\x80\xFD[a3\xC1\x84a,\x11V[\x92Pa3\xCF` \x85\x01a1\x8FV[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xEAW`\0\x80\xFD[a3\xF6\x86\x82\x87\x01a2\x02V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a4cW`\0\x80\xFD[\x81Qa\x15!\x81a,\x8EV[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a5\x12W`\0\x80\xFD[a\x15!\x82a1\x8FV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a5EWa5Ea5\x1BV[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a5\xD0Wa5\xD0a5\x1BV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a6<W\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a6\x1EV[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a6]W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a6sW`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a6\x84W`\0\x80\xFD[\x80Qa6\x92a02\x82a/\xB8V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a6\xB1W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a6\xCFW\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a6\xB6V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a6\xF4Wa6\xF4a5\x1BV[P\x02\x90V[`\0\x82a7\x16WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a7=Wa7=a5\x1BV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15a7YWa7Ya5\x1BV[P\x01\x90V[`\0\x81a7mWa7ma5\x1BV[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a7\x93Wa7\x93a5\x1BV[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a7\xAEWa7\xAEa5\x1BV[PP\x03\x90V[`\0`\x01`\xFF\x1B\x82\x14\x15a7\xCAWa7\xCAa5\x1BV[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a7\xF1Wa7\xF1a5\x1BV[\x03\x93\x92PPPV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x14\xC7\x072\xD3\x15\x8F\xE10\xCD\xB5\xA2\xD8\x19\xA72\xD7-\xA4$\xC0\"<\x80\x8A\xE9v\x82\xE49\x9E\x0CdsolcC\0\x08\x0C\x003",
    );
    /**Event with signature `MinimumStakeForQuorumUpdated(uint8,uint96)` and selector `0x26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf`.
```solidity
event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct MinimumStakeForQuorumUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
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
        impl alloy_sol_types::SolEvent for MinimumStakeForQuorumUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "MinimumStakeForQuorumUpdated(uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                38u8,
                238u8,
                207u8,
                242u8,
                183u8,
                11u8,
                10u8,
                113u8,
                16u8,
                79u8,
                244u8,
                217u8,
                64u8,
                186u8,
                113u8,
                98u8,
                210u8,
                58u8,
                149u8,
                194u8,
                72u8,
                119u8,
                31u8,
                196u8,
                135u8,
                167u8,
                190u8,
                23u8,
                165u8,
                150u8,
                179u8,
                207u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    minimumStake: data.0,
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for MinimumStakeForQuorumUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&MinimumStakeForQuorumUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &MinimumStakeForQuorumUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d`.
```solidity
event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct OperatorStakeUpdate {
        #[allow(missing_docs)]
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorStakeUpdate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorStakeUpdate(bytes32,uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                47u8,
                82u8,
                125u8,
                82u8,
                126u8,
                149u8,
                216u8,
                254u8,
                64u8,
                174u8,
                197u8,
                83u8,
                119u8,
                116u8,
                59u8,
                183u8,
                121u8,
                8u8,
                125u8,
                163u8,
                246u8,
                208u8,
                208u8,
                143u8,
                18u8,
                227u8,
                100u8,
                68u8,
                218u8,
                98u8,
                50u8,
                125u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    operatorId: topics.1,
                    quorumNumber: data.0,
                    stake: data.1,
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
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.stake),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.operatorId.clone())
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
                out[1usize] = <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.operatorId);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorStakeUpdate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorStakeUpdate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorStakeUpdate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `QuorumCreated(uint8)` and selector `0x831a9c86c45bb303caf3f064be2bc2b9fd4ecf19e47c4ac02a61e75dabfe55b4`.
```solidity
event QuorumCreated(uint8 indexed quorumNumber);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct QuorumCreated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        impl alloy_sol_types::SolEvent for QuorumCreated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumCreated(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                131u8,
                26u8,
                156u8,
                134u8,
                196u8,
                91u8,
                179u8,
                3u8,
                202u8,
                243u8,
                240u8,
                100u8,
                190u8,
                43u8,
                194u8,
                185u8,
                253u8,
                78u8,
                207u8,
                25u8,
                228u8,
                124u8,
                74u8,
                192u8,
                42u8,
                97u8,
                231u8,
                93u8,
                171u8,
                254u8,
                85u8,
                180u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { quorumNumber: topics.1 }
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
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for QuorumCreated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&QuorumCreated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &QuorumCreated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyAddedToQuorum(uint8,address)` and selector `0x10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f5404`.
```solidity
event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyAddedToQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        impl alloy_sol_types::SolEvent for StrategyAddedToQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyAddedToQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                16u8,
                86u8,
                94u8,
                86u8,
                202u8,
                203u8,
                243u8,
                46u8,
                202u8,
                38u8,
                121u8,
                69u8,
                240u8,
                84u8,
                254u8,
                192u8,
                46u8,
                89u8,
                117u8,
                0u8,
                50u8,
                209u8,
                19u8,
                211u8,
                48u8,
                33u8,
                130u8,
                173u8,
                150u8,
                127u8,
                84u8,
                4u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
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
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyAddedToQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyAddedToQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StrategyAddedToQuorum) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyMultiplierUpdated(uint8,address,uint256)` and selector `0x11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75`.
```solidity
event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyMultiplierUpdated {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
        #[allow(missing_docs)]
        pub strategy: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub multiplier: alloy::sol_types::private::primitives::aliases::U256,
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
        impl alloy_sol_types::SolEvent for StrategyMultiplierUpdated {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyMultiplierUpdated(uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                17u8,
                165u8,
                100u8,
                19u8,
                34u8,
                218u8,
                29u8,
                255u8,
                86u8,
                164u8,
                182u8,
                110u8,
                170u8,
                195u8,
                31u8,
                250u8,
                70u8,
                82u8,
                149u8,
                236u8,
                233u8,
                7u8,
                205u8,
                22u8,
                52u8,
                55u8,
                121u8,
                59u8,
                77u8,
                0u8,
                154u8,
                117u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
                    multiplier: data.1,
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.multiplier),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyMultiplierUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyMultiplierUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyMultiplierUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `StrategyRemovedFromQuorum(uint8,address)` and selector `0x31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f7`.
```solidity
event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StrategyRemovedFromQuorum {
        #[allow(missing_docs)]
        pub quorumNumber: u8,
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
        impl alloy_sol_types::SolEvent for StrategyRemovedFromQuorum {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyRemovedFromQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                49u8,
                250u8,
                46u8,
                44u8,
                210u8,
                128u8,
                201u8,
                55u8,
                94u8,
                19u8,
                255u8,
                207u8,
                61u8,
                129u8,
                226u8,
                55u8,
                129u8,
                0u8,
                24u8,
                110u8,
                64u8,
                88u8,
                248u8,
                211u8,
                221u8,
                182u8,
                144u8,
                184u8,
                45u8,
                205u8,
                49u8,
                247u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    quorumNumber: topics.1,
                    strategy: data.0,
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
                        &self.strategy,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.quorumNumber.clone())
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
                    8,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.quorumNumber);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for StrategyRemovedFromQuorum {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StrategyRemovedFromQuorum> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &StrategyRemovedFromQuorum,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Constructor`.
```solidity
constructor(address _registryCoordinator, address _delegationManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _delegationManager: alloy::sol_types::private::Address,
    }
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
            impl ::core::convert::From<constructorCall> for UnderlyingRustTuple<'_> {
                fn from(value: constructorCall) -> Self {
                    (value._registryCoordinator, value._delegationManager)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _delegationManager: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                        &self._registryCoordinator,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._delegationManager,
                    ),
                )
            }
        }
    };
    /**Function with signature `MAX_WEIGHING_FUNCTION_LENGTH()` and selector `0x7c172347`.
```solidity
function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHCall {}
    ///Container type for the return parameters of the [`MAX_WEIGHING_FUNCTION_LENGTH()`](MAX_WEIGHING_FUNCTION_LENGTHCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct MAX_WEIGHING_FUNCTION_LENGTHReturn {
        pub _0: u8,
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
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WEIGHING_FUNCTION_LENGTHCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
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
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for MAX_WEIGHING_FUNCTION_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WEIGHING_FUNCTION_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_WEIGHING_FUNCTION_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MAX_WEIGHING_FUNCTION_LENGTH()";
            const SELECTOR: [u8; 4] = [124u8, 23u8, 35u8, 71u8];
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
    /**Function with signature `WEIGHTING_DIVISOR()` and selector `0x5e5a6775`.
```solidity
function WEIGHTING_DIVISOR() external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORCall {}
    ///Container type for the return parameters of the [`WEIGHTING_DIVISOR()`](WEIGHTING_DIVISORCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct WEIGHTING_DIVISORReturn {
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
            impl ::core::convert::From<WEIGHTING_DIVISORCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for WEIGHTING_DIVISORCall {
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
            impl ::core::convert::From<WEIGHTING_DIVISORReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for WEIGHTING_DIVISORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WEIGHTING_DIVISORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = WEIGHTING_DIVISORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "WEIGHTING_DIVISOR()";
            const SELECTOR: [u8; 4] = [94u8, 90u8, 103u8, 117u8];
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
    /**Function with signature `addStrategies(uint8,(address,uint96)[])` and selector `0xc601527d`.
```solidity
function addStrategies(uint8 quorumNumber, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesCall {
        pub quorumNumber: u8,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`addStrategies(uint8,(address,uint96)[])`](addStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct addStrategiesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<addStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesCall) -> Self {
                    (value.quorumNumber, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _strategyParams: tuple.1,
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
            impl ::core::convert::From<addStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: addStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for addStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for addStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "addStrategies(uint8,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [198u8, 1u8, 82u8, 125u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistry::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
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
    /**Function with signature `deregisterOperator(bytes32,bytes)` and selector `0xbd29b8cd`.
```solidity
function deregisterOperator(bytes32 operatorId, bytes memory quorumNumbers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`deregisterOperator(bytes32,bytes)`](deregisterOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct deregisterOperatorReturn {}
    #[allow(
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
            impl ::core::convert::From<deregisterOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumbers: tuple.1,
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
            impl ::core::convert::From<deregisterOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for deregisterOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for deregisterOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "deregisterOperator(bytes32,bytes)";
            const SELECTOR: [u8; 4] = [189u8, 41u8, 184u8, 205u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getCurrentStake(bytes32,uint8)` and selector `0x5401ed27`.
```solidity
function getCurrentStake(bytes32 operatorId, uint8 quorumNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getCurrentStake(bytes32,uint8)`](getCurrentStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getCurrentStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getCurrentStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentStake(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [84u8, 1u8, 237u8, 39u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getCurrentTotalStake(uint8)` and selector `0xd5eccc05`.
```solidity
function getCurrentTotalStake(uint8 quorumNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getCurrentTotalStake(uint8)`](getCurrentTotalStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getCurrentTotalStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getCurrentTotalStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getCurrentTotalStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getCurrentTotalStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentTotalStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getCurrentTotalStake(uint8)";
            const SELECTOR: [u8; 4] = [213u8, 236u8, 204u8, 5u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getLatestStakeUpdate(bytes32,uint8)` and selector `0xf851e198`.
```solidity
function getLatestStakeUpdate(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistry.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getLatestStakeUpdate(bytes32,uint8)`](getLatestStakeUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getLatestStakeUpdateReturn {
        pub _0: <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getLatestStakeUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestStakeUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistry::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getLatestStakeUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getLatestStakeUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getLatestStakeUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestStakeUpdateReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getLatestStakeUpdate(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [248u8, 81u8, 225u8, 152u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getStakeAtBlockNumber(bytes32,uint8,uint32)` and selector `0xfa28c627`.
```solidity
function getStakeAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getStakeAtBlockNumber(bytes32,uint8,uint32)`](getStakeAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getStakeAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getStakeAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [250u8, 40u8, 198u8, 39u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)` and selector `0xf2be94ae`.
```solidity
function getStakeAtBlockNumberAndIndex(uint8 quorumNumber, uint32 blockNumber, bytes32 operatorId, uint256 index) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)`](getStakeAtBlockNumberAndIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeAtBlockNumberAndIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
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
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexCall) -> Self {
                    (
                        value.quorumNumber,
                        value.blockNumber,
                        value.operatorId,
                        value.index,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberAndIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        operatorId: tuple.2,
                        index: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeAtBlockNumberAndIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeAtBlockNumberAndIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberAndIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [242u8, 190u8, 148u8, 174u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getStakeHistory(bytes32,uint8)` and selector `0x2cd95940`.
```solidity
function getStakeHistory(bytes32 operatorId, uint8 quorumNumber) external view returns (IStakeRegistry.StakeUpdate[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getStakeHistory(bytes32,uint8)`](getStakeHistoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryReturn {
        pub _0: alloy::sol_types::private::Vec<
            <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getStakeHistoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<IStakeRegistry::StakeUpdate>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakeHistoryReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<IStakeRegistry::StakeUpdate>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistory(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [44u8, 217u8, 89u8, 64u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getStakeHistoryLength(bytes32,uint8)` and selector `0x4bd26e09`.
```solidity
function getStakeHistoryLength(bytes32 operatorId, uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getStakeHistoryLength(bytes32,uint8)`](getStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeHistoryLengthReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getStakeHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                    }
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
            impl ::core::convert::From<getStakeHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeHistoryLengthCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeHistoryLength(bytes32,uint8)";
            const SELECTOR: [u8; 4] = [75u8, 210u8, 110u8, 9u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getStakeUpdateAtIndex(uint8,bytes32,uint256)` and selector `0xac6bfb03`.
```solidity
function getStakeUpdateAtIndex(uint8 quorumNumber, bytes32 operatorId, uint256 index) external view returns (IStakeRegistry.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getStakeUpdateAtIndex(uint8,bytes32,uint256)`](getStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateAtIndexReturn {
        pub _0: <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<getStakeUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operatorId: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistry::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakeUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeUpdateAtIndex(uint8,bytes32,uint256)";
            const SELECTOR: [u8; 4] = [172u8, 107u8, 251u8, 3u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
    /**Function with signature `getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)` and selector `0xdd9846b9`.
```solidity
function getStakeUpdateIndexAtBlockNumber(bytes32 operatorId, uint8 quorumNumber, uint32 blockNumber) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
        pub blockNumber: u32,
    }
    ///Container type for the return parameters of the [`getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)`](getStakeUpdateIndexAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getStakeUpdateIndexAtBlockNumberReturn {
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
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                u8,
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
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateIndexAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        blockNumber: tuple.2,
                    }
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
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getStakeUpdateIndexAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getStakeUpdateIndexAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateIndexAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)";
            const SELECTOR: [u8; 4] = [221u8, 152u8, 70u8, 185u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
                    <alloy::sol_types::sol_data::Uint<
                        8,
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)` and selector `0xc8294c56`.
```solidity
function getTotalStakeAtBlockNumberFromIndex(uint8 quorumNumber, uint32 blockNumber, uint256 index) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexCall {
        pub quorumNumber: u8,
        pub blockNumber: u32,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)`](getTotalStakeAtBlockNumberFromIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeAtBlockNumberFromIndexReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                u32,
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
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeAtBlockNumberFromIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        blockNumber: tuple.1,
                        index: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeAtBlockNumberFromIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeAtBlockNumberFromIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeAtBlockNumberFromIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)";
            const SELECTOR: [u8; 4] = [200u8, 41u8, 76u8, 86u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
    /**Function with signature `getTotalStakeHistoryLength(uint8)` and selector `0x0491b41c`.
```solidity
function getTotalStakeHistoryLength(uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`getTotalStakeHistoryLength(uint8)`](getTotalStakeHistoryLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeHistoryLengthReturn {
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
            impl ::core::convert::From<getTotalStakeHistoryLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<getTotalStakeHistoryLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeHistoryLength(uint8)";
            const SELECTOR: [u8; 4] = [4u8, 145u8, 180u8, 28u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `getTotalStakeIndicesAtBlockNumber(uint32,bytes)` and selector `0x81c07502`.
```solidity
function getTotalStakeIndicesAtBlockNumber(uint32 blockNumber, bytes memory quorumNumbers) external view returns (uint32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberCall {
        pub blockNumber: u32,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`getTotalStakeIndicesAtBlockNumber(uint32,bytes)`](getTotalStakeIndicesAtBlockNumberCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeIndicesAtBlockNumberReturn {
        pub _0: alloy::sol_types::private::Vec<u32>,
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u32, alloy::sol_types::private::Bytes);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeIndicesAtBlockNumberCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        blockNumber: tuple.0,
                        quorumNumbers: tuple.1,
                    }
                }
            }
        }
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
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeIndicesAtBlockNumberReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeIndicesAtBlockNumberCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeIndicesAtBlockNumber(uint32,bytes)";
            const SELECTOR: [u8; 4] = [129u8, 192u8, 117u8, 2u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.blockNumber),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `getTotalStakeUpdateAtIndex(uint8,uint256)` and selector `0xb6904b78`.
```solidity
function getTotalStakeUpdateAtIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistry.StakeUpdate memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexCall {
        pub quorumNumber: u8,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`getTotalStakeUpdateAtIndex(uint8,uint256)`](getTotalStakeUpdateAtIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct getTotalStakeUpdateAtIndexReturn {
        pub _0: <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeUpdateAtIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistry::StakeUpdate,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for getTotalStakeUpdateAtIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeUpdateAtIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "getTotalStakeUpdateAtIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [182u8, 144u8, 75u8, 120u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `initializeQuorum(uint8,uint96,(address,uint96)[])` and selector `0xff694a77`.
```solidity
function initializeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeQuorum(uint8,uint96,(address,uint96)[])`](initializeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeQuorumReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<initializeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        _strategyParams: tuple.2,
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
            impl ::core::convert::From<initializeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeQuorum(uint8,uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [255u8, 105u8, 74u8, 119u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
                    <alloy::sol_types::sol_data::Array<
                        IStakeRegistry::StrategyParams,
                    > as alloy_sol_types::SolType>::tokenize(&self._strategyParams),
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
    /**Function with signature `minimumStakeForQuorum(uint8)` and selector `0xc46778a5`.
```solidity
function minimumStakeForQuorum(uint8) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`minimumStakeForQuorum(uint8)`](minimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct minimumStakeForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<minimumStakeForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<minimumStakeForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for minimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumStakeForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumStakeForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "minimumStakeForQuorum(uint8)";
            const SELECTOR: [u8; 4] = [196u8, 103u8, 120u8, 165u8];
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
    /**Function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`.
```solidity
function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsCall {
        pub quorumNumber: u8,
        pub strategyIndices: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
        pub newMultipliers: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
    }
    ///Container type for the return parameters of the [`modifyStrategyParams(uint8,uint256[],uint96[])`](modifyStrategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<modifyStrategyParamsCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsCall) -> Self {
                    (value.quorumNumber, value.strategyIndices, value.newMultipliers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyStrategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        strategyIndices: tuple.1,
                        newMultipliers: tuple.2,
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
            impl ::core::convert::From<modifyStrategyParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyStrategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyStrategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyStrategyParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyStrategyParams(uint8,uint256[],uint96[])";
            const SELECTOR: [u8; 4] = [32u8, 182u8, 98u8, 152u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.strategyIndices),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<96>,
                    > as alloy_sol_types::SolType>::tokenize(&self.newMultipliers),
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
    /**Function with signature `registerOperator(address,bytes32,bytes)` and selector `0x25504777`.
```solidity
function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`registerOperator(address,bytes32,bytes)`](registerOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct registerOperatorReturn {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
        >,
        pub _1: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<registerOperatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorId: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
                >,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<registerOperatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registerOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "registerOperator(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [37u8, 80u8, 71u8, 119u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
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
            impl ::core::convert::From<registryCoordinatorCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorCall {
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
            impl ::core::convert::From<registryCoordinatorReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: registryCoordinatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for registryCoordinatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for registryCoordinatorCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = registryCoordinatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`.
```solidity
function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesCall {
        pub quorumNumber: u8,
        pub indicesToRemove: alloy::sol_types::private::Vec<
            alloy::sol_types::private::primitives::aliases::U256,
        >,
    }
    ///Container type for the return parameters of the [`removeStrategies(uint8,uint256[])`](removeStrategiesCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::primitives::aliases::U256,
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
            impl ::core::convert::From<removeStrategiesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesCall) -> Self {
                    (value.quorumNumber, value.indicesToRemove)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        indicesToRemove: tuple.1,
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
            impl ::core::convert::From<removeStrategiesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for removeStrategiesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for removeStrategiesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<256>>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "removeStrategies(uint8,uint256[])";
            const SELECTOR: [u8; 4] = [95u8, 31u8, 45u8, 119u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::Uint<256>,
                    > as alloy_sol_types::SolType>::tokenize(&self.indicesToRemove),
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
    /**Function with signature `setMinimumStakeForQuorum(uint8,uint96)` and selector `0xbc9a40c3`.
```solidity
function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`setMinimumStakeForQuorum(uint8,uint96)`](setMinimumStakeForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setMinimumStakeForQuorumReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<setMinimumStakeForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
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
            impl ::core::convert::From<setMinimumStakeForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setMinimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setMinimumStakeForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinimumStakeForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setMinimumStakeForQuorum(uint8,uint96)";
            const SELECTOR: [u8; 4] = [188u8, 154u8, 64u8, 195u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.minimumStake),
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
    /**Function with signature `strategiesPerQuorum(uint8,uint256)` and selector `0x9f3ccf65`.
```solidity
function strategiesPerQuorum(uint8, uint256) external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategiesPerQuorum(uint8,uint256)`](strategiesPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategiesPerQuorumReturn {
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<strategiesPerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
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
            impl ::core::convert::From<strategiesPerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategiesPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategiesPerQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategiesPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategiesPerQuorum(uint8,uint256)";
            const SELECTOR: [u8; 4] = [159u8, 60u8, 207u8, 101u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Function with signature `strategyParams(uint8,uint256)` and selector `0x08732461`.
```solidity
function strategyParams(uint8, uint256) external view returns (address strategy, uint96 multiplier);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsCall {
        pub _0: u8,
        pub _1: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategyParams(uint8,uint256)`](strategyParamsCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsReturn {
        pub strategy: alloy::sol_types::private::Address,
        pub multiplier: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<strategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0, _1: tuple.1 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<strategyParamsReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsReturn) -> Self {
                    (value.strategy, value.multiplier)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        strategy: tuple.0,
                        multiplier: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParams(uint8,uint256)";
            const SELECTOR: [u8; 4] = [8u8, 115u8, 36u8, 97u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
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
    /**Function with signature `strategyParamsByIndex(uint8,uint256)` and selector `0xadc804da`.
```solidity
function strategyParamsByIndex(uint8 quorumNumber, uint256 index) external view returns (IStakeRegistry.StrategyParams memory);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexCall {
        pub quorumNumber: u8,
        pub index: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`strategyParamsByIndex(uint8,uint256)`](strategyParamsByIndexCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsByIndexReturn {
        pub _0: <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
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
            impl ::core::convert::From<strategyParamsByIndexCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsByIndexCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        index: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (IStakeRegistry::StrategyParams,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<strategyParamsByIndexReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsByIndexReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsByIndexCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsByIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StrategyParams,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsByIndex(uint8,uint256)";
            const SELECTOR: [u8; 4] = [173u8, 200u8, 4u8, 218u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `strategyParamsLength(uint8)` and selector `0x3ca5a5f5`.
```solidity
function strategyParamsLength(uint8 quorumNumber) external view returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`strategyParamsLength(uint8)`](strategyParamsLengthCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct strategyParamsLengthReturn {
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
            impl ::core::convert::From<strategyParamsLengthCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { quorumNumber: tuple.0 }
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
            impl ::core::convert::From<strategyParamsLengthReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for strategyParamsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "strategyParamsLength(uint8)";
            const SELECTOR: [u8; 4] = [60u8, 165u8, 165u8, 245u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    /**Function with signature `updateOperatorStake(address,bytes32,bytes)` and selector `0x66acfefe`.
```solidity
function updateOperatorStake(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint192);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorStakeCall {
        pub operator: alloy::sol_types::private::Address,
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumbers: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`updateOperatorStake(address,bytes32,bytes)`](updateOperatorStakeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct updateOperatorStakeReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U192,
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
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
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
            impl ::core::convert::From<updateOperatorStakeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        operatorId: tuple.1,
                        quorumNumbers: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U192,
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
            impl ::core::convert::From<updateOperatorStakeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateOperatorStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateOperatorStakeCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateOperatorStake(address,bytes32,bytes)";
            const SELECTOR: [u8; 4] = [102u8, 172u8, 254u8, 254u8];
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
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.operatorId),
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
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `weightOfOperatorForQuorum(uint8,address)` and selector `0x1f9b74e0`.
```solidity
function weightOfOperatorForQuorum(uint8 quorumNumber, address operator) external view returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumCall {
        pub quorumNumber: u8,
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`weightOfOperatorForQuorum(uint8,address)`](weightOfOperatorForQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct weightOfOperatorForQuorumReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U96,
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
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, alloy::sol_types::private::Address);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<weightOfOperatorForQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumCall) -> Self {
                    (value.quorumNumber, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for weightOfOperatorForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<weightOfOperatorForQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for weightOfOperatorForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for weightOfOperatorForQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = weightOfOperatorForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "weightOfOperatorForQuorum(uint8,address)";
            const SELECTOR: [u8; 4] = [31u8, 155u8, 116u8, 224u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self.quorumNumber),
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
    ///Container for all the [`StakeRegistry`](self) function calls.
    pub enum StakeRegistryCalls {
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        addStrategies(addStrategiesCall),
        delegation(delegationCall),
        deregisterOperator(deregisterOperatorCall),
        getCurrentStake(getCurrentStakeCall),
        getCurrentTotalStake(getCurrentTotalStakeCall),
        getLatestStakeUpdate(getLatestStakeUpdateCall),
        getStakeAtBlockNumber(getStakeAtBlockNumberCall),
        getStakeAtBlockNumberAndIndex(getStakeAtBlockNumberAndIndexCall),
        getStakeHistory(getStakeHistoryCall),
        getStakeHistoryLength(getStakeHistoryLengthCall),
        getStakeUpdateAtIndex(getStakeUpdateAtIndexCall),
        getStakeUpdateIndexAtBlockNumber(getStakeUpdateIndexAtBlockNumberCall),
        getTotalStakeAtBlockNumberFromIndex(getTotalStakeAtBlockNumberFromIndexCall),
        getTotalStakeHistoryLength(getTotalStakeHistoryLengthCall),
        getTotalStakeIndicesAtBlockNumber(getTotalStakeIndicesAtBlockNumberCall),
        getTotalStakeUpdateAtIndex(getTotalStakeUpdateAtIndexCall),
        initializeQuorum(initializeQuorumCall),
        minimumStakeForQuorum(minimumStakeForQuorumCall),
        modifyStrategyParams(modifyStrategyParamsCall),
        registerOperator(registerOperatorCall),
        registryCoordinator(registryCoordinatorCall),
        removeStrategies(removeStrategiesCall),
        setMinimumStakeForQuorum(setMinimumStakeForQuorumCall),
        strategiesPerQuorum(strategiesPerQuorumCall),
        strategyParams(strategyParamsCall),
        strategyParamsByIndex(strategyParamsByIndexCall),
        strategyParamsLength(strategyParamsLengthCall),
        updateOperatorStake(updateOperatorStakeCall),
        weightOfOperatorForQuorum(weightOfOperatorForQuorumCall),
    }
    #[automatically_derived]
    impl StakeRegistryCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [4u8, 145u8, 180u8, 28u8],
            [8u8, 115u8, 36u8, 97u8],
            [31u8, 155u8, 116u8, 224u8],
            [32u8, 182u8, 98u8, 152u8],
            [37u8, 80u8, 71u8, 119u8],
            [44u8, 217u8, 89u8, 64u8],
            [60u8, 165u8, 165u8, 245u8],
            [75u8, 210u8, 110u8, 9u8],
            [84u8, 1u8, 237u8, 39u8],
            [94u8, 90u8, 103u8, 117u8],
            [95u8, 31u8, 45u8, 119u8],
            [102u8, 172u8, 254u8, 254u8],
            [109u8, 20u8, 169u8, 135u8],
            [124u8, 23u8, 35u8, 71u8],
            [129u8, 192u8, 117u8, 2u8],
            [159u8, 60u8, 207u8, 101u8],
            [172u8, 107u8, 251u8, 3u8],
            [173u8, 200u8, 4u8, 218u8],
            [182u8, 144u8, 75u8, 120u8],
            [188u8, 154u8, 64u8, 195u8],
            [189u8, 41u8, 184u8, 205u8],
            [196u8, 103u8, 120u8, 165u8],
            [198u8, 1u8, 82u8, 125u8],
            [200u8, 41u8, 76u8, 86u8],
            [213u8, 236u8, 204u8, 5u8],
            [221u8, 152u8, 70u8, 185u8],
            [223u8, 92u8, 247u8, 35u8],
            [242u8, 190u8, 148u8, 174u8],
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
            [255u8, 105u8, 74u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryCalls {
        const NAME: &'static str = "StakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 31usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(_) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WEIGHTING_DIVISOR(_) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategies(_) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::delegation(_) => {
                    <delegationCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::deregisterOperator(_) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentStake(_) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getCurrentTotalStake(_) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getLatestStakeUpdate(_) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumber(_) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeAtBlockNumberAndIndex(_) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistory(_) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeHistoryLength(_) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateAtIndex(_) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getStakeUpdateIndexAtBlockNumber(_) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeAtBlockNumberFromIndex(_) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeHistoryLength(_) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeIndicesAtBlockNumber(_) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::getTotalStakeUpdateAtIndex(_) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeQuorum(_) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumStakeForQuorum(_) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyStrategyParams(_) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registerOperator(_) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::registryCoordinator(_) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::removeStrategies(_) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinimumStakeForQuorum(_) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategiesPerQuorum(_) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParams(_) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsByIndex(_) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::strategyParamsLength(_) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::updateOperatorStake(_) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::weightOfOperatorForQuorum(_) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<StakeRegistryCalls>] = &[
                {
                    fn getTotalStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeHistoryLength)
                    }
                    getTotalStakeHistoryLength
                },
                {
                    fn strategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::strategyParams)
                    }
                    strategyParams
                },
                {
                    fn weightOfOperatorForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::weightOfOperatorForQuorum)
                    }
                    weightOfOperatorForQuorum
                },
                {
                    fn modifyStrategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::modifyStrategyParams)
                    }
                    modifyStrategyParams
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getStakeHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeHistory)
                    }
                    getStakeHistory
                },
                {
                    fn strategyParamsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::strategyParamsLength)
                    }
                    strategyParamsLength
                },
                {
                    fn getStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeHistoryLength)
                    }
                    getStakeHistoryLength
                },
                {
                    fn getCurrentStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getCurrentStake)
                    }
                    getCurrentStake
                },
                {
                    fn WEIGHTING_DIVISOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::WEIGHTING_DIVISOR)
                    }
                    WEIGHTING_DIVISOR
                },
                {
                    fn removeStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <removeStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::removeStrategies)
                    }
                    removeStrategies
                },
                {
                    fn updateOperatorStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::updateOperatorStake)
                    }
                    updateOperatorStake
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn MAX_WEIGHING_FUNCTION_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::MAX_WEIGHING_FUNCTION_LENGTH)
                    }
                    MAX_WEIGHING_FUNCTION_LENGTH
                },
                {
                    fn getTotalStakeIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeIndicesAtBlockNumber)
                    }
                    getTotalStakeIndicesAtBlockNumber
                },
                {
                    fn strategiesPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateAtIndex)
                    }
                    getStakeUpdateAtIndex
                },
                {
                    fn strategyParamsByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::strategyParamsByIndex)
                    }
                    strategyParamsByIndex
                },
                {
                    fn getTotalStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeUpdateAtIndex)
                    }
                    getTotalStakeUpdateAtIndex
                },
                {
                    fn setMinimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::setMinimumStakeForQuorum)
                    }
                    setMinimumStakeForQuorum
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn minimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::minimumStakeForQuorum)
                    }
                    minimumStakeForQuorum
                },
                {
                    fn addStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <addStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::addStrategies)
                    }
                    addStrategies
                },
                {
                    fn getTotalStakeAtBlockNumberFromIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getTotalStakeAtBlockNumberFromIndex)
                    }
                    getTotalStakeAtBlockNumberFromIndex
                },
                {
                    fn getCurrentTotalStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getCurrentTotalStake)
                    }
                    getCurrentTotalStake
                },
                {
                    fn getStakeUpdateIndexAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeUpdateIndexAtBlockNumber)
                    }
                    getStakeUpdateIndexAtBlockNumber
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::delegation)
                    }
                    delegation
                },
                {
                    fn getStakeAtBlockNumberAndIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumberAndIndex)
                    }
                    getStakeAtBlockNumberAndIndex
                },
                {
                    fn getLatestStakeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getLatestStakeUpdate)
                    }
                    getLatestStakeUpdate
                },
                {
                    fn getStakeAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
                },
                {
                    fn initializeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::initializeQuorum)
                    }
                    initializeQuorum
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
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::delegation(inner) => {
                    <delegationCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::updateOperatorStake(inner) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(inner) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::WEIGHTING_DIVISOR(inner) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::addStrategies(inner) => {
                    <addStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::deregisterOperator(inner) => {
                    <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentStake(inner) => {
                    <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getCurrentTotalStake(inner) => {
                    <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getLatestStakeUpdate(inner) => {
                    <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumber(inner) => {
                    <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeAtBlockNumberAndIndex(inner) => {
                    <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistory(inner) => {
                    <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeHistoryLength(inner) => {
                    <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateAtIndex(inner) => {
                    <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getStakeUpdateIndexAtBlockNumber(inner) => {
                    <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeAtBlockNumberFromIndex(inner) => {
                    <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeHistoryLength(inner) => {
                    <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeIndicesAtBlockNumber(inner) => {
                    <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::getTotalStakeUpdateAtIndex(inner) => {
                    <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeQuorum(inner) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::minimumStakeForQuorum(inner) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::modifyStrategyParams(inner) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registerOperator(inner) => {
                    <registerOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::registryCoordinator(inner) => {
                    <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::removeStrategies(inner) => {
                    <removeStrategiesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategiesPerQuorum(inner) => {
                    <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParams(inner) => {
                    <strategyParamsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsByIndex(inner) => {
                    <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::strategyParamsLength(inner) => {
                    <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::updateOperatorStake(inner) => {
                    <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::weightOfOperatorForQuorum(inner) => {
                    <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`StakeRegistry`](self) events.
    pub enum StakeRegistryEvents {
        MinimumStakeForQuorumUpdated(MinimumStakeForQuorumUpdated),
        OperatorStakeUpdate(OperatorStakeUpdate),
        QuorumCreated(QuorumCreated),
        StrategyAddedToQuorum(StrategyAddedToQuorum),
        StrategyMultiplierUpdated(StrategyMultiplierUpdated),
        StrategyRemovedFromQuorum(StrategyRemovedFromQuorum),
    }
    #[automatically_derived]
    impl StakeRegistryEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                16u8,
                86u8,
                94u8,
                86u8,
                202u8,
                203u8,
                243u8,
                46u8,
                202u8,
                38u8,
                121u8,
                69u8,
                240u8,
                84u8,
                254u8,
                192u8,
                46u8,
                89u8,
                117u8,
                0u8,
                50u8,
                209u8,
                19u8,
                211u8,
                48u8,
                33u8,
                130u8,
                173u8,
                150u8,
                127u8,
                84u8,
                4u8,
            ],
            [
                17u8,
                165u8,
                100u8,
                19u8,
                34u8,
                218u8,
                29u8,
                255u8,
                86u8,
                164u8,
                182u8,
                110u8,
                170u8,
                195u8,
                31u8,
                250u8,
                70u8,
                82u8,
                149u8,
                236u8,
                233u8,
                7u8,
                205u8,
                22u8,
                52u8,
                55u8,
                121u8,
                59u8,
                77u8,
                0u8,
                154u8,
                117u8,
            ],
            [
                38u8,
                238u8,
                207u8,
                242u8,
                183u8,
                11u8,
                10u8,
                113u8,
                16u8,
                79u8,
                244u8,
                217u8,
                64u8,
                186u8,
                113u8,
                98u8,
                210u8,
                58u8,
                149u8,
                194u8,
                72u8,
                119u8,
                31u8,
                196u8,
                135u8,
                167u8,
                190u8,
                23u8,
                165u8,
                150u8,
                179u8,
                207u8,
            ],
            [
                47u8,
                82u8,
                125u8,
                82u8,
                126u8,
                149u8,
                216u8,
                254u8,
                64u8,
                174u8,
                197u8,
                83u8,
                119u8,
                116u8,
                59u8,
                183u8,
                121u8,
                8u8,
                125u8,
                163u8,
                246u8,
                208u8,
                208u8,
                143u8,
                18u8,
                227u8,
                100u8,
                68u8,
                218u8,
                98u8,
                50u8,
                125u8,
            ],
            [
                49u8,
                250u8,
                46u8,
                44u8,
                210u8,
                128u8,
                201u8,
                55u8,
                94u8,
                19u8,
                255u8,
                207u8,
                61u8,
                129u8,
                226u8,
                55u8,
                129u8,
                0u8,
                24u8,
                110u8,
                64u8,
                88u8,
                248u8,
                211u8,
                221u8,
                182u8,
                144u8,
                184u8,
                45u8,
                205u8,
                49u8,
                247u8,
            ],
            [
                131u8,
                26u8,
                156u8,
                134u8,
                196u8,
                91u8,
                179u8,
                3u8,
                202u8,
                243u8,
                240u8,
                100u8,
                190u8,
                43u8,
                194u8,
                185u8,
                253u8,
                78u8,
                207u8,
                25u8,
                228u8,
                124u8,
                74u8,
                192u8,
                42u8,
                97u8,
                231u8,
                93u8,
                171u8,
                254u8,
                85u8,
                180u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for StakeRegistryEvents {
        const NAME: &'static str = "StakeRegistryEvents";
        const COUNT: usize = 6usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::MinimumStakeForQuorumUpdated)
                }
                Some(
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorStakeUpdate)
                }
                Some(<QuorumCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::QuorumCreated)
                }
                Some(
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyAddedToQuorum)
                }
                Some(
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyMultiplierUpdated)
                }
                Some(
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StrategyRemovedFromQuorum)
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
    impl alloy_sol_types::private::IntoLogData for StakeRegistryEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyAddedToQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyMultiplierUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StrategyRemovedFromQuorum(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StakeRegistryInstance<T, P, N> {
        StakeRegistryInstance::<T, P, N>::new(address, provider)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StakeRegistryInstance<T, P, N>>,
    > {
        StakeRegistryInstance::<
            T,
            P,
            N,
        >::deploy(provider, _registryCoordinator, _delegationManager)
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
        _delegationManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        StakeRegistryInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _registryCoordinator, _delegationManager)
    }
    /**A [`StakeRegistry`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StakeRegistry`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StakeRegistryInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StakeRegistryInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StakeRegistryInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StakeRegistryInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
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
            _registryCoordinator: alloy::sol_types::private::Address,
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<StakeRegistryInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _delegationManager,
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
            _delegationManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _delegationManager,
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
    impl<T, P: ::core::clone::Clone, N> StakeRegistryInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StakeRegistryInstance<T, P, N> {
            StakeRegistryInstance {
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
    > StakeRegistryInstance<T, P, N> {
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
        ///Creates a new call builder for the [`MAX_WEIGHING_FUNCTION_LENGTH`] function.
        pub fn MAX_WEIGHING_FUNCTION_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_WEIGHING_FUNCTION_LENGTHCall, N> {
            self.call_builder(
                &MAX_WEIGHING_FUNCTION_LENGTHCall {
                },
            )
        }
        ///Creates a new call builder for the [`WEIGHTING_DIVISOR`] function.
        pub fn WEIGHTING_DIVISOR(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, WEIGHTING_DIVISORCall, N> {
            self.call_builder(&WEIGHTING_DIVISORCall {})
        }
        ///Creates a new call builder for the [`addStrategies`] function.
        pub fn addStrategies(
            &self,
            quorumNumber: u8,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, addStrategiesCall, N> {
            self.call_builder(
                &addStrategiesCall {
                    quorumNumber,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(
                &deregisterOperatorCall {
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentStake`] function.
        pub fn getCurrentStake(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentStakeCall, N> {
            self.call_builder(
                &getCurrentStakeCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getCurrentTotalStake`] function.
        pub fn getCurrentTotalStake(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalStakeCall, N> {
            self.call_builder(
                &getCurrentTotalStakeCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getLatestStakeUpdate`] function.
        pub fn getLatestStakeUpdate(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestStakeUpdateCall, N> {
            self.call_builder(
                &getLatestStakeUpdateCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumber`] function.
        pub fn getStakeAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeAtBlockNumberCall, N> {
            self.call_builder(
                &getStakeAtBlockNumberCall {
                    operatorId,
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumberAndIndex`] function.
        pub fn getStakeAtBlockNumberAndIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getStakeAtBlockNumberAndIndexCall,
            N,
        > {
            self.call_builder(
                &getStakeAtBlockNumberAndIndexCall {
                    quorumNumber,
                    blockNumber,
                    operatorId,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeHistory`] function.
        pub fn getStakeHistory(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryCall, N> {
            self.call_builder(
                &getStakeHistoryCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeHistoryLength`] function.
        pub fn getStakeHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryLengthCall, N> {
            self.call_builder(
                &getStakeHistoryLengthCall {
                    operatorId,
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeUpdateAtIndex`] function.
        pub fn getStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeUpdateAtIndexCall, N> {
            self.call_builder(
                &getStakeUpdateAtIndexCall {
                    quorumNumber,
                    operatorId,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getStakeUpdateIndexAtBlockNumber`] function.
        pub fn getStakeUpdateIndexAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getStakeUpdateIndexAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getStakeUpdateIndexAtBlockNumberCall {
                    operatorId,
                    quorumNumber,
                    blockNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeAtBlockNumberFromIndex`] function.
        pub fn getTotalStakeAtBlockNumberFromIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getTotalStakeAtBlockNumberFromIndexCall,
            N,
        > {
            self.call_builder(
                &getTotalStakeAtBlockNumberFromIndexCall {
                    quorumNumber,
                    blockNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeHistoryLength`] function.
        pub fn getTotalStakeHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeHistoryLengthCall, N> {
            self.call_builder(
                &getTotalStakeHistoryLengthCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeIndicesAtBlockNumber`] function.
        pub fn getTotalStakeIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            getTotalStakeIndicesAtBlockNumberCall,
            N,
        > {
            self.call_builder(
                &getTotalStakeIndicesAtBlockNumberCall {
                    blockNumber,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`getTotalStakeUpdateAtIndex`] function.
        pub fn getTotalStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeUpdateAtIndexCall, N> {
            self.call_builder(
                &getTotalStakeUpdateAtIndexCall {
                    quorumNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`initializeQuorum`] function.
        pub fn initializeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeQuorumCall, N> {
            self.call_builder(
                &initializeQuorumCall {
                    quorumNumber,
                    minimumStake,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`minimumStakeForQuorum`] function.
        pub fn minimumStakeForQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, minimumStakeForQuorumCall, N> {
            self.call_builder(&minimumStakeForQuorumCall { _0 })
        }
        ///Creates a new call builder for the [`modifyStrategyParams`] function.
        pub fn modifyStrategyParams(
            &self,
            quorumNumber: u8,
            strategyIndices: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
            newMultipliers: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U96,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyStrategyParamsCall, N> {
            self.call_builder(
                &modifyStrategyParamsCall {
                    quorumNumber,
                    strategyIndices,
                    newMultipliers,
                },
            )
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(
                &registerOperatorCall {
                    operator,
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`registryCoordinator`] function.
        pub fn registryCoordinator(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, registryCoordinatorCall, N> {
            self.call_builder(&registryCoordinatorCall {})
        }
        ///Creates a new call builder for the [`removeStrategies`] function.
        pub fn removeStrategies(
            &self,
            quorumNumber: u8,
            indicesToRemove: alloy::sol_types::private::Vec<
                alloy::sol_types::private::primitives::aliases::U256,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, removeStrategiesCall, N> {
            self.call_builder(
                &removeStrategiesCall {
                    quorumNumber,
                    indicesToRemove,
                },
            )
        }
        ///Creates a new call builder for the [`setMinimumStakeForQuorum`] function.
        pub fn setMinimumStakeForQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<T, &P, setMinimumStakeForQuorumCall, N> {
            self.call_builder(
                &setMinimumStakeForQuorumCall {
                    quorumNumber,
                    minimumStake,
                },
            )
        }
        ///Creates a new call builder for the [`strategiesPerQuorum`] function.
        pub fn strategiesPerQuorum(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategiesPerQuorumCall, N> {
            self.call_builder(&strategiesPerQuorumCall { _0, _1 })
        }
        ///Creates a new call builder for the [`strategyParams`] function.
        pub fn strategyParams(
            &self,
            _0: u8,
            _1: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsCall, N> {
            self.call_builder(&strategyParamsCall { _0, _1 })
        }
        ///Creates a new call builder for the [`strategyParamsByIndex`] function.
        pub fn strategyParamsByIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsByIndexCall, N> {
            self.call_builder(
                &strategyParamsByIndexCall {
                    quorumNumber,
                    index,
                },
            )
        }
        ///Creates a new call builder for the [`strategyParamsLength`] function.
        pub fn strategyParamsLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsLengthCall, N> {
            self.call_builder(
                &strategyParamsLengthCall {
                    quorumNumber,
                },
            )
        }
        ///Creates a new call builder for the [`updateOperatorStake`] function.
        pub fn updateOperatorStake(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorStakeCall, N> {
            self.call_builder(
                &updateOperatorStakeCall {
                    operator,
                    operatorId,
                    quorumNumbers,
                },
            )
        }
        ///Creates a new call builder for the [`weightOfOperatorForQuorum`] function.
        pub fn weightOfOperatorForQuorum(
            &self,
            quorumNumber: u8,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, weightOfOperatorForQuorumCall, N> {
            self.call_builder(
                &weightOfOperatorForQuorumCall {
                    quorumNumber,
                    operator,
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
    > StakeRegistryInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`MinimumStakeForQuorumUpdated`] event.
        pub fn MinimumStakeForQuorumUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, MinimumStakeForQuorumUpdated, N> {
            self.event_filter::<MinimumStakeForQuorumUpdated>()
        }
        ///Creates a new event filter for the [`OperatorStakeUpdate`] event.
        pub fn OperatorStakeUpdate_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorStakeUpdate, N> {
            self.event_filter::<OperatorStakeUpdate>()
        }
        ///Creates a new event filter for the [`QuorumCreated`] event.
        pub fn QuorumCreated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, QuorumCreated, N> {
            self.event_filter::<QuorumCreated>()
        }
        ///Creates a new event filter for the [`StrategyAddedToQuorum`] event.
        pub fn StrategyAddedToQuorum_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyAddedToQuorum, N> {
            self.event_filter::<StrategyAddedToQuorum>()
        }
        ///Creates a new event filter for the [`StrategyMultiplierUpdated`] event.
        pub fn StrategyMultiplierUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyMultiplierUpdated, N> {
            self.event_filter::<StrategyMultiplierUpdated>()
        }
        ///Creates a new event filter for the [`StrategyRemovedFromQuorum`] event.
        pub fn StrategyRemovedFromQuorum_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StrategyRemovedFromQuorum, N> {
            self.event_filter::<StrategyRemovedFromQuorum>()
        }
    }
}
